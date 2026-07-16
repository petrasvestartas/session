# 26 Reverse-Z — depth precision that survives the whole scene

Lesson 23's edges poked through the box; clip-range and edge-nudge band-aids (24) only worked *around*
it. The real disease: **f32 depth precision collapses in the distance.** Reverse-Z cures it properly,
closing the depth story lesson 12 opened — the last camera lesson.

## Why depth precision collapses — and how reversing fixes it

A perspective matrix stores depth as `~1/z`: most of the `0→1` range goes to the scene's first slice,
and everything far away crushes into a sliver near `1.0`. Measured: a point halfway out lands at
**0.991**, packed against its neighbours — f32 can't tell them apart → z-fighting.

```
standard depth   near ─0.0──────────────────────────0.99──1.0─ far   ← everything far crammed here
                       │ lots of precision │  ← wasted on near │
```

The trick: an **f32 float** already has most of *its* precision near **0.0** (the exponent packs tiny
steps there). Flip depth so **far sits at 0.0, near at 1.0**, and the two lopsided curves cancel — fine
float steps land exactly where perspective crushes things, giving near-**uniform** precision. Same
measurement, reversed: the midpoint moves to **0.009**, where the float has bits to spare.

```
reverse-Z        far ─0.0──────────────────────────────────1.0─ near
                      │ float's fine steps live here, where perspective crushed depth │
```

It costs nothing and needs no new buffer — we already render to a **Depth32Float** target (lesson
12/24), the half that makes reverse-Z work. Four small flips turn it on:

```
1. projection   swap near/far args      → near maps to 1, far maps to 0
2. depth test   Less → Greater          → "nearer" now means LARGER depth
3. depth clear  1.0 → 0.0               → the cleared buffer starts at the far plane (0)
4. edge nudge   −1e-5 → +1e-5           → pull edges toward the camera = toward LARGER depth now
```

The background pipeline keeps `Always` (depth-direction-agnostic); everything else that used `Less`
flips to `Greater`.

## Files we touch

```
src/engine/camera.rs               # swap near/far in perspective AND ortho
src/engine/pipelines/build.rs      # Less → Greater on triangle, grid, edges (NOT background)
src/engine/gpu.rs                  # depth clear 1.0 → 0.0
src/shaders/edges.wgsl             # nudge sign −1e-5 → +1e-5
```

## Step 1 — reverse both projections: `src/engine/camera.rs`

In `view_proj`, swap the last two arguments of **both** projection calls — that single swap produces
the reversed mapping (verified: near→1, far→0 for perspective *and* ortho):

```rust
        let projection = if self.perspective {
            //                                          near ↓        far ↓   — swapped
            Xform::perspective(f64::to_radians(60.0), aspect, dist * 10.0, dist * 0.01)
        } else {
            let h = dist * f64::to_radians(30.0).tan();
            let r = dist * 100.0;
            Xform::orthographic(-aspect * h, aspect * h, -h, h, r, -r)   // near/far swapped: r, -r
        };
```

(Ortho depth is linear, so reversing it doesn't improve *its* precision — but both projections must
map far→0 / near→1 for the single depth-test direction below to work for both.)

## Step 2 — flip the depth test: `src/engine/pipelines/build.rs`

In three pipelines — `build_triangle_pipeline`, `build_grid_pipeline`, `build_edges_pipeline` — change
the depth compare from `Less` to `Greater`. With far at 0 and near at 1, the **nearer** fragment now
has the **larger** depth, so "keep nearer" means `Greater`:

```rust
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                // (false for grid/edges — unchanged)
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Greater),   // ← was Less
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
```

**Leave `build_background_pipeline` alone** — its `Always` compare doesn't care which way depth runs;
flip it and the gradient starts failing the test.

## Step 3 — clear depth to the far value: `src/engine/gpu.rs`

The depth buffer must now start at the *far* plane, `0.0` instead of `1.0`. In `clear()`, the depth
attachment's clear value:

```rust
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0.0),   // ← was 1.0 (far is 0 under reverse-Z)
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
```

Miss this and the whole scene fails the `Greater` test against a `1.0`-filled buffer — a blank frame,
only the depth-agnostic background left.

## Step 4 — flip the edge nudge: `src/shaders/edges.wgsl`

The nudge still pulls edges a hair toward the camera so they beat the face they sit on — but "toward
the camera" now means **larger** depth, so it flips from subtract to add:

```wgsl
    o.pos.z = o.pos.z + 1e-5 * o.pos.w;   // was  - 1e-5  (reverse-Z: nearer = larger z)
```

Leave the sign as `−` and every edge sinks *behind* its face instead — they vanish.

## Step 5 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Up close it looks identical — near still occludes far, edges still sit on their faces. The win shows
when you **zoom way out**: the far-field z-fighting and shimmering standard depth gives at distance is
gone, precision now spread evenly instead of hoarded near the camera. (Sanity check: flip *only* the
compare to `Greater` without Step 3 and the scene goes blank — proof the clear value and test direction
are a matched pair.)

## Recap

```
Ch 24/23: we dodged far z-fighting with a tight clip range + an edge depth nudge — band-aids.
Ch 26:    reverse-Z cures it. Perspective's 1/z crush and an f32 float's precision-near-0 are
          opposite curves; mapping far→0 / near→1 cancels them for uniform precision on a
          Depth32Float target. Four flips: swap near/far in both projections; depth test Less→
          Greater; depth clear 1.0→0.0; edge nudge −→+. Background stays Always. No new buffers.
```

Edited: `camera.rs` (near/far swapped in perspective + ortho), `pipelines/build.rs` (`Greater` on
triangle/grid/edges), `gpu.rs` (depth clear `0.0`), `shaders/edges.wgsl` (`+1e-5` nudge).

## Next

`27-webgpu-only.md` — Phase 4 opens. Storage buffers and compute are still forbidden under
`downlevel_webgl2_defaults`; lesson 27 drops the WebGL fallback, switches to `Limits::default()`, and
adds a "WebGPU required" overlay — unlocking the instancing, GPU arena, and cylinder-line lessons that
carry the app to real scale.
