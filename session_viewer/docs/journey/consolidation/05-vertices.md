# 2.1 · Buffers that describe a shared mesh

**Section 2: 3D Meshes, Camera Matrices, Depth, and Ray Picking** · **Checkpoints:** `05-vertices` → `06-indices` (2 steps) · **Production source:** `session_viewer/src`

> **Goal:** Draw a diamond from four positions and six small index numbers.

## 1. Visual WebGPU Schematic

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="220" viewBox="0 0 840 220" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="216" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="20" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="135" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">1. Indexed Mesh</text>
  <polygon points="65,68 185,68 65,148" fill="url(#hatch)" stroke="#111111" stroke-width="1.75"/>
  <polygon points="185,68 185,148 65,148" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <circle cx="65" cy="68" r="4" fill="#111111"/><text x="44" y="68" font-family="monospace" font-size="10" font-weight="700" fill="#111111">v0</text>
  <circle cx="185" cy="68" r="4" fill="#111111"/><text x="193" y="68" font-family="monospace" font-size="10" font-weight="700" fill="#111111">v1</text>
  <circle cx="65" cy="148" r="4" fill="#111111"/><text x="44" y="152" font-family="monospace" font-size="10" font-weight="700" fill="#111111">v2</text>
  <circle cx="185" cy="148" r="4" fill="#111111"/><text x="193" y="152" font-family="monospace" font-size="10" font-weight="700" fill="#111111">v3</text>
  <text x="135" y="176" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">idx: [0,1,2, 2,1,3]</text>
  <text x="135" y="191" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">4 Verts · 2 Triangles</text>
  <line x1="250" y1="110" x2="298" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="304" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="419" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">2. view_proj * pos</text>
  <circle cx="332" cy="112" r="5" fill="#111111"/>
  <text x="332" y="132" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Eye</text>
  <line x1="332" y1="112" x2="495" y2="62" stroke="#111111" stroke-width="1.25" stroke-dasharray="3 3"/>
  <line x1="332" y1="112" x2="495" y2="158" stroke="#111111" stroke-width="1.25" stroke-dasharray="3 3"/>
  <rect x="384" y="86" width="24" height="50" rx="2" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="462" y="64" width="38" height="92" rx="3" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="396" y="80" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">Near</text>
  <text x="481" y="58" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">Far</text>
  <text x="419" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">mat4x4&lt;f32&gt; Uniform</text>
  <line x1="534" y1="110" x2="582" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="588" y="20" width="232" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="704" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">3. Z-Depth &amp; Ray Pick</text>
  <rect x="688" y="68" width="84" height="62" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5" stroke-dasharray="3 3"/>
  <rect x="642" y="84" width="84" height="62" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="2"/>
  <line x1="604" y1="142" x2="676" y2="108" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <circle cx="680" cy="106" r="4" fill="#111111"/>
  <text x="644" y="78" font-family="monospace" font-size="9" font-weight="700" fill="#111111">z=0.2 (Front)</text>
  <text x="730" y="63" font-family="monospace" font-size="9" fill="#111111">z=0.8</text>
  <text x="704" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">Depth32Float + Ray Hit</text>
</svg>

| Layer | Construct | Role |
| :--- | :--- | :--- |
| **Rust (`bytemuck`)** | `#[repr(C)]` + `cast_slice` | Zero-copy upload of `Vertex` & `CameraUniform` bytes to GPU buffers. |
| **WGSL** | `view_proj * vec4(pos, 1.0)` | Projects 3D world vertices into 4D clip space. |
| **WebGPU** | `Depth32Float` + Ray Unproject | Z-buffer occludes back faces; `inv(view_proj)` casts 3D pick rays. |

![Recorded visual output for Lesson 2.1 (06-indices)](../../screenshots/journey/06-indices-browser.png)

---

## 2. Complete Code Overview (All Steps Open)

<a id="checkpoint-05-vertices"></a>

### Step 1 of 2 · Let Rust supply the corners (`05-vertices`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/triangle.wgsl, src/re…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">vertex layout</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">two triangles</text>
</svg>

Move the corner positions out of WGSL and into a Rust vertex buffer. Six coordinate pairs draw two triangles forming a rectangle. Shared corners are duplicated here; the next lesson removes that duplication.

Describe the buffer layout explicitly: two `f32` values, eight bytes per vertex, matching shader `@location(0)`. Buffer slot and shader location are separate settings even though both are zero here.

> **Check:** *Who owns the positions, and how does the shader know where each pair of numbers begins?* — Rust supplies the positions and uploads their bytes. The renderer retains the GPU buffer. Its vertex layout says that a vertex occupies eight bytes and location 0 contains two f32 values starting at byte zero. The shader reads that location; it no longer chooses corners from its own array.

### 1. `src/triangle.wgsl`

Replace the shader. It now receives one position per invocation instead of looking up a hard-coded corner.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
--8<-- "journey/code/05-vertices-01.wgsl"
```

### 2. `src/renderer.rs`

Define six positions and upload their vertex buffer. Describe two f32 coordinates at each eight-byte vertex. Retain the uploaded vertex buffer in Renderer. Bind the vertex buffer and draw all six positions.

Edit the existing block to match this code; keep unchanged lines:

```rust
use wgpu::util::DeviceExt;

const POSITIONS: [[f32; 2]; 6] = [
    [-0.6, -0.5], [0.6, -0.5], [0.6, 0.6],
    [-0.6, -0.5], [0.6, 0.6], [-0.6, 0.6],
];

pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    vertices: wgpu::Buffer,
}

impl Renderer {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let bytes: Vec<u8> = POSITIONS.iter().flatten()
            .flat_map(|value| value.to_ne_bytes()).collect();
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("rectangle positions"),
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
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
                    array_stride: 8,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        Self { device, queue, pipeline, vertices }
    }

    pub fn draw(&self, view: &wgpu::TextureView, background: &crate::background::Background) {
        let [r, g, b] = background.rgb();
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r, g, b, a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_vertex_buffer(0, self.vertices.slice(..));
            pass.draw(0..POSITIONS.len() as u32, 0..1);
        }
        self.queue.submit([encoder.finish()]);
    }
}
```

### 3. `src/browser.rs`

Report that the uploaded corners form a rectangle.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/05-vertices-window-1.rs"
```

**Result after `05-vertices`:** The GPU draws a rectangle from six buffered positions. Change one corner in the Rust vertex array, save, and check the changed outline. Restore it.

---

<a id="checkpoint-06-indices"></a>

### Step 2 of 2 · Share a corner between triangles (`06-indices`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/renderer.rs, src/brow…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">vertex shader</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">two joined triangles</text>
</svg>

Store four corner positions and connect them with indices: `0, 1, 2` and `0, 2, 3`. The two triangles now share corners 0 and 2.

The vertex buffer answers where a corner is; the index buffer answers which corners form a triangle. Changing a shared position changes both triangles.

> **Check:** *Why do we draw six indices when there are only four positions?* — Each triangle needs three corner references. The first triangle reads positions 0, 1 and 2; the second reads 0, 2 and 3. Positions 0 and 2 are shared. Six index entries describe two triangles while only four coordinate pairs are stored.

### 1. `src/renderer.rs`

Define four diamond corners and six indices; retain an index buffer. Upload the indices as native-endian u16 bytes. Retain the index buffer in Renderer. Bind the u16 index buffer and draw its six entries.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_vertex_buffer(0, self.vertices.slice(..));
            pass.draw(0..POSITIONS.len() as u32, 0..1);
        }
        self.queue.submit([encoder.finish()]);
    }
```

Replace it with:

```rust
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_vertex_buffer(0, self.vertices.slice(..));
            pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint16);
            pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
        }
        self.queue.submit([encoder.finish()]);
    }
```

**Locate this block:**

```rust
            multiview_mask: None,
            cache: None,
        });
        Self { device, queue, pipeline, vertices }
    }

    pub fn draw(&self, view: &wgpu::TextureView, background: &crate::background::Background) {
```

Replace it with:

```rust
            multiview_mask: None,
            cache: None,
        });
        Self { device, queue, pipeline, vertices, indices }
    }

    pub fn draw(&self, view: &wgpu::TextureView, background: &crate::background::Background) {
```

**Locate this block:**

```rust
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
```

Replace it with:

```rust
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let bytes: Vec<u8> = INDICES.iter().flat_map(|index| index.to_ne_bytes()).collect();
        let indices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("diamond indices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::INDEX,
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
```

**Locate this block:**

```rust
use wgpu::util::DeviceExt;

const POSITIONS: [[f32; 2]; 6] = [
    [-0.6, -0.5], [0.6, -0.5], [0.6, 0.6],
    [-0.6, -0.5], [0.6, 0.6], [-0.6, 0.6],
];

pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    vertices: wgpu::Buffer,
}

impl Renderer {
```

Replace it with:

```rust
use wgpu::util::DeviceExt;

const POSITIONS: [[f32; 2]; 4] = [
    [-0.6, 0.0], [0.0, -0.6], [0.6, 0.0], [0.0, 0.6],
];
const INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
}

impl Renderer {
```

**Complete updated block reference:**

```rust
use wgpu::util::DeviceExt;

const POSITIONS: [[f32; 2]; 4] = [
    [-0.6, 0.0], [0.0, -0.6], [0.6, 0.0], [0.0, 0.6],
];
const INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
}

impl Renderer {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let bytes: Vec<u8> = POSITIONS.iter().flatten()
            .flat_map(|value| value.to_ne_bytes()).collect();
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("rectangle positions"),
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let bytes: Vec<u8> = INDICES.iter().flat_map(|index| index.to_ne_bytes()).collect();
        let indices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("diamond indices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::INDEX,
        });
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
                    array_stride: 8,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        Self { device, queue, pipeline, vertices, indices }
    }

    pub fn draw(&self, view: &wgpu::TextureView, background: &crate::background::Background) {
        let [r, g, b] = background.rgb();
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r, g, b, a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_vertex_buffer(0, self.vertices.slice(..));
            pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint16);
            pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
        }
        self.queue.submit([encoder.finish()]);
    }
}
```

### 2. `src/browser.rs`

Report that four shared corners form two triangles.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/06-indices-window-1.rs"
```

**Result after `06-indices`:** A diamond is drawn from four vertices. Follow the six indices into those vertices: they must describe two triangles that share the middle edge.

---

## 3. Build, Verify & Save

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j1
CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 2.1
npm --prefix ../session_tests run course -- save 2.1
```

Open `http://127.0.0.1:8780/` to inspect the live WebGPU canvas, or use [checkpoint recovery](../recovery.md).

Continue with [2.2 · Camera state and the world-to-clip transform](07-uniforms.md).
