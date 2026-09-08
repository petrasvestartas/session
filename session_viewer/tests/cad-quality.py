#!/usr/bin/env python3
"""Native CAD images and retained edge IDs; artifacts stay outside tracked assets."""
import argparse
import json
import os
from pathlib import Path
import subprocess


def run_logged(command, path, environment):
    """Keep adapter and validation evidence with each unmodified native capture."""
    with path.open("w") as log:
        subprocess.run(command, env=environment, stdout=log, stderr=log, check=True)


def read_ppm(path):
    """Read the harness's exact RGB8 P6 image without a third-party image package."""
    header, size, maximum, data = path.read_bytes().split(b"\n", 3)
    assert header == b"P6" and maximum == b"255"
    width, height = map(int, size.split())
    assert len(data) == width * height * 3
    return width, height, data


def row_values(path):
    """Sample the smooth sphere interior, away from silhouette coverage and seam ink."""
    width, height, data = read_ppm(path)
    assert (width, height) == (700, 520)
    return [data[(260 * width + x) * 3] for x in range(270, 430)]


def verify(output):
    """Check actual shading variation, flat unlit output and absence of reversed solid faces."""
    lit = row_values(output / "sphere-fill.ppm")
    unlit = row_values(output / "sphere-unlit.ppm")
    maximum_step = max(abs(a - b) for a, b in zip(lit, lit[1:]))
    assert maximum_step <= 3 and max(lit) - min(lit) >= 5
    assert len(set(unlit)) == 1
    for path in output.glob("*.ppm"):
        _, _, data = read_ppm(path)
        red = 0
        for i in range(0, len(data), 3):
            r, g, b = data[i:i + 3]
            red += r > g + 30 and r > b + 30
        assert red == 0, f"unexpected reversed-face diagnostic: {path}"
        log = path.with_suffix(".log").read_text()
        assert "[ERROR]" not in log and "fallback" not in log.lower(), log
    identity = (output / "ids.log").read_text()
    assert identity.count("segments retain") == 6
    assert "panicked" not in identity
    metrics = {"captures": 12, "width": 700, "height": 520,
               "sphere_interior_max_step": maximum_step,
               "sphere_lit_range": max(lit) - min(lit),
               "sphere_unlit_value": unlit[0], "source_identity_uploads": 6}
    (output / "metrics.json").write_text(json.dumps(metrics, indent=2) + "\n")
    print(json.dumps(metrics))


def main():
    """Generate or inspect the bounded source/affine and lit/unlit CAD capture matrix."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=Path("/tmp/cad-fixture"))
    parser.add_argument("--binary-dir", type=Path,
                        default=Path("target/x86_64-unknown-linux-gnu/debug/examples"))
    parser.add_argument("--check-existing", action="store_true")
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=True)
    if not args.check_existing:
        environment = dict(os.environ, VIEWER_NO_GRID="1", VIEWER_W="700", VIEWER_H="520",
                           BENCH_NO_MARKERS="1")
        subprocess.run([str(args.binary_dir / "cad_fixture"), str(args.output)], check=True)
        run_logged([str(args.binary_dir / "check_cad_fixture"), str(args.output)],
                   args.output / "ids.log", environment)
        for kind in ["cylinder", "sphere", "hole", "crease"]:
            for variant in ["source", "affine"]:
                stem = f"{kind}-{variant}"
                run_logged([str(args.binary_dir / "selftest"), str(args.output / f"{stem}.ppm"),
                            str(args.output / f"{stem}.pb")], args.output / f"{stem}.log", environment)
        for kind in ["sphere", "crease"]:
            for style in ["fill", "unlit"]:
                draw = dict(environment, VIEWER_NO_EDGES="1")
                if style == "unlit":
                    draw["VIEWER_NO_LIT"] = "1"
                stem = f"{kind}-{style}"
                run_logged([str(args.binary_dir / "selftest"), str(args.output / f"{stem}.ppm"),
                            str(args.output / f"{kind}-source.pb")], args.output / f"{stem}.log", draw)
    verify(args.output)


if __name__ == "__main__":
    main()
