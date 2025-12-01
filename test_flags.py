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
    "num_traits": ["std"],
    "ahash": ["std"],
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
]


def remove_items(list: list[str], items: list[str]):
    values = [flags[x] for x in items]
    for x in items:
        if x in list:
            list.remove(x)
    for v in values:
        remove_items(list, v)


unique_sub_lists = {}

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

total_combinations = []

total = 0
for sub_list_tuple, keys in unique_sub_lists.items():
    val = partial_permutation_count(len(sub_list_tuple))
    count = len(keys)
    total += val * count
    total_combinations.append(sub_list_tuple)
    print(
        f"Keys {keys} -> {len(sub_list_tuple)} flags, {val} permutations each, {count} occurrences: {sub_list_tuple}"
    )

print("Estimated Total:", total)
# total_combinations = [list(x) for x in total_combinations]
# [x.sort() for x in total_combinations]
# [print(list(x)) for x in total_combinations]


import itertools


def all_orders(lst):
    out = []
    n = len(lst)
    for r in range(1, n + 1):
        for combo in itertools.combinations(lst, r):
            for perm in itertools.permutations(combo):
                out.append(perm)
    return out


final_combinations: list[list[str]] = []
for x in total_combinations:
    comb = all_orders(x)
    comb = [list(x) for x in comb]
    [x.sort() for x in comb]
    comb = [tuple(x) for x in comb]
    final_combinations.extend(comb)


print("With duplicates:", len(final_combinations))
final_combinations = list(set(final_combinations))
print("Without duplicates:", len(final_combinations))
final_combinations.append([])

import subprocess
import time
from itertools import combinations
from rich.console import Console
from rich.progress import (
    Progress,
    SpinnerColumn,
    BarColumn,
    TimeRemainingColumn,
)

console = Console()

errors = []

total = len(final_combinations)
start_time = time.time()

with Progress(
    SpinnerColumn(),
    "[progress.description]{task.description}",
    BarColumn(),
    "[progress.percentage]{task.percentage:>3.0f}%",
    TimeRemainingColumn(),
    console=console,
) as progress:
    task = progress.add_task("Building combinations...", total=total)

    for idx, combo in enumerate(final_combinations, start=1):
        console.print(
            f"\n[cyan]Trying combination {idx}/{total}:[/cyan] [bold]{', '.join(combo)}[/bold]"
        )

        cmd = ["cargo", "check", "--no-default-features"]
        for feature in combo:
            cmd.extend(["--features", feature])

        result = subprocess.run(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            encoding="utf-8",
            errors="replace",
        )

        if result.returncode != 0:
            errors.append((combo, result.stderr))
            console.print(f"[red]Check failed for combination:[/red] {combo}")

        progress.update(task, advance=1)

end_time = time.time()
elapsed = end_time - start_time

console.print(f"\n[green]Check finished in {elapsed:.2f} seconds[/green]")

import os

file = "compile_error.txt"
if errors:
    with open(file, "w", encoding="utf-8") as f:
        for combination, console_output in errors:
            print("Combination failed:", combination, file=f)
            print("Error:\n%s" % console_output, file=f)
    console.print(
        f"[red]{len(errors)} combination(s) failed. See compile_error.txt for details.[/red]"
    )
else:
    console.print("[bold green]Compiled without errors![/bold green]")
    if os.path.exists(file):
        os.remove(file)
