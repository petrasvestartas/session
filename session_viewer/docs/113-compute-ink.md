# 113 Compute ink — segments through the splat lane

> **Big picture.** *Phase 15.* Lesson [40](40-compute-splatting.md) proved it for points:
> when millions of primitives are subpixel, atomic compute rasterization beats the vertex
> pipeline. At fit view, a sheet's 350k segments are mostly SHORTER than a pixel, and
> dense hatching means 50 fragment invocations per pixel in the raster lane — the same
> overdraw disease the cloud lane had (420 ms → 87 ms). Segments become a THIRD record
> set writing the SAME `splat_depth_buf`/`splat_color_buf`; the resolve triangle never
> learns there was a new primitive type.

## Design

- One thread per segment: project both endpoints, clip, then DDA the pixel span writing
  `atomicMax(depth_bits)` per covered pixel — `cs_seg_depth` / `cs_seg_color`, the exact
  two-pass shape of `splat.wgsl` (no 64-bit atomics, positive-f32-bits ordering, the
  2D dispatch grid — every trap already documented in 39 applies verbatim).
- Records: reuse the 36-word format — `(first, count, cum, rbits)` indexes the segment
  table instead of the point tables; the matrix column is the sheet row's mvp×model.
- Length cap: a segment longer than ~24 px falls back to the raster lane this frame
  (a compute thread crawling 500 pixels serializes the workgroup). The cull that decides
  is one compare on the projected length — the classic "compute lane for far zoom, quad
  lane for near zoom" split, same as clouds vs glyphs.
- AA without blending: winner-takes-all atomics alias. Add `coverage: array<atomic<u32>>`
  (one per pixel); threads `atomicAdd` fixed-point coverage; resolve divides color by it
  toward the background — Schütz's "high-quality shading" adapted from points to ink.

## Steps (sketch)

1. `splat_seg.wgsl`: project + clip + DDA loop, the two entry points, the coverage add.
2. gpu: `seg_recs` buffer + bind groups (the segment table is ALREADY a storage buffer —
   the flat lane reads it; the compute lane binds the same buffer read-only).
3. Prelude: third record build (over planar docs' segment ranges when their projected
   density crosses the threshold) + third dispatch pair; depth passes of ALL record sets
   before ANY color pass (42's ordering law, now with three lanes).
4. The per-sheet toggle composes with 100: impostor OFF + far zoom → compute ink;
   impostor ON → neither lane runs.

## Verify

- Fit view, impostors disabled: presented fps (rAF probe, never the counter) at least
  doubles on the sheet-heavy scene; hatched regions show the biggest win.
- Zoom in: sheets cross the length cap back to the raster lane with no visual pop
  (screenshot both sides of the boundary).
- The coverage resolve: hairlines at fit view look GRAY, not sparkling black dust.

## Recap

```
Ch 101: COMPUTE INK. Segments join points in the atomic pixel race: DDA per thread,
        two passes, shared pixel buffers, third record set. Length-capped so near zoom
        stays on the AA raster lane; atomicAdd coverage buys back antialiasing at fit.
        Hatching overdraw dies the same death the cloud fit-view did in 39.
```
