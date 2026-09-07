#!/usr/bin/env python3
"""_probe_matrix.py <selftest> <mk_hidden_line_probe> <out_dir>  ->  108 hidden-line renders

Three fixtures (regular, warped, authored) x three cameras (top orthographic, down, iso) x
three distances (1, 4, 16) x MSAA (1, 4). Every render must have zero magenta pixels (hidden
ink showing) and at least 500 blue pixels (visible ink retained), 300 for the authored
hairline fixture."""
import json, os, pathlib, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _count_colors import read_ppm

CAMERAS = [("top", {"VIEWER_VIEW": "top"}), ("down", {"VIEWER_ORBIT": "0,209"}), ("iso", {})]


def counts(path):
    w, h, px = read_ppm(path)
    blue = magenta = 0
    for k in range(0, w * h * 3, 3):
        r, g, b = px[k], px[k + 1], px[k + 2]
        if g <= 60 and b >= 195:
            if r <= 60:
                blue += 1
            elif r >= 195:
                magenta += 1
    return blue, magenta


def main():
    selftest, maker, out = sys.argv[1], sys.argv[2], pathlib.Path(sys.argv[3])
    out.mkdir(parents=True, exist_ok=True)
    base = {k: v for k, v in os.environ.items() if not k.startswith(("VIEWER_", "HIDDEN_LINE_"))}
    fixtures = {"regular": {}, "warped": {"HIDDEN_LINE_PROBE_WARPED": "1"}, "authored": {"HIDDEN_LINE_PROBE_AUTHORED": "1"}}
    for name, flags in fixtures.items():
        subprocess.run([maker, str(out / f"{name}.pb")], env=dict(base, **flags), check=True, capture_output=True)
    results, failures = [], 0
    for fixture in fixtures:
        for camera, settings in CAMERAS:
            for distance in (1, 4, 16):
                for msaa in (1, 4):
                    stem = f"{fixture}_{camera}_{distance}_{msaa}"
                    ppm = out / f"{stem}.ppm"
                    env = dict(base, VIEWER_W="1400", VIEWER_H="900", VIEWER_NO_GRID="1", VIEWER_DISTANCE_SCALE=str(distance), VIEWER_MSAA=str(msaa), **settings)
                    run = subprocess.run([selftest, str(ppm), str(out / f"{fixture}.pb")], env=env, capture_output=True, text=True)
                    (out / f"{stem}.log").write_text(run.stdout + run.stderr)
                    run.check_returncode()
                    blue, magenta = counts(ppm)
                    ok = magenta == 0 and blue >= (300 if fixture == "authored" else 500)
                    failures += not ok
                    results.append(dict(case=stem, blue=blue, magenta=magenta, ok=ok))
                    print(f"{'ok  ' if ok else 'FAIL'} {stem}: blue {blue} magenta {magenta}")
    (out / "results.json").write_text(json.dumps(results, indent=2))
    print(f"{len(results)} cases, {failures} failures")
    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
