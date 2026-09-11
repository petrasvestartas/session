#!/usr/bin/env python3
"""Draw the one map of the viewer, and put it at the head of every step with that step lit.

The course explains each mechanism where it is built, which leaves the reader without an answer
to the question that matters while typing: where does this file sit in the whole viewer? This
module answers it the same way every time. One map, eleven zones, three states per zone -
already built, being touched by this step (pink, the colour every diagram in the course already
uses for its subject), or still ahead. A reader who has seen the map once can place any file in
it at a glance, which is what prose cannot do.

    python3 docs/locator.py            # regenerate the maps and the lesson references
    python3 docs/locator.py --check    # fail when a lesson is out of date
"""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys

HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("draw", HERE / "illustrations" / "draw.py")
DRAW = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(DRAW)
PAL, esc, width = DRAW.PAL, DRAW.esc, DRAW.width

DIRECTIVE = re.compile(r"<!-- (file|supplied): (\S+)(.*?)-->")
HEADING = re.compile(r"^(#{2,3}) (Step|Part) ([^\n]*)$", re.M)
IMAGE = re.compile(r"^!\[[^\]]*\]\(illustrations/locator-[0-9a-f]+\.svg\)\n\n?", re.M)

DIM, LIT_FILL, LIT_STROKE = "#8a8a90", PAL["pink"], PAL["pink_band"]

# One zone per box on the map. Order is reading order within the row; the matchers are tried in
# order, so a longer prefix must come before the directory that contains it.
ZONES = [
    ("page",    0, "Page",         "index.html · Trunk · Cargo", ["session_viewer/index.html", "session_viewer/Trunk.toml",
                                                                  "session_viewer/Cargo", "session_viewer/.cargo",
                                                                  "session_viewer/assets/"]),
    ("network", 0, "Network",      "fetch · manifest · stream",  ["session_viewer/src/app/fetch.rs", "session_viewer/src/app/live.rs",
                                                                  "session_viewer/src/app/stream.rs", "session_viewer/src/app/manifest.rs",
                                                                  "session_viewer/src/app/validate.rs", "session_viewer/src/app/decode.rs",
                                                                  "session_viewer/src/app/route.rs", "session_viewer/src/app/loader.rs",
                                                                  "session_viewer/src/app/cloud_query.rs", "session_viewer/src/app/sheet_query.rs"]),
    ("kernel",  0, "Kernel",       "documents in f64",           ["session_rust/", "session_proto/", "session_py/", "session_cpp/"]),
    ("scene",   0, "Scene + walk", "documents → rows",           ["session_viewer/src/app/scene", "session_viewer/src/app/walk/",
                                                                  "session_viewer/src/app/selection.rs"]),
    ("shell",   1, "Shell",        "lib.rs · App",      ["session_viewer/src/lib.rs", "session_viewer/src/app/mod.rs",
                                                                  "session_viewer/src/app/feedback.rs", "session_viewer/src/app/inspection",
                                                                  "session_viewer/src/app/knobs.rs", "session_viewer/src/selftest",
                                                                  "session_viewer/src/text_quality.rs", "session_viewer/src/fixture.rs"]),
    ("input",   1, "Input",        "pointer · keys",     ["session_viewer/src/app/input.rs", "session_viewer/src/app/touch.rs"]),
    ("state",   1, "State",        "camera, pick",         ["session_viewer/src/state", "session_viewer/src/camera.rs",
                                                                  "session_viewer/src/math.rs"]),
    ("gpucore", 1, "GPU core",     "device · frame", ["session_viewer/src/engine/gpu/mod.rs", "session_viewer/src/engine/gpu/device.rs",
                                                                  "session_viewer/src/engine/gpu/present.rs", "session_viewer/src/engine/gpu/render.rs",
                                                                  "session_viewer/src/engine/gpu/frame.rs", "session_viewer/src/engine/gpu/objects.rs",
                                                                  "session_viewer/src/engine/gpu/targets.rs", "session_viewer/src/engine/gpu/buffers.rs",
                                                                  "session_viewer/src/engine/gpu/instance.rs", "session_viewer/src/engine/gpu/upload.rs",
                                                                  "session_viewer/src/engine/gpu/view.rs", "session_viewer/src/engine/pipelines",
                                                                  "session_viewer/src/engine/mod.rs", "session_viewer/src/engine/performance.rs",
                                                                  "session_viewer/src/engine/text.rs"]),
    ("lanes",   1, "Lanes",        "one per kind",         ["session_viewer/src/engine/gpu/"]),
    ("shaders", 1, "Shaders",      "WGSL",            ["session_viewer/src/shaders/"]),
    ("pixels",  1, "Pixels",       "the canvas",                 []),
]
BY_KEY = {z[0]: z for z in ZONES}


def zone_of(path):
    for key, _, _, _, matchers in ZONES:
        if any(path.startswith(m) for m in matchers):
            return key
    return None


def geometry():
    """Fixed positions: the map never moves, only the highlight does."""
    top = [z for z in ZONES if z[1] == 0]
    bottom = [z for z in ZONES if z[1] == 1]
    place = {}
    for rowi, zones in ((0, top), (1, bottom)):
        y = 92 if rowi == 0 else 208
        margin, gap = 28.0, 16.0
        w = (1180 - 2 * margin - gap * (len(zones) - 1)) / len(zones)
        for i, z in enumerate(zones):
            place[z[0]] = (margin + i * (w + gap), y, w, 66.0)
    return place


PLACE = geometry()

for _key, _row, _label, _sub, _m in ZONES:
    _w = PLACE[_key][2]
    assert max(width(_label, "l"), width(_sub, "s")) + 24 <= _w, f"locator label overflows its box: {_key}"


def render(lit, built):
    """One map. `lit` is this step, `built` is everything the reader already has."""
    lit_names = ", ".join(BY_KEY[k][2] for k in ZONES_ORDER if k in lit) or "nothing new"
    desc = (f"The whole viewer as one map. This step works in: {lit_names}. "
            "A solid box is something you have already built, a dashed one is still ahead, "
            "and the pink box is where the code on this page lives.")
    c = DRAW.Canvas("Where this step sits in the viewer", desc, 1180, 330)
    c.text(28, 34, "Where you are", "l", fill=PAL["grey"])
    for key, rowi, label, sub, _ in ZONES:
        x, y, w, h = PLACE[key]
        on, here = key in built, key in lit
        fill = LIT_FILL if here else "none"
        stroke = LIT_STROKE if here else (PAL["white"] if on else DIM)
        dash = "" if (on or here) else ' stroke-dasharray="6 5"'
        c.raw(f'<rect x="{x:.1f}" y="{y:.1f}" width="{w:.1f}" height="{h:.1f}" rx="{DRAW.RADIUS}" '
              f'fill="{fill}" stroke="{stroke}" stroke-width="{2.2 if here else 1.4}"{dash}/>')
        ink = PAL["black"] if here else (PAL["white"] if on else DIM)
        c.text(x + 12, y + 26, label, "l", fill=ink, keep=True)
        c.text(x + 12, y + 48, sub, "s", fill=ink if here else (PAL["text2"] if on else DIM), keep=True)
    # the two paths and where they meet
    sx, sy, sw, sh = PLACE["scene"]
    gx, gy, _, _ = PLACE["gpucore"]
    c.raw(f'<path class="ar" d="M{sx + sw / 2:.1f},{sy + sh:.1f} L{sx + sw / 2:.1f},{sy + sh + 26:.1f} '
          f'L{gx + 60:.1f},{sy + sh + 26:.1f} L{gx + 60:.1f},{gy - 4:.1f}"/>')
    c.text(sx + sw / 2 + 10, sy + sh + 20, "rows", "s", fill=PAL["text2"])
    lx, ly, lw, _ = PLACE["lanes"]
    c.raw(f'<path class="dash" d="M{lx + lw / 2:.1f},{ly:.1f} L{lx + lw / 2:.1f},{ly - 30:.1f} '
          f'L{sx + sw - 30:.1f},{ly - 30:.1f} L{sx + sw - 30:.1f},{sy + sh + 4:.1f}"/>')
    c.text(lx + lw / 2 - 150, ly - 36, "a pick answer travels back up", "s", fill=PAL["orange"])
    c.text(28, 300, "documents come in along the top row; a frame is drawn along the bottom one", "s", fill=PAL["text2"])
    body = "".join(c.parts)
    name = "locator-" + hashlib.sha1(body.encode()).hexdigest()[:10] + ".svg"
    c.write(name)
    return name, lit_names


ZONES_ORDER = [z[0] for z in ZONES]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    # The wall map: every zone built, none lit. `docs/map.md` teaches this one picture.
    all_keys = {z[0] for z in ZONES}
    c_name, _ = render(set(), all_keys)
    (HERE / "illustrations" / "map.svg").write_text((HERE / "illustrations" / c_name).read_text())
    (HERE / "illustrations" / c_name).unlink()
    series = json.loads((HERE / "reconstruction" / "series.json").read_text())
    steps = series["steps"]
    stale, drawn = [], set()
    for index, step in enumerate(steps):
        lesson = next(p for p in HERE.glob(f"{step['id']}-*.md"))
        text = IMAGE.sub("", lesson.read_text())
        if index:
            previous = set(steps[index - 1]["files"])
        else:
            previous = {f for f in step["files"] if not f.startswith("session_viewer/")}
        built = {zone_of(f) for f in previous} - {None}
        out, last = [], 0
        for head in HEADING.finditer(text):
            body_end = text.find("\n## ", head.end())
            chunk = text[head.end():body_end if body_end != -1 else len(text)]
            nxt = HEADING.search(text, head.end())
            chunk = text[head.end():nxt.start() if nxt else len(text)]
            touched = set()
            for d in DIRECTIVE.finditer(chunk):
                if d.group(1) == "file":
                    path = d.group(3).split()[0] if d.group(2) == step["id"] else None
                    if path:
                        touched.add(zone_of(path))
            touched -= {None}
            if not touched:
                continue
            name, names = render(touched, built | touched)
            drawn.add(name)
            alt = (f"Where this step sits in the viewer: {names}, "
                   f"with {len(built | touched)} of {len(ZONES)} zones built so far.")
            out.append(text[last:head.end()])
            out.append(f"\n\n![{alt}](illustrations/{name})")
            last = head.end()
            built |= touched
        out.append(text[last:])
        new = "".join(out)
        if new != lesson.read_text():
            if args.check:
                stale.append(lesson.name)
            else:
                lesson.write_text(new)
    if stale:
        raise SystemExit("locators out of date: " + ", ".join(stale))
    print("locators current" if args.check else f"wrote {len(drawn)} maps into 24 lessons")


if __name__ == "__main__":
    main()
