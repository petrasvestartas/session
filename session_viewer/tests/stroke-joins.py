#!/usr/bin/env python3
"""Check joined vertices and subdivision opacity against source geometry, without goldens."""
import argparse
import json
import math
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
WIDTH, HEIGHT = 1200, 800


def dot(first, second):
    return sum(a * b for a, b in zip(first, second))


class Camera:
    """The native log supplies metres for the eye and millimetres for orthographic height."""
    def __init__(self, log):
        line = next(line.split("census camera: ", 1)[1] for line in log.splitlines()
                    if "census camera: " in line)
        fields = dict(part.split("=") for part in line.split())
        self.eye = [float(value) * 1000 for value in fields["CENSUS_EYE"].split(",")]
        self.forward = [float(value) for value in fields["CENSUS_FWD"].split(",")]
        self.up = [float(value) for value in fields["CENSUS_UP"].split(",")]
        f, u = self.forward, self.up
        self.right = [f[1]*u[2]-f[2]*u[1], f[2]*u[0]-f[0]*u[2], f[0]*u[1]-f[1]*u[0]]
        self.half_height = float(fields["CENSUS_ORTHO_H"])

    def project(self, point):
        delta = [value - eye for value, eye in zip(point, self.eye)]
        half_height = self.half_height or dot(delta, self.forward) * math.tan(math.pi / 6)
        return (WIDTH / 2 + HEIGHT / 2 * dot(delta, self.right) / half_height,
                HEIGHT / 2 - HEIGHT / 2 * dot(delta, self.up) / half_height)


def read_ppm(path):
    magic, size, maximum, pixels = path.read_bytes().split(b"\n", 3)
    assert magic == b"P6" and maximum == b"255"
    assert tuple(map(int, size.split())) == (WIDTH, HEIGHT)
    assert len(pixels) == WIDTH * HEIGHT * 3
    return pixels


def vertex_coverage(case, camera, pixels, selected):
    """Inspect actual joint cores, including the circle's historically missing diagonal tie."""
    inspected = set()
    for point in case["points"]:
        px, py = camera.project(point)
        # Ordinary one-pixel pens have weaker corner coverage; selected cores are wider.
        radius = 0.71 if selected else 0.55
        for y in range(math.floor(py) - 1, math.floor(py) + 2):
            for x in range(math.floor(px) - 1, math.floor(px) + 2):
                if not (0 <= x < WIDTH and 0 <= y < HEIGHT):
                    continue
                if math.hypot(x + 0.5 - px, y + 0.5 - py) <= radius:
                    inspected.add(y * WIDTH + x)
    assert inspected, "fixture has no projected vertex samples"
    missing = []
    for index in inspected:
        red, green, blue = pixels[index*3:index*3+3]
        visible = red >= 220 and green >= 220 and blue <= 100 if selected else min(red, green) < 247
        if not visible:
            missing.append([index % WIDTH, index // WIDTH])
    return {"samples": len(inspected), "missing": missing}


def render(command, environment, log):
    result = subprocess.run(command, env=environment, capture_output=True, text=True)
    output = result.stdout + result.stderr
    log.write_text(output)
    result.check_returncode()
    assert "[ERROR]" not in output and "panicked" not in output, log
    return output


def main():
    target = Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target"))
    examples = target / "x86_64-unknown-linux-gnu/debug/examples"
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--renderer", type=Path, default=examples / "selftest")
    parser.add_argument("--generator", type=Path, default=examples / "mk_stroke_joins")
    parser.add_argument("--output", type=Path, default=target / "stroke-joins")
    parser.add_argument("--check-only", action="store_true", help="recheck existing captures")
    options = parser.parse_args()
    output = options.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    fixtures = output / "fixtures"
    environment = {key: value for key, value in os.environ.items() if not key.startswith("VIEWER_")}
    if not options.check_only:
        render([str(options.generator.resolve()), str(fixtures)], environment, output / "generator.log")
    cases = json.loads((fixtures / "cases.json").read_text())
    rows = []
    for view, camera_knobs in [("top", {"VIEWER_VIEW": "top"}), ("iso", {})]:
        for samples in [1, 4]:
            for selected in [False, True]:
                captures = {}
                for case in cases:
                    stem = f'{case["name"]}-{view}-{samples}-{int(selected)}'
                    image, log = output / (stem + ".ppm"), output / (stem + ".log")
                    if not options.check_only:
                        knobs = dict(VIEWER_W=str(WIDTH), VIEWER_H=str(HEIGHT), VIEWER_NO_GRID="1",
                                     BENCH_NO_MARKERS="1", VIEWER_MSAA=str(samples), **camera_knobs)
                        if selected:
                            knobs["VIEWER_SELECT"] = "joined stroke"
                        render([str(options.renderer.resolve()), str(image),
                                str(fixtures / (case["name"] + ".pb"))],
                               dict(environment, **knobs), log)
                    pixels, camera = read_ppm(image), Camera(log.read_text())
                    measured = vertex_coverage(case, camera, pixels, selected)
                    rows.append(dict(case=stem, passed=not measured["missing"], **measured))
                    captures[case["name"]] = (pixels, camera)
                single, dense = captures["straight"], captures["dense_straight"]
                assert vars(single[1]) == vars(dense[1]), "paired fixtures changed camera"
                # Integrated ink detects cap overlap that gets darker when a straight line
                # is subdivided. A bounded allowance includes existing MSAA fringe thinning.
                def ink(data):
                    return sum(255 - min(data[index:index+3]) for index in range(0, len(data), 3))
                ratio = ink(dense[0]) / ink(single[0])
                rows.append(dict(case=f"opacity-{view}-{samples}-{int(selected)}",
                                 passed=0.90 <= ratio <= 1.08, dense_single_ink_ratio=ratio))
    (output / "results.json").write_text(json.dumps(rows, indent=2) + "\n")
    failures = [row for row in rows if not row["passed"]]
    for row in failures:
        print(json.dumps(row))
    print(f"{len(rows) - len(failures)}/{len(rows)} stroke-join checks pass (32 captures)")
    raise SystemExit(bool(failures))


if __name__ == "__main__":
    main()
