#!/usr/bin/env python3
"""Require 766/766 exposed seam pixels and zero hidden ink for finite-triangle visibility."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
WIDTH, HEIGHT = 1200, 900
EXPECTED_CORE = 766


def read_ppm(path):
    """Read the native RGB8 capture without third-party image packages."""
    magic, size, maximum, pixels = path.read_bytes().split(b"\n", 3)
    assert magic == b"P6" and maximum == b"255", path
    assert tuple(map(int, size.split())) == (WIDTH, HEIGHT), path
    assert len(pixels) == WIDTH * HEIGHT * 3, path
    return pixels


def black(pixels, index):
    """Classify the opaque black pen separately from the grey faces and white background."""
    return max(pixels[3 * index:3 * index + 3]) < 125


def camera(log):
    """Require the clean and nearby captures to use the same actually rendered camera."""
    return next(line.split("census camera: ", 1)[1] for line in log.splitlines()
                if "census camera: " in line)


def run(command, environment, log):
    """Keep adapter, camera and validation diagnostics alongside each capture."""
    result = subprocess.run(command, env=environment, capture_output=True, text=True)
    output = result.stdout + result.stderr
    log.write_text(output)
    result.check_returncode()
    assert "[ERROR]" not in output and "panicked" not in output, log
    return output


def check(output):
    """Keep the original interior reference band and whole-image hidden-ink assertion."""
    clean = read_ppm(output / "clean.ppm")
    nearby = read_ppm(output / "nearby.ppm")
    hidden = read_ppm(output / "hidden.ppm")
    assert camera((output / "clean.log").read_text()) == camera((output / "nearby.log").read_text())
    # The fixed 1200x900 perspective fixture puts an interior part of the straight seam
    # in these rows. Ignore its endpoints; do not crop the hidden scene or tolerate leaks.
    core = [index for index in range(WIDTH * HEIGHT)
            if 430 < index // WIDTH < 697 and black(clean, index)]
    retained = sum(black(nearby, index) for index in core)
    leaks = sum(black(hidden, index) for index in range(WIDTH * HEIGHT))
    measured = {"expected_visible_core": EXPECTED_CORE, "visible_core": len(core),
                "retained": retained, "hidden_black_pixels": leaks,
                "passed": len(core) == EXPECTED_CORE and retained == EXPECTED_CORE and leaks == 0}
    print(f"Visible shared edge: {retained}/{len(core)} core samples retained")
    print(f"Covered shared edge: {leaks} black pixels in the whole image")
    return measured


def main():
    """Generate and render three fixed scenes, or recheck existing captures."""
    target = Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target"))
    examples = target / "x86_64-unknown-linux-gnu/debug/examples"
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--renderer", type=Path, default=examples / "selftest")
    parser.add_argument("--generator", type=Path, default=examples / "mk_triangle_visibility")
    parser.add_argument("--output", type=Path, default=target / "triangle-visibility")
    parser.add_argument("--check-only", action="store_true", help="recheck existing captures")
    options = parser.parse_args()
    output = options.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    # The exact 766-sample oracle was measured with a 1.5 px pen, independent of UI defaults.
    knobs = {"VIEWER_W": str(WIDTH), "VIEWER_H": str(HEIGHT), "VIEWER_MSAA": "1",
             "VIEWER_THICKNESS": "1.5",
             "VIEWER_NO_OUTLINES": "1", "VIEWER_NO_GRID": "1", "VIEWER_NO_BACKFACE": "1",
             "BENCH_NO_MARKERS": "1"}
    environment = {key: value for key, value in os.environ.items() if not key.startswith("VIEWER_")}
    environment.update(knobs)
    if not options.check_only:
        run([str(options.generator.resolve()), str(output)], environment, output / "generator.log")
        for name in ["clean", "nearby", "hidden"]:
            run([str(options.renderer.resolve()), str(output / (name + ".ppm")),
                 str(output / (name + ".pb"))], environment, output / (name + ".log"))
    measured = check(output)
    measured["capture_settings"] = knobs
    measured["fixture_sha256"] = {name: hashlib.sha256((output / (name + ".pb")).read_bytes()).hexdigest()
                                  for name in ["clean", "nearby", "hidden"]}
    (output / "results.json").write_text(json.dumps(measured, indent=2) + "\n")
    raise SystemExit(not measured["passed"])


if __name__ == "__main__":
    main()
