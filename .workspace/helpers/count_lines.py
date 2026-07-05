#!/usr/bin/env python3
"""
Scans a directory for all .rs files and counts their total lines,
displaying the results as a tree sorted by line count (descending).

Usage:
    python count_rs_lines.py [directory] [options]

Options:
    --no-color          Disable colored output
    --exclude DIR       Exclude additional directory names (repeatable)
    --min-lines N       Only show files/dirs with at least N lines
    --top N             Only show the top N files per directory
"""

import os
import re
import sys
from pathlib import Path
from typing import Literal, TypedDict, TypeGuard

sys.stdout.reconfigure(  # pyright: ignore[reportUnknownMemberType, reportAttributeAccessIssue]
    encoding="utf-8"
)

# ── tunables ─────────────────────────────────────────────────────────────────
DEFAULT_EXCLUDE: set[str] = {"target", ".git", ".vscode", "build.rs"}
use_color: bool = True
USE_VIBRANT: bool = True
MINIBAR_WIDTH: int = 20


def c_rgb(r: int, g: int, b: int, text: str) -> str:
    return f"\033[38;2;{r};{g};{b}m{text}\033[0m" if use_color else text


def c(code: str, text: str) -> str:
    return f"\033[{code}m{text}\033[0m" if use_color else text


RGB = tuple[int, int, int]
PaletteEntry = tuple[str | None, RGB | None]

PALETTE_DEFAULT: dict[str, PaletteEntry] = {
    "blue_bold": ("1;34", None),
    "yellow": ("33", None),
    "dim": ("2", None),
    "dim2": ("2", None),
    "bold": ("1", None),
    "red": ("31", None),
    "cyan": ("36", None),
    "bar_code": ("36", None),
    "bar_comments": ("32", None),
    "bar_blank": ("2;90", None),
}

PALETTE_VIBRANT: dict[str, PaletteEntry] = {
    "blue_bold": (None, (0, 140, 255)),
    "yellow": (None, (255, 210, 0)),
    "dim": (None, (140, 140, 140)),
    "dim2": (None, (100, 100, 100)),
    "bold": ("1", None),
    "red": (None, (255, 60, 60)),
    "cyan": (None, (0, 255, 255)),
    "bar_code": (None, (49, 186, 224)),
    "bar_comments": (None, (18, 130, 37)),
    "bar_blank": (None, (85, 70, 85)),
}

P: dict[str, PaletteEntry] = PALETTE_VIBRANT if USE_VIBRANT else PALETTE_DEFAULT


def apply(key: str, text: str) -> str:
    code, rgb = P[key]
    if rgb:
        return c_rgb(*rgb, text)
    return c(code or "", text)


def blue_bold(t: str) -> str:
    return apply("blue_bold", t)


def yellow(t: str) -> str:
    return apply("yellow", t)


def dim(t: str) -> str:
    return apply("dim", t)


def dim2(t: str) -> str:
    return apply("dim2", t)


def bold(t: str) -> str:
    return apply("bold", t)


def red(t: str) -> str:
    return apply("red", t)


def cyan(t: str) -> str:
    return apply("cyan", t)


def mini_bar(code: int, comments: int, blank: int, width: int = MINIBAR_WIDTH) -> str:
    total = code + comments + blank
    if total == 0:
        return apply("bar_blank", "░" * width)

    c_w = round(code / total * width)
    m_w = round(comments / total * width)
    b_w = width - c_w - m_w
    if b_w < 0:
        c_w += b_w
        b_w = 0

    return (
        apply("bar_code", "█" * c_w)
        + apply("bar_comments", "█" * m_w)
        + apply("bar_blank", "░" * b_w)
    )


# ── symbol counting ───────────────────────────────────────────────────────────
# Every brace scope gets tagged with a Placement when it's opened:
#   "trait"   → the body of a `trait Foo { ... }` (or `pub trait`, `const trait`)
#   "impl"    → the body of an `impl ... { ... }`
#   "closure" → any other brace scope at all: fn/method bodies, if/match/loop
#               blocks, mod bodies, plain `{ ... }` blocks, closure literals
#               (`|x| { ... }`), etc. Per spec, visibility is meaningless here.
#
# An item's own *placement* is the context of the nearest enclosing brace at
# the moment the item's keyword is seen (i.e. the top of brace_stack), not
# the context the item itself might open. "open" (top-level) is represented
# as an empty brace_stack.
#
# Kinds tracked: fn, const, type, struct, enum, trait, static, mod, impl.
# `impl` blocks have no real "visibility" (impls aren't pub/private in Rust),
# so they're counted by placement only, under a single pseudo-visibility "na".

Placement = Literal["open", "trait", "impl", "closure"]
Visibility = Literal["pub", "priv", "na"]

PLACEMENTS: tuple[Placement, ...] = ("open", "trait", "impl", "closure")
VISIBILITIES: tuple[Visibility, ...] = ("pub", "priv")

# Kinds whose visibility is tracked normally (pub/priv) when placement=open/trait/impl,
# and collapsed to "na" when placement=closure.
KIND_ORDER: list[str] = [
    "fn",
    "const",
    "type",
    "struct",
    "enum",
    "trait",
    "static",
]
# `impl` is special: no visibility concept at all, just placement.
IMPL_KIND = "impl"

# Matches the keyword for each kind, *after* any leading visibility/modifier
# tokens (pub, pub(crate), async, default, unsafe, const(for fn)) have been
# stripped from the start of the line. Order doesn't matter since each kind
# is mutually exclusive per line for our purposes (we take the first match).
_KIND_RE: dict[str, re.Pattern[str]] = {
    "fn": re.compile(r"\bfn\b"),
    "const": re.compile(r"\bconst\b"),
    "type": re.compile(r"\btype\b"),
    "struct": re.compile(r"\bstruct\b"),
    "enum": re.compile(r"\benum\b"),
    "trait": re.compile(r"\btrait\b"),
    "static": re.compile(r"\bstatic\b"),
}

# A line is only considered an "item declaration line" for kind-detection if,
# after stripping leading visibility/modifier tokens, it starts with one of
# these keywords (in this priority order — e.g. `const fn` must classify as
# fn, not const; `pub const NAME` classifies as const).
_LEADING_MODIFIERS_RE: re.Pattern[str] = re.compile(
    r"^(?:"
    + r"pub(?:\s*\([^)]*\))?\s+|"  # pub, pub(crate), pub(super), …
    + r"default\s+|"
    + r"unsafe\s+|"
    + r"async\s+|"
    + r"extern(?:\s+\"[^\"]*\")?\s+|"
    + r")*"
)

# After stripping modifiers, `const fn` must be classified as `fn` (the
# `const` here just means "callable in const context", not a const item).
_CONST_FN_RE: re.Pattern[str] = re.compile(r"^const\s+fn\b")

# Ordered (kind, regex-on-stripped-line) checks; first match wins.
_KIND_CHECKS: list[tuple[str, re.Pattern[str]]] = [
    ("fn", re.compile(r"^fn\b")),
    ("fn", _CONST_FN_RE),  # const fn → fn (checked via stripped text below)
    ("trait", re.compile(r"^(?:const\s+)?trait\b")),
    ("impl", re.compile(r"^impl\b")),
    ("const", re.compile(r"^const\b")),
    ("type", re.compile(r"^type\b")),
    ("struct", re.compile(r"^struct\b")),
    ("enum", re.compile(r"^enum\b")),
    ("static", re.compile(r"^static\b")),
]

# Patterns that determine whether an opening `{` on the *same or next line*
# should push "trait" or "impl" context onto the brace stack. Anything else
# that opens a brace pushes "closure".
_TRAIT_OPEN_RE: re.Pattern[str] = re.compile(r"\b(?:pub\s+)?(?:const\s+)?trait\b")
_IMPL_OPEN_RE: re.Pattern[str] = re.compile(r"\bimpl\b")

# Used to strip out string/char literal contents before brace counting, so
# that braces or quotes inside literals don't confuse the brace stack.
# Double-quoted strings, with backslash-escapes handled.
_STRING_LITERAL_RE: re.Pattern[str] = re.compile(r'"(?:\\.|[^"\\])*"')
# Char literals: either an escaped sequence ('\n', '\'', '\\', '\u{1F600}')
# or a single plain character ('a', '{', etc.), always closed by a quote.
# This intentionally does NOT match a lone `'` that isn't closed on the same
# line — which is exactly what Rust lifetime syntax looks like ('a, 'static,
# '_, &'a self, <'a, T>) — so lifetimes are left untouched and don't get
# mistaken for the start of a literal.
_CHAR_LITERAL_RE: re.Pattern[str] = re.compile(r"'(?:\\.|[^'\\])'")


def _nearest_context(stack: list[Placement]) -> Placement:
    """Return the innermost brace context, or 'open' if the stack is empty."""
    return stack[-1] if stack else "open"


def _empty_symbol_counts() -> dict[str, dict[str, dict[str, int]]]:
    """counts[kind][placement][visibility] = n"""
    counts: dict[str, dict[str, dict[str, int]]] = {}
    for kind in KIND_ORDER:
        counts[kind] = {p: {"pub": 0, "priv": 0, "na": 0} for p in PLACEMENTS}
    counts[IMPL_KIND] = {p: {"na": 0} for p in PLACEMENTS}
    return counts


def _classify_kind(stripped_line: str) -> str | None:
    """Return the item kind for a (modifier-stripped) line, or None."""
    for kind, pat in _KIND_CHECKS:
        if pat.match(stripped_line):
            return kind
    return None


def count_symbols(filepath: Path) -> dict[str, dict[str, dict[str, int]]]:
    """
    Count Rust symbols with kind + visibility + placement awareness.

    Every item (fn, const, type, struct, enum, trait, static, mod, impl) is
    classified by:
      - kind:       what it is
      - visibility: pub | priv (meaningless / "na" for impl, and for
                    anything whose placement is "closure")
      - placement:  open | trait | impl | closure — the nearest enclosing
                    brace context at the point the item's keyword appears

    The scanner maintains a context stack mirroring brace nesting. Each
    pushed brace is tagged "trait", "impl", or "closure" depending on what
    keyword (if any) immediately preceded it.
    """
    counts: dict[str, dict[str, dict[str, int]]] = _empty_symbol_counts()

    brace_stack: list[Placement] = []

    # When a trait/impl keyword is seen we set this so that the *next* `{`
    # (which may be on the same or a following line, even after a multi-line
    # `where` clause with several bound lines) gets the right context pushed.
    # Held indefinitely until either the `{` is found or it's unambiguously
    # superseded (e.g. a `;` closes the item without a body, or another
    # trait/impl keyword is seen first). Any `{` that fires with no pending
    # trait/impl context pushes "closure" instead.
    pending_context: Literal["trait", "impl"] | None = None

    try:
        with open(filepath, "r", encoding="utf-8", errors="ignore") as f:
            in_block_comment: bool = False

            for raw in f:
                line: str = raw.strip()

                # ── comment handling ──────────────────────────────────────
                if in_block_comment:
                    if "*/" in line:
                        in_block_comment = False
                    continue
                if not line or line.startswith("//") or line.startswith("*"):
                    continue
                if "/*" in line:
                    in_block_comment = True
                    # fall through: line may still contain real tokens before /*

                # ── symbol detection (before brace accounting) ────────────
                # Placement is the context *before* this line's braces are
                # processed (the keyword precedes its own `{`).
                placement: Placement = _nearest_context(brace_stack)

                modifiers_match = _LEADING_MODIFIERS_RE.match(line)
                leading = modifiers_match.group(0) if modifiers_match else ""
                stripped_for_kind = line[len(leading) :]
                is_pub = leading.lstrip().startswith("pub")

                kind = _classify_kind(stripped_for_kind)
                if kind is not None:
                    if kind == IMPL_KIND:
                        counts[IMPL_KIND][placement]["na"] += 1
                    else:
                        if placement == "closure":
                            # Per spec: visibility is meaningless once an
                            # item is local to a fn/block/closure body.
                            vis: str = "na"
                        elif placement == "trait":
                            # `pub` cannot legally appear inside a trait
                            # body — visibility there is inherited from the
                            # trait itself, so "priv" would be misleading.
                            vis = "na"
                        else:
                            vis = "pub" if is_pub else "priv"
                        counts[kind][placement][vis] += 1

                # ── set/refresh pending context for upcoming `{` ─────────
                # A new trait/impl keyword always (re)establishes the
                # pending context, even if one was already pending (covers
                # back-to-back items like `trait Foo {} impl Foo for Bar {`
                # on separate lines, and nested trait-in-impl edge cases).
                if _TRAIT_OPEN_RE.search(line):
                    pending_context = "trait"
                elif _IMPL_OPEN_RE.search(line):
                    pending_context = "impl"

                # ── brace accounting ──────────────────────────────────────
                # Strip string and char literals before counting braces, so
                # that quote/brace characters *inside* literals don't fool
                # the brace stack. A regex-based strip is far more robust
                # here than a manual character walk: Rust char literals can
                # contain escaped backslashes and escaped quotes ('\\', '\''),
                # and lifetimes ('a, 'static, '_, 'a in &'a self) use a
                # single, never-closed quote — a manual walker has to special
                # -case every one of these and is easy to get subtly wrong.
                # The regex approach handles all of them in one pass:
                #   - "..." / "...\""...  → double-quoted strings (with escapes)
                #   - '\\' / '\'' / 'x'   → char literals (escaped or plain)
                #   - 'a / 'static / '_   → left alone (not matched, since
                #     lifetimes aren't closed by a second quote)
                stripped: str = _STRING_LITERAL_RE.sub('""', line)
                stripped = _CHAR_LITERAL_RE.sub("' '", stripped)

                # A `;` can appear *inside* a brace-opening line without
                # meaning "no body follows" — e.g. fixed-size array types
                # like `[T; N]` in `impl<T, const N: usize> Trait for [T; N] {`.
                # Only treat `;` as a signal that a trait/impl keyword didn't
                # open a body (e.g. a trait method declared without a
                # default impl, terminated by `;`) when this line contains
                # no `{` at all — if a `{` does appear later on the line,
                # that brace is the real signal and the `;` before it was
                # just part of an expression/type, not a statement end.
                line_has_brace: bool = "{" in stripped

                ch: str
                for ch in stripped:
                    if ch == "{":
                        if pending_context is not None:
                            brace_stack.append(pending_context)
                        else:
                            brace_stack.append("closure")
                        pending_context = None
                    elif ch == "}":
                        if brace_stack:
                            _ = brace_stack.pop()
                    elif (
                        ch == ";" and pending_context is not None and not line_has_brace
                    ):
                        # A `;` before any `{` (and none later on this line
                        # either) means the trait/impl keyword we saw didn't
                        # open a body on this statement after all (e.g. a
                        # trait method declaration with no default body
                        # terminated by `;`, or a macro/attribute false
                        # positive). Drop the stale pending context so it
                        # doesn't leak into some unrelated later brace.
                        pending_context = None

    except OSError:
        pass
    return counts


def merge_symbols(
    a: dict[str, dict[str, dict[str, int]]], b: dict[str, dict[str, dict[str, int]]]
) -> dict[str, dict[str, dict[str, int]]]:
    out: dict[str, dict[str, dict[str, int]]] = {}
    for kind in a:
        out[kind] = {}
        for placement in a[kind]:
            out[kind][placement] = {
                vis: a[kind][placement].get(vis, 0) + b[kind][placement].get(vis, 0)
                for vis in a[kind][placement]
            }
    return out


def symbol_total(sym: dict[str, dict[str, dict[str, int]]]) -> int:
    return sum(
        n
        for kind in sym.values()
        for placement in kind.values()
        for n in placement.values()
    )


# Short placement tags used in the compact inline summary.
_PLACEMENT_TAG: dict[Placement, str] = {
    "open": "",
    "trait": "trait",
    "impl": "impl",
    "closure": "closure",
}


def fmt_symbols(sym: dict[str, dict[str, dict[str, int]]]) -> str:
    """
    Return a compact human-readable symbol summary, omitting zeros.

    Format per entry: "<vis><placement> <kind> <n>", e.g.:
      "pub fn 12"  "priv fn 4"  "trait fn 6"  "impl fn 9"  "closure fn 3"
      "impl 5" (impl blocks themselves, placement=open omitted)
      "trait impl 1" (an impl block written inside a trait body — rare)
    """
    parts: list[str] = []

    for kind in [*KIND_ORDER, IMPL_KIND]:
        per_placement = sym.get(kind, {})
        for placement in PLACEMENTS:
            vis_counts = per_placement.get(placement, {})
            tag = _PLACEMENT_TAG[placement]

            if kind == IMPL_KIND:
                n = vis_counts.get("na", 0)
                if n == 0:
                    continue
                label = f"{tag} impl" if tag else "impl"
                parts.append(f"{label} {n}")
                continue

            if placement in ("closure", "trait"):
                n = vis_counts.get("na", 0)
                if n == 0:
                    continue
                label = f"{tag} {kind}" if tag else kind
                parts.append(f"{label} {n}")
                continue

            for vis in ("pub", "priv"):
                n = vis_counts.get(vis, 0)
                if n == 0:
                    continue
                vis_label = "pub" if vis == "pub" else "priv"
                label = f"{vis_label} {tag} {kind}" if tag else f"{vis_label} {kind}"
                parts.append(f"{label} {n}")

    return "  ".join(parts)


# ── line counting ─────────────────────────────────────────────────────────────
# (total_lines, code_lines, comment_lines, blank_lines)
LineCounts = tuple[int, int, int, int]


def count_lines(filepath: Path) -> LineCounts:
    """Return (total, code, comments, blank)."""
    total: int = 0
    code: int = 0
    comments: int = 0
    blank: int = 0
    try:
        with open(filepath, "r", encoding="utf-8", errors="ignore") as f:
            in_block: bool = False
            for raw in f:
                total += 1
                line: str = raw.strip()
                if not line:
                    blank += 1
                    continue
                if in_block:
                    comments += 1
                    if "*/" in line:
                        in_block = False
                elif line.startswith("//") or line.startswith("#!"):
                    comments += 1
                elif "/*" in line:
                    in_block = True
                    comments += 1
                else:
                    code += 1
    except OSError:
        pass
    return total, code, comments, blank


# ── tree builder ──────────────────────────────────────────────────────────────
class FileEntry(TypedDict):
    """One scanned .rs file and its line/symbol counts."""

    name: str
    lines: int
    code_lines: int
    comment_lines: int
    blank_lines: int
    symbols: dict[str, dict[str, dict[str, int]]]


class DirEntry(TypedDict):
    """A directory node in the scan tree, holding files and child directories."""

    name: str
    files: list[FileEntry]
    subdirs: list["DirEntry"]
    total_lines: int
    code_lines: int
    comment_lines: int
    blank_lines: int
    symbols: dict[str, dict[str, dict[str, int]]]


def build_tree(root: Path, exclude: set[str], min_lines: int = 0) -> DirEntry:
    tree: DirEntry = {
        "name": root.name,
        "files": [],
        "subdirs": [],
        "total_lines": 0,
        "code_lines": 0,
        "comment_lines": 0,
        "blank_lines": 0,
        "symbols": _empty_symbol_counts(),
    }

    try:
        entries: list[Path] = sorted(root.iterdir(), key=lambda e: e.name.lower())
    except PermissionError:
        return tree

    for entry in entries:
        if entry.name in exclude or entry.name.startswith("."):
            continue
        if entry.is_dir():
            sub: DirEntry = build_tree(entry, exclude, min_lines)
            if sub["total_lines"] > 0:
                tree["subdirs"].append(sub)
                tree["total_lines"] += sub["total_lines"]
                tree["code_lines"] += sub["code_lines"]
                tree["comment_lines"] += sub["comment_lines"]
                tree["blank_lines"] += sub["blank_lines"]
                tree["symbols"] = merge_symbols(tree["symbols"], sub["symbols"])
        elif entry.is_file() and entry.suffix == ".rs":
            total, code, comm, blank = count_lines(entry)
            if total < min_lines:
                continue
            sym: dict[str, dict[str, dict[str, int]]] = count_symbols(entry)
            file_entry: FileEntry = {
                "name": entry.name,
                "lines": total,
                "code_lines": code,
                "comment_lines": comm,
                "blank_lines": blank,
                "symbols": sym,
            }
            tree["files"].append(file_entry)
            tree["total_lines"] += total
            tree["code_lines"] += code
            tree["comment_lines"] += comm
            tree["blank_lines"] += blank
            tree["symbols"] = merge_symbols(tree["symbols"], sym)

    tree["subdirs"].sort(key=lambda d: d["total_lines"], reverse=True)
    tree["files"].sort(key=lambda f: f["lines"], reverse=True)
    return tree


# A node in the mixed subdirs+files listing rendered by print_tree.
TreeItem = DirEntry | FileEntry


def _is_dir_entry(item: TreeItem) -> "TypeGuard[DirEntry]":
    """True if item is a DirEntry (has a 'subdirs' key), narrowing its type."""
    return "subdirs" in item


# ── printer ───────────────────────────────────────────────────────────────────
def print_tree(
    tree: DirEntry,
    grand_total: int,
    prefix: str = "",
    is_last: bool = True,
    is_root: bool = True,
    top_n: int | None = None,
) -> None:
    total: int = tree["total_lines"]
    pct: float = total / grand_total * 100 if grand_total else 0
    m_bar: str = mini_bar(
        tree["code_lines"], tree["comment_lines"], tree["blank_lines"]
    )
    sym_str: str = fmt_symbols(tree["symbols"])

    if is_root:
        print(f"{blue_bold(tree['name'])}  {dim(f'({total:,} lines)')}  {m_bar}")
        if sym_str:
            print(f"  {dim2(sym_str)}")
    else:
        conn = "└── " if is_last else "├── "
        pct_str = dim2(f"{pct:.1f}%")
        print(
            f"{prefix}{conn}{blue_bold(tree['name'] + '/')}  "
            + f"{dim(f'({total:,} lines)')}  {m_bar}  {pct_str}"
        )
        if sym_str:
            child_indent = prefix + ("    " if is_last else "│   ")
            print(f"{child_indent}  {dim2(sym_str)}")

    child_prefix: str = prefix + ("    " if is_last else "│   ")

    def _sort_key(x: TreeItem) -> int:
        return (
            x["total_lines"]
            if _is_dir_entry(x)
            else x[
                "lines"
            ]  # pyright: ignore[reportUnknownVariableType, reportGeneralTypeIssues]
        )

    all_items: list[TreeItem] = sorted(
        [*tree["subdirs"], *tree["files"]],
        key=_sort_key,
        reverse=True,
    )

    if top_n is not None:
        dirs: list[TreeItem] = [i for i in all_items if _is_dir_entry(i)]
        files: list[TreeItem] = [i for i in all_items if not _is_dir_entry(i)][:top_n]
        all_items = sorted(
            [*dirs, *files],
            key=_sort_key,
            reverse=True,
        )

    n: int = len(all_items)
    for i, item in enumerate(all_items):
        item_last: bool = i == n - 1
        if _is_dir_entry(item):
            print_tree(item, grand_total, child_prefix, item_last, False, top_n)
        else:
            conn = "└── " if item_last else "├── "
            lines: int = (
                # pyright: ignore[reportUnknownVariableType, reportGeneralTypeIssues]
                item["lines"]
            )
            f_pct: float = (  # pyright: ignore[reportUnknownVariableType]
                lines / grand_total * 100 if grand_total else 0
            )
            f_bar: str = mini_bar(
                item["code_lines"], item["comment_lines"], item["blank_lines"]
            )
            cp: float = (  # pyright: ignore[reportUnknownVariableType]
                item["code_lines"] / lines * 100 if lines else 0
            )
            mp: float = (  # pyright: ignore[reportUnknownVariableType]
                item["comment_lines"] / lines * 100 if lines else 0
            )
            bp: float = (  # pyright: ignore[reportUnknownVariableType]
                item["blank_lines"] / lines * 100 if lines else 0
            )
            badge: str = dim(f"[code {cp:.0f}%  comm {mp:.0f}%  blank {bp:.0f}%]")
            f_sym_str: str = fmt_symbols(item["symbols"])
            sym_suffix: str = f"  {dim2(f_sym_str)}" if f_sym_str else ""
            print(
                f"{child_prefix}{conn}{yellow(item['name'])}  "
                + f"{dim(f'({lines:,} lines)')}  {f_bar}  {dim(f'{f_pct:.1f}%')}  {badge}"
                + f"{sym_suffix}"
            )


# ── summary ───────────────────────────────────────────────────────────────────
def print_summary(tree: DirEntry, file_count: int) -> None:
    total: int = tree["total_lines"]
    code: int = tree["code_lines"]
    comm: int = tree["comment_lines"]
    blnk: int = tree["blank_lines"]
    sym: dict[str, dict[str, dict[str, int]]] = tree["symbols"]

    def pct(v: int) -> str:
        return dim(f"{v/total*100:.1f}%") if total else ""

    print()
    print(bold("─" * 60))
    print(bold("  Summary"))
    print(bold("─" * 60))
    print(f"  {'Files scanned':<22} {file_count:>8,}")
    print(f"  {'Total lines':<22} {(total-blnk):>8,} ({total:,})")
    print(f"  {'  ↳ Code':<22} {code:>8,}  {pct(code)}")
    print(f"  {'  ↳ Comments':<22} {comm:>8,}  {pct(comm)}")
    print(f"  {'  ↳ Blank':<22} {blnk:>8,}  {pct(blnk)}")
    print(bold("─" * 60))
    print(bold("  Symbols (whole tree)"))
    print(bold("─" * 60))

    for kind in KIND_ORDER:
        per_placement = sym.get(kind, {})
        kind_total = sum(
            v for placement in per_placement.values() for v in placement.values()
        )
        if kind_total == 0:
            continue
        print(f"  {bold(kind):<10} {kind_total:>6,}")
        for placement in PLACEMENTS:
            vis_counts = per_placement.get(placement, {})
            placement_label = "open" if placement == "open" else placement
            if placement in ("closure", "trait"):
                n = vis_counts.get("na", 0)
                if n == 0:
                    continue
                print(f"      ↳ {placement_label:<10} {n:>6,}")
            else:
                pub_n = vis_counts.get("pub", 0)
                priv_n = vis_counts.get("priv", 0)
                if pub_n == 0 and priv_n == 0:
                    continue
                detail = "  ".join(
                    s
                    for s in (
                        f"pub {pub_n:,}" if pub_n else "",
                        f"priv {priv_n:,}" if priv_n else "",
                    )
                    if s
                )
                print(f"      ↳ {placement_label:<10} {detail}")

    impl_per_placement = sym.get(IMPL_KIND, {})
    impl_total = sum(v.get("na", 0) for v in impl_per_placement.values())
    if impl_total:
        print(f"  {bold('impl'):<10} {impl_total:>6,}")
        for placement in PLACEMENTS:
            n = impl_per_placement.get(placement, {}).get("na", 0)
            if n == 0:
                continue
            print(f"      ↳ {placement:<10} {n:>6,}")

    print(bold("─" * 60))


# ── arg parsing ───────────────────────────────────────────────────────────────
# (directory, exclude_set, min_lines, top_n)
ParsedArgs = tuple[Path, set[str], int, int | None]


def parse_args() -> ParsedArgs:
    global use_color
    args: list[str] = sys.argv[1:]
    directory: Path = Path(".")
    exclude: set[str] = set(DEFAULT_EXCLUDE)
    min_lines: int = 0
    top_n: int | None = None

    i: int = 0
    while i < len(args):
        a: str = args[i]
        if a == "--no-color":
            use_color = False
        elif a == "--exclude" and i + 1 < len(args):
            i += 1
            exclude.add(args[i])
        elif a == "--min-lines" and i + 1 < len(args):
            i += 1
            try:
                min_lines = int(args[i])
            except ValueError:
                print(f"Invalid --min-lines: {args[i]}", file=sys.stderr)
                sys.exit(1)
        elif a == "--top" and i + 1 < len(args):
            i += 1
            try:
                top_n = int(args[i])
            except ValueError:
                print(f"Invalid --top: {args[i]}", file=sys.stderr)
                sys.exit(1)
        elif not a.startswith("--"):
            directory = Path(a)
        i += 1

    return directory, exclude, min_lines, top_n


# ── main ──────────────────────────────────────────────────────────────────────
def main() -> None:
    target, exclude, min_lines, top_n = parse_args()

    if not target.exists():
        print(red(f"Error: '{target}' does not exist."), file=sys.stderr)
        sys.exit(1)
    if not target.is_dir():
        print(red(f"Error: '{target}' is not a directory."), file=sys.stderr)
        sys.exit(1)

    print(f"\nScanning {bold(str(target.resolve()))} for .rs files…")
    print(dim(f"Excluding: {', '.join(sorted(exclude))}") + "\n")

    tree: DirEntry = build_tree(target, exclude, min_lines)

    if tree["total_lines"] == 0:
        print("No .rs files found.")
        return

    print_tree(tree, tree["total_lines"], top_n=top_n)

    file_count: int = 0
    for _dirpath, dirnames, filenames in os.walk(target):
        dirnames[:] = [
            d for d in dirnames if d not in exclude and not d.startswith(".")
        ]
        file_count += sum(1 for f in filenames if f.endswith(".rs"))

    print_summary(tree, file_count)


if __name__ == "__main__":
    main()
