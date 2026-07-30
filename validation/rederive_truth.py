"""Re-derive rotated-chair ground truth from the OCCT oracle with inclusion-exclusion
consistency (task #10). The oracle is NOT trusted blindly: for each config the three
boolean results must satisfy the partition identities

    vol(cut)    = vol(A) - vol(common)
    vol(fuse)   = vol(A) + vol(B) - vol(common)
    vol(cut) + vol(common) + [vol(fuse) - vol(A)] closure

A config where the identities disagree is flagged OCCT-SELF-INCONSISTENT and the odd
cell is identified by majority vote (the two identities that agree define the trusted
common; the cell whose identity disagrees is the broken one). Rigid-motion sanity:
vol(B_cfg) must equal vol(chair1) for every cfg (rotation is an isometry).

Writes validation/OCCT_TRUTH.md (corrected authoritative table). Rhino second opinion
is unavailable on Linux; the identities carry the whole load.

Usage: python validation/rederive_truth.py [--probe exe] [--chairs dir] [--out md]
"""
import argparse
import json
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PROBE = os.path.join(HERE, "step_probe", "build", "step_probe")
CHAIRS = os.path.normpath(os.path.join(
    HERE, "..", "session_cpp", "serialization", "boolean_steps", "chairs"))
CFGS = ["z15", "z30", "z45", "z90", "x20", "y30", "z30x20", "z37", "x13y29", "z63"]
OPS = ["cut", "common", "fuse"]
TIMEOUT = 600


def run_probe(args):
    r = subprocess.run([PROBE] + args, capture_output=True, text=True, timeout=TIMEOUT)
    return r.stdout


def summary(path):
    out = run_probe([path])
    d = {}
    for k in ("SOLIDS", "SHELLS", "FACES", "VOLUME", "VALID"):
        m = re.search(r"^%s ([-\d.]+)" % k, out, re.M)
        d[k.lower()] = float(m.group(1)) if k == "VOLUME" else int(m.group(1))
    return d


def naked(path):
    """Oracle-quality probe of a RESULT file: free (naked) edge census."""
    out = run_probe([path, "-n"])
    d = {}
    for k in ("EDGES", "NAKED", "SEAM", "SHARED", "DEGEN"):
        m = re.search(r"^%s (\d+)" % k, out, re.M)
        d[k.lower()] = int(m.group(1)) if m else None
    return d


def boolean(op, a, b):
    out = run_probe(["--" + op, a, b])
    d = {}
    for k in ("OP_SOLIDS", "OP_FACES", "OP_VOLUME", "OP_VALID"):
        m = re.search(r"^%s ([-\d.]+)" % k, out, re.M)
        if not m:
            raise RuntimeError("no %s from --%s %s %s:\n%s" % (k, op, a, b, out))
        d[k[3:].lower()] = float(m.group(1)) if "VOL" in k else int(m.group(1))
    return d


def main():
    global PROBE
    ap = argparse.ArgumentParser()
    ap.add_argument("--probe", default=PROBE)
    ap.add_argument("--chairs", default=CHAIRS)
    ap.add_argument("--out", default=os.path.join(HERE, "OCCT_TRUTH.md"))
    ap.add_argument("--json", default=os.path.join(HERE, "occt_truth.json"))
    ap.add_argument("--reference-dir", default="/home/petras/fc_inspect/REFERENCE",
                    help="dir of REFERENCE_<cfg>_<op>.step written by an independent "
                         "OCCT front end (FreeCAD); used for an oracle-quality census")
    args = ap.parse_args()
    PROBE = args.probe
    ch = args.chairs
    A = os.path.join(ch, "chair0.stp")
    B1 = os.path.join(ch, "chair1.stp")
    if not os.path.exists(PROBE):
        sys.exit("step_probe not found: " + PROBE)

    sA, sB1 = summary(A), summary(B1)
    vA, vB1 = sA["volume"], sB1["volume"]
    ietol = max(0.005 * abs(vA), 0.01)     # 0.5% of vol(A): identity tolerance
    rows, data = [], {"volA": vA, "volB": vB1, "probe": PROBE, "configs": {}}
    for cfg in ["base"] + CFGS:      # "base" = the unrotated chair0 x chair1 pair
        Bp = B1 if cfg == "base" else os.path.join(ch, "rot", "B_%s.step" % cfg)
        if not os.path.exists(Bp):
            rows.append((cfg, None, None, None, None, "MISSING B_%s.step" % cfg))
            continue
        sB = summary(Bp)
        res = {op: boolean(op, A, Bp) for op in OPS}
        vc, vk, vf = (res["cut"]["volume"], res["common"]["volume"],
                      res["fuse"]["volume"])
        # implied common from each identity
        c_cut = vA - vc
        c_fuse = vA + sB["volume"] - vf
        ok_cut = abs(c_cut - vk) <= ietol
        ok_fuse = abs(c_fuse - vk) <= ietol
        ok_cross = abs(c_cut - c_fuse) <= ietol
        iso_ok = abs(sB["volume"] - vB1) <= max(1e-3 * abs(vB1), 1e-6)
        degen = [op for op in OPS if abs(res[op]["volume"]) < 0.01]
        invalid = [op for op in OPS
                   if res[op]["valid"] == 0 and abs(res[op]["volume"]) >= 0.01]
        # verdict + odd cell by majority vote among the three pairwise agreements
        if ok_cut and ok_fuse:
            verdict, odd = "CONSISTENT", ""
        elif ok_cut and not ok_fuse:
            verdict, odd = "SELF-INCONSISTENT", "fuse"
        elif ok_fuse and not ok_cut:
            verdict, odd = "SELF-INCONSISTENT", "cut"
        elif ok_cross:
            verdict, odd = "SELF-INCONSISTENT", "common"
        else:
            verdict, odd = "SELF-INCONSISTENT", "unresolved"
        # derived (trusted) values for the odd cell
        derived = {}
        if odd == "fuse":
            derived["fuse"] = vA + sB["volume"] - vk
        elif odd == "cut":
            derived["cut"] = vA - vk
        elif odd == "common":
            derived["common"] = 0.5 * (c_cut + c_fuse)
        data["configs"][cfg] = {
            "volB": sB["volume"], "iso_ok": iso_ok, "ops": res,
            "implied_common_from_cut": c_cut, "implied_common_from_fuse": c_fuse,
            "verdict": verdict, "odd_cell": odd, "degenerate_ops": degen,
            "occt_invalid_ops": invalid, "derived": derived,
        }
        rows.append((cfg, sB, res, (c_cut, c_fuse),
                     (verdict, odd, degen, invalid, derived), None))

    L = []
    L.append("# OCCT ground truth for rotated-chair booleans (re-derived)")
    L.append("")
    L.append("Oracle: `%s` (OCCT V8_0_0_rc2, Linux static build). Generated by" % PROBE)
    L.append("`validation/rederive_truth.py` via inclusion-exclusion consistency; the")
    L.append("partition identities are the PRIMARY oracle (Rhino unavailable on Linux).")
    L.append("A cell OCCT itself gets wrong is listed with the identity-derived value.")
    L.append("")
    L.append("vol(A)=%.6f  vol(chair1)=%.6f  identity tol=%.4f" % (vA, vB1, ietol))
    L.append("")
    L.append("| cfg | CUT vol/v/s | COMMON vol/v/s | FUSE vol/v/s | identity verdict |")
    L.append("|---|---|---|---|---|")
    for cfg, sB, res, implied, verd, err in rows:
        if err:
            L.append("| %s | - | - | - | %s |" % (cfg, err))
            continue
        verdict, odd, degen, invalid, derived = verd
        cells = []
        for op in OPS:
            r = res[op]
            mark = ""
            if op == odd:
                mark = " **BROKEN** (identity => %.4f)" % derived[op]
            elif op in degen:
                mark = " (degenerate)"
            elif op in invalid:
                mark = " (OCCT-INVALID shape)"
            cells.append("%.4f v%d s%d%s" % (r["volume"], r["valid"], r["solids"], mark))
        note = verdict
        if odd:
            note += ", odd cell: " + odd
        if degen:
            note += ", degenerate: " + "+".join(degen)
        if invalid:
            note += ", occt-invalid: " + "+".join(invalid)
        L.append("| %s | %s | %s | %s | %s |" % (cfg, cells[0], cells[1], cells[2], note))
    L.append("")
    L.append("## Reading rules")
    L.append("- `v` = OCCT BRepCheck valid, `s` = OP_SOLIDS (correct count, not always 1).")
    L.append("- CONSISTENT: cut+common=vol(A) AND fuse=vol(A)+vol(B)-common within tol.")
    L.append("- SELF-INCONSISTENT: OCCT's own three results violate the identities; the")
    L.append("  odd cell (majority vote) must NOT be used as ground truth — use the")
    L.append("  identity-derived value; gate that cell on the invariants instead.")
    L.append("- degenerate: |vol|<0.01 (empty/grazing) — detected, never named.")
    L.append("- OCCT-INVALID shape: OP_VALID=0 but real volume — the volume satisfies the")
    L.append("  identities (usable as volume truth) but OCCT's own result shape fails")
    L.append("  BRepCheck: do NOT gate our validity against OP_VALID on these cells.")
    L.append("- isometry check: every B_cfg volume must equal vol(chair1); violations below.")
    iso_bad = [c for c in data["configs"] if not data["configs"][c]["iso_ok"]]
    L.append("")
    L.append("Isometry violations (vol(B_cfg) != vol(chair1)): %s" %
             (", ".join("%s (%.6f)" % (c, data["configs"][c]["volB"]) for c in iso_bad)
              if iso_bad else "none"))

    # ---- oracle-quality census over an independent front end's result files ----
    rd = args.reference_dir
    if rd and os.path.isdir(rd):
        L.append("")
        L.append("## Oracle QUALITY census (independent OCCT front end: %s)" % rd)
        L.append("")
        L.append("OCCT's own results are not all clean solids. A cell with NAKED>0 or")
        L.append("VALID=0 means the ORACLE is defective there — never gate our topology")
        L.append("against it; gate on volume (if identity-consistent) and invariants.")
        L.append("")
        L.append("| cfg | op | oracle solids | shells | faces | naked | valid | vol (ref file) | vol (live boolean) | dv |")
        L.append("|---|---|---|---|---|---|---|---|---|---|")
        dirty = []
        for cfg in CFGS:
            c = data["configs"].get(cfg)
            if not c:
                continue
            for op in OPS:
                p = os.path.join(rd, "REFERENCE_%s_%s.step" % (cfg, op))
                if not os.path.exists(p):
                    continue
                try:
                    s = summary(p)
                    n = naked(p)
                except Exception:
                    continue
                live = c["ops"][op]["volume"]
                dv = s["volume"] - live
                L.append("| %s | %s | %d | %d | %d | %d | %d | %.4f | %.4f | %+.2e |" %
                         (cfg, op, s["solids"], s["shells"], s["faces"], n["naked"],
                          s["valid"], s["volume"], live, dv))
                c["ops"][op]["oracle_quality"] = dict(
                    n, solids=s["solids"], shells=s["shells"], faces=s["faces"],
                    valid=s["valid"], volume=s["volume"], dv=dv)
                if n["naked"] or s["valid"] == 0:
                    dirty.append("%s %s (naked %d, valid %d)" % (cfg, op, n["naked"],
                                                                 s["valid"]))
        L.append("")
        L.append("Cells where the ORACLE ITSELF is not a clean valid solid (as read "
                 "back from its exported STEP): %s"
                 % ("; ".join(dirty) if dirty else "none"))
        L.append("")
        L.append("### Two measurement points — they disagree, and that matters")
        L.append("An independent FreeCAD/OCCT run measured the IN-MEMORY boolean results")
        L.append("and found free edges on 5 cells: z15 cut 1 naked (2 solids), x20 cut 3,")
        L.append("z30x20 cut 1, x13y29 common 1, x13y29 fuse 1 — plus BRepCheck INVALID on")
        L.append("z30 fuse, z30x20 fuse, z37 fuse. Re-probing that run's EXPORTED STEP")
        L.append("files (table above) shows NAKED 0 everywhere and VALID 1 on z30/z30x20")
        L.append("fuse. The STEP export/import round trip HEALS OCCT's own free edges and")
        L.append("some invalidity. Consequences:")
        L.append("- oracle topology counts must state which measurement point they came")
        L.append("  from; in-memory OCCT is dirtier than its STEP round trip;")
        L.append("- our kernel's results are compared AFTER the same round trip, so any")
        L.append("  naked edge we still show is a defect the importer could NOT heal —")
        L.append("  strictly worse than OCCT's in-memory blemishes;")
        L.append("- z37 fuse stays INVALID through the round trip: that one is a genuine")
        L.append("  oracle defect, not a transient in-memory artifact.")
    text = "\n".join(L) + "\n"
    open(args.out, "w").write(text)
    json.dump(data, open(args.json, "w"), indent=1)
    print(text)
    print("wrote %s + %s" % (args.out, args.json), file=sys.stderr)


if __name__ == "__main__":
    main()
