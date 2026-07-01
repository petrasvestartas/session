I have everything needed. Here is the implementation-depth analysis.

---

# OCCT BuilderFace region-building, seam handling, and same-domain unification — NURBS port

All paths below are under
`C:/pc/3_code/code_rust/session/validation/occt_oracle/build/deps/occt/src/occt/src/`
abbreviated as `…/`. Key files:
- `…/ModelingAlgorithms/TKBO/BOPAlgo/BOPAlgo_BuilderFace.cxx`
- `…/ModelingAlgorithms/TKBO/BOPAlgo/BOPAlgo_Builder_2.cxx`
- `…/ModelingAlgorithms/TKBO/BOPTools/BOPTools_AlgoTools3D.cxx`
- `…/ModelingAlgorithms/TKBO/IntTools/IntTools_FClass2d.cxx`
- `…/FoundationClasses/TKMath/CSLib/CSLib_Class2d.cxx`
- `…/FoundationClasses/TKMath/Poly/Poly.hxx`
- `…/ModelingAlgorithms/TKBO/BOPTools/BOPTools_Set.cxx`
- `…/ModelingAlgorithms/TKBO/BOPTools/BOPTools_AlgoTools.cxx`
- `…/ModelingAlgorithms/TKBO/BOPAlgo/BOPAlgo_Tools.hxx`
- `…/ModelingAlgorithms/TKBO/IntTools/IntTools_Context.cxx`

---

## 0. THE ARCHITECTURAL INVARIANT (why this fixes the 9% volume bug)

The session does `imprint each operand independently → classify → select → sew (Hausdorff)`. OCCT never sews coincident-but-distinct edges. Instead:

1. The section curve A∩B is computed **once** by the PaveFiller and stored as a single edge in the DS (one `TShape`). Both operand faces reference the **same** edge object.
2. `BuildSplitFaces` (`BOPAlgo_Builder_2.cxx:231`) iterates over **every** source face and, into that one face's edge pool, appends: its bounding-edge splits, its IN edges, and its **section (Sc) edges** — each section edge twice, FORWARD and REVERSED (`BOPAlgo_Builder_2.cxx:478-489`):

```cpp
// 1.3 Section edges  (Builder_2.cxx:478-489)
for (j=1; j<=aNbPBSc; ++j) {
  nSp = aMPBSc(j)->Edge();
  aSp = myDS->Shape(nSp);          // <-- SAME edge index for BOTH faces of the FF pair
  aSp.Orientation(TopAbs_FORWARD);  aLE.Append(aSp);
  aSp.Orientation(TopAbs_REVERSED); aLE.Append(aSp);
}
```

So when face F_A (operand 1) and face F_B (operand 2) are each split, the new faces on either side of the section reference an **identical** edge (same vertices, same 3D curve). No two coincident-but-different edges are ever created, so there is nothing to Hausdorff-match.

3. `FillSameDomainFaces` (`BOPAlgo_Builder_2.cxx:571`) then collapses the *coincident split faces* coming from the two operands into one representative.

The result is watertight **by construction**: every section edge is shared by exactly the faces that meet there. The session's post-hoc co-splitting corrupts the region because the two operands' UV arrangements are computed independently and disagree on the interior topology. Adopt the model above: build a single per-face edge pool that includes the shared section edge (both orientations), run a wire-splitter, classify regions, then unify same-domain faces.

`FillImagesFaces` order (`Builder_2.cxx:213-227`): `BuildSplitFaces → FillSameDomainFaces → FillInternalVertices`.

---

## (a) `classify_wire_as_hole(wire_uv)` — signed area in NURBS UV

OCCT path: `IntTools_FClass2d::Init` (`IntTools_FClass2d.cxx:76-582`) builds a UV polygon per wire and signs its area; `IsHole()` (`:69`) returns `myIsHole`. The fast pre-check `IsGrowthWire` (`BuilderFace.cxx:862-874`) short-circuits.

### What OCCT actually does
- Face is forced FORWARD before sampling (`FClass2d.cxx:104`). Classification is **always** in the FORWARD-face frame; the overall face orientation is re-applied later (`Builder_2.cxx:541-542`).
- For each edge of the wire, in **wire order** respecting the edge's FORWARD/REVERSED orientation (`:235-247` flips the sampling direction and `du` sign for REVERSED), sample the pcurve at `nbs` parameters (`nbs = Geom2dInt::NbSamples(C)`, ×4 if curved, `:228-232`).
- Append each sampled UV point to `SeqPnt2d`, skipping points whose **3D** image coincides with the previous (`:297-334`) — this drops degenerate/seam contributions. Track `Umin/Umax/Vmin/Vmax` (`:287-294`).
- Signed area via `Poly::PolygonProperties` (`FClass2d.cxx:433`; def `Poly.hxx:168-199`):

```cpp
// Poly.hxx:186-197  (shoelace via cross product against a reference point)
aRefPnt = P[lower];
aPrevPt = P[lower+1] - aRefPnt;
area = 0;
for (i = lower+2 .. upper) {
    aCurrPt = P[i] - aRefPnt;
    area   += aPrevPt.Crossed(aCurrPt);   // 2D cross = px*cy - py*cx
    aPrevPt = aCurrPt;
}
area *= 0.5;
```

- Decision (`FClass2d.cxx:512-528`):
  - `|area| < SquareConfusion` → **bad wire**, `TabOrien=-1`, force use of the exact face classifier later.
  - `area > 0` → `myIsHole = false`, `TabOrien=1` (outer / growth, CCW).
  - `area < 0` → `myIsHole = true`, `TabOrien=0` (hole, CW).

Note the convergence loop `:439-501`: if the polygon's deflection (`max(FlecheU,FlecheV)`) exceeds the area/perimeter "expected thickness", it re-discretizes more finely (`GCPnts_QuasiUniformDeflection`) to avoid a self-intersecting polygon flipping the area sign. Replicate this guard for thin slivers.

### NURBS pseudo-code

```text
fn classify_wire_as_hole(wire_uv, surface) -> Classification {   // {Outer, Hole, Bad}
    P = []                                    // UV polygon
    prev3d = None
    for trim in wire_uv.trims_in_wire_order() {
        pc = trim.pcurve                      // NurbsCurve in (u,v)
        if trim.is_degenerate() || trim.is_seam(surface) { continue }
        (t0,t1) = pc.domain()
        n = max(2, nb_samples(pc) * (pc.is_curved() ? 4 : 1))
        params = linspace(t0, t1, n)
        if trim.orientation == REVERSED { params.reverse() }   // FClass2d.cxx:235-247
        for t in params {
            uv  = pc.point_at(t)
            p3d = surface.point_at(uv.u, uv.v)
            if prev3d.is_some() && p3d.dist(prev3d) < CONFUSION { continue } // :297-334
            P.push(uv); prev3d = Some(p3d)
        }
    }
    if P.len() < 4 { return Bad }                              // :414, :531
    area = signed_area_shoelace(P)                            // Poly.hxx:186-197
    if area.abs() < SQUARE_CONFUSION { return Bad }           // :512
    return area > 0 ? Outer : Hole                            // :519-528 (CCW outer, CW hole)
}

fn signed_area_shoelace(P) -> f64 {
    r = P[0]; prev = P[1]-r; a = 0.0;
    for i in 2..P.len() { cur = P[i]-r; a += prev.x*cur.y - prev.y*cur.x; prev = cur; }
    0.5 * a
}
```

NURBS adaptation notes:
- **Direction convention** is the load-bearing detail. OCCT's "positive area = outer" assumes the FORWARD-face natural normal. Pin your convention: build all wires in the FORWARD frame, classify, then apply the face's global orientation to the final faces (mirror of `Builder_2.cxx:541-542`). If your NurbsSurface normal convention differs, negate the area test once, globally — never per-wire.
- **Fast growth check** (`BuilderFace.cxx:438`, `IsGrowthWire :862`): before running the area test, if the wire shares **any** edge with a wire already classified as a hole, it is automatically a growth. Keep a running set `MHE` of hole-wire edge IDs; `wire ∩ MHE ≠ ∅ ⇒ Outer`. This is both a speedup and a robustness fallback for ambiguous areas.
- **`Bad` handling**: when area is ~0 (degenerate sliver, self-touching pcurve), do not guess. Fall back to the exact even-odd classifier (next section's `contains_point_uv`) with the surface-resolution tolerance, exactly as `FClass2d::Perform` does when `TabOrien(1)==-1` (`FClass2d.cxx:646,688-716`).

---

## (a′) point-in-region test (`contains_point_uv`) — needed by (b) and (d)

`IntTools_FClass2d::Perform` (`:598-760`) classifies a UV point against the **whole face** (all wires), combining per-wire even-odd results with `TabOrien`. The per-wire primitive is `CSLib_Class2d::SiDans` → `InternalSiDansOuOn` (`CSLib_Class2d.cxx:201-323`): a normalized even-odd ray cast.

```text
fn contains_point_uv(uv, face) -> State {     // IN / OUT / ON
    // face holds per-wire polygons with orientation tag t in {+1 outer, 0 hole, -1 bad}
    if any wire is Bad { return exact_classifier(uv, face) }   // FClass2d.cxx:646,688
    inside = +1
    for (poly, t) in face.wire_polygons {
        c = si_dans(poly, uv)        // +1 inside poly, -1 outside, 0 on boundary
        if c == 0 { return exact_classifier(uv, face) }        // :673-680
        if (c==+1 && t==0) || (c==-1 && t==+1) { inside = -1; break } // :655-670
    }
    inside==+1 ? IN : OUT
}

fn si_dans(poly, q) -> i32 {          // even-odd, CSLib_Class2d.cxx:201-241
    // poly is normalized to [0,1]^2 box first (Transform2d); do the same or skip if you
    // work in raw UV — only matters for periodic surfaces (see below)
    nbc = 0; prev = poly[0]-q; sh = sign(prev.y);
    for i in 1..=poly.len() {         // closed: poly[len]==poly[0]
        cur = poly[i % poly.len()] - q;
        nh = sign(cur.y);
        if nh != sh {
            if prev.x>0 && cur.x>0 { nbc+=1 }
            else if prev.x>0 || cur.x>0 {
                if prev.x - prev.y*(cur.x-prev.x)/(cur.y-prev.y) > 0 { nbc+=1 }
            }
            sh = nh;
        }
        prev = cur;
    }
    (nbc & 1)==1 ? +1 : -1
}
```

NURBS adaptation notes:
- **Periodic recadrage**: for closed NurbsSurfaces (cyl/sphere), if the first classification yields OUT, OCCT shifts the query point by ±period in U and/or V and reclassifies (`FClass2d.cxx:718-758`). Port this: when `surface.is_u_closed`, retry with `u±period_u`; same for V. This is what makes seam-straddling caps classify correctly (task #15).
- The `Transform2d` normalization to `[0,1]²` (`CSLib_Class2d.cxx:67-68, 328-341`) is only a conditioning trick; you can skip it if you tolerance correctly, but keep it if you reuse OCCT's tolerance `Tolu/=du`.

---

## (b) `nest_wires_into_faces(wires)` — outer + holes via BVH + midpoint test

OCCT `BOPAlgo_BuilderFace::PerformAreas` (`BuilderFace.cxx:386-604`).

### Algorithm
1. Get surface/loc/tol once (`:391-395`). If no loops and face is infinite, emit one natural-restriction face (`:400-411`).
2. **Classify** each wire (one closed wire = one candidate face) into `aNewFaces` (growths) and `aHoleFaces` (holes), using `IsGrowthWire` fast check then `FClass2d.IsHole()` (`:438-455`). For each hole, register its edges into `aMHE` so subsequent wires sharing them are forced growth.
3. If no holes, done (`:458-463`).
4. Build a **2D AABB BVH** over hole faces (`BOPTools_Box2dTree`, `:467-481`): `BRepTools::AddUVBounds` per hole, `Build()`.
5. For each growth face, query the BVH with the growth's UV box (`:490-507`) to get candidate holes; for each candidate run the **exact** `IsInside(hole, growth)` test (`:515`). If the hole is already assigned to another outer, keep the **tightest** outer — if the new outer is itself inside the previous one, replace (`:519-530`).
6. Invert to `face → [holes]` (`:535-547`); attach any unused holes to a natural-restriction face if the face box is open (`:550-571`).
7. Add hole wires as inner loops, **re-init the classifier** for the now-complete face (`:597-598`), append to `myAreas` (`:574-603`).

`IsInside` (`BuilderFace.cxx:812-858`): take the first non-degenerate wire edge **not** shared by the face; evaluate its pcurve at the **midpoint** `(t1+t2)/2`; classify with `FClass2d.Perform(P2D)==IN`.

### NURBS pseudo-code

```text
fn nest_wires_into_faces(wires_uv, surface, tol) -> Vec<Face> {
    outers = []; holes = []
    mhe = HashSet::new()                     // edges of known holes
    for w in wires_uv {
        let cls = if !(w.edges() ∩ mhe).empty() { Outer }      // IsGrowthWire
                  else { classify_wire_as_hole(w, surface) }    // (a)
        match cls {
            Outer|Bad => outers.push(make_face(surface, w, tol)),
            Hole      => { holes.push(make_face(surface, w, tol)); mhe.extend(w.edges()); }
        }
    }
    if holes.empty() { return outers }

    // BVH over hole UV-boxes (BuilderFace.cxx:467-481)
    bvh = Bvh2d::build(holes.map(|h| uv_bbox(h)))
    assign : Map<Hole, Outer> = {}
    for o in &outers {
        for k in bvh.query(uv_bbox(o)) {                 // candidate holes
            h = holes[k]
            if !is_inside(h.wire, o, surface) { continue }   // exact midpoint test
            match assign.get(h) {
                Some(prev) if is_inside(o.wire, prev, surface) => assign[h]=o, // tighter
                None => assign[h]=o,
                _ => {}
            }
        }
    }
    // attach holes, rebuild faces
    for o in &mut outers {
        for (h,f) in &assign { if f==o { o.add_inner_loop(h.wire) } }
        o.classifier = rebuild_classifier(o, surface, tol)    // :597-598
    }
    // unused holes -> natural-restriction face if face is open/periodic (:550-571)
    leftover = holes.filter(|h| !assign.contains(h))
    if !leftover.empty() && uv_bbox_open(surface) {
        let f = natural_restriction_face(surface, tol);
        for h in leftover { f.add_inner_loop(h.wire) }
        outers.push(f)
    }
    outers
}

fn is_inside(wire, face, surface) -> bool {            // BuilderFace.cxx:812-858
    for e in wire.edges() {
        if e.is_degenerate() { continue }
        if face.contains_edge(e) { return false }      // shared edge => not strictly inside
        pc = e.pcurve_on(face); (t1,t2)=pc.domain()
        return contains_point_uv(pc.point_at((t1+t2)/2), face) == IN
    }
    false
}
```

NURBS adaptation notes:
- The **midpoint** of the pcurve is used, not an endpoint — endpoints are shared vertices and classify ON. Keep that.
- "Tightest containment" (`:519-530`) handles nested holes-in-islands. If multiple outers contain a hole, pick the one of **minimum area** (equivalently: the outer whose own interior point is inside the other). Computing `signed_area` from (a) gives you the area for free; you can replace the double `is_inside` with an area comparison if your `is_inside` is expensive.
- Your existing `split_by_uv_curves` over-fragments. Replace it: feed the single face's edge pool (bounding splits + section edge both orientations) into a **wire-splitter** (minimal-left-turn traversal at each vertex, choosing the next half-edge by smallest signed angle in UV) to assemble closed wires, then call `nest_wires_into_faces`. Do **not** build a full planar arrangement — that creates spurious faces.

---

## (c) `split_seam_pcurve(edge, face)` — the two-pcurve rule for periodic surfaces

OCCT `BOPTools_AlgoTools3D::DoSplitSEAMOnFace` (`AlgoTools3D.cxx:57-225`). Trigger site: `BuildSplitFaces` (`Builder_2.cxx:424-450`) — when the original edge is closed on the face (a seam) and its split is not yet closed, add the second pcurve.

### What it does
1. Determine periodicity & period (`:84-140`): `UPeriod = umax-umin` if `IsUClosed`, etc. (Also unwraps `Geom_RectangularTrimmedSurface` to its basis surface — for NURBS, just read the surface's closed-ness flags.)
2. Eval the existing pcurve at the intermediate parameter: `C2D1->D1(aT, aP2D, aVec2D)` → point `(u,v)`, tangent (`:143-147`).
3. Decide which seam the curve is on (`:159-185`), using surface **resolution** as tolerance:
   - `|u - umin| < UResolution(tol)` → left seam, partner pcurve at `u + UPeriod` (`bIsLeft=true`).
   - `|u - umax| < UResolution(tol)` → right seam, partner at `u - UPeriod`.
   - analog for V with `VResolution(tol)`.
4. If neither → not a seam, return false (`:187-190`).
5. Make two trimmed copies of the pcurve; translate the second by `(Δu,Δv)` = the period offset (`:194-200`).
6. Choose ordering via `aScPr = tangent · axis` (axis = `(-1,0)` for U-seam, `(0,1)` for V-seam) and `bIsLeft`, then `UpdateEdge(edge, first, second, face, tol)` storing **both** pcurves on the edge (`:202-223`). The edge is now "closed on face": its FORWARD coedge reads one pcurve, its REVERSED coedge the other.

The 3-arg overload (`:229-310`) is the fallback: it projects the split's midpoint onto the **original** seam edge's two pcurves to recover the exact partner location and tangent sign when the simple period offset is ambiguous.

### NURBS pseudo-code

```text
fn split_seam_pcurve(trim, face, surface, tol) -> bool {
    // periodicity
    (umin,umax,vmin,vmax) = surface.domain()
    pu = surface.is_u_closed() ? (umax-umin) : 0.0
    pv = surface.is_v_closed() ? (vmax-vmin) : 0.0
    if pu==0 && pv==0 { return false }

    pc = trim.pcurve                       // NurbsCurve in UV
    t  = intermediate_param(pc.domain())
    (P, T) = pc.point_and_tangent(t)       // C2D1->D1, AlgoTools3D.cxx:143-147

    du = surface.u_resolution(tol)         // tol / |dS/du|
    dv = surface.v_resolution(tol)
    (u1,v1,is_left,axis) = (P.u, P.v, false, none)
    if pu>0 {
        if (P.u-umin).abs() < du { u1 = P.u+pu; is_left=true;  axis=(-1,0) }
        else if (P.u-umax).abs() < du { u1 = P.u-pu; is_left=false; axis=(-1,0) }
    }
    if pv>0 {
        if (P.v-vmin).abs() < dv { v1 = P.v+pv; is_left=true;  axis=(0,1) }
        else if (P.v-vmax).abs() < dv { v1 = P.v-pv; is_left=false; axis=(0,1) }
    }
    if u1==P.u && v1==P.v { return false }   // not on a seam (:187-190)

    sc = dot(T, axis)                        // :192
    c1 = pc.clone()                          // original
    c2 = pc.clone().translate(u1-P.u, v1-P.v) // partner (:194-200)

    // ordering (:202-223): which pcurve is FORWARD-use vs REVERSED-use
    (fwd, rev) = if !is_left { (sc<0)? (c2,c1):(c1,c2) }
                 else         { (sc<0)? (c1,c2):(c2,c1) }
    trim.set_two_pcurves(fwd, rev)           // FORWARD coedge reads fwd, REVERSED reads rev
    trim.closed_on_face = true
    true
}
```

NURBS adaptation notes:
- Session pcurves are NurbsCurves in UV, so the "translate by period" is literally adding `period` to every control point's u (or v) coordinate — exact, no refit. This is the cheapest and most exact part of the whole port.
- **`u_resolution(tol)`** = `tol / max‖∂S/∂u‖` over the edge; this is the right seam-detection tolerance (matching `GeomAdaptor_Surface::UResolution`, `AlgoTools3D.cxx:155-157`). Using a raw UV tolerance instead will misclassify near-seam edges on sphere poles.
- The **ordering** (which pcurve is FORWARD vs REVERSED) is what determines material side. Get it wrong and the seam-straddling face flips and the solid leaks. The `dot(tangent, axis)` rule with the `is_left` branch is the exact recipe — port the four-way branch verbatim (`:202-223`).
- For a **sphere**, both U-seam (the meridian) and the V poles are degenerate; the pole edges are degenerate (single point), so they are skipped by classification (a) and never get a second pcurve. Only the meridian seam gets two pcurves.
- Detection should run inside `BuildSplitFaces` exactly where OCCT does (`Builder_2.cxx:424-450`): only when `original_edge.is_closed_on(face)` (`is_u_closed||is_v_closed` AND the edge is a U- or V-isoline, via `IsEdgeIsoline`) and the split is not already closed.

---

## (d) `unify_same_domain_faces(splitFacesA, splitFacesB)` — collapse coincident splits

OCCT `BOPAlgo_Builder::FillSameDomainFaces` (`Builder_2.cxx:571-830`).

### Algorithm
1. From all Face/Face interferences, collect candidate face indices once (`:592-613`); **sort** them (`:616`) so the representative is the min-index face.
2. For each candidate's split images, compute an **order-independent edge-set key** and group faces by equal key (`AddEdgeSet`, `:555-567`, `:648-665`). The key (`BOPTools_Set::Add`, `BOPTools_Set.cxx:152-211`) is the multiset of edge `TShape`s: degenerate edges skipped (`:170-173`), INTERNAL edges expanded to both orientations (`:177-188`), hash = sum of per-edge normalized hashes (`:207-209`); equality is exact set comparison (`IsEqual`, `:116-148`).
3. Within each group of ≥2 faces, form all pairs (`:684-708`). **Planar bounded** faces with equal edge set are declared SD immediately (`:695-700`). Others go to a parallel `AreFacesSameDomain` check.
4. `AreFacesSameDomain` (`AlgoTools.cxx:1099-1159`): find a point strictly inside F1 via the hatcher (`PointInFace`, `:1113`); the validity tolerance is `tolF1 + tolF2 + max(fuzz, Confusion)` augmented by max edge tolerance (`:1128-1153`); SD iff `IsValidPointForFace(P, F2, tol)` (`:1156`). The latter (`IntTools_Context.cxx:658-684`) projects P onto F2's surface, requires `dist ≤ tol`, and classifies the projected UV `IsPointInOnFace`.
5. Build an **undirected adjacency** of SD pairs (`FillMap`, `BOPAlgo_Tools.hxx:83-102` — adds both `n1→n2` and `n2→n1`) and extract **connected components** (`MakeBlocks`, `:45-80` — a BFS/flood over the adjacency with a fence set).
6. For each block, pick **one representative**: the face with min DS index, preferring original faces (`:756-786`); bind every face in the block to it (`myShapesSD`, `:789-794`).
7. Rewrite all face images to the representative and record origins (`:799-826`). The two coincident split faces (one from A, one from B) now point to a single shared face.

### NURBS pseudo-code

```text
fn unify_same_domain_faces(all_split_faces, surfaces, fuzz) -> Map<Face,Face> {
    // 1. group by order-independent edge-set key
    groups : Map<EdgeSetKey, Vec<Face>> = {}
    for f in all_split_faces {
        key = edge_set_key(f)                 // BOPTools_Set
        groups[key].push(f)
    }

    // 2. candidate SD pairs within each group
    pairs = []
    for (_, fs) in groups {
        if fs.len() < 2 { continue }
        for i in 0..fs.len() { for j in i+1..fs.len() {
            if planar_bounded(fs[i]) && planar_bounded(fs[j]) {
                adjacency.union(fs[i], fs[j])           // shortcut, :695-700
            } else { pairs.push((fs[i], fs[j])) }
        }}
    }

    // 3. exact SD test (AlgoTools.cxx:1099-1159)
    for (a,b) in pairs {
        if are_faces_same_domain(a, b, surfaces, fuzz) { adjacency.union(a,b) }
    }

    // 4. connected components -> representative (BOPAlgo_Tools MakeBlocks)
    sd : Map<Face,Face> = {}
    for comp in adjacency.connected_components() {
        rep = comp.min_by_key(|f| f.ds_index())          // prefer original/min index
        for f in comp { sd[f] = rep }
    }
    sd                                                   // image[f] := sd[image[f]]
}

fn edge_set_key(f) -> EdgeSetKey {                       // BOPTools_Set.cxx:152-211
    ids = []
    for e in f.edges() {
        if e.is_degenerate() { continue }
        if e.orientation == INTERNAL { ids.push((e.id, FWD)); ids.push((e.id, REV)); }
        else { ids.push((e.id, e.orientation)); }
    }
    ids.sort();  EdgeSetKey{ sum: ids.map(hash).sum(), set: ids }   // hash + exact set
}

fn are_faces_same_domain(fA, fB, surfaces, fuzz) -> bool {   // :1099-1159
    (p3d, _uvA) = point_strictly_inside(fA, surfaces[fA])    // hatcher/centroid
    tol = tol(fA) + tol(fB) + max(fuzz, CONFUSION) + max_edge_tol  // :1128-1153
    // IsValidPointForFace (IntTools_Context.cxx:658-684):
    (uvB, dist) = surfaces[fB].closest_point(p3d)            // project
    if dist > tol { return false }
    contains_point_uv(uvB, fB) != OUT                        // IN or ON
}
```

NURBS adaptation notes:
- This is the **missing step** in the session. Because the section edge is shared (Section 0), F_A and F_B that bound the same region have **identical edge-set keys** → land in the same group → SD check passes (coincident surfaces) → collapse to one. The session's Hausdorff-sew never collapses; it keeps two near-coincident faces with two near-coincident edges, which is why watertightness was only achievable by corrupting the region.
- **`edge_set_key`**: keys are edge *identities* (topology-edge indices in `m_topology_edges`), not geometry. This only works if the shared section edge is genuinely one object referenced by both faces — i.e., you must implement Section 0 first. If you skip Section 0 and try to key by edge geometry hashes with a tolerance, you reintroduce the matching ambiguity.
- **SD test for NURBS**: `point_strictly_inside(fA)` — use a hatcher-style ray in UV (intersect a vertical UV line with the trims, take the midpoint of the first interior span; mirror of `BOPTools_AlgoTools3D::PointInFace`, `AlgoTools3D.cxx:885-917`) rather than a naive centroid, which can fall in a hole. Then `surfaces[fB].closest_point` is your existing projection; tolerance is the sum-of-tolerances formula (`:1153`) — do not use a single hard-coded epsilon.
- **Representative choice** (`:756-786`): keep the face whose surface you want to survive in the result (OCCT uses min DS index = the earlier operand). Re-point both operands' shells to it; this is what yields a manifold 2-faces-per-edge result. After unification, every section edge is referenced by exactly the two surviving faces that meet there → `is_solid` true **and** correct volume.
- **Orientation on collapse**: when you re-point an image to the representative, you may need to flip it. OCCT decides via `IsSplitToReverse` for faces (`AlgoTools.cxx:1278`: same surface ⇒ compare orientations; else compare face normals at an interior point). For the edge re-use in `BuildSplitFaces` it uses the **edge** variant `IsSplitToReverse(theESp,theEOr)` (`:1384`): sample the split edge tangent at ~11 interior params, project the point onto the original edge, compare tangent dot product; `dot<0 ⇒ reverse` (`:1423-…`). Port this so the shared edge/face keeps consistent material orientation on both sides.

---

## Minimal port order (dependency-correct)

1. **Section edge as a single shared object** (Section 0) — change imprint so the A∩B curve produces one edge in a shared edge table, referenced (FORWARD+REVERSED) by both operands' face edge pools. This is the prerequisite; without it (d) cannot key by identity.
2. `split_seam_pcurve` (c) — needed before wire-splitting on closed NurbsSurfaces, else cylinder/sphere seam faces are open.
3. Wire-splitter + `classify_wire_as_hole` (a) + `contains_point_uv` (a′) + `nest_wires_into_faces` (b) — replaces `split_by_uv_curves`; produces per-face regions with correct holes.
4. `unify_same_domain_faces` (d) — collapses the coincident A/B splits; this is what flips both `is_solid=1` **and** restores correct volume.

Verification hooks already in the repo: `volume()` (must match OCCT to <1e-9 relative, not 9%), `contains_point` (regression for (a′) periodic recadrage), and the oracle-gated examples (task #6/#19) for cyl-cyl and box-sphere (task #15).