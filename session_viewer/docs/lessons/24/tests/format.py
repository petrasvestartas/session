#!/usr/bin/env python3
"""Check viewer Rust formatting without traversing sibling Session packages."""
import argparse
from pathlib import Path
import subprocess
import sys


def viewer_files(repository):
    """Enumerate handwritten viewer source/examples as explicit rustfmt inputs."""
    files = []
    for directory in (repository / "src", repository / "examples"):
        for file in directory.rglob("*.rs"):
            if file.is_file():
                if not file.resolve().is_relative_to(repository):
                    raise ValueError(f"Rust source escapes the viewer: {file}")
                files.append(file)
    return sorted(files)


def main():
    """Check by default; --write applies the same scoped Rust 2024 formatting policy."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--write", action="store_true", help="format the listed viewer files in place")
    args = parser.parse_args()
    repository = Path(__file__).resolve().parents[1]
    files = viewer_files(repository)
    if not files:
        parser.error("no Rust files found under src/ or examples/")
    command = ["rustfmt", "--edition", "2024", "--config", "skip_children=true"]
    if not args.write:
        command.append("--check")
    command.extend(map(str, files))
    action = "Formatting" if args.write else "Checking formatting of"
    print(f"{action} {len(files)} viewer Rust files", flush=True)
    return subprocess.run(command, cwd=repository, check=False).returncode


if __name__ == "__main__":
    sys.exit(main())
