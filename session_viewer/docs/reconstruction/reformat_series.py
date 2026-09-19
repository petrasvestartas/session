#!/usr/bin/env python3
"""Put every checkpoint of the course through `house_format.py`, then rebuild the patches.

`series` walks the step commits of a repository made by `series_git.py materialize`, formats
the viewer files of each step's tree (never the supplied tests, examples, fixtures and kernel
ports) and commits the result as the same step on a new branch. The formatter is deterministic
and idempotent, so a line a step did not touch is identical in the neighbouring steps and the
diff between two formatted steps is the formatted version of the original change - no rebase,
no conflict. `series_git.py regenerate --ref <new>` and `retile.py` then rewrite the contract.

`extensions` does the same for the lessons under docs/extensions, which start from checkpoint
21: each original patch is applied to the original tree, the result formatted, and the new
patch is the diff between two formatted trees. The integrated chain ends at the viewer as it is
in the checkout, so the last step that touches a file also carries what production changed in
it since the chain was recorded. Run `extensions.py --verify` and `--write` afterwards.
"""

import argparse
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile

HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location(
    "course_pages", HERE.parent / "course_pages.py"
)
COURSE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(COURSE)
FORMAT_SPEC = importlib.util.spec_from_file_location(
    "house_format", HERE / "house_format.py"
)
FORMAT = importlib.util.module_from_spec(FORMAT_SPEC)
FORMAT_SPEC.loader.exec_module(FORMAT)
SUBJECT = re.compile(r"^step (\d\d[a-z]?): (.*)$")
EXTENSIONS = HERE.parent / "extensions"


def git(cwd, *args, check=True, **kwargs):
    return subprocess.run(
        ["git", "-c", "user.email=course@local", "-c", "user.name=course", *args],
        cwd=cwd,
        check=check,
        capture_output=True,
        text=True,
        **kwargs,
    ).stdout


def formatted_paths(root, prefix=""):
    """The viewer files the course shows, as paths under `root`."""
    viewer = root / "session_viewer" if (root / "session_viewer").is_dir() else root
    out = []
    for path in sorted(viewer.rglob("*")):
        if path.suffix not in FORMAT.FORMATTERS or not path.is_file():
            continue
        relative = path.relative_to(viewer).as_posix()
        if relative.startswith(
            ("assets/", "docs/", "dist/", "target/")
        ) or COURSE.supplied("session_viewer/" + relative):
            continue
        out.append(path)
    return out


PORT_SPEC = importlib.util.spec_from_file_location("port_math", HERE / "port_math.py")
PORT = importlib.util.module_from_spec(PORT_SPEC)
PORT_SPEC.loader.exec_module(PORT)


def format_tree(root, keep_headers, port=False):
    changed = 0
    for path in formatted_paths(root):
        text = path.read_text()
        ported = PORT.port(text, path.as_posix()) if port and path.suffix == ".rs" else text
        formatted = FORMAT.format_text(path, ported, keep_headers)
        if formatted != text:
            path.write_text(formatted)
            changed += 1
    return changed


def steps(repo, ref):
    out = []
    for line in git(repo, "log", "--reverse", "--format=%H%x1f%s", ref).splitlines():
        commit, subject = line.split("\x1f", 1)
        match = SUBJECT.match(subject)
        if match:
            out.append((commit, match.group(1), match.group(2)))
    return out


def series(repo, old, new, keep_headers, port=False):
    """Every step of `old`, formatted, as the same step on branch `new`."""
    base = git(repo, "rev-list", "--max-parents=0", old).strip()
    git(repo, "checkout", "-q", "-B", new, base)
    for commit, step_id, title in steps(repo, old):
        git(repo, "read-tree", "-u", "--reset", commit)
        changed = format_tree(repo, keep_headers, port)
        git(repo, "add", "-A", "-f")
        git(repo, "commit", "-q", "--allow-empty", "-m", f"step {step_id}: {title}")
        git(repo, "tag", "-f", f"{new}/{step_id}")
        print(f"step {step_id}: {changed} files formatted")


def apply_patch(directory, patch):
    subprocess.run(
        ["git", "apply", str(patch)],
        cwd=directory,
        check=True,
        capture_output=True,
        text=True,
    )


def patch_between(previous, current, patch):
    """A git patch from tree `previous` to tree `current`, in the format of docs/extensions."""
    with tempfile.TemporaryDirectory(prefix="viewer-extension-diff-") as temp:
        repo = Path(temp)
        git(repo, "init", "-q")
        shutil.copytree(previous, repo, dirs_exist_ok=True)
        git(repo, "add", "-A", "-f")
        git(repo, "commit", "-q", "--allow-empty", "-m", "previous")
        for path in list(repo.iterdir()):
            if path.name != ".git":
                shutil.rmtree(path) if path.is_dir() else path.unlink()
        shutil.copytree(current, repo, dirs_exist_ok=True)
        git(repo, "add", "-A", "-f")
        diff = git(repo, "diff", "--cached", "--no-renames", "--binary")
    lines = [line for line in diff.split("\n") if not line.startswith("index ")]
    patch.write_text("\n".join(lines))


RUNTIME = ("src", "Cargo.toml", "Cargo.lock", "index.html")


def viewer_snapshot(source, destination):
    """The runtime and build files of the crate: what the extension lessons change and check."""
    destination.mkdir(parents=True)
    for name in RUNTIME:
        path = source / name
        if path.is_dir():
            shutil.copytree(path, destination / name)
        elif path.is_file():
            shutil.copy2(path, destination / name)


def extensions(repo, ref, production, keep_headers):
    """Rebuild every extension patch from formatted trees; the integrated chain ends at production."""
    lessons = json.loads((EXTENSIONS / "lessons.json").read_text())["lessons"]
    with tempfile.TemporaryDirectory(prefix="viewer-extension-format-") as temp:
        work = Path(temp)
        original = work / "original"
        original.mkdir()
        git(
            repo,
            "worktree",
            "add",
            "-q",
            "--detach",
            str(work / "checkout"),
            ref,
        )
        try:
            for name in (
                "session_viewer",
                "session_rust",
                "session_proto",
                "session_cpp",
                "session_py",
                "bash",
            ):
                if (work / "checkout" / name).exists():
                    shutil.copytree(
                        work / "checkout" / name, original / name, symlinks=True
                    )
        finally:
            git(repo, "worktree", "remove", "--force", str(work / "checkout"))
        base = work / "base"
        viewer_snapshot(original / "session_viewer", base)
        format_tree(base, keep_headers)
        for lesson in lessons:
            current = work / f"{lesson['id']}-original"
            shutil.copytree(original, current, symlinks=True)
            previous = base
            for number, step in enumerate(lesson["steps"], 1):
                if step.get("kernel_patch"):
                    apply_patch(
                        current / "session_rust", EXTENSIONS / step["kernel_patch"]
                    )
                apply_patch(current / "session_viewer", EXTENSIONS / step["patch"])
                formatted = work / f"{lesson['id']}-{number}"
                if lesson.get("chapters") and number == len(lesson["steps"]):
                    viewer_snapshot(production, formatted)
                else:
                    viewer_snapshot(current / "session_viewer", formatted)
                format_tree(formatted, keep_headers)
                patch_between(previous, formatted, EXTENSIONS / step["patch"])
                print(f"{lesson['id']} step {number}: {step['patch']} rewritten")
                previous = formatted


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    make = sub.add_parser(
        "series", help="format every step of a materialized series onto a new branch"
    )
    make.add_argument("--repo", type=Path, required=True)
    make.add_argument("--old", default="series")
    make.add_argument("--new", default="formatted")
    make.add_argument("--keep-headers", action="store_true")
    make.add_argument("--port", action="store_true", help="also run port_math.py over every step")
    ext = sub.add_parser(
        "extensions", help="rebuild the docs/extensions patches from formatted trees"
    )
    ext.add_argument("--repo", type=Path, required=True)
    ext.add_argument(
        "--ref", default="step/21", help="the original checkpoint 21 the extension patches apply to"
    )
    ext.add_argument(
        "--production",
        type=Path,
        default=HERE.parent.parent,
        help="the viewer crate the integrated chain ends at",
    )
    ext.add_argument("--keep-headers", action="store_true")
    args = parser.parse_args()
    if args.command == "series":
        series(args.repo.resolve(), args.old, args.new, args.keep_headers, args.port)
    else:
        extensions(
            args.repo.resolve(), args.ref, args.production.resolve(), args.keep_headers
        )


if __name__ == "__main__":
    main()
