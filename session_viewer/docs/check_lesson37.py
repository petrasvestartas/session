import difflib
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LESSON = ROOT / "docs" / "lessons" / "37" / "src"
SOURCE = ROOT / "src"
SLASH = {".rs", ".wgsl", ".js", ".ts", ".css", ".c", ".h", ".cpp"}
HASH = {".toml", ".yaml", ".yml", ".sh", ".py"}


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

        if (
            rust
            and text[i] == "r"
            and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_"))
        ):
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


def hash_comments(text):
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


def html_comments(text):
    """Drop <!-- --> comments."""
    while (start := text.find("<!--")) >= 0:
        end = text.find("-->", start)
        text = text[:start] + (text[end + 3 :] if end >= 0 else "")

    return text


def code(path):
    """Lines without comments, trailing spaces or blank lines; None for binary."""
    try:
        text = path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return None

    if path.suffix in SLASH:
        text = slash(text, path.suffix == ".rs")
    elif path.suffix in HASH:
        text = hash_comments(text)
    elif path.suffix == ".html":
        text = html_comments(text)

    return [line.rstrip() for line in text.splitlines() if line.strip()]


def files(root):
    return {
        path.relative_to(root).as_posix() for path in root.rglob("*") if path.is_file()
    }


lesson = files(LESSON)
source = files(SOURCE)
differ = []
for name in sorted(lesson & source):
    old = code(LESSON / name)
    new = code(SOURCE / name)
    if old is None or new is None:
        if (LESSON / name).read_bytes() != (SOURCE / name).read_bytes():
            differ.append((name, 0, 0))
        continue

    if old == new:
        continue

    delta = [
        line
        for line in difflib.unified_diff(old, new, lineterm="", n=0)
        if line[:1] in "+-" and line[:3] not in ("+++", "---")
    ]
    differ.append(
        (
            name,
            sum(line[0] == "-" for line in delta),
            sum(line[0] == "+" for line in delta),
        )
    )

for name in sorted(source - lesson):
    print(f"only in src: {name}")

for name in sorted(lesson - source):
    print(f"only in lessons/37: {name}")

for name, removed, added in differ:
    print(f"differs: {name}: {removed} lines only in lessons/37, {added} only in src")

print(
    f"files: {len(source - lesson)} only in src, {len(lesson - source)} only in lessons/37, {len(differ)} of {len(lesson & source)} shared differ"
)
print(
    f"lines: {sum(row[1] for row in differ)} only in lessons/37, {sum(row[2] for row in differ)} only in src (shared files, comments stripped)"
)
sys.exit(1 if differ or lesson != source else 0)

"""
description: lessons/37/src must equal session_viewer/src once comments are stripped (teaching comments may differ).
Compares every file with //, /* */, # and <!-- --> comments, trailing spaces and blank lines removed; lists files
found on one side only and the differing line counts. Exits 1 on any difference.

directory: cd ~/code/code_cpp/wood_research/session/session_viewer
run: python3 docs/check_lesson37.py
"""
