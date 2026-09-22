#!/usr/bin/env python3
"""Check the isolated grey box's red edges and seven attached dark vertex markers."""

import sys
from _count_colors import read_ppm


def components(pixels, width, height):
    """Group eight-connected marker pixels without merging across image boundaries."""
    remaining = set(pixels)
    groups = []
    while remaining:
        seed = remaining.pop()
        group, pending = [seed], [seed]
        while pending:
            pixel = pending.pop()
            x, y = pixel % width, pixel // width
            for row in range(max(0, y - 1), min(height, y + 2)):
                for column in range(max(0, x - 1), min(width, x + 2)):
                    neighbor = row * width + column
                    if neighbor in remaining:
                        remaining.remove(neighbor)
                        group.append(neighbor)
                        pending.append(neighbor)
        groups.append(group)
    return groups


def connected(first, second, red, width, height):
    """A visible box edge has red coverage along its axis between two vertex markers."""
    covered = 0
    for sample in range(1, 30):
        fraction = sample / 30
        x = round(first[0] + fraction * (second[0] - first[0]))
        y = round(first[1] + fraction * (second[1] - first[1]))
        hit = False
        for row in range(max(0, y - 3), min(height, y + 4)):
            for column in range(max(0, x - 3), min(width, x + 4)):
                hit |= row * width + column in red
        covered += hit
    return covered >= 27


def main():
    """The fixed isometric fixture exposes nine edges and seven original cube vertices."""
    width, height, rgb = read_ppm(sys.argv[1])
    red, dark = set(), set()
    for offset in range(0, len(rgb), 3):
        r, g, b = rgb[offset:offset + 3]
        if r > 180 and g < 100 and b < 100:
            red.add(offset // 3)
        if max(r, g, b) < 120:
            dark.add(offset // 3)
    markers = [group for group in components(dark, width, height) if len(group) >= 20]
    assert len(markers) == 7, f"expected seven visible vertex markers, found {len(markers)}"
    centres = [(sum(pixel % width for pixel in group) / len(group), sum(pixel // width for pixel in group) / len(group)) for group in markers]
    edges = sum(connected(first, second, red, width, height) for index, first in enumerate(centres) for second in centres[index + 1:])
    assert edges == 9, f"expected nine continuous red edges attached to markers, found {edges}"
    assert 43000 <= len(red) <= 48000, f"red edge coverage changed: {len(red)}"
    assert 700 <= len(dark) <= 1000, f"vertex marker coverage changed: {len(dark)}"
    print(f"close-up box: {edges} red edges, {len(markers)} dark vertices; red {len(red)}, dark {len(dark)} pixels")


if __name__ == "__main__":
    main()

# python3 tests/depth/_closeup_box.py /tmp/grey_box.ppm
