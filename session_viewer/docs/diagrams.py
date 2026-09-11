#!/usr/bin/env python3
"""The course's flowcharts: editable `.d2` sources in, committed SVGs out.

Why a file per diagram rather than a fenced block in the lesson: the same reason lesson code is
not in the Markdown. The source is a small text file anyone can edit, the rendered SVG is
committed so building the site needs nothing, and one command re-renders after an edit.

    python3 docs/diagrams.py            # render every changed source
    python3 docs/diagrams.py --check    # fail if a committed SVG is out of date
    python3 docs/diagrams.py --all      # re-render everything

It fetches its own renderer: d2 is pinned below and cached under `target/tools/`, so a fresh
checkout (or a future session on another machine) needs one command and a network connection the
first time, and nothing after that. If d2 ever became unavailable the `.d2` files are still plain
text describing the graph, and the committed SVGs keep the site working meanwhile.
"""
import argparse
import hashlib
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tarfile
import urllib.request

HERE = Path(__file__).resolve().parent
SOURCES = HERE / "diagrams"
OUT = HERE / "illustrations"
CACHE = HERE.parent / "target" / "tools"
D2_VERSION = "v0.9.0"
D2_URL = f"https://github.com/terrastruct/d2/releases/download/{D2_VERSION}/d2-{D2_VERSION}-linux-amd64.tar.gz"

# The palette, in one place. A diagram uses exactly these fills, and `style` in the old Mermaid
# sources became `class: focus` - the one node a diagram is about.
HEADER = '''vars: { d2-config: { layout-engine: elk; pad: 20; theme-id: 0 } }
style: { fill: "#ffffff" }
classes: {
  box:   { style: { fill: "#e9e9ec"; stroke: "#e9e9ec"; border-radius: 6; font-size: 15; font-color: "#111111" } }
  focus: { style: { fill: "#fa9ebc"; stroke: "#fa9ebc"; border-radius: 6; font-size: 15; font-color: "#111111" } }
}
'''
EDGE = '{ style: { stroke: "#0b1957"; stroke-width: 2; font-size: 13; font-color: "#455b6b" } }'
EDGE_SOFT = '{ style: { stroke: "#8a8fb0"; stroke-width: 2; stroke-dash: 4; font-size: 13; font-color: "#455b6b" } }'


def d2_binary():
    """The pinned renderer, fetched once into target/tools and reused."""
    local = CACHE / f"d2-{D2_VERSION}" / "bin" / "d2"
    if local.is_file():
        return local
    found = shutil.which("d2")
    if found:
        return Path(found)
    CACHE.mkdir(parents=True, exist_ok=True)
    archive = CACHE / "d2.tar.gz"
    print(f"fetching {D2_URL}", flush=True)
    urllib.request.urlretrieve(D2_URL, archive)
    with tarfile.open(archive) as tar:
        tar.extractall(CACHE)
    archive.unlink()
    if not local.is_file():
        sys.exit(f"d2 did not unpack to {local}")
    local.chmod(0o755)
    return local


def render(source, binary, out):
    result = subprocess.run([str(binary), str(source), str(out)], capture_output=True, text=True,
                            env=dict(os.environ, D2_THEME="0"))
    if result.returncode != 0:
        sys.exit(f"d2 failed on {source.name}:\n{result.stdout}{result.stderr}")
    # d2 writes only a viewBox on the outer svg, so an <img> has no intrinsic size and the page
    # stretches it to the column width - a 489px diagram rendered at 970px, with type to match.
    # Give it its own size back; the stylesheet still allows it to shrink on a narrow screen.
    svg = out.read_text()
    head = re.match(r"(<\?xml[^>]*\?>)?(<svg\b[^>]*>)", svg)
    if head and "width=" not in head.group(2):
        box = re.search(r'viewBox="0 0 ([\d.]+) ([\d.]+)"', head.group(2))
        if box:
            sized = head.group(2)[:-1] + f' width="{box.group(1)}" height="{box.group(2)}">'
            out.write_text(svg[:head.start(2)] + sized + svg[head.end(2):])


def unsized(sources):
    """A diagram the sizing rule does not name is silently blown up to the column width.

    course.css keeps each generated SVG at its own size by matching its file name, and a new
    lesson stem matches nothing - which looks like a deliberately huge drawing rather than a
    bug. So the rule is read back here and every diagram checked against it.
    """
    css = (HERE / "stylesheets/course.css").read_text()
    fragments = re.findall(r'img\[src\*="illustrations/([^"]*)"\]', css)
    missed = [s.stem for s in sources if not any(s.stem.startswith(f) for f in fragments)]
    if missed:
        raise SystemExit("course.css has no size rule for: " + ", ".join(missed))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--all", action="store_true")
    args = parser.parse_args()
    sources = sorted(SOURCES.glob("*.d2"))
    if not sources:
        sys.exit(f"no diagram sources under {SOURCES}")
    binary = d2_binary()
    stale, made = [], 0
    for src in sources:
        svg = OUT / (src.stem + ".svg")
        if not args.all and svg.is_file() and svg.stat().st_mtime >= src.stat().st_mtime and not args.check:
            continue
        if args.check:
            if not svg.is_file():
                stale.append(src.stem)
                continue
            probe = OUT / (src.stem + ".probe.svg")
            render(src, binary, probe)
            if probe.read_bytes() != svg.read_bytes():
                stale.append(src.stem)
            probe.unlink()
            continue
        render(src, binary, svg)
        made += 1
    if stale:
        raise SystemExit("diagrams out of date: " + ", ".join(stale))
    unsized(sources)
    print("diagrams current" if args.check else f"rendered {made} of {len(sources)} diagrams")


if __name__ == "__main__":
    main()
