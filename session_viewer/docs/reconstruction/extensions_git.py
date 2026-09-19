#!/usr/bin/env python3
"""The lessons under docs/extensions as git history on top of checkpoint 21.

`append` adds the integrated chain to a series branch as steps 22.. (one commit per patch of
the `integrated` lesson) and gives every other extension lesson its own branch `alt/<id>` off
step 21, one commit per step. That is the shape `fold.py` and `check_steps.py` work on: the
integrated chain ends at production, so a production edit folds into the step that owns its
lines, and a step that stops compiling is found by name. `regenerate` writes every lesson's
patches back from those commits, in the format docs/extensions keeps (`a/` and `b/` prefixes
relative to the crate, no index lines).
"""

import argparse
import json
from pathlib import Path
import re
import subprocess

HERE = Path(__file__).resolve().parent
EXTENSIONS = HERE.parent / "extensions"
SUBJECT = re.compile(r"^step (\d\d[a-z]?): (.*)$")
FIRST_TAIL = 22


def git(repo, *args, **kwargs):
    return subprocess.run(
        ["git", "-c", "user.email=course@local", "-c", "user.name=course", *args],
        cwd=repo,
        check=True,
        capture_output=True,
        text=True,
        **kwargs,
    ).stdout


def lessons():
    return json.loads((EXTENSIONS / "lessons.json").read_text())["lessons"]


def write_lessons(records):
    data = json.loads((EXTENSIONS / "lessons.json").read_text())
    data["lessons"] = records
    (EXTENSIONS / "lessons.json").write_text(json.dumps(data, indent=2) + "\n")


def step_commits(repo, ref):
    out = []
    for line in git(repo, "log", "--reverse", "--format=%H%x1f%s", ref).splitlines():
        commit, subject = line.split("\x1f", 1)
        match = SUBJECT.match(subject)
        if match:
            out.append((commit, match.group(1), match.group(2)))
    return out


def apply_and_commit(repo, patch, step_id, title):
    subprocess.run(
        ["git", "apply", "--directory=session_viewer", str(patch)],
        cwd=repo,
        check=True,
        capture_output=True,
        text=True,
    )
    git(repo, "add", "-A", "-f", "session_viewer")
    git(repo, "commit", "-q", "--allow-empty", "-m", f"step {step_id}: {title}")


def append(repo, ref):
    """The integrated chain as steps 22.. on `ref`; every other lesson on `alt/<id>` off step 21."""
    base = [
        commit for commit, step_id, _ in step_commits(repo, ref) if step_id == "21"
    ][0]
    for lesson in lessons():
        if lesson.get("chapters"):
            git(repo, "checkout", "-q", ref)
            for number, step in enumerate(lesson["steps"], FIRST_TAIL):
                apply_and_commit(
                    repo, EXTENSIONS / step["patch"], f"{number}", step["title"]
                )
                print(f"step {number}: {step['title']}")
        else:
            branch = f"alt/{lesson['id']}"
            git(repo, "checkout", "-q", "-B", branch, base)
            for number, step in enumerate(lesson["steps"], 1):
                apply_and_commit(
                    repo,
                    EXTENSIONS / step["patch"],
                    f"{FIRST_TAIL + number - 1}",
                    step["title"],
                )
                print(f"{branch} step {number}: {step['title']}")
    git(repo, "checkout", "-q", ref)


def patch_text(repo, previous, current):
    diff = git(
        repo,
        "diff",
        "--no-renames",
        "--binary",
        "--src-prefix=a/",
        "--dst-prefix=b/",
        previous,
        current,
        "--",
        "session_viewer",
    )
    lines = []
    for line in diff.split("\n"):
        if line.startswith("index "):
            continue
        lines.append(
            line.replace("a/session_viewer/", "a/").replace("b/session_viewer/", "b/")
        )
    return "\n".join(lines)


def regenerate(repo, ref):
    """Every lesson's patches from its commits: the tail of `ref` and the `alt/<id>` branches."""
    base = [
        commit for commit, step_id, _ in step_commits(repo, ref) if step_id == "21"
    ][0]
    records = lessons()
    for lesson in records:
        branch = ref if lesson.get("chapters") else f"alt/{lesson['id']}"
        chain = [
            commit
            for commit, step_id, _ in step_commits(repo, branch)
            if int(step_id[:2]) >= FIRST_TAIL
        ]
        if len(chain) != len(lesson["steps"]):
            raise SystemExit(
                f"{lesson['id']}: {len(chain)} commits for {len(lesson['steps'])} steps"
            )
        previous = base
        for step, commit in zip(lesson["steps"], chain):
            (EXTENSIONS / step["patch"]).write_text(patch_text(repo, previous, commit))
            step.pop("kernel_patch", None)
            print(f"{lesson['id']}: {step['patch']} written")
            previous = commit
    write_lessons(records)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    add = sub.add_parser(
        "append", help="commit every extension lesson onto the series repository"
    )
    add.add_argument("--repo", type=Path, required=True)
    add.add_argument("--ref", default="series")
    regen = sub.add_parser(
        "regenerate", help="write docs/extensions patches from the commits"
    )
    regen.add_argument("--repo", type=Path, required=True)
    regen.add_argument("--ref", default="series")
    args = parser.parse_args()
    if args.command == "append":
        append(args.repo.resolve(), args.ref)
    else:
        regenerate(args.repo.resolve(), args.ref)


if __name__ == "__main__":
    main()
