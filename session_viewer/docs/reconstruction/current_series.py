#!/usr/bin/env python3
"""Pin integrated patches and the current workspace completion as eleven checkpoints.

Maintainer tool only. Numbered checkpoints and extension patches are inputs, never rewritten.
The final current checkpoint uses patches/current-11.patch, with workspace-relative paths.
Run after docs/serve.sh build has populated the checkpoint-21 cache.
"""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import shutil
import tempfile

HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("course_pages", HERE.parent / "course_pages.py")
COURSE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(COURSE)
REPLAY = COURSE.REPLAY


def inventory(root):
    base = REPLAY.read_json(HERE / "series.json")["steps"][-1]
    source = HERE.parent.parent / "target/docs/course-cache/snapshots" / base["id"]
    REPLAY.check_files(source, base["files"])
    workspace = root / "workspace"
    shutil.copytree(source, workspace)
    REPLAY.run(["git", "init", "--quiet"], workspace, dict(os.environ), root / "init.log")
    lesson = next(item for item in REPLAY.read_json(HERE.parent / "extensions/lessons.json")["lessons"]
                  if item["id"] == "integrated")
    files = dict(base["files"])
    records = []
    for index, item in enumerate(lesson["steps"], 1):
        if item.get("kernel_patch"):
            raise ValueError("a kernel patch needs its own explicit checkpoint directory")
        patch = HERE.parent / "extensions" / item["patch"]
        directory = "session_viewer"
        if index == 11:
            patch = HERE / "patches/current-11.patch"
            directory = ""
        changes = COURSE.parse_patch(patch.read_text(), directory)
        REPLAY.run(["git", "apply", *([f"--directory={directory}"] if directory else []), str(patch)], workspace,
                   dict(os.environ), root / f"current-{index}.log")
        for name, change in changes.items():
            if change.kind == "delete":
                files.pop(name, None)
            else:
                files[name] = REPLAY.digest(workspace / name)
        records.append({"id": f"current-{index}", "title": ("Finish the command workspace and soft ambient lighting" if index == 11 else item["title"]),
                        "patch": ("patches/current-11.patch" if index == 11 else "../extensions/" + item["patch"]),
                        "directory": directory, "patch_sha256": REPLAY.digest(patch),
                        "assets": [], "files": dict(sorted(files.items()))})
    return {"format": 1, "base": base["id"], "base_files": base["files"], "steps": records}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    scratch = HERE.parent.parent / "target/docs"
    with tempfile.TemporaryDirectory(prefix="current-series-", dir=scratch) as temporary:
        result = inventory(Path(temporary))
    target = HERE / "current-series.json"
    if args.check:
        if not target.is_file() or REPLAY.read_json(target) != result:
            raise SystemExit("current checkpoint inventory is stale")
        print("PASS current checkpoint patches and inventories")
    else:
        target.write_text(json.dumps(result, indent=2) + "\n")
        print(f"Recorded {len(result['steps'])} current checkpoints")


if __name__ == "__main__":
    main()
