#!/usr/bin/env python3
"""v2_trace_diff -- name the FIRST STAGE at which an OCCT trace and a v2 trace diverge.

    v2_trace_diff.py <occt.trace> <v2.trace> [--tol 1e-6] [--rel 1e-6] [-v] [--row]

Both files are line-oriented "TAG key=value ..." records (validation/occt_trace/occt_trace.cpp
and src/v2/v2_dump.cpp write the same schema).  The two kernels do NOT share an index space, a
surface parameterisation, or a curve representation, so nothing here compares an index or a
parameter: every check is a COUNT or a 3D GEOMETRY match within a stated tolerance.

Stages are checked in pipeline order and the first FAIL is the answer; later stages are still
reported, because "everything after the first divergence is downstream" is a hypothesis, not a
fact, and the table is what lets a reader test it.

Statuses
  OK    counts equal and every record matched within tolerance
  FAIL  a count differs, or a record has no partner within tolerance
  N/A   the v2 trace's CAP record says this kernel does not populate the stage
  PROBE the v2 side comes from the interference PROBE arena, which the production v2 boolean
        does not run (src/v2/v2_dump.h) -- reported, never counted as a divergence
  INFO  a representation-policy quantity (raw arena sizes, degenerate-edge convention, curve
        type) -- printed so a reader sees it, never voted on

THE REFERENCE IS OCCT, WHICH IS NOT TRUTH.  kb/occt_trace_findings.md measured OCCT itself off
by 3.2e-3 in volume on sph_cyl at 23 deg (converged truth 50.3880515, OCCT 50.3846998).  A
res.volume FAIL there can mean our side is CLOSER.  Read res.volume against the converged value,
not against the reference column, whenever the case sits near a tangency.
"""

import math
import sys

# --------------------------------------------------------------------------- parsing


def parse(path):
    recs = []
    with open(path) as fh:
        for line in fh:
            line = line.rstrip("\n")
            if not line:
                continue
            parts = line.split(" ")
            tag = parts[0]
            kv = {}
            for p in parts[1:]:
                if "=" in p:
                    k, v = p.split("=", 1)
                    kv[k] = v
            recs.append((tag, kv))
    return recs


def sel(recs, _tag, **filt):
    out = []
    for t, kv in recs:
        if t != _tag:
            continue
        ok = True
        for k, v in filt.items():
            if kv.get(k) != v:
                ok = False
                break
        if ok:
            out.append(kv)
    return out


def selfinal(recs, tag):
    """Records of `tag` at the LAST pipeline state.  OCCT dumps tag=afterFF and tag=final;
    the v2 dump emits tag=final only.  Prefer final, fall back to whatever exists."""
    r = sel(recs, tag, tag="final")
    if r:
        return r
    r = [kv for t, kv in recs if t == tag and "tag" not in kv]
    if r:
        return r
    return sel(recs, tag, tag="afterFF")


def f(kv, k, d=float("nan")):
    v = kv.get(k)
    if v is None or v == "-":
        return d
    try:
        return float(v)
    except ValueError:
        return d


def i(kv, k, d=0):
    """Integer field.  Parsed through float so a value written as "1.0" or "1e2" is READ rather
    than silently replaced by the default -- a default-on-garbage path is a divergence the
    harness cannot see, and mutation_test.sh caught exactly that (res.naked, MISS -> CAUGHT)."""
    v = kv.get(k)
    if v is None or v == "-":
        return d
    try:
        return int(v)
    except ValueError:
        pass
    try:
        return int(round(float(v)))
    except ValueError:
        return d


def pt(kv, k="p"):
    v = kv.get(k)
    if not v or v == "-":
        return None
    try:
        x, y, z = v.split(",")
        return (float(x), float(y), float(z))
    except ValueError:
        return None


def dist(a, b):
    return math.sqrt((a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2 + (a[2] - b[2]) ** 2)


# --------------------------------------------------------------------------- matching


def match_points(A, B, tol):
    """Greedy nearest-neighbour match of two point multisets.

    Returns (n_matched, worst_matched_distance, unmatched_A, unmatched_B).  Greedy over the
    globally sorted candidate list is exact whenever the two sets are separated by more than
    2*tol, which is the regime every check here runs in; when they are not, the reported
    unmatched counts are an upper bound on the true mismatch, never an under-count.
    """
    if not A and not B:
        return 0, 0.0, [], []
    cand = []
    for ia, a in enumerate(A):
        for ib, b in enumerate(B):
            d = dist(a, b)
            if d <= tol:
                cand.append((d, ia, ib))
    cand.sort()
    ua, ub = set(range(len(A))), set(range(len(B)))
    worst = 0.0
    n = 0
    for d, ia, ib in cand:
        if ia in ua and ib in ub:
            ua.discard(ia)
            ub.discard(ib)
            worst = max(worst, d)
            n += 1
    return n, worst, [A[k] for k in sorted(ua)], [B[k] for k in sorted(ub)]


def match_scalars(A, B, rel, absv):
    """Sorted pairwise comparison of two scalar multisets.  Returns (n_bad, worst_rel)."""
    a, b = sorted(A), sorted(B)
    if len(a) != len(b):
        return max(len(a), len(b)) - min(len(a), len(b)), float("inf")
    bad, worst = 0, 0.0
    for x, y in zip(a, b):
        e = abs(x - y)
        r = e / max(abs(x), abs(y), 1e-30)
        worst = max(worst, r)
        if e > absv and r > rel:
            bad += 1
    return bad, worst


# --------------------------------------------------------------------------- resolvers


def vertex_tables(recs):
    """DSVERT index -> point / new-flag, plus the same-domain resolver.

    Both kernels leave an SD-superseded vertex IN the dump (OCCT does it in box_box_touch_fuse,
    v2 does it for every fused section node), so the only comparable vertex set is the CANONICAL
    one: resolve every index through the SD chain, then deduplicate.  Comparing the raw DSVERT
    list instead reports 40 vs 24 on box x box, where the two kernels in fact name the same 24
    points -- that was the first thing this harness got wrong."""
    pts, new, sd = {}, {}, {}
    for kv in selfinal(recs, "DSVERT"):
        ix = i(kv, "i", -1)
        p = pt(kv)
        if p is not None:
            pts[ix] = p
        new[ix] = kv.get("new") == "1"
        s = i(kv, "sd", -1)
        if s >= 0:
            sd[ix] = s

    def resolve(ix):
        seen = 0
        while ix in sd and seen < 64:
            ix = sd[ix]
            seen += 1
        return ix

    def point(ix):
        return pts.get(resolve(ix))

    return pts, new, resolve, point


def canonical_vertices(recs, only_new=False):
    pts, new, resolve, _ = vertex_tables(recs)
    out, seen = [], set()
    for ix in sorted(pts):
        c = resolve(ix)
        if c in seen:
            continue
        seen.add(c)
        if only_new and not new.get(c, False):
            continue
        if c in pts:
            out.append(pts[c])
    return out


def caps(recs):
    c = sel(recs, "CAP")
    if not c:
        return set(), set()
    have = set((c[0].get("stages") or "").split(","))
    missing = set((c[0].get("missing") or "").split(","))
    return have, missing


# --------------------------------------------------------------------------- stages

class Stage:
    def __init__(self, name, status, o, v, detail=""):
        self.name, self.status, self.o, self.v, self.detail = name, status, o, v, detail


def build_stages(O, V, tol, rel):
    S = []
    o_pts, o_new, o_res, ovp = vertex_tables(O)
    v_pts, v_new, v_res, vvp = vertex_tables(V)
    _, vmissing = caps(V)

    def add(name, status, o, v, detail=""):
        S.append(Stage(name, status, o, v, detail))

    def cmp_counts(name, o, v, detail=""):
        add(name, "OK" if o == v else "FAIL", o, v, detail)

    def info(name, o, v, detail=""):
        add(name, "INFO", o, v, detail)

    # ---- 1. input -----------------------------------------------------------------------
    oa, va = sel(O, "ARG"), sel(V, "ARG")

    def ndegen_edges(recs, argi):
        return sum(1 for k in sel(recs, "AEDGE", a=str(argi)) if k.get("degen") == "1")

    for n, (ao, av) in enumerate(zip(oa, va)):
        # NON-DEGENERATE edges only.  OCCT carries a DEGENERATED edge over every pole and apex
        # (measured: a sphere is 3 edges, 2 of them degenerate); this kernel's primitives carry
        # no edge record at a pole at all.  That is a documented convention difference, not a
        # defect, and comparing raw nedge makes it the first divergence of every curved case and
        # hides everything downstream.
        od, vd = ndegen_edges(O, n), ndegen_edges(V, n)
        cmp_counts(
            "input.counts.arg%d" % n,
            "%d/%d/%d" % (i(ao, "nface"), i(ao, "nedge") - od, i(ao, "nvert")),
            "%d/%d/%d" % (i(av, "nface"), i(av, "nedge") - vd, i(av, "nvert")),
            "nface/nedge(non-degenerate)/nvert",
        )
        info("input.degen_edges.arg%d" % n, od, vd,
             "pole/apex edge convention: OCCT emits one, our primitives emit none")
    for n, (ao, av) in enumerate(zip(oa, va)):
        bad, worst = match_scalars([f(ao, "vol")], [f(av, "vol")], 1e-6, 1e-9)
        add("input.volume.arg%d" % n, "OK" if bad == 0 else "FAIL",
            "%.9g" % f(ao, "vol"), "%.9g" % f(av, "vol"), "rel=%.3g" % worst)
    for n in range(min(len(oa), len(va))):
        A = [pt(kv) for kv in sel(O, "AVERT", a=str(n))]
        B = [pt(kv) for kv in sel(V, "AVERT", a=str(n))]
        A = [p for p in A if p]
        B = [p for p in B if p]
        m, worst, ua, ub = match_points(A, B, tol)
        add("input.vertices.arg%d" % n,
            "OK" if (len(A) == len(B) and not ua and not ub) else "FAIL",
            len(A), len(B),
            "matched=%d worst=%.3g unmatched_occt=%d unmatched_v2=%d" % (m, worst, len(ua), len(ub)))

    # ---- 2. arena -----------------------------------------------------------------------
    # RAW arena sizes are POLICY, not geometry: OCCT builds a pave-block pool only for an edge
    # that interferes, the v2 front materialises every edge's pool up front, and OCCT's DS also
    # carries SOLID/SHELL/WIRE rows.  These are printed as INFO so a reader sees the policy, and
    # never voted on; the comparable quantities are the derived ones below.
    for ty in ("VERTEX", "EDGE", "FACE"):
        info("ds.si.%s" % ty.lower(),
             len(sel(O, "SI", tag="final", type=ty)),
             len(sel(V, "SI", tag="final", type=ty)),
             "raw arena size (materialisation policy differs)")

    A = canonical_vertices(O)
    B = canonical_vertices(V)
    m, worst, ua, ub = match_points(A, B, tol)
    add("ds.vertices", "OK" if (len(A) == len(B) and not ua and not ub) else "FAIL",
        len(A), len(B),
        "SD-resolved+dedup; matched=%d worst=%.3g miss_occt=%s miss_v2=%s"
        % (m, worst, fmtpts(ua), fmtpts(ub)))

    A = canonical_vertices(O, only_new=True)
    B = canonical_vertices(V, only_new=True)
    m, worst, ua, ub = match_points(A, B, tol)
    add("ds.newvertices", "OK" if (len(A) == len(B) and not ua and not ub) else "FAIL",
        len(A), len(B),
        "the intersection nodes; matched=%d worst=%.3g miss_occt=%s miss_v2=%s"
        % (m, worst, fmtpts(ua), fmtpts(ub)))

    # SPLIT PAVES: paves at an intersection node ON AN OPERAND EDGE (SI rank 0 or 1).  Two
    # filters, both load bearing: the two bounding paves of every edge are bookkeeping, and a
    # pave on a CREATED edge (rank -1) is a section carrier -- OCCT keeps those inside
    # BOPDS_Curve where they surface as SECPB, the v2 arena keeps them as real edges, so
    # counting them here would compare a representation choice, not a geometric one.
    def edge_rank(recs):
        return {i(k, "i", -1): i(k, "rank", -1)
                for k in sel(recs, "SI", tag="final") if k.get("type") == "EDGE"}

    def split_paves(recs, res, newmap, ptf):
        rk = edge_rank(recs)
        out = []
        for k in selfinal(recs, "PAVE"):
            if rk.get(i(k, "e", -1), -1) < 0:
                continue
            c = res(i(k, "v", -1))
            if newmap.get(c, False):
                p = ptf(c)
                if p:
                    out.append(p)
        return out

    A = split_paves(O, o_res, o_new, ovp)
    B = split_paves(V, v_res, v_new, vvp)
    m, worst, ua, ub = match_points(A, B, tol)
    add("ds.split_paves", "OK" if (len(A) == len(B) and not ua and not ub) else "FAIL",
        len(A), len(B),
        "paves at intersection nodes; matched=%d worst=%.3g miss_occt=%s miss_v2=%s"
        % (m, worst, fmtpts(ua), fmtpts(ub)))

    # SPLITS: how many original edges got cut, and into how many extra pieces.
    def splits(recs):
        rk = edge_rank(recs)
        per = {}
        for k in selfinal(recs, "PB"):
            if rk.get(i(k, "orig", -1), -1) < 0:
                continue
            per[k.get("orig")] = per.get(k.get("orig"), 0) + 1
        nedges = sum(1 for v in per.values() if v > 1)
        nextra = sum(v - 1 for v in per.values())
        return nedges, nextra, len(per)

    oe, ox, on = splits(O)
    ve, vx, vn = splits(V)
    cmp_counts("ds.split_edges", oe, ve, "original edges carrying more than one pave block")
    cmp_counts("ds.pblock_splits", ox, vx, "sum(blocks-1) over original edges")
    info("ds.pblocks", len(selfinal(O, "PB")), len(selfinal(V, "PB")),
         "raw; pools on %d vs %d edges (materialisation policy)" % (on, vn))
    cmp_counts("ds.cblocks", len(selfinal(O, "CB")), len(selfinal(V, "CB")))

    ofi, vfi = selfinal(O, "FI"), selfinal(V, "FI")
    if "faceinfo" in vmissing:
        add("ds.faceinfo", "N/A", len(ofi), len(vfi),
            "the production v2 front never populates BdsFaceInfo In/On/Sc; face selection goes "
            "through classify-once + InParts instead")
    else:
        cmp_counts("ds.faceinfo", len(ofi), len(vfi))
        cmp_counts(
            "ds.faceinfo.sets",
            "%d/%d/%d" % (sum(i(k, "nIn") for k in ofi), sum(i(k, "nOn") for k in ofi),
                          sum(i(k, "nSc") for k in ofi)),
            "%d/%d/%d" % (sum(i(k, "nIn") for k in vfi), sum(i(k, "nOn") for k in vfi),
                          sum(i(k, "nSc") for k in vfi)),
            "In/On/Sc",
        )

    # ---- 3. interferences (v2 side is a PROBE) -------------------------------------------
    for tg in ("IVV", "IVE", "IVF", "IEE", "IEF"):
        no, nv = len(selfinal(O, tg)), len(selfinal(V, tg))
        st = "PROBE" if nv or no else "PROBE"
        add("interf.%s" % tg.lower(), st, no, nv,
            "v2 side is the probe arena; the production v2 boolean runs no VV/VE/EE/VF/EF")

    # ---- 4. section ----------------------------------------------------------------------
    osec, vsec = selfinal(O, "SEC"), selfinal(V, "SEC")
    cmp_counts("sec.curves", len(osec), len(vsec))

    def ends(k):
        a, b = pt(k, "p0"), pt(k, "p1")
        return (a, b)

    A = [e for e in (ends(k) for k in osec) if e[0] and e[1]]
    B = [e for e in (ends(k) for k in vsec) if e[0] and e[1]]
    # a section curve is matched when BOTH endpoints match, in either order
    used = set()
    matched, worst = 0, 0.0
    unm_o = []
    for a in A:
        best, bi = None, -1
        for n, b in enumerate(B):
            if n in used:
                continue
            d1 = max(dist(a[0], b[0]), dist(a[1], b[1]))
            d2 = max(dist(a[0], b[1]), dist(a[1], b[0]))
            d = min(d1, d2)
            if best is None or d < best:
                best, bi = d, n
        if bi >= 0 and best is not None and best <= tol:
            used.add(bi)
            matched += 1
            worst = max(worst, best)
        else:
            unm_o.append(a[0])
    unm_v = [B[n][0] for n in range(len(B)) if n not in used]
    add("sec.endpoints", "OK" if (len(A) == len(B) and not unm_o and not unm_v) else "FAIL",
        len(A), len(B),
        "matched=%d worst=%.3g miss_occt=%s miss_v2=%s"
        % (matched, worst, fmtpts(unm_o), fmtpts(unm_v)))

    bad, worst = match_scalars([f(k, "len") for k in osec if k.get("len") not in (None, "-")],
                               [f(k, "len") for k in vsec if k.get("len") not in (None, "-")],
                               1e-4, 1e-7)
    add("sec.lengths", "OK" if bad == 0 else "FAIL",
        len(osec), len(vsec), "bad=%d worst_rel=%.3g" % (bad, worst))

    # INFO, not a vote: OCCT names a Geom curve type outright, while v2_dump has to MEASURE one
    # from samples and its recogniser resolves only Line / Circle / Degenerated exactly --
    # everything else, an OCCT Ellipse included, comes back "BSpline".  Reporting that as a
    # divergence would be the harness inventing one (measured: box_cone_p2 is Ellipse x4 vs
    # BSpline x3 purely from the recogniser).
    info("sec.types",
         ",".join(sorted(k.get("type", "?") for k in osec)),
         ",".join(sorted(k.get("type", "?") for k in vsec)),
         "v2 side is a sampled recogniser: only Line/Circle/Degenerated are resolved exactly")

    # SECTION PAVE BLOCKS.  OCCT splits a section curve into one BOPDS_Curve per seam-delimited
    # arc, so sec.curves can differ from ours purely by grouping; the BLOCKS and their endpoint
    # nodes are the load-bearing geometry, and they are compared separately for exactly that
    # reason (measured: sph_cyl_roty24 is 3 curves vs 2 but 3 blocks vs 3).
    cmp_counts("sec.pblocks", len(selfinal(O, "SECPB")), len(selfinal(V, "SECPB")))
    A = [p for p in (ovp(i(k, v, -1)) for k in selfinal(O, "SECPB") for v in ("v1", "v2")) if p]
    B = [p for p in (vvp(i(k, v, -1)) for k in selfinal(V, "SECPB") for v in ("v1", "v2")) if p]
    m, worst, ua, ub = match_points(A, B, tol)
    add("sec.block_nodes", "OK" if (len(A) == len(B) and not ua and not ub) else "FAIL",
        len(A), len(B),
        "block endpoint nodes; matched=%d worst=%.3g miss_occt=%s miss_v2=%s"
        % (m, worst, fmtpts(ua), fmtpts(ub)))

    # ---- 5. result -----------------------------------------------------------------------
    ores, vres = sel(O, "RES"), sel(V, "RES")
    if not ores or not vres:
        add("res.present", "FAIL", len(ores), len(vres), "one side produced no RES record")
        return S
    ro, rv = ores[0], vres[0]

    if "imgface" in vmissing:
        add("res.imgface", "N/A", "-", "-", "v2_boolean does not expose an input->output face map")

    cmp_counts("res.faces", i(ro, "nface"), i(rv, "nface"))
    bad, worst = match_scalars([f(k, "area") for k in sel(O, "RESFACE")],
                               [f(k, "area") for k in sel(V, "RESFACE")], 1e-4, 1e-9)
    add("res.face_areas", "OK" if bad == 0 else "FAIL",
        len(sel(O, "RESFACE")), len(sel(V, "RESFACE")), "bad=%d worst_rel=%.3g" % (bad, worst))

    cmp_counts("res.edges", i(ro, "nedge") - i(ro, "ndegen"), i(rv, "nedge") - i(rv, "ndegen"),
               "non-degenerate only, same convention as input.counts")
    info("res.degen_edges", i(ro, "ndegen"), i(rv, "ndegen"))
    cmp_counts("res.verts", i(ro, "nvert"), i(rv, "nvert"))
    A = [p for p in (pt(k) for k in sel(O, "RESVERT")) if p]
    B = [p for p in (pt(k) for k in sel(V, "RESVERT")) if p]
    m, worst, ua, ub = match_points(A, B, max(tol, 1e-5))
    add("res.vert_positions", "OK" if (len(A) == len(B) and not ua and not ub) else "FAIL",
        len(A), len(B), "matched=%d worst=%.3g miss_occt=%s miss_v2=%s"
        % (m, worst, fmtpts(ua), fmtpts(ub)))

    cmp_counts("res.shells", i(ro, "nshell"), i(rv, "nshell"))
    cmp_counts("res.solids", i(ro, "nsolid"), i(rv, "nsolid"))
    cmp_counts("res.naked", i(ro, "naked"), i(rv, "naked"))
    cmp_counts("res.valid", i(ro, "valid"), i(rv, "valid"))

    bad, worst = match_scalars([f(ro, "vol")], [f(rv, "vol")], rel, 1e-9)
    add("res.volume", "OK" if bad == 0 else "FAIL",
        "%.9g" % f(ro, "vol"), "%.9g" % f(rv, "vol"), "rel=%.3g" % worst)
    bad, worst = match_scalars([f(ro, "area")], [f(rv, "area")], rel, 1e-9)
    add("res.area", "OK" if bad == 0 else "FAIL",
        "%.9g" % f(ro, "area"), "%.9g" % f(rv, "area"), "rel=%.3g" % worst)
    return S


def fmtpts(pts, n=2):
    if not pts:
        return "-"
    s = ";".join("(%.6g,%.6g,%.6g)" % p for p in pts[:n])
    if len(pts) > n:
        s += ";+%d" % (len(pts) - n)
    return s


# --------------------------------------------------------------------------- main


def main(argv):
    if len(argv) < 3:
        print(__doc__)
        return 2
    occt, v2 = argv[1], argv[2]
    tol, rel, verbose, row, md = 1e-6, 1e-6, False, False, False
    k = 3
    while k < len(argv):
        if argv[k] == "--tol":
            tol = float(argv[k + 1]); k += 2
        elif argv[k] == "--rel":
            rel = float(argv[k + 1]); k += 2
        elif argv[k] in ("-v", "--verbose"):
            verbose = True; k += 1
        elif argv[k] == "--row":
            row = True; k += 1
        elif argv[k] == "--md":
            md = True; k += 1
        else:
            k += 1

    O, V = parse(occt), parse(v2)
    name = (sel(O, "TRACE") or [{}])[0].get("name", "?")
    op = (sel(O, "TRACE") or [{}])[0].get("op", "?")
    S = build_stages(O, V, tol, rel)
    fails = [s for s in S if s.status == "FAIL"]
    first = fails[0] if fails else None
    # every failing stage, not only the first: "everything after the first divergence is
    # downstream" is a hypothesis, and the full signature is what lets a reader test it.
    allf = ",".join(s.name for s in fails) or "-"

    if md:
        print("| %s | %s | %s | %s | %s | %d | %s |"
              % (name, op, first.name if first else "**none**",
                 first.o if first else "-", first.v if first else "-",
                 len(fails), allf))
        return 0

    if row:
        print("%-24s %-7s %-22s %-13s %-13s %-2d %s"
              % (name, op, first.name if first else "NONE",
                 first.o if first else "-", first.v if first else "-",
                 len(fails), allf))
        return 0

    print("=== %s (%s)   occt=%s  v2=%s" % (name, op, occt, v2))
    print("%-26s %-14s %-14s %-6s %s" % ("STAGE", "OCCT", "V2", "", "DETAIL"))
    for s in S:
        if not verbose and s.status in ("OK",):
            print("%-26s %-14s %-14s %-6s" % (s.name, s.o, s.v, s.status))
        else:
            print("%-26s %-14s %-14s %-6s %s" % (s.name, s.o, s.v, s.status, s.detail))
    print("")
    if first:
        print("FIRST DIVERGENCE: %s   occt=%s  v2=%s   %s"
              % (first.name, first.o, first.v, first.detail))
        print("ALL FAILING STAGES (%d): %s" % (len(fails), allf))
    else:
        print("FIRST DIVERGENCE: none -- the two kernels agree on every compared stage")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
