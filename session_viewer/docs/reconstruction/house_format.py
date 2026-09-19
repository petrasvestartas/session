#!/usr/bin/env python3
"""House style for the viewer sources the course shows: Rust, WGSL and the page.

The rules are the ones the session reviewer applies to the kernels, under each language's own
syntax: no file header, a blank line between items and around every loop or branch, a field's
comment on the field's own line, one field and one statement per line, no block body on the
header line. Rust goes through rustfmt afterwards, so what it prints is the last word on
wrapping. The result is deterministic and idempotent, which is what lets every checkpoint of the
course be formatted independently and still diff cleanly against its neighbours.

Usage:
    python3 house_format.py <file or directory>...     # format in place
    python3 house_format.py --check <paths>...         # exit 1 when a file would change
    python3 house_format.py --keep-headers <paths>...  # leave file header comments alone
"""

import argparse
from pathlib import Path
import re
import subprocess
import sys

ITEM = re.compile(
    r"^(pub(\([^)]*\))?\s+)?(async\s+|unsafe\s+|const\s+|extern\s+\"[^\"]*\"\s+)*"
    r"(fn|struct|enum|union|impl|trait|type|const|static|mod|macro_rules!)\b"
)
CONTROL_RUST = re.compile(r"^('\w+:\s*)?(if|for|while|loop|match)\b")
CONTROL_C = re.compile(r"^(if|for|while|loop|switch|do)\b")
FIELD_OWNER = re.compile(r"^(pub(\([^)]*\))?\s+)?(struct|enum|union)\s+\w+.*\{\s*$")
CLOSER = re.compile(r"^\}")


def indent_of(line):
    return len(line) - len(line.lstrip(" "))


def strip_header(lines, marker):
    """Drop the comment block that opens the file, and the blank lines under it."""
    at = 0
    while at < len(lines) and lines[at].startswith(marker):
        at += 1
    if at == 0:
        return lines
    while at < len(lines) and not lines[at].strip():
        at += 1
    return lines[at:]


def blank_before(lines, index, out):
    """Append a blank line unless the previous emitted line already separates."""
    if not out:
        return
    previous = out[-1].rstrip()
    if not previous or previous.endswith(("{", "(", "[", "=>", ",")):
        return
    out.append("")


def is_attribute(line):
    """A Rust `#[...]` or a WGSL stage attribute on its own line."""
    stripped = line.lstrip()
    return stripped.startswith("#[") or (stripped.startswith("@") and not stripped.endswith(";"))


def is_comment(line):
    stripped = line.lstrip()
    return stripped.startswith("//") and not stripped.startswith("///")


def is_doc(line):
    return line.lstrip().startswith("///")


def lead_start(lines, index, comments=True):
    """The first line of the attributes and comments that sit directly above `index`."""
    start = index
    while start > 0:
        above = lines[start - 1]
        if is_attribute(above) or is_doc(above) or (comments and is_comment(above)):
            start -= 1
        elif above.rstrip().endswith("]") and attribute_opener(lines, start - 1) is not None:
            start = attribute_opener(lines, start - 1)
        else:
            break
    return start


def attribute_opener(lines, index):
    """The line where a `#[...]` that closes on `index` opens, or None when it is not one."""
    for probe in range(index, max(-1, index - 12), -1):
        stripped = lines[probe].lstrip()
        if stripped.startswith("#["):
            return probe
        if not stripped or stripped.startswith(("}", "//")) or stripped.endswith((";", "{")):
            return None
    return None


def separate_items(lines):
    """A blank line between items, including a fn after `use` and one impl after another."""
    leads = set()
    for index, line in enumerate(lines):
        stripped = line.strip()
        if not ITEM.match(stripped):
            continue
        if stripped.startswith(("mod ", "pub mod ")) and stripped.endswith(";"):
            continue
        if (
            stripped.startswith(("type ", "pub type "))
            and stripped.endswith(";")
            and index > 0
            and lines[index - 1].strip().startswith(("type ", "pub type "))
        ):
            continue
        leads.add(lead_start(lines, index))
    out = []
    for index, line in enumerate(lines):
        if index in leads:
            blank_before(lines, index, out)
        out.append(line)
    return out


def block_end(lines, start, indent):
    """Index of the `}` that closes the block opened at `start`, following `else` chains."""
    index = start
    while index < len(lines) and not lines[index].rstrip().endswith("{"):
        index += 1
    while True:
        index += 1
        while index < len(lines) and not (
            indent_of(lines[index]) == indent and CLOSER.match(lines[index].lstrip())
        ):
            index += 1
        if index >= len(lines):
            return len(lines) - 1
        closing = lines[index].strip()
        if closing.endswith("{") and "else" in closing:
            continue
        return index


def separate_control(lines, control):
    """A blank line before and after every statement-level loop, branch and match."""
    starts = set()
    ends = set()
    for index, line in enumerate(lines):
        stripped = line.strip()
        if not control.match(stripped):
            continue
        if stripped.startswith("do") and not stripped.startswith("do {"):
            continue
        if "{" not in stripped and not lines_open_later(lines, index):
            continue
        starts.add(lead_start(lines, index))
        if stripped.endswith("{") or "{" not in stripped:
            ends.add(block_end(lines, index, indent_of(line)))
        elif stripped.endswith(("}", "};")):
            ends.add(index)
    out = []
    for index, line in enumerate(lines):
        if index in starts:
            blank_before(lines, index, out)
        out.append(line)
        if index in ends and index + 1 < len(lines):
            following = lines[index + 1].strip()
            if following and not following.startswith(
                ("}", ")", "]", ".", "else", "?")
            ):
                out.append("")
    return out


def lines_open_later(lines, index):
    """A condition wrapped over several lines opens its block on a later line."""
    indent = indent_of(lines[index])
    for probe in range(index + 1, min(index + 12, len(lines))):
        line = lines[probe]
        if line.rstrip().endswith("{"):
            return True
        if line.strip() and indent_of(line) <= indent:
            return False
    return False


def field_comments(lines):
    """Struct fields and enum variants carry their doc as a comment on the right."""
    out = []
    index = 0
    while index < len(lines):
        line = lines[index]
        if not FIELD_OWNER.match(line.strip()):
            out.append(line)
            index += 1
            continue
        indent = indent_of(line)
        out.append(line)
        index += 1
        docs = []
        while index < len(lines):
            line = lines[index]
            stripped = line.strip()
            if indent_of(line) == indent and stripped.startswith("}"):
                out.append(line)
                index += 1
                break
            if stripped.startswith("///"):
                docs.append(stripped[3:].strip())
                index += 1
                continue
            if not docs or not stripped or stripped.startswith("#["):
                out.append(line)
                index += 1
                continue
            text = " ".join(docs)
            docs = []
            if not text.endswith((".", "!", "?", ":")):
                text += "."
            if "//" in stripped:
                out.append(line)
            else:
                out.append(f"{line.rstrip()} // {text}")
            index += 1
    return out


def attach_attributes(lines):
    """No blank line between an attribute and the item it decorates."""
    out = []
    for index, line in enumerate(lines):
        if not line.strip() and out and index + 1 < len(lines) and lines[index + 1].strip():
            previous = out[-1]
            if is_attribute(previous) or (previous.rstrip().endswith("]") and attribute_opener(lines, index - 1) is not None):
                continue
        out.append(line)
    return out


def rustfmt(text):
    result = subprocess.run(
        ["rustfmt", "--edition", "2024", "--emit", "stdout"],
        input=text,
        capture_output=True,
        text=True,
    )
    if result.returncode:
        raise SystemExit(result.stderr)
    return result.stdout


def format_rust(text, keep_headers):
    lines = text.split("\n")
    if not keep_headers:
        lines = strip_header(lines, "//!")
    lines = field_comments(lines)
    lines = attach_attributes(lines)
    lines = separate_items(lines)
    lines = separate_control(lines, CONTROL_RUST)
    return rustfmt("\n".join(lines))


def split_statements(body):
    """Statements of a one-line block, split on `;` outside strings and brackets."""
    parts = []
    depth = 0
    quote = None
    start = 0
    for at, char in enumerate(body):
        if quote:
            if char == quote and body[at - 1] != "\\":
                quote = None
        elif char in "\"'`":
            quote = char
        elif char in "([{":
            depth += 1
        elif char in ")]}":
            depth -= 1
        elif char == ";" and depth == 0:
            parts.append(body[start : at + 1].strip())
            start = at + 1
    tail = body[start:].strip()
    if tail:
        parts.append(tail)
    return parts


def matching_brace(line, open_at):
    depth = 0
    quote = None
    for at in range(open_at, len(line)):
        char = line[at]
        if quote:
            if char == quote and line[at - 1] != "\\":
                quote = None
        elif char in "\"'`":
            quote = char
        elif char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return at
    return -1


def expand_blocks(lines, unit):
    """`head { a; b; } tail` becomes the head, one statement per line, the closer, the tail."""
    out = []
    pending = list(lines)
    while pending:
        line = pending.pop(0)
        stripped = line.strip()
        if stripped.startswith("//") or stripped.startswith("/*"):
            out.append(line)
            continue
        open_at = line.find("{")
        if open_at < 0:
            out.append(line)
            continue
        close_at = matching_brace(line, open_at)
        inner = line[open_at + 1 : close_at].strip() if close_at > 0 else ""
        if (
            close_at < 0
            or ";" not in inner
            or "'" in line[:open_at]
            and line.count("'") % 2
        ):
            out.append(line)
            continue
        indent = line[: indent_of(line)]
        head = line[:open_at].rstrip()
        tail = line[close_at + 1 :].strip()
        out.append(f"{head} {{" if head.strip() else f"{indent}{{")
        for statement in split_statements(inner):
            out.append(f"{indent}{unit}{statement}")
        if tail:
            pending.insert(0, f"{indent}}} {tail}")
        else:
            out.append(f"{indent}}}")
    return out


def one_field_per_line(lines):
    """WGSL struct members, one per line."""
    out = []
    inside = False
    for line in lines:
        stripped = line.strip()
        if re.match(r"^struct\s+\w+\s*\{\s*$", stripped):
            inside = True
            out.append(line)
            continue
        if inside and stripped.startswith("}"):
            inside = False
            out.append(line)
            continue
        if (
            inside
            and stripped.count(",") > 1
            and "//" not in stripped
            and "(" not in stripped
        ):
            indent = line[: indent_of(line)]
            for field in stripped.rstrip(",").split(","):
                out.append(f"{indent}{field.strip()},")
            continue
        out.append(line)
    return out


def space_commas(line):
    """`a,b` becomes `a, b`; WGSL has no strings, so every comma is code."""
    return re.sub(r",(?=\S)", ", ", line)


def stage_on_own_line(lines):
    """`@vertex fn` becomes the stage attribute above the function."""
    out = []
    for line in lines:
        match = re.match(
            r"^(@(?:vertex|fragment|compute)(?:\s+@workgroup_size\([^)]*\))?)\s+(fn\s.*)$",
            line.strip(),
        )
        if match:
            out.append(match.group(1))
            out.append(match.group(2))
        else:
            out.append(line)
    return out


def format_wgsl(text, keep_headers):
    lines = text.split("\n")
    if not keep_headers:
        lines = strip_header(lines, "//")
    lines = [
        space_commas(line) if not line.lstrip().startswith("//") else line
        for line in lines
    ]
    lines = expand_wgsl_structs(lines)
    lines = one_field_per_line(lines)
    lines = stage_on_own_line(lines)
    lines = expand_blocks(lines, "    ")
    lines = separate_wgsl_items(lines)
    lines = separate_control(lines, CONTROL_C)
    return "\n".join(collapse_blanks(lines))


def expand_wgsl_structs(lines):
    """`struct S { a: f32, b: f32 };` becomes one member per line."""
    out = []
    for line in lines:
        match = re.match(r"^(\s*)(struct\s+\w+)\s*\{(.*)\}\s*;?\s*$", line)
        if match and match.group(3).strip():
            out.append(f"{match.group(1)}{match.group(2)} {{")
            for field in match.group(3).split(","):
                if field.strip():
                    out.append(f"{match.group(1)}    {field.strip()},")
            out.append(f"{match.group(1)}}}")
        else:
            out.append(line)
    return out


WGSL_ITEM = re.compile(
    r"^(@|fn\s|struct\s|const\s|var\b|override\s|alias\s|enable\s|requires\s)"
)


def separate_wgsl_items(lines):
    """A blank line between functions and structs; bindings and constants may stay in runs."""
    leads = set()
    for index, line in enumerate(lines):
        stripped = line.strip()
        if indent_of(line) or not WGSL_ITEM.match(stripped):
            continue
        run = stripped.startswith(
            (
                "@group",
                "const ",
                "var<",
                "var ",
                "override ",
                "alias ",
                "enable ",
                "requires ",
            )
        )
        if (
            run
            and index > 0
            and lines[index - 1]
            .strip()
            .startswith(
                (
                    "@group",
                    "const ",
                    "var<",
                    "var ",
                    "override ",
                    "alias ",
                    "enable ",
                    "requires ",
                )
            )
        ):
            continue
        leads.add(lead_start(lines, index))
    out = []
    for index, line in enumerate(lines):
        if index in leads:
            blank_before(lines, index, out)
        out.append(line)
    return out


def collapse_blanks(lines):
    out = []
    for line in lines:
        if not line.strip() and out and not out[-1].strip():
            continue
        out.append(line.rstrip())
    while out and not out[-1]:
        out.pop()
    return out + [""]


def format_css(text, indent):
    """One rule per block, one declaration per line, `property: value;`."""
    out = []
    text = text.strip()
    position = 0
    while position < len(text):
        comment = re.match(r"\s*/\*.*?\*/", text[position:], re.S)
        if comment:
            for line in comment.group(0).strip().split("\n"):
                out.append(f"{indent}{line.strip()}")
            position += comment.end()
            continue
        open_at = text.find("{", position)
        if open_at < 0:
            break
        close_at = text.find("}", open_at)
        selector = re.sub(r",\s*", ", ", " ".join(text[position:open_at].split()))
        out.append(f"{indent}{selector} {{")
        body = text[open_at + 1 : close_at]
        for declaration in split_statements(body):
            declaration = declaration.rstrip(";").strip()
            comment_at = declaration.find("/*")
            if comment_at == 0:
                out.append(f"{indent}  {declaration}")
                continue
            name, _, value = declaration.partition(":")
            value = re.sub(r",(?=\S)", ", ", " ".join(value.split()))
            out.append(f"{indent}  {name.strip()}: {value};")
        out.append(f"{indent}}}")
        position = close_at + 1
    return "\n".join(out)


def format_js(text, indent):
    lines = [line.rstrip() for line in text.strip("\n").split("\n")]
    base = min((indent_of(line) for line in lines if line.strip()), default=0)
    lines = [line[base:] if line.strip() else "" for line in lines]
    lines = expand_blocks(lines, "  ")
    lines = split_multi_statements(lines)
    lines = [re.sub(r"\{(\w+):(\w+)\}", r"{ \1: \2 }", line) for line in lines]
    lines = reindent_js(lines)
    lines = separate_functions(lines)
    lines = separate_control(lines, CONTROL_C)
    return "\n".join(
        f"{indent}{line}" if line else "" for line in collapse_blanks(lines)[:-1]
    )


def reindent_js(lines):
    """Two spaces per brace depth; a line that continues the previous statement gets two more."""
    out = []
    depth = 0
    hanging = False
    previous = ""
    for line in lines:
        stripped = line.strip()
        if not stripped:
            out.append("")
            continue
        level = depth - (1 if stripped.startswith("}") else 0)
        if stripped.startswith("//"):
            out.append("  " * level + stripped)
            continue
        extra = 2 if hanging else 0
        if HEADER_ONLY.match(previous):
            extra = 2
        out.append("  " * level + " " * extra + stripped)
        depth += stripped.count("{") - stripped.count("}")
        hanging = stripped.endswith(("(", ",", "+", "&&", "||", "=")) or (stripped.startswith("'") and not stripped.endswith(";"))
        previous = stripped
    return out


HEADER_ONLY = re.compile(r"^(if|for|while)\s*\(.*\)$")


HEADER_BODY = re.compile(r"^(if|for|while)(\s*)\((.*)\)\s+([^{].*;)$")


def matching_paren(line, open_at):
    depth = 0
    for at in range(open_at, len(line)):
        if line[at] == "(":
            depth += 1
        elif line[at] == ")":
            depth -= 1
            if depth == 0:
                return at
    return -1


def split_multi_statements(lines):
    """Two statements on one line become two lines."""
    out = []
    for line in lines:
        stripped = line.strip()
        if stripped.startswith("//"):
            out.append(line)
            continue
        indent = line[: indent_of(line)]
        header = HEADER_BODY.match(stripped)
        if header and matching_paren(stripped, header.start(3) - 1) == header.end(3):
            out.append(f"{indent}{header.group(1)} ({header.group(3)})")
            out.append(f"{indent}    {header.group(4)}")
            continue
        parts = split_statements(stripped)
        if len(parts) > 1:
            out.extend(f"{indent}{part}" for part in parts)
        else:
            out.append(line)
    return out


def separate_functions(lines):
    leads = set()
    for index, line in enumerate(lines):
        stripped = line.strip()
        if indent_of(line) == 0 and re.match(r"^(async\s+)?function\b", stripped):
            leads.add(lead_start(lines, index))
    out = []
    for index, line in enumerate(lines):
        if index in leads:
            blank_before(lines, index, out)
        out.append(line)
        if line.startswith("}") and index + 1 < len(lines) and lines[index + 1].strip():
            out.append("")
    return out


INLINE_TAGS = {"p", "button", "a", "span", "b", "i", "em", "strong", "code", "output"}
VOID_TAGS = {"meta", "link", "br", "hr", "img", "input"}


def format_html(text, keep_headers):
    """One tag per line, nested two spaces, style and script blocks formatted as CSS and JS."""
    tokens = re.split(
        r"(<style[^>]*>.*?</style>|<script[^>]*>.*?</script>|<!--.*?-->|<[^>]+>)",
        text,
        flags=re.S,
    )
    out = []
    depth = 0
    opened = []
    for token in tokens:
        if not token.strip():
            continue
        indent = "  " * depth
        if token.startswith("<style"):
            head, _, rest = token.partition(">")
            body = rest[: -len("</style>")]
            out.append(f"{indent}{head}>")
            out.append(format_css(body, indent + "  "))
            out.append(f"{indent}</style>")
        elif token.startswith("<script"):
            head, _, rest = token.partition(">")
            body = rest[: -len("</script>")]
            out.append(f"{indent}{head}>")
            out.append(format_js(body, indent + "  "))
            out.append(f"{indent}</script>")
        elif token.startswith("<!--"):
            for line in token.split("\n"):
                out.append(f"{indent}{line.strip()}")
        elif token.startswith("</"):
            depth = max(0, depth - 1)
            if opened and opened[-1] == len(out) - 1:
                out[-1] += token
            else:
                out.append(f"{'  ' * depth}{token}")
            if opened:
                opened.pop()
        elif token.startswith("<"):
            name = re.match(r"<!?(\w+)", token).group(1).lower()
            token = re.sub(
                r'style="([^"]*)"',
                lambda m: 'style="' + inline_style(m.group(1)) + '"',
                token,
            )
            out.append(f"{indent}{token}")
            if (
                name not in VOID_TAGS
                and not token.endswith("/>")
                and name != "!doctype"
                and name != "doctype"
            ):
                depth += 1
                opened.append(len(out) - 1)
        else:
            out[-1] += token.strip()
    return "\n".join(out) + "\n"


def inline_style(style):
    return " ".join(
        f"{name.strip()}: {value.strip()};"
        for name, _, value in (
            declaration.partition(":")
            for declaration in style.split(";")
            if declaration.strip()
        )
    )


FORMATTERS = {".rs": format_rust, ".wgsl": format_wgsl, ".html": format_html}


def format_text(path, text, keep_headers):
    formatter = FORMATTERS.get(path.suffix)
    if formatter is None:
        return text
    return formatter(text, keep_headers)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("paths", nargs="+", type=Path)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--keep-headers", action="store_true")
    args = parser.parse_args()
    files = []
    for path in args.paths:
        if path.is_dir():
            files.extend(
                sorted(child for child in path.rglob("*") if child.suffix in FORMATTERS)
            )
        else:
            files.append(path)
    changed = 0
    for path in files:
        text = path.read_text()
        formatted = format_text(path, text, args.keep_headers)
        if formatted != text:
            changed += 1
            if args.check:
                print(f"would change {path}")
            else:
                path.write_text(formatted)
    if args.check and changed:
        sys.exit(1)


if __name__ == "__main__":
    main()
