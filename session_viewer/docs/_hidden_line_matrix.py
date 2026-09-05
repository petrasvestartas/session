#!/usr/bin/env python3
"""Render the task camera matrix and census actual original-scene object/segment IDs.

Example (executables must be built from the intended before/after source):
  python3 docs/_hidden_line_matrix.py SELFTEST CENSUS_PLATES FLOOR.pb OUT --require-zero

Each census receives the camera actually logged by its render. The JSON records only
render/census knobs, source/binary hashes, and results; it never records the process environment.
"""

import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess


CAMERAS = {
    "iso": {},
    "down": {"VIEWER_ORBIT": "0,209"},
    "tilted": {"VIEWER_ORBIT": "0,150"},
    "side": {"VIEWER_ORBIT": "200,60"},
    "far2p6": {"VIEWER_ORBIT": "0,209", "VIEWER_ZOOM": "-10"},
    "far4p3": {"VIEWER_ORBIT": "0,209", "VIEWER_ZOOM": "-15"},
    "top": {"VIEWER_VIEW": "top"},
}


def digest(path):
    """Identify the actual executable/fixture rather than its mutable filename."""
    value = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            value.update(chunk)
    return value.hexdigest()


def execute(args, environment, log):
    """Preserve the complete diagnostic log and fail on validation or census errors."""
    result = subprocess.run(args, env=environment, text=True, capture_output=True)
    output = result.stdout + result.stderr
    log.write_text(output)
    if result.returncode:
        raise RuntimeError(f"exit {result.returncode}: inspect {log}\n{output[-4000:]}")
    return output


def main():
    """Run all 42 cases by default; optional filters support focused diagnosis."""
    parser = argparse.ArgumentParser(description=__doc__)
    for argument in ("renderer", "census", "scene", "output"):
        parser.add_argument(argument, type=Path)
    parser.add_argument("--require-zero", action="store_true", help="fail if any fully covered original-source core pixel surfaces")
    parser.add_argument("--resume", action="store_true", help="reuse completed cases only when all executable/fixture hashes match")
    parser.add_argument("--camera", choices=CAMERAS)
    parser.add_argument("--scale", type=int, choices=(1, 4, 16))
    options = parser.parse_args()
    paths = {key: getattr(options, key).resolve() for key in ("renderer", "census", "scene", "output")}
    paths["output"].mkdir(parents=True, exist_ok=True)
    metadata = {key: {"path": str(paths[key]), "sha256": digest(paths[key])} for key in ("renderer", "census", "scene")}
    metadata["require_zero"] = options.require_zero
    destination = paths["output"] / "matrix.json"
    report = {"metadata": metadata, "results": []}
    if options.resume and destination.exists():
        report = json.loads(destination.read_text())
        if report["metadata"] != metadata:
            raise RuntimeError("resume requires identical renderer, census, scene and zero requirement")
    done = {(row["camera"], row["scale"], row["style"]) for row in report["results"]}
    environment = {key: value for key, value in os.environ.items() if not key.startswith(("VIEWER_", "CENSUS_"))}
    for camera, settings in CAMERAS.items():
        if options.camera and camera != options.camera:
            continue
        for scale in (1, 4, 16):
            if options.scale and scale != options.scale:
                continue
            for style in ("flat", "tubes"):
                if (camera, scale, style) in done:
                    continue
                stem = f"{camera}_{scale}_{style}"
                output = paths["output"]
                knobs = dict(settings, VIEWER_W="1800", VIEWER_H="1400", VIEWER_MSAA="4", VIEWER_DISTANCE_SCALE=str(scale), VIEWER_LINE_STYLE=style, VIEWER_IDS=str(output / f"{stem}.ids"))
                rendered = execute([paths["renderer"], output / f"{stem}.ppm", paths["scene"]], dict(environment, **knobs), output / f"{stem}.log")
                camera_log = next(line.split("census camera: ", 1)[1] for line in rendered.splitlines() if "census camera: " in line)
                actual_camera = dict(part.split("=", 1) for part in camera_log.split())
                census_knobs = dict(actual_camera, CENSUS_RENDERED_IDS=knobs["VIEWER_IDS"])
                if options.require_zero:
                    census_knobs["CENSUS_REQUIRE_ZERO"] = "1"
                measured = execute([paths["census"], paths["scene"]], dict(environment, **knobs, **census_knobs), output / f"{stem}_census.log")
                result = next(line for line in measured.splitlines() if line.startswith("RENDERED_IDS covered samples"))
                report["results"].append(dict(camera=camera, scale=scale, style=style, result=result, viewer=knobs, census=census_knobs))
                destination.write_text(json.dumps(report, indent=2) + "\n")
                print(f"{stem}: {result}", flush=True)
    print(f"Recorded {len(report['results'])} completed cases in {destination}")


if __name__ == "__main__":
    main()
