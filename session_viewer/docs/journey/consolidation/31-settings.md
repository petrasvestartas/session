# 3.6 · Shared GPU geometry and releasable source ownership

**Goal:** Verify whole-origin protection, independent duplicate imports and checked release epochs. · **Steps:** 23 · **Source:** `src/app/decode.rs,src/app/loader.rs,src/state.rs`

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 3.6 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/gpu_mesh.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/triangle.wgsl</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/gpu_geometry.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">23 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/decode.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/loader.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Keep selection out of the…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Placed Kernel Mesh (.pb)</text>
  <rect x="598" y="62" width="66" height="56" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="631" y="88" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">.pb</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="631" y="104" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">Decode</text>
  <line x1="664" y1="90" x2="704" y2="90" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <polygon points="716,128 760,54 804,128" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <line x1="716" y1="128" x2="760" y2="100" stroke="#111111" stroke-width="1.25" stroke-dasharray="3 3"/>
  <line x1="804" y1="128" x2="760" y2="100" stroke="#111111" stroke-width="1.25" stroke-dasharray="3 3"/>
  <line x1="760" y1="54" x2="760" y2="100" stroke="#111111" stroke-width="1.25" stroke-dasharray="3 3"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Shared GPU geometry and rel…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Incremental GPU Sync</text>
  <rect x="876" y="52" width="196" height="40" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="70" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">sync_scene = false</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="84" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Orbit: 64B Camera Only</text>
  <rect x="876" y="102" width="196" height="40" rx="5" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="120" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">sync_scene = true</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="134" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Edit: Upload Dirty Buffers</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Protect history and futur…</text>
</svg>
</div>

## <a id="checkpoint-31-settings"></a>1. Keep selection out of the vertex data

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/gpu_mesh.rs, src/triang…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Object placement and sele…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">eighty uniform bytes</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">vertex shader; source col…</text>
</svg>
</div>

**What this code does:** Put placement and selection in an explicitly packed object uniform.

### 1. `src/gpu_mesh.rs`

Upload original vertex colours regardless of which object is selected. Use one encoder for the complete object uniform. Encode the matrix, selection and padding at their shader offsets.

**Locate this existing block:**

```rust
}

impl GpuMesh {
    pub fn upload(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        selected: bool,
    ) -> Self {
        let mesh = &object.mesh;
        let mut bytes = Vec::with_capacity(mesh.vertices().len() * 24);
        for vertex in mesh.vertices() {
            let mut display = *vertex;
            if selected {
                display[3..].copy_from_slice(&[1.0, 0.75, 0.05]);
            }
            for value in display {
                bytes.extend_from_slice(&value.to_ne_bytes());
            }
        }
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mesh vertices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let bytes: Vec<u8> = mesh
            .indices()
            .iter()
            .flat_map(|index| index.to_ne_bytes())
            .collect();
        let indices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mesh indices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::INDEX,
        });
        let bytes: Vec<u8> = object
            .model
            .m
            .iter()
            .flat_map(|value| (*value as f32).to_ne_bytes())
            .collect();
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

fn settings(object: &Object, selected: bool) -> [u8; 80] {
    let mut bytes = [0; 80];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    let flag: f32 = if selected {
        1.0
    } else {
        0.0
    };
    bytes[64..68].copy_from_slice(&flag.to_ne_bytes());
    bytes
}

impl GpuMesh {
    pub fn upload(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        selected: bool,
    ) -> Self {
        let mesh = &object.mesh;
        let mut bytes = Vec::with_capacity(mesh.vertices().len() * 24);
        for vertex in mesh.vertices() {
            for value in vertex {
                bytes.extend_from_slice(&value.to_ne_bytes());
            }
        }
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mesh vertices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let bytes: Vec<u8> = mesh
            .indices()
            .iter()
            .flat_map(|index| index.to_ne_bytes())
            .collect();
        let indices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mesh indices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::INDEX,
        });
        let bytes = settings(object, selected);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
```

### 2. `src/triangle.wgsl`

Describe the same eighty-byte object layout in WGSL. Read placement from the object settings structure. Choose the displayed colour per object without rewriting vertices.

**Locate this existing block:**

```wgsl
@group(0) @binding(0) var<uniform> transform: mat4x4<f32>;
@group(1) @binding(0) var<uniform> model: mat4x4<f32>;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
    @location(1) world: vec3<f32>,
}

@vertex
fn vertex(@location(0) position: vec3<f32>, @location(1) colour: vec3<f32>) -> VertexOutput {
    var output: VertexOutput;
    let world = model * vec4<f32>(position, 1.0);
    output.position = transform * world;
    output.colour = colour;
    output.world = world.xyz;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(0) @binding(0) var<uniform> transform: mat4x4<f32>;
struct ObjectSettings {
    model: mat4x4<f32>,
    selection: vec4<f32>,
}
@group(1) @binding(0) var<uniform> object: ObjectSettings;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
    @location(1) world: vec3<f32>,
}

@vertex
fn vertex(@location(0) position: vec3<f32>, @location(1) colour: vec3<f32>) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = transform * world;
    output.colour = select(colour, vec3<f32>(1.0, 0.75, 0.05), object.selection.x > 0.5);
    output.world = world.xyz;
```

## <a id="checkpoint-31a-geometry"></a>2. Give immutable GPU geometry one owner

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/gpu_geometry.rs, src/li…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">CPU display Rc</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">object row with its own u…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">draw</text>
</svg>
</div>

**What this code does:** Separate vertex/index storage from each object’s uniform and retain its CPU source.

### 1. `src/gpu_geometry.rs`

Create an owner for immutable buffers and the CPU display allocation that produced them.

Create the file and type:

```rust
use crate::mesh::Mesh;
use std::rc::Rc;
use wgpu::util::DeviceExt;

pub struct GpuGeometry {
    pub source: Rc<Mesh>,
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    index_count: u32,
}

impl GpuGeometry {
    pub fn upload(device: &wgpu::Device, source: Rc<Mesh>) -> Self {
        let bytes: Vec<u8> = source
            .vertices()
            .iter()
            .flatten()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shared mesh vertices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let bytes: Vec<u8> = source
            .indices()
            .iter()
            .flat_map(|index| index.to_ne_bytes())
            .collect();
        let indices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shared mesh indices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::INDEX,
        });
        let index_count = source.indices().len() as u32;
        Self {
            source,
            vertices,
            indices,
            index_count,
        }
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.set_index_buffer(
            self.indices.slice(..),
            wgpu::IndexFormat::Uint16,
        );
        pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}
```

### 2. `src/lib.rs`

Make geometry storage available to the draw row.

**Locate this existing block:**

```rust
mod gesture_tests;
pub mod gpu_mesh;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod gesture_tests;
pub mod gpu_geometry;
pub mod gpu_mesh;
```

### 3. `src/gpu_mesh.rs`

Use a shared geometry owner. Keep only the geometry owner beside the object bind group. Accept an existing geometry owner while keeping object settings independent. Return one draw row without copying its shared geometry. Delegate immutable bindings and indexed drawing to the geometry owner.

**Locate this existing block:**

```rust
use crate::scene::Object;
use wgpu::util::DeviceExt;

pub struct GpuMesh {
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    index_count: u32,
    model_group: wgpu::BindGroup,
}

fn settings(object: &Object, selected: bool) -> [u8; 80] {
    let mut bytes = [0; 80];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    let flag: f32 = if selected {
        1.0
    } else {
        0.0
    };
    bytes[64..68].copy_from_slice(&flag.to_ne_bytes());
    bytes
}

impl GpuMesh {
    pub fn upload(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        selected: bool,
    ) -> Self {
        let mesh = &object.mesh;
        let mut bytes = Vec::with_capacity(mesh.vertices().len() * 24);
        for vertex in mesh.vertices() {
            for value in vertex {
                bytes.extend_from_slice(&value.to_ne_bytes());
            }
        }
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mesh vertices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let bytes: Vec<u8> = mesh
            .indices()
            .iter()
            .flat_map(|index| index.to_ne_bytes())
            .collect();
        let indices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mesh indices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::INDEX,
        });
        let bytes = settings(object, selected);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let model_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("object placement"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: model.as_entire_binding(),
            }],
        });
        Self {
            vertices,
            indices,
            model_group,
            index_count: mesh.indices().len() as u32,
        }
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_bind_group(1, &self.model_group, &[]);
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.set_index_buffer(
            self.indices.slice(..),
            wgpu::IndexFormat::Uint16,
        );
        pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{gpu_geometry::GpuGeometry, scene::Object};
use std::rc::Rc;
use wgpu::util::DeviceExt;

pub struct GpuMesh {
    pub geometry: Rc<GpuGeometry>,
    model_group: wgpu::BindGroup,
}

fn settings(object: &Object, selected: bool) -> [u8; 80] {
    let mut bytes = [0; 80];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    let flag: f32 = if selected {
        1.0
    } else {
        0.0
    };
    bytes[64..68].copy_from_slice(&flag.to_ne_bytes());
    bytes
}

impl GpuMesh {
    pub fn upload(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        selected: bool,
    ) -> Self {
        let geometry = Rc::new(GpuGeometry::upload(
            device,
            Rc::clone(&object.mesh),
        ));
        Self::with_geometry(device, layout, object, selected, geometry)
    }

    pub fn with_geometry(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        selected: bool,
        geometry: Rc<GpuGeometry>,
    ) -> Self {
        let bytes = settings(object, selected);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let model_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("object placement"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: model.as_entire_binding(),
            }],
        });
        Self {
            geometry,
            model_group,
        }
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_bind_group(1, &self.model_group, &[]);
        self.geometry.draw(pass);
    }
```

## <a id="checkpoint-31b-cache"></a>3. Reuse uploads while their geometry is alive

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/geometry_cache.rs, src/…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Scene row</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">weak cache lookup</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">object settings</text>
</svg>
</div>

**What this code does:** Cache GPU geometry by its retained CPU allocation and release dead cache entries.

### 1. `src/geometry_cache.rs`

Keep weak cache entries and cumulative upload evidence without pinning unused GPU buffers.

Create the file and type:

```rust
use crate::{gpu_geometry::GpuGeometry, mesh::Mesh};
use std::{
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Default)]
pub struct GeometryCache {
    entries: HashMap<*const Mesh, Weak<GpuGeometry>>,
    pub uploads: usize,
}

impl GeometryCache {
    pub fn get(&mut self, device: &wgpu::Device, source: &Rc<Mesh>) -> Rc<GpuGeometry> {
        let key = Rc::as_ptr(source);
        if let Some(geometry) = self
            .entries
            .get(&key)
            .and_then(Weak::upgrade)
        {
            return geometry;
        }
        let geometry = Rc::new(GpuGeometry::upload(
            device,
            Rc::clone(source),
        ));
        self.entries
            .insert(key, Rc::downgrade(&geometry));
        self.uploads += 1;
        geometry
    }

    pub fn prune(&mut self) {
        self.entries
            .retain(|_, geometry| geometry.strong_count() > 0);
    }

    pub fn entries(&self) -> usize {
        self.entries.len()
    }
}
```

### 2. `src/lib.rs`

Expose the geometry cache to the renderer.

**Locate this existing block:**

```rust
mod fit_tests;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod fit_tests;
pub mod geometry_cache;
#[cfg(test)]
```

### 3. `src/renderer.rs`

Keep geometry reuse and object allocation counts with the renderer. Let the same scene synchronization populate startup rows. Initialize startup through the same ownership path as later edits. Reuse live geometry before replacing the old GPU rows, then prune dead weak entries. Report cumulative geometry uploads, settings allocations and the reserved write counters.

**Locate this existing block:**

```rust
    meshes: Vec<GpuMesh>,
    uniform: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    depth: wgpu::TextureView,
}

impl Renderer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 24,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let layout = pipeline.get_bind_group_layout(1);
        let meshes = scene
            .objects()
            .iter()
            .map(|object| GpuMesh::upload(&device, &layout, object, false))
            .collect();
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform"),
            size: 64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view settings"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        let depth = Self::depth(&device, 640, 480);
        Self {
            device,
            queue,
            pipeline,
            meshes,
            uniform,
            view_group,
            depth,
        }
    }

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        self.depth = Self::depth(&self.device, size.width, size.height);
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        let layout = self.pipeline.get_bind_group_layout(1);
        self.meshes = scene
            .objects()
            .iter()
            .map(|object| {
                GpuMesh::upload(
                    &self.device,
                    &layout,
                    object,
                    selected == Some(object.id),
                )
            })
            .collect();
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    meshes: Vec<GpuMesh>,
    geometry: crate::geometry_cache::GeometryCache,
    settings_allocations: usize,
    uniform: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    depth: wgpu::TextureView,
}

impl Renderer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 24,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform"),
            size: 64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view settings"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        let depth = Self::depth(&device, 640, 480);
        let mut renderer = Self {
            device,
            queue,
            pipeline,
            meshes: Vec::new(),
            geometry: Default::default(),
            settings_allocations: 0,
            uniform,
            view_group,
            depth,
        };
        renderer.set_scene(scene, None);
        renderer
    }

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        self.depth = Self::depth(&self.device, size.width, size.height);
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut meshes = Vec::with_capacity(scene.objects().len());
        for object in scene.objects() {
            let geometry = self.geometry.get(&self.device, &object.mesh);
            meshes.push(GpuMesh::with_geometry(
                &self.device,
                &layout,
                object,
                selected == Some(object.id),
                geometry,
            ));
            self.settings_allocations += 1;
        }
        self.meshes = meshes;
        self.geometry.prune();
    }

    pub fn stats(&self) -> [usize; 4] {
        [
            self.geometry.uploads,
            self.settings_allocations,
            0,
            0,
        ]
    }
```

### 4. `src/browser.rs`

Wait until the renderer exists before reporting GPU diagnostics. Inspect CPU and GPU state after startup synchronization. Update diagnostics after the actual committed action. Pass renderer accounting into the private diagnostics function. Expose counters through a hidden canvas attribute rather than feature controls.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        "data-object-count",
        &editor.scene.objects().len().to_string(),
    )?;
```

Replace it with:

```rust
        "data-object-count",
        &editor.scene.objects().len().to_string(),
    )?;
    canvas.set_attribute(
        "data-gpu-stats",
        &format!("{:?}", renderer.stats()),
    )?;
```

**Locate this block:**

```rust
    Ok(())
}

fn inspect(editor: &Editor) -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or("No document")?;
```

Replace it with:

```rust
    Ok(())
}

fn inspect(editor: &Editor, renderer: &Renderer) -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or("No document")?;
```

**Locate this block:**

```rust
                    }
                }
            }
            if let Err(error) = inspect(&editor) {
                report(&format!(
                    "Cannot report projection: {error:?}"
                ));
```

Replace it with:

```rust
                    }
                }
            }
            if let Err(error) = inspect(&editor, &renderer) {
                report(&format!(
                    "Cannot report projection: {error:?}"
                ));
```

**Locate this block:**

```rust
        &editor.scene,
    );
    renderer.resize(Viewport { width, height });
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
```

Replace it with:

```rust
        &editor.scene,
    );
    renderer.resize(Viewport { width, height });
    inspect(&editor, &renderer)?;
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
```

**Locate this block:**

```rust
    let mut editor = Editor::default();
    inspect(&editor)?;
    let mut renderer = Renderer::new(
        device,
        queue,
```

Replace it with:

```rust
    let mut editor = Editor::default();
    let mut renderer = Renderer::new(
        device,
        queue,
```

## <a id="checkpoint-31c-incremental"></a>4. Update only changed object settings

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/gpu_mesh.rs, src/render…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Scene row</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">matrix or selection byte…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">next draw</text>
</svg>
</div>

**What this code does:** Retain GPU rows by object and geometry identity, then write only changed uniform ranges.

### 1. `src/gpu_mesh.rs`

Retain row identity, its writable settings buffer and the bytes last sent. Allow queued updates to the settings buffer. Keep settings ownership beside the bind group. Queue only changed matrix or selection ranges and count their actual calls and bytes.

**Locate this existing block:**

```rust
    model_group: wgpu::BindGroup,
}

fn settings(object: &Object, selected: bool) -> [u8; 80] {
    let mut bytes = [0; 80];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    let flag: f32 = if selected {
        1.0
    } else {
        0.0
    };
    bytes[64..68].copy_from_slice(&flag.to_ne_bytes());
    bytes
}

impl GpuMesh {
    pub fn upload(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        selected: bool,
    ) -> Self {
        let geometry = Rc::new(GpuGeometry::upload(
            device,
            Rc::clone(&object.mesh),
        ));
        Self::with_geometry(device, layout, object, selected, geometry)
    }

    pub fn with_geometry(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        selected: bool,
        geometry: Rc<GpuGeometry>,
    ) -> Self {
        let bytes = settings(object, selected);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let model_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("object placement"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: model.as_entire_binding(),
            }],
        });
        Self {
            geometry,
            model_group,
        }
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    model_group: wgpu::BindGroup,
    pub id: crate::scene::ObjectId,
    model: wgpu::Buffer,
    previous: [u8; 80],
}

fn settings(object: &Object, selected: bool) -> [u8; 80] {
    let mut bytes = [0; 80];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    let flag: f32 = if selected {
        1.0
    } else {
        0.0
    };
    bytes[64..68].copy_from_slice(&flag.to_ne_bytes());
    bytes
}

impl GpuMesh {
    pub fn upload(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        selected: bool,
    ) -> Self {
        let geometry = Rc::new(GpuGeometry::upload(
            device,
            Rc::clone(&object.mesh),
        ));
        Self::with_geometry(device, layout, object, selected, geometry)
    }

    pub fn with_geometry(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        selected: bool,
        geometry: Rc<GpuGeometry>,
    ) -> Self {
        let bytes = settings(object, selected);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let model_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("object placement"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: model.as_entire_binding(),
            }],
        });
        Self {
            geometry,
            model_group,
            id: object.id,
            model,
            previous: bytes,
        }
    }

    pub fn update(
        &mut self,
        queue: &wgpu::Queue,
        object: &Object,
        selected: bool,
    ) -> [usize; 2] {
        let next = settings(object, selected);
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80)] {
            if next[start..end] != self.previous[start..end] {
                queue.write_buffer(&self.model, start as u64, &next[start..end]);
                writes[0] += 1;
                writes[1] += end - start;
            }
        }
        self.previous = next;
        writes
    }
```

### 2. `src/renderer.rs`

Accumulate settings write evidence. Initialize the cumulative write counters. Retain matching GPU rows, preserve scene draw order and drop unused rows before cache pruning. Report actual update calls and bytes rather than reserved zeros.

**Locate this existing block:**

```rust
    settings_allocations: usize,
    uniform: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    depth: wgpu::TextureView,
}

impl Renderer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 24,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform"),
            size: 64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view settings"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        let depth = Self::depth(&device, 640, 480);
        let mut renderer = Self {
            device,
            queue,
            pipeline,
            meshes: Vec::new(),
            geometry: Default::default(),
            settings_allocations: 0,
            uniform,
            view_group,
            depth,
        };
        renderer.set_scene(scene, None);
        renderer
    }

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        self.depth = Self::depth(&self.device, size.width, size.height);
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut meshes = Vec::with_capacity(scene.objects().len());
        for object in scene.objects() {
            let geometry = self.geometry.get(&self.device, &object.mesh);
            meshes.push(GpuMesh::with_geometry(
                &self.device,
                &layout,
                object,
                selected == Some(object.id),
                geometry,
            ));
            self.settings_allocations += 1;
        }
        self.meshes = meshes;
        self.geometry.prune();
    }

    pub fn stats(&self) -> [usize; 4] {
        [
            self.geometry.uploads,
            self.settings_allocations,
            0,
            0,
        ]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    settings_allocations: usize,
    settings_writes: usize,
    settings_bytes: usize,
    uniform: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    depth: wgpu::TextureView,
}

impl Renderer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 24,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform"),
            size: 64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view settings"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        let depth = Self::depth(&device, 640, 480);
        let mut renderer = Self {
            device,
            queue,
            pipeline,
            meshes: Vec::new(),
            geometry: Default::default(),
            settings_allocations: 0,
            settings_writes: 0,
            settings_bytes: 0,
            uniform,
            view_group,
            depth,
        };
        renderer.set_scene(scene, None);
        renderer
    }

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        self.depth = Self::depth(&self.device, size.width, size.height);
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene.objects() {
            let matching = old.iter().position(|row| {
                row.id == object.id
                    && std::rc::Rc::ptr_eq(&row.geometry.source, &object.mesh)
            });
            let row = if let Some(index) = matching {
                let mut row = old.swap_remove(index);
                let [calls, bytes] = row.update(
                    &self.queue,
                    object,
                    selected == Some(object.id),
                );
                self.settings_writes += calls;
                self.settings_bytes += bytes;
                row
            } else {
                let geometry = self.geometry.get(&self.device, &object.mesh);
                self.settings_allocations += 1;
                GpuMesh::with_geometry(
                    &self.device,
                    &layout,
                    object,
                    selected == Some(object.id),
                    geometry,
                )
            };
            self.meshes.push(row);
        }
        drop(old);
        self.geometry.prune();
    }

    pub fn stats(&self) -> [usize; 4] {
        [
            self.geometry.uploads,
            self.settings_allocations,
            self.settings_writes,
            self.settings_bytes,
        ]
```

## <a id="checkpoint-32-history"></a>5. Find the owners retained by history

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/history.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Scene snapshots</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">history reset</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">ownership tests</text>
</svg>
</div>

**What this code does:** Inspect undo/redo roots and release both branches explicitly.

### 1. `src/history.rs`

Expose borrowed snapshot roots and an explicit reset of both branches. Prove deletion differs from dropping the owners retained by Undo and Redo.

**Locate this existing block:**

```rust
impl History {
    pub fn edit(&mut self, scene: &mut Scene, action: impl FnOnce(&mut Scene)) {
        self.try_edit(scene, |scene| {
            action(scene);
            Ok(())
        })
        .expect("Infallible scene edit");
    }

    pub fn try_edit(
        &mut self,
        scene: &mut Scene,
        action: impl FnOnce(&mut Scene) -> Result<(), &'static str>,
    ) -> Result<(), &'static str> {
        let before = scene.clone();
        if let Err(error) = action(scene) {
            scene.restore(before);
            return Err(error);
        }
        if self.undo.len() == LIMIT {
            self.undo.remove(0);
        }
        self.undo.push(before);
        self.redo.clear();
        Ok(())
    }

    pub fn undo(&mut self, scene: &mut Scene) -> bool {
        Self::travel(&mut self.undo, &mut self.redo, scene)
    }

    pub fn redo(&mut self, scene: &mut Scene) -> bool {
        Self::travel(&mut self.redo, &mut self.undo, scene)
    }

    fn travel(from: &mut Vec<Scene>, to: &mut Vec<Scene>, scene: &mut Scene) -> bool {
        let Some(saved) = from.pop() else {
            return false;
        };
        to.push(scene.clone());
        scene.restore(saved);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
impl History {
    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        self.undo.iter().chain(&self.redo)
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn edit(&mut self, scene: &mut Scene, action: impl FnOnce(&mut Scene)) {
        self.try_edit(scene, |scene| {
            action(scene);
            Ok(())
        })
        .expect("Infallible scene edit");
    }

    pub fn try_edit(
        &mut self,
        scene: &mut Scene,
        action: impl FnOnce(&mut Scene) -> Result<(), &'static str>,
    ) -> Result<(), &'static str> {
        let before = scene.clone();
        if let Err(error) = action(scene) {
            scene.restore(before);
            return Err(error);
        }
        if self.undo.len() == LIMIT {
            self.undo.remove(0);
        }
        self.undo.push(before);
        self.redo.clear();
        Ok(())
    }

    pub fn undo(&mut self, scene: &mut Scene) -> bool {
        Self::travel(&mut self.undo, &mut self.redo, scene)
    }

    pub fn redo(&mut self, scene: &mut Scene) -> bool {
        Self::travel(&mut self.redo, &mut self.undo, scene)
    }

    fn travel(from: &mut Vec<Scene>, to: &mut Vec<Scene>, scene: &mut Scene) -> bool {
        let Some(saved) = from.pop() else {
            return false;
        };
        to.push(scene.clone());
        scene.restore(saved);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn cleared_history_releases_a_deleted_source_and_display() {
        let mut scene = Scene::demo();
        let mut history = History::default();
        let id = scene.objects()[0].id;
        let display = Rc::downgrade(&scene.objects()[0].mesh);
        let source = Rc::downgrade(&scene.objects()[0].geometry);
        history.edit(&mut scene, |scene| {
            scene.remove(id);
        });
        assert!(display.upgrade().is_some() && source.upgrade().is_some());
        assert_eq!(history.scenes().count(), 1);
        history.clear();
        assert!(display.upgrade().is_none() && source.upgrade().is_none());
        assert!(!history.undo(&mut scene) && !history.redo(&mut scene));
        assert_eq!(scene.objects().len(), 1);
    }

    #[test]
    fn clearing_both_branches_releases_a_redo_only_object() {
        let mut scene = Scene::demo();
        let mut history = History::default();
        history.edit(&mut scene, Scene::toggle_extra);
        let display = Rc::downgrade(&scene.objects()[2].mesh);
        history.undo(&mut scene);
        assert!(display.upgrade().is_some());
        history.clear();
        assert!(display.upgrade().is_none());
        assert_eq!(history.scenes().count(), 0);
        assert!(!history.redo(&mut scene));
    }

    #[test]
```

## <a id="checkpoint-32a-cpu"></a>6. Count shared CPU displays once

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/mesh.rs, src/editor.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Borrowed scene roots and…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">identity sets</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">scoped CPU ledger</text>
</svg>
</div>

**What this code does:** Account for active/history rows, retained sources/documents and unique display payloads.

### 1. `src/mesh.rs`

Count allocated display-vector capacity rather than just occupied elements.

**Locate this existing block:**

```rust
    }

    pub fn vertices(&self) -> &[[f32; 6]] {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn payload_bytes(&self) -> usize {
        self.vertices.capacity() * std::mem::size_of::<[f32; 6]>()
            + self.indices.capacity() * std::mem::size_of::<u16>()
    }

    pub fn vertices(&self) -> &[[f32; 6]] {
```

### 2. `src/editor.rs`

Inspect active and retained snapshot roots without creating new owners.

**Locate this existing block:**

```rust
impl Editor {
    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
impl Editor {
    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
```

### 3. `src/renderer.rs`

Expose borrowed live geometry owners for accounting and weak lifetime checks.

**Locate this existing block:**

```rust
    }

    pub fn stats(&self) -> [usize; 4] {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn geometry_owners(
        &self,
    ) -> impl Iterator<Item = &std::rc::Rc<crate::gpu_geometry::GpuGeometry>> {
        self.meshes.iter().map(|row| &row.geometry)
    }

    pub fn stats(&self) -> [usize; 4] {
```

### 4. `src/memory.rs`

Count unique live owners and known display capacities across editor and GPU roots.

Create the file and type:

```rust
use crate::{mesh::Mesh, scene::Scene};
use std::{collections::HashSet, rc::Rc};

pub fn cpu<'a>(
    scenes: impl IntoIterator<Item = &'a Scene>,
    gpu_displays: impl IntoIterator<Item = &'a Rc<Mesh>>,
) -> [usize; 6] {
    let (mut sources, mut documents, mut displays) = (
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
    );
    let mut count = [0; 6];
    for scene in scenes {
        count[5] += 1;
        for object in scene.objects() {
            count[0] += 1;
            sources.insert(Rc::as_ptr(&object.geometry));
            if displays.insert(Rc::as_ptr(&object.mesh)) {
                count[4] += object.mesh.payload_bytes();
            }
            if let Some(source) = &object.source {
                if documents.insert(Rc::as_ptr(&source.document)) {
                    for mesh in &source.document.objects.meshes {
                        sources.insert(Rc::as_ptr(mesh));
                    }
                }
            }
        }
    }
    for display in gpu_displays {
        if displays.insert(Rc::as_ptr(display)) {
            count[4] += display.payload_bytes();
        }
    }
    count[1] = sources.len();
    count[2] = documents.len();
    count[3] = displays.len();
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_snapshots_count_spare_capacity_once() {
        let scene = Scene::demo();
        let saved = scene.clone();
        let baseline = cpu([&scene], std::iter::empty());
        let shared = cpu([&scene, &saved], std::iter::empty());
        assert_eq!(shared[0], 4);
        assert_eq!(shared[5], 2);
        assert_eq!(&shared[1..5], &baseline[1..5]);
        let mut vertices = Vec::with_capacity(16);
        vertices.extend([[0.0; 6]; 3]);
        let mut indices = Vec::with_capacity(16);
        indices.extend([0, 1, 2]);
        let extra = Rc::new(Mesh::new(vertices, indices).unwrap());
        let retained = cpu([&scene], [&extra, &extra]);
        assert_eq!(retained[3], baseline[3] + 1);
        assert_eq!(retained[4], baseline[4] + 16 * 24 + 16 * 2);
    }

    #[test]
    fn a_document_retains_sources_whose_display_row_was_removed() {
        let mut scene = Scene::demo();
        scene
            .import(crate::document::load(&crate::specimen::bytes()).unwrap())
            .unwrap();
        let id = scene.objects()[2].id;
        scene.remove(id);
        let count = cpu([&scene], std::iter::empty());
        assert_eq!(&count[..4], &[4, 5, 1, 4]);
    }
}
```

### 5. `src/lib.rs`

Expose the scoped ownership ledger.

**Locate this existing block:**

```rust
pub mod history;
pub mod mesh;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod history;
pub mod memory;
pub mod mesh;
```

### 6. `src/browser.rs`

Publish accounting only through private canvas diagnostics.

**Locate this existing block:**

```rust
        &format!("{:?}", renderer.stats()),
    )?;
    canvas.set_attribute(
        "data-projection",
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        &format!("{:?}", renderer.stats()),
    )?;
    let cpu = crate::memory::cpu(
        editor.scenes(),
        renderer
            .geometry_owners()
            .map(|owner| &owner.source),
    );
    canvas.set_attribute("data-cpu-usage", &format!("{cpu:?}"))?;
    canvas.set_attribute(
        "data-projection",
```

## <a id="checkpoint-32b-gpu"></a>7. Count each shared GPU buffer once

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 7 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/gpu_geometry.rs, src/gp…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Live GPU rows</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unique geometry owners an…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">buffer-size ledger</text>
</svg>
</div>

**What this code does:** Measure live document buffer sizes separately from cumulative allocation counters.

### 1. `src/gpu_geometry.rs`

Read actual buffer sizes, including initialization alignment.

**Locate this existing block:**

```rust
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn allocated_bytes(&self) -> u64 {
        self.vertices.size() + self.indices.size()
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
```

### 2. `src/gpu_mesh.rs`

Report the independent settings buffer owned by one GPU row.

**Locate this existing block:**

```rust
    }

    pub fn update(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn settings_bytes(&self) -> u64 {
        self.model.size()
    }

    pub fn update(
```

### 3. `src/memory.rs`

Count shared geometry once while retaining each row’s separate settings cost.

**Locate this existing block:**

```rust
}

#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub fn gpu<'a>(
    rows: impl IntoIterator<Item = &'a crate::gpu_mesh::GpuMesh>,
) -> [u64; 4] {
    let mut geometry = HashSet::new();
    let mut count = [0; 4];
    for row in rows {
        if geometry.insert(Rc::as_ptr(&row.geometry)) {
            count[0] += 1;
            count[1] += row.geometry.allocated_bytes();
        }
        count[2] += 1;
        count[3] += row.settings_bytes();
    }
    count
}

#[cfg(test)]
```

### 4. `src/renderer.rs`

Distinguish live document resources from cumulative upload/write counters.

**Locate this existing block:**

```rust
    }

    pub fn stats(&self) -> [usize; 4] {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn usage(&self) -> [u64; 4] {
        crate::memory::gpu(&self.meshes)
    }

    pub fn stats(&self) -> [usize; 4] {
```

### 5. `src/browser.rs`

Expose live document buffer sizes without adding runtime controls.

**Locate this existing block:**

```rust
    canvas.set_attribute(
        "data-projection",
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    canvas.set_attribute(
        "data-gpu-usage",
        &format!("{:?}", renderer.usage()),
    )?;
    canvas.set_attribute(
        "data-projection",
```

## <a id="checkpoint-32c-close"></a>8. Close the document without resetting the view

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 8 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene.rs, src/editor.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Close action</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">scene storage release and…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">empty renderer synchroniz…</text>
</svg>
</div>

**What this code does:** Drop active rows and both history branches while preserving the local ID counter and camera.

### 1. `src/scene.rs`

Release row storage while preserving issued identities; reuse this inside atomic replacement.

**Locate this existing block:**

```rust
    }

    pub fn replace(
        &mut self,
        loaded: crate::document::Loaded,
    ) -> Result<(), &'static str> {
        self.objects.clear();
        self.extra = None;
        self.import(loaded)
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn clear(&mut self) {
        self.objects = Vec::new();
        self.extra = None;
    }

    pub fn replace(
        &mut self,
        loaded: crate::document::Loaded,
    ) -> Result<(), &'static str> {
        self.clear();
        self.import(loaded)
```

### 2. `src/editor.rs`

Give closing a typed editor action. Close outside undoable document edits and preserve application view settings. Check source release, both history branches, identity reuse and view preservation.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    use super::*;

    #[test]
```

Replace it with:

```rust
    use super::*;

    #[test]
    fn close_ends_history_and_keeps_issued_ids_and_view_settings() {
        let mut editor = Editor::default();
        editor
            .apply(Action::Import(crate::specimen::bytes()))
            .unwrap();
        editor.apply(Action::AddBox).unwrap();
        editor.apply(Action::Undo).unwrap();
        let old: Vec<_> = editor
            .scenes()
            .flat_map(|scene| scene.objects().iter().map(|row| row.id))
            .collect();
        editor.apply(Action::SelectNext).unwrap();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap();
        editor.apply(Action::Background).unwrap();
        let view = editor.camera.uniform();
        let background = editor.background.rgb();
        editor.apply(Action::Close).unwrap();
        assert!(editor.scene.objects().is_empty() && editor.selected.is_none());
        assert_eq!(editor.scenes().count(), 1);
        editor.apply(Action::Undo).unwrap();
        editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.objects().is_empty());
        assert_eq!(editor.camera.uniform(), view);
        assert_eq!(editor.background.rgb(), background);
        editor.apply(Action::AddBox).unwrap();
        assert!(!old.contains(&editor.scene.objects()[0].id));
    }

    #[test]
    fn repeated_close_is_empty_and_drops_sources_without_gpu_owners() {
        let mut editor = Editor::default();
        let display = std::rc::Rc::downgrade(&editor.scene.objects()[0].mesh);
        let source = std::rc::Rc::downgrade(&editor.scene.objects()[0].geometry);
        editor.apply(Action::Close).unwrap();
        editor.apply(Action::Close).unwrap();
        assert!(display.upgrade().is_none() && source.upgrade().is_none());
        assert_eq!(
            crate::memory::cpu(editor.scenes(), std::iter::empty()),
            [0, 0, 0, 0, 0, 1]
        );
    }

    #[test]
```

**Locate this block:**

```rust
                    .ray(screen)
                    .and_then(|ray| crate::picking::pick(&self.scene, &ray));
            }
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history
```

Replace it with:

```rust
                    .ray(screen)
                    .and_then(|ray| crate::picking::pick(&self.scene, &ray));
            }
            Action::Close => {
                self.scene.clear();
                self.history.clear();
                self.selected = None;
            }
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history
```

**Locate this block:**

```rust
    Replace(Vec<u8>),
    AddBox,
    Translate([f64; 3]),
    ToggleExtra,
```

Replace it with:

```rust
    Replace(Vec<u8>),
    Close,
    AddBox,
    Translate([f64; 3]),
    ToggleExtra,
```

## <a id="checkpoint-32d-command"></a>9. Close through the command line and revoke reads

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 9 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/browser.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Typed Close</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">editor close</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">dock result</text>
</svg>
</div>

**What this code does:** Wire Close into the dock and cancel pending delivery before closing the editor.

### 1. `src/browser.rs`

Offer closing only through the actual dock vocabulary. Revoke pending file ownership before the editor closes. Retain the submitted operation kind across action consumption. Report close only after its scene operation and renderer synchronization succeed.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            };
            if let Some(action) = action {
                let replacement = matches!(&action, Action::Replace(_));
                match editor.apply(action) {
                    Ok(Change::Scene) => {
                        renderer.set_scene(&editor.scene, editor.selected);
                        if event.type_() == "viewer-file" {
                            let message = if replacement {
```

Replace it with:

```rust
            };
            if let Some(action) = action {
                let replacement = matches!(&action, Action::Replace(_));
                let closing = matches!(&action, Action::Close);
                match editor.apply(action) {
                    Ok(Change::Scene) => {
                        renderer.set_scene(&editor.scene, editor.selected);
                        if closing {
                            report("Document closed.");
                            panel.result("Document closed.");
                        } else if event.type_() == "viewer-file" {
                            let message = if replacement {
```

**Locate this block:**

```rust
            } else if event.type_() == "viewer-file" {
                crate::file_input::action(&event)
            } else if let Some(line) = line {
                if line == "cancel open" {
                    request.borrow_mut().cancel();
                    report("Open cancelled.");
                    panel.result("Open cancelled.");
```

Replace it with:

```rust
            } else if event.type_() == "viewer-file" {
                crate::file_input::action(&event)
            } else if let Some(line) = line {
                if line == "close" {
                    request.borrow_mut().cancel();
                    Some(Action::Close)
                } else if line == "cancel open" {
                    request.borrow_mut().cancel();
                    report("Open cancelled.");
                    panel.result("Open cancelled.");
```

**Locate this block:**

```rust
            "Cancel Open",
            "Save",
            "Example Box",
            "Example Triangle",
```

Replace it with:

```rust
            "Cancel Open",
            "Close",
            "Save",
            "Example Box",
            "Example Triangle",
```

## <a id="checkpoint-32e-release"></a>10. Prove release does not retain the old document

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 10 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/editor.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Weak old owners</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">editor roots gone</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">fresh document IDs</text>
</svg>
</div>

**What this code does:** Check imported history owners, the CPU/GPU release boundary, late read failures and reopening.

### 1. `src/editor.rs`

Verify imported source/document release across history and fresh identities after reopening.

**Locate this existing block:**

```rust
    #[test]
    fn resetting_the_view_preserves_its_current_aspect() {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    #[test]
    fn close_releases_imported_history_documents_and_reopening_uses_fresh_ids() {
        use std::rc::Rc;
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        editor
            .apply(Action::Import(bytes.clone()))
            .unwrap();
        editor
            .apply(Action::Replace(bytes.clone()))
            .unwrap();
        let displays: Vec<_> = editor
            .scenes()
            .flat_map(|scene| {
                scene
                    .objects()
                    .iter()
                    .map(|row| Rc::downgrade(&row.mesh))
            })
            .collect();
        let sources: Vec<_> = editor
            .scenes()
            .flat_map(|scene| {
                scene
                    .objects()
                    .iter()
                    .map(|row| Rc::downgrade(&row.geometry))
            })
            .collect();
        let documents: Vec<_> = editor
            .scenes()
            .flat_map(|scene| {
                scene.objects().iter().filter_map(|row| {
                    row.source
                        .as_ref()
                        .map(|source| Rc::downgrade(&source.document))
                })
            })
            .collect();
        let old: Vec<_> = editor
            .scenes()
            .flat_map(|scene| scene.objects().iter().map(|row| row.id))
            .collect();
        assert!(!documents.is_empty());
        editor.apply(Action::Close).unwrap();
        assert!(
            displays
                .iter()
                .all(|owner| owner.upgrade().is_none())
        );
        assert!(
            sources
                .iter()
                .all(|owner| owner.upgrade().is_none())
        );
        assert!(
            documents
                .iter()
                .all(|owner| owner.upgrade().is_none())
        );
        editor.apply(Action::Import(bytes)).unwrap();
        assert!(
            editor
                .scene
                .objects()
                .iter()
                .all(|row| !old.contains(&row.id))
        );
    }

    #[test]
    fn resetting_the_view_preserves_its_current_aspect() {
```

## <a id="checkpoint-32f-metadata"></a>11. Keep row metadata separate from editable geometry

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 11 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/lib.rs, src/row_metadat…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Prepared source</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared scene snapshots</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">source ownership proof</text>
</svg>
</div>

**What this code does:** Retain names, original source GUIDs and visibility/locking flags without keeping kernel owners alive.

### 1. `src/lib.rs`

Give independently retained row metadata its own small module.

**Locate this existing block:**

```rust
pub mod prepared;
pub mod scene;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod prepared;
pub mod row_metadata;
pub mod scene;
```

### 2. `src/row_metadata.rs`

Copy source identity and flags without copying or retaining kernel payload; test independent lifetime and history.

Create the file and type:

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RowMetadata {
    pub source_guid: String,
    pub name: String,
    pub is_visible: bool,
    pub is_locked: bool,
}

impl RowMetadata {
    pub fn from_mesh(mesh: &session_rust::Mesh) -> Self {
        Self {
            source_guid: mesh.guid().to_owned(),
            name: mesh.name.clone(),
            is_visible: mesh.is_visible,
            is_locked: mesh.is_locked,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::editor::{Action, Editor};
    use std::rc::Rc;

    #[test]
    fn metadata_does_not_keep_a_closed_kernel_source_alive() {
        let mut editor = Editor::default();
        let mut mesh = session_rust::Mesh::create_box(1.0, 1.0, 1.0);
        mesh.name = "Released beam".into();
        mesh.is_visible = false;
        mesh.is_locked = true;
        let id = editor
            .scene
            .insert(crate::prepared::PreparedMesh::new(mesh).unwrap())
            .unwrap();
        let row = editor
            .scene
            .objects()
            .iter()
            .find(|row| row.id == id)
            .unwrap();
        let weak = Rc::downgrade(&row.geometry);
        let metadata = Rc::clone(&row.metadata);
        editor.apply(Action::Close).unwrap();
        assert!(weak.upgrade().is_none());
        assert_eq!(metadata.name, "Released beam");
        assert!(!metadata.is_visible && metadata.is_locked);
        assert!(!metadata.source_guid.is_empty());
    }

    #[test]
    fn duplicate_sources_keep_metadata_through_move_and_undo() {
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        editor
            .apply(Action::Replace(bytes.clone()))
            .unwrap();
        editor.apply(Action::Import(bytes)).unwrap();
        let original = &editor.scene.objects()[0];
        let copy = &editor.scene.objects()[3];
        assert_eq!(original.metadata, copy.metadata);
        assert_ne!(original.guid, copy.guid);
        let metadata = Rc::clone(&original.metadata);
        editor.apply(Action::SelectNext).unwrap();
        editor
            .apply(Action::Translate([0.25, 0.0, 0.0]))
            .unwrap();
        assert!(Rc::ptr_eq(
            &metadata,
            &editor.scene.objects()[0].metadata
        ));
        editor.apply(Action::Undo).unwrap();
        assert!(Rc::ptr_eq(
            &metadata,
            &editor.scene.objects()[0].metadata
        ));
    }
}
```

### 3. `src/scene.rs`

Retain descriptive values independently of the editable source. Capture metadata before transferring prepared source owners into the row.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        {
            guid = uuid::Uuid::new_v4().to_string();
        }
        self.objects.push(Object {
            id,
            guid,
            mesh: Rc::new(prepared.display),
```

Replace it with:

```rust
        {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let metadata = Rc::new(crate::row_metadata::RowMetadata::from_mesh(
            &prepared.geometry,
        ));
        self.objects.push(Object {
            id,
            guid,
            metadata,
            mesh: Rc::new(prepared.display),
```

**Locate this block:**

```rust
    pub guid: String,
    pub mesh: Rc<Mesh>,
    pub geometry: Rc<session_rust::Mesh>,
    pub model: session_rust::Xform,
```

Replace it with:

```rust
    pub guid: String,
    pub metadata: Rc<crate::row_metadata::RowMetadata>,
    pub mesh: Rc<Mesh>,
    pub geometry: Rc<session_rust::Mesh>,
    pub model: session_rust::Xform,
```

### 4. `src/browser.rs`

Expose row values through hidden inspection for actual Chrome import/history checks.

**Locate this existing block:**

```rust
        &editor.scene.objects().len().to_string(),
    )?;
    canvas.set_attribute(
        "data-gpu-stats",
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        &editor.scene.objects().len().to_string(),
    )?;
    let metadata: Vec<_> = editor
        .scene
        .objects()
        .iter()
        .map(|row| {
            let m = &row.metadata;
            (
                &row.guid,
                &m.source_guid,
                &m.name,
                m.is_visible,
                m.is_locked,
            )
        })
        .collect();
    canvas.set_attribute(
        "data-row-metadata",
        &serde_json::to_string(&metadata)
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
    )?;
    canvas.set_attribute(
        "data-gpu-stats",
```

## <a id="checkpoint-32fa-access"></a>12. Ask whether an editable source is available

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 12 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene.rs, src/document.…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Scene row</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Option of borrowed editab…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">checked source consumers</text>
</svg>
</div>

**What this code does:** Route saving, source accounting and owner checks through borrowed source accessors.

### 1. `src/scene.rs`

Borrow source owners through an API that can express later source absence.

**Locate this existing block:**

```rust
impl Object {
    pub fn world_point(&self, vertex: [f32; 6]) -> [f64; 3] {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
impl Object {
    pub fn geometry(&self) -> Option<&Rc<session_rust::Mesh>> {
        Some(&self.geometry)
    }

    pub fn source(&self) -> Option<&crate::document::Source> {
        self.source.as_ref()
    }

    pub fn world_point(&self, vertex: [f32; 6]) -> [f64; 3] {
```

### 2. `src/document.rs`

Require original editable geometry instead of silently exporting display floats.

**Locate this existing block:**

```rust
    for object in scene.objects() {
        let mut mesh = object.geometry.to_proto();
        mesh.guid = object.guid.clone();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    for object in scene.objects() {
        let geometry = object
            .geometry()
            .ok_or("Reload editable sources before saving")?;
        let mut mesh = geometry.to_proto();
        mesh.guid = object.guid.clone();
```

### 3. `src/memory.rs`

Count kernel owners only when they are available. Keep provenance absence independent of row/display accounting.

**Locate this existing block:**

```rust
            count[0] += 1;
            sources.insert(Rc::as_ptr(&object.geometry));
            if displays.insert(Rc::as_ptr(&object.mesh)) {
                count[4] += object.mesh.payload_bytes();
            }
            if let Some(source) = &object.source {
                if documents.insert(Rc::as_ptr(&source.document)) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            count[0] += 1;
            if let Some(geometry) = object.geometry() {
                sources.insert(Rc::as_ptr(geometry));
            }
            if displays.insert(Rc::as_ptr(&object.mesh)) {
                count[4] += object.mesh.payload_bytes();
            }
            if let Some(source) = object.source() {
                if documents.insert(Rc::as_ptr(&source.document)) {
```

### 4. `src/source_tests.rs`

Optional authoring fixture. Update generated_source_and_display_survive_move_and_history through borrowed source access while preserving every existing assertion. Optional authoring fixture. Update imported_source_survives_a_change_of_display_row through borrowed source access while preserving every existing assertion.

**Locate this existing block:**

```rust
    let object = &editor.scene.objects()[2];
    let geometry = Rc::clone(&object.geometry);
    let display = Rc::clone(&object.mesh);
    let point = geometry.vertex_point(0).unwrap();
    let guid = geometry.guid().to_owned();
    let before = object.model.m;
    editor
        .apply(Action::Translate([2.0, -1.0, 0.5]))
        .unwrap();
    let moved = editor.scene.objects()[2].model.m;
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects()[2].model.m, before);
    editor.apply(Action::Redo).unwrap();
    let restored = &editor.scene.objects()[2];
    assert_eq!(restored.model.m, moved);
    assert!(Rc::ptr_eq(&geometry, &restored.geometry));
    assert!(Rc::ptr_eq(&display, &restored.mesh));
    assert_eq!(
        restored.geometry.vertex_point(0).unwrap(),
        point
    );
    assert_eq!(restored.geometry.guid(), guid);
    assert!(restored.source.is_none());
}

#[test]
fn imported_source_survives_a_change_of_display_row() {
    let mut editor = Editor::default();
    editor
        .apply(Action::Import(specimen::bytes()))
        .unwrap();
    let imported = editor.scene.objects()[2].clone();
    let first = editor.scene.objects()[0].id;
    let second = editor.scene.objects()[1].id;
    editor.scene.remove(first);
    editor.scene.remove(second);
    let current = &editor.scene.objects()[0];
    assert_eq!(current.id, imported.id);
    assert!(Rc::ptr_eq(
        &current.geometry,
        &imported.geometry
    ));
    let source = current.source.as_ref().unwrap();
    assert_eq!(current.geometry.guid(), source.guid);
    assert!(
        source
            .document
            .objects
            .meshes
            .iter()
            .any(|m| m.guid() == source.guid)
    );
    assert!(Rc::ptr_eq(
        &source.document,
        &imported.source.as_ref().unwrap().document
    ));
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let object = &editor.scene.objects()[2];
    let geometry = Rc::clone(object.geometry().unwrap());
    let display = Rc::clone(&object.mesh);
    let point = geometry.vertex_point(0).unwrap();
    let guid = geometry.guid().to_owned();
    let before = object.model.m;
    editor
        .apply(Action::Translate([2.0, -1.0, 0.5]))
        .unwrap();
    let moved = editor.scene.objects()[2].model.m;
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects()[2].model.m, before);
    editor.apply(Action::Redo).unwrap();
    let restored = &editor.scene.objects()[2];
    assert_eq!(restored.model.m, moved);
    assert!(Rc::ptr_eq(
        &geometry,
        restored.geometry().unwrap()
    ));
    assert!(Rc::ptr_eq(&display, &restored.mesh));
    assert_eq!(
        restored
            .geometry()
            .unwrap()
            .vertex_point(0)
            .unwrap(),
        point
    );
    assert_eq!(restored.geometry().unwrap().guid(), guid);
    assert!(restored.source().is_none());
}

#[test]
fn imported_source_survives_a_change_of_display_row() {
    let mut editor = Editor::default();
    editor
        .apply(Action::Import(specimen::bytes()))
        .unwrap();
    let imported = editor.scene.objects()[2].clone();
    let first = editor.scene.objects()[0].id;
    let second = editor.scene.objects()[1].id;
    editor.scene.remove(first);
    editor.scene.remove(second);
    let current = &editor.scene.objects()[0];
    assert_eq!(current.id, imported.id);
    assert!(Rc::ptr_eq(
        current.geometry().unwrap(),
        imported.geometry().unwrap()
    ));
    let source = current.source().unwrap();
    assert_eq!(
        current.geometry().unwrap().guid(),
        source.guid
    );
    assert!(
        source
            .document
            .objects
            .meshes
            .iter()
            .any(|m| m.guid() == source.guid)
    );
    assert!(Rc::ptr_eq(
        &source.document,
        &imported.source().unwrap().document
    ));
```

### 5. `src/imported_source_tests.rs`

Optional authoring fixture. Update import_shares_the_retained_sessions_source_allocation through borrowed source access while preserving every existing assertion.

**Locate this existing block:**

```rust
    let imported = editor.scene.objects()[2].clone();
    let geometry = Rc::clone(&imported.geometry);
    let source = imported.source.as_ref().unwrap();
    assert!(
        source
            .document
            .objects
            .meshes
            .iter()
            .any(|m| Rc::ptr_eq(m, &geometry))
    );
    let first = editor.scene.objects()[0].id;
    let second = editor.scene.objects()[1].id;
    editor.scene.remove(first);
    editor.scene.remove(second);
    let moved_row = &editor.scene.objects()[0];
    assert_eq!(moved_row.id, imported.id);
    assert!(Rc::ptr_eq(&moved_row.geometry, &geometry));
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let imported = editor.scene.objects()[2].clone();
    let geometry = Rc::clone(imported.geometry().unwrap());
    let source = imported.source().unwrap();
    assert!(
        source
            .document
            .objects
            .meshes
            .iter()
            .any(|m| Rc::ptr_eq(m, &geometry))
    );
    let first = editor.scene.objects()[0].id;
    let second = editor.scene.objects()[1].id;
    editor.scene.remove(first);
    editor.scene.remove(second);
    let moved_row = &editor.scene.objects()[0];
    assert_eq!(moved_row.id, imported.id);
    assert!(Rc::ptr_eq(
        moved_row.geometry().unwrap(),
        &geometry
    ));
}
```

### 6. `src/snapshot_tests.rs`

Optional authoring fixture. Update snapshot_uses_live_local_sources_and_separate_placements through borrowed source access while preserving every existing assertion.

**Locate this existing block:**

```rust
        .clone();
    let source = object.geometry.to_proto();
    let message = proto::Session::decode(
        document::snapshot(&editor.scene)
            .unwrap()
            .as_slice(),
    )
    .unwrap();
    let meshes = &message.objects.as_ref().unwrap().meshes;
    assert_eq!(meshes.len(), 2);
    let saved = meshes
        .iter()
        .find(|mesh| mesh.guid == object.guid)
        .unwrap();
    assert_eq!(saved.vertices, source.vertices);
    assert_eq!(saved.faces, source.faces);
    assert_eq!(saved.objectcolor, source.objectcolor);
    assert_eq!(
        (saved.is_visible, saved.is_locked),
        (source.is_visible, source.is_locked)
    );
    let placement = message
        .xforms
        .iter()
        .find(|entry| entry.guid == object.guid)
        .unwrap();
    assert_eq!(
        placement.xform.as_ref().unwrap().matrix,
        object.model.m.to_vec()
    );
    assert_eq!(
        message
            .tree
            .unwrap()
            .root
            .unwrap()
            .children
            .len(),
        meshes.len()
    );
    assert_eq!(object.geometry.guid(), source.guid);
    editor.apply(Action::Undo).unwrap();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .clone();
    let source = object.geometry().unwrap().to_proto();
    let message = proto::Session::decode(
        document::snapshot(&editor.scene)
            .unwrap()
            .as_slice(),
    )
    .unwrap();
    let meshes = &message.objects.as_ref().unwrap().meshes;
    assert_eq!(meshes.len(), 2);
    let saved = meshes
        .iter()
        .find(|mesh| mesh.guid == object.guid)
        .unwrap();
    assert_eq!(saved.vertices, source.vertices);
    assert_eq!(saved.faces, source.faces);
    assert_eq!(saved.objectcolor, source.objectcolor);
    assert_eq!(
        (saved.is_visible, saved.is_locked),
        (source.is_visible, source.is_locked)
    );
    let placement = message
        .xforms
        .iter()
        .find(|entry| entry.guid == object.guid)
        .unwrap();
    assert_eq!(
        placement.xform.as_ref().unwrap().matrix,
        object.model.m.to_vec()
    );
    assert_eq!(
        message
            .tree
            .unwrap()
            .root
            .unwrap()
            .children
            .len(),
        meshes.len()
    );
    assert_eq!(
        object.geometry().unwrap().guid(),
        source.guid
    );
    editor.apply(Action::Undo).unwrap();
```

## <a id="checkpoint-32fb-boundary"></a>13. Make editable ownership a private row boundary

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 13 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/editor.rs, src/history.…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Remaining native ownershi…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">borrowed accessors</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">private source fields</text>
</svg>
</div>

**What this code does:** Finish migrating owner checks and prevent other modules from bypassing source availability.

### 1. `src/editor.rs`

Update repeated_close_is_empty_and_drops_sources_without_gpu_owners through borrowed source access while preserving every existing assertion. Update close_releases_imported_history_documents_and_reopening_uses_fresh_ids through borrowed source access while preserving every existing assertion.

**Locate this existing block:**

```rust
        let display = std::rc::Rc::downgrade(&editor.scene.objects()[0].mesh);
        let source = std::rc::Rc::downgrade(&editor.scene.objects()[0].geometry);
        editor.apply(Action::Close).unwrap();
        editor.apply(Action::Close).unwrap();
        assert!(display.upgrade().is_none() && source.upgrade().is_none());
        assert_eq!(
            crate::memory::cpu(editor.scenes(), std::iter::empty()),
            [0, 0, 0, 0, 0, 1]
        );
    }

    #[test]
    fn close_releases_imported_history_documents_and_reopening_uses_fresh_ids() {
        use std::rc::Rc;
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        editor
            .apply(Action::Import(bytes.clone()))
            .unwrap();
        editor
            .apply(Action::Replace(bytes.clone()))
            .unwrap();
        let displays: Vec<_> = editor
            .scenes()
            .flat_map(|scene| {
                scene
                    .objects()
                    .iter()
                    .map(|row| Rc::downgrade(&row.mesh))
            })
            .collect();
        let sources: Vec<_> = editor
            .scenes()
            .flat_map(|scene| {
                scene
                    .objects()
                    .iter()
                    .map(|row| Rc::downgrade(&row.geometry))
            })
            .collect();
        let documents: Vec<_> = editor
            .scenes()
            .flat_map(|scene| {
                scene.objects().iter().filter_map(|row| {
                    row.source
                        .as_ref()
                        .map(|source| Rc::downgrade(&source.document))
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let display = std::rc::Rc::downgrade(&editor.scene.objects()[0].mesh);
        let source =
            std::rc::Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        editor.apply(Action::Close).unwrap();
        editor.apply(Action::Close).unwrap();
        assert!(display.upgrade().is_none() && source.upgrade().is_none());
        assert_eq!(
            crate::memory::cpu(editor.scenes(), std::iter::empty()),
            [0, 0, 0, 0, 0, 1]
        );
    }

    #[test]
    fn close_releases_imported_history_documents_and_reopening_uses_fresh_ids() {
        use std::rc::Rc;
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        editor
            .apply(Action::Import(bytes.clone()))
            .unwrap();
        editor
            .apply(Action::Replace(bytes.clone()))
            .unwrap();
        let displays: Vec<_> = editor
            .scenes()
            .flat_map(|scene| {
                scene
                    .objects()
                    .iter()
                    .map(|row| Rc::downgrade(&row.mesh))
            })
            .collect();
        let sources: Vec<_> = editor
            .scenes()
            .flat_map(|scene| {
                scene
                    .objects()
                    .iter()
                    .map(|row| Rc::downgrade(row.geometry().unwrap()))
            })
            .collect();
        let documents: Vec<_> = editor
            .scenes()
            .flat_map(|scene| {
                scene.objects().iter().filter_map(|row| {
                    row.source()
                        .map(|source| Rc::downgrade(&source.document))
```

### 2. `src/history.rs`

Update cleared_history_releases_a_deleted_source_and_display through borrowed source access while preserving every existing assertion.

**Locate this existing block:**

```rust
        let display = Rc::downgrade(&scene.objects()[0].mesh);
        let source = Rc::downgrade(&scene.objects()[0].geometry);
        history.edit(&mut scene, |scene| {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let display = Rc::downgrade(&scene.objects()[0].mesh);
        let source = Rc::downgrade(scene.objects()[0].geometry().unwrap());
        history.edit(&mut scene, |scene| {
```

### 3. `src/file_identity_tests.rs`

Optional authoring fixture. Update duplicate_imports_keep_distinct_stored_identity_through_history through borrowed source access while preserving every existing assertion.

**Locate this existing block:**

```rust
    assert_eq!(distinct.len(), objects.len());
    assert_eq!(objects[2].guid, objects[2].geometry.guid());
    assert_eq!(
        objects[2].geometry.guid(),
        objects[5].geometry.guid()
    );
    assert_eq!(
        objects[2].source.as_ref().unwrap().guid,
        objects[5].source.as_ref().unwrap().guid
    );
    assert_ne!(objects[2].guid, objects[5].guid);
    assert!(uuid::Uuid::parse_str(&objects[5].guid).is_ok());
    let copy = objects[5].clone();
    editor.apply(Action::SelectNext).unwrap();
    editor.apply(Action::Delete).unwrap();
    let current = editor
        .scene
        .objects()
        .iter()
        .find(|object| object.id == copy.id)
        .unwrap();
    assert_eq!(current.guid, copy.guid);
    editor.apply(Action::Undo).unwrap();
    editor.apply(Action::Redo).unwrap();
    let current = editor
        .scene
        .objects()
        .iter()
        .find(|object| object.id == copy.id)
        .unwrap();
    assert_eq!(current.guid, copy.guid);
    assert_eq!(current.geometry.guid(), copy.geometry.guid());
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(distinct.len(), objects.len());
    assert_eq!(
        objects[2].guid,
        objects[2].geometry().unwrap().guid()
    );
    assert_eq!(
        objects[2].geometry().unwrap().guid(),
        objects[5].geometry().unwrap().guid()
    );
    assert_eq!(
        objects[2].source().unwrap().guid,
        objects[5].source().unwrap().guid
    );
    assert_ne!(objects[2].guid, objects[5].guid);
    assert!(uuid::Uuid::parse_str(&objects[5].guid).is_ok());
    let copy = objects[5].clone();
    editor.apply(Action::SelectNext).unwrap();
    editor.apply(Action::Delete).unwrap();
    let current = editor
        .scene
        .objects()
        .iter()
        .find(|object| object.id == copy.id)
        .unwrap();
    assert_eq!(current.guid, copy.guid);
    editor.apply(Action::Undo).unwrap();
    editor.apply(Action::Redo).unwrap();
    let current = editor
        .scene
        .objects()
        .iter()
        .find(|object| object.id == copy.id)
        .unwrap();
    assert_eq!(current.guid, copy.guid);
    assert_eq!(
        current.geometry().unwrap().guid(),
        copy.geometry().unwrap().guid()
    );
}
```

### 4. `src/document_tests.rs`

Optional authoring fixture. Update import_retains_source_and_undo_removes_the_whole_file through borrowed source access while preserving every existing assertion. Optional authoring fixture. Update repeated_imports_keep_distinct_viewer_ids_and_original_source_guids through borrowed source access while preserving every existing assertion.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    let objects = editor.scene.objects();
    assert_ne!(objects[2].id, objects[5].id);
    assert_eq!(
        objects[2].source.as_ref().unwrap().guid,
        objects[5].source.as_ref().unwrap().guid
    );
    assert!(!Rc::ptr_eq(
        &objects[2].source.as_ref().unwrap().document,
        &objects[5].source.as_ref().unwrap().document
    ));
```

Replace it with:

```rust
    let objects = editor.scene.objects();
    assert_ne!(objects[2].id, objects[5].id);
    assert_eq!(
        objects[2].source().unwrap().guid,
        objects[5].source().unwrap().guid
    );
    assert!(!Rc::ptr_eq(
        &objects[2].source().unwrap().document,
        &objects[5].source().unwrap().document
    ));
```

**Locate this block:**

```rust
    let restored = &editor.scene.objects()[2];
    assert_eq!(restored.id, id);
    assert!(Rc::ptr_eq(
        &restored.source.as_ref().unwrap().document,
        &document
    ));
}
```

Replace it with:

```rust
    let restored = &editor.scene.objects()[2];
    assert_eq!(restored.id, id);
    assert!(Rc::ptr_eq(
        &restored.source().unwrap().document,
        &document
    ));
}
```

**Locate this block:**

```rust
    assert_eq!(editor.scene.objects().len(), 5);
    let source = editor.scene.objects()[2]
        .source
        .as_ref()
        .unwrap();
    let document = Rc::clone(&source.document);
    let guid = source.guid.clone();
    let id = editor.scene.objects()[2].id;
```

Replace it with:

```rust
    assert_eq!(editor.scene.objects().len(), 5);
    let source = editor.scene.objects()[2].source().unwrap();
    let document = Rc::clone(&source.document);
    let guid = source.guid.clone();
    let id = editor.scene.objects()[2].id;
```

### 5. `src/replace_tests.rs`

Optional authoring fixture. Update replacement_history_retains_camera_and_distinct_local_identity through borrowed source access while preserving every existing assertion. Optional authoring fixture. Update refused_replacement_preserves_selection_source_owners_and_redo through borrowed source access while preserving every existing assertion.

**Locate this existing block:**

```rust
    assert!(Rc::ptr_eq(
        &editor.scene.objects()[0].geometry,
        &old.geometry
    ));
    assert!(Rc::ptr_eq(
        &editor.scene.objects()[0].mesh,
        &old.mesh
    ));
    editor.apply(Action::Redo).unwrap();
    assert_eq!(
        editor
            .scene
            .objects()
            .iter()
            .map(|object| object.id)
            .collect::<Vec<_>>(),
        ids
    );
    assert_eq!(
        (editor.camera.distance, editor.camera.aspect),
        (7.0, 2.0)
    );
}

#[test]
fn refused_replacement_preserves_selection_source_owners_and_redo() {
    let mut editor = Editor::default();
    editor.apply(Action::AddBox).unwrap();
    editor.apply(Action::Undo).unwrap();
    editor.apply(Action::SelectNext).unwrap();
    let selected = editor.selected;
    let old = editor.scene.objects()[0].clone();
    assert!(
        editor
            .apply(Action::Replace(vec![255]))
            .is_err()
    );
    assert_eq!(editor.selected, selected);
    assert_eq!(editor.scene.objects().len(), 2);
    assert!(Rc::ptr_eq(
        &editor.scene.objects()[0].geometry,
        &old.geometry
    ));
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert!(Rc::ptr_eq(
        editor.scene.objects()[0].geometry().unwrap(),
        old.geometry().unwrap()
    ));
    assert!(Rc::ptr_eq(
        &editor.scene.objects()[0].mesh,
        &old.mesh
    ));
    editor.apply(Action::Redo).unwrap();
    assert_eq!(
        editor
            .scene
            .objects()
            .iter()
            .map(|object| object.id)
            .collect::<Vec<_>>(),
        ids
    );
    assert_eq!(
        (editor.camera.distance, editor.camera.aspect),
        (7.0, 2.0)
    );
}

#[test]
fn refused_replacement_preserves_selection_source_owners_and_redo() {
    let mut editor = Editor::default();
    editor.apply(Action::AddBox).unwrap();
    editor.apply(Action::Undo).unwrap();
    editor.apply(Action::SelectNext).unwrap();
    let selected = editor.selected;
    let old = editor.scene.objects()[0].clone();
    assert!(
        editor
            .apply(Action::Replace(vec![255]))
            .is_err()
    );
    assert_eq!(editor.selected, selected);
    assert_eq!(editor.scene.objects().len(), 2);
    assert!(Rc::ptr_eq(
        editor.scene.objects()[0].geometry().unwrap(),
        old.geometry().unwrap()
    ));
```

### 6. `src/save_roundtrip_tests.rs`

Optional authoring fixture. Update both_imported_copies_reopen_with_their_stored_identity through borrowed source access while preserving every existing assertion.

**Locate this existing block:**

```rust
        assert_eq!(prepared.model.m, object.model.m);
        assert_eq!(prepared.geometry.name, object.geometry.name);
    }
    assert_eq!(
        editor.scene.objects()[2].geometry.guid(),
        editor.scene.objects()[5].geometry.guid()
    );
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        assert_eq!(prepared.model.m, object.model.m);
        assert_eq!(
            prepared.geometry.name,
            object.geometry().unwrap().name
        );
    }
    assert_eq!(
        editor.scene.objects()[2]
            .geometry()
            .unwrap()
            .guid(),
        editor.scene.objects()[5]
            .geometry()
            .unwrap()
            .guid()
    );
```

### 7. `src/row_metadata.rs`

Update metadata_does_not_keep_a_closed_kernel_source_alive through borrowed source access while preserving every existing assertion.

**Locate this existing block:**

```rust
            .unwrap();
        let weak = Rc::downgrade(&row.geometry);
        let metadata = Rc::clone(&row.metadata);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            .unwrap();
        let weak = Rc::downgrade(row.geometry().unwrap());
        let metadata = Rc::clone(&row.metadata);
```

### 8. `src/scene.rs`

Confine editable kernel storage to the scene row implementation. Prevent other modules from bypassing imported-source availability.

**Locate this existing block:**

```rust
    pub mesh: Rc<Mesh>,
    pub geometry: Rc<session_rust::Mesh>,
    pub model: session_rust::Xform,
    pub source: Option<crate::document::Source>,
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub mesh: Rc<Mesh>,
    geometry: Rc<session_rust::Mesh>,
    pub model: session_rust::Xform,
    source: Option<crate::document::Source>,
}
```

## <a id="checkpoint-32fc-version"></a>14. Record a reload version without retaining geometry

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 14 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Cargo.toml, src/lib.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Own a small document head…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Own a small document head…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Own a small document head…</text>
</svg>
</div>

**What this code does:** Own a small document header, a distinct import ID and an exact file fingerprint.

### 1. `Cargo.toml`

Pin SHA-256 for immutable file-version checks.

**Locate this existing block:**

```toml
prost = "=0.14.4"
uuid = { version = "=1.26.1", features = ["v4", "js"] }
```

Edit the existing block to match this code; keep unchanged lines:

```toml
prost = "=0.14.4"
sha2 = "=0.10.8"
uuid = { version = "=1.26.1", features = ["v4", "js"] }
```

### 2. `src/lib.rs`

Introduce geometry-free reload metadata.

**Locate this existing block:**

```rust
pub mod offset;
pub mod picking;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod offset;
pub mod origin;
pub mod picking;
```

### 3. `src/origin.rs`

Keep descriptive protobuf data and a byte version without retaining kernel objects.

Create the file and type:

```rust
use session_rust::proto;
use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileVersion([u8; 32]);

impl FileVersion {
    pub fn of(bytes: &[u8]) -> Self {
        Self(Sha256::digest(bytes).into())
    }

    pub fn hex(&self) -> String {
        self.0
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect()
    }
}

pub struct Origin {
    pub id: uuid::Uuid,
    pub header: proto::Session,
    pub version: FileVersion,
}

impl Origin {
    pub fn new(message: &proto::Session, bytes: &[u8]) -> Self {
        let header = proto::Session {
            name: message.name.clone(),
            guid: message.guid.clone(),
            tree: message.tree.clone(),
            graph: message.graph.clone(),
            bvh_boxes: message.bvh_boxes.clone(),
            xforms: message.xforms.clone(),
            ..Default::default()
        };
        Self {
            id: uuid::Uuid::new_v4(),
            header,
            version: FileVersion::of(bytes),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_version_matches_a_known_vector_and_changed_bytes_differ() {
        assert_eq!(
            FileVersion::of(b"abc").hex(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            FileVersion::of(b"abc"),
            FileVersion::of(b"abc")
        );
        assert_ne!(
            FileVersion::of(b"abc"),
            FileVersion::of(b"abd")
        );
    }

    #[test]
    fn identical_files_have_distinct_origins_and_geometry_free_headers() {
        use prost::Message;
        let bytes = crate::specimen::bytes();
        let message = proto::Session::decode(bytes.as_slice()).unwrap();
        let first = Origin::new(&message, &bytes);
        let second = Origin::new(&message, &bytes);
        assert_ne!(first.id, second.id);
        assert_eq!(first.version, second.version);
        assert_eq!(first.header.name, message.name);
        assert_eq!(first.header.guid, message.guid);
        assert_eq!(first.header.tree, message.tree);
        assert_eq!(first.header.xforms, message.xforms);
        assert!(first.header.objects.is_none() && first.header.definitions.is_none());
    }
}
```

## <a id="checkpoint-32fd-origin"></a>15. Attach one origin to an imported document

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 15 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/document.rs, src/lib.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Share one geometry-free o…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Share one geometry-free o…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Share one geometry-free o…</text>
</svg>
</div>

**What this code does:** Share one geometry-free origin across imported rows and history.

### 1. `src/document.rs`

Retain reload metadata separately from the kernel Session. Record accepted original bytes and a geometry-free header before kernel construction. Share one import origin among all its source rows.

**Locate this existing block:**

```rust
    pub guid: String,
}

pub struct Loaded {
    pub document: Rc<Session>,
    pub meshes: Vec<PreparedMesh>,
}

pub fn load(bytes: &[u8]) -> Result<Loaded, &'static str> {
    if bytes.len() > MAX_BYTES {
        return Err("This checkpoint accepts files up to 4 MiB");
    }
    let message =
        proto::Session::decode(bytes).map_err(|_| "Invalid session protobuf")?;
    validate(&message)?;
    let document =
        Rc::new(Session::from_proto(message).map_err(|_| "Cannot construct session")?);
    let mut meshes = Vec::new();
    for mesh in document.objects.meshes.iter() {
        let mut prepared = PreparedMesh::from_shared(Rc::clone(mesh))?;
        prepared.model = document
            .xforms
            .get(mesh.guid())
            .cloned()
            .unwrap_or_else(session_rust::Xform::identity);
        prepared.source = Some(Source {
            document: Rc::clone(&document),
            guid: mesh.guid().to_owned(),
        });
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub guid: String,
    pub origin: Rc<crate::origin::Origin>,
}

pub struct Loaded {
    pub document: Rc<Session>,
    pub meshes: Vec<PreparedMesh>,
}

pub fn load(bytes: &[u8]) -> Result<Loaded, &'static str> {
    if bytes.len() > MAX_BYTES {
        return Err("This checkpoint accepts files up to 4 MiB");
    }
    let message =
        proto::Session::decode(bytes).map_err(|_| "Invalid session protobuf")?;
    validate(&message)?;
    let origin = Rc::new(crate::origin::Origin::new(&message, bytes));
    let document =
        Rc::new(Session::from_proto(message).map_err(|_| "Cannot construct session")?);
    let mut meshes = Vec::new();
    for mesh in document.objects.meshes.iter() {
        let mut prepared = PreparedMesh::from_shared(Rc::clone(mesh))?;
        prepared.model = document
            .xforms
            .get(mesh.guid())
            .cloned()
            .unwrap_or_else(session_rust::Xform::identity);
        prepared.source = Some(Source {
            document: Rc::clone(&document),
            guid: mesh.guid().to_owned(),
            origin: Rc::clone(&origin),
        });
```

### 2. `src/lib.rs`

Check source ownership independently of drawing.

**Locate this existing block:**

```rust
pub mod origin;
pub mod picking;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod origin;
#[cfg(test)]
mod origin_tests;
pub mod picking;
```

### 3. `src/origin_tests.rs`

Optional authoring fixture. Observe true kernel expiration while retaining only header/version metadata.

Create the file and type:

```rust
use crate::editor::{Action, Editor};
use std::rc::Rc;

#[test]
fn origin_does_not_pin_a_closed_source_document() {
    let mut editor = Editor::default();
    editor
        .apply(Action::Replace(crate::specimen::bytes()))
        .unwrap();
    let row = &editor.scene.objects()[0];
    let source = row.source().unwrap();
    let origin = Rc::clone(&source.origin);
    let document = Rc::downgrade(&source.document);
    let mesh = Rc::downgrade(row.geometry().unwrap());
    assert!(
        editor
            .scene
            .objects()
            .iter()
            .all(|row| Rc::ptr_eq(&row.source().unwrap().origin, &origin))
    );
    editor.apply(Action::Close).unwrap();
    assert!(document.upgrade().is_none() && mesh.upgrade().is_none());
    assert_eq!(origin.header.name, "Three-piece frame");
    assert!(origin.header.objects.is_none());
}

#[test]
fn duplicate_imports_and_history_keep_their_own_origins() {
    let mut editor = Editor::default();
    let bytes = crate::specimen::bytes();
    editor
        .apply(Action::Replace(bytes.clone()))
        .unwrap();
    editor.apply(Action::Import(bytes)).unwrap();
    let first = Rc::clone(
        &editor.scene.objects()[0]
            .source()
            .unwrap()
            .origin,
    );
    let second = Rc::clone(
        &editor.scene.objects()[3]
            .source()
            .unwrap()
            .origin,
    );
    assert_ne!(first.id, second.id);
    assert_eq!(first.version, second.version);
    editor.apply(Action::SelectNext).unwrap();
    editor
        .apply(Action::Translate([0.25, 0.0, 0.0]))
        .unwrap();
    editor.apply(Action::Undo).unwrap();
    assert!(Rc::ptr_eq(
        &first,
        &editor.scene.objects()[0]
            .source()
            .unwrap()
            .origin
    ));
    assert!(Rc::ptr_eq(
        &second,
        &editor.scene.objects()[3]
            .source()
            .unwrap()
            .origin
    ));
}
```

### 4. `src/browser.rs`

Inspect shared import identity and exact versions without adding controls.

**Locate this existing block:**

```rust
        .collect();
    canvas.set_attribute(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .collect();
    let origins: Vec<_> = editor
        .scene
        .objects()
        .iter()
        .filter_map(|row| {
            row.source().map(|source| {
                (
                    &row.guid,
                    source.origin.id.to_string(),
                    source.origin.version.hex(),
                )
            })
        })
        .collect();
    canvas.set_attribute(
        "data-source-origins",
        &serde_json::to_string(&origins)
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
    )?;
    canvas.set_attribute(
```

## <a id="checkpoint-32fe-location"></a>16. Give a reload URL an explicit owner

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 16 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/lib.rs, src/reload_url.…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Retain an owned Blob URL…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Retain an owned Blob URL…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Retain an owned Blob URL…</text>
</svg>
</div>

**What this code does:** Retain an owned Blob URL through imports and history, then release it with its last owner.

### 1. `src/lib.rs`

Keep browser URL lifetime explicit and independent of kernel ownership.

**Locate this existing block:**

```rust
pub mod prepared;
pub mod row_metadata;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod prepared;
pub mod reload_url;
pub mod row_metadata;
```

### 2. `src/reload_url.rs`

Revoke a viewer-created Blob URL when its actual resource owner drops.

Create the file and type:

```rust
pub struct ReloadUrl(String);

impl ReloadUrl {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    #[cfg(target_arch = "wasm32")]
    pub fn from_file(file: &web_sys::File) -> Result<Self, wasm_bindgen::JsValue> {
        web_sys::Url::create_object_url_with_blob(file).map(Self)
    }
}

impl Drop for ReloadUrl {
    fn drop(&mut self) {
        #[cfg(target_arch = "wasm32")]
        let _ = web_sys::Url::revoke_object_url(&self.0);
    }
}
```

### 3. `src/origin.rs`

Retain an optional reload location without retaining geometry. Ordinary native loads start without an owned browser location.

**Locate this existing block:**

```rust
    pub version: FileVersion,
}

impl Origin {
    pub fn new(message: &proto::Session, bytes: &[u8]) -> Self {
        let header = proto::Session {
            name: message.name.clone(),
            guid: message.guid.clone(),
            tree: message.tree.clone(),
            graph: message.graph.clone(),
            bvh_boxes: message.bvh_boxes.clone(),
            xforms: message.xforms.clone(),
            ..Default::default()
        };
        Self {
            id: uuid::Uuid::new_v4(),
            header,
            version: FileVersion::of(bytes),
        }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub version: FileVersion,
    pub location: Option<std::rc::Rc<crate::reload_url::ReloadUrl>>,
}

impl Origin {
    pub fn new(message: &proto::Session, bytes: &[u8]) -> Self {
        let header = proto::Session {
            name: message.name.clone(),
            guid: message.guid.clone(),
            tree: message.tree.clone(),
            graph: message.graph.clone(),
            bvh_boxes: message.bvh_boxes.clone(),
            xforms: message.xforms.clone(),
            ..Default::default()
        };
        Self {
            id: uuid::Uuid::new_v4(),
            header,
            version: FileVersion::of(bytes),
            location: None,
        }
```

### 4. `src/document.rs`

Accept location ownership before a read/validation error can occur. Adopt the already owned URL only into validated source metadata.

**Locate this existing block:**

```rust
pub fn load(bytes: &[u8]) -> Result<Loaded, &'static str> {
    if bytes.len() > MAX_BYTES {
        return Err("This checkpoint accepts files up to 4 MiB");
    }
    let message =
        proto::Session::decode(bytes).map_err(|_| "Invalid session protobuf")?;
    validate(&message)?;
    let origin = Rc::new(crate::origin::Origin::new(&message, bytes));
    let document =
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn load(bytes: &[u8]) -> Result<Loaded, &'static str> {
    load_at(bytes, None)
}

pub fn load_at(
    bytes: &[u8],
    location: Option<Rc<crate::reload_url::ReloadUrl>>,
) -> Result<Loaded, &'static str> {
    if bytes.len() > MAX_BYTES {
        return Err("This checkpoint accepts files up to 4 MiB");
    }
    let message =
        proto::Session::decode(bytes).map_err(|_| "Invalid session protobuf")?;
    validate(&message)?;
    let mut origin = crate::origin::Origin::new(&message, bytes);
    origin.location = location;
    let origin = Rc::new(origin);
    let document =
```

### 5. `src/editor.rs`

Keep a URL owner alive until an import commits or fails. Preserve the same atomic insertion and replacement policy for located files.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                self.history
                    .try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
```

Replace it with:

```rust
                self.history
                    .try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ImportAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history
                    .try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ReplaceAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.replace(loaded)
                    })?;
            }
```

**Locate this block:**

```rust
    Replace(Vec<u8>),
    Close,
    AddBox,
    Translate([f64; 3]),
```

Replace it with:

```rust
    Replace(Vec<u8>),
    ImportAt(
        Vec<u8>,
        std::rc::Rc<crate::reload_url::ReloadUrl>,
    ),
    ReplaceAt(
        Vec<u8>,
        std::rc::Rc<crate::reload_url::ReloadUrl>,
    ),
    Close,
    AddBox,
    Translate([f64; 3]),
```

### 6. `src/origin_tests.rs`

Optional authoring fixture. Prove successful/history and failed-import URL lifetimes with Weak observers.

**Locate this existing block:**

```rust
    ));
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ));
}

#[test]
fn located_sources_keep_the_url_through_history_and_release_it_on_close() {
    let mut editor = Editor::default();
    let url = Rc::new(crate::reload_url::ReloadUrl::new(
        "test:source".into(),
    ));
    let weak = Rc::downgrade(&url);
    editor
        .apply(Action::ReplaceAt(
            crate::specimen::bytes(),
            url,
        ))
        .unwrap();
    editor.apply(Action::SelectNext).unwrap();
    editor.apply(Action::Delete).unwrap();
    editor.apply(Action::Undo).unwrap();
    assert!(weak.upgrade().is_some());
    editor.apply(Action::Close).unwrap();
    assert!(weak.upgrade().is_none());
    let url = Rc::new(crate::reload_url::ReloadUrl::new(
        "test:invalid".into(),
    ));
    let weak = Rc::downgrade(&url);
    assert!(
        editor
            .apply(Action::ImportAt(vec![255], url))
            .is_err()
    );
    assert!(weak.upgrade().is_none());
}
```

## <a id="checkpoint-32ff-bridge"></a>17. Adopt the selected file as a reloadable source

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 17 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/file_input.rs, src/brow…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Transfer the accepted Fil…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Transfer the accepted Fil…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Transfer the accepted Fil…</text>
</svg>
</div>

**What this code does:** Transfer the accepted File through synchronous delivery and create its URL at adoption.

### 1. `src/file_input.rs`

Hold an accepted File only across one synchronous editor delivery. Give the accepted read access to its scoped handoff slot. Transfer the File only after request ownership has been accepted. Pair bytes with their retained immutable File during dispatch. Always drop an unclaimed File after synchronous delivery, including dispatch failure. Adopt URL ownership only for a validated event with an accepted File.

**Locate this existing block:**

```rust
}

pub fn choose(
    event: &web_sys::Event,
    request: Rc<RefCell<crate::read_gate::ReadGate>>,
    mode: Mode,
) {
    let Some(input) = event.target().and_then(|target| {
        target
            .dyn_into::<web_sys::HtmlInputElement>()
            .ok()
    }) else {
        return;
    };
    if input.id() != "open" {
        return;
    }
    let Some(file) = input.files().and_then(|files| files.get(0)) else {
        request.borrow_mut().cancel();
        return;
    };
    input.set_value("");
    let Some(id) = request.borrow().pending() else {
        return;
    };
    super::browser::report("Reading the selected file…");
    wasm_bindgen_futures::spawn_local(async move {
        if request.borrow().pending() != Some(id) {
            return;
        }
        if file.size() > crate::document::MAX_BYTES as f64 {
            let accepted = request.borrow_mut().finish(id);
            if accepted {
                failure("This checkpoint accepts files up to 4 MiB");
            }
            return;
        }
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        let accepted = request.borrow_mut().finish(id);
        if !accepted {
            return;
        }
        let result = result.and_then(|buffer| deliver(buffer, mode));
        if let Err(error) = result {
            failure(&format!("Cannot read file: {error:?}"));
        }
    });
}

fn failure(message: &str) {
    let detail = web_sys::CustomEventInit::new();
    detail.set_detail(&JsValue::from_str(message));
    let result =
        web_sys::CustomEvent::new_with_event_init_dict("viewer-file-error", &detail)
            .and_then(|event| {
                web_sys::window()
                    .ok_or_else(|| JsValue::from_str("No window"))?
                    .dispatch_event(&event)
            });
    if result.is_err() {
        super::browser::report(message);
    }
}

fn deliver(buffer: JsValue, mode: Mode) -> Result<(), JsValue> {
    let detail = web_sys::CustomEventInit::new();
    let payload = js_sys::Array::new();
    payload.push(&JsValue::from_bool(mode == Mode::Replace));
    payload.push(&js_sys::Uint8Array::new(&buffer));
    detail.set_detail(&payload);
    let event = web_sys::CustomEvent::new_with_event_init_dict("viewer-file", &detail)?;
    web_sys::window()
        .ok_or("No browser window")?
        .dispatch_event(&event)?;
    Ok(())
}

pub fn action(event: &web_sys::Event) -> Option<crate::editor::Action> {
    let event = event.dyn_ref::<web_sys::CustomEvent>()?;
    let payload = event
        .detail()
        .dyn_into::<js_sys::Array>()
        .ok()?;
    if payload.length() != 2 {
        return None;
    }
    let replace = payload.get(0).as_bool()?;
    let array = payload
        .get(1)
        .dyn_into::<js_sys::Uint8Array>()
        .ok()?;
    if array.length() as usize > crate::document::MAX_BYTES {
        return None;
    }
    Some(if replace {
        crate::editor::Action::Replace(array.to_vec())
    } else {
        crate::editor::Action::Import(array.to_vec())
    })
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub type Delivery = Rc<RefCell<Option<web_sys::File>>>;

pub fn choose(
    event: &web_sys::Event,
    request: Rc<RefCell<crate::read_gate::ReadGate>>,
    mode: Mode,
    delivery: Delivery,
) {
    let Some(input) = event.target().and_then(|target| {
        target
            .dyn_into::<web_sys::HtmlInputElement>()
            .ok()
    }) else {
        return;
    };
    if input.id() != "open" {
        return;
    }
    let Some(file) = input.files().and_then(|files| files.get(0)) else {
        request.borrow_mut().cancel();
        return;
    };
    input.set_value("");
    let Some(id) = request.borrow().pending() else {
        return;
    };
    super::browser::report("Reading the selected file…");
    wasm_bindgen_futures::spawn_local(async move {
        if request.borrow().pending() != Some(id) {
            return;
        }
        if file.size() > crate::document::MAX_BYTES as f64 {
            let accepted = request.borrow_mut().finish(id);
            if accepted {
                failure("This checkpoint accepts files up to 4 MiB");
            }
            return;
        }
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        let accepted = request.borrow_mut().finish(id);
        if !accepted {
            return;
        }
        let result = result.and_then(|buffer| deliver(buffer, mode, file, &delivery));
        if let Err(error) = result {
            failure(&format!("Cannot read file: {error:?}"));
        }
    });
}

fn failure(message: &str) {
    let detail = web_sys::CustomEventInit::new();
    detail.set_detail(&JsValue::from_str(message));
    let result =
        web_sys::CustomEvent::new_with_event_init_dict("viewer-file-error", &detail)
            .and_then(|event| {
                web_sys::window()
                    .ok_or_else(|| JsValue::from_str("No window"))?
                    .dispatch_event(&event)
            });
    if result.is_err() {
        super::browser::report(message);
    }
}

fn deliver(
    buffer: JsValue,
    mode: Mode,
    file: web_sys::File,
    delivery: &Delivery,
) -> Result<(), JsValue> {
    let detail = web_sys::CustomEventInit::new();
    let payload = js_sys::Array::new();
    payload.push(&JsValue::from_bool(mode == Mode::Replace));
    payload.push(&js_sys::Uint8Array::new(&buffer));
    detail.set_detail(&payload);
    let event = web_sys::CustomEvent::new_with_event_init_dict("viewer-file", &detail)?;
    *delivery.borrow_mut() = Some(file);
    let result = web_sys::window()
        .ok_or_else(|| JsValue::from_str("No browser window"))
        .and_then(|window| window.dispatch_event(&event));
    delivery.borrow_mut().take();
    result.map(|_| ())
}

pub fn action(
    event: &web_sys::Event,
    delivery: &Delivery,
) -> Result<Option<crate::editor::Action>, JsValue> {
    let Some(event) = event.dyn_ref::<web_sys::CustomEvent>() else {
        return Ok(None);
    };
    let Ok(payload) = event.detail().dyn_into::<js_sys::Array>() else {
        return Ok(None);
    };
    if payload.length() != 2 {
        return Ok(None);
    }
    let Some(replace) = payload.get(0).as_bool() else {
        return Ok(None);
    };
    let Ok(array) = payload
        .get(1)
        .dyn_into::<js_sys::Uint8Array>()
    else {
        return Ok(None);
    };
    if array.length() as usize > crate::document::MAX_BYTES {
        return Ok(None);
    }
    let Some(file) = delivery.borrow_mut().take() else {
        return Ok(None);
    };
    let location = Rc::new(crate::reload_url::ReloadUrl::from_file(
        &file,
    )?);
    Ok(Some(if replace {
        crate::editor::Action::ReplaceAt(array.to_vec(), location)
    } else {
        crate::editor::Action::ImportAt(array.to_vec(), location)
    }))
}
```

### 2. `src/browser.rs`

Share one scoped delivery slot between reader and editor callback. Keep request tickets and File adoption in the same delivery path. Report adoption failure without claiming an import committed. Keep Replace success reporting accurate for the located action. Expose only hidden origin URL inspection for actual lifetime checks.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    &row.guid,
                    source.origin.id.to_string(),
                    source.origin.version.hex(),
                )
```

Replace it with:

```rust
                    &row.guid,
                    source.origin.id.to_string(),
                    source.origin.version.hex(),
                    source
                        .origin
                        .location
                        .as_ref()
                        .map(|location| location.value()),
                )
```

**Locate this block:**

```rust
                None
            };
            if let Some(action) = action {
                let replacement = matches!(&action, Action::Replace(_));
                let closing = matches!(&action, Action::Close);
                match editor.apply(action) {
                    Ok(Change::Scene) => {
```

Replace it with:

```rust
                None
            };
            if let Some(action) = action {
                let replacement = matches!(
                    &action,
                    Action::Replace(_) | Action::ReplaceAt(_, _)
                );
                let closing = matches!(&action, Action::Close);
                match editor.apply(action) {
                    Ok(Change::Scene) => {
```

**Locate this block:**

```rust
                }
                None
            } else if event.type_() == "viewer-file" {
                crate::file_input::action(&event)
            } else if let Some(line) = line {
                if line == "close" {
                    request.borrow_mut().cancel();
```

Replace it with:

```rust
                }
                None
            } else if event.type_() == "viewer-file" {
                match crate::file_input::action(&event, &delivery) {
                    Ok(action) => action,
                    Err(error) => {
                        let message = format!("Cannot retain reload file: {error:?}");
                        report(&message);
                        panel.answer("Open", &message);
                        None
                    }
                }
            } else if let Some(line) = line {
                if line == "close" {
                    request.borrow_mut().cancel();
```

**Locate this block:**

```rust
                    &event,
                    std::rc::Rc::clone(&request),
                    read_mode.get(),
                );
                None
            } else if event.type_() == "viewer-file-error" {
```

Replace it with:

```rust
                    &event,
                    std::rc::Rc::clone(&request),
                    read_mode.get(),
                    std::rc::Rc::clone(&delivery),
                );
                None
            } else if event.type_() == "viewer-file-error" {
```

**Locate this block:**

```rust
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(
        move |event: web_sys::Event| {
            let shortcut = event
```

Replace it with:

```rust
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let delivery: crate::file_input::Delivery =
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(
        move |event: web_sys::Event| {
            let shortcut = event
```

## <a id="checkpoint-32fg-state"></a>18. Separate loaded and released editable ownership

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 18 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/lib.rs, src/edit_source…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Represent source residenc…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Represent source residenc…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Represent source residenc…</text>
</svg>
</div>

**What this code does:** Represent source residency without changing retained display, identity or placement.

### 1. `src/lib.rs`

Confine source residency representation to the viewer implementation.

**Locate this existing block:**

```rust
pub mod camera;
pub mod editor;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod camera;
mod edit_source;
pub mod editor;
```

### 2. `src/edit_source.rs`

Keep Released ownership free of kernel mesh and Session values.

Create the file and type:

```rust
use std::rc::Rc;

#[derive(Clone)]
pub enum EditSource {
    Loaded {
        geometry: Rc<session_rust::Mesh>,
        source: Option<crate::document::Source>,
    },
    Released {
        origin: Rc<crate::origin::Origin>,
        epoch: u64,
    },
}
```

### 3. `src/scene.rs`

Retain public display fields while replacing only editable ownership. Borrow loaded owners safely while retaining origin and epoch in the released state. Preserve the validated insertion contract and display owner. Keep partial-replacement ownership assertions through the existing accessor boundary.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        assert_eq!(scene.objects.len(), before.len());
        for (object, old) in scene.objects.iter().zip(&before) {
            assert_eq!(object.id, old.id);
            assert!(Rc::ptr_eq(&object.geometry, &old.geometry));
            assert!(Rc::ptr_eq(&object.mesh, &old.mesh));
```

Replace it with:

```rust
        assert_eq!(scene.objects.len(), before.len());
        for (object, old) in scene.objects.iter().zip(&before) {
            assert_eq!(object.id, old.id);
            assert!(Rc::ptr_eq(
                object.geometry().unwrap(),
                old.geometry().unwrap()
            ));
            assert!(Rc::ptr_eq(&object.mesh, &old.mesh));
```

**Locate this block:**

```rust
        let metadata = Rc::new(crate::row_metadata::RowMetadata::from_mesh(
            &prepared.geometry,
        ));
        self.objects.push(Object {
            id,
            guid,
            metadata,
            mesh: Rc::new(prepared.display),
            geometry: prepared.geometry,
            model: prepared.model,
            source: prepared.source,
        });
        Ok(id)
    }
```

Replace it with:

```rust
        let metadata = Rc::new(crate::row_metadata::RowMetadata::from_mesh(
            &prepared.geometry,
        ));
        let editable = crate::edit_source::EditSource::Loaded {
            geometry: prepared.geometry,
            source: prepared.source,
        };
        self.objects.push(Object {
            id,
            guid,
            metadata,
            mesh: Rc::new(prepared.display),
            model: prepared.model,
            editable,
        });
        Ok(id)
    }
```

**Locate this block:**

```rust
    pub mesh: Rc<Mesh>,
    geometry: Rc<session_rust::Mesh>,
    pub model: session_rust::Xform,
    source: Option<crate::document::Source>,
}

impl Object {
    pub fn geometry(&self) -> Option<&Rc<session_rust::Mesh>> {
        Some(&self.geometry)
    }

    pub fn source(&self) -> Option<&crate::document::Source> {
        self.source.as_ref()
    }

    pub fn world_point(&self, vertex: [f32; 6]) -> [f64; 3] {
```

Replace it with:

```rust
    pub mesh: Rc<Mesh>,
    pub model: session_rust::Xform,
    editable: crate::edit_source::EditSource,
}

impl Object {
    pub fn geometry(&self) -> Option<&Rc<session_rust::Mesh>> {
        match &self.editable {
            crate::edit_source::EditSource::Loaded { geometry, .. } => Some(geometry),
            _ => None,
        }
    }

    pub fn source(&self) -> Option<&crate::document::Source> {
        match &self.editable {
            crate::edit_source::EditSource::Loaded { source, .. } => source.as_ref(),
            _ => None,
        }
    }

    pub fn origin(&self) -> Option<&Rc<crate::origin::Origin>> {
        match &self.editable {
            crate::edit_source::EditSource::Loaded { source, .. } => {
                source.as_ref().map(|source| &source.origin)
            }
            crate::edit_source::EditSource::Released { origin, .. } => Some(origin),
        }
    }

    pub fn release_epoch(&self) -> Option<u64> {
        match &self.editable {
            crate::edit_source::EditSource::Released { epoch, .. } => Some(*epoch),
            _ => None,
        }
    }

    pub fn world_point(&self, vertex: [f32; 6]) -> [f64; 3] {
```

### 4. `src/browser.rs`

Retain origin inspection even when editable kernel ownership is absent.

**Locate this existing block:**

```rust
        .filter_map(|row| {
            row.source().map(|source| {
                (
                    &row.guid,
                    source.origin.id.to_string(),
                    source.origin.version.hex(),
                    source
                        .origin
                        .location
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .filter_map(|row| {
            row.origin().map(|origin| {
                (
                    &row.guid,
                    origin.id.to_string(),
                    origin.version.hex(),
                    origin
                        .location
```

## <a id="checkpoint-32fh-policy"></a>19. Protect sources that cannot be unloaded faithfully

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 19 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/scene.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Require a reload location…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Require a reload location…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Require a reload location…</text>
</svg>
</div>

**What this code does:** Require a reload location, original kernel allocation and unchanged row metadata.

### 1. `src/scene.rs`

Only original, located imported sources can be restored by their original file. Apply a validated origin set without touching retained display/model/metadata owners. Prove the conditions that protect data not faithfully reloadable from the original file.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    use super::*;

    #[test]
```

Replace it with:

```rust
    use super::*;

    #[test]
    fn unloading_policy_protects_generated_unlocated_and_modified_sources() {
        let mut scene = Scene::demo();
        assert!(!scene.objects[0].can_unload());
        let bytes = crate::specimen::bytes();
        scene
            .replace(crate::document::load(&bytes).unwrap())
            .unwrap();
        assert!(!scene.objects[0].can_unload());
        let url = Rc::new(crate::reload_url::ReloadUrl::new(
            "test:policy".into(),
        ));
        scene
            .replace(crate::document::load_at(&bytes, Some(url)).unwrap())
            .unwrap();
        assert!(scene.objects.iter().all(Object::can_unload));
        let row = &mut scene.objects[0];
        Rc::make_mut(&mut row.metadata).is_locked = true;
        assert!(!row.can_unload());
        Rc::make_mut(&mut row.metadata).is_locked = row.geometry().unwrap().is_locked;
        assert!(row.can_unload());
        let source = row.source().cloned();
        let geometry = Rc::new(session_rust::Mesh::create_box(1.0, 1.0, 1.0));
        row.editable = crate::edit_source::EditSource::Loaded { geometry, source };
        assert!(!row.can_unload());
    }

    #[test]
```

**Locate this block:**

```rust
        &self.objects
    }

    pub fn insert(&mut self, prepared: PreparedMesh) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
```

Replace it with:

```rust
        &self.objects
    }

    pub(crate) fn objects_mut(&mut self) -> &mut [Object] {
        &mut self.objects
    }

    pub(crate) fn unload(
        &mut self,
        origins: &std::collections::HashSet<uuid::Uuid>,
        epoch: u64,
    ) {
        for row in &mut self.objects {
            if let Some(origin) = row
                .origin()
                .filter(|origin| origins.contains(&origin.id))
                .cloned()
            {
                row.editable =
                    crate::edit_source::EditSource::Released { origin, epoch };
            }
        }
    }

    pub fn insert(&mut self, prepared: PreparedMesh) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
```

**Locate this block:**

```rust
    }

    pub fn world_point(&self, vertex: [f32; 6]) -> [f64; 3] {
        let local = session_rust::Point::new(
            vertex[0] as f64,
```

Replace it with:

```rust
    }

    pub fn can_unload(&self) -> bool {
        let Some(source) = self.source() else {
            return false;
        };
        let Some(geometry) = self.geometry() else {
            return false;
        };
        source.origin.location.is_some()
            && self.metadata.source_guid == source.guid
            && geometry.guid() == source.guid
            && self.metadata.name == geometry.name
            && self.metadata.is_visible == geometry.is_visible
            && self.metadata.is_locked == geometry.is_locked
            && source
                .document
                .objects
                .meshes
                .iter()
                .any(|mesh| Rc::ptr_eq(mesh, geometry))
    }

    pub fn world_point(&self, vertex: [f32; 6]) -> [f64; 3] {
        let local = session_rust::Point::new(
            vertex[0] as f64,
```

## <a id="checkpoint-32fi-history"></a>20. Unload sources across active and history roots

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 20 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/history.rs, src/editor.…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Release a validated whole…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Release a validated whole…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Release a validated whole…</text>
</svg>
</div>

**What this code does:** Release a validated whole-origin set without clearing Undo/Redo or changing placements.

### 1. `src/history.rs`

Visit both history branches without cloning or dropping their rows.

**Locate this existing block:**

```rust
    }

    pub fn clear(&mut self) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub(crate) fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        self.undo.iter_mut().chain(&mut self.redo)
    }

    pub fn clear(&mut self) {
```

### 2. `src/editor.rs`

Add a residency operation distinct from document close. Issue epochs independently of document history and Close. Start the first viewer-owned release sequence without wraparound. Validate every retained origin row before issuing an epoch or changing any owner. Keep source unloading out of document Undo transactions.

**Locate this existing block:**

```rust
    Close,
    AddBox,
    Translate([f64; 3]),
    ToggleExtra,
    SelectNext,
    Delete,
    Undo,
    Redo,
    Background,
    Zoom(f32),
    Pan(f32, f32),
    Orbit(f64, f64),
    Isometric,
    ResetView,
    Fit,
    FitSelected,
    Projection(crate::camera::Projection),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Change {
    View,
    Scene,
}

pub struct Editor {
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub camera: Camera,
    pub background: Background,
    history: History,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            scene: Scene::demo(),
            selected: None,
            camera: Camera::default(),
            background: Background::default(),
            history: History::default(),
        }
    }
}

impl Editor {
    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Pick(screen) => {
                self.selected = self
                    .camera
                    .ray(screen)
                    .and_then(|ray| crate::picking::pick(&self.scene, &ray));
            }
            Action::Close => {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Close,
    UnloadSources,
    AddBox,
    Translate([f64; 3]),
    ToggleExtra,
    SelectNext,
    Delete,
    Undo,
    Redo,
    Background,
    Zoom(f32),
    Pan(f32, f32),
    Orbit(f64, f64),
    Isometric,
    ResetView,
    Fit,
    FitSelected,
    Projection(crate::camera::Projection),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Change {
    View,
    Scene,
}

pub struct Editor {
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub camera: Camera,
    pub background: Background,
    history: History,
    last_release: u64,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            scene: Scene::demo(),
            selected: None,
            camera: Camera::default(),
            background: Background::default(),
            history: History::default(),
            last_release: 0,
        }
    }
}

impl Editor {
    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        std::iter::once(&mut self.scene).chain(self.history.scenes_mut())
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
        let mut origins: std::collections::HashSet<_> = self
            .scenes()
            .flat_map(|scene| scene.objects())
            .filter(|row| row.can_unload())
            .filter_map(|row| row.origin().map(|origin| origin.id))
            .collect();
        for row in self
            .scenes()
            .flat_map(|scene| scene.objects())
        {
            if let Some(origin) = row.origin() {
                if !row.can_unload() {
                    origins.remove(&origin.id);
                }
            }
        }
        if origins.is_empty() {
            return Err("No eligible imported sources to unload");
        }
        let epoch = self
            .last_release
            .checked_add(1)
            .ok_or("Release epochs exhausted")?;
        for scene in self.scenes_mut() {
            scene.unload(&origins, epoch);
        }
        self.last_release = epoch;
        Ok(())
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Pick(screen) => {
                self.selected = self
                    .camera
                    .ray(screen)
                    .and_then(|ray| crate::picking::pick(&self.scene, &ray));
            }
            Action::UnloadSources => self.unload_sources()?,
            Action::Close => {
```

### 3. `src/scene.rs`

Refuse an edit until its original editable source has been restored.

**Locate this existing block:**

```rust
            .ok_or("Object not found")?;
        object.model = model;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            .ok_or("Object not found")?;
        if object.geometry().is_none() {
            return Err("Reload editable sources before Move");
        }
        object.model = model;
```

### 4. `src/editor.rs`

Keep selection/history unchanged when a cold edit requires hydration.

**Locate this existing block:**

```rust
            Action::Delete => {
                if let Some(id) = self.selected.take() {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            Action::Delete => {
                if let Some(id) = self.selected {
                    let row = self
                        .scene
                        .objects()
                        .iter()
                        .find(|row| row.id == id)
                        .ok_or("Object not found")?;
                    if row.geometry().is_none() {
                        return Err("Reload editable sources before Delete");
                    }
                }
                if let Some(id) = self.selected.take() {
```

## <a id="checkpoint-32fj-proof"></a>21. Prove unloading preserves placed history

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 21 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/lib.rs, src/unload_test…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Weak kernel/document obse…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained display</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">cold model-history travel</text>
</svg>
</div>

**What this code does:** Observe imported kernel expiration while retained displays and placements survive Undo/Redo.

### 1. `src/lib.rs`

Verify actual source release independently of GPU drawing.

**Locate this existing block:**

```rust
mod shortcuts_tests;
pub mod viewport;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod shortcuts_tests;
#[cfg(test)]
mod unload_tests;
pub mod viewport;
```

### 2. `src/unload_tests.rs`

Optional authoring fixture. Prove old source values expire while placed drawing data and history remain available.

Create the file and type:

```rust
use crate::editor::{Action, Editor};
use std::rc::Rc;

#[test]
fn unloading_drops_imported_kernel_owners_but_keeps_display_and_history() {
    let mut editor = Editor::default();
    let url = Rc::new(crate::reload_url::ReloadUrl::new(
        "test:unload".into(),
    ));
    editor
        .apply(Action::ReplaceAt(
            crate::specimen::bytes(),
            url,
        ))
        .unwrap();
    editor.apply(Action::SelectNext).unwrap();
    let row = &editor.scene.objects()[0];
    let id = row.id;
    let guid = row.guid.clone();
    let display = Rc::clone(&row.mesh);
    let source = Rc::downgrade(row.geometry().unwrap());
    let document = Rc::downgrade(&row.source().unwrap().document);
    let initial = row.model.m;
    let view = editor.camera.uniform();
    editor
        .apply(Action::Translate([0.25, 0.0, 0.0]))
        .unwrap();
    let moved = editor.scene.objects()[0].model.m;
    let roots = editor.scenes().count();
    editor.apply(Action::UnloadSources).unwrap();
    assert!(source.upgrade().is_none() && document.upgrade().is_none());
    let row = &editor.scene.objects()[0];
    assert_eq!(row.id, id);
    assert_eq!(row.guid, guid);
    assert_eq!(row.model.m, moved);
    assert!(Rc::ptr_eq(&display, &row.mesh));
    assert_eq!(row.release_epoch(), Some(1));
    assert!(row.geometry().is_none() && row.source().is_none());
    assert_eq!(editor.scenes().count(), roots);
    assert_eq!(editor.camera.uniform(), view);
    assert!(crate::document::snapshot(&editor.scene).is_err());
    assert!(
        editor
            .apply(Action::Translate([1.0, 0.0, 0.0]))
            .is_err()
    );
    assert_eq!(editor.scenes().count(), roots);
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects()[0].model.m, initial);
    assert_eq!(
        editor.scene.objects()[0].release_epoch(),
        Some(1)
    );
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.objects()[0].model.m, moved);
    assert!(source.upgrade().is_none());
}
```

## <a id="checkpoint-32fk-command"></a>22. Unload editable sources through the command line

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 22 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/browser.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Keep the drawing and GPU…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Keep the drawing and GPU…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Keep the drawing and GPU…</text>
</svg>
</div>

**What this code does:** Keep the drawing and GPU allocations while the dock unloads eligible imported sources.

### 1. `src/browser.rs`

Offer residency changes through the real command dock. Keep unload separate from Close and from keyboard shortcuts. Retain the submitted action kind across consumption. Report success only after validated residency and renderer synchronization complete. Inspect loaded/released state without adding runtime teaching controls.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            })
        })
        .collect();
    canvas.set_attribute(
```

Replace it with:

```rust
            })
        })
        .collect();
    let residency: Vec<_> = editor
        .scene
        .objects()
        .iter()
        .map(|row| {
            (
                &row.guid,
                row.geometry().is_some(),
                row.release_epoch(),
            )
        })
        .collect();
    canvas.set_attribute(
        "data-source-residency",
        &serde_json::to_string(&residency)
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
    )?;
    canvas.set_attribute(
```

**Locate this block:**

```rust
                    Action::Replace(_) | Action::ReplaceAt(_, _)
                );
                let closing = matches!(&action, Action::Close);
                match editor.apply(action) {
                    Ok(Change::Scene) => {
                        renderer.set_scene(&editor.scene, editor.selected);
                        if closing {
                            report("Document closed.");
                            panel.result("Document closed.");
                        } else if event.type_() == "viewer-file" {
```

Replace it with:

```rust
                    Action::Replace(_) | Action::ReplaceAt(_, _)
                );
                let closing = matches!(&action, Action::Close);
                let unloading = matches!(&action, Action::UnloadSources);
                match editor.apply(action) {
                    Ok(Change::Scene) => {
                        renderer.set_scene(&editor.scene, editor.selected);
                        if unloading {
                            report("Editable sources unloaded; display retained.");
                            panel
                                .result("Editable sources unloaded; display retained.");
                        } else if closing {
                            report("Document closed.");
                            panel.result("Document closed.");
                        } else if event.type_() == "viewer-file" {
```

**Locate this block:**

```rust
                    }
                }
            } else if let Some(line) = line {
                if line == "close" {
                    request.borrow_mut().cancel();
                    Some(Action::Close)
                } else if line == "cancel open" {
```

Replace it with:

```rust
                    }
                }
            } else if let Some(line) = line {
                if line == "unload sources" {
                    Some(Action::UnloadSources)
                } else if line == "close" {
                    request.borrow_mut().cancel();
                    Some(Action::Close)
                } else if line == "cancel open" {
```

**Locate this block:**

```rust
            "Close",
            "Save",
            "Example Box",
            "Example Triangle",
```

Replace it with:

```rust
            "Close",
            "Unload Sources",
            "Save",
            "Example Box",
            "Example Triangle",
```

## <a id="checkpoint-32fl-guards"></a>23. Protect history and future reload tickets

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 23 of 23</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/editor.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Modified history origin</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">release; exhausted epoch</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/decode.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">no mutation</text>
</svg>
</div>

**What this code does:** Verify whole-origin protection, independent duplicate imports and checked release epochs.

### 1. `src/editor.rs`

Check modified history, independent imports and exhausted/reopened release issuance.

**Locate this existing block:**

```rust
    #[test]
    fn close_ends_history_and_keeps_issued_ids_and_view_settings() {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    #[test]
    fn a_modified_history_row_protects_only_its_own_import_origin() {
        use std::rc::Rc;
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        let first = Rc::new(crate::reload_url::ReloadUrl::new(
            "test:first".into(),
        ));
        editor
            .apply(Action::ReplaceAt(bytes.clone(), first))
            .unwrap();
        let origin = editor.scene.objects()[0]
            .origin()
            .unwrap()
            .id;
        let original = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        let second = Rc::new(crate::reload_url::ReloadUrl::new(
            "test:second".into(),
        ));
        editor
            .apply(Action::ImportAt(bytes, second))
            .unwrap();
        let copy = Rc::downgrade(editor.scene.objects()[3].geometry().unwrap());
        editor.apply(Action::SelectNext).unwrap();
        editor
            .apply(Action::Translate([0.25, 0.0, 0.0]))
            .unwrap();
        editor.apply(Action::Undo).unwrap();
        let saved = editor.history.scenes_mut().last().unwrap();
        assert_eq!(
            saved.objects()[0].origin().unwrap().id,
            origin
        );
        Rc::make_mut(&mut saved.objects_mut()[0].metadata).is_locked = true;
        assert!(editor.scene.objects()[0].can_unload());
        editor.apply(Action::UnloadSources).unwrap();
        assert!(original.upgrade().is_some());
        assert!(copy.upgrade().is_none());
        assert!(
            editor.scene.objects()[0]
                .geometry()
                .is_some()
        );
        assert!(
            editor.scene.objects()[3]
                .geometry()
                .is_none()
        );
        assert_eq!(editor.last_release, 1);
        assert!(editor.apply(Action::UnloadSources).is_err());
        assert_eq!(editor.last_release, 1);
    }

    #[test]
    fn release_epochs_do_not_wrap_or_reset_on_close() {
        use std::rc::Rc;
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        let url = Rc::new(crate::reload_url::ReloadUrl::new(
            "test:epoch".into(),
        ));
        editor
            .apply(Action::ReplaceAt(
                bytes.clone(),
                Rc::clone(&url),
            ))
            .unwrap();
        editor.last_release = u64::MAX;
        assert_eq!(
            editor.apply(Action::UnloadSources),
            Err("Release epochs exhausted")
        );
        assert!(
            editor.scene.objects()[0]
                .geometry()
                .is_some()
        );
        editor.last_release = 0;
        editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(
            editor.scene.objects()[0].release_epoch(),
            Some(1)
        );
        editor.apply(Action::Close).unwrap();
        editor
            .apply(Action::ImportAt(bytes, url))
            .unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(
            editor.scene.objects()[0].release_epoch(),
            Some(2)
        );
    }

    #[test]
    fn close_ends_history_and_keeps_issued_ids_and_view_settings() {
```

## Build & Run

![Recorded visual output for Lesson 3.6 (32fl-guards)](../../screenshots/journey/32fl-guards-browser.png)

```sh
npm --prefix ../session_tests run course -- dependencies 3.6
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 3.6
npm --prefix ../session_tests run course -- save 3.6
```

Continue with [3.7 · Restore exact sources and replay the captured edit](32g-keys.md).
