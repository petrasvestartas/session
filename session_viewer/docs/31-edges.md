# 31 Edges — lines as cylinders, the whole scene in one draw

Every edge in the scene becomes a **cylinder**: one **instance row** in a flat segment table, one
**unit-cylinder template**, and **one draw** for all of them. It's lesson 29's instancing pointed at
line geometry — `@builtin(instance_index)` now picks a *segment* instead of a *copy*.

<svg viewBox="0 0 680 190" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="one template plus N segment rows equals one draw" style="max-width:100%;height:auto;font:12px ui-monospace,monospace">
  <text x="70" y="20" fill="#888" text-anchor="middle">1 template</text>
  <ellipse cx="70" cy="48" rx="28" ry="9" fill="none" stroke="#6fb3ff" stroke-width="1.5"/>
  <line x1="42" y1="48" x2="42" y2="118" stroke="#6fb3ff" stroke-width="1.5"/>
  <line x1="98" y1="48" x2="98" y2="118" stroke="#6fb3ff" stroke-width="1.5"/>
  <ellipse cx="70" cy="118" rx="28" ry="9" fill="none" stroke="#6fb3ff" stroke-width="1.5"/>
  <text x="70" y="146" fill="#d7dae0" text-anchor="middle">unit cylinder +Z</text>
  <text x="150" y="90" fill="#6fb3ff" font-size="16">▶</text>
  <text x="330" y="20" fill="#888" text-anchor="middle">segments[] — one row per edge</text>
  <rect x="240" y="34" width="190" height="88" fill="none" stroke="#3a3a3a"/>
  <line x1="240" y1="56" x2="430" y2="56" stroke="#3a3a3a"/>
  <line x1="240" y1="78" x2="430" y2="78" stroke="#3a3a3a"/>
  <line x1="240" y1="100" x2="430" y2="100" stroke="#3a3a3a"/>
  <text x="250" y="50" fill="#d7dae0">p0  radius  p1  id  color</text>
  <text x="250" y="72" fill="#d7dae0">p0  radius  p1  id  color</text>
  <text x="250" y="94" fill="#d7dae0">p0  radius  p1  id  color</text>
  <text x="250" y="116" fill="#555">… N</text>
  <text x="452" y="90" fill="#6fb3ff" font-size="16">▶</text>
  <text x="470" y="132" fill="#666" font-size="10">1 draw_indexed</text>
  <g fill="none" stroke="#6fb3ff" stroke-width="1.5">
    <rect x="565" y="58" width="55" height="55"/>
    <rect x="585" y="43" width="55" height="55"/>
    <line x1="565" y1="58" x2="585" y2="43"/><line x1="620" y1="58" x2="640" y2="43"/>
    <line x1="565" y1="113" x2="585" y2="98"/><line x1="620" y1="113" x2="640" y2="98"/>
  </g>
  <text x="602" y="146" fill="#d7dae0" text-anchor="middle">every edge, one call</text>
</svg>

Two things make it linework, not just instanced tubes:

- **Rotation in the shader** — no per-edge matrix. Each segment stores only its two endpoints (32 B);
  the shader aligns the template's **+Z** to `(p1 − p0)` per instance, building the frame on the fly.
- **Screen-constant thickness** — a `line_thickness` (px) uniform expands the world radius per depth
  so the tube holds a fixed pixel width at any zoom. Changing width is **one uniform write**.

Draw count stays flat as the scene grows:

```
Ch 30:  background + grid + arena              =  3 draws
Ch 31:  background + grid + arena + cylinders  =  4 draws   ← +1 carries EVERY edge
```

Box (12 edges), dodecahedron (30), and three tessellated BReps all ride that single `cylinders` call.
The 1px LineList (`edges.wgsl`, `Pipelines.edges`) stays for overlays — we add the cylinder path fresh
and delete nothing.

## Files we touch

```
# NEW — instanced unit-cylinder; align +Z to (p1−p0); screen-constant radius
src/shaders/cylinder.wgsl
# build_cylinder_pipeline + the template's position-only vertex layout
src/engine/pipelines/build.rs
src/engine/pipelines/mod.rs    # Pipelines gains `cylinder`, threaded through new()
# CylinderSegment row + segment/template/line buffers; one cylinder draw
src/engine/gpu.rs
session_rust/src/mesh.rs       # kernel: edges_with_colors() — edges walked in linecolor order
```

The group-2 `instances` table, its layout and its bind group are **unchanged from 30** — the cylinder
pass reuses them for each edge's object transform and flags.

## Step 1 — the segment row: `src/engine/gpu.rs`

One row per edge: two **local** endpoints, a `radius` sentinel, the `instance_id` (which object's
`instances[]` row to use), and a per-edge color. In `std430` these pack into three 16-byte rows with
**no padding** — `radius` and `instance_id` fill the gaps each `vec3` leaves.

**1a. Find the `Instance` struct at the very bottom of the file** (currently the last item,
`struct Instance { model: [f32; 16], … }`) and add `CylinderSegment` right after it:

```rust
// One edge = one instance of the unit-cylinder template. Endpoints are LOCAL (the object's
// instances[instance_id].model is applied in the shader, like the arena vertices in 30).
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CylinderSegment {
    p0: [f32; 3],       // 12 B  — start, mesh-local
    radius: f32,        //  4 B  — 0.0 → screen-constant px (default); > 0 → world-mm override
    p1: [f32; 3],       // 12 B  — end, mesh-local        (p0..instance_id = 32 B of geometry)
    instance_id: u32,   //  4 B  — row in instances[]: object model + flags (hide/select later)
    color: [f32; 4],    // 16 B  — per-edge (black crease, naked color, …)
}                       // = 48 B total, three 16-byte rows, zero padding
```

<svg viewBox="0 0 540 82" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="CylinderSegment byte layout: 48 bytes in three 16-byte rows" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="88"  y="14" fill="#888" text-anchor="middle">row 0 · 16 B</text>
  <text x="256" y="14" fill="#888" text-anchor="middle">row 1 · 16 B</text>
  <text x="436" y="14" fill="#888" text-anchor="middle">row 2 · 16 B</text>
  <g stroke="#0d0f12" stroke-width="1">
    <rect x="16"  y="22" width="126" height="28" fill="#2b4a63"/>
    <rect x="142" y="22" width="42"  height="28" fill="#3a3a3a"/>
    <rect x="184" y="22" width="126" height="28" fill="#2b4a63"/>
    <rect x="310" y="22" width="42"  height="28" fill="#3a3a3a"/>
    <rect x="352" y="22" width="168" height="28" fill="#5a4a2b"/>
  </g>
  <g fill="#d7dae0" text-anchor="middle">
    <text x="79"  y="40">p0 · 12</text>
    <text x="163" y="40">rad</text>
    <text x="247" y="40">p1 · 12</text>
    <text x="331" y="40">id</text>
    <text x="436" y="40">color · 16</text>
  </g>
  <g fill="#666" text-anchor="middle" font-size="10">
    <text x="16"  y="64">0</text>
    <text x="142" y="64">12</text>
    <text x="184" y="64">16</text>
    <text x="310" y="64">28</text>
    <text x="352" y="64">32</text>
    <text x="520" y="64">48</text>
  </g>
</svg>

> `radius` and `instance_id` sit in the 4-byte tail each `vec3` leaves — that's how the row stays a
> tight 48 B. Color is inline (not from `instances[]`) so one mesh can carry distinct edge colors — a
> black crease beside a colored naked edge, which Step 2 unlocks.

## Step 2 — per-edge colors in the kernel: `session_rust/src/mesh.rs`

`edges()` returns edges **sorted** `(u < v)`, but `linecolors` is filled in **`add_face` insertion
order** — same length, *not* index-aligned, so you can't zip them. Per-edge colors need an edge walk
in the order `add_face` seeded the colors.

**2a. After `naked_edges` (~line 1286, next to `edges()`), add `edges_with_colors`** — it replays the
face traversal, so the Nth unique edge lines up with the Nth `linecolors` entry:

```rust
/// Edges paired with their stored line color, walked in the SAME order `add_face`
/// seeded `linecolors` (first-discovery during face traversal) — so color N belongs
/// to edge N. `edges()` sorts `(u < v)` and therefore does NOT align with `linecolors`;
/// this walk does. Robust for meshes built face-by-face; note that `remove_face`
/// truncates `linecolors` from the end, so a mesh edited by face removal can desync
/// (rebuild the colors after structural edits).
pub fn edges_with_colors(&self) -> Vec<(usize, usize, Color)> {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut out: Vec<(usize, usize, Color)> = Vec::new();
    let mut ci = 0usize;
    let mut fkeys: Vec<usize> = self.face.keys().copied().collect();
    fkeys.sort_unstable();
    for fk in fkeys {
        let vs = &self.face[&fk];
        for i in 0..vs.len() {
            let u = vs[i];
            let v = vs[(i + 1) % vs.len()];
            let e = if u < v { (u, v) } else { (v, u) };
            if seen.insert(e) {
                let c = self.linecolors.get(ci).cloned().unwrap_or_else(Color::black);
                out.push((e.0, e.1, c));
                ci += 1;
            }
        }
    }
    out
}
```

`HashSet` and `Color` are already in scope. This is the permanent per-edge-color source — crease,
naked, and selection edges can each carry their own color now.

> **Later:** the demo builds segments once in `Gpu::new`. When edits arrive (lesson 34+), wrap this in
> a cached `Mesh::gpu_edges()` mirroring `gpu_mesh()`, so only the edited mesh re-extracts.

## Step 3 — the cylinder shader: `src/shaders/cylinder.wgsl`

Create the file. Per instance the vertex shader reads `segments[instance_index]`, transforms the
endpoints to world, aligns the template's +Z to the edge, and expands the ring by a screen-constant
radius. `CylinderSegment` and `Instance` must match their Rust twins byte-for-byte (48 B / 96 B).

```wgsl
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

// Matches the Rust `Instance` (96 B) from lessons 29/30 — reused verbatim for edge transforms.
struct Instance {
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;

// Matches the Rust `CylinderSegment` (48 B) from Step 1 — field order and sizes must be identical.
struct CylinderSegment {
    p0: vec3<f32>,
    radius: f32,
    p1: vec3<f32>,
    instance_id: u32,
    color: vec4<f32>,
};
@group(3) @binding(0) var<storage, read> segments: array<CylinderSegment>;

// Screen-constant line width, driven from the camera (see Step 5).
struct LineUniform {
    thickness: f32,   // desired on-screen width, in pixels
    proj_y: f32,      // vertical projection scale × unit scale  (persp: cot(fovy/2) · mm→m)
    ortho_h: f32,     // ortho world half-height × unit scale; 0.0 in perspective
    vp_h: f32,        // framebuffer height, in pixels
};

// World-space radius that projects to `thickness` px, constant regardless of zoom.
fn screen_radius(clip_w: f32, u: LineUniform) -> f32 {
    if (u.ortho_h > 0.0) {
        return u.thickness * u.ortho_h / u.vp_h;          // ortho: depth-independent
    }
    return u.thickness * clip_w / (u.proj_y * u.vp_h);    // persp: grows with depth (∝ clip.w)
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(@location(0) tmpl: vec3<f32>, @builtin(instance_index) si: u32) -> VsOut {
    let seg   = segments[si];
    let model = instances[seg.instance_id].model;

    // Endpoints → world (object transform in the shader, exactly like the arena in 30).
    let w0 = (model * vec4<f32>(seg.p0, 1.0)).xyz;
    let w1 = (model * vec4<f32>(seg.p1, 1.0)).xyz;

    // Align template +Z to (w1 − w0); build an orthonormal frame around the axis.
    let axis  = w1 - w0;
    let len   = length(axis);
    let dir   = select(vec3<f32>(0.0, 0.0, 1.0), axis / len, len > 1e-9);
    // Reference axis for the frame: must NEVER be parallel to dir, or cross() returns zero and the
    // tube collapses to NaN (invisible). Rule: use Z as the reference, and swap to X only when dir
    // itself is near-Z. (Getting this backwards — X as the default — silently deletes every
    // X-parallel edge: 4 edges of any box vanish. A real bug we shipped and a reader caught.)
    let ref0  = select(vec3<f32>(0.0, 0.0, 1.0), vec3<f32>(1.0, 0.0, 0.0), abs(dir.z) > 0.9);
    let right = normalize(cross(ref0, dir));
    let up    = cross(dir, right);

    // Centreline point at this template-z — independent of radius, so we can read clip.w first.
    let centre = w0 + dir * (len * tmpl.z);
    let clip_c = mvp * vec4<f32>(centre, 1.0);

    // Screen-constant radius, unless the segment overrides it with a world-mm radius (> 0).
    let r = select(screen_radius(clip_c.w, line), seg.radius, seg.radius > 0.0);

    let world = centre + (right * tmpl.x + up * tmpl.y) * r;
    var o: VsOut;
    o.pos   = mvp * vec4<f32>(world, 1.0);
    o.color = seg.color;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    return in.color;   // edges read as flat lines; add lighting later if you want shaded tubes
}
```

Two gotchas: **centreline first** — `r` needs `clip.w`, so read it from the radius-independent centre
point before expanding. And WGSL `select(a, b, cond)` returns **`b` when `cond` is true**, so the
radius line reads "world-mm when `seg.radius > 0`, else screen-constant".

## Step 4 — the cylinder pipeline: `src/engine/pipelines/build.rs` + `mod.rs`

**4a. In `build.rs`, add the template's position-only vertex layout** near the top, right after the
existing `INSTANCE_ID_ATTRIBS` / `instance_id_layout` block (the template carries just a `vec3`
position at `@location(0)`, stride 12):

```rust
const CYL_TEMPLATE_ATTRIBS: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    offset: 0,
    shader_location: 0,
    format: wgpu::VertexFormat::Float32x3,
}];

fn cyl_template_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: 12,                           // one vec3<f32> per template vertex
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &CYL_TEMPLATE_ATTRIBS,
    }
}
```

**4b. Add `build_cylinder_pipeline` after `build_edges_pipeline`.** Four bind groups (mvp, line,
`instances`, `segments`); solid `TriangleList` tubes with depth **write on** — they protrude from
faces, so no depth bias needed:

```rust
pub fn build_cylinder_pipeline(
    device: &wgpu::Device,
    color_format: wgpu::TextureFormat,
    mvp_layout: &wgpu::BindGroupLayout,
    line_layout: &wgpu::BindGroupLayout,
    instance_layout: &wgpu::BindGroupLayout,
    segment_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("cylinder.shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/cylinder.wgsl").into()),
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("cylinder.layout"),
        bind_group_layouts: &[Some(mvp_layout), Some(line_layout),
                              Some(instance_layout), Some(segment_layout)],
        immediate_size: 0,
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("cylinder"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[cyl_template_layout()],   // slot 0 — the unit-cylinder positions
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
            cull_mode: None,                     // thin tubes — keep both faces
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(true),     // solid tubes occlude correctly, no bias needed
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

**4c. In `mod.rs`, add the pipeline to `Pipelines`.** Find `pub struct Pipelines { … }` and add a
field after `edges`:

```rust
    pub edges: wgpu::RenderPipeline,
    pub cylinder: wgpu::RenderPipeline,   // ← ADD THIS LINE
    pub background: wgpu::RenderPipeline,
```

Then find `impl Pipelines { pub fn new(...) }`. Add the two new layouts as parameters and build the
pipeline (import it alongside the others at the top: `use build::build_cylinder_pipeline;`):

```rust
    pub fn new(
        device: &wgpu::Device,
        color_format: wgpu::TextureFormat,
        aspect_layout: &wgpu::BindGroupLayout,
        time_layout: &wgpu::BindGroupLayout,
        instance_layout: &wgpu::BindGroupLayout,
        line_layout: &wgpu::BindGroupLayout,        // ← new
        segment_layout: &wgpu::BindGroupLayout,     // ← new
    ) -> Self {
        Self {
            triangle: build_triangle_pipeline(device, color_format, aspect_layout,
                                              time_layout, instance_layout),
            grid: build_grid_pipeline(device, color_format, aspect_layout),
            edges: build_edges_pipeline(device, color_format, aspect_layout),
            cylinder: build_cylinder_pipeline(device, color_format, aspect_layout,
                                              line_layout, instance_layout, segment_layout),
            background: build_background_pipeline(device, color_format),
        }
    }
```

(`aspect_layout` is the mvp/group-0 layout — the same one the triangle and grid pipelines already
receive.)

## Step 5 — build the segments, the template, and the line uniform: `src/engine/gpu.rs`

**5a. Collect one segment per edge in the arena loop** (`for (ri, (mesh, model, color)) in
objects…`, ~line 165). Declare a `segments` vec just above the loop, beside `verts` / `vids` / `idx`:

```rust
        let mut segments: Vec<CylinderSegment> = Vec::new();   // ← ADD beside verts/vids/idx
```

and **inside** the loop, after `for &i in &rm.indices { idx.push(base + i); }`, extract this mesh's
edges (endpoints stay local — the shader applies `model` via `instances[ri]`):

```rust
            // Edges → one cylinder segment each; instance_id = this object's row (ri).
            // Point::to_f32() / Color::to_f32() are the kernel's GPU-edge casts (as Xform::to_f32).
            for (a, b, col) in mesh.edges_with_colors() {
                let pa = mesh.vertex_point(a).unwrap();
                let pb = mesh.vertex_point(b).unwrap();
                segments.push(CylinderSegment {
                    p0: pa.to_f32(),
                    radius: 0.0,                                    // screen-constant px
                    p1: pb.to_f32(),
                    instance_id: ri as u32,
                    // per-edge rgba (opaque by default)
                    color: col.to_f32(),
                });
            }
```

> Smooth BReps dump their full tessellation wireframe here. For the clean CAD look, filter to creases
> (large dihedral) + `naked_edges` — same machinery, only the edge *selection* changes.

**5b. Add the unit-cylinder template** as a free function at the bottom of the file — along **+Z**,
radius 1, `z ∈ [0,1]`, with cap fans. `CYL_SIDES` is the perf knob (6–8 is plenty at 1–2 px):

```rust
const CYL_SIDES: u32 = 12;

// Unit cylinder along +Z (radius 1, z in [0,1]) with cap fans. The shader rescales xy by the
// screen-constant radius and maps z along (p1 − p0), so this template is registered ONCE.
fn unit_cylinder(sides: u32) -> (Vec<[f32; 3]>, Vec<u32>) {
    let mut v: Vec<[f32; 3]> = Vec::new();
    let mut idx: Vec<u32> = Vec::new();
    for s in 0..sides {                                   // side rings: (bottom, top) per facet
        let a = s as f32 / sides as f32 * std::f32::consts::TAU;
        v.push([a.cos(), a.sin(), 0.0]);
        v.push([a.cos(), a.sin(), 1.0]);
    }
    for s in 0..sides {
        let b0 = 2 * s;
        let b1 = 2 * ((s + 1) % sides);
        idx.extend_from_slice(&[b0, b1, b1 + 1, b0, b1 + 1, b0 + 1]);   // two tris per side quad
    }
    let cb = v.len() as u32; v.push([0.0, 0.0, 0.0]);     // bottom + top cap centres
    let ct = v.len() as u32; v.push([0.0, 0.0, 1.0]);
    for s in 0..sides {
        let b0 = 2 * s;
        let b1 = 2 * ((s + 1) % sides);
        idx.extend_from_slice(&[cb, b1, b0, ct, b0 + 1, b1 + 1]);        // bottom + top fan
    }
    (v, idx)
}
```

**5c. Upload the template, segments, and line uniform, and build their bind groups.** Find the block
that uploads the arena buffers (`let arena_ibo = device.create_buffer_init(…);`, ~line 217) and, in
the blank line before `// Pipelines`, insert:

```rust
        // Unit-cylinder template (positions only) — one mesh, instanced per edge.
        let (cyl_v, cyl_i) = unit_cylinder(CYL_SIDES);
        let cyl_index_count = cyl_i.len() as u32;
        let cyl_template_vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("cyl.template.vbo"), contents: bytemuck::cast_slice(&cyl_v),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let cyl_template_ibo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("cyl.template.ibo"), contents: bytemuck::cast_slice(&cyl_i),
            usage: wgpu::BufferUsages::INDEX,
        });

        // One storage row per edge (VERTEX-visible, read-only) — the segment table.
        let segment_count = segments.len() as u32;
        let segment_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("segments.buffer"), contents: bytemuck::cast_slice(&segments),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });
        let segment_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("segments.layout"),
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
        let segment_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("segments.bind_group"), layout: &segment_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0, resource: segment_buffer.as_entire_binding() }],
        });

        // Line uniform — screen-constant thickness; rewritten each frame from the camera.
        let line_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("line.buffer"),
            contents: bytemuck::bytes_of(&LineUniform {
                thickness: 2.0, proj_y: 1.0, ortho_h: 0.0, vp_h: config.height as f32 }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let line_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("line.layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false, min_binding_size: None },
                count: None,
            }],
        });
        let line_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("line.bind_group"), layout: &line_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0, resource: line_buffer.as_entire_binding() }],
        });
```

and add the `LineUniform` struct next to `CylinderSegment` at the bottom of the file:

```rust
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct LineUniform {
    thickness: f32,   // on-screen width, px
    proj_y: f32,      // vertical projection scale × unit scale (persp: cot(fovy/2) · mm→m)
    ortho_h: f32,     // ortho world half-height × unit scale; 0.0 in perspective
    vp_h: f32,        // framebuffer height, px
}                     // 16 B — one vec4, no padding
```

**5d. Pass the two new layouts to `Pipelines::new`.** Find the call
(`let pipelines = Pipelines::new(&device, config.format, &mvp_layout, &time_layout, &instance_layout);`,
~line 224) and append them:

```rust
        let pipelines = Pipelines::new(&device, config.format, &mvp_layout, &time_layout,
                                       &instance_layout, &line_layout, &segment_layout);
```

**5e. Store the new fields.** In `pub struct Gpu { … }`, after `pub instance_bind_group: …`, add:

```rust
    pub cyl_template_vbo: wgpu::Buffer,
    pub cyl_template_ibo: wgpu::Buffer,
    pub cyl_index_count: u32,
    pub segment_buffer: wgpu::Buffer,
    pub segment_bind_group: wgpu::BindGroup,
    pub segment_count: u32,
    pub line_buffer: wgpu::Buffer,
    pub line_bind_group: wgpu::BindGroup,
```

and name them all in the `Ok(Self { … })` initializer at the end of `new` (order does not matter):

```rust
            cyl_template_vbo,
            cyl_template_ibo,
            cyl_index_count,
            segment_buffer,
            segment_bind_group,
            segment_count,
            line_buffer,
            line_bind_group,
```

## Step 6 — draw every edge in one call: `src/engine/gpu.rs`

**6a. In `clear()`, right after the arena draw** (`pass.draw_indexed(0..self.arena_index_count, 0,
0..1);`), add the cylinder pass — four bind groups, the template buffers, one instanced draw over all
segments:

```rust
            // Edges — ONE draw for the WHOLE scene's linework (segments + unit-cylinder template)
            pass.set_pipeline(&self.pipelines.cylinder);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.set_bind_group(1, &self.line_bind_group, &[]);
            pass.set_bind_group(2, &self.instance_bind_group, &[]);
            pass.set_bind_group(3, &self.segment_bind_group, &[]);
            pass.set_vertex_buffer(0, self.cyl_template_vbo.slice(..));
            pass.set_index_buffer(self.cyl_template_ibo.slice(..), wgpu::IndexFormat::Uint32);
            // one template, N edges
            pass.draw_indexed(0..self.cyl_index_count, 0, 0..self.segment_count);
            draws += 1;
```

### The wiring — how `set_bind_group` reaches `@group` (the four groups, explained once)

This is the first draw with **four** bind groups, and the same shape recurs in every later pipeline
(spheres 32a, clouds 32b, gumball 52…). Learn it here and it never confuses you again.

**The number is the only wire.** `set_bind_group(N, X)` says *"put bind group `X` into slot **N**."* The
shader's `@group(N)` says *"read slot **N**."* Nothing else connects the Rust to the WGSL — not the
names, not the order. Match the numbers and the shader sees your data; swap two and it silently reads the
wrong buffer.

| slot | Rust draw — `set_bind_group(N, …)` | shader — `@group(N) @binding(0)` | the data |
|:--:|---|---|---|
| **0** | `mvp_bind_group` | `var<uniform> mvp` | camera matrix |
| **1** | `line_bind_group` | `var<uniform> line` | thickness + projection |
| **2** | `instance_bind_group` | `var<storage> instances` | per-object model + color |
| **3** | `segment_bind_group` | `var<storage> segments` | every edge in the scene |

<svg viewBox="0 0 690 232" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="each set_bind_group(N) plugs a bind group into slot N and the shader reads it as @group(N); the number is the only connection" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="345" y="16" fill="#d7dae0" text-anchor="middle">set_bind_group(N, …)  ⟷  @group(N) — the NUMBER is the only wire</text>
  <text x="130" y="38" fill="#888" text-anchor="middle">Rust — clear() draw</text>
  <text x="430" y="38" fill="#888" text-anchor="middle">cylinder.wgsl — reads slot N</text>
  <rect x="14" y="52" width="232" height="28" fill="none" stroke="#6fb3ff"/>
  <text x="22" y="70" fill="#d7dae0" font-size="10">set_bind_group(0, mvp_bind_group)</text>
  <line x1="246" y1="66" x2="259" y2="66" stroke="#6fb3ff"/>
  <circle cx="271" cy="66" r="12" fill="#11161c" stroke="#6fb3ff"/><text x="271" y="70" fill="#6fb3ff" text-anchor="middle">0</text>
  <line x1="283" y1="66" x2="296" y2="66" stroke="#6fb3ff"/>
  <rect x="296" y="52" width="252" height="28" fill="none" stroke="#3a3a3a"/>
  <text x="304" y="70" fill="#d7dae0" font-size="10">@group(0) @binding(0) mvp</text>
  <text x="556" y="70" fill="#666" font-size="10">camera matrix</text>
  <rect x="14" y="94" width="232" height="28" fill="none" stroke="#6fb3ff"/>
  <text x="22" y="112" fill="#d7dae0" font-size="10">set_bind_group(1, line_bind_group)</text>
  <line x1="246" y1="108" x2="259" y2="108" stroke="#6fb3ff"/>
  <circle cx="271" cy="108" r="12" fill="#11161c" stroke="#6fb3ff"/><text x="271" y="112" fill="#6fb3ff" text-anchor="middle">1</text>
  <line x1="283" y1="108" x2="296" y2="108" stroke="#6fb3ff"/>
  <rect x="296" y="94" width="252" height="28" fill="none" stroke="#3a3a3a"/>
  <text x="304" y="112" fill="#d7dae0" font-size="10">@group(1) @binding(0) line</text>
  <text x="556" y="112" fill="#666" font-size="10">thickness</text>
  <rect x="14" y="136" width="232" height="28" fill="none" stroke="#6fb3ff"/>
  <text x="22" y="154" fill="#d7dae0" font-size="10">set_bind_group(2, instance_bind_group)</text>
  <line x1="246" y1="150" x2="259" y2="150" stroke="#6fb3ff"/>
  <circle cx="271" cy="150" r="12" fill="#11161c" stroke="#6fb3ff"/><text x="271" y="154" fill="#6fb3ff" text-anchor="middle">2</text>
  <line x1="283" y1="150" x2="296" y2="150" stroke="#6fb3ff"/>
  <rect x="296" y="136" width="252" height="28" fill="none" stroke="#3a3a3a"/>
  <text x="304" y="154" fill="#d7dae0" font-size="10">@group(2) @binding(0) instances</text>
  <text x="556" y="154" fill="#666" font-size="10">per-object model</text>
  <rect x="14" y="178" width="232" height="28" fill="none" stroke="#5bbf87"/>
  <text x="22" y="196" fill="#d7dae0" font-size="10">set_bind_group(3, segment_bind_group)</text>
  <line x1="246" y1="192" x2="259" y2="192" stroke="#5bbf87"/>
  <circle cx="271" cy="192" r="12" fill="#11161c" stroke="#5bbf87"/><text x="271" y="196" fill="#5bbf87" text-anchor="middle">3</text>
  <line x1="283" y1="192" x2="296" y2="192" stroke="#5bbf87"/>
  <rect x="296" y="178" width="252" height="28" fill="none" stroke="#5bbf87"/>
  <text x="304" y="196" fill="#d7dae0" font-size="10">@group(3) @binding(0) segments</text>
  <text x="556" y="196" fill="#666" font-size="10">every edge</text>
  <text x="345" y="224" fill="#e06c6c" text-anchor="middle" font-size="10">swap any two numbers → the shader reads the wrong buffer. that's the entire contract.</text>
</svg>

`@binding(M)` is the *sub-slot inside* a group. Here every group holds one resource, so it's always
`@binding(0)`, and the bind group's `entries: &[BindGroupEntry { binding: 0, … }]` matches it.

**Why each resource is THREE objects** (the buffer + layout + bind-group boilerplate). Picture a **wall
socket**: the **layout** is the socket's *shape* ("a storage buffer at binding 0, vertex-visible") —
defines what fits, holds no data; the **buffer** is the *appliance* — the actual bytes
(`segment_buffer`); the **bind group** wires that one buffer to a matching plug. Then
`set_bind_group(3, &segment_bind_group)` plugs it into **outlet #3**, and `@group(3)` reads outlet #3.

<svg viewBox="0 0 690 210" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a buffer holding the bytes and a layout describing the shape combine into a bind group, which set_bind_group plugs into the slot the shader reads" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <defs><marker id="a2b31" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker><marker id="a2g31" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#5bbf87"/></marker></defs>
  <text x="345" y="16" fill="#d7dae0" text-anchor="middle">one resource = THREE gpu objects (the "segments" example)</text>
  <rect x="12" y="40" width="150" height="40" fill="none" stroke="#3a3a3a"/>
  <text x="87" y="58" fill="#d7dae0" text-anchor="middle" font-size="10">segments: Vec&lt;…&gt;</text>
  <text x="87" y="72" fill="#666" text-anchor="middle" font-size="9">your data — CPU</text>
  <text x="188" y="52" fill="#6fb3ff" text-anchor="middle" font-size="9">storage_buffer()</text>
  <line x1="162" y1="60" x2="212" y2="60" stroke="#6fb3ff" marker-end="url(#a2b31)"/>
  <rect x="214" y="40" width="150" height="40" fill="none" stroke="#6fb3ff"/>
  <text x="289" y="58" fill="#d7dae0" text-anchor="middle" font-size="10">segment_buffer</text>
  <text x="289" y="72" fill="#666" text-anchor="middle" font-size="9">the BYTES, on GPU</text>
  <rect x="60" y="126" width="180" height="40" fill="none" stroke="#5bbf87"/>
  <text x="150" y="144" fill="#d7dae0" text-anchor="middle" font-size="10">segment_layout</text>
  <text x="150" y="158" fill="#666" text-anchor="middle" font-size="9">the SHAPE — storage@0, vertex</text>
  <rect x="410" y="80" width="160" height="50" fill="none" stroke="#6fb3ff" stroke-width="1.3"/>
  <text x="490" y="100" fill="#d7dae0" text-anchor="middle" font-size="10">segment_bind_group</text>
  <text x="490" y="114" fill="#666" text-anchor="middle" font-size="9">the PLUG: this buffer,</text>
  <text x="490" y="125" fill="#666" text-anchor="middle" font-size="9">in that shape</text>
  <line x1="364" y1="66" x2="408" y2="92" stroke="#6fb3ff" marker-end="url(#a2b31)"/>
  <line x1="240" y1="150" x2="408" y2="116" stroke="#5bbf87" marker-end="url(#a2g31)"/>
  <text x="600" y="72" fill="#6fb3ff" text-anchor="middle" font-size="9">set_bind_group(3)</text>
  <line x1="570" y1="105" x2="610" y2="105" stroke="#6fb3ff" marker-end="url(#a2b31)"/>
  <rect x="612" y="86" width="72" height="40" fill="none" stroke="#5bbf87"/>
  <text x="648" y="103" fill="#5bbf87" text-anchor="middle" font-size="10">slot 3</text>
  <text x="648" y="117" fill="#666" text-anchor="middle" font-size="9">@group(3)</text>
  <text x="345" y="196" fill="#888" text-anchor="middle" font-size="10">layout = socket shape · buffer = the appliance · bind group = appliance on a matching plug</text>
</svg>

Every later pipeline repeats this exact recipe — one buffer + one bind group per resource, plugged into
the slot its `@group(N)` reads. The code *looks* repetitive because it **is** one recipe per resource;
32b walks the same four slots for the point cloud.

**6b. Refresh the screen-constant thickness.** The tube radius depends on the viewport height and the
camera's projection, so rewrite the line uniform each frame. Find the two `write_buffer` calls at the
top of `clear()` (the `time_buffer` and `mvp_buffer` writes) and add a third beside them:

`clear()` can't see the camera, so thread the two numbers through from `state.rs` — the camera owns
them (they're lesson-16's projection values). In `state.rs`, before calling `clear`:

```rust
        // The SAME numbers the projection was built from (16). S = the mm→m scale in view_proj.
        let (proj_y, ortho_h) = if self.camera.perspective {
            ((1.0 / f64::to_radians(30.0).tan() * 0.001) as f32, 0.0f32)     // cot(fovy/2) · S
        } else {
            // 16's ortho half-height
            let h = self.camera.distance * f64::to_radians(30.0).tan();
            // h ÷ S — see the unit note
            (1.0f32, (h / 0.001) as f32)
        };
```

pass them into `clear(…, proj_y, ortho_h)`, and fill the uniform there:

```rust
        let line = LineUniform {
            // px — later driven by the egui slider (lesson 51)
            thickness: 2.0,
            proj_y,
            // 0.0 selects the perspective branch in the shader
            ortho_h,
            vp_h: self.config.height as f32,
        };
        self.queue.write_buffer(&self.line_buffer, 0, bytemuck::bytes_of(&line));
```

> **The unit note (why × S on one side and ÷ S on the other).** The shader computes a radius in
> *world* units (mm). In perspective, `clip_w` already carries the view_proj's mm→m scale `S`, so
> `proj_y` must carry one `S` too — they cancel. In ortho there is no `clip_w` in the formula, so the
> scale must ride `ortho_h` instead, inverted: `h / S`. **Leaving `ortho_h` at 0.0 in ortho mode is
> the classic miss** (this lesson originally did — a reader caught it): the shader silently runs the
> *perspective* branch with ortho's constant `clip_w = 1`, and thickness stops matching perspective
> AND stops tracking zoom. Sanity check both modes: a `thickness: 8.0` line ≈ 8 px at any zoom, and
> toggling `Space` must not change any line's width by a pixel.

> `proj_y` / `ortho_h` come from your lesson-16 projection × the `mm→m` scale; constants above are the
> perspective default (fovy 60°, scale `0.001`). Cleanest is to compute both in
> `state.rs::render` from the `Camera`.

## Step 7 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

The five solids from lesson 30, now wearing crisp dark edges — all round tubes that hold their pixel
width as you zoom. Console (F12):

```
perf: 60.0 fps | 16.67 ms | 4 draws | 5 objects
```

**4 draws**, and it stays 4 as the scene grows — the table grows, the call count doesn't. Bump
`thickness` to `8.0` in `clear()` and the tubes fatten with **zero** re-upload.

## Recap

```
Ch 29: ONE mesh replayed N times — @builtin(instance_index) picks the row (identical geometry).
Ch 30: N DIFFERENT meshes in one arena, one draw_indexed; the vertex→row link is a per-vertex id.
Ch 31: EDGES as cylinders. One unit-cylinder template + one CylinderSegment row per edge (48 B:
       local p0/p1, radius sentinel, instance_id → object model+flags, inline per-edge color) in a
       flat storage table. ONE draw_indexed(0..template_idx, 0, 0..segments) for the WHOLE scene's
       linework. The vertex shader aligns +Z to (p1−p0) per instance — no per-segment matrix — and
       expands the radius to a SCREEN-CONSTANT pixel width via the `line` camera uniform. Per-edge
       colors come from the kernel's edges_with_colors() (edges walked in linecolor order, unlike
       the sorted edges()). The 23 LineList pipeline survives for overlays. 4 draws / 5 objects,
       flat as the scene grows; thickness changes are a single uniform write.
```

Edited: `session_rust/src/mesh.rs` (`edges_with_colors()` — per-edge colors in linecolor order),
`shaders/cylinder.wgsl` (NEW — instanced unit cylinder, +Z align, screen-constant radius),
`engine/pipelines/build.rs` (`build_cylinder_pipeline` + template layout), `engine/pipelines/mod.rs`
(`Pipelines.cylinder`), `engine/gpu.rs` (`CylinderSegment` + `LineUniform` rows, segment/template/line
buffers, per-frame thickness, one cylinder `draw_indexed`).

## Next

`32a-point-spheres.md` — the endpoints and points parked here arrive in two parts: first **sphere
glyphs** (a unit-sphere template instanced for line/curve endpoints and edit handles), then in 32b
**screen-space SDF billboards** for point clouds — 2 triangles per point drawn as anti-aliased
circles, ~70× cheaper than spheres. Same instance-table idea, one draw each; the 0-D counterpart to
this lesson's 1-D tubes.
