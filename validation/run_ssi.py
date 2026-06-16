"""Compare our surface_surface against the OCCT oracle on a battery of surface
pairs. Both sides build the SAME finite geometry from primitive specs; we
compare the 3D intersection curves as point sets (symmetric Hausdorff) plus
branch counts. Run with: python validation/run_ssi.py
"""
import os, sys, subprocess, math
HERE = os.path.dirname(os.path.abspath(__file__))
ORACLE = os.path.join(HERE, "occt_oracle", "build", "Release", "oracle.exe")

from session_py import Xform, Vector
from session_py.primitives import Primitives as P
from session_py.intersection import surface_surface

# spec = (kind, [params], (tx,ty,tz,ax,ay,az,deg) or None)
# kinds/params match the oracle: cylinder[r,h] sphere[r] cone[r1,r2,h] torus[rmaj,rmin] plane[]
BATTERY = [
    ("cyl_x_cyl_ortho",   ("cylinder",[1,8],None),            ("cylinder",[2,8],(-4,0,4,0,1,0,90))),
    ("sphere_x_cyl_free", ("sphere",[2],None),                ("cylinder",[0.3,6],(1.3,0,-3,0,0,1,0))),
    ("sphere_x_plane",    ("sphere",[2],None),                ("plane",[],(0,0,0.4,0,0,1,0))),
    ("torus_x_plane",     ("torus",[2,0.5],None),             ("plane",[],(0,0,0,0,0,1,0))),
    ("cyl_x_plane_perp",  ("cylinder",[1,4],(0,0,-2,0,0,1,0)),("plane",[],None)),
    ("cyl_x_plane_tilt",  ("cylinder",[1,4],(0,0,-2,0,0,1,0)),("plane",[],(0,0,0,0,1,0,20))),
    ("disjoint",          ("sphere",[1],(0,0,0,0,0,1,0)),     ("sphere",[1],(10,0,0,0,0,1,0))),
]


def build_ours(spec):
    kind, prm, xf = spec
    if kind == "cylinder":
        s = P.cylinder_surface(0, 0, 0, prm[0], prm[1])
    elif kind == "sphere":
        s = P.sphere_surface(0, 0, 0, prm[0])
    elif kind == "cone":
        s = P.cone_surface(0, 0, 0, prm[0], prm[1], prm[2])
    elif kind == "torus":
        s = P.torus_surface(0, 0, 0, prm[0], prm[1])
    elif kind == "plane":
        from session_py import NurbsSurface, Point
        r = 100.0
        s = NurbsSurface.create(False, False, 1, 1, 2, 2,
                                [Point(-r,-r,0), Point(-r,r,0), Point(r,-r,0), Point(r,r,0)])
    if xf is not None:
        tx,ty,tz,ax,ay,az,deg = xf
        if deg != 0:
            s.transform(Xform.rotation(Vector(ax,ay,az), deg, True))
        s.transform(Xform.translation(tx,ty,tz))
    return s


def ours_curves(a, b):
    triples = surface_surface(a, b)
    out = []
    for c3, pa, pb in triples:
        d0, d1 = c3.domain()
        out.append([c3.point_at(d0 + (d1-d0)*k/80.0) for k in range(81)])
    return out


def occt_curves(specA, specB):
    if not os.path.exists(ORACLE):
        return None
    req = os.path.join(HERE, "_ssi_req.txt")
    res = os.path.join(HERE, "_ssi_out.txt")
    def emit(f, spec):
        kind, prm, xf = spec
        f.write("SURF " + kind + "".join(" " + str(x) for x in prm) + "\n")
        if xf is not None:
            f.write("XF " + " ".join(str(x) for x in xf) + "\n")
    with open(req, "w") as f:
        f.write("OP ssi\n")
        emit(f, specA); emit(f, specB)
        f.write("TOL 1e-6\n")
    subprocess.run([ORACLE, req, res], check=True)
    toks = open(res).read().split()
    if toks and toks[0] == "ERROR":
        return {"error": " ".join(toks)}
    curves = []
    i = 0
    while i < len(toks):
        if toks[i] == "NCURVES":
            i += 2
        elif toks[i] == "CURVE":
            m = int(toks[i+1]); i += 2
            pts = []
            for _ in range(m):
                pts.append((float(toks[i]), float(toks[i+1]), float(toks[i+2]))); i += 3
            curves.append(pts)
        else:
            i += 1
    return curves


def _pt_seg(p, a, b):
    abx, aby, abz = b[0]-a[0], b[1]-a[1], b[2]-a[2]
    apx, apy, apz = p[0]-a[0], p[1]-a[1], p[2]-a[2]
    ll = abx*abx + aby*aby + abz*abz
    t = 0.0 if ll < 1e-30 else max(0.0, min(1.0, (apx*abx+apy*aby+apz*abz)/ll))
    cx, cy, cz = a[0]+t*abx, a[1]+t*aby, a[2]+t*abz
    return math.sqrt((p[0]-cx)**2 + (p[1]-cy)**2 + (p[2]-cz)**2)


def _pt_to_curves(p, curves):
    best = float('inf')
    for c in curves:
        for i in range(len(c)-1):
            d = _pt_seg(p, c[i], c[i+1])
            if d < best:
                best = d
    return best


def curve_hausdorff(ca, cb):
    """Symmetric Hausdorff between two sets of polylines (point-to-segment),
    robust to differing sample density."""
    da = max(_pt_to_curves(p, cb) for c in ca for p in c)
    db = max(_pt_to_curves(p, ca) for c in cb for p in c)
    return max(da, db)


def _inv_xf_point(p, xf):
    """Inverse of the spec transform (rotate(axis,deg) then translate(t)):
    p_local = Rot(axis,-deg) (p - t)."""
    if xf is None:
        return (p[0], p[1], p[2])
    tx, ty, tz, ax, ay, az, deg = xf
    q = (p[0]-tx, p[1]-ty, p[2]-tz)
    if deg == 0:
        return q
    al = math.sqrt(ax*ax+ay*ay+az*az)
    k = (ax/al, ay/al, az/al)
    th = -deg*math.pi/180.0
    c, s = math.cos(th), math.sin(th)
    kd = k[0]*q[0]+k[1]*q[1]+k[2]*q[2]
    kx = (k[1]*q[2]-k[2]*q[1], k[2]*q[0]-k[0]*q[2], k[0]*q[1]-k[1]*q[0])
    return tuple(q[i]*c + kx[i]*s + k[i]*kd*(1-c) for i in range(3))


def exact_surface_dist(p, spec):
    """Exact distance from p to the analytic surface described by spec
    (parameterization-independent ground truth for the quadric battery)."""
    kind, prm, xf = spec
    q = _inv_xf_point(p, xf)
    if kind == "sphere":
        return abs(math.sqrt(q[0]**2 + q[1]**2 + q[2]**2) - prm[0])
    if kind == "cylinder":
        return abs(math.sqrt(q[0]**2 + q[1]**2) - prm[0])
    if kind == "plane":
        return abs(q[2])
    if kind == "cone":
        r1, r2, h = prm
        alpha = math.atan2(r2 - r1, h)
        axd = q[2]
        perp = math.sqrt(q[0]**2 + q[1]**2)
        return abs(perp - (r1 + axd*math.tan(alpha))) * math.cos(alpha)
    if kind == "torus":
        R, r = prm
        return abs(math.sqrt((math.sqrt(q[0]**2 + q[1]**2) - R)**2 + q[2]**2) - r)
    return 0.0


def implicit_error(curves, sa, sb):
    """Max EXACT distance of our intersection curves to BOTH analytic surfaces
    (a correct intersection point lies on both; independent of OCCT sampling and
    of the buggy Closest.surface_point seam behaviour)."""
    worst = 0.0
    for c in curves:
        for p in c:
            worst = max(worst, exact_surface_dist(p, sa), exact_surface_dist(p, sb))
    return worst


def main():
    print(f"{'case':22} {'ours #':7} {'occt #':7} {'Hausdorff':12} {'on-surfaces':12} verdict")
    print("-" * 84)
    for name, sa, sb in BATTERY:
        a, b = build_ours(sa), build_ours(sb)
        oc = ours_curves(a, b)
        kc = occt_curves(sa, sb)
        if isinstance(kc, dict):
            print(f"{name:22} occt-error: {kc['error']}"); continue
        n_ours = sum(1 for c in oc if len(c) > 1)
        n_occt = sum(1 for c in (kc or []) if len(c) > 1)
        if n_ours == 0 and n_occt == 0:
            print(f"{name:22} {0:<7} {0:<7} {'(both empty)':12} {'-':12} MATCH"); continue
        if n_ours == 0 or n_occt == 0:
            print(f"{name:22} {n_ours:<7} {n_occt:<7} {'one empty':12} {'-':12} DIVERGE"); continue
        h = curve_hausdorff(oc, kc)
        impl = implicit_error(oc, sa, sb)
        verdict = "1e-6" if impl < 1e-6 else ("MATCH" if h < 0.02 else ("close" if h < 0.1 else "DIVERGE"))
        print(f"{name:22} {n_ours:<7} {n_occt:<7} {h:<12.4f} {impl:<12.2e} {verdict}")


if __name__ == "__main__":
    main()
