# 32 Points — sphere glyphs for handles, SDF billboards at scale

Points are 0-D: line/curve endpoints and edit handles want a round, pickable marker; a `PointCloud`
wants millions. Both are lesson 31's instance-table trick again — one **template**, one **row per
point**, **one draw** — with two templates. A **unit sphere** instanced per handle (the 0-D twin of
31's cylinder-per-edge), and, at cloud scale, a **screen-space billboard**: 2 triangles that draw as
an anti-aliased circle in the fragment shader, ~70× cheaper than a sphere.

<svg viewBox="0 0 680 172" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="sphere glyph path for handles vs billboard path for clouds" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="170" y="18" fill="#888" text-anchor="middle">handles / endpoints — few</text>
  <circle cx="80" cy="80" r="30" fill="none" stroke="#6fb3ff" stroke-width="1.5"/>
  <ellipse cx="80" cy="80" rx="30" ry="11" fill="none" stroke="#6fb3ff" stroke-width="0.8"/>
  <ellipse cx="80" cy="80" rx="11" ry="30" fill="none" stroke="#6fb3ff" stroke-width="0.8"/>
  <text x="80" y="128" fill="#d7dae0" text-anchor="middle">unit sphere</text>
  <text x="80" y="144" fill="#555" text-anchor="middle">74 v · 144 tris</text>
  <text x="200" y="76" fill="#6fb3ff" font-size="16">▶</text>
  <text x="205" y="92" fill="#666" font-size="10">instance / glyph</text>
  <text x="510" y="18" fill="#888" text-anchor="middle">PointCloud — millions</text>
  <rect x="470" y="52" width="56" height="56" fill="none" stroke="#3a3a3a"/>
  <line x1="470" y1="52" x2="526" y2="108" stroke="#3a3a3a"/>
  <circle cx="498" cy="80" r="24" fill="none" stroke="#6fb3ff" stroke-width="1.5"/>
  <text x="498" y="128" fill="#d7dae0" text-anchor="middle">6-vert quad</text>
  <text x="498" y="144" fill="#555" text-anchor="middle">2 tris · SDF circle</text>
  <text x="560" y="80" fill="#666" font-size="10">fs draws the circle</text>
</svg>

## Why

Endpoints and edit handles need round dots that sit correctly in depth and can be picked — a real
sphere. A 100k-point cloud can't afford 144 triangles each, but it also doesn't need 3-D roundness:
a flat circle that always faces you looks identical and costs 2 triangles. So: **spheres where the
count is small and 3-D matters, billboards where the count is huge.** Same instance-table idea as 31,
one draw call each.

## Files we touch

```
src/shaders/sphere.wgsl        # NEW — unit sphere instanced per glyph; screen-constant radius
src/shaders/point.wgsl         # NEW — 6-vert billboard; SDF circle in the fragment shader
src/engine/pipelines/build.rs  # build_sphere_pipeline + build_point_pipeline
src/engine/pipelines/mod.rs    # Pipelines gains `sphere` and `point`
src/engine/gpu.rs              # GlyphPoint + CloudPoint rows, sphere template, glyph/cloud buffers, two draws
```

The group-2 `instances` table and the group-1 `line` uniform are **unchanged from 31** — both new
passes reuse them (the object transform, and the screen-constant sizing).

## Step 1 — the glyph row: `src/engine/gpu.rs`

One row per handle: a **local** centre, a `radius` sentinel (`0` → screen-constant px, `> 0` →
world-mm), the `instance_id` for the object transform, and a colour. Same 48 B / three-row shape as
`CylinderSegment` — but note the padding is back:

**1a. Find `CylinderSegment` (bottom of the file) and add `GlyphPoint` right after it:**

```rust
// One handle = one instance of the unit-sphere template. `center` is mesh-local; the object's
// instances[instance_id].model is applied in the shader, exactly like the cylinder segments in 31.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct GlyphPoint {
    center: [f32; 3],   // 12 B  — mesh-local
    radius: f32,        //  4 B  — 0.0 → screen-constant px; > 0 → world-mm  (fills center's tail)
    color: [f32; 4],    // 16 B  — per-glyph rgba
    instance_id: u32,   //  4 B  — row in instances[]
    _pad: [u32; 3],     // 12 B  — one trailing scalar, so the row DOES need padding here
}                       // = 48 B total, three 16-byte rows
```

> The cylinder row escaped padding because it had **two** scalars (`radius`, `instance_id`), one for
> each `vec3`'s 16-byte tail. A point has only **one** `vec3` (`center`), so `radius` fills its tail
> but the lone `instance_id` after `color` leaves a 12-byte gap — hence the explicit `_pad`. std430
> inserts it in the shader; `bytemuck` needs it spelled out in Rust.

## Step 2 — the unit-sphere template: `src/engine/gpu.rs`

**2a. Add a `unit_sphere()` free function** at the bottom, beside `unit_cylinder()` — a UV sphere on
the origin, radius 1, position-only (the same `cyl_template_layout` slot feeds it; a flat glyph needs
no normals). `LONS`/`LATS` are the perf knob (12×6 → 74 verts / 432 indices = 144 tris):

```rust
const SPH_LONS: usize = 12;
const SPH_LATS: usize = 6;

// Unit sphere on the origin, radius 1. The shader offsets each template vertex by the screen-constant
// radius around the glyph's world centre — no frame needed (a sphere is symmetric), unlike 31's tube.
fn unit_sphere() -> (Vec<[f32; 3]>, Vec<u32>) {
    let pi = std::f32::consts::PI;
    let mut v: Vec<[f32; 3]> = Vec::new();
    let mut idx: Vec<u32> = Vec::new();
    v.push([0.0, 0.0, 1.0]);                                   // north pole
    for k in 1..=SPH_LATS {
        let phi = k as f32 * pi / (SPH_LATS + 1) as f32;
        let (z, r) = (phi.cos(), phi.sin());
        for i in 0..SPH_LONS {
            let t = i as f32 * 2.0 * pi / SPH_LONS as f32;
            v.push([r * t.cos(), r * t.sin(), z]);
        }
    }
    let south = v.len() as u32; v.push([0.0, 0.0, -1.0]);      // south pole
    for i in 0..SPH_LONS {                                     // top cap fan
        idx.extend_from_slice(&[0, 1 + i as u32, 1 + ((i + 1) % SPH_LONS) as u32]);
    }
    for k in 0..(SPH_LATS - 1) {                               // middle bands
        let (ra, rb) = ((1 + k * SPH_LONS) as u32, (1 + (k + 1) * SPH_LONS) as u32);
        for i in 0..SPH_LONS {
            let (a0, a1) = (ra + i as u32, ra + ((i + 1) % SPH_LONS) as u32);
            let (b0, b1) = (rb + i as u32, rb + ((i + 1) % SPH_LONS) as u32);
            idx.extend_from_slice(&[a0, a1, b0, a1, b1, b0]);
        }
    }
    let lr = (1 + (SPH_LATS - 1) * SPH_LONS) as u32;           // bottom cap fan (reversed)
    for i in 0..SPH_LONS {
        idx.extend_from_slice(&[south, lr + ((i + 1) % SPH_LONS) as u32, lr + i as u32]);
    }
    (v, idx)
}
```

## Step 3 — the sphere shader: `src/shaders/sphere.wgsl`

Create the file. Per instance it reads `glyphs[instance_index]`, transforms the **centre** to world
via the object's `instances[]` row (centre only — the radius stays screen-constant, unaffected by
object scale), then offsets the template vertex by that radius. No frame construction — a sphere is
radially symmetric, so unlike 31's cylinder there's no `+Z` to align. Bindings and the
`screen_radius` helper are 31's, verbatim:

```wgsl
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct Instance {
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;

// Matches the Rust `GlyphPoint` (48 B) — field order and sizes identical.
struct GlyphPoint {
    center: vec3<f32>,
    radius: f32,
    color: vec4<f32>,
    instance_id: u32,
};
@group(3) @binding(0) var<storage, read> glyphs: array<GlyphPoint>;

struct LineUniform {
    thickness: f32,
    proj_y: f32,
    ortho_h: f32,
    vp_h: f32,
};

fn screen_radius(clip_w: f32, u: LineUniform) -> f32 {
    if (u.ortho_h > 0.0) {
        return u.thickness * u.ortho_h / u.vp_h;
    }
    return u.thickness * clip_w / (u.proj_y * u.vp_h);
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(@location(0) tmpl: vec3<f32>, @builtin(instance_index) gi: u32) -> VsOut {
    let g      = glyphs[gi];
    let model  = instances[g.instance_id].model;
    let centre = (model * vec4<f32>(g.center, 1.0)).xyz;   // centre only — radius is scale-invariant
    let clip_c = mvp * vec4<f32>(centre, 1.0);

    // Handles read a touch larger than lines (×3); a world-mm radius (> 0) overrides.
    let base = screen_radius(clip_c.w, line) * 3.0;
    let r    = select(base, g.radius, g.radius > 0.0);

    let world = centre + tmpl * r;                          // symmetric — offset straight out
    var o: VsOut;
    o.pos   = mvp * vec4<f32>(world, 1.0);
    o.color = g.color;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    return in.color;   // flat glyph; add lighting later if you want shaded handles
}
```

## Step 4 — the sphere pipeline: `src/engine/pipelines/build.rs` + `mod.rs`

**4a. In `build.rs`, add `build_sphere_pipeline` after `build_cylinder_pipeline`.** It is
`build_cylinder_pipeline` with the shader path swapped — same four bind groups (mvp, line,
`instances`, the new `glyphs`), same position-only template layout, solid `TriangleList`, depth
**write on** (opaque spheres occlude correctly):

```rust
pub fn build_sphere_pipeline(
    device: &wgpu::Device,
    color_format: wgpu::TextureFormat,
    mvp_layout: &wgpu::BindGroupLayout,
    line_layout: &wgpu::BindGroupLayout,
    instance_layout: &wgpu::BindGroupLayout,
    glyph_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("sphere.shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/sphere.wgsl").into()),
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("sphere.layout"),
        bind_group_layouts: &[Some(mvp_layout), Some(line_layout), Some(instance_layout), Some(glyph_layout)],
        immediate_size: 0,
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("sphere"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[cyl_template_layout()],   // reused — position-only, stride 12
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: color_format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(true),
            depth_compare: Some(wgpu::CompareFunction::Greater),  // reverse-Z (lesson 26)
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState { count: MSAA_SAMPLES, mask: !0, alpha_to_coverage_enabled: false },
        multiview_mask: None,
        cache: None,
    })
}
```

**4b. In `mod.rs`, add the field and build it.** Import `build_sphere_pipeline`, add
`pub sphere: wgpu::RenderPipeline,` after `cylinder`, add a `glyph_layout: &wgpu::BindGroupLayout`
parameter to `new()`, and:

```rust
            sphere: build_sphere_pipeline(device, color_format, aspect_layout, line_layout, instance_layout, glyph_layout),
```

## Step 5 — build the glyphs and upload: `src/engine/gpu.rs`

**5a. Collect one glyph per endpoint in the arena loop** (`for (ri, (mesh, model, color)) in
objects…`, the same loop that fills `segments` in 31). Declare a `glyphs` vec above the loop, beside
`segments`:

```rust
        let mut glyphs: Vec<GlyphPoint> = Vec::new();          // ← ADD beside segments
```

and, **inside** the loop after the edge extraction, emit a glyph at each naked-edge endpoint (the
handle set — mesh interior vertices stay hidden; `naked_vertices(true)` is the boundary):

```rust
            // Endpoints → one sphere glyph each; instance_id = this object's row (ri).
            for vk in mesh.naked_vertices(true) {
                let p = mesh.vertex_point(vk).unwrap();
                glyphs.push(GlyphPoint {
                    center: p.to_f32(),
                    radius: 0.0,                                // screen-constant px
                    color: [0.1, 0.1, 0.1, 1.0],               // dark handle
                    instance_id: ri as u32,
                    _pad: [0; 3],
                });
            }
```

**5b. Build the sphere template and the glyph buffer**, right beside the cylinder template / segment
buffer block (5c in lesson 31). It is the identical pattern — template VBO/IBO + a storage buffer,
layout, and bind group:

```rust
        // Unit-sphere template (positions only) — one mesh, instanced per glyph.
        let (sph_v, sph_i) = unit_sphere();
        let sph_index_count = sph_i.len() as u32;
        let sph_template_vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("sph.template.vbo"), contents: bytemuck::cast_slice(&sph_v), usage: wgpu::BufferUsages::VERTEX,
        });
        let sph_template_ibo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("sph.template.ibo"), contents: bytemuck::cast_slice(&sph_i), usage: wgpu::BufferUsages::INDEX,
        });

        let glyph_count = glyphs.len() as u32;
        let glyph_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("glyphs.buffer"), contents: bytemuck::cast_slice(&glyphs),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });
        let glyph_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("glyphs.layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false, min_binding_size: None,
                },
                count: None,
            }],
        });
        let glyph_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("glyphs.bind_group"), layout: &glyph_layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: glyph_buffer.as_entire_binding() }],
        });
```

Pass `&glyph_layout` into `Pipelines::new` (Step 4b), and store the six new fields on `Gpu`
(`sph_template_vbo/ibo`, `sph_index_count`, `glyph_buffer`, `glyph_bind_group`, `glyph_count`) exactly
as 31 stored the cylinder set.

## Step 6 — draw the spheres in one call: `src/engine/gpu.rs`

**6a. In `clear()`, right after the cylinder draw**, add the sphere pass — same four bind groups
(swap `glyphs` for `segments` at group 3), the sphere template buffers, one instanced draw over all
glyphs:

```rust
            // Handles — ONE draw for every endpoint/edit sphere in the scene.
            pass.set_pipeline(&self.pipelines.sphere);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.set_bind_group(1, &self.line_bind_group, &[]);
            pass.set_bind_group(2, &self.instance_bind_group, &[]);
            pass.set_bind_group(3, &self.glyph_bind_group, &[]);
            pass.set_vertex_buffer(0, self.sph_template_vbo.slice(..));
            pass.set_index_buffer(self.sph_template_ibo.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.sph_index_count, 0, 0..self.glyph_count);   // one template, N glyphs
            draws += 1;
```

## Step 7 — PointCloud at scale: the billboard path

144 triangles per point is fine for a handful of handles; a `PointCloud` needs a flat circle instead.
The row is **32 B** (`CloudPoint`), there is **no template** — six vertices come straight from
`@builtin(vertex_index)` (the lesson-25 buffer-less trick), and the fragment shader draws the circle
with a signed-distance test so it stays crisp at any size.

**7a. Add `CloudPoint` next to `GlyphPoint`:**

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

**7b. Create `src/shaders/point.wgsl`** — six corners, expanded in NDC by the screen size, circle via
SDF in the fragment:

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

**7c. `build_point_pipeline`** is `build_sphere_pipeline` with three changes: no vertex buffer
(`buffers: &[]` — corners come from `vertex_index`), **alpha blending on** (the SDF edge is
translucent), and depth **write off** (billboards are transparent overlays):

```rust
        // in the fragment target:
        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
        // in depth_stencil:
        depth_write_enabled: Some(false),
```

**7d. Build a `points` buffer** exactly like the glyph buffer (from `pointcloud.get_points()` in the
arena loop), and **draw** after the spheres — no index buffer, six verts per point:

```rust
            pass.set_pipeline(&self.pipelines.point);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.set_bind_group(1, &self.line_bind_group, &[]);
            pass.set_bind_group(2, &self.instance_bind_group, &[]);
            pass.set_bind_group(3, &self.point_bind_group, &[]);
            pass.draw(0..6 * self.point_count, 0..1);   // 6 verts per point, no template
            draws += 1;
```

## Step 8 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Line/polyline endpoints now wear dark spheres that hold their pixel size as you zoom; drop in a
`PointCloud` and it draws as flat circles. Console (F12):

```
perf: 60.0 fps | 16.67 ms | 6 draws | 5 objects
```

**6 draws** — the four from 31 plus **one** sphere call (every handle) and **one** point call (the
whole cloud). Load a 100k-point cloud and it stays 6: the tables grow, the call count doesn't.

## Recap

```
Ch 31: EDGES as cylinders — 1-D tubes, one template + one row per edge, one draw, +Z aligned.
Ch 32: POINTS, two ways. Handles/endpoints = a unit-SPHERE template instanced per GlyphPoint (48 B:
       local center, radius sentinel, instance_id, color) — the 0-D twin of 31, but symmetric so no
       frame, just center + tmpl·r. PointClouds = screen-space BILLBOARDS: a 32 B CloudPoint, NO
       template (6 verts from @builtin(vertex_index)), a 2-triangle quad the fragment shader fills as
       an SDF circle — ~70× cheaper than a sphere, identical on screen. One draw each; both reuse the
       group-2 instances table and the group-1 line uniform. 6 draws / N objects, flat as the scene grows.
```

Edited: `shaders/sphere.wgsl` (NEW — instanced unit sphere, screen-constant radius), `shaders/point.wgsl`
(NEW — 6-vert billboard, SDF circle), `engine/pipelines/build.rs` (`build_sphere_pipeline` +
`build_point_pipeline`), `engine/pipelines/mod.rs` (`Pipelines.sphere` / `.point`), `engine/gpu.rs`
(`GlyphPoint` + `CloudPoint` rows, sphere template, glyph/cloud buffers, two draws).

## Next

`33-camera-relative.md` — f32 world positions jitter far from the origin even with the f64 kernel. The
fix is **camera-relative rendering**: make the camera target the origin (f64), subtract it from every
instance row's translation before the f32 cast, and keep vertices local. A demo at x = 10 km stops
shimmering — the last precision piece before loading real scenes.
