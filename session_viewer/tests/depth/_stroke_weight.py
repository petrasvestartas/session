#!/usr/bin/env python3
"""_stroke_weight.py <ppm> <ids>  ->  the weight of every red (joint) and blue (free) stroke

The strokes come from the id frame the renderer writes under VIEWER_IDS: one group of pixels
per (object, segment), so nothing antialiasing does to a colour can split one edge into two.
Weight is the ink the stroke actually laid, not the pixels that passed a threshold - a core
taken at alpha >= 0.5 quantises a 1.2 px pen to one pixel or two depending on where the line
falls between pixel centres, a 2x swing with no change in weight. So each core is dilated by
one pixel to take in the antialiasing fringe, every pixel contributes the coverage its lacking
channel shows, and the total is divided by the core's projected length.

A core missing two or more consecutive steps along its major axis is partly hidden - its
bounding box stays long while its ink does not - so it is named and left out of the floors. One
missing step is not: that is the alpha >= 0.5 core losing a pixel to sub-pixel phase.

Only the red-to-blue ratio is compared, so the background level cancels and needs to be no more
than consistent; it is the green channel's mode across the frame, 255 for these renders, whose
white clear and grey faces put every stroke over one of two levels.

The cross-section rule asks whether a stroke is even along its length, which a stroke 15 to 20
px long cannot answer - the caps are most of it - so it applies from a projected length of
SECTION_LENGTH px up.

The magenta rule is the exception: it reads the colour frame, every pixel of it, because ink
that must never show can surface below alpha 0.5 and carry no id at all.

Floors are measured, not assumed. The run is the twelve cases the joint probe covers - six
cameras at distances 1 and 4, 1800x1400, MSAA 4.

The red minimum over that run was 89% with the floor at 85% while every segment laid its
own caps. Since joined strokes share one join plane and exactly one segment owns each shared
sample (tests/stroke-joins.py checks the integrated ink), a joint segment carries less ink
at its ends than a free stroke of the same length, and the run re-measured on 2026-09-09 at
the 1.5 px pen gives a minimum of 82% (the iso camera at distance 1; every other case 86% or
more) with the floor at 78%, the same four-point margin. The 82% is still the bottom edge of
the box resting on the plate: the box's own top face, 400 mm nearer the eye, covers the inner
half of that edge past the foreshortened front face, so any viewer hides it.

The cross-section minimum over that run was 63% with the floor at 60%, both on a near-edge-on
blue stroke whose core skips single steps - the residual spec 3.4 names, a face under about
2 px wide having no same-face neighbour and dropping out at a grazing angle. Re-measured on
2026-09-09 with the shared-join renderer at the 1.5 px pen, that same stroke (the side camera
at distance 1) reads 59.7%, every other case 77% or more; the floor is 56%, the same
three-point margin.
"""
import os
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _count_colors import read_ppm

SEGMENT_BIT = 0x80000000
COUNT_FLOOR = 12
SECTION_LENGTH = 40
RED_FLOOR = 0.78
SECTION_FLOOR = 0.56


def read_ids(path):
    """The HLI2 id frame: a magic, two u32 of size, then per pixel the object id + 1 and the
    sub id. Only what the ink pass kept at alpha >= 0.5 carries an id, so a hidden stroke is
    simply absent."""
    with open(path, "rb") as handle:
        data = handle.read()
    assert data[:4] == b"HLI2", data[:4]
    width, height = struct.unpack_from("<II", data, 4)
    return width, height, data[12:]


def segments(width, height, ids):
    """Pixels per (object id, segment index), for the sub ids whose high bit marks a stroke."""
    out = {}
    for index in range(width * height):
        obj, sub = struct.unpack_from("<II", ids, 8 * index)
        if obj and sub & SEGMENT_BIT:
            out.setdefault((obj, sub ^ SEGMENT_BIT), []).append((index % width, index // width))
    return out


def background(px):
    """The level the ink is drawn over, as the green channel's mode across the frame."""
    counts = [0] * 256
    for green in px[1::3]:
        counts[green] += 1
    return counts.index(max(counts))


def hue(red, green, blue):
    """A stroke's colour by dominance, not by saturation: an antialiased pixel keeps the hue it
    was drawn in long after it stops being pure."""
    if red > green + 60 and red > blue + 60:
        return "red"
    if blue > red + 60 and blue > green + 60:
        return "blue"
    if red > green + 60 and blue > green + 60:
        return "magenta"
    return None


def colour(pixels, frame):
    """The group's colour is the hue most of its core pixels carry, so a few blended pixels
    where two strokes cross cannot rename an edge. `frame` is (width, pixel bytes)."""
    width, px = frame
    tally = {}
    for x, y in pixels:
        k = 3 * (y * width + x)
        name = hue(px[k], px[k + 1], px[k + 2])
        if name:
            tally[name] = tally.get(name, 0) + 1
    return max(tally, key=tally.get) if tally else None


def neighbours(pixel, size):
    """The pixel and its 8-connected halo, clipped to the frame `size` = (width, height): a
    coordinate off the frame has no colour to read, and indexing it would fold round to some
    other row's pixel."""
    width, height = size
    x, y = pixel
    return [(x + dx, y + dy) for dx in (-1, 0, 1) for dy in (-1, 0, 1)
            if 0 <= x + dx < width and 0 <= y + dy < height]


def fringe(core, claimed, key, size):
    """The core plus its halo, minus whatever another segment's core owns: the antialiasing
    shoulder belongs to this stroke, the neighbour it touches at a corner does not."""
    out = set(core)
    for pixel in core:
        for near in neighbours(pixel, size):
            if near not in out and claimed.get(near, key) == key:
                out.add(near)
    return out


def dilate(core, size):
    """The core plus its halo, with no regard for who else claims a pixel. The section test asks
    how even a stroke is, and a neighbour standing on its shoulder at a corner is not that
    stroke thinning."""
    out = set(core)
    for pixel in core:
        out.update(neighbours(pixel, size))
    return out


def magenta_pixels(px):
    """Hidden ink that surfaced, counted over the whole colour frame and not only where the id
    frame kept a stroke, so a leak under alpha 0.5 is still seen. The test is the pure-magenta
    channel one `_probe_matrix.py` uses rather than this file's hue dominance: a red stroke
    crossing a blue one blends to a purple that dominance calls magenta, measured at 1 to 11
    such pixels in every one of the twelve joint cases against zero by this test."""
    return sum(1 for k in range(0, len(px), 3) if px[k] >= 195 and px[k + 1] <= 60 and px[k + 2] >= 195)


def alpha(kind, level, red, green):
    """Coverage from the channel the ink lacks - green for red and magenta, red for blue - so a
    stroke over any background reads as the fraction of the pixel it took."""
    lacking = red if kind == "blue" else green
    return min(1.0, max(0.0, (level - lacking) / level))


def coverage(pixels, kind, level, frame):
    """The ink each pixel carries, keyed by pixel. `frame` is (width, pixel bytes)."""
    width, px = frame
    out = {}
    for x, y in pixels:
        k = 3 * (y * width + x)
        out[(x, y)] = alpha(kind, level, px[k], px[k + 1])
    return out


def measure(core, ink, wide):
    """Weight, thinnest interior cross-section over the median one - None for a stroke too short
    to be even - and whether the core is broken rather than merely dotted. Steps run along the
    major axis and the ends are dropped, because a cap is legitimately thinner than the shaft.
    Weight comes from the owned ink and evenness from the unowned `wide`: dropping ownership
    from the weight too would move the red-to-blue ratio by up to 5%."""
    xs = [p[0] for p in core]
    ys = [p[1] for p in core]
    dx, dy = max(xs) - min(xs) + 1, max(ys) - min(ys) + 1
    axis = 0 if dx >= dy else 1
    diagonal = (dx * dx + dy * dy) ** 0.5
    weight = sum(ink.values()) / diagonal
    steps = sorted({p[axis] for p in core})
    occluded = any(later - earlier > 2 for earlier, later in zip(steps, steps[1:]))
    # The interior is the core's, not the ink's: dilation adds a step of cap at each end, and a
    # cap carries a fraction of the shaft's ink whatever the stroke's weight.
    interior = set(steps[2:-2])
    sections = {}
    for pixel, value in wide.items():
        if pixel[axis] in interior:
            sections[pixel[axis]] = sections.get(pixel[axis], 0.0) + value
    if diagonal < SECTION_LENGTH or not sections:
        return weight, None, occluded
    counts = sorted(sections.values())
    median = counts[len(counts) // 2]
    return weight, min(counts) / median, occluded


def main():
    """Print the background level, the blue median, every red stroke's weight and its ratio to
    that median, the strokes left out as partly hidden, the worst cross-section and the magenta
    count; exit nonzero when any of the three rules breaks."""
    width, height, px = read_ppm(sys.argv[1])
    id_width, id_height, ids = read_ids(sys.argv[2])
    if (id_width, id_height) != (width, height):
        raise SystemExit(f"id frame is {id_width}x{id_height}, image is {width}x{height}")
    frame = (width, px)
    size = (width, height)
    level = background(px)
    magenta = magenta_pixels(px)
    cores = segments(width, height, ids)
    claimed = {pixel: key for key, pixels in cores.items() for pixel in pixels}
    red, blue, skipped = [], [], []
    for key, core in sorted(cores.items()):
        kind = colour(core, frame)
        if kind not in ("red", "blue") or len(core) < COUNT_FLOOR:
            continue
        ink = coverage(fringe(core, claimed, key, size), kind, level, frame)
        weight, section, occluded = measure(core, ink, coverage(dilate(core, size), kind, level, frame))
        if occluded:
            skipped.append(f"obj {key[0]} seg {key[1]} {kind}")
        else:
            (red if kind == "red" else blue).append((weight, section))
    print(f"background {level}; skipped as partly hidden: {', '.join(skipped) or 'none'}")
    if not red or not blue:
        print(f"FAIL: red {len(red)} blue {len(blue)} segments")
        sys.exit(1)
    blue_median = sorted(v[0] for v in blue)[len(blue) // 2]
    ratios = [v[0] / blue_median for v in red]
    sections = [v[1] for v in red + blue if v[1] is not None]
    worst_section = min(sections) if sections else 1.0
    print(f"blue: {len(blue)} segments, median weight {blue_median:.2f} px")
    print(f"red: {len(red)} segments, weights {[round(v[0], 2) for v in red]}")
    print(f"red ratios {[round(100 * r) for r in ratios]}%, worst {100 * min(ratios):.0f}% of blue")
    print(f"worst cross-section {100 * worst_section:.0f}% of {len(sections)} strokes "
          f"{SECTION_LENGTH} px or longer; magenta {magenta}")
    ok = min(ratios) >= RED_FLOOR and worst_section >= SECTION_FLOOR and magenta == 0
    print("weight OK" if ok else "FAIL")
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
