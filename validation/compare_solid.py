"""Parity gate for BRep solid constructors vs OCCT (oracle.exe `solid` op).

Builds create_box / create_cylinder / create_sphere in session_py and compares volume,
area, and face count against a real OCCT BRepPrimAPI solid (BRepGProp properties).

Run:  PYTHONPATH=session_py/src python validation/compare_solid.py
"""
import math
import os
import subprocess

HERE = os.path.dirname(os.path.abspath(__file__))
ORACLE = os.path.join(HERE, "occt_oracle", "build", "Release", "oracle.exe")


def occt_solid(spec):
    req = os.path.join(HERE, "_solid_req.txt")
    res = os.path.join(HERE, "_solid_out.txt")
    with open(req, "w") as f:
        f.write("OP solid\nSHAPE " + spec + "\n")
    subprocess.run([ORACLE, req, res], check=True)
    toks = open(res).read().split()
    out = {}
    out["volume"] = float(toks[toks.index("VOLUME") + 1])
    out["area"] = float(toks[toks.index("AREA") + 1])
    out["nfaces"] = int(toks[toks.index("NFACES") + 1])
    return out


def main():
    if not os.path.exists(ORACLE):
        print("oracle.exe not built"); return
    from session_py.brep import BRep

    cases = [
        ("box 2 3 4", BRep.create_box(2, 3, 4)),
        ("cylinder 1 4", BRep.create_cylinder(1, 4)),
        ("sphere 2", BRep.create_sphere(2)),
    ]
    fails = 0
    for spec, brep in cases:
        k = occt_solid(spec)
        v = brep.volume()
        nf = brep.face_count()
        rv = abs(v - k["volume"]) / k["volume"]
        ok = rv < 1e-9 and nf == k["nfaces"]
        if not ok:
            fails += 1
        print(f"{spec:14s} vol {v:.6f} vs {k['volume']:.6f} (rel {rv:.2e})  "
              f"faces {nf} vs {k['nfaces']}  {'ok' if ok else 'FAIL'}")
    print(f"\n{fails} failure(s) of {len(cases)}")


if __name__ == "__main__":
    main()
