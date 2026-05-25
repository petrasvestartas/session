# session_viewer — A Full WebGPU Tutorial

A WebAssembly 3D viewer for session geometry, built with wgpu + winit + Trunk.
All geometry is authored in **millimetres**. The viewer automatically converts to metres
(1 viewer unit = 1 m) by baking a 0.001 scale into every view-projection matrix.

---

## Quick Start

```
cd session_viewer
cargo clean
cmd //c "taskkill /F /IM trunk.exe"
trunk serve
```

Open http://localhost:8080 in your browser. Hot-reload is active on file save.

---

## How This Tutorial Is Organised

This file follows the same arc as https://sotrh.github.io/learn-wgpu/beginner/tutorial4-buffer
but maps every concept directly onto the session_viewer source code so you can see
*exactly* where each wgpu concept lives and why.

```
Section 1  — The GPU Object Hierarchy (Instance → Device → Queue)
Section 2  — What is a Buffer?
Section 3  — Vertex Structs and bytemuck
Section 4  — VertexBufferLayout
Section 5  — Creating Buffers
Section 6  — GpuArena: Dynamic Growing Buffers
Section 7  — Index Buffers
Section 8  — The Instance Storage Buffer
Section 9  — The Camera Uniform Buffer
Section 10 — Bind Groups and Bind Group Layouts
Section 11 — Render Pipelines
Section 12 — Shaders (WGSL)
Section 13 — The Render Loop
Section 14 — Camera: Projection, Named Views, MM Scale
Section 15 — The Grid — Procedural LineList (No VBO)
Section 16 — Camera Controls
```

---

## Section 1 — The GPU Object Hierarchy

Before any buffer can exist you need five wgpu objects. They form a strict
creation hierarchy — each one depends on the previous.

```
Browser / OS
     │
     │ (driver layer)
     ▼
wgpu::Instance                ← handle to the GPU driver stack
     │  .request_adapter()
     ▼
wgpu::Adapter                 ← handle to one physical GPU
     │  .request_device()
     ├──────────────────────────────────────────┐
     ▼                                          ▼
wgpu::Device                             wgpu::Queue
(logical GPU — create                    (submit command
 buffers, textures,                       buffers to the GPU)
 pipelines, bind groups)
     │
     │  .create_surface()  (needs the window first)
     ▼
wgpu::Surface                 ← the drawable canvas
     │  .configure()
     ▼
wgpu::SurfaceConfiguration    ← pixel format, width/height, vsync mode
```

In `lib.rs` `State::new()`:

```rust
// Step 1 — GPU driver stack
let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
    backends: wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::GL,
    ..
});

// Step 2 — physical GPU adapter
let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
    compatible_surface: Some(&surface),
    ..
}).await?;

// Step 3 — logical device + submit queue (always come in a pair)
let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
    required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
    ..
}).await?;
```

`device` creates things. `queue` sends them to the GPU.
Every buffer, texture, pipeline, and bind group is owned by a specific `device`.

---

## Section 2 — What is a Buffer?

> "A buffer is a blob of data on the GPU. It stores data contiguously and can be
>  accessed by the CPU (to write) and the GPU (to read, millions of times per frame)."
> — learn-wgpu tutorial 4

A buffer is **not** a Rust `Vec`. It is allocated in GPU-accessible memory.
The CPU hands data in once (or occasionally); the GPU reads it during every draw call.

Session uses four distinct kinds:

```
┌─────────────────────────────────────────────────────────────────────┐
│ VERTEX BUFFER (VBO)                                                 │
│                                                                     │
│ Contents:  per-vertex data — position, normal, colour               │
│ Written:   once when geometry is added / updated                    │
│ Read by:   vertex shader — once per vertex per draw call            │
│ Size:      MeshVertex=28 B × vertex count  (e.g. 4096 vertices      │
│            = 114 688 B ≈ 112 KB for a mesh arena)                   │
│ Usage:     VERTEX | COPY_DST                                        │
│                                                                     │
│ Code:  GpuArena<MeshVertex>.vbo   (gpu_arena.rs)                    │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│ INDEX BUFFER (IBO)                                                  │
│                                                                     │
│ Contents:  u32 indices into the VBO                                 │
│ Written:   once when geometry is added                              │
│ Read by:   GPU during draw_indexed() — assembles triangles / lines  │
│            from reused vertices without duplicating position data   │
│ Size:      4 B × index count  (e.g. 8192 indices = 32 768 B)        │
│ Usage:     INDEX | COPY_DST                                         │
│                                                                     │
│ Code:  GpuArena<MeshVertex>.ibo   (gpu_arena.rs)                    │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│ UNIFORM BUFFER                                                      │
│                                                                     │
│ Contents:  one value, same for every vertex in the draw call        │
│ Written:   every frame (camera moves every frame)                   │
│ Read by:   vertex shader + fragment shader via @group(0) @binding(0)│
│ Size:      64 bytes — one 4×4 f32 matrix                            │
│ Usage:     UNIFORM | COPY_DST                                       │
│                                                                     │
│ Code:  camera_buf   (pipelines.rs create_camera_buffer)             │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│ STORAGE BUFFER (read-only)                                          │
│                                                                     │
│ Contents:  array of per-object data — transform, colour, flags      │
│ Written:   when object is added, moved, recoloured, or selected     │
│ Read by:   vertex shader via @group(0) @binding(1)                  │
│            indexed by @builtin(instance_index)                      │
│ Size:      96 B × object count  (e.g. 1024 objects = 98 304 B)      │
│ Usage:     STORAGE | COPY_DST | COPY_SRC                            │
│                                                                     │
│ Code:  GpuSession.instance_buffer   (gpu_session.rs)                │
└─────────────────────────────────────────────────────────────────────┘
```

All four live in the GPU simultaneously. The render loop binds them and
the GPU reads them without any further CPU involvement per draw call.

---

## Section 3 — Vertex Structs and bytemuck

### The tutorial's vertex

The learn-wgpu tutorial defines:

```rust
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],   // 12 bytes — XYZ as 32-bit floats
    color:    [f32; 3],   // 12 bytes — RGB as 32-bit floats
}                         // total: 24 bytes per vertex
```

`#[repr(C)]` forces C memory layout — field order is guaranteed, no reordering.
Without it Rust can reorder fields for alignment, making the byte offsets
unpredictable for the GPU.

### bytemuck — safe byte casting

The GPU only accepts `&[u8]` (raw bytes). `bytemuck` lets you safely cast
a Rust struct to bytes without `unsafe`:

```
Your struct in Rust memory        Raw bytes sent to GPU
                                  (what queue.write_buffer sees)
┌───────────────────────┐         ┌─────────────────────┐
│ position: [f32; 3]    │  ──►   │ 00 00 80 3F  (1.0f)  │
│   x = 1.0             │         │ 00 00 00 00  (0.0f)  │
│   y = 0.0             │         │ 00 00 00 00  (0.0f)  │
│   z = 0.0             │         │ ...                  │
│ color:    [f32; 3]    │         │ ...                  │
│   r = 0.5             │         │ ...12 bytes...       │
│   g = 0.2             │         └─────────────────────┘
│   b = 0.8             │
└───────────────────────┘

bytemuck::bytes_of(&vertex)   → &[u8] of the struct
bytemuck::cast_slice(&verts)  → &[u8] of a Vec<Vertex>
```

`Pod` ("Plain Old Data") means the struct has no padding, no pointers,
no invalid bit patterns. `Zeroable` means all-zeros is a valid value.
Both are required for `bytemuck` to allow the cast.

### Session's three vertex types

Session uses colour as `[u8; 4]` instead of `[f32; 4]`, cutting colour
bandwidth in half. The GPU automatically converts `Unorm8x4` (0–255) to
`vec4<f32>` (0.0–1.0) in the shader with no cost.

```
┌─────────────────────────────────────────────────────────────────────┐
│  MeshVertex — 28 bytes                    (gpu_session.rs line 160) │
│                                                                     │
│  byte  0 │ position [f32; 3] │ 12 B │ @location(0)  Float32x3      │
│  byte 12 │ normal   [f32; 3] │ 12 B │ @location(1)  Float32x3      │
│  byte 24 │ color    [u8;  4] │  4 B │ @location(2)  Unorm8x4       │
│                                                                     │
│  memory:  0    4    8   12   16   20   24  25  26  27               │
│           ├────┼────┼────┼────┼────┼────┼───┼───┼───┤              │
│           │ px │ py │ pz │ nx │ ny │ nz │ R │ G │ B │ A            │
│           └─────────────┴─────────────┴───────────────┘            │
│             position         normal         colour                  │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│  LineVertex — 16 bytes                    (gpu_session.rs line 182) │
│                                                                     │
│  byte  0 │ position [f32; 3] │ 12 B │ @location(0)  Float32x3      │
│  byte 12 │ color    [u8;  4] │  4 B │ @location(1)  Unorm8x4       │
│                                                                     │
│  memory:  0    4    8   12  13  14  15                              │
│           ├────┼────┼────┼───┼───┼───┤                             │
│           │ px │ py │ pz │ R │ G │ B │ A                           │
│           └─────────────┴───────────┘                              │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│  PointVertex — 16 bytes    (same layout as LineVertex)              │
└─────────────────────────────────────────────────────────────────────┘
```

Rust definitions (gpu_session.rs):

```rust
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MeshVertex {
    pub position: [f32; 3],   // 12 bytes
    pub normal:   [f32; 3],   // 12 bytes
    pub color:    [u8; 4],    //  4 bytes
}                             // total: 28 bytes

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LineVertex {
    pub position: [f32; 3],   // 12 bytes
    pub color:    [u8; 4],    //  4 bytes
}                             // total: 16 bytes
```

---

## Section 4 — VertexBufferLayout

After the vertex struct, wgpu needs a **map** from byte offsets to shader
`@location(N)` inputs. This is the `VertexBufferLayout`.

```
VertexBufferLayout says:
  "each vertex is N bytes apart"  →  array_stride
  "advance per vertex or per instance"  →  step_mode
  "these byte ranges map to these shader inputs"  →  attributes

                   array_stride = 28 bytes
                   ◄───────────────────────►
VBO bytes:   [Vertex 0              ][Vertex 1              ]...
              px py pz nx ny nz R G B A  px py pz nx ny nz R G B A
              ↑           ↑           ↑
              offset:0    offset:12   offset:24
              Float32x3   Float32x3   Unorm8x4
              @location(0) @location(1) @location(2)
```

Session's layout method (gpu_session.rs):

```rust
impl MeshVertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![
            0 => Float32x3,   // position  @location(0)
            1 => Float32x3,   // normal    @location(1)
            2 => Unorm8x4,    // color     @location(2)
        ];

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
```

`wgpu::vertex_attr_array!` is a macro that computes the byte offsets for you
from the sequential field sizes. You still need to ensure the struct field
order matches what you declare.

### What happens without VertexBufferLayout?

The grid pipeline uses `buffers: &[]` — no vertex buffer at all.
Vertex positions are computed inside the vertex shader from
`@builtin(vertex_index)`. The layout is literally empty:

```rust
vertex: wgpu::VertexState {
    module: &shader,
    entry_point: Some("vs_main"),
    buffers: &[],    // ← no VBO; shader generates positions procedurally
    ..
},
```

---

## Section 5 — Creating Buffers

wgpu has two ways to create a buffer:

```
device.create_buffer()       — allocates empty GPU memory, no initial data
                               used when you will write data later via queue

device.create_buffer_init()  — allocates + uploads initial data in one call
                               requires wgpu::util::DeviceExt trait in scope
                               used for data you have ready at creation time
```

### Usage flags — the GPU needs to know in advance

```
VERTEX   → can be bound as a vertex buffer (set_vertex_buffer)
INDEX    → can be bound as an index buffer (set_index_buffer)
UNIFORM  → can be bound as a uniform buffer (small, read-only, per-draw)
STORAGE  → can be bound as a storage buffer (large, indexed per-object)
COPY_DST → CPU can write to it via queue.write_buffer
COPY_SRC → GPU can copy from it to another buffer
```

You must declare all usages at creation. You cannot add them later.

### Camera buffer — created with `create_buffer_init`

```rust
// pipelines.rs
pub fn create_camera_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    use wgpu::util::DeviceExt;
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label:    Some("session.camera"),
        contents: bytemuck::bytes_of(&CameraUniform::default()),
        usage:    wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}
```

The buffer starts as an identity matrix. Every frame `queue.write_buffer`
overwrites the same 64 bytes with the new camera transform.

### VBO — created with `create_buffer` (empty, written later)

```rust
// gpu_arena.rs
let vbo = device.create_buffer(&wgpu::BufferDescriptor {
    label:              Some("gpu_session.tri.vbo"),
    size:               4096 * 28,  // 4096 MeshVertices × 28 bytes
    usage:              wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    mapped_at_creation: false,
});
```

Data arrives later via `queue.write_buffer` as each geometry is added.

### Instance buffer — created with `create_buffer` + extra flags

```rust
// gpu_session.rs
let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
    label: Some("gpu_session.instances"),
    size:  1024 * 96,   // 1024 objects × 96 bytes each
    usage: wgpu::BufferUsages::STORAGE
         | wgpu::BufferUsages::COPY_DST
         | wgpu::BufferUsages::COPY_SRC,  // needed to copy when growing
    mapped_at_creation: false,
});
```

`COPY_SRC` is needed here because when the buffer grows, we copy old data
into the new buffer with `encoder.copy_buffer_to_buffer`.

---

## Section 6 — GpuArena: Dynamic Growing Buffers

The tutorial creates one static buffer for a single pentagon.
Session uses `GpuArena<V>` — a **dynamic growing arena** that holds *all*
objects of one topology type in a single VBO + IBO pair.

```
GpuArena<MeshVertex>
┌───────────────────────────────────────────────────────────────────┐
│  VBO (wgpu::Buffer)  — one contiguous block for all meshes        │
│                                                                   │
│  slot "abc"          │ slot "def"  │ slot "ghi"  │   free...      │
│  (triangle, 3 verts) │ (quad, 4 v) │ (box, 24 v) │               │
│  v0 v1 v2            │ v0 v1 v2 v3 │ v0..v23     │               │
│  ◄──────────────────►│◄───────────►│◄───────────►│               │
│   vertex_range 0..3   vertex_range  vertex_range                  │
│                       3..7          7..31                         │
│                                                                   │
│  IBO (wgpu::Buffer)  — indices for all meshes                     │
│                                                                   │
│  [0,1,2,            │ [0,1,2,     │ [0,1,2, ...               │
│   slot offset=0]     │  1,2,3,     │  slot offset=7]            │
│                      │  slot off=3]│                             │
└───────────────────────────────────────────────────────────────────┘

ArenaSlot (stored in HashMap<String, ArenaSlot>):
  vertex_range:  0..3      ← byte range in the VBO
  index_range:   0..3      ← byte range in the IBO
  instance_id:   0         ← index into the instance storage buffer
```

### Allocation strategy

```
When adding a new geometry:

 1. Does the free list have a big enough gap?
       yes → reuse that gap (first-fit)
       no  → try cursor (append to end)
               if cursor at capacity → grow (double the buffer, copy existing)
```

```rust
// gpu_arena.rs — cursor allocation path
fn allocate_vertex_range(&mut self, n: u32, ..) -> Range<u32> {
    // Try free list first
    if let Some(idx) = self.free_verts.iter().position(|r| r.len() >= n) {
        // ... reuse the hole
    }
    // Append at cursor
    if self.cursor_verts + n <= self.capacity_verts {
        let start = self.cursor_verts;
        self.cursor_verts += n;
        return start..(start + n);
    }
    // Buffer full — grow (allocates new 2× buffer, copies everything)
    self.grow_vertex_buffer(self.cursor_verts + n, ..);
    // ... then append at cursor
}
```

### Growing a buffer — why COPY_SRC is needed

wgpu buffers are immutable in size after creation. Growing means:
1. Allocate a new buffer at 2× the old size
2. Copy all existing data from old → new via `encoder.copy_buffer_to_buffer`
3. Replace the old buffer reference

```rust
fn grow_vertex_buffer(&mut self, needed: u32, device: &wgpu::Device, queue: &wgpu::Queue) {
    let new_cap = self.capacity_verts.max(1) * 2;  // double
    let new_vbo = device.create_buffer(&wgpu::BufferDescriptor {
        size:  new_cap as u64 * size_of::<V>() as u64,
        usage: wgpu::BufferUsages::VERTEX
             | wgpu::BufferUsages::COPY_DST
             | wgpu::BufferUsages::COPY_SRC,  // ← needed on old buffer too
        ..
    });
    let mut encoder = device.create_command_encoder(..);
    encoder.copy_buffer_to_buffer(
        &self.vbo,    0,   // source: old buffer
        &new_vbo,     0,   // dest:   new buffer
        bytes_used,        // only copy the live portion
    );
    queue.submit(once(encoder.finish()));
    self.vbo = new_vbo;    // drop the old buffer (reference count → 0)
    self.capacity_verts = new_cap;
}
```

This GPU-to-GPU copy is much faster than re-uploading from CPU because the
data never crosses the PCIe bus again.

---

## Section 7 — Index Buffers

### The tutorial's pentagon example

Without indices, a quad needs 6 vertices (two triangles, duplicating 2 corner verts):

```
Without IBO (6 vertices):          With IBO (4 verts + 6 indices):

  v0──v1                             v0──v1
  │  ╲ │    6 verts × 24 B = 144 B  │    │    4 verts × 24 B = 96 B
  │   ╲│                            │    │  + 6 inds  ×  4 B = 24 B
  v3──v2                             v3──v2               total: 120 B

  The duplication pays off at scale:
  A 10 000 vertex mesh: IBO saves ~33% memory and avoids redundant
  vertex shader invocations for shared verts.
```

### How session uses index buffers for meshes

Every mesh added to the tri arena has both a VBO slice and an IBO slice:

```
Tri arena VBO:                    Tri arena IBO:
  [v0 v1 v2 v3 ...]               [0 1 2  0 2 3 ...]
   ◄─ slot "abc" ─►                ◄─── slot "abc" ──►
   vertex_range 0..4               index_range  0..6

Draw call emitted for slot "abc":
  pass.set_vertex_buffer(0, tri.vbo.slice(..));
  pass.set_index_buffer(ibo.slice(..), IndexFormat::Uint32);
  pass.draw_indexed(
      0..6,          ← index_range (6 indices)
      0,             ← base_vertex (vertex_range.start)
      0..1,          ← instance range (instance_id)
  );

The GPU:
  index 0 → read vertex at (base_vertex + 0) = v0
  index 1 → read vertex at (base_vertex + 1) = v1
  index 2 → read vertex at (base_vertex + 2) = v2
  → triangle 0: v0, v1, v2
  index 0 → v0  (reused! no duplicate in VBO)
  index 2 → v2  (reused!)
  index 3 → v3
  → triangle 1: v0, v2, v3
```

The `base_vertex` offset is what lets multiple geometries share one VBO —
each slot's indices are relative to 0 but the base_vertex shifts them to
the right position in the global buffer.

### Lines do not need indices for the geometry they represent

A polyline with N points has N-1 line segments = 2(N-1) index entries.
An index buffer is still used because it allows the same VBO point to be
referenced by two adjacent segments without duplication.

```rust
// gpu_session.rs — building line indices for a polyline
let n = verts.len();
let mut inds: Vec<u32> = Vec::with_capacity((n - 1) * 2);
for i in 0..n - 1 {
    inds.push(i as u32);
    inds.push((i + 1) as u32);
}
// indices: [0,1, 1,2, 2,3, 3,4, ...]
//           seg0  seg1  seg2  seg3
```

Points have no index buffer at all — one draw call per point, no adjacency.

---

## Section 8 — The Instance Storage Buffer

The tutorial draws one object per pipeline setup. Session stores **all**
per-object data in a storage buffer, so a single bind group serves
every object of every topology.

### InstanceData — 96 bytes per object

```
Instance buffer (STORAGE, binding 1):

offset  0  ┌──────────────────────────────────────────────────┐
           │  model  [[f32;4];4]   64 bytes                   │
           │  The object's 4×4 transform matrix (world space) │
           │  Xform: rotation, scale, translation combined     │
offset 64  ├──────────────────────────────────────────────────┤
           │  color  [f32; 4]      16 bytes                   │
           │  Per-object RGBA tint in 0..=1.0                 │
offset 80  ├──────────────────────────────────────────────────┤
           │  object_id  u32        4 bytes                   │
           │  Mirrors instance_id — readable by fragment       │
           │  shader for GPU picking                          │
offset 84  ├──────────────────────────────────────────────────┤
           │  flags  u32            4 bytes                   │
           │  bit 0 = selected (highlight colour)             │
           │  bit 1 = hovered   (cursor-over highlight)       │
           │  bit 2 = hidden    (skip draw)                   │
offset 88  ├──────────────────────────────────────────────────┤
           │  _pad  [u32; 2]        8 bytes                   │
           │  Padding to 16-byte alignment (WebGPU requirement│
           │  for storage buffer struct alignment)            │
offset 96  └──────────────────────────────────────────────────┘

instance[0] = mesh "abc" (triangle)
instance[1] = polyline "def"
instance[2] = point cloud "ghi"
...
```

### The WGSL side — reading from the storage buffer

```wgsl
// mesh.wgsl
struct Instance {
    model:     mat4x4<f32>,  // 64 B
    tint:      vec4<f32>,    // 16 B
    object_id: u32,          //  4 B
    flags:     u32,          //  4 B
    _pad0:     u32,          //  4 B
    _pad1:     u32,          //  4 B
}

@group(0) @binding(1) var<storage, read> instances: array<Instance>;

@vertex
fn vs_main(in: VsIn, @builtin(instance_index) iid: u32) -> VsOut {
    let inst  = instances[iid];           // look up this object's data
    let world = inst.model * vec4<f32>(in.position, 1.0);
    // ...
    out.color = in.color * inst.tint;    // vertex colour × per-object tint
}
```

`@builtin(instance_index)` is the `iid` in the `instance_range` passed to
`draw_indexed(index_range, base_vertex, instance_id..(instance_id+1))`.

### What this means for the draw loop

```
Without instance buffer:          With instance buffer:
  1 draw call per object             1 draw call per object
  upload model matrix per call       instance buffer already on GPU
  upload colour per call             instance buffer already on GPU
  re-bind pipeline per type          one bind group for all types

  100 objects × 3 pipeline swaps     100 draws, 1 bind group
  + 100 uniform uploads per frame    + ~0 CPU work per frame
  = heavy CPU-GPU sync               = very light CPU work
```

Updating one object's colour is just `queue.write_buffer` on 96 bytes
at the right slot offset — no VBO re-upload, no draw call changes:

```rust
// gpu_session.rs
pub fn update_color(&mut self, guid: &str, color: [f32; 4], queue: &wgpu::Queue) -> bool {
    let id = self.pick.instance_id(guid)?;
    self.instances_cpu[id as usize].color = color;
    let offset = id as u64 * size_of::<InstanceData>() as u64;
    queue.write_buffer(&self.instance_buffer, offset, bytemuck::bytes_of(&self.instances_cpu[id as usize]));
    true
}
```

---

## Section 9 — The Camera Uniform Buffer

The camera buffer is the single piece of data that changes every frame.

```
CPU (every frame):                    GPU (vertex shader):

Camera::view_proj()                   @group(0) @binding(0)
    │                                 var<uniform> camera: Camera;
    │  returns [[f32;4];4]
    │  = proj × (view × scale(0.001)) fn vs_main(in: VsIn, ..) -> VsOut {
    │                                     let clip = camera.view_proj
    └──► CameraUniform { view_proj }              * vec4(in.position, 1.0);
                │                      }
                ▼
    queue.write_buffer(
        &camera_buf,    ← which buffer
        0,              ← byte offset (overwrite from start)
        bytemuck::bytes_of(&cam),  ← 64 bytes
    )
```

```rust
// lib.rs — update() called every frame
pub fn update(&mut self) {
    self.controller.update_camera(&mut self.camera);
    let cam = CameraUniform { view_proj: self.camera.view_proj() };
    self.queue.write_buffer(&self.camera_buf, 0, bytemuck::bytes_of(&cam));
}
```

The 64-byte write is the *only* per-frame CPU→GPU upload for the camera.
Vertex data, instance data — those sit unchanged on the GPU until something
in the scene actually changes.

### CameraUniform struct

```rust
// pipelines.rs
#[repr(C, align(16))]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],   // 64 bytes — column-major 4×4 f32 matrix
}
```

`align(16)` matches the WebGPU uniform buffer alignment requirement.
Without it some GPU drivers misread the matrix boundary.

---

## Section 10 — Bind Groups and Bind Group Layouts

A bind group is the **glue** between a CPU buffer and a shader `@group(N) @binding(M)`.

```
CPU side                           GPU shader side
─────────────────────────────      ─────────────────────────────────────
wgpu::Buffer (camera_buf)    ──►   @group(0) @binding(0) var<uniform> camera
wgpu::Buffer (instance_buf)  ──►   @group(0) @binding(1) var<storage, read> instances
```

The layout describes *what kind* of thing is at each slot.
The bind group binds *actual buffers* to those slots.

### Layout — declared once, shared across all four pipelines

```rust
// pipelines.rs
fn build_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding:    0,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    ..
                },
                ..
            },
            wgpu::BindGroupLayoutEntry {
                binding:    1,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    ..
                },
                ..
            },
        ],
    })
}
```

### Bind group — created once, reused every frame

```rust
// pipelines.rs
pub fn build_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    camera_buffer:   &wgpu::Buffer,
    instance_buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: camera_buffer.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: instance_buffer.as_entire_binding() },
        ],
    })
}
```

This one bind group is set once per render pass and serves all four pipelines:

```rust
// lib.rs — render()
render_pass.set_bind_group(0, &self.bind_group, &[]);
// Now every subsequent pipeline reads the same camera + instances
render_pass.set_pipeline(&self.pipelines.grid);
render_pass.draw(0..298, 0..1);
render_pass.set_pipeline(&self.pipelines.mesh);
gpu_session.draw_meshes(&mut render_pass);
// etc.
```

---

## Section 11 — Render Pipelines

A pipeline is the full description of one drawing mode: which shaders,
which vertex layout, which topology, how to blend colours, how to write
depth. Session has four:

```
┌────────────────────────────────────────────────────────────────────┐
│ MESH pipeline                                                      │
│   topology:        TriangleList                                    │
│   vertex layout:   MeshVertex (28 B: position+normal+color)        │
│   shader:          mesh.wgsl  (Lambert lighting + back-face red)   │
│   depth_write:     true   depth_compare: Less                      │
│   cull_mode:       None   (back-face handled in shader)            │
└────────────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────────────┐
│ LINE pipeline                                                      │
│   topology:        LineList                                        │
│   vertex layout:   LineVertex (16 B: position+color)               │
│   shader:          line.wgsl  (pass-through colour)                │
│   depth_write:     true   depth_compare: Less                      │
└────────────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────────────┐
│ POINT pipeline                                                     │
│   topology:        PointList                                       │
│   vertex layout:   PointVertex (16 B: same as LineVertex)          │
│   shader:          line.wgsl  (reused — same pass-through)         │
│   depth_write:     true   depth_compare: Less                      │
└────────────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────────────┐
│ GRID pipeline    ← special: no vertex buffer at all                │
│   topology:        LineList                                        │
│   vertex layout:   none — positions from @builtin(vertex_index)    │
│   shader:          grid.wgsl  (procedural positions)               │
│   depth_write:     false  depth_compare: Always                    │
│   drawn FIRST: always renders behind geometry (zero z-fighting)    │
└────────────────────────────────────────────────────────────────────┘
```

All four share one `build_pipeline` helper for mesh/line/point.
The grid has its own `build_grid_pipeline` with different depth settings.

```rust
// pipelines.rs — generic pipeline builder
fn build_pipeline(
    device: &wgpu::Device,
    label:         &str,
    wgsl:          &str,
    vertex_layout: wgpu::VertexBufferLayout<'static>,
    topology:      wgpu::PrimitiveTopology,
    color_format:  wgpu::TextureFormat,
    depth_format:  Option<wgpu::TextureFormat>,
    bgl:           &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    // ...
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        vertex: wgpu::VertexState {
            buffers: &[vertex_layout],   // ← how to read the VBO
            ..
        },
        primitive: wgpu::PrimitiveState {
            topology,                    // ← TriangleList / LineList / PointList
            front_face: wgpu::FrontFace::Ccw,  // counter-clockwise = front
            cull_mode: None,             // back faces NOT culled (handled in shader)
            ..
        },
        depth_stencil: depth_format.map(|fmt| wgpu::DepthStencilState {
            depth_write_enabled: Some(true),
            depth_compare: Some(wgpu::CompareFunction::Less),
            ..
        }),
        ..
    })
}
```

### Depth buffer — why Less and why geometry doesn't fight with the grid

```
Frame start: depth buffer cleared to 1.0  (furthest possible)

Grid drawn first with depth_compare = Always, depth_write = false:
  Every grid pixel passes (Always) and writes its colour.
  Depth buffer UNCHANGED — still all 1.0.

Meshes/lines/points drawn with depth_compare = Less, depth_write = true:
  A geometry pixel at depth 0.3 < 1.0 → passes → writes colour AND depth 0.3.
  Another geometry pixel at depth 0.7 vs stored 0.3 → 0.7 > 0.3 → FAILS → discarded.

Result:
  Grid is always behind geometry.
  Geometry occludes other geometry correctly.
  No z-fighting anywhere.
```

---

## Section 12 — Shaders (WGSL)

WGSL (WebGPU Shading Language) is the only shader language that runs
natively in the browser. The `naga` library (inside wgpu) cross-compiles it
to SPIR-V on native, MSL on Metal, HLSL on DX12.

### mesh.wgsl — Lambert shading + back-face detection

```wgsl
// Structs must match the Rust InstanceData layout exactly.
struct Camera   { view_proj: mat4x4<f32>, }
struct Instance {
    model: mat4x4<f32>, tint: vec4<f32>,
    object_id: u32, flags: u32, _pad0: u32, _pad1: u32,
}

@group(0) @binding(0) var<uniform> camera: Camera;
@group(0) @binding(1) var<storage, read> instances: array<Instance>;

// Vertex shader inputs — must match MeshVertex layout()
struct VsIn {
    @location(0) position: vec3<f32>,
    @location(1) normal:   vec3<f32>,
    @location(2) color:    vec4<f32>,   // Unorm8x4 → auto-converted to 0..1
}

@vertex
fn vs_main(in: VsIn, @builtin(instance_index) iid: u32) -> VsOut {
    let inst  = instances[iid];
    let world = inst.model * vec4<f32>(in.position, 1.0);  // model transform
    // camera.view_proj already contains MM_TO_UNIT × view × projection
    out.clip_pos = camera.view_proj * world;
    out.normal   = normalize((inst.model * vec4<f32>(in.normal, 0.0)).xyz);
    out.color    = in.color * inst.tint;
    return out;
}

@fragment
fn fs_main(in: VsOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    if !front {
        return vec4<f32>(0.8, 0.1, 0.1, in.color.a);  // back face = red
    }
    let light_dir = normalize(vec3<f32>(0.4, 0.8, 0.5));
    let n_dot_l   = max(dot(in.normal, light_dir), 0.0);
    let ambient   = 0.25;
    let lit       = ambient + (1.0 - ambient) * n_dot_l;
    return vec4<f32>(in.color.rgb * lit, in.color.a);
}
```

`@builtin(front_facing)` is a free boolean the GPU provides — no extra
data is needed. Front-facing means the triangle's vertices arrive
counter-clockwise from the camera's perspective.

### grid.wgsl — fully procedural, no VBO

```wgsl
const STEP:        f32 = 1000.0;  // mm per grid cell (= 1 m after MM_TO_UNIT)
const N_HALF:      u32 = 36u;     // grid lines per side
const GRID_HALF_F: f32 = 36000.0; // = N_HALF × STEP
const N_GREY:      u32 = 144u;    // N_HALF × 2 (lines/direction) × 2 (verts/line)

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    var wp: vec3<f32>;
    var c:  vec3<f32>;

    if vid < N_GREY {
        // Block A: X-parallel grey lines (Y varies, skip Y=0)
        let line_idx = vid / 2u;
        let ep       = vid % 2u;                             // endpoint 0 or 1
        let y_raw    = (f32(line_idx) - f32(N_HALF)) * STEP; // −36000..35000
        let y        = y_raw + select(0.0, STEP, y_raw >= 0.0); // skip 0
        let x        = select(-GRID_HALF_F, GRID_HALF_F, ep == 1u);
        wp = vec3<f32>(x, y, 0.0);
        c  = COLOR_GREY;
    } else if vid < N_GREY * 2u {
        // Block B: Y-parallel grey lines (X varies, skip X=0)
        // ...symmetric...
    } else {
        // Block C: coloured axis segments
        // X axis: grey (neg) + red (pos)
        // Y axis: grey (neg) + green (pos)
        // Z axis: blue (pos only)
    }

    var out: VsOut;
    out.color    = vec4<f32>(c, 1.0);
    out.clip_pos = camera.view_proj * vec4<f32>(wp, 1.0);
    return out;
}
```

The vertex index arithmetic maps a flat integer 0..298 to a 3D world point.
No buffer read at all — the GPU computes the position from the index alone.

---

## Section 13 — The Render Loop

Every animation frame runs this sequence inside `render()` in `lib.rs`:

```
Frame start
    │
    ▼
 1. get_current_texture()
       │
       │ ← surface hands us a SurfaceTexture to draw into
       ▼
 2. create TextureView from the surface texture
       │ (TextureView = handle that tells the render pass where to write pixels)
       ▼
 3. create CommandEncoder
       │ (records GPU commands into a command buffer; nothing runs yet)
       ▼
 4. begin_render_pass()
    │   ├─ color_attachments → [our TextureView]  (clear to grey)
    │   └─ depth_stencil     → [depth_view]        (clear to 1.0)
    │
    │ Inside the render pass:
    ├─ set_bind_group(0, &bind_group)        ← binds camera + instances
    │
    ├─ set_pipeline(grid)
    │  draw(0..298, 0..1)                   ← 298 verts, no VBO, depth=Always
    │
    ├─ set_pipeline(mesh)
    │  for each mesh slot:
    │    draw_indexed(index_range, base_vertex, instance_id..id+1)
    │
    ├─ set_pipeline(line)
    │  for each line slot:
    │    draw_indexed(...)  or  draw(...)
    │
    └─ set_pipeline(point)
       for each point slot:
         draw(vertex_range, instance_id..id+1)
    │
    ▼
 5. encoder.finish()   ← seals the command buffer
    │
    ▼
 6. queue.submit(once(encoder.finish()))
       │ ← sends the whole buffer to the GPU in one syscall
       │ ← GPU executes asynchronously; CPU continues
       ▼
 7. surface_texture.present()
       │ ← schedules the frame for display after GPU finishes
       ▼
Frame end
```

The `{}` scope around the render pass forces `render_pass` to drop before
`encoder.finish()`. This is required because `begin_render_pass` borrows
`encoder` mutably — the borrow must end before `encoder.finish()` is callable.

```rust
// lib.rs
let mut encoder = self.device.create_command_encoder(..);
{
    let mut render_pass = encoder.begin_render_pass(..);
    // ... all draw calls ...
}   // ← render_pass dropped here, mutable borrow ends
self.queue.submit(iter::once(encoder.finish()));
output.present();
```

---

## Section 14 — Camera: Projection, Named Views, MM Scale

### The MM_TO_UNIT scale

Session geometry is in **millimetres**. The viewer works in **metres**.
Rather than scaling every object individually, the scale is baked into
the camera's view-projection matrix:

```rust
// camera.rs
const MM_TO_UNIT: f32 = 0.001;

pub fn view_proj(&self) -> [[f32; 4]; 4] {
    let view  = Xform::look_at_right_handed(&eye, &tgt, &up);
    let scale = Xform::scale_xyz(MM_TO_UNIT, MM_TO_UNIT, MM_TO_UNIT);

    let view_scaled = &view * &scale;   // scale IS the first transform applied
    (&proj * &view_scaled).to_cols()
}
```

```
Vertex at 1500 mm in session geometry
         │
         │  view_scaled = view × scale(0.001)
         ▼
1500 × 0.001 = 1.5 viewer units = 1.5 m
         │
         │  proj (perspective or ortho)
         ▼
Clip space position (−1..+1 in each axis)
```

Everything goes through the same camera uniform — meshes, lines, points,
and the grid. No per-object scaling is needed.

The grid uses mm positions (`STEP=1000`, `GRID_HALF_F=36000`) and ends
up at correct world-scale positions because it goes through the same matrix.

### Depth buffer precision

```
Depth buffer: 24-bit integer → ~16 million distinct depth values.
Those 16M values are spread across [near, far].

PERSPECTIVE (non-linear / logarithmic distribution):
   ┌─────────────────────────────────────────────────────┐
   │  near                                          far  │
   │  ██████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ │
   │  (most bits clustered near the camera)              │
   └─────────────────────────────────────────────────────┘
   Problem: far/near ratio drives precision.
     near=0.001, far=100000 → ratio = 100,000,000 → only 4 bits remain at 10m!
     Adaptive near = distance × 0.001:
       at distance=3 → near=0.003, far/near ≈ 33,000,000 (too high)
     Actually: near = distance × 0.001, far = 100,000
       at distance=3 → near=0.003 → ratio=33M (WebGPU uses reversed-Z, usually ok)
       at distance=1000 → near=1 → ratio=100,000 ✓ (acceptable precision)
     Rule: keep far/near ≤ ~100,000. Adaptive near achieves this at any zoom.

ORTHO (linear / uniform distribution):
   ┌─────────────────────────────────────────────────────┐
   │  near                                          far  │
   │  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ │
   │  (bits spread evenly — no compression at near end)  │
   └─────────────────────────────────────────────────────┘
   Ratio irrelevant! near can even be NEGATIVE.
   Negative near is required: when looking from the right, objects on
   the left are BEHIND the camera. Positive near would clip them.
   Solution: near = −100,000, far = +100,000. Grid (±36,000 mm = ±36 units)
   is safely inside this range.
```

```rust
// camera.rs
ProjMode::Perspective => {
    let near = (self.distance * 0.001_f32).max(0.0001_f32);
    Xform::perspective(PI / 3.0, self.aspect, near, 100_000.0)
}
ProjMode::Ortho => {
    let h = self.ortho_scale;
    let w = h * self.aspect;
    Xform::orthographic(-w, w, -h, h, -100_000.0, 100_000.0)
}
```

### Named views — quaternion math

The camera's default offset from target is `[0, -distance, 0]` (south).
A named view rotates this offset to align with the desired direction.

```
Top view (key T):
  We want camera above target, looking down (-Z direction from camera).
  Rotate [0,−1,0] → [0,0,+1] = rotate 90° clockwise around X axis
  = from_axis_angle([1,0,0], -π/2)

  camera.last_right = [1,0,0]  ← preset to prevent gimbal flip
  (when fwd ≈ world_up, the cross-product to compute right is near-zero)
```

```
           Before (default):          After (T):

            camera                     target
             ●                           ●
              \  fwd                     │  fwd (-Z down)
               ↘                        ↓
              target                    camera
               ●                         ●
```

```rust
// camera.rs
pub fn set_named_view(&mut self, view: NamedView) {
    let half_pi = Tolerance::PI / 2.0;
    match view {
        NamedView::Top    => {
            self.orientation = Quaternion::from_axis_angle(Vector::new(1.0,0.0,0.0), -half_pi);
            self.last_right = [1.0, 0.0, 0.0];  // prevent gimbal singularity
        }
        NamedView::Bottom => {
            self.orientation = Quaternion::from_axis_angle(Vector::new(1.0,0.0,0.0),  half_pi);
            self.last_right = [1.0, 0.0, 0.0];
        }
        NamedView::Right  => {
            self.orientation = Quaternion::from_axis_angle(Vector::new(0.0,0.0,1.0),  half_pi);
        }
        NamedView::Left   => {
            self.orientation = Quaternion::from_axis_angle(Vector::new(0.0,0.0,1.0), -half_pi);
        }
    }
    self.proj_mode   = ProjMode::Ortho;      // ← always switch to ortho
    self.ortho_scale = self.distance;         // ← match apparent size
    self.update_position();
}
```

Switching perspective → ortho sets `ortho_scale = distance` so geometry
appears the same size — the frustum height matches the perspective FOV height
at that distance.

---

## Section 15 — The Grid — Procedural LineList

The grid has no vertex buffer. 298 vertices are generated purely from
`@builtin(vertex_index)` inside the vertex shader.

```
298 vertices split into three blocks:

Block A: X-parallel grey lines (vid 0..143)
  72 lines × 2 endpoints = 144 vertices
  Lines at Y ∈ {−36000,−35000,...,−1000, 1000,...,36000} mm
  (Y=0 skipped — drawn as the coloured Y axis instead)
  Each line: (−36000, Y, 0) → (36000, Y, 0)

Block B: Y-parallel grey lines (vid 144..287)
  72 lines × 2 endpoints = 144 vertices
  Lines at X ∈ {−36000,...,−1000, 1000,...,36000} mm
  Each line: (X, −36000, 0) → (X, 36000, 0)

Block C: axis segments (vid 288..297)
  10 vertices total:
  X axis negative: (−36000, 0, 0) → (0, 0, 0)   GREY   vid 288,289
  X axis positive: (0, 0, 0) → (36000, 0, 0)     RED    vid 290,291
  Y axis negative: (0, −36000, 0) → (0, 0, 0)    GREY   vid 292,293
  Y axis positive: (0, 0, 0) → (0, 36000, 0)     GREEN  vid 294,295
  Z axis positive: (0, 0, 0) → (0, 0, 36000)     BLUE   vid 296,297
```

The arithmetic for Block A:

```wgsl
const STEP:        f32 = 1000.0;
const N_HALF:      u32 = 36u;
const N_GREY:      u32 = 144u;

// vid 0..143: X-parallel lines
let line_idx = vid / 2u;            // 0..71: which line
let ep       = vid % 2u;            // 0=left endpoint, 1=right endpoint

// y_raw:  line 0 → −36000, line 1 → −35000, ..., line 35 → −1000,
//         line 36 → 0, line 37 → 1000, ..., line 71 → 35000
let y_raw = (f32(line_idx) - f32(N_HALF)) * STEP;

// Skip y=0: if y_raw < 0, keep it; if y_raw >= 0, add STEP (1000) to jump over 0
let y = y_raw + select(0.0, STEP, y_raw >= 0.0);

// Endpoint: ep=0 → left (−HALF), ep=1 → right (+HALF)
let x = select(-GRID_HALF_F, GRID_HALF_F, ep == 1u);
```

The render call (`lib.rs`):

```rust
render_pass.set_pipeline(&self.pipelines.grid);
render_pass.draw(0..298, 0..1);
//               ^^^^^^  vertex range passed to vs_main as vertex_index
//                       0..1 = one instance (instance_index = 0, unused by grid)
```

There is no `set_vertex_buffer` before this draw — the pipeline has
`buffers: &[]` so wgpu does not expect one.

---

## Section 16 — Camera Controls

```
Input               Action
─────────────────   ─────────────────────────────────────────────────
Right-drag          Orbit (Z-up turntable)
Shift+Right-drag    Pan (move target + position together)
Scroll              Zoom: perspective → distance, ortho → ortho_scale
WASD / ↑↓←→        Keyboard pan (speed proportional to distance)
C / F               Reset to initial perspective view

P                   Perspective projection
O                   Ortho (ortho_scale = current distance)
T                   Top view + ortho
B                   Bottom view + ortho
L                   Left view + ortho
R                   Right view + ortho
```

### Orbit — Z-up turntable

The camera orbits by rotating its quaternion orientation, not by computing
Euler angles. This avoids gimbal lock entirely.

```
Mouse drag (dx, dy):
  yaw   = −dx × orbit_speed          // rotation around world Z axis
  pitch = −dy × orbit_speed          // rotation around current right axis

  yaw_q   = Quaternion::from_axis_angle(world_up, yaw)
  pitch_q = Quaternion::from_axis_angle(last_right, pitch)

  orientation = normalize(yaw_q × pitch_q × old_orientation)
```

`last_right` is tracked every frame from the camera's forward direction.
When forward ≈ world_up (top/bottom view), the cross-product becomes near-zero
and is replaced by the preset `[1,0,0]` to keep orbit stable at the poles.

### Ortho zoom

Ortho mode does NOT move the camera closer — that would change parallax
(perspective effect) but ortho has none. Instead, the frustum half-height
`ortho_scale` shrinks or grows:

```
ortho_scale large:              ortho_scale small:
  ┌──────────────────────┐         ┌──────┐
  │                      │         │      │
  │    (scene appears    │         │      │ (scene appears zoomed in,
  │       small)         │         │      │  just fewer world units
  │                      │         │      │  fit on screen)
  └──────────────────────┘         └──────┘
```

```rust
ProjMode::Ortho => {
    camera.ortho_scale = (camera.ortho_scale * factor).clamp(0.001, 100_000.0);
    // do NOT move camera.distance — that would change parallax
}
```

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        BROWSER (WASM)                           │
│                                                                 │
│   winit Window                                                  │
│       │                                                         │
│       │  events (mouse, keyboard, resize)                       │
│       ▼                                                         │
│   CameraController ──► Camera ──► view_proj matrix             │
│                                        │                        │
│                                        │ MM_TO_UNIT × 0.001     │
│                                        ▼                        │
│                              CameraUniform (64 B uniform buf)   │
│                                        │                        │
│   Session (CPU) ──► GpuSession ────────┤                        │
│                          │             │                        │
│                     ┌────┴────┐        │                        │
│                     │GpuArena │        │                        │
│                     │  .vbo   │        │ @group(0)              │
│                     │  .ibo   │        │   binding 0 = camera   │
│                     └────┬────┘        │   binding 1 = instances│
│                          │             │                        │
│                     InstanceData       │                        │
│                     storage buffer─────┘                        │
│                          │                                      │
│                          ▼                                      │
│                     Render Pass (wgpu)                          │
│                          │                                      │
│                 ┌────────┼────────┬──────────┐                  │
│                 │        │        │          │                  │
│              grid     mesh      line      point                 │
│             pipeline pipeline pipeline  pipeline                │
│           LineList  TriList  LineList  PointList                 │
│           no VBO    IBO+VBO  IBO+VBO   VBO only                 │
│           depth=    depth=   depth=    depth=                   │
│           Always    Less     Less      Less                     │
│                 │        │        │          │                  │
│                 ▼        ▼        ▼          ▼                  │
│                        Screen                                   │
└─────────────────────────────────────────────────────────────────┘
```

---

## Dependencies

```
cargo install trunk
rustup target add wasm32-unknown-unknown
```

| Crate | Role |
|---|---|
| wgpu | WebGPU graphics API (compiles to WebGL2 in browser) |
| winit | Window + event loop |
| wasm-bindgen | Rust ↔ JavaScript bridge |
| bytemuck | Safe cast between `&T` and `&[u8]` for GPU uploads |
| session_rust | Geometry kernel (mm units) + GpuSession + GpuArena |

---

## Reference Tutorials

- https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/
- https://sotrh.github.io/learn-wgpu/beginner/tutorial4-buffer/#what-is-a-buffer
- https://github.com/sotrh/learn-wgpu/tree/master/code/beginner/tutorial4-buffer
