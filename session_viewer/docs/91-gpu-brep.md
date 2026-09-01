# 91 GPU BRep — faces assemble, seams dissolve

> **Big picture.** *Phase 10b closes.* A BRep (46) is faces + shared edge curves + topology.
> On the GPU it is **nothing new**: every face is a 74 grid trimmed by 75's loops, every
> edge is a 73 curve into the segment table. This lesson is therefore mostly an *assembly*
> lesson — plus one deep idea about what the CPU mesher was working hardest at, and why the
> GPU path gets to skip it. `BRep::mesh()` spends its phases (brep.rs: classify faces,
> extract shared-edge discretizations, mesh CDT faces against matched boundaries) making
> neighboring faces agree on the SAME boundary points, so the export mesh is watertight.
> Per-fragment trimming makes that labor a **display non-requirement**: two adjacent faces
> each clip their own fragments against the *same trim curve*, so the visual crack between
> them is bounded by the loop discretization — sub-pixel — and the drawn edge curve covers
> the seam with a pen width on top of that. Watertight *geometry* still comes from the CPU
> mesher; it just stopped being the price of drawing.

<svg viewBox="0 0 680 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a brep splits into per-face trimmed grids and shared edge curves; both faces clip against the same curve so the seam is covered; one dispatch per face, edges through the curve lane" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g transform="translate(30,18)">
    <path d="M 0,64 L 64,86 L 138,58 L 74,40 Z" fill="none" stroke="#6fb3ff" stroke-width="2"/>
    <path d="M 0,64 L 0,22 L 74,0 L 74,40" fill="none" stroke="#6fb3ff" stroke-width="2"/>
    <path d="M 74,0 L 146,18 L 138,58" fill="none" stroke="#6fb3ff" stroke-width="2"/>
    <path d="M 74,40 L 138,58" stroke="#e05555" stroke-width="3"/>
    <text x="73" y="112" fill="#888" text-anchor="middle">one shared edge, TWO faces clip against it</text>
  </g>
  <g transform="translate(250,22)" font-size="11">
    <text x="0" y="10" fill="#d7dae0">face k  → 74 grid + 75 loops   (one dispatch each)</text>
    <text x="0" y="32" fill="#d7dae0">edge j  → 73 segments           (m_curves_3d, real curves)</text>
    <text x="0" y="54" fill="#d7dae0">vertex i → sphere markers        (unchanged, 32a lane)</text>
    <text x="0" y="84" fill="#888">seam math: both faces stop at the SAME pcurve/edge —</text>
    <text x="0" y="100" fill="#888">crack ≤ loop discretization ≈ sub-pixel, pen covers the rest</text>
    <text x="0" y="122" fill="#666" font-size="10">watertight EXPORT: still BRep::mesh() on the CPU, by design</text>
  </g>
</svg>

## Files we touch

```
src/app/scene.rs   # the BRep arm: per-face 74/75 reservations, per-edge 73 reservations
src/engine/gpu/mod.rs  # dispatch loop covers faces of all objects; nothing else
```

No new shaders. That is the payoff of the producer pattern: the fourth type is pure wiring.

## Step 1 — the arm decomposes, the producers consume

46's arm drew `b.mesh()` from the cache. The GPU arm walks topology instead — for each face,
its surface + its loops' pcurves; for each edge, its 3D curve:

```rust
    Geometry::BRep(b) => {
        objects_base.push((/* like 68 */));
        for f in 0..b.m_faces.len() {
            let (srf, loops) = brep_face_uv(b, f);      // surface ref + UV loop polygons,
                                                        // discretized like 75 (the pcurves
                                                        // live in b.m_curves_2d via m_trims)
            let (gu, gv) = grid_for(srf);
            // 74 reservation + index grid, 75 loop upload — verbatim from those arms
        }
        for c in &b.m_curves_3d {
            // 73 reservation: spans × 64 clamped, seg_base, FACING_UNKNOWN
        }
        for v in &b.m_vertices {
            // vertex markers: unchanged, the 32a glyph push
        }
    }
```

`brep_face_uv` is the one genuinely new function, and it is a *reader*: face → loops →
trims → pcurve indices, chasing `m_faces / m_loops / m_trims / m_curves_2d` — the topology
walk 68 already taught, ending in `disc_loop` calls instead of a CDT.

## Step 2 — why the seams may drop out of the contract

Read `BRep::mesh()` (brep.rs) once more with 75 eyes. Phase 1 classifies faces; phase 2
extracts each shared edge's discretization; phase 3 *forces every face's CDT to conform to
those exact boundary points*. All of that exists so that triangle edges on face A coincide
bit-for-bit with triangle edges on face B — watertightness for booleans, offsets, export.

The GPU path renders each face's grid clipped by the same edge curve from both sides. The two
clipped boundaries disagree only by (a) the loop polygon's chord error — mesh_q's own budget,
chosen sub-pixel — and (b) f32 eval noise, ~1e-5 of object scale (measured in 73). Neither is
visible, and 46's real edge curves draw ON the seam with a pen width anyway. So the display
contract is: **cracks exist, below one pixel, under a drawn edge.** State it, measure it in
the acceptance, and keep `BRep::mesh()` for every consumer that needs actual watertight
triangles (picking proxy included — 47 raycasts the CPU mesh, coarse, as in 74).

The one case that can widen a crack: a face whose *surface* is much coarser-gridded than its
neighbor (different `grid_for` outcomes) AND a nearly edge-on view. The fix is the same law
the whole phase uses — density decided per face from ITS spans — plus the edge pen. If a model
ever defeats both, the escalation is boundary-strip snapping (the outer ring of boundary cells
snaps onto the loop polygon), noted in 75 and deliberately not built until a real model
demands it.

## Step 3 — one dispatch walk

`Gpu::dispatch_curves` and `dispatch_surfaces` (+ classify) already loop over per-object
lists; the BRep arm just pushed into the same lists. The whole scene re-tessellates through
THREE compute pipelines in one encoder, ordered classify-after-grid per face; the zoom-bucket
cadence from 73 governs all of it. There is nothing else — which is the lesson.

## What you should see

Load a boolean-result BRep from the kernel test set (a `cut` chair or the z30×20 box pair —
the shapes with curved trimmed faces AND shared edges everywhere). Acceptance, three parts:

1. **Fidelity**: GPU build vs 46's CPU cache build, same camera — silhouettes and shading
   match; the trim boundaries land under the drawn edge curves on every face pair.
2. **Seam audit**: zoom an edge until one boundary cell spans the screen — the crack, where
   findable at all, is under the edge pen. Toggle edges off (the 51-style filter) to inspect
   the raw seam honestly; sub-pixel at working zooms.
3. **The economics**: `add_file` on a many-face BRep no longer pays CPU CDT per face — load
   stall drops to the topology walk + uploads; re-tessellation on zoom is a dispatch, not a
   remesh.

```
Ch 76: Phase 10b closes on wiring, not shaders: face = 74 grid + 75 trim, edge = 73
        curve, vertex = 32a glyph — the BRep arm is a topology READER (m_faces→m_loops→
        m_trims→m_curves_2d) feeding three existing producers. The CPU mesher's shared-edge
        matching is an EXPORT requirement, not a display one: both faces clip against the
        same curve, cracks ≤ loop chord error (sub-pixel), edge pen on top. BRep::mesh()
        remains the watertight truth for booleans/export/picking.
```

Edited: `scene.rs` (BRep arm decomposition + `brep_face_uv`), `gpu/mod.rs` (dispatch walk
covers all lists). No new shader files — the point.

## Next

`92-ground-grid.md` resumes the mainline — an *analytic* plane drawn without vertices, which
after this phase reads as the degenerate case of everything 10b built: geometry whose GPU
representation is a formula, not a table.
