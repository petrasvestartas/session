# Build Spec — Watertight + Volume-Correct Primitive-Pair Booleans via OCCT Shared-Section-Edge Architecture

All session paths are under `C:/pc/3_code/code_rust/session/session_cpp/src/`. Line anchors below were verified against the live tree (2026-06-30).

---

## 1. TARGET ARCHITECTURE — new boolean data flow

### 1.1 The single invariant that replaces imprint→independent-split→Hausdorff-sew

Today `BRep::boolean` (`brep.cpp:2272`) does: `split_by_brep(other)` on A **and** B independently (`:2284-2285`) → classify by `contains_point` (`:2354-2370`) → `subset`+concatenate (`:2375-2418`) → `imprint_edges` (`:2424`) → **`sew_coincident_edges` Hausdorff** (`:2430`, body `:1976`). The Hausdorff sew reconciles **two independently-fitted NURBS section curves** (A's circle vs B's circle) after the fact; that reconciliation is what corrupts the region (9% volume) and what cannot be made watertight without distorting geometry.

**Replace with OCCT's invariant: the section curve A∩B is computed ONCE, becomes ONE `m_curves_3d` entry and ONE `BRepEdge`, and that same edge index is referenced by trims on faces of BOTH operands.** There is then nothing to Hausdorff-match: the shared boundary is one geometric object, so the two operands' splits are forced to agree bit-exactly.

### 1.2 New pipeline (numbered; maps to OCCT `BOPAlgo_BOP::PerformInternal1` order)

```
boolean(A, B, op):
 (0) candidate face-pairs (AABB overlap prefilter)               [brep.cpp::boolean, new]
 (1) PaveFiller analogue — make_shared_section_edges(A,B):       [intersection.cpp + brep.cpp, NEW]
       for each (fa,fb): surface_surface(fa,fb) -> {(c3d, pa, pb)}  (intersection.cpp:3359)
       create ONE m_curves_3d[c3d], paves (split vertices) on it,
       split-once into segments; per segment: ONE BRepEdge e,
       TWO BRepTrims (pa-seg on fa.edge=e, pb-seg on fb.edge=e).
       PostTreatFF: intersect section edges among themselves (Closest::curve_curve).
 (2) split_seam_pcurve for periodic faces (cyl/sphere/cone/torus) [nurbssurface_trimmed.cpp, NEW]
 (3) FillImagesFaces — split EACH face by its section pcurves:    [nurbssurface_trimmed.cpp]
       split_face_by_wires(face, section_pcurves)  (WireSplitter, replaces split_by_uv_curves:534)
       -> minimal directed loops; classify_wire_as_hole; nest_wires_into_faces.
       Every produced sub-face keeps the SHARED edge index from (1).
 (4) FillSameDomainFaces — unify_same_domain_faces:               [brep.cpp, NEW]
       group split faces by order-independent edge-set key (edge identity from (1));
       collapse coincident A/B faces to one representative.
 (5) classify + select per op — classify_face_state + table:      [brep.cpp, replaces :2354-2370]
       sA=state(fA in B), sB=state(fB in A) via contains_point + ON(same-domain);
       select fuse/cut/common; reversal emergent (IsSplitToReverse).
 (6) assemble_solid — shell growth with GetFaceOff dihedral +     [brep.cpp, NEW/robustness]
       hole nesting by volume sign. Replaces imprint_edges+sew_coincident_edges (:2424-2430).
 (7) verify: is_solid (:896), volume sign/magnitude, contains_point.
```

### 1.3 New/changed functions by file

| File | Function | Status | OCCT analogue |
|---|---|---|---|
| `intersection.cpp` | `axe_op` / `struct AxeRel` | **NEW** | `IntAna_QuadQuadGeo::AxeOperator` |
| `intersection.cpp` | `ssi_sphere_cylinder/_sphere_cone/_cylinder_cylinder/_cylinder_cone/_cylinder_torus/_cone_cone/_cone_torus/_sphere_torus/_torus_torus` | **NEW** | QQG per-pair `Perform` |
| `intersection.cpp` | extend `ssi_plane_cylinder` (`:2552`), `ssi_plane_torus`, `ssi_plane_cone` | **CHANGE** | QQG lines/parab/hyper branches |
| `intersection.cpp` | `analytic_periodic_pullback` (generalize `analytic_sphere_pullback` `:2811`) | **CHANGE** | `IntPatch_LineConstructor` seam split |
| `intersection.cpp` | `analytic_ssi` dispatch (`:2937`/`:2961`) | **CHANGE** | III `Perform` `iTT` switch |
| `brep.cpp` | `make_shared_section_edges` (paves, split-once, PostTreatFF) | **NEW** | `BOPAlgo_PaveFiller_6/7` |
| `brep.cpp` | `boolean` data flow (`:2272`) | **REWRITE** | `BOPAlgo_BOP::PerformInternal1` |
| `brep.cpp` | `unify_same_domain_faces`, `edge_set_key`, `are_faces_same_domain` | **NEW** | `BOPAlgo_Builder::FillSameDomainFaces` |
| `brep.cpp` | `classify_face_state` (extend `classify` lambda `:2354`), `select_faces_for_op` | **CHANGE** | `IsInternalFace`+`BuildBOP` table |
| `brep.cpp` | `assemble_solid` (`GetFaceOff`, hole nesting), retire `sew_coincident_edges` (`:1976`) for A∩B | **NEW** | `ShellSplitter`+`BuilderSolid` |
| `nurbssurface_trimmed.cpp` | `split_face_by_wires`, `Angle2D_NURBS`, `ClockWiseAngle`, `Path` | **NEW** (replaces `split_by_uv_curves` `:534`) | `BOPAlgo_WireSplitter` |
| `nurbssurface_trimmed.cpp` | `classify_wire_as_hole`, `contains_point_uv`, `nest_wires_into_faces` | **NEW** | `BuilderFace`/`FClass2d` |
| `nurbssurface_trimmed.cpp` | `split_seam_pcurve` | **NEW** | `AlgoTools3D::DoSplitSEAMOnFace` |
| `closest.cpp` | reuse `Closest::surface_point` (`closest.h:45`), `curve_point` (`:21`), `curve_curve` (`:30`) | **REUSE** | `IsValidPointForFace`, `IsVertexOnLine`, PostTreatFF |

Data structures (already present, `brep.h`): `BRepEdge.curve_3d_index` (`:31`), `BRepTrim.curve_2d_index`/`edge_index` (`:38-39`), `BRepFace.surface_index` (`:52`), pools `m_surfaces`/`m_curves_3d`/`m_curves_2d`/`m_topology_edges` (`:76-83`). **No new structs required** — the shared edge is just two trims with the same `edge_index`. This is the whole point.

---

## 2. SEQUENCED PHASES

Dependency rule: P0 is the architectural pivot; P1–P6 are the watertight/volume backbone (each rides on shared edges); P7–P9 are orthogonal SSI-exactness extensions that widen pair coverage. **Every phase ends by re-running the green regression set: box-box, box-cyl, sphere-cyl, box-sphere-volume — all 3 ops.**

---

### P0 — Shared section-edge backbone (PROOF-OF-ARCHITECTURE: box-sphere watertight)

**Goal.** Make the A∩B section a single shared edge; eliminate Hausdorff sew; yield watertight box-sphere with correct volume. This is the minimal phase (detailed in §3).

**Exact changes.**
- `brep.cpp::boolean` (`:2272`): replace the two independent `split_by_brep` calls (`:2284-2285`) and the `sew_coincident_edges` (`:2430`) with the shared-edge build.
- **NEW `make_shared_section_edges(A,B)`** (`brep.cpp`): for each candidate face-pair (fa,fb), call `Intersection::surface_surface(fa_surf, fb_surf)` (`intersection.cpp:3359`) → list of `(c3d, pa, pb)` triples (already produced via `analytic_ssi:2937` / `analytic_pcurve:2705` / `analytic_sphere_pullback:2811`). For each triple:
  - append `c3d` to `result.m_curves_3d` once → index `ce`;
  - split `c3d` at **seam paves only** (where `pb` crosses the periodic surface seam — reuse the seam-crossing logic already inside `analytic_sphere_pullback`, which returns multiple arcs) so the box side and sphere side share the same segment count and the same seam vertex;
  - per segment: one `BRepEdge{curve_3d_index=ce_seg}` → index `e`; one trim with `curve_2d_index=pa_seg, edge_index=e` registered into fa's loop; one trim `curve_2d_index=pb_seg, edge_index=e` into fb's loop.
- Feed `pa` segments to fa's `split_by_uv_curves` and `pb` segments to fb's; **post-assign** the produced boundary edges to the shared `e` by matching the pcurve segment (not by Hausdorff).
- Delete the A∩B branch of `sew_coincident_edges`; keep `imprint_edges` (`:1810`) only for intra-operand T-junctions.

**NURBS adaptation.** The triple `(c3d, pa, pb)` is exactly OCCT's `IntTools_Curve {Curve, FirstCurve2d, SecondCurve2d}`. For box-sphere the section is a circle (plane-sphere is exact, `analytic_ssi:2961` family); `pa` is a circle in the box-face UV (non-periodic, 1 segment), `pb` is the sphere longitude/latitude pullback (periodic — split at the meridian seam into arcs, `analytic_sphere_pullback:2811`). The seam vertex is the single shared pave.

**Oracle gate.** box-sphere {fuse, cut, common}: `is_solid()==true`; `checkprops -s area` matches OCCT; `volume` rel-error < 1e-9 (no longer 9%). Green set unchanged.

**Risk.** Medium. Segment correspondence between pa and pb if seam-split counts diverge → mismatched edge count. Mitigation: derive segment boundaries from a single pave list on `c3d`, then `.segment()` all three curves on identical parameters (true sub-curve extraction, parent knots preserved). Rollback: feature-flag `SESSION_BOOL_SHARED_EDGES`; fall back to the existing `split_by_brep`+`sew_coincident_edges` path.

---

### P1 — Pave engine: split-exactly-once + PostTreatFF (Section A, full)

**Goal.** Generalize P0's seam-only paves to arbitrary section topology (interior split vertices, bound paves, closing pave on closed curves, section-section crossings). Unlocks any pair whose section is multi-segment or where two section curves meet (3-surface corners).

**Exact changes (`brep.cpp::make_shared_section_edges`).** Port the OCCT pave model:
- `struct Pave{vid,t}`, `struct PaveBlock{orig,edge,p1,p2,ext[]}`; `master` pave block per section curve.
- `PutPavesOnCurve`: project existing On/In vertices onto `c3d` via `Closest::curve_point` (`closest.h:21`); dedup parametrically with `contains_param(t, c3d.resolution(tol))`.
- `PutBoundPaveOnCurve`: make endpoint vertices when free and valid-for-faces.
- `PutClosingPaveOnCurve`: detect `c3d.point_at(t0)≈point_at(t1)`; append a second pave reusing the same vid at the opposite bound (splits a closed circle into a closeable edge).
- `update_paveblock`: `std::sort` ext+bounds by parameter, emit consecutive `[ti,ti+1]` segments → one shared edge each (P0's MakeEdge+two-pcurve core).
- `PostTreatFF`: run `Closest::curve_curve` (`closest.h:30`) over all section edges; split where they cross; fuse coincident vids.

**NURBS adaptation.** `segment` on `c3d`/`pa`/`pb` over identical `[ti,ti+1]` (parent knot parametrisation preserved → OCCT same-parameter guarantee, no refit). Pave dedup tolerance = `curve.resolution(tol3d)`; vertex dedup is 3D tolerance-ball.

**Oracle gate.** box-cylinder {fuse,cut,common} re-verified (multi-segment lateral-cap sections); box-cone {cut} where the plane produces 2 arcs. is_solid + area + volume vs OCCT.

**Risk.** Medium. Parametric dedup tolerance mis-set → duplicate or dropped paves. Mitigation: assert monotone sorted params, no zero-length segments (`|t1-t2|<pconf` skip). Rollback: disable PostTreatFF + closing-pave; P0 path still serves box-sphere.

---

### P2 — Seam pcurve duplication for periodic faces (Section C-c)

**Goal.** A section/boundary edge lying on a cylinder/sphere/cone/torus seam must carry TWO pcurves (one per side) so the seam face closes. Prerequisite for wire-splitting on closed surfaces.

**Exact changes.** **NEW `split_seam_pcurve(trim, face, surface, tol)`** in `nurbssurface_trimmed.cpp`, called from the face-split driver (P3) only when `original_edge.is_closed_on(face)`:
- periods `pu=is_u_closed?(umax-umin):0`, `pv` analog;
- eval pcurve at mid-param → point P, tangent T; detect seam via `|P.u-umin|<u_resolution(tol)` etc.;
- clone pcurve, translate the partner copy by `±period` in u/v (exact: add period to every control point's u/v — no refit);
- order (fwd,rev) by `dot(T, axis)` with the `is_left` four-way branch; `trim.set_two_pcurves`.

**NURBS adaptation.** `u_resolution(tol)=tol/max‖∂S/∂u‖` over the edge (matches `GeomAdaptor::UResolution`); never a raw UV epsilon (mis-fires at sphere poles). Sphere poles are degenerate (skipped); only the meridian seam gets two pcurves.

**Oracle gate.** sphere-cyl re-verified; box-sphere seam-straddling cap (task #15) hardened. Sets up cyl-cyl seam handling.

**Risk.** Medium-high. Wrong fwd/rev ordering flips the seam face → solid leaks. Mitigation: port the four-way branch verbatim; unit-test the cylinder seam in isolation (`v=const` line, both sides). Rollback: skip for non-periodic faces (box family unaffected).

---

### P3 — WireSplitter UV arrangement (Section B) — replaces `split_by_uv_curves`

**Goal.** Stop over-fragmenting (current `split_by_uv_curves:534` emits every arrangement cell). Emit only minimal directed loops; each directed edge consumed exactly once; section pcurve fed FORWARD+REVERSED so it bounds a region on each side. This is the watertightness mechanism inside one face.

**Exact changes (`nurbssurface_trimmed.cpp`).** **NEW `split_face_by_wires(face, section_pcurves, tol3d)`**:
- assemble directed edges: boundary trims once (orientation honored); each section pcurve TWICE (fwd + reversed) sharing one section-edge id; `is_section` flag set explicitly.
- `SmartMap: vertex_id → list<EdgeInfo{edge,inFlag,isInside=is_section,passed,angle}>`; vertex snap by `tol2d = max(tol3d/uv_to_3d, snap_floor)` reusing `uv_to_3d` (`:557-559`).
- fast path: every vertex 1-in/1-out and no duplicated edge → single `MakeWire`.
- **`Angle2D_NURBS`**: curvature-bounded step `dt` from analytic pcurve derivative (`NurbsCurve::evaluate(t,1)`), `dt=max(tol2d/speed, PConf)`, bump by `acos(R/(R+tol2d))` on curved pcurves; angle = `atan2` of travel direction in `[0,2π)`.
- **`Path` walk** with `ClockWiseAngle(angleIn,angleOut)` leftmost rule (min `dA`) and the **`nWaysInside==1` section-follow override**; closure-pop-and-resume stack; seam guards (2D coincidence + `|Δu|,|Δv|`) on periodic surfaces.

**NURBS adaptation.** Use cv-based `evaluate(t,1)` for tangent, not finite-difference `tangent_at`. `Coord2d` = pcurve endpoint at the vertex param honoring `reversed`. Optional `RefineAngles` (tangent/convergent nodes) deferred to P9 if cyl-cyl misorders.

**Oracle gate.** box-box, box-cyl, box-sphere, sphere-cyl ALL re-verified (P3 must be ≥ as good as the old splitter on green cases) + box-cone {fuse,cut,common} now correct (multi-region faces). is_solid + area.

**Risk.** High — this replaces a load-bearing, currently-green function. Mitigation: keep `split_by_uv_curves` intact behind a flag; A/B both splitters on the green set and assert identical face count/area before switching default. Rollback: flag flip restores `split_by_uv_curves`.

---

### P4 — Region nesting: hole classification + point-in-UV + outer/hole pairing (Section C-a/a'/b)

**Goal.** Turn the wires from P3 into trimmed faces with correct inner loops; classify outer vs hole; nest holes into growths.

**Exact changes (`nurbssurface_trimmed.cpp`).**
- **`classify_wire_as_hole`**: signed shoelace area of the UV polygon (sample pcurves in wire order, drop 3D-coincident consecutive points, FORWARD-face frame); `area>0`→outer, `<0`→hole, `|area|<sq_conf`→Bad→exact classifier. `IsGrowthWire` fast check: wire shares an edge with a known hole ⇒ outer.
- **`contains_point_uv`**: even-odd ray cast (`si_dans`) combined with per-wire orientation; periodic recadrage (retry at `u±period`,`v±period`) for closed surfaces.
- **`nest_wires_into_faces`**: BVH over hole UV-boxes; `is_inside(hole, outer)` via pcurve-midpoint classify; tightest-containment for nested holes.

**NURBS adaptation.** Pin the area-sign↔normal convention globally (build all wires in FORWARD frame, apply face orientation last); if NurbsSurface normal convention differs, negate the test once globally. Reuse `Closest::surface_point` only for the `Bad` exact fallback.

**Oracle gate.** box-cylinder where a cap face gets a hole (annulus); box-torus precursor. is_solid + area + volume.

**Risk.** Medium. Area-sign convention wrong → outer/hole swapped → inverted face. Mitigation: single global convention test on a known annulus; periodic recadrage unit-tested on sphere cap. Rollback: independent of P3 walk; can default holes off (single-loop faces still correct).

---

### P5 — Same-domain face unification (Section C-d)

**Goal.** Collapse coincident split faces from the two operands (e.g. tangent/coplanar contact) into one representative, by **edge identity** (only possible because P0 made the section edge a single object). This is the missing step that flips `is_solid` AND fixes volume for contact configs.

**Exact changes (`brep.cpp`).**
- **`edge_set_key(f)`**: order-independent multiset of edge indices (degenerate skipped, INTERNAL doubled); hash = sum of per-edge hashes; exact set equality.
- **`unify_same_domain_faces`**: group by key; within a group, planar-bounded+equal-key ⇒ SD immediately; else `are_faces_same_domain` (point strictly inside fA via hatcher → `Closest::surface_point` projects onto fB → `dist ≤ tolA+tolB+fuzz+maxEdgeTol` AND `contains_point_uv != OUT`); union-find → representative (min index); re-point images, flip via `IsSplitToReverse` if needed.

**NURBS adaptation.** `point_strictly_inside` = hatcher-style interior UV sample (reuse the `face_sample` largest-triangle centroid already at `brep.cpp:2299-2350`), nudged > tol off every pcurve. Tolerance is the sum formula, never a hard epsilon.

**Oracle gate.** Any pair with coincident/tangent contact faces (e.g. box-box face-flush fuse, cylinder-cylinder coaxial Same). is_solid + 2-faces-per-edge manifold check + volume.

**Risk.** Low-medium. Over-merging distinct near-coincident faces. Mitigation: edge-identity key (not geometry hash) makes false merges impossible unless P0 wrongly shared an edge. Rollback: no-op when no group has ≥2 members (box-sphere, box-cyl unaffected).

---

### P6 — Shell assembly with `GetFaceOff` dihedral + hole nesting (Section D-c/d) [robustness]

**Goal.** Correct assembly when a section edge bounds >2 faces in the intermediate decomposition (T-junctions / non-manifold), and global orientation via dihedral walk + hole-by-volume-sign. Gated behind "non-regular connexity block detected" so green convex cases stay on the fast path.

**Exact changes (`brep.cpp`).** **NEW `assemble_solid(faces)`** replacing the tail `imprint_edges`+`sew_coincident_edges` (`:2424-2430`):
- connexity blocks by shared-edge identity; regular block (every edge exactly 2 faces) → direct shell + `OrientFacesOnShell`.
- non-regular → `SplitBlock`: drop dangling faces (edge with 1 face) to fixpoint; grow shell choosing neighbor across each free edge by `GetEdgeOff` (opposite orientation) then `GetFaceOff` min-dihedral when >2 candidates.
- `GetFaceOff`: `Px,Tgt` from `c3d.D1(t)`; `face_normal_on_edge` via trim pcurve → `unit(Su×Sv)` flipped by `face.reversed XOR trim.reversed`; binormal `N×Tgt` refined a step into the face; `AngleWithRef = atan2(dot(cross(B1,B2),REF), dot(B1,B2))`, fold to `[0,2π)`, pick MIN.
- hole nesting: `IsHole` = **sign of `BRep::volume()`** (`:896`-region); `IsInside` = `contains_point` (`:941`); innermost by mutual `contains_point`.

**NURBS adaptation.** All vector algebra on `NurbsSurface`/`NurbsCurve` evals; `refine_into_face` step `dt≈2(tolE+tolF)` bumped to ~5e-4 for spheres/freeform.

**Oracle gate.** cyl-cyl perpendicular-equal (2-ellipse section creating a 4-face edge), cone-cone shared-apex. is_solid + volume + manifold edge degree.

**Risk.** High (dihedral sign errors flip shells). Mitigation: regular-block fast path keeps ALL green cases off this code; enable only when degree>2 detected; unit-test `GetFaceOff` on a hand-built 3-face edge. Rollback: regular-only path = current behavior for convex booleans.

---

### P7 — SSI degeneracies group 1: plane/quadric exact branches + `AxeOp` (Section E §1, J, K, plane-cone)

**Goal.** Exact section curves (lines/parabola/hyperbola/extra circles) for plane-vs-quadric, so box-cone/box-torus sections are exact (shared-edge identity holds without marcher fitting).

**Exact changes (`intersection.cpp`).** Add `axe_op`/`AxeRel` (coaxiality primitive, exact OCCT tolerances: `EPS_DIST=1e-14`, `EPS_PARA=1e-12`, `EPS_MINI_CIRCLE=1e-9`, cyl-cyl `ΔR/Rmax≤1e-13`). Extend `ssi_plane_cylinder` (`:2552`) with axis-∥ 1/2-line branch; `ssi_plane_torus` with plane-contains-axis 2-minor-circle branch; `ssi_plane_cone` with parabola/hyperbola/apex-line branches. Generalize `analytic_sphere_pullback` → `analytic_periodic_pullback` for cylinder/cone/torus circle outputs (seam-split arcs). Port `RefineDir` axis snap before parallel tests.

**Oracle gate.** box-cone {fuse,cut,common} (oblique plane → hyperbola/parabola), box-torus {fuse,cut,common} (concentric + minor circles). is_solid + area + volume; tri-state contract preserved (`handled=true,empty`⇒recognised-no-curve).

**Risk.** Low-medium (pure curve generation; backbone already watertight). Mitigation: `handled=false` falls to marcher, so a missing branch degrades gracefully. Rollback: revert dispatch entries; marcher resumes.

---

### P8 — SSI degeneracies group 2: sphere/cyl/cone coaxial pairs (Section E A,B,C,D)

**Goal.** Exact closed-form for cylinder-sphere, cone-sphere, cylinder-cone, cylinder-cylinder (parallel-lines, perp-equal-ellipses, coaxial-Same).

**Exact changes (`intersection.cpp`).** Add `ssi_sphere_cylinder` (coaxial 1/2 circles), `ssi_sphere_cone` (axial quadratic → 0/1/2 circles), `ssi_cylinder_cone` (coaxial 2 circles), `ssi_cylinder_cylinder` (∥→0/1/2 lines, ⟂-equal→2 ellipses, coaxial→Same, else `false`→marcher). Wire into `analytic_ssi` dispatch (`:2961`).

**Oracle gate.** cylinder-sphere re-verified exact; cone-sphere, cylinder-cone, cylinder-cylinder (coaxial + perp-equal) {all 3 ops}. General skew cyl-cyl still rides marcher + P6 assembly.

**Risk.** Low-medium. Wrong tangent (1 vs 2 curve) via discriminant. Mitigation: discriminant `≤tol²`⇒single circle; radius `≤1e-9`⇒point. Rollback: per-routine `false`→marcher.

---

### P9 — SSI degeneracies group 3: torus family + cone-cone + RefineAngles (Section E E,F,G,H,I + B§4)

**Goal.** Close the remaining pairs: cylinder-torus, cone-torus, sphere-torus, torus-torus, cone-cone (coaxial+apex). Add `RefineAngles` if cyl-cyl/tangent nodes misorder.

**Exact changes.** `intersection.cpp`: `ssi_cylinder_torus`, `ssi_cone_torus` (up to 4 circles), `ssi_sphere_torus`, `ssi_torus_torus`, `ssi_cone_cone` (same-axis circles + coincident-apex lines), all gated on ring-torus `RMin<RMaj` + coaxiality via `axe_op`. `nurbssurface_trimmed.cpp`: `RefineAngles`/`RefineAngle2D` (2D NURBS∩line via polyline + Newton) for tangent section curves at boundary nodes.

**Oracle gate.** cylinder-torus, cone-torus, sphere-torus, torus-torus, cone-cone {all 3 ops}. **Full 15×3 matrix green**: is_solid==true everywhere, area + volume within 1e-9 rel of OCCT.

**Risk.** Medium. 4-circle torus configs + Villarceau (left to marcher). Mitigation: `RefineAngles` is second-order (skipped when `dA<delta`); add only if a dataset misorders. Rollback: per-routine `false`→marcher; RefineAngles is additive.

---

## 3. MINIMAL FIRST PHASE (P0) — watertight box-sphere without corrupting volume

This is the proof that the architecture works; it touches the fewest functions and keeps every green case.

**Scope.** Only the shared-edge construction + removal of Hausdorff sew. No WireSplitter, no same-domain, no SSI additions (plane-sphere is already exact, `analytic_ssi:2961`).

**Steps.**
1. In `brep.cpp::boolean` (`:2272`), behind flag `SESSION_BOOL_SHARED_EDGES`, replace independent `split_by_brep` (`:2284-2285`) with one pass:
   - for each box-face fb (plane) that the sphere fa intersects, call `surface_surface(fa,fb)` (`:3359`) → `(c3d_circle, pa_sphere_uv, pb_plane_uv)`.
   - append `c3d_circle` to `m_curves_3d` ONCE (index `ce`).
   - `pa` is the sphere-side pullback — periodic; split at the meridian seam (existing logic in `analytic_sphere_pullback:2811`) into K arcs. Determine the K split parameters on `c3d` (the seam-crossing params) → K shared paves.
   - `.segment(c3d, pa, pb)` on the SAME K+1 parameter spans (parent knots preserved). For each span s: one `BRepEdge{curve_3d_index = ce_segment_s}` → index `e_s`; trim `{curve_2d_index=pa_s, edge_index=e_s}` into the sphere face's loop; trim `{curve_2d_index=pb_s, edge_index=e_s}` into the box face's loop. (Box side pb is non-periodic → its spans just inherit the same vertices.)
2. Split the sphere face by `pa` arcs and each box face by its `pb` circle using existing `split_by_uv_curves` (`:534`) — UNCHANGED — but **post-assign** each produced section-boundary edge to the precomputed shared `e_s` by matching its pcurve span (identity, not Hausdorff).
3. Classify (`:2354-2370`) and subset/combine (`:2375-2418`) UNCHANGED — `subset` already preserves shared edge indices.
4. **Skip `sew_coincident_edges` (`:2430`)** entirely for A∩B; keep `imprint_edges` (`:2424`) for intra-box T-junctions only.
5. Verify: `is_solid()` (`:896`) == true; `volume` rel-error vs OCCT < 1e-9; `checkprops -s area` match.

**Why it works.** The circle is one `m_curves_3d` entry; the sphere cap and the box hole reference the identical `BRepEdge` per arc with a shared seam vertex. There are no two near-coincident curves to reconcile, so the 9% corruption (a Hausdorff artifact) cannot occur and the result is watertight by construction.

**Rollback.** Flag off ⇒ exact current path (independent split + Hausdorff sew). Green set proven before flipping the default.

---

## 4. RISK / ROLLBACK SUMMARY

| Phase | Primary risk | Rollback lever |
|---|---|---|
| P0 | pa/pb seam-segment count mismatch | `SESSION_BOOL_SHARED_EDGES` flag → old split+sew |
| P1 | parametric pave dedup tol → dup/dropped split | disable closing-pave + PostTreatFF; P0 path serves box-sphere |
| P2 | seam fwd/rev order flips face → leak | skip for non-periodic faces (box family untouched) |
| P3 | replaces green `split_by_uv_curves` | keep old splitter behind flag; A/B assert equal area on green set |
| P4 | area-sign convention swaps outer/hole | global convention test; default holes off → single-loop faces still valid |
| P5 | over-merge distinct faces | edge-IDENTITY key (impossible unless P0 wrong); no-op when no ≥2 group |
| P6 | dihedral sign flips shells | regular-block fast path = current behavior; enable only on edge-degree>2 |
| P7 | missing analytic branch | `handled=false` → marcher (graceful) |
| P8 | tangent 1-vs-2 curve discriminant | discriminant/radius guards; per-routine `false`→marcher |
| P9 | 4-circle torus / Villarceau | per-routine `false`→marcher; RefineAngles additive/skippable |

**Global invariant enforced at every gate:** the four green cases (box-box, box-cyl, sphere-cyl, box-sphere-volume) must pass `is_solid` + area + volume before a phase's default flag is flipped. Each phase is independently flag-gated so any regression reverts to the prior watertight state without losing earlier phases.

**Key files (absolute):** `C:/pc/3_code/code_rust/session/session_cpp/src/brep.cpp` (boolean `:2272`, classify `:2354`, combine `:2375`, sew `:1976`, is_solid `:896`, contains_point `:941`, face_sample `:2299`), `.../intersection.cpp` (surface_surface `:3359`, analytic_ssi `:2937`/dispatch `:2961`, analytic_pcurve `:2705`, analytic_sphere_pullback `:2811`, RecogSurface `:2228`, ssi_plane_cylinder `:2552`), `.../nurbssurface_trimmed.cpp` (split_by_uv_curves `:534`, uv_to_3d `:557-559`), `.../closest.h` (surface_point `:45`, curve_point `:21`, curve_curve `:30`, surface_curve `:54`), `.../brep.h` (BRepEdge `:30`, BRepTrim `:37`, BRepFace `:51`, pools `:76-83`).