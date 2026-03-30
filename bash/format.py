#!/usr/bin/env python3
"""Deterministic autoformatter for session_cpp, session_py, session_rust.

Enforces consistent style across all three languages:
  1. Spaces after commas in constructor args: Point(0,0,0) -> Point(0, 0, 0)
  2. Multi-line expansion of lists with 2+ geometry constructors on one line
  3. One Point/Vector per line when inside collection constructors

Usage:
    python bash/format.py                    # all languages
    python bash/format.py --py               # Python only
    python bash/format.py --cpp              # C++ only
    python bash/format.py --rust             # Rust only
    python bash/format.py --dry-run          # show changes without writing
    python bash/format.py path/to/file.cpp   # single file
"""
import re
import sys
import os
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# Types whose constructor args get space-after-comma treatment
TYPES_PY   = r'Point|Vector|Color|Line|Plane|Polyline|NurbsCurve|NurbsSurface|BoundingBox|Quaternion|Xform|Matrix'
TYPES_CPP  = TYPES_PY
TYPES_RUST = r'Point::new|Vector::new|Color::new|Line::new|Plane::new|Polyline::new|NurbsCurve::new|NurbsSurface::new|BoundingBox::new|Quaternion::new|Xform::new|Matrix::new'

# Constructors that hold geometry items (triggers multi-line expansion)
GEO_CTORS_PY   = r'(?:Point|Vector|Color)\s*\('
GEO_CTORS_CPP  = r'(?:Point|Vector|Color)\s*\('
GEO_CTORS_RUST = r'(?:Point|Vector|Color)::new\s*\('

# Brace-like list openers per language
LIST_OPEN_CPP  = r'[{[]'
LIST_OPEN_PY   = r'[\[\(]'
LIST_OPEN_RUST = r'(?:vec!\[|\[)'


def find_matching_close(s, start):
    """Find matching closing bracket/paren for opener at s[start]."""
    opener = s[start]
    closer = {'(': ')', '[': ']', '{': '}'}[opener]
    depth = 1
    i = start + 1
    while i < len(s):
        if s[i] == opener:
            depth += 1
        elif s[i] == closer:
            depth -= 1
            if depth == 0:
                return i
        elif s[i] == '"' or s[i] == "'":
            q = s[i]
            i += 1
            while i < len(s) and s[i] != q:
                if s[i] == '\\':
                    i += 1
                i += 1
        i += 1
    return -1


def add_spaces_after_commas_in_ctors(line, lang):
    """Rule 1: Add space after commas inside known constructors."""
    if lang == 'rust':
        pat = re.compile(r'(' + TYPES_RUST + r')\s*\(')
    elif lang == 'py':
        pat = re.compile(r'(' + TYPES_PY + r')\s*\(')
    else:
        pat = re.compile(r'(' + TYPES_CPP + r')\s*\(')

    result = []
    i = 0
    while i < len(line):
        m = pat.search(line, i)
        if not m:
            result.append(line[i:])
            break

        # append everything before this match
        result.append(line[i:m.start()])

        # find the opening paren
        paren_start = line.index('(', m.start())
        paren_end = find_matching_close(line, paren_start)
        if paren_end == -1:
            result.append(line[m.start():])
            break

        # extract the full constructor including parens
        ctor_prefix = line[m.start():paren_start + 1]
        inner = line[paren_start + 1:paren_end]
        # add spaces after commas (but not if already there)
        inner = re.sub(r',(?! )', ', ', inner)
        result.append(ctor_prefix + inner + ')')
        i = paren_end + 1

    return ''.join(result)


def add_spaces_in_brace_lists(line):
    """Rule 1b: Add spaces after commas in bare {x,y,z} numeric lists (C++)."""
    def fix_brace_inner(m):
        inner = m.group(1)
        inner = re.sub(r',(?! )', ', ', inner)
        return '{' + inner + '}'

    # match {num,num,...} patterns (bare coordinate tuples in C++)
    return re.sub(
        r'\{((?:-?[\d.]+(?:e[+-]?\d+)?,\s*)*-?[\d.]+(?:e[+-]?\d+)?)\}',
        fix_brace_inner, line
    )


def count_geo_ctors(line, lang):
    """Count geometry constructors on a line."""
    if lang == 'rust':
        return len(re.findall(GEO_CTORS_RUST, line))
    elif lang == 'py':
        return len(re.findall(GEO_CTORS_PY, line))
    return len(re.findall(GEO_CTORS_CPP, line))


def split_geo_items(inner, lang):
    """Split a comma-separated list respecting parentheses depth."""
    items = []
    depth = 0
    current = []
    for ch in inner:
        if ch in '([{':
            depth += 1
            current.append(ch)
        elif ch in ')]}':
            depth -= 1
            current.append(ch)
        elif ch == ',' and depth == 0:
            items.append(''.join(current).strip())
            current = []
        else:
            current.append(ch)
    if current:
        items.append(''.join(current).strip())
    return [item for item in items if item]


def expand_multiline(lines, lang):
    """Rule 2: Expand lines with 2+ geometry constructors in collections to one-per-line."""
    if lang == 'rust':
        geo_pat = re.compile(GEO_CTORS_RUST)
    elif lang == 'py':
        geo_pat = re.compile(GEO_CTORS_PY)
    else:
        geo_pat = re.compile(GEO_CTORS_CPP)

    result = []
    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.lstrip()
        indent = line[:len(line) - len(stripped)]

        # count geo constructors on this line
        n_geo = count_geo_ctors(line, lang)

        # skip lines that are already one-per-line or have < 2 geo ctors
        if n_geo < 2:
            result.append(line)
            i += 1
            continue

        # skip std::vector<Point> declarations with Point(...) inside
        if 'std::vector<Point>' in line and '=' in line and '{' in line:
            # check if this is a declaration like:
            #   std::vector<Point> pts = { Point(...), Point(...) };
            # which may span multiple lines - leave alone
            result.append(line)
            i += 1
            continue

        # find the collection opener that contains the geo ctors
        expanded = try_expand_line(line, indent, lang, geo_pat)
        if expanded:
            result.extend(expanded)
        else:
            result.append(line)
        i += 1

    return result


def try_expand_line(line, indent, lang, geo_pat):
    """Try to expand a single line with multiple geo ctors into multi-line."""
    stripped = line.strip()

    # detect the collection pattern and its bounds
    # look for patterns like:
    #   C++:  Polyline pl({Point(...), Point(...), ...});
    #   C++:  Polyline({Point(...), Point(...), ...})
    #   Py:   Polyline([Point(...), Point(...), ...])
    #   Rust: Polyline::new(vec![Point::new(...), Point::new(...), ...])

    # find the innermost collection bracket that contains the geo ctors
    # strategy: find all geo ctor positions, then find the enclosing bracket pair

    first_geo = geo_pat.search(line)
    if not first_geo:
        return None

    # walk backwards from first geo ctor to find the opening bracket
    pos = first_geo.start()
    bracket_start = -1
    for j in range(pos - 1, -1, -1):
        if line[j] in '{[':
            bracket_start = j
            break

    if bracket_start == -1:
        return None

    bracket_end = find_matching_close(line, bracket_start)
    if bracket_end == -1:
        return None

    # check: at least 2 geo ctors inside this bracket pair
    inner = line[bracket_start + 1:bracket_end]
    n_inner = len(geo_pat.findall(inner))
    if n_inner < 2:
        return None

    prefix = line[:bracket_start + 1]  # everything up to and incl opening bracket
    suffix = line[bracket_end:]          # closing bracket and everything after

    # split inner items
    items = split_geo_items(inner, lang)
    if len(items) < 2:
        return None

    # determine inner indent: prefix indent + 4 spaces
    inner_indent = indent + '    '
    # if prefix itself is indented, use prefix length for alignment?
    # simpler: just use indent + 4 spaces
    # detect if we're inside a deeper nesting
    prefix_stripped = prefix.strip()
    if prefix_stripped and prefix_stripped[0] in '{[':
        # bare list, use indent + 4
        inner_indent = indent + '    '
    else:
        inner_indent = indent + '    '

    expanded = [prefix.rstrip()]
    for j, item in enumerate(items):
        item = item.strip()
        # add trailing comma to all items (trailing comma style)
        if j < len(items) - 1:
            if not item.endswith(','):
                item += ','
        else:
            # last item: trailing comma for Rust/Python, no trailing comma for C++
            if lang in ('rust', 'py'):
                if not item.endswith(','):
                    item += ','
            else:
                # C++: remove trailing comma from last item
                item = item.rstrip(',')
        expanded.append(inner_indent + item)
    expanded.append(indent + suffix.lstrip())

    return expanded


def join_continuation_lines(lines, lang):
    """Pre-pass: join lines that are continuations of a multi-geo-ctor statement.

    e.g. in C++:
        Polyline pl({Point(7,0,0), Point(10,0,0), Point(10,1,0),
                     Point(8,1,0), Point(8,3,0),  Point(7,3,0)});

    Only joins when BOTH the first line AND continuation lines contain geo ctors,
    indicating a collection of geometry objects split across lines.
    """
    result = []
    i = 0
    while i < len(lines):
        line = lines[i]

        stripped = line.strip()
        if not stripped or stripped.startswith('//') or stripped.startswith('#'):
            result.append(line)
            i += 1
            continue

        # count open/close brackets
        opens = sum(1 for c in line if c in '([{')
        closes = sum(1 for c in line if c in ')]}')

        if opens > closes and count_geo_ctors(line, lang) >= 2:
            # line has 2+ geo ctors and unclosed brackets - likely a collection split across lines
            line_indent = line[:len(line) - len(line.lstrip())]
            joined = line.rstrip()
            depth = opens - closes
            j = i + 1
            while j < len(lines) and depth > 0:
                next_line = lines[j].strip()
                if joined and not joined.endswith(' '):
                    joined += ' '
                joined += next_line
                opens_j = sum(1 for c in next_line if c in '([{')
                closes_j = sum(1 for c in next_line if c in ')]}')
                depth += opens_j - closes_j
                j += 1
            # collapse internal multiple spaces, preserve leading indent
            content = joined.lstrip()
            content = re.sub(r'  +', ' ', content)
            result.append(line_indent + content)
            i = j
        elif opens > closes and count_geo_ctors(line, lang) == 1:
            # line has 1 geo ctor and unclosed brackets - check if continuations have geo ctors
            depth = opens - closes
            j = i + 1
            has_continuation_geo = False
            while j < len(lines) and depth > 0:
                next_stripped = lines[j].strip()
                if count_geo_ctors(next_stripped, lang) >= 1:
                    has_continuation_geo = True
                opens_j = sum(1 for c in next_stripped if c in '([{')
                closes_j = sum(1 for c in next_stripped if c in ')]}')
                depth += opens_j - closes_j
                j += 1
            if has_continuation_geo:
                # join them
                line_indent = line[:len(line) - len(line.lstrip())]
                joined = line.rstrip()
                depth = opens - closes
                k = i + 1
                while k < len(lines) and depth > 0:
                    next_line = lines[k].strip()
                    if joined and not joined.endswith(' '):
                        joined += ' '
                    joined += next_line
                    opens_k = sum(1 for c in next_line if c in '([{')
                    closes_k = sum(1 for c in next_line if c in ')]}')
                    depth += opens_k - closes_k
                    k += 1
                content = joined.lstrip()
                content = re.sub(r'  +', ' ', content)
                result.append(line_indent + content)
                i = j
            else:
                result.append(line)
                i += 1
        else:
            result.append(line)
            i += 1

    return result


def format_file(filepath, dry_run=False):
    """Format a single file. Returns True if changes were made."""
    filepath = Path(filepath)
    ext = filepath.suffix

    if ext == '.py':
        lang = 'py'
    elif ext in ('.cpp', '.h', '.hpp', '.cc'):
        lang = 'cpp'
    elif ext == '.rs':
        lang = 'rust'
    else:
        return False

    with open(filepath, 'r', encoding='utf-8') as f:
        original = f.read()

    lines = original.split('\n')

    # Rule 1: spaces after commas in constructors
    lines = [add_spaces_after_commas_in_ctors(l, lang) for l in lines]

    # Rule 1b (C++ only): spaces in bare {x,y,z} lists
    if lang == 'cpp':
        lines = [add_spaces_in_brace_lists(l) for l in lines]

    # Pre-pass: join continuation lines
    lines = join_continuation_lines(lines, lang)

    # Rule 1 again after joining (joined lines may have new comma-spacing issues)
    lines = [add_spaces_after_commas_in_ctors(l, lang) for l in lines]

    # Rule 2: expand multi-geo-ctor lines
    lines = expand_multiline(lines, lang)

    # Final: strip trailing whitespace
    lines = [l.rstrip() for l in lines]

    result = '\n'.join(lines)

    if result != original:
        if dry_run:
            import difflib
            print(f"  WOULD CHANGE: {filepath.relative_to(ROOT)}")
            orig_lines = original.splitlines(keepends=True)
            res_lines = result.splitlines(keepends=True)
            diff = difflib.unified_diff(orig_lines, res_lines, n=1)
            for d in diff:
                if d.startswith('---') or d.startswith('+++'):
                    continue
                try:
                    print(f"    {d.rstrip()}")
                except UnicodeEncodeError:
                    print(f"    {d.rstrip().encode('ascii', 'replace').decode()}")
        else:
            with open(filepath, 'w', encoding='utf-8', newline='\n') as f:
                f.write(result)
            print(f"  FIXED: {filepath.relative_to(ROOT)}")
        return True
    return False


def collect_files(lang):
    """Collect source and test files for a language."""
    files = []
    if lang == 'py':
        d = ROOT / 'session_py' / 'src' / 'session_py'
        if d.exists():
            files.extend(d.glob('*.py'))
    elif lang == 'cpp':
        d = ROOT / 'session_cpp' / 'src'
        if d.exists():
            files.extend(d.glob('*.cpp'))
            files.extend(d.glob('*.h'))
    elif lang == 'rust':
        d = ROOT / 'session_rust' / 'src'
        if d.exists():
            files.extend(d.glob('*.rs'))
    # filter out mini_test, __init__, lib.rs
    skip = {'__init__.py', 'mini_test.py', 'mini_test.rs', 'mini_test.h', 'lib.rs'}
    return [f for f in files if f.name not in skip]


def main():
    args = sys.argv[1:]
    dry_run = '--dry-run' in args
    args = [a for a in args if a != '--dry-run']

    langs = []
    single_file = None

    for arg in args:
        if arg in ('--py', '--python'):
            langs.append('py')
        elif arg == '--cpp':
            langs.append('cpp')
        elif arg == '--rust':
            langs.append('rust')
        elif os.path.isfile(arg):
            single_file = arg

    if single_file:
        single_file = Path(single_file).resolve()
        changed = format_file(single_file, dry_run)
        if not changed:
            print(f"  OK: {single_file}")
        return

    if not langs:
        langs = ['cpp', 'py', 'rust']

    total_changed = 0
    total_files = 0
    for lang in langs:
        files = collect_files(lang)
        if not files:
            continue
        print(f"[{lang}]")
        for f in sorted(files):
            total_files += 1
            if format_file(f, dry_run):
                total_changed += 1

    print(f"\n{total_changed}/{total_files} files changed.")


if __name__ == '__main__':
    main()
