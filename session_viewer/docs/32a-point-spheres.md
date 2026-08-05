# 32a Points I — sphere glyphs for handles and endpoints

> **Big picture.** *Phase 4 — one scene, one draw call.* The goal of this phase: no matter how many
> objects a file contains, the frame stays a handful of draw calls. 29 gave us instancing, 30 the mesh
> arena, 31 every line as an instanced cylinder. Points are the last geometry kind without a path —
> this lesson gives the *few-but-important* points (endpoints, edit handles) a real 3-D marker; 32b
> handles the *millions* case. After the pair, everything the kernel can dump has a way onto the GPU.

Endpoints and edit handles want a round, pickable marker that sits correctly in depth — a real sphere.
It's lesson 31's trick again, one dimension down: one **unit-sphere template**, one **row per point**,
**one draw** for every handle in the scene.

<svg viewBox="0 0 380 172" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a unit sphere template is instanced once per handle point" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="170" y="18" fill="#888" text-anchor="middle">handles / endpoints — few, 3-D matters</text>
  <circle cx="80" cy="80" r="30" fill="none" stroke="#6fb3ff" stroke-width="1.5"/>
  <ellipse cx="80" cy="80" rx="30" ry="11" fill="none" stroke="#6fb3ff" stroke-width="0.8"/>
  <ellipse cx="80" cy="80" rx="11" ry="30" fill="none" stroke="#6fb3ff" stroke-width="0.8"/>
  <text x="80" y="128" fill="#d7dae0" text-anchor="middle">unit sphere</text>
  <text x="80" y="144" fill="#555" text-anchor="middle">74 v · 144 tris</text>
  <text x="200" y="76" fill="#6fb3ff" font-size="16">▶</text>
  <text x="210" y="92" fill="#666" font-size="10">one row per glyph</text>
  <text x="300" y="70" fill="#d7dae0" text-anchor="middle">GlyphPoint[]</text>
  <text x="300" y="86" fill="#666" text-anchor="middle" font-size="10">one draw, N spheres</text>
</svg>

## Files we touch

```
src/shaders/sphere.wgsl        # NEW — unit sphere instanced per glyph; screen-constant radius
src/engine/pipelines/build.rs  # build_sphere_pipeline
src/engine/pipelines/mod.rs    # Pipelines gains `sphere`
src/engine/gpu.rs              # GlyphPoint row, sphere template, glyph buffer, storage_buffer guard, one draw
```

The group-2 `instances` table and the group-1 `line` uniform are **unchanged from 31** — the sphere
pass reuses both (the object transform, and the screen-constant sizing).

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
no normals). `SPH_LONS`/`SPH_LATS` are the perf knob (12×6 → 74 verts / 432 indices = 144 tris):

```rust
const SPH_LONS: usize = 12;
const SPH_LATS: usize = 6;

// Unit sphere on the origin, radius 1. The shader offsets each template vertex by the
// screen-constant radius around the glyph's world centre — no frame needed (a sphere is
// symmetric), unlike 31's tube.
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
    // centre only — radius is scale-invariant
    let centre = (model * vec4<f32>(g.center, 1.0)).xyz;
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
        bind_group_layouts: &[Some(mvp_layout), Some(line_layout),
                              Some(instance_layout), Some(glyph_layout)],
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
        multisample: wgpu::MultisampleState {
            count: MSAA_SAMPLES, mask: !0, alpha_to_coverage_enabled: false },
        multiview_mask: None,
        cache: None,
    })
}
```

**4b. In `mod.rs`, add the field and build it.** Import `build_sphere_pipeline`, add
`pub sphere: wgpu::RenderPipeline,` after `cylinder`, add a `glyph_layout: &wgpu::BindGroupLayout`
parameter to `new()`, and:

```rust
            sphere: build_sphere_pipeline(device, color_format, aspect_layout,
                                          line_layout, instance_layout, glyph_layout),
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

> **Naked vs. every vertex.** `naked_vertices(true)` is the *handle* set — only boundary endpoints
> (open polylines, mesh holes). On a scene of **closed** solids it is **empty**, so no handles appear
> (see *The empty-buffer trap* below — that empty `glyphs` vec is exactly what crashed the frame).
> Swap it for `mesh.vertices()` to drop a sphere on **every** vertex: a debug point view that also fixes
> a second problem — 31's cylinders have **flat caps**, so where two thick edges meet at a corner the
> caps leave a wedge gap that grows with line thickness. A sphere seated at the shared vertex fills the
> gap and rounds the joint. For that use, drop the shader's `* 3.0` so the ball matches the tube radius
> (×1) instead of reading as an oversized handle (×3).

**5b. Build the sphere template and the glyph buffer**, right beside the cylinder template / segment
buffer block (5c in lesson 31). It is the identical pattern — template VBO/IBO + a storage buffer,
layout, and bind group:

```rust
        // Unit-sphere template (positions only) — one mesh, instanced per glyph.
        let (sph_v, sph_i) = unit_sphere();
        let sph_index_count = sph_i.len() as u32;
        let sph_template_vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("sph.template.vbo"), contents: bytemuck::cast_slice(&sph_v),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let sph_template_ibo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("sph.template.ibo"), contents: bytemuck::cast_slice(&sph_i),
            usage: wgpu::BufferUsages::INDEX,
        });

        let glyph_count = glyphs.len() as u32;                 // real count — may be 0
        let glyph_buffer = storage_buffer(&device, "glyphs.buffer", &glyphs);   // guarded — see below
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
            entries: &[wgpu::BindGroupEntry {
                binding: 0, resource: glyph_buffer.as_entire_binding() }],
        });
```

Pass `&glyph_layout` into `Pipelines::new` (Step 4b), and store the six new fields on `Gpu`
(`sph_template_vbo/ibo`, `sph_index_count`, `glyph_buffer`, `glyph_bind_group`, `glyph_count`) exactly
as 31 stored the cylinder set.

## Step 5c — the empty-buffer trap ⚠️ (critical)

The scene here is five **closed** solids, so `naked_vertices(true)` returns nothing and `glyphs` is
**empty**. `bytemuck::cast_slice(&[])` is zero bytes, and wgpu **refuses to bind a 0-byte storage
buffer** — the frame dies the instant the bind group is built:

```
wgpu on_uncaptured_error: Buffer with 'glyphs.buffer' label binding size is zero
```

<svg viewBox="0 0 380 96" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="empty vec makes a zero-byte buffer which wgpu rejects; the guard pads to one dummy row" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="40" y="20" fill="#888">glyphs: []</text>
  <rect x="40" y="30" width="70" height="20" fill="none" stroke="#e06c6c" stroke-width="1.2"/>
  <text x="75" y="44" fill="#e06c6c" text-anchor="middle">0 B ✗</text>
  <text x="150" y="44" fill="#666">→ bind rejected</text>
  <text x="40" y="76" fill="#888">guarded</text>
  <rect x="40" y="86" width="70" height="0.1"/>
  <rect x="120" y="30" width="70" height="20" fill="none" stroke="#5bbf87" stroke-width="1.2"/>
  <text x="155" y="44" fill="#5bbf87" text-anchor="middle">48 B ✓</text>
  <text x="230" y="44" fill="#666">1 dummy row, draw 0..0</text>
</svg>

This is **not** specific to glyphs — the same crash hits `instances` (a scene with no objects) or
`segments` (a mesh with no edges). Any storage buffer built from a `Vec` that *can* be empty needs the
guard. Add one helper at the bottom of `gpu.rs`, beside `unit_sphere()`:

```rust
/// A read-only storage buffer that is never zero-sized (wgpu can't bind a 0-byte buffer).
/// When `data` is empty we still allocate one zeroed element; the real element count is
/// tracked separately, so the draw call issues 0 instances and nothing renders.
fn storage_buffer<T: bytemuck::Pod>(device: &wgpu::Device, label: &str, data: &[T]) -> wgpu::Buffer {
    use wgpu::util::DeviceExt;
    let one = [T::zeroed()];
    let contents: &[u8] = if data.is_empty() { bytemuck::cast_slice(&one) } else { bytemuck::cast_slice(data) };
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    })
}
```

**Route all three storage buffers through it** — retrofit 30's `instances` and 31's `segments` too, so
no scene can crash the frame (each was a `device.create_buffer_init(… STORAGE …)` block; collapse to one line):

```rust
let instance_buffer = storage_buffer(&device, "instance.buffer", &instances);   // was create_buffer_init
let segment_buffer  = storage_buffer(&device, "segments.buffer", &segments);    // was create_buffer_init
let glyph_buffer    = storage_buffer(&device, "glyphs.buffer",   &glyphs);      // Step 5b
```

> **Why one zeroed row, not a min-binding-size?** The *bound* buffer must be ≥ one `T`, but the *draw*
> uses the real count (`glyph_count`, still `0`), so `draw_indexed(.., 0..0)` renders nothing. The dummy
> row exists only to satisfy the binding — it is never drawn. `T: bytemuck::Pod` gives us `T::zeroed()`
> for free.

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
            // one template, N glyphs
            pass.draw_indexed(0..self.sph_index_count, 0, 0..self.glyph_count);
            draws += 1;
```

## Run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

On the closed-solid scene the naked set is empty, so you'll see 31's lines but **no** handle spheres —
and, thanks to `storage_buffer`, **no crash**. Add an open polyline (or switch `naked_vertices` →
`mesh.vertices()`) and its vertices wear dark spheres that hold their pixel size as you zoom. Console
(F12) shows **5 draws** — 31's four plus the sphere call, however many glyphs there are (0 glyphs draws
nothing, but the pass stays valid).

## Recap

```
Ch 31: EDGES as cylinders — one template + one row per edge, one draw, +Z aligned.
Ch 32a: HANDLE POINTS as spheres — the 0-D twin. GlyphPoint (48 B: local center, radius sentinel,
        instance_id, color — note the _pad; one vec3 leaves a 12-byte tail). unit_sphere() template
        (12×6 UV, 144 tris), sphere.wgsl offsets template verts around the world centre — symmetric,
        so no +Z frame math. Same four bind groups as 31 (glyphs at group 3). ONE draw for all
        handles. storage_buffer() guards EVERY storage buffer against the zero-size bind crash
        (empty scene / closed mesh / no edges → one zeroed dummy row, real count stays 0, draw 0..0).
```

Edited: `shaders/sphere.wgsl` (NEW), `engine/pipelines/build.rs` (`build_sphere_pipeline`),
`engine/pipelines/mod.rs` (`Pipelines.sphere`), `engine/gpu.rs` (`GlyphPoint`, `unit_sphere()`,
glyph buffer + bind group, `storage_buffer` guard, one draw).

## Next

`32b-point-clouds.md` — 144 triangles per point is fine for a dozen handles, fatal for a 100k-point
`PointCloud`. The other half of points: a **screen-space billboard** — 2 triangles that the fragment
shader fills as an anti-aliased circle — ~70× cheaper, visually identical at point sizes.
