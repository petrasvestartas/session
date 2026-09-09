#!/usr/bin/env python3
"""Measure every `<!-- check: NN -->` marker: type the lesson up to it, then run cargo check.

The lesson's file directives are applied literally, in the order the reader meets them, into a
workspace reconstructed at the previous checkpoint. At each check marker the WASM library is
checked. Results go to compile-points.json, which the lesson audit and the rendered pages read;
a marker without a recorded pass renders as UNMEASURED and fails the audit.
"""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("course_pages", HERE.parent / "course_pages.py")
COURSE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(COURSE)
REPLAY = COURSE.REPLAY


def workspace_at(root, series, step_id):
    """One reusable workspace at the checkpoint before `step_id` (a stable path keeps cargo warm)."""
    steps = series["steps"]
    index = [step["id"] for step in steps].index(step_id)
    previous = steps[index - 1] if index else None
    target = root / "steps" / "current"
    state = target / ".reconstruction-state.json"
    if target.is_dir() and previous is not None and state.is_file():
        try:
            if json.loads(state.read_text()).get("checkpoint") == previous["id"]:
                REPLAY.check_files(target, previous["files"])
                return target
        except ValueError:
            pass
    if target.exists():
        shutil.rmtree(target)
    REPLAY.initialize(target)
    REPLAY.run(["git", "init", "--quiet"], target, dict(os.environ), root / "logs/init.log")
    for step in steps[:index]:
        REPLAY.apply_step(target, step, dict(os.environ), root / "logs" / step["id"])
    return target


def cargo_check(viewer, env, log):
    """One WASM library check with the house build limits."""
    log.parent.mkdir(parents=True, exist_ok=True)
    with log.open("w") as output:
        result = subprocess.run(["cargo", "check", "--locked", "--lib"], cwd=viewer, env=env,
                                stdout=output, stderr=subprocess.STDOUT)
    return result.returncode == 0


def measure(step, lesson_markdown, workspace, env, log_dir):
    """Apply directives in order; run cargo check at every check marker."""
    results = {}
    position = 0
    for match in COURSE.DIRECTIVE.finditer(lesson_markdown):
        kind, step_id, rest = match.groups()
        if step_id != step.id:
            continue
        if kind == "file":
            options = COURSE.parse_args_text(rest)
            name = options["path"]
            change = step.changes[name]
            path = workspace / name
            if change.kind == "delete":
                if path.exists():
                    path.unlink()
            elif change.kind == "create" or options["whole"]:
                path.parent.mkdir(parents=True, exist_ok=True)
                if options["lines"]:
                    first, last = options["lines"]
                    lines = change.new_text.split("\n")
                    if lines and lines[-1] == "":
                        lines.pop()
                    chunk = "\n".join(lines[first - 1:last]) + "\n"
                    existing = path.read_text() if first > 1 and path.is_file() else ""
                    path.write_text(existing + chunk)
                else:
                    path.write_text(change.new_text)
            else:
                hunks = options["hunks"] or list(range(1, len(change.hunks) + 1))
                current = path.read_text()
                for number in hunks:
                    current = COURSE.apply_hunk(name, change.hunks[number - 1], change.old_text, current)
                path.write_text(COURSE.finish_newline(current, change.new_text))
        elif kind == "supplied":
            for name in step.supplied_files():
                path = workspace / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(step.changes[name].new_text)
            REPLAY.copy_assets(workspace, step.record)
        elif kind == "check":
            ok = cargo_check(workspace / "session_viewer", env, log_dir / f"check-{position}.log")
            results[str(position)] = "ok" if ok else "fail"
            print(f"  check at position {position}: {'ok' if ok else 'FAIL'}", flush=True)
        position += 1
    REPLAY.check_files(workspace, step.record["files"])
    (workspace / ".reconstruction-state.json").write_text(json.dumps({
        "checkpoint": step.id, "files": step.record["files"]}, indent=2) + "\n")
    return results


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lesson", action="append", help="step id, repeatable; default every lesson")
    parser.add_argument("--root", type=Path, default=Path.home() / ".cache/session-viewer-course",
                        help="reusable reconstruction and cargo cache")
    args = parser.parse_args()
    root = args.root.resolve()
    audit_root = root / "audit"
    if audit_root.exists():
        shutil.rmtree(audit_root)
    audit_root.mkdir(parents=True)
    (audit_root / "empty").mkdir()
    series, steps = COURSE.snapshot_steps(audit_root)
    wanted = args.lesson or [step.id for step in steps]
    env = dict(os.environ, REGEN_PROTO="0", NO_COLOR="true", CARGO_BUILD_JOBS="4",
               CARGO_TARGET_DIR=str(root / "target"))
    points_path = HERE / "compile-points.json"
    points = json.loads(points_path.read_text()) if points_path.is_file() else {}
    for step in steps:
        if step.id not in wanted:
            continue
        lesson = COURSE.lesson_for(step.id)
        print(f"lesson {step.id}: {lesson.name}", flush=True)
        workspace = workspace_at(root, series, step.id)
        results = measure(step, lesson.read_text(), workspace, env, root / "logs" / step.id)
        points[step.id] = results
        points_path.write_text(json.dumps(points, indent=2, sort_keys=True) + "\n")
    print(f"recorded {points_path}")


if __name__ == "__main__":
    sys.exit(main())
