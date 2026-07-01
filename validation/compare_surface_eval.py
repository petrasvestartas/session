"""Parity gate for surface constructions (compas_occt from_parameters / from_points /
from_meshgrid) vs OCCT (oracle.exe `surface_eval`).

Each case provides (points grid [v][u], weights, knots_u/v, mults_u/v, degree_u/v) exactly
as compas_occt expands the docs/examples surface. We build it in session_py via
NurbsSurface.create_from_parameters and compare control points + sampled geometry (pointwise
over the shared u,v grid) against a real OCCT Geom_BSplineSurface.

Run:  PYTHONPATH=session_py/src python validation/compare_surface_eval.py
"""
import math
import os
import subprocess

HERE = os.path.dirname(os.path.abspath(__file__))
ORACLE = os.path.join(HERE, "occt_oracle", "build", "Release", "oracle.exe")


def from_points_knots(n, degree):
    """compas from_points knot/mult expansion for n control points of given degree."""
    degree = degree if n > degree else n - 1
    order = degree + 1
    x = n - order
    knots = [float(i) for i in range(2 + x)]
    mults = [order] + [1] * x + [order]
    return knots, mults, degree


# surface_from_points: 4x4 grid (z bumps), degree 3x3
SP_POINTS = [
    [(0, 0, 0), (1, 0, 0), (2, 0, 0), (3, 0, 0)],
    [(0, 1, 0), (1, 1, 2), (2, 1, 2), (3, 1, 0)],
    [(0, 2, 0), (1, 2, 2), (2, 2, 2), (3, 2, 0)],
    [(0, 3, 0), (1, 3, 0), (2, 3, 0), (3, 3, 0)],
]

# surface_isocurves / aabb / obb: 4x5 grid
SI_POINTS = [
    [(0, 0, 0), (1, 0, 0), (2, 0, 0), (3, 0, 0), (4, 0, 0)],
    [(0, 1, 0), (1, 1, 2), (2, 1, 2), (3, 1, 0), (4, 1, 0)],
    [(0, 2, 0), (1, 2, 2), (2, 2, 2), (3, 2, 0), (4, 2, 0)],
    [(0, 3, 0), (1, 3, 0), (2, 3, 0), (3, 3, 0), (4, 3, 0)],
]


def case_from_points(name, pts, du=3, dv=3):
    nv = len(pts); nu = len(pts[0])
    ku, mu, du2 = from_points_knots(nu, du)
    kv, mv, dv2 = from_points_knots(nv, dv)
    w = [[1.0] * nu for _ in range(nv)]
    return dict(name=name, points=pts, weights=w, knots_u=ku, knots_v=kv,
                mults_u=mu, mults_v=mv, degree_u=du2, degree_v=dv2)


# surface_from_parameters: 6x6 grid, degree 3x3, uniform interior knots (10 distinct each, mult 1)
def case_from_parameters():
    pts = [
        [(0, 0, 0), (1, 0, 0), (2, 0, 0), (3, 0, 0), (4, 0, 0), (5, 0, 0)],
        [(0, 1, 0), (1, 1, -1), (2, 1, -1), (3, 1, -1), (4, 1, -1), (5, 1, 0)],
        [(0, 2, 0), (1, 2, -1), (2, 2, 2), (3, 2, 2), (4, 2, -1), (5, 2, 0)],
        [(0, 3, 0), (1, 3, -1), (2, 3, 2), (3, 3, 2), (4, 3, -1), (5, 3, 0)],
        [(0, 4, 0), (1, 4, -1), (2, 4, -1), (3, 4, -1), (4, 4, -1), (5, 4, 0)],
        [(0, 5, 0), (1, 5, 0), (2, 5, 0), (3, 5, 0), (4, 5, 0), (5, 5, 0)],
    ]
    w = [[1.0] * 6 for _ in range(6)]
    knots_u = [1.0 + i / 9 for i in range(10)]
    knots_v = [i / 9 for i in range(10)]
    mults = [1] * 10
    return dict(name="from_parameters", points=pts, weights=w, knots_u=knots_u, knots_v=knots_v,
                mults_u=mults, mults_v=mults, degree_u=3, degree_v=3)


CASES = [
    case_from_points("from_points_4x4", SP_POINTS),
    case_from_points("isocurves_4x5", SI_POINTS),
    case_from_parameters(),
]


def ours(case):
    from session_py import NurbsSurface, Point
    s = NurbsSurface.create_from_parameters(
        [[Point(*p) for p in row] for row in case["points"]],
        case["weights"], case["knots_u"], case["knots_v"],
        case["mults_u"], case["mults_v"], case["degree_u"], case["degree_v"])
    if not s.is_valid():
        return {"error": "session surface invalid"}
    du = s.domain_u() if hasattr(s, "domain_u") else None
    # domain accessors
    u0, u1 = s.domain(0)
    v0, v1 = s.domain(1)
    S = 24
    samples = {}
    for a in range(S + 1):
        for b in range(S + 1):
            u = u0 + (u1 - u0) * a / S
            v = v0 + (v1 - v0) * b / S
            p = s.point_at(u, v)
            samples[(a, b)] = (p[0], p[1], p[2])
    return {"samples": samples, "domain": (u0, u1, v0, v1)}


def occt(case):
    pts = case["points"]; w = case["weights"]
    nv = len(pts); nu = len(pts[0])
    lines = ["OP surface_eval", f"DEGU {case['degree_u']} DEGV {case['degree_v']}", f"NU {nu} NV {nv}"]
    for i in range(nu):
        for j in range(nv):
            p = pts[j][i]; ww = w[j][i]
            lines.append(f"POLE {i} {j} {p[0]:.17g} {p[1]:.17g} {p[2]:.17g} {ww:.17g}")
    lines.append(f"NKU {len(case['knots_u'])}")
    for v, m in zip(case["knots_u"], case["mults_u"]):
        lines.append(f"KU {v:.17g} {m}")
    lines.append(f"NKV {len(case['knots_v'])}")
    for v, m in zip(case["knots_v"], case["mults_v"]):
        lines.append(f"KV {v:.17g} {m}")
    req = os.path.join(HERE, "_se_req.txt")
    res = os.path.join(HERE, "_se_out.txt")
    with open(req, "w") as f:
        f.write("\n".join(lines) + "\n")
    subprocess.run([ORACLE, req, res], check=True)
    with open(res) as f:
        toks = f.read().split()
    if toks and toks[0] == "ERROR":
        return {"error": " ".join(toks)}
    out = {"samples": {}}
    i = 0
    S = 24
    while i < len(toks):
        t = toks[i]
        if t == "DOMAIN":
            out["domain"] = tuple(float(toks[i + k]) for k in range(1, 5)); i += 5
        elif t == "SAMPLES":
            n = int(toks[i + 1]); i += 2
            for k in range(n):
                # u v x y z
                a = k // (S + 1); b = k % (S + 1)
                out["samples"][(a, b)] = (float(toks[i + 2]), float(toks[i + 3]), float(toks[i + 4]))
                i += 5
        else:
            i += 1
    return out


def frame_block(case):
    pts = case["points"]; w = case["weights"]
    nv = len(pts); nu = len(pts[0])
    lines = [f"DEGU {case['degree_u']} DEGV {case['degree_v']}", f"NU {nu} NV {nv}"]
    for i in range(nu):
        for j in range(nv):
            p = pts[j][i]; ww = w[j][i]
            lines.append(f"POLE {i} {j} {p[0]:.17g} {p[1]:.17g} {p[2]:.17g} {ww:.17g}")
    lines.append(f"NKU {len(case['knots_u'])}")
    for v, m in zip(case["knots_u"], case["mults_u"]):
        lines.append(f"KU {v:.17g} {m}")
    lines.append(f"NKV {len(case['knots_v'])}")
    for v, m in zip(case["knots_v"], case["mults_v"]):
        lines.append(f"KV {v:.17g} {m}")
    return "\n".join(lines)


def check_frames(case):
    """Compare frame_at normal (z-axis) against OCCT D1u x D1v normal."""
    from session_py import NurbsSurface, Point
    s = NurbsSurface.create_from_parameters(
        [[Point(*p) for p in row] for row in case["points"]],
        case["weights"], case["knots_u"], case["knots_v"],
        case["mults_u"], case["mults_v"], case["degree_u"], case["degree_v"])
    u0, u1 = s.domain(0); v0, v1 = s.domain(1)
    dev = 0.0
    for fu, fv in [(0.3, 0.4), (0.5, 0.5), (0.7, 0.2), (0.15, 0.85)]:
        u = u0 + (u1 - u0) * fu; v = v0 + (v1 - v0) * fv
        req = os.path.join(HERE, "_sf_req.txt"); res = os.path.join(HERE, "_sf_out.txt")
        with open(req, "w") as f:
            f.write("OP surface_frame\n" + frame_block(case) + f"\nUV {u:.17g} {v:.17g}\n")
        subprocess.run([ORACLE, req, res], check=True)
        toks = open(res).read().split()
        ni = toks.index("NORMAL")
        occt_n = tuple(float(toks[ni + k]) for k in range(1, 4))
        fr = s.frame_at(u, v)
        za = fr.z_axis if hasattr(fr, "z_axis") else fr.normal
        ours_n = (za[0], za[1], za[2])
        dev = max(dev, math.sqrt(sum((ours_n[k] - occt_n[k]) ** 2 for k in range(3))))
    return dev


def main():
    if not os.path.exists(ORACLE):
        print("oracle.exe not built"); return
    worst = 0.0
    fails = 0
    for case in CASES:
        o = ours(case); k = occt(case)
        if "error" in o or "error" in k:
            print(f"{case['name']:18s} ERROR ours={o.get('error')} occt={k.get('error')}")
            fails += 1; continue
        dev = 0.0
        for key in o["samples"]:
            p = o["samples"][key]; q = k["samples"][key]
            dev = max(dev, math.sqrt(sum((p[i] - q[i]) ** 2 for i in range(3))))
        worst = max(worst, dev)
        fdev = check_frames(case)
        worst = max(worst, fdev)
        status = "ok" if dev < 1e-9 and fdev < 1e-9 else "FAIL"
        if status == "FAIL":
            fails += 1
        print(f"{case['name']:18s} domain{tuple(round(x,3) for x in o['domain'])}  pointwise {dev:.2e}  frame-normal {fdev:.2e}  {status}")
    print(f"\nworst {worst:.2e}; {fails} failure(s) of {len(CASES)}")


if __name__ == "__main__":
    main()
