import sys
from collections import Counter
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from cut import KEEP, LESSONS, strip, table

BUDGET = 250
SKIP = ("tests/", "examples/", "assets/", "Cargo.lock")


def typed(lesson):
    """{path: Counter of code lines} of the viewer code a reader types for crate `lesson`."""
    root = LESSONS / lesson
    out = {}
    for path in root.rglob("*"):
        rel = path.relative_to(root).as_posix()
        if not path.is_file() or KEEP.intersection(path.parts) or rel.startswith(SKIP):
            continue

        lines = strip(rel, path.read_bytes())
        if isinstance(lines, list):
            out[rel] = Counter(lines)

    return out


def added(lesson, parent):
    """{path: lines} crate `lesson` adds to `parent`, comments and blank lines not counted."""
    new, old = typed(lesson), typed(parent) if parent else {}
    out = {}
    for path, lines in new.items():
        count = sum((lines - old.get(path, Counter())).values())
        if count:
            out[path] = count

    return out


def main():
    detail = set(sys.argv[1:])
    over = 0
    for lesson, parent, _page in table("SERIES.txt", 3):
        files = added(lesson, None if parent == "-" else parent)
        total = sum(files.values())
        flag = "  over" if total > BUDGET else ""
        over += total > BUDGET
        print(f"{lesson:>4} {total:5d}{flag}")
        if lesson in detail or "all" in detail:
            for path, count in sorted(files.items(), key=lambda x: -x[1]):
                print(f"         {count:5d} {path}")

    print(f"over budget: {over}")
    return 1 if over else 0


if __name__ == "__main__":
    sys.exit(main())

"""
description: typed lines per lesson - the viewer code (Rust, WGSL, toml, html; tests, examples, assets and Cargo.lock
excluded) a crate adds to its parent in SERIES.txt, comments and blank lines not counted. A lesson is a newbie's hour,
at most 250 lines. Prints one row per lesson and exits 1 when any lesson is over. `check_budget.py 04a` (or `all`)
also lists the files a lesson adds, largest first.

directory: cd ~/code/code_cpp/wood_research/session/session_viewer
run: python3 docs/check_budget.py [lesson ...|all]
"""
