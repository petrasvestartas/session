# 2.1 · Buffers that describe a shared mesh

**Steps:** 2 · **Source:** `src/camera.rs` · `src/engine/gpu/render.rs` · `src/shaders/scene.wgsl`

## <a id="checkpoint-05-vertices"></a>1. Let Rust supply the corners

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-27a25a21" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-27a25a21" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-27a25a21" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-27a25a21" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-27a25a21" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">05-VERTICES · 24B STRIDE</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">#[repr(C)] Interleaved Vertex Memory Layout &amp; GPU VertexBufferLayout</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">array_stride = 24B · @location(0), @location(1)</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-27a25a21)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · CPU #[repr(C)] Vertex (24 Bytes)</text>
  <rect x="38" y="76" width="296" height="48" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <rect x="38" y="76" width="148" height="48" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="112" y="98" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">position: [f32; 3]</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="112" y="114" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#2563eb">offset 0..12 (Float32x3)</text>
  <rect x="186" y="76" width="148" height="48" fill="#fce7f3" stroke="#db2777" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="260" y="98" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">color: [f32; 3]</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="260" y="114" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#db2777">offset 12..24 (Float32x3)</text>
  <line x1="38" y1="142" x2="334" y2="142" stroke="#111111" stroke-width="1.5" marker-end="url(#arr-27a25a21)"/>
  <line x1="38" y1="134" x2="38" y2="150" stroke="#111111" stroke-width="1.5"/>
  <line x1="186" y1="134" x2="186" y2="150" stroke="#111111" stroke-width="1.5"/>
  <line x1="334" y1="134" x2="334" y2="150" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="164" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">stride = size_of::&lt;Vertex&gt;() = 24 B</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-27a25a21)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-27a25a21)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · DMA Upload &amp; Slot 0 Binding</text>
  <rect x="416" y="76" width="288" height="46" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="96" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">bytemuck::cast_slice(VERTICES)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="113" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#16a34a">create_buffer_init(BufferUsages::VERTEX)</text>
  <rect x="416" y="132" width="288" height="52" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="153" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">@location(0) pos: vec3&lt;f32&gt;</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#db2777">@location(1) color: vec3&lt;f32&gt;</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-27a25a21)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-27a25a21)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Interpolated Vertex Colors</text>
  <rect x="806" y="68" width="256" height="122" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="934,78 854,174 1014,174" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <circle cx="934" cy="78" r="5" fill="#dc2626" stroke="#111111" stroke-width="1.2"/>
  <circle cx="854" cy="174" r="5" fill="#16a34a" stroke="#111111" stroke-width="1.2"/>
  <circle cx="1014" cy="174" r="5" fill="#2563eb" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="146" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v0, v1, v2 from Rust</text>
</svg>
</div>

### 1. `src/triangle.wgsl`

**Locate this existing block:**

```wgsl
@vertex
fn vertex(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
    let corners = array<vec2<f32>, 3>(
        vec2<f32>(-0.6, -0.5),
        vec2<f32>(0.6, -0.5),
        vec2<f32>(0.0, 0.6),
    );
    return vec4<f32>(corners[index], 0.0, 1.0);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@vertex
fn vertex(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(position, 0.0, 1.0);
}
```

### 2. `src/renderer.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.draw(0..3, 0..1);
        }
```

Replace it with:

```rust
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_vertex_buffer(0, self.vertices.slice(..));
            pass.draw(0..POSITIONS.len() as u32, 0..1);
        }
```

**Locate this block:**

```rust
            device,
            queue,
            pipeline,
        }
    }
```

Replace it with:

```rust
            device,
            queue,
            pipeline,
            vertices,
        }
    }
```

**Locate this block:**

```rust
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
```

Replace it with:

```rust
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
```

**Locate this block:**

```rust
        queue: wgpu::Queue,
        format: wgpu::TextureFormat,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
```

Replace it with:

```rust
        queue: wgpu::Queue,
        format: wgpu::TextureFormat,
    ) -> Self {
        let bytes: Vec<u8> = POSITIONS
            .iter()
            .flatten()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("rectangle positions"),
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
```

**Locate this block:**

```rust
pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
}

impl Renderer {
```

Replace it with:

```rust
use wgpu::util::DeviceExt;

const POSITIONS: [[f32; 2]; 6] = [
    [-0.6, -0.5],
    [0.6, -0.5],
    [0.6, 0.6],
    [-0.6, -0.5],
    [0.6, 0.6],
    [-0.6, 0.6],
];

pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    vertices: wgpu::Buffer,
}

impl Renderer {
```

### 3. `src/browser.rs`

**Locate this existing block:**

```rust
    click.forget();
    report("Change the background. The triangle keeps its shape and colour.");
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    click.forget();
    report("Six uploaded corners make one rectangle.");
    Ok(())
```

## <a id="checkpoint-06-indices"></a>2. Share a corner between triangles

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-f3064ae5" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-f3064ae5" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-f3064ae5" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-f3064ae5" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-f3064ae5" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">06-INDICES · TOPOLOGY</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Shared Quad Vertices (4 Verts = 96B) &amp; Uint16 Triangle Index Buffer [0,1,2, 0,2,3]</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">4 verts + 6 u16 indices vs 6 dup verts</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-f3064ae5)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · Quad Topology: Shared Diagonal (v0, v2)</text>
  <polygon points="96,82 96,170 266,170" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <polygon points="96,82 266,170 266,82" fill="#dcfce7" stroke="#111111" stroke-width="1.75"/>
  <line x1="96" y1="82" x2="266" y2="170" stroke="#dc2626" stroke-width="2.5" stroke-dasharray="5 3"/>
  <circle cx="96" cy="82" r="4.5" fill="#dc2626"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="72" y="80" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">v0</text>
  <circle cx="96" cy="170" r="4.5" fill="#111111"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="72" y="176" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v1</text>
  <circle cx="266" cy="170" r="4.5" fill="#dc2626"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="278" y="176" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">v2</text>
  <circle cx="266" cy="82" r="4.5" fill="#111111"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="278" y="80" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v3</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="144" y="148" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">T0 (0,1,2)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="208" y="110" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">T1 (0,2,3)</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-f3064ae5)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-f3064ae5)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · IndexBuffer (BufferUsages::INDEX)</text>
  <rect x="418" y="78" width="284" height="44" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="424" y="84" width="132" height="32" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="490" y="104" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">[ 0,  1,  2 ]</text>
  <rect x="564" y="84" width="132" height="32" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="630" y="104" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">[ 0,  2,  3 ]</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="148" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">set_index_buffer(slice, Uint16)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="170" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">pass.draw_indexed(0..6, 0, 0..1)</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-f3064ae5)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-f3064ae5)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · 2-Triangle Indexed Quad Output</text>
  <rect x="806" y="68" width="256" height="122" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="866,84 866,174 1002,174" fill="#60a5fa" stroke="#111111" stroke-width="1.5"/>
  <polygon points="866,84 1002,174 1002,84" fill="#3b82f6" stroke="#111111" stroke-width="1.5"/>
  <line x1="866" y1="84" x2="1002" y2="174" stroke="#ffffff" stroke-width="1.2" stroke-dasharray="3 2"/>
</svg>
</div>

### 1. `src/renderer.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_vertex_buffer(0, self.vertices.slice(..));
            pass.draw(0..POSITIONS.len() as u32, 0..1);
        }
```

Replace it with:

```rust
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_vertex_buffer(0, self.vertices.slice(..));
            pass.set_index_buffer(
                self.indices.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
        }
```

**Locate this block:**

```rust
            queue,
            pipeline,
            vertices,
        }
    }
```

Replace it with:

```rust
            queue,
            pipeline,
            vertices,
            indices,
        }
    }
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
        let bytes: Vec<u8> = INDICES
            .iter()
            .flat_map(|index| index.to_ne_bytes())
            .collect();
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
    [-0.6, -0.5],
    [0.6, -0.5],
    [0.6, 0.6],
    [-0.6, -0.5],
    [0.6, 0.6],
    [-0.6, 0.6],
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
    [-0.6, 0.0],
    [0.0, -0.6],
    [0.6, 0.0],
    [0.0, 0.6],
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

### 2. `src/browser.rs`

**Locate this existing block:**

```rust
    click.forget();
    report("Six uploaded corners make one rectangle.");
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    click.forget();
    report("Four shared corners make two triangles.");
    Ok(())
```

## Build & Run

![Recorded visual output for Lesson 2.1 (06-indices)](../../screenshots/journey/06-indices-browser.png)

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j1
CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 2.1
npm --prefix ../session_tests run course -- save 2.1
```

Continue with [2.2 · Camera state and the world-to-clip transform](07-uniforms.md).
