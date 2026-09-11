#!/usr/bin/env python3
"""Check that every `file.rs:LINE` citation in the guides still points where it says.

The four extending guides cite this code base by line, and a line moves whenever anything above
it is edited - a doc comment inserted in one commit silently invalidated 49 of them. A citation
that points at the wrong line is worse than none: it sends a reader to code that does not do
what the sentence says.

A citation passes when the file exists, the line is inside it, and at least one identifier
named in backticks nearby appears within a few lines of the target. Citations into paths this
repository does not have (the archived viewer) are counted and skipped, not failed.
"""
from __future__ import annotations
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
VIEWER = HERE.parent
ROOTS = [VIEWER, VIEWER.parent]
CITE = re.compile(r"`([\w./-]+\.(?:rs|wgsl)):(\d+)(?:-(\d+))?`")
IDENT = re.compile(r"`([A-Za-z_][\w:<>]*(?:::[\w<>]+)*)`")
WINDOW = 8


def resolve(path: str) -> Path | str | None:
    """The file a citation names. A bare `mod.rs` names four different files in this tree, so a
    bare name resolves only when the tree holds exactly one; otherwise the citation has to say
    which, and "ambiguous" comes back instead of a file."""
    for root in ROOTS:
        p = root / path
        if p.is_file():
            return p
    if "/" not in path:
        found = [q for q in (VIEWER / "src").rglob(path)]
        if len(found) == 1:
            return found[0]
        if len(found) > 1:
            return "ambiguous"
    return None


def nearby_identifiers(line: str, upto: int) -> list[str]:
    """The backticked names in this sentence before the citation: what the citation is about."""
    head = line[:upto]
    cut = max(head.rfind(". "), head.rfind("— "), head.rfind("; "))
    return [m.group(1) for m in IDENT.finditer(head[cut + 1:])]


def check(page: Path) -> tuple[int, int, list[str]]:
    hits = skipped = 0
    bad: list[str] = []
    for number, line in enumerate(page.read_text().splitlines(), 1):
        for m in CITE.finditer(line):
            path, first = m.group(1), int(m.group(2))
            last = int(m.group(3) or first)
            target = resolve(path)
            if target is None:
                skipped += 1
                continue
            if target == "ambiguous":
                bad.append(f"{page.name}:{number}: `{path}` names more than one file - write the path from src/")
                continue
            hits += 1
            body = target.read_text().splitlines()
            if last > len(body):
                bad.append(f"{page.name}:{number}: {path}:{first}-{last} but the file has {len(body)} lines")
                continue
            names = nearby_identifiers(line, m.start())
            whole = "\n".join(body)
            present = [n for n in names if n.split("::")[-1] in whole]
            if not present:
                # Nothing the sentence names is in the file at all: the guide is pointing at
                # where something NEW would go, not citing code that exists. Only the line is
                # checkable, and it already was.
                continue
            lo, hi = max(0, first - 1 - WINDOW), min(len(body), last + WINDOW)
            window = "\n".join(body[lo:hi])
            if not any(n.split("::")[-1] in window for n in present):
                bad.append(f"{page.name}:{number}: {path}:{first} is not where "
                           + " or ".join(present) + " is")
    return hits, skipped, bad


def main() -> int:
    pages = sorted(HERE.glob("extend-*.md"))
    hits = skipped = 0
    bad: list[str] = []
    for page in pages:
        h, s, b = check(page)
        hits, skipped, bad = hits + h, skipped + s, bad + b
    for line in bad:
        print(line)
    print(f"{'FAIL' if bad else 'PASS'} {hits} citations into this code base checked, "
          f"{skipped} into the archive skipped, {len(bad)} wrong")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
