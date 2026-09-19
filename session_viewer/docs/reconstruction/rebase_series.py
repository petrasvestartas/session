#!/usr/bin/env python3
"""Put every step's viewer on the kernel base that kernel-base.json pins now.

Works on a repository made by `series_git.py materialize`. A new branch starts from a base
commit holding the freshly pinned archives (`pin_kernel.py`), and every step of `--old` is
replayed onto it as the same step with the same viewer tree - `session_viewer/` and nothing
else - so whatever a step used to change under the kernel or the parity sources is dropped:
the maintained kernel is the kernel now, in every lesson. Then `fold.py` for the viewer edits
that go with the new kernel, `check_steps.py`, and `series_git.py regenerate --ref <new>`.
"""

import argparse
import importlib.util
from pathlib import Path
import re
import shutil
import subprocess
import tempfile

HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("course_replay", HERE / "replay.py")
REPLAY = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(REPLAY)
SUBJECT = re.compile(r"^step (\d\d[a-z]?): (.*)$")


def git(repo, *args, **kwargs):
    return subprocess.run(
        ["git", "-c", "user.email=course@local", "-c", "user.name=course", *args],
        cwd=repo,
        check=True,
        capture_output=True,
        text=True,
        **kwargs,
    ).stdout


def steps(repo, ref):
    out = []
    for line in git(repo, "log", "--reverse", "--format=%H%x1f%s", ref).splitlines():
        commit, subject = line.split("\x1f", 1)
        match = SUBJECT.match(subject)
        if match:
            out.append((commit, match.group(1), match.group(2)))
    return out


def rebase(repo, old, new):
    with tempfile.TemporaryDirectory(
        prefix="viewer-kernel-base-", dir=repo.parent
    ) as temp:
        base = Path(temp) / "base"
        REPLAY.initialize(base)
        git(repo, "checkout", "-q", "--orphan", new)
        git(repo, "rm", "-rfq", "--cached", ".")
        for path in list(repo.iterdir()):
            if path.name != ".git":
                shutil.rmtree(path) if path.is_dir() else path.unlink()
        shutil.copytree(base, repo, dirs_exist_ok=True)
    git(repo, "add", "-A", "-f")
    git(repo, "commit", "-q", "-m", "base: pinned kernel and siblings")
    git(repo, "tag", "-f", f"{new}/base")
    for commit, step_id, title in steps(repo, old):
        git(repo, "rm", "-rfq", "--cached", "--ignore-unmatch", "session_viewer")
        shutil.rmtree(repo / "session_viewer", ignore_errors=True)
        git(repo, "read-tree", "--prefix=session_viewer/", f"{commit}:session_viewer")
        git(repo, "checkout-index", "-a", "-f")
        git(repo, "add", "-A", "-f", "session_viewer")
        git(repo, "commit", "-q", "--allow-empty", "-m", f"step {step_id}: {title}")
        git(repo, "tag", "-f", f"{new}/{step_id}")
        print(f"step {step_id} on the pinned kernel")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, required=True)
    parser.add_argument(
        "--old", default="series", help="the branch whose viewer trees are kept"
    )
    parser.add_argument("--new", default="rebased", help="the branch to create")
    args = parser.parse_args()
    rebase(args.repo.resolve(), args.old, args.new)


if __name__ == "__main__":
    main()
