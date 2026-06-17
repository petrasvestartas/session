# 05 Vertex Buffer

Move the triangle's corners out of the shader and into a **buffer on the GPU**.

Until now the 3 corners were hard-coded inside `triangle.wgsl` and picked by
`vertex_index`. That only works for shapes you can type by hand. Real geometry — a
mesh from the kernel — has thousands of vertices that live in memory and must be
*uploaded* to the GPU. This chapter is the first time we send our own data across:
the smallest possible upload, one triangle. The machinery here is exactly what later
chapters reuse for meshes, lines, and points.


## Mental model (read this first)

- **Vertex buffer** — a flat block of bytes on the GPU holding one struct per vertex.
  The GPU walks it, handing one struct to the vertex shader per invocation.
- **Vertex layout** (`VertexBufferLayout`) — the *map* that tells the GPU how to read
  those bytes: "each vertex is 24 bytes; bytes 0–11 are `@location(0)` as 3 floats,
  bytes 12–23 are `@location(1)` as 3 floats". Rust struct ⇄ shader inputs must agree.
- **`bytemuck`** — lets us reinterpret a `&[Vertex]` as the raw `&[u8]` the GPU wants,
  with no copying. The struct must be `#[repr(C)]` + `Pod` (plain old data) for this
  to be sound.

The flow we're building:

```
Rust: [Vertex; 3]  ──bytemuck──▶  bytes  ──create_buffer──▶  GPU vertex buffer
                                                                    │
draw: set_vertex_buffer(0, …) ; draw(0..3)  ──▶  shader reads @location(0),(1)
```


## Files we touch

```
session_viewer/src/
├── shaders/triangle.wgsl                 # EDIT — read inputs instead of index lookup
└── engine/
    ├── pipelines/build.rs                 # EDIT — add `Vertex` type + give it to the pipeline
    └── gpu.rs                             # EDIT — create the buffer, bind & draw it
```

No new files. (In the archive this `Vertex` type lives in `engine/gpu/types.rs`
alongside `MeshVertex`/`LineVertex`; we keep it next to the pipeline for now and move
it later when there's more than one.)


## Step 1 — describe a vertex: `engine/pipelines/build.rs`

At the **top** of `build.rs`, add the vertex struct and its layout. This is a trimmed
copy of the archive's `MeshVertex` (position + colour, minus the normal and packed
colour we don't need yet):

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
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

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

> `vertex_attr_array![0 => Float32x3, 1 => Float32x3]` computes the byte offsets for
> you (location 1 starts at offset 12). If you reorder the struct fields, update this.


## Step 2 — give the layout to the pipeline (same file)

In `build_triangle_pipeline`, the `VertexState` currently says `buffers: &[]` ("no
vertex buffer — corners are in the shader"). Now there *is* one — pass its layout:

```rust
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[Vertex::layout()],   // <- was &[]
            compilation_options: Default::default(),
        },
```

That single change tells the pipeline to expect one vertex buffer shaped like `Vertex`.


## Step 3 — read inputs in the shader: `shaders/triangle.wgsl`

Replace the whole file. The vertex shader no longer builds arrays and indexes them —
it just **receives** a position and colour per vertex:

```wgsl
// Corners now come from a vertex buffer (one Vertex per invocation), not from
// vertex_index. @location(0)/(1) line up with Vertex::ATTRIBS in build.rs.

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
    var out: VsOut;
    out.pos   = vec4<f32>(in.position, 1.0);   // already in clip space (camera comes later)
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
```

The locations are the contract: `@location(0)` here ⇄ `0 => Float32x3` in `ATTRIBS` ⇄
`position` field. Get one out of sync and wgpu rejects the pipeline.


## Step 4 — create and store the buffer: `engine/gpu.rs`

**(a)** At the top of `gpu.rs`, bring in the upload helper and the `Vertex` type:

```rust
use wgpu::util::DeviceExt;                          // for create_buffer_init
use crate::engine::pipelines::build::Vertex;
```

**(b)** Add two fields to `struct Gpu` (the buffer and how many vertices it holds):

```rust
pub struct Gpu {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub pipelines: Pipelines,
    pub vertex_buffer: wgpu::Buffer,    // <- ADD
    pub num_vertices: u32,              // <- ADD
}
```

**(c)** In `Gpu::new`, after `let pipelines = …`, define the 3 corners and upload them,
then add both fields to the returned struct:

```rust
        let pipelines = Pipelines::new(&device, config.format);

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
        Ok(Self { surface, device, queue, config, pipelines, vertex_buffer, num_vertices })
```


## Step 5 — bind the buffer in the draw: `engine/gpu.rs`

In `clear`, the render pass currently does `set_pipeline` then `draw(0..3, 0..1)`. Bind
the vertex buffer to slot 0 between them, and use `num_vertices` instead of the literal:

```rust
            pass.set_pipeline(&self.pipelines.triangle);
            pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));   // <- ADD
            pass.draw(0..self.num_vertices, 0..1);                     // was 0..3
```

`slice(..)` means "the whole buffer". Slot `0` matches the single `buffers: &[…]` entry
we set in Step 2.


## Step 6 — run it

```bash
cd session_viewer && trunk serve   # http://localhost:8770  (Chrome/Edge)
```

You should see the **exact same rainbow triangle** as Chapter 3 — that's the point.
Nothing changed on screen; what changed is *where the data lives*. The corners are now
real bytes on the GPU that we control from Rust, not constants baked in the shader.

Quick proof it's data-driven: tweak a `position` or `color` in `TRIANGLE`, save, and the
triangle moves/recolours without touching the shader.


## What changed vs Chapter 3 (recap)

```
Chapter 3:  corners hard-coded in WGSL, picked by vertex_index
Chapter 4 (Resize): unchanged geometry, correct canvas size
Chapter 5:  corners in a GPU vertex buffer; shader reads @location inputs
            └── the upload path every future mesh/line/point reuses
```

Edited: `triangle.wgsl` (inputs), `build.rs` (`Vertex` + layout), `gpu.rs` (buffer +
bind + draw). Untouched: `lib.rs`, `state.rs`.


## Compare to the archive

`session_viewer_archive` does exactly this, scaled up:
- The vertex types live in `engine/gpu/types.rs`: `MeshVertex` (position + **normal** +
  colour + `instance_id`), `LineVertex`, `PointVertex` — each with the same
  `ATTRIBS` / `layout()` pattern you just wrote.
- Colour there is packed as `Unorm8x4` (4 bytes, `[u8;4]`) instead of 3 floats, to
  halve vertex size — an optimisation worth doing once vertex counts get large.
- Buffers aren't created once for a fixed triangle; they live in a growable `GpuArena`
  that re-uploads as the scene changes. Same `create_buffer_init` / `cast_slice` core.


## Next

The triangle is data now, but still flat in clip space and the colour is baked into the
geometry. Next (`07-uniforms.md`) we send a value from Rust into the shader via a
**uniform + bind group** — the same mechanism that will later carry the camera matrix.
That leads into **MVP matrix** and the **orbit camera**, where this vertex data finally
sits in a real 3D world. (Index/cube geometry comes a little later — see `_ROADMAP.md`.)
