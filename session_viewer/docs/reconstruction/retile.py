#!/usr/bin/env python3
"""Move `lines=A-B` piece boundaries in the lessons after a checkpoint file changed length.

A whole file typed in pieces is cut at line numbers. When history is rewritten (`fold.py`,
a split, a lean-up) the file at that step grows or shrinks and every later boundary shifts. This
aligns the old and new text of each such file (`git show <old>:<name>` against `<new>:<name>`
in the series repository) and rewrites each directive's range so the pieces still tile the new
file in order: a boundary inside a changed region moves to the end of that region. `hunks=`
directives are remapped the same way, from the hunk headers of the patch bytes the lessons matched
(`--old-patches`, a copy taken before `series_git.py regenerate`) to the regenerated patch: an old
hunk maps to every new hunk whose old-line span overlaps its own. Run `course_pages.py --audit` afterwards.
"""
import argparse
import difflib
from pathlib import Path
import re
import subprocess

HERE = Path(__file__).resolve().parent
DIRECTIVE = re.compile(r"^<!-- file: (\d\d[a-z]?) (\S+)(.*?) -->$", re.MULTILINE)
LINES = re.compile(r"lines=(\d+)-(\d+)")
HUNKS = re.compile(r"hunks=([\d,-]+)")
HEADER = re.compile(r"^@@ -(\d+)(?:,(\d+))? ", re.MULTILINE)
SECTION = re.compile(r"(?m)^(?=diff --git )")
SUBJECT = re.compile(r"^step (\d\d[a-z]?): ")


def commits(repo, ref):
    out = {}
    for line in subprocess.run(["git", "log", "--reverse", "--format=%H%x1f%s", ref], cwd=repo, check=True,
                               capture_output=True, text=True).stdout.splitlines():
        commit, subject = line.split("\x1f", 1)
        match = SUBJECT.match(subject)
        if match:
            out[match.group(1)] = commit
    return out


def show(repo, commit, name):
    result = subprocess.run(["git", "show", f"{commit}:{name}"], cwd=repo, capture_output=True, text=True)
    return result.stdout.split("\n") if result.returncode == 0 else None


def line_map(old, new):
    """old 1-based line -> new 1-based line for equal lines; a changed old line maps to the new
    line after the replacement (so a piece ending in a change ends after the change)."""
    mapping = {}
    for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, old, new, autojunk=False).get_opcodes():
        if tag == "equal":
            for offset in range(i2 - i1):
                mapping[i1 + offset + 1] = j1 + offset + 1
        else:
            for line in range(i1 + 1, i2 + 1):
                mapping[line] = j2
    return mapping


def hunk_spans(patch_text, name):
    """[(old_start, old_end)] of every hunk of `name` in a patch, 1-based inclusive."""
    for chunk in SECTION.split(patch_text):
        if chunk.startswith("diff --git ") and chunk.splitlines()[0].split(" b/", 1)[1] == name:
            spans = []
            for match in HEADER.finditer(chunk):
                start, count = int(match.group(1)), int(match.group(2) or 1)
                spans.append((start, start + max(count, 1) - 1))
            return spans
    return []


def parse_range(text):
    numbers = set()
    for part in text.split(","):
        if "-" in part:
            first, last = part.split("-")
            numbers.update(range(int(first), int(last) + 1))
        else:
            numbers.add(int(part))
    return sorted(numbers)


def format_range(numbers):
    return f"{numbers[0]}-{numbers[-1]}" if numbers == list(range(numbers[0], numbers[-1] + 1)) else ",".join(map(str, numbers))


def remap_hunks(repo, docs, old_patches, old_commits, new_commits, step, name, matches, text):
    """Rewrite hunks= ranges after the patch of `name` at `step` was re-cut."""
    previous = list(old_commits)[list(old_commits).index(step) - 1] if list(old_commits).index(step) else None
    old_patch = (old_patches / f"{step}.patch").read_text()
    new_patch = (docs / "reconstruction/patches" / f"{step}.patch").read_text()
    old_spans, new_spans = hunk_spans(old_patch, name), hunk_spans(new_patch, name)
    if old_spans == new_spans or not old_spans or not new_spans:
        return text
    mapping = {}
    if previous is not None:
        old_base, new_base = show(repo, old_commits[previous], name), show(repo, new_commits[previous], name)
        if old_base is not None and new_base is not None and old_base != new_base:
            mapping = line_map(old_base, new_base)
    used = set()
    for match in matches:
        old_numbers = parse_range(HUNKS.search(match.group(3)).group(1))
        new_numbers = []
        for number in old_numbers:
            start, end = old_spans[number - 1]
            start, end = mapping.get(start, start), mapping.get(end, end)
            for index, (new_start, new_end) in enumerate(new_spans, 1):
                if new_start <= end and start <= new_end and index not in used:
                    new_numbers.append(index)
                    used.add(index)
        if not new_numbers:
            continue
        rest = HUNKS.sub(f"hunks={format_range(new_numbers)}", match.group(3))
        text = text.replace(match.group(0), f"<!-- file: {step} {name}{rest} -->", 1)
    return text


def retile(repo, old_ref, new_ref, docs, old_patches):
    lessons = sorted(docs.glob("[0-9][0-9]*-*.md"))
    old_commits, new_commits = commits(repo, old_ref), commits(repo, new_ref)
    for lesson in lessons:
        text = lesson.read_text()
        original = text
        pieces, hunked = {}, {}
        for match in DIRECTIVE.finditer(text):
            step, name, rest = match.groups()
            if LINES.search(rest):
                pieces.setdefault((step, name), []).append(match)
            elif HUNKS.search(rest) and old_patches is not None:
                hunked.setdefault((step, name), []).append(match)
        for (step, name), matches in hunked.items():
            text = remap_hunks(repo, docs, old_patches, old_commits, new_commits, step, name, matches, text)
        changed = text != original
        for (step, name), matches in pieces.items():
            old, new = show(repo, old_commits[step], name), show(repo, new_commits[step], name)
            if old is None or new is None or old == new:
                continue
            if old and old[-1] == "":
                old.pop()
            if new and new[-1] == "":
                new.pop()
            mapping = line_map(old, new)
            ends = [int(LINES.search(m.group(3)).group(2)) for m in matches]
            new_ends = [len(new) if end == len(old) else mapping.get(end, end) for end in ends]
            new_ends[-1] = len(new)
            starts = [1] + [end + 1 for end in new_ends[:-1]]
            for match, first, last in zip(matches, starts, new_ends):
                rest = LINES.sub(f"lines={first}-{last}", match.group(3))
                text = text.replace(match.group(0), f"<!-- file: {step} {name}{rest} -->", 1)
            changed = True
            print(f"{lesson.name}: {name} at {step} retiled to {list(zip(starts, new_ends))}")
        if changed:
            lesson.write_text(text)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, required=True)
    parser.add_argument("--old", default="series", help="the branch the lessons currently match")
    parser.add_argument("--new", required=True, help="the rewritten branch")
    parser.add_argument("--docs", type=Path, default=HERE.parent, help="directory of the lesson Markdown")
    parser.add_argument("--old-patches", type=Path, help="copy of patches/ taken before regenerating; enables hunks= remapping")
    args = parser.parse_args()
    retile(args.repo.resolve(), args.old, args.new, args.docs.resolve(),
           args.old_patches.resolve() if args.old_patches else None)


if __name__ == "__main__":
    main()
