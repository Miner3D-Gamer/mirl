import os

BLACKLIST = {"__pycache__", ".git", ".idea", "target"}

EXCLUDE_HIDDEN = True

print("digraph {")

node_ids: dict[str, int] = {}
next_id = 0


def include(name: str) -> bool:
    if name in BLACKLIST:
        return False
    if EXCLUDE_HIDDEN and (name.startswith(".") or name.startswith("_")):
        return False
    return True


for root, dirs, files in os.walk("."):
    dirs[:] = sorted(d for d in dirs if include(d))

    rel = os.path.relpath(root, ".")
    if rel == ".":
        rel = os.path.basename(os.getcwd()) or "."

    node_ids[root] = next_id
    print(f'    {next_id} [label="{rel.replace('\\', '/')}" shape=box]')
    next_id += 1

for root, dirs, files in os.walk("."):
    dirs[:] = sorted(d for d in dirs if include(d))

    parent = node_ids[root]
    for d in dirs:
        child = os.path.join(root, d)
        if child in node_ids:
            print(f"    {parent} -> {node_ids[child]}")

print("}")
