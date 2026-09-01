# 114 Segment batches — cull ink before it costs

> **Big picture.** *Phase 15.* Schütz 2022 batches points (10k per batch + a box) so the
> GPU can drop whole batches before threads spawn. Our sheets deserve the same: zoomed
> into one corner, 95% of a sheet's segments are outside the view, yet every one costs a
> vertex-shader clip today. Batches also unlock COMPRESSION: endpoints stored relative to
> the batch box need 16 bits per axis, not 32.

## Design

- At walk time, chunk each planar doc's segment range into batches of 4096 with an AABB
  each (`Vec<SegBatch { first, count, bbox_min, bbox_max, row }>` on the sheet's lane).
  Walk order is already spatial for PDF sheets (the importer emits in page order), so no
  clustering pass is needed — measure the box overlap and only sort if it's pathological.
- Per frame, one small compute pass: batch AABB vs the view frustum (55's six planes,
  already in a uniform) → write survivors' `(first_index, count)` into a compacted
  `draw_indexed_indirect` argument buffer; ONE indirect draw per sheet replaces the
  all-segments draw. (Browser WebGPU has no multi_draw_indirect_count — emit fixed-slot
  args and zero the culled counts; the GPU skips zero-count draws for the cost of a
  command.)
- Quantized storage (second step, optional until memory hurts): `SegQ { p0: [u16; 3],
  p1: [u16; 3], color: u32, width_r: u16, _pad: u16 }` = 20 B vs the 32 B
  `CylinderSegment`; dequantize by batch box in the vertex shader (two fma). The RAW
  table stays for picking — compression is a DISPLAY format, the same f64→f32 boundary
  rule the whole viewer follows (display lies a little, truth stays on the CPU).

## Steps (sketch)

1. Walk: emit `SegBatch` rows beside the sheet's segment range (grow_bounds already
   computes what the boxes need — reuse the loop).
2. `seg_cull.wgsl`: one thread per batch, six plane dots, write indirect args.
3. gpu: the arg buffer (`INDIRECT | STORAGE`), the cull dispatch in the prelude
   (before the render pass), `draw_indexed_indirect` in the flat-ink lane.
4. Later: the `SegQ` table + dequant in `ribbon.wgsl`/`cylinder.wgsl` behind a lane flag.

## Verify

- Zoom into a sheet corner: `drawn/total` on the HUD shows batches culled ~95%; orbit at
  the edge — nothing pops (test the boxes with the AABB-intersects-but-center-outside
  case, 55's classic).
- Perf: vertex time of the flat lane drops proportionally (bench_frames before/after).
- Quantized step: goldens shift by at most 1 px of line placement at max zoom-out
  (quantization error = batch_extent / 65535 — compute it, assert it subpixel).

## Recap

```
Ch 102: SEG BATCHES. 4096-segment batches with AABBs, GPU frustum cull into indirect
        draws — zoomed-in sheets stop paying for their off-screen 95%. Batch-relative
        snorm16 endpoints cut the display table 32 → 20 B/segment; the raw table keeps
        the truth for picking. Schütz's batch idea, applied to ink.
```
