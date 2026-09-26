import argparse
import re
import shutil
import sys
from pathlib import Path

DOCS = Path(__file__).resolve().parent
LESSONS = DOCS / "lessons"
MASTER = "37"
KEEP = {"target", "dist"}
MARKER = re.compile(
    r"^\s*(//|#|<!--|/\*)\s*--8<-- \[(start|end):([^\]]+)\]\s*(-->|\*/)?\s*$"
)
REGISTER = re.compile(r"register:([A-Za-z0-9_-]+)")
LESSON_NAME = re.compile(r"^(\d{2,3}[a-z]?)(?:-|$)")
SLASH = {".rs", ".wgsl", ".js", ".cjs", ".ts", ".css", ".c", ".h", ".cpp"}
HASH = {".toml", ".yaml", ".yml", ".sh", ".py"}


def table(name, width):
    """Rows of a whitespace-separated list file, `#` comments and blank lines skipped."""
    rows = []
    for number, row in enumerate((LESSONS / name).read_text().splitlines(), 1):
        row = row.split("#", 1)[0].split()
        if not row:
            continue

        if len(row) != width:
            sys.exit(f"{name}:{number}: expected {width} fields, got {row}")

        rows.append(row)

    return rows


class Course:
    """The series order, each file's first lesson and each registration line's lesson."""

    def __init__(self):
        self.series = [row[0] for row in table("SERIES.txt", 3)]
        self.order = {lesson: i for i, lesson in enumerate(self.series)}
        self.first = {}
        for path, lesson in table("FILES.txt", 2):
            self.known(lesson, f"FILES.txt {path}")
            self.first[path] = lesson

        self.register = {}
        for path, tag, lesson in table("REGISTER.txt", 3):
            self.known(lesson, f"REGISTER.txt {path} {tag}")
            if (path, tag) in self.register:
                sys.exit(f"REGISTER.txt: {path} {tag} listed twice")

            self.register[(path, tag)] = lesson

    def known(self, lesson, where):
        if lesson not in self.order:
            sys.exit(f"{where}: lesson {lesson} is not in SERIES.txt")

    def later(self, a, b):
        """The later of two lessons."""
        return a if self.order[a] >= self.order[b] else b

    def lessons(self, path, lines):
        """The lesson of every line of master file `path`: section, registration tag or the file's first lesson."""
        first = self.first[path]
        stack = [first]
        out = []
        for number, line in enumerate(lines, 1):
            marker = MARKER.match(line)
            if marker:
                kind, name = marker.group(2), marker.group(3)
                named = LESSON_NAME.match(name)
                if kind == "start":
                    lesson = (
                        named.group(1)
                        if named and named.group(1) in self.order
                        else stack[-1]
                    )
                    if self.later(lesson, stack[-1]) != lesson:
                        sys.exit(
                            f"{path}:{number}: section {name} is earlier than the lines around it ({stack[-1]})"
                        )

                    stack.append(lesson)
                    out.append(lesson)
                else:
                    if len(stack) == 1:
                        sys.exit(f"{path}:{number}: [end:{name}] without a start")

                    out.append(stack.pop())

                continue

            lesson = stack[-1]
            tag = REGISTER.search(line)
            if tag:
                key = (path, tag.group(1))
                if key not in self.register:
                    sys.exit(
                        f"{path}:{number}: register:{tag.group(1)} is not in REGISTER.txt"
                    )

                lesson = self.later(self.register[key], lesson)

            out.append(lesson)

        if len(stack) != 1:
            sys.exit(f"{path}: section left open")

        # an attribute line directly above a registration line joins its lesson
        for i in range(len(lines) - 2, -1, -1):
            if lines[i].lstrip().startswith("#[") and REGISTER.search(lines[i + 1]):
                out[i] = self.later(out[i], out[i + 1])

        return out

    def compose(self, lesson):
        """{path: bytes} of crate `lesson`, cut from the master."""
        master = LESSONS / MASTER
        crate = {}
        for path, first in self.first.items():
            if self.order[first] > self.order[lesson]:
                continue

            data = (master / path).read_bytes()
            try:
                text = data.decode("utf-8")
            except UnicodeDecodeError:
                crate[path] = data
                continue

            if lesson != MASTER:
                text = self.cut(path, text, lesson)

            crate[path] = text.encode("utf-8")

        return crate

    def cut(self, path, text, lesson):
        """The lines of `text` up to `lesson`, blank runs left by a cut collapsed, the master id renamed."""
        if not text:
            return text

        lines = text.split("\n")
        end = lines[-1] == ""
        if end:
            lines.pop()

        keep = []
        gap = False
        for line, owner in zip(lines, self.lessons(path, lines)):
            if self.order[owner] > self.order[lesson]:
                gap = True
                continue

            if gap and not line.strip() and keep and not keep[-1].strip():
                continue

            gap = False
            keep.append(line)

        while gap and keep and not keep[-1].strip():
            keep.pop()

        text = "\n".join(keep) + ("\n" if end else "")
        text = text.replace(f"lessons/{MASTER}/", f"lessons/{lesson}/")
        return re.sub(rf"([Cc]heckpoint ){MASTER}\b", rf"\g<1>{lesson}", text)

    def missing(self):
        """Master files FILES.txt does not list, and listed files the master does not have."""
        master = LESSONS / MASTER
        on_disk = {
            p.relative_to(master).as_posix()
            for p in master.rglob("*")
            if p.is_file() and not KEEP.intersection(p.relative_to(master).parts)
        }
        return sorted(on_disk - set(self.first)), sorted(set(self.first) - on_disk)


def write(root, crate):
    """Replace the crate directory `root` with `crate`, keeping build output."""
    root.mkdir(parents=True, exist_ok=True)
    for entry in root.iterdir():
        if entry.name in KEEP:
            continue

        shutil.rmtree(entry) if entry.is_dir() else entry.unlink()

    for path, data in crate.items():
        target = root / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(data)


def strip(path, data):
    """Code lines without comments, trailing spaces or blank lines; the raw bytes for a binary file."""
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError:
        return data

    suffix = Path(path).suffix
    if suffix in SLASH:
        text = slash(text, suffix == ".rs")
    elif suffix in HASH:
        text = hashes(text)
    elif suffix in (".html", ".md"):
        text = re.sub(r"<!--.*?-->", "", text, flags=re.S)

    return [line.rstrip() for line in text.splitlines() if line.strip()]


def slash(text, rust):
    """Drop // and /* */ comments, keeping strings, raw strings and char literals intact."""
    out = []
    i = 0
    while i < len(text):
        if text.startswith("//", i):
            i = text.find("\n", i)
            i = len(text) if i < 0 else i
            continue

        if text.startswith("/*", i):
            depth = 0
            while i < len(text):
                if text.startswith("/*", i):
                    depth += 1 if rust or depth == 0 else 0
                    i += 2
                elif text.startswith("*/", i):
                    depth -= 1
                    i += 2
                    if depth == 0:
                        break
                else:
                    i += 1
            continue

        raw = (
            rust
            and text[i] == "r"
            and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_"))
        )
        if raw:
            j = i + 1
            while j < len(text) and text[j] == "#":
                j += 1
            if j < len(text) and text[j] == '"':
                close = '"' + "#" * (j - i - 1)
                end = text.find(close, j + 1)
                end = len(text) if end < 0 else end + len(close)
                out.append(text[i:end])
                i = end
                continue

        if text[i] == '"':
            j = i + 1
            while j < len(text) and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            out.append(text[i : j + 1])
            i = j + 1
            continue

        if rust and text[i] == "'":
            if text.startswith("\\", i + 1):
                end = text.find("'", i + 3) + 1
                out.append(text[i:end])
                i = end
                continue

            if i + 2 < len(text) and text[i + 2] == "'":
                out.append(text[i : i + 3])
                i += 3
                continue

        out.append(text[i])
        i += 1

    return "".join(out)


def hashes(text):
    """Drop # comments outside quotes."""
    out = []
    for line in text.splitlines():
        quote = None
        for i, char in enumerate(line):
            if quote:
                quote = None if char == quote and line[i - 1] != "\\" else quote
            elif char in "\"'":
                quote = char
            elif char == "#":
                line = line[:i]
                break

        out.append(line)

    return "\n".join(out)


def check(course, baseline):
    """Differences between every composed crate and its raw cut in `baseline`, comments stripped."""
    total = 0
    for lesson in course.series:
        crate = course.compose(lesson)
        root = baseline / lesson
        if not root.is_dir():
            print(f"{lesson}: no baseline at {root}")
            total += 1
            continue

        saved = {
            p.relative_to(root).as_posix(): p.read_bytes()
            for p in root.rglob("*")
            if p.is_file() and not KEEP.intersection(p.relative_to(root).parts)
        }
        found = [f"only in the cut: {path}" for path in sorted(set(crate) - set(saved))]
        found += [
            f"only in the baseline: {path}" for path in sorted(set(saved) - set(crate))
        ]
        for path in sorted(set(crate) & set(saved)):
            if strip(path, crate[path]) != strip(path, saved[path]):
                found.append(f"differs: {path}")

        total += len(found)
        for line in found:
            print(f"{lesson}: {line}")

    print(f"total: {total}")
    return total


def main():
    parser = argparse.ArgumentParser(
        description="Cut every lesson crate from the master docs/lessons/37."
    )
    parser.add_argument(
        "lessons",
        nargs="*",
        help="only these lessons (default: every one but the master)",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="compare every cut with the raw-cut baseline, comments stripped",
    )
    parser.add_argument(
        "--baseline", type=Path, default=Path.home() / ".cache/viewer-push/recut/raw"
    )
    parser.add_argument(
        "--out",
        type=Path,
        help="write the crates under this directory instead of docs/lessons",
    )
    args = parser.parse_args()
    course = Course()

    unlisted, absent = course.missing()
    for path in unlisted:
        print(f"not in FILES.txt: {path}")
    for path in absent:
        print(f"listed in FILES.txt, missing in lessons/{MASTER}: {path}")
    if unlisted or absent:
        return 1

    if args.check:
        return 1 if check(course, args.baseline) else 0

    for lesson in args.lessons or [x for x in course.series if x != MASTER]:
        course.known(lesson, "argument")
        write((args.out or LESSONS) / lesson, course.compose(lesson))

    return 0


if __name__ == "__main__":
    sys.exit(main())

"""
description: cut every lesson crate from the master docs/lessons/37 (the viewer src plus teaching comments and
snippet markers). A line belongs to lesson NN when it sits in a section `NN` or `NN-<slug>`, or carries
`register:<tag>` (lesson from REGISTER.txt; an attribute line directly above follows it); every other line belongs to
its file's first lesson in FILES.txt. Crate N is every file with first lesson <= N, keeping its lines of lesson <= N,
with blank runs left by a cut collapsed and `lessons/37/`, `Checkpoint 37` renamed. --check compares every cut,
comments stripped, with the raw cuts saved under ~/.cache/viewer-push/recut/raw/<id>/.

directory: cd ~/code/code_cpp/wood_research/session/session_viewer
run: python3 docs/cut.py [lesson ...]; python3 docs/cut.py --check
"""
