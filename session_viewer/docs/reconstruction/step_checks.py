#!/usr/bin/env python3
"""Measure what compiles at the end of every numbered step of every lesson.

`compile_points.py` answers "does the marked place compile". This answers the reader's own
question: after this step, can I build? A file Rust has not been told about cannot break the
crate, so the early steps of a big lesson pass without compiling a line of the new code, and
the step that declares the modules is where the answer starts meaning something. Results go
to step-checks.json, which `step_status.py` renders into the lessons.
"""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import sys

HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("compile_points", HERE / "compile_points.py")
CP = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CP)
COURSE = CP.COURSE
REPLAY = CP.REPLAY
HEADING = re.compile(r"^#{2,3} (Step|Part) ([^\n]*)$", re.M)


def events(markdown):
    """Step headings and directives, in reading order."""
    marks = [(m.start(), "heading", m.group(0).lstrip("# ")) for m in HEADING.finditer(markdown)]
    marks += [(m.start(), "directive", m) for m in COURSE.DIRECTIVE.finditer(markdown)]
    marks.sort(key=lambda item: item[0])
    return marks


def measure(step, markdown, workspace, env, log_dir):
    """Apply directives in order; cargo check at the end of every step that wrote a file."""
    results, order, current, touched, index = {}, [], None, False, 0

    def check(label):
        nonlocal index
        index += 1
        ok = CP.cargo_check(workspace / "session_viewer", env, log_dir / f"step-{index}.log")
        results[label] = "ok" if ok else "fail"
        order.append(label)
        print(f"  {label}: {'ok' if ok else 'FAIL'}", flush=True)

    for _, kind, payload in events(markdown):
        if kind == "heading":
            if current and touched:
                check(current)
            current, touched = payload, False
            continue
        directive, step_id, rest = payload.groups()
        if step_id != step.id or directive not in ("file", "supplied"):
            continue
        CP.apply_directive(step, directive, rest, workspace)
        touched = True
    if current and touched:
        check(current)
    REPLAY.check_files(workspace, step.record["files"])
    (workspace / ".reconstruction-state.json").write_text(json.dumps({
        "checkpoint": step.id, "files": step.record["files"]}, indent=2) + "\n")
    return {"order": order, "results": results}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lesson", action="append", help="step id, repeatable; default every lesson")
    parser.add_argument("--root", type=Path, default=Path.home() / ".cache/session-viewer-course")
    args = parser.parse_args()
    root = args.root.resolve()
    audit_root = root / "audit"
    if audit_root.exists():
        shutil.rmtree(audit_root)
    audit_root.mkdir(parents=True)
    (audit_root / "empty").mkdir()
    series, steps = COURSE.snapshot_steps(audit_root)
    wanted = args.lesson or [step.id for step in steps]
    # 195 distinct source states: incremental fingerprints would fill the disk and buy nothing,
    # because no two checks share a state.
    env = dict(os.environ, REGEN_PROTO="0", NO_COLOR="true", CARGO_BUILD_JOBS="4",
               CARGO_INCREMENTAL="0", CARGO_TARGET_DIR=str(root / "target"))
    out_path = HERE / "step-checks.json"
    out = json.loads(out_path.read_text()) if out_path.is_file() else {}
    for step in steps:
        if step.id not in wanted:
            continue
        lesson = COURSE.lesson_for(step.id)
        print(f"lesson {step.id}: {lesson.name}", flush=True)
        workspace = CP.workspace_at(root, series, step.id)
        out[step.id] = measure(step, lesson.read_text(), workspace, env, root / "logs" / step.id)
        out_path.write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
    print(f"recorded {out_path}")


if __name__ == "__main__":
    sys.exit(main())
