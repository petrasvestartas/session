#!/usr/bin/env python3
"""_orbit_check.py <selftest> <scene.pb> <out_dir> [--compare <other_out_dir>]

Renders the scene at 36 yaw steps of 10 degrees (35 orbit units at 0.005 rad each), counts
near-black pixels (every channel under 60) per frame, and fails when a frame differs from
either neighbour by more than 12% of the series mean: ink that flips with the view. Measured
on the BRep probe (cylinder, cone, curved NURBS surface, flat grey fills): mean 685, and
foreshortening alone moves the count by up to 6.1% between steps.

With --compare the same-numbered frames of an earlier run are loaded and their near-black masks
compared pixel by pixel; more than 0.5% of the series' near-black pixels differing fails. That
is the orientation test: the ok and flipped BRep probes differ only in the winding of two face
uses, so ink that reads a face's orientation would move whole edges, while ink that reads only
depth differs by the few edge pixels whose winning triangle changed with the triangle order."""
import os
import pathlib
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _count_colors import read_ppm

STEP = 35
FRAMES = 36


def black_mask(path):
    """One byte per pixel, 1 where every channel is under 60: the ink, taken apart from the
    grey fills and the white clear, as a mask that can be both counted and diffed."""
    w, h, px = read_ppm(path)
    return bytearray(1 if px[k] < 60 and px[k + 1] < 60 and px[k + 2] < 60 else 0 for k in range(0, w * h * 3, 3))


def parse_args(argv):
    """The binary, the scene, this run's output directory and the directory to compare against
    (None without --compare), which must already hold FRAMES frames from an earlier run."""
    other = None
    if "--compare" in argv:
        at = argv.index("--compare")
        other = pathlib.Path(argv[at + 1])
        argv = argv[:at] + argv[at + 2:]
    return argv[1], argv[2], pathlib.Path(argv[3]), other


def differing_pixels(out, other):
    """Pixels whose near-black verdict differs between the two runs, summed over all frames."""
    total = 0
    for i in range(FRAMES):
        name = f"orbit_{i:02d}.ppm"
        mine, theirs = black_mask(out / name), black_mask(other / name)
        if len(mine) != len(theirs):
            raise SystemExit(f"{name}: {len(mine)} pixels here, {len(theirs)} in {other}")
        total += sum(1 for a, b in zip(mine, theirs) if a != b)
    return total


def main():
    """Render the 36 frames, report the near-black series and its worst neighbour change, then
    the orientation diff when one was asked for; exit nonzero on either failure."""
    binary, scene, out, other = parse_args(sys.argv)
    out.mkdir(parents=True, exist_ok=True)
    base = {k: v for k, v in os.environ.items() if not k.startswith("VIEWER_")}
    series = []
    for i in range(FRAMES):
        ppm = out / f"orbit_{i:02d}.ppm"
        # Flat fills, because the mask must read ink and nothing else: the headlight shades a
        # face from its normal, so reversing a face use darkens it - measured 82 to 56 on the
        # cylinder's two reversed faces - and grey fill pixels cross the near-black threshold
        # where no ink moved at all. The headlight is off unless VIEWER_LIT asks for it.
        env = dict(base, VIEWER_W="1000", VIEWER_H="700", VIEWER_NO_GRID="1", VIEWER_NO_BACKFACE="1",
                   VIEWER_THICKNESS="1.5",
                   VIEWER_ORBIT=f"{i * STEP},0")
        subprocess.run([binary, str(ppm), scene], env=env, check=True, capture_output=True)
        series.append(sum(black_mask(ppm)))
    mean = sum(series) / len(series)
    worst = 0.0
    for i, value in enumerate(series):
        for j in (i - 1, (i + 1) % FRAMES):
            worst = max(worst, abs(value - series[j]) / mean)
    print("black pixels per orbit:", series)
    print(f"mean {mean:.0f}, worst neighbour change {100 * worst:.1f}%")
    if worst > 0.12:
        print("FAIL: ink flips with the view")
        sys.exit(1)
    if other is not None:
        count = differing_pixels(out, other)
        # Reversing a face use reorders that face's triangles; the finite-triangle visibility
        # test then picks a different winning primitive at a few edge pixels (measured 10 of
        # ~30000 near-black pixels over 36 frames). Ink that read the face orientation would
        # move whole edge chains: hundreds of pixels per frame. Allow 0.5% of the series total.
        total = sum(series)
        print(f"orientation diff against {other}: {count} differing pixels of {total}")
        if count > 0.005 * total:
            print("FAIL: ink depends on face orientation")
            sys.exit(1)
    print("orbit OK")


if __name__ == "__main__":
    main()
