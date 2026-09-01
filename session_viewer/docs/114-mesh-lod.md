# 114 Mesh LOD — fewer triangles when they'd be smaller than pixels

> **Big picture.** *Phase 15.* The full Nanite LOD DAG is months of work (seam locking,
> graph partitioning); Bevy's own posts document the grind. The 90% answer is DISCRETE:
> 2-3 simplified levels per big mesh, picked by projected size. Curved types get LOD for
> FREE — `mesh_q(angle, chord)` at coarser quality IS the simplifier, and the tess cache (46–49)
> can hold a level per quality. Scanned meshes need a real simplifier — quadric error
> metrics (QEM), the classic, ~200 lines, viewer-side (display only; the kernel's truth
> is never simplified).

## Design

- QEM in brief: every vertex accumulates a 4×4 quadric (sum of its faces' plane outer
  products); an edge collapse's cost is the quadric error at the optimal collapsed
  position; greedily collapse the cheapest edges (binary heap) until the target count.
  Guard rails: skip collapses that flip a face normal (dot with old normal < 0) or
  touch a boundary edge (edge with one face) — CAD scans have real borders.
- Levels: L0 = full, L1 = 25%, L2 = 6% triangle count, built lazily on first demand
  per level (a background-ish task: build L1 the first frame the mesh is small on
  screen — one frame of hitch beats always paying).
- Selection: projected object-box height in pixels vs thresholds (~400 px → L1,
  ~100 px → L2), with hysteresis (switch back up at 1.3× the threshold) so orbiting at
  a boundary doesn't flicker. Composes with 104: each level gets its own meshlets.
- Storage: levels live in the tess/LOD cache beside 46's entries, keyed (guid, level);
  the arena uploads the ACTIVE level's range (50's per-object arena makes the swap a
  free+alloc, not a rebuild).

## Steps (sketch)

1. `simplify.rs`: quadrics, heap, collapse loop, the two guards; tests on a sphere
   (volume within 2% at L1) and a bordered patch (boundary vertices unmoved).
2. Cache + lazy build + level pick in the walk/draw path; instance flag carries the
   active level so picking can warn (picks still hit L0 truth via the kernel mesh).
3. Curved types: thread a quality index into the tess-cache fill's `mesh_q` call — their
   "simplifier" is re-tessellation, cached per level like everything else.

## Verify

- The scan at fit view drops to L2: silhouette visually identical at that size (SSIM or
  eyeball), solid-pass time ÷ ~3 on top of 104's cull.
- Orbit across a threshold: one swap, no flicker (hysteresis working).
- Boundary test: a trimmed sheet's cut edge stays exact at every level.

## Recap

```
Ch 105: DISCRETE LOD. QEM edge-collapse (~200 lines, viewer-only, kernel truth intact):
        L1 25% / L2 6%, picked by projected size with hysteresis, lazily built, stored
        beside the tess cache, meshlet-composed. Curved types re-tessellate coarser via
        mesh_q instead — their exact geometry IS the LOD source. The Nanite DAG stays
        unbuilt on purpose.
```
