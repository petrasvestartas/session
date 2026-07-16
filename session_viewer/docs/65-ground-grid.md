# 65 Analytic ground + infinite grid — the stage

> **Big picture.** *Phase 11 — rendering quality, engineered FAST (65–69).* The archive's pretty
> "arctic" look cost ~200 texture fetches per pixel per frame and crawled on integrated GPUs; this
> phase rebuilds the same look on a budget, under one **user rule: quality must never drop while
> interacting** — the savings come from architecture (skip unchanged frames, half-res AO), never from
> degrading motion. First the stage the look stands on: a white ground plane reaching the horizon
> with a distance fade — computed **analytically per pixel**, because the obvious alternative (a
> gigantic quad) genuinely flickers.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a fullscreen triangle's fragments each intersect the camera ray with the z zero plane; hits shade ground with a horizon fade and write exact depth" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <line x1="20" y1="70" x2="660" y2="70" stroke="#3a3a3a"/>
  <circle cx="90" cy="34" r="7" fill="none" stroke="#d7dae0"/><text x="90" y="22" fill="#888" text-anchor="middle">camera</text>
  <line x1="96" y1="38" x2="300" y2="70" stroke="#6fb3ff" stroke-width="1.2" marker-end="url(#ah65)"/>
  <line x1="96" y1="36" x2="530" y2="70" stroke="#6fb3ff" stroke-width="1.2" opacity="0.6" marker-end="url(#ah65)"/>
  <line x1="96" y1="33" x2="660" y2="62" stroke="#6fb3ff" stroke-width="1.2" opacity="0.3"/>
  <text x="300" y="88" fill="#d7dae0" text-anchor="middle">hit: solid</text>
  <text x="530" y="88" fill="#888" text-anchor="middle">far: faded</text>
  <text x="645" y="52" fill="#666" text-anchor="middle">miss: sky</text>
  <text x="340" y="116" fill="#666" text-anchor="middle">per-pixel ray ∩ z=0 in the fragment shader — exact at every distance, writes exact frag_depth</text>
  <defs><marker id="ah65" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Why never a giant quad

The tempting version — a 1,000,000-unit quad at `z = 0` — fails twice, and the archive hit both: its
far corners overflow f32 precision after the camera transform, so the surface **shimmers** as you
orbit (the same disease 33 cured for objects), and its interpolated depth fights the grid lines at
grazing angles. The analytic plane has neither problem: every fragment computes its own exact
ray–plane hit in camera-relative space, and writes its own exact `frag_depth`. Zero vertices, perfect
at every distance, one fullscreen triangle.

## Files we touch

```
src/shaders/ground.wgsl        # NEW — fullscreen triangle; fs does ray ∩ z=0, fade, frag_depth
src/engine/pipelines/build.rs  # build_ground_pipeline (depth WRITE on, compare Greater)
src/engine/gpu/mod.rs          # draw it after the background, before the meshes
```

## Step 1 — the shader: `src/shaders/ground.wgsl`

The vertex stage is 25's buffer-less fullscreen triangle. The fragment stage reconstructs the
camera ray through its pixel — using the **inverse view-projection** supplied per frame (the same
matrix 41 inverts on the CPU; here it rides a uniform) — intersects `z = 0`, shades, and fades:

```wgsl
struct GroundUniform {
    inv_view_proj: mat4x4<f32>,   // camera-relative clip → camera-relative world (33's frame!)
    cam_rel_eye: vec4<f32>,       // eye − origin (xyz), fade radius (w)
    ground_z: f32,                // plane height in camera-relative space: −origin.z
    _pad: vec3<f32>,
};
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> g: GroundUniform;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) ndc: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> VsOut {
    // one triangle covering the screen (lesson 25's trick)
    let xy = vec2<f32>(f32((vi << 1u) & 2u), f32(vi & 2u)) * 2.0 - 1.0;
    var o: VsOut;
    o.pos = vec4<f32>(xy, 0.0, 1.0);
    o.ndc = xy;
    return o;
}

struct FsOut {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32,
};

@fragment
fn fs_main(in: VsOut) -> FsOut {
    // Unproject this pixel at two depths → the camera ray (41's math, on the GPU, camera-relative).
    let n4 = g.inv_view_proj * vec4<f32>(in.ndc, 1.0, 1.0);    // reverse-Z near
    let f4 = g.inv_view_proj * vec4<f32>(in.ndc, 0.5, 1.0);    // well-conditioned far (41's rule!)
    let p0 = n4.xyz / n4.w;
    let dir = normalize(f4.xyz / f4.w - p0);

    // ray ∩ plane z = ground_z (the WORLD z=0 expressed camera-relative)
    let denom = dir.z;
    if (abs(denom) < 1e-7) { discard; }
    let t = (g.ground_z - p0.z) / denom;
    if (t <= 0.0) { discard; }                                  // plane is behind the camera
    let hit = p0 + dir * t;

    // horizon fade: alpha falls off with distance from the eye's footprint
    let d = length(hit.xy - g.cam_rel_eye.xy);
    let alpha = 1.0 - smoothstep(g.cam_rel_eye.w * 0.55, g.cam_rel_eye.w, d);
    if (alpha < 0.003) { discard; }

    // exact depth so real geometry occludes correctly (project the hit, take clip z/w)
    let clip = mvp * vec4<f32>(hit, 1.0);
    var o: FsOut;
    // arctic white, premultiplied by blend
    o.color = vec4<f32>(0.985, 0.985, 0.99, alpha);
    o.depth = clip.z / clip.w;
    return o;
}
```

Two details carrying earlier lessons' weight: the unproject uses `ndc_z = 0.5` for the far point —
41's conditioning rule applies on the GPU too — and everything is **camera-relative** (the uniform
feeds `inverse(view_proj)` of the already-rebased matrix and `ground_z = −origin.z`), so the ground
never shimmers at 10 km, same as every object since 33.

## Step 2 — pipeline + draw: `src/engine/pipelines/build.rs` + `gpu/mod.rs`

`build_ground_pipeline` is the background pipeline's shape (fullscreen, no vertex buffers) with two
changes: **alpha blending on** (the fade) and **depth write ON, compare Greater** (reverse-Z — the
ground is real geometry that occludes and is occluded; `frag_depth` makes that exact):

```rust
        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(true),
            depth_compare: Some(wgpu::CompareFunction::Greater),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
```

Draw order in `clear()`: background gradient (25) → **ground** → grid (20) → meshes/lines/points.
The per-frame uniform fills from values already at hand: `view_proj.inverse()` (the kernel's full
4×4 inverse — fixed during lesson 41; this matrix contains the projection, which the old affine-only
version got wrong), `eye − origin`, fade radius ≈ 30× the camera distance (feels infinite without
banding), and `−origin[2]`.

> **Grid upgrade (optional but natural here).** Lesson 20's vertex grid is 50 fixed lines — fine on
> the demo, small on a big scene. The same analytic trick renders an **infinite** grid: in this very
> shader (or a sibling), `fract(hit.xy / step)` finds distance to the nearest grid line,
> `fwidth` turns it into an antialiased ~1 px stroke, two scales (minor/major) blend by zoom. If you
> take the upgrade, the 20 pipeline retires and its draw call goes with it; the axes lines (X red /
> Y green) stay as the two colored segments they already are.

## Step 3 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- A white floor now reaches the horizon in every direction — orbit low: it *ends* in a soft fade, not
  a hard polygon edge, and there is **no shimmer** anywhere (the giant-quad disease — if you want to
  see it once, replace the shader with a huge quad and orbit; then come back).
- Objects sit *on* it: their bases are occluded exactly at contact (the `frag_depth`), shadows-of-
  -occlusion to come in 67 will land on it, and the grid draws crisply over it with no z-fighting at
  grazing angles.
- Perf HUD: +1 draw call, fullscreen but almost branchless — frame time moves barely at all. The
  stage is set for the money lessons: render-on-demand (66) and GTAO (67).

## Recap

```
Ch 64: Phase 10 closed — all geometry first-class.
Ch 65: THE STAGE. Analytic ground: a fullscreen triangle whose FRAGMENTS each unproject their pixel
       (ndc_z 0.5 far — 41's conditioning rule holds on the GPU), intersect z=0 in CAMERA-RELATIVE
       space (33's frame: inv of the rebased view_proj, ground_z = −origin.z — no 10 km shimmer),
       shade arctic white with a smoothstep horizon fade, and write EXACT frag_depth (clip z/w) so
       occlusion is real. Depth write ON, Greater (reverse-Z), alpha blend for the fade. NEVER a
       giant quad — f32 far-corner shimmer + interpolated-depth z-fights, both archive-verified.
       Optional: the same hit powers a fract/fwidth INFINITE grid, retiring 20's 50-line one.
```

Edited: `shaders/ground.wgsl` (NEW), `engine/pipelines/build.rs` (`build_ground_pipeline`),
`engine/gpu/mod.rs` (uniform + draw between background and grid).

## Next

`66-render-on-demand.md` — the single biggest perf win in the whole course, and it never touches the
image: draw only when something changed. CAD apps do this; games can't. A static scene costs zero
GPU; the frame that IS drawn is always full quality — the user rule, honored by architecture.
