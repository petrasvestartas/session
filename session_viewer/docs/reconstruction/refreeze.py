#!/usr/bin/env python3
"""Fold production changes into the final checkpoint: patch, inventories, baseline and tree hash.

Use after editing the viewer: it diffs a clean reconstruction of the final checkpoint against the
production checkout, appends the differences to the final patch, updates the step inventory,
baseline hashes and the production tree hash everywhere they are recorded. Taught files (not
`supplied`) still need a directive in the final lesson; the audit reports them.
"""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import shutil
import subprocess
import tempfile

HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("course_pages", HERE.parent / "course_pages.py")
COURSE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(COURSE)
REPLAY = COURSE.REPLAY
CONVERGE_SPEC = importlib.util.spec_from_file_location("converge", HERE / "converge.py")
CONVERGE = importlib.util.module_from_spec(CONVERGE_SPEC)
CONVERGE_SPEC.loader.exec_module(CONVERGE)
IGNORED_DIRS = {"target", "dist", "node_modules", "__pycache__", ".git"}


def dump(path, data):
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n")


def production_files(production):
    """Every tracked-looking file of the production viewer, as `session_viewer/...` names."""
    for path in production.rglob("*"):
        if path.is_file() and not any(part in IGNORED_DIRS for part in path.relative_to(production).parts):
            yield "session_viewer/" + path.relative_to(production).as_posix()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--workspace", type=Path, required=True, help="clean reconstruction of the final checkpoint")
    parser.add_argument("--production", type=Path, default=HERE.parent.parent, help="the production session_viewer checkout")
    parser.add_argument("--note", default="", help="sentence appended to the baseline and verification scopes")
    parser.add_argument("--previous", type=Path, help="clean reconstruction of the checkpoint before the final one; "
                        "needed when a changed file already has a section in the final patch")
    args = parser.parse_args()
    workspace, production = args.workspace.resolve(), args.production.resolve()
    series = REPLAY.read_json(HERE / "series.json")
    final = series["steps"][-1]
    patch = HERE / final["patch"]
    patch_text = patch.read_text()
    sections = {}
    for chunk in re.split(r"(?m)^(?=diff --git )", patch_text):
        if chunk.startswith("diff --git "):
            sections[chunk.splitlines()[0].split(" b/", 1)[1]] = chunk
    baseline = REPLAY.read_json(HERE / "baseline.json")
    # Only the checkpoint inventory and the supplied test tooling; docs and assets stay outside.
    candidates = {name for name in final["files"] if name.startswith("session_viewer/")}
    candidates |= {name for name in baseline["files"] if name.startswith("session_viewer/")}
    candidates |= {name for name in production_files(production) if name.startswith("session_viewer/tests/")}
    changed = []
    for name in sorted(candidates):
        source = production / name[len("session_viewer/"):]
        current = workspace / name
        if not source.is_file():
            continue
        if b"\0" in source.read_bytes()[:8000] or CONVERGE.packaging_reason(name):
            continue
        if not current.is_file() or REPLAY.digest(source) != REPLAY.digest(current):
            changed.append(name)
    if not changed:
        print("nothing to fold: production matches the final checkpoint")
        return
    already = [name for name in changed if name in sections]
    if already and args.previous is None:
        raise SystemExit("these files already have a section in the final patch; pass --previous <checkpoint before final>: "
                         + ", ".join(already))
    scratch = HERE.parent.parent / "target/docs"
    scratch.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(dir=scratch) as temporary:
        repo = Path(temporary)
        subprocess.run(["git", "init", "-q"], cwd=repo, check=True)
        for name in changed:
            # A file the final patch already edits is diffed from the previous checkpoint, and its
            # old section is dropped, so the patch keeps exactly one section per file.
            current = (args.previous.resolve() / name) if name in sections else (workspace / name)
            if current.is_file():
                (repo / name).parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(current, repo / name)
        subprocess.run(["git", "add", "-A"], cwd=repo, check=True)
        subprocess.run(["git", "-c", "user.email=course@local", "-c", "user.name=course", "commit", "-q", "--allow-empty", "-m", "final"], cwd=repo, check=True)
        for name in changed:
            (repo / name).parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(production / name[len("session_viewer/"):], repo / name)
        subprocess.run(["git", "add", "-A"], cwd=repo, check=True)
        diff = subprocess.run(["git", "diff", "--cached", "--full-index", "--no-renames"], cwd=repo, check=True,
                              capture_output=True, text=True).stdout
    for name in already:
        patch_text = patch_text.replace(sections[name], "")
    patch.write_text(patch_text + diff)
    final["patch_sha256"] = hashlib.sha256(patch.read_bytes()).hexdigest()
    for name in changed:
        final["files"][name] = REPLAY.digest(production / name[len("session_viewer/"):])
        if name in baseline["files"]:
            baseline["files"][name] = final["files"][name]
    encoding = "".join(name + "\0" + checksum + "\n" for name, checksum in sorted(baseline["files"].items()))
    tree = hashlib.sha256(encoding.encode()).hexdigest()
    baseline["tree_sha256"] = tree
    if args.note:
        baseline["scope"] += " " + args.note
    series["production_tree"] = tree
    dump(HERE / "baseline.json", baseline)
    dump(HERE / "series.json", series)
    for extra in ("verification.json", "convergence.json"):
        path = HERE / extra
        if path.is_file():
            data = REPLAY.read_json(path)
            if "production_tree" in data:
                data["production_tree"] = tree
            if extra == "verification.json" and args.note:
                data["scope"] += " " + args.note
            if extra == "convergence.json":
                for name in changed:
                    if name in data.get("files", {}):
                        data["files"][name] = final["files"][name]
            dump(path, data)
    taught = [name for name in changed if not COURSE.supplied(name)]
    print(f"folded {len(changed)} files into {final['patch']}; production tree {tree[:12]}")
    if taught:
        print("taught files that need a directive in the final lesson:", ", ".join(taught))


if __name__ == "__main__":
    main()
