"""Import STEP files into headless Rhino 8 and report what Rhino ACTUALLY sees:
per-brep solidity/validity, per-face loop counts, and faces whose trims were
dropped (face area ~= full underlying-surface area => displayed untrimmed).
Run with: session_rhino/.venv/Scripts/python.exe validation/rhino_step_probe.py [files...]
"""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
STEP_DIR = os.path.join(ROOT, "session_cpp", "serialization", "boolean_steps")

import rhinoinside
rhinoinside.load(r"C:\Program Files\Rhino 8\System")
import Rhino  # noqa: E402

DEFAULT = [
    "freeform_common_box.step", "box_common_sph.step", "ebox_fuse_ecylF.step",
    "ebox_common_ecylF.step", "box_common_cyl.step", "cone_common_cyl.step",
    "ebox_fuse_eboxC.step", "ebox_fuse_eboxT.step",
]

def area(geo):
    try:
        amp = Rhino.Geometry.AreaMassProperties.Compute(geo)
        return amp.Area if amp else -1.0
    except Exception:
        return -1.0

def report_brep(tag, name, b):
    print("  %s '%s' faces=%d solid=%s valid=%s" %
          (tag, name, b.Faces.Count, b.IsSolid, b.IsValid))
    for fi in range(b.Faces.Count):
        f = b.Faces[fi]
        nloops = f.Loops.Count
        fa = area(f.DuplicateFace(False))
        us = f.UnderlyingSurface()
        sa = area(us.ToBrep())
        flag = ""
        # trimmed face whose area equals the full surface patch => trims lost
        if nloops <= 1 and sa > 0 and fa > 0 and abs(fa - sa) < 1e-6 * max(1.0, sa):
            flag = " FULL-SURFACE"
        st = "%s" % us.__class__.__name__
        extra = ""
        rc = us.TryGetPlane()
        try:
            got, pl = rc
        except TypeError:
            got, pl = rc, None
        if got and pl is not None:
            extra = " org(%.2f,%.2f,%.2f) n(%.2f,%.2f,%.2f)" % (
                pl.Origin.X, pl.Origin.Y, pl.Origin.Z,
                pl.Normal.X, pl.Normal.Y, pl.Normal.Z)
        print("    f%d loops=%d area=%.4f srf_area=%.4f %s%s%s" %
              (fi, nloops, fa, sa, st, flag, extra))

def probe(path):
    doc = Rhino.RhinoDoc.CreateHeadless(None)
    try:
        ok = doc.Import(path)
        print("== %s import=%s objects=%d idefs=%d" %
              (os.path.basename(path), ok, doc.Objects.Count, doc.InstanceDefinitions.Count))
        for i, obj in enumerate(doc.Objects):
            geo = obj.Geometry
            name = obj.Attributes.Name or ""
            if isinstance(geo, Rhino.Geometry.Brep):
                report_brep("brep%d" % i, name, geo)
            elif isinstance(geo, Rhino.Geometry.InstanceReferenceGeometry):
                idef = doc.InstanceDefinitions.FindId(geo.ParentIdefId)
                print("  obj%d instance-ref '%s' -> idef '%s'" %
                      (i, name, idef.Name if idef else "?"))
            else:
                print("  obj%d %s (not brep) '%s'" % (i, type(geo).__name__, name))
        for d in range(doc.InstanceDefinitions.Count):
            idef = doc.InstanceDefinitions[d]
            objs = idef.GetObjects()
            print("  idef%d '%s' objects=%d" % (d, idef.Name, len(objs) if objs else 0))
            if objs:
                for j, obj in enumerate(objs):
                    geo = obj.Geometry
                    if isinstance(geo, Rhino.Geometry.Brep):
                        report_brep("  idef%d.brep%d" % (d, j), obj.Attributes.Name or "", geo)
                    else:
                        print("    idef%d.obj%d %s" % (d, j, type(geo).__name__))
    finally:
        doc.Dispose()

if __name__ == "__main__":
    names = sys.argv[1:] or DEFAULT
    for n in names:
        p = n if os.path.isabs(n) else os.path.join(STEP_DIR, n)
        if not os.path.exists(p):
            print("== %s MISSING" % n)
            continue
        probe(p)
