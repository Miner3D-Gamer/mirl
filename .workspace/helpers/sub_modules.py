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
        if is_tracked_in_index(path):
            print(
                f"SKIP {path}: already tracked in git as plain files (not a submodule). "
                f"If you actually want this converted to a submodule, first run "
                f"`git rm -r --cached {path}` (this only untracks it, doesn't delete "
                f"the files), then re-run this script."
            )
            continue

        repo_url = f"https://github.com/{GITHUB_ORG}/{os.path.basename(path)}.git"
        run(["git", "submodule", "add", repo_url, path])

    for path in sorted(to_remove):
        ok = run(["git", "submodule", "deinit", "-f", path])
        if not ok:
            print(f"Aborting removal of {path}: deinit failed, leaving it intact.")
            continue


def is_tracked_in_index(path):
    """True if git already tracks files under this path (as plain files, not a submodule)."""
    result = subprocess.run(
        ["git", "ls-files", path],
        cwd=WORKSPACE_ROOT,
        capture_output=True,
        text=True,
    )
    return bool(result.stdout.strip())


if __name__ == "__main__":
    main()
