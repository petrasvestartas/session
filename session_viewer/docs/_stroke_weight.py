#!/usr/bin/env python3
"""_stroke_weight.py <ppm> <ids>  ->  the weight of every red (joint) and blue (free) stroke

The strokes come from the id frame the renderer writes under VIEWER_IDS, not from colour blobs:
a 1 px stroke is saturated only along its core and not even there where it steps a column, so a
colour component finder reports one edge as a handful of dashes. One id group is one straight
edge of one object, whatever antialiasing did to it, and its weight is its pixel count over its
bounding-box diagonal, the edge's projected length. Fails when a red stroke weighs under 90% of
the blue median, when an interior cross-section is under 80% of its stroke's median, or when a
magenta pixel survives. Magenta is counted over every group, the 12 px floor included, because
the rule there is zero, not a weight."""
import os
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _count_colors import read_ppm

SEGMENT_BIT = 0x80000000
FLOOR = 12


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


def colour(pixels, width, px):
    """The group's colour is the hue most of its pixels carry, so a few blended pixels where two
    strokes cross cannot rename an edge."""
    tally = {}
    for x, y in pixels:
        k = 3 * (y * width + x)
        name = hue(px[k], px[k + 1], px[k + 2])
        if name:
            tally[name] = tally.get(name, 0) + 1
    return max(tally, key=tally.get) if tally else None


def measure(comp):
    """Weight, and the thinnest interior cross-section as a fraction of the median one. The ends
    are dropped because a cap is legitimately thinner than the shaft."""
    xs, ys = [p[0] for p in comp], [p[1] for p in comp]
    dx, dy = max(xs) - min(xs) + 1, max(ys) - min(ys) + 1
    weight = len(comp) / (dx * dx + dy * dy) ** 0.5
    axis = 0 if dx >= dy else 1
    sections = {}
    for p in comp:
        sections[p[axis]] = sections.get(p[axis], 0) + 1
    keys = sorted(sections)[2:-2]
    if not keys:
        return weight, 1.0
    counts = sorted(sections[k] for k in keys)
    median = counts[len(counts) // 2]
    return weight, min(counts) / median


def main():
    """Print the blue median, every red stroke's weight and its ratio to that median, the worst
    cross-section and the magenta count; exit nonzero when any of the three rules breaks."""
    width, height, px = read_ppm(sys.argv[1])
    id_width, id_height, ids = read_ids(sys.argv[2])
    if (id_width, id_height) != (width, height):
        raise SystemExit(f"id frame is {id_width}x{id_height}, image is {width}x{height}")
    red, blue, magenta = [], [], 0
    for pixels in segments(width, height, ids).values():
        kind = colour(pixels, width, px)
        if kind == "magenta":
            magenta += len(pixels)
        elif len(pixels) < FLOOR:
            continue
        elif kind == "red":
            red.append(measure(pixels))
        elif kind == "blue":
            blue.append(measure(pixels))
    if not red or not blue:
        print(f"FAIL: red {len(red)} blue {len(blue)} segments")
        sys.exit(1)
    blue_median = sorted(v[0] for v in blue)[len(blue) // 2]
    ratios = [v[0] / blue_median for v in red]
    worst_section = min(v[1] for v in red + blue)
    print(f"blue: {len(blue)} segments, median weight {blue_median:.2f} px/px")
    print(f"red: {len(red)} segments, weights {[round(v[0], 2) for v in red]}")
    print(f"red ratios {[round(100 * r) for r in ratios]}%, worst {100 * min(ratios):.0f}% of blue")
    print(f"worst cross-section {100 * worst_section:.0f}% of its stroke's median; magenta {magenta}")
    ok = min(ratios) >= 0.9 and worst_section >= 0.8 and magenta == 0
    print("weight OK" if ok else "FAIL")
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
