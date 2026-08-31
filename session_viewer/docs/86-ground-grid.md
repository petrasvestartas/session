# 86 Analytic ground + infinite grid — the stage

> **Big picture.** *Phase 11 — rendering quality, engineered FAST (70–74).* The archive's pretty
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
    _pad0: f32,                   // three SCALAR pads, not one vec3<f32> — see the byte-map below
    _pad1: f32,
    _pad2: f32,
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
    // Unproject this pixel at two depths → the camera ray (54's math, on the GPU, camera-relative).
    let n4 = g.inv_view_proj * vec4<f32>(in.ndc, 1.0, 1.0);    // reverse-Z near
    let f4 = g.inv_view_proj * vec4<f32>(in.ndc, 0.5, 1.0);    // well-conditioned far (46's rule!)
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

> **Why three scalar pads, not `_pad: vec3<f32>`.** The Rust mirror uploaded per frame is
> `mat4 + vec4 + f32 + [f32; 3]` = **96 B**. A `vec3<f32>` in WGSL carries **16-byte alignment**, so
> after `ground_z` ends at byte 84 the compiler bumps the `vec3` to 96, pads the struct out to **112 B**,
> and the min-binding-size no longer matches the 96 B Rust upload — the bind panics at first frame.
> Three plain `f32` pads sit at 84/88/92, keeping WGSL at 96 B, exact with Rust.

<svg viewBox="0 0 520 148" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="GroundUniform byte map: a vec3 pad forces 16-byte alignment leaving an 84 to 96 hole and inflating the WGSL struct to 112 bytes versus the 96-byte Rust mirror; three scalar pads fix it" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="10" y="16" fill="#888">GroundUniform — offsets in bytes</text>

  <text x="10" y="48" fill="#e06c6c">vec3 pad ✗</text>
  <rect x="120" y="36" width="205" height="20" fill="none" stroke="#555"/>
  <text x="222" y="50" fill="#888" text-anchor="middle">inv_view_proj (64 B)</text>
  <rect x="325" y="36" width="51" height="20" fill="none" stroke="#6fb3ff"/>
  <text x="350" y="50" fill="#6fb3ff" text-anchor="middle">vec4</text>
  <rect x="376" y="36" width="13" height="20" fill="none" stroke="#d7dae0"/>
  <text x="382" y="70" fill="#666" text-anchor="middle" font-size="9">z</text>
  <rect x="389" y="36" width="38" height="20" fill="none" stroke="#e06c6c" stroke-dasharray="3 2"/>
  <text x="408" y="50" fill="#e06c6c" text-anchor="middle" font-size="9">hole</text>
  <rect x="427" y="36" width="38" height="20" fill="none" stroke="#e06c6c"/>
  <text x="446" y="50" fill="#e06c6c" text-anchor="middle" font-size="9">vec3</text>
  <text x="472" y="50" fill="#e06c6c">= 112 B</text>
  <text x="382" y="30" fill="#666" text-anchor="middle" font-size="9">84</text>
  <text x="427" y="30" fill="#666" text-anchor="middle" font-size="9">96</text>

  <text x="10" y="112" fill="#5bbf87">3 scalars ✓</text>
  <rect x="120" y="100" width="205" height="20" fill="none" stroke="#555"/>
  <text x="222" y="114" fill="#888" text-anchor="middle">inv_view_proj (64 B)</text>
  <rect x="325" y="100" width="51" height="20" fill="none" stroke="#6fb3ff"/>
  <text x="350" y="114" fill="#6fb3ff" text-anchor="middle">vec4</text>
  <rect x="376" y="100" width="13" height="20" fill="none" stroke="#d7dae0"/>
  <text x="382" y="134" fill="#666" text-anchor="middle" font-size="9">z</text>
  <rect x="389" y="100" width="13" height="20" fill="none" stroke="#5bbf87"/>
  <rect x="402" y="100" width="13" height="20" fill="none" stroke="#5bbf87"/>
  <rect x="415" y="100" width="13" height="20" fill="none" stroke="#5bbf87"/>
  <text x="408" y="114" fill="#5bbf87" text-anchor="middle" font-size="9">f×3</text>
  <text x="434" y="114" fill="#5bbf87">= 96 B — matches Rust</text>
</svg>

Two details carrying earlier lessons' weight: the unproject uses `ndc_z = 0.5` for the far point —
46's conditioning rule applies on the GPU too — and everything is **camera-relative** (the uniform
feeds `inverse(view_proj)` of the already-rebased matrix and `ground_z = −origin.z`), so the ground
never shimmers at 10 km, same as every object since 33.

> **Three honest caveats on this shader.**
> **Blend × depth-write.** The fade band alpha-blends yet writes *full* `frag_depth` — a
> half-faded ground fragment still occludes whatever is behind it. In practice the band sits at
> the horizon where nothing real lives; but if you ever fade the ground *near* geometry (small
> scenes, top-down views), objects below the plane will be hard-clipped by an invisible surface.
> The fix, when needed: fade by dithered `discard` instead of alpha, and keep the depth write.
> **The near-parallel seam.** As `dir.z → 0`, `t → ∞` and the hit coordinate loses f32 precision —
> a thin shimmering band right at the horizon, exactly where the eye is looking. The fade radius
> hides it only if the fade completes *before* that distance; if you see the seam, tighten
> `cam_rel_eye.w`'s inner smoothstep edge, don't touch the ray math.
> **`fwidth` AA breaks down at the horizon.** The optional grid upgrade's antialiasing assumes
> `fwidth(hit.xy / step)` is about one pixel — at grazing angles the derivative explodes, the
> "1 px stroke" becomes the whole screen, and the grid dissolves into gray mush near the horizon.
> Clamp the computed line width (or fade the grid out with distance, like the ground itself)
> before it reaches that regime.

## Step 2 — pipeline + draw: `src/engine/pipelines/build.rs` + `gpu/mod.rs`

`build_ground_pipeline` is the background pipeline's shape (fullscreen, no vertex buffers) with two
changes: **alpha blending on** (the fade) and **depth write ON, compare Greater** (reverse-Z — the
ground is real geometry that occludes and is occluded; `frag_depth` makes that exact). Construct it
**inside `Pipelines::new(device, samples, …)`, from the `samples` it receives** — never as a
standalone one-off. MSAA is dynamic here (`msaa_for` returns 1 for flat-only scenes, 4 once any
solid exists), and `set_scene` rebuilds *all* pipelines on the 1×↔4× flip; a ground pipeline pinned
to one sample count panics on the first solid append with a sample-count mismatch:

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
4×4 inverse — fixed during lesson 63; this matrix contains the projection, which the old affine-only
version got wrong — and it returns `Option<Xform>`, so unwrap or early-out), `eye − origin`, fade
radius ≈ 30× `camera.distance_world()` (feels infinite without banding), and `−origin[2]`. The
radius **must** come from `distance_world()` — that's the camera distance in world **mm**;
`camera.distance` itself is in *metres*, and using it makes the fade 1000× too tight: the floor
dies out a hand-span from the camera. (Alternative: derive the radius from `gpu.scene_min`/
`scene_max` — refreshed each `set_scene` — so the fade scales with the scene instead of the zoom.)

> **Grid upgrade (optional but natural here).** Lesson 20's vertex grid is 50 fixed vertices (25 segments) — fine on
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
- Objects sit *on* it: their bases are occluded exactly at contact (the `frag_depth`), the ambient
  occlusion to come in 72 will land on it, and the grid draws crisply over it with no z-fighting at
  grazing angles.
- Perf HUD: +1 draw call, fullscreen but almost branchless — frame time moves barely at all. The
  stage is set for the money lessons: render-on-demand (78) and GTAO (79).

## Recap

```
Ch 69: Phase 10 closed — all geometry first-class.
Ch 70: THE STAGE. Analytic ground: a fullscreen triangle whose FRAGMENTS each unproject their pixel
       (ndc_z 0.5 far — 46's conditioning rule holds on the GPU), intersect z=0 in CAMERA-RELATIVE
       space (33's frame: inv of the rebased view_proj, ground_z = −origin.z — no 10 km shimmer),
       shade arctic white with a smoothstep horizon fade, and write EXACT frag_depth (clip z/w) so
       occlusion is real. Depth write ON, Greater (reverse-Z), alpha blend for the fade. NEVER a
       giant quad — f32 far-corner shimmer + interpolated-depth z-fights, both archive-verified.
       Optional: the same hit powers a fract/fwidth INFINITE grid, retiring 20's 50-vertex one.
       Caveats, all at the horizon: the fade band blends yet writes FULL depth (dithered discard
       if it ever clips real geometry); the near-parallel ray loses precision in a thin seam;
       fwidth AA dissolves at grazing angles — fade out before all three.
```

Edited: `shaders/ground.wgsl` (NEW), `engine/pipelines/build.rs` (`build_ground_pipeline`),
`engine/gpu/mod.rs` (uniform + draw between background and grid).

## Next

`87-render-on-demand.md` — the single biggest perf win in the whole course, and it never touches the
image: draw only when something changed. CAD apps do this; games can't. A static scene costs zero
GPU; the frame that IS drawn is always full quality — the user rule, honored by architecture.
