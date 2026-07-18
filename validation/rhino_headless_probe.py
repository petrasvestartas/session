"""Headless Rhino 8 STEP probe (no desktop instance needed).

Bootstrap that WORKS on this machine (2026-07-18): pythonnet 3 + CoreCLR with
Rhino's own runtimeconfig -- plain `rhinoinside.load()` E_FAILs because it loads
the .NET Framework CLR while Rhino 8's assemblies are .NET Core.

Reports, per brep of each STEP file: IsSolid / IsValid / naked-edge count /
faces whose trims were dropped (area == full underlying surface) + IsValidWithLog.
Run: session_rhino/.venv/Scripts/python.exe validation/rhino_headless_probe.py [files...]
"""
import sys

from clr_loader import get_coreclr
from pythonnet import set_runtime

set_runtime(get_coreclr(runtime_config=r"C:\Program Files\Rhino 8\System\Yak.runtimeconfig.json"))
import clr  # noqa: E402

clr.AddReference(r"C:\Program Files\Rhino 8\System\RhinoCommon.dll")
import Rhino  # noqa: E402
from Rhino.Runtime.InProcess import RhinoCore  # noqa: E402

_core = RhinoCore()

DEFAULT = [
    "session_cpp/serialization/boolean_steps/chairs/chair0_cut_chair1.step",
    "session_cpp/serialization/boolean_steps/chairs/chair0_common_chair1.step",
    "session_cpp/serialization/boolean_steps/chairs/chair0_fuse_chair1.step",
]


def area(geo):
    try:
        amp = Rhino.Geometry.AreaMassProperties.Compute(geo)
        return amp.Area if amp else -1.0
    except Exception:
        return -1.0


def probe(path):
    doc = Rhino.RhinoDoc.CreateHeadless(None)
    try:
        ok = doc.Import(path)
        breps = []
        for obj in doc.Objects:
            g = obj.Geometry
            if isinstance(g, Rhino.Geometry.Brep):
                breps.append(g)
            elif isinstance(g, Rhino.Geometry.Extrusion):
                breps.append(g.ToBrep(True))
        print("== %s import=%s breps=%d" % (path.split('/')[-1], ok, len(breps)))
        for bi, b in enumerate(breps):
            naked = sum(1 for e in b.Edges
                        if e.Valence == Rhino.Geometry.EdgeAdjacency.Naked)
            full, bad = 0, []
            for fi in range(b.Faces.Count):
                f = b.Faces[fi]
                try:
                    fa = area(f.DuplicateFace(False))
                    sa = area(f.UnderlyingSurface().ToBrep())
                    if f.Loops.Count <= 1 and sa > 0 and fa > 0 and abs(fa - sa) < 1e-6 * max(1.0, sa):
                        full += 1
                        bad.append(fi)
                except Exception:
                    pass
            print("   brep%d faces=%d solid=%s valid=%s naked=%d fullsurf=%d %s" %
                  (bi, b.Faces.Count, b.IsSolid, b.IsValid, naked, full,
                   ("dropped-trims: %s" % bad[:12]) if bad else ""))
            if not b.IsValid:
                _, log = b.IsValidWithLog()
                print("      log:", str(log).replace("\n", " | ")[:400])
    finally:
        doc.Dispose()


if __name__ == "__main__":
    files = sys.argv[1:] or DEFAULT
    for f in files:
        probe(f)
