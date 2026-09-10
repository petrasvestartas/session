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


SCENE_LINES = [
    "@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;\n",
    "@group(1) @binding(0) var<uniform> line: LineUniform;\n",
    "@group(2) @binding(0) var<storage, read> instances: array<Instance>;\n",
    "@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;\n",
    "// A point of object `i` in the anchored frame: rotation/scale from the row, translation\n",
    "// from the 16 B table a re-anchor rewrites.\n",
    "// The sub id a marker answers: ink, not a face, to the pick window; no row behind it.\n",
    "// Same 96-byte instance record as the arena: model 0, color 64, flags 80,\n",
    "// thickness 84, spacing 88; the storage-array stride rounds up to 96 bytes.\n",
    "// pad 84, spacing 88; the storage-array stride rounds up to 96 bytes.\n",
    "// Object-local point to the camera's rebased world frame.\n",
]
SCENE_BLOCKS = [
    re.compile(r"^struct Instance \{.*?\n\};?\n", re.M | re.S),
    re.compile(r"^struct LineUniform \{.*?\n\};?\n", re.M | re.S),
    re.compile(r"^fn place\(.*?\n\}\n", re.M | re.S),
    re.compile(r"^fn oct16_decode\(.*?\n\}\n", re.M | re.S),
    re.compile(r"^const (FLAG_\w+|FACING_UNKNOWN|DISC_ID_TAG|SELECT_COLOR|MM_TO_M|HAIRLINE_MIN_ALPHA): [^\n]*\n", re.M),
]


def strip_scene(text):
    """Remove the declarations `scene.wgsl` now provides from a lane shader."""
    for line in SCENE_LINES:
        text = text.replace(line, "")
    for block in SCENE_BLOCKS:
        text = block.sub("", text)
    return re.sub(r"\n{3,}", "\n\n", text)


def apply_to_step(repo, commit, assigned, additions=(), strips=(), patches=()):
    """Check out the step, apply its hunks against the current file text, amend."""
    git(repo, "checkout", "-q", commit)
    for name, source in additions:
        target = repo / name
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(source.read_bytes())
    for name in strips:
        target = repo / name
        target.write_text(strip_scene(target.read_text()))
    for patch in patches:
        result = subprocess.run(["patch", "-p1", "--fuzz=3", "--no-backup-if-mismatch", "-r", "-"], cwd=repo,
                                input=Path(patch).read_text(), capture_output=True, text=True)
        if result.returncode != 0:
            sys.exit(f"cannot apply {patch} at {commit[:12]}:\n{result.stdout}{result.stderr}")
    for name, bodies in assigned.items():
        path = repo / name
        bodies = sorted(bodies, key=lambda body: int(HUNK.match(body).group(1)))
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
    parser.add_argument("--pin", action="append", default=[], metavar="FILE:FROM[#N]=TO",
                        help="move the hunks (or hunk N of --plan's list) blame assigned to step FROM for FILE into step TO")
    parser.add_argument("--add", action="append", default=[], metavar="FILE=STEP",
                        help="a file new to production: create it at STEP with its production bytes (repeatable)")
    parser.add_argument("--strip-scene", action="append", default=[], metavar="FILE=STEP",
                        help="at STEP remove the scene-contract declarations from this lane shader (they live in scene.wgsl)")
    parser.add_argument("--patch", action="append", default=[], metavar="STEP=PATCHFILE",
                        help="a hand-written unified diff to apply at STEP after the folded hunks")
    parser.add_argument("--drop", action="append", default=[], metavar="FILE:STEP#N",
                        help="do not apply hunk N of FILE at STEP (a --strip-scene already made the change)")
    args = parser.parse_args()
    repo = args.repo.resolve()
    production = args.production.resolve()
    final_id, assignment = plan(repo, args.ref, production)
    # Indices in --pin refer to the list --plan printed, so resolve every pin against the
    # untouched lists before moving anything.
    moves = collections.defaultdict(list)
    for pin in args.pin:
        spec, to = pin.split("=")
        name, source = spec.rsplit(":", 1)
        index = None
        if "#" in source:
            source, number = source.split("#")
            index = int(number) - 1
        bodies = assignment.get(source, {}).get(name)
        if not bodies:
            sys.exit(f"nothing assigned to {name} at {source}")
        indices = [index] if index is not None else list(range(len(bodies)))
        for i in indices:
            moves[(source, name)].append((i, to))
    for (source, name), moved in moves.items():
        bodies = assignment[source][name]
        keep = [body for i, body in enumerate(bodies) if i not in {i for i, _ in moved}]
        for i, to in moved:
            assignment[to][name].append(bodies[i])
        if keep:
            assignment[source][name] = keep
        else:
            assignment[source].pop(name)
    for drop in args.drop:
        spec, number = drop.rsplit("#", 1)
        name, source = spec.rsplit(":", 1)
        moves.setdefault((source, name), [])
        assignment[source][name][int(number) - 1] = None
    for (source, name), _ in list(moves.items()):
        assignment[source][name] = [body for body in assignment[source][name] if body is not None]
        if not assignment[source][name]:
            assignment[source].pop(name)
    additions, strips, patches = {}, {}, {}
    for extra in args.patch:
        step, path = extra.split("=")
        patches.setdefault(step, []).append(path)
    for add in args.add:
        name, step = add.split("=")
        additions.setdefault(step, []).append(name)
    for strip in args.strip_scene:
        name, step = strip.split("=")
        strips.setdefault(step, []).append(name)
    if args.only:
        assignment = {step: {name: bodies for name, bodies in files.items() if name in args.only}
                      for step, files in assignment.items()}
        assignment = {step: files for step, files in assignment.items() if files}
    order = [step_id for _, step_id in steps(repo, args.ref)]
    assignment = {step: files for step, files in assignment.items() if files}
    for step in list(additions) + list(strips) + list(patches):
        assignment.setdefault(step, {})
    stripped, applied = set(), {}
    for step in sorted(assignment, key=order.index):
        for name, bodies in sorted(assignment[step].items()):
            print(f"{step}  {name}  {len(bodies)} hunk(s)")
            if args.plan:
                for body in bodies:
                    first = next((line[1:].strip() for line in body.splitlines()[1:] if line.startswith(("+", "-")) and line[1:].strip()), "")
                    print(f"        {body.splitlines()[0].split(' @@')[0]}  {first[:90]}")
        for name in additions.get(step, []):
            print(f"{step}  {name}  new file")
        for name in strips.get(step, []):
            print(f"{step}  {name}  scene declarations stripped")
    if args.plan or not assignment:
        return
    for step in sorted(assignment, key=order.index):
        commits = dict((step_id, commit) for commit, step_id in steps(repo, args.ref))
        old = commits[step]
        new = apply_to_step(repo, old, assignment[step], [(name, production / name) for name in additions.get(step, [])],
                            strips.get(step, []), patches.get(step, []))
        for name in strips.get(step, []):
            stripped.add(name)
        for name, bodies in assignment[step].items():
            applied.setdefault(name, []).extend(bodies)
        for patch in patches.get(step, []):
            text = Path(patch).read_text()
            name = text.split("+++ b/", 1)[1].split("\n", 1)[0]
            applied.setdefault(name, []).append(text.split("\n", 2)[2])
        result = git(repo, "rebase", "--onto", new, old, args.ref, check=False)
        while result.returncode != 0:
            # A later step touched lines next to what this fold changed. Take that step's own
            # version of the file (its evolution is the truth), then redo what the fold had
            # already done to the file: the strip, and every hunk folded into an earlier step.
            conflicted = git(repo, "diff", "--name-only", "--diff-filter=U").stdout.split()
            if not conflicted:
                print(result.stdout, result.stderr)
                sys.exit(f"rebase stopped after {step} without a conflict to resolve; see {repo}")
            for name in conflicted:
                git(repo, "checkout", "--theirs", "--", name)
                path = repo / name
                if name in stripped:
                    path.write_text(strip_scene(path.read_text()))
                for body in applied.get(name, []):
                    subprocess.run(["patch", "-p1", "-N", "--fuzz=3", "--no-backup-if-mismatch", "-r", "-", str(path)],
                                   cwd=repo, input=f"--- a/{name}\n+++ b/{name}\n{body}", capture_output=True, text=True)
                git(repo, "add", "--", name)
                print(f"  resolved {name} while replaying a later step")
            result = git(repo, "-c", "core.editor=true", "rebase", "--continue", check=False)
        print(f"folded into {step}: {', '.join(sorted(assignment[step]))}")
    print(json.dumps({step: sorted(files) for step, files in assignment.items()}, indent=1))


if __name__ == "__main__":
    main()
