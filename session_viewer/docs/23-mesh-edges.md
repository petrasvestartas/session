# 23 Mesh edges — the CAD look, part one

A shaded solid without edges is a rendering; with dark edges it's a **drawing** — every CAD default
(Rhino, SolidWorks, Fusion) is shaded-plus-edges, because edges carry the shape when faces share a
color. This lesson adds them: the kernel lists each mesh's unique edges, we upload them once as line
segments, and a third pipeline draws them dark over the solids.

**v1 on purpose.** These are 1 px hardware lines — thickness needs the storage-buffer cylinder path
(unlocked at lesson 27, landed at 31). The *extraction* written today is permanent (31/62/63 reuse
it); only the pipeline is temporary, surviving as the wireframe/overlay path, like the archive's
`line.wgsl`.

## Why

```
mesh.edges()      kernel: every unique undirected edge, as (vkey_a, vkey_b) pairs
                  box = 12 edges, dodecahedron = 30
      │  two RenderVertex per edge (dark color), uploaded ONCE per mesh
      ▼
edges pipeline    LineList, drawn AFTER the solids, depth-tested (Less, no write)
```

The catch is **z-fighting**: an edge sits *exactly on* the surface it outlines, so line and face
fragments have near-equal depth and flicker. The classic fix, `DepthBiasState`, **doesn't apply to
line topologies in WebGPU** (triangles-only) — so we bias in the shader instead, pulling each edge a
hair toward the camera:

```
o.pos.z = o.pos.z - 1e-4 * o.pos.w;    // after the perspective divide: z_ndc shifts by 1e-4
```

which beats the face's depth everywhere without visibly floating. (Lesson 31's tubes skip this — a
cylinder physically protrudes from the surface.)

## Files we touch

```
src/shaders/edges.wgsl             # NEW — passthrough + the depth nudge
src/engine/pipelines/build.rs      # build_edges_pipeline (LineList, RenderVertex layout)
src/engine/pipelines/mod.rs        # add `edges` to Pipelines
src/engine/gpu.rs                  # edge buffer per mesh; draw them after the solids
```

## Step 1 — the edges shader: `src/shaders/edges.wgsl`

Reads the camera and a `RenderVertex` (position @0, color @2, normal @1 unused), passes color
through, and applies the depth nudge:

```wgsl
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;

struct VsIn {
    @location(0) position: vec3<f32>,
    @location(2) color: vec3<f32>,
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(in: VsIn) -> VsOut {
    var o: VsOut;
    o.pos = mvp * vec4<f32>(in.position, 1.0);
    o.pos.z = o.pos.z - 1e-4 * o.pos.w;   // shader-side depth bias (WebGPU has none for lines)
    o.color = in.color;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
```

## Step 2 — the pipeline: `src/engine/pipelines/build.rs`

Copy `build_grid_pipeline`, changing three things: label(s), shader file, and — since edges DO have
a vertex buffer, unlike the vertexless grid — the buffers. Topology stays `LineList`; depth stays
write-off + `Less` (the shader nudge does the rest). Add below `build_grid_pipeline`:

```rust
pub fn build_edges_pipeline(
    device: &wgpu::Device,
    color_format: wgpu::TextureFormat,
    aspect_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("edges.shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/edges.wgsl").into()),
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("edges.layout"),
        bind_group_layouts: &[Some(aspect_layout)],   // group 0 = camera mvp only
        immediate_size: 0,
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("edges"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[RenderVertex::layout()],   // ← real vertices this time (grid had &[])
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
            topology: wgpu::PrimitiveTopology::LineList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(false),   // test against depth, never write it
            depth_compare: Some(wgpu::CompareFunction::Less),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview_mask: None,
        cache: None,
    })
}
```

(`RenderVertex` is already imported at the top of `build.rs` — the triangle pipeline uses
`RenderVertex::layout()` since lesson 19.)

## Step 3 — register it: `src/engine/pipelines/mod.rs`

Same move as lesson 20 — an `edges` field on `Pipelines`, built with the camera layout only. Add
`build_edges_pipeline` to the existing `use build::{…}` line, then:

```rust
pub struct Pipelines {
    pub triangle: wgpu::RenderPipeline,
    pub grid: wgpu::RenderPipeline,
    pub edges: wgpu::RenderPipeline,      // ← new
}
```

In `Pipelines::new`, find `grid: build_grid_pipeline(device, color_format, aspect_layout),` and add
after it:

```rust
            edges: build_edges_pipeline(device, color_format, aspect_layout),
```

## Step 4 — extract + upload the edges: `src/engine/gpu.rs`

Add `RenderVertex` to the kernel import, and a buffer list to the struct:

```rust
use session_rust::{Color, Mesh, RenderVertex, Xform};   // + RenderVertex
```

```rust
    pub edge_buffers: Vec<(wgpu::Buffer, u32)>,   // one (vbo, vertex_count) per mesh
```

In `new()`, right after `let meshes = vec![…]`, ask the kernel for each mesh's edges and flatten to
line vertices — two per edge — via `RenderVertex::point(pt, &color)`, a kernel constructor taking a
`Point` + a `&Color` that does **both conversions inside**: f64→f32 position cast and
`Color`→`[f32; 4]` unpack. No `as f32`, no `[c.r, c.g, c.b, c.a]` at the call site (it also zeroes
the normal, which the unlit edge shader ignores).

**Don't invent the color — read it from the kernel.** `Mesh::add_face` seeds every new edge
`Color::black()` (mesh.rs / mesh.cpp / mesh.py — one shared default across languages); `RenderVertex::
point` takes the `&Color` directly. We sample **one** color per mesh, not per edge, on purpose:
`edges()` returns sorted `(u,v)` order while `linecolors` is `add_face` insertion order — **not
index-aligned**, so `linecolors[i]` would mismatch `edges()[i]` once edges carry different colors.
Every edge shares the default here, so one sample is exact; true per-edge colors wait for the
first-class edge path in lesson 31.

```rust
        let mut edge_buffers: Vec<(wgpu::Buffer, u32)> = Vec::new();
        for mesh in &meshes {
            // Edge color straight from the kernel (all edges default to Color::black()).
            let ec = mesh.edge_color();
            let mut verts: Vec<RenderVertex> = Vec::new();
            for (a, b) in mesh.edges() {
                let pa = mesh.vertex_point(a).unwrap();
                let pb = mesh.vertex_point(b).unwrap();
                verts.push(RenderVertex::point(pa, &ec));
                verts.push(RenderVertex::point(pb, &ec));
            }
            let vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("edges.vbo"),
                contents: bytemuck::cast_slice(&verts),
                usage: wgpu::BufferUsages::VERTEX,
            });
            edge_buffers.push((vbo, verts.len() as u32));
        }
```

Return `edge_buffers` in `Ok(Self { … })`. (Built once, like `gpu_mesh` — these meshes never change
here. When edits arrive, edge buffers rebuild on the same invalidation GpuMesh uses.)

## Step 5 — draw them after the solids: `src/engine/gpu.rs`

In `clear()`, after the mesh loop: edges bind only the camera (group 0), one plain `draw` per mesh
(lines aren't indexed):

```rust
            // edges last — the shader nudges them toward the camera so they beat the faces
            pass.set_pipeline(&self.pipelines.edges);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            for (vbo, count) in &self.edge_buffers {
                pass.set_vertex_buffer(0, vbo.slice(..));
                pass.draw(0..*count, 0..1);
            }
```

## Step 6 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

The box shows its 12 dark edges, each dodecahedron its 30 — everything reads as a *part*, not a
blob. Orbit: edges stay glued to the surface, no flicker (the nudge at work). The smooth
dodecahedron shows all 30 edges too — a real CAD display would outline only *creases* on a smooth
solid; that dihedral-angle filter arrives with the cylinder path in lesson 31.

## Recap

```
Ch 22: shaded solids — flat or smooth — but shapes still merge into blobs.
Ch 23: the kernel's mesh.edges() → two dark RenderVertex per edge, uploaded once per mesh, drawn
       by a third pipeline (LineList) AFTER the solids. Z-fight solved in the SHADER
       (o.pos.z -= 1e-4 * w) because WebGPU depth bias is triangles-only. v1 on purpose: 1 px
       hardware lines; lesson 31 upgrades them to screen-constant-thickness tubes and reuses
       today's extraction unchanged.
```

Edited: `shaders/edges.wgsl` (new), `pipelines/build.rs` (`build_edges_pipeline`),
`pipelines/mod.rs` (`edges` field), `engine/gpu.rs` (`edge_buffers` build in `new()`, draw loop
after solids in `clear()`).

## Next

`24-msaa.md` — those 1 px edges (and the grid) are jagged. One lesson of 4× MSAA — a multisampled
color+depth target resolved to the surface — smooths every line and silhouette in the scene.
