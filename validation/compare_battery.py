"""Compare our create_interpolated against Rhino across the battery.
Run with the python that has session_py: python validation/compare_battery.py
Requires _rhino_battery.json (produced by rhino_battery.py).
"""
import os, sys, json, math
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from interp_cases import CASES

from session_py import NurbsCurve, Point
from session_py.nurbsknot import CurveNurbsKnotStyle

STYLE = {
    "uniform": CurveNurbsKnotStyle.Uniform,
    "chord": CurveNurbsKnotStyle.Chord,
    "sqrt": CurveNurbsKnotStyle.ChordSquareRoot,
    "uniform_periodic": CurveNurbsKnotStyle.UniformPeriodic,
    "chord_periodic": CurveNurbsKnotStyle.ChordPeriodic,
    "sqrt_periodic": CurveNurbsKnotStyle.ChordSquareRootPeriodic,
}

with open(os.path.join(HERE, "_rhino_battery.json")) as f:
    RH = json.load(f)


def ours(style, pts):
    c = NurbsCurve.create_interpolated([Point(*p) for p in pts], STYLE[style])
    if not c.is_valid():
        return None
    d0, d1 = c.domain()
    samples = [[c.point_at(d0 + (d1 - d0) * k / 64.0)[d] for d in range(3)] for k in range(65)]
    return {"degree": c.degree(), "cv_count": c.cv_count(), "samples": samples}


def hausdorff(a, b):
    def dist(p, q):
        return math.sqrt(sum((p[i] - q[i]) ** 2 for i in range(3)))
    return max(max(min(dist(x, y) for y in b) for x in a),
              max(min(dist(x, y) for y in a) for x in b))


print(f"{'case':22} {'ours deg/cv':12} {'rhino deg/cv':13} {'shape Hausdorff':16} verdict")
print("-" * 80)
for name, style, pts in CASES:
    rh = RH.get(name, {})
    o = ours(style, pts)
    if "error" in rh:
        print(f"{name:22} rhino-error: {rh['error']}")
        continue
    if o is None:
        print(f"{name:22} OURS INVALID  rhino deg{rh['degree']} cv{rh['cv_count']}")
        continue
    od = f"{o['degree']}/{o['cv_count']}"
    rd = f"{rh['degree']}/{rh['cv_count']}"
    h = hausdorff(o["samples"], rh["samples"])
    match = (o["degree"] == rh["degree"] and o["cv_count"] == rh["cv_count"] and h < 1e-4)
    verdict = "MATCH" if match else ("shape~" if h < 1e-4 else "DIVERGE")
    print(f"{name:22} {od:12} {rd:13} {h:<16.2e} {verdict}")
