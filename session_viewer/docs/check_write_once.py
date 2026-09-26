import difflib
import re
import sys
from pathlib import Path

LESSONS = Path(__file__).resolve().parent / "lessons"
SKIP = {"target", "dist", "node_modules", "__pycache__"}
KINDS = ("removed", "modified", "inserted")
MARKERS = False  # True also counts moved `--8<--` snippet markers, which the rendered page strips
MARKER = re.compile(
    r"^\s*(//|#|<!--|/\*)\s*--8<-- \[(start|end):[^\]]+\]\s*(-->|\*/)?\s*$"
)


def files(crate):
    return {
        path.relative_to(crate).as_posix()
        for path in crate.rglob("*")
        if path.is_file() and not SKIP.intersection(path.relative_to(crate).parts)
    }


def lines(path, rename=None):
    """(line number, text) pairs without snippet markers; the parent's own id reads as the child's; None for binary."""
    try:
        text = path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return None

    if rename:
        old, new = rename
        text = text.replace(f"lessons/{old}/", f"lessons/{new}/")
        text = re.sub(rf"([Cc]heckpoint ){re.escape(old)}\b", rf"\g<1>{new}", text)

    return [
        (number, line)
        for number, line in enumerate(text.splitlines(), 1)
        if MARKERS or not MARKER.match(line)
    ]


def violations(old, new):
    """(kind, line, text): removed or modified lines numbered in the parent, mid-file inserts without register: in the child."""
    found = []
    matcher = difflib.SequenceMatcher(
        None, [line for _, line in old], [line for _, line in new], autojunk=False
    )
    for tag, i1, i2, j1, j2 in matcher.get_opcodes():
        if tag == "equal":
            continue

        if tag == "insert":
            if i1 == len(old):
                continue

            found += [
                ("inserted", *new[j])
                for j in range(j1, j2)
                if "register:" not in new[j][1]
            ]
            continue

        found += [
            ("removed" if tag == "delete" else "modified", *old[i])
            for i in range(i1, i2)
        ]

    return found


def check(parent, lesson):
    """Violations per file name where lesson breaks write-once against parent."""
    report = {}
    for name in sorted(files(LESSONS / parent) & files(LESSONS / lesson)):
        old_path = LESSONS / parent / name
        new_path = LESSONS / lesson / name
        if old_path.read_bytes() == new_path.read_bytes():
            continue

        old = lines(old_path, (parent, lesson))
        new = lines(new_path)
        if old is None or new is None:
            report[name] = [("modified", 0, "binary file")]
            continue

        found = violations(old, new)
        if found:
            report[name] = found

    return report


total = 0
series = [
    row.split()
    for row in (LESSONS / "SERIES.txt").read_text().splitlines()
    if row.strip()
]
for lesson, parent, _page in series:
    if parent == "-":
        continue

    report = check(parent, lesson)
    count = sum(len(found) for found in report.values())
    total += count
    if not report:
        continue

    print(f"{parent} -> {lesson}: {count} lines in {len(report)} files")
    for name, found in report.items():
        kinds = ", ".join(
            f"{n} {kind}"
            for kind in KINDS
            if (n := sum(row[0] == kind for row in found))
        )
        print(
            f"  {name}: {kinds}; first {found[0][0]} at line {found[0][1]}: {found[0][2].strip()[:80]}"
        )

print(f"total: {total}")
sys.exit(1 if total else 0)

"""
description: write-once gate for the course - for each pair in lessons/SERIES.txt, a file both crates have may only
grow by lines appended at its end, or by single lines tagged `register:` inserted into a registration list.
Removed and modified lines are numbered in the parent file, inserted lines in the child file. Exits 1 on any violation.
Snippet markers are skipped (MARKERS = True counts them), and the parent's own id in `lessons/<id>/` and
`Checkpoint <id>` reads as the child's, so the per-crate dist dir and status line are no violation.

directory: cd ~/code/code_cpp/wood_research/session/session_viewer
run: python3 docs/check_write_once.py
"""
