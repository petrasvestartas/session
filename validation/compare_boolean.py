"""Automated parity scorecard for BRep solid booleans vs OCCT (oracle `OP boolean`).

Runs the 15-primitive-pair x 3-op matrix (box, sphere, cylinder, cone, torus in overlapping
configs -- the same placements as brep_test.cpp's "ZZ NxN Boolean Matrix"). For each cell it
builds both operands in session_py and via OCCT `BRepAlgoAPI_{Fuse,Cut,Common}`, and compares:
  - volume (relative error; gate 1e-6 for the readout, 1e-9 is the eventual bar),
  - face count (exact),
  - is_solid() (watertight),
and prints one row per cell so the whole boolean frontier is visible at a glance.

The primitive placements are encoded ONCE as an oracle XF 7-tuple (tx ty tz ax ay az deg) and the
matching session Xform is derived from the SAME numbers, so both sides place operands identically.

Run:  PYTHONPATH=session_py/src python validation/compare_boolean.py
Env:  SESSION_BOOL_SHARED_EDGES=1 to exercise the shared-section-edge path (C++ only today).
"""
import math
import os
import subprocess

HERE = os.path.dirname(os.path.abspath(__file__))
ORACLE = os.path.join(HERE, "occt_oracle", "build", "Release", "oracle.exe")

IDENT = (0, 0, 0, 0, 0, 1, 0)  # tx ty tz ax ay az deg  (deg=0 -> no rotation)

# key -> (oracle_kind, params, xf 7-tuple).  Mirrors brep_test.cpp:783-792.
PLACEMENTS = {
    "box":   ("box",      (4, 4, 4),   IDENT),
    "sph":   ("sphere",   (2.5,),      IDENT),
    "cyl":   ("cylinder", (1.5, 6),    (0, 0, -3, 0, 0, 1, 0)),
    "cone":  ("cone",     (2.0, 4.0),  (0, 0, -2, 0, 0, 1, 0)),
    "tor":   ("torus",    (2.0, 0.8),  IDENT),
    "box2":  ("box",      (2, 2, 2),   (2, 0, 0, 0, 0, 1, 0)),
    "sph2":  ("sphere",   (2.0,),      (2, 0, 0, 0, 0, 1, 0)),
    "cyl2":  ("cylinder", (1.5, 6),    (-3, 0, 0, 0, 1, 0, 90)),
    "cone2": ("cone",     (2.0, 4.0),  (0, 0, 2, 1, 0, 0, 180)),
    "tor2":  ("torus",    (2.0, 0.8),  (2, 0, 0, 0, 0, 1, 0)),
}

# (label, keyA, keyB)  -- the 15 unordered primitive pairs, overlapping configs.
PAIRS = [
    ("box  x box ", "box",  "box2"),
    ("box  x sph ", "box",  "sph"),
    ("box  x cone", "box",  "cone"),
    ("box  x cyl ", "box",  "cyl"),
    ("box  x tor ", "box",  "tor"),
    ("sph  x sph ", "sph",  "sph2"),
    ("sph  x cone", "sph",  "cone"),
    ("sph  x cyl ", "sph",  "cyl"),
    ("sph  x tor ", "sph",  "tor"),
    ("cone x cone", "cone", "cone2"),
    ("cone x cyl ", "cone", "cyl"),
    ("cone x tor ", "cone", "tor"),
    ("cyl  x cyl ", "cyl",  "cyl2"),
    ("cyl  x tor ", "cyl",  "tor"),
    ("tor  x tor ", "tor",  "tor2"),
]


def shape_str(key):
    kind, params, xf = PLACEMENTS[key]
    return "SHAPE %s %s XF %s" % (
        kind,
        " ".join(repr(float(x)) for x in params),
        " ".join(repr(float(x)) for x in xf),
    )


def occt_boolean(mode, ka, kb):
    req = os.path.join(HERE, "_bool_req.txt")
    res = os.path.join(HERE, "_bool_out.txt")
    with open(req, "w") as f:
        f.write("OP boolean\nMODE %s\n%s\n%s\n" % (mode, shape_str(ka), shape_str(kb)))
    subprocess.run([ORACLE, req, res], check=True)
    toks = open(res).read().split()
    if "VOLUME" not in toks or "NFACES" not in toks:
        return None
    return {
        "volume": float(toks[toks.index("VOLUME") + 1]),
        "nfaces": int(toks[toks.index("NFACES") + 1]),
    }


def session_xform(xf):
    from session_py.xform import Xform
    tx, ty, tz, ax, ay, az, deg = xf
    t = Xform.translation(tx, ty, tz)
    if deg == 0:
        return t
    if ax:
        r = Xform.rotation_x(deg, degrees=True)
    elif ay:
        r = Xform.rotation_y(deg, degrees=True)
    else:
        r = Xform.rotation_z(deg, degrees=True)
    return t * r


def session_brep(key):
    from session_py.brep import BRep
    kind, params, xf = PLACEMENTS[key]
    ctor = {
        "box": BRep.create_box,
        "sphere": BRep.create_sphere,
        "cylinder": BRep.create_cylinder,
        "cone": BRep.create_cone,
        "torus": BRep.create_torus,
    }[kind]
    return ctor(*params).transformed(session_xform(xf))


def main():
    if not os.path.exists(ORACLE):
        print("oracle.exe not built:", ORACLE)
        return
    hdr = "%-13s %-4s | %11s %11s %9s | %4s %5s | solid | verdict" % (
        "pair", "op", "our_vol", "occt_vol", "rel", "ourF", "occtF")
    print(hdr)
    print("-" * len(hdr))
    fails = 0
    total = 0
    for label, ka, kb in PAIRS:
        for mode in ("cut", "common", "fuse"):
            total += 1
            k = occt_boolean(mode, ka, kb)
            try:
                A = session_brep(ka)
                B = session_brep(kb)
                if mode == "cut":
                    r = A.boolean_difference(B)
                elif mode == "common":
                    r = A.boolean_intersection(B)
                else:
                    r = A.boolean_union(B)
                v = r.volume()
                nf = r.face_count()
                solid = bool(r.is_solid())
            except Exception as e:
                print("%-13s %-4s | THREW: %s" % (label, mode, e))
                fails += 1
                continue
            if k is None:
                print("%-13s %-4s | our v=%.4f f=%d s=%d | OCCT FAILED" % (
                    label, mode, v, nf, int(solid)))
                fails += 1
                continue
            rel = abs(v - k["volume"]) / k["volume"] if k["volume"] else abs(v - k["volume"])
            ok = (rel < 1e-6) and (nf == k["nfaces"]) and solid
            if not ok:
                fails += 1
            print("%-13s %-4s | %11.4f %11.4f %9.2e | %4d %5d | %5d | %s" % (
                label, mode, v, k["volume"], rel, nf, k["nfaces"], int(solid),
                "OK" if ok else "FAIL"))
    print("\n%d/%d cells OK (volume rel<1e-6 AND exact faces AND is_solid)" % (total - fails, total))


if __name__ == "__main__":
    main()
