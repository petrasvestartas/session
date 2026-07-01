"""Parity gate for curve constructions (compas_occt from_parameters / from_points /
from_line / from_circle / from_ellipse) vs OCCT (oracle.exe `curve_eval`).

Each case is the exact (points, weights, knots, mults, degree) that compas_occt expands
for the corresponding docs/examples curve. We build the curve in session_py via
NurbsCurve.create_from_parameters and compare control points + sampled geometry (set
Hausdorff over the shared domain) against a real OCCT Geom_BSplineCurve.

Run with the python that has session_py importable:
    PYTHONPATH=session_py/src python validation/compare_curve_eval.py
"""
import math
import os
import subprocess

HERE = os.path.dirname(os.path.abspath(__file__))
ORACLE = os.path.join(HERE, "occt_oracle", "build", "Release", "oracle.exe")

W = 0.5 * math.sqrt(2.0)


def circle_points(r):
    dx = (r, 0, 0)
    dy = (0, r, 0)
    o = (0, 0, 0)

    def add(*vs):
        return tuple(sum(v[i] for v in vs) for i in range(3))

    def neg(v):
        return (-v[0], -v[1], -v[2])

    return [
        add(o, neg(dy)),
        add(o, neg(dy), neg(dx)),
        add(o, neg(dx)),
        add(o, dy, neg(dx)),
        add(o, dy),
        add(o, dy, dx),
        add(o, dx),
        add(o, neg(dy), dx),
        add(o, neg(dy)),
    ]


def ellipse_points(major, minor):
    dx = (major, 0, 0)
    dy = (0, minor, 0)
    o = (0, 0, 0)

    def add(*vs):
        return tuple(sum(v[i] for v in vs) for i in range(3))

    def neg(v):
        return (-v[0], -v[1], -v[2])

    return [
        add(o, neg(dy)),
        add(o, neg(dy), neg(dx)),
        add(o, neg(dx)),
        add(o, dy, neg(dx)),
        add(o, dy),
        add(o, dy, dx),
        add(o, dx),
        add(o, neg(dy), dx),
        add(o, neg(dy)),
    ]


P4 = [(0, 0, 0), (3, 6, 0), (6, -3, 3), (10, 0, 0)]
CONIC_KNOTS = [0.0, 0.25, 0.5, 0.75, 1.0]
CONIC_MULTS = [3, 2, 2, 2, 3]
CONIC_W = [1, W, 1, W, 1, W, 1, W, 1]

CASES = [
    # curve_from_points / curve_from_parameters
    dict(name="from_points", points=P4, weights=[1.0] * 4, knots=[0.0, 1.0], mults=[4, 4], degree=3),
    # curve_from_line
    dict(name="from_line", points=[(0, 0, 0), (3, 3, 0)], weights=[1.0, 1.0], knots=[0.0, 1.0], mults=[2, 2], degree=1),
    # curve_from_circle (radius 1)
    dict(name="from_circle", points=circle_points(1.0), weights=CONIC_W, knots=CONIC_KNOTS, mults=CONIC_MULTS, degree=2),
    # curve_from_ellipse (major 2, minor 1)
    dict(name="from_ellipse", points=ellipse_points(2.0, 1.0), weights=CONIC_W, knots=CONIC_KNOTS, mults=CONIC_MULTS, degree=2),
    # curve_comparison1 (degree-2 variants on 3 points)
    dict(name="cmp1_c1", points=[(3, 0, 0), (4, 3, 0), (5, 0, 0)], weights=[1, 1, 1], knots=[0.0, 1.0], mults=[3, 3], degree=2),
    dict(name="cmp1_c2", points=[(3, 0, 0), (4, 3, 0), (5, 0, 0)], weights=[1, 2, 1], knots=[0.0, 1.0], mults=[3, 3], degree=2),
    dict(name="cmp1_c3", points=[(3, 0, 0), (4, 3, 0), (5, 0, 0)], weights=[1, 1, 1], knots=[0.0, 1.0, 2.0, 3.0, 4.0, 5.0], mults=[1, 1, 1, 1, 1, 1], degree=2),
    # curve_comparison2 (degree-3 variants on 4 points)
    dict(name="cmp2_c1", points=[(4, 0, 0), (5, 2, 0), (6, -2, 0), (7, 0, 0)], weights=[1, 1, 1, 1], knots=[0.0, 1.0], mults=[4, 4], degree=3),
    dict(name="cmp2_c2", points=[(4, 0, 0), (5, 2, 0), (6, -2, 0), (7, 0, 0)], weights=[1, 2, 2, 1], knots=[0.0, 1.0], mults=[4, 4], degree=3),
    dict(name="cmp2_c3", points=[(4, 0, 0), (5, 2, 0), (6, -2, 0), (7, 0, 0)], weights=[1, 1, 1, 1], knots=[0.0, 1 / 3, 2 / 3, 1.0], mults=[3, 1, 1, 3], degree=3),
    dict(name="cmp2_c4", points=[(4, 0, 0), (5, 2, 0), (6, -2, 0), (7, 0, 0)], weights=[1, 1, 1, 1], knots=[0.0, 1 / 5, 2 / 5, 3 / 5, 4 / 5, 1.0], mults=[2, 1, 1, 1, 1, 2], degree=3),
    dict(name="cmp2_c5", points=[(4, 0, 0), (5, 2, 0), (6, -2, 0), (7, 0, 0)], weights=[1, 1, 1, 1], knots=[0.0, 1 / 7, 2 / 7, 3 / 7, 4 / 7, 5 / 7, 6 / 7, 1.0], mults=[1, 1, 1, 1, 1, 1, 1, 1], degree=3),
    dict(name="cmp2_c6", points=[(4, 0, 0), (5, 2, 0), (6, -2, 0), (7, 0, 0)], weights=[1, 1, 1, 1], knots=[0.0, 0.5, 1.0], mults=[3, 1, 3], degree=2),
]


def ours(case):
    from session_py import NurbsCurve, Point
    pts = [Point(*p) for p in case["points"]]
    c = NurbsCurve.create_from_parameters(pts, case["weights"], case["knots"], case["mults"], case["degree"])
    if not c.is_valid():
        return {"error": "session curve invalid"}
    d0, d1 = c.domain()
    samples = [tuple(c.point_at(d0 + (d1 - d0) * k / 64.0)[d] for d in range(3)) for k in range(65)]
    cvs = [tuple(c.get_cv(i)[d] for d in range(3)) for i in range(c.cv_count())]
    return {"degree": c.degree(), "samples": samples, "cvs": cvs, "domain": (d0, d1)}


def occt(case):
    req = os.path.join(HERE, "_ce_req.txt")
    res = os.path.join(HERE, "_ce_out.txt")
    with open(req, "w") as f:
        f.write("OP curve_eval\n")
        f.write(f"DEG {case['degree']}\n")
        f.write(f"NPOLES {len(case['points'])}\n")
        for p, w in zip(case["points"], case["weights"]):
            f.write(f"POLE {p[0]} {p[1]} {p[2]} {w}\n")
        f.write(f"NKNOTS {len(case['knots'])}\n")
        for v, m in zip(case["knots"], case["mults"]):
            f.write(f"KNOT {v} {m}\n")
    subprocess.run([ORACLE, req, res], check=True)
    with open(res) as f:
        toks = f.read().split()
    if toks and toks[0] == "ERROR":
        return {"error": " ".join(toks)}
    out = {"cvs": [], "samples": []}
    i = 0
    while i < len(toks):
        t = toks[i]
        if t == "DEG":
            out["degree"] = int(toks[i + 1]); i += 2
        elif t == "NPOLES":
            n = int(toks[i + 1]); i += 2
            for _ in range(n):
                x, y, z, w = (float(toks[i + d]) for d in range(4))
                out["cvs"].append((x, y, z)); i += 4
        elif t == "KNOTS":
            n = int(toks[i + 1]); i += 2 + 2 * n
        elif t == "SAMPLES":
            n = int(toks[i + 1]); i += 2
            for _ in range(n):
                out["samples"].append(tuple(float(toks[i + d]) for d in range(3))); i += 3
        else:
            i += 1
    return out


def hausdorff(a, b):
    def d(p, q):
        return math.sqrt(sum((p[i] - q[i]) ** 2 for i in range(3)))

    def directed(xs, ys):
        return max(min(d(x, y) for y in ys) for x in xs)

    return max(directed(a, b), directed(b, a))


def main():
    if not os.path.exists(ORACLE):
        print("oracle.exe not built"); return
    worst = 0.0
    fails = 0
    for case in CASES:
        o = ours(case)
        k = occt(case)
        if "error" in o or "error" in k:
            print(f"{case['name']:14s} ERROR ours={o.get('error')} occt={k.get('error')}")
            fails += 1
            continue
        cv_dev = max(hausdorff([cv], [k["cvs"][i]]) for i, cv in enumerate(o["cvs"])) if o["cvs"] else 0.0
        hd = hausdorff(o["samples"], k["samples"])
        worst = max(worst, hd)
        status = "ok" if hd < 1e-6 and cv_dev < 1e-9 else "FAIL"
        if status == "FAIL":
            fails += 1
        print(f"{case['name']:14s} deg {o['degree']} domain[{o['domain'][0]:.4g},{o['domain'][1]:.4g}]  CVdev {cv_dev:.2e}  Hausdorff {hd:.2e}  {status}")
    print(f"\nworst Hausdorff {worst:.2e}; {fails} failure(s) of {len(CASES)}")


if __name__ == "__main__":
    main()
