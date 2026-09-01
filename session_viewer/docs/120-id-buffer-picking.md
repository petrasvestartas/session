# 120 Id-buffer picking — hover and marquee stop scaling with the scene

> **Big picture.** *Phase 15.* The CPU ray (57) is exact and resolves topology — keep
> it as the CLICK. But hover highlight runs every mouse move, and a marquee over dense
> linework tests thousands of segments; both scale with the scene. The GPU already
> rasterizes every visible primitive — let it also write WHO: an r32uint id per pixel,
> then read back one pixel (hover) or the marquee rect. O(pixels), whatever the object
> count. WebGPU readback is async — a ring of small mapped buffers eats the 1-2 frame
> latency without ever stalling.

## Design

- Id pass: every lane's shader gains a sibling pipeline writing `instances[id]`'s row
  to an `R32Uint` target (fragment returns `vec4u(row+1)`; 0 = background). Runs ONLY
  when something wants it: hover (throttled to ~30 Hz), or an active marquee. Reuses
  each lane's vertex shader; the fragment is three lines.
- Readback ring: 3 × `MAP_READ|COPY_DST` buffers. Frame N copies the cursor texel (or
  marquee rect) into ring[N%3] and calls `map_async`; the handler stores the result;
  the UI reads the freshest resolved value. Never block, never `device.poll` on wasm
  (the browser polls).
- Hover: 1×1 copy at the cursor → row → highlight flag (60's tint). Latency 1-2 frames
  — imperceptible for hover, and the CPU ray still answers the actual click exactly.
- Marquee: copy the rect, build the `HashSet<u32>` of rows CPU-side (a 400×300 marquee
  is 480 KB — fine), map rows→guids for selection. Crossing-vs-window semantics: the
  id-buffer gives WINDOW (fully-or-partially visible pixels); for CAD's crossing-select
  the pixel set IS the crossing answer — anything that painted a pixel in the rect.
- Sub-entity later: an `Rg32Uint` target carries (row, subentity) when 58's vertex/edge
  hover moves over — same machinery, one more channel.

## Steps (sketch)

1. `id.wgsl` fragments per lane + the `R32Uint` target + pipelines (share layouts).
2. The ring (`PickRing { bufs: [Buffer; 3], pending: [Option<PickReq>; 3] }`) and its
   `resolve()` polled from `State::render`.
3. Hover wiring behind a flag; marquee path replaces the CPU rect test in 60 for scenes
   over a size threshold (keep the CPU path — headless selftest has no async frames).
4. Occluded-cycling (Rhino's click-through): re-run the id pass with already-picked rows
   suppressed — a second copy, only on the repeat-click gesture.

## Verify

- Hover over the 210k-object scene: flat cost regardless of what's under the cursor
  (perf HUD), highlight lags ≤2 frames (count them with a high-speed capture if bored).
- Marquee over dense hatching: instant, and matches the CPU marquee's result on a small
  scene (assert set equality in a test scene — the two paths must agree).
- Click precision unchanged — clicks still go through the CPU ray.

## Recap

```
Ch 107: ID BUFFER. A uint id per pixel from sibling pipelines; a 3-deep async readback
        ring; hover = 1 texel, marquee = the rect's pixel set — O(pixels), scene-size
        blind. CPU ray keeps the click and the topology; the selftest keeps the CPU
        marquee. Latency hidden, never stalled.
```
