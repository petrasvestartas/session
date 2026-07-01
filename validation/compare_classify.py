"""Parity gate for BRep point-in-solid classification vs OCCT (oracle.exe `classify` op,
BRepClass3d_SolidClassifier).

Builds create_box/cylinder/sphere in session_py, classifies a set of points with
contains_point (ray-cast parity), and compares IN/OUT against OCCT. ON points (on the
boundary, measure zero) are skipped — mesh-based parity is for strict IN/OUT.

Run:  PYTHONPATH=session_py/src python validation/compare_classify.py
"""
import os
import subprocess

HERE = os.path.dirname(os.path.abspath(__file__))
ORACLE = os.path.join(HERE, "occt_oracle", "build", "Release", "oracle.exe")


def occt_classify(spec, pts):
    req = os.path.join(HERE, "_cls_req.txt")
    res = os.path.join(HERE, "_cls_out.txt")
    with open(req, "w") as f:
        f.write("OP classify\nSHAPE " + spec + " XF 0 0 0 0 0 1 0\n")
        f.write(f"NPTS {len(pts)}\n")
        for p in pts:
            f.write(f"PT {p[0]} {p[1]} {p[2]}\n")
    subprocess.run([ORACLE, req, res], check=True)
    toks = open(res).read().split()
    i = toks.index("STATES")
    n = int(toks[i + 1])
    return toks[i + 2:i + 2 + n]


def main():
    if not os.path.exists(ORACLE):
        print("oracle.exe not built"); return
    from session_py.brep import BRep

    cases = [
        ("box 2 3 4", BRep.create_box(2, 3, 4),
         [(0, 0, 0), (0.9, 1.4, 1.9), (1.1, 0, 0), (5, 0, 0), (-0.5, -1.0, -1.5), (0, 0, 2.5)]),
        ("cylinder 1 4", BRep.create_cylinder(1, 4),
         [(0, 0, 2), (0.9, 0, 2), (0, 0, -1), (1.1, 0, 2), (0, 0, 5), (0.5, 0.5, 1)]),
        ("sphere 2", BRep.create_sphere(2),
         [(0, 0, 0), (1.5, 0, 0), (3, 0, 0), (0, 0, 1.9), (1.2, 1.2, 0), (1.0, 1.0, 1.0)]),
    ]
    total = 0
    fails = 0
    for spec, brep, pts in cases:
        states = occt_classify(spec, pts)
        boundary = brep.mesh()
        for p, st in zip(pts, states):
            if st == "ON":
                continue  # boundary point, skip
            total += 1
            ours = brep.contains_point(p, boundary)
            occt_in = (st == "IN")
            ok = ours == occt_in
            if not ok:
                fails += 1
                print(f"  MISMATCH {spec} {p}: ours={'IN' if ours else 'OUT'} occt={st}")
        print(f"{spec:14s} {len(pts)} points checked vs OCCT")
    print(f"\n{total - fails}/{total} IN/OUT classifications match OCCT; {fails} failure(s)")


if __name__ == "__main__":
    main()
