#!/usr/bin/env python3

import subprocess
from pathlib import Path

# Root directory to scan
ROOT = Path.cwd()

# Directory names to exclude
BLACKLIST = {
    "target",
}


def run_git(repo: Path, *args: str) -> subprocess.CompletedProcess:
    """Run a Git command inside the repository."""
    return subprocess.run(
        ["git", *args],
        cwd=repo,
        capture_output=True,
        text=True,
    )


def has_branches(repo: Path) -> bool:
    """Check for existing local or remote-tracking branches."""
    result = run_git(repo, "branch", "-a", "--list")

    if result.returncode != 0:
        print(f"[ERROR] Cannot inspect branches in {repo}: {result.stderr.strip()}")
        return True  # Fail safe: do not modify an uncertain repository.

    return bool(result.stdout.strip())


def initialize_repository(repo: Path) -> None:
    """Create an orphan master branch, commit, and push."""
    commands = [
        ["checkout", "--orphan", "master"],
        ["commit", "--allow-empty", "-m", "Initial empty commit"],
        ["push", "-u", "origin", "master"],
    ]

    for command in commands:
        print(f"[RUN] {repo}: git {' '.join(command)}")

        result = run_git(repo, *command)

        if result.returncode != 0:
            print(
                f"[ERROR] Command failed in {repo}\n"
                f"  git {' '.join(command)}\n"
                f"  {result.stderr.strip()}"
            )
            return

    print(f"[OK] Initialized and pushed {repo}")


def scan(root: Path) -> None:
    """Recursively find Git repositories, skipping blacklisted directories."""
    for current, dirs, _ in __import__("os").walk(root):
        dirs[:] = [directory for directory in dirs if directory not in BLACKLIST]

        current_path = Path(current)

        if ".git" not in dirs:
            continue

        repo = current_path

        # Don't descend into the Git metadata directory.
        dirs.remove(".git")

        print(f"[CHECK] {repo}")

        if has_branches(repo):
            print(f"[SKIP] Existing branch found in {repo}")
        else:
            initialize_repository(repo)


if __name__ == "__main__":
    scan(ROOT)
