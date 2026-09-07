#!/usr/bin/env python3
"""_stroke_weight.py <ppm>  ->  stroke weights of the red (joint) and blue (free) edges

A stroke is a 4-connected component of saturated red or blue pixels. Its weight is its pixel
count over its bounding-box diagonal (a straight edge's projected length). Fails when a red
component weighs under 90% of the blue median, when any interior cross-section of a component
is under 80% of that component's median, or when a magenta pixel exists."""
import os, sys
from collections import deque
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _count_colors import read_ppm


def classify(r, g, b):
    if r >= 180 and g <= 90 and b <= 90:
        return "red"
    if b >= 180 and r <= 90 and g <= 90:
        return "blue"
    if r >= 180 and g <= 90 and b >= 180:
        return "magenta"
    return None


def components(w, h, cells, kind):
    seen, out = set(), []
    for start in [p for p, k in cells.items() if k == kind]:
        if start in seen:
            continue
        queue, comp = deque([start]), []
        seen.add(start)
        while queue:
            x, y = queue.popleft()
            comp.append((x, y))
            for nx, ny in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)):
                if (nx, ny) in cells and cells[(nx, ny)] == kind and (nx, ny) not in seen:
                    seen.add((nx, ny))
                    queue.append((nx, ny))
        if len(comp) >= 12:
            out.append(comp)
    return out


def measure(comp):
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
    w, h, px = read_ppm(sys.argv[1])
    cells = {}
    for y in range(h):
        for x in range(w):
            k = 3 * (y * w + x)
            kind = classify(px[k], px[k + 1], px[k + 2])
            if kind:
                cells[(x, y)] = kind
    magenta = sum(1 for k in cells.values() if k == "magenta")
    red = [measure(c) for c in components(w, h, cells, "red")]
    blue = [measure(c) for c in components(w, h, cells, "blue")]
    if not red or not blue:
        print(f"FAIL: red {len(red)} blue {len(blue)} components")
        sys.exit(1)
    blue_median = sorted(v[0] for v in blue)[len(blue) // 2]
    worst_red = min(v[0] for v in red) / blue_median
    worst_section = min(v[1] for v in red + blue)
    print(f"blue: {len(blue)} edges, median weight {blue_median:.2f} px/px")
    print(f"red: {len(red)} edges, weights {[round(v[0], 2) for v in red]}, worst {100 * worst_red:.0f}% of blue")
    print(f"worst cross-section {100 * worst_section:.0f}% of its edge's median; magenta {magenta}")
    ok = worst_red >= 0.9 and worst_section >= 0.8 and magenta == 0
    print("weight OK" if ok else "FAIL")
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
