"""Compare interpolated-curve-from-points across our kernel vs OCCT (oracle.exe).
Rhino is compared separately via the session_rhino headless bridge.

Run with the python that has session_py importable:
    python validation/compare_interp.py
"""
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ORACLE = os.path.join(HERE, "occt_oracle", "build", "Release", "oracle.exe")

# Canonical point set (from the existing create_interpolated test).
POINTS = [
    (14.0, 9.0, 0.0),
    (21.0, 22.0, 0.0),
    (26.0, 10.0, 0.0),
    (35.0, 19.0, 0.0),
    (41.0, 13.0, 0.0),
]


def ours():
    from session_py import NurbsCurve, Point
    from session_py.nurbsknot import CurveNurbsKnotStyle
    pts = [Point(*p) for p in POINTS]
    c = NurbsCurve.create_interpolated(pts, CurveNurbsKnotStyle.Chord)
    cvs = [tuple(round(c.get_cv(i)[d], 6) for d in range(3)) for i in range(c.cv_count())]
    knots = [round(float(k), 6) for k in c.get_nurbsknots()]
    d0, d1 = c.domain()
    samples = [tuple(round(c.point_at(d0 + (d1 - d0) * k / 64.0)[d], 6) for d in range(3)) for k in range(65)]
    return {"degree": c.degree(), "cv_count": c.cv_count(), "cvs": cvs, "knots": knots, "samples": samples}


def occt():
    if not os.path.exists(ORACLE):
        return {"error": "oracle.exe not built"}
    req = os.path.join(HERE, "_interp_req.txt")
    res = os.path.join(HERE, "_interp_out.txt")
    with open(req, "w") as f:
        f.write("OP interpolate\n")
        f.write(f"NPTS {len(POINTS)}\n")
        for p in POINTS:
            f.write(f"PT {p[0]} {p[1]} {p[2]}\n")
        f.write("PERIODIC 0\n")
    subprocess.run([ORACLE, req, res], check=True)
    out = {"cvs": [], "knots": [], "samples": []}
    with open(res) as f:
        toks = f.read().split()
    i = 0
    if toks and toks[0] == "ERROR":
        return {"error": " ".join(toks)}
    # OK DEG d NPOLES p ... KNOTS k ... SAMPLES s ...
    while i < len(toks):
        t = toks[i]
        if t == "DEG":
            out["degree"] = int(toks[i + 1]); i += 2
        elif t == "NPOLES":
            n = int(toks[i + 1]); i += 2
            for _ in range(n):
                out["cvs"].append(tuple(round(float(toks[i + d]), 6) for d in range(4)))
                i += 4
        elif t == "KNOTS":
            n = int(toks[i + 1]); i += 2
            for _ in range(n):
                out["knots"].append((round(float(toks[i]), 6), int(toks[i + 1])))
                i += 2
        elif t == "SAMPLES":
            n = int(toks[i + 1]); i += 2
            for _ in range(n):
                out["samples"].append(tuple(round(float(toks[i + d]), 6) for d in range(3)))
                i += 3
        else:
            i += 1
    out["cv_count"] = len(out["cvs"])
    return out


def hausdorff(a, b):
    import math
    def d(p, q):
        return math.sqrt(sum((p[i] - q[i]) ** 2 for i in range(3)))
    def directed(xs, ys):
        return max(min(d(x, y) for y in ys) for x in xs)
    return max(directed(a, b), directed(b, a))


def main():
    o = ours()
    k = occt()
    print("=== OURS (create_interpolated, Chord) ===")
    print("degree", o["degree"], "cv_count", o["cv_count"])
    print("CVs:", o["cvs"])
    print("knots:", o["knots"])
    print()
    print("=== OCCT (GeomAPI_Interpolate) ===")
    if "error" in k:
        print("ERROR:", k["error"])
    else:
        print("degree", k.get("degree"), "cv_count", k["cv_count"])
        print("CVs:", k["cvs"])
        print("knots:", k["knots"])
        print()
        print("=== shape deviation (sampled, set Hausdorff) ===")
        print("ours vs OCCT:", round(hausdorff(o["samples"], k["samples"]), 6))
        # endpoint interpolation check
        import math
        def near(p, q):
            return math.sqrt(sum((p[i] - q[i]) ** 2 for i in range(3)))
        print("ours passes through P0,P-1:",
              round(near(o["samples"][0], POINTS[0]), 9),
              round(near(o["samples"][-1], POINTS[-1]), 9))
        print("OCCT passes through P0,P-1:",
              round(near(k["samples"][0], POINTS[0]), 9),
              round(near(k["samples"][-1], POINTS[-1]), 9))


if __name__ == "__main__":
    main()
