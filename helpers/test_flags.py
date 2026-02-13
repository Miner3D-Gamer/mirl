import subprocess
import time
import os
from rich.progress import Progress, SpinnerColumn, BarColumn, TimeRemainingColumn
from rich.console import Console
import re
from dataclasses import dataclass, field
from typing import List, Dict, Set
from collections import defaultdict


@dataclass
class RustError:
    """Represents a single Rust compilation error or warning."""

    error_type: str  # 'error' or 'warning'
    code: str  # e.g., 'E0599', 'unused_imports'
    message: str
    location: str  # file:line:col
    full_text: str
    feature_flags: Set[str] = field(default_factory=set)

    def __hash__(self):
        return hash((self.error_type, self.code, self.message, self.location))

    def __eq__(self, other):
        if not isinstance(other, RustError):
            return False
        return (
            self.error_type == other.error_type
            and self.code == other.code
            and self.message == other.message
            and self.location == other.location
        )


def parse_rust_errors(content: str) -> Dict[str, List[RustError]]:
    """
    Parse a file containing multiple Rust compilation attempts.
    Returns a dictionary mapping unique errors to their list of occurrences.
    """
    # Split by the separator line
    sections = re.split(r"-{80,}", content)

    all_errors = defaultdict(list)

    for section in sections:
        if not section.strip():
            continue

        # Extract feature combination
        feature_match = re.search(r"Combination failed: \((.*?)\)", section)
        features = tuple()
        if feature_match:
            features_str = feature_match.group(1)
            # Parse the tuple representation
            features = tuple(
                f.strip().strip("'\"")
                for f in features_str.split(",")
                if f.strip().strip("'\"")
            )

        # Parse individual errors/warnings
        errors = parse_section_errors(section, features)

        for error in errors:
            all_errors[error].append(error)

    return all_errors


def parse_section_errors(section: str, features: tuple) -> List[RustError]:
    """Parse all errors and warnings from a single compilation section."""
    errors = []

    # Split section into individual error/warning blocks
    # Each block starts with 'error[' or 'warning:' and contains location info
    lines = section.split("\n")
    current_block = []
    in_error_block = False

    for line in lines:
        # Check if this is the start of an error or warning with details
        if (
            re.match(r"^(error\[|warning:)\s*", line)
            and "-->" in section[section.find(line) : section.find(line) + 200]
        ):
            # Save previous block if it exists
            if current_block and in_error_block:
                block_text = "\n".join(current_block)
                parsed = parse_error_block(block_text, features)
                if parsed:
                    errors.append(parsed)

            # Start new block
            current_block = [line]
            in_error_block = True
        elif in_error_block:
            # Continue building current block
            current_block.append(line)
            # Check if we've reached the end of this error (empty line or next error)
            if not line.strip() or re.match(
                r"^(error\[|warning:|For more information)", line
            ):
                if line.strip() and not re.match(r"^For more information", line):
                    # Don't include the next error's first line
                    current_block = current_block[:-1]
                if current_block:
                    block_text = "\n".join(current_block)
                    parsed = parse_error_block(block_text, features)
                    if parsed:
                        errors.append(parsed)
                current_block = (
                    [line]
                    if line.strip() and not re.match(r"^For more information", line)
                    else []
                )
                in_error_block = bool(current_block)

    # Don't forget the last block
    if current_block and in_error_block:
        block_text = "\n".join(current_block)
        parsed = parse_error_block(block_text, features)
        if parsed:
            errors.append(parsed)

    return errors


def parse_error_block(block: str, features: tuple) -> RustError:
    """Parse a single error or warning block."""
    # Skip compilation summary errors
    if "could not compile" in block and "-->" not in block:
        return None # pyright: ignore[reportReturnType]

    # Determine if it's an error or warning
    is_warning = block.strip().startswith("warning:")
    error_type = "warning" if is_warning else "error"

    # Extract error code
    code_match = re.search(r"(?:error|warning)\[([^\]]+)\]", block)
    if not code_match and is_warning:
        code_match = re.search(r"#\[warn\(([^\]]+)\)\]", block)
    code = code_match.group(1) if code_match else "unknown"

    # Extract location
    location_match = re.search(r"-->\s*([^\n]+)", block)
    location = location_match.group(1).strip() if location_match else ""

    # Extract main message (first line)
    first_line = block.split("\n")[0]
    message_match = re.search(
        r"(?:error|warning)(?:\[[^\]]+\])?\s*:\s*(.+)", first_line
    )
    message = message_match.group(1).strip() if message_match else ""

    error = RustError(
        error_type=error_type,
        code=code,
        message=message,
        location=location,
        full_text=block.strip(),
    )
    error.feature_flags.add(features) # pyright: ignore[reportArgumentType]
    return error


def deduplicate_and_sort_errors(
    error_dict: Dict[RustError, List[RustError]],
) -> List[tuple]:
    """
    Deduplicate errors and sort them intelligently.
    Returns list of (error, feature_sets) tuples.
    """
    # Merge duplicate errors and collect all feature flags
    unique_errors = {}

    for base_error, occurrences in error_dict.items():
        key = (
            base_error.error_type,
            base_error.code,
            base_error.message,
            base_error.location,
        )

        if key not in unique_errors:
            unique_errors[key] = base_error

        # Merge all feature flags
        for occurrence in occurrences:
            unique_errors[key].feature_flags.update(occurrence.feature_flags)

    # Sort errors: errors before warnings, then by code, then by location
    def sort_key(item):
        error = item[1]
        return (
            0 if error.error_type == "error" else 1,  # Errors first
            error.code,
            error.location,
        )

    sorted_errors = sorted(unique_errors.items(), key=sort_key)

    return [(key, error) for key, error in sorted_errors]


def format_output(sorted_errors: List[tuple]) -> str:
    """Format the deduplicated errors into a readable string."""
    output = []

    # Summary statistics
    total_errors = sum(1 for _, e in sorted_errors if e.error_type == "error")
    total_warnings = sum(1 for _, e in sorted_errors if e.error_type == "warning")

    output.append("=" * 80)
    output.append(
        f"SUMMARY: {total_errors} unique errors, {total_warnings} unique warnings"
    )
    output.append("=" * 80)
    output.append("")

    for key, error in sorted_errors:
        # Header with feature flags
        flags_list = sorted(error.feature_flags, key=lambda x: (len(x), x))

        # Format flags more compactly
        if len(flags_list) > 10:
            # Show count and sample
            flags_str = f"  {len(flags_list)} feature combinations (showing first 10):"
            flags_str += "\n" + "\n".join(
                f"    - {flag if flag else '(no features)'}" for flag in flags_list[:10]
            )
        else:
            flags_str = "\n".join(
                f"  - {flag if flag else '(no features)'}" for flag in flags_list
            )

        output.append("=" * 80)
        output.append(f"[{error.error_type.upper()}] {error.code}")
        output.append(f"Feature combinations: {len(flags_list)}")
        output.append(flags_str)
        output.append("-" * 80)
        output.append(f"Message: {error.message}")
        if error.location:
            output.append(f"Location: {error.location}")
        output.append("")
        output.append("Full error:")
        output.append(error.full_text)
        output.append("")

    return "\n".join(output)


def parse_and_deduplicate_rust_errors(file_content: str) -> str:
    """
    Main function to parse, deduplicate, and sort Rust compilation errors.

    Args:
        file_content: String content of the error log file

    Returns:
        Formatted string with deduplicated errors grouped by feature flags
    """
    error_dict = parse_rust_errors(file_content)
    sorted_errors = deduplicate_and_sort_errors(error_dict) # pyright: ignore[reportArgumentType]
    return format_output(sorted_errors)


console = Console()


def check_feature_combo(combo, idx, total):
    """Check a single feature combination."""
    cmd = [
        "cargo",
        "+nightly",
        "check",
        "-Zfeatures=itarget",
        "--lib",
        "--no-default-features",
        "-q",  # Quieter output for speed
    ]
    for feature in combo:
        cmd.extend(["--features", feature])

    start = time.time()
    try:
        result = subprocess.run(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            encoding="utf-8",
            errors="replace",
            timeout=300,
        )
        duration = time.time() - start

        return {
            "combo": combo,
            "idx": idx,
            "returncode": result.returncode,
            "stderr": result.stderr,
            "duration": duration,
            "success": True,
        }
    except subprocess.TimeoutExpired:
        duration = time.time() - start
        return {
            "combo": combo,
            "idx": idx,
            "returncode": -1,
            "stderr": "Process timed out after 300 seconds",
            "duration": duration,
            "success": False,
        }
    except Exception as e:
        duration = time.time() - start
        return {
            "combo": combo,
            "idx": idx,
            "returncode": -1,
            "stderr": f"Exception: {str(e)}",
            "duration": duration,
            "success": False,
        }


def run_checks(final_combinations, total):
    """Run all feature combination checks."""
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
                executor.submit(check_feature_combo, combo, idx, total): (combo, idx)
                for idx, combo in enumerate(final_combinations, start=1)
            }

            for future in as_completed(futures):
                try:
                    result = future.result()
                    combo = result["combo"]
                    idx = result["idx"]
                    duration = result["duration"]
                    durations.append(duration)

                    avg_duration = sum(durations) / len(durations) if durations else 0

                    if result["returncode"] != 0:
                        errors.append((combo, result["stderr"]))
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

    file = "compile_error.txt"
    if errors:
        final = ""
        for combination, console_output in errors:
            final += "Combination failed: " + str(combination) + "\n"
            final += "Error:\n" + str(console_output) + "\n"
            final += "-" * 80 + "\n"
        result = parse_and_deduplicate_rust_errors(final)
        with open(file, "w", encoding="utf-8") as f:
            for combination, console_output in errors:
                f.write(result)
        console.print(
            f"\n[red]❌ {len(errors)} combination(s) failed ({len(errors)/total*100:.1f}%). "
            f"See compile_error.txt for details.[/red]"
        )
    else:
        console.print(
            "\n[bold green]✅ All combinations compiled successfully![/bold green]"
        )
        if os.path.exists(file):
            os.remove(file)

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
        "ahash": ["std"],
        "miri": [],
        "console": [],
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
    ]

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
    # total_combinations = [list(x) for x in total_combinations]
    # [x.sort() for x in total_combinations]
    # [print(list(x)) for x in total_combinations]

    import itertools

    def all_orders(lst: list[tuple[str, ...]]):
        out: list[tuple[tuple[str, ...], ...]] = []
        n = len(lst)
        for r in range(1, n + 1):
            for combo in itertools.combinations(lst, r):
                for perm in itertools.permutations(combo):
                    out.append(perm)
        return out

    final_combinations: list[list[tuple[str, ...]]] = []
    for x in total_combinations:
        comb = all_orders(x)  # type: ignore
        comb = [list(x) for x in comb]
        [x.sort() for x in comb]
        comb = [tuple(x) for x in comb]
        final_combinations.extend(comb)

    print("With duplicates:", len(final_combinations) + 2)
    final_combinations = list(set(final_combinations))
    print("Without duplicates:", len(final_combinations) + 2)
    final_combinations.append([])
    final_combinations.insert(0, ["all"])  # type: ignore

    import subprocess
    import time
    from rich.console import Console
    from rich.progress import (
        Progress,
        SpinnerColumn,
        BarColumn,
        TimeRemainingColumn,
    )

    console = Console()

    errors: list[tuple[list[tuple[str, ...]], str]] = []

    total = len(final_combinations)
    start_time = time.time()
    from concurrent.futures import ProcessPoolExecutor, as_completed
    import os
    from rich.progress import Progress, SpinnerColumn, BarColumn, TimeRemainingColumn
    from rich.console import Console

    from concurrent.futures import ThreadPoolExecutor, as_completed

    run_checks(final_combinations, total)
