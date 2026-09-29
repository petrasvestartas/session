#!/usr/bin/env python3
"""count_colors.py <ppm> [tol]  ->  "blue N magenta N"

Counts pixels of a binary P6 PPM whose every channel is within `tol` (default 60/255) of pure
blue (0,0,255) or pure magenta (255,0,255)."""
import sys


def read_ppm(path):
    with open(path, "rb") as f:
        data = f.read()
    tokens = []
    i = 0
    while len(tokens) < 4:
        while data[i:i + 1].isspace():
            i += 1
        if data[i:i + 1] == b"#":
            while data[i:i + 1] not in (b"\n", b""):
                i += 1
            continue
        j = i
        while not data[j:j + 1].isspace():
            j += 1
        tokens.append(data[i:j])
        i = j
    i += 1
    assert tokens[0] == b"P6", tokens[0]
    w, h = int(tokens[1]), int(tokens[2])
    return w, h, data[i:i + w * h * 3]


def main():
    path = sys.argv[1]
    tol = int(sys.argv[2]) if len(sys.argv) > 2 else 60
    w, h, px = read_ppm(path)
    blue = magenta = 0
    for k in range(0, w * h * 3, 3):
        r, g, b = px[k], px[k + 1], px[k + 2]
        if g <= tol and b >= 255 - tol:
            if r <= tol:
                blue += 1
            elif r >= 255 - tol:
                magenta += 1
    print(f"blue {blue} magenta {magenta}")


if __name__ == "__main__":
    main()
