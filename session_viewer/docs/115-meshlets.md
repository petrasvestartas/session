# 115 Meshlets — clusters cull what triangles can't

> **Big picture.** *Phase 15.* A 1.3M-triangle scan is one draw today — perfect until
> most of it faces away or sits off-screen, and the vertex shader still runs 4M times.
> Clusters of ~124 triangles with a bounding box + a NORMAL CONE let a compute pass drop
> geometry at 124-triangle granularity: NVIDIA's CAD-scene meshlet demo measured 4.6 →
> 1.3 ms on exactly our kind of dense parts. No mesh shaders in WebGPU — but we don't
> need them: cull in compute, draw the survivors with ONE indirect draw over a compacted
> index range. No METIS, no libraries: a greedy triangle-strip grower is 80 lines.

## Design

- Builder (viewer-side, at `push_mesh` time for meshes over ~50k tris): walk triangles
  in index order, grow a meshlet until 64 verts/124 tris or the bbox exceeds a fraction
  of the object box; emit `Meshlet { first_tri, tri_count, bbox, cone_axis_oct: u32,
  cone_cutoff: f32 }`. Cone = average face normal, cutoff = max deviation dot — a
  back-facing TEST is `dot(view_dir, axis) < -cutoff` (the standard cluster cull).
  Index-order growth exploits the same locality the PDF walk gave 102's batches; run
  the kernel's vertex-cache-friendly ordering first if a scan's order is shuffled.
- Per frame: one thread per meshlet — frustum test (bbox) + cone test → append the
  survivor's index range into a compacted index buffer via an atomic cursor; then one
  `draw_indexed_indirect` of the compacted range per big mesh.
- The compaction write is index COPYING (124×3 u32s per surviving meshlet) — bandwidth,
  not compute; measure before optimizing it away with per-meshlet indirect slots.
- Small meshes (< 50k tris) never enter the lane: cluster overhead loses below that,
  as Bevy's own numbers admit. This is a BIG-mesh lane, opt-in by size.

## Steps (sketch)

1. `meshlets.rs` (viewer): the greedy builder + tests (`#[cfg(test)]`: every triangle
   in exactly one meshlet; cones contain their faces).
2. gpu: meshlet table + compacted index buffer + cursor; `meshlet_cull.wgsl`.
3. The solid pass draws big meshes through the compacted range (`draw_indexed_indirect`),
   others exactly as today.
4. HUD: `meshlets drawn/total` beside the object counts.

## Verify

- The 1.3M-tri scan: orbit → meshlets drawn drops to ~35-50% (backface cone) and vertex
  time with it; zoom into a detail → frustum drops the rest. Target the nvpro ballpark:
  3-4× on the solid pass, measured by bench_frames.
- The builder tests; and a visual A/B — compacted-path render must be pixel-identical
  to the plain path (same triangles, different order — depth test makes order moot).

## Recap

```
Ch 104: MESHLETS. ~124-tri clusters with bbox + normal cone, built greedily in index
        order at load; one compute cull (frustum + cone) compacts surviving index
        ranges; one indirect draw per big mesh. 3-4× on dense scans, zero effect on
        small objects, no mesh shaders needed, no libraries — 80 lines of builder.
```
