import subprocess
from typing import Any, Optional
from collections import defaultdict

filter_name = "mirl"

result = subprocess.run(
    ["cargo", "public-api", "--features", "all"],
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    text=True,
    encoding="utf-8",
    errors="replace",
)

file = (result.stdout or "") + (result.stderr or "")

data = file.split("\n")

stuff = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ|_!1234567890"

paths: list[str] = []
for line in data:
    if not line.startswith("pub"):
        continue

    cleaned_line = line
    while "<" in cleaned_line and ">" in cleaned_line:
        start = cleaned_line.find("<")
        end = cleaned_line.find(">", start)
        if end != -1:
            cleaned_line = cleaned_line[:start] + cleaned_line[end + 1 :]
        else:
            break

    name = ""
    potentials = cleaned_line.split(" ")
    for i in potentials:
        if not i.__contains__("::"):
            continue
        name = i.replace("::", "|")
        break

    refined = ""
    for i in name:
        if not i in stuff:
            break
        refined += i

    if refined:
        paths.append(refined.replace("|", "::"))

paths = list(set(paths))
sort_by_depth = True

depth = [x.count("::") + 1 for x in paths]
length = [len(x) - x.count("::") * 2 for x in paths]
path_data = list(zip(paths, depth, length))

if sort_by_depth:
    # Primary sort by depth, secondary sort by length
    path_data.sort(key=lambda x: (x[1], x[2], x[0]))
    sort_criteria = "depth (then length)"
else:
    # Primary sort by length, secondary sort by depth
    path_data.sort(key=lambda x: (x[2], x[1], x[0]))
    sort_criteria = "length (then depth)"

if True:
    max_path_width = max(len(path) for path, _, _ in path_data)
    max_path_width = max(max_path_width, len("Path"))

    print(f"Paths sorted by {sort_criteria}:")
    print("-" * (max_path_width + 20))
    print(f"{'Path'.ljust(max_path_width)} {'Depth'.rjust(8)} {'Length'.rjust(8)}")
    print("-" * (max_path_width + 20))

    for path, d, l in path_data:
        print(f"{path.ljust(max_path_width)} {str(d).rjust(8)} {str(l).rjust(8)}")
    quit()

from typing import Any, Optional


if False:
    import collections
    import json

    def create_dict_from_paths(paths: list[str]):
        result: collections.OrderedDict[Any, Any] = collections.OrderedDict()

        for path in paths:
            parts = path.split("::")

            current = result
            for part in parts[:-1]:
                if part not in current:
                    current[part] = {}
                current = current[part]

            current[parts[-1]] = {}

        return result

    value = create_dict_from_paths(paths)

    print(json.dumps(value, indent=4))
    quit()

from collections import defaultdict


# type: ignore, my beloved
def visualize_paths(
    paths: list[str], prefix: str = "", lines: Optional[list[str]] = None
) -> list[str]:
    if lines is None:
        lines = []

    # Build a nested dict/tree
    tree = lambda: defaultdict(tree)  # type: ignore
    root = tree()  # type: ignore
    for path in paths:
        parts = path.split("::")
        node = root  # type: ignore
        for part in parts:
            node = node[part]  # type: ignore

    def _walk(node, root, prefix=""):  # type: ignore
        keys = list(node.keys())  # type: ignore
        for i, key in enumerate(keys):  # type: ignore
            is_last = i == len(keys) - 1  # type: ignore
            if root:
                # Root node: print without └── / ├──
                lines.append(f"{key}")
                new_prefix = ""
            else:
                marker = "└──" if is_last else "├──"
                lines.append(f"{prefix}{marker} {key}")
                new_prefix = prefix + ("    " if is_last else "│   ")
            _walk(node[key], False, new_prefix)  # type: ignore

    _walk(root, True)
    return lines


full = "\n".join(visualize_paths([p for p in paths if p.startswith(filter_name)]))

with open("paths.txt", "w", encoding="utf-8") as f:
    f.write(full)
