I have everything needed. Note the file lives under `ModelingData/TKGeomBase`, not `TKGeomAlgo` — I'll cite real paths. Below is the implementation-depth deliverable.

---

# OCCT analytic quadric–quadric intersection → session NURBS SSI

Paths (abbreviated below):
- **QQG** = `…/occt/src/ModelingData/TKGeomBase/IntAna/IntAna_QuadQuadGeo.cxx`
- **III** = `…/occt/src/ModelingAlgorithms/TKGeomAlgo/IntPatch/IntPatch_ImpImpIntersection.cxx`
- **FF** = `…/occt/src/ModelingAlgorithms/TKBO/IntTools/IntTools_FaceFace.cxx`
- **SES** = `C:/pc/3_code/code_rust/session/session_cpp/src/intersection.cpp`

---

## 0. The OCCT three-layer architecture (so you map to the right layer)

OCCT splits quadric×quadric into THREE classes; only the first is "closed-form conics":

1. **`IntAna_QuadQuadGeo`** (QQG) — *geometric* recogniser. Returns one of `IntAna_ResultType` = `{Empty, Same, Point, Line, Circle, Ellipse, Parabola, Hyperbola, PointAndCircle, NoGeometricSolution}` (`IntAna_ResultType.hxx`). It only succeeds for **special configs** (coaxial / parallel / perpendicular-equal / tangent / shared-apex). For everything else it returns **`IntAna_NoGeometricSolution`**. This is the layer your `ssi_*` routines mirror.
2. **`IntAna_IntQuadQuad`** — *algebraic* quartic solver producing `IntAna_Curve` (parametric quartic branches). OCCT calls this when QQG returns `NoGeometricSolution` for cyl-sphere / cyl-cone / cone-cone / cone-sphere (III:8126 `IntAna_IntQuadQuad anaint(Cy,Sp,Tol)` → `IntPatch_ALine`). **This is what your marcher replaces.**
3. **`ComputationMethods` / `WorkWithBoundaries` (WLine walking)** inside III — used ONLY for cyl-cyl general (III:7799-7822) and for the implicit-parametric path. Also marcher territory.

The dispatch in III::Perform keys on a 2-digit type code `iTT = iT1*10+iT2` (III:2547) with `1=Plane 2=Cyl 3=Cone 4=Sphere 5=Torus` (III:2549-2740). FaceFace (FF::Perform, FF:328) decides plane-plane special (FF:393 `PerformPlanes`), plane-quad analytic (FF:438/450), else hands to `IntPatch_Intersection` (FF:521-525) with `isGeomInt = isTreatAnalityc(...)` (FF:255-322) — which forces walking when a plane/cyl ellipse is too eccentric (`aMajorR < 100000*aMinorR`, FF:316).

---

## 1. The coaxiality / axis primitive: `AxeOperator` (QQG:69-230)

Every degenerate quadric-quadric branch is gated by `AxeOperator` on the two `gp_Ax1` axes. You MUST port this once; all new routines call it. Constructor QQG:124-207, accessors QQG:79-91:

```
AxeOperator(Ax1 A1, Ax1 A2, epsD=1e-14, epsPar=Precision::Angular()=1e-12):
  V1=unit(A1.dir); V2=unit(A2.dir); P1=A1.loc; P2=A2.loc   // RefineDir snaps near-axis dirs, QQG:2887
  parallel = |V1×V2| ~ 0   i.e. V1.IsParallel(V2, epsPar)            (QQG:145)
  perp = cross(V1,V2)
  if parallel: distance = distance(line(A1), P2)                     (QQG:150-151)
  else:        distance = |unit(perp) · (P2-P1)|   // common-perp dist(QQG:155)
  coplanar = (distance < epsD) AND |det33(V1,V2,P1-P2)| <= epsD       (QQG:159-174)
  normal   = |V1·V2| < epsPar                                         (QQG:176)
  if coplanar && !parallel: ptintersect = solve P1+A*V1 (QQG:179-201)
Same()      = parallel && distance < epsD          // truly coaxial   (QQG:83)
Intersect() = coplanar && !parallel                // axes cross      (QQG:87)
Normal()    = |V1·V2| < epsPar                     // perpendicular   (QQG:91)
PtIntersect()                                                          (QQG:79)
Distance(d,p1,p2)  // signed gap + foot params, used for cyl-cyl point (QQG:211-230)
```

Tolerances (QQG::InitTolerances, QQG:348-356) — use these EXACT constants in the session:
```
EPSILON_DISTANCE              = 1e-14      // axis gap / coplanarity
EPSILON_ANGLE_CONE            = 1e-12      // = Precision::Angular()
EPSILON_AXES_PARA             = 1e-12      // = Precision::Angular(), parallel test
EPSILON_MINI_CIRCLE_RADIUS    = 1e-9       // 0.01*Confusion → circle degenerates to point
EPSILON_CYLINDER_DELTA_RADIUS = 1e-13      // RELATIVE |R1-R2|/Rmax for cyl-cyl ellipse
EPSILON_CYLINDER_DELTA_DISTANCE = 1e-7     // = Precision::Confusion()
```
The session already has the geometry helpers to build this: `ssi_dot/ssi_cross/ssi_unit` (SES:2126-2133), `ortho_basis` (SES:2136), `exact_circle` (SES:2152), `exact_ellipse` (SES:2171), `line_cone` (SES:2520). Add a `struct Axis{V3 loc, dir;}` + `AxeOp` free function returning `{parallel,coplanar,normal,distance,ptIntersect}`.

---

## 2. The 15 unordered pairs — type, degenerate cases, closed-form

For each: **general type** | **degenerate (canonical centered/coaxial) case** | **OCCT method (QQG line)**.

### (1) plane–plane — QQG::Perform(Pln,Pln) QQG:386-509
- General: **1 Line** (`dir1 = n1×n2`, anchor at QQG:430-440; refined via `IntAna_IntConicQuad` when angle tiny, QQG:467-505).
- Degenerate: normals collinear (`|n1×n2| ≤ TolAng`) → **Same** (if both signed dists ≤ Tol) or **Empty** (QQG:410-414).
- Session: `ssi_plane_plane` ✓ (SES:2631) — clips line to BOTH UV rects; `empty` flag = recognised-but-disjoint.

### (2) plane–sphere — QQG::Perform(Pln,Sph) QQG:965-1004
- General: **Circle**. center = projection of sphere centre onto plane; `param1 = √(r²−d²)`, `dir1=plane normal`, `dir2=plane XDir` (QQG:990-1001).
- Degenerate: `||d|−r| < Epsilon(r)` (tangent) → **Point** (QQG:982-989).
- Session: `ssi_plane_sphere` ✓ (SES:2540). *Gap:* it returns false on tangency (`|d|≥r`, SES:2544) → marcher; OCCT emits a point. Minor.

### (3) plane–cylinder — QQG::Perform(Pln,Cyl,Tolang,Tol,H) QQG:540-707
- General (plane not ∥ axis): **Ellipse**, `param1 = r/|cosθ|` (major), `param1bis = r` (minor) (QQG:690-703); if `sint < Tol/r` (axis ⟂ plane) → **Circle** r (QQG:680-689).
- Degenerate (axis ∥ plane, `IntAna_IntConicQuad::IsParallel`, QQG:589): **Line(s)** — `||d|−r|<Tol` → **1 tangent line** (QQG:597-620); `|d|<r` → **2 lines** at `±√(r²−d²)` along `axis×n` (QQG:621-659); else **Empty** (QQG:665-668).
- Session: `ssi_plane_cylinder` ✓ (SES:2552). *Gap:* `|w·n|<1e-7` (axis ∥ plane) → returns false → marcher (SES:2556). **Add the 1/2-line branch** so a plane cutting a cylinder lengthwise stays exact.

### (4) plane–cone — QQG::Perform(Pln,Cone,Tolang,Tol) QQG:737-938
Richest conic case. apex signed dist `dist`; `cost=|axis·n|`, `sint=|axis×n|`, `costa = cost·cosα − sint·sinα` (= sin of (plane,generatrix) angle, QQG:781).
- Apex ON plane (`|dist|<Tol`, QQG:784): `|costa|<Tolang` → **1 Line** (plane ∥ generatrix, QQG:790-801); `cost<sinα` → **2 Lines** through apex (QQG:802-811); else **Point** (apex) (QQG:812-817).
- Apex OFF plane: `cost<Tolang` (plane contains axis) → **Hyperbola** ×2 branches (QQG:825-837); `|costa|<Tolang` → **Parabola** (QQG:857-867); `sint<Tolang` (plane ⟂ axis) → **Circle** r=`dist·tanα` (QQG:868-876); `cost<sinα` → **Hyperbola** (QQG:877-890); else (`cost>sinα`) → **Ellipse** (QQG:891-903). (Ellipse/hyperbola param-magnitude guards QQG:910-935.)
- Session: `ssi_plane_cone` ✓ (SES:2571) handles ⟂-axis **Circle** (SES:2575) and the **Ellipse** (via two `line_cone` chord solves, SES:2591-2602). *Gaps:* parabola, hyperbola, and the apex-on-plane line/point cases all return false → marcher. For booleans the ellipse/circle cases are the common ones; add parabola+hyperbola+apex-lines for full parity.

### (5) plane–torus — QQG::Perform(Pln,Tor,Tol) QQG:2170-2256
Only ring torus (`RMin<RMaj`, else `NoGeometricSolution`, QQG:2178). Two analytic configs:
- **plane ⟂ axis** (`bParallel`, axis ∥ plane-normal, QQG:2200): **1 or 2 concentric Circles** radius `RMaj ± √(RMin²−d²)` centred on axis at the plane (QQG:2222-2235).
- **plane contains axis** (`bNormal`, axis ⟂ plane-normal AND axis through plane, QQG:2238): **2 Circles** radius `RMin`, centres at `torLoc ± RMaj·(axis×n)` (QQG:2247-2254) — the two tube cross-sections.
- Oblique → `NoGeometricSolution` (QQG:2191-2195) → quartic (incl. Villarceau). OCCT does NOT do Villarceau closed-form.
- Session: `ssi_plane_torus` ✓ (SES:2610) handles ONLY the ⟂-axis concentric-circle case (`||w·n|−1|>1e-7 → false`, SES:2614). **Add the plane-contains-axis branch → 2 minor-radius circles** (detect `|w·n| < 1e-7`).

### (6) sphere–sphere — QQG::Perform(Sph,Sph,Tol) QQG:2039-2139
- General: **Circle**. `Alpha=½(R1²−R2²+d²)/d`, `Beta=√(R1²−Alpha²)`, center `O1+Alpha·dir`, `param1=Beta`, `dir1=O1→O2` (QQG:2118-2133).
- Degenerate: coincident+equal → **Same** (QQG:2064); external/internal tangent (`0≤t≤Tol`) or `Beta≤EPS_MINI` → **Point** (QQG:2086-2096, 2122-2126); disjoint/contained → **Empty** (QQG:2108-2110).
- Session: ✓ inline in `analytic_ssi` (SES:2967-2982). *Gap:* no tangent→point; emits nothing when `dist∉(|R1−R2|,R1+R2)`. Fine for booleans.

### (7) sphere–cylinder — QQG::Perform(Cyl,Sph,Tol) QQG:1364-1396  → III::IntCySp:7967
- Closed-form ONLY when **coaxial**: `A1A2.Intersect() && sphereCentre on axis`, or `A1A2.Same()` (QQG:1369). Then `Rsph<Rcyl` → **Empty** (QQG:1371); else **1 or 2 Circles** radius `Rcyl` at axial offset `±√(Rsph²−Rcyl²)` (QQG:1377-1389). (`dist≈0` → tangent: single circle, QQG:1384 guard.)
- Non-coaxial → `NoGeometricSolution` → OCCT quartic ALine (III:8123-8222).
- Session: **MISSING.** Add `ssi_sphere_cylinder`.

### (8) sphere–cone — QQG::Perform(Sph,Cone,Tol) QQG:1920-2008  → III::IntCoSp
- Closed-form ONLY when **coaxial** (QQG:1929, same Intersect/Same test). Solve quadratic in `x` along the axis: `(1+t²)x² + 2t²·d·x + (−R² + d²t²) = 0` with `t=tanα`, `d=dist(apex,centre)` via `math_DirectPolynomialRoots` (QQG:1949-1952). 0/1/2 roots → **0/1/2 Circles**, radius `|t·(d+x)|`, centre `apex+(d+x)·coneDir` (QQG:1962-1996). Radius ≤ `EPS_MINI` → **PointAndCircle** (QQG:1974, 1991).
- Non-coaxial → `NoGeometricSolution`.
- Session: **MISSING.** Add `ssi_sphere_cone` — you already have the quadratic-root pattern in `line_cone` (SES:2520); reuse a 1-D `math_DirectPolynomialRoots` equivalent.

### (9) cylinder–cylinder — QQG::Perform(Cyl,Cyl,Tol) QQG:1035-1282  → III::IntCyCy:7756 / CyCyAnalyticalIntersect:4815
- **Parallel axes** (`A1A2.Parallel()`, QQG:1057): coincident → **Same**/**Empty** (QQG:1059-1069); else circle-circle in the base plane (QQG:1085-1204): `d>R1+R2`→Empty; `d==R1+R2`→**1 tangent Line** (QQG:1091-1102); `d>|R1−R2|`→**2 Lines** (QQG:1103-1184); `d≈|R1−R2|`→**1 tangent Line** internal (QQG:1185-1199); else Empty.
- **Perpendicular + equal radius + intersecting** (`RmR_Relative ≤ 1e-13 && A1A2.Intersect()`, QQG:1209): **2 Ellipses** through the axis-cross point; `dir1=d1+d2`, `dir2=d1−d2`, major `R/|sin(½A)|`, minor `R/|sin(½(π−A))|`, both with `param*bis=R` (QQG:1213-1250). (Equal radius coaxial collapses → **Same**, QQG:1224-1228.)
- `|d−(R1+R2)| < Tol` non-parallel → **Point** (external tangent, QQG:1254-1275).
- Else → **`NoGeometricSolution`** → cyl-cyl WLine walking (III:7799+). The general two-cylinder curve is a quartic; OCCT does NOT give it closed-form. (`CyCyAnalyticalIntersect` consumes only Line/Ellipse/Point/Same/Empty, III:4841-5146.)
- Session: **MISSING.** Add `ssi_cylinder_cylinder` for parallel-lines + perp-equal-ellipse + coaxial-Same; leave the general quartic to the (now branch-complete) marcher.

### (10) cylinder–cone — QQG::Perform(Cyl,Cone,Tol) QQG:1313-1333  → III::IntCyCo:8234
- Closed-form ONLY when **coaxial** (`A1A2.Same()`, QQG:1317): **2 Circles** radius `Rcyl` at axial offset `± Rcyl/tanα` from apex (QQG:1318-1327).
- Else → `NoGeometricSolution` → quartic ALine.
- Session: **MISSING.** Add `ssi_cylinder_cone`.

### (11) cylinder–torus — QQG::Perform(Cyl,Tor,Tol) QQG:2287-2341  → III::IntCyTo
- Ring torus + **coaxial** required (axis ∥, axis through cyl loc, QQG:2309). **1 or 2 Circles** radius `Rcyl` at axial offset `±√(RMin²−(Rcyl−RMaj)²)` (QQG:2325-2340). Empty if `Rcyl` outside `[RMaj−RMin, RMaj+RMin]` (QQG:2319-2323). Single circle when `Rcyl == RMaj±RMin` (tangent, QQG:2334 guard).
- Else → `NoGeometricSolution` (quartic).
- Session: **MISSING.** Add `ssi_cylinder_torus`.

### (12) cone–cone — QQG::Perform(Cone,Cone,Tol) QQG:1428-1889  → III::IntCoCo:8553
Four analytic branches:
- **Same axis** (`A1A2.Same()`, QQG:1473): unequal half-angles → **2 Circles** at `x=d·tg2/(tg1±tg2)` (QQG:1481-1500); equal half-angles → **Same** (apices coincide) or **1 Circle** (QQG:1501-1516).
- **Parallel axes + equal angle** (QQG:1519): reduce to a plane-cone cut → Ellipse/Circle/Hyperbola/Line (QQG:1550-1600).
- **Coincident apices** (`aDA1A2 < Tol²`, QQG:1603): empty / **1 Line** (tangent cones) / **2 Lines** (crossing) via a 2-D apex analysis + plane-plane (QQG:1664-1756). This is the "shared apex → lines" canonical case.
- **Common generatrix / intersecting axes** (`A1A2.Intersect()`, QQG:1759): sets `myCommonGen`, reduces to plane-cone → Ellipse/Circle/Parabola/Hyperbola (QQG:1831-1882).
- Else → `NoGeometricSolution`.
- Session: **MISSING.** For canonical coaxial booleans you need the **Same-axis → circles** and **shared-apex → lines** branches; the rest can defer to marcher.

### (13) cone–torus — QQG::Perform(Cone,Tor,Tol) QQG:2372-2484  → III::IntCoTo
- Ring torus + **coaxial** (axis ∥, apex on axis, QQG:2392). Up to **4 Circles**: rotate the torus axis line by ±α about the tube, intersect with each tube-circle, distance test `aDist ≤ RMin+Tol` (QQG:2416-2450). `pt/dir/param` filled into the 4 slots (QQG:2452-2483).
- Else → `NoGeometricSolution`.
- Session: **MISSING.** Add `ssi_cone_torus` (returns up to 4 circles).

### (14) sphere–torus — QQG::Perform(Sph,Tor,Tol) QQG:2515-2580  → III::IntSpTo
- Ring torus + **sphere centre on torus axis** (`lin(axis).Distance(sphCentre) < EPS_DIST`, QQG:2533). Treat as circle-circle between the sphere meridian and the tube circle at `torLoc + RMaj·xDir`: `Alpha=½(RMin²−Rsph²+d²)/d`, `Beta=√(RMin²−Alpha²)`; **1 or 2 Circles** (QQG:2554-2579).
- Else → `NoGeometricSolution`.
- Session: **MISSING.** Add `ssi_sphere_torus`.

### (15) torus–torus — QQG::Perform(Tor,Tor,Tol) QQG:2611-2690  → III::IntToTo
- **Coaxial** required (axis ∥, axis through loc2, QQG:2631). Coincident equal → **Same** (QQG:2637). Else circle-circle between the two tube circles at `loc_i+RMaj_i·xDir`: `Alpha=½(RMin1²−RMin2²+d²)/d`, `Beta=√(RMin1²−Alpha²)`; **1 or 2 Circles** (QQG:2649-2689).
- Else → `NoGeometricSolution`.
- Session: **MISSING.** Add `ssi_torus_torus`.

**Note on Viviani / sphere-cyl r=R/2:** OCCT does NOT special-case Viviani. Sphere∩cylinder is closed-form ONLY when coaxial (case 8); the classic Viviani config (cylinder of radius R/2 *tangent internally*, axis offset by R/2, NOT coaxial) returns `NoGeometricSolution` → quartic ALine. So in the session it stays on the marcher — but the marcher must find the single self-crossing figure-eight branch (the "find all branches" fix matters here).

---

## 3. New session routines to add (signatures, pseudo-code, dispatch)

All mirror the existing `static bool ssi_*(const RecogSurface&…, NurbsCurve&/vector<NurbsCurve>&)` pattern (SES:2540-2624) and return `false` when not analytically handled so the dispatcher falls to the marcher (`analytic_ssi`, SES:2937-3001). Add a tiny `AxeOp` first.

```cpp
struct AxeRel { bool parallel, coplanar, normal; double distance; V3 ptInt; bool ptValid; };
static AxeRel axe_op(const V3& l1,const V3& d1raw,const V3& l2,const V3& d2raw){
  V3 v1=ssi_unit(d1raw), v2=ssi_unit(d2raw);          // (QQG:139-140 RefineDir optional)
  V3 perp=ssi_cross(v1,v2); double pl=std::sqrt(ssi_dot(perp,perp));
  AxeRel r{}; r.parallel = pl < 1e-12;                 // EPSILON_AXES_PARA (QQG:145)
  V3 d{l2[0]-l1[0],l2[1]-l1[1],l2[2]-l1[2]};
  if(r.parallel){ V3 c=ssi_cross(v1,d); r.distance=std::sqrt(ssi_dot(c,c)); }
  else r.distance = std::abs(ssi_dot(ssi_unit(perp), d));            // (QQG:151/155)
  // coplanar: dist<epsD AND det[v1;v2;d]~0  (QQG:159-174)
  double det = v1[0]*(v2[1]*d[2]-v2[2]*d[1]) - v2[0]*(v1[1]*d[2]-v1[2]*d[1])
             + d[0]*(v1[1]*v2[2]-v1[2]*v2[1]);
  r.coplanar = (r.distance<1e-14) && (std::abs(det)<=1e-14);
  r.normal   = std::abs(ssi_dot(v1,v2)) < 1e-12;
  if(r.coplanar && !r.parallel){ /* QQG:179-201 closed form for ptInt */ r.ptValid=true; }
  return r;
}
inline bool axe_same(const AxeRel&r){ return r.parallel && r.distance<1e-14; }      // QQG:83
inline bool axe_intersect(const AxeRel&r){ return r.coplanar && !r.parallel; }      // QQG:87
```

**(A) `ssi_sphere_cylinder(sph, cyl, out)`** — QQG:1364
```
A = axe_op(cyl.axis, sph.center→axis);  if(!axe_same(A) && !(axe_intersect(A) && pt_on_axis(sph.center))) return false;
if (sph.r < cyl.r) return true;  // recognised, Empty
dist = sqrt(sph.r² − cyl.r²); proj = project(sph.center onto cyl axis);   // == sph.center if truly coaxial
(xa,ya)=ortho_basis(cyl.dir);
out.push_back(exact_circle(proj ± dist·cyl.dir, xa, ya, cyl.r));   // 1 if dist<RealEpsilon else 2
return true;
```

**(B) `ssi_sphere_cone(sph, cone, out)`** — QQG:1920
```
if (!coaxial(cone.axis, sph.center)) return false;
d = dist(cone.apex, sph.center);  coneDir = (d>eps)? unit(apex→center) : cone.axis;
t = tan(cone.halfAngle);
roots = solveQuadratic(1+t², 2t²d, −sph.r² + d²t²);          // QQG:1950
for x in roots: r = |t·(d+x)|; if(r>EPS_MINI) out.push_back(exact_circle(apex+(d+x)coneDir, ⟂basis, r));
return true;   // even with 0 roots → recognised, no curve
```

**(C) `ssi_cylinder_cylinder(c1, c2, out, &out_lines)`** — QQG:1035 (most branches)
```
A = axe_op(c1.axis, c2.axis); RmR=|R1−R2|; RmRrel=RmR/max(R1,R2);
if (A.parallel){
   if (A.distance<=Tol) { if(RmR<=Tol) SAME; else EMPTY; return true; }
   // base-plane circle∩circle → 0/1/2 LINES (QQG:1085-1204), dir = c1.dir
   build lines exactly as QQG:1091/1103/1185 (use the aCos/aSin rotation, QQG:1141-1182);
   return true;
}
if (RmRrel<=1e-13 && axe_intersect(A)){            // perpendicular-equal → 2 ellipses
   P=A.ptInt; Aang=angle(d1,d2);
   dirMaj=unit(d1+d2); dirMin=unit(d1−d2);
   major=R/|sin(Aang/2)|; minor=R/|sin((π−Aang)/2)|;   // (QQG:1235-1236)
   out.push_back(exact_ellipse(P,dirMaj,dirMin, max(major,R), R));   // semi-axes sorted, QQG:1238-1250
   out.push_back(exact_ellipse(P,dirMin,dirMaj, max(minor,R), R));
   return true;
}
if (|A.distance−(R1+R2)|<Tol) { POINT; return true; }
return false;   // NoGeometricSolution → marcher
```

**(D) `ssi_cylinder_cone`** — QQG:1313: `if(!axe_same) return false;` then 2 circles radius `Rcyl` at `apex ± (Rcyl/tan α)·dir`.

**(E) `ssi_cylinder_torus`** — QQG:2287: ring-torus + coaxial; `dist=√(RMin²−(Rcyl−RMaj)²)`; 1/2 circles radius `Rcyl` at `torLoc ± dist·axis`; Empty if `Rcyl∉[RMaj−RMin,RMaj+RMin]`.

**(F) `ssi_cone_cone`** — QQG:1428: implement at least **Same-axis** (QQG:1473-1516 → circles, with `x=d·tg2/(tg1±tg2)`) and **coincident-apex** (QQG:1603-1756 → 1/2 lines through apex). Defer parallel-equal/common-gen to marcher (return false).

**(G) `ssi_cone_torus`** — QQG:2372: coaxial; up to 4 circles via the rotate-axis-by-±α + tube-circle-distance construction (QQG:2407-2450).

**(H) `ssi_sphere_torus`** — QQG:2515: sphere-centre-on-axis; circle-circle `Alpha/Beta` (QQG:2556-2579); 1/2 circles.

**(I) `ssi_torus_torus`** — QQG:2611: coaxial; Same if coincident+equal; else circle-circle `Alpha/Beta` (QQG:2666-2689); 1/2 circles.

**(J) extend `ssi_plane_torus`** — add the **plane-contains-axis** branch (QQG:2238-2254): detect `|w·n| < 1e-7`, then 2 circles radius `RMin` centred at `torLoc ± RMaj·(axis×n)`.

**(K) extend `ssi_plane_cylinder`** — add the **axis-∥-plane** branch (1/2 lines, QQG:597-668) instead of returning false at SES:2556.

**Dispatch additions** in `analytic_ssi` (SES:2983 `} else { return res; }` is the current dead-end). Replace the `else` with the new pairs, e.g.:
```cpp
else if (pairIs(ra,rb,SPHERE,CYLINDER)) handled = ssi_sphere_cylinder(sphereOf(), cylOf(), c3_list);
else if (pairIs(ra,rb,SPHERE,CONE))     handled = ssi_sphere_cone(...);
else if (pairIs(ra,rb,CYLINDER,CYLINDER))handled = ssi_cylinder_cylinder(...);   // may emit ellipses+lines
… cone_cone, cylinder_cone, cylinder_torus, cone_torus, sphere_torus, torus_torus …
else return res;   // truly unhandled → marcher
```
Keep the tri-state contract (SES:2691): `handled=true` with empty `c3_list` ⇒ `HIT`/`NO_HIT` (recognised, no curve); `handled=false` ⇒ `NOT_ANALYTIC` (marcher). Note `ssi_cylinder_cylinder` returns `false` ONLY for the genuine `NoGeometricSolution` (general skew) so the marcher takes it.

---

## 4. Robust degeneracy detection (per the request)

- **Coaxiality:** ALWAYS via `axe_op` (§1), never raw `dir·dir`. `Same()` = parallel (`|d1×d2|<1e-12`) AND axis gap `<1e-14`. "Centre on axis" = `axe_intersect && dist(point, axisLine) < 1e-14`. The `RefineDir` snap (QQG:2887-2938) — collapse a near-`(1,0,0)` axis to exactly `(1,0,0)` — matters for centred/axis-aligned test geometry; port it before the parallel test or canonical boxes/cylinders will read as "almost parallel".
- **Radius relations:**
  - cyl-cyl ellipse needs RELATIVE equality `|R1−R2|/max(R1,R2) ≤ 1e-13` (QQG:1052/1209), not absolute.
  - sphere-cyl coaxial: `Rsph<Rcyl` ⇒ Empty (QQG:1371); `√(Rsph²−Rcyl²)<RealEpsilon` ⇒ single tangent circle.
  - cyl-torus / sphere-torus / torus-torus: gate on `RMin<RMaj` first (ring torus, QQG:2297/2380/2643); membership `Rcyl∈[RMaj−RMin,RMaj+RMin]`; single circle at the interval endpoints (tangent, QQG:2334).
  - Any computed circle radius `≤ EPSILON_MINI_CIRCLE_RADIUS=1e-9` ⇒ degenerate to a **point** (QQG:1974/2122).
- **Tangency vs 2-curve:** the discriminant sign of the underlying quadratic (`Beta`/`aSin2`/disc) decides 1 vs 2. Tangent = discriminant `≤ Tol²` (QQG:1144 `4R1²·sin² < Tol²`; QQG:2120 `Beta≤EPS_MINI`). Emit ONE circle, not two coincident.

---

## 5. FaceFace dispatch, tangency, closed curves (how OCCT wires the layers)

- **Tangent faces:** if `myIntersector.TangentFaces()` (FF:531), FaceFace returns with NO section curve (FF:532-535) — coincident/tangent surfaces are handled by the same-domain SD path, not by an edge. Your analytic routines must likewise signal "Same" distinctly from "Empty" so the boolean treats it as a shared face, not a section.
- **Closed conic split:** when QQG yields a full **Circle/Ellipse** the GLine is a closed conic; FaceFace/`IntPatch_LineConstructor` splits it at the face's UV restrictions and `GeomInt_IntSS::BuildPCurves` (FF:818/829) builds the 2D pcurves on each surface. Your `analytic_pcurve` (SES:2705) is the NURBS analogue; the closed circle must be split where it crosses the cylinder/sphere **seam** — exactly what `analytic_sphere_pullback` (SES:2811) does (longitude is exact via `atan2`, meridian by bisection, seam crossings cut to land on `u0/u1`). Add the equivalent seam-split for the new circle outputs on cylinder/cone/torus faces (the cylinder pcurve at SES:2741-2768 already builds a `v=const` line but bails on partial arcs).
- **`TolReached3d`** (FF:608) is recomputed from max sampled deviation — analytic conics give ~0; only the marcher's WLine needs it.

---

## 6. The architecture fix (CONTEXT's core concern), mapped to NURBS

The session boolean (brep.cpp `boolean`: imprint A and B independently SES-side at brep.cpp:2283-2352, then `imprint_edges` + `sew_coincident_edges` Hausdorff at brep.cpp:2424-2430) is wrong precisely because **A's circle and B's circle are two independently-fitted NURBS curves** glued by a point-to-polyline Hausdorff (brep.cpp:1987-2041). OCCT instead computes the section **once** (`IntTools_FaceFace::MakeCurve`, FF:690) and `IntTools_Context`/PaveFiller store a SINGLE `Geom_Curve` plus its TWO pcurves; the edge built from it is shared by faces from BOTH operands.

Your `analytic_ssi` already produces exactly this shape: one exact `cc3` plus `pa = analytic_pcurve(a,…)` and `pb = analytic_pcurve(b,…)` (SES:2991-2998) — the `(c3, pcurve_a, pcurve_b)` triple. The fix is to **change the boolean to consume that triple as a single shared edge**, not to imprint each operand separately:

1. Compute the section triple ONCE per face-pair (this is `surface_surface`, already returns the triple, SES:489).
2. Build ONE `m_curves_3d` entry from `cc3` (the SAME control points for both sides).
3. Build TWO `m_curves_2d` (pcurves) `pa`, `pb`; split each operand's face with its own pcurve via `split_by_uv_curves` (brep.cpp:1767) — but record that both resulting edges reference the *same* `m_curves_3d` index.
4. In sewing, edges that already share a `m_curves_3d` index are merged by identity (no Hausdorff). This removes the 9%-volume corruption: the shared boundary is one geometric object, so the two operands' splits are forced to agree exactly, instead of being reconciled after the fact.

For the degenerate canonical configs the new closed-form `ssi_*` routines make `cc3` an EXACT rational circle/ellipse/line (via `exact_circle`/`exact_ellipse`/degree-1), so both pcurves are exact `v=const` / longitude-split lines — no marcher, no fitting tolerance, and the shared edge is bit-identical on both faces.

---

### Summary table (session action)

| pair | OCCT closed form (QQG line) | session now | action |
|---|---|---|---|
| plane-plane | Line (386) | ✓ ssi_plane_plane | keep |
| plane-sphere | Circle/Point (965) | ✓ | (opt) add tangent-point |
| plane-cyl | Ellipse/Circle/Lines (540) | ✓ partial | **add axis-∥ lines (597)** |
| plane-cone | Circ/Ell/Parab/Hyper/Lines (737) | ✓ circ+ell | add parabola/hyperbola/apex |
| plane-torus | concentric or 2 minor circ (2170) | ✓ ⟂ only | **add plane-contains-axis (2238)** |
| sphere-sphere | Circle (2039) | ✓ inline | keep |
| sphere-cyl | coaxial 1/2 circ (1364) | ✗ | **add ssi_sphere_cylinder** |
| sphere-cone | coaxial quad→1/2 circ (1920) | ✗ | **add ssi_sphere_cone** |
| cyl-cyl | ∥→lines, ⟂eq→2 ell, else quartic (1035) | ✗ | **add ssi_cylinder_cylinder** |
| cyl-cone | coaxial 2 circ (1313) | ✗ | **add ssi_cylinder_cone** |
| cyl-torus | coaxial 1/2 circ (2287) | ✗ | **add ssi_cylinder_torus** |
| cone-cone | coaxial circ / apex lines / etc (1428) | ✗ | **add ssi_cone_cone (coaxial+apex)** |
| cone-torus | coaxial up to 4 circ (2372) | ✗ | **add ssi_cone_torus** |
| sphere-torus | centre-on-axis 1/2 circ (2515) | ✗ | **add ssi_sphere_torus** |
| torus-torus | coaxial Same/1/2 circ (2611) | ✗ | **add ssi_torus_torus** |

Relevant session files: `C:/pc/3_code/code_rust/session/session_cpp/src/intersection.cpp` (analytic SSI SES:2120-3001; dispatch SES:2937; pcurve SES:2705; sphere seam pullback SES:2811), `C:/pc/3_code/code_rust/session/session_cpp/src/intersection.h` (API line 489), `C:/pc/3_code/code_rust/session/session_cpp/src/brep.cpp` (boolean imprint/classify/sew lines 2283-2430, sew Hausdorff 1976-2087, split_by_uv_curves 1767).