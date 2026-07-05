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
types = [
    "fn",
    "type",
    "enum",
    "struct",
    # "bytes",
    "impl",
    "trait",
    "static",
    "const",
]
types = [f" {x} " for x in types]


paths: dict[str, str] = {filter_name: "crate"}
for line in data:
    if not line.startswith("pub"):
        continue
    line_type = None
    for i in types:
        if line.__contains__(i):
            # if i == " impl ":
            #     print(line, i)
            # print(line, i)
            line_type = i[1:-1]
            break

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

    if line_type is None:
        *parent, self =refined.split("|")
        parent = "::".join(parent)
        # print(parent)
        value = paths.get(parent)
        if value is None:
            line_type = "crate"
        else:
            if value == "struct":
                line_type = "struct_field"
            elif value == "enum":
                line_type = "enum_field"
            elif value == "trait":
                print("Undefined", refined, value)
                quit()
            elif refined.__contains__("!"):
                line_type = "macro"
            elif (value == "crate" or value == "mod") and self.islower() :
                # print(parent, f"'{value}'", type(value))
                line_type = "mod"
            elif value == "enum_field":
                line_type = "enum_struct_field"
            else:
                print("Undefined", parent, f"'{value}'")
                quit()
    if refined:
        paths[refined.replace("|", "::")] = line_type


if False:
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
    paths: list[tuple[str, str]], prefix: str = "", lines: Optional[list[str]] = None
) -> list[str]:
    if lines is None:
        lines = []

    # Build a nested dict/tree with type info
    tree = lambda: defaultdict(tree)  # type: ignore
    root = tree()  # type: ignore

    for path, type in paths:
        parts = path.split("::")
        node = root  # type: ignore
        for i, part in enumerate(parts):
            node = node[part]  # type: ignore
            # Store type at the leaf node
            if i == len(parts) - 1:
                node["__type__"] = type  # type: ignore

    def _walk(node, root, prefix=""):  # type: ignore
        keys = [k for k in node.keys() if k != "__type__"]  # type: ignore
        for i, key in enumerate(keys):  # type: ignore
            is_last = i == len(keys) - 1  # type: ignore
            node_type = node[key].get("__type__", "")  # type: ignore

            if root:
                # Root node: print without └── / ├──
                lines.append(f"{key} ({node_type})")
                new_prefix = ""
            else:
                marker = "└──" if is_last else "├──"
                lines.append(f"{prefix}{marker} {key} ({node_type})")
                new_prefix = prefix + ("    " if is_last else "│   ")
            _walk(node[key], False, new_prefix)  # type: ignore

    _walk(root, True)
    return lines


# def get_type(line: str) -> str:
#     last = line.split("::")[-1]
#     if last.isupper():
#         return "Const"
#     if last.islower():
#         return "Function"
#     return "Struct/Enum/EnumVariant"
#     print(line)
#     quit()


# for i in paths:
#     t = get_type(i)
#     print(i, t)

full = "\n".join(
    visualize_paths(
        [(key, item) for key, item in paths.items() if key.startswith(filter_name)]
    )
)

with open("paths.txt", "w", encoding="utf-8") as f:
    f.write(full)
