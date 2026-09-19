#!/usr/bin/env python3
"""Re-pin the kernel base and the parity sources to the revisions the sibling checkouts are at.

Every archive in kernel-base.json is `git archive` of the listed paths at one revision of the
sibling repository. This rewrites each archive from the sibling's HEAD (or `--revision`), records
the revision and the archive's sha256, and leaves the rest of the record alone. Run
`series_git.py materialize` afterwards: the base commit of the series is these archives.

Usage:
    python3 pin_kernel.py                 # every source at its checkout's HEAD
    python3 pin_kernel.py --only session_rust
"""

import argparse
import hashlib
import json
from pathlib import Path
import subprocess

HERE = Path(__file__).resolve().parent
SESSION = HERE.parent.parent.parent


def archive(directory, revision, paths, target):
    subprocess.run(
        [
            "git",
            "-C",
            str(SESSION / directory),
            "archive",
            "--format=tar.gz",
            "-o",
            str(target),
            revision,
            *paths,
        ],
        check=True,
    )
    return hashlib.sha256(target.read_bytes()).hexdigest()


def head(directory):
    return subprocess.run(
        ["git", "-C", str(SESSION / directory), "rev-parse", "HEAD"],
        check=True,
        capture_output=True,
        text=True,
    ).stdout.strip()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--only", action="append", help="directory to re-pin (repeatable); default all"
    )
    args = parser.parse_args()
    path = HERE / "kernel-base.json"
    metadata = json.loads(path.read_text())
    sources = [dict(metadata, directory="session_rust")] + metadata.get(
        "parity_sources", []
    )

    for source in sources:
        directory = source["directory"]
        if args.only and directory not in args.only:
            continue
        revision = head(directory)
        checksum = archive(
            directory, revision, source["paths"], HERE / source["archive"]
        )
        record = metadata if directory == "session_rust" else source
        record["revision"] = revision
        record["sha256"] = checksum
        print(
            f"{directory} pinned at {revision[:12]} ({source['archive']} {checksum[:12]})"
        )

    path.write_text(json.dumps(metadata, indent=2) + "\n")


if __name__ == "__main__":
    main()
