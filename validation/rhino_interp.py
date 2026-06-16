"""Rhino ground-truth interpolated curve via headless RhinoCommon.
Run with the rhino venv:
    session_rhino/.venv/Scripts/python.exe validation/rhino_interp.py
"""
POINTS = [
    (14.0, 9.0, 0.0),
    (21.0, 22.0, 0.0),
    (26.0, 10.0, 0.0),
    (35.0, 19.0, 0.0),
    (41.0, 13.0, 0.0),
]

import rhinoinside
rhinoinside.load(r"C:\Program Files\Rhino 8\System")
import Rhino  # noqa: E402
import System  # noqa: E402

pts = System.Collections.Generic.List[Rhino.Geometry.Point3d]()
for (x, y, z) in POINTS:
    pts.Add(Rhino.Geometry.Point3d(x, y, z))

crv = Rhino.Geometry.Curve.CreateInterpolatedCurve(
    pts, 3, Rhino.Geometry.CurveKnotStyle.Chord)
nc = crv.ToNurbsCurve()

print("RHINO CreateInterpolatedCurve degree=3 Chord")
print("degree", nc.Degree, "cv_count", nc.Points.Count)
cvs = []
for i in range(nc.Points.Count):
    cp = nc.Points[i]
    loc = cp.Location
    cvs.append((round(loc.X, 6), round(loc.Y, 6), round(loc.Z, 6), round(cp.Weight, 6)))
print("CVs:", cvs)
knots = [round(nc.Knots[i], 6) for i in range(nc.Knots.Count)]
print("knots:", knots)
samples = []
d0 = nc.Domain.T0
d1 = nc.Domain.T1
for k in range(65):
    t = d0 + (d1 - d0) * k / 64.0
    p = nc.PointAt(t)
    samples.append((round(p.X, 6), round(p.Y, 6), round(p.Z, 6)))
print("SAMPLE0", samples[0], "SAMPLE_END", samples[-1])
# write samples for diffing
import json, os
with open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "_rhino_interp.json"), "w") as f:
    json.dump({"cvs": cvs, "knots": knots, "samples": samples}, f)
