#!/usr/bin/env python3

import argparse
import os
import subprocess
from pathlib import Path

# Root directory to scan
ROOT = Path.cwd()

# Directory names to exclude
BLACKLIST = {
    "target",
    ".git",
    "node_modules",
    ".venv",
    "venv",
    "__pycache__",
}


def run_git(repo: Path, *args: str) -> subprocess.CompletedProcess:
    """Run a Git command inside the repository."""
    return subprocess.run(
        ["git", *args],
        cwd=repo,
        capture_output=True,
        text=True,
    )


def has_branch(repo: Path, branch: str) -> bool:
    """Check whether a local branch exists."""
    result = run_git(repo, "show-ref", "--verify", "--quiet", f"refs/heads/{branch}")
    return result.returncode == 0


def current_branch(repo: Path) -> str | None:
    """Return the current branch, or None for detached HEAD."""
    result = run_git(repo, "symbolic-ref", "--quiet", "--short", "HEAD")

    if result.returncode != 0:
        return None

    return result.stdout.strip()


def rename_branch(repo: Path, dry_run: bool) -> None:
    """Rename local main to master without losing Git history."""
    main_exists = has_branch(repo, "main")
    master_exists = has_branch(repo, "master")

    if main_exists and master_exists:
        print(f"[SKIP] Both main and master exist in {repo}")
        return

    if not main_exists and not master_exists:
        print(f"[SKIP] Neither main nor master exists in {repo}")
        return

    if master_exists:
        print(f"[OK] Already has master: {repo}")
        return

    branch = current_branch(repo)
    print(f"[CHECK] Current branch: {branch or 'detached HEAD'}")

    if dry_run:
        print(f"[DRY-RUN] Would rename main to master in {repo}")
        return

    result = run_git(repo, "branch", "-m", "main", "master")

    if result.returncode != 0:
        print(
            f"[ERROR] Failed to rename branch in {repo}\n" f"  {result.stderr.strip()}"
        )
        return

    print(f"[OK] Renamed main to master in {repo}")


def scan(root: Path, dry_run: bool) -> None:
    """Recursively find Git repositories, skipping blacklisted directories."""
    for current, dirs, files in os.walk(root, topdown=True):
        dirs[:] = sorted(directory for directory in dirs if directory not in BLACKLIST)

        current_path = Path(current)
        
        # # Support normal repositories and worktrees/submodules.
        # if ".git" not in dirs and ".git" not in files:
        #     continue

        repo = current_path

        print(f"\n[CHECK] {repo}")

        # Don't descend into Git metadata.
        if ".git" in dirs:
            dirs.remove(".git")

        rename_branch(repo, dry_run)


if __name__ == "__main__":
    dry_run = False

    root = Path.cwd()

    print(f"[INFO] Root: {root}")
    print(f"[INFO] Dry run: {dry_run}")
    print(f"[INFO] Excluded: {', '.join(sorted(BLACKLIST))}")

    scan(root, dry_run)

    print("\n[DONE] Scan complete.")
