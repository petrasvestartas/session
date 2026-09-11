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
IMAGE = re.compile(r"^!\[[^\]]*\]\(illustrations/locator-[0-9a-f]+\.svg\)(\{[^}]*\})?\n\n?", re.M)
MARK = re.compile(r"^<span class=\"zone-mark\"[^>]*></span>\n\n?", re.M)

# Colours chosen for the black page directly: Canvas.raw() remaps light fills to dark so a
# label stays readable on a white box, which is the wrong direction for boxes drawn ON the page.
# Three fills, far enough apart to read at a glance: pink here, a light slate for what you have
# already built, and a fill barely above the page for what is still ahead.
AHEAD_FILL, AHEAD_INK = "#1e1e22", "#6e6e76"
BUILT_FILL, BUILT_INK = "#4a4a54", "#f4f4f6"
LIT_FILL, LIT_INK = "#fa9ebc", "#111111"

# One zone per box on the map. Order is reading order within the row; the matchers are tried in
# order, so a longer prefix must come before the directory that contains it.
ZONES = [
    ("page",    0, "Page",         "index.html · Trunk · Cargo", ["session_viewer/index.html", "session_viewer/Trunk.toml",
                                                                  "session_viewer/Cargo", "session_viewer/.cargo",
                                                                  "session_viewer/assets/", "session_viewer/docs/build_site.sh",
                                                                  "session_viewer/index.html"]),
    ("network", 0, "Network",      "fetch · manifest · stream",  ["session_viewer/src/app/fetch.rs", "session_viewer/src/app/live.rs",
                                                                  "session_viewer/src/app/stream.rs", "session_viewer/src/app/manifest.rs",
                                                                  "session_viewer/src/app/validate.rs", "session_viewer/src/app/decode.rs",
                                                                  "session_viewer/src/app/route.rs", "session_viewer/src/app/loader.rs",
                                                                  "session_viewer/src/app/cloud_query.rs", "session_viewer/src/app/sheet_query.rs"]),
    ("kernel",  0, "Kernel",       "documents in f64",           ["session_rust/", "session_proto/", "session_py/", "session_cpp/"]),
    ("scene",   0, "Scene + walk", "documents → rows",           ["session_viewer/src/app/scene", "session_viewer/src/app/walk/",
                                                                  "session_viewer/src/app/selection.rs", "session_viewer/src/scene.rs",
                                                                  "session_viewer/src/engine/text.rs"]),
    ("shell",   1, "Shell",        "lib.rs · App",      ["session_viewer/src/lib.rs", "session_viewer/src/app/mod.rs",
                                                                  "session_viewer/src/app/feedback.rs", "session_viewer/src/app/inspection",
                                                                  "session_viewer/src/app/knobs.rs", "session_viewer/src/selftest",
                                                                  "session_viewer/src/text_quality.rs", "session_viewer/src/fixture.rs",
                                                                  "session_viewer/src/text_layout.rs",
                                                                  "session_viewer/src/engine/performance.rs"]),
    ("input",   1, "Input",        "pointer · keys",     ["session_viewer/src/app/input.rs", "session_viewer/src/app/touch.rs"]),
    ("state",   1, "State",        "camera, pick",         ["session_viewer/src/state", "session_viewer/src/camera.rs",
                                                                  "session_viewer/src/math.rs"]),
    ("gpucore", 1, "GPU core",     "device · frame", ["session_viewer/src/engine/gpu/mod.rs", "session_viewer/src/engine/gpu/device.rs",
                                                                  "session_viewer/src/engine/gpu/present.rs", "session_viewer/src/engine/gpu/render.rs",
                                                                  "session_viewer/src/engine/gpu/frame.rs", "session_viewer/src/engine/gpu/objects.rs",
                                                                  "session_viewer/src/engine/gpu/targets.rs", "session_viewer/src/engine/gpu/buffers.rs",
                                                                  "session_viewer/src/engine/gpu/instance.rs", "session_viewer/src/engine/gpu/upload.rs",
                                                                  "session_viewer/src/engine/gpu/view.rs", "session_viewer/src/engine/pipelines",
                                                                  "session_viewer/src/engine/mod.rs"]),
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
        y = 76.0 if rowi == 0 else 200.0
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
    c = DRAW.Canvas("Where this step sits in the viewer", desc, 1180, 312)
    c.text(28, 34, "Where you are", "l", fill=PAL["grey"])
    for key, rowi, label, sub, _ in ZONES:
        x, y, w, h = PLACE[key]
        on, here = key in built, key in lit
        fill = LIT_FILL if here else (BUILT_FILL if on else AHEAD_FILL)
        # straight into parts: no colour remapping, these boxes sit on the page, not on white
        c.parts.append(f'<rect x="{x:.1f}" y="{y:.1f}" width="{w:.1f}" height="{h:.1f}" rx="{DRAW.RADIUS}" '
                       f'fill="{fill}"/>')
        ink = LIT_INK if here else (BUILT_INK if on else AHEAD_INK)
        sub_ink = LIT_INK if here else ("#c9ccd6" if on else AHEAD_INK)
        c.text(x + 12, y + 26, label, "l", fill=ink, keep=True)
        c.text(x + 12, y + 48, sub, "s", fill=sub_ink, keep=True)
    sx, sy, sw, sh = PLACE["scene"]
    gx, gy, gw, _ = PLACE["gpucore"]
    lx, ly, lw, _ = PLACE["lanes"]
    # the rows the walk produced fall into the frame path; nothing else crosses between the rows
    down = sx + sw / 2
    c.parts.append(f'<path d="M{down:.1f},{sy + sh:.1f} L{down:.1f},158 L{gx + gw / 2:.1f},158 '
                   f'L{gx + gw / 2:.1f},{gy - 2:.1f}" fill="none" stroke="{BUILT_INK}" stroke-width="1.6" '
                   f'marker-end="url(#a)"/>')
    c.text(gx + gw / 2 + 10, 154, "rows, uploaded once", "s", fill="#c9ccd6")
    # and one answer travels the other way
    up = lx + lw / 2
    c.parts.append(f'<path d="M{up:.1f},{ly:.1f} L{up:.1f},182 L{sx + sw - 26:.1f},182 '
                   f'L{sx + sw - 26:.1f},{sy + sh + 2:.1f}" fill="none" stroke="{PAL["orange"]}" '
                   f'stroke-width="1.6" stroke-dasharray="7 5" marker-end="url(#a)"/>')
    c.text(up + 14, 177, "a pick answer travels back up", "s", fill=PAL["orange"])
    # the key, so the map explains itself
    kx = 28.0
    for fill, ink, label in ((LIT_FILL, LIT_INK, "this step"), (BUILT_FILL, BUILT_INK, "already built"),
                             (AHEAD_FILL, AHEAD_INK, "still ahead")):
        c.parts.append(f'<rect x="{kx:.1f}" y="272" width="26" height="16" rx="3" fill="{fill}"/>')
        c.text(kx + 34, 285, label, "s", fill="#c9ccd6", keep=True)
        kx += 34 + width(label, "s") + 26
    c.text(kx + 12, 285, "documents come in along the top row; a frame is drawn along the bottom one",
           "s", fill="#c9ccd6", keep=True)
    body = "".join(c.parts)
    name = "locator-" + hashlib.sha1(body.encode()).hexdigest()[:10] + ".svg"
    c.write(name)
    return name, lit_names



def strip(lit, built):
    """The same map, compressed to a bar that can sit pinned under the header while you read."""
    c = DRAW.Canvas("Where this step sits in the viewer",
                    "The eleven zones of the viewer as a bar; the pink one is where the code on this page lives.",
                    1180, 92)
    margin, gap, h = 16.0, 8.0, 30.0
    top = [z for z in ZONES if z[1] == 0]
    bottom = [z for z in ZONES if z[1] == 1]
    for zones, y in ((top, 8.0), (bottom, 50.0)):
        w = (1180 - 2 * margin - gap * (len(zones) - 1)) / len(zones)
        for i, (key, _, label, _, _) in enumerate(zones):
            x = margin + i * (w + gap)
            on, here = key in built, key in lit
            fill = LIT_FILL if here else (BUILT_FILL if on else AHEAD_FILL)
            c.parts.append(f'<rect x="{x:.1f}" y="{y:.1f}" width="{w:.1f}" height="{h:.1f}" rx="3" '
                           f'fill="{fill}"/>')
            ink = LIT_INK if here else (BUILT_INK if on else AHEAD_INK)
            c.text(x + w / 2, y + 20, label, "s", anchor="middle", fill=ink, keep=True)
    body = "".join(c.parts)
    name = "strip-" + hashlib.sha1(body.encode()).hexdigest()[:10] + ".svg"
    c.write(name)
    return name


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
        text = MARK.sub("", IMAGE.sub("", lesson.read_text()))
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
            if None in touched:
                unplaced = sorted({d.group(3).split()[0] for d in DIRECTIVE.finditer(chunk)
                                   if d.group(1) == "file" and d.group(2) == step["id"]
                                   and zone_of(d.group(3).split()[0]) is None})
                raise SystemExit(f"{lesson.name}: no zone for {', '.join(unplaced)} - "
                                 "add it to ZONES, or the step silently gets no map")
            if not touched:
                continue
            name, names = render(touched, built | touched)
            bar = strip(touched, built | touched)
            drawn.add(name); drawn.add(bar)
            alt = (f"Where this step sits in the viewer: {names}, "
                   f"with {len(built | touched)} of {len(ZONES)} zones built so far.")
            out.append(text[last:head.end()])
            out.append(f"\n\n![{alt}](illustrations/{name}){{ .locator data-strip=\"illustrations/{bar}\" }}")
            last = head.end()
            built |= touched
            # One mark per code block, so scrolling to a file inside a step moves the highlight
            # to that file's zone rather than leaving the whole step lit.
            for d in DIRECTIVE.finditer(text, head.end(), (nxt.start() if nxt else len(text))):
                if d.group(1) != "file" or d.group(2) != step["id"]:
                    continue
                zone = zone_of(d.group(3).split()[0])
                if zone is None:
                    continue
                one = strip({zone}, built)
                drawn.add(one)
                out.append(text[last:d.start()])
                out.append(f'<span class="zone-mark" data-strip="illustrations/{one}" '
                           f'data-zone="{BY_KEY[zone][2]}"></span>\n\n')
                last = d.start()
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
