#!/usr/bin/env python3
"""Remove files from the whole checkpoint series: patches, inventories, baseline and tree hash.

`refreeze.py` folds production EDITS into the final patch; a file deleted from production is
outside its reach. This drops the named files (or every file under a prefix) from every patch
section that creates or edits them, from each step's inventory, from the baseline and the
convergence inventory, and recomputes the production tree hash everywhere it is recorded. A
binary asset that only the dropped files used is dropped with them. Run the course audit after.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re

HERE = Path(__file__).resolve().parent
SECTION = re.compile(r"(?m)^(?=diff --git )")


def dump(path, data):
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n")


def matches(name, names, prefixes):
    return name in names or any(name.startswith(prefix) for prefix in prefixes)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("names", nargs="*", help="series names such as session_viewer/tests/teapot.cjs")
    parser.add_argument("--prefix", action="append", default=[], help="drop every file under this series prefix (repeatable)")
    args = parser.parse_args()
    names, prefixes = set(args.names), args.prefix
    series = json.loads((HERE / "series.json").read_text())
    dropped = set()
    for step in series["steps"]:
        path = HERE / step["patch"]
        kept = []
        for chunk in SECTION.split(path.read_text()):
            if chunk.startswith("diff --git "):
                name = chunk.splitlines()[0].split(" b/", 1)[1]
                if matches(name, names, prefixes):
                    dropped.add(name)
                    continue
            kept.append(chunk)
        path.write_text("".join(kept))
        step["patch_sha256"] = hashlib.sha256(path.read_bytes()).hexdigest()
        step["files"] = {name: checksum for name, checksum in step["files"].items()
                         if not matches(name, names, prefixes)}
        step["assets"] = [asset for asset in step.get("assets", [])
                          if not matches(asset["target"], names, prefixes)]
    baseline = json.loads((HERE / "baseline.json").read_text())
    baseline["files"] = {name: checksum for name, checksum in baseline["files"].items()
                         if not matches(name, names, prefixes)}
    encoding = "".join(name + "\0" + checksum + "\n" for name, checksum in sorted(baseline["files"].items()))
    tree = hashlib.sha256(encoding.encode()).hexdigest()
    baseline["tree_sha256"] = tree
    series["production_tree"] = tree
    dump(HERE / "baseline.json", baseline)
    dump(HERE / "series.json", series)
    for extra in ("verification.json", "convergence.json"):
        path = HERE / extra
        if not path.is_file():
            continue
        data = json.loads(path.read_text())
        if "production_tree" in data:
            data["production_tree"] = tree
        if "files" in data:
            data["files"] = {name: checksum for name, checksum in data["files"].items()
                             if not matches(name, names, prefixes)}
        dump(path, data)
    print(f"dropped {len(dropped)} files from the series; production tree {tree[:12]}")
    for name in sorted(dropped):
        print(" ", name)


if __name__ == "__main__":
    main()
