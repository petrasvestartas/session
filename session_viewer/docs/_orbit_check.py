#!/usr/bin/env python3
"""_orbit_check.py <selftest> <scene.pb> <out_dir>  ->  black-pixel series over 36 orbits

Renders the scene at 36 yaw steps of 10 degrees (35 orbit units at 0.005 rad each), counts
near-black pixels (every channel under 60) per frame, and fails when a frame differs from
either neighbour by more than 12% of the series mean: ink that flips with the view. Measured
on the BRep probe (cylinder, cone, curved NURBS surface, grey fills): foreshortening alone
moves the count by up to 8.1% between steps."""
import os, pathlib, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _count_colors import read_ppm

STEP = 35
FRAMES = 36


def black_pixels(path):
    w, h, px = read_ppm(path)
    return sum(1 for k in range(0, w * h * 3, 3) if px[k] < 60 and px[k + 1] < 60 and px[k + 2] < 60)


def main():
    binary, scene, out = sys.argv[1], sys.argv[2], pathlib.Path(sys.argv[3])
    out.mkdir(parents=True, exist_ok=True)
    base = {k: v for k, v in os.environ.items() if not k.startswith("VIEWER_")}
    series = []
    for i in range(FRAMES):
        ppm = out / f"orbit_{i:02d}.ppm"
        env = dict(base, VIEWER_W="1000", VIEWER_H="700", VIEWER_NO_GRID="1", VIEWER_NO_BACKFACE="1",
                   VIEWER_ORBIT=f"{i * STEP},0")
        subprocess.run([binary, str(ppm), scene], env=env, check=True, capture_output=True)
        series.append(black_pixels(ppm))
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
    print("orbit OK")


if __name__ == "__main__":
    main()
