I now have the complete algorithm and both sides of the mapping. Here is the implementation-depth analysis.

---

# OCCT WireSplitter → NURBS face-splitting: exact pseudo-code

**Source files** (all under `C:/pc/3_code/code_rust/session/validation/occt_oracle/build/deps/occt/src/occt/src/ModelingAlgorithms/TKBO/BOPAlgo/`):
- `BOPAlgo_WireSplitter.cxx` (driver / connexity blocks)
- `BOPAlgo_WireSplitter_1.cxx` (the real algorithm: `SplitBlock`, `Angle2D`, `Path`, `ClockWiseAngle`, `RefineAngles`)
- `BOPAlgo_WireSplitter.lxx` (`BOPAlgo_EdgeInfo` class + `MakeWire`)
- `BOPAlgo_BuilderFace.cxx` (how the edge set is fed + post-classification of loops into faces)
- `BOPTools_ConnexityBlock.hxx` (the block container, `IsRegular`)

I cite these as `WS.cxx:N`, `WS1.cxx:N`, `WS.lxx:N`, `BF.cxx:N` below.

---

## 0. Where this sits in the boolean pipeline (the part the session gets wrong)

OCCT does **not** split each operand independently and re-match. The section curve is computed once (PaveFiller), turned into **one** `TopoDS_Edge`, and that **same edge** is inserted into the WireEdgeSet of **every** face it lies on, from **both** operands (`BOPAlgo_BuilderFace::PerformLoops`, `BF.cxx:257-270`). Inside a single face split, that one section edge is fed **twice** — once FORWARD, once REVERSED — so it can bound a region on each of its two sides. WireSplitter then re-assembles minimal closed wires purely from 2D angle ordering. Faces are made and the holes/growths are sorted afterward by `IntTools_FClass2d` (`BF.cxx:386-604`). The session must adopt this: **one shared section edge object referenced by faces of both solids**, and feed it with both orientations into the per-face arrangement.

**`EdgeInfo` fields** (`WS.lxx:24-71`): `myEdge`, `myPassed` (walk visited flag), `myInFlag` (In=arrives, Out=leaves), `myIsInside` (true ⇒ section/interior edge, false ⇒ original boundary edge), `myAngle` (2D angle at the vertex). These five fields are the entire per-edge-end state you must replicate.

---

## 1. Building `mySmartMap` (vertex → list of incident edge-ends)

`WS1.cxx:125-189`. `mySmartMap` is an indexed map keyed by **vertex (orientation-ignored** — `TopTools_ShapeMapHasher` uses `IsSame`), value = list of `EdgeInfo`. A second map `aMS` is used to tag boundary-vs-section.

```
SmartMap  : map<VertexKey, list<EdgeInfo>>     // VertexKey ignores orientation
aMS       : set<EdgeKey>                        // edges seen an ODD number of times so far
VertMap   : map<VertexKey, bool isClosedSeam>

for each directed edge E in block.edges:                      // WS1.cxx:133
    if not HasCurveOnSurface(E, face): continue               // need a pcurve  (WS1.cxx:137)
    bIsClosed = IsDegenerate(E) or IsSeamClosed(E, face)      // WS1.cxx:142

    // --- boundary vs section detection (KEY) ---
    if not aMS.add(E):            // already present  => appears >=2x
        if not bIsClosed: aMS.remove(E)                        // WS1.cxx:144-145
    // net effect: aMS ends holding edges that occur an ODD number of
    // times = the ORIGINAL boundary edges (occur once). Section edges,
    // fed FORWARD+REVERSED, occur twice => absent from aMS => "inside".

    i = 0
    for each vertex-occurrence V in E (in topo order):         // WS1.cxx:148
        idx = SmartMap.findOrAdd(V, empty_list)
        ei = EdgeInfo()
        ei.edge   = E
        ei.inFlag = (V.orientation == REVERSED)   // REVERSED vtx = edge END = arrives = IN
                                                   //  FORWARD vtx = edge START = leaves = OUT
        SmartMap[idx].append(ei)                               // WS1.cxx:163-166

        if i==0: v1 = V
        else:    bIsClosed = bIsClosed or v1.IsSame(V)         // self-loop edge
        VertMap[V] = VertMap.get(V) or bIsClosed               // WS1.cxx:177-187
        i++
```

Then a **"nothing to do" fast path** (`WS1.cxx:191-286`): if every vertex has exactly one In and one Out **and** no edge is duplicated (no edge appears with both orientations), the block is already a single clean wire — just `MakeWire` and return. Otherwise run the angle arrangement. (`ConnexityBlock::IsRegular`, `CB.hxx:50`, gates this at the `WS.cxx:185-197` level too: regular blocks skip `SplitBlock` entirely.)

**Set `IsInside` per edge-end** (`WS1.cxx:301`): `ei.isInside = not aMS.contains(E)`. So *section edges are "inside", boundary edges are not*. This flag drives the section-following rule in §3.

> **NURBS mapping.** VertexKey = your `BRepVertex` index (geometric, orientation-free). An "edge-end" = one directed use of a trim at one of its two vertices. Feed: (a) each **original boundary trim** of face F once with its stored orientation; (b) each **section pcurve** on F **twice** — forward and reversed — as two directed edges sharing the same underlying section-edge id. `isInside = (edge is a section pcurve)`; you can set this directly instead of the `aMS` parity trick, but the parity trick also auto-detects seams, so keep an explicit `is_section` flag on each fed edge.

---

## 2. `Angle2D` — 2D angle of an edge-end at a vertex, from the pcurve tangent with curvature-aware step

`WS1.cxx:758-830`. This is **not** the chord vertex→other-vertex; it is a *local* direction taken a tiny, curvature-bounded step `dt` into the curve, so that two edges leaving the same point in nearly the same direction but curving apart still get distinct, correctly-ordered angles.

```
Angle2D(V, E, face, surf, bIsIN):                              // WS1.cxx:758
    tV = Parameter(V, E, face)            // pcurve param at this vertex
    if Infinite(tV): return 0
    (C2d, first, last) = CurveOnSurface(E, face)               // the pcurve
    tol2d = 2 * Tolerance2D(V, surf)                           // WS1.cxx:777

    dt = max( C2d.Resolution(tol2d), PConfusion )              // WS1.cxx:782
    // Resolution(tol) = parameter step that displaces the 2D point by `tol`
    //                 = tol / |C2d'(t)|   (pcurve speed)

    if type(C2d) != Line:                                      // WS1.cxx:785
        LProp = CLProps2d(C2d, tV, order=2)                    // curvature at tV
        if LProp.IsTangentDefined():
            R = LProp.Curvature()                              // = |kappa|
            if R > PConfusion:
                R = 1/R                                        // radius
                cosphi = R / (R + tol2d)
                dt = max(dt, acos(cosphi))   // bigger step on tight curves (WS1.cxx:795)

    aTX = 0.05*(last-first)                                    // cap  (WS1.cxx:800)
    if aTX < 5e-5: aTX = min(5e-5, (last-first)/2)
    if dt > aTX:   dt = aTX

    // step INTO the curve interior from the vertex end
    if |tV-first| < |tV-last|: tV1 = tV + dt                   // WS1.cxx:812
    else:                      tV1 = tV - dt

    Pv  = C2d.D0(tV)                                           // vertex UV point
    Pv1 = C2d.D0(tV1)                                          // offset UV point

    // direction of TRAVEL along the wire at this end:
    vec = bIsIN ? (Pv1 -> Pv)      // arriving: heading toward the vertex
                : (Pv  -> Pv1)     // leaving:  heading away from the vertex   (WS1.cxx:824)
    return Angle(unit(vec))        // atan2 normalized to [0, 2π)   (WS1.cxx:834-842)
```

`Angle(dir)` = CCW angle from +U axis, folded into `[0, 2π)` (`WS1.cxx:834-842`).

The angle is **stored once per edge-end** in `SplitBlock` (`WS1.cxx:292-310`): the vertex is given orientation `REVERSED` if In else `FORWARD`, then `ei.angle = Angle2D(...)`. Note both In and Out angles are *directions of travel*: for an In edge the angle points **toward** the vertex (the arrival heading); this is why `ClockWiseAngle` later reverses it with `+π`.

> **NURBS mapping (cv-based tangent).** The pcurve is a `NurbsCurve` in UV (x=u, y=v, z=0). Use the analytic, control-point-based derivative — `NurbsCurve::evaluate(t, 1)` returns `[point, C'(t)]` (`nurbscurve.cpp:2084`), NOT the finite-difference `tangent_at`. Then:
> - `|C2d'(t)|` for `Resolution`: `du = evaluate(tV,1)[1]; speed = hypot(du.x, du.y); dt0 = tol2d/speed`.
> - curvature `R`: `NurbsCurve::curvature_at` adapted to 2D (use only x,y of `evaluate(t,2)`): `kappa = |x'·y'' − y'·x''| / speed³`.
> - `tol2d ≈ tolerance3d / uv_to_3d`, where `uv_to_3d` is the local 3D-length-per-unit-UV already computed in `split_by_uv_curves` (`nurbssurface_trimmed.cpp:557-559`).
> - Keep OCCT's `dt`-chord (`D0(tV)`, `D0(tV1)`) rather than using `C'` directly as the angle — the curvature-bounded chord is what makes high-curvature nodes order correctly and reproduces OCCT. The analytic `C'` is used only to size `dt` and (optionally) to sanity-orient the chord.
> - `Parameter(V,E,face)` = the pcurve domain endpoint corresponding to that vertex (`t0` if vertex is the trim start, `t1` if end; swap when `trim.reversed`).

---

## 3. The Path / wire walk + `ClockWiseAngle` + emission

### 3a. `ClockWiseAngle(angleIn, angleOut)` — the leftmost rule

`WS1.cxx:611-649`. Returns the angle in `(0, 2π]` swept **clockwise** from the reverse-of-incoming direction to the candidate outgoing direction:

```
ClockWiseAngle(angleIn, angleOut):                            // WS1.cxx:611
    AIn  = angleIn  mod 2π
    AOut = angleOut mod 2π
    A1   = (AIn + π) mod 2π        // reverse of the arrival heading: points BACK along the in-edge
    A2   = AOut                    // the leaving heading of the candidate
    dA   = A1 - A2
    if dA <= 0:        dA += 2π                               // fold into (0, 2π]
    else if dA <= 1e-14: dA = 2π   // candidate == reverse-incoming (U-turn on same edge) => last resort
    return dA
```

**Selection rule** (`WS1.cxx:585-589`): among all unpassed outgoing edges at the arrival vertex, pick the one with **minimum `dA`**. Minimum `dA` = the first edge you meet sweeping **clockwise** starting from `A1` (the back-pointing direction). Geometrically this is the **most counterclockwise turn relative to forward travel** = "keep the face interior on your left" = leftmost rule. It traces minimal regions; outer loops come out CCW, holes CW (later separated by `FClass2d`, `BF.cxx:438-456`).

Guards on candidates:
- the same edge you came in on gets angle `2π` (`WS1.cxx:554-556`) — never chosen unless forced.
- if only one way out (`iCnt==1`), take it unconditionally (`WS1.cxx:547-552`).
- seam vertices (`bIsClosed`): a candidate whose 2D start point is farther than `2·Tolerance2D` from the arrival 2D point `Pb` is skipped — keeps you on the correct side of a seam (`WS1.cxx:561-571`).

### 3b. The walk

`WS1.cxx:350-607`. Driver calls `Path` from **every unpassed Out edge-end** (`WS1.cxx:323-345`).

```
Path(start_vertex Va, start_out_edge Eouta, start_info):      // WS1.cxx:350
    LS=[]; VertVa=[]; CoordVa=[]; InfoSeq=[]      // parallel stacks: edge, vertex, UV-pt, info*
    loop forever:
        // don't escape back out the very edge we entered the block on
        if len(LS)==1 and LS[0].IsSame(Eouta): return         // WS1.cxx:385-393

        info.passed = True                                    // WS1.cxx:395 (mark used)
        LS.append(Eouta); VertVa.append(Va); InfoSeq.append(info)
        Pa = Coord2d(Va_forward, Eouta, face); CoordVa.append(Pa)
        Vb = otherVertex(Eouta, Va)                           // WS1.cxx:405
        Pb = Coord2d(Vb, Eouta, face)
        aLEInfo = SmartMap[Vb]
        tol2d  = 2*Tolerance2D(Vb); tol2d2 = tol2d²
        bIsClosed = VertMap[Vb]

        // ---- CLOSURE TEST: did we return to an earlier vertex? ----
        bHasEdge=False
        for i = len(LS) down to 1:                            // WS1.cxx:420
            buf.prepend(LS[i])
            if not bHasEdge:
                bHasEdge = not Degenerate(LS[i]); if not bHasEdge: continue
            sameV = VertVa[i].IsSame(Vb)
            sameV2d = sameV
            if sameV and bIsClosed:                           // seam: also require 2D coincidence
                sameV2d = (CoordVa[i].sqDist(Pb) < tol2d2)
                         and |ΔU|<2·Utol and |ΔV|<2·Vtol      // WS1.cxx:443-456
            if sameV and sameV2d:
                if not (buf has exactly 2 identical edges):   // reject degenerate 2-edge loop
                    W = MakeWire(buf)                         // EMIT closed wire (WS.lxx:87)
                    aCB.loops.append(W)                       // WS1.cxx:471-476
                truncate LS,VertVa,CoordVa,InfoSeq to length i-1   // pop emitted loop
                if i-1 < 1: clear all; return
                Eouta = LS.last; info = InfoSeq.last          // resume from before the loop
                break

        // ---- CHOOSE NEXT OUTGOING EDGE ----
        angleIn = AngleIn(Eouta, aLEInfo)        // stored angle of the in-edge  (WS1.cxx:714)
        iCnt    = NbWaysOut(aLEInfo)             // # unpassed out-edges          (WS1.cxx:691)
        isBoundary = not info.isInside           // did we arrive on a boundary edge?
        nWaysInside = 0; onlyWayIn = null; minA = +inf; pick = null

        for each ei in aLEInfo:                                // WS1.cxx:527
            if ei.isIn or ei.passed: continue
            if iCnt==0: return                  // dead end
            if iCnt==1: pick=ei; break          // forced
            if ei.edge.IsSame(Eouta): a = 2π
            else:
                if bIsClosed and Coord2dVf(ei.edge).sqDist(Pb) > tol2d2: continue  // wrong seam side
                a = ClockWiseAngle(angleIn, ei.angle)
            if isBoundary and ei.isInside:                    // arrived on boundary, this is a section edge
                nWaysInside++; onlyWayIn = ei
            if a < minA - eps: minA = a; pick = ei            // leftmost

        if nWaysInside == 1: pick = onlyWayIn                 // *** SECTION-FOLLOW RULE *** WS1.cxx:592-595
        if pick == null: return

        Va = Vb; Eouta = pick.edge; info = pick               // advance
```

**The two rules that make the split watertight and correct:**

1. **`nWaysInside==1` override (`WS1.cxx:592-595`):** when you arrive at a vertex along a **boundary** edge and there is **exactly one section edge** leaving it, you *must* take the section edge regardless of angle. This guarantees the section curve is walked into and the face is actually cut (rather than the walk skating along the boundary), and that the same section edge gets consumed from each side — the foundation of watertightness.

2. **Closure pop + resume (`WS1.cxx:460-512`):** when the walk revisits a vertex, only the **tail** sub-loop is emitted as a wire and popped; the walk **resumes from the remaining stack head**. One `Path` invocation can thus emit several nested wires. Every edge-end is marked `passed` exactly once, so each directed edge is used by exactly one wire — sections (2 directed copies) end up in two different wires, one per side.

`MakeWire` (`WS.lxx:87-98`) just builds a `TopoDS_Wire` from the edge list and stamps `Closed`.

> **NURBS mapping.** `Coord2d(V,E,face)` = pcurve UV value at the vertex param (use the pcurve `NurbsCurve` endpoint, honoring `reversed`). `Coord2dVf(E)` = UV of the FORWARD vertex of E (`WS1.cxx:668`). `otherVertex` from `BRepEdge.start_vertex/end_vertex`. `Degenerate` ⇒ a trim of zero 3D length (seam-collapse); you mostly won't have these for freeform sections. `bIsClosed`/seam handling only matters on periodic surfaces (cylinder/sphere) — there the 2D coincidence + `|ΔU|,|ΔV|` gate (`WS1.cxx:443-456`) prevents stitching across the seam, exactly the cyl-cyl case in task #19.

---

## 4. `RefineAngles` — tangent/convergent pcurves at a node

`WS1.cxx:893-1114`. Only fires at vertices that lie **on the face boundary** with exactly **two boundary edge-ends** (one In `aA2`, one Out `aA1`) plus some interior/section edges (`iCntBnd==2` gate, `WS1.cxx:954`). The straight-tangent `Angle2D` can mis-order a section pcurve that is **tangent or convergent** to the boundary at the node; this re-derives the angle from the curve's actual geometry a hair further in.

```
RefineAngles(V, face, edgeList):                              // WS1.cxx:914
    find aA1 = angle of boundary OUT edge, aA2 = angle of boundary IN edge
    if count(boundary edges) != 2: return
    delta = ClockWiseAngle(aA2, aA1)         // interior angular sector at V (in→out)

    for each interior OUT edge ei (not boundary, not in):
        dA = ClockWiseAngle(aA2, ei.angle)
        if dA < delta: continue              // already inside the sector: fine
        // it's outside/tangent -> recompute
        ok, newA = RefineAngle2D(V, ei.edge, face, aA1, aA2, delta)   // WS1.cxx:1022
        if ok: record newA
        else if (#interior==2):              // clamp just inside the sector
            newA = (newA <= aA1) ? aA1+AngularTol : aA2-AngularTol
            record newA
    apply recorded angles; if edge isIn add π   // WS1.cxx:1011-1016

RefineAngle2D(V,E,face, aA1,aA2,delta):                       // WS1.cxx:1022
    C2d = pcurve(E); tV = Parameter(V,E,face); Pv = C2d.D0(tV)
    tOp = far end param (the t1/t0 away from tV)
    MaxDT = 0.3*(t2-t1)
    for dir_angle in [aA1, aA2+π]:                            // both boundary directions
        L = 2D line through Pv with direction (cos,sin)(dir_angle)
        hits = Geom2dInt_GInter(C2d over [t1,t2], L)          // 2D curve–line intersect
        choose hit with max paramOnLine, |t_onCurve - tV| < MaxDT
        if found at tHit:
            t = tHit + 0.01*(tOp - tHit)     // step 1% along curve toward far end
            P = C2d.D0(t); a = Angle(unit(Pv->P))
            if ClockWiseAngle(aA2, a) < delta: return (true, a)   // now inside sector
    return (false, _)
```

> **NURBS mapping.** Replace `Geom2dInt_GInter(pcurve, line)` with a 2D NURBS-curve ∩ line solve in UV: sample the pcurve into a UV polyline (you already do this in `split_by_uv_curves`, `nurbssurface_trimmed.cpp:585-620`), find the segment crossing the boundary-direction line through `Pv`, refine with 2D Newton on `f(t)=cross(C2d(t)−Pv, dir)`. Then the 1%-step-along-curve + recompute-angle is identical. This step is **second-order**: for transversal section curves the `dA < delta` test skips it. Implement the main walk first; add `RefineAngles` only if cyl-cyl/tangent datasets misorder.

---

## 5. Complete `split_face_by_wires` (NURBS, session-ready)

```
# INPUT: face F (surface S + its existing boundary trims as pcurves in UV),
#        section_pcurves[]  (UV NurbsCurves from Intersection::surface_surface,
#                            already seam-split to lie inside S's UV domain),
#        tolerance3d
# OUTPUT: list of closed UV wires; each is a candidate region of F
#         (later -> trimmed face; growth/hole sorted by point-in-2D classify)

split_face_by_wires(F, section_pcurves, tol3d):

  # ---------- 0. assemble directed edges + vertices ----------
  uv_to_3d  = local 3D length per unit UV at S mid-domain            # trimmed.cpp:557
  tol2d     = max(tol3d/uv_to_3d, snap_floor)
  Vmap      = SpatialHash<UVpoint -> vertex_id>(snap = tol2d)
  edges     = []     # each: {id, pcurve, v_start, v_end, is_section}

  for trim in F.boundary_trims:                       # feed boundary ONCE
     pc = trim.pcurve (apply trim.reversed)
     vs = Vmap.intern(pc.start_uv); ve = Vmap.intern(pc.end_uv)
     edges += {pc, vs, ve, is_section=False}

  for sc in section_pcurves:                          # feed section TWICE (both dirs)
     vs = Vmap.intern(sc.start_uv); ve = Vmap.intern(sc.end_uv)
     edges += {sc,             vs, ve, is_section=True}     # forward
     edges += {sc.reversed(),  ve, vs, is_section=True}     # reversed (shared 3D edge id!)

  # ---------- 1. SmartMap: vertex -> list of edge-ends ----------  (§1)
  SmartMap = map<vid -> list<EdgeInfo>>
  for e in edges:
     for (V, isStart) in [(e.v_start,True),(e.v_end,False)]:
         ei = EdgeInfo(edge=e, inFlag = not isStart,    # start=OUT, end=IN
                       isInside = e.is_section, passed=False, angle=NaN)
         SmartMap[V].append(ei)

  # fast path: every vertex 1-in/1-out AND no edge duplicated -> single wire (§1)
  if regular(SmartMap): return [ MakeWire(all edges) ]

  # ---------- 2. angles ----------  (§2)
  for V in SmartMap:
     for ei in SmartMap[V]:
         ei.angle = Angle2D_NURBS(V, ei.edge, ei.inFlag, S, tol2d)
  RefineAngles(F, SmartMap)            # (§4) optional; needed for tangent/seam nodes

  # ---------- 3. walk ----------  (§3)
  wires = []
  for V in SmartMap:
     for ei in SmartMap[V]:
         if (not ei.inFlag) and (not ei.passed):       # every unpassed OUT end
             Path(V, ei.edge, ei, SmartMap, wires, tol2d, VertMap)
  return wires


Angle2D_NURBS(V, E, bIsIN, S, tol2d):                 # cv-based tangent (§2)
  C  = E.pcurve;  (t0,t1) = C.domain();  tV = (V is start)? t0 : t1
  d1 = C.evaluate(tV,1)[1]                             # analytic UV derivative
  speed = hypot(d1.x,d1.y)
  dt = max(tol2d/max(speed,eps), PConfusion)
  kappa = curvature2d(C, tV)                           # |x'y''-y'x''|/speed^3
  if kappa > PConfusion:
      R = 1/kappa;  dt = max(dt, acos(R/(R+tol2d)))
  aTX = 0.05*(t1-t0); if aTX<5e-5: aTX=min(5e-5,(t1-t0)/2)
  dt = min(dt, aTX)
  tV1 = (|tV-t0| < |tV-t1|) ? tV+dt : tV-dt
  Pv = C.point_at(tV); Pv1 = C.point_at(tV1)
  vec = bIsIN ? (Pv1 - Pv) : (Pv - Pv1)                # direction of travel
  return atan2(vec.y, vec.x) wrapped to [0,2π)


Path(Va, Eouta, info, SmartMap, wires, tol2d, VertMap):   # (§3b)
  LS=[];VertVa=[];CoordVa=[];InfoSeq=[]
  while True:
     if len(LS)==1 and LS[0].id==Eouta.id: return
     info.passed=True
     LS+=Eouta; VertVa+=Va; InfoSeq+=info
     Pa = Eouta.uv_at(Va); CoordVa+=Pa
     Vb = Eouta.otherVertex(Va); Pb = Eouta.uv_at(Vb)
     L  = SmartMap[Vb]; tol2d2=(2*tol2d)^2; bClosed=VertMap[Vb]

     # closure pop + resume
     buf=[]; hasEdge=False
     for i in reversed(range(len(LS))):
        buf.prepend(LS[i])
        if not hasEdge: hasEdge = not degenerate(LS[i]); 
        if not hasEdge: continue
        same = (VertVa[i].id==Vb.id)
        if same and bClosed: same = CoordVa[i].sqDist(Pb)<tol2d2   # seam guard
        if same:
            if not (len(buf)==2 and buf[0].id==buf[1].id):
                wires += MakeWire(buf)
            truncate LS,VertVa,CoordVa,InfoSeq to i      # pop
            if i<1: return
            Eouta=LS[-1]; info=InfoSeq[-1]; break

     # choose next out-edge: leftmost, with section-follow override
     angleIn=AngleIn(Eouta,L); iCnt=nWaysOut(L)
     isBoundary = not info.isInside
     nIn=0; onlyIn=null; minA=+inf; pick=null
     for ei in L:
        if ei.inFlag or ei.passed: continue
        if iCnt==1: pick=ei; break
        if ei.edge.id==Eouta.id: a=2π
        elif bClosed and ei.edge.uv_forward().sqDist(Pb)>tol2d2: continue
        else: a = ClockWiseAngle(angleIn, ei.angle)
        if isBoundary and ei.isInside: nIn++; onlyIn=ei
        if a < minA-eps: minA=a; pick=ei
     if nIn==1: pick=onlyIn                # *** follow the section curve ***
     if pick==null: return
     Va=Vb; Eouta=pick.edge; info=pick
```

After this, each returned wire becomes a trimmed-face candidate; classify outer-vs-hole by sampling a UV midpoint of one wire edge and testing point-in-2D against the other wires (OCCT's `IntTools_FClass2d`, `BF.cxx:438-456` / `BF.cxx:812-858`), nest holes into growths (`BF.cxx:483-547`), then in 3D keep/drop regions by your existing `BRep::contains_point` classification and select per fuse/cut/common.

---

## Key correctness notes vs the current session

- **Over-fragmentation cause:** `split_by_uv_curves` (`nurbssurface_trimmed.cpp:534`) is a pure UV arrangement with no In/Out edge-end accounting and no leftmost angular walk, so it emits every cell. WireSplitter emits only the **minimal directed loops**, and each directed edge is consumed exactly once — sections naturally land in exactly two wires (one per side).
- **Watertightness comes for free** because the section pcurve is one shared object fed with both orientations (`§0`, `§1`), and the `nWaysInside==1` rule (`WS1.cxx:592-595`) forces the walk through it from each side. No post-hoc co-splitting, which is what corrupted the 9%-volume solid.
- **`isInside`/section flag** (`WS1.cxx:301`, via `aMS` parity) is the single most important bit to port: it distinguishes section edges (must be traversed, appear twice) from boundary edges (appear once).
- **Angle convention is fixed:** `Angle` ∈ [0,2π) CCW from +U (`WS1.cxx:834`); `ClockWiseAngle` reverses the incoming by +π then takes the clockwise sweep in (0,2π] (`WS1.cxx:611`); **minimum** of that = leftmost / interior-on-left turn (`WS1.cxx:585`).
- **Periodic surfaces (cyl-cyl, task #19):** the seam guards at `WS1.cxx:443-456` and `WS1.cxx:561-571` (2D coincidence + `|ΔU|,|ΔV|` tolerances) are mandatory — without them the walk stitches across the U-seam.

Relevant session files to edit: `C:/pc/3_code/code_rust/session/session_cpp/src/nurbssurface_trimmed.cpp` (replace/augment `split_by_uv_curves` at line 534 with this walk), `C:/pc/3_code/code_rust/session/session_cpp/src/brep.cpp` (line 1767 caller, and the boolean imprint→classify path), using `NurbsCurve::evaluate` (`nurbscurve.cpp:2084`) for cv-based UV tangents and `Intersection::surface_surface` (`intersection.h:489`) to obtain the shared section pcurves on both operands.