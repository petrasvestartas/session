# 2.1 · Buffers that describe a shared mesh

**Section 2: 3D Meshes, Camera Matrices, Depth, and Ray Picking** · **Lesson 2.1** (Redraft `2.1`) · **2 checkpoints (`05-vertices` → `06-indices`)** · **Study time: 2–4 h**

> **What You Will Learn & Build:** Draw a rectangle from six positions stored in a GPU buffer. By the final checkpoint (`06-indices`), you will draw a diamond from four positions and six small index numbers.

## 1. Visual Pipeline & WebGPU Architecture (Black & White Schematics)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 220" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="212" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Lesson 2.1 Execution Flow: Buffers that describe a shared mesh</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">Concrete data pipeline you build in this lesson (2 Checkpoints: 05-vertices → 06-indices)</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="92" x2="151" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">Rust positions</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="92" x2="316" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">bytes</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="92" x2="481" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">vertex layout</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="92" x2="646" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">shader location 0</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="92" x2="811" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">two triangles</text>
    
    <rect x="18" y="162" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="181" font-size="11" font-weight="700" fill="#111111">Files You Edit in Lesson 2.1:</text>
    <text x="215" y="181" font-size="11" fill="#111111">src/triangle.wgsl  ·  src/renderer.rs  ·  src/browser.rs</text>
    <text x="32" y="198" font-size="10.5" fill="#111111">2 Checkpoints: 05-vertices → 06-indices — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
</svg>
</div>

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Chapter 2 WebGPU Pipeline: 3D Vertex/Index Buffers, Camera Uniform (mat4x4), Depth32Float & Ray Picking</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">How 3D world coordinates project onto the 2D canvas with hardware Z-buffering—and how a 2D mouse click unprojects back into a 3D ray.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    <!-- CPU Buffers -->
    <rect x="18" y="68" width="185" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="110" y="88" text-anchor="middle" font-size="12" font-weight="700" fill="#111111">1. Rust CPU Data (#[repr(C)])</text>
    <text x="28" y="108" font-size="11" fill="#111111">• vertices: Vec&lt;[f32; 3]&gt;</text>
    <text x="28" y="125" font-size="11" fill="#111111">• indices:  Vec&lt;u32&gt; (shared!)</text>
    <text x="28" y="142" font-size="11" fill="#111111">• camera.view_proj: [[f32;4];4]</text>
    <text x="28" y="162" font-size="10.5" font-weight="600" fill="#111111">bytemuck::cast_slice(&amp;data)</text>
    <text x="28" y="180" font-size="10.5" fill="#111111">• Undo / Redo Snapshot Stack</text>
    <text x="28" y="197" font-size="10.5" fill="#111111">• Orbit / Pan / Zoom Camera</text>

    <line x1="203" y1="115" x2="233" y2="115" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <!-- Vertex Shader -->
    <rect x="235" y="68" width="205" height="90" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="337" y="87" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">2. @vertex Shader Stage</text>
    <text x="245" y="105" font-size="10.5" fill="#111111">@location(0) pos: vec3&lt;f32&gt;</text>
    <text x="245" y="122" font-size="10.5" fill="#111111">@group(0) @binding(0) view_proj</text>
    <text x="245" y="142" font-size="11" font-weight="700" fill="#111111">clip = view_proj * vec4(pos, 1.0)</text>

    <line x1="440" y1="115" x2="470" y2="115" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <!-- Raster + Depth32Float -->
    <rect x="472" y="68" width="185" height="90" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="564" y="87" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">3. Depth32Float Z-Test</text>
    <text x="482" y="105" font-size="10.5" fill="#111111">NDC = clip.xyz / clip.w</text>
    <text x="482" y="122" font-size="10.5" font-weight="600" fill="#111111">if z_fragment &lt;= z_buffer[x,y]:</text>
    <text x="482" y="139" font-size="10.5" fill="#111111">  write pixel &amp; update z_buffer</text>

    <line x1="657" y1="115" x2="685" y2="115" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <!-- Screen & Reverse Ray Pick -->
    <rect x="687" y="68" width="133" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="753" y="87" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">4. Canvas &amp; Pick</text>
    <text x="696" y="106" font-size="10.5" fill="#111111">• Nearer mesh</text>
    <text x="696" y="120" font-size="10.5" fill="#111111">  occludes rear</text>
    <text x="696" y="142" font-size="10.5" font-weight="600" fill="#111111">• Click (px, py):</text>
    <text x="696" y="157" font-size="10" fill="#111111">  inv_view_proj</text>
    <text x="696" y="172" font-size="10" fill="#111111">  -&gt; 3D Ray(O, D)</text>
    <text x="696" y="187" font-size="10" fill="#111111">  -&gt; min t hit ID!</text>

    <!-- Bottom Reverse Arrow for Ray Picking -->
    <rect x="235" y="168" width="422" height="45" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="446" y="187" text-anchor="middle" font-size="11" font-weight="600" fill="#111111">CPU Ray Picking (Inverse Path): Screen (px, py) → NDC (-1..+1) → inv(view_proj) → World Ray (O + t·D)</text>
    <text x="446" y="203" text-anchor="middle" font-size="10.5" fill="#111111">Tests every triangle via Möller–Trumbore; smallest positive t wins (matching hardware Depth32Float occlusion!)</text>
    
</svg>
</div>

## 2. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

Before writing code, review how the three layers in this chapter talk to each other:

| Layer | Construct | Plain-English Explanation for Students |
| :--- | :--- | :--- |
| **Rust (`bytemuck`)** | `#[repr(C)]` + `Pod` / `Zeroable` | Guarantees Rust structs (`Vertex`, `CameraUniform`) have exact C/GPU byte layout so `bytemuck::cast_slice` uploads raw bytes to `wgpu::Buffer` with zero copying. |
| **WGSL Shader** | `@group(0) @binding(0) var<uniform> view_proj: mat4x4<f32>` | Transforms 3D world vertices into 4D homogeneous clip coordinates (`view_proj * vec4<f32>(pos, 1.0)`). |
| **WebGPU / Math** | `Depth32Float` + Inverse Ray `inv(view_proj)` | Forward pass uses a 32-bit float Z-buffer (`LessEqual`) so nearer triangles occlude farther ones; clicking unprojects `(px, py)` back into a 3D ray to pick the closest hit `t`. |

**Authoritative production source (`session_viewer/src`):** `session_viewer/src`

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`05-vertices`](#checkpoint-05-vertices) | Draw a rectangle from six positions stored in a GPU buffer. | `src/triangle.wgsl`, `src/renderer.rs`, `src/browser.rs` | `Rust positions → bytes → vertex buffer → vertex layout → shader location 0 → two triangles` |
| **2** | [`06-indices`](#checkpoint-06-indices) | Draw a diamond from four positions and six small index numbers. | `src/renderer.rs`, `src/browser.rs` | `Six indices → four shared positions → vertex shader → two joined triangles` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Read the **Concept Walkthrough** and **Self-Check Q&A** first, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-05-vertices"></a>

### Step 1 of 2 · Let Rust supply the corners (`05-vertices`)

- **Goal:** Draw a rectangle from six positions stored in a GPU buffer.
- **Data Flow Trace:** `Rust positions → bytes → vertex buffer → vertex layout → shader location 0 → two triangles`
- **Files You Touch:** `src/triangle.wgsl` (*Replace the shader. It now receives one position per invocation instead of looking up a hard-coded corner.*) · `src/renderer.rs` (*Define six positions and upload their vertex buffer.*) · `src/browser.rs` (*Report that the uploaded corners form a rectangle.*)

Move the corner positions out of WGSL and into a Rust vertex buffer. Six coordinate pairs draw two triangles forming a rectangle. Shared corners are duplicated here; the next lesson removes that duplication.

Describe the buffer layout explicitly: two `f32` values, eight bytes per vertex, matching shader `@location(0)`. Buffer slot and shader location are separate settings even though both are zero here.

`iter().flatten()` visits each coordinate, `flat_map` converts it to bytes, and `collect` owns the resulting byte vector. Importing the `DeviceExt` trait enables `create_buffer_init`. It creates and fills GPU storage; the temporary CPU byte vector can then be dropped.

> **Self-Check Question:** *Who owns the positions, and how does the shader know where each pair of numbers begins?*  
> **Answer:** Rust supplies the positions and uploads their bytes. The renderer retains the GPU buffer. Its vertex layout says that a vertex occupies eight bytes and location 0 contains two f32 values starting at byte zero. The shader reads that location; it no longer chooses corners from its own array.

- **What You Should See:** The GPU draws a rectangle from six buffered positions. Change one corner in the Rust vertex array, save, and check the changed outline. Restore it.
- **Hands-On Lab Experiment:** Predict which triangle changes if you move only the first position to [-0.9, -0.5]. Try it. The other occurrence of that corner stays put, opening a mismatch along the shared edge. Restore the value. This is the duplication that indices will remove.

<details>
<summary>Implementation 05-vertices: Let Rust supply the corners</summary>

Draw a rectangle from six positions stored in a GPU buffer.

Data flow: Rust positions → bytes → vertex buffer → vertex layout → shader location 0 → two triangles.

### 1. `src/triangle.wgsl`

Replace the shader. It now receives one position per invocation instead of looking up a hard-coded corner.

<details>
<summary>Locate the existing block</summary>

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

@fragment
fn fragment() -> @location(0) vec4<f32> {
    return vec4<f32>(0.9, 0.25, 0.45, 1.0);
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
--8<-- "journey/code/05-vertices-01.wgsl"
```

### 2. `src/renderer.rs`

Define six positions and upload their vertex buffer. Describe two f32 coordinates at each eight-byte vertex. Retain the uploaded vertex buffer in Renderer. Bind the vertex buffer and draw all six positions.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Self {
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
                buffers: &[],
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
        Self { device, queue, pipeline }
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
            pass.draw(0..3, 0..1);
        }
        self.queue.submit([encoder.finish()]);
    }
}
```

</details>

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

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/05-vertices-fullscreen-2.rs"
```

```text
--8<-- "journey/code/05-vertices-fullscreen-3.rs"
```

```text
--8<-- "journey/code/05-vertices-fullscreen-4.rs"
```

```text
--8<-- "journey/code/05-vertices-fullscreen-5.rs"
```

</details>

### 3. `src/browser.rs`

Report that the uploaded corners form a rectangle.

<details>
<summary>Locate the existing block</summary>

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Change the background. The triangle keeps its shape and colour.");
    Ok(())
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/05-vertices-window-1.rs"
```

</details>

<a id="checkpoint-06-indices"></a>

### Step 2 of 2 · Share a corner between triangles (`06-indices`)

- **Goal:** Draw a diamond from four positions and six small index numbers.
- **Data Flow Trace:** `Six indices → four shared positions → vertex shader → two joined triangles`
- **Files You Touch:** `src/renderer.rs` (*Define four diamond corners and six indices; retain an index buffer.*) · `src/browser.rs` (*Report that four shared corners form two triangles.*)

Store four corner positions and connect them with indices: `0, 1, 2` and `0, 2, 3`. The two triangles now share corners 0 and 2.

The vertex buffer answers where a corner is; the index buffer answers which corners form a triangle. Changing a shared position changes both triangles.

These indices are `u16`, so bind the index buffer as `Uint16`. Each index occupies two bytes; each position still occupies eight. `draw_indexed` chooses index entries, adds its base-vertex value, then draws the requested instances. The shader keeps its previous input layout.

> **Self-Check Question:** *Why do we draw six indices when there are only four positions?*  
> **Answer:** Each triangle needs three corner references. The first triangle reads positions 0, 1 and 2; the second reads 0, 2 and 3. Positions 0 and 2 are shared. Six index entries describe two triangles while only four coordinate pairs are stored.

- **What You Should See:** A diamond is drawn from four vertices. Follow the six indices into those vertices: they must describe two triangles that share the middle edge.
- **Hands-On Lab Experiment:** Move position 0 from [-0.6, 0.0] to [-0.9, 0.0]. Predict which triangles change. Both should move their left corner together, leaving no crack. Then restore the coordinate and temporarily draw only 0..3 indices: only the lower triangle should remain. Restore 0..6 before comparing.

<details>
<summary>Implementation 06-indices: Share a corner between triangles</summary>

Draw a diamond from four positions and six small index numbers.

Data flow: Six indices → four shared positions → vertex shader → two joined triangles.

### 1. `src/renderer.rs`

Define four diamond corners and six indices; retain an index buffer. Upload the indices as native-endian u16 bytes. Retain the index buffer in Renderer. Bind the u16 index buffer and draw its six entries.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_vertex_buffer(0, self.vertices.slice(..));
            pass.draw(0..POSITIONS.len() as u32, 0..1);
        }
        self.queue.submit([encoder.finish()]);
    }
```

</details>

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

<details>
<summary>Locate this block</summary>

```rust
            multiview_mask: None,
            cache: None,
        });
        Self { device, queue, pipeline, vertices }
    }

    pub fn draw(&self, view: &wgpu::TextureView, background: &crate::background::Background) {
```

</details>

Replace it with:

```rust
            multiview_mask: None,
            cache: None,
        });
        Self { device, queue, pipeline, vertices, indices }
    }

    pub fn draw(&self, view: &wgpu::TextureView, background: &crate::background::Background) {
```

<details>
<summary>Locate this block</summary>

```rust
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
```

</details>

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

<details>
<summary>Locate this block</summary>

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

</details>

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

<details>
<summary>Complete updated block reference</summary>

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

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/06-indices-fullscreen-2.rs"
```

```text
--8<-- "journey/code/06-indices-fullscreen-3.rs"
```

```text
--8<-- "journey/code/06-indices-fullscreen-4.rs"
```

```text
--8<-- "journey/code/06-indices-fullscreen-5.rs"
```

</details>

### 2. `src/browser.rs`

Report that four shared corners form two triangles.

<details>
<summary>Locate the existing block</summary>

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Six uploaded corners make one rectangle.");
    Ok(())
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/06-indices-window-1.rs"
```

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 06-indices
npm --prefix ../session_tests run course -- check 06-indices
npm --prefix ../session_tests run course -- save 06-indices
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j1
CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

A diamond is drawn from four vertices. Follow the six indices into those vertices: they must describe two triangles that share the middle edge.

- **Final Lesson Experiment:** Move position 0 from [-0.6, 0.0] to [-0.9, 0.0]. Predict which triangles change. Both should move their left corner together, leaving no crack. Then restore the coordinate and temporarily draw only 0..3 indices: only the lower triangle should remain. Restore 0..6 before comparing.

Continue with [Camera state and the world-to-clip transform](07-uniforms.md).
