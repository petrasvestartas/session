# 32b Points II — billboard clouds at scale

> **Big picture.** *Phase 4 — one scene, one draw call.* 32a gave handles a real instanced sphere —
> 144 triangles each, fine for dozens. A `PointCloud` has *millions*, and it doesn't need 3-D
> roundness: a flat circle that always faces the camera looks identical at point sizes and costs
> **2 triangles instead of 144**. This is the standard scale trick in every CAD/point-cloud viewer:
> spend geometry only where the eye can tell the difference.

<svg viewBox="0 0 380 172" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a 6-vertex quad whose fragment shader draws an anti-aliased circle" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="190" y="18" fill="#888" text-anchor="middle">PointCloud — millions, 2-D is enough</text>
  <rect x="90" y="52" width="56" height="56" fill="none" stroke="#3a3a3a"/>
  <line x1="90" y1="52" x2="146" y2="108" stroke="#3a3a3a"/>
  <circle cx="118" cy="80" r="24" fill="none" stroke="#6fb3ff" stroke-width="1.5"/>
  <text x="118" y="128" fill="#d7dae0" text-anchor="middle">6-vert quad</text>
  <text x="118" y="144" fill="#555" text-anchor="middle">2 tris · SDF circle</text>
  <text x="230" y="76" fill="#666" font-size="10">fs draws the circle:</text>
  <text x="230" y="92" fill="#666" font-size="10">alpha = 1 − length(corner)</text>
  <text x="230" y="112" fill="#555" font-size="10">~70× cheaper than a sphere</text>
</svg>

## Files we touch

```
src/shaders/point.wgsl         # NEW — 6-vert billboard; SDF circle in the fragment shader
src/engine/pipelines/build.rs  # build_point_pipeline (a 3-line variation of the sphere one)
src/engine/pipelines/mod.rs    # Pipelines gains `point`
src/engine/gpu.rs              # CloudPoint row, points buffer, one draw
```

## Step 1 — the cloud row: `src/engine/gpu.rs`

**1a. Add `CloudPoint` next to `GlyphPoint`** (32a). Half the size — at a million points, bytes/row is
the budget that matters:

```rust
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CloudPoint {
    position: [f32; 3],   // 12 B — mesh-local
    instance_id: u32,     //  4 B — fills position's tail
    color: [f32; 4],      // 16 B
}                         // = 32 B total, two 16-byte rows, zero padding
```

> No per-point radius: cloud points all take the global `line.thickness` (px). A per-point size would
> need a lone `f32` after `color` — a third 16-byte row (→ 48 B, like `GlyphPoint`). Skip it until a
> cloud actually needs varying dot sizes; at millions of points the 16 B/row saved is real.

## Step 2 — the billboard shader: `src/shaders/point.wgsl`

Create the file. There is **no template mesh**: six corner positions come straight from
`@builtin(vertex_index)` (the lesson-25 buffer-less trick), expanded around the point in NDC by the
screen size. The fragment shader turns the square into a circle with a signed-distance test, so the
edge stays crisp and anti-aliased at any size:

```wgsl
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;
struct Instance { model: mat4x4<f32>, color: vec4<f32>, flags: u32, };
@group(2) @binding(0) var<storage, read> instances: array<Instance>;
struct CloudPoint { position: vec3<f32>, instance_id: u32, color: vec4<f32>, };
@group(3) @binding(0) var<storage, read> points: array<CloudPoint>;
struct LineUniform { thickness: f32, proj_y: f32, ortho_h: f32, vp_h: f32, };

// One logical point = 6 verts (2 triangles); corner is vertex_index % 6.
const CORNERS = array<vec2<f32>, 6>(
    vec2<f32>(-1.0, -1.0), vec2<f32>( 1.0, -1.0), vec2<f32>( 1.0, 1.0),
    vec2<f32>(-1.0, -1.0), vec2<f32>( 1.0,  1.0), vec2<f32>(-1.0, 1.0),
);

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) corner: vec2<f32>,   // -1..1 within the quad
};

@vertex
fn vs_main(@builtin(vertex_index) vid: u32, @builtin(instance_index) pi: u32) -> VsOut {
    let p      = points[pi];
    let model  = instances[p.instance_id].model;
    let world  = (model * vec4<f32>(p.position, 1.0)).xyz;
    let clip   = mvp * vec4<f32>(world, 1.0);
    let corner = CORNERS[vid % 6u];
    let px     = line.thickness;
    // Expand in NDC by px pixels; vp_h maps px→NDC, clip.w cancels the perspective divide.
    let off    = corner * px * 2.0 / line.vp_h * clip.w;
    var o: VsOut;
    o.pos    = vec4<f32>(clip.xy + off, clip.zw);
    o.color  = p.color;
    o.corner = corner;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let d = length(in.corner);            // SDF circle: soft, anti-aliased edge
    let a = clamp((1.0 - d) * 8.0, 0.0, 1.0);
    if (a < 0.01) { discard; }
    return vec4<f32>(in.color.rgb, in.color.a * a);
}
```

> Expanding by `vp_h` on both axes makes the circle slightly oval on a non-square viewport. The exact
> fix is a `vp_w` field on the line uniform (`off = corner * px * 2 / vec2(vp_w, vp_h) * clip.w`); it's
> deferred here to keep `LineUniform` a tight 16 B. Near-square windows won't notice.

## Step 3 — the point pipeline: `src/engine/pipelines/build.rs` + `mod.rs`

**3a. Add `build_point_pipeline` after `build_sphere_pipeline`.** It is `build_sphere_pipeline` with
three changes — no vertex buffer (corners come from `vertex_index`), **alpha blending on** (the SDF
edge is translucent), and depth **write off** (billboards are transparent overlays):

```rust
        // in vertex state:
        buffers: &[],                                    // no template — vertex_index builds the quad
        // in the fragment target:
        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
        // in depth_stencil:
        depth_write_enabled: Some(false),
```

**3b. In `mod.rs`**, add `pub point: wgpu::RenderPipeline,` after `sphere` and build it with the same
layouts (the `points` storage buffer reuses the glyph bind-group layout shape — one read-only storage
entry at binding 0).

## Step 4 — build the buffer + draw: `src/engine/gpu.rs`

**4a. Collect cloud points in the arena loop** — for a `PointCloud` object, one `CloudPoint` per point
from `pointcloud.get_points()`, `instance_id: ri` — and build `point_buffer` / `point_bind_group` /
`point_count` exactly like 32a's glyph set (same storage-buffer pattern, same layout).

**4b. Draw after the spheres** in `clear()` — no index buffer, six verts per point:

```rust
            pass.set_pipeline(&self.pipelines.point);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.set_bind_group(1, &self.line_bind_group, &[]);
            pass.set_bind_group(2, &self.instance_bind_group, &[]);
            pass.set_bind_group(3, &self.point_bind_group, &[]);
            pass.draw(0..6 * self.point_count, 0..1);   // 6 verts per point, no template
            draws += 1;
```

## Run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Drop in a `PointCloud` and it draws as flat circles. Console (F12):

```
perf: 60.0 fps | 16.67 ms | 6 draws | 5 objects
```

**6 draws** — 31's four + one sphere call (32a) + one point call (the whole cloud). Load a 100k-point
cloud and it stays 6: the tables grow, the call count doesn't.

## Recap

```
Ch 32a: handle points = instanced unit spheres, one draw.
Ch 32b: CLOUD points = screen-space BILLBOARDS. CloudPoint (32 B: local position, instance_id in the
        vec3 tail, color — zero padding, half a GlyphPoint; bytes/row is the budget at 1M points). NO
        template: 6 verts from @builtin(vertex_index) make a quad, expanded in NDC by line.thickness;
        the fragment shader cuts it to an anti-aliased circle (alpha = 1 − length(corner)). Pipeline =
        sphere's with buffers:&[], alpha blend ON, depth write OFF. One draw for the whole cloud.
        Points done both ways: spheres where 3-D matters, billboards where count does.
```

Edited: `shaders/point.wgsl` (NEW), `engine/pipelines/build.rs` (`build_point_pipeline`),
`engine/pipelines/mod.rs` (`Pipelines.point`), `engine/gpu.rs` (`CloudPoint`, points buffer, one draw).

## Next

`33-camera-relative.md` — f32 world positions jitter far from the origin even with the f64 kernel. The
fix is **camera-relative rendering**: make the camera target the origin (f64), subtract it from every
instance row's translation before the f32 cast, and keep vertices local. A demo at x = 10 km stops
shimmering — the last precision piece before loading real scenes.
