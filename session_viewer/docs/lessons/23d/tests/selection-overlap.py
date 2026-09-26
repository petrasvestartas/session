#!/usr/bin/env python3
"""Compare selected yellow cores with identical-geometry controls in forty native views."""
import argparse
import json
import math
import os
from pathlib import Path
import subprocess


ROOT = Path(__file__).resolve().parents[1]
WIDTH, HEIGHT = 1200, 800
RETENTION = 0.97
CAMERAS = {
    "top": {"VIEWER_VIEW": "top"},
    "iso": {},
    "steep": {"VIEWER_ORBIT": "0,80"},
    "side": {"VIEWER_ORBIT": "300,120"},
}


def run_logged(command, log, environment):
    """Retain native adapter/validation evidence even if fixture generation or rendering fails."""
    with log.open("w") as handle:
        subprocess.run(command, env=environment, stdout=handle, stderr=handle, check=True)


def read_ppm(path):
    """Read the native RGB8 capture without a third-party image dependency."""
    magic, size, maximum, data = path.read_bytes().split(b"\n", 3)
    assert magic == b"P6" and maximum == b"255"
    assert tuple(map(int, size.split())) == (WIDTH, HEIGHT)
    assert len(data) == WIDTH * HEIGHT * 3
    return data


def yellow(data, index):
    """Require actual yellow core coverage, excluding white backgrounds and black silhouettes."""
    red, green, blue = data[index * 3:index * 3 + 3]
    return red >= 220 and green >= 220 and blue <= 100


def dot(first, second):
    """The camera log and fixture coordinates share one Cartesian projection."""
    return sum(a * b for a, b in zip(first, second))


class Camera:
    """Project original millimetre coordinates using the camera actually rendered."""

    def __init__(self, log):
        """Eye coordinates are logged in metres; orthographic half-height is already in mm."""
        line = next(line.split("census camera: ", 1)[1] for line in log.splitlines()
                    if "census camera: " in line)
        knobs = dict(part.split("=") for part in line.split())
        self.eye = [float(value) * 1000 for value in knobs["CENSUS_EYE"].split(",")]
        self.forward = [float(value) for value in knobs["CENSUS_FWD"].split(",")]
        self.up = [float(value) for value in knobs["CENSUS_UP"].split(",")]
        forward, up = self.forward, self.up
        self.right = [forward[1] * up[2] - forward[2] * up[1],
                      forward[2] * up[0] - forward[0] * up[2],
                      forward[0] * up[1] - forward[1] * up[0]]
        self.half_height = float(knobs["CENSUS_ORTHO_H"])

    def project(self, point):
        """Return a source position in physical framebuffer pixels."""
        delta = [point[index] - self.eye[index] for index in range(3)]
        half_height = self.half_height or dot(delta, self.forward) * math.tan(math.pi / 6)
        return [WIDTH * 0.5 + HEIGHT * 0.5 * dot(delta, self.right) / half_height,
                HEIGHT * 0.5 - HEIGHT * 0.5 * dot(delta, self.up) / half_height]

    def segment_pixels(self, segment, radius=2):
        """Inspect a bounded strip around the source axis, excluding endpoints by fixture design."""
        first, second = [self.project(point) for point in segment]
        dx, dy = second[0] - first[0], second[1] - first[1]
        length_squared = dx * dx + dy * dy
        assert length_squared > 0
        columns = range(max(0, int(min(first[0], second[0]) - radius - 1)),
                        min(WIDTH, int(max(first[0], second[0]) + radius + 2)))
        rows = range(max(0, int(min(first[1], second[1]) - radius - 1)),
                     min(HEIGHT, int(max(first[1], second[1]) + radius + 2)))
        for y in rows:
            for x in columns:
                along = ((x + 0.5 - first[0]) * dx + (y + 0.5 - first[1]) * dy) / length_squared
                distance_squared = ((x + 0.5 - first[0] - along * dx) ** 2
                                    + (y + 0.5 - first[1] - along * dy) ** 2)
                if 0 <= along <= 1 and distance_squared <= radius * radius:
                    yield y * WIDTH + x


def retained_core(actual, reference, pixels):
    """Black competing strokes must not remove the yellow present in the control capture."""
    core = [index for index in pixels if yellow(reference, index)]
    retained = sum(yellow(actual, index) for index in core)
    return {"reference": len(core), "retained": retained,
            "fraction": retained / max(1, len(core))}


def measure(case, actual, reference, camera):
    """Check every overlap, a tight crossing window, and strictly zero yellow behind the solid."""
    scores = [retained_core(actual, reference, camera.segment_pixels(segment))
              for segment in case["segments"]]
    if case["focus"]:
        x, y = camera.project(case["focus"])
        pixels = [row * WIDTH + column
                  for row in range(max(0, int(y) - 2), min(HEIGHT, int(y) + 3))
                  for column in range(max(0, int(x) - 2), min(WIDTH, int(x) + 3))]
        scores.append(dict(focus=True, **retained_core(actual, reference, pixels)))
    leaks = 0
    if case["hidden"]:
        leaks = sum(yellow(actual, index)
                    for index in camera.segment_pixels(case["hidden"], radius=1.25))
    passed = all(score["reference"] >= 3 and score["fraction"] >= RETENTION
                 for score in scores) and leaks == 0
    return {"passed": passed, "segments": scores, "hidden_yellow": leaks}


def captures(options, fixtures, case, stem, environment):
    """Reference geometry and occlusion stay identical; only competing unselected ink changes."""
    images, cameras = [], []
    for suffix in ("", "-reference"):
        label = stem + suffix
        image = options.output / f"{label}.ppm"
        log = options.output / f"{label}.log"
        if not options.check_only:
            run_logged([options.renderer, image, fixtures / f'{case["kind"]}{suffix}.pb'],
                       log, environment)
        text = log.read_text()
        assert "[ERROR]" not in text and "panicked" not in text, log
        images.append(read_ppm(image))
        cameras.append(Camera(text))
    assert vars(cameras[0]) == vars(cameras[1]), "control changed the framing or physical geometry"
    return images[0], images[1], cameras[0]


def main():
    """Generate through the maintained Cargo example, then run all five fixtures in eight views."""
    target = Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target")).resolve()
    binaries = target / "x86_64-unknown-linux-gnu/debug/examples"
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--renderer", type=Path, default=binaries / "selftest")
    parser.add_argument("--generator", type=Path, default=binaries / "mk_selection_overlap")
    parser.add_argument("--output", type=Path, default=target / "selection-overlap")
    parser.add_argument("--check-only", action="store_true", help="recheck retained captures without rendering")
    options = parser.parse_args()
    options.renderer = options.renderer.resolve()
    options.generator = options.generator.resolve()
    options.output = options.output.resolve()
    options.output.mkdir(parents=True, exist_ok=True)
    fixtures = options.output / "fixtures"
    # Do not inherit a camera, selection or outline-disable knob from an unrelated harness.
    environment = {key: value for key, value in os.environ.items() if not key.startswith("VIEWER_")}
    if not options.check_only:
        run_logged([options.generator, fixtures], options.output / "fixture.log", environment)
    cases = json.loads((fixtures / "cases.json").read_text())
    assert len(cases) == 5
    results = []
    for case in cases:
        for name, settings in CAMERAS.items():
            for samples in (1, 4):
                stem = f'{case["kind"]}-{name}-{samples}'
                knobs = dict(environment, VIEWER_SELECT="selected target", VIEWER_NO_GRID="1", VIEWER_OUTLINES="1",
                             BENCH_NO_MARKERS="1", VIEWER_NO_BACKFACE="1", VIEWER_MSAA=str(samples),
                             VIEWER_W=str(WIDTH), VIEWER_H=str(HEIGHT), **settings)
                actual, reference, camera = captures(options, fixtures, case, stem, knobs)
                row = dict(case=case["kind"], camera=name, msaa=samples,
                           **measure(case, actual, reference, camera))
                results.append(row)
                print(json.dumps(row), flush=True)
                (options.output / "results.json").write_text(json.dumps(results, indent=2) + "\n")
    passed = sum(row["passed"] for row in results)
    print(f"{passed}/{len(results)} selected-overlap cases pass")
    raise SystemExit(passed != len(results))


if __name__ == "__main__":
    main()
