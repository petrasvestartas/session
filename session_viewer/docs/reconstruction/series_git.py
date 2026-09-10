#!/usr/bin/env python3
"""Edit the checkpoint series as git history: materialize it, edit commits, regenerate the contract.

`materialize` reconstructs every checkpoint into one git repository, one commit per step on top
of a base commit holding the pinned kernel and its siblings, tagged `step/NN`. Edit that history
with ordinary git (amend a commit and rebase the rest onto it, insert a commit, drop paths), then
`regenerate` rewrites the patches, every step's inventory, the patch and production tree hashes,
baseline.json, convergence.json and verification.json from the branch. A step is a commit whose
subject is `step NN: title`; binary files changed by a step are recorded as hash-checked assets
rather than diffed. Run `course_pages.py --audit` and `compile_points.py` afterwards.
"""
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import subprocess

HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("course_replay", HERE / "replay.py")
REPLAY = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(REPLAY)
CONVERGE_SPEC = importlib.util.spec_from_file_location("converge", HERE / "converge.py")
CONVERGE = importlib.util.module_from_spec(CONVERGE_SPEC)
CONVERGE_SPEC.loader.exec_module(CONVERGE)
SECTION = re.compile(r"(?m)^(?=diff --git )")
SUBJECT = re.compile(r"^step (\d\d[a-z]?): (.*)$")
STATE = ".reconstruction-state.json"


def git(repo, *args, **kwargs):
    return subprocess.run(["git", "-c", "user.email=course@local", "-c", "user.name=course", *args],
                          cwd=repo, check=True, capture_output=True, text=True, **kwargs).stdout


def dump(path, data):
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n")


def materialize(repo):
    """Base commit from the pinned archives, then one commit per verified step."""
    series = REPLAY.read_json(HERE / "series.json")
    REPLAY.initialize(repo)
    git(repo, "init", "--quiet", "-b", "series")
    git(repo, "add", "-A", "-f")
    git(repo, "commit", "--quiet", "-m", "base: pinned kernel and siblings")
    git(repo, "tag", "step/base")
    env = dict(os.environ)
    REPLAY.print = lambda *args, **kwargs: None
    for step in series["steps"]:
        REPLAY.apply_step(repo, step, env, repo.parent / f"{repo.name}-evidence" / step["id"])
        (repo / STATE).unlink()
        git(repo, "add", "-A", "-f")
        git(repo, "commit", "--quiet", "--allow-empty", "-m", f"step {step['id']}: {step['title']}")
        git(repo, "tag", f"step/{step['id']}")
    shutil.rmtree(repo.parent / f"{repo.name}-evidence", ignore_errors=True)
    print(f"materialized {len(series['steps'])} steps in {repo} on branch 'series'")


def commits(repo, ref):
    """(hash, id, title) for every step commit after the base, oldest first."""
    out = []
    for line in git(repo, "log", "--reverse", "--format=%H%x1f%s", ref).splitlines():
        commit, subject = line.split("\x1f", 1)
        match = SUBJECT.match(subject)
        if match:
            out.append((commit, match.group(1), match.group(2)))
        elif not subject.startswith("base:"):
            raise SystemExit(f"commit {commit[:12]} is not a step: {subject!r}")
    return out


def inventory(repo, commit):
    """sha256 of every tracked file at a commit, keyed by workspace path."""
    entries = []
    for line in git(repo, "ls-tree", "-r", "-z", commit).split("\0"):
        if line:
            meta, name = line.split("\t", 1)
            entries.append((meta.split()[2], name))
    blobs = subprocess.run(["git", "cat-file", "--batch"], cwd=repo, check=True, capture_output=True,
                           input="".join(sha + "\n" for sha, _ in entries).encode()).stdout
    files, binary, at = {}, set(), 0
    for sha, name in entries:
        header_end = blobs.index(b"\n", at)
        size = int(blobs[at:header_end].split()[2])
        data = blobs[header_end + 1:header_end + 1 + size]
        at = header_end + 1 + size + 1
        files[name] = hashlib.sha256(data).hexdigest()
        if b"\0" in data[:8000]:
            binary.add(name)
    return files, binary


def regenerate(repo, ref, meta_path, production):
    """Rewrite patches, inventories and every recorded hash from the branch."""
    series = REPLAY.read_json(HERE / "series.json")
    known = {step["id"]: step for step in series["steps"]}
    meta = json.loads(Path(meta_path).read_text()) if meta_path else {}
    steps = []
    previous = git(repo, "rev-list", "--max-parents=0", ref).strip()
    previous_files = inventory(repo, previous)[0]
    previous_known = None
    for commit, step_id, title in commits(repo, ref):
        files, binary = inventory(repo, commit)
        record = known.get(step_id, {})
        # An untouched step keeps its verified patch bytes, so hunk numbers in lesson directives
        # stay valid; only a step whose tree or predecessor tree changed is diffed afresh.
        untouched = (record.get("files") == files and previous_known is not None
                     and previous_known.get("files") == previous_files
                     and (HERE / record["patch"]).is_file()
                     and hashlib.sha256((HERE / record["patch"]).read_bytes()).hexdigest() == record["patch_sha256"])
        if step_id == "00" and record.get("files") == files and (HERE / record.get("patch", "")).is_file():
            untouched = hashlib.sha256((HERE / record["patch"]).read_bytes()).hexdigest() == record["patch_sha256"]
        record = {"assets": list(record.get("assets", [])), "browser": record.get("browser", meta.get(step_id, {}).get("browser", {})),
                  "files": files, "id": step_id, "patch": f"patches/{step_id}.patch", "patch_sha256": "",
                  "title": meta.get(step_id, {}).get("title", record.get("title", title))}
        changed = [line.split("\t", 1)[1] for line in git(repo, "diff", "--name-status", "--no-renames", previous, commit).splitlines()]
        assets = {asset["target"]: asset for asset in record["assets"] if asset["target"] in files}
        for name in changed:
            if name in binary and name not in assets:
                data = subprocess.run(["git", "show", f"{commit}:{name}"], cwd=repo, check=True, capture_output=True).stdout
                source = f"fixtures/{files[name]}-{Path(name).name}"
                (HERE / source).write_bytes(data)
                assets[name] = {"sha256": files[name], "source": source, "target": name}
        record["assets"] = [assets[name] for name in sorted(assets)]
        patch = HERE / record["patch"]
        if not untouched:
            diff = git(repo, "diff", "--full-index", "--no-renames", previous, commit)
            kept = [chunk for chunk in SECTION.split(diff)
                    if not chunk.startswith("diff --git ") or chunk.splitlines()[0].split(" b/", 1)[1] not in binary]
            patch.write_text("".join(kept))
        record["patch_sha256"] = hashlib.sha256(patch.read_bytes()).hexdigest()
        steps.append(record)
        previous, previous_files, previous_known = commit, files, known.get(step_id)
    for stale in set(known) - {step["id"] for step in steps}:
        (HERE / known[stale]["patch"]).unlink(missing_ok=True)
    series["steps"] = steps
    # The baseline is the PRODUCTION inventory: every recorded file that still exists, plus every
    # viewer file the final checkpoint now carries, at the bytes production has. converge.py then
    # requires each of them to match the final checkpoint unless it is a documented packaging difference.
    final = steps[-1]["files"]
    baseline = REPLAY.read_json(HERE / "baseline.json")
    names = set(baseline["files"]) | {name for name in final if name.startswith("session_viewer/")}
    kept_files = {name: REPLAY.digest(production / name) for name in sorted(names) if (production / name).is_file()}
    for name, checksum in kept_files.items():
        if name in final and final[name] != checksum and CONVERGE.packaging_reason(name) is None:
            print(f"warning: {name} differs between production and checkpoint {steps[-1]['id']}; converge.py will fail")
    baseline["files"] = kept_files
    baseline["final_checkpoint"] = steps[-1]["id"]
    encoding = "".join(name + "\0" + checksum + "\n" for name, checksum in sorted(kept_files.items()))
    tree = hashlib.sha256(encoding.encode()).hexdigest()
    baseline["tree_sha256"] = tree
    series["production_tree"] = tree
    dump(HERE / "baseline.json", baseline)
    dump(HERE / "series.json", series)
    for extra in ("verification.json", "convergence.json"):
        path = HERE / extra
        if path.is_file():
            data = REPLAY.read_json(path)
            data["production_tree"] = tree
            if "files" in data:
                data["files"] = {name: kept_files[name] for name in data["files"] if name in kept_files}
            dump(path, data)
    print(f"regenerated {len(steps)} steps ({', '.join(step['id'] for step in steps)}); production tree {tree[:12]}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    make = sub.add_parser("materialize", help="reconstruct the series as a git repository")
    make.add_argument("--repo", type=Path, required=True, help="new, empty directory")
    regen = sub.add_parser("regenerate", help="rewrite the contract from a branch of that repository")
    regen.add_argument("--repo", type=Path, required=True)
    regen.add_argument("--ref", default="series")
    regen.add_argument("--meta", help="JSON {id: {title, browser}} for steps the series does not know yet")
    regen.add_argument("--production", type=Path, default=HERE.parent.parent.parent,
                       help="the checkout that holds session_viewer/ (the baseline is its inventory)")
    args = parser.parse_args()
    if args.command == "materialize":
        materialize(args.repo.resolve())
    else:
        regenerate(args.repo.resolve(), args.ref, args.meta, args.production.resolve())


if __name__ == "__main__":
    main()
