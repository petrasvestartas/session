# 2.1 · Buffers that describe a shared mesh

**Goal:** Draw a diamond from four positions and six small index numbers. · **Steps:** 2 · **Source:** `src/camera.rs,src/engine/gpu/render.rs,src/shaders/scene.wgsl`

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">6 Duplicate Vertices (2 Triangles)</text>
  <polygon points="74,60 174,60 74,142" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <polygon points="196,68 196,150 96,150" fill="#fef9c3" stroke="#d97706" stroke-width="1.75"/>
  <circle cx="74" cy="60" r="4" fill="#111111"/>
  <circle cx="174" cy="60" r="4" fill="#111111"/>
  <circle cx="74" cy="142" r="4" fill="#111111"/>
  <circle cx="196" cy="68" r="4" fill="#111111"/>
  <circle cx="196" cy="150" r="4" fill="#111111"/>
  <circle cx="96" cy="150" r="4" fill="#111111"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="268" y="96" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">6 verts</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="268" y="114" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#dc2626">2 shared</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">4 Unique Vertices + Index [0,1,2, 0,2,3]</text>
  <polygon points="560,48 648,104 560,160 472,104" fill="#dcfce7" stroke="#16a34a" stroke-width="2"/>
  <line x1="560" y1="48" x2="560" y2="160" stroke="#111111" stroke-width="1.75" stroke-dasharray="4 3"/>
  <circle cx="560" cy="48" r="4.5" fill="#dc2626"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="580" y="52" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v0</text>
  <circle cx="648" cy="104" r="4.5" fill="#16a34a"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="664" y="108" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v1</text>
  <circle cx="560" cy="160" r="4.5" fill="#2563eb"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="580" y="164" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v2</text>
  <circle cx="472" cy="104" r="4.5" fill="#d97706"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="452" y="108" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v3</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Viewer Output (draw_indexed 0..6)</text>
  <rect x="814" y="44" width="240" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="934,54 1004,103 934,152 864,103" fill="#ed5c87" stroke="#111111" stroke-width="2"/>
</svg>
</div>

## <a id="checkpoint-05-vertices"></a>1. Let Rust supply the corners

**What this code does:** Draw a rectangle from six positions stored in a GPU buffer.

### 1. `src/triangle.wgsl`

Replace the shader. It now receives one position per invocation instead of looking up a hard-coded corner.

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

Define six positions and upload their vertex buffer. Describe two f32 coordinates at each eight-byte vertex. Retain the uploaded vertex buffer in Renderer. Bind the vertex buffer and draw all six positions.

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

Report that the uploaded corners form a rectangle.

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

**What this code does:** Draw a diamond from four positions and six small index numbers.

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

Report that four shared corners form two triangles.

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

![Recorded visual output for Lesson 2.1 (06-indices)](../../../screenshots/journey/06-indices-browser.png)

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j1
CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 2.1
npm --prefix ../session_tests run course -- save 2.1
```

Continue with [2.2 · Camera state and the world-to-clip transform](../../consolidation/07-uniforms.md).
