#!/usr/bin/env python3
"""
Generate a NumbersN trait (Rust) that extends the previous NumbersM trait
in the mirl_extensions_core numeric-literal pattern, and copy the result
to the clipboard.

Usage:
    python3 gen_numbers_trait.py <low> <high> [--out FILE]

Example (256..511 -> 512, extending Numbers256):
    python3 gen_numbers_trait.py 256 511 --out numbers512.rs

The trait name is always "Numbers{high+1}" (e.g. high=511 -> Numbers512),
and it's assumed to extend "Numbers{low}" (e.g. low=256 -> Numbers256).
This matches the existing progression: Numbers128 -> Numbers256 -> Numbers512 -> Numbers1024 ...
"""

from typing import Any


import argparse
import subprocess
import sys


def copy_to_clipboard(text: str) -> bool:
    """Try a series of clipboard backends. Returns True if one worked."""
    # 1) pyperclip, if installed
    try:
        import pyperclip  # type: ignore

        pyperclip.copy(text)
        return True
    except Exception:
        pass

    # 2) platform-specific CLI tools
    candidates = [
        ["pbcopy"],  # macOS
        ["xclip", "-selection", "clipboard"],  # Linux (X11)
        ["xsel", "--clipboard", "--input"],  # Linux (X11, alt)
        ["wl-copy"],  # Linux (Wayland)
        ["clip.exe"],  # WSL / Windows
    ]
    for cmd in candidates:
        try:
            proc = subprocess.run(cmd, input=text.encode("utf-8"), check=True)
            if proc.returncode == 0:
                return True
        except (FileNotFoundError, subprocess.CalledProcessError):
            continue

    return False


def generate(low: int, high: int) -> str:
    trait_name = f"Numbers{high + 1}"
    prev_trait = f"Numbers{low}"
    supports_trait = f"SupportsRange{high + 1}"
    seed = low - 1  # last number belonging to the previous trait

    lines: list[str] = []
    lines.append(f"use crate::Numbers128;")
    lines.append(f"use crate::{{{prev_trait}}};")
    lines.append(f"use mirl_extensions_core::{{One, {supports_trait}, Zero}};")
    lines.append("")
    lines.append(f"/// Get any number between {low} and {high}")
    lines.append("///")
    lines.append(
        f"/// An extended version of the [`{prev_trait}`] trait covering all numbers from {low} to {high}"
    )
    lines.append("#[allow(missing_docs)]")
    lines.append(f"pub const trait {trait_name}: {supports_trait} + {prev_trait} {{")
    for i in range(low, high + 1):
        lines.append(f"    fn num_{i}() -> Self;")
    lines.append("}")
    lines.append(
        f"const impl<T: {supports_trait} + [const] One + [const] Zero + [const] core::ops::Add<Output = T> + [const] {prev_trait} + [const] Numbers128> {trait_name}"
    )
    lines.append("    for T")
    lines.append("{")
    for i in range(low, high + 1):
        prev = seed if i == low else i - 1
        lines.append(f"    fn num_{i}() -> Self {{")
        lines.append(f"        Self::num_{prev}() + Self::num_1()")
        lines.append("    }")
    lines.append("}")
    lines.append("")

    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "low", type=int, help="first number in the new range (e.g. 256)"
    )
    parser.add_argument(
        "high", type=int, help="last number in the new range (e.g. 511)"
    )
    parser.add_argument(
        "--out", type=str, default=None, help="optional file to also write to disk"
    )
    args = parser.parse_args()

    code = generate(args.low, args.high)

    if args.out:
        with open(args.out, "w") as f:
            f.write(code)
        print(f"Wrote {args.out}")

    if copy_to_clipboard(code):
        print("Copied to clipboard.")
    else:
        print(
            "Could not access a clipboard backend (pyperclip/xclip/xsel/wl-copy/pbcopy "
            "all unavailable). Install one of these, or use --out to write to a file instead.",
            file=sys.stderr,
        )

    method_count = args.high - args.low + 1
    print(f"Generated {method_count} methods ({args.low}..={args.high}).")


if __name__ == "__main__":
    main()
