# OCCT study — plane∩cone conics & cylinder∩cylinder tangency arrangement

Implementation spec for the two remaining SSI/arrangement holes blocking the **box–cone** and
**cyl–cyl (Steinmetz)** boolean pairs. Companion to `OCCT_STUDY_pairs.md` (which already enumerates
the 15 quadric-pair closed forms and the `IntAna_QuadQuadGeo` recogniser). This doc does **not**
re-derive the SSI recognition; it specifies the two things `pairs.md` listed as TODO without math:

* GAP 1 — the **non-perpendicular plane∩cone conics** (parabola / **hyperbola**), their **exact
  rational-quadratic NURBS** construction (not sampled+fitted), and their **cone-UV pullback** where
  both `u` and `v` vary.
* GAP 2 — the **cyl∩cyl Steinmetz** section's **tangency-vertex arrangement** (the kernel SSI already
  emits the 2 ellipses; the hole is stitching arcs that meet at tangent vertices + the periodic seam).

### Path legend (oracle tree, abbreviated)

```
QQG  = …/ModelingData/TKGeomBase/IntAna/IntAna_QuadQuadGeo.cxx
III  = …/ModelingAlgorithms/TKGeomAlgo/IntPatch/IntPatch_ImpImpIntersection.cxx
CVH  = …/FoundationClasses/TKMath/Convert/Convert_HyperbolaToBSplineCurve.cxx
CVP  = …/FoundationClasses/TKMath/Convert/Convert_ParabolaToBSplineCurve.cxx
PJC  = …/ModelingData/TKGeomBase/ProjLib/ProjLib_Cone.cxx
WS1  = …/ModelingAlgorithms/TKBO/BOPAlgo/BOPAlgo_WireSplitter_1.cxx
SES  = C:/pc/3_code/code_rust/session/session_cpp/src/intersection.cpp
```

All absolute oracle paths are rooted at
`C:/pc/3_code/code_rust/session/validation/occt_oracle/build/deps/occt/src/occt/src/`.

---

# GAP 1 — PLANE × CONE non-perpendicular conics (box–cone)

A box's **side** faces cut a cone in a **hyperbola** (plane parallel to, or containing, the axis);
end/oblique faces give an **ellipse** or **parabola**; the perpendicular cap gives a **circle**.
The kernel's `ssi_plane_cone` (SES:2685–2755) already produces the **circle** (SES:2735–2744) and
**ellipse** (SES:2745–2749, exact via `build_exact_plane_cone_ellipse`), but for the
parabola/hyperbola it **samples + fits** (`sample_plane_cone_arcs` SES:2629–2684 → `fit_conic_arc`
SES:2594–2604) — an approximation. GAP 1 replaces that with the exact rational-quadratic builders.

### OCCT files read

* **QQG:737–938** `Perform(const gp_Pln&, const gp_Cone&, Tolang, Tol)` — full conic classifier.
  * setup: `dist` (signed apex→plane) **QQG:757**; `axey = n × axis`, `axex = axey × n` **QQG:765–766**;
    `cosa/sina` **QQG:770–771**; `sint = |axey|` **QQG:775**, `cost = |axis·n|` **QQG:776**;
    `costa = cost·cosa − sint·sina = cos(t+α)` **QQG:781**.
  * apex-on-plane (`|dist|<Tol`) **QQG:784–818**: 1 line / 2 lines / point.
  * **hyperbola, plane ∥ axis** (`cost<Tolang`) **QQG:825–837**.
  * **parabola** (`|costa|<Tolang`) **QQG:857–867**.
  * **circle** (`sint<Tolang`) **QQG:868–876**.
  * **hyperbola, general** (`cost<sina`) **QQG:877–890**.
  * **ellipse** (`cost>sina`) **QQG:891–903**.
  * magnitude guards → fall back to numeric (`EllipseLimit 1e9`, `HyperbolaLimit 2e6`) **QQG:910–935**.
* **QQG:2789–2869** accessors `Ellipse()/Parabola()/Hyperbola()` — the `(pt,dir1,dir2,param,parambis)`
  → `gp_Elips/gp_Parab/gp_Hypr` mapping (below).
* **III:3680–3755** `IntPatch_ImpImpIntersection` builds the `IntPatch_GLine` from the IntAna conic
  (Ellipse 3680, Parabola 3701, Hyperbola **3726–3754**, two branches via `inter.Hyperbola(i)`).
* **III:1959–1962** `GLine → Geom_Hyperbola/Geom_Parabola` (the curve that is then converted to BSpline).
* **CVH (whole file)** rational-quadratic hyperbola→BSpline; **CVP (whole file)** parabola→BSpline.
* **PJC:106–155** `ProjLib_Cone::Project(gp_Circ)` (v=z/cosα, u=atan2); **PJC:67–102** `Project(gp_Lin)`
  (generatrix→u=const); **PJC:157–170** ellipse/parab/hypr → **numeric** `ProjLib_Projector`.

## 1(a) — classify & parametrize the plane∩cone conic (closed form)

Inputs in the kernel's notation (`ssi_plane_cone`, SES:2685–2701): apex `V`, **unit** axis `w` (toward
the nappe where radius grows), half-angle `α ∈ (0, π/2)`; plane point `o`, **unit** normal `n`.
Build the in-plane axis frame:

```
axey = n × w                         # in-plane, ⟂ to the apex-foot direction   (QQG:765)
axex = axey × n                      # in-plane projection of w                  (QQG:766)
cosa = cos α ,  sina = |sin α|
cost = |w·n|        (= |cos∠(axis,normal)|)                                       (QQG:776)
sint = |axey| = sqrt(max(0,1−cost²))                                             (QQG:775)
costa = cost·cosa − sint·sina   (= cos(t+α), the “plane ∥ generatrix” test)      (QQG:781)
D0   = (V − o)·n                 (signed apex distance; QQG `dist`)               (QQG:757)
```

**Decision (the discriminant is `cost` vs `sina` — the Dandelin condition):**

| condition (apex OFF plane, `|D0|≥Tol`) | type | QQG |
|---|---|---|
| `cost < Tolang`        | **Hyperbola** (plane contains axis), 2 branches | 825 |
| `|costa| < Tolang`     | **Parabola** (plane ∥ a generatrix)             | 857 |
| `sint < Tolang`        | **Circle** (plane ⟂ axis)                        | 868 |
| `cost < sina`          | **Hyperbola** (general), 2 branches              | 877 |
| `cost > sina`          | **Ellipse**                                      | 891 |

Apex ON plane (`|D0| < Tol`): `|costa|<Tolang` → 1 line through apex; `cost<sina` → 2 lines through apex
with `dh = sqrt(sina²−cost²)/cosa`, directions `axex ± dh·axey` (QQG:802–811); else → point=apex.
(The kernel already implements all three apex-on-plane sub-cases, SES:2704–2728 — keep.)

**Center / axes / parameters.** Let `C0 = axis ∩ plane` (always exists when apex off plane):

```
s0 = −D0 / (w·n)            # axis param:  C0 = V + s0·w
C0 = V + s0·w
distance = |V − C0|        # QQG `distance`, QQG:849
```

Axis orientation fix (**QQG:851–855**): if the axis∩plane point lies on the apex's *negative* nappe,
reverse `axex, axey`. For the kernel's apex cone (`RefRadius=0`) the test reduces to: choose `axex` so
that `axex·(C0 − V) ≥ 0` (transverse axis points into the physical nappe).

**ELLIPSE** (`cost>sina`, QQG:893–902) — already in kernel; listed for completeness:
```
a (semi-major) = cost·sina·cosa·distance / (cost² − sina²)          = param1
b (semi-minor) = cost·sina·distance / sqrt(cost² − sina²)           = param1bis
δc             = sint·sina²·distance / (cost² − sina²)
Center  Ce = C0 + δc·axex_hat
Frame   X = axex_hat (transverse, in plane) , Z = n , Y = n × X
```

**HYPERBOLA, general** (`cost<sina`, QQG:877–889):
```
R (semi-transverse / “MajorRadius”) = cost·sina·cosa·distance / (sina² − cost²)   = param1
r (semi-conjugate  / “MinorRadius”) = cost·sina·distance / sqrt(sina² − cost²)     = param1bis
δc                                  = sint·sina²·distance / (sina² − cost²)
Center  Ch = C0 − δc·axex_hat
Frame   X = axex_hat , Z = n , Y = n × X
2 branches (QQG accessor 2863–2867): gp_Hypr(Ax2(Ch, Z=n, X=+axex), R, r)
                                     gp_Hypr(Ax2(Ch, Z=n, X=−axex), R, r)
```
For a **single-nappe** apex cone only the branch opening toward the nappe is physical; the box face
selects one branch (pick the branch whose vertex `Ch ± R·X` has axial `s>0`).

**HYPERBOLA, plane contains axis** (`cost≈0`, QQG:829–836) — the box **side**-face case, simplest:
```
Center  Ch = V − D0·n                 # apex projected onto the cutting plane
R = |D0| / tan α                      # = |D0/Tan(angl)|   (semi-transverse)        param1
r = |D0|                              # (semi-conjugate)                            param1bis
Frame   X = axex , Z = n , Y = n × X
```

**PARABOLA** (`|costa|≈0`, QQG:857–866):
```
δc = distance / (2·cosa)
Vertex Cp = C0 − δc·axex_hat
Focal  F  = δc·sina²                  # QQG param1 is the gp_Parab FOCAL length
                                      # (gp_Parab Parameter p = 2F ; eqn y² = 2p·x = 4F·x)
Frame  X = axex_hat (axis of parabola, toward opening) , Z = n , Y = n × X
```

**CIRCLE** (`sint≈0`, QQG:868–875): center `C0`, radius `distance·tan α`, normal `w` — kernel already
exact (SES:2735–2744).

> `gp_Ax2(P, Vz, Vx)` convention: `Vz` = the conic-plane **normal** (= `n` here), `Vx` = the in-plane
> X (transverse) axis. The conic therefore lies in the cutting plane spanned by `X = axex` and
> `Y = n×X = axey`. Confirmed by `IntAna_QuadQuadGeo::Hyperbola()` QQG:2863 / `Parabola()` QQG:2844 /
> `Ellipse()` QQG:2809.

## 1(b) — exact 3D NURBS for each conic (rational quadratic, bounded arc)

Local parametrizations (all in the conic frame, origin = center/vertex, lying in the cutting plane):

```
Ellipse / Circle  P(θ) = C + a·cosθ·X + b·sinθ·Y          → keep exact_ellipse/exact_circle (9-CV)
Hyperbola (gp_Hypr) P(u) = C + R·cosh(u)·X + r·sinh(u)·Y   (one branch; u∈ℝ)
Parabola  (gp_Parab) P(t) = Cp + (t²/(4F))·X + t·Y         (t∈ℝ)
```

**A single degree-2, 3-control-point Bézier segment is EXACT for any sub-arc of a conic.** OCCT's
`Convert_*ToBSplineCurve` emit exactly that: 3 poles, 2 clamped knots `{UF mult 3, UL mult 3}`,
non-periodic (CVH:44–50, CVP:45–51).

**HYPERBOLA arc `u∈[u1,u2]`** — `Convert_HyperbolaToBSplineCurve` (CVH:33–82), the RATIONAL case:
```
UF = min(u1,u2) , UL = max(u1,u2)
weights = { 1 , cosh((UL−UF)/2) , 1 }                                          (CVH:65–67)
delta = sinh(UL − UF)                                                          (CVH:69)
# local-frame poles (X-major along R, Y along S·r ; S = ±1 frame handedness):
P1 = ( R·cosh(UF) ,            S·r·sinh(UF) )                                  (CVH:72)
P2 = ( R·(sinh(UL)−sinh(UF))/delta , S·r·(cosh(UL)−cosh(UF))/delta )           (CVH:70–73)
P3 = ( R·cosh(UL) ,            S·r·sinh(UL) )                                  (CVH:74)
```
`S = sign(det[X,Y]) = +1` if `(X × Y)·Z > 0` else `−1` (CVH:58; with the frame above it is `+1`).
The middle pole `P2` is the **projective tangent-intersection** point and carries weight `cosh>1`
(this is what bends a straight CV polygon into a hyperbola). Map each local `(x,y)` to 3D and
homogenise:
```
Q_i = C + x_i·X + y_i·Y
CV_i (4D, like exact_ellipse) = ( Q_i·w_i , w_i )      # store x·w,y·w,z·w,w
NurbsCurve(degree=2, rational=true, 3 CV, knots {0,0,0,1,1,1})
```

**PARABOLA arc `t∈[t1,t2]`** — `Convert_ParabolaToBSplineCurve` (CVP:33–72), **NON-rational** (poly):
```
weights = { 1 , 1 , 1 }                                                        (CVP:53–55)
p = 2F  (= gp_Parab::Parameter)   # so x = t²/(2p) = t²/(4F)
# local poles (CVP:62–64):
P1 = ( UF²/(2p) , S·UF )
P2 = ( UF·UL/(2p) , S·(UF+UL)/2 )
P3 = ( UL²/(2p) , S·UL )
# map to 3D as above with all weights = 1 (degree-2 non-rational Bézier).
```

**Clip range `[u1,u2]` / `[t1,t2]` from the cone-height and box-face extents.** The arc must satisfy
two clips; take the **intersection** of the two parameter windows:

1. **Cone height** `s ∈ [0, H]` (kernel already computes `H = cone_axial_extent`, SES:2692). Axial
   coordinate along the arc:
   ```
   Hyperbola: s(u) = (C−V)·w + R·(X·w)·cosh u + r·(Y·w)·sinh u  =  k0 + k1·cosh u + k2·sinh u
   ```
   Solve `s(u)=0` and `s(u)=H` in closed form via `E = e^u`:
   ```
   (k1+k2)·E² − 2·(target−k0)·E + (k1−k2) = 0 ,  u = ln(E_physical)
   ```
   (take the root with `E>0` on the chosen branch). Parabola: `s(t)` is quadratic in `t` → ordinary
   quadratic roots.
2. **Box side-face rectangle**: express the face as 4 boundary lines in the cutting-plane 2D frame
   `(X,Y)`; intersect the conic with each (cosh/sinh or quadratic in the local coord) and keep the
   innermost `[u1,u2]`.

Emit ONE Bézier segment per clipped arc. (If a very wide arc is needed, subdivide at equal `Δu` — each
sub-arc is again an exact 3-CV segment; one segment suffices for a planar box face.)

## 1(c) — analytic PULLBACK onto the CONE UV (both u and v vary)

OCCT's cone parametrization (ElSLib / **PJC:144** `V = z/Cos(SemiAngle)`, **PJC:135–142** `U=atan2`):
```
u(P) = atan2( (P−Loc)·Ycone , (P−Loc)·Xcone )                 # longitude (periodic, the cone u)
z(P) = (P − Loc)·w                                            # axial coordinate
v(P) = z / cos α                                              # SLANT distance  (the cone v)
radius at v:  ρ(v) = RefRadius + v·sin α                      # apex cone: RefRadius=0 ⇒ ρ = z·tan α
```
For a `v=const` circle OCCT returns a **2D line** `D2d=(±1,0)` (PJC:146–153); for a generatrix a
`u=const` line `D2d=(0,±1)` (PJC:92–98). For **ellipse/parabola/hyperbola** OCCT has **no closed-form
pullback** — `ProjLib_Cone::Project(Hypr/Parab/Elips)` defers to the **numeric** `ProjLib_Projector`
(PrjResolve Newton) (PJC:157–170).

The kernel's existing `analytic_cone_pullback` (SES:3175–3267) already reproduces the correct closed
behaviour and is the right tool for the hyperbola — it is currently defined but **not wired into the
dispatch**. Its map for a point `P(t)` on the arc:
```
u(t) = wrap( atan2( (P−A)·Yc , (P−A)·Xc ) )       # A = apex; longitude, inverted through a
                                                  #   u→longitude table (NURBS u is NONLINEAR
                                                  #   in angle — SES:3203–3224)
v(t) = v0 + ( (P−A)·w − h0 ) / (h1 − h0) · (v1−v0) # axial height is LINEAR in v ⇒ closed form
                                                  #   (SES:3190–3192) — matches v=z/cosα
```
Both `u` and `v` vary along a hyperbola. **Seam:** where the unwrapped `u` crosses the periodic seam
(domain `u0/u1`), split into in-domain arcs that land **exactly** on `u0/u1` (the `kof()/seam_cont`
logic, SES:3244–3266). A hyperbola typically stays within a sub-interval and never wraps, so usually
no split fires; the generic logic handles it when it does.

On the **box's PLANE side**, `analytic_pcurve`'s PLANE branch (SES:2861–2887) inverts the affine
`(u,v)→3D` map and remaps the conic's control points **directly** — so the exact rational hyperbola CVs
become exact pcurve CVs (same degree/knots/weights). No sampling.

## Kernel integration (GAP 1)

* **Extend `ssi_plane_cone`** (SES:2685). Keep the classifier (SES:2729–2734 already sets
  `is_hyperbola/is_parabola`). Replace the sampled fallback (SES:2750–2752) with two exact builders
  mirroring `build_exact_plane_cone_ellipse` (SES:2605):
  * `build_exact_plane_cone_hyperbola(o,n,V,w,α,H,faceRect, out)` → compute `(cost,sint,distance,δc,R,r)`
    per 1(a) → `gp_Hypr` local frame → `Convert_HyperbolaToBSplineCurve` 3-CV rational segment per 1(b),
    clipped to `s∈[0,H]` ∩ faceRect. Emit one branch (or both if the cone is two-nappe).
  * `build_exact_plane_cone_parabola(...)` → analogous with weights `{1,1,1}`.
  * Keep `sample_plane_cone_arcs`+`fit_conic_arc` ONLY as a last-resort fallback when a clip fails
    (degenerate window) or when QQG's magnitude guard (QQG:910–935: `R>2e6`) would trip.
* **Wire the cone-side pullback.** In `analytic_ssi` (SES:3583–3590), when a side is `CONE`, call
  `analytic_cone_pullback` (SES:3175) for that side instead of `analytic_pcurve` (which only handles a
  `v=const` circle, SES:2953–2977); keep `analytic_pcurve` (PLANE branch) for the box side. Mirror the
  pattern already implied for SPHERE via `analytic_sphere_pullback` (SES:3045).
* **Dispatch** is unchanged — `ssi_plane_cone` is already routed (SES:3539–3540); it just returns
  exact curves now instead of fitted ones.

---

# GAP 2 — CYLINDER × CYLINDER Steinmetz + tangency-vertex arrangement

Two equal perpendicular cylinders intersect in a **figure-eight = two ellipses** meeting at **two
tangency vertices** (where the surfaces are mutually tangent, normals parallel). The kernel SSI
(`ssi_cylinder_cylinder`, SES:3374) **already emits the 2 exact ellipses** (SES:3409–3422). The holes
are: (i) the UV **arrangement** can't stitch arcs that **cross / meet at a tangent vertex**, and (ii)
the periodic-`u` seam with **varying `v`** (the arcs are `v=±R·cos u`, not `v=const`).

### OCCT files read

* **QQG:1035–1282** `Perform(Cylinder, Cylinder, Tol)`:
  * parallel axes → circle∩circle in the base plane → 0/1/2 **lines** **QQG:1057–1205**
    (chord half-angle `aCos = ½(R1²−R2²+d²)/(R1·d)`, `aSin=√(1−aCos²)`, rotate base vector **QQG:1141–1182**).
  * **perpendicular + equal radius + intersecting axes** → **2 ellipses** **QQG:1209–1250**.
  * external tangent (`|d−(R1+R2)|<Tol`, non-parallel) → **point** **QQG:1254–1275**.
  * else → `IntAna_NoGeometricSolution` **QQG:1278** → general quartic (marcher).
* **III:4815–5156** `CyCyAnalyticalIntersect`; the **IntAna_Ellipse case III:4987–5141**:
  builds the 2 ellipses **and** the 2 shared **MULTIPLE (tangency) points** `pmult1/pmult2` at ellipse
  parameters `0.5π` and `1.5π` (III:4994–5001, `SetMultiple(true)` III:5000–5001), and **adds them as
  vertices to BOTH GLines** (III:5061–5065 and III:5137–5138). `IntAna_Parabola/Hyperbola` are an
  error here (III:5144–5146) — cyl-cyl never produces those.
* **WS1** wire walk: `SplitBlock` selection loop **WS1:519–606** (pick **min `ClockWiseAngle`**,
  WS1:585); `ClockWiseAngle` **WS1:611–649**; `Angle2D` (curvature-scaled **secant** angle)
  **WS1:758–830**; `Angle` (ref `(1,0)`) **WS1:834–843**; `RefineAngles` **WS1:893–1018**;
  `RefineAngle2D` (2D probe) **WS1:1022–1114**. `RefineAngles(myFace,…)` is invoked once up front,
  WS1:315.

## 2(a) — exact analytic SSI for two cylinders

`AxeOperator(axis1, axis2)` first (the kernel uses `ssi_cross` + `point_axis_dist`, SES:3379–3382).
Let `w1,w2` = unit axes, `R1,R2` radii, `d` = axis gap.

**Parallel axes** (`|w1×w2|≈0`) — circle∩circle in the base plane (kernel SES:3381–3408):
```
d > R1+R2          → empty
d = R1+R2          → 1 tangent line ∥ w1                                       (QQG:1091)
|R1−R2| < d < R1+R2 → 2 lines ∥ w1 at the two base-circle intersection points  (QQG:1103–1184)
d = |R1−R2|        → 1 internal tangent line                                   (QQG:1185)
d < |R1−R2|        → empty
coaxial & R1=R2    → Same                                                      (QQG:1061)
```
Line foot via `aCos = ½(R1²−R2²+d²)/(R1·d)`, half-chord `h = R1·√(1−aCos²)`, on the common-perp
direction; axial extent clipped to both faces' `[s_lo, s_hi]` (kernel `cyl_span`, SES:3292).

**Perpendicular + equal radius + intersecting** (`|R1−R2|/Rmax ≤ 1e-13`, axes meet at `P`) — the
**Steinmetz** case (QQG:1209–1250; kernel SES:3409–3422):
```
A  = ∠(w1,w2)
major1 = R / |sin(A/2)|       minor = R       # ellipse-1
major2 = R / |sin((π−A)/2)|   minor = R       # ellipse-2     (QQG:1235–1236)
minorDir = unit(w1 × w2)                      # shared MINOR axis = common perpendicular (= world Z
                                              #   for axes in a plane)
ellipse1 = exact_ellipse(P, majDir=unit(w1+w2), minDir=minorDir, R/|sin(A/2)|, R)
ellipse2 = exact_ellipse(P, majDir=unit(w1−w2), minDir=minorDir, R/|cos(A/2)|, R)
```
**Tangency / self-crossing vertices** (the missing piece): the two ellipses meet at the two **ends of
the shared minor axis**:
```
T+ = P + R·minorDir          T− = P − R·minorDir
```
These are OCCT's `pmult1/pmult2` at ellipse parameters `0.5π / 1.5π` (III:4994–4995): with the major
axis at parameter 0, `ElCLib::Value(0.5π, ellipse) = center + minorRadius·Ydir = center + R·minorDir`.
At `T±` the two cylinders are **mutually tangent** (both surface normals are `±minorDir`), so the
section self-crosses there. For perpendicular equal cylinders `T± = P ± R·Z` = the figure-eight apexes
`(0,0,±R)`.

**Else** (general skew, or unequal perpendicular) → `NoGeometricSolution` → marcher. Kernel returns
`false` here (SES:3410) so the marcher takes the quartic.

## 2(b) — section as edges meeting at shared vertices; the tangency angle tie-break

**Representation.** OCCT carries the section as `IntPatch_GLine`s with explicit **vertices**, and the
two tangency points are added to **both** ellipse GLines as **shared multiple vertices** (III:5061–5065,
5137–5138). Downstream the edges are split at every vertex, so each ellipse becomes **two arcs
`T+→T−`** and the **four arcs share the two vertices `T±`**. The face wire is then rebuilt by
`BOPAlgo_WireSplitter`.

**Wire walk** (`SplitBlock`, WS1:519–606). At each vertex, from the incoming edge (reference angle
`anAngleIn = AngleIn`, WS1:519) choose, among outgoing-and-not-yet-passed edges, the one with the
**smallest `ClockWiseAngle(anAngleIn, anAngleOut)`** (WS1:585 `anAngle < aMinAngle − eps`). "Turn as
sharply clockwise as possible" ⇒ trace a consistent face loop. Same edge reversed → angle `2π`
(WS1:556).

**`ClockWiseAngle`** (WS1:611–649):
```
AIn,AOut ∈ [0,2π);  A1 = AIn + π (mod 2π);  dA = A1 − AOut;
if dA ≤ 0      dA += 2π
else if dA ≤ 1e-14  dA = 2π        # a perfectly BACK-tangent edge is the U-turn (2π), not 0
return dA                          # ∈ (0, 2π]
```

**The tangency fix is in `Angle2D`** (WS1:758–830): the per-edge angle at a vertex is **not** the exact
tangent — it is a finite **SECANT** over a **curvature-scaled** parameter step:
```
dt = max( Resolution(tol2d), acos( R/(R+tol2d) ) )   # R = local 2D curvature radius   (WS1:790–796)
dt = min( dt, 0.05·(last−first) )                    # cap                              (WS1:800–810)
aPV  = pcurve(vertexParam)
aPV1 = pcurve(vertexParam ± dt)                       # step INTO the edge
dir  = bIsIN ? (aPV1→aPV) : (aPV→aPV1)                # incoming reversed                (WS1:824)
angle = atan2 of dir relative to (1,0)                                                   (WS1:826–827, 834)
```
This finite secant **separates two curves that share the same tangent at the vertex but curve apart** —
exactly the Steinmetz `T±` case, where the two cosine pcurves cross with equal-magnitude opposite
slope. Transversal crossings already have distinct tangents, so the secant only matters at tangency.

**`RefineAngles`** (WS1:893–1018) — explicit tangency disambiguation. For a vertex with **exactly 2
boundary edges** (`iCntBnd==2`, WS1:954) and some interior (section) edges, any interior edge whose
secant angle falls **outside** the boundary wedge `[aA2, aA1]` (`aDA ≥ aDelta`, WS1:975) is re-probed by
`RefineAngle2D` (WS1:1022–1114): intersect the edge's pcurve against the two boundary rays
(`Geom2dInt_GInter`, WS1:1069), take the true departure parameter, step a fraction `aCf = 0.01` of the
way in (WS1:1099), recompute the angle, and accept if it now lands inside the wedge. If that fails and
there are 2 interior edges, **force** `aA = aA1 + Precision::Angular()` or `aA2 − Precision::Angular()`
(WS1:987). Net effect: at a tangency vertex the section edges are nudged just inside the correct side so
the min-angle walk picks a consistent loop instead of stalling on a `0 / 2π` tie.

## 2(c) — pullback onto each cylinder UV (v varies) + periodic-seam split

Cylinder parametrization (exact, `ProjLib_Cylinder`): `u = atan2(local y, local x)` (angle),
`v = (P − Loc)·w` (axial). For the perpendicular equal-cylinder case (axes `X`,`Y`, radius `R`), on
**cyl-1** the two Steinmetz ellipses pull back to two **cosine** curves:
```
on cyl-1:  v(u) = +R·cos u        and        v(u) = −R·cos u
```
Derivation: a point on cyl-1 is `(x, R·cos u, R·sin u)` with `v = x`; on the section `y = ±x` ⇒
`x = ±R·cos u`, so `v = ±R·cos u`. The two cosines **cross at `u = π/2, 3π/2` (`v = 0`)** — which are
exactly the vertices `T±` (`u=π/2 → (0,0,R)`, `u=3π/2 → (0,0,−R)`). **`v` varies along each arc.**

Explicit map for a sampled arc point `P(t)` (mirror `analytic_cone_pullback`, but `v` is the raw axial
coordinate — no `cosα` factor):
```
u(t) = wrap( atan2( (P−Loc)·Y1 , (P−Loc)·X1 ) )      # longitude, inverted via u→angle table
v(t) = v0 + ((P−Loc)·w1 − z0)/(z1−z0)·(v1−v0)         # axial, LINEAR ⇒ closed-form v
```

**Periodic `u`-seam.** `u` wraps `[u0,u1)`; each cosine arc crosses the seam once. Split where the
unwrapped `u` crosses `u0/u1`: end the arc **exactly** on the seam, restart on the opposite seam at the
**same `v`** — identical to the `kof()/seam_cont` interpolation in `analytic_cone_pullback`
(SES:3244–3266) and `analytic_sphere_pullback` (SES:3146–3167). So one ellipse → up to 2 in-domain arcs
per seam crossing, each anchored exactly on `u0`/`u1`.

## Kernel integration (GAP 2)

1. **SSI — emit the tangency vertices and split the ellipses.** Extend `ssi_cylinder_cylinder`
   (SES:3409–3422): after building the 2 ellipses, also compute `T± = Pint ± R·unit(w1×w2)` and **split
   each ellipse at its `T±` parameters (`π/2`, `3π/2`)** into two `T+→T−` arcs, tagging the four arcs
   with the two shared vertex ids. This mirrors III:4987–5141 (the `pmult1/pmult2` + `AddVertex`
   construction) and gives the arrangement explicit edges that meet at shared vertices.
2. **Pullback — add `analytic_cylinder_pullback`.** Model it on `analytic_cone_pullback` (SES:3175):
   `u` via `atan2` + a `u→longitude` table, `v` = axial **linear**, seam-split on `u`. Route `CYLINDER`
   sides to it in the dispatch (SES:3583–3590) instead of `analytic_pcurve` — whose CYLINDER branch
   (SES:2892–2919) only emits a full `v=const` wrap and **bails on partial / `v`-varying arcs**
   (SES:2911, 2913). (Either add the new function, or generalise that branch to the `v(u)=±R·cos u`
   case.)
3. **Arrangement — port the BOPAlgo_WireSplitter vertex model** into `split_by_uv_curves`
   (brep.cpp:1767):
   * compute each edge's vertex angle as a **curvature-scaled finite secant** (`Angle2D`, WS1:758–830),
     **not** the raw tangent — this single change lets tangent arcs be ordered;
   * at each vertex pick the **min `ClockWiseAngle`** outgoing edge (WS1:585, 611);
   * add the `RefineAngle2D` nudge (WS1:1022) for interior edges tangent to a boundary;
   * because the marcher terminates at `n_a ∥ n_b` tangencies (intersection.h:487), **do not rely on
     the marcher to cross `T±`** — seed the four arcs directly from the known `T±` vertices and the
     ellipse parameters (step 1), so the arrangement is fed complete arcs that already share the two
     tangency vertices.

---

## Summary — key formulas

**GAP 1 (plane∩cone).** Classifier discriminant: `cost=|w·n|` vs `sina=sin α` ⇒ ellipse (`cost>sina`)
/ parabola (`costa=cost·cosa−sint·sina≈0`) / hyperbola (`cost<sina`) / circle (`sint≈0`); center on the
axis∩plane point offset by `δc·axex`. **Hyperbola (box side)** when plane ∥ axis: `R=|D0|/tanα`,
`r=|D0|`, center `V−D0·n`. **Exact 3D arc** = degree-2 rational Bézier (3 CV): poles
`(R·cosh u, S·r·sinh u)` with **middle pole** `(R(sinhU2−sinhU1)/sinh(U2−U1), …)` and **weights
`{1, cosh((U2−U1)/2), 1}`**; parabola is the same shape with weights `{1,1,1}` and poles
`(t²/(4F), t)`; clip `u` by solving `s(u)=k0+k1·cosh u+k2·sinh u = 0,H` (quadratic in `e^u`).
**Cone pullback**: `u=atan2` (longitude, table-inverted), `v=z/cosα` (linear) — both vary; seam-split on
`u` (reuse `analytic_cone_pullback`).

**GAP 2 (cyl∩cyl Steinmetz).** Two ellipses through the axis-cross `P`: major dirs `unit(w1±w2)`, shared
minor dir `unit(w1×w2)`, semis `(R/|sin(A/2)|, R)` and `(R/|cos(A/2)|, R)`. **Tangency vertices**
`T± = P ± R·unit(w1×w2)` (= ellipse params `π/2, 3π/2` = OCCT `pmult1/pmult2`), shared by all four arcs.
**Pullback** on each cylinder: `v = ±R·cos u` (v varies), crossing at `u=π/2,3π/2`; seam-split on the
periodic `u`. **Arrangement**: order edges by a **curvature-scaled finite secant angle** (not the
tangent — the only thing that separates tangent arcs), walk by **min `ClockWiseAngle`** (back-tangent
⇒ `2π`), disambiguate tangents with `RefineAngle2D` (probe `aCf=0.01` into the edge, nudge by
`Precision::Angular()`).
