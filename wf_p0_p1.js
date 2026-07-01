export const meta = {
  name: 'p0-p1-shared-section-edges',
  description: 'BUILDSPEC P0 (shared section-edge backbone) + P1 (pave engine / PostTreatFF), flag-gated SESSION_BOOL_SHARED_EDGES, oracle-verified, green set must stay green',
  phases: [
    { title: 'P0-impl',   detail: 'make_shared_section_edges + SESSION_BOOL_SHARED_EDGES, box-sphere watertight w/o 9% corruption' },
    { title: 'P0-verify', detail: 'adversarial: green set unchanged + box-sphere shared-edge is_solid + volume rel-err' },
    { title: 'P1-impl',   detail: 'generalize to OCCT pave engine: closing/bound/interior paves + PostTreatFF' },
    { title: 'P1-verify', detail: 'adversarial: box-cyl multi-segment + box-cone 2-arc cut, green set unchanged' },
  ],
}

// ---------- shared context (the BUILDSPEC P0/P1 spec) ----------
const REPO = 'C:\\pc\\3_code\\code_rust\\session';
const CPP  = REPO + '\\session_cpp';
const COMMON = `
You are working in the C++ geometry kernel at ${CPP}. Do NOT touch session_rust/ or session_py/.
Do NOT git commit. Do NOT add any "Co-Authored-By" or Claude attribution anywhere.

REPO REFERENCES (read first):
- ${REPO}\\OCCT_BOOLEAN_BUILDSPEC.md  — the sequenced spec. Read sections "P0 — Shared section-edge backbone",
  "P1 — Pave engine", and "3. MINIMAL FIRST PHASE (P0)".
- ${REPO}\\OCCT_STUDY_sec.md — section-edge / IntTools_Curve pseudo-code.
- ${CPP}\\src\\brep.cpp — the boolean pipeline: BRep::boolean(), imprint_edges(), co_refine_coincident_edges()
  (the CURRENT P0 stopgap), sew_coincident_edges(), subset(), is_solid(), volume(). Locate these by NAME (line
  numbers in the spec are stale — the file was edited since).
- ${CPP}\\src\\intersection.cpp — Intersection::surface_surface() returns (c3d, pa, pb) triples (OCCT
  IntTools_Curve {Curve, FirstCurve2d, SecondCurve2d}); analytic_ssi / analytic_pcurve / analytic_sphere_pullback
  / analytic_cone_pullback produce the exact section curve + per-surface UV pullbacks.
- ${CPP}\\src\\nurbssurface_trimmed.{h,cpp} — split_by_uv_curves(); NurbsCurve::segment(t0,t1) extracts a true
  sub-curve preserving parent knots (OCCT same-parameter guarantee — use it, do NOT refit).

BUILD/TEST (warm build_rel; the WireSplitter work has already landed on this tree):
  cmake --build build_rel --target point_minitest --config Release      # from ${CPP}
  ./build_rel/Release/point_minitest.exe                                # prints "[cpp-minitest] N/N passed"
If a link error says "cannot open ... point_minitest.exe" (transient exe lock), wait ~60s and retry the build ONCE.

GREEN SET (must stay byte-identical with the flag OFF, and correct with the flag ON):
  box-box, box-cyl, sphere-cyl, box-sphere  — all of {fuse, cut, common}. The full suite baseline is 746/746.
`;

const IMPL = {
  type: 'object', additionalProperties: false,
  properties: {
    success:        { type: 'boolean' },
    build_clean:    { type: 'boolean' },
    suite_count:    { type: 'string',  description: 'e.g. "746/746"' },
    flag_off_green: { type: 'boolean', description: 'with SESSION_BOOL_SHARED_EDGES unset, suite unchanged' },
    flag_on_results:{ type: 'string',  description: 'per-pair {faces, volume, is_solid} with the flag ON' },
    files_changed:  { type: 'array',   items: { type: 'string' } },
    new_functions:  { type: 'array',   items: { type: 'string' } },
    notes:          { type: 'string',  description: 'honest account of what works, what does not, residual risk' },
  },
  required: ['success','build_clean','suite_count','flag_off_green','flag_on_results','notes'],
};

const VERDICT = {
  type: 'object', additionalProperties: false,
  properties: {
    verdict:             { type: 'string', enum: ['pass','fail'] },
    flag_off_green:      { type: 'boolean' },
    regressions:         { type: 'array', items: { type: 'string' } },
    box_sphere_is_solid: { type: 'boolean' },
    volume_rel_err:      { type: 'number', description: 'box-sphere volume rel-error vs oracle (P0 target <1e-3, stretch <1e-9)' },
    issues:              { type: 'array', items: { type: 'string' } },
    recommendation:      { type: 'string' },
  },
  required: ['verdict','flag_off_green','regressions','recommendation'],
};

// ============================ P0 ============================
phase('P0-impl');
const p0 = await agent(`${COMMON}

TASK = BUILDSPEC P0: SHARED SECTION-EDGE BACKBONE (flag SESSION_BOOL_SHARED_EDGES).
Goal: make the A∩B section a SINGLE shared edge referenced by trims on faces of BOTH operands, so
watertightness is automatic (no Hausdorff sew). PROOF target: box(4)∩sphere(2.5) {fuse,cut,common} all
is_solid=1 with CORRECT volume (the current co_refine stopgap is watertight but a different code path; P0 is the
principled version behind its own flag).

IMPLEMENT (new, in brep.cpp):
1. NEW free/member fn make_shared_section_edges(A, B, result): for each candidate face-pair (fa in A, fb in B),
   call Intersection::surface_surface(fa.surface, fb.surface) -> list of (c3d, pa, pb) triples. For each triple:
   - append c3d to result.m_curves_3d ONCE -> index ce;
   - determine SEAM PAVES: the parameters on c3d where the periodic side's pullback (pb on a periodic fb, or pa
     on a periodic fa) crosses the surface seam (reuse the seam-crossing arc boundaries already computed inside
     analytic_sphere_pullback / analytic_cone_pullback, which return MULTIPLE arcs). Build a sorted pave-param
     list on c3d (endpoints + seam crossings; for a closed circle add a closing pave so it splits into a
     closeable edge).
   - For each consecutive [t_i, t_{i+1}]: NurbsCurve::segment all THREE of (c3d, pa, pb) on the SAME params
     (parent knots preserved — true sub-curve, no refit). Create ONE BRepEdge{curve_3d_index = (c3d segment)}
     -> index e. Register a trim {curve_2d_index = pa_seg, edge_index = e} into fa's loop and a trim
     {curve_2d_index = pb_seg, edge_index = e} into fb's loop — SAME edge_index e on both sides (the shared edge).
2. Feed the pa segments to fa's split (split_by_uv_curves) and the pb segments to fb's; POST-ASSIGN the produced
   boundary edges to the shared e by matching the pcurve segment identity (NOT by Hausdorff distance).
3. In BRep::boolean(): behind \`static const bool s_shared = (std::getenv("SESSION_BOOL_SHARED_EDGES") != nullptr);\`,
   when s_shared use make_shared_section_edges instead of the independent split_by_brep + the A∩B branch of
   sew_coincident_edges. Keep imprint_edges for intra-operand T-junctions. When the flag is UNSET, the existing
   path (co_refine + sew) runs UNCHANGED — the suite MUST stay 746/746.

CHECKPOINTS:
- Build clean. With the flag UNSET: ./build_rel/Release/point_minitest.exe == 746/746 (no regression).
- Add a SMALL removable probe (gated by its own env e.g. SESSION_P0_PROBE) in brep_test.cpp building
  box(4) {fuse,cut,common} sphere(2.5) and printing faces / volume / is_solid. Run
  \`SESSION_BOOL_SHARED_EDGES=1 SESSION_P0_PROBE=1 ./build_rel/Release/point_minitest.exe\`. Target: all 3 ops
  is_solid=1; cut≈9.546, common≈54.454, fuse≈75 (sum cut+common≈64). REMOVE the probe before finishing.
- Risk/mitigation (from spec): pa/pb seam-segment COUNT must match (derive both from the SINGLE pave list on c3d).
  If counts diverge -> mismatched edges. Assert equal segment counts; if not, log and fall back.

Return the IMPL struct honestly. The flag-OFF 746/746 is the hard gate; flag-ON box-sphere watertight is the goal.`,
  { label: 'P0-impl', phase: 'P0-impl', schema: IMPL });
log(`P0-impl: success=${p0 && p0.success} suite=${p0 && p0.suite_count} flagOffGreen=${p0 && p0.flag_off_green}`);

phase('P0-verify');
const p0v = await agent(`${COMMON}

TASK = ADVERSARIALLY VERIFY BUILDSPEC P0 (do NOT re-implement; only verify by building/running and trying to
break it). The P0 implementer reported: ${JSON.stringify(p0)}.

1. Confirm the build is clean and, with SESSION_BOOL_SHARED_EDGES UNSET, the suite is 746/746 (flag-off must be a
   no-op). If it is not 746/746, that is an automatic FAIL — report the failing tests.
2. Re-add a tiny probe (own env gate) OR reuse the implementer's probe to run, with SESSION_BOOL_SHARED_EDGES=1,
   the FULL green set: box-box, box-cyl, sphere-cyl, box-sphere — ALL of {fuse,cut,common}. For each: faces,
   volume, is_solid. Compare volumes to known-good values (box-sphere cut≈9.546/common≈54.454/fuse≈75;
   box-cyl com=9pi-ish per config; box-box exact). Any green-set pair that becomes is_solid=0 OR shifts volume
   under the flag is a REGRESSION.
3. Try to BREAK it: an off-center box-sphere, a box-sphere where the section does NOT straddle the seam, and a
   contained sphere — does the shared-edge path stay watertight + correct? Report any case that breaks.
4. Remove any probe you added.

Return the VERDICT struct. Be skeptical — default to verdict='fail' if flag-off is not 746/746 or any green-set
regression appears under the flag.`,
  { label: 'P0-verify', phase: 'P0-verify', schema: VERDICT });
log(`P0-verify: ${p0v && p0v.verdict} flagOffGreen=${p0v && p0v.flag_off_green} regressions=${(p0v && p0v.regressions ? p0v.regressions.length : '?')}`);

// ============================ P1 ============================
phase('P1-impl');
const p1 = await agent(`${COMMON}

CONTEXT: P0 (make_shared_section_edges, flag SESSION_BOOL_SHARED_EDGES) is implemented and verified.
P0 result: ${JSON.stringify(p0)}. P0 verify: ${JSON.stringify(p0v)}.

TASK = BUILDSPEC P1: PAVE ENGINE (split-exactly-once + PostTreatFF). Generalize P0's seam-only paves to arbitrary
section topology so multi-segment sections and section-section crossings (3-surface corners) become shared edges.
Extend make_shared_section_edges in brep.cpp (still behind SESSION_BOOL_SHARED_EDGES). Port the OCCT pave model:
- struct Pave{ int vid; double t; }; struct PaveBlock{ orig curve idx; edge idx; Pave p1,p2; std::vector<Pave> ext; };
- PutPavesOnCurve: project existing On/In vertices onto c3d via Closest::curve_point; dedup parametrically with
  contains_param(t, c3d.resolution(tol)).
- PutBoundPaveOnCurve: endpoint vertices when free and valid-for-faces.
- PutClosingPaveOnCurve: if c3d.point_at(t0) ≈ point_at(t1), append a second pave reusing the same vid at the
  opposite bound (splits a closed circle into a closeable edge).
- update_paveblock: std::sort ext+bounds by parameter; emit consecutive [t_i,t_{i+1}] segments -> one shared edge
  each (the P0 MakeEdge + two-pcurve core). Skip zero-length segments (|t1-t2| < pconf).
- PostTreatFF: run Closest::curve_curve over all section edges; split where two section curves cross; fuse
  coincident vids (3-surface corner vertices shared across all incident faces).
segment c3d/pa/pb over IDENTICAL [t_i,t_{i+1}] (parent knots preserved). Pave dedup tol = curve.resolution(tol3d);
vertex dedup is a 3D tolerance-ball.

CHECKPOINTS:
- Build clean; flag UNSET -> suite still 746/746.
- Probe (own env gate, removable): with SESSION_BOOL_SHARED_EDGES=1, verify box(4)-cyl(1.5,6) {fuse,cut,common}
  (multi-segment lateral+cap section) is_solid=1 + correct volume, AND box(4)-cone(2,4) {cut} where the plane
  produces 2 arcs is at least is_solid=1 (volume best-effort). The P0 box-sphere case must STILL pass. Remove the probe.
- Mitigation: assert monotone sorted params, no zero-length segments. If PostTreatFF over-splits, gate it so the
  P0 box-sphere path is unaffected.

Return the IMPL struct honestly.`,
  { label: 'P1-impl', phase: 'P1-impl', schema: IMPL });
log(`P1-impl: success=${p1 && p1.success} suite=${p1 && p1.suite_count} flagOffGreen=${p1 && p1.flag_off_green}`);

phase('P1-verify');
const p1v = await agent(`${COMMON}

TASK = ADVERSARIALLY VERIFY BUILDSPEC P1 (verify only; do not re-implement). P1 impl reported: ${JSON.stringify(p1)}.
1. Flag UNSET -> suite 746/746 (else FAIL).
2. Flag ON (SESSION_BOOL_SHARED_EDGES=1): re-run the FULL green set (box-box, box-cyl, sphere-cyl, box-sphere all
   3 ops) AND the new P1 targets (box-cyl multi-segment already in green set; box-cone {cut} 2-arc). Any green-set
   regression (is_solid flip or volume shift) is a FAIL.
3. Try to break PostTreatFF: a config where two section curves nearly-touch (should NOT be fused) and one where
   they truly cross (should be fused) — report mis-splits / mis-fuses.
4. Remove any probe you added.
Return VERDICT. Default to 'fail' if flag-off is not 746/746 or any green-set regression appears.`,
  { label: 'P1-verify', phase: 'P1-verify', schema: VERDICT });
log(`P1-verify: ${p1v && p1v.verdict} regressions=${(p1v && p1v.regressions ? p1v.regressions.length : '?')}`);

return { p0, p0v, p1, p1v };
