#!/usr/bin/env python3
"""_shade_scanline.py <ppm> [--max-second-diff N] [--max-backface N]

Two facts about one frame of the shade probe:

  second_diff <max>   the largest absolute second difference of luminance along the scanline
                      through the centre of the frame's non-white bounding box, taken over the
                      first run of non-white pixels on that row with 4 px trimmed at each end
                      (the antialiased silhouette). A flat-shaded sphere steps at every facet
                      boundary; a smooth one is a gradient whose 8-bit quantisation gives
                      second differences of 1 or 2.
  backface <count>    pixels within WINDOW of the red the frame holds for a back face,
                      (231, 62, 62): a face wound inside out, or the inside of an open shell
                      seen through its opening.

Floors are measured, never assumed: the numbers in the ink suite are the ones this script
printed on the day they were written. Exits 1 when a floor is exceeded."""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _count_colors import read_ppm

TRIM = 4

# The shader's BACKFACE_COLOR is linear (0.80, 0.05, 0.05) and the frame is sRGB, so the bytes
# it writes are not 255 * that: measured (231, 62, 62) on the teapot's top view, 2026-09-08.
# The old linear constant (204, 13, 13) was 49 away on green and blue and matched no pixel any
# frame could hold, which made every back-face check blind.
BACKFACE = (231, 62, 62)

# The window is 20, not the 30 this check carried while it matched nothing: the mixed scene
# draws a Color::red() curve on the box top, and antialiasing that pure red against the pale
# face gives (254, 41, 39) - 23, 21 and 23 from the back-face red, inside a window of 30 and
# outside one of 20. Measured 2026-09-08: at 30 the mixed top counts 19 curve pixels and no
# back face at all, at 20 it counts 0; the teapot's top view keeps 6048 of its 6291, so the
# narrower window still sees a back face wherever there is one.
WINDOW = 20


def luminance(px, k):
    """Rec. 709 luma of pixel k of a packed RGB byte string."""
    return 0.2126 * px[k] + 0.7152 * px[k + 1] + 0.0722 * px[k + 2]


def non_white(px, k):
    """The white clear is (255, 255, 255); anything darker in any channel is the object."""
    return px[k] < 250 or px[k + 1] < 250 or px[k + 2] < 250


def bbox(w, h, px):
    """Rows and columns that hold a non-white pixel: (y0, y1, x0, x1), inclusive."""
    y0, y1, x0, x1 = h, -1, w, -1
    for y in range(h):
        for x in range(w):
            if non_white(px, 3 * (y * w + x)):
                y0, y1, x0, x1 = min(y0, y), max(y1, y), min(x0, x), max(x1, x)
    return y0, y1, x0, x1


def scanline(w, px, y):
    """Luma along the first non-white run of row y, trimmed by TRIM px at both ends."""
    xs = [x for x in range(w) if non_white(px, 3 * (y * w + x))]
    if not xs:
        return []
    run = [xs[0]]
    for x in xs[1:]:
        if x != run[-1] + 1:
            break
        run.append(x)
    run = run[TRIM:len(run) - TRIM]
    return [luminance(px, 3 * (y * w + x)) for x in run]


def second_diff(lum):
    """The largest |l[i-1] - 2 l[i] + l[i+1]| along the line, 0 for fewer than three samples."""
    best = 0.0
    for i in range(1, len(lum) - 1):
        best = max(best, abs(lum[i - 1] - 2.0 * lum[i] + lum[i + 1]))
    return best


def backface(px):
    """Pixels within WINDOW of BACKFACE in every channel."""
    n = 0
    for k in range(0, len(px), 3):
        if all(abs(px[k + c] - BACKFACE[c]) <= WINDOW for c in range(3)):
            n += 1
    return n


def main():
    args = sys.argv[1:]
    path = args[0]
    max_sd = float(args[args.index("--max-second-diff") + 1]) if "--max-second-diff" in args else None
    max_bf = int(args[args.index("--max-backface") + 1]) if "--max-backface" in args else None
    w, h, px = read_ppm(path)
    y0, y1, _, _ = bbox(w, h, px)
    if y1 < 0:
        print("empty frame")
        return 1
    lum = scanline(w, px, (y0 + y1) // 2)
    sd = second_diff(lum)
    bf = backface(px)
    print(f"second_diff {sd:.2f} over {len(lum)} px")
    print(f"backface {bf}")
    bad = (max_sd is not None and sd > max_sd) or (max_bf is not None and bf > max_bf)
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
