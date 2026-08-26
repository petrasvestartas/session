# GPU tessellation — study + phase plan

**The question this answers**: after the pointcloud lessons (36–39) and before BVH/selection
(40, 46–50), should the curriculum insert geometry lessons — plane shaders, GPU-tessellated
NURBS curves/surfaces, GPU BRep tessellation (concave, holes)? Or is the geometry-type
coverage already complete?

## Verdict

**Type coverage is complete; do not insert anything between 39 and 40.** Every geometry type
already has its lesson: NurbsCurve 65, NurbsSurface 66, iso-curves 67, BRep 68, trimmed 69,
and "plane" needs no tessellation at all — its two real viewer forms are already lessons
(ground grid 70, work-plane 80; both are vertexless/analytic shaders, not geometry).

**What is genuinely missing is not a type — it is where tessellation RUNS.** Lessons 67–69
tessellate on the CPU (`divide_by_count`, `mesh_grid`, `mesh_q` CDT) and cache. Moving
tessellation into compute shaders is a real, teachable, archive-grade upgrade — but it is an
*architecture* phase, and it belongs **immediately after 69** (call it Phase 10b, lessons
73–76), not before 40, for two hard reasons:

1. **40–50 are built on CPU-side triangles.** The BVH (40), frustum culling (41), and the
   whole picking stack (46–49) raycast CPU-resident tessellations. GPU-resident geometry has
   nothing on the CPU to raycast — teaching it first would force GPU picking or readback
   before the basic toolbox exists. (The GPU phase below keeps a coarse CPU proxy for picking,
   which only makes sense once picking exists.)
2. **You cannot teach "move it to the GPU" before the CPU version exists on screen.** 65–69
   establish what is being replaced and ship the correctness reference to diff against
   (same-scene CPU-vs-GPU pixel diff is each lesson's acceptance test).

## What the kernel already knows (the CPU sources to port or reuse)

| CPU piece | Where | GPU fate |
|---|---|---|
| Cox–de Boor basis + span search | `nurbscurve.rs:3319` `basis_functions`, span via knot search | **Port to WGSL** — per-sample independent, fixed-size arrays at max degree |
| Curve sampling | `divide_by_count/length`, `point_at` | **Port** — one compute invocation per sample |
| Surface eval + normals | `nurbssurface.rs` `point_at(u,v)`, `normal_at` (du×dv from `basis_functions_derivatives`) | **Port** — one invocation per (u,v) grid site |
| Deflection criteria | `mesh_q(max_angle_deg, chord_factor)` refinement rules | **Reuse as LOD law** (density chosen from spans/curvature), not as a mesher |
| Trim-loop discretization | `mesh_q` step 1 (adaptive pcurve polygonization) | **Stays CPU** at load — tiny, and the GPU consumes the polygon |
| Bowyer–Watson CDT | `nurbssurface_trimmed.rs:12` | **Does NOT port.** Incremental/sequential by nature; GPU Delaunay is research-grade. Replaced by trim-by-fragment (below) |
| BRep face classification + shared-edge matching | `brep.rs` mesh phases 1–3 | Classification reused; seam-matching becomes unnecessary for display (see 76) |

## The one idea that makes trimming GPU-shaped

On the CPU you *polygonize the trim region* (CDT with loops as constraints). On the GPU you
**never polygonize it**: tessellate the FULL UV rectangle as a dumb grid (trivially parallel),
and decide *per fragment* whether its (u,v) lies inside the trim loops — a winding-rule
coverage test, exactly like 2D vector graphics. Concave boundaries and holes are handled by
the winding rule for free; there is no meshing problem left. This is the established
literature approach (Guthe et al. 2005, GPU NURBS trimming; Schollmeyer & Fröhlich 2009;
stencil-trimming before that).

Ladder within the lesson:
1. **Trim mask**: rasterize the UV loops once into a small R8 coverage texture (nonzero
   winding); the surface fragment shader samples it at the interpolated (u,v) and discards.
2. **Cell classification** (compute): grid cells fully-inside draw plain (early-Z intact, no
   discard); fully-outside collapse degenerate; only *boundary* cells pay the mask+discard
   path. This bounds the discard cost — the same early-Z discipline the flat-lines rework
   established (frag_depth/discard kill early-Z; confine them).
3. Optional crisp silhouette: snap the outer ring of boundary cells onto the trim polyline
   (marching-squares flavored) in compute. Usually unnecessary — the mask is sub-pixel at
   sane resolutions and the real trim EDGE curve is drawn on top by the linework lane anyway.

## Phase 10b — proposed lessons (insert after 69, before 70)

**73 GPU curves — the segment table gets a compute producer.**
Control points + knots in a storage buffer; one invocation per sample; de Boor in WGSL
(fixed max-degree arrays); rational = homogeneous accumulate, divide by w at the end. Output
is **CylinderSegment rows written straight into the existing shared segment table** — the
Tubes and Flat lanes, density taper, markers, everything downstream works unchanged and both
line styles come for free. Sample count = the kernel's span rule × a screen factor,
recomputed only when zoom crosses a ×2 threshold (render-on-demand friendly). Draw via
indirect count. Picking keeps the coarse CPU polyline as proxy — GPU output is display-only.

**74 GPU surfaces — the arena gets a compute producer.**
(u,v) grid eval → RenderVertex (position + du×dv normal) into an arena region (45); one
static index grid per resolution class. Density per direction chosen up front from the
`mesh_q` angle/chord criteria applied to spans — a pre-pass estimate, not per-frame work.
Iso-curves (67) fall out of the same dispatch: the same basis evaluations emit iso-lines as
segment rows.

**75 GPU trimming — the CDT stays home.**
The trim-by-fragment ladder above, on top of 74's grid. Acceptance: the archive's trimmed
test set (circle hole, concave boundary) pixel-diffed against `mesh_q` CPU output; the
early-Z cost of boundary-cell discard measured with the bench harness (`bench_lines` pattern).

**76 GPU BRep — faces assemble, seams dissolve.**
A BRep = 74+75 per face + its real edge network through 73. The CPU mesher's hardest
labor — matching shared-edge discretizations so faces stay watertight — is *not needed for
display*: both adjacent faces clip fragments against the same trim curve, so the crack is
bounded by mask resolution (sub-pixel), and the drawn edge curve covers the seam with pen
width regardless. State the trade honestly: watertight *export* still uses the CPU mesher;
the GPU path is the display path. Per-face LOD, one indirect draw per lane.

## Cross-cutting rules (hard-won this week — bake into every 69x lesson)

- **No frag_depth, ever** — fixed-function depth keeps early-Z; discard only inside
  classified boundary cells.
- **Produce into the EXISTING tables** (segment table, vertex arena) — line styles, markers,
  density taper, hidden-filter, selection tinting all keep working with zero per-type code.
- **Re-tessellate on zoom thresholds, not per frame**; cache like 66 taught (shape-keyed,
  transform-immune).
- **CPU proxies remain the picking source** — a later lesson (natural neighbor: 81
  advanced-perf) can introduce GPU picking if ever needed.
- WebGPU has **no hardware tessellation and no geometry shaders** — compute-writes-buffers is
  the only (and better) path.

## What NOT to build

- GPU Delaunay/CDT (sequential algorithm, research-grade on GPU, zero payoff over the mask).
- A special "plane shader" lesson before 40 — the plane's viewer forms are 70/80 already.
- Per-frame adaptive tessellation (fights render-on-demand; thresholded LOD is enough).
