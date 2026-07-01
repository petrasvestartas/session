"""Parity gate for curve operations (compas_occt docs/examples) vs OCCT (oracle.exe).

For each example curve we build it in session_py, serialize its exact poles/knots to the
oracle, and compare the session operation result against the real OCCT result on the
identical curve:
  - length              vs GCPnts_AbscissaPoint::Length
  - closest_point       vs GeomAPI_ProjectPointOnCurve         (curve_closest_point.py)
  - divide_by_count     vs GCPnts_UniformAbscissa              (curve_divide.py)
  - trim (segment)      vs Geom_BSplineCurve::Segment          (curve_segmentation.py)
  - closest curve-curve vs GeomAPI_ExtremaCurveCurve           (curve_closest_parameters_curve.py)

Run:  PYTHONPATH=session_py/src python validation/compare_curve_ops.py
"""
import math
import os
import subprocess

HERE = os.path.dirname(os.path.abspath(__file__))
ORACLE = os.path.join(HERE, "occt_oracle", "build", "Release", "oracle.exe")


def curve_block(c):
    """Serialize a session NurbsCurve into the oracle DEG/NPOLES/.../NKNOTS/... block."""
    deg = c.degree()
    n = c.cv_count()
    kon = [float(k) for k in c.get_nurbsknots()]      # OpenNURBS (len order+cv-2)
    full = [kon[0]] + kon + [kon[-1]]                  # OCCT full knots (len cv+order)
    distinct, mults = [], []
    for v in full:
        if distinct and abs(v - distinct[-1]) < 1e-12:
            mults[-1] += 1
        else:
            distinct.append(v); mults.append(1)
    lines = [f"DEG {deg}", f"NPOLES {n}"]
    for i in range(n):
        p = c.get_cv(i); w = c.weight(i)
        lines.append(f"POLE {p[0]:.17g} {p[1]:.17g} {p[2]:.17g} {w:.17g}")
    lines.append(f"NKNOTS {len(distinct)}")
    for v, m in zip(distinct, mults):
        lines.append(f"KNOT {v:.17g} {m}")
    return "\n".join(lines)


def run_oracle(req_text):
    req = os.path.join(HERE, "_op_req.txt")
    res = os.path.join(HERE, "_op_out.txt")
    with open(req, "w") as f:
        f.write(req_text)
    subprocess.run([ORACLE, req, res], check=True)
    with open(res) as f:
        return f.read().split()


def dist(p, q):
    return math.sqrt(sum((p[i] - q[i]) ** 2 for i in range(3)))


def make_from_points(pts):
    from session_py import NurbsCurve, Point
    p = [Point(*x) for x in pts]
    return NurbsCurve.create_from_parameters(
        p, [1.0] * len(p), [0.0, 1.0], [len(p), len(p)], len(p) - 1)


def make_interp(pts):
    from session_py import NurbsCurve, Point
    from session_py.nurbsknot import CurveNurbsKnotStyle, CurveInterpStyle
    p = [Point(*x) for x in pts]
    return NurbsCurve.create_interpolated(p, CurveNurbsKnotStyle.Chord, CurveInterpStyle.Occt)


def test_length():
    c = make_from_points([(0, 0, 0), (3, -6, 0), (6, 2, 0), (9, -2, 0)])
    toks = run_oracle("OP curve_length\n" + curve_block(c) + "\n")
    occt_len = float(toks[toks.index("LENGTH") + 1])
    ours = c.length()
    return "length", abs(ours - occt_len), f"ours={ours:.6f} occt={occt_len:.6f}"


def test_closest_point():
    from session_py import Point
    c = make_interp([(0, 0, 0), (3, 0, 2), (6, 0, -3), (8, 0, 0)])
    tp = (2, -1, 0)
    toks = run_oracle("OP curve_closest\n" + curve_block(c) + f"\nPT {tp[0]} {tp[1]} {tp[2]}\n")
    i = toks.index("CLOSEST")
    occt_pt = (float(toks[i + 1]), float(toks[i + 2]), float(toks[i + 3]))
    ours_pt = c.closest_point(Point(*tp))
    ours = (ours_pt[0], ours_pt[1], ours_pt[2])
    return "closest_point", dist(ours, occt_pt), f"ours={tuple(round(x,5) for x in ours)} occt={tuple(round(x,5) for x in occt_pt)}"


def test_divide():
    c = make_from_points([(0, 0, 0), (3, -6, 0), (6, 2, 0), (9, -2, 0)])
    N = 10
    toks = run_oracle("OP curve_divide\n" + curve_block(c) + f"\nN {N}\n")
    i = toks.index("NPARAMS")
    nump = int(toks[i + 1])
    j = i + 2
    occt_pts = []
    for _ in range(nump):
        # t x y z
        occt_pts.append((float(toks[j + 1]), float(toks[j + 2]), float(toks[j + 3])))
        j += 4
    pts, params = c.divide_by_count(N + 1, include_endpoints=True)
    ours_pts = [(p[0], p[1], p[2]) for p in pts]
    # Hausdorff between the two point sets (arc-equal division points)
    def hd(a, b):
        return max(max(min(dist(x, y) for y in b) for x in a),
                   max(min(dist(x, y) for y in a) for x in b))
    return "divide", hd(ours_pts, occt_pts), f"ours_n={len(ours_pts)} occt_n={len(occt_pts)}"


def test_segment():
    c = make_from_points([(0, 0, 0), (3, 6, 0), (6, -3, 3), (10, 0, 0)])
    u, v = 0.2, 0.5
    toks = run_oracle("OP curve_segment\n" + curve_block(c) + f"\nUV {u} {v}\n")
    i = toks.index("SAMPLES")
    ns = int(toks[i + 1]); j = i + 2
    occt_s = []
    for _ in range(ns):
        occt_s.append((float(toks[j]), float(toks[j + 1]), float(toks[j + 2]))); j += 3
    seg = c  # trim mutates in place; copy via from params
    from session_py import NurbsCurve, Point
    seg = NurbsCurve.create_from_parameters(
        [Point(*x) for x in [(0, 0, 0), (3, 6, 0), (6, -3, 3), (10, 0, 0)]],
        [1, 1, 1, 1], [0, 1], [4, 4], 3)
    seg.trim(u, v)
    d0, d1 = seg.domain()
    ours_s = [tuple(seg.point_at(d0 + (d1 - d0) * k / 64.0)[d] for d in range(3)) for k in range(65)]

    def hd(a, b):
        return max(max(min(dist(x, y) for y in b) for x in a),
                   max(min(dist(x, y) for y in a) for x in b))
    return "segment", hd(ours_s, occt_s), f"domain[{d0:.4g},{d1:.4g}]"


def test_curve_curve():
    c0 = make_from_points([(0, 0, 0), (3, 6, 0), (6, -3, 3), (10, 0, 0)])
    c1 = make_from_points([(6, -3, 0), (3, 1, 0), (6, 6, 3), (3, 12, 0)])
    toks = run_oracle("OP curve_extrema\n" + curve_block(c0) + "\n" + curve_block(c1) + "\n")
    iu = toks.index("U")
    occt_u = float(toks[iu + 1]); occt_v = float(toks[toks.index("V") + 1])
    ipa = toks.index("PA"); ipb = toks.index("PB")
    occt_pa = (float(toks[ipa + 1]), float(toks[ipa + 2]), float(toks[ipa + 3]))
    occt_pb = (float(toks[ipb + 1]), float(toks[ipb + 2]), float(toks[ipb + 3]))
    (u, v), d = c0.closest_parameters_curve(c1, return_distance=True)
    (pa, pb), _ = c0.closest_points_curve(c1, return_distance=True)
    ours_pa = (pa[0], pa[1], pa[2]); ours_pb = (pb[0], pb[1], pb[2])
    dev = max(dist(ours_pa, occt_pa), dist(ours_pb, occt_pb), abs(u - occt_u), abs(v - occt_v))
    return "curve_curve", dev, f"u={u:.5f}/{occt_u:.5f} v={v:.5f}/{occt_v:.5f}"


def main():
    if not os.path.exists(ORACLE):
        print("oracle.exe not built"); return
    tests = [test_length, test_closest_point, test_divide, test_segment, test_curve_curve]
    for t in tests:
        try:
            name, dev, info = t()
            status = "ok" if dev < 1e-4 else "FAIL"
            print(f"{name:16s} dev {dev:.2e}  {status}   {info}")
        except Exception as e:
            print(f"{t.__name__:16s} EXCEPTION {e}")


if __name__ == "__main__":
    main()
