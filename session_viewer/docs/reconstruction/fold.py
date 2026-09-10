#!/usr/bin/env python3
"""Fold production edits into the steps that teach the edited lines, by rewriting git history.

Works on a repository made by `series_git.py materialize`. For every viewer file that differs
between the final step and production, each hunk is blamed against the final step: the step that
last touched the hunk's old lines (or, for a pure insertion, its context) is where the change is
folded. Steps are then amended oldest first and the later steps rebased on top; a conflict stops
the run with the repository left mid-rebase for a person to resolve (`git status`, edit,
`git add`, `git rebase --continue`, then rerun with `--resume`). `--plan` only prints the assignment.
"""
import argparse
import collections
import json
from pathlib import Path
import re
import subprocess
import sys

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import converge  # noqa: E402

HUNK = re.compile(r"^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@")
SUBJECT = re.compile(r"^step (\d\d[a-z]?): ")


def git(repo, *args, check=True, **kwargs):
    return subprocess.run(["git", "-c", "user.email=course@local", "-c", "user.name=course", *args],
                          cwd=repo, check=check, capture_output=True, text=True, **kwargs)


def steps(repo, ref):
    """Ordered [(commit, id)] for the step commits of `ref`."""
    out = []
    for line in git(repo, "log", "--reverse", "--format=%H%x1f%s", ref).stdout.splitlines():
        commit, subject = line.split("\x1f", 1)
        match = SUBJECT.match(subject)
        if match:
            out.append((commit, match.group(1)))
    return out


def hunks(diff):
    """Split a unified diff of one file into (old_start, old_len, text) hunks."""
    out, current = [], None
    for line in diff.splitlines(keepends=True):
        match = HUNK.match(line)
        if match:
            current = [int(match.group(1)), int(match.group(2) or 1), line]
            out.append(current)
        elif current is not None:
            current[2] += line
    return out


def blame_step(repo, commit, name, first, count, order):
    """The latest step among the lines [first, first+count) of `name` at `commit`."""
    if count == 0:
        first, count = max(first, 1), 1
    out = git(repo, "blame", "-l", "-L", f"{first},+{count}", commit, "--", name).stdout
    subjects = {}
    latest = None
    for line in out.splitlines():
        sha = line.split()[0]
        if sha not in subjects:
            subjects[sha] = git(repo, "log", "-1", "--format=%s", sha).stdout.strip()
        match = SUBJECT.match(subjects[sha])
        step = match.group(1) if match else None
        if step and (latest is None or order[step] > order[latest]):
            latest = step
    return latest or "00"


def plan(repo, ref, production):
    final, final_id = steps(repo, ref)[-1]
    order = {step_id: index for index, (_, step_id) in enumerate(steps(repo, ref))}
    tracked = git(repo, "ls-tree", "-r", "--name-only", final).stdout.splitlines()
    assignment = collections.defaultdict(lambda: collections.defaultdict(list))
    for name in tracked:
        if not name.startswith("session_viewer/") or name.startswith("session_viewer/docs/") or converge.packaging_reason(name):
            continue
        source = production / name
        if not source.is_file():
            continue
        blob = subprocess.run(["git", "show", f"{final}:{name}"], cwd=repo, check=True, capture_output=True).stdout
        if b"\0" in source.read_bytes()[:8000] or b"\0" in blob[:8000] or blob == source.read_bytes():
            continue
        text = blob.decode()
        diff = subprocess.run(["diff", "-u", "-", str(source)], input=text, capture_output=True, text=True).stdout
        for old_start, old_len, body in hunks(diff):
            if old_len:
                step = blame_step(repo, final, name, old_start, old_len, order)
            else:
                step = blame_step(repo, final, name, max(old_start, 1), 2, order)
            assignment[step][name].append(body)
    return final_id, assignment


def apply_to_step(repo, commit, assigned, additions=()):
    """Check out the step, apply its hunks against the current file text, amend."""
    git(repo, "checkout", "-q", commit)
    for name, source in additions:
        target = repo / name
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(source.read_bytes())
    for name, bodies in assigned.items():
        path = repo / name
        patch = f"--- a/{name}\n+++ b/{name}\n" + "".join(bodies)
        result = subprocess.run(["patch", "-p1", "--fuzz=3", "--no-backup-if-mismatch", "-r", "-", str(path)],
                                cwd=repo, input=patch, capture_output=True, text=True)
        if result.returncode != 0:
            sys.exit(f"cannot fold into {name} at {commit[:12]}:\n{result.stdout}{result.stderr}")
    git(repo, "add", "-A", "-f")
    git(repo, "commit", "-q", "--amend", "--no-edit")
    return git(repo, "rev-parse", "HEAD").stdout.strip()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, required=True)
    parser.add_argument("--ref", default="series")
    parser.add_argument("--production", type=Path, default=HERE.parent.parent.parent)
    parser.add_argument("--plan", action="store_true", help="print the hunk assignment and stop")
    parser.add_argument("--only", action="append", help="fold only these files (repeatable)")
    parser.add_argument("--pin", action="append", default=[], metavar="FILE:FROM=TO",
                        help="move the hunks blame assigned to step FROM for FILE into step TO (repeatable)")
    parser.add_argument("--add", action="append", default=[], metavar="FILE=STEP",
                        help="a file new to production: create it at STEP with its production bytes (repeatable)")
    args = parser.parse_args()
    repo = args.repo.resolve()
    production = args.production.resolve()
    final_id, assignment = plan(repo, args.ref, production)
    for pin in args.pin:
        spec, to = pin.split("=")
        name, source = spec.rsplit(":", 1)
        bodies = assignment.get(source, {}).pop(name, None)
        if bodies is None:
            sys.exit(f"nothing assigned to {name} at {source}")
        assignment[to][name].extend(bodies)
    additions = {}
    for add in args.add:
        name, step = add.split("=")
        additions.setdefault(step, []).append(name)
    if args.only:
        assignment = {step: {name: bodies for name, bodies in files.items() if name in args.only}
                      for step, files in assignment.items()}
        assignment = {step: files for step, files in assignment.items() if files}
    order = [step_id for _, step_id in steps(repo, args.ref)]
    assignment = {step: files for step, files in assignment.items() if files}
    for step in additions:
        assignment.setdefault(step, {})
    for step in sorted(assignment, key=order.index):
        for name, bodies in sorted(assignment[step].items()):
            print(f"{step}  {name}  {len(bodies)} hunk(s)")
            if args.plan:
                for body in bodies:
                    first = next((line[1:].strip() for line in body.splitlines()[1:] if line.startswith(("+", "-")) and line[1:].strip()), "")
                    print(f"        {body.splitlines()[0].split(' @@')[0]}  {first[:90]}")
        for name in additions.get(step, []):
            print(f"{step}  {name}  new file")
    if args.plan or not assignment:
        return
    for step in sorted(assignment, key=order.index):
        commits = dict((step_id, commit) for commit, step_id in steps(repo, args.ref))
        old = commits[step]
        new = apply_to_step(repo, old, assignment[step], [(name, production / name) for name in additions.get(step, [])])
        result = git(repo, "rebase", "--onto", new, old, args.ref, check=False)
        if result.returncode != 0:
            print(result.stdout, result.stderr)
            sys.exit(f"conflict while rebasing the steps after {step}; resolve in {repo}, then `git rebase --continue`")
        print(f"folded into {step}: {', '.join(sorted(assignment[step]))}")
    print(json.dumps({step: sorted(files) for step, files in assignment.items()}, indent=1))


if __name__ == "__main__":
    main()
