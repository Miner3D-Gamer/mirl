import os
import shutil
import configparser
import subprocess

WORKSPACE_ROOT = "."
DRY_RUN = False
GITHUB_ORG = "Miner3D-Gamer"
SKIP_DIRS = {".git", "target"}


def run(cmd, cwd=None):
    if DRY_RUN:
        print("DRY:", " ".join(cmd))
        return True
    try:
        subprocess.run(cmd, cwd=cwd, check=True)
        return True
    except subprocess.CalledProcessError as e:
        print(f"FAILED: {' '.join(cmd)} -> {e}")
        return False


def norm(path):
    """Normalize to the same style git uses in .gitmodules (relative, no ./)."""
    return os.path.normpath(os.path.relpath(path, WORKSPACE_ROOT)).replace(os.sep, "/")


def load_gitmodules():
    path = os.path.join(WORKSPACE_ROOT, ".gitmodules")
    if not os.path.exists(path):
        return {}
    config = configparser.ConfigParser()
    config.read(path)
    modules = {}
    for section in config.sections():
        if "path" in config[section] and "url" in config[section]:
            modules[norm(config[section]["path"])] = config[section]["url"]
    return modules


def find_crates():
    crates = []
    for root, dirs, files in os.walk(WORKSPACE_ROOT):
        # prune dirs we should never descend into
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS and not d.startswith(".")]
        # print(root, files)
        if "Cargo.toml" in files:
            rel = norm(root)
            if rel == ".":
                continue
            crates.append(rel)
            # a crate directory shouldn't itself contain nested crates we treat
            # as separate top-level results unless you actually want that;
            # comment the next line out if you DO want nested crates detected.
            dirs[:] = []
    return crates


def is_git_submodule(path):
    """True if git already tracks this path as a submodule (not just a plain dir)."""
    result = subprocess.run(
        ["git", "config", "--file", ".gitmodules", "--get-regexp", "path"],
        cwd=WORKSPACE_ROOT,
        capture_output=True,
        text=True,
    )
    return path in result.stdout


def main():
    existing = load_gitmodules()
    found = set(find_crates())
    existing_set = set(existing.keys())

    print(f"Scanning from: {os.path.abspath(WORKSPACE_ROOT)}")
    print(f"Found {len(found)} crate(s): {sorted(found)}")
    print(
        f"Existing {len(existing_set)} submodule(s) in .gitmodules: {sorted(existing_set)}"
    )

    to_add = found - existing_set
    to_remove = existing_set - found

    if to_add:
        print("Crates to add as submodules:")
        for p in to_add:
            print("  +", p)
    if to_remove:
        print("Submodules no longer present as crates (will remove):")
        for p in to_remove:
            print("  -", p)

    if not to_add and not to_remove:
        print("Nothing to do — .gitmodules already matches workspace crates.")
        return

    # --- add missing ---
    for path in sorted(to_add):
        repo_url = f"https://github.com/{GITHUB_ORG}/{os.path.basename(path)}.git"

        # # If the directory already has files but ISN'T a submodule yet,
        # # `git submodule add` will refuse. That's a real conflict —
        # # don't silently blow the directory away. Flag it instead.
        # if os.path.isdir(path) and os.listdir(path):
        #     print(
        #         f"SKIP {path}: directory exists with content but isn't a "
        #         f"registered submodule. Resolve manually (is this meant to "
        #         f"be a plain workspace member, not a submodule?)."
        #     )
        #     continue

        run(["git", "submodule", "add", repo_url, path])

    # --- remove deleted ---
    for path in sorted(to_remove):
        ok = run(["git", "submodule", "deinit", "-f", path])
        if not ok:
            print(f"Aborting removal of {path}: deinit failed, leaving it intact.")
            continue
        # ok = run(["git", "rm", "-f", path])
        # if not ok:
        #     print(f"WARNING: git rm failed for {path}; submodule metadata may be "
        #           f"partially deinitialized. Check `git status` / .git/modules.")
        #     continue
        # if not DRY_RUN and os.path.isdir(path):
        #     shutil.rmtree(path)
        # elif DRY_RUN:
        #     print(f"DRY: shutil.rmtree({path})")


if __name__ == "__main__":
    main()
