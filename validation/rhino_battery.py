"""Dump Rhino CreateInterpolatedCurve results for the whole battery to JSON.
Run with: session_rhino/.venv/Scripts/python.exe validation/rhino_battery.py
"""
import os, sys, json
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from interp_cases import CASES

import rhinoinside
rhinoinside.load(r"C:\Program Files\Rhino 8\System")
import Rhino  # noqa: E402
import System  # noqa: E402

KS = {
    "uniform": Rhino.Geometry.CurveKnotStyle.Uniform,
    "chord": Rhino.Geometry.CurveKnotStyle.Chord,
    "sqrt": Rhino.Geometry.CurveKnotStyle.ChordSquareRoot,
    "uniform_periodic": Rhino.Geometry.CurveKnotStyle.UniformPeriodic,
    "chord_periodic": Rhino.Geometry.CurveKnotStyle.ChordPeriodic,
    "sqrt_periodic": Rhino.Geometry.CurveKnotStyle.ChordSquareRootPeriodic,
}

out = {}
for name, style, pts in CASES:
    # Rhino makes a CLOSED periodic interpolated curve only when the point list
    # repeats the first point at the end; our create_interpolated takes the
    # distinct list. Feed Rhino the closed list for periodic styles so both
    # build the same cv-(n+3) closed periodic curve.
    rh_pts = pts + [pts[0]] if "periodic" in style else pts
    plist = System.Collections.Generic.List[Rhino.Geometry.Point3d]()
    for (x, y, z) in rh_pts:
        plist.Add(Rhino.Geometry.Point3d(float(x), float(y), float(z)))
    try:
        crv = Rhino.Geometry.Curve.CreateInterpolatedCurve(plist, 3, KS[style])
        if crv is None:
            out[name] = {"error": "rhino returned None"}
            continue
        nc = crv.ToNurbsCurve()
        cvs = []
        for i in range(nc.Points.Count):
            cp = nc.Points[i]
            loc = cp.Location
            cvs.append([loc.X, loc.Y, loc.Z, cp.Weight])
        knots = [nc.Knots[i] for i in range(nc.Knots.Count)]
        d0, d1 = nc.Domain.T0, nc.Domain.T1
        samples = []
        for k in range(65):
            t = d0 + (d1 - d0) * k / 64.0
            p = nc.PointAt(t)
            samples.append([p.X, p.Y, p.Z])
        out[name] = {"degree": nc.Degree, "cv_count": nc.Points.Count,
                     "cvs": cvs, "knots": knots, "samples": samples,
                     "rational": nc.IsRational}
    except Exception as e:
        out[name] = {"error": str(e)}

with open(os.path.join(HERE, "_rhino_battery.json"), "w") as f:
    json.dump(out, f)
print("wrote", len(out), "cases")
for k, v in out.items():
    print(k, "ERR:" + v["error"] if "error" in v else f"deg{v['degree']} cv{v['cv_count']}")
