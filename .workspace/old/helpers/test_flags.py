from __future__ import annotations
import subprocess
import time
import os
from typing import Optional
import re
from rich.progress import Progress, SpinnerColumn, BarColumn, TimeRemainingColumn
from rich.console import Console
from dataclasses import dataclass, field

from concurrent.futures import ThreadPoolExecutor, as_completed

error_file = "compile_error.txt"

SLOW_THRESHOLD_SECONDS = 120
SLOW_LOG_DIR = "logs"


@dataclass
class RustError:
    """Represents a single Rust compilation error or warning."""

    error_type: str  # 'error' or 'warning'
    code: str  # e.g. 'E0599', 'unused_imports'
    message: str
    location: str  # file:line:col
    full_text: str
    # Each element is a frozenset of feature flags active during that occurrence.
    # Using frozenset makes the set of occurrences itself hashable/comparable.
    feature_combinations: list[frozenset[str]] = field(default_factory=list)

    # ------------------------------------------------------------------
    # Identity is based on the error itself, not which features triggered it
    # ------------------------------------------------------------------
    def _key(self) -> tuple[str, str, str, str]:
        return (self.error_type, self.code, self.message, self.location)

    def __hash__(self) -> int:
        return hash(self._key())

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, RustError):
            return False
        return self._key() == other._key()

    def merge(self, other: "RustError") -> None:
        """Absorb feature combinations from another occurrence of the same error."""
        for combo in other.feature_combinations:
            if combo not in self.feature_combinations:
                self.feature_combinations.append(combo)


# ---------------------------------------------------------------------------
# Top-level entry point
# ---------------------------------------------------------------------------


def parse_and_deduplicate_rust_errors(file_content: str) -> str:
    """
    Parse, deduplicate, and format Rust compilation errors.

    Args:
        file_content: Raw content of the error log file.

    Returns:
        Human-readable, deduplicated error report.
    """
    unique_errors, unparsed_sections = _parse_rust_errors(file_content)
    sorted_errors = _sort_errors(unique_errors)
    return _format_output(sorted_errors, unparsed_sections)


# ---------------------------------------------------------------------------
# Parsing
# ---------------------------------------------------------------------------

# Matches the separator that divides compilation attempts in the log.
_SEPARATOR_RE = re.compile(r"-{80,}")

# Matches the start of an error/warning block produced by rustc.
# Examples:
#   error[E0599]: no method named `foo` found …
#   warning: unused import: `std::fmt`
_BLOCK_START_RE = re.compile(
    r"^(error(\[[^\]]+\])?|warning(\[[^\]]+\])?)\s*:", re.MULTILINE
)

# Matches the source location line: --> src/lib.rs:42:8
_LOCATION_RE = re.compile(r"-->\s*(.+)")

# Matches an error/warning code in brackets: [E0599] or [unused_imports]
_CODE_RE = re.compile(r"\[([^\]]+)\]")

# Matches the human-readable message after the colon on the first line.
_MESSAGE_RE = re.compile(r"^(?:error|warning)(?:\[[^\]]+\])?\s*:\s*(.+)", re.IGNORECASE)

# Matches the "could not compile …" summary line (not a real error block).
_SUMMARY_RE = re.compile(r"could not compile")

# Matches the feature-flag combination header inside a section.
_FEATURES_RE = re.compile(r"Combination (?:failed|succeeded)[^:]*:\s*\(([^)]*)\)")

# Matches lines injected by the `cross` wrapper tool.
_CROSS_LINE_RE = re.compile(r"^\s*\[cross\].*$", re.MULTILINE)


def _strip_cross_noise(section: str) -> str:
    """Remove [cross] prefix lines injected by the cross wrapper tool."""
    return _CROSS_LINE_RE.sub("", section)


def _extract_features(section: str) -> frozenset[str]:
    """Return the set of feature flags active for this compilation section."""
    m = _FEATURES_RE.search(section)
    if not m:
        return frozenset()
    raw = m.group(1)
    features = {
        f.strip().strip("'\"") for f in raw.split(",") if f.strip().strip("'\"")
    }
    return frozenset(features)


def _split_into_blocks(section: str) -> list[str]:
    """
    Split a compilation section into individual rustc error/warning blocks.

    Strategy: find all positions where a new block starts (via _BLOCK_START_RE),
    then slice the text between consecutive starts.  This avoids the fragile
    line-by-line state machine and correctly handles multi-line snippets,
    help text, notes, etc.
    """
    starts = [m.start() for m in _BLOCK_START_RE.finditer(section)]
    if not starts:
        return []

    blocks: list[str] = []
    for i, start in enumerate(starts):
        end = starts[i + 1] if i + 1 < len(starts) else len(section)
        block = section[start:end].strip()
        if block:
            blocks.append(block)
    return blocks


def _parse_block(block: str, features: frozenset[str]) -> Optional[RustError]:
    """
    Parse a single rustc error/warning block into a RustError.

    Returns None for non-actionable blocks (e.g. "could not compile" summaries).
    """
    # Skip summary lines that don't have a source location.
    # Only check the first line to avoid discarding real errors that mention
    # "could not compile" somewhere in their body text.
    first_line = block.splitlines()[0] if block.strip() else ""
    if _SUMMARY_RE.search(first_line) and not _LOCATION_RE.search(block):
        return None

    # error type
    error_type = "warning" if first_line.lstrip().startswith("warning") else "error"

    # error code  – prefer the bracketed code on the first line
    code_m = _CODE_RE.search(first_line)
    if not code_m:
        # fall back: look for #[warn(...)] or #[allow(...)] annotations in the block
        code_m = re.search(r"#\[(?:warn|deny|allow)\(([^\)]+)\)\]", block)
    code = code_m.group(1).strip() if code_m else "unknown"

    # source location
    loc_m = _LOCATION_RE.search(block)
    location = loc_m.group(1).strip() if loc_m else ""

    # human-readable message (first line, after the colon)
    msg_m = _MESSAGE_RE.match(first_line.strip())
    message = msg_m.group(1).strip() if msg_m else first_line.strip()

    return RustError(
        error_type=error_type,
        code=code,
        message=message,
        location=location,
        full_text=block,
        feature_combinations=[features],
    )


def _parse_rust_errors(content: str) -> tuple[dict[RustError, RustError], list[str]]:
    """
    Parse the full log and return:
      - a dict of unique RustError → canonical RustError (duplicates merged)
      - a list of raw section texts that could not be parsed at all
    """
    unique: dict[RustError, RustError] = {}
    unparsed: list[str] = []

    for section in _SEPARATOR_RE.split(content):
        if not section.strip():
            continue

        section = _strip_cross_noise(section)

        features = _extract_features(section)
        blocks = _split_into_blocks(section)

        if not blocks:
            # No recognizable error/warning blocks found — keep the raw text.
            stripped = section.strip()
            if stripped:
                unparsed.append(stripped)
            continue

        parsed_any = False
        for raw_block in blocks:
            error = _parse_block(raw_block, features)
            if error is None:
                continue
            parsed_any = True
            if error in unique:
                unique[error].merge(error)
            else:
                unique[error] = error

        if not parsed_any:
            # Blocks were found but every one was filtered (e.g. all summaries).
            # Fall back to the raw section so nothing is silently dropped.
            stripped = section.strip()
            if stripped:
                unparsed.append(stripped)

    return unique, unparsed


# ---------------------------------------------------------------------------
# Sorting
# ---------------------------------------------------------------------------


def _sort_errors(unique: dict[RustError, RustError]) -> list[RustError]:
    """Sort errors: hard errors first, then by code, then by location."""

    def _key(e: RustError) -> tuple[int, str, str]:
        return (
            0 if e.error_type == "error" else 1,
            e.code,
            e.location,
        )

    return sorted(unique.values(), key=_key)


# ---------------------------------------------------------------------------
# Formatting
# ---------------------------------------------------------------------------


def _format_feature_combo(combo: frozenset[str]) -> str:
    if not combo:
        return "(no features)"
    return ", ".join(sorted(combo))


def _format_output(errors: list[RustError], unparsed: list[str]) -> str:
    lines: list[str] = []

    total_errors = sum(1 for e in errors if e.error_type == "error")
    total_warnings = sum(1 for e in errors if e.error_type == "warning")

    sep = "=" * 80
    dash = "-" * 80

    lines += [
        sep,
        f"SUMMARY: {total_errors} unique error(s), {total_warnings} unique warning(s)"
        + (f", {len(unparsed)} unparsed section(s)" if unparsed else ""),
        sep,
        "",
    ]

    for error in errors:
        combos = sorted(
            (_format_feature_combo(c) for c in error.feature_combinations),
            key=lambda s: (s == "(no features)", s),
        )
        n = len(combos)

        lines.append(sep)
        lines.append(f"[{error.error_type.upper()}] {error.code}")
        lines.append(f"Triggered by {n} feature combination(s):")

        display = combos if n <= 10 else combos[:10]
        for combo_str in display:
            lines.append(f"  - {combo_str}")
        if n > 10:
            lines.append(f"  … and {n - 10} more")

        lines.append(dash)
        lines.append(f"Message:  {error.message}")
        if error.location:
            lines.append(f"Location: {error.location}")
        lines.append("")
        lines.append("Full error:")
        lines.append(error.full_text)
        lines.append("")

    if unparsed:
        lines.append(sep)
        lines.append(f"UNPARSED SECTIONS ({len(unparsed)})")
        lines.append(
            "The following output could not be parsed and is reproduced verbatim:"
        )
        lines.append(sep)
        lines.append("")
        for i, raw in enumerate(unparsed, start=1):
            lines.append(f"--- Unparsed section {i} ---")
            lines.append(raw)
            lines.append("")

    return "\n".join(lines)


console = Console()


# ---------------------------------------------------------------------------
# Slow-run log helpers
# ---------------------------------------------------------------------------


def _combo_log_path(combo, target: str | None) -> str:
    """Return a stable file path for a combo's slow-run log."""
    label = "_".join(sorted(combo)) if combo else "no_features"
    if target:
        label = f"{target}__{label}"
    safe = re.sub(r"[^\w\-.]", "_", label)
    return os.path.join(SLOW_LOG_DIR, f"{safe}.txt")


def _write_slow_log(combo, target: str | None, stderr: str) -> None:
    os.makedirs(SLOW_LOG_DIR, exist_ok=True)
    path = _combo_log_path(combo, target)
    with open(path, "w", encoding="utf-8") as f:
        label = ", ".join(combo) if combo else "(no features)"
        f.write(f"Slow combination log\nCombo: {label}\nTarget: {target or 'host'}\n")
        f.write("=" * 80 + "\n")
        f.write(stderr)


def _delete_slow_log(combo, target: str | None) -> None:
    path = _combo_log_path(combo, target)
    if os.path.exists(path):
        os.remove(path)


# ---------------------------------------------------------------------------
# Live error-file helpers
# ---------------------------------------------------------------------------

import threading

# In-memory store of all (combo, stderr) failures seen so far.
# Protected by a lock so concurrent futures can safely append and reformat.
_error_accumulator: list[tuple[tuple[str, ...], str]] = []
_error_accumulator_lock = threading.Lock()


def _record_error_and_rewrite(combo, stderr: str) -> None:
    """
    Add this failure to the in-memory list, then rebuild and overwrite
    compile_error.txt from scratch using the full parse+deduplicate pipeline.
    """
    with _error_accumulator_lock:
        _error_accumulator.append((combo, stderr))
        raw = ""
        for c, s in _error_accumulator:
            raw += "Combination failed: " + str(c) + "\n"
            raw += "Error:\n" + s + "\n"
            raw += "-" * 80 + "\n"
        parsed = parse_and_deduplicate_rust_errors(raw)
        with open(error_file, "w", encoding="utf-8") as f:
            f.write(parsed)


def _run_with_slow_log(
    cmd: list[str],
    combo,
    target: str | None,
    timeout: int = 900,
) -> tuple[int, str, float]:
    """
    Run *cmd*, collecting stderr.  If the process is still running after
    SLOW_THRESHOLD_SECONDS a watchdog thread writes a slow-log immediately.
    On success the slow-log is deleted; on failure it is kept.

    Returns (returncode, stderr_text, duration).
    """
    import threading

    start = time.time()
    stderr_chunks: list[str] = []
    slow_log_written = threading.Event()

    try:
        proc = subprocess.Popen(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            encoding="utf-8",
            errors="replace",
        )
    except Exception as e:
        return -1, f"Exception: {e}", time.time() - start

    # Watchdog: fires SLOW_THRESHOLD_SECONDS after the process starts.
    def _watchdog():
        proc.wait()  # no-op if it finishes first — timer will have been cancelled

    def _slow_trigger():
        if proc.poll() is None:  # still running
            # Snapshot whatever stderr we have so far
            partial = "".join(stderr_chunks)
            _write_slow_log(
                combo,
                target,
                f"[still running at {SLOW_THRESHOLD_SECONDS}s]\n{partial}",
            )
            slow_log_written.set()

    timer = threading.Timer(SLOW_THRESHOLD_SECONDS, _slow_trigger)
    timer.daemon = True
    timer.start()

    try:
        stdout_data, stderr_data = proc.communicate(timeout=timeout)
    except subprocess.TimeoutExpired:
        proc.kill()
        stdout_data, stderr_data = proc.communicate()
        timer.cancel()
        duration = time.time() - start
        msg = f"Process timed out after {timeout} seconds\n{stderr_data or stdout_data}"
        if slow_log_written.is_set():
            _write_slow_log(combo, target, msg)  # overwrite with full output
        return -1, msg, duration

    timer.cancel()
    duration = time.time() - start
    returncode = proc.returncode

    if returncode != 0:
        # if slow_log_written.is_set():
        #     # Overwrite the partial log with the complete stderr
        _write_slow_log(combo, target, stderr_data)
    else:
        _delete_slow_log(combo, target)

    return returncode, stderr_data, duration


def check_feature_combo(combo, idx, target: str | None = None):
    """Check a single feature combination."""
    cmd = [
        "cargo" if target is None else "cross",
        "+nightly",
        "check",
        "--lib",
        "--no-default-features",
        "-q",
    ]
    if target:
        cmd.extend(["--target", target])
    for feature in combo:
        cmd.extend(["--features", feature])

    returncode, stderr, duration = _run_with_slow_log(cmd, combo, target)
    return {
        "combo": combo,
        "idx": idx,
        "returncode": returncode,
        "stderr": stderr,
        "duration": duration,
        "success": returncode == 0,
    }


def run_checks_smart(
    final_combinations: list[str], total: int, target: str | None = None
):

    os.environ["CARGO_INCREMENTAL"] = "1"
    os.environ.setdefault("CARGO_TARGET_DIR", "target")

    # Step 1: compile with "all" features first to warm up the dependency cache
    console.print(
        "[bold cyan]Step 1/2:[/bold cyan] Compiling with --features all to warm dependency cache..."
    )
    warm_cmd = [
        "cargo" if target is None else "cross",
        "+nightly",
        "check",
        # "-Zfeatures=itarget",
        "--lib",
        "--no-default-features",
        "--features",
        "all",
    ]
    if target:
        warm_cmd.extend(["--target", target])

    warm_returncode, warm_stderr, warm_duration = _run_with_slow_log(
        warm_cmd, combo=("warmup",), target=target, timeout=600
    )
    if warm_returncode != 0:
        console.print(
            "[yellow]⚠ 'all' features build failed — continuing anyway.[/yellow]"
        )
        console.print(warm_stderr[:2000])
    else:
        console.print(
            f"[green]✓ Warm-up build succeeded ({warm_duration:.1f}s).[/green]\n"
        )

    # Step 2: run all remaining combinations in parallel threads
    # Filter out the "all" sentinel — it was already handled above
    remaining = [c for c in final_combinations if c != ("all",) and list(c) != ["all"]]
    remaining_total = len(remaining)

    console.print(
        f"[bold cyan]Step 2/2:[/bold cyan] Checking {remaining_total} feature combinations "
        f"in parallel...\n"
    )

    start_time = time.time()
    errors = []
    durations = []
    max_workers = min(os.cpu_count() or 4, 8)

    # Reset the accumulator so errors from a previous target don't carry over.
    with _error_accumulator_lock:
        _error_accumulator.clear()

    with Progress(
        SpinnerColumn(),
        "[progress.description]{task.description}",
        BarColumn(),
        "[progress.percentage]{task.percentage:>3.0f}%",
        TimeRemainingColumn(),
        console=console,
    ) as progress:
        task = progress.add_task("Checking combinations...", total=remaining_total)

        with ThreadPoolExecutor(max_workers=max_workers) as executor:
            futures = {
                executor.submit(check_feature_combo, combo, idx, target): (combo, idx)
                for idx, combo in enumerate(remaining, start=1)
            }

            for future in as_completed(futures):
                try:
                    result = future.result()
                    combo: list[str] = result[
                        "combo"
                    ]  # pyright: ignore[reportAssignmentType]
                    idx: int = result["idx"]  # pyright: ignore[reportAssignmentType]
                    duration = result["duration"]
                    durations.append(duration)

                    avg_duration = sum(durations) / len(durations) if durations else 0
                    label = ", ".join(combo) if combo else "(no features)"

                    if result["returncode"] != 0:
                        errors.append((combo, result["stderr"]))
                        _record_error_and_rewrite(
                            combo,
                            result["stderr"],  # pyright: ignore[reportArgumentType]
                        )
                        console.print(
                            f"[red]✗[/red] [{idx}/{remaining_total}] [red]{label}[/red] "
                            + f"[dim]({duration:.2f}s)[/dim]"
                        )
                    else:
                        console.print(
                            f"[green]✓[/green] [{idx}/{remaining_total}] [green]{label}[/green] "
                            + f"[dim]({duration:.2f}s, avg: {avg_duration:.2f}s)[/dim]"
                        )

                    progress.update(task, advance=1)
                except Exception as e:
                    console.print(f"[red]Error processing result: {e}[/red]")
                    progress.update(task, advance=1)

    elapsed = time.time() - start_time

    # error file is already up to date — written live after each failure

    console.print("\n" + "=" * 60)
    if durations:
        console.print("[bold cyan]Statistics:[/bold cyan]")
        console.print(f"  Total combinations: {remaining_total}")
        console.print(f"  Total time: {elapsed:.2f}s")
        console.print(f"  Total serial time would be: {sum(durations):.2f}s")
        console.print(f"  Average per combo: {sum(durations) / len(durations):.2f}s")
        console.print(f"  Fastest: {min(durations):.2f}s")
        console.print(f"  Slowest: {max(durations):.2f}s")
        console.print(f"  Parallel workers: {max_workers}")
        console.print(f"  Speedup: ~{(sum(durations) / elapsed):.1f}x")

    if errors:
        console.print(
            f"\n[red]❌ {len(errors)} combination(s) failed "
            f"({len(errors) / remaining_total * 100:.1f}%). "
            f"See {error_file} for details.[/red]"
        )
    else:
        console.print(
            "\n[bold green]✅ All combinations compiled successfully![/bold green]"
        )
        if os.path.exists(error_file):
            os.remove(error_file)

    console.print("=" * 60)


def run_checks(final_combinations: list[list[tuple[str, ...]]], total: int):
    """Run all feature combination checks."""
    from concurrent.futures import ThreadPoolExecutor, as_completed

    os.environ["CARGO_INCREMENTAL"] = "1"
    os.environ.setdefault("CARGO_TARGET_DIR", "target")

    start_time = time.time()
    errors = []
    durations = []
    max_workers = min(os.cpu_count() or 4, 8)

    with Progress(
        SpinnerColumn(),
        "[progress.description]{task.description}",
        BarColumn(),
        "[progress.percentage]{task.percentage:>3.0f}%",
        TimeRemainingColumn(),
        console=console,
    ) as progress:
        task = progress.add_task("Building combinations...", total=total)

        with ThreadPoolExecutor(max_workers=max_workers) as executor:
            futures = {
                executor.submit(check_feature_combo, combo, idx): (combo, idx)
                for idx, combo in enumerate(final_combinations, start=1)
            }

            for future in as_completed(futures):
                try:
                    result = future.result()
                    combo: list[str] = result[
                        "combo"
                    ]  # pyright: ignore[reportAssignmentType]
                    idx = result["idx"]
                    duration = result["duration"]
                    durations.append(duration)

                    avg_duration = sum(durations) / len(durations) if durations else 0

                    if result["returncode"] != 0:
                        errors.append((combo, result["stderr"]))

                        # Live write
                        _record_error_and_rewrite(combo, result["stderr"])

                        console.print(
                            f"[red]✗[/red] [{idx}/{total}] [red]{', '.join(combo)}[/red] "
                            f"[dim]({duration:.2f}s)[/dim]"
                        )
                    else:
                        console.print(
                            f"[green]✓[/green] [{idx}/{total}] [green]{', '.join(combo)}[/green] "
                            f"[dim]({duration:.2f}s, avg: {avg_duration:.2f}s)[/dim]"
                        )

                    progress.update(task, advance=1)
                except Exception as e:
                    console.print(f"[red]Error processing result: {e}[/red]")
                    progress.update(task, advance=1)

    end_time = time.time()
    elapsed = end_time - start_time

    if durations:
        console.print("\n" + "=" * 60)
        console.print("[bold cyan]Statistics:[/bold cyan]")
        console.print(f"  Total combinations: {total}")
        console.print(f"  Total time: {elapsed:.2f}s")
        console.print(f"  Total time without parallelization: {sum(durations):.2f}s")
        console.print(f"  Average per combo: {sum(durations) / len(durations):.2f}s")
        console.print(f"  Fastest: {min(durations):.2f}s")
        console.print(f"  Slowest: {max(durations):.2f}s")
        console.print(f"  Parallel workers: {max_workers}")
        console.print(f"  Speedup: ~{(sum(durations) / elapsed):.1f}x")

    if errors:
        # error file already up to date (written live)
        console.print(
            f"\n[red]❌ {len(errors)} combination(s) failed ({len(errors)/total*100:.1f}%). "
            f"See compile_error.txt for details.[/red]"
        )
    else:
        console.print(
            "\n[bold green]✅ All combinations compiled successfully![/bold green]"
        )
        if os.path.exists(error_file):
            os.remove(error_file)

    console.print("=" * 60)


if __name__ == "__main__":
    from copy import deepcopy
    import math

    def partial_permutation_count(m: int, include_empty: bool = False) -> int:
        total = sum(math.factorial(m) // math.factorial(m - k) for k in range(1, m + 1))
        return total + (1 if include_empty else 0)

    flags: dict[str, list[str]] = {
        "imagery": [],
        "svg": [],
        "minifb": ["system", "keycodes", "keyboard_query", "svg", "std"],
        "glfw": ["system", "keycodes", "std"],
        "system": ["std"],
        "font_support": ["std"],
        "keycodes": ["std"],
        "keyboard_query": ["std"],
        "discord": ["std"],
        "std": [],
        "num_traits": [],
        "miri": [],
        "console": [],
        "serde": [],
        "indexmap": ["std"],
        "ahash": ["std"],
        "internal_use_ahash": ["ahash", "std"],
        "internal_use_indexmap": ["std", "indexmap"],
        "dashmap": ["std"],
        "masstree": ["std"],
        "bitcode": ["std"],
        "wincode": [],
        "strum": [],
        "enum_ext": ["std"],
        # "nightly": [],
    }
    # Flags that only really depend on other flags and as such do not need to be checked
    no_correlation = [
        "discord",
        "keyboard_query",
        "font_support",
        "glfw",
        "minifb",
        "imagery",
        "keycodes",
        "svg",
        "system",
        "ahash",
        "miri",
        "console",
        "serde",
        "indexmap",
        "internal_use_ahash",
        "internal_use_indexmap",
        "dashmap",
        "masstree",
        "bitcode",
        "wincode",
        "strum",
        "enum_ext",
    ]

    targets = [
        "x86_64-pc-windows-msvc",
        "x86_64-unknown-linux-gnu",
        "x86_64-apple-darwin",
        "wasm32-unknown-unknown",
    ]

    out = subprocess.check_output(["rustc", "-vV"], text=True)

    host = None
    for line in out.splitlines():
        if line.startswith("host:"):
            host = line.split(":", 1)[1].strip()
            break

    targets = [x if x != host else None for x in targets]

    def remove_items(list: list[str], items: list[str]):
        values = [flags[x] for x in items]
        for x in items:
            if x in list:
                list.remove(x)
        for v in values:
            remove_items(list, v)

    unique_sub_lists: dict[tuple[str, ...], list[str]] = {}

    for key, items in flags.items():
        sub_list = list(deepcopy(flags))
        for i in no_correlation:
            if i != key:
                if not i in sub_list:
                    print(i, sub_list)
                sub_list.remove(i)
        remove_items(sub_list, items)

        sub_list_tuple = tuple(sorted(sub_list))

        if sub_list_tuple not in unique_sub_lists:
            unique_sub_lists[sub_list_tuple] = []
        unique_sub_lists[sub_list_tuple].append(key)

    total_combinations: list[tuple[str, ...]] = []

    total = 0
    for sub_list_tuple, keys in unique_sub_lists.items():
        val = partial_permutation_count(len(sub_list_tuple))
        count = len(keys)
        total += val * count
        total_combinations.append(sub_list_tuple)
        print(
            f"Keys {keys} -> {len(sub_list_tuple)} flags, {val} permutations each, {count} occurrences: {sub_list_tuple}"
        )

    print("Estimated Total:", total + 2)

    import itertools

    def all_orders(lst: list[tuple[str, ...]]):
        out: list[tuple[tuple[str, ...], ...]] = []
        n = len(lst)
        for r in range(1, n + 1):
            for combo in itertools.combinations(lst, r):
                for perm in itertools.permutations(combo):
                    out.append(perm)
        return out

    final_combinations: list[tuple[str, ...]] = []
    for x in total_combinations:
        comb = all_orders(x)  # type: ignore
        comb = [list(x) for x in comb]
        [x.sort() for x in comb]
        comb = [tuple(x) for x in comb]
        final_combinations.extend(comb)

    print("With duplicates:", len(final_combinations) + 2)
    final_combinations = list(set(final_combinations))
    print("Without duplicates:", len(final_combinations) + 2)
    final_combinations.append(())
    final_combinations.insert(0, ("all",))  # type: ignore

    total = len(final_combinations)
    # run_checks_smart(final_combinations, total)  # pyright: ignore[reportArgumentType]
    # Run Windows check first
    # console.print(
    #     "[bold cyan]Checking Windows...[/bold cyan]"
    # )  # pyright: ignore[reportArgumentType]
    # run_checks_smart(final_combinations, total)

    # Only continue if no errors were produced

    if os.path.exists(error_file):
        os.remove(error_file)

    for target in targets:
        if os.path.exists("compile_error.txt"):
            console.print(f"\n[red]Error detected, skipping {target} [/red]")
            quit()
        console.print(f"\n[bold cyan]Checking {target}...[/bold cyan]")
        run_checks_smart(final_combinations, total, target=target)
        # if os.path.exists("compile_error.txt"):
        #     break  # Stop on first platform that fails
