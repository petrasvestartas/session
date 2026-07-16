# 06 Vertex Buffer

Move the triangle's corners out of the shader and into a **buffer on the GPU**.

Until now the 3 corners were hard-coded in `triangle.wgsl`, picked by
`vertex_index` — fine by hand, but a real kernel mesh has thousands of vertices that
must be *uploaded*. This chapter sends the first real data across: one triangle, the
smallest upload — the same machinery later chapters reuse for meshes, lines, points.


## Mental model (read this first)

- **Vertex buffer** — flat GPU bytes, one struct per vertex/invocation.
- **Vertex layout** (`VertexBufferLayout`) — the *map* for those bytes: "24
  bytes/vertex; 0–11 = `@location(0)`, 12–23 = `@location(1)`", 3 floats each. Rust
  struct ⇄ shader inputs must agree.
- **`bytemuck`** — reinterprets `&[Vertex]` as raw `&[u8]`, no copy; needs
  `#[repr(C)]` + `Pod`.

## Files we touch

```
session_viewer/src/
├── shaders/triangle.wgsl                 # EDIT — read inputs instead of index lookup
└── engine/
    ├── pipelines/build.rs                 # EDIT — add `Vertex` type + give it to the pipeline
    └── gpu.rs                             # EDIT — create the buffer, bind & draw it
```

No new files. (In the archive, `Vertex` lives in `engine/gpu/types.rs` alongside
`MeshVertex`/`LineVertex`; we keep it next to the pipeline until there's more than one.)


## Step 1 — describe a vertex: `engine/pipelines/build.rs`

At the **top** of `build.rs`, add the vertex struct and layout — a trimmed copy of
the archive's `MeshVertex` (position + colour, minus normal and packed colour):

```rust
use bytemuck::{Pod, Zeroable};

/// One corner of geometry as the GPU stores it: 24 bytes — a position and an RGB colour.
/// `#[repr(C)]` fixes the field order so `bytemuck` can hand the raw bytes to the GPU.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    // Which bytes map to which shader @location. Must match `VsIn` in triangle.wgsl.
    const ATTRIBS: [wgpu::VertexAttribute; 2] = [
        wgpu::VertexAttribute {
            offset: 0,                                  // position starts at byte 0
            shader_location: 0,
            format: wgpu::VertexFormat::Float32x3,
        },
        wgpu::VertexAttribute {
            offset: 12,                                 // color starts after position (3 × 4 B)
            shader_location: 1,
            format: wgpu::VertexFormat::Float32x3,
        },
    ];

    /// The "how to read this buffer" descriptor handed to the pipeline.
    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,  // 24
            step_mode: wgpu::VertexStepMode::Vertex,   // advance once per vertex
            attributes: &Self::ATTRIBS,
        }
    }
}
```

> The `offset` is where each field's bytes begin inside one 24-byte vertex: `position` at 0,
> `color` right after it at 12 (3 × 4-byte floats). Reorder the struct fields → update the offsets.


## Step 2 — give the layout to the pipeline (same file)

In `build_triangle_pipeline`, `VertexState` currently says `buffers: &[]` (no vertex
buffer, corners live in the shader). Now there's one — pass its layout:

```rust
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[Vertex::layout()],   // <- was &[]
            compilation_options: Default::default(),
        },
```

That one change tells the pipeline to expect a vertex buffer shaped like `Vertex`.


## Step 3 — read inputs in the shader: `shaders/triangle.wgsl`

Replace the whole file — the vertex shader no longer builds/indexes arrays, it just
**receives** a position and colour per vertex:

```wgsl
// Corners now come from a vertex buffer (one Vertex per invocation), not from
// vertex_index. @location(0)/(1) line up with Vertex::ATTRIBS in build.rs.
// `aspect` carries over from chapter 05 — keep it so the triangle stays in shape.

@group(0) @binding(0) var<uniform> aspect: f32;   // = width / height

struct VsIn {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(in: VsIn) -> VsOut {
    var o: VsOut;                          // name it `o`, NOT `out` — see note below
    var p = vec4<f32>(in.position, 1.0);   // already in clip space (camera comes later)
    p.x = p.x / aspect;                    // chapter 05 aspect correction
    o.pos   = p;
    o.color = in.color;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
```

> **Don't name the variable `out`.** It's a reserved WGSL word — `var out: VsOut;`
> fails to parse, so the compiler reports every `out.pos`/`return out` after it as
> *"out is not declared"*, not the real cause. Use `o` (or `output`). `in` as a
> *parameter* name is fine — it isn't reserved.

> Keep `@group(0) @binding(0)` — `build.rs`'s layout and `gpu.rs`'s bind group for
> `aspect` already exist from chapter 05; the shader must still declare it or wgpu
> rejects the bound-but-undeclared group.

The locations are the contract: `@location(0)` here ⇄ `0 => Float32x3` in `ATTRIBS`
⇄ `position`. One out of sync and wgpu rejects the pipeline.


## Step 4 — create and store the buffer: `engine/gpu.rs`

**(a)** Make the `build` module public so `gpu.rs` can import `Vertex`. In
`engine/pipelines/mod.rs`, change the first line:

```rust
pub mod build;   // was: mod build;
```

At the top of `gpu.rs`, import `Vertex`. (`use wgpu::util::DeviceExt;` is already in
scope from chapter 05's aspect buffer — reuse it, don't duplicate.)

```rust
use crate::engine::pipelines::build::Vertex;
```

**(b)** Add two fields to `struct Gpu` — the buffer and its vertex count:

```rust
pub struct Gpu {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub pipelines: Pipelines,
    pub aspect_buffer: wgpu::Buffer,        // from chapter 05 (aspect)
    pub aspect_bind_group: wgpu::BindGroup, // from chapter 05 (aspect)
    pub vertex_buffer: wgpu::Buffer,    // <- ADD
    pub num_vertices: u32,              // <- ADD
}
```

**(c)** In `Gpu::new`, after `let pipelines = …`, define the 3 corners, upload them,
and add both fields to the returned struct:

```rust
        // `aspect_buffer` / `aspect_layout` / `aspect_bind_group` were set up just above
        // (chapter 05); `Pipelines::new` already takes the aspect layout.
        let pipelines = Pipelines::new(&device, config.format, &aspect_layout);

        // The triangle data, now Rust-side. Same 3 corners as before.
        const TRIANGLE: &[Vertex] = &[
            Vertex { position: [ 0.0,  0.5, 0.0], color: [1.0, 0.0, 0.0] },  // top    — red
            Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },  // left   — green
            Vertex { position: [ 0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },  // right  — blue
        ];
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("triangle.vbo"),
            contents: bytemuck::cast_slice(TRIANGLE),   // &[Vertex] → &[u8]
            usage: wgpu::BufferUsages::VERTEX,
        });
        let num_vertices = TRIANGLE.len() as u32;

        log::info!("viewer init OK — surface {}x{}, format {:?}", config.width, config.height, config.format);
        Ok(Self { surface, device, queue, config, pipelines, aspect_buffer, aspect_bind_group, vertex_buffer, num_vertices })
```


## Step 5 — bind the buffer in the draw: `engine/gpu.rs`

In `clear`, the render pass does `set_pipeline` then `draw(0..3, 0..1)`. Bind the
vertex buffer to slot 0 between them, and use `num_vertices` instead of the literal:

```rust
            pass.set_pipeline(&self.pipelines.triangle);
            pass.set_bind_group(0, &self.aspect_bind_group, &[]);      // keep — from chapter 05
            pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));   // <- ADD
            pass.draw(0..self.num_vertices, 0..1);                     // was 0..3
```

`slice(..)` means "the whole buffer". Vertex-buffer slot `0` matches the `buffers:
&[…]` entry from Step 2; bind-group slot `0` is a *separate* namespace (`aspect`) —
keep that line or shape correction is lost.


## Step 6 — run it

```bash
cd session_viewer && trunk serve   # http://localhost:8770  (Chrome/Edge)
```

Same **rainbow triangle** as Chapter 4 — that's the point. Nothing changed on
screen, only *where the data lives*: real GPU bytes controlled from Rust, not
constants baked in the shader.

Proof it's data-driven: tweak a `position`/`color` in `TRIANGLE`, save — the
triangle moves/recolours with no shader change.


## What changed vs Chapter 4 (recap)

```
Chapter 4:  corners hard-coded in WGSL, picked by vertex_index
Chapter 5 (Resize): unchanged geometry, correct canvas size
Chapter 6:  corners in a GPU vertex buffer; shader reads @location inputs
            └── the upload path every future mesh/line/point reuses
```

Edited: `triangle.wgsl` (inputs), `build.rs` (`Vertex` + layout), `gpu.rs` (buffer +
bind + draw). Untouched: `lib.rs`, `state.rs`.


## Compare to the archive

`session_viewer_archive` does exactly this, scaled up:
- Vertex types live in `engine/gpu/types.rs`: `MeshVertex` (+ **normal** + colour +
  `instance_id`), `LineVertex`, `PointVertex` — same `ATTRIBS`/`layout()` pattern.
- Colour is packed `Unorm8x4` (4 bytes) vs 3 floats, halving vertex size at scale.
- Buffers live in a growable `GpuArena`, re-uploaded as the scene changes, not a
  fixed triangle. Same `create_buffer_init`/`cast_slice` core.


## Next

The triangle is data now, but still flat in clip space, colour baked into the
geometry. Next (`07-uniforms.md`): a value sent from Rust via a **uniform + bind
group** — the mechanism that later carries the camera matrix, leading to **MVP** and
the **orbit camera**. (Index/cube geometry comes later — see `_ROADMAP.md`.)
</content>
