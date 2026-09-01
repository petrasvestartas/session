# 112 Quantized meshes — the arena on a diet

> **Big picture.** *Phase 15.* `RenderVertex` spends full f32s on values the screen
> can't distinguish: a position inside a bounded object needs ~16 bits per axis before
> the error is subpixel at any sane zoom, and a normal needs 2 bytes total — we PROVED
> that in 36 when oct16 shaded 342k lion points. Bevy's equivalent change measured ~42%
> off mesh memory with identical renders. Ours: 40 B/vertex → 16.

## Design

- `RenderVertexQ { pos: [u16; 3], nrm_oct: u16, color: u32, _pad: u16 }` — 16 B, wgpu
  vertex formats `Unorm16x4`-friendly (pad position with the oct normal in .w if the
  attribute count matters).
- Dequantization frame = the OBJECT's local box (already computed: `object_bounds` rows).
  The vertex shader does `pos = mix(bbox_min, bbox_max, unorm)` — two fma per vertex —
  with the box carried per-instance (two vec4s appended to the instance row, which is
  a storage buffer read the shader already does).
- The quantizer lives in `push_mesh`: after `to_render()`, fold each vertex against the
  local box. The RAW mesh stays in the kernel `Session` — picking, save and reconcile
  hashes never see quantized data (display format, not truth — the house rule).
- Normals: `oct16` EXISTS (36's helper, scene.rs) — reuse it verbatim; the WGSL decoder
  is `splat.wgsl`'s `oct16_decode`, lifted into a shared include.
- Degenerate guard: a flat object (a sheet's fill) has a zero-thickness box axis —
  clamp each axis extent to ≥1e-6 before dividing, or the unorm divides by zero.

## Steps (sketch)

1. `RenderVertexQ` + the quantize fold in `push_mesh` (behind `VIEWER_VQ=1` first — an
   env flag A/B, like every perf change in this course).
2. Instance rows grow `bbox_min/bbox_max` vec4 pairs; `rebuild_instances` fills them
   from `object_bounds`.
3. `triangle.wgsl`: dequant + oct decode at the top of `vs_main`; nothing downstream
   changes (lit/print paths read the same varyings).
4. Flip the default once the A/B numbers are in the recap.

## Verify

- Goldens within a 0.1% pixel-diff budget at standard zoom (quantization is visible only
  in a diff image, not to eyes — save the diff to prove it).
- Memory panel on the bunny scene: arena bytes ×0.4 (40→16 B/vertex).
- Upload time per file drops proportionally (the parse log's upload column).

## Recap

```
Ch 103: QUANTIZED ARENA. 40 → 16 B/vertex: unorm16 positions dequantized by the
        object box carried on the instance row (two fma), oct16 normals (36's helper,
        shared decoder). Display format only — kernel truth untouched, picking exact.
        A/B behind VIEWER_VQ, adopted on numbers.
```
