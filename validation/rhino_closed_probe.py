"""Probe how Rhino builds CLOSED/periodic interpolated curves.
Run with: session_rhino/.venv/Scripts/python.exe validation/rhino_closed_probe.py
"""
import rhinoinside
rhinoinside.load(r"C:\Program Files\Rhino 8\System")
import Rhino  # noqa: E402
import System  # noqa: E402

PTS = [(4,20,0),(-2,20,0),(-2,25,0),(-10,28,0),(-10,21,0),(0,15,0)]

def mk(points):
    lst = System.Collections.Generic.List[Rhino.Geometry.Point3d]()
    for (x,y,z) in points: lst.Add(Rhino.Geometry.Point3d(float(x),float(y),float(z)))
    return lst

def report(tag, nc):
    if nc is None: print(tag, "-> None"); return
    nc = nc.ToNurbsCurve()
    print(f"{tag}: deg{nc.Degree} cv{nc.Points.Count} closed={nc.IsClosed} periodic={nc.IsPeriodic}")
    print("   first CV", [round(nc.Points[0].Location.X,3), round(nc.Points[0].Location.Y,3)],
          "last CV", [round(nc.Points[nc.Points.Count-1].Location.X,3), round(nc.Points[nc.Points.Count-1].Location.Y,3)])
    print("   knots", [round(nc.Knots[i],3) for i in range(nc.Knots.Count)])

KS = Rhino.Geometry.CurveKnotStyle
# (a) periodic style, distinct points
report("ChordPeriodic, 6 distinct", Rhino.Geometry.Curve.CreateInterpolatedCurve(mk(PTS), 3, KS.ChordPeriodic))
# (b) periodic style, first point repeated at end (closed point list)
report("ChordPeriodic, first repeated", Rhino.Geometry.Curve.CreateInterpolatedCurve(mk(PTS+[PTS[0]]), 3, KS.ChordPeriodic))
# (c) chord (non-periodic) with first repeated
report("Chord, first repeated", Rhino.Geometry.Curve.CreateInterpolatedCurve(mk(PTS+[PTS[0]]), 3, KS.Chord))
