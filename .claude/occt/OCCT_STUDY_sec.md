I now have all the pieces needed. Here is the implementation-depth analysis.

---

# OCCT Shared Section-Edge Construction — Deep Read + NURBS Implementation

All citations are `file:line` under
`.../TKBO/BOPAlgo/` and `.../TKBO/BOPTools/` and `.../TKBO/BOPDS/`.

## Data structures (the contract you must replicate)

`BOPDS_Pave` (`BOPDS_Pave.hxx:27-77`): `{ myIndex: vertex_id, myParameter: t }`. Ordered by parameter via `IsLess`/`operator<` (`:59-61`). This ordering is the entire basis of "split exactly once, parameter-ordered."

`BOPDS_PaveBlock` (`BOPDS_PaveBlock.hxx:35-198`): `{ myEdge, myOriginalEdge, myPave1, myPave2, myExtPaves: ListOfPave }`. Key invariants:
- `IsSplitEdge() = (myEdge != myOriginalEdge)` (`PaveBlock.cxx:97-100`) — a pave block is the *original* edge until split.
- `ContainsParameter(t, tol, &ind)` (`PaveBlock.cxx:223-243`) — parameter-level dedup of paves.
- `Update(theLPB, theFlag)` (`PaveBlock.cxx:247-309`) — collects `myExtPaves` (+ the two bound paves if `theFlag`), **`std::sort` by parameter** (`:288`), then emits consecutive `[Pave_i, Pave_{i+1}]` pave blocks. **This is the "split exactly once" engine.**

`BOPDS_CommonBlock` (`BOPDS_CommonBlock.hxx:38-148`): `{ myPaveBlocks: ListOfPaveBlock, myFaces: ListOfInteger, myTolerance }`. The **first** pave block (`PaveBlock1`, `:88`) is the single *representative* edge shared by every coincident pave block and every listed face. `SetRealPaveBlock` (`:132`) reorders so the chosen edge is representative.

`BOPDS_Curve` (the per-intersection record): holds the `IntTools_Curve aIC = {Curve()=3D Geom_Curve, FirstCurve2d()=pcurve on F1, SecondCurve2d()=pcurve on F2}`, a *master* pave block `PaveBlock1` that accumulates ALL ext-paves, and the resulting `PaveBlocks()` list. Created in `PerformFF`.

---

## (1) Face/Face intersection → a section edge created ONCE with a 3D curve + a pcurve on EACH face

**PerformFF** (`BOPAlgo_PaveFiller_6.cxx:288-626`):
- Iterates candidate face pairs, runs `IntTools_FaceFace::Perform` in parallel (`:533`). Options `bApprox, bCompC2D1, bCompC2D2` (`:330-332`) request the approximated 3D curve **and** a pcurve on S1 and on S2 simultaneously.
- Per resulting curve, `CheckCurve` validates and builds a bbox (`:603`), then stores a `BOPDS_Curve aNC` with `SetCurve(aIC)`, `SetBox`, `SetTolerance(max(curveTol, tolFF))` (`:606-612`). So a single `IntTools_Curve` already carries `{c3d, pc_on_F1, pc_on_F2}`.

**MakeBlocks** (`:651-1108`) turns one `BOPDS_Curve` into shared section edges. The edge-creation core (`:987-1009`):
```
BOPTools_AlgoTools::MakeEdge(aIC, aV1, aT1, aV2, aT2, aTolR3D, aES);   // 3D edge from the section curve
BOPTools_AlgoTools::MakePCurve(aES, aF1, aF2, aIC,
                               PCurveOnS1(), PCurveOnS2(), myContext);  // attach pc on BOTH faces to the SAME edge
aLPBC.Append(aPB);                                                     // pave block -> this curve
aMSCPB.Add(aES, aCPB);                                                 // register the edge ONCE
```
- `MakeEdge` (`BOPTools_AlgoTools.cxx:1663-1680`) → `MakeSectEdge` (`BOPTools_AlgoTools_2.cxx:102-120`): `BRepBuilderAPI_MakeEdge(aC /*the section 3D curve itself*/, aV1, aV2, aP1, aP2)` then `BB.Range(aE, aP1, aP2)` to **preserve the parent curve's parameter range** — this is OCCT's "same-parameter" guarantee: the edge's 3D curve *is* the intersection curve, parametrised identically.
- `MakePCurve` (`BOPTools_AlgoTools.cxx:1591-1659`): loops `i=0,1`; for face `i` it takes `aIC.FirstCurve2d()` / `SecondCurve2d()` (`:1623,1628`); if present, `AdjustPCurveOnFace` then `aBB.UpdateEdge(aE, aC2DA, aFFWD, tol)` (`:1655`) — i.e. it pins the pcurve onto the SAME edge for that face. Final `BRepLib::SameParameter(aE)` (`:1658`) reconciles 3D/2D parametrisation.

The single `aES` is keyed in `aMSCPB` once and referenced by both `aF1` and `aF2` (both pcurves live on it). Later `UpdateFaceInfo`/`PutSEInOtherFaces` (`:1091,1098`) push its pave block into the `In` set of *both* faces. **That single shared edge with two pcurves is the watertightness mechanism.**

---

## (2) Paves (split vertices) and split-exactly-once

Paves are inserted into the curve's *master* pave block (`aNC.PaveBlock1()`), in this strict order inside `MakeBlocks`:

1. **Existing On/In vertices** — `PutPavesOnCurve` (`:782`, body `2269-2318`) → `PutPaveOnCurve` (`2847-2956`): `IsVertexOnLine(V, tolV, aIC, tol, &aT)` projects the vertex onto the curve; if on it, dedup via `aPB->ContainsParameter(aT, Resolution(tol), nVUsed)` (`:2892`). If new → `AppendExtPave(Pave{nV, aT})` (`:2934`), and bump the vertex tolerance to cover the gap (`:2940-2952`). EF/seam-crossing vertices are put first (`:2284-2289`).
2. **Stick / EF paves** — `PutStickPavesOnCurve` (`:795`), `PutEFPavesOnCurve` (`:799`) for tangential/edge-face crossings (`2583-2731`).
3. **Bound paves (curve endpoints)** — `PutBoundPaveOnCurve` (`:806`, body `2207-2265`): `getBoundPaves` (`2156-2203`) checks whether each end already has a pave; for a free, valid end (`IsValidPointForFaces`, `:2237`) it `MakeNewVertex`, `UpdateVertex(aIC, aT[j], aVn)`, and `AppendExtPave` (`:2257-2260`).
4. **Closing pave (closed curve)** — `PutClosingPaveOnCurve` (`:824`, body `3370-3469`): find the pave sitting at one bound `aT[j]` (`:3399-3413`); if `dist(P_vertex, P_opposite_bound) ≤ tolV+tolP` the curve is closed (`:3428-3433`); append a **second pave reusing the SAME vertex id** at the opposite parameter `aTOp` (`:3464-3468`). This is what splits a closed NURBS circle into a closeable edge.

**Split exactly once** (`:858-1030`): per curve, `aPB1->Update(aLPB, false)` (`:862`) sorts all ext-paves by parameter and emits consecutive pave blocks. Each block `[aT1,aT2]` with vertices `[nV1,nV2]` becomes exactly one edge via `MakeEdge`+`MakePCurve` above. The actual topological split of *original* operand edges happens later in **MakeSplitEdges** (`BOPAlgo_PaveFiller_7.cxx:362-533`):
```
aE = Shape(nE); aE.Orientation(FORWARD);
aV1.Orientation(FORWARD); aV2.Orientation(REVERSED);     // _7.cxx:464-470
BOPTools_AlgoTools::MakeSplitEdge(aE, aV1,aT1, aV2,aT2, aSp);
```
`MakeSplitEdge` (`BOPTools_AlgoTools_2.cxx:136-...`) does `E.EmptyCopy()` (`:144`) — copies the edge's geometry (same 3D curve, same parameters) but empties topology, then re-adds the two vertices oriented by `aP1<aP2` (`:147-168`). **EmptyCopy + reuse of parent curve = same-parameter split with no re-fit.** A pave block is split only if it has new vertices, or once if `aLPB.Extent()==1` (`:443-447`); common blocks split once and assign the representative edge (`:455-460`).

---

## (3) Common blocks — both faces reference the IDENTICAL edge

Two mechanisms unify coincident edges:

**A. Reuse during MakeBlocks** — `IsExistingPaveBlock` (two variants):
- vs *shared edges* `aLSE` (`:1908-1959`): take the section block's mid-point `aPm`, and for each shared edge run `ComputePE(aPm, tol, aE, &aTx, &aDist)` (`:1949`); if on it, return that edge `nEOut` and just bump its tolerance (`:892-900`) — **no new edge created**.
- vs On/In pave blocks via a box tree (`:1963-2152`): finds the existing pave block `aPBOut` closest to the section curve (`:2143-2147`). If `aPBOut` is present in one face's FaceInfo but not the other's (`:936-940`), it **registers that existing edge into the missing face** (`pFaces->Append(nF)`, `:957-962`) and queues it via `PreparePostTreatFF`. So the coincident edge ends up shared by both faces without duplication.

**B. PerformCommonBlocks** (`BOPAlgo_Tools.cxx:99-168`): groups pave blocks that geometrically coincide (via `MakeBlocks` grouping, `:113`) into one `BOPDS_CommonBlock`; merges their face lists (`:142-150`), then:
```
aCB->SetPaveBlocks(aLPB);            // all coincident pave blocks
aCB->SetFaces(aLFaces);              // all faces they belong to
for pb in aLPB: pDS->SetCommonBlock(pb, aCB);   // every pb now points to one CB
aCB->SetTolerance(ComputeToleranceOfCB(...));   // :164-166 / :226-332
```
`MakePCurves` (`_7.cxx:573-682`) then, for a common block, copies the pcurve from whichever member edge already `HasCurveOnSurface(aEx, aF1F)` onto the representative (`:631-677`) so the single representative edge carries a valid pcurve on *every* face in `aCB->Faces()`.

---

## (4) IMPLEMENTATION pseudo-code — `make_shared_section_edges(BRep& A, BRep& B)`

NURBS-mapped, mirroring the OCCT flow above. Session types in `«»`.

```text
struct Pave        { int vid; double t; }                  // BOPDS_Pave
struct PaveBlock   { int orig_edge; int edge=-1;           // BOPDS_PaveBlock
                     Pave p1,p2; vector<Pave> ext; }
struct SectionCurve{ NurbsCurve c3d;                       // BOPDS_Curve.IntTools_Curve
                     NurbsCurve2d pcA, pcB;                // UV pcurves on fa, fb
                     double t0,t1; AABB box; double tol;
                     PaveBlock master;                     // accumulates ext paves
                     vector<PaveBlock> blocks; }
struct SharedEdge  { int eid; NurbsCurve c3d; double t0,t1;
                     int v1,v2;
                     map<int,NurbsCurve2d> pcurve_by_face;  // <-- the watertight key
                     array<int,2> owner_faces; }
GlobalRegistry: edges[eid]->SharedEdge, vertices[vid]->Point/tol,
                face.section_pbs[fid]->set<PaveBlock>      // = OCCT FaceInfo "In"
                commonblocks: union-find over pave blocks

make_shared_section_edges(A, B):
  R = {}                                  // shared section edges
  SECTIONS = []
  // ---- PerformFF : PaveFiller_6.cxx:288-626 ----
  for (fa, fb) in candidate_face_pairs(A, B):            // AABB-overlap prefilter
      (curves, pts) = Intersection::surface_surface(fa, fb)   // returns (c3d, pcA, pcB) each
      for nc in curves:
          if !check_curve(nc): continue                  // IntTools_Tools::CheckCurve :603
          sc = SectionCurve{ c3d=nc.c3d, pcA=nc.pcA, pcB=nc.pcB,
                             t0,t1 = c3d.domain(), box=bbox(nc.c3d).enlarge(tolFF),
                             tol=max(nc.tol, tolFF) }
          sc.master = PaveBlock{ orig_edge=-1,
                                 p1=Pave{-1,t0}, p2=Pave{-1,t1}, ext=[] }
          sc.fa=fa; sc.fb=fb
          SECTIONS.push(sc)
      for p in pts: make_or_reuse_vertex(p, tolFF)        // isolated touch points

  // ---- MakeBlocks : PaveFiller_6.cxx:651-1108 ----
  for sc in SECTIONS:
      MVOnIn = vertices already On/In BOTH fa and fb      // SubShapesOnIn :742
      LSE    = edges shared by fa,fb                       // SharedEdges :743

      // (2a) put existing On/In vertices  (PutPavesOnCurve/PutPaveOnCurve)
      for vid in MVOnIn:
          (ok, t) = vertex_on_curve(vid, sc.c3d, sc.tol)   // == project + closest_parameters_curve
          if !ok: continue
          if sc.master.contains_param(t, resolution(sc.c3d, sc.tol)): continue   // dedup :2892
          sc.master.ext.push(Pave{vid, t})
          grow_vertex_tol(vid, dist(point(vid), sc.c3d.point_at(t)))

      // (2b) bound paves  (PutBoundPaveOnCurve)
      for j in {0,1}:
          if end j has no pave near t_j:
              p_end = sc.c3d.point_at(t_j)
              if !point_valid_for_faces(p_end, fa, fb, sc.tol): continue   // IsValidPointForFaces
              vid = make_new_vertex(p_end, sc.tol)
              sc.master.ext.push(Pave{vid, t_j})

      // (2c) closing pave on CLOSED curve  (PutClosingPaveOnCurve :3370)
      if sc.c3d.point_at(t0).is_equal(sc.c3d.point_at(t1), confusion):
          find pave P at one bound t_j with vertex vP
          if dist(point(vP), sc.c3d.point_at(other_bound)) <= tol(vP)+tolP:
              sc.master.ext.push(Pave{vP.vid, other_bound})   // SAME vid, opposite param

      // (2d) split-once : PaveBlock::Update  (sort by param, emit consecutive blocks)
      sc.blocks = update_paveblock(sc.master)              // sort ext+bounds by t; pair consecutively

      // (3) make section edges
      for pb in sc.blocks:
          if |pb.p1.t - pb.p2.t| < pconf: continue
          tm = 0.5*(pb.p1.t+pb.p2.t)
          if !valid_block_for_faces(sc, tm, fa, fb): continue   // IsValidBlockForFaces :884
                                                                 // classify UV midpoint in BOTH faces
          // common-block / reuse detection (IsExistingPaveBlock :1908 / :1963)
          eid = find_coincident_existing_edge(sc, pb, LSE ∪ onin_edges(fa,fb))
          if eid != NONE:
              ensure_edge_in_face(eid, fa); ensure_edge_in_face(eid, fb)   // attach missing pcurve
              union_find.union(pb, edges[eid].pb)                          // -> CommonBlock
              continue
          // --- create the shared edge ONCE ---
          v1 = vertex(pb.p1.vid); v2 = vertex(pb.p2.vid)
          c3d_seg = sc.c3d.segment(pb.p1.t, pb.p2.t)        // MakeSectEdge: reuse parent param
          pcA_seg = sc.pcA.segment(pb.p1.t, pb.p2.t)        // MakePCurve on fa
          pcB_seg = sc.pcB.segment(pb.p1.t, pb.p2.t)        // MakePCurve on fb
          eid = new_edge_id()
          se  = SharedEdge{ eid, c3d=c3d_seg, t0=pb.p1.t, t1=pb.p2.t,
                            v1=pb.p1.vid, v2=pb.p2.vid,
                            pcurve_by_face={ fa:pcA_seg, fb:pcB_seg },
                            owner_faces={fa,fb} }
          R[eid] = se;  pb.edge = eid;  edges[eid] = se
          face.section_pbs[fa].add(pb);  face.section_pbs[fb].add(pb)   // In BOTH faces

  // ---- PostTreatFF : intersect new section edges among themselves ----
  // run make_shared_section_edges-style E/E intersection over R's edges,
  // splitting any section edge that another section edge crosses,
  // fusing coincident vertices (union their vids).            (PaveFiller_6.cxx:1135)
  recursively_split_section_edges(R)

  // ---- PerformCommonBlocks : Tools.cxx:99-168 ----
  for grp in union_find.groups():
      cb = CommonBlock{ pave_blocks=grp, faces=∪ faces(grp) }
      rep = choose_min_original_edge(grp)                  // representative edge
      for pb in grp: pb.edge = rep.edge                    // all reference identical edge
      for f in cb.faces:
          if !edges[rep.edge].pcurve_by_face.has(f):
              edges[rep.edge].pcurve_by_face[f] = pullback(edges[rep.edge].c3d, surface(f))

  return R   // shared section edges + split vertices, ready for face splitting
```

Helper `find_coincident_existing_edge` (the common-block test, mirrors `IsExistingPaveBlock`):
```text
pm = sc.c3d.point_at(tm)
for e in candidates (box-tree by pm ± tol):
    (q, te) = e.c3d.closest_point(pm)                    // ComputePE
    if dist(pm,q) <= tol(e)+tol(sc):
        // confirm endpoints too (P(t1),P(t2)) and (optionally) tangent alignment :2077-2103
        if endpoints_coincide(sc,pb,e): return e.eid
return NONE
```

---

## (5) NURBS-adaptation notes (mapping each analytic OCCT step)

- **`IntTools_Curve {c3d, FirstCurve2d, SecondCurve2d}` → your `(c3d:NurbsCurve, pcA:NurbsCurve2d, pcB:NurbsCurve2d)`** from `Intersection::surface_surface`. You already have all three, so `PerformFF`'s `bCompC2D1/bCompC2D2` work is free — do NOT recompute pcurves by projection (that is exactly the independent-split bug). Use the matched pair the intersector returns.

- **Same-parameter (`MakeSectEdge`, `_2.cxx:117`; `EmptyCopy`, `_2.cxx:144)`**: never re-fit. The section edge's 3D curve is `sc.c3d.segment(t1,t2)` and its two pcurves are `sc.pcA.segment(t1,t2)`, `sc.pcB.segment(t1,t2)` over the *identical* `[t1,t2]`. Because all three share the parameter domain, `BRepLib::SameParameter` (`AlgoTools.cxx:1658`) is unnecessary — your `NurbsCurve.segment` must keep the parent knot parametrisation (a true sub-curve extraction, not a reparametrised copy).

- **"Closing pave on a closed curve" = splitting a closed NURBS circle** (`PutClosingPaveOnCurve :3370`): detect `c3d.point_at(t0) ≈ c3d.point_at(t1)`. With only the closing pave you get one closed edge `v1==v2` over `[t0,t1]`. With an interior pave you get two arcs that share both endpoint vertices. Extract each arc with `NurbsCurve.segment` on the 3D curve AND on `pcA`/`pcB` so the UV trims close up too.

- **`IsValidBlockForFaces` (`:884`)**: classify the curve mid-point in UV. Use `pcA.point_at(tm)` and `pcB.point_at(tm)` and your `contains_point` (loop/trim classification in UV) on `fa` and `fb` respectively. Keep the block only if inside the trimmed region of BOTH.

- **Vertex dedup is 3D, pave dedup is parametric**: vertex coincidence uses tolerance balls (`ComputeVV`, `AlgoTools.cxx:1684-1730`); pave dedup on a single curve uses `ContainsParameter` with `param_tol = curve.resolution(tol3d)` (`PaveBlock.cxx:223`, used `:2890`). Replicate both — `closest_parameters_curve` for the parametric one.

- **Common block → shared edge** (`Tools.cxx:99-168`): a `CommonBlock` is a union-find class of coincident pave blocks plus a face list, collapsed to ONE representative edge. In the session this is a registry where `pb.edge` indices converge to one `eid`, and `SharedEdge.pcurve_by_face` gains an entry per face (via surface `pullback` for any face that lacked the pcurve, as `MakePCurves _7.cxx:631-677` does). This is precisely what your post-hoc co-splitting attempt lacked: the edge must be created once and *registered into both faces' In-sets* during construction, not reconciled afterward.

- **`PostTreatFF` (`:1135`)**: the just-made section edges must themselves be intersected/split where curves from different face-pairs cross (e.g. three-surface corners), with vertex fusion. In NURBS terms, recurse the same pave/split machinery over `R`'s edges (curve-curve intersection + `update_paveblock`), unioning coincident vertex ids — otherwise T-junctions between section edges leak.

### Authoritative citations
- FF→curves+pcurves, edge made once + pcurve on both faces: `BOPAlgo_PaveFiller_6.cxx:288-626` (PerformFF), `:651-1108` (MakeBlocks), edge core `:987-1009`.
- MakeEdge/MakeSectEdge/MakePCurve: `BOPTools_AlgoTools.cxx:1663-1680`, `:1591-1659`; `BOPTools_AlgoTools_2.cxx:102-120`.
- Paves & split-once: `PutPavesOnCurve` `:2269-2318`, `PutPaveOnCurve` `:2847-2956`, `PutBoundPaveOnCurve` `:2156-2265`, `PutClosingPaveOnCurve` `:3370-3469`; `PaveBlock::Update` (`BOPDS_PaveBlock.cxx:247-309`), `ContainsParameter` `:223-243`.
- Split edge / EmptyCopy: `BOPAlgo_PaveFiller_7.cxx:362-569`; `BOPTools_AlgoTools_2.cxx:124-168`.
- Common blocks: `BOPAlgo_PaveFiller_6.cxx:1908-2152` (IsExistingPaveBlock), `BOPAlgo_Tools.cxx:99-222` (PerformCommonBlocks), `ComputeToleranceOfCB` `:226-332`; pcurve copy onto representative `BOPAlgo_PaveFiller_7.cxx:631-677`.
- Data structures: `BOPDS_PaveBlock.hxx`, `BOPDS_Pave.hxx`, `BOPDS_CommonBlock.hxx`.