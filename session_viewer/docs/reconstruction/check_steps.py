#!/usr/bin/env python3
"""cargo check the viewer library at every step of a series branch; report the steps that fail.

Run after `fold.py`: blame places each hunk where its lines were last touched, which can put one
half of a cross-file change a step after the other half. A step that does not compile names
the files to `--pin` earlier.
"""
import argparse
import os
from pathlib import Path
import subprocess
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fold import steps  # noqa: E402


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, required=True)
    parser.add_argument("--ref", default="series")
    parser.add_argument("--from-step", help="first step to check")
    parser.add_argument("--target-dir", type=Path, default=Path.home() / ".cache/session-viewer-course/steps-target")
    args = parser.parse_args()
    repo = args.repo.resolve()
    tree = repo.parent / f"{repo.name}-check"
    if not tree.exists():
        subprocess.run(["git", "worktree", "add", "-q", "--detach", str(tree), args.ref], cwd=repo, check=True)
    env = dict(os.environ, REGEN_PROTO="0", NO_COLOR="true", CARGO_TARGET_DIR=str(args.target_dir.resolve()),
               CARGO_BUILD_JOBS="8")
    failed = []
    started = args.from_step is None
    for commit, step_id in steps(repo, args.ref):
        started = started or step_id == args.from_step
        if not started:
            continue
        subprocess.run(["git", "checkout", "-q", "--detach", commit], cwd=tree, check=True)
        result = subprocess.run(["cargo", "check", "--locked", "--lib"], cwd=tree / "session_viewer", env=env,
                                capture_output=True, text=True)
        status = "ok" if result.returncode == 0 else "FAIL"
        print(f"{step_id}: {status}", flush=True)
        if result.returncode != 0:
            errors = [line for line in result.stderr.splitlines() if line.startswith("error") or line.strip().startswith("-->")]
            print("\n".join("    " + line for line in errors[:24]), flush=True)
            failed.append(step_id)
    print("failing steps:", ", ".join(failed) if failed else "none")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
