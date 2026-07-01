I have everything needed. Full path roots used in citations:
- `…/TKBO/BOPAlgo/` (BOPAlgo_*.cxx) and `…/TKBO/BOPTools/` (BOPTools_AlgoTools.cxx), under `C:/pc/3_code/code_rust/session/validation/occt_oracle/build/deps/occt/src/occt/src/ModelingAlgorithms/`.

---

# OCCT boolean: face classification, per-op selection, solid assembly — implementation-depth port to NURBS

## 0. THE ARCHITECTURAL FIX (why session's imprint→sew is wrong, and what to copy)

The session bug: each operand's faces are split independently, then sewn by Hausdorff edge matching. OCCT never does this. The section curve is created **once**, stored as a **single edge in the DS**, and that *same* edge object is handed to the face-splitter of **both** operands.

Proof in source — `BuildSplitFaces` (`BOPAlgo_Builder_2.cxx:231`). For face `aF` it assembles one edge list `aLE`:
- bounding edges + their images (`:357-460`),
- **IN** edges `aMPBIn` (`:463-475`), pushed FORWARD and REVERSED,
- **section** edges `aMPBSc` (`:478-489`): `nSp = aPB->Edge(); aSp = myDS->Shape(nSp)` — pushed FORWARD and REVERSED.

`myDS->Shape(nSp)` is the *same DS index* for the section edge shared between face-A_i and face-B_j. So when `BOPAlgo_BuilderFace` splits A_i and B_j, the two resulting faces literally reference the identical edge (same TShape pointer). Watertightness is structural, not reconstructed. The PaveFiller already stored that edge's pcurve on surface A **and** on surface B.

**NURBS mapping (the single most important change):** when you call `Intersection::surface_surface(A_i, B_j) -> (curve3d, pcurve_a, pcurve_b)`, you must allocate:
- ONE `m_curves_3d` entry (the section curve3d) and ONE `BRepEdge` `e` (`brep.h:30`),
- TWO `BRepTrim` (`brep.h:37`): trim_a with `curve_2d_index → pcurve_a`, trim_b with `curve_2d_index → pcurve_b`, **both with `edge_index = e`**.

Then your `split_by_uv_curves` for face A_i is fed pcurve_a, for B_j is fed pcurve_b, but the produced sub-faces on each side point at the *same* `e`. Never Hausdorff-match afterwards. This is the fix for the 9% volume corruption: co-splitting post-hoc gives two *different* edges with two *different* 3D curves; here there is one curve, two pcurves, one edge.

The four sub-algorithms below all assume this shared-edge graph exists.

---

## (a) `classify_face_state(splitFace F, otherSolid S) -> IN | OUT | ON`

OCCT splits this into an ON test (same-domain) and an IN/OUT test, with a cheap local "angle" path and a robust global "point" fallback.

**ON / same-domain** — `BOPTools_AlgoTools::AreFacesSameDomain` (`BOPTools_AlgoTools.cxx:1099-1159`):
```
P, uv1 = point_in_face_interior(F)                 # AlgoTools.cxx:1113 PointInFace (hatcher)
tol = tolF1 + tolF2 + max(fuzz, confusion)         # :1128-1153 (incl. max edge tol)
ON  iff  IsValidPointForFace(P, otherFace, tol)    # :1156 project P→surf(G), classify uv in G, dist<tol
```

**IN/OUT** — `BOPTools_AlgoTools::IsInternalFace` (`:782-863`):
1. Local angle path (`:802-851`): find an edge of F shared with S's edge map; via `GetFaceOff` decide whether F is the "off" face that turns into S (`iRet==1 → IN`). Cheap, robust near coincidences.
2. Fallback `ComputeState(F, S, tol, bounds, ctx)` (`:642-690`): find an edge of F that is **not** on S's boundary, take its midpoint, `ComputeState(point, S)` = `BRepClass3d_SolidClassifier::Perform` (ray cast, `:765-778`). If every edge is on S, use `PointInFace` interior point and classify that.

**NURBS pseudo-code (reuse session `contains_point`, exactly as the task states):**
```
classify_face_state(F, S):
    P, uv = point_in_face_interior(F)        # strictly interior, > tol from every pcurve
    # ON test
    for G in S.faces with AABB(G) ∋ P:
        (uvG, d) = closest_point(surface(G), P)     # session closest_point
        if d < tolF + tolG + fuzz and uv_inside_trimmed_domain(G, uvG):
            sameOrient = dot(normal(F,uv), normal(G,uvG)) > 0
            record_SD_pair(F, G, sameOrient)
            return ON
    # IN/OUT: ray cast on a deep-interior point
    return S.contains_point(P) ? IN : OUT
```
Notes: pick `P` from a non-shared edge midpoint first (mirrors `ComputeState` `:653-664`) to avoid points sitting on S's boundary; only fall back to deep interior. For curved split faces keep `P` ≥ a few·tol off the boundary — the OCCT `PointNearEdge` fallback (`:682`) exists precisely for thin faces. Connexity-block optimization (classify one face per edge-connected block, `BOPAlgo_Tools.cxx:1401`/`1477`) is optional speed only.

---

## (b) `select_faces_for_op(op, statesA, statesB)` — the exact IN/OUT/ON rule + reversing

OCCT has **two** equivalent code paths. The default (closed solids) selects whole **cells** (`BuildRC`, `BOPAlgo_BOP.cxx:572`); the open-solid fallback `BuildBOP` rebuilds from the **face set** (`:867-878`) — that face-set form is what the session should implement. Both yield the same table.

### Cell-level (what OCCT literally does) — `BuildRC`, `BOPAlgo_BOP.cxx:572-849`
Each operand solid was already split into cells (images) by `BuildSplitSolids` (`Builder_3.cxx:465-492`: a solid's draft faces **plus** the other operand's IN-faces, both orientations, fed to `BuilderSolid`). Then:
- `aMArgsIm` = arg cells, `aMToolsIm` = tool cells (`:642-690`).
- COMMON (`:752-757`): keep arg cell `S` iff it is contained in tool images (`bContains` = map identity **or** same-domain face-set `BOPTools_Set`, `:744-750`).
- CUT (`:759-765`): keep arg cell iff **not** contained in tool images.
- CUT21 (`:698-702`): swap the iterate/check maps.
- FUSE (`:583-598`): take **all** splits into a compound, then `BuildSolid` (`:1075`) drops shared membrane faces by keeping only faces that occur **once** (`MapFacesToBuildSolids` + `aLSx.Extent()==1`, `:1193-1203`).

### Face-membrane table (implement this in NURBS — equals OCCT `BuildBOP` path)
With `sA(f)=classify_face_state(f∈A, B)` and `sB(f)=classify_face_state(f∈B, A)`; ON faces come in coincident pairs `(fA,fB)` with `sameOrient` from (a):

```
FUSE (A ∪ B):
    keep fA where sA==OUT ;  keep fB where sB==OUT
    ON pair: keep ONE rep if sameOrient (skins touching from same side); DROP if opposite (interior membrane cancels)
    orientation: as-is (outward normals already valid)

CUT (A − B):
    keep fA where sA==OUT                      # outer skin of A
    keep fB where sB==IN, REVERSED             # B's surface inside A → cavity wall, normal flipped
    ON pair: keep the opposite-orientation rep (tool caps A flush), reversed; DROP same-orientation
CUT21 (B − A): symmetric swap

COMMON (A ∩ B):
    keep fA where sA==IN ;  keep fB where sB==IN
    ON pair: keep ONE rep if sameOrient; DROP if opposite
    orientation: as-is
```

**Reversing — how OCCT actually realizes it.** Per-face reversal is *not* hand-applied in selection; it is emergent:
1. The membrane primitive is `IsSplitToReverse(faceSplit, faceOrig)` (`BOPTools_AlgoTools.cxx:1278-1380`): if same underlying surface → reverse iff orientations differ (`:1290-1293`); else find interior point of the split, project onto original surface, compare **surface normals**, reverse iff `dot < 0` (`:1378`). This is applied when stamping faces into a draft shell (`BuildDraftSolid`, `Builder_3.cxx:301-306`).
2. Shell coherence then propagates orientation across shared edges (`OrientFacesOnShell`, `AlgoTools.cxx:349-492`: if the shared edge has the **same** orientation in both neighbors, reverse the second, `:442-447`).
3. The whole-shell flip for CUT cavities is decided by the hole test in (d).

**NURBS:** implement `IsSplitToReverse` with `NurbsSurface` normals (`dot(n_split, n_orig)<0`); same-surface shortcut compares `face.reversed`/`trim.reversed` flags. Do NOT bake CUT's reversal into the table by hand — feed the table's faces, then let shell assembly (c) + hole test (d) fix global orientation, exactly as OCCT.

---

## (c) `assemble_solid(faces)` — shared-edge identity + `GetFaceOff` dihedral tie-break

`BOPAlgo_ShellSplitter::SplitBlock` (`BOPAlgo_ShellSplitter.cxx:150-415`) + `BuilderSolid::PerformLoops` (`BOPAlgo_BuilderSolid.cxx:222`).

### Step 1 — connexity blocks + regular fast path
`MakeConnexityBlocks` (`AlgoTools.cxx:181-243`) groups faces connected through shared edges. A block is **regular** iff every connection edge has exactly 2 faces (`:235`); regular → `MakeShell` + `OrientFacesOnShell` directly (`ShellSplitter.cxx:633-641`). Non-regular (an edge with >2 faces — the boolean T-junction case) → `SplitBlock`.

### Step 2 — drop dangling faces
`PerformShapesToAvoid` (`BuilderSolid.cxx:129-218`) and `SplitBlock`'s pre-loop (`ShellSplitter.cxx:181-218`): iteratively remove any face owning an edge with only **1** adjacent (non-internal, non-degenerate) face — it can never close. Repeat to fixpoint.

### Step 3 — grow shell by edge-adjacency, choosing neighbor by min dihedral
`SplitBlock` (`ShellSplitter.cxx:245-414`):
```
EFmap = edge -> [faces]                              # global, shared-edge identity
for each unused start face F0:
    shell = {F0};  localEF = edges(F0)
    for F in shell (growing):
        for edge E of F that is still "free" in shell (localEF[E] < 2):   # :277-285
            if E internal or degenerate: continue                          # :287-296
            cand = []                                                      # :308-335
            for G in EFmap[E], G != F, G not added:
                if not GetEdgeOff(E, G, EL): continue   # EL = E's occurrence in G with OPPOSITE orient
                cand.append((EL, G))
            if cand empty: continue
            if len(cand)==1: sel = cand[0].G
            else:            sel = GetFaceOff(E, F, cand)   # min-dihedral tie-break   :353
            if sel not added: add sel to shell; update localEF
    RefineShell(shell) -> sub-shells; closed ones are loops                 # :367, :437
```
- `GetEdgeOff` (`AlgoTools.cxx:1067-1095`): returns the occurrence of `E` inside candidate `G` whose orientation is the **reverse** of `E` in `F`. This is what guarantees the two faces are coherently oriented across the shared edge (manifold sewing by identity, no Hausdorff).
- `RefineShell` (`:437-609`) splits the grown shell at **stop edges**: edges with >2 faces (`:457`), 2 faces with the **same** edge orientation (non-manifold, `:470-474`), or internal-double (`:500-503`). Closed remnants → loops.

### Step 4 — `GetFaceOff`: the dihedral selection (`AlgoTools.cxx:962-1063`, helper `GetFaceDir:2052`)
For an edge with >2 incident faces, pick the neighbor reached first when rotating around the edge from the reference face's interior — this walks one coherent side of the solid:
```
C3 = curve3d(E);  t = midparam;  Px = C3.D0(t)                 # :982-984
Tgt = EdgeTangent(E, t)            (oriented by E.orientation)  # :986-988
# reference plane ⟂ Tgt at Px; all binormals projected into it  # :990-992
# reference face F1:
N1  = face_normal_on_edge(F1, E, t)   (reverse if F1 REVERSED)  # GetFaceDir :2067-2071
B1  = N1 × Tgt                                                  # binormal, points INTO face  :2074
B1  = refine_into_face(F1, Px, B1, dt)   # step dt into face, reproject  FindPointInFace :2104
REF = N1 × B1                                                   # angle axis  :1006
best = +inf
for (E2, F2) in candidates:                                    # :1014-1061
    Tgt2 = (E2.orient == E.orient) ? Tgt : -Tgt                # :1020
    N2 = face_normal_on_edge(F2, E2, t); B2 = N2 × Tgt2; refine
    ang = AngleWithRef(B1, B2, REF)                            # signed, (−π,π]   :1031
    if |ang|<angular:  ang = (F2==F1)? π : (sameSurf? 2π : ang)# folds  :1033-1043
    if ang < 0: ang += 2π                                      # → [0,2π)  :1051-1054
    if ang < best: best = ang; FOff = F2                       # MIN dihedral  :1056-1060
```
`AngleWithRef(v1,v2,ref)` exact formula (gp_Vec semantics): `atan2( dot(cross(v1,v2), unit(ref)), dot(v1,v2) )`.

**NURBS adaptation of `GetFaceOff` (all available from `NurbsSurface`/`NurbsCurve`):**
- `Px, Tgt`: `curve3d.D1(t)` of the section `NurbsCurve`, normalize derivative; flip by edge orientation.
- `face_normal_on_edge`: pull edge param → uv via the trim's 2D `NurbsCurve` (`m_curves_2d[trim.curve_2d_index]`), evaluate `Su×Sv` of `NurbsSurface`, normalize; flip by `face.reversed XOR trim.reversed`.
- `B = N×Tgt`, then `refine_into_face`: step in uv by `dt/‖Su‖, dt/‖Sv‖` along the B direction, re-evaluate, recompute `B = unit(Px→Pstep)` (mirrors `FindPointInFace` `:2150-2171`). Pick `dt ≈ 2(tolE+tolF)`, bumped to ~5e-4 for spheres/freeform (`MinStep3D:2228-2240`).
- The rest is exact vector algebra; no Geom2d/IntAna needed.

`OrientFacesOnShell` (`AlgoTools.cxx:349-492`) finalizes: BFS faces, and for each shared 2-face edge, if `Orientation(E,F1)==Orientation(E,F2)` and neither is a seam, reverse F2 (`:442-447`). NURBS: compare the sign of the trim's direction along the shared edge in each face (`trim.reversed`); flip the face's `reversed` flag to disagree.

---

## (d) shell growth / hole nesting — `BuilderSolid::PerformAreas` (`BOPAlgo_BuilderSolid.cxx:395-591`)

```
for shell in loops:                                            # :411
    growth = IsGrowthShell(shell, holeFaces)                   # fast: contains a known hole face  :835
          or not IsHole(shell)                                 # real test  :424
    if growth: newSolids += solid(shell)                       # :430-434
    else:      holeShells += shell; holeFaces += faces(shell)  # :437-438

if no holes: return newSolids                                  # :442
BVH over hole boxes                                            # :461-476
for solid in newSolids:                                        # :482
    for hole in BVH.select(box(solid)):                        # :502
        if IsInside(hole, solid):                              # :509  face of hole, ComputeState vs solid == IN
            if hole already assigned to prev:
                if IsInside(solid, prev): reassign hole→solid  # keep INNERMOST  :513-519
            else: assign hole→solid
add each hole as inner shell into its solid; reload classifier # :561,565
holes inside nothing → standalone (inverted) solids            # :571-588
```
- `IsHole` (`:794-801`) = `BRepClass3d_SolidClassifier::PerformInfinitePoint`; `State()==IN` ⇒ shell normals point inward ⇒ it bounds a cavity. `IsInside` (`:806-830`) = classify a face of one shell vs the other solid (`ComputeState`, `IN`).

**NURBS adaptation:**
- `IsHole` ⇒ **sign of `BRep::volume()`** (you already compute it by the divergence/flux integral): a coherently-oriented growth shell has volume > 0; a hole shell (inward normals) integrates < 0. Equivalent to `PerformInfinitePoint` and avoids needing an infinite-point classifier. (Or: `contains_point` of a far-away point — IN ⇒ hole.)
- `IsInside(hole, solid)` ⇒ `solid.contains_point(P)` for `P` = interior point of any hole face.
- innermost-of-multiple ⇒ mutual `contains_point` test (`:516`).
- Add hole shell into solid's `m_loops`/shell list as an inner shell (its faces keep inward orientation so the solid's net flux subtracts the cavity).

---

## NURBS adaptation cheat-sheet (every OCCT Geom/IntAna step → session equivalent)

| OCCT primitive (file:line) | session NURBS equivalent |
|---|---|
| `BRep_Tool::Curve(E)` D0/D1 | `m_curves_3d[edge.curve_3d_index]` eval (`brep.h:31`,`77`) |
| pcurve `Geom2d` on face | `m_curves_2d[trim.curve_2d_index]` (`brep.h:38`,`78`) |
| `BRep_Tool::Surface(F)` | `m_surfaces[face.surface_index]` (`brep.h:52`,`76`) |
| `GetNormalToFaceOnEdge` (`:2067`) | pull edge t→uv via trim pcurve; `unit(Su×Sv)`; flip by `face.reversed XOR trim.reversed` |
| `EdgeTangent` (`:986`) | `curve3d.D1` normalized, flip by edge orientation |
| `BRepClass3d_SolidClassifier::Perform(P)` (`:773`) | `BRep::contains_point(P)` (`brep.h:155`) |
| `PerformInfinitePoint` (`:799`) | **sign of `BRep::volume()`** (`brep.h:150`), or `contains_point(far)` |
| `IsValidPointForFace(P,F,tol)` (`:1156`) | `closest_point(surf,P)`→uv; uv inside trimmed domain (pcurve winding) AND dist<tol |
| `PointInFace` hatcher (`:1113`) | interior uv sample (reuse `split_by_uv_curves` cells / trim centroid pullback), nudge > tol off every pcurve |
| `IsSplitToReverse(face)` (`:1378`) | `dot(n_split, n_orig)<0`; same-surface ⇒ compare `reversed` flags (`:1290`) |
| `AngleWithRef(v1,v2,ref)` | `atan2(dot(cross(v1,v2),unit(ref)), dot(v1,v2))` |
| `WireSplitter`/IntAna UV arrangement | constrain `split_by_uv_curves` to the exact edge set of `BuildSplitFaces` `aLE` (bounding split ⊕ IN both-orient ⊕ SECTION both-orient, `Builder_2.cxx:354-489`); keep minimal loops |
| seam handling `DoSplitSEAMOnFace` (`Builder_2.cxx:430`) | your B5 seam-aware NURBS pcurves (cylinder/sphere) — closed edges get FORWARD+REVERSED copies (`:444-447`) |

**Sequencing to copy (`BOPAlgo_BOP::PerformInternal1` order, `BOPAlgo_BOP.cxx:411-568`):** split edges (shared section edges created once) → `FillImagesFaces` (split faces of BOTH operands against the shared edges) → `FillSameDomainFaces` (mark ON pairs) → `FillImagesSolids` (split each solid into cells using other operand's IN faces, both orientations) → `BuildRC`/select per op → `BuilderSolid` (shell assembly (c) + hole nesting (d)).

**Two failure modes the session must avoid:** (1) never Hausdorff-sew after the fact — one section curve, two pcurves, one shared `BRepEdge`; (2) don't hand-apply CUT reversal — feed the (b) table's faces and let `IsSplitToReverse` + `OrientFacesOnShell` + `IsHole` (volume sign) fix global orientation, exactly as OCCT does.