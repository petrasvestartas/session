# 100 Sheet impostors — a drawing becomes a texture until you look closely

> **Big picture.** *Phase 15 — at scale (100–107): the research findings of
> `_RESEARCH_GPU_CAD.md` implemented as source, no third-party code.* A 2D sheet is
> STATIC — the same 350k segments every frame — and most frames it covers a few hundred
> pixels. Rasterizing it once into a texture and drawing ONE quad bounds the per-frame
> cost by the SCREEN, not the dataset. This is what PDF viewers and map renderers do
> (tiles + zoom pyramid); vello's "sparse strips" is the same insight one level deeper.
> We adapt the idea to our own pipeline — we already own an offscreen renderer.

## Design

- `SheetImpostor { texture, view, zoom_bucket: i32, world_rect: [f64; 4], row: u32 }` —
  one per PLANAR doc (the walk's `planar` flag already identifies sheets).
- Rasterize with the machinery we have: a scoped `render_offscreen` of ONLY that sheet's
  rows (a one-doc row range — instance flags hide the rest for the offscreen pass), at a
  resolution derived from the sheet's current screen size, rounded UP to the next
  power-of-two "zoom bucket" so small zoom changes don't re-rasterize.
- Per frame: project the sheet's world rect; if `screen_px` fits the cached bucket, draw
  the impostor quad (textured, alpha-clipped, depth-tested at the sheet plane) and SKIP
  the sheet's segment/glyph rows via their instance flags. If the camera zooms past the
  bucket (or the sheet is edited — reconcile's `changed` includes any of its guids),
  re-rasterize into the next bucket and swap.
- Hysteresis: switch to live geometry when the sheet exceeds ~1.5x the texture's texel
  density on screen — near zoom must always be REAL vectors (measure text/hairlines).
- Memory budget: an LRU of at most N textures (default 8 × 2048²×RGBA8 = 128 MB ceiling;
  evict farthest sheet). A 10-sheet scene idles at ONE texture per visible sheet and zero
  vector work.

## Steps (sketch — retarget at typing time)

1. `engine/gpu`: an `impostor` module — texture cache keyed by doc row-range, an
   `impostor.wgsl` (textured quad, `discard` on alpha 0, sheet-plane depth), one pipeline.
2. `render_offscreen_rows(range, w, h)` — the existing offscreen path with a row filter:
   set FLAG_HIDDEN on everything outside the range for the pass, restore after (the flags
   round-trip is two `write_buffer`s over the instance table).
3. `State::render`: the bucket decision per sheet, the swap, the flag toggles.
4. Reconcile hook: `changed`/`added`/`removed` guid in a sheet's doc → invalidate its
   impostor (one `HashMap::remove`).

## Verify

- Fit view of the 10-sheet PDF scene: `drawn` collapses from ~350k segments to N quads +
  the 3D geometry; wheel-zoom into a sheet: it swaps to live vectors before text blurs
  (screenshot the swap boundary — the two frames must be visually identical).
- Memory panel: texture cache plateaus at the LRU cap; zooming across all sheets evicts.
- Edit a sheet object (delete a line): the impostor refreshes on the next frame.

## Recap

```
Ch 100: IMPOSTORS. A static sheet renders once into a zoom-bucketed texture; per-frame
        cost becomes screen-bound. Own machinery only: render_offscreen + a quad pipeline.
        Hysteresis keeps near zoom vector-true; LRU bounds memory; reconcile invalidates.
        The vello "retained raster" idea without the dependency.
```
