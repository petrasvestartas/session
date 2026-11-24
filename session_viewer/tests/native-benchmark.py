#!/usr/bin/env python3
"""Reproducible 5-run native GPU baseline for Session Viewer (Milestone M1).

Checks CPU frequency scaling first, runs release `selftest` 5 times per scenario
at 900x700 (60 frames, GPU timestamp queries enabled), computes the median of
medians across runs, and prints a Markdown table + JSON summary.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import statistics
import subprocess
import sys
import tempfile
from pathlib import Path

VIEWER_DIR = Path(__file__).resolve().parent.parent
DEFAULT_BIN = VIEWER_DIR / "target" / "x86_64-unknown-linux-gnu" / "release" / "examples" / "selftest"

SCENARIOS = [
    ("timber-floor, rest", ["tests/fixtures/timber-floor.pb"], {}),
    (
        "timber-floor, orbit drag",
        ["tests/fixtures/timber-floor.pb"],
        {"VIEWER_FRAMES_ORBIT": "2", "VIEWER_INTERACT": "1"},
    ),
    ("view_local.yaml, rest", ["assets/view_local.yaml"], {}),
    (
        "view_local.yaml, orbit drag",
        ["assets/view_local.yaml"],
        {"VIEWER_FRAMES_ORBIT": "2", "VIEWER_INTERACT": "1"},
    ),
]

FRAMES_RE = re.compile(r"frames: n=(\d+) median ([\d.]+) ms \(([\d.]+) fps\) min ([\d.]+) max ([\d.]+)")
DRAG_RE = re.compile(r"drag:\s*p50 ([\d.]+) ms,?\s+p95 ([\d.]+) ms")
TIER_RE = re.compile(r"drag: tier (\d+) from frame (\d+), ([\d.]+) ms after the first slow frame; p50 ([\d.]+) ms there")
GPU_PASS_RE = re.compile(r"gpu ms, median per pass: (.+)")
NON_BG_RE = re.compile(r"non-background pixels: (\d+) \(([\d.]+)%\)")


def check_cpu_throttle() -> tuple[int | None, int | None]:
    p_cur = Path("/sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq")
    p_max = Path("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq")
    if not (p_cur.exists() and p_max.exists()):
        return None, None
    cur = int(p_cur.read_text().strip())
    mx = int(p_max.read_text().strip())
    if cur < mx:
        raise SystemExit(
            f"ABORT: CPU throttled ({cur} kHz < {mx} kHz). Restore full frequency before benchmarking."
        )
    return cur, mx


def parse_run(stdout: str) -> dict:
    out: dict = {"passes": {}}
    m = FRAMES_RE.search(stdout)
    if m:
        out["frames_n"] = int(m.group(1))
        out["wall_p50_ms"] = float(m.group(2))
        out["wall_min_ms"] = float(m.group(4))
        out["wall_max_ms"] = float(m.group(5))
    d = DRAG_RE.search(stdout)
    if d:
        out["drag_p50_ms"] = float(d.group(1))
        out["drag_p95_ms"] = float(d.group(2))
    g = GPU_PASS_RE.search(stdout)
    if g:
        for item in g.group(1).split(","):
            parts = item.strip().split()
            if len(parts) == 2:
                out["passes"][parts[0]] = float(parts[1])
    nb = NON_BG_RE.search(stdout)
    if nb:
        out["non_bg_pixels"] = int(nb.group(1))
        out["non_bg_pct"] = float(nb.group(2))
    return out


def med(values: list[float]) -> float:
    return round(statistics.median(values), 3) if values else 0.0


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--bin", type=Path, default=DEFAULT_BIN, help="Path to release selftest binary")
    ap.add_argument("--runs", type=int, default=5, help="Repetitions per scenario")
    ap.add_argument("--frames", type=int, default=60, help="Frames per repetition")
    ap.add_argument("--save-ppm-dir", type=Path, default=None, help="Optional dir to save golden PPMs")
    ap.add_argument("--json-out", type=Path, default=None, help="Optional path to write JSON report")
    args = ap.parse_args()

    cur_khz, max_khz = check_cpu_throttle()
    if not args.bin.exists():
        raise SystemExit(f"Missing binary: {args.bin}")

    if args.save_ppm_dir:
        args.save_ppm_dir.mkdir(parents=True, exist_ok=True)

    summary_rows = []
    with tempfile.TemporaryDirectory(prefix="viewer-m1-bench-") as tmp:
        for label, inputs, extra_env in SCENARIOS:
            runs = []
            slug = re.sub(r"[^a-z0-9]+", "-", label.lower()).strip("-")
            for r in range(args.runs):
                ppm_path = (
                    args.save_ppm_dir / f"{slug}.ppm"
                    if (args.save_ppm_dir and r == 0)
                    else Path(tmp) / f"{slug}-{r}.ppm"
                )
                env = os.environ.copy()
                env.update(
                    {
                        "VIEWER_W": "900",
                        "VIEWER_H": "700",
                        "VIEWER_FRAMES": str(args.frames),
                        "VIEWER_GPU_TIMING": "1",
                    }
                )
                env.update(extra_env)
                proc = subprocess.run(
                    [str(args.bin), str(ppm_path), *inputs],
                    cwd=VIEWER_DIR,
                    env=env,
                    check=True,
                    capture_output=True,
                    text=True,
                )
                parsed = parse_run(proc.stdout)
                runs.append(parsed)

            pass_keys = list(runs[0]["passes"].keys()) if runs else []
            med_passes = {k: med([x["passes"][k] for x in runs if k in x["passes"]]) for k in pass_keys}
            row = {
                "scenario": label,
                "runs": args.runs,
                "frames": args.frames,
                "wall_p50_ms": med([x["wall_p50_ms"] for x in runs if "wall_p50_ms" in x]),
                "drag_p50_ms": med([x["drag_p50_ms"] for x in runs if "drag_p50_ms" in x])
                if any("drag_p50_ms" in x for x in runs)
                else None,
                "drag_p95_ms": med([x["drag_p95_ms"] for x in runs if "drag_p95_ms" in x])
                if any("drag_p95_ms" in x for x in runs)
                else None,
                "passes_ms": med_passes,
                "non_bg_pixels": runs[0].get("non_bg_pixels"),
                "raw_runs": runs,
            }
            summary_rows.append(row)

    print(f"# Native GPU Baseline ({args.runs} runs x {args.frames} frames, 900x700, CPU {cur_khz}/{max_khz} kHz)\n")
    print(
        "| Scenario | Wall p50 (ms) | Drag p95 (ms) | `tiles` | `faces` | `ink.strokes` | `ink.outlines` | `ink.lanes` | `ink.markers` | `ink.text` | `ink_total` |"
    )
    print("|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|")
    for r in summary_rows:
        p = r["passes_ms"]
        dp95 = f"{r['drag_p95_ms']:.2f}" if r["drag_p95_ms"] is not None else "—"
        print(
            f"| {r['scenario']} | {r['wall_p50_ms']:.2f} | {dp95} | "
            f"{p.get('tiles', 0.0):.3f} | {p.get('faces', 0.0):.3f} | "
            f"{p.get('ink.strokes', 0.0):.3f} | {p.get('ink.outlines', 0.0):.3f} | "
            f"{p.get('ink.lanes', 0.0):.3f} | {p.get('ink.markers', 0.0):.3f} | "
            f"{p.get('ink.text', 0.0):.3f} | **{p.get('ink_total', 0.0):.3f}** |"
        )

    payload = {
        "cpu_scaling_max_khz": cur_khz,
        "cpu_max_khz": max_khz,
        "runs": args.runs,
        "frames": args.frames,
        "viewport": [900, 700],
        "scenarios": summary_rows,
    }
    if args.json_out:
        args.json_out.parent.mkdir(parents=True, exist_ok=True)
        args.json_out.write_text(json.dumps(payload, indent=2) + "\n")


if __name__ == "__main__":
    main()
