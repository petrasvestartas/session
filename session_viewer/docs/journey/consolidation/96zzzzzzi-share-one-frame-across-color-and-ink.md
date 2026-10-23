# 22.2 · Contact fills and original face queries

**Goal:** Select the actual original face beneath a readonly contact, without inventing an editable face for its display polygon. · **Steps:** 5 · **Source:** [engine/gpu/render.rs](../../../src/engine/gpu/render.rs) · [engine/gpu/pass.rs](../../../src/engine/gpu/pass.rs) · [engine/gpu/targets.rs](../../../src/engine/gpu/targets.rs) · [engine/gpu/frame.rs](../../../src/engine/gpu/frame.rs) · [engine/gpu/view.rs](../../../src/engine/gpu/view.rs) · [engine/pipelines/mod.rs](../../../src/engine/pipelines/mod.rs) · [shaders/triangle.wgsl](../../../src/shaders/triangle.wgsl) · [shaders/ribbon.wgsl](../../../src/shaders/ribbon.wgsl) · [shaders/vector.wgsl](../../../src/shaders/vector.wgsl) · [shaders/ink_visibility.wgsl](../../../src/shaders/ink_visibility.wgsl) · [app/scene.rs](../../../src/app/scene.rs) (`src/shaders/scene.wgsl,src/engine/gpu/msaa.rs,src/engine/gpu/render.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">4× MSAA Subpixel Samples (s0..s3) Inside Pixel Square</text>
  <rect x="108" y="50" width="108" height="104" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <line x1="116" y1="146" x2="208" y2="56" stroke="#2563eb" stroke-width="2.5"/>
  <polygon points="116,146 208,56 216,56 216,154 116,154" fill="#dbeafe" fill-opacity="0.65"/>
  <circle cx="142" cy="76" r="5" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <circle cx="188" cy="68" r="5" fill="#2563eb" stroke="#111111" stroke-width="1.75"/>
  <circle cx="136" cy="124" r="5" fill="#2563eb" stroke="#111111" stroke-width="1.75"/>
  <circle cx="182" cy="116" r="5" fill="#2563eb" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="252" y="106" font-family="monospace" font-size="10" font-weight="700" fill="#111111">cov = 3/4</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Key + Fill + Rim Rig + Ground Plane Major/Minor Grid</text>
  <polygon points="432,140 492,98 688,98 628,140" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="462" y1="119" x2="658" y2="119" stroke="#dc2626" stroke-width="1.75"/>
  <line x1="560" y1="98" x2="500" y2="140" stroke="#16a34a" stroke-width="1.75"/>
  <polygon points="520,122 560,62 600,122" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <ellipse cx="560" cy="126" rx="34" ry="8" fill="#64748b" fill-opacity="0.45"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Anti-Aliased CAD Viewport + Ground Grid &amp; Contact Shadow</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="826,144 876,104 1042,104 992,144" fill="#f8fafc" stroke="#cbd5e1" stroke-width="1.2"/>
  <ellipse cx="934" cy="130" rx="36" ry="8" fill="#94a3b8" fill-opacity="0.5"/>
  <polygon points="894,126 934,58 974,126" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
</svg>
</div>

Layer original contact polygons and boundaries using the same current frame and source ownership as ordinary ink.

## Ownership and data flow

Contact geometry remains an original semantic feature, with fill and boundary layers derived from its source rather than arbitrary depth nudges. Shared frame metadata reaches colour and integer/query paths; contact overlays must not steal the underlying original face address. Preserve authored contact colour, opaque/unlit semantics and selected-owner behavior while retaining original Element ancestry.

## The maintained implementation

app/walk/mod.rs and mesh_ink.rs prepare maintained original features; src/shaders/triangle.wgsl detects CONTACT_ALPHA and applies LAYER_CONTACT through depth_layer. engine/gpu/faces.rs retains actual source-face IDs and render.rs orders highlight/contact/ink layers. The interim course contact lane remains staged wherever it differs from these maintained records.

All production paths above are relative to `session_viewer/src`. The implementation below preserves the runnable teaching sequence. Early `src/renderer.rs`, `src/browser.rs` and local WGSL are staged prerequisites; production drawing remains in `engine/gpu`, `engine/pipelines` and `src/shaders`. Later rendering lessons must converge to those maintained paths.

Production source reference: [engine/gpu/render.rs](../../../src/engine/gpu/render.rs), [engine/gpu/pass.rs](../../../src/engine/gpu/pass.rs), [engine/gpu/targets.rs](../../../src/engine/gpu/targets.rs), [engine/gpu/frame.rs](../../../src/engine/gpu/frame.rs), [engine/gpu/view.rs](../../../src/engine/gpu/view.rs), [engine/pipelines/mod.rs](../../../src/engine/pipelines/mod.rs), [shaders/triangle.wgsl](../../../src/shaders/triangle.wgsl), [shaders/ribbon.wgsl](../../../src/shaders/ribbon.wgsl), [shaders/vector.wgsl](../../../src/shaders/vector.wgsl), [shaders/ink_visibility.wgsl](../../../src/shaders/ink_visibility.wgsl), [app/scene.rs](../../../src/app/scene.rs), [build.rs](../../../build.rs).

## <a id="checkpoint-96zzzzzzi-share-one-frame-across-color-and-ink"></a>1. Share one frame across color and ink

**What this code does:** Remove repeated ink-frame buffers and updates while keeping one explicit surface/ink layout and independent request-specific picking.

### 1. `src/surface_view.rs`

Give existing ink shaders their unchanged viewport prefix, followed by surface settings and the existing kernel-derived eye.

**Locate this existing block:**

```rust
pub const BYTES: u64 = 96;
pub fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    crate::gpu_layout::uniform(device, BYTES)
}
pub fn bytes(matrix: [f32; 16], display: [f32; 4]) -> [u8; 96] {
    let frame = session_rust::Xform::from_matrix(matrix.map(f64::from));
    let eye = frame.eye();
    let mut values = [0.; 24];
    values[..16].copy_from_slice(&matrix);
    values[16..20].copy_from_slice(&display);
    values[20..].copy_from_slice(&[
        eye[0] as f32,
        eye[1] as f32,
        eye[2] as f32,
        frame.ortho_half_height() as f32,
    ]);
    let mut bytes = [0; 96];
    for (out, value) in bytes.chunks_exact_mut(4).zip(values) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub const BYTES: u64 = 112;
pub fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    crate::gpu_layout::uniform(device, BYTES)
}
pub fn bytes(
    matrix: [f32; 16],
    display: [f32; 4],
    size: [u32; 2],
    density: f32,
) -> [u8; 112] {
    let frame = session_rust::Xform::from_matrix(matrix.map(f64::from));
    let eye = frame.eye();
    let mut values = [0.; 28];
    values[..16].copy_from_slice(&matrix);
    values[16..20].copy_from_slice(&[
        size[0] as f32,
        size[1] as f32,
        density,
        display[1],
    ]);
    values[20..24].copy_from_slice(&display);
    values[24..].copy_from_slice(&[
        eye[0] as f32,
        eye[1] as f32,
        eye[2] as f32,
        frame.ortho_half_height() as f32,
    ]);
    let mut bytes = [0; 112];
    for (out, value) in bytes.chunks_exact_mut(4).zip(values) {
```

### 2. `src/triangle.wgsl`

Read surface settings after the shared ink prefix.

**Locate this existing block:**

```wgsl
struct ViewSettings { transform: mat4x4<f32>, display: vec4<f32>, eye:vec4<f32>, }
@group(0) @binding(0) var<uniform> view: ViewSettings;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct ViewSettings { transform: mat4x4<f32>, viewport:vec4<f32>, display: vec4<f32>, eye:vec4<f32>, }
@group(0) @binding(0) var<uniform> view: ViewSettings;
```

### 3. `src/renderer.rs`

Encode the correct full physical viewport and density once for this frame owner.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            },
            f32::from(options[1]),
        ];
        let bytes = crate::surface_view::bytes(*transform, display);
        self.queue
```

Replace it with:

```rust
            },
            f32::from(options[1]),
        ];
        let bytes =
            crate::surface_view::bytes(*transform, display, self.size, self.density);
        self.queue
```

**Locate this block:**

```rust
            );
            let bytes = crate::surface_view::bytes(transform, [0.; 4]);
            self.queue
                .write_buffer(&self.uniform, 0, &bytes);
            let tables = {
```

Replace it with:

```rust
            );
            let bytes =
                crate::surface_view::bytes(transform, [0.; 4], self.size, self.density);
            self.queue
                .write_buffer(&self.uniform, 0, &bytes);
            let tables = {
```

### 4. `src/pick_gpu.rs`

Encode the correct full physical viewport and density once for this frame owner.

**Locate this existing block:**

```rust
        image.window = window;
        let bytes = crate::surface_view::bytes(request.transform, [0.; 4]);
        queue.write_buffer(&self.view, 0, &bytes);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        image.window = window;
        let bytes = crate::surface_view::bytes(
            request.transform,
            [0.; 4],
            window.full,
            request.density,
        );
        queue.write_buffer(&self.view, 0, &bytes);
```

### 5. `src/stroke_gpu.rs`

Remove duplicated per-lane presentation state. Borrow a frame group instead of owning another uniform. The renderer and request owners perform the only frame writes. Borrow the color frame at draw time. Use the same borrowed group in the finite path. Use the borrowed frame in the base ink path. Borrow the actual request frame for original IDs. Keep the base integer path on the request frame. Pass the request frame through finite picking. Share the captured frame with finite original IDs. Retain the same request in the base fallback. Use the same explicit layout even when a shader reads only its prefix. Remove the redundant buffer and bind-group allocations. Keep settings in their existing renderer owner. Keep only lane geometry, pipelines and finite bindings.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            pick_pipeline,
            ids: None,
            id_data: Vec::new(),
            uniform,
            group,
            vertices: None,
```

Replace it with:

```rust
            pick_pipeline,
            ids: None,
            id_data: Vec::new(),
            vertices: None,
```

**Locate this block:**

```rust
                cache: None,
            })
        });
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("stroke view"),
            size: 80,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("stroke view"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        Self {
            opacity: std::cell::Cell::new(1.),
            flags: None,
            flag_data: Vec::new(),
            finite: None,
```

Replace it with:

```rust
                cache: None,
            })
        });
        Self {
            flags: None,
            flag_data: Vec::new(),
            finite: None,
```

**Locate this block:**

```rust
        pickable: bool,
        physical: bool,
    ) -> Self {
        let view_layout = crate::gpu_layout::uniform(device, 80);
        let crop_layout = crate::gpu_layout::uniform(device, 80);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ink settings"),
```

Replace it with:

```rust
        pickable: bool,
        physical: bool,
    ) -> Self {
        let view_layout = crate::surface_view::layout(device);
        let crop_layout = crate::gpu_layout::uniform(device, 80);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ink settings"),
```

**Locate this block:**

```rust
                depth,
            )
        } else {
            self.pick(pass, crop)
        }
    }
    pub fn pick_bytes(&self) -> u64 {
```

Replace it with:

```rust
                depth,
            )
        } else {
            self.pick(pass, view, crop)
        }
    }
    pub fn pick_bytes(&self) -> u64 {
```

**Locate this block:**

```rust
            ink.pick(
                device,
                pass,
                &self.group,
                crop,
                vertices,
                ids,
```

Replace it with:

```rust
            ink.pick(
                device,
                pass,
                view,
                crop,
                vertices,
                ids,
```

**Locate this block:**

```rust
        &self,
        device: &wgpu::Device,
        pass: &mut wgpu::RenderPass<'_>,
        crop: &wgpu::BindGroup,
        tables: Option<&crate::finite_ink::Tables>,
        depth: &wgpu::TextureView,
```

Replace it with:

```rust
        &self,
        device: &wgpu::Device,
        pass: &mut wgpu::RenderPass<'_>,
        view: &wgpu::BindGroup,
        crop: &wgpu::BindGroup,
        tables: Option<&crate::finite_ink::Tables>,
        depth: &wgpu::TextureView,
```

**Locate this block:**

```rust
            return;
        };
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &self.group, &[]);
        pass.set_bind_group(1, crop, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.set_vertex_buffer(1, ids.slice(..));
```

Replace it with:

```rust
            return;
        };
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, view, &[]);
        pass.set_bind_group(1, crop, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.set_vertex_buffer(1, ids.slice(..));
```

**Locate this block:**

```rust
        Ok(())
    }

    pub fn pick(&self, pass: &mut wgpu::RenderPass<'_>, crop: &wgpu::BindGroup) {
        let (Some(vertices), Some(ids), Some(pipeline)) = (
            &self.vertices,
            &self.ids,
```

Replace it with:

```rust
        Ok(())
    }

    pub fn pick(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        view: &wgpu::BindGroup,
        crop: &wgpu::BindGroup,
    ) {
        let (Some(vertices), Some(ids), Some(pipeline)) = (
            &self.vertices,
            &self.ids,
```

**Locate this block:**

```rust
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.draw(
            0..self.vertices_per_instance,
```

Replace it with:

```rust
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, view, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.draw(
            0..self.vertices_per_instance,
```

**Locate this block:**

```rust
            ink.set(device, tables, depth)
        }
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        let Some(vertices) = &self.vertices else {
            return;
        };
        if self.finite.as_ref().is_some_and(|ink| {
            ink.draw(
                pass,
                &self.group,
                vertices,
                self.flags
                    .as_ref()
```

Replace it with:

```rust
            ink.set(device, tables, depth)
        }
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>, view: &wgpu::BindGroup) {
        let Some(vertices) = &self.vertices else {
            return;
        };
        if self.finite.as_ref().is_some_and(|ink| {
            ink.draw(
                pass,
                view,
                vertices,
                self.flags
                    .as_ref()
```

**Locate this block:**

```rust
        Ok(())
    }

    pub fn set_opacity(&self, value: f32) {
        self.opacity.set(value);
    }
    pub fn view(
        &self,
        queue: &wgpu::Queue,
        transform: &[f32; 16],
        size: [u32; 2],
        density: f32,
    ) {
        if self.vertices.is_none() {
            return;
        }
        let bytes: Vec<u8> = transform
            .iter()
            .copied()
            .chain([
                size[0] as f32,
                size[1] as f32,
                density,
                self.opacity.get(),
            ])
            .flat_map(f32::to_ne_bytes)
            .collect();
        queue.write_buffer(&self.uniform, 0, &bytes);
    }

    pub fn set_flags(
        &mut self,
        device: &wgpu::Device,
```

Replace it with:

```rust
        Ok(())
    }

    pub fn set_flags(
        &mut self,
        device: &wgpu::Device,
```

**Locate this block:**

```rust
    pick_pipeline: Option<wgpu::RenderPipeline>,
    ids: Option<wgpu::Buffer>,
    id_data: Vec<u8>,
    uniform: wgpu::Buffer,
    group: wgpu::BindGroup,
    vertices: Option<wgpu::Buffer>,
    data: Vec<u8>,
    stride: u64,
```

Replace it with:

```rust
    pick_pipeline: Option<wgpu::RenderPipeline>,
    ids: Option<wgpu::Buffer>,
    id_data: Vec<u8>,
    vertices: Option<wgpu::Buffer>,
    data: Vec<u8>,
    stride: u64,
```

**Locate this block:**

```rust
pub struct Lane {
    opacity: std::cell::Cell<f32>,
    flags: Option<wgpu::Buffer>,
    flag_data: Vec<u32>,
    finite: Option<crate::finite_ink::Ink>,
```

Replace it with:

```rust
pub struct Lane {
    flags: Option<wgpu::Buffer>,
    flag_data: Vec<u32>,
    finite: Option<crate::finite_ink::Ink>,
```

### 6. `src/finite_ink.rs`

Keep finite color and integer pipelines compatible with the same frame.

**Locate this existing block:**

```rust
        let attributes=match stride{44=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32].to_vec(),72=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Float32x3,5=>Float32x3,6=>Uint32].to_vec(),32=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x4].to_vec(),_=>panic!("Unsupported finite ink layout")};
        let view = crate::gpu_layout::uniform(device, 80);
        let crop = crate::gpu_layout::uniform(device, 80);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let attributes=match stride{44=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32].to_vec(),72=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Float32x3,5=>Float32x3,6=>Uint32].to_vec(),32=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x4].to_vec(),_=>panic!("Unsupported finite ink layout")};
        let view = crate::surface_view::layout(device);
        let crop = crate::gpu_layout::uniform(device, 80);
```

### 7. `src/pick_gpu.rs`

Stop overwriting individual lane frames for a pick.

**Locate this existing block:**

```rust
        queue.write_buffer(&self.crop, 0, &bytes);
        for lane in ink {
            lane.view(
                queue,
                &request.transform,
                window.full,
                request.density,
            );
        }
        if let Some(sheets) = sheets {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        queue.write_buffer(&self.crop, 0, &bytes);
        if let Some(sheets) = sheets {
```

### 8. `src/pick_encode.rs`

Draw every ordinary ink ID with the request-owned group.

**Locate this existing block:**

```rust
                &mut pass,
                crop_group,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                &mut pass,
                view_group,
                crop_group,
```

### 9. `src/highlight.rs`

Remove the wrapper’s redundant frame-update traversal. Borrow the same explicit frame through the wrapper. Forward the borrowed frame without extra state.

**Locate this existing block:**

```rust
    }

    pub fn view(
        &self,
        queue: &wgpu::Queue,
        transform: &[f32; 16],
        size: [u32; 2],
        density: f32,
    ) {
        for lane in [&self.strokes, &self.paths, &self.markers] {
            lane.view(queue, transform, size, density);
        }
    }

    pub fn mesh(&self, pass: &mut wgpu::RenderPass<'_>) {
        if let Some(mesh) = &self.mesh {
            mesh.draw_pulled(pass);
        }
    }
    pub fn ink(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.strokes.draw(pass);
        self.paths.draw(pass);
    }
    pub fn points(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.markers.draw(pass);
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn mesh(&self, pass: &mut wgpu::RenderPass<'_>) {
        if let Some(mesh) = &self.mesh {
            mesh.draw_pulled(pass);
        }
    }
    pub fn ink(&self, pass: &mut wgpu::RenderPass<'_>, view: &wgpu::BindGroup) {
        self.strokes.draw(pass, view);
        self.paths.draw(pass, view);
    }
    pub fn points(&self, pass: &mut wgpu::RenderPass<'_>, view: &wgpu::BindGroup) {
        self.markers.draw(pass, view);
    }
```

### 10. `src/tool_preview.rs`

Remove the wrapper’s redundant frame-update traversal. Borrow the same explicit frame through the wrapper. Forward the borrowed frame without extra state.

**Locate this existing block:**

```rust
    }
    pub fn view(
        &self,
        queue: &wgpu::Queue,
        transform: &[f32; 16],
        size: [u32; 2],
        density: f32,
    ) {
        self.strokes
            .view(queue, transform, size, density);
        self.markers
            .view(queue, transform, size, density);
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.strokes.draw(pass);
        self.markers.draw(pass);
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>, view: &wgpu::BindGroup) {
        self.strokes.draw(pass, view);
        self.markers.draw(pass, view);
    }
```

### 11. `src/measure_gpu.rs`

Remove the wrapper’s redundant frame-update traversal. Borrow the same explicit frame through the wrapper. Forward the borrowed frame without extra state.

**Locate this existing block:**

```rust
    }
    pub fn view(
        &self,
        queue: &wgpu::Queue,
        transform: &[f32; 16],
        size: [u32; 2],
        density: f32,
    ) {
        self.strokes
            .view(queue, transform, size, density)
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.strokes.draw(pass)
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>, view: &wgpu::BindGroup) {
        self.strokes.draw(pass, view)
    }
```

### 12. `src/guide_gpu.rs`

Remove the wrapper’s redundant frame-update traversal. Borrow the same explicit frame through the wrapper. Forward the borrowed frame without extra state.

**Locate this existing block:**

```rust
    }
    pub fn view(
        &self,
        queue: &wgpu::Queue,
        matrix: &[f32; 16],
        size: [u32; 2],
        density: f32,
    ) {
        self.segments
            .view(queue, matrix, size, density);
        self.marks.view(queue, matrix, size, density)
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.segments.draw(pass);
        self.marks.draw(pass)
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>, view: &wgpu::BindGroup) {
        self.segments.draw(pass, view);
        self.marks.draw(pass, view)
    }
```

### 13. `src/highlight.rs`

Selection reads the existing shared opacity.

**Locate this existing block:**

```rust
    }

    pub fn set_opacity(&self, value: f32) {
        for lane in [&self.strokes, &self.paths, &self.markers] {
            lane.set_opacity(value);
        }
    }

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

### 14. `src/renderer.rs`

Keep the single renderer opacity owner. One shared frame replaces this lane or wrapper update. Measurement and guides borrow the same color frame. Borrow the existing color group for this draw.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                occlusion_query_set: None,
                multiview_mask: None,
            });
            self.measurement.draw(&mut pass);
            self.guides.draw(&mut pass);
        }
```

Replace it with:

```rust
                occlusion_query_set: None,
                multiview_mask: None,
            });
            self.measurement
                .draw(&mut pass, &self.view_group);
            self.guides.draw(&mut pass, &self.view_group);
        }
```

**Locate this block:**

```rust
                multiview_mask: None,
            });
            pass.set_bind_group(2, &clip_group, &[]);
            self.highlight.ink(&mut pass);
            self.mesh_ink.draw(&mut pass);
            self.strokes.draw(&mut pass);
            self.sheets.draw(&mut pass);
            self.paths.draw(&mut pass);
            self.markers.draw(&mut pass);
            self.highlight.points(&mut pass);
            self.preview.draw(&mut pass);
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
```

Replace it with:

```rust
                multiview_mask: None,
            });
            pass.set_bind_group(2, &clip_group, &[]);
            self.highlight
                .ink(&mut pass, &self.view_group);
            self.mesh_ink
                .draw(&mut pass, &self.view_group);
            self.strokes
                .draw(&mut pass, &self.view_group);
            self.sheets.draw(&mut pass);
            self.paths.draw(&mut pass, &self.view_group);
            self.markers
                .draw(&mut pass, &self.view_group);
            self.highlight
                .points(&mut pass, &self.view_group);
            self.preview
                .draw(&mut pass, &self.view_group);
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
```

**Locate this block:**

```rust
                ],
                ..Default::default()
            });
            self.grid.draw(&mut pass);
            pass.set_bind_group(2, &clip_group, &[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
```

Replace it with:

```rust
                ],
                ..Default::default()
            });
            self.grid.draw(&mut pass, &self.view_group);
            pass.set_bind_group(2, &clip_group, &[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
```

**Locate this block:**

```rust
            crate::surface_view::bytes(*transform, display, self.size, self.density);
        self.queue
            .write_buffer(&self.uniform, 0, &bytes);
        self.strokes.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        self.sheets.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        self.paths.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        self.markers.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        self.grid.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        self.mesh_ink.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        self.highlight.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        self.preview.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        self.measurement.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        self.guides.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        let [r, g, b] = if self.ambient_enabled.get() {
            [0.94; 3]
        } else {
```

Replace it with:

```rust
            crate::surface_view::bytes(*transform, display, self.size, self.density);
        self.queue
            .write_buffer(&self.uniform, 0, &bytes);
        self.sheets.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        let [r, g, b] = if self.ambient_enabled.get() {
            [0.94; 3]
        } else {
```

**Locate this block:**

```rust
        }
        for lane in [
            &self.mesh_ink,
            &self.strokes,
            &self.paths,
            &self.markers,
            &self.grid,
        ] {
            lane.set_opacity(value);
        }
        self.highlight.set_opacity(value);
        if let Some(o) = self.outline.borrow_mut().as_mut() {
            o.invalidate();
        }
```

Replace it with:

```rust
        }
        if let Some(o) = self.outline.borrow_mut().as_mut() {
            o.invalidate();
        }
```

### 15. `src/surface_lighting_tests.rs`

Retain the exact camera/display test and extend its actual viewport-prefix contract.

**Locate this existing block:**

```rust
fn surface_frame_bytes_preserve_camera_and_display_domains() {
    assert_eq!(crate::surface_view::BYTES, 96);
    for unit in [Unit::Millimeters, Unit::Meters] {
        for projection in [
            Projection::Perspective,
            Projection::Orthographic,
        ] {
            let mut c = camera();
            c.unit = unit;
            c.distance = 3. / unit.to_meters();
            c.projection = projection;
            c.target = [1e9, -1e9, 1e9];
            let matrix = c.view_projection_anchored(c.target).to_f32();
            let bytes = crate::surface_view::bytes(matrix, [1., 0.7, 2., 1.]);
            let values = bytes
                .chunks_exact(4)
                .map(|b| f32::from_ne_bytes(b.try_into().unwrap()))
                .collect::<Vec<_>>();
            assert_eq!(&values[..16], &matrix);
            assert_eq!(&values[16..20], &[1., 0.7, 2., 1.]);
            assert!(values.iter().all(|v| v.is_finite()));
            if projection == Projection::Perspective {
                assert_eq!(values[23], 0.);
                let forward = c.orientation.rotate_vector(Vector::y_axis());
                for i in 0..3 {
                    let expected = -forward[i] * c.distance;
                    assert!(
                        (values[20 + i] as f64 - expected).abs() < c.distance * 2e-6
                    );
                }
            } else {
                assert!(values[23] > 0.);
            }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
fn surface_frame_bytes_preserve_camera_and_display_domains() {
    assert_eq!(crate::surface_view::BYTES, 112);
    for unit in [Unit::Millimeters, Unit::Meters] {
        for projection in [
            Projection::Perspective,
            Projection::Orthographic,
        ] {
            let mut c = camera();
            c.unit = unit;
            c.distance = 3. / unit.to_meters();
            c.projection = projection;
            c.target = [1e9, -1e9, 1e9];
            let matrix = c.view_projection_anchored(c.target).to_f32();
            let bytes =
                crate::surface_view::bytes(matrix, [1., 0.7, 2., 1.], [640, 480], 2.);
            let values = bytes
                .chunks_exact(4)
                .map(|b| f32::from_ne_bytes(b.try_into().unwrap()))
                .collect::<Vec<_>>();
            assert_eq!(&values[..16], &matrix);
            assert_eq!(&values[16..20], &[640., 480., 2., 0.7]);
            assert_eq!(&values[20..24], &[1., 0.7, 2., 1.]);
            assert!(values.iter().all(|v| v.is_finite()));
            if projection == Projection::Perspective {
                assert_eq!(values[27], 0.);
                let forward = c.orientation.rotate_vector(Vector::y_axis());
                for i in 0..3 {
                    let expected = -forward[i] * c.distance;
                    assert!(
                        (values[24 + i] as f64 - expected).abs() < c.distance * 2e-6
                    );
                }
            } else {
                assert!(values[27] > 0.);
            }
```

### 16. `src/shared_view_tests.rs`

Supply real mixed-original color/request/pick restoration controls.

Create the file and type:

```rust
use crate::{
    camera::Projection,
    cap_gpu_tests::{pixels_at, renderer, wait},
    scene::Scene,
};
use session_rust::{Point, Polyline};
fn project(matrix: [f32; 16], p: [f32; 3]) -> [f32; 2] {
    let c: [f32; 4] = std::array::from_fn(|row| {
        p.iter()
            .enumerate()
            .fold(matrix[12 + row], |s, (i, v)| {
                s + matrix[i * 4 + row] * v
            })
    });
    [c[0] / c[3], c[1] / c[3]]
}
#[test]
fn shared_color_and_request_frames_preserve_actual_original_ink() {
    let mut scene = Scene::empty();
    scene
        .insert(
            crate::prepared::PreparedMesh::triangle(
                [
                    [-0.8, -0.7, 0.],
                    [0.8, -0.7, 0.],
                    [0., 0.8, 0.],
                ],
                [0.2, 0.4, 0.7],
            )
            .unwrap(),
        )
        .unwrap();
    let mut path = Polyline::new(vec![
        Point::new(-0.7, -0.3, 0.25),
        Point::new(0., 0.1, 0.25),
        Point::new(0.7, 0.3, 0.25),
    ]);
    path.width = 3.;
    let _ = path.guid();
    let source = path.to_proto();
    scene
        .insert_path(
            crate::chain::PreparedChain::polyline(std::rc::Rc::new(path)).unwrap(),
        )
        .unwrap();
    let mut point = Point::new(0.55, -0.45, 0.3);
    point.width = 12.;
    let owner = scene
        .insert_point(
            crate::marker::PreparedPoint::new(std::rc::Rc::new(point)).unwrap(),
        )
        .unwrap();
    let mut r = renderer(&scene);
    let uploads = r.stats()[0];
    let path_uploads = r.path_usage();
    let mut c = crate::camera_defaults_tests::top_fixture();
    c.aspect = 4. / 3.;
    for projection in [
        Projection::Orthographic,
        Projection::Perspective,
    ] {
        c.projection = projection;
        for density in [1., 2.] {
            r.set_density(density);
            let matrix = c.uniform();
            let before = pixels_at(&r, matrix);
            assert!(
                before
                    .chunks_exact(4)
                    .filter(|p| p[..3] != [255; 3])
                    .count()
                    > 2000
            );
            let target = project(matrix, [0.55, -0.45, 0.3]);
            r.request_pick(target, matrix).unwrap();
            let mut later = crate::camera_defaults_tests::top_fixture();
            later.aspect = c.aspect;
            later.projection = c.projection;
            later.pan(0.1, 0.);
            let _ = pixels_at(&r, later.uniform());
            wait(&r);
            assert_eq!(
                r.take_pick(matrix)
                    .unwrap()
                    .unwrap()
                    .unwrap()[0],
                owner.value(),
                "The request retains its original transform/viewport while the later color frame changes"
            );
            assert_eq!(
                pixels_at(&r, matrix),
                before,
                "Color frame restores exact original surface and ink pixels"
            );
        }
    }
    assert_eq!(r.stats()[0], uploads);
    assert_eq!(r.path_usage(), path_uploads);
    assert_eq!(
        match &scene.paths()[0].prepared.source {
            crate::chain::Source::Polyline(p) => p.to_proto(),
            _ => panic!(),
        },
        source
    );
}
```

### 17. `src/renderer.rs`

Register optional native GPU controls outside the browser graph.

**Locate this existing block:**

```rust
#[path = "surface_controls_gpu_tests.rs"]
mod surface_controls_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[path = "surface_controls_gpu_tests.rs"]
mod surface_controls_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "shared_view_tests.rs"]
mod shared_view_tests;
```

## <a id="checkpoint-96zzzzzzj-fill-original-contact-polygons"></a>2. Fill contact polygons

**What this code does:** Connect original contact polygons, their existing perimeter and original object owner through one derived mesh with bounded shared surface depth.

### 1. `src/source_mesh_topology.rs`

Reuse the existing full-ring Newell sum without per-face coordinate allocations. Read either mesh slots or original outline coordinates through one accessor. Keep every existing topology caller on the same normal algorithm.

**Locate this existing block:**

```rust
/// Unit normal of one face, None when degenerate.
fn face_normal(vs: &[usize], vpos: &[[f64; 3]], slots: &SlotMap) -> Option<[f64; 3]> {
    if vs.len() < 3 {
        return None;
    }

    // Newell normal: sums over every edge
    let mut n = [0.0f64; 3];

    for i in 0..vs.len() {
        let a = vpos[slots.slot(vs[i])];
        let b = vpos[slots.slot(vs[(i + 1) % vs.len()])];
        n[0] += (a[1] - b[1]) * (a[2] + b[2]);
        n[1] += (a[2] - b[2]) * (a[0] + b[0]);
        n[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }

    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();

    if len > Tolerance::ZERO_TOLERANCE {
        Some([n[0] / len, n[1] / len, n[2] / len])
    } else {
        None
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
/// Unit normal of one face, None when degenerate.
pub fn polygon_normal(
    count: usize,
    point: impl Fn(usize) -> [f64; 3],
) -> Option<[f64; 3]> {
    if count < 3 {
        return None;
    }

    // Newell normal: sums over every edge
    let mut n = [0.0f64; 3];

    for i in 0..count {
        let a = point(i);
        let b = point((i + 1) % count);
        n[0] += (a[1] - b[1]) * (a[2] + b[2]);
        n[1] += (a[2] - b[2]) * (a[0] + b[0]);
        n[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }

    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();

    if len > Tolerance::ZERO_TOLERANCE {
        Some([n[0] / len, n[1] / len, n[2] / len])
    } else {
        None
    }
}

fn face_normal(vs: &[usize], vpos: &[[f64; 3]], slots: &SlotMap) -> Option<[f64; 3]> {
    polygon_normal(vs.len(), |i| vpos[slots.slot(vs[i])])
}
```

### 2. `src/feature_display.rs`

Try actual area before the existing float-collapsed point/path fallback.

**Locate this existing block:**

```rust
                for (part, outline) in feature.outlines.iter().enumerate() {
                    let point = outline.get_point(0).filter(|p| {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                for (part, outline) in feature.outlines.iter().enumerate() {
                    if feature.feature_type == "contact"
                        && let Some(mut fill) = contact_fill(outline)?
                    {
                        fill.model = model.clone();
                        let id = scene.insert(fill)?;
                        outlines.push(FeatureTree::Piece(
                            format!("piece {}", part + 1),
                            id,
                        ));
                        continue;
                    }
                    let point = outline.get_point(0).filter(|p| {
```

### 3. `src/contact_feature_tests.rs`

Supply actual Element-source, concave perimeter, original-owner GPU and depth/occlusion controls.

Create the file and type:

```rust
use crate::{
    camera::Projection,
    cap_gpu_tests::{pixels_at, renderer, wait},
    editor::{Action, Editor},
    scene::Scene,
};
use prost::Message;
use session_rust::element::ElementFeature;
use session_rust::{Element, Geometry, Mesh, Point, Polyline, Session, Xform};

fn polygon() -> Polyline {
    Polyline::new(
        [
            [-550., -350., 0.],
            [0., -350., 0.],
            [550., -350., 0.],
            [550., 350., 0.],
            [150., 350., 0.],
            [150., 0., 0.],
            [-150., 0., 0.],
            [-150., 350., 0.],
            [-550., 350., 0.],
            [-550., -350., 0.],
        ]
        .into_iter()
        .map(|p| Point::new(p[0], p[1], p[2]))
        .collect(),
    )
}
fn panel() -> Mesh {
    let mut mesh = Mesh::from_vertices_and_faces(
        vec![
            Point::new(-900., -650., 0.),
            Point::new(900., -650., 0.),
            Point::new(900., 650., 0.),
            Point::new(-900., 650., 0.),
        ],
        vec![vec![0, 1, 2, 3]],
    );
    mesh.set_objectcolor(session_rust::Color::new(0.2, 0.4, 0.6, 1.));
    mesh
}
fn original(reverse: bool) -> Session {
    let mut element = Element::new("Original contact panel");
    element.set_geometry(panel());
    let mut outline = polygon();
    if reverse {
        outline.coords = outline
            .coords
            .chunks_exact(3)
            .rev()
            .flatten()
            .copied()
            .collect();
    }
    element.add_feature(ElementFeature::new(
        "contact",
        0,
        vec![outline],
        "concave contact",
    ));
    let mut session = Session::new("Original contact feature");
    session.add_element(element, None);
    session
}
fn imported(reverse: bool) -> (Editor, session_rust::proto::Session) {
    let source = original(reverse).to_proto();
    let mut editor = Editor::default();
    editor.scene = Scene::empty();
    crate::scene_document::import(
        &mut editor.scene,
        &source.encode_to_vec(),
        "original-contact",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    editor.camera = crate::camera_defaults_tests::top_fixture();
    editor.camera.distance = 3000.;
    editor.camera.radius = 1500.;
    editor.camera.aspect = 4. / 3.;
    (editor, source)
}
fn retained(editor: &Editor) -> session_rust::proto::Element {
    let Geometry::Element(element) = editor.scene.objects()[0]
        .original
        .as_ref()
        .unwrap()
    else {
        panic!("Original Element retained")
    };
    element.to_proto()
}
fn same_element(a: &session_rust::proto::Element, b: &session_rust::proto::Element) {
    assert_eq!(a.geometry_type, b.geometry_type);
    assert_eq!(a.geometry_type, "Mesh");
    assert_eq!(
        session_rust::proto::Mesh::decode(a.geometry_data.as_slice()).unwrap(),
        session_rust::proto::Mesh::decode(b.geometry_data.as_slice()).unwrap(),
        "Every embedded original mesh field is retained regardless of protobuf map order"
    );
    let (mut a, mut b) = (a.clone(), b.clone());
    a.geometry_data.clear();
    b.geometry_data.clear();
    assert_eq!(
        a, b,
        "Every original Element metadata/feature field is retained"
    );
}
fn project(matrix: [f32; 16], p: [f64; 3]) -> [f32; 2] {
    let p = p.map(|v| v as f32);
    let c: [f32; 4] = std::array::from_fn(|row| {
        p.iter()
            .enumerate()
            .fold(matrix[12 + row], |s, (i, v)| {
                s + matrix[i * 4 + row] * v
            })
    });
    [c[0] / c[3], c[1] / c[3]]
}
fn pixel<'a>(image: &'a [u8], matrix: [f32; 16], p: [f64; 3]) -> &'a [u8] {
    let p = project(matrix, p);
    let x = ((p[0] + 1.) * 160.).floor() as usize;
    let y = ((1. - p[1]) * 120.).floor() as usize;
    assert!(x < 320 && y < 240);
    &image[(y * 320 + x) * 4..(y * 320 + x + 1) * 4]
}
fn depth(r: &super::Renderer, points: &[[u32; 2]]) -> Vec<f32> {
    use wgpu::util::DeviceExt;
    let coords = points
        .iter()
        .enumerate()
        .map(|(i, p)| {
            format!(
                "values[{i}u]=textureLoad(depth,vec2<i32>({},{}),0);",
                p[0], p[1]
            )
        })
        .collect::<String>();
    let source = format!(
        "@group(0) @binding(0) var depth:texture_depth_2d; @group(0) @binding(1) var<storage,read_write> values:array<f32>; @compute @workgroup_size(1) fn main(){{{coords}}}"
    );
    let module = r
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Actual contact color-pass depth"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
    let pipeline = r
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &module,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });
    let output = r
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &vec![0u8; points.len() * 4],
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        });
    let read = r
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: output.size(),
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
    let group = r
        .device
        .create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&r.depth),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: output.as_entire_binding(),
                },
            ],
        });
    let mut encoder = r
        .device
        .create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.dispatch_workgroups(1, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&output, 0, &read, 0, output.size());
    r.queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    read.slice(..)
        .map_async(wgpu::MapMode::Read, move |v| {
            tx.send(v).unwrap()
        });
    wait(r);
    rx.recv().unwrap().unwrap();
    let data = read.slice(..).get_mapped_range();
    let result = data
        .chunks_exact(4)
        .map(|b| f32::from_ne_bytes(b.try_into().unwrap()))
        .collect();
    drop(data);
    read.unmap();
    result
}

#[test]
fn original_contact_sources_and_complete_perimeters_are_retained() {
    for reverse in [false, true] {
        let (mut editor, source) = imported(reverse);
        assert_eq!(editor.scene.objects().len(), 2);
        assert!(
            editor.scene.paths().is_empty(),
            "One derived mesh reuses its own border, without a duplicate path"
        );
        let expected = &source.objects.as_ref().unwrap().elements[0];
        same_element(&retained(&editor), expected);
        let owner = editor.scene.objects()[0].id;
        let fill = &editor.scene.objects()[1];
        assert!(editor.scene.forest.derived(fill.id));
        assert_eq!(
            crate::feature_owner::display_owner(&editor.scene, fill.id),
            Some(owner)
        );
        assert_eq!(fill.mesh.parts.faces.len(), 1);
        assert_eq!(
            fill.mesh.parts.edges.len(),
            9,
            "The original nine-edge concave ring has no triangulation diagonals"
        );
        assert!(fill.mesh.indices().len() > 9);
        assert!(
            fill.mesh
                .normals()
                .iter()
                .all(|n| n[2].abs() > 0.99 && n[0] == 0. && n[1] == 0.)
        );
        assert!(
            fill.mesh
                .alphas()
                .iter()
                .all(|v| (*v - 1. / 255.).abs() < 1e-7)
        );
        let originals = crate::pick_sources::Sources::new(&editor.scene).unwrap();
        assert_eq!(originals.mesh_ink.len(), 13);
        assert!(
            originals
                .mesh_ids
                .iter()
                .all(|p| p[0] == owner.value())
        );
        let archive = crate::scene_archive::save(&editor.scene).unwrap();
        let bundle = crate::archive::Bundle::decode(&archive).unwrap();
        assert_eq!(bundle.documents.len(), 1);
        let saved =
            session_rust::proto::Session::decode(bundle.documents[0].as_slice())
                .unwrap();
        let saved = saved.objects.unwrap();
        assert!(
            saved.meshes.is_empty(),
            "Derived contact display is excluded from the original archive"
        );
        assert_eq!(saved.elements.len(), 1);
        assert_eq!(saved.elements[0].features, expected.features);
        same_element(&saved.elements[0], expected);
        editor
            .apply(Action::ElementInteractions(Some(false)))
            .unwrap();
        assert!(
            !editor
                .scene
                .visible(editor.scene.objects()[1].id)
        );
        editor
            .apply(Action::ElementInteractions(Some(true)))
            .unwrap();
        assert!(
            editor
                .scene
                .visible(editor.scene.objects()[1].id)
        );
        same_element(&retained(&editor), expected);
    }
    for outline in [
        Polyline::new(vec![
            Point::new(0., 0., 0.),
            Point::new(1., 0., 0.),
            Point::new(2., 0., 0.),
        ]),
        Polyline::new(vec![
            Point::new(1., 2., 3.),
            Point::new(1., 2., 3.),
        ]),
    ] {
        assert!(
            crate::feature_display::contact_fill(&outline)
                .unwrap()
                .is_none()
        );
    }
    if let Ok(root) = std::env::var("COURSE_CONTACT_EXPORT") {
        std::fs::create_dir_all(&root).unwrap();
        let (editor, source) = imported(false);
        std::fs::write(
            format!("{root}/original-contact.pb"),
            source.encode_to_vec(),
        )
        .unwrap();
        std::fs::write(
            format!("{root}/original-contact.session"),
            crate::scene_archive::save(&editor.scene).unwrap(),
        )
        .unwrap();
    }
}

#[test]
fn actual_contact_fill_and_original_owner_survive_presentation() {
    let (mut editor, _) = imported(false);
    let source = retained(&editor);
    let owner = editor.scene.objects()[0].id;
    let mut r = renderer(&editor.scene);
    r.set_opacity(1.);
    let uploads = r.stats()[0];
    for projection in [
        Projection::Orthographic,
        Projection::Perspective,
    ] {
        editor.camera.projection = projection;
        for density in [1., 2.] {
            r.set_density(density);
            let matrix = editor.camera.uniform();
            let before = pixels_at(&r, matrix);
            let inside = pixel(&before, matrix, [-350., -150., 0.]);
            assert!(
                (225..=232).contains(&inside[0])
                    && (23..=28).contains(&inside[1])
                    && (23..=28).contains(&inside[2]),
                "Actual unlit contact fill: {inside:?}"
            );
            assert_eq!(
                pixel(&before, matrix, [0., 200., 0.]),
                &[51, 102, 153, 255],
                "Concave notch retains host pixels"
            );
            r.request_pick(project(matrix, [-350., -150., 0.]), matrix)
                .unwrap();
            wait(&r);
            assert_eq!(
                r.take_pick(matrix)
                    .unwrap()
                    .unwrap()
                    .unwrap()[0],
                owner.value(),
                "Contact is picked as its original Element, never a generated mesh owner"
            );
            editor
                .apply(Action::Picked(Some(owner)))
                .unwrap();
            r.select_objects(&editor.scene, &editor.highlighted_objects())
                .unwrap();
            let selected = pixels_at(&r, matrix);
            let p = pixel(&selected, matrix, [-350., -150., 0.]);
            assert!(
                p[0] > 240 && p[1] > 240 && p[2] < 20,
                "Actual original selection fills the contact yellow: {p:?}"
            );
            editor.apply(Action::Picked(None)).unwrap();
            r.select_objects(&editor.scene, &editor.highlighted_objects())
                .unwrap();
            assert_eq!(pixels_at(&r, matrix), before);
            r.set_lighting(true, true);
            assert_eq!(
                pixel(
                    &pixels_at(&r, matrix),
                    matrix,
                    [-350., -150., 0.]
                ),
                inside,
                "Contact remains unlit and independent of backface color"
            );
            r.set_lighting(false, false);
            r.use_uniform_surface_records();
            assert_eq!(pixels_at(&r, matrix), before);
            r.set_scene(&editor.scene, None);
            assert_eq!(pixels_at(&r, matrix), before);
            assert_eq!(r.stats()[0], uploads);
            same_element(&retained(&editor), &source);
        }
    }
    editor.scene.clear();
    r.set_scene(&editor.scene, None);
    pixels_at(&r, editor.camera.uniform());
    assert_eq!(r.usage()[0], 0);
}

#[test]
fn actual_contact_depth_is_nearer_without_crossing_an_occluder() {
    let (mut editor, _) = imported(false);
    let source = retained(&editor);
    let mut r = renderer(&editor.scene);
    r.set_opacity(1.);
    for projection in [
        Projection::Orthographic,
        Projection::Perspective,
    ] {
        editor.camera.projection = projection;
        let matrix = editor.camera.uniform();
        pixels_at(&r, matrix);
        let points = [[-350., -150., 0.], [0., 200., 0.]].map(|p| {
            let p = project(matrix, p);
            [
                ((p[0] + 1.) * 160.).floor() as u32,
                ((1. - p[1]) * 120.).floor() as u32,
            ]
        });
        let z = depth(&r, &points);
        assert!(
            z[0] > z[1],
            "Actual contact color vertices pull toward the eye while the host notch stays on the source plane: {z:?}"
        );
        assert!(
            z[0] <= z[1] * 1.0001 + 1e-12,
            "Layer stays bounded: {z:?}"
        );
        let mut blocked = editor.scene.clone();
        let mut roof = Mesh::from_vertices_and_faces(
            vec![
                Point::new(-600., -400., 20.),
                Point::new(-100., -400., 20.),
                Point::new(-100., 100., 20.),
                Point::new(-600., 100., 20.),
            ],
            vec![vec![0, 1, 2, 3]],
        );
        roof.set_objectcolor(session_rust::Color::new(0.1, 0.8, 0.2, 1.));
        let blocker = blocked
            .insert(crate::prepared::PreparedMesh::new(roof).unwrap())
            .unwrap();
        r.set_scene(&blocked, None);
        let image = pixels_at(&r, matrix);
        let p = pixel(&image, matrix, [-350., -150., 20.]);
        assert!(
            p[1] > 190 && p[0] < 35,
            "Real nearer geometry still occludes the bounded contact: {p:?}"
        );
        r.request_pick(project(matrix, [-350., -150., 20.]), matrix)
            .unwrap();
        wait(&r);
        assert_eq!(
            r.take_pick(matrix)
                .unwrap()
                .unwrap()
                .unwrap()[0],
            blocker.value()
        );
        r.set_scene(&editor.scene, None);
        pixels_at(&r, matrix);
        same_element(&retained(&editor), &source);
    }
}
```

### 4. `src/renderer.rs`

Keep optional native GPU validation outside the browser implementation.

**Locate this existing block:**

```rust
#[path = "shared_view_tests.rs"]
mod shared_view_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[path = "shared_view_tests.rs"]
mod shared_view_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "contact_feature_tests.rs"]
mod contact_feature_tests;
```

### 5. `src/gpu_mesh.rs`

Carry the original integer owner, including zero for an unpickable display. Write the resolved owner into the existing object record. Keep standalone uploaded originals on their own checked identity.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            object,
            Style {
                selected,
                pickable: !object.metadata.is_locked,
                colour: None,
```

Replace it with:

```rust
            object,
            Style {
                selected,
                owner: if object.metadata.is_locked {
                    0
                } else {
                    object.id.value()
                },
                colour: None,
```

**Locate this block:**

```rust
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&u32::from(style.selected).to_ne_bytes());
    let id = if style.pickable {
        object.id.value()
    } else {
        0
    };
    bytes[68..72].copy_from_slice(&id.to_ne_bytes());
    let flags = if matches!(&object.original,Some(session_rust::Geometry::Plane(p)) if crate::clipping_geometry::is_clipping(p))
    {
```

Replace it with:

```rust
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&u32::from(style.selected).to_ne_bytes());
    let id = style.owner;
    bytes[68..72].copy_from_slice(&id.to_ne_bytes());
    let flags = if matches!(&object.original,Some(session_rust::Geometry::Plane(p)) if crate::clipping_geometry::is_clipping(p))
    {
```

**Locate this block:**

```rust
    pub selected: bool,
    pub pickable: bool,
    pub colour: Option<[f32; 4]>,
}
```

Replace it with:

```rust
    pub selected: bool,
    pub owner: u32,
    pub colour: Option<[f32; 4]>,
}
```

### 6. `src/renderer.rs`

Use the existing feature adapter for color and request-specific surface records.

**Locate this existing block:**

```rust
        selected,
        pickable: scene.pick_id(id.value()).is_some(),
        colour: scene.face_rgba(id),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        selected,
        owner: crate::feature_owner::display_owner(scene, id)
            .map_or(0, ObjectId::value),
        colour: scene.face_rgba(id),
```

### 7. `src/global_outline_tests.rs`

Retain every exact original record/pixel/anchor assertion with the explicit owner field.

**Locate this existing block:**

```rust
        selected: false,
        pickable: true,
        colour: None,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        selected: false,
        owner: owner.value(),
        colour: None,
```

### 8. `src/contact_depth.wgsl`

Pull the contact toward the eye by the current bounded depth/slope layer, leaving screen position and source coordinates fixed.

Create the file and type:

```wgsl
fn contact_layer(clip:vec4<f32>,world:vec3<f32>,normal:vec3<f32>,m:mat4x4<f32>,size:vec2<f32>,eye:vec4<f32>)->vec4<f32> {
  if(clip.w<=0.||dot(normal,normal)==0.) {return clip;}
  let n=normalize(normal);
  let u=normalize(cross(n,select(vec3<f32>(1.,0.,0.),vec3<f32>(0.,1.,0.),abs(n.x)>0.9)));
  let v=cross(n,u);let reach=select(length(eye.xyz-world),eye.w,eye.w>0.)*0.01;
  let a=m*vec4<f32>(world+u*reach,1.);let b=m*vec4<f32>(world+v*reach,1.);
  var slope=0.;
  if(a.w>0.&&b.w>0.) {
    let p=vec3<f32>(clip.xy/clip.w*size*0.5,clip.z/clip.w);
    let du=vec3<f32>(a.xy/a.w*size*0.5,a.z/a.w)-p;
    let dv=vec3<f32>(b.xy/b.w*size*0.5,b.z/b.w)-p;
    let det=du.x*dv.y-du.y*dv.x;
    if(abs(det)>=1e-12) {slope=max(abs((du.z*dv.y-dv.z*du.y)/det),abs((du.x*dv.z-dv.x*du.z)/det));}
  }
  let z=abs(clip.z/clip.w);let pull=min(4.7683716e-7*z+0.125*slope,1e-4*z);
  return vec4<f32>(clip.xy,clip.z+pull*clip.w,clip.w);
}
```

### 9. `src/clipping_gpu.rs`

Expand the shared helper only in shaders which explicitly include it.

**Locate this existing block:**

```rust
pub fn source(base: &str, group: u32, binding: u32) -> String {
    format!(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn source(base: &str, group: u32, binding: u32) -> String {
    let base = base.replace(
        r#"#include "contact_depth.wgsl""#,
        include_str!("contact_depth.wgsl"),
    );
    format!(
```

### 10. `src/triangle.wgsl`

Share contact placement between ordinary, pulled and integer surface entries. Use the actual original-facing plane and full physical frame for the layer.

**Locate this existing block:**

```wgsl
}

const CONTACT_FLAG:u32=65536u;
fn geometry(position: vec3<f32>, colour: vec4<f32>, normal: vec3<f32>, object: ObjectSettings) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    let contact=abs(colour.a-1./255.)<0.5/255.;
    let displayed=select(colour.rgb,object.face_colour.xyz,object.face_colour.w>0.5&&!contact);
    output.colour = select(displayed, SELECTION_COLOR, (object.selection.x & 1u) != 0u);
    output.mirrored=select(0u,1u,determinant(mat3x3<f32>(object.model[0].xyz,object.model[1].xyz,object.model[2].xyz))<0.);
    output.world = world.xyz;
    output.clip_flags=select(object.selection.z,(object.selection.z&~(8192u|32768u))|CONTACT_FLAG,contact);
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
}

#include "contact_depth.wgsl"
const CONTACT_FLAG:u32=65536u;
fn geometry(position: vec3<f32>, colour: vec4<f32>, normal: vec3<f32>, object: ObjectSettings) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    let contact=abs(colour.a-1./255.)<0.5/255.;
    let displayed=select(colour.rgb,object.face_colour.xyz,object.face_colour.w>0.5&&!contact);
    output.colour = select(displayed, SELECTION_COLOR, (object.selection.x & 1u) != 0u);
    output.mirrored=select(0u,1u,determinant(mat3x3<f32>(object.model[0].xyz,object.model[1].xyz,object.model[2].xyz))<0.);
    output.world = world.xyz;
    output.clip_flags=select(object.selection.z,(object.selection.z&~(8192u|32768u))|CONTACT_FLAG,contact);
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    if(contact) {output.position=contact_layer(output.position,world.xyz,output.normal,view.transform,view.viewport.xy,view.eye);}
    return output;
```

### 11. `src/face_pick.wgsl`

Supply the actual normal to the same source-face vertex function.

**Locate this existing block:**

```wgsl
 let vertex=positions[indices[index+object.range.x]];
 let placed=geometry(packed_position(vertex),unpack4x8unorm(vertex.color),vec3<f32>(0.),object);
 return FaceOutput(crop.transform*placed.position,faces[index/3u]+source.base,placed.world,placed.clip_flags,object.selection.y);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
 let vertex=positions[indices[index+object.range.x]];
 let placed=geometry(packed_position(vertex),unpack4x8unorm(vertex.color),oct32_decode(vertex.normal),object);
 return FaceOutput(crop.transform*placed.position,faces[index/3u]+source.base,placed.world,placed.clip_flags,object.selection.y);
```

### 12. `src/feature_display.rs`

Keep contact display preparation in its existing feature module. Use the original kernel types already owned by feature preparation. Prepare one contact mesh, retaining its full normal, display marker and existing polygon Parts.

Replace the entire file with:

```rust
use crate::prepared::PreparedMesh;
use crate::{
    layer_edit::FeatureTree,
    scene::{ObjectId, Scene},
};
use session_rust::{Color, Mesh, Point, Polyline};
use session_rust::{Element, Session};
use std::rc::Rc;

pub fn append(
    scene: &mut Scene,
    id: ObjectId,
    session: &Session,
    element: &Element,
) -> Result<(), &'static str> {
    let model = scene
        .placement(id)
        .ok_or("Element placement is missing")?;
    let forest = std::mem::take(&mut scene.forest);
    let created = scene.created.take();
    let current = Rc::make_mut(&mut scene.view).current.take();
    let prepared = (|| {
        let mut groups = Vec::new();
        let mut pieces = |indices: &[usize]| -> Result<Vec<FeatureTree>, &'static str> {
            let mut rows = Vec::new();
            for &index in indices {
                let feature = &element.features[index];
                if !feature.visible {
                    continue;
                }
                let mut outlines = Vec::new();
                for (part, outline) in feature.outlines.iter().enumerate() {
                    if feature.feature_type == "contact"
                        && let Some(mut fill) = contact_fill(outline)?
                    {
                        fill.model = model.clone();
                        let id = scene.insert(fill)?;
                        outlines.push(FeatureTree::Piece(
                            format!("piece {}", part + 1),
                            id,
                        ));
                        continue;
                    }
                    let point = outline.get_point(0).filter(|p| {
                        outline.coords.chunks_exact(3).all(|q| {
                            [q[0] as f32, q[1] as f32, q[2] as f32] == p.to_f32()
                        })
                    });
                    let piece = if let Some(mut point) = point {
                        point.set_guid(uuid::Uuid::new_v4().to_string());
                        point.name = format!("piece {}", part + 1);
                        point.pointcolor = outline.linecolor.clone();
                        point.width = 12.0;
                        point.is_visible = true;
                        point.is_locked = false;
                        let piece = scene.insert_point(
                            crate::marker::PreparedPoint::new(Rc::new(point))?,
                        )?;
                        scene.place_point(piece, model.clone())?;
                        piece
                    } else {
                        let mut source = outline.clone();
                        source.set_guid(uuid::Uuid::new_v4().to_string());
                        source.name = format!("piece {}", part + 1);
                        source.width = 2.0;
                        source.is_visible = true;
                        source.is_locked = false;
                        source.arrowhead = session_rust::Arrowhead::NONE;
                        let piece = scene.insert_path(
                            crate::chain::PreparedChain::polyline(Rc::new(source))?,
                        )?;
                        scene.place_following(piece, model.clone())?;
                        piece
                    };
                    if !crate::feature_plan::interaction(feature) {
                        Rc::make_mut(&mut scene.view).hide(&[piece], true);
                    }
                    outlines.push(FeatureTree::Piece(
                        format!("piece {}", part + 1),
                        piece,
                    ));
                }
                if !outlines.is_empty() {
                    rows.push(FeatureTree::Group(
                        crate::feature_plan::label(feature),
                        outlines,
                    ));
                }
            }
            Ok(rows)
        };
        let own = pieces(&crate::feature_plan::own(element))?;
        if !own.is_empty() {
            groups.push(FeatureTree::Group("attributes".into(), own));
        }
        let mut hosted = Vec::new();
        for relation in crate::feature_plan::hosted(session, element) {
            let children = pieces(&relation.features)?;
            if !children.is_empty() {
                hosted.push(FeatureTree::Group(relation.label, children));
            }
        }
        if !hosted.is_empty() {
            groups.push(FeatureTree::Group(
                "interactions".into(),
                hosted,
            ));
        }
        Ok(groups)
    })();
    scene.forest = forest;
    scene.created = created;
    Rc::make_mut(&mut scene.view).current = current;
    crate::layer_edit::attach_features(scene, id, prepared?)
}

pub(crate) fn contact_fill(
    outline: &Polyline,
) -> Result<Option<PreparedMesh>, &'static str> {
    if outline.coords.len() % 3 != 0 {
        return Err("Contact needs complete points");
    }
    let Some(normal) =
        crate::source_mesh_topology::polygon_normal(outline.coords.len() / 3, |i| {
            let p = &outline.coords[i * 3..];
            [p[0], p[1], p[2]]
        })
    else {
        return Ok(None);
    };
    let points = outline
        .coords
        .chunks_exact(3)
        .map(|p| Point::new(p[0], p[1], p[2]))
        .collect();
    let mut mesh = Mesh::from_polylines(vec![points], None);
    mesh.set_objectcolor(Color::new(0.9, 0.1, 0.1, 1.));
    let mut fill = PreparedMesh::new(mesh)?;
    let count = fill.display.vertices().len();
    fill.display = fill
        .display
        .with_normals(vec![normal.map(|v| v as f32); count])?
        .with_alphas(vec![1. / 255.; count])?;
    Ok(Some(fill))
}
```

## <a id="checkpoint-96zzzzzzk-broadcast-ordinary-ink-metadata"></a>3. Broadcast ordinary ink metadata

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Half-Space Clip: d_i = dot(P_i, n_clip) + w_clip</text>
  <polygon points="58,138 118,58 258,58 198,138" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <line x1="164" y1="44" x2="164" y2="156" stroke="#dc2626" stroke-width="2.5" stroke-dasharray="5 3"/>
  <line x1="164" y1="98" x2="218" y2="98" stroke="#dc2626" stroke-width="2.2" marker-end="url(#arrR)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="236" y="102" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">d &lt; 0</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Triangle-Plane Edge Segments -&gt; Welded Closed Cap Polygon</text>
  <polygon points="448,70 668,70 638,138 418,138" fill="#fef08a" stroke="#dc2626" stroke-width="2.2"/>
  <line x1="458" y1="70" x2="428" y2="138" stroke="#dc2626" stroke-width="1.5"/>
  <line x1="508" y1="70" x2="478" y2="138" stroke="#dc2626" stroke-width="1.5"/>
  <line x1="558" y1="70" x2="528" y2="138" stroke="#dc2626" stroke-width="1.5"/>
  <line x1="608" y1="70" x2="578" y2="138" stroke="#dc2626" stroke-width="1.5"/>
  <circle cx="448" cy="70" r="4" fill="#111111"/>
  <circle cx="668" cy="70" r="4" fill="#111111"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Live Section Cut with Capped Solid Interior</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="838,138 882,66 948,66 948,138" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
  <polygon points="948,66 994,82 994,146 948,138" fill="#fef08a" stroke="#dc2626" stroke-width="2.2"/>
</svg>
</div>

**What this code does:** Replace repeated zero clipping flags with one broadcast record in ordinary line, path and point batches.

### 1. `src/stroke_gpu.rs`

Keep one metadata mode on the existing lane. Reset only the records actually addressed by the pipeline. Allocate one default flag for a nonempty constant lane. Validate constant and indexed metadata counts. Choose a mode when creating the existing finite pipelines. Keep the selected stride in color and integer pipelines. Initialize the same bounded default. Start with the ordinary constant mode.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        Self {
            flags: None,
            flag_data: Vec::new(),
            finite: None,
```

Replace it with:

```rust
        Self {
            flags: None,
            flag_data: Vec::new(),
            varying: false,
            finite: None,
```

**Locate this block:**

```rust
        &mut self,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
    ) {
        self.finite = Some(crate::finite_ink::Ink::new(
            device,
            format,
            self.stride,
            self.strict_depth,
        ));
        self.set_flags(
            device,
            vec![0; self.data.len() / self.stride as usize],
        )
        .expect("Original ink clipping flags");
    }
    pub fn finite_frame(
        &self,
```

Replace it with:

```rust
        &mut self,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        varying: bool,
    ) {
        self.varying = varying;
        self.finite = Some(crate::finite_ink::Ink::new(
            device,
            format,
            self.stride,
            self.strict_depth,
            varying,
        ));
        self.reset_flags(device)
            .expect("Original ink clipping flags");
    }
    pub fn finite_frame(
        &self,
```

**Locate this block:**

```rust
        };
        self.data = data;
        if self.finite.is_some() {
            self.set_flags(
                device,
                vec![0; self.data.len() / self.stride as usize],
            )?;
        }
        Ok(())
    }

    pub fn set_flags(
        &mut self,
        device: &wgpu::Device,
        flags: Vec<u32>,
    ) -> Result<(), &'static str> {
        if flags.len() != self.data.len() / self.stride as usize {
            return Err("Clipping flags must match original ink instances");
        }
        if flags == self.flag_data {
```

Replace it with:

```rust
        };
        self.data = data;
        if self.finite.is_some() {
            self.reset_flags(device)?;
        }
        Ok(())
    }

    fn reset_flags(&mut self, device: &wgpu::Device) -> Result<(), &'static str> {
        let count = self.data.len() / self.stride as usize;
        self.set_flags(
            device,
            vec![
                0;
                if self.varying {
                    count
                } else {
                    count.min(1)
                }
            ],
        )
    }
    pub fn set_flags(
        &mut self,
        device: &wgpu::Device,
        flags: Vec<u32>,
    ) -> Result<(), &'static str> {
        let count = self.data.len() / self.stride as usize;
        if flags.len()
            != if self.varying {
                count
            } else {
                count.min(1)
            }
        {
            return Err("Clipping flags must match original ink instances");
        }
        if flags == self.flag_data {
```

**Locate this block:**

```rust
    flag_data: Vec<u32>,
    finite: Option<crate::finite_ink::Ink>,
    strict_depth: bool,
    pipeline: wgpu::RenderPipeline,
```

Replace it with:

```rust
    flag_data: Vec<u32>,
    varying: bool,
    finite: Option<crate::finite_ink::Ink>,
    strict_depth: bool,
    pipeline: wgpu::RenderPipeline,
```

### 2. `src/finite_ink.rs`

Use one selected layout per existing pipeline. Zero stride broadcasts the complete existing u32 record. Make metadata addressing explicit at construction. Preserve identical color and cropped integer addressing.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            stride,
            grid,
            true,
            &view,
```

Replace it with:

```rust
            stride,
            grid,
            true,
            varying,
            &view,
```

**Locate this block:**

```rust
            stride,
            grid,
            false,
            &view,
            &crop,
            &layout,
```

Replace it with:

```rust
            stride,
            grid,
            false,
            varying,
            &view,
            &crop,
            &layout,
```

**Locate this block:**

```rust
        format: wgpu::TextureFormat,
        stride: u64,
        grid: bool,
    ) -> Self {
        let attributes=match stride{44=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32].to_vec(),72=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Float32x3,5=>Float32x3,6=>Uint32].to_vec(),32=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x4].to_vec(),_=>panic!("Unsupported finite ink layout")};
        let view = crate::surface_view::layout(device);
```

Replace it with:

```rust
        format: wgpu::TextureFormat,
        stride: u64,
        grid: bool,
        varying: bool,
    ) -> Self {
        let attributes=match stride{44=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32].to_vec(),72=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Float32x3,5=>Float32x3,6=>Uint32].to_vec(),32=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x4].to_vec(),_=>panic!("Unsupported finite ink layout")};
        let view = crate::surface_view::layout(device);
```

**Locate this block:**

```rust
    }
    let flags = wgpu::vertex_attr_array![8=>Uint32];
    buffers.push(wgpu::VertexBufferLayout {
        array_stride: 4,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &flags,
    });
```

Replace it with:

```rust
    }
    let flags = wgpu::vertex_attr_array![8=>Uint32];
    buffers.push(wgpu::VertexBufferLayout {
        array_stride: if varying {
            4
        } else {
            0
        },
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &flags,
    });
```

**Locate this block:**

```rust
    picking: bool,
    view: &wgpu::BindGroupLayout,
    crop: &wgpu::BindGroupLayout,
    layout: &wgpu::BindGroupLayout,
```

Replace it with:

```rust
    picking: bool,
    varying: bool,
    view: &wgpu::BindGroupLayout,
    crop: &wgpu::BindGroupLayout,
    layout: &wgpu::BindGroupLayout,
```

### 3. `src/renderer.rs`

Mesh borders retain source-dependent clipping flags. Ordinary original lines, paths and points broadcast one record.

**Locate this existing block:**

```rust
        };
        for lane in [
            &mut renderer.mesh_ink,
            &mut renderer.strokes,
            &mut renderer.paths,
            &mut renderer.markers,
        ] {
            lane.enable_finite(&renderer.device, format);
        }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        };
        renderer
            .mesh_ink
            .enable_finite(&renderer.device, format, true);
        for lane in [
            &mut renderer.strokes,
            &mut renderer.paths,
            &mut renderer.markers,
        ] {
            lane.enable_finite(&renderer.device, format, false);
        }
```

### 4. `src/highlight.rs`

Mixed selected clipping-plane strokes retain indexed flags. Selected paths and points use constant flags.

**Locate this existing block:**

```rust
    ) {
        for lane in [
            &mut self.strokes,
            &mut self.paths,
            &mut self.markers,
        ] {
            lane.enable_finite(device, format)
        }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ) {
        self.strokes
            .enable_finite(device, format, true);
        for lane in [&mut self.paths, &mut self.markers] {
            lane.enable_finite(device, format, false)
        }
```

### 5. `src/ink_metadata_tests.rs`

Supply real zero-stride GPU draws/IDs,8192-instance allocations, mixed indexed flags and complete release controls.

Create the file and type:

```rust
use crate::{renderer::Renderer, scene::Scene, stroke_gpu::Lane};
fn renderer(scene: &Scene) -> Renderer {
    crate::cap_gpu_tests::renderer(scene)
}
#[test]
fn ordinary_ink_broadcasts_one_record_and_releases_it() {
    let r = renderer(&Scene::empty());
    let device = &r.device;
    let stroke = crate::stroke::Stroke {
        start: [0.; 3],
        end: [1., 0., 0.],
        colour: [0., 0., 0., 1.],
        width: 1.,
    };
    let mut lines = Lane::new(device, wgpu::TextureFormat::Rgba8Unorm);
    lines.enable_finite(
        device,
        wgpu::TextureFormat::Rgba8Unorm,
        false,
    );
    lines
        .set(device, std::iter::repeat_n(stroke, 8192))
        .unwrap();
    assert_eq!(
        lines.clipping_bytes(),
        [4, 4],
        "8192ordinary line instances need one CPU/GPU flags record, previously32768bytes each"
    );
    assert!(
        lines
            .set_flags(device, vec![0; 8192])
            .is_err()
    );
    lines.set(device, []).unwrap();
    assert_eq!(lines.clipping_bytes(), [0, 0]);
    let mut paths = Lane::joined(device, wgpu::TextureFormat::Rgba8Unorm);
    paths.enable_finite(
        device,
        wgpu::TextureFormat::Rgba8Unorm,
        false,
    );
    paths
        .set_chains(
            device,
            std::iter::repeat_n(
                crate::chain::Segment {
                    stroke,
                    previous: stroke.start,
                    next: stroke.end,
                    heads: 3,
                },
                8192,
            ),
        )
        .unwrap();
    assert_eq!(paths.clipping_bytes(), [4, 4]);
    paths.set_chains(device, []).unwrap();
    assert_eq!(paths.clipping_bytes(), [0, 0]);
    let mut points = Lane::markers(device, wgpu::TextureFormat::Rgba8Unorm);
    points.enable_finite(
        device,
        wgpu::TextureFormat::Rgba8Unorm,
        false,
    );
    points
        .set_points(
            device,
            std::iter::repeat_n(
                crate::marker::Marker {
                    center: [0.; 3],
                    diameter: 4.,
                    colour: [0., 0., 0., 1.],
                },
                8192,
            ),
        )
        .unwrap();
    assert_eq!(points.clipping_bytes(), [4, 4]);
    points.set_points(device, []).unwrap();
    assert_eq!(points.clipping_bytes(), [0, 0]);
    let mut varying = Lane::new(device, wgpu::TextureFormat::Rgba8Unorm);
    varying.enable_finite(device, wgpu::TextureFormat::Rgba8Unorm, true);
    varying
        .set(device, std::iter::repeat_n(stroke, 3))
        .unwrap();
    varying
        .set_flags(device, vec![0, 4096, 0])
        .unwrap();
    assert_eq!(varying.clipping_bytes(), [12, 12]);
    assert!(varying.set_flags(device, vec![0]).is_err());
    varying.set(device, []).unwrap();
    assert_eq!(varying.clipping_bytes(), [0, 0]);
}
#[test]
fn actual_zero_stride_ink_draws_and_picks_every_original() {
    let mut scene = Scene::empty();
    let mut ids = Vec::new();
    for i in 0..32 {
        let y = -0.8 + i as f64 * 0.05;
        let line = session_rust::Line::new(-0.7, y, -0.5, 0.7, y, -0.5);
        let row = scene
            .insert_line(
                crate::stroke::PreparedLine::new(std::rc::Rc::new(line)).unwrap(),
            )
            .unwrap();
        ids.push(row);
    }
    let mut r = renderer(&scene);
    r.set_opacity(1.);
    let matrix = crate::cap_gpu_tests::matrix();
    for density in [1., 2.] {
        r.set_density(density);
        let image = crate::cap_gpu_tests::pixels_at(&r, matrix);
        for (i, id) in ids.iter().enumerate() {
            let y = -0.8 + i as f32 * 0.05;
            let row = ((1. - y) * 120.).floor() as usize;
            let pixel = &image[(row * 320 + 160) * 4..(row * 320 + 161) * 4];
            assert!(
                pixel[..3].iter().all(|v| *v < 150),
                "Actual zero-stride line{i} remains visible at density{density}: {pixel:?}"
            );
            r.request_pick([0., y], matrix).unwrap();
            crate::cap_gpu_tests::wait(&r);
            assert_eq!(
                r.take_pick(matrix)
                    .unwrap()
                    .unwrap()
                    .unwrap()[0],
                id.value(),
                "The actual first, middle and last original instances retain their GPU IDs"
            );
        }
    }
    scene.clear();
    r.set_scene(&scene, None);
    crate::cap_gpu_tests::pixels_at(&r, matrix);
    assert_eq!(r.clipping_ink_bytes(), [0, 0]);
}
```

### 6. `src/renderer.rs`

Connect optional native validation outside the browser runtime.

**Locate this existing block:**

```rust
#[path = "contact_feature_tests.rs"]
mod contact_feature_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[path = "contact_feature_tests.rs"]
mod contact_feature_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "ink_metadata_tests.rs"]
mod ink_metadata_tests;
```

## <a id="checkpoint-96zzzzzzl-layer-original-contact-boundaries"></a>4. Layer contact boundaries

**What this code does:** Keep original contact borders complete at steep angles when the projection budget falls back to physical depth.

### 1. `src/stroke_gpu.rs`

Keep existing flag callers and flatten only indexed [flag,normal] records. Validate both indexed words; constant records stay one word.

**Locate this existing block:**

```rust
    ) -> Result<(), &'static str> {
        let count = self.data.len() / self.stride as usize;
        if flags.len()
            != if self.varying {
                count
            } else {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ) -> Result<(), &'static str> {
        self.set_records(
            device,
            if self.varying {
                flags
                    .into_iter()
                    .flat_map(|f| [f, crate::packed_vertex::NO_NORMAL])
                    .collect()
            } else {
                flags
            },
        )
    }
    pub fn set_records(
        &mut self,
        device: &wgpu::Device,
        flags: Vec<u32>,
    ) -> Result<(), &'static str> {
        let count = self.data.len() / self.stride as usize;
        if flags.len()
            != if self.varying {
                count * 2
            } else {
```

### 2. `src/finite_ink.rs`

Assemble only the lane’s chosen metadata interface. Use one or two explicit words in the existing attribute. Indexed strokes carry their contact normal; ordinary ink retains zero stride.

**Locate this existing block:**

```rust
) -> wgpu::RenderPipeline {
    let source = crate::finite_sources::source_style(stride, picking, grid);
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("finite original ink"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    let clipping = crate::clipping_gpu::layout(device);
    let groups = if picking {
        vec![
            Some(view),
            Some(crop),
            Some(layout),
            Some(&clipping),
        ]
    } else {
        vec![Some(view), Some(layout), Some(&clipping)]
    };
    let pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("finite original ink"),
            bind_group_layouts: &groups,
            immediate_size: 0,
        });
    let mut buffers = vec![wgpu::VertexBufferLayout {
        array_stride: stride,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes,
    }];
    let ids = wgpu::vertex_attr_array![7=>Uint32x2];
    if picking {
        buffers.push(wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &ids,
        });
    }
    let flags = wgpu::vertex_attr_array![8=>Uint32];
    buffers.push(wgpu::VertexBufferLayout {
        array_stride: if varying {
            4
        } else {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
) -> wgpu::RenderPipeline {
    let source = crate::finite_sources::source_metadata(stride, picking, grid, varying);
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("finite original ink"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    let clipping = crate::clipping_gpu::layout(device);
    let groups = if picking {
        vec![
            Some(view),
            Some(crop),
            Some(layout),
            Some(&clipping),
        ]
    } else {
        vec![Some(view), Some(layout), Some(&clipping)]
    };
    let pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("finite original ink"),
            bind_group_layouts: &groups,
            immediate_size: 0,
        });
    let mut buffers = vec![wgpu::VertexBufferLayout {
        array_stride: stride,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes,
    }];
    let ids = wgpu::vertex_attr_array![7=>Uint32x2];
    if picking {
        buffers.push(wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &ids,
        });
    }
    let flags = [wgpu::VertexAttribute {
        format: if varying {
            wgpu::VertexFormat::Uint32x2
        } else {
            wgpu::VertexFormat::Uint32
        },
        offset: 0,
        shader_location: 8,
    }];
    buffers.push(wgpu::VertexBufferLayout {
        array_stride: if varying {
            8
        } else {
```

### 3. `src/finite_sources.rs`

Keep the existing constant shader validation entry meaningful. Declare explicit flag/facing accessors for the actual input type.

**Locate this existing block:**

```rust
pub fn source_style(stride: u64, pick: bool, grid: bool) -> String {
    let (base, ids) = match stride {
        44 => (
            include_str!("finite_stroke.wgsl"),
            include_str!("finite_stroke_pick.wgsl"),
        ),
        72 => (
            include_str!("finite_chain.wgsl"),
            include_str!("finite_chain_pick.wgsl"),
        ),
        32 => (
            include_str!("finite_marker.wgsl"),
            include_str!("finite_marker_pick.wgsl"),
        ),
        _ => panic!("Unsupported finite ink layout"),
    };
    let base = crate::finite_project_source::expand(base)
        .replace(
            "#include \"finite_visibility.wgsl\"",
            include_str!("finite_visibility.wgsl"),
        )
        .replace(
            "#include \"ink_plane.wgsl\"",
            include_str!("ink_plane.wgsl"),
        );
    let base = if grid {
        base.replace(
            "const FINITE_GRID:bool=false;",
            "const FINITE_GRID:bool=true;",
        )
    } else {
        base
    };
    let source = if pick {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn source_style(stride: u64, pick: bool, grid: bool) -> String {
    source_metadata(stride, pick, grid, false)
}
pub fn source_metadata(stride: u64, pick: bool, grid: bool, varying: bool) -> String {
    let (base, ids) = match stride {
        44 => (
            include_str!("finite_stroke.wgsl"),
            include_str!("finite_stroke_pick.wgsl"),
        ),
        72 => (
            include_str!("finite_chain.wgsl"),
            include_str!("finite_chain_pick.wgsl"),
        ),
        32 => (
            include_str!("finite_marker.wgsl"),
            include_str!("finite_marker_pick.wgsl"),
        ),
        _ => panic!("Unsupported finite ink layout"),
    };
    let base = crate::finite_project_source::expand(base)
        .replace(
            "#include \"finite_visibility.wgsl\"",
            include_str!("finite_visibility.wgsl"),
        )
        .replace(
            "#include \"ink_plane.wgsl\"",
            include_str!("ink_plane.wgsl"),
        );
    let base = if grid {
        base.replace(
            "const FINITE_GRID:bool=false;",
            "const FINITE_GRID:bool=true;",
        )
    } else {
        base
    };
    let (ty, words) = if varying {
        ("vec2<u32>", "m")
    } else {
        ("u32", "vec2<u32>(m,0x80008000u)")
    };
    let base = format!(
        "alias InkRecord={ty};\nfn ink_words(m:InkRecord)->vec2<u32>{{return {words};}}\n{base}"
    );
    let source = if pick {
```

### 4. `src/renderer.rs`

Move metadata to the existing lane without retaining a duplicate CPU table.

**Locate this existing block:**

```rust
            .expect("Source edge IDs match");
        let planes = crate::clipping_rows::owners(scene);
        let flags = self
            .sources
            .mesh_ids
            .iter()
            .map(|id| {
                crate::sub_id::decode(id[1])
                    .and_then(|(_, address)| self.sources.edges.get(address))
                    .map_or(0, |target| {
                        if planes.contains(&target.owner) {
                            4096
                        } else {
                            0
                        }
                    })
            })
            .collect();
        self.mesh_ink
            .set_flags(&self.device, flags)
            .expect("Original plane ink flags match source addresses");
        self.markers
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            .expect("Source edge IDs match");
        self.mesh_ink
            .set_records(
                &self.device,
                std::mem::take(&mut self.sources.mesh_metadata),
            )
            .expect("Original ink metadata");
        self.markers
```

### 5. `src/pick_sources.rs`

Align explicit numeric records with the existing perimeter batch. Collect clipping owners once per source pass. Reuse the actual contact normal, inverse-transpose placement and existing packed codec. Append one explicit record per existing edge. Control-net edges have no contact facing. Account for actual retained records before upload.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        (self.faces.capacity() + self.edges.capacity() + self.controls.capacity())
            * std::mem::size_of::<Target>()
            + self.mesh_bases.capacity() * std::mem::size_of::<(ObjectId, u32)>()
            + self.mesh_ink.capacity() * std::mem::size_of::<crate::stroke::Stroke>()
```

Replace it with:

```rust
        (self.faces.capacity() + self.edges.capacity() + self.controls.capacity())
            * std::mem::size_of::<Target>()
            + self.mesh_bases.capacity() * std::mem::size_of::<(ObjectId, u32)>()
            + self.mesh_metadata.capacity() * 4
            + self.mesh_ink.capacity() * std::mem::size_of::<crate::stroke::Stroke>()
```

**Locate this block:**

```rust
        for (owner, stroke) in scene.control_nets_at(anchor) {
            sources.mesh_ink.push(stroke);
            sources.mesh_ids.push([owner.value(), 0]);
        }
        for row in scene.control_markers_at(anchor) {
            let ids = append(
```

Replace it with:

```rust
        for (owner, stroke) in scene.control_nets_at(anchor) {
            sources.mesh_ink.push(stroke);
            sources.mesh_ids.push([owner.value(), 0]);
            sources
                .mesh_metadata
                .extend([0, crate::packed_vertex::NO_NORMAL]);
        }
        for row in scene.control_markers_at(anchor) {
            let ids = append(
```

**Locate this block:**

```rust
                    width: 1.,
                });
                sources.mesh_ids.push(id(row.id, ids[1]));
            }
        }
        for row in scene
```

Replace it with:

```rust
                    width: 1.,
                });
                sources.mesh_ids.push(id(row.id, ids[1]));
                sources.mesh_metadata.extend([flag, facing]);
            }
        }
        for row in scene
```

**Locate this block:**

```rust
                    },
                )?;
            }
            let mut cad_edges = row
                .cad
                .as_ref()
```

Replace it with:

```rust
                    },
                )?;
            }
            let mut facing = crate::packed_vertex::NO_NORMAL;
            if row
                .mesh
                .alphas()
                .first()
                .is_some_and(|a| (*a - 1. / 255.).abs() < 0.5 / 255.)
            {
                let n = row.mesh.normals()[0];
                let m = crate::normals::matrix(&row.model)?;
                facing = crate::packed_vertex::oct32(std::array::from_fn(|i| {
                    (0..3).map(|j| m[j * 4 + i] * n[j]).sum()
                }));
            }
            let flag = if planes.contains(&row.id) {
                4096
            } else {
                0
            };
            let mut cad_edges = row
                .cad
                .as_ref()
```

**Locate this block:**

```rust
                },
            ]
        };
        for row in scene
            .objects()
            .iter()
```

Replace it with:

```rust
                },
            ]
        };
        let planes = crate::clipping_rows::owners(scene);
        for row in scene
            .objects()
            .iter()
```

**Locate this block:**

```rust
    pub mesh_ids: Vec<[u32; 2]>,
    pub line_ids: Vec<[u32; 2]>,
    pub path_ids: Vec<[u32; 2]>,
    pub marker_ids: Vec<[u32; 2]>,
```

Replace it with:

```rust
    pub mesh_ids: Vec<[u32; 2]>,
    pub mesh_metadata: Vec<u32>,
    pub line_ids: Vec<[u32; 2]>,
    pub path_ids: Vec<[u32; 2]>,
    pub marker_ids: Vec<[u32; 2]>,
```

### 6. `src/finite_stroke.wgsl`

Read the existing full shared frame. Reuse the surface depth owner; the normal decoder is already assembled. Read flags from the lane’s actual record. Layer both endpoints and retain the bounded depth rise before near clipping. Keep color and integer metadata layouts identical. Share geometry for cropped integer picking. Carry the same flat rise to color and pick fragments. Pass contact rise explicitly into existing visibility.

**Locate this existing block:**

```wgsl
#include "near_clip.wgsl"
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32 }
@group(0) @binding(0) var<uniform> view: View;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
}

fn hidden(colour: vec4<f32>,flags:u32) -> Output {
    return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0));
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32,flags:u32) -> Output {
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    if (near_distance(a) < 0.0 && near_distance(b) < 0.0) { return hidden(colour,flags); }
    a=near_endpoint(a,b);
    b=near_endpoint(b,a);
    if (a.w <= 0.0 || b.w <= 0.0) { return hidden(colour,flags); }
    let delta = (b.xy / b.w - a.xy / a.w) * view.size;
    let length2 = dot(delta, delta);
    if (length2 < 0.000001) { return hidden(colour,flags); }
    let normal = vec2<f32>(-delta.y, delta.x) * inverseSqrt(length2);
    let corners = array<vec2<f32>, 6>(vec2<f32>(0.0, -1.0), vec2<f32>(1.0, -1.0),
        vec2<f32>(0.0, 1.0), vec2<f32>(0.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let corner = corners[index];
    let half_width = width * view.density * 0.5;
    let extent = half_width + 0.70711;
    var position = select(a, b, corner.x > 0.5);
    position = vec4<f32>(position.xy + normal * extent * 2.0 / view.size * corner.y * position.w, position.zw);
    return Output(position, colour, corner.y * extent, half_width,flags,finite_axis(a,b),vec2<f32>(a.z/a.w,b.z/b.w),finite_screen(position));
}

#include "stroke_coverage.wgsl"

@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32,@location(8) flags:u32) -> Output { return geometry(start, end, colour, width, index,flags); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
#include "near_clip.wgsl"
#include "contact_depth.wgsl"
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32, display:vec4<f32>, eye:vec4<f32> }
@group(0) @binding(0) var<uniform> view: View;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
    @location(12) @interpolate(flat) rise:f32,
}

fn hidden(colour: vec4<f32>,flags:u32) -> Output {
    return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0));
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32,metadata:InkRecord) -> Output {
    let words=ink_words(metadata);let flags=words.x;
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    let z=vec2<f32>(a.z,b.z);let n=oct32_decode(words.y);
    a=contact_layer(a,start,n,view.transform,view.size,view.eye);
    b=contact_layer(b,end,n,view.transform,view.size,view.eye);
    let rise=max(0.,max((a.z-z.x)/max(a.w,1e-20),(b.z-z.y)/max(b.w,1e-20)));
    if (near_distance(a) < 0.0 && near_distance(b) < 0.0) { return hidden(colour,flags); }
    a=near_endpoint(a,b);
    b=near_endpoint(b,a);
    if (a.w <= 0.0 || b.w <= 0.0) { return hidden(colour,flags); }
    let delta = (b.xy / b.w - a.xy / a.w) * view.size;
    let length2 = dot(delta, delta);
    if (length2 < 0.000001) { return hidden(colour,flags); }
    let normal = vec2<f32>(-delta.y, delta.x) * inverseSqrt(length2);
    let corners = array<vec2<f32>, 6>(vec2<f32>(0.0, -1.0), vec2<f32>(1.0, -1.0),
        vec2<f32>(0.0, 1.0), vec2<f32>(0.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let corner = corners[index];
    let half_width = width * view.density * 0.5;
    let extent = half_width + 0.70711;
    var position = select(a, b, corner.x > 0.5);
    position = vec4<f32>(position.xy + normal * extent * 2.0 / view.size * corner.y * position.w, position.zw);
    return Output(position, colour, corner.y * extent, half_width,flags,finite_axis(a,b),vec2<f32>(a.z/a.w,b.z/b.w),finite_screen(position));
}

#include "stroke_coverage.wgsl"

@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32,@location(8) metadata:InkRecord) -> Output { return geometry(start, end, colour, width, index,metadata); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false,vec2<f32>(input.rise,0.));if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

### 7. `src/finite_stroke_pick.wgsl`

Keep color and integer metadata layouts identical. Share geometry for cropped integer picking. Carry the same flat rise to color and pick fragments. Pass contact rise explicitly into existing visibility.

**Locate this existing block:**

```wgsl
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
}
@vertex
fn pick_vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>,@location(8) flags:u32) -> PickOutput {
    let source = geometry(start, end, colour, width, index,flags);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.side = source.side;
    output.half_width = source.half_width;
    output.clip_flags=source.clip_flags;
    output.finite_axis_value=source.finite_axis_value;output.finite_depths=source.finite_depths;output.finite_pixel=source.finite_pixel;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.side, input.half_width,input.clip_flags,input.finite_axis_value,input.finite_depths,input.finite_pixel);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true)<=0.){discard;}
    return input.id;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
    @location(12) @interpolate(flat) rise:f32,
}
@vertex
fn pick_vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>,@location(8) metadata:InkRecord) -> PickOutput {
    let source = geometry(start, end, colour, width, index,metadata);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.side = source.side;
    output.half_width = source.half_width;
    output.clip_flags=source.clip_flags;
    output.finite_axis_value=source.finite_axis_value;output.finite_depths=source.finite_depths;output.finite_pixel=source.finite_pixel;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.side, input.half_width,input.clip_flags,input.finite_axis_value,input.finite_depths,input.finite_pixel);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true,vec2<f32>(input.rise,0.))<=0.){discard;}
    return input.id;
```

### 8. `src/finite_stroke.wgsl`

Hidden geometry carries no rise. Keep the actual bounded rise with the source axis.

**Locate this existing block:**

```wgsl
fn hidden(colour: vec4<f32>,flags:u32) -> Output {
    return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0));
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32,metadata:InkRecord) -> Output {
    let words=ink_words(metadata);let flags=words.x;
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    let z=vec2<f32>(a.z,b.z);let n=oct32_decode(words.y);
    a=contact_layer(a,start,n,view.transform,view.size,view.eye);
    b=contact_layer(b,end,n,view.transform,view.size,view.eye);
    let rise=max(0.,max((a.z-z.x)/max(a.w,1e-20),(b.z-z.y)/max(b.w,1e-20)));
    if (near_distance(a) < 0.0 && near_distance(b) < 0.0) { return hidden(colour,flags); }
    a=near_endpoint(a,b);
    b=near_endpoint(b,a);
    if (a.w <= 0.0 || b.w <= 0.0) { return hidden(colour,flags); }
    let delta = (b.xy / b.w - a.xy / a.w) * view.size;
    let length2 = dot(delta, delta);
    if (length2 < 0.000001) { return hidden(colour,flags); }
    let normal = vec2<f32>(-delta.y, delta.x) * inverseSqrt(length2);
    let corners = array<vec2<f32>, 6>(vec2<f32>(0.0, -1.0), vec2<f32>(1.0, -1.0),
        vec2<f32>(0.0, 1.0), vec2<f32>(0.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let corner = corners[index];
    let half_width = width * view.density * 0.5;
    let extent = half_width + 0.70711;
    var position = select(a, b, corner.x > 0.5);
    position = vec4<f32>(position.xy + normal * extent * 2.0 / view.size * corner.y * position.w, position.zw);
    return Output(position, colour, corner.y * extent, half_width,flags,finite_axis(a,b),vec2<f32>(a.z/a.w,b.z/b.w),finite_screen(position));
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
fn hidden(colour: vec4<f32>,flags:u32) -> Output {
    return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0),0.);
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32,metadata:InkRecord) -> Output {
    let words=ink_words(metadata);let flags=words.x;
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    let z=vec2<f32>(a.z,b.z);let n=oct32_decode(words.y);
    a=contact_layer(a,start,n,view.transform,view.size,view.eye);
    b=contact_layer(b,end,n,view.transform,view.size,view.eye);
    let rise=max(0.,max((a.z-z.x)/max(a.w,1e-20),(b.z-z.y)/max(b.w,1e-20)));
    if (near_distance(a) < 0.0 && near_distance(b) < 0.0) { return hidden(colour,flags); }
    a=near_endpoint(a,b);
    b=near_endpoint(b,a);
    if (a.w <= 0.0 || b.w <= 0.0) { return hidden(colour,flags); }
    let delta = (b.xy / b.w - a.xy / a.w) * view.size;
    let length2 = dot(delta, delta);
    if (length2 < 0.000001) { return hidden(colour,flags); }
    let normal = vec2<f32>(-delta.y, delta.x) * inverseSqrt(length2);
    let corners = array<vec2<f32>, 6>(vec2<f32>(0.0, -1.0), vec2<f32>(1.0, -1.0),
        vec2<f32>(0.0, 1.0), vec2<f32>(0.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let corner = corners[index];
    let half_width = width * view.density * 0.5;
    let extent = half_width + 0.70711;
    var position = select(a, b, corner.x > 0.5);
    position = vec4<f32>(position.xy + normal * extent * 2.0 / view.size * corner.y * position.w, position.zw);
    return Output(position, colour, corner.y * extent, half_width,flags,finite_axis(a,b),vec2<f32>(a.z/a.w,b.z/b.w),finite_screen(position),rise);
}
```

### 9. `src/finite_stroke_pick.wgsl`

Copy the geometry’s rise into the cropped output. Preserve the complete shared coverage interface.

**Locate this existing block:**

```wgsl
    output.finite_axis_value=source.finite_axis_value;output.finite_depths=source.finite_depths;output.finite_pixel=source.finite_pixel;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.side, input.half_width,input.clip_flags,input.finite_axis_value,input.finite_depths,input.finite_pixel);
    if (!wanted(input.id.y, crop.mode)) { discard; }
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    output.finite_axis_value=source.finite_axis_value;output.finite_depths=source.finite_depths;output.finite_pixel=source.finite_pixel;
    output.rise=source.rise;output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.side, input.half_width,input.clip_flags,input.finite_axis_value,input.finite_depths,input.finite_pixel,input.rise);
    if (!wanted(input.id.y, crop.mode)) { discard; }
```

### 10. `src/finite_visibility.wgsl`

Expose explicit sag only to segment visibility. Forward the existing axis tolerance. Keep grid behavior unchanged. Reuse existing sag-aware plane and projected-triangle predicates.

**Locate this existing block:**

```wgsl
}


fn finite_alpha(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32,picking:bool)->f32{
 let delta=axis.zw-axis.xy;let t=clamp(dot(pixel-axis.xy,delta)/max(dot(delta,delta),1e-20),0.,1.);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,depth))){return 0.;}
 let hidden=!finite_visible(pixel,axis,depths,flags);return finite_hidden_alpha(hidden,picking);
}

fn finite_disc_alpha(pixel:vec2<f32>,centre:vec2<f32>,depth:f32,flags:u32,picking:bool)->f32{
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(centre,view.size,depth))){return 0.;}
 let tolerance=abs(depth)*DEPTH_REL_TOL+abs(1.-depth)*2.3841858e-7;
 let hidden=!finite_section(centre,depth,tolerance)||!ink_disc_visible(pixel,centre,depth,0u);
 return finite_hidden_alpha(hidden,picking);
}

fn finite_visible(pixel:vec2<f32>,ends:vec4<f32>,depths:vec2<f32>,flags:u32)->bool{
 if(FINITE_GRID){return finite_grid_visible(pixel,ends,depths,flags);}
 let axis=finite_ink_axis(pixel,ends,depths);let tolerance=abs(axis.depth)*DEPTH_REL_TOL+abs(1.-axis.depth)*2.3841858e-7;
 if(!finite_section(axis.at,axis.depth,tolerance)){return false;}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
}


fn finite_alpha(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32,picking:bool,sag:vec2<f32>)->f32{
 let delta=axis.zw-axis.xy;let t=clamp(dot(pixel-axis.xy,delta)/max(dot(delta,delta),1e-20),0.,1.);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,depth))){return 0.;}
 let hidden=!finite_visible(pixel,axis,depths,flags,sag);return finite_hidden_alpha(hidden,picking);
}

fn finite_disc_alpha(pixel:vec2<f32>,centre:vec2<f32>,depth:f32,flags:u32,picking:bool)->f32{
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(centre,view.size,depth))){return 0.;}
 let tolerance=abs(depth)*DEPTH_REL_TOL+abs(1.-depth)*2.3841858e-7;
 let hidden=!finite_section(centre,depth,tolerance)||!ink_disc_visible(pixel,centre,depth,0u);
 return finite_hidden_alpha(hidden,picking);
}

fn finite_visible(pixel:vec2<f32>,ends:vec4<f32>,depths:vec2<f32>,flags:u32,sag:vec2<f32>)->bool{
 if(FINITE_GRID){return finite_grid_visible(pixel,ends,depths,flags);}
 var axis=finite_ink_axis(pixel,ends,depths);axis.sag=sag;let tolerance=abs(axis.depth)*DEPTH_REL_TOL+abs(1.-axis.depth)*2.3841858e-7;
 if(!finite_section(axis.at,axis.depth,tolerance)){return false;}
```

### 11. `src/finite_chain.wgsl`

Ordinary connected paths have no contact rise.

**Locate this existing block:**

```wgsl
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false,vec2<f32>(0.));if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

### 12. `src/finite_chain_pick.wgsl`

Ordinary connected paths have no contact rise.

**Locate this existing block:**

```wgsl
    if (colour(source).a <= 0.0) { discard; }
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true)<=0.){discard;}
    return input.id;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if (colour(source).a <= 0.0) { discard; }
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true,vec2<f32>(0.))<=0.){discard;}
    return input.id;
```

### 13. `src/contact_feature_tests.rs`

Expose the unchanged original-source validation fixture. Share the unchanged fixture projection.

**Locate this existing block:**

```rust
}
fn imported(reverse: bool) -> (Editor, session_rust::proto::Session) {
    let source = original(reverse).to_proto();
    let mut editor = Editor::default();
    editor.scene = Scene::empty();
    crate::scene_document::import(
        &mut editor.scene,
        &source.encode_to_vec(),
        "original-contact",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    editor.camera = crate::camera_defaults_tests::top_fixture();
    editor.camera.distance = 3000.;
    editor.camera.radius = 1500.;
    editor.camera.aspect = 4. / 3.;
    (editor, source)
}
fn retained(editor: &Editor) -> session_rust::proto::Element {
    let Geometry::Element(element) = editor.scene.objects()[0]
        .original
        .as_ref()
        .unwrap()
    else {
        panic!("Original Element retained")
    };
    element.to_proto()
}
fn same_element(a: &session_rust::proto::Element, b: &session_rust::proto::Element) {
    assert_eq!(a.geometry_type, b.geometry_type);
    assert_eq!(a.geometry_type, "Mesh");
    assert_eq!(
        session_rust::proto::Mesh::decode(a.geometry_data.as_slice()).unwrap(),
        session_rust::proto::Mesh::decode(b.geometry_data.as_slice()).unwrap(),
        "Every embedded original mesh field is retained regardless of protobuf map order"
    );
    let (mut a, mut b) = (a.clone(), b.clone());
    a.geometry_data.clear();
    b.geometry_data.clear();
    assert_eq!(
        a, b,
        "Every original Element metadata/feature field is retained"
    );
}
fn project(matrix: [f32; 16], p: [f64; 3]) -> [f32; 2] {
    let p = p.map(|v| v as f32);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}
pub(super) fn imported(reverse: bool) -> (Editor, session_rust::proto::Session) {
    let source = original(reverse).to_proto();
    let mut editor = Editor::default();
    editor.scene = Scene::empty();
    crate::scene_document::import(
        &mut editor.scene,
        &source.encode_to_vec(),
        "original-contact",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    editor.camera = crate::camera_defaults_tests::top_fixture();
    editor.camera.distance = 3000.;
    editor.camera.radius = 1500.;
    editor.camera.aspect = 4. / 3.;
    (editor, source)
}
fn retained(editor: &Editor) -> session_rust::proto::Element {
    let Geometry::Element(element) = editor.scene.objects()[0]
        .original
        .as_ref()
        .unwrap()
    else {
        panic!("Original Element retained")
    };
    element.to_proto()
}
fn same_element(a: &session_rust::proto::Element, b: &session_rust::proto::Element) {
    assert_eq!(a.geometry_type, b.geometry_type);
    assert_eq!(a.geometry_type, "Mesh");
    assert_eq!(
        session_rust::proto::Mesh::decode(a.geometry_data.as_slice()).unwrap(),
        session_rust::proto::Mesh::decode(b.geometry_data.as_slice()).unwrap(),
        "Every embedded original mesh field is retained regardless of protobuf map order"
    );
    let (mut a, mut b) = (a.clone(), b.clone());
    a.geometry_data.clear();
    b.geometry_data.clear();
    assert_eq!(
        a, b,
        "Every original Element metadata/feature field is retained"
    );
}
pub(super) fn project(matrix: [f32; 16], p: [f64; 3]) -> [f32; 2] {
    let p = p.map(|v| v as f32);
```

### 14. `src/ink_metadata_tests.rs`

Indexed contact-capable records now carry two words; ordinary8192-instance assertions remain exactly4bytes.

**Locate this existing block:**

```rust
        .unwrap();
    assert_eq!(varying.clipping_bytes(), [12, 12]);
    assert!(varying.set_flags(device, vec![0]).is_err());
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .unwrap();
    assert_eq!(varying.clipping_bytes(), [24, 24]);
    assert!(varying.set_flags(device, vec![0]).is_err());
```

### 15. `src/contact_boundary_tests.rs`

Validate actual original contact pixels, complete projected fallback, transformed normals and owner picks.

Create the file and type:

```rust
use crate::{camera::Projection, editor::Action, renderer::Renderer, scene::Scene};
use session_rust::{Mesh, Point, Xform};
fn scene(
    angle: f64,
    projection: Projection,
    mirror: bool,
) -> (crate::editor::Editor, crate::scene::ObjectId) {
    let (mut editor, _) = super::contact_feature_tests::imported(false);
    editor.camera.projection = projection;
    let owner = editor
        .scene
        .objects()
        .iter()
        .find(|r| !editor.scene.forest.derived(r.id))
        .unwrap()
        .id;
    editor.select_objects([owner], false);
    let model = &Xform::rotation_x(angle, true)
        * &Xform::scale_xyz(
            if mirror {
                -1.3
            } else {
                1.
            },
            0.8,
            1.7,
        );
    editor
        .apply(Action::Transformation {
            kind: crate::transform_draft::Kind::Rotate,
            delta: model.clone(),
        })
        .unwrap();
    editor.select_objects([], false);
    assert!(
        editor
            .scene
            .objects()
            .iter()
            .all(|r| r.model.m == model.m),
        "The actual parent and contact must both follow the edit"
    );
    for i in 0..20 {
        let x = 10000. + i as f64 * 20.;
        let mesh = Mesh::from_vertices_and_faces(
            vec![
                Point::new(x, 0., -20.),
                Point::new(x + 10., 0., -20.),
                Point::new(x + 10., 10., -20.),
                Point::new(x, 10., -20.),
            ],
            vec![vec![0, 1, 2, 3]],
        );
        editor
            .scene
            .insert(crate::prepared::PreparedMesh::new(mesh).unwrap())
            .unwrap();
    }
    (editor, owner)
}
fn renderer(scene: &Scene, bounded: bool) -> Renderer {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let adapter =
        pollster::block_on(instance.request_adapter(&Default::default())).unwrap();
    let mut descriptor = wgpu::DeviceDescriptor::default();
    if bounded {
        descriptor
            .required_limits
            .max_storage_buffer_binding_size = 2048;
    }
    let (device, queue) =
        pollster::block_on(adapter.request_device(&descriptor)).unwrap();
    device.on_uncaptured_error(std::sync::Arc::new(|e| {
        panic!("Actual contact perimeter: {e}")
    }));
    let mut renderer = Renderer::new(
        device,
        queue,
        wgpu::TextureFormat::Rgba8Unorm,
        scene,
    );
    renderer.resize(crate::viewport::Viewport {
        width: 320,
        height: 240,
    });
    renderer.set_opacity(1.);
    renderer
}
#[test]
fn actual_contact_perimeters_survive_bounded_projection_fallback() {
    for projection in [
        Projection::Orthographic,
        Projection::Perspective,
    ] {
        for angle in [0., 60., 75., 85.] {
            for mirror in [false, true] {
                let (editor, owner) = scene(angle, projection, mirror);
                let matrix = editor.camera.uniform();
                let (mut full, mut bounded) = (
                    renderer(&editor.scene, false),
                    renderer(&editor.scene, true),
                );
                for density in [1., 2.] {
                    full.set_density(density);
                    bounded.set_density(density);
                    let a = crate::cap_gpu_tests::pixels_at(&full, matrix);
                    let b = crate::cap_gpu_tests::pixels_at(&bounded, matrix);
                    assert!(full.tile_usage()[0] > 96);
                    assert_eq!(
                        &bounded.tile_usage()[..2],
                        &[96, 16],
                        "Real storage limit exercises bounded fallback"
                    );
                    let corners = [
                        [-550., -350., 0.],
                        [550., -350., 0.],
                        [550., 350., 0.],
                        [-550., 350., 0.],
                    ]
                    .map(|p| {
                        let p = editor
                            .scene
                            .placement(owner)
                            .unwrap()
                            .transform_point(&Point::new(p[0], p[1], p[2]));
                        let at = super::contact_feature_tests::project(
                            matrix,
                            [p[0], p[1], p[2]],
                        );
                        [(at[0] + 1.) * 160., (1. - at[1]) * 120.]
                    });
                    let min = std::array::from_fn::<_, 2, _>(|i| {
                        (corners
                            .iter()
                            .map(|p| p[i])
                            .fold(f32::INFINITY, f32::min)
                            - 2. * density)
                            .floor()
                            .max(0.) as usize
                    });
                    let max = std::array::from_fn::<_, 2, _>(|i| {
                        (corners
                            .iter()
                            .map(|p| p[i])
                            .fold(f32::NEG_INFINITY, f32::max)
                            + 2. * density)
                            .ceil()
                            .min([320., 240.][i]) as usize
                    });
                    let differences = (min[1]..max[1])
                        .flat_map(|y| (min[0]..max[0]).map(move |x| (x, y)))
                        .filter(|(x, y)| {
                            let at = (y * 320 + x) * 4;
                            a[at..at + 4] != b[at..at + 4]
                        })
                        .collect::<Vec<_>>();
                    assert!(
                        differences.is_empty(),
                        "Every pixel of the contact, complete perimeter, notch and surrounding band matches exactly: {projection:?}, {angle}, mirror={mirror}, density={density}, different={differences:?}"
                    );
                    let local = Point::new(-350., -150., 0.);
                    let p = editor
                        .scene
                        .placement(owner)
                        .unwrap()
                        .transform_point(&local);
                    let at = super::contact_feature_tests::project(
                        matrix,
                        [p[0], p[1], p[2]],
                    );
                    for r in [&mut full, &mut bounded] {
                        r.request_pick(at, matrix).unwrap();
                        crate::cap_gpu_tests::wait(r);
                        assert_eq!(
                            r.take_pick(matrix)
                                .unwrap()
                                .unwrap()
                                .unwrap()[0],
                            owner.value()
                        );
                    }
                }
            }
        }
    }
}
#[test]
fn contact_metadata_follows_original_parent_placement() {
    for mirror in [false, true] {
        let (editor, owner) = scene(75., Projection::Perspective, mirror);
        let sources = crate::pick_sources::Sources::new(&editor.scene).unwrap();
        assert_eq!(
            sources.mesh_metadata.len(),
            sources.mesh_ink.len() * 2
        );
        let contact = editor
            .scene
            .objects()
            .iter()
            .find(|r| editor.scene.forest.derived(r.id))
            .unwrap();
        let normal = contact.mesh.normals()[0];
        let m = crate::normals::matrix(&contact.model).unwrap();
        let expected = crate::packed_vertex::oct32(std::array::from_fn(|i| {
            (0..3)
                .map(|j| m[j * 4 + i] * normal[j])
                .sum()
        }));
        let contacts = sources
            .mesh_metadata
            .chunks_exact(2)
            .zip(&sources.mesh_ids)
            .filter(|(m, _)| m[1] != crate::packed_vertex::NO_NORMAL)
            .collect::<Vec<_>>();
        assert_eq!(contacts.len(), 9);
        assert!(
            contacts
                .iter()
                .all(|(m, id)| *m == [0, expected] && id[0] == owner.value())
        );
        let archive = crate::scene_archive::save(&editor.scene).unwrap();
        let mut restored = Scene::empty();
        crate::scene_document::open(&mut restored, &archive).unwrap();
        let again = crate::pick_sources::Sources::new(&restored).unwrap();
        assert_eq!(again.mesh_metadata, sources.mesh_metadata);
        assert!(restored.paths().is_empty());
    }
}
```

### 16. `src/renderer.rs`

Connect optional native validation outside the browser runtime.

**Locate this existing block:**

```rust
#[path = "ink_metadata_tests.rs"]
mod ink_metadata_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[path = "ink_metadata_tests.rs"]
mod ink_metadata_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "contact_boundary_tests.rs"]
mod contact_boundary_tests;
```

## <a id="checkpoint-96zzzzzzm-query-original-faces-under-contact-overlays"></a>5. Query faces under contact overlays

**What this code does:** Select the actual original face beneath a readonly contact, without inventing an editable face for its display polygon.

### 1. `src/gpu_mesh.rs`

Retain source-query eligibility on the existing CPU mesh. Classify original and derived display rows explicitly. Standalone original geometry and the existing highlight adapter remain valid. Keep the flag out of the packed GPU record. Update eligibility without a GPU setting write.

**Locate this existing block:**

```rust
    pub id: crate::scene::ObjectId,
    model: wgpu::Buffer,
    translation: [f64; 3],
    anchor: [f64; 3],
    previous: Record,
    primitive_base: u32,
    xray: bool,
    cap_flags: u32,
    cap_token: u32,
}

#[derive(Clone, Copy)]
pub struct Style {
    pub selected: bool,
    pub owner: u32,
    pub colour: Option<[f32; 4]>,
}

fn settings(
    object: &Object,
    style: Style,
    cap_flags: u32,
    cap_token: u32,
    primitive_base: u32,
    index_start: u32,
    anchor: [f64; 3],
) -> Record {
    let mut bytes = [0; BYTES];
    let model = crate::render_space::placed(&object.model, anchor).to_f32();
    for (index, value) in model.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&u32::from(style.selected).to_ne_bytes());
    let id = style.owner;
    bytes[68..72].copy_from_slice(&id.to_ne_bytes());
    let flags = if matches!(&object.original,Some(session_rust::Geometry::Plane(p)) if crate::clipping_geometry::is_clipping(p))
    {
        4096u32
    } else {
        0
    };
    bytes[72..76].copy_from_slice(&(flags | cap_flags).to_ne_bytes());
    bytes[76..80].copy_from_slice(&cap_token.to_ne_bytes());
    let normals = crate::normals::matrix(&object.model)
        .expect("Scene placements are checked before insertion or editing");
    for (index, value) in normals.iter().enumerate() {
        bytes[80 + index * 4..84 + index * 4].copy_from_slice(&value.to_ne_bytes());
    }
    if let Some(colour) = style.colour {
        for (index, value) in colour[..3]
            .iter()
            .copied()
            .chain([1.])
            .enumerate()
        {
            bytes[144 + index * 4..148 + index * 4]
                .copy_from_slice(&value.to_ne_bytes());
        }
    }
    bytes[RANGE..RANGE + 4].copy_from_slice(&index_start.to_ne_bytes());
    bytes[RANGE + 4..RANGE + 8].copy_from_slice(&primitive_base.to_ne_bytes());
    bytes
}

impl GpuMesh {
    pub fn upload(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        selected: bool,
    ) -> Self {
        Self::upload_at(device, layout, object, selected, [0.; 3])
    }
    pub fn upload_at(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        selected: bool,
        anchor: [f64; 3],
    ) -> Self {
        let geometry = Rc::new(GpuGeometry::upload(
            device,
            Rc::clone(&object.mesh),
        ));
        Self::with_geometry_at(
            device,
            layout,
            object,
            Style {
                selected,
                owner: if object.metadata.is_locked {
                    0
                } else {
                    object.id.value()
                },
                colour: None,
            },
            geometry,
            anchor,
        )
    }

    pub fn with_geometry(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        style: Style,
        geometry: Rc<GpuGeometry>,
    ) -> Self {
        Self::with_geometry_at(
            device, layout, object, style, geometry, [0.; 3],
        )
    }
    pub fn with_geometry_at(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        style: Style,
        geometry: Rc<GpuGeometry>,
        anchor: [f64; 3],
    ) -> Self {
        let bytes = settings(
            object,
            style,
            0,
            0,
            0,
            geometry.index_start,
            anchor,
        );
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
            translation: object.model.m[12..15].try_into().unwrap(),
            anchor,
            geometry,
            model_group,
            id: object.id,
            model,
            previous: bytes,
            primitive_base: 0,
            xray: false,
            cap_flags: 0,
            cap_token: 0,
        }
    }

    pub fn rebase(&mut self, queue: &wgpu::Queue, anchor: [f64; 3]) -> [usize; 2] {
        assert!(anchor.iter().all(|v| v.is_finite()));
        self.anchor = anchor;
        let bytes = crate::render_space::relative(self.translation, anchor)
            .into_iter()
            .flat_map(f32::to_ne_bytes)
            .collect::<Vec<_>>();
        if self.previous[48..60] == bytes {
            return [0, 0];
        }
        self.previous[48..60].copy_from_slice(&bytes);
        queue.write_buffer(&self.model, 48, &bytes);
        [1, 12]
    }
    pub(crate) fn model_buffer(&self) -> &wgpu::Buffer {
        &self.model
    }
    pub(crate) fn packed(&self) -> Record {
        self.previous
    }
    pub fn selected(&self) -> bool {
        u32::from_ne_bytes(self.previous[64..68].try_into().unwrap()) & 1 != 0
    }

    pub fn set_ambient_radius(&mut self, queue: &wgpu::Queue, radius: f32) {
        let bytes = radius.to_ne_bytes();
        if self.previous[RANGE + 8..RANGE + 12] != bytes {
            self.previous[RANGE + 8..RANGE + 12].copy_from_slice(&bytes);
            queue.write_buffer(&self.model, (RANGE + 8) as u64, &bytes);
        }
    }
    pub fn primitive_base(&self) -> u32 {
        self.primitive_base
    }
    pub fn set_primitive_base(
        &mut self,
        queue: &wgpu::Queue,
        base: u32,
    ) -> Result<(), &'static str> {
        if base
            .checked_add(self.geometry.index_count / 3)
            .is_none_or(|end| end >= crate::primitive_image::CAP_START)
        {
            return Err("Primitive range overlaps reserved caps");
        }
        if self.primitive_base != base {
            self.previous[RANGE + 4..RANGE + 8].copy_from_slice(&base.to_ne_bytes());
            queue.write_buffer(
                &self.model,
                (RANGE + 4) as u64,
                &self.previous[RANGE + 4..RANGE + 8],
            );
            self.primitive_base = base;
        }
        Ok(())
    }

    pub fn set_xray(&mut self, queue: &wgpu::Queue, on: bool) {
        if self.xray == on {
            return;
        }
        self.xray = on;
        let flags = u32::from_ne_bytes(self.previous[72..76].try_into().unwrap());
        let flags = (flags & !32768)
            | if on && flags & 8192 != 0 {
                32768
            } else {
                0
            };
        self.previous[72..76].copy_from_slice(&flags.to_ne_bytes());
        queue.write_buffer(&self.model, 72, &self.previous[72..76]);
    }
    pub fn closed(&self) -> bool {
        !self.xray && self.cap_flags & crate::solid_orientation::CLOSED != 0
    }
    pub fn pickable(&self) -> bool {
        u32::from_ne_bytes(self.previous[68..72].try_into().unwrap()) != 0
    }
    pub fn set_caps(&mut self, queue: &wgpu::Queue, flags: u32, token: u32) {
        if self.cap_flags == flags && self.cap_token == token {
            return;
        }
        let plane =
            u32::from_ne_bytes(self.previous[72..76].try_into().unwrap()) & 4096;
        self.previous[72..76].copy_from_slice(
            &(plane
                | flags
                | if self.xray && flags & 8192 != 0 {
                    32768
                } else {
                    0
                })
            .to_ne_bytes(),
        );
        self.previous[76..80].copy_from_slice(&token.to_ne_bytes());
        queue.write_buffer(&self.model, 72, &self.previous[72..80]);
        self.cap_flags = flags;
        self.cap_token = token;
    }
    pub fn settings_bytes(&self) -> u64 {
        self.model.size()
    }

    pub fn update(
        &mut self,
        queue: &wgpu::Queue,
        object: &Object,
        style: Style,
    ) -> [usize; 2] {
        self.translation = object.model.m[12..15].try_into().unwrap();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub id: crate::scene::ObjectId,
    pub source_faces: bool,
    model: wgpu::Buffer,
    translation: [f64; 3],
    anchor: [f64; 3],
    previous: Record,
    primitive_base: u32,
    xray: bool,
    cap_flags: u32,
    cap_token: u32,
}

#[derive(Clone, Copy)]
pub struct Style {
    pub source_faces: bool,
    pub selected: bool,
    pub owner: u32,
    pub colour: Option<[f32; 4]>,
}

fn settings(
    object: &Object,
    style: Style,
    cap_flags: u32,
    cap_token: u32,
    primitive_base: u32,
    index_start: u32,
    anchor: [f64; 3],
) -> Record {
    let mut bytes = [0; BYTES];
    let model = crate::render_space::placed(&object.model, anchor).to_f32();
    for (index, value) in model.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&u32::from(style.selected).to_ne_bytes());
    let id = style.owner;
    bytes[68..72].copy_from_slice(&id.to_ne_bytes());
    let flags = if matches!(&object.original,Some(session_rust::Geometry::Plane(p)) if crate::clipping_geometry::is_clipping(p))
    {
        4096u32
    } else {
        0
    };
    bytes[72..76].copy_from_slice(&(flags | cap_flags).to_ne_bytes());
    bytes[76..80].copy_from_slice(&cap_token.to_ne_bytes());
    let normals = crate::normals::matrix(&object.model)
        .expect("Scene placements are checked before insertion or editing");
    for (index, value) in normals.iter().enumerate() {
        bytes[80 + index * 4..84 + index * 4].copy_from_slice(&value.to_ne_bytes());
    }
    if let Some(colour) = style.colour {
        for (index, value) in colour[..3]
            .iter()
            .copied()
            .chain([1.])
            .enumerate()
        {
            bytes[144 + index * 4..148 + index * 4]
                .copy_from_slice(&value.to_ne_bytes());
        }
    }
    bytes[RANGE..RANGE + 4].copy_from_slice(&index_start.to_ne_bytes());
    bytes[RANGE + 4..RANGE + 8].copy_from_slice(&primitive_base.to_ne_bytes());
    bytes
}

impl GpuMesh {
    pub fn upload(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        selected: bool,
    ) -> Self {
        Self::upload_at(device, layout, object, selected, [0.; 3])
    }
    pub fn upload_at(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        selected: bool,
        anchor: [f64; 3],
    ) -> Self {
        let geometry = Rc::new(GpuGeometry::upload(
            device,
            Rc::clone(&object.mesh),
        ));
        Self::with_geometry_at(
            device,
            layout,
            object,
            Style {
                source_faces: true,
                selected,
                owner: if object.metadata.is_locked {
                    0
                } else {
                    object.id.value()
                },
                colour: None,
            },
            geometry,
            anchor,
        )
    }

    pub fn with_geometry(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        style: Style,
        geometry: Rc<GpuGeometry>,
    ) -> Self {
        Self::with_geometry_at(
            device, layout, object, style, geometry, [0.; 3],
        )
    }
    pub fn with_geometry_at(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        object: &Object,
        style: Style,
        geometry: Rc<GpuGeometry>,
        anchor: [f64; 3],
    ) -> Self {
        let bytes = settings(
            object,
            style,
            0,
            0,
            0,
            geometry.index_start,
            anchor,
        );
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
            source_faces: style.source_faces,
            translation: object.model.m[12..15].try_into().unwrap(),
            anchor,
            geometry,
            model_group,
            id: object.id,
            model,
            previous: bytes,
            primitive_base: 0,
            xray: false,
            cap_flags: 0,
            cap_token: 0,
        }
    }

    pub fn rebase(&mut self, queue: &wgpu::Queue, anchor: [f64; 3]) -> [usize; 2] {
        assert!(anchor.iter().all(|v| v.is_finite()));
        self.anchor = anchor;
        let bytes = crate::render_space::relative(self.translation, anchor)
            .into_iter()
            .flat_map(f32::to_ne_bytes)
            .collect::<Vec<_>>();
        if self.previous[48..60] == bytes {
            return [0, 0];
        }
        self.previous[48..60].copy_from_slice(&bytes);
        queue.write_buffer(&self.model, 48, &bytes);
        [1, 12]
    }
    pub(crate) fn model_buffer(&self) -> &wgpu::Buffer {
        &self.model
    }
    pub(crate) fn packed(&self) -> Record {
        self.previous
    }
    pub fn selected(&self) -> bool {
        u32::from_ne_bytes(self.previous[64..68].try_into().unwrap()) & 1 != 0
    }

    pub fn set_ambient_radius(&mut self, queue: &wgpu::Queue, radius: f32) {
        let bytes = radius.to_ne_bytes();
        if self.previous[RANGE + 8..RANGE + 12] != bytes {
            self.previous[RANGE + 8..RANGE + 12].copy_from_slice(&bytes);
            queue.write_buffer(&self.model, (RANGE + 8) as u64, &bytes);
        }
    }
    pub fn primitive_base(&self) -> u32 {
        self.primitive_base
    }
    pub fn set_primitive_base(
        &mut self,
        queue: &wgpu::Queue,
        base: u32,
    ) -> Result<(), &'static str> {
        if base
            .checked_add(self.geometry.index_count / 3)
            .is_none_or(|end| end >= crate::primitive_image::CAP_START)
        {
            return Err("Primitive range overlaps reserved caps");
        }
        if self.primitive_base != base {
            self.previous[RANGE + 4..RANGE + 8].copy_from_slice(&base.to_ne_bytes());
            queue.write_buffer(
                &self.model,
                (RANGE + 4) as u64,
                &self.previous[RANGE + 4..RANGE + 8],
            );
            self.primitive_base = base;
        }
        Ok(())
    }

    pub fn set_xray(&mut self, queue: &wgpu::Queue, on: bool) {
        if self.xray == on {
            return;
        }
        self.xray = on;
        let flags = u32::from_ne_bytes(self.previous[72..76].try_into().unwrap());
        let flags = (flags & !32768)
            | if on && flags & 8192 != 0 {
                32768
            } else {
                0
            };
        self.previous[72..76].copy_from_slice(&flags.to_ne_bytes());
        queue.write_buffer(&self.model, 72, &self.previous[72..76]);
    }
    pub fn closed(&self) -> bool {
        !self.xray && self.cap_flags & crate::solid_orientation::CLOSED != 0
    }
    pub fn pickable(&self) -> bool {
        u32::from_ne_bytes(self.previous[68..72].try_into().unwrap()) != 0
    }
    pub fn set_caps(&mut self, queue: &wgpu::Queue, flags: u32, token: u32) {
        if self.cap_flags == flags && self.cap_token == token {
            return;
        }
        let plane =
            u32::from_ne_bytes(self.previous[72..76].try_into().unwrap()) & 4096;
        self.previous[72..76].copy_from_slice(
            &(plane
                | flags
                | if self.xray && flags & 8192 != 0 {
                    32768
                } else {
                    0
                })
            .to_ne_bytes(),
        );
        self.previous[76..80].copy_from_slice(&token.to_ne_bytes());
        queue.write_buffer(&self.model, 72, &self.previous[72..80]);
        self.cap_flags = flags;
        self.cap_token = token;
    }
    pub fn settings_bytes(&self) -> u64 {
        self.model.size()
    }

    pub fn update(
        &mut self,
        queue: &wgpu::Queue,
        object: &Object,
        style: Style,
    ) -> [usize; 2] {
        self.source_faces = style.source_faces;
        self.translation = object.model.m[12..15].try_into().unwrap();
```

### 2. `src/renderer.rs`

Instances and locked originals are not generated attribute display pieces.

**Locate this existing block:**

```rust
    crate::gpu_mesh::Style {
        selected,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    crate::gpu_mesh::Style {
        source_faces: !scene.forest.derived(id),
        selected,
```

### 3. `src/pick_sources.rs`

Classify the source row once, then append only its actual original faces.

**Locate this existing block:**

```rust
            ));
            for &key in &row.mesh.parts.faces {
                append(
                    &mut sources.faces,
                    Kind::Face,
                    Target {
                        owner: row.id,
                        part: Part::Face(key),
                    },
                )?;
            }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            ));
            if !scene.forest.derived(row.id) {
                for &key in &row.mesh.parts.faces {
                    append(
                        &mut sources.faces,
                        Kind::Face,
                        Target {
                            owner: row.id,
                            part: Part::Face(key),
                        },
                    )?;
                }
            }
```

### 4. `src/face_pick.rs`

Do not allocate generated display-face metadata; preserve one row per mesh.

**Locate this existing block:**

```rust
            let geometry = &mesh.geometry;
            if geometry.source.parts.triangles.is_empty() {
                self.rows.push(None);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            let geometry = &mesh.geometry;
            if !mesh.source_faces || geometry.source.parts.triangles.is_empty() {
                self.rows.push(None);
```

### 5. `src/pick_encode.rs`

Ignore derived depth only for source-face queries; keep all original occluders.

**Locate this existing block:**

```rust
        for mesh in meshes {
            if !records
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        for mesh in meshes {
            if mode == crate::pick_sources::Mode::Face && !mesh.source_faces {
                continue;
            }
            if !records
```

### 6. `src/global_outline_tests.rs`

Preserve the unchanged original-source uniform/reference test.

**Locate this existing block:**

```rust
    let style = crate::gpu_mesh::Style {
        selected: false,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let style = crate::gpu_mesh::Style {
        source_faces: true,
        selected: false,
```

### 7. `src/contact_feature_tests.rs`

Reuse the unchanged original contact ring validation fixture.

**Locate this existing block:**

```rust
use session_rust::{Element, Geometry, Mesh, Point, Polyline, Session, Xform};

fn polygon() -> Polyline {
    Polyline::new(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use session_rust::{Element, Geometry, Mesh, Point, Polyline, Session, Xform};

pub(super) fn polygon() -> Polyline {
    Polyline::new(
```

### 8. `src/source_face_policy_tests.rs`

Supply actual original face GPU/source/archive, locked occlusion, instance and BRep controls.

Create the file and type:

```rust
use crate::{
    camera::Projection,
    editor::{Action, Editor},
    pick_sources::{Mode, Part, Sources, Target},
    renderer::Renderer,
    scene::Scene,
};
use prost::Message;
use session_rust::{
    Element, Mesh, Point, Session, Xform,
    element::{ElementFeature, ElementGeometry},
    proto,
};

fn export_source(name: &str, editor: &Editor) {
    if let Some(root) = std::env::var_os("VIEWER_SOURCE_FACE_PROOF_DIR") {
        let root = std::path::Path::new(&root);
        std::fs::create_dir_all(root).unwrap();
        let archive = crate::scene_archive::save(&editor.scene).unwrap();
        let bundle = crate::archive::Bundle::decode(&archive).unwrap();
        assert_eq!(bundle.documents.len(), 1);
        std::fs::write(root.join(format!("{name}.session")), archive).unwrap();
        std::fs::write(
            root.join(format!("{name}.pb")),
            &bundle.documents[0],
        )
        .unwrap();
    }
}
fn export_picture(name: &str, pixels: &[u8]) {
    if let Some(root) = std::env::var_os("VIEWER_SOURCE_FACE_PROOF_DIR") {
        use std::io::Write;
        let mut file = std::fs::File::create(
            std::path::Path::new(&root).join(format!("{name}.ppm")),
        )
        .unwrap();
        write!(file, "P6\n320 240\n255\n").unwrap();
        for p in pixels.chunks_exact(4) {
            file.write_all(&p[..3]).unwrap();
        }
    }
}
fn panel(sparse: bool, half: f64) -> Mesh {
    let mut mesh = Mesh::from_vertices_and_faces(
        vec![
            Point::new(-half, -650., 0.),
            Point::new(0., -650., 0.),
            Point::new(half, -650., 0.),
            Point::new(-half, 650., 0.),
            Point::new(0., 650., 0.),
            Point::new(half, 650., 0.),
        ],
        vec![vec![0, 1, 4, 3], vec![1, 2, 5, 4]],
    );
    if sparse {
        mesh.remove_face(0);
        mesh.remove_face(1);
        assert_eq!(
            mesh.add_face(vec![0, 1, 4, 3], Some(17)),
            Some(17)
        );
        assert_eq!(
            mesh.add_face(vec![1, 2, 5, 4], Some(93)),
            Some(93)
        );
    }
    mesh.set_objectcolor(session_rust::Color::new(0.2, 0.4, 0.6, 1.));
    mesh
}
fn imported(geometry: ElementGeometry, z: f64) -> Editor {
    let mut element = Element::new("Original faces beneath readonly contact");
    element.set_element_geometry(geometry);
    let mut outline = super::contact_feature_tests::polygon();
    for p in outline.coords.chunks_exact_mut(3) {
        p[2] = z;
    }
    element.add_feature(ElementFeature::new(
        "contact",
        0,
        vec![outline],
        "readonly contact",
    ));
    let mut source = Session::new("Original face query");
    source.add_element(element, None);
    let mut editor = Editor::default();
    editor.scene = Scene::empty();
    crate::scene_document::import(
        &mut editor.scene,
        &source.to_proto().encode_to_vec(),
        "original-face-query",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    editor.camera = crate::camera_defaults_tests::top_fixture();
    editor.camera.distance = 3000.;
    editor.camera.radius = 1500.;
    editor.camera.aspect = 4. / 3.;
    editor
}
fn point(editor: &Editor, local: [f64; 3]) -> [f64; 3] {
    let p = editor.scene.objects()[0]
        .model
        .transform_point(&Point::new(local[0], local[1], local[2]));
    [p[0], p[1], p[2]]
}
fn pick(
    r: &mut Renderer,
    scene: &Scene,
    matrix: [f32; 16],
    p: [f64; 3],
    mode: Mode,
) -> Option<Target> {
    r.set_pick_mode(mode);
    r.request_pick(
        super::contact_feature_tests::project(matrix, p),
        matrix,
    )
    .unwrap();
    crate::cap_gpu_tests::wait(r);
    r.take_pick(matrix)
        .unwrap()
        .unwrap()
        .and_then(|raw| r.resolve_pick(scene, raw))
}
fn archive(
    scene: &Scene,
) -> (
    Vec<proto::Session>,
    Vec<proto::Mesh>,
    Vec<proto::BRep>,
    serde_json::Value,
) {
    let bundle =
        crate::archive::Bundle::decode(&crate::scene_archive::save(scene).unwrap())
            .unwrap();
    let mut docs = bundle
        .documents
        .iter()
        .map(|b| proto::Session::decode(b.as_slice()).unwrap())
        .collect::<Vec<_>>();
    let (mut meshes, mut breps) = (Vec::new(), Vec::new());
    for doc in &mut docs {
        for objects in doc
            .objects
            .iter_mut()
            .chain(doc.definitions.iter_mut())
        {
            for element in &mut objects.elements {
                match element.geometry_type.as_str() {
                    "Mesh" => {
                        meshes.push(
                            proto::Mesh::decode(element.geometry_data.as_slice())
                                .unwrap(),
                        );
                        element.geometry_data.clear();
                    }
                    "BRep" => {
                        breps.push(
                            proto::BRep::decode(element.geometry_data.as_slice())
                                .unwrap(),
                        );
                        element.geometry_data.clear();
                    }
                    _ => {}
                }
            }
        }
    }
    (
        docs,
        meshes,
        breps,
        serde_json::to_value(&bundle.metadata).unwrap(),
    )
}
fn pixel<'a>(image: &'a [u8], matrix: [f32; 16], point: [f64; 3]) -> &'a [u8] {
    let p = super::contact_feature_tests::project(matrix, point);
    let x = ((p[0] + 1.) * 160.).floor() as usize;
    let y = ((1. - p[1]) * 120.).floor() as usize;
    crate::cap_gpu_tests::at(image, x, y)
}

#[test]
fn source_faces_under_contacts_retain_sparse_keys_and_original_pixels() {
    for mirror in [false, true] {
        let mut editor = imported(ElementGeometry::Mesh(panel(true, 900.)), 0.);
        let owner = editor.scene.objects()[0].id;
        if !mirror {
            export_source("sparse", &editor);
        }
        editor.select_objects([owner], false);
        editor
            .apply(Action::Transformation {
                kind: crate::transform_draft::Kind::Rotate,
                delta: &Xform::rotation_x(30., true)
                    * &Xform::scale_xyz(
                        if mirror {
                            -1.2
                        } else {
                            1.
                        },
                        0.8,
                        1.4,
                    ),
            })
            .unwrap();
        editor.select_objects([], false);
        let original = archive(&editor.scene);
        let sources = Sources::new(&editor.scene).unwrap();
        assert_eq!(
            sources
                .faces
                .iter()
                .map(|t| t.part)
                .collect::<Vec<_>>(),
            [Part::Face(17), Part::Face(93)]
        );
        assert!(
            sources
                .faces
                .iter()
                .all(|t| t.owner == owner)
        );
        assert_eq!(sources.mesh_bases.len(), 2);
        let mut r = crate::cap_gpu_tests::renderer(&editor.scene);
        r.set_opacity(1.);
        let uploads = r.stats()[0];
        for projection in [
            Projection::Orthographic,
            Projection::Perspective,
        ] {
            editor.camera.projection = projection;
            let matrix = editor.camera.uniform();
            for density in [1., 2.] {
                r.set_density(density);
                let before = crate::cap_gpu_tests::pixels_at(&r, matrix);
                if !mirror && projection == Projection::Orthographic && density == 1. {
                    export_picture("sparse-native-original", &before);
                }
                for (p, key) in [
                    ([-350., -150., 0.], 17),
                    ([350., -150., 0.], 93),
                ] {
                    let p = point(&editor, p);
                    assert_eq!(
                        pick(&mut r, &editor.scene, matrix, p, Mode::Face),
                        Some(Target {
                            owner,
                            part: Part::Face(key)
                        })
                    );
                    assert_eq!(
                        pick(
                            &mut r,
                            &editor.scene,
                            matrix,
                            p,
                            Mode::Object
                        ),
                        Some(Target {
                            owner,
                            part: Part::Object
                        })
                    );
                }
                let target = Target {
                    owner,
                    part: Part::Face(17),
                };
                r.set_highlight(&editor.scene, Some(target))
                    .unwrap();
                let selected = crate::cap_gpu_tests::pixels_at(&r, matrix);
                if !mirror && projection == Projection::Orthographic && density == 1. {
                    export_picture("sparse-native-left-face", &selected);
                }
                let yellow = pixel(
                    &selected,
                    matrix,
                    point(&editor, [-750., -450., 0.]),
                );
                assert!(
                    yellow[0] > 240 && yellow[1] > 240 && yellow[2] < 20,
                    "Actual original face is highlighted: {yellow:?}"
                );
                assert_eq!(
                    pixel(
                        &selected,
                        matrix,
                        point(&editor, [-350., -150., 0.])
                    ),
                    pixel(
                        &before,
                        matrix,
                        point(&editor, [-350., -150., 0.])
                    ),
                    "Readonly red contact remains above the original face highlight"
                );
                r.set_highlight(&editor.scene, None).unwrap();
                assert_eq!(
                    crate::cap_gpu_tests::pixels_at(&r, matrix),
                    before
                );
                r.use_uniform_surface_records();
                assert_eq!(
                    pick(
                        &mut r,
                        &editor.scene,
                        matrix,
                        point(&editor, [-350., -150., 0.]),
                        Mode::Face
                    ),
                    Some(target)
                );
                assert_eq!(
                    crate::cap_gpu_tests::pixels_at(&r, matrix),
                    before
                );
                r.set_scene(&editor.scene, None);
                assert_eq!(r.stats()[0], uploads);
                assert_eq!(archive(&editor.scene), original);
            }
        }
    }
}

#[test]
fn readonly_contacts_cannot_fabricate_faces_or_remove_original_occlusion() {
    let editor = imported(ElementGeometry::Mesh(panel(true, 300.)), 0.);
    export_source("overhang", &editor);
    let owner = editor.scene.objects()[0].id;
    let matrix = editor.camera.uniform();
    let mut r = crate::cap_gpu_tests::renderer(&editor.scene);
    r.set_opacity(1.);
    crate::cap_gpu_tests::pixels_at(&r, matrix);
    assert_eq!(
        pick(
            &mut r,
            &editor.scene,
            matrix,
            [-500., -150., 0.],
            Mode::Object
        ),
        Some(Target {
            owner,
            part: Part::Object
        })
    );
    assert_eq!(
        pick(
            &mut r,
            &editor.scene,
            matrix,
            [-500., -150., 0.],
            Mode::Face
        ),
        None,
        "Readonly overhang cannot invent an original face"
    );
    let cursor = super::contact_feature_tests::project(matrix, [-500., -150., 0.]);
    let edge = super::contact_feature_tests::project(matrix, [-300., -150., 0.]);
    assert!(
        (cursor[0] - edge[0]).abs() * 160.
            > crate::pick_window::Window::new(cursor, [320, 240], 1.)
                .unwrap()
                .radius as f32
                + 1.,
        "The overhang sample is beyond the actual cursor pick radius"
    );
    assert_eq!(
        pick(
            &mut r,
            &editor.scene,
            matrix,
            [-200., -150., 0.],
            Mode::Face
        ),
        Some(Target {
            owner,
            part: Part::Face(17)
        })
    );
    for parts in [false, true] {
        let mut scene = editor.scene.clone();
        let blocker = if parts {
            crate::prepared::PreparedMesh::new(Mesh::from_vertices_and_faces(
                vec![
                    Point::new(-1200., -1000., 20.),
                    Point::new(500., -1000., 20.),
                    Point::new(500., 1000., 20.),
                    Point::new(-1200., 1000., 20.),
                ],
                vec![vec![0, 1, 2, 3]],
            ))
            .unwrap()
        } else {
            let mut p = crate::prepared::PreparedMesh::triangle(
                [
                    [-1800., -1500., 20.],
                    [1000., -1500., 20.],
                    [-400., 2000., 20.],
                ],
                [0.1, 0.8, 0.2],
            )
            .unwrap();
            p.display.parts = Default::default();
            p
        };
        let blocker = scene.insert(blocker).unwrap();
        std::rc::Rc::make_mut(&mut scene.view).lock(&[blocker], true);
        r.set_scene(&scene, None);
        crate::cap_gpu_tests::pixels_at(&r, matrix);
        assert_eq!(
            pick(
                &mut r,
                &scene,
                matrix,
                [-200., -150., 20.],
                Mode::Face
            ),
            None,
            "Locked original geometry retains occlusion, including rows without source face metadata; parts={parts}"
        );
    }
}

#[test]
fn shared_instance_faces_remain_original_without_geometry_uploads() {
    let mut scene = Scene::empty();
    crate::scene_document::import(
        &mut scene,
        &crate::instance_specimen::display_bytes(),
        "",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    let ids = scene
        .objects()
        .iter()
        .map(|row| row.id)
        .collect::<Vec<_>>();
    assert!(std::rc::Rc::ptr_eq(
        &scene.objects()[0].mesh,
        &scene.objects()[1].mesh
    ));
    assert!(
        ids.iter()
            .all(|id| !scene.forest.derived(*id))
    );
    let mut r = crate::cap_gpu_tests::renderer(&scene);
    let camera = crate::camera_defaults_tests::top_fixture();
    let matrix = camera.uniform();
    let uploads = r.stats()[0];
    crate::cap_gpu_tests::pixels_at(&r, matrix);
    for density in [1., 2.] {
        r.set_density(density);
        for (i, id) in ids.iter().enumerate() {
            assert_eq!(
                pick(
                    &mut r,
                    &scene,
                    matrix,
                    [
                        if i == 0 {
                            -0.6
                        } else {
                            0.6
                        },
                        0.,
                        0.
                    ],
                    Mode::Face
                ),
                Some(Target {
                    owner: *id,
                    part: Part::Face(0)
                })
            );
        }
        r.use_uniform_surface_records();
        assert_eq!(
            pick(
                &mut r,
                &scene,
                matrix,
                [-0.6, 0., 0.],
                Mode::Face
            ),
            Some(Target {
                owner: ids[0],
                part: Part::Face(0)
            })
        );
    }
    assert_eq!(r.stats()[0], uploads);
}

#[test]
fn actual_brep_contact_queries_the_original_top_face() {
    let mut editor = imported(
        ElementGeometry::BRep(session_rust::BRep::create_box(
            1800., 1300., 100.,
        )),
        50.,
    );
    export_source("brep", &editor);
    let row = &editor.scene.objects()[0];
    let owner = row.id;
    assert!(row.cad.is_some());
    let mesh = &row.mesh;
    let slot = mesh
        .indices()
        .chunks_exact(3)
        .zip(&mesh.parts.triangles)
        .find(|(tri, _)| {
            tri.iter()
                .all(|i| (mesh.vertices()[*i as usize][2] - 50.).abs() < 1e-4)
        })
        .map(|(_, slot)| *slot as usize)
        .expect("Actual original BRep top triangles");
    let key = mesh.parts.faces[slot];
    if let Some(root) = std::env::var_os("VIEWER_SOURCE_FACE_PROOF_DIR") {
        std::fs::write(
            std::path::Path::new(&root).join("brep-face-key.json"),
            key.to_string(),
        )
        .unwrap();
    }
    let source = archive(&editor.scene);
    let mut r = crate::cap_gpu_tests::renderer(&editor.scene);
    r.set_opacity(1.);
    for projection in [
        Projection::Orthographic,
        Projection::Perspective,
    ] {
        editor.camera.projection = projection;
        let matrix = editor.camera.uniform();
        let before = crate::cap_gpu_tests::pixels_at(&r, matrix);
        for density in [1., 2.] {
            r.set_density(density);
            assert_eq!(
                pick(
                    &mut r,
                    &editor.scene,
                    matrix,
                    [-350., -150., 50.],
                    Mode::Face
                ),
                Some(Target {
                    owner,
                    part: Part::Face(key)
                })
            );
            assert_eq!(
                pick(
                    &mut r,
                    &editor.scene,
                    matrix,
                    [-350., -150., 50.],
                    Mode::Object
                ),
                Some(Target {
                    owner,
                    part: Part::Object
                })
            );
        }
        r.set_density(1.);
        assert_eq!(
            crate::cap_gpu_tests::pixels_at(&r, matrix),
            before
        );
    }
    assert_eq!(archive(&editor.scene), source);
}
```

### 9. `src/renderer.rs`

Connect optional Native validation outside the browser runtime.

**Locate this existing block:**

```rust
#[path = "contact_boundary_tests.rs"]
mod contact_boundary_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[path = "contact_boundary_tests.rs"]
mod contact_boundary_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "source_face_policy_tests.rs"]
mod source_face_policy_tests;
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 22.2
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 22.2
npm --prefix ../session_tests run course -- save 22.2
```

Continue with [22.3 · Original boundary pens, joins and vertex markers](96zzzzzzn-respect-original-boundary-pens.md).
