# 22.2 · Contact fills and original face queries

**Section 10: Shared GPU Arenas, Floating-Origin Precision, Multisampling, and Production Parity** · **Checkpoints:** `96zzzzzzi-share-one-frame-across-color-and-ink` → `96zzzzzzm-query-original-faces-under-contact-overlays` (5 steps) · **Production `session_viewer/src` target:** [engine/gpu/render.rs](../../../../src/engine/gpu/render.rs) · [engine/gpu/pass.rs](../../../../src/engine/gpu/pass.rs) · [engine/gpu/targets.rs](../../../../src/engine/gpu/targets.rs) · [engine/gpu/frame.rs](../../../../src/engine/gpu/frame.rs) · [engine/gpu/view.rs](../../../../src/engine/gpu/view.rs) · [engine/pipelines/mod.rs](../../../../src/engine/pipelines/mod.rs) · [shaders/triangle.wgsl](../../../../src/shaders/triangle.wgsl) · [shaders/ribbon.wgsl](../../../../src/shaders/ribbon.wgsl) · [shaders/vector.wgsl](../../../../src/shaders/vector.wgsl) · [shaders/ink_visibility.wgsl](../../../../src/shaders/ink_visibility.wgsl) · [app/scene.rs](../../../../src/app/scene.rs) (`session_viewer/src/lib.rs`) · **Goal:** Select the actual original face beneath a readonly contact, without inventing an editable face for its display polygon.

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 22.2 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/surface_view.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/triangle.wgsl</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/renderer.rs</text>
  <text x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">96zzzzzzi-share-one-frame-across-color-and-ink -&gt; 96zzzzzzm-query-original-faces-under-contact-overlays</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/device.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/>
  <text x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Share one frame across co…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Vertex RGB &amp; Uniforms</text>
  <polygon points="610,56 690,56 610,126" fill="#bfdbfe" stroke="#111111" stroke-width="1.75"/>
  <polygon points="690,56 690,126 610,126" fill="#fde68a" stroke="#111111" stroke-width="1.75"/>
  <circle cx="610" cy="56" r="4" fill="#ef4444"/><text x="590" y="58" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v0</text>
  <circle cx="690" cy="56" r="4" fill="#22c55e"/><text x="696" y="58" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v1</text>
  <circle cx="610" cy="126" r="4" fill="#3b82f6"/><text x="590" y="130" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v2</text>
  <circle cx="690" cy="126" r="4" fill="#f59e0b"/><text x="696" y="130" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v3</text>
  <rect x="720" y="64" width="90" height="54" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="765" y="88" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">view_proj</text>
  <text x="765" y="104" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">mat4x4&lt;f32&gt;</text>
  <text x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Contact fills and original…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Depth32Float &amp; Ray Pick</text>
  <rect x="936" y="54" width="96" height="62" rx="4" fill="#93c5fd" stroke="#111111" stroke-width="1.5" stroke-dasharray="3 3"/>
  <rect x="892" y="74" width="96" height="62" rx="4" fill="#fdba74" stroke="#111111" stroke-width="2"/>
  <line x1="864" y1="138" x2="932" y2="102" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <circle cx="936" cy="100" r="5" fill="#facc15" stroke="#111111" stroke-width="1.5"/>
  <text x="896" y="68" font-family="monospace" font-size="9" font-weight="700" fill="#111111">z=0.2 &lt; z=0.7</text>
  <text x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Query original faces unde…</text>
</svg>
</div>

## <a id="checkpoint-96zzzzzzi-share-one-frame-across-color-and-ink"></a>Step 1 of 5 · Share one frame across color and ink (`96zzzzzzi-share-one-frame-across-color-and-ink`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1/5: 96zzzzzzi-share-o…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/surface_view.rs, src/tr…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Existing renderer frame</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">borrowed surface and ink…</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">separate borrowed pick gr…</text>
</svg>
</div>

Every ordinary ink lane currently builds and uploads another copy of the camera and viewport. These copies also belong to selection, preview, measurement and guide wrappers. Put the physical viewport, density and opacity immediately after the matrix in the shared frame. Existing ink shaders can read that eighty-byte prefix; surface display settings and the eye follow it. Use one explicit layout and pass borrowed groups into drawing. Color uses the renderer frame, and integer picking uses its existing request frame. The sheet lane has a different projection-scale field: keep that calibrated frame separate. This also makes the real viewport available for the coming contact-depth work without another camera owner.

> **Check:** *Why keep the request frame independent if all ordinary ink shares a layout?* — Layout compatibility does not make the latest color camera the owner of an asynchronous pick. The pick uses its captured full physical viewport, density and transform until its readback completes.

### 1. `src/surface_view.rs`

Give existing ink shaders their unchanged viewport prefix, followed by surface settings and the existing kernel-derived eye.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzzzzzi-share-one-frame-across-color-and-ink-01.rs"
```

### 2. `src/triangle.wgsl`

Read surface settings after the shared ink prefix.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct ViewSettings { transform: mat4x4<f32>, viewport:vec4<f32>, display: vec4<f32>, eye:vec4<f32>, }
```

### 3. `src/renderer.rs`

Encode the correct full physical viewport and density once for this frame owner.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        self.prepare_clouds(*transform);
        let options=self.surface_options.get();
        let display=[f32::from(normals),self.opacity.get(),if self.ambient_enabled.get(){2.}else{f32::from(options[0])},f32::from(options[1])];
        let bytes=crate::surface_view::bytes(*transform,display);
        self.queue.write_buffer(&self.uniform, 0, &bytes);
        self.strokes.view(&self.queue, transform, self.size, self.density);
        self.sheets.view(&self.queue,transform,self.size,self.density);
```

Replace it with:

```rust
        self.prepare_clouds(*transform);
        let options=self.surface_options.get();
        let display=[f32::from(normals),self.opacity.get(),if self.ambient_enabled.get(){2.}else{f32::from(options[0])},f32::from(options[1])];
        let bytes=crate::surface_view::bytes(*transform,display,self.size,self.density);
        self.queue.write_buffer(&self.uniform, 0, &bytes);
        self.strokes.view(&self.queue, transform, self.size, self.density);
        self.sheets.view(&self.queue,transform,self.size,self.density);
```

**Locate this block:**

```rust
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) {
            let mut clipping=self.clipping.borrow_mut();clipping.frame_anchored(&self.queue,transform,self.anchor(),self.size[1],self.density);
            let bytes=crate::surface_view::bytes(transform,[0.;4]);self.queue.write_buffer(&self.uniform,0,&bytes);
            let tables={let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);let mut encoder=self.device.create_command_encoder(&Default::default());tiles.encode(&self.device,&mut encoder,&self.view_group,transform);self.queue.submit([encoder.finish()]);tiles.map_report();crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.primitive.view.clone()}};
            self.picker.submit_finite(&self.device, &self.queue, request,
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces,self.text.borrow().as_ref(),self.clouds.borrow().as_ref(),Some(&self.sheets),Some(&tables),&clipping.group,&clipping.buffer,Some(&mut self.caps.borrow_mut()),Some(&self.instances)); }
```

Replace it with:

```rust
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) {
            let mut clipping=self.clipping.borrow_mut();clipping.frame_anchored(&self.queue,transform,self.anchor(),self.size[1],self.density);
            let bytes=crate::surface_view::bytes(transform,[0.;4],self.size,self.density);self.queue.write_buffer(&self.uniform,0,&bytes);
            let tables={let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);let mut encoder=self.device.create_command_encoder(&Default::default());tiles.encode(&self.device,&mut encoder,&self.view_group,transform);self.queue.submit([encoder.finish()]);tiles.map_report();crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.primitive.view.clone()}};
            self.picker.submit_finite(&self.device, &self.queue, request,
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces,self.text.borrow().as_ref(),self.clouds.borrow().as_ref(),Some(&self.sheets),Some(&tables),&clipping.group,&clipping.buffer,Some(&mut self.caps.borrow_mut()),Some(&self.instances)); }
```

**Complete updated block reference:**

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
    surface_options:std::cell::Cell<[bool;2]>,
    world_bounds:std::cell::RefCell<Option<crate::bounds::Bounds>>,
    space:crate::render_space::Origin,
    opacity:std::cell::Cell<f32>,
    ambient_enabled:std::cell::Cell<bool>,
    ambient:std::cell::RefCell<Option<crate::ambient_gpu::Ambient>>,
    ambient_rows:crate::ambient_rows::Rows,
    pyramid:std::cell::RefCell<Option<crate::pyramid_gpu::Pyramid>>,
    pyramid_revision:u64,
    caps:std::cell::RefCell<crate::caps_gpu::Caps>,
    clipping:std::cell::RefCell<crate::clipping_gpu::Clipping>,
    tiles:std::cell::RefCell<crate::tile_gpu::Tiles>,
    guides:crate::guide_gpu::Guide,
    measurement:crate::measure_gpu::Measurement,
    sheets:crate::sheet_gpu::Lane,
    clouds:std::cell::RefCell<Option<crate::cloud_gpu::Lane>>,
    cloud_view:std::cell::Cell<crate::cloud_gpu::View>,
    cloud_error:std::cell::RefCell<Option<String>>,
    outline:std::cell::RefCell<Option<crate::outline_gpu::Outline>>,
    global_outline:std::cell::RefCell<Option<crate::outline_gpu::Outline>>,
    outlines_enabled:std::cell::Cell<bool>,
    outline_composite:std::cell::RefCell<Option<crate::outline_composite::Composite>>,
    text:std::cell::RefCell<Option<crate::scene_text_gpu::Lane>>,
    text_error:std::cell::RefCell<Option<String>>,
    full_text_fonts:std::cell::RefCell<Option<Vec<Vec<u8>>>>,
    presentation_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    scene_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    format:wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    instances:crate::instance_gpu::Lane,
    instance_rows:std::collections::HashSet<ObjectId>,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
    highlight: crate::highlight::Highlight,
    preview: crate::tool_preview::Preview,
    widget: crate::widget::Widget,
    meshes: Vec<GpuMesh>,
    geometry: crate::geometry_cache::GeometryCache,
    settings_allocations: usize,
    settings_writes: usize,
    settings_bytes: usize,
    uniform: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    depth: wgpu::TextureView,
    primitive:crate::primitive_image::Image,
    strokes: crate::stroke_gpu::Lane,
    paths: crate::stroke_gpu::Lane,
    markers: crate::stroke_gpu::Lane,
    grid: crate::stroke_gpu::Lane,
    grid_settings: Option<crate::grid::Settings>,
    grid_plane:crate::cplane::CPlane,grid_anchor:[f64;3],
    size: [u32; 2],
    density: f32,
    picker: crate::pick_gpu::Picker,
    sources: crate::pick_sources::Sources,
    faces: crate::face_pick::Faces,
    mesh_ink: crate::stroke_gpu::Lane,
    pick_mode:crate::pick_sources::Mode,
    pick_cursor:Option<([f32;2],[u64;3])>,
}

impl Renderer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        crate::shader_check::check(&device, crate::clipping_gpu::source(include_str!("cloud_points.wgsl"),2,0), "Cloud points");
        crate::shader_check::check(&device, include_str!("cloud_resolve.wgsl").into(), "Cloud resolve");
        crate::shader_check::check(&device, crate::finite_project_source::source(), "Finite projection");
        crate::shader_check::check(&device, crate::finite_project_source::tiles(), "Finite tiles");
        crate::shader_check::check(&device, crate::finite_project_source::scan(), "Finite scan");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0).into()),
        });
        let view_layout = crate::surface_view::layout(&device);
        let model_layout = crate::gpu_layout::uniform(&device, crate::object_record::BYTES as u64);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"), bind_group_layouts: &[Some(&view_layout), Some(&model_layout),Some(&crate::clipping_gpu::layout(&device)),Some(&crate::gpu_geometry::GpuGeometry::storage_layout(&device))], immediate_size: 0 });
        let mesh_pipeline = |depth,primitive_write| device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[crate::draw_slots::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState{format,blend:Some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(primitive_write))],
            }),
            primitive: Default::default(),
            depth_stencil: Some(depth),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let pipeline = mesh_pipeline(crate::depth::surface(),true);
        let tiles=std::cell::RefCell::new(crate::tile_gpu::Tiles::new(&device,&view_layout));
        let instances=crate::instance_gpu::Lane::new(&device,&queue,format,&view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink(),false);
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
        let widget = crate::widget::Widget::new(&device, format);
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: crate::surface_view::BYTES,
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
        let primitive=crate::primitive_image::Image::new(&device,[640,480]);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
        let grid = crate::stroke_gpu::Lane::grid(&device, format);
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {surface_options:Default::default(),world_bounds:Default::default(),space:Default::default(),opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, primitive, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy,grid_anchor:[0.;3], size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode:Default::default(),pick_cursor:None };
        for lane in [&mut renderer.mesh_ink,&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers]{lane.enable_finite(&renderer.device,format);}
        renderer.highlight.enable_finite(&renderer.device,format);
        renderer.set_scene(scene, None);
        renderer
    }

    pub fn anchor(&self)->[f64;3] {self.space.anchor()}

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: crate::depth::FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        if self.size != [size.width, size.height] { self.cancel_pick(); }
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
        self.primitive=crate::primitive_image::Image::new(&self.device,self.size);
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 && density != self.density { self.cancel_pick(); self.density = density; }
    }

    pub fn stroke_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.strokes.usage();
        [count, bytes, self.strokes.uploads, self.strokes.uploaded_bytes]
    }

    pub fn path_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.paths.usage();
        [count, bytes, self.paths.uploads, self.paths.uploaded_bytes]
    }

    pub fn point_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.markers.usage();
        [count, bytes, self.markers.uploads, self.markers.uploaded_bytes]
    }

    #[cfg(test)]
    pub(crate) fn use_uniform_surface_records(&mut self) {self.instances.ordinary.clear();}

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        let anchor=self.anchor();
        self.ambient_rows.set(scene);if let Some(ambient)=self.ambient.borrow_mut().as_mut(){if self.ambient_enabled.get(){ambient.sources.set(&self.device,&self.queue,&self.ambient_rows.specs);}}
        self.pyramid_revision=self.pyramid_revision.wrapping_add(1);
        self.clipping.borrow_mut().set(scene);
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        self.cancel_pick(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, None).expect("Empty selection");
        self.set_source_ink(scene);
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let matching = old.iter().position(|row| row.id == object.id
                && std::rc::Rc::ptr_eq(&row.geometry.source, &object.mesh));
            let row = if let Some(index) = matching {
                let mut row = old.swap_remove(index);
                let [calls, bytes] = row.update(&self.queue, object,style(scene,object.id,selected==Some(object.id)));
                self.settings_writes += calls;
                self.settings_bytes += bytes;
                row
            } else {
                let geometry = self.geometry.get(&crate::text_gpu_support::GpuCtx::new(&self.device,&self.queue), &object.mesh);
                self.settings_allocations += 1;
                GpuMesh::with_geometry_at(&self.device,&layout,object,
                    style(scene,object.id,selected==Some(object.id)),geometry,anchor)
            };
            self.meshes.push(row);
        }
        drop(old);
        self.caps.borrow_mut().set_scene(&self.queue,scene,&mut self.meshes,&self.clipping.borrow().rows);
        for mesh in &mut self.meshes{mesh.set_xray(&self.queue,self.opacity.get()==0.);}
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,true).expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        self.sheets.set_anchored(&self.device,&self.queue,scene,selected.as_slice(),&self.sources.sheet_bases,anchor).expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        if let Err(error)=self.set_clouds(scene,selected.as_slice()) {*self.cloud_error.borrow_mut()=Some(error.to_string());}
        if let Err(error)=self.set_scene_text(scene,selected.as_slice()) {*self.text_error.borrow_mut()=Some(error.to_string());}
        *self.world_bounds.borrow_mut()=scene.bounds();
    }

    pub fn grow_camera(&self,camera:&mut crate::camera::Camera) {
        if let Some(bounds)=self.world_bounds.borrow().as_ref() {camera.grow_extent(bounds);}
    }

    pub fn camera_matrix(&self,camera:&crate::camera::Camera)->[f32;16] {camera.view_projection_anchored(self.anchor()).to_f32()}
    pub fn rebase_delay(&self,now:f64)->Option<u32> {self.space.retry_after_ms(now)}
    pub fn prepare_origin(&mut self,camera:&crate::camera::Camera,scene:&Scene,target:Option<crate::pick_sources::Target>,ids:&[ObjectId],now:f64)->Result<crate::render_space::Rebase,&'static str> {
        let previous=self.anchor();let mut frame=self.space.update(camera.target,camera.distance,now).ok_or("Invalid camera origin")?;
        frame.moved&=previous.map(f64::to_bits)!=frame.anchor.map(f64::to_bits);
        if frame.moved {
            self.pyramid_revision=self.pyramid_revision.wrapping_add(1);
            for mesh in &mut self.meshes {let[calls,bytes]=mesh.rebase(&self.queue,frame.anchor);self.settings_writes+=calls;self.settings_bytes+=bytes;}
            self.set_source_ink(scene);
            if !self.highlight.rebase_face(&self.queue,frame.anchor,target){self.set_highlight(scene,target)?;}
            self.select_objects(scene,ids)?;
            self.set_grid_on(self.grid_settings,self.grid_plane)?;
        }
        self.cloud_camera(camera);Ok(frame)
    }

    fn set_source_ink(&mut self,scene:&Scene) {
        let anchor=self.anchor();
        self.sources = crate::pick_sources::Sources::anchored(scene,anchor).expect("Prepared source topology is bounded");
        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().copied()).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().copied()).expect("Source edge IDs match");
        let planes=crate::clipping_rows::owners(scene);let flags=self.sources.mesh_ids.iter().map(|id|crate::sub_id::decode(id[1]).and_then(|(_,address)|self.sources.edges.get(address)).map_or(0,|target|if planes.contains(&target.owner){4096}else{0})).collect();self.mesh_ink.set_flags(&self.device,flags).expect("Original plane ink flags match source addresses");
        self.markers.set_points(&self.device, scene.points().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut marker=row.marker_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {marker.colour=colour;}marker}).chain(scene.control_markers_at(anchor).into_iter().map(|c| c.display)))
            .expect("Prepared point placement fits the display range");
        self.paths.set_chains(&self.device, scene.paths().iter().filter(|row| scene.visible(row.id)).flat_map(|row| {let mut segments=row.segments_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {for segment in &mut segments {segment.stroke.colour=colour;}}segments}))
            .expect("Prepared path placement fits the display range");
        self.strokes.set(&self.device, scene.lines().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut stroke=row.stroke_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {stroke.colour=colour;}stroke}))
            .expect("Prepared line placement fits the display range");
        self.markers.set_ids(&self.device, self.sources.marker_ids.iter().copied()).expect("Marker owners match");
        self.paths.set_ids(&self.device, self.sources.path_ids.iter().copied()).expect("Path owners match");
        self.strokes.set_ids(&self.device, self.sources.line_ids.iter().copied()).expect("Line owners match");
    }

    pub fn request_pick(&mut self, screen: [f32; 2], transform: [f32; 16]) -> Result<(), &'static str> {
        let window = crate::pick_window::Window::new(screen, self.size, self.density).ok_or("Invalid pick cursor")?;
        let origin=self.anchor();self.picker.queue_anchored(crate::pick_gpu::Request{window,transform,density:self.density,mode:self.pick_mode},origin)?;
        self.pick_cursor=Some((screen,origin.map(f64::to_bits)));self.pump_pick(transform);Ok(())
    }

    fn pump_pick(&mut self, transform: [f32; 16]) {
        let Ok(view) = crate::pick_view::View::anchored(transform,self.size,self.density,self.anchor()) else { self.cancel_pick(); return; };
        if self.pick_mode == crate::pick_sources::Mode::Face && !self.faces.ready(self.meshes.len()) {
            self.faces.set(&self.device, &self.meshes, &self.sources.mesh_bases,&self.clipping.borrow().buffer);
        }
        if let Some(lane)=self.text.borrow_mut().as_mut() {if let Err(error)=lane.prepare(&self.text_context(),&self.text_frame(transform)) {*self.text_error.borrow_mut()=Some(error.to_string());return;}}
        self.clipping.borrow_mut().frame_anchored(&self.queue,transform,self.anchor(),self.size[1],self.density);
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) {
            let mut clipping=self.clipping.borrow_mut();clipping.frame_anchored(&self.queue,transform,self.anchor(),self.size[1],self.density);
            let bytes=crate::surface_view::bytes(transform,[0.;4],self.size,self.density);self.queue.write_buffer(&self.uniform,0,&bytes);
            let tables={let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);let mut encoder=self.device.create_command_encoder(&Default::default());tiles.encode(&self.device,&mut encoder,&self.view_group,transform);self.queue.submit([encoder.finish()]);tiles.map_report();crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.primitive.view.clone()}};
            self.picker.submit_finite(&self.device, &self.queue, request,
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces,self.text.borrow().as_ref(),self.clouds.borrow().as_ref(),Some(&self.sheets),Some(&tables),&clipping.group,&clipping.buffer,Some(&mut self.caps.borrow_mut()),Some(&self.instances)); }
    }

    pub fn take_pick(&mut self, transform: [f32; 16]) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let view = match crate::pick_view::View::anchored(transform,self.size,self.density,self.anchor()) { Ok(view) => view, Err(error) => {
            self.cancel_pick(); return Some(Err(error));
        } };
        match self.picker.take(view) {
            Some(Ok(crate::pick_gpu::Reply::Hit(hit)))=>{self.pick_cursor=None;Some(Ok(hit))},
            Some(Ok(crate::pick_gpu::Reply::Stale))=>{
                if let Some((screen,_))=self.pick_cursor.filter(|(_,origin)|*origin!=self.anchor().map(f64::to_bits)) {
                    if let Err(error)=self.request_pick(screen,transform){self.cancel_pick();return Some(Err(error));}
                } else {
                    self.pump_pick(transform);
                    if self.picker.usage()[0]==0&&!self.picker.queued(){self.pick_cursor=None;}
                }
                None
            },
            Some(Err(error))=>{self.cancel_pick();Some(Err(error))},None=>None,
        }
    }

    pub fn set_pick_mode(&mut self, mode: crate::pick_sources::Mode) {
        if self.pick_mode != mode { self.cancel_pick(); self.pick_mode = mode; }
    }

    pub fn resolve_pick(&self, scene: &Scene, pixel: [u32; 2]) -> Option<crate::pick_sources::Target> {
        self.sources.resolve(scene, pixel, self.pick_mode)
    }

    pub fn part_usage(&self) -> [u64; 4] { let [count, bytes] = self.mesh_ink.usage(); [count, bytes, self.faces.bytes(), self.sources.bytes() as u64] }

    pub fn cancel_pick(&mut self) {self.pick_cursor=None;self.picker.cancel();}

    pub fn pick_usage(&self) -> [u64; 5] {
        let [active, pending, bytes] = self.picker.usage();
        [active, pending, bytes + self.strokes.pick_bytes() + self.paths.pick_bytes() + self.markers.pick_bytes() + self.mesh_ink.pick_bytes() + self.mesh_ink.usage()[1] + self.faces.bytes(),
            self.picker.submitted, self.picker.coalesced]
    }

    pub fn geometry_owners(&self) -> impl Iterator<Item = &std::rc::Rc<crate::gpu_geometry::GpuGeometry>> {
        self.meshes.iter().map(|row| &row.geometry)
    }

    pub fn usage(&self)->[u64;4]{let mut usage=crate::memory::gpu(&self.meshes);usage[1]+=self.geometry.allocated_bytes();usage}
    pub fn arena_cpu_bytes(&self)->usize{self.geometry.cpu_bytes()}

    pub fn uploaded_bytes(&self) -> u64 {
        self.instances.written_bytes + self.geometry.uploaded_bytes + self.settings_allocations as u64 * crate::object_record::BYTES as u64 + self.settings_bytes as u64
            + self.strokes.uploaded_bytes + self.paths.uploaded_bytes + self.markers.uploaded_bytes + self.grid.uploaded_bytes
    }

    pub fn set_grid(&mut self,settings:Option<crate::grid::Settings>)->Result<(),&'static str>{self.set_grid_on(settings,crate::cplane::CPlane::Xy)}
    pub fn set_grid_on(&mut self,settings:Option<crate::grid::Settings>,plane:crate::cplane::CPlane)->Result<(),&'static str>{
        let anchor=self.anchor();
        if settings==self.grid_settings&&plane==self.grid_plane&&anchor==self.grid_anchor{return Ok(())}
        let strokes=settings.map(|s|s.strokes_on_at(plane,anchor)).transpose()?.unwrap_or_default();
        self.grid.set(&self.device,strokes)?;self.grid_settings=settings;self.grid_plane=plane;self.grid_anchor=anchor;Ok(())
    }

    pub fn grid_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.grid.usage(); [count, bytes, self.grid.uploads, self.grid.uploaded_bytes]
    }

    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        let anchor=self.anchor();self.highlight.set_at(&self.device,&self.pipeline.get_bind_group_layout(1),scene,target,anchor)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
        let anchor=self.anchor();
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        for mesh in &mut self.meshes { if let Some(row) = scene.objects().iter().find(|r| r.id == mesh.id) {
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,false)?;
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;self.sheets.set_anchored(&self.device,&self.queue,scene,ids,&self.sources.sheet_bases,anchor)?;
        if !ids.is_empty() { self.highlight.objects_at(&self.device,scene,ids,anchor)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
    }

    pub fn set_guides(&mut self,overlay:&crate::guide_gpu::Overlay)->Result<(),&'static str>{if self.guides.set(&self.device,overlay)?{self.set_scene_measure_text().map_err(|_|"Cannot prepare editing guide label")?}Ok(())}
    pub fn guide_bytes(&self)->u64{self.guides.bytes()}
    pub fn set_measurement(&mut self,mark:Option<&crate::measure_mark::Mark>)->Result<(),&'static str>{let anchor=self.anchor();if self.measurement.set_at(&self.device,mark,anchor)?{self.set_scene_measure_text().map_err(|_|"Cannot prepare measurement annotation")?;}Ok(())}
    pub fn measurement_bytes(&self)->u64{self.measurement.bytes()}
    fn merge_measurement(&self,mut labels:Vec<crate::text::TextLabel>)->Vec<crate::text::TextLabel>{if let Some(mark)=&self.measurement.mark{labels.extend(mark.labels(&labels));}labels.extend(self.guides.overlay.labels(&labels));labels}
    fn set_scene_measure_text(&self)->anyhow::Result<()>{let mut labels=self.scene_text.borrow().clone();labels.extend(self.presentation_text.borrow().clone());self.adopt_text(self.merge_measurement(labels))}
    pub fn set_preview(&mut self,draft:Option<&crate::tool::Draft>)->Result<(),&'static str>{let anchor=self.anchor();self.preview.set_at(&self.device,draft,anchor)}
    pub fn preview_bytes(&self) -> u64 { self.preview.bytes() }

    pub fn set_gizmo(&mut self, camera:&crate::camera::Camera, origin:Option<[f64;3]>, active:f32) {
        let anchor=self.anchor();let matrix=camera.view_projection_anchored(anchor);
        let layout=origin.map(|p|std::array::from_fn(|i|p[i]-anchor[i])).and_then(|p|crate::widget_layout::world_per_pixel(&matrix,p,self.size[1] as f64/self.density as f64)
            .and_then(|scale|crate::widget_layout::layout(&matrix,p,scale,self.size,active)));
        self.widget.prepare(&self.device,&self.queue,layout);
    }
    pub fn gizmo_usage(&self) -> [u64;4] { self.widget.usage() }
    pub fn gizmo_extent(&self) -> Option<[u32;2]> {self.widget.extent()}

    fn text_context(&self)->crate::text_gpu_support::GpuCtx {crate::text_gpu_support::GpuCtx::new(&self.device,&self.queue)}
    fn text_frame(&self,transform:[f32;16])->crate::scene_text_gpu::TextFrame {crate::scene_text_gpu::TextFrame {mvp:transform,origin:self.anchor(),framebuffer:self.size,logical:[self.size[0] as f64/self.density as f64,self.size[1] as f64/self.density as f64],ortho_half_height:0.,clip:{let clip=self.clipping.borrow();let mut planes=[[0.;4];6];if clip.enabled {for (p,row) in planes.iter_mut().zip(&clip.rows){*p=[row.plane.normal[0],row.plane.normal[1],row.plane.normal[2],row.plane.offset];}}planes}}}
    fn adopt_text(&self,labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        if labels.is_empty() {self.text.borrow_mut().take();self.text_error.borrow_mut().take();return Ok(());}
        let mut lane=self.text.borrow_mut();if lane.is_none() {let mut new=crate::scene_text_gpu::Lane::new(&self.text_context(),crate::text_gpu_support::Target {format:self.format,samples:1});if let Some(fonts)=self.full_text_fonts.borrow().as_ref() {new.document.replace_fonts(fonts.clone())?;}*lane=Some(new);}lane.as_mut().unwrap().set_labels(labels)
    }
    pub fn set_text(&self,mut labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        for (i,label) in labels.iter_mut().enumerate() {label.id=u32::MAX-i as u32;label.object=None;}
        let mut merged=self.scene_text.borrow().clone();merged.extend(labels.clone());self.adopt_text(self.merge_measurement(merged))?;*self.presentation_text.borrow_mut()=labels;Ok(())
    }
    fn set_scene_text(&self,scene:&Scene,selected:&[ObjectId])->anyhow::Result<()> {
        let labels=scene.text_labels(selected).map_err(anyhow::Error::msg)?;let mut merged=labels.clone();merged.extend(self.presentation_text.borrow().clone());self.adopt_text(self.merge_measurement(merged))?;*self.scene_text.borrow_mut()=labels;Ok(())
    }
    pub fn refresh_text_bounds(&self,scene:&mut Scene)->bool {
        let mut changed=false;
        if let Some(lane)=self.text.borrow().as_ref() {for row in scene.texts_mut() {if let Some(run)=lane.document.runs.iter().find(|run|run.label.id==row.id.value()) {
            let extent=run.buffer.layout_runs().fold([0f64;2],|[w,h],line|[w.max(line.line_w as f64),h.max((line.line_top+line.line_height) as f64)]);
            if row.prepared.extent!=extent {row.prepared.extent=extent;changed=true;}
        }}}
        if changed {*self.world_bounds.borrow_mut()=scene.bounds();}
        changed
    }
    pub fn text_stats(&self)->Option<crate::scene_text_gpu::TextStats> {self.text.borrow().as_ref().map(|lane|lane.stats.clone())}
    pub fn text_scene_usage(&self)->[u64;4] {self.text.borrow().as_ref().map_or([0;4],|lane|[lane.allocated_bytes(),lane.texture_bytes(),lane.stats.world_plane_rasterizations,lane.document.runs.len() as u64])}
    pub fn replace_text_fonts(&self,fonts:Vec<Vec<u8>>)->anyhow::Result<()> {if let Some(lane)=self.text.borrow_mut().as_mut() {lane.document.replace_fonts(fonts.clone())?;} else {let mut document=crate::text::TextDocument::new();document.replace_fonts(fonts.clone())?;}*self.full_text_fonts.borrow_mut()=Some(fonts);Ok(())}
    pub fn clear_text_fonts(&self) {self.full_text_fonts.borrow_mut().take();}

    pub fn text_error(&self)->Option<String> {self.text_error.borrow().clone()}
    pub fn text_usage(&self)->[u64;6] {self.text.borrow().as_ref().map_or([0;6],|lane|{[lane.document.shape_count,lane.stats.preparations,lane.stats.skipped_preparations,lane.stats.raster_images as u64,lane.stats.raster_image_capacity_bytes as u64,lane.stats.atlas_resets]})}
    pub fn text_demo(&self)->anyhow::Result<()> {self.set_text(vec![crate::text_example::label()])}

    pub fn cloud_camera(&self,camera:&crate::camera::Camera) {self.cloud_view.set(crate::cloud_gpu::View::anchored(camera,self.anchor()));self.sheets.camera(camera);}
    pub fn cloud_usage(&self)->[u64;5] {self.clouds.borrow().as_ref().map_or([0;5],|lane|lane.usage())}
    pub fn cloud_error(&self)->Option<String> {self.cloud_error.borrow().clone()}
    fn set_clouds(&self,scene:&Scene,selected:&[ObjectId])->Result<(),&'static str> {
        if scene.clouds().is_empty()&&scene.streams().is_empty() {self.clouds.borrow_mut().take();self.cloud_error.borrow_mut().take();return Ok(());}
        let mut owner=self.clouds.borrow_mut();let lane=owner.get_or_insert_with(||crate::cloud_gpu::Lane::new(&self.text_context(),self.format,&self.clipping.borrow().group));lane.set_scene(&self.text_context(),scene,selected)
    }
    fn prepare_clouds(&self,transform:[f32;16]) {
        if let Some(lane)=self.clouds.borrow_mut().as_mut() {lane.set_clipping(self.clipping.borrow().writes);if let Err(error)=lane.prepare(&self.text_context(),transform,self.size,self.density,self.cloud_view.get()) {lane.failed();*self.cloud_error.borrow_mut()=Some(error.to_string());} else {self.cloud_error.borrow_mut().take();}}
    }
    pub fn set_opacity(&mut self,value:f32){if !value.is_finite(){return;}let value=value.clamp(0.,1.);if self.opacity.get()==value{return;}self.opacity.set(value);self.cancel_pick();self.pyramid_revision=self.pyramid_revision.wrapping_add(1);for mesh in &mut self.meshes{mesh.set_xray(&self.queue,value==0.);}for lane in [&self.mesh_ink,&self.strokes,&self.paths,&self.markers,&self.grid]{lane.set_opacity(value);}self.highlight.set_opacity(value);if let Some(o)=self.outline.borrow_mut().as_mut(){o.invalidate();}if let Some(o)=self.global_outline.borrow_mut().as_mut(){o.invalidate();}}
    pub fn opacity(&self)->f32{self.opacity.get()}
    pub fn set_outlines(&self,on:bool){self.outlines_enabled.set(on);if !on{self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();}}
    pub fn global_outline_usage(&self)->[u64;4]{self.global_outline.borrow().as_ref().map_or([0;4],|o|o.usage())}
    pub fn outline_usage(&self)->[u64;4] {self.outline.borrow().as_ref().map_or([0;4],|outline|outline.usage())}

    pub fn highlight_bytes(&self) -> u64 { self.highlight.bytes() }

    pub fn sheet_camera(&self,camera:&crate::camera::Camera) {self.sheets.camera(camera);}
    #[cfg(test)]
    pub(crate) fn tile_buffers(&self)->[wgpu::Buffer;2]{let tiles=self.tiles.borrow();[tiles.projected.clone(),tiles.buffer.clone()]}
    pub fn set_clipping(&self,enabled:bool,solid:bool){let mut clip=self.clipping.borrow_mut();clip.enabled=enabled;clip.solid=solid;self.caps.borrow_mut().enabled=enabled;}
    pub fn set_lighting(&self,on:bool,backfaces:bool){self.surface_options.set([on,backfaces]);}
    pub fn set_ambient(&self,enabled:bool){if self.ambient_enabled.replace(enabled)==enabled{return;}let mut ambient=self.ambient.borrow_mut();if enabled{let owner=ambient.get_or_insert_with(||crate::ambient_gpu::Ambient::new(&self.device,self.format));owner.sources.set(&self.device,&self.queue,&self.ambient_rows.specs);}else if let Some(owner)=ambient.as_mut(){owner.clear(&self.device,&self.queue);}}
    pub fn ambient_usage(&self)->[u64;5]{self.ambient.borrow().as_ref().map_or([0;5],crate::ambient_gpu::Ambient::usage)}
    pub fn ambient_error(&self)->Option<&'static str>{self.ambient.borrow().as_ref().and_then(|owner|owner.error)}
    pub fn set_depth_pyramid(&self,enabled:bool){let mut owner=self.pyramid.borrow_mut();if enabled {if owner.is_none(){*owner=Some(crate::pyramid_gpu::Pyramid::new(&self.device));}}else{*owner=None;}}
    pub fn depth_pyramid_usage(&self)->[u64;4]{self.pyramid.borrow().as_ref().map_or([0;4],crate::pyramid_gpu::Pyramid::usage)}
    pub fn depth_pyramid_error(&self)->Option<&'static str>{self.pyramid.borrow().as_ref().and_then(|p|p.error)}
    pub fn cap_error(&self)->Option<&'static str>{self.caps.borrow().pick_error}
    pub fn cap_usage(&self)->[u64;5]{self.caps.borrow().usage()}
    pub fn clipping_usage(&self)->[u64;5]{let clip=self.clipping.borrow();[clip.rows.len()as u64,u64::from(clip.enabled),u64::from(clip.solid),clip.writes,clip.buffer.size()]}
    pub fn clipping_ink_bytes(&self)->[u64;2]{[&self.mesh_ink,&self.strokes,&self.paths,&self.markers,&self.grid].into_iter().map(crate::stroke_gpu::Lane::clipping_bytes).fold(self.highlight.clipping_bytes(),|a,b|[a[0]+b[0],a[1]+b[1]])}
    pub fn clipping_error(&self)->Option<&'static str>{self.clipping.borrow().error}
    pub fn tile_usage(&self)->[u64;7]{self.tiles.borrow().usage()}
    pub fn sheet_usage(&self)->[u64;5] {self.sheets.usage()}
    pub fn instance_usage(&self)->[u64;5] {self.instances.usage()}
    pub fn instance_allocations(&self)->[u64;3] {self.instances.allocation_parts()}
    pub fn stats(&self) -> [usize; 4] {
        [self.geometry.uploads, self.settings_allocations, self.settings_writes, self.settings_bytes]
    }

    pub fn draw(
        &self,
        view: &wgpu::TextureView,
        background: &crate::background::Background,
        transform: &[f32; 16],
    ) { self.draw_mode(view, background, transform, false); }

    pub fn draw_mode(&self, view: &wgpu::TextureView, background: &crate::background::Background,
        transform: &[f32; 16], normals: bool,
    ) {
        let (clip_group,clip_revision)={let mut clip=self.clipping.borrow_mut();clip.frame_anchored(&self.queue,*transform,self.anchor(),self.size[1],self.density);(clip.group.clone(),clip.writes)};
        self.caps.borrow_mut().prepare(&self.device,&self.queue,self.format,self.size,{let clip=self.clipping.borrow();clip.enabled&&!clip.rows.is_empty()});
        self.prepare_clouds(*transform);
        let options=self.surface_options.get();
        let display=[f32::from(normals),self.opacity.get(),if self.ambient_enabled.get(){2.}else{f32::from(options[0])},f32::from(options[1])];
        let bytes=crate::surface_view::bytes(*transform,display,self.size,self.density);
        self.queue.write_buffer(&self.uniform, 0, &bytes);
        self.strokes.view(&self.queue, transform, self.size, self.density);
        self.sheets.view(&self.queue,transform,self.size,self.density);
        self.paths.view(&self.queue, transform, self.size, self.density);
        self.markers.view(&self.queue, transform, self.size, self.density);
        self.grid.view(&self.queue, transform, self.size, self.density);
        self.mesh_ink.view(&self.queue, transform, self.size, self.density);
        self.highlight.view(&self.queue, transform, self.size, self.density);
        self.preview.view(&self.queue, transform, self.size, self.density);
        self.measurement.view(&self.queue,transform,self.size,self.density);self.guides.view(&self.queue,transform,self.size,self.density);
        let [r, g, b] = if self.ambient_enabled.get() { [0.94; 3] } else { background.rgb() };
        let mut encoder = self.device.create_command_encoder(&Default::default());
        let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);tiles.encode(&self.device,&mut encoder,&self.view_group,*transform);
        let tables=crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.primitive.view.clone()};for lane in [&self.mesh_ink,&self.strokes,&self.paths,&self.markers]{lane.finite_frame(&self.device,&tables,&self.depth);}self.highlight.finite_frame(&self.device,&tables,&self.depth);
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(crate::depth::FAR as f32),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
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
                }),Some(wgpu::RenderPassColorAttachment{
                    view:&self.primitive.view,depth_slice:None,resolve_target:None,
                    ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store},
                })],
                ..Default::default()
            });
            self.grid.draw(&mut pass);
            pass.set_bind_group(2,&clip_group,&[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {if !self.instances.draw_ordinary(&mut pass,mesh){pass.set_pipeline(&self.pipeline);self.instances.bind_own(&mut pass);mesh.draw_pulled(&mut pass);}}
            if self.opacity.get()>=1.{self.instances.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            self.highlight.mesh(&mut pass);
        }
        self.caps.borrow_mut().draw(&self.device,&mut encoder,view,&self.primitive.view,&self.depth,&self.view_group,&clip_group,&self.meshes,if self.clipping.borrow().enabled{self.clipping.borrow().rows.len()as u32}else{0},Some(&self.instances));
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("cloud depth composite"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});if let Some(clouds)=self.clouds.borrow().as_ref(){clouds.draw(&mut pass);}}
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite scene ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:None,stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});
            pass.set_bind_group(2,&clip_group,&[]);
            self.highlight.ink(&mut pass);
            self.mesh_ink.draw(&mut pass);
            self.strokes.draw(&mut pass);self.sheets.draw(&mut pass);
            self.paths.draw(&mut pass);
            self.markers.draw(&mut pass);
            self.highlight.points(&mut pass);
            self.preview.draw(&mut pass);
        }
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("measurement overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});self.measurement.draw(&mut pass);self.guides.draw(&mut pass);}
        if self.outlines_enabled.get()&&!self.meshes.is_empty(){
            let ctx=self.text_context();
            let mut global=self.global_outline.borrow_mut();let global=global.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));
            global.encode_masks(&ctx,&mut encoder,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,true,Some(&self.primitive.view));
            let mut selected=self.outline.borrow_mut();
            if self.meshes.iter().any(GpuMesh::selected){let selected=selected.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));selected.encode_masks(&ctx,&mut encoder,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,false,Some(&self.primitive.view));}else{selected.take();}
            if let Some(alpha)=global.alpha_view(){self.outline_composite.borrow_mut().get_or_insert_with(||crate::outline_composite::Composite::new(&self.device,self.format)).draw(&self.device,&mut encoder,view,alpha,selected.as_ref().and_then(|s|s.alpha_view()));}
        }else{
            self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();
            if self.meshes.iter().any(GpuMesh::selected){let mut outline=self.outline.borrow_mut();let outline=outline.get_or_insert_with(||crate::outline_gpu::Outline::new(&self.text_context(),self.format));outline.encode(&self.text_context(),&mut encoder,view,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision);}else{self.outline.borrow_mut().take();}
        }
        self.widget.draw(&mut encoder,view);
        self.text_error.borrow_mut().take();
        if let Some(lane)=self.text.borrow_mut().as_mut() {
            match lane.prepare(&self.text_context(),&self.text_frame(*transform)) {
            Ok(())=>{
                let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("scene glyphs"),depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {view:&self.depth,depth_ops:Some(wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view,depth_slice:None,resolve_target:None,ops:wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});if let Err(error)=lane.draw(&mut pass) {*self.text_error.borrow_mut()=Some(error.to_string());}
            }
            Err(error)=>{*self.text_error.borrow_mut()=Some(error.to_string());}
            }
        }
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.depth,&self.view_group,&clip_group,&clip.buffer,&self.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground_at(self.anchor()),[self.pyramid_revision,clip_revision],Some(&self.instances));}}
        if let Some(pyramid)=self.pyramid.borrow_mut().as_mut(){pyramid.encode(&self.device,&self.queue,&mut encoder,&self.depth,*transform,self.size,self.density,[self.pyramid_revision,clip_revision]);}
        self.queue.submit([encoder.finish()]);tiles.map_report();
    }
}

fn style(scene:&Scene,id:ObjectId,selected:bool)->crate::gpu_mesh::Style {crate::gpu_mesh::Style {selected,pickable:scene.pick_id(id.value()).is_some(),colour:scene.face_rgba(id)}}

fn rgba(rgb:[u8;3])->[f32;4] {[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]}

#[cfg(all(test,not(target_arch="wasm32")))]
impl Renderer {
 pub fn outline_alpha(&self)->Vec<u8>{self.read_outline_alpha(false)}
 pub fn global_outline_alpha(&self)->Vec<u8>{self.read_outline_alpha(true)}
 fn read_outline_alpha(&self,global:bool)->Vec<u8> {
  let owner=if global{self.global_outline.borrow()}else{self.outline.borrow()};let Some(outline)=owner.as_ref() else {return Vec::new();};let Some(texture)=outline.alpha_texture() else {return Vec::new();};let [width,height]=self.size;let pitch=width.div_ceil(256)*256;
  let buffer=self.device.create_buffer(&wgpu::BufferDescriptor {label:Some("actual outline alpha readback"),size:pitch as u64*height as u64,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});let mut encoder=self.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo {buffer:&buffer,layout:wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},wgpu::Extent3d {width,height,depth_or_array_layers:1});self.queue.submit([encoder.finish()]);let (tx,rx)=std::sync::mpsc::channel();buffer.slice(..).map_async(wgpu::MapMode::Read,move|r|tx.send(r).unwrap());self.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();let bytes=buffer.slice(..).get_mapped_range();let pixels=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize].iter().copied()).collect();drop(bytes);buffer.unmap();pixels
 }
}

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="pyramid_gpu_tests.rs"]
mod pyramid_gpu_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="ambient_gpu_tests.rs"]
mod ambient_gpu_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="global_outline_tests.rs"]
mod global_outline_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="opacity_gpu_tests.rs"]
mod opacity_gpu_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="camera_extent_tests.rs"]
mod camera_extent_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="surface_lighting_tests.rs"]
mod surface_lighting_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="surface_controls_gpu_tests.rs"]
mod surface_controls_tests;
```

### 4. `src/pick_gpu.rs`

Encode the correct full physical viewport and density once for this frame owner.

Edit the existing block to match this code; keep unchanged lines:

```rust
        let bytes=crate::surface_view::bytes(request.transform,[0.;4],window.full,request.density);
```

### 5. `src/stroke_gpu.rs`

Remove duplicated per-lane presentation state. Borrow a frame group instead of owning another uniform. The renderer and request owners perform the only frame writes. Borrow the color frame at draw time. Use the same borrowed group in the finite path. Use the borrowed frame in the base ink path. Borrow the actual request frame for original IDs. Keep the base integer path on the request frame. Pass the request frame through finite picking. Share the captured frame with finite original IDs. Retain the same request in the base fallback. Use the same explicit layout even when a shader reads only its prefix. Remove the redundant buffer and bind-group allocations. Keep settings in their existing renderer owner. Keep only lane geometry, pipelines and finite bindings.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(),
            depth_stencil: Some(crate::depth::ink()), multisample: Default::default(), multiview_mask: None, cache: None,
        }));
        let uniform = device.create_buffer(&wgpu::BufferDescriptor { label: Some("stroke view"), size: 80,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("stroke view"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: uniform.as_entire_binding() }] });
        Self {opacity:std::cell::Cell::new(1.),flags:None,flag_data:Vec::new(),finite:None,strict_depth, pipeline, pick_pipeline, ids: None, id_data: Vec::new(), uniform, group, vertices: None, data: Vec::new(), stride, vertices_per_instance, uploads: 0, uploaded_bytes: 0 }
    }

    fn layout_depth(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
```

Replace it with:

```rust
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(),
            depth_stencil: Some(crate::depth::ink()), multisample: Default::default(), multiview_mask: None, cache: None,
        }));
        Self {flags:None,flag_data:Vec::new(),finite:None,strict_depth, pipeline, pick_pipeline, ids: None, id_data: Vec::new(),  vertices: None, data: Vec::new(), stride, vertices_per_instance, uploads: 0, uploaded_bytes: 0 }
    }

    fn layout_depth(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
```

**Locate this block:**

```rust
    fn layout_target(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
        stride: u64, vertices_per_instance: u32, attributes: &[wgpu::VertexAttribute], depth: wgpu::DepthStencilState, pickable:bool, physical:bool,
    ) -> Self {
        let view_layout = crate::gpu_layout::uniform(device, 80);
        let crop_layout = crate::gpu_layout::uniform(device, 80);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ink settings"), bind_group_layouts: &[Some(&view_layout)], immediate_size: 0 });
```

Replace it with:

```rust
    fn layout_target(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
        stride: u64, vertices_per_instance: u32, attributes: &[wgpu::VertexAttribute], depth: wgpu::DepthStencilState, pickable:bool, physical:bool,
    ) -> Self {
        let view_layout=crate::surface_view::layout(device);
        let crop_layout = crate::gpu_layout::uniform(device, 80);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ink settings"), bind_group_layouts: &[Some(&view_layout)], immediate_size: 0 });
```

**Locate this block:**

```rust
        self.id_data = data; Ok(())
    }

    pub fn pick(&self, pass: &mut wgpu::RenderPass<'_>, crop: &wgpu::BindGroup) {
        let (Some(vertices), Some(ids), Some(pipeline)) = (&self.vertices, &self.ids, &self.pick_pipeline) else { return; };
        pass.set_pipeline(pipeline); pass.set_bind_group(0, &self.group, &[]); pass.set_bind_group(1, crop, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..)); pass.set_vertex_buffer(1, ids.slice(..));
        pass.draw(0..self.vertices_per_instance, 0..(self.data.len() as u64 / self.stride) as u32);
    }

    pub fn pick_finite(&self,device:&wgpu::Device,pass:&mut wgpu::RenderPass<'_>,crop:&wgpu::BindGroup,tables:Option<&crate::finite_ink::Tables>,depth:&wgpu::TextureView){
        if let(Some(ink),Some(tables),Some(vertices),Some(ids))=(&self.finite,tables,&self.vertices,&self.ids){ink.pick(device,pass,&self.group,crop,vertices,ids,self.flags.as_ref().expect("Finite original clipping flags"),(self.data.len()as u64/self.stride)as u32,self.vertices_per_instance,tables,depth)}else{self.pick(pass,crop)}
    }
    pub fn pick_bytes(&self) -> u64 { self.ids.as_ref().map_or(0, wgpu::Buffer::size) }
```

Replace it with:

```rust
        self.id_data = data; Ok(())
    }

    pub fn pick(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup, crop: &wgpu::BindGroup) {
        let (Some(vertices), Some(ids), Some(pipeline)) = (&self.vertices, &self.ids, &self.pick_pipeline) else { return; };
        pass.set_pipeline(pipeline); pass.set_bind_group(0, view, &[]); pass.set_bind_group(1, crop, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..)); pass.set_vertex_buffer(1, ids.slice(..));
        pass.draw(0..self.vertices_per_instance, 0..(self.data.len() as u64 / self.stride) as u32);
    }

    pub fn pick_finite(&self,device:&wgpu::Device,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,crop:&wgpu::BindGroup,tables:Option<&crate::finite_ink::Tables>,depth:&wgpu::TextureView){
        if let(Some(ink),Some(tables),Some(vertices),Some(ids))=(&self.finite,tables,&self.vertices,&self.ids){ink.pick(device,pass,view,crop,vertices,ids,self.flags.as_ref().expect("Finite original clipping flags"),(self.data.len()as u64/self.stride)as u32,self.vertices_per_instance,tables,depth)}else{self.pick(pass,view,crop)}
    }
    pub fn pick_bytes(&self) -> u64 { self.ids.as_ref().map_or(0, wgpu::Buffer::size) }
```

**Locate this block:**

```rust
    pub fn clipping_bytes(&self)->[u64;2]{[self.flags.as_ref().map_or(0,wgpu::Buffer::size),self.flag_data.capacity()as u64*4]}
    pub fn enable_finite(&mut self,device:&wgpu::Device,format:wgpu::TextureFormat){self.finite=Some(crate::finite_ink::Ink::new(device,format,self.stride,self.strict_depth));self.set_flags(device,vec![0;self.data.len()/self.stride as usize]).expect("Original ink clipping flags");}
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){if let Some(ink)=&self.finite{ink.set(device,tables,depth)}}
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        let Some(vertices) = &self.vertices else { return; };
        if self.finite.as_ref().is_some_and(|ink|ink.draw(pass,&self.group,vertices,self.flags.as_ref().expect("Finite original clipping flags"),(self.data.len()as u64/self.stride)as u32,self.vertices_per_instance)){return}
        pass.set_pipeline(&self.pipeline); pass.set_bind_group(0, &self.group, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.draw(0..self.vertices_per_instance, 0..(self.data.len() as u64 / self.stride) as u32);
    }
```

Replace it with:

```rust
    pub fn clipping_bytes(&self)->[u64;2]{[self.flags.as_ref().map_or(0,wgpu::Buffer::size),self.flag_data.capacity()as u64*4]}
    pub fn enable_finite(&mut self,device:&wgpu::Device,format:wgpu::TextureFormat){self.finite=Some(crate::finite_ink::Ink::new(device,format,self.stride,self.strict_depth));self.set_flags(device,vec![0;self.data.len()/self.stride as usize]).expect("Original ink clipping flags");}
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){if let Some(ink)=&self.finite{ink.set(device,tables,depth)}}
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup) {
        let Some(vertices) = &self.vertices else { return; };
        if self.finite.as_ref().is_some_and(|ink|ink.draw(pass,view,vertices,self.flags.as_ref().expect("Finite original clipping flags"),(self.data.len()as u64/self.stride)as u32,self.vertices_per_instance)){return}
        pass.set_pipeline(&self.pipeline); pass.set_bind_group(0, view, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.draw(0..self.vertices_per_instance, 0..(self.data.len() as u64 / self.stride) as u32);
    }
```

**Locate this block:**

```rust
        self.data = data; if self.finite.is_some(){self.set_flags(device,vec![0;self.data.len()/self.stride as usize])?;} Ok(())
    }

    pub fn set_opacity(&self,value:f32){self.opacity.set(value);}
    pub fn view(&self, queue: &wgpu::Queue, transform: &[f32; 16], size: [u32; 2], density: f32) {
        if self.vertices.is_none() { return; }
        let bytes: Vec<u8> = transform.iter().copied().chain([size[0] as f32, size[1] as f32, density, self.opacity.get()])
            .flat_map(f32::to_ne_bytes).collect();
        queue.write_buffer(&self.uniform, 0, &bytes);
    }

    pub fn set_flags(&mut self,device:&wgpu::Device,flags:Vec<u32>)->Result<(),&'static str>{
        if flags.len()!=self.data.len()/self.stride as usize{return Err("Clipping flags must match original ink instances")}
        if flags==self.flag_data{return Ok(())}use wgpu::util::DeviceExt;self.flags=if flags.is_empty(){None}else{let bytes=flags.iter().copied().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("original ink clipping flags"),contents:&bytes,usage:wgpu::BufferUsages::VERTEX}))};self.flag_data=flags;Ok(())
```

Replace it with:

```rust
        self.data = data; if self.finite.is_some(){self.set_flags(device,vec![0;self.data.len()/self.stride as usize])?;} Ok(())
    }

    pub fn set_flags(&mut self,device:&wgpu::Device,flags:Vec<u32>)->Result<(),&'static str>{
        if flags.len()!=self.data.len()/self.stride as usize{return Err("Clipping flags must match original ink instances")}
        if flags==self.flag_data{return Ok(())}use wgpu::util::DeviceExt;self.flags=if flags.is_empty(){None}else{let bytes=flags.iter().copied().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("original ink clipping flags"),contents:&bytes,usage:wgpu::BufferUsages::VERTEX}))};self.flag_data=flags;Ok(())
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
    opacity:std::cell::Cell<f32>,
    flags:Option<wgpu::Buffer>,flag_data:Vec<u32>,
    finite:Option<crate::finite_ink::Ink>,
    strict_depth:bool,
```

Replace it with:

```rust
pub struct Lane {
    flags:Option<wgpu::Buffer>,flag_data:Vec<u32>,
    finite:Option<crate::finite_ink::Ink>,
    strict_depth:bool,
```

**Complete updated block reference:**

```rust
pub struct Lane {
    flags:Option<wgpu::Buffer>,flag_data:Vec<u32>,
    finite:Option<crate::finite_ink::Ink>,
    strict_depth:bool,
    pipeline: wgpu::RenderPipeline,
    pick_pipeline: Option<wgpu::RenderPipeline>,
    ids: Option<wgpu::Buffer>,
    id_data: Vec<u8>,
    vertices: Option<wgpu::Buffer>,
    data: Vec<u8>,
    stride: u64,
    vertices_per_instance: u32,
    pub uploads: u64,
    pub uploaded_bytes: u64,
}

impl Lane {
    pub fn set(&mut self, device: &wgpu::Device, strokes: impl IntoIterator<Item = crate::stroke::Stroke>) -> Result<(), &'static str> {
        let mut data = Vec::new();
        for stroke in strokes {
            if stroke.width <= 0.0 || stroke.values().iter().any(|value| !value.is_finite()) {
                return Err("Invalid stroke display data");
            }
            data.extend(stroke.bytes());
        }
        if self.stride != 44 { return Err("Stroke data needs the stroke layout"); }
        self.set_bytes(device, data)
    }

    pub fn set_guides(&mut self,device:&wgpu::Device,segments:&[crate::guide_gpu::Segment])->Result<(),&'static str>{if self.stride!=48{return Err("Guide data needs the guide layout")}let mut bytes=Vec::with_capacity(segments.len()*48);for segment in segments{if segment.stroke.width<=0.||segment.stroke.values().iter().any(|v|!v.is_finite()){return Err("Invalid guide stroke")}bytes.extend(segment.stroke.bytes());bytes.extend(u32::from(segment.dashed).to_ne_bytes());}self.set_bytes(device,bytes)}
    pub fn set_chains(&mut self, device: &wgpu::Device, segments: impl IntoIterator<Item = crate::chain::Segment>) -> Result<(), &'static str> {
        if self.stride != 72 { return Err("Connected data needs the connected layout"); }
        let mut data = Vec::new();
        for segment in segments {
            if segment.stroke.width <= 0.0 || segment.heads > 3
                || segment.stroke.values().iter().chain(segment.previous.iter()).chain(segment.next.iter()).any(|v| !v.is_finite()) {
                return Err("Invalid connected display data");
            }
            data.extend(segment.bytes());
        }
        self.set_bytes(device, data)
    }

    pub fn set_points(&mut self, device: &wgpu::Device, markers: impl IntoIterator<Item = crate::marker::Marker>) -> Result<(), &'static str> {
        if self.stride != 32 { return Err("Marker data needs the marker layout"); }
        let mut data = Vec::new();
        for marker in markers {
            if marker.diameter <= 0.0 || marker.values().iter().any(|v| !v.is_finite()) { return Err("Invalid marker display data"); }
            data.extend(marker.bytes());
        }
        self.set_bytes(device, data)
    }

    fn set_bytes(&mut self, device: &wgpu::Device, data: Vec<u8>) -> Result<(), &'static str> {
        use wgpu::util::DeviceExt;
        u32::try_from(data.len() as u64 / self.stride).map_err(|_| "Too many stroke instances")?;
        if data == self.data { return Ok(()); }
        self.vertices = if data.is_empty() { None } else {
            self.uploads += 1; self.uploaded_bytes += data.len() as u64;
            Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("stroke instances"), contents: &data, usage: wgpu::BufferUsages::VERTEX,
            }))
        };
        self.data = data; if self.finite.is_some(){self.set_flags(device,vec![0;self.data.len()/self.stride as usize])?;} Ok(())
    }

    pub fn set_flags(&mut self,device:&wgpu::Device,flags:Vec<u32>)->Result<(),&'static str>{
        if flags.len()!=self.data.len()/self.stride as usize{return Err("Clipping flags must match original ink instances")}
        if flags==self.flag_data{return Ok(())}use wgpu::util::DeviceExt;self.flags=if flags.is_empty(){None}else{let bytes=flags.iter().copied().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("original ink clipping flags"),contents:&bytes,usage:wgpu::BufferUsages::VERTEX}))};self.flag_data=flags;Ok(())
    }
    pub fn clipping_bytes(&self)->[u64;2]{[self.flags.as_ref().map_or(0,wgpu::Buffer::size),self.flag_data.capacity()as u64*4]}
    pub fn enable_finite(&mut self,device:&wgpu::Device,format:wgpu::TextureFormat){self.finite=Some(crate::finite_ink::Ink::new(device,format,self.stride,self.strict_depth));self.set_flags(device,vec![0;self.data.len()/self.stride as usize]).expect("Original ink clipping flags");}
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){if let Some(ink)=&self.finite{ink.set(device,tables,depth)}}
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup) {
        let Some(vertices) = &self.vertices else { return; };
        if self.finite.as_ref().is_some_and(|ink|ink.draw(pass,view,vertices,self.flags.as_ref().expect("Finite original clipping flags"),(self.data.len()as u64/self.stride)as u32,self.vertices_per_instance)){return}
        pass.set_pipeline(&self.pipeline); pass.set_bind_group(0, view, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.draw(0..self.vertices_per_instance, 0..(self.data.len() as u64 / self.stride) as u32);
    }

    pub fn set_ids(&mut self, device: &wgpu::Device, ids: impl IntoIterator<Item = [u32; 2]>) -> Result<(), &'static str> {
        use wgpu::util::DeviceExt;
        let data: Vec<u8> = ids.into_iter().flatten().flat_map(u32::to_ne_bytes).collect();
        if data.len() as u64 / 8 != self.data.len() as u64 / self.stride { return Err("Pick IDs must match display instances"); }
        if data == self.id_data { return Ok(()); }
        self.ids = if data.is_empty() { None } else { Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("ink IDs"), contents: &data, usage: wgpu::BufferUsages::VERTEX })) };
        self.id_data = data; Ok(())
    }

    pub fn pick(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup, crop: &wgpu::BindGroup) {
        let (Some(vertices), Some(ids), Some(pipeline)) = (&self.vertices, &self.ids, &self.pick_pipeline) else { return; };
        pass.set_pipeline(pipeline); pass.set_bind_group(0, view, &[]); pass.set_bind_group(1, crop, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..)); pass.set_vertex_buffer(1, ids.slice(..));
        pass.draw(0..self.vertices_per_instance, 0..(self.data.len() as u64 / self.stride) as u32);
    }

    pub fn pick_finite(&self,device:&wgpu::Device,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,crop:&wgpu::BindGroup,tables:Option<&crate::finite_ink::Tables>,depth:&wgpu::TextureView){
        if let(Some(ink),Some(tables),Some(vertices),Some(ids))=(&self.finite,tables,&self.vertices,&self.ids){ink.pick(device,pass,view,crop,vertices,ids,self.flags.as_ref().expect("Finite original clipping flags"),(self.data.len()as u64/self.stride)as u32,self.vertices_per_instance,tables,depth)}else{self.pick(pass,view,crop)}
    }
    pub fn pick_bytes(&self) -> u64 { self.ids.as_ref().map_or(0, wgpu::Buffer::size) }

    pub fn usage(&self) -> [u64; 2] {
        [self.data.len() as u64 / self.stride, self.vertices.as_ref().map_or(0, wgpu::Buffer::size)]
    }

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout(device, format, &crate::stroke::shader(), 44, 6,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32])
    }

    pub fn joined(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout(device, format, &crate::chain::shader(), 72, 18,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32,
                4 => Float32x3, 5 => Float32x3, 6 => Uint32])
    }

    pub fn markers(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout(device, format, &crate::marker::shader(), 32, 6,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32, 2 => Float32x4])
    }

    fn layout(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
        stride: u64, vertices_per_instance: u32, attributes: &[wgpu::VertexAttribute]) -> Self {
        Self::layout_depth(device, format, source, stride, vertices_per_instance, attributes, crate::depth::ink(), true)
    }

    pub fn guides(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{let mut depth=crate::depth::ink();depth.depth_compare=Some(wgpu::CompareFunction::Always);Self::layout_depth(device,format,&crate::finite_project_source::expand(include_str!("guide.wgsl")),48,6,&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Uint32],depth,true)}
    pub fn overlay(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{
        let mut depth=crate::depth::ink();depth.depth_compare=Some(wgpu::CompareFunction::Always);
        Self::layout_depth(device,format,&crate::stroke::shader(),44,6,&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32],depth,true)
    }
    pub fn grid(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout_target(device, format, include_str!("grid.wgsl"), 44, 2,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32], crate::depth::grid(), false, true)
    }

    fn layout_target(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
        stride: u64, vertices_per_instance: u32, attributes: &[wgpu::VertexAttribute], depth: wgpu::DepthStencilState, pickable:bool, physical:bool,
    ) -> Self {
        let view_layout=crate::surface_view::layout(device);
        let crop_layout = crate::gpu_layout::uniform(device, 80);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ink settings"), bind_group_layouts: &[Some(&view_layout)], immediate_size: 0 });
        let pick_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ink pick settings"), bind_group_layouts: &[Some(&view_layout), Some(&crop_layout)], immediate_size: 0 });
        let pick_source = match stride { 48=>include_str!("guide_pick.wgsl"),44 => include_str!("stroke_pick.wgsl"), 72 => include_str!("chain_pick.wgsl"), 32 => include_str!("marker_pick.wgsl"), _ => unreachable!() };
        let pick_shader = pickable.then(||device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("ink IDs"),
            source: wgpu::ShaderSource::Wgsl(format!("{source}\n{}\n{pick_source}", include_str!("pick_crop.wgsl")).into()) }));
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("stroke"), source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let strict_depth=depth.depth_compare==Some(crate::depth::CLOSER);
        let mut targets=vec![Some(wgpu::ColorTargetState{format,blend:if physical{None}else{Some(wgpu::BlendState::ALPHA_BLENDING)},write_mask:wgpu::ColorWrites::ALL})];
        if physical{targets.push(Some(crate::primitive_image::target(true)));}
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("stroke"), layout: Some(&layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout { array_stride: stride, step_mode: wgpu::VertexStepMode::Instance, attributes }],
            },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &targets,
            }),
            primitive: wgpu::PrimitiveState{topology:if physical{wgpu::PrimitiveTopology::LineList}else{wgpu::PrimitiveTopology::TriangleList},..Default::default()},
            depth_stencil: Some(depth),
            multisample: Default::default(), multiview_mask: None, cache: None,
        });
        let pick_pipeline = pick_shader.as_ref().map(|pick_shader|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("ink IDs"), layout: Some(&pick_layout),
            vertex: wgpu::VertexState { module: &pick_shader, entry_point: Some("pick_vertex"), compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout { array_stride: stride, step_mode: wgpu::VertexStepMode::Instance, attributes },
                    wgpu::VertexBufferLayout { array_stride: 8, step_mode: wgpu::VertexStepMode::Instance, attributes: &wgpu::vertex_attr_array![7 => Uint32x2] }] },
            fragment: Some(wgpu::FragmentState { module: &pick_shader, entry_point: Some("pick_fragment"), compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(),
            depth_stencil: Some(crate::depth::ink()), multisample: Default::default(), multiview_mask: None, cache: None,
        }));
        Self {flags:None,flag_data:Vec::new(),finite:None,strict_depth, pipeline, pick_pipeline, ids: None, id_data: Vec::new(),  vertices: None, data: Vec::new(), stride, vertices_per_instance, uploads: 0, uploaded_bytes: 0 }
    }

    fn layout_depth(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
        stride: u64, vertices_per_instance: u32, attributes: &[wgpu::VertexAttribute], depth: wgpu::DepthStencilState, pickable:bool,
    ) -> Self {
        Self::layout_target(device,format,source,stride,vertices_per_instance,attributes,depth,pickable,false)
    }
}

#[cfg(test)]
#[path="grid_pick_pipeline_tests.rs"]
mod pick_pipeline_tests;
```

### 6. `src/finite_ink.rs`

Keep finite color and integer pipelines compatible with the same frame.

Edit the existing block to match this code; keep unchanged lines:

```rust
  let view=crate::surface_view::layout(device);let crop=crate::gpu_layout::uniform(device,80);
```

### 7. `src/pick_gpu.rs`

Stop overwriting individual lane frames for a pick.

Edit the existing block to match this code; keep unchanged lines:

```rust
        if let Some(sheets)=sheets {sheets.view(queue,&request.transform,window.full,request.density);}
```

### 8. `src/pick_encode.rs`

Draw every ordinary ink ID with the request-owned group.

Edit the existing block to match this code; keep unchanged lines:

```rust
            for lane in ink {lane.pick_finite(device,&mut pass,view_group,crop_group,tables.as_ref(),&image.depth);}
```

### 9. `src/highlight.rs`

Remove the wrapper’s redundant frame-update traversal. Borrow the same explicit frame through the wrapper. Forward the borrowed frame without extra state.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        Ok(())
    }

    pub fn view(&self, queue: &wgpu::Queue, transform: &[f32; 16], size: [u32; 2], density: f32) {
        for lane in [&self.strokes, &self.paths, &self.markers] { lane.view(queue, transform, size, density); }
    }

    pub fn mesh(&self, pass: &mut wgpu::RenderPass<'_>) { if let Some(mesh) = &self.mesh { mesh.draw_pulled(pass); } }
    pub fn ink(&self, pass: &mut wgpu::RenderPass<'_>) { self.strokes.draw(pass); self.paths.draw(pass); }
    pub fn points(&self, pass: &mut wgpu::RenderPass<'_>) { self.markers.draw(pass); }
    pub fn clipping_bytes(&self)->[u64;2]{[&self.strokes,&self.paths,&self.markers].into_iter().map(Lane::clipping_bytes).fold([0;2],|a,b|[a[0]+b[0],a[1]+b[1]])}
    pub fn bytes(&self) -> u64 { self.mesh.as_ref().map_or(0, |m| m.geometry.allocated_bytes() + m.settings_bytes())
        + self.strokes.usage()[1] + self.paths.usage()[1] + self.markers.usage()[1] }
```

Replace it with:

```rust
        Ok(())
    }

    pub fn mesh(&self, pass: &mut wgpu::RenderPass<'_>) { if let Some(mesh) = &self.mesh { mesh.draw_pulled(pass); } }
    pub fn ink(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup) { self.strokes.draw(pass,view); self.paths.draw(pass,view); }
    pub fn points(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup) { self.markers.draw(pass,view); }
    pub fn clipping_bytes(&self)->[u64;2]{[&self.strokes,&self.paths,&self.markers].into_iter().map(Lane::clipping_bytes).fold([0;2],|a,b|[a[0]+b[0],a[1]+b[1]])}
    pub fn bytes(&self) -> u64 { self.mesh.as_ref().map_or(0, |m| m.geometry.allocated_bytes() + m.settings_bytes())
        + self.strokes.usage()[1] + self.paths.usage()[1] + self.markers.usage()[1] }
```

**Complete updated block reference:**

```rust
use crate::{gpu_mesh::GpuMesh, pick_sources::{Part, Target}, scene::Scene, stroke_gpu::Lane};
use std::rc::Rc;

pub struct Highlight {target:Option<Target>,mesh:Option<GpuMesh>,strokes:Lane,paths:Lane,markers:Lane}

impl Highlight {
    #[cfg(test)]
    pub(crate) fn geometry_for_test(&self)->Option<&Rc<crate::gpu_geometry::GpuGeometry>> {self.mesh.as_ref().map(|m|&m.geometry)}
    pub fn rebase_face(&mut self,queue:&wgpu::Queue,anchor:[f64;3],target:Option<Target>)->bool {
        if self.target!=target||!target.is_some_and(|t|matches!(t.part,Part::Face(_))){return false;}
        let Some(mesh)=self.mesh.as_mut() else{return false;};mesh.rebase(queue,anchor);true
    }

    pub fn set_opacity(&self,value:f32){for lane in [&self.strokes,&self.paths,&self.markers]{lane.set_opacity(value);}}

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self {target:None,mesh:None,strokes: Lane::new(device, format), paths: Lane::joined(device, format), markers: Lane::markers(device, format) }
    }

    pub fn enable_finite(&mut self,device:&wgpu::Device,format:wgpu::TextureFormat){for lane in [&mut self.strokes,&mut self.paths,&mut self.markers]{lane.enable_finite(device,format)}}
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){for lane in [&self.strokes,&self.paths,&self.markers]{lane.finite_frame(device,tables,depth)}}
    pub fn set(&mut self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout, scene: &Scene, target: Option<Target>) -> Result<(), &'static str> {self.set_at(device,layout,scene,target,[0.;3])}
    pub fn set_at(&mut self,device:&wgpu::Device,layout:&wgpu::BindGroupLayout,scene:&Scene,target:Option<Target>,anchor:[f64;3])->Result<(),&'static str>{
        self.target=target.filter(|t|t.valid(scene));self.mesh=None;let mut strokes = Vec::new(); let mut paths = Vec::new(); let mut markers = Vec::new();
        let yellow = crate::colour::selection();
        if let Some(target)=self.target {match target.part {
            Part::Face(key) => {
                let row = scene.objects().iter().find(|r| r.id == target.owner).unwrap();
                let face = row.mesh.parts.faces.iter().position(|&f| f == key).unwrap() as u32;
                // Highlights own derived display data; changing selection changes presentation without mutating source geometry or adding a document history transaction.
                let mut vertices = Vec::new(); let mut normals = Vec::new(); let mut indices = Vec::new();
                for (tri, &source) in row.mesh.indices().chunks_exact(3).zip(&row.mesh.parts.triangles) { if source == face {
                    for &index in tri { indices.push(vertices.len() as u32); vertices.push(row.mesh.vertices()[index as usize]); normals.push(row.mesh.normals()[index as usize]); }
                } }
                let display = crate::mesh::Mesh::new(vertices, indices)?.with_normals(normals)?;
                let mut object = row.clone(); object.mesh = Rc::new(display);
                self.mesh = Some(GpuMesh::upload_at(device,layout,&object,true,anchor));
            }
            Part::BRepEdge(edge)=>{
                let row=scene.objects().iter().find(|r|r.id==target.owner).unwrap();let points=&row.cad.as_ref().unwrap().edges.iter().find(|border|border.edge==edge).unwrap().points;
                for pair in points.windows(2) {let ends=[pair[0],pair[1]].map(|p|crate::render_space::placed(&row.model,anchor).transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());strokes.push(crate::stroke::Stroke {start:ends[0],end:ends[1],colour:yellow,width:3.});}
            }
            Part::SurfaceEdge(edge)=>{
                let row=scene.objects().iter().find(|r|r.id==target.owner).unwrap();let surface=row.surface.as_ref().unwrap();let boundary=surface.grid.boundaries.iter().find(|b|b.edge==edge).unwrap();
                for ends in &boundary.segments {let ends=ends.map(|index|{let p=surface.grid.samples[index as usize].point;crate::render_space::placed(&row.model,anchor).transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32()});strokes.push(crate::stroke::Stroke {start:ends[0],end:ends[1],colour:yellow,width:3.});}
            }
            Part::MeshEdge(a, b) => {
                let row = scene.objects().iter().find(|r| r.id == target.owner).unwrap();
                let index = row.mesh.parts.edges.iter().position(|&edge| edge == (a, b)).unwrap();
                let ends = row.mesh.parts.endpoints[index].map(|p| crate::render_space::placed(&row.model,anchor).transform_point(&session_rust::Point::new(p[0], p[1], p[2])).to_f32());
                strokes.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: yellow, width: 3. });
            }
            Part::PathEdge(index) => {
                let row = scene.paths().iter().find(|r| r.id == target.owner).unwrap(); let points=row.points_at(anchor);
                strokes.push(crate::stroke::Stroke { start: points[index].to_f32(), end: points[index + 1].to_f32(), colour: yellow, width: row.prepared.width + 2. });
            }
            Part::Curve => {
                if let Some(row) = scene.lines().iter().find(|r| r.id == target.owner) {
                    strokes.push(crate::stroke::Stroke { colour: yellow, width: row.prepared.display.width + 2., ..row.stroke_at(anchor) });
                } else if let Some(row) = scene.paths().iter().find(|r| r.id == target.owner) {
                    paths.extend(row.segments_at(anchor).into_iter().map(|mut segment| { segment.stroke.colour = yellow; segment.stroke.width += 2.; segment }));
                }
            }
            Part::Control(_) | Part::SourceControl(_) => {
                let marker = scene.control_markers_at(anchor).into_iter().find(|m| m.parent == target.owner && m.part == target.part).unwrap();
                markers.push(crate::marker::Marker { colour: [1., 1., 0., 1.], ..marker.display });
            }
            Part::CloudPoint(id)=>{
                let p=crate::cloud_pick::position_at(scene,target.owner,id,anchor).ok_or("Cloud point is missing")?;
                markers.push(crate::marker::Marker {center:p.to_f32(),diameter:11.,colour:yellow});
            }
            Part::SheetEntity(_)|Part::Object => {}
        } }
        let flag=self.target.map_or(0,|t|if crate::clipping_rows::owners(scene).contains(&t.owner){4096}else{0});let count=strokes.len();self.strokes.set(device, strokes)?;self.strokes.set_flags(device,vec![flag;count])?; self.paths.set_chains(device, paths)?; self.markers.set_points(device, markers)?; Ok(())
    }

    pub fn objects(&mut self, device: &wgpu::Device, scene: &Scene, ids: &[crate::scene::ObjectId]) -> Result<(), &'static str> {self.objects_at(device,scene,ids,[0.;3])}
    pub fn objects_at(&mut self,device:&wgpu::Device,scene:&Scene,ids:&[crate::scene::ObjectId],anchor:[f64;3])->Result<(),&'static str>{
        self.target=None;self.mesh=None;let yellow = crate::colour::selection();
        self.strokes.set(device, scene.lines().iter().filter(|r| ids.contains(&r.id) && scene.visible(r.id)).map(|r|
            crate::stroke::Stroke { colour: yellow, width: r.prepared.display.width + 2., ..r.stroke_at(anchor) }).chain(scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|r.mesh.parts.endpoints.iter().map(move|pair|{let ends=pair.map(|p|crate::render_space::placed(&r.model,anchor).transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());crate::stroke::Stroke{start:ends[0],end:ends[1],colour:yellow,width:3.}}))))?;
        let planes=crate::clipping_rows::owners(scene);let flags=scene.lines().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)).map(|_|0).chain(scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|std::iter::repeat_n(if planes.contains(&r.id){4096}else{0},r.mesh.parts.endpoints.len()))).collect();self.strokes.set_flags(device,flags)?;
        self.paths.set_chains(device, scene.paths().iter().filter(|r| ids.contains(&r.id) && scene.visible(r.id)).flat_map(|r|
            r.segments_at(anchor).into_iter().map(|mut s| { s.stroke.colour = yellow; s.stroke.width += 2.; s })))?;
        self.markers.set_points(device, scene.points().iter().filter(|r| ids.contains(&r.id) && scene.visible(r.id)).map(|r|
            crate::marker::Marker { colour: yellow, diameter: r.prepared.display.diameter + 4., ..r.marker_at(anchor) }))?;
        Ok(())
    }

    pub fn mesh(&self, pass: &mut wgpu::RenderPass<'_>) { if let Some(mesh) = &self.mesh { mesh.draw_pulled(pass); } }
    pub fn ink(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup) { self.strokes.draw(pass,view); self.paths.draw(pass,view); }
    pub fn points(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup) { self.markers.draw(pass,view); }
    pub fn clipping_bytes(&self)->[u64;2]{[&self.strokes,&self.paths,&self.markers].into_iter().map(Lane::clipping_bytes).fold([0;2],|a,b|[a[0]+b[0],a[1]+b[1]])}
    pub fn bytes(&self) -> u64 { self.mesh.as_ref().map_or(0, |m| m.geometry.allocated_bytes() + m.settings_bytes())
        + self.strokes.usage()[1] + self.paths.usage()[1] + self.markers.usage()[1] }
}
```

### 10. `src/tool_preview.rs`

Remove the wrapper’s redundant frame-update traversal. Borrow the same explicit frame through the wrapper. Forward the borrowed frame without extra state.

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{stroke_gpu::Lane, tool::Draft};

pub struct Preview { strokes: Lane, markers: Lane }

impl Preview {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self { strokes: Lane::new(device, format), markers: Lane::markers(device, format) }
    }
    pub fn set(&mut self,device:&wgpu::Device,draft:Option<&Draft>)->Result<(),&'static str>{self.set_at(device,draft,[0.;3])}
    pub fn set_at(&mut self,device:&wgpu::Device,draft:Option<&Draft>,anchor:[f64;3])->Result<(),&'static str>{
        let yellow = crate::colour::selection(); let mut strokes = Vec::new(); let mut markers = Vec::new();
        if let Some(draft) = draft {
            for wire in if draft.extend.is_some(){Vec::new()}else{draft.outlines()} {for pair in wire.windows(2) {if pair[0]!=pair[1] {strokes.push(crate::stroke::Stroke {start:crate::render_space::relative(pair[0],anchor),end:crate::render_space::relative(pair[1],anchor),width:2.,colour:yellow});}}}
            if let Some(cursor) = draft.cursor {
                markers.push(crate::marker::Marker { center: crate::render_space::relative(cursor,anchor), diameter: 8., colour: yellow });

        } }
        self.strokes.set(device, strokes)?; self.markers.set_points(device, markers)?; Ok(())
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup) { self.strokes.draw(pass,view); self.markers.draw(pass,view); }
    pub fn bytes(&self) -> u64 { self.strokes.usage()[1] + self.markers.usage()[1] }
}
```

### 11. `src/measure_gpu.rs`

Remove the wrapper’s redundant frame-update traversal. Borrow the same explicit frame through the wrapper. Forward the borrowed frame without extra state.

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{measure_mark::Mark, stroke_gpu::Lane};
pub struct Measurement {
    pub mark: Option<Mark>,
    anchor:[f64;3],
    strokes: Lane,
}
impl Measurement {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self {
            mark: None,anchor:[0.;3],
            strokes: Lane::overlay(device, format),
        }
    }
    pub fn set(&mut self,device:&wgpu::Device,mark:Option<&Mark>)->Result<(),&'static str>{self.set_at(device,mark,[0.;3]).map(|_|())}
    pub fn set_at(&mut self,device:&wgpu::Device,mark:Option<&Mark>,anchor:[f64;3])->Result<bool,&'static str>{
        if self.mark.as_ref() == mark&&self.anchor==anchor {
            return Ok(false);
        }
        let mut strokes = Vec::new();
        if let Some(mark) = mark {
            for p in mark.points.windows(2) {
                if p[0] != p[1] {
                    strokes.push(crate::stroke::Stroke {
                        start: crate::render_space::relative(p[0],anchor),
                        end: crate::render_space::relative(p[1],anchor),
                        width: 1.5,
                        colour: [20. / 255., 20. / 255., 20. / 255., 1.],
                    });
                }
            }
        }
        self.strokes.set(device, strokes)?;
        self.mark = mark.cloned();self.anchor=anchor;
        Ok(true)
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup) {
        self.strokes.draw(pass,view)
    }
    pub fn bytes(&self) -> u64 {
        self.strokes.usage()[1]
    }
}
```

### 12. `src/guide_gpu.rs`

Remove the wrapper’s redundant frame-update traversal. Borrow the same explicit frame through the wrapper. Forward the borrowed frame without extra state.

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{stroke::Stroke,marker::Marker,stroke_gpu::Lane};
#[derive(Clone,Debug,PartialEq)]
pub struct Segment{pub stroke:Stroke,pub dashed:bool}
#[derive(Clone,Debug,Default,PartialEq)]
pub struct Overlay{pub anchor:[f64;3],pub segments:Vec<Segment>,pub marks:Vec<Marker>,pub label:Option<([f64;3],String)>}
impl Overlay{
    pub fn at(anchor:[f64;3])->Self {Self {anchor,..Default::default()}}
    pub fn mark(&mut self,point:[f64;3],diameter:f32,colour:[f32;4]){self.marks.push(Marker{center:crate::render_space::relative(point,self.anchor),diameter,colour});}
    pub fn wire(&mut self,points:&[[f64;3]],colour:[f32;4],dashed:bool){for p in points.windows(2){if p[0]!=p[1]{self.segments.push(Segment{stroke:Stroke{start:crate::render_space::relative(p[0],self.anchor),end:crate::render_space::relative(p[1],self.anchor),colour,width:2.},dashed})}}}
    pub fn labels(&self,existing:&[crate::text::TextLabel])->Vec<crate::text::TextLabel>{self.label.as_ref().map_or_else(Vec::new,|(point,label)|crate::measure_mark::Mark{points:vec![*point],label:label.clone()}.labels(existing))}
}
pub struct Guide{pub overlay:Overlay,segments:Lane,marks:Lane}
impl Guide{
    pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{Self{overlay:Default::default(),segments:Lane::guides(device,format),marks:Lane::markers(device,format)}}
    pub fn set(&mut self,device:&wgpu::Device,overlay:&Overlay)->Result<bool,&'static str>{if &self.overlay==overlay{return Ok(false)}self.segments.set_guides(device,&overlay.segments)?;self.marks.set_points(device,overlay.marks.iter().copied())?;self.overlay=overlay.clone();Ok(true)}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup){self.segments.draw(pass,view);self.marks.draw(pass,view)}
    pub fn bytes(&self)->u64{self.segments.usage()[1]+self.marks.usage()[1]}
}
```

### 13. `src/highlight.rs`

Selection reads the existing shared opacity.

Edit the existing block to match this code; keep unchanged lines:

```rust

```

### 14. `src/renderer.rs`

Keep the single renderer opacity owner. One shared frame replaces this lane or wrapper update. Measurement and guides borrow the same color frame. Borrow the existing color group for this draw.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite scene ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:None,stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});
            pass.set_bind_group(2,&clip_group,&[]);
            self.highlight.ink(&mut pass);
            self.mesh_ink.draw(&mut pass);
            self.strokes.draw(&mut pass);self.sheets.draw(&mut pass);
            self.paths.draw(&mut pass);
            self.markers.draw(&mut pass);
            self.highlight.points(&mut pass);
            self.preview.draw(&mut pass);
        }
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("measurement overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});self.measurement.draw(&mut pass);self.guides.draw(&mut pass);}
        if self.outlines_enabled.get()&&!self.meshes.is_empty(){
            let ctx=self.text_context();
            let mut global=self.global_outline.borrow_mut();let global=global.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));
```

Replace it with:

```rust
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite scene ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:None,stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});
            pass.set_bind_group(2,&clip_group,&[]);
            self.highlight.ink(&mut pass,&self.view_group);
            self.mesh_ink.draw(&mut pass,&self.view_group);
            self.strokes.draw(&mut pass,&self.view_group);self.sheets.draw(&mut pass);
            self.paths.draw(&mut pass,&self.view_group);
            self.markers.draw(&mut pass,&self.view_group);
            self.highlight.points(&mut pass,&self.view_group);
            self.preview.draw(&mut pass,&self.view_group);
        }
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("measurement overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});self.measurement.draw(&mut pass,&self.view_group);self.guides.draw(&mut pass,&self.view_group);}
        if self.outlines_enabled.get()&&!self.meshes.is_empty(){
            let ctx=self.text_context();
            let mut global=self.global_outline.borrow_mut();let global=global.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));
```

**Locate this block:**

```rust
                })],
                ..Default::default()
            });
            self.grid.draw(&mut pass);
            pass.set_bind_group(2,&clip_group,&[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
```

Replace it with:

```rust
                })],
                ..Default::default()
            });
            self.grid.draw(&mut pass,&self.view_group);
            pass.set_bind_group(2,&clip_group,&[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
```

**Locate this block:**

```rust
        let display=[f32::from(normals),self.opacity.get(),if self.ambient_enabled.get(){2.}else{f32::from(options[0])},f32::from(options[1])];
        let bytes=crate::surface_view::bytes(*transform,display,self.size,self.density);
        self.queue.write_buffer(&self.uniform, 0, &bytes);
        self.strokes.view(&self.queue, transform, self.size, self.density);
        self.sheets.view(&self.queue,transform,self.size,self.density);
        self.paths.view(&self.queue, transform, self.size, self.density);
        self.markers.view(&self.queue, transform, self.size, self.density);
        self.grid.view(&self.queue, transform, self.size, self.density);
        self.mesh_ink.view(&self.queue, transform, self.size, self.density);
        self.highlight.view(&self.queue, transform, self.size, self.density);
        self.preview.view(&self.queue, transform, self.size, self.density);
        self.measurement.view(&self.queue,transform,self.size,self.density);self.guides.view(&self.queue,transform,self.size,self.density);
        let [r, g, b] = if self.ambient_enabled.get() { [0.94; 3] } else { background.rgb() };
        let mut encoder = self.device.create_command_encoder(&Default::default());
        let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);tiles.encode(&self.device,&mut encoder,&self.view_group,*transform);
```

Replace it with:

```rust
        let display=[f32::from(normals),self.opacity.get(),if self.ambient_enabled.get(){2.}else{f32::from(options[0])},f32::from(options[1])];
        let bytes=crate::surface_view::bytes(*transform,display,self.size,self.density);
        self.queue.write_buffer(&self.uniform, 0, &bytes);
        self.sheets.view(&self.queue,transform,self.size,self.density);
        let [r, g, b] = if self.ambient_enabled.get() { [0.94; 3] } else { background.rgb() };
        let mut encoder = self.device.create_command_encoder(&Default::default());
        let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);tiles.encode(&self.device,&mut encoder,&self.view_group,*transform);
```

**Locate this block:**

```rust
    fn prepare_clouds(&self,transform:[f32;16]) {
        if let Some(lane)=self.clouds.borrow_mut().as_mut() {lane.set_clipping(self.clipping.borrow().writes);if let Err(error)=lane.prepare(&self.text_context(),transform,self.size,self.density,self.cloud_view.get()) {lane.failed();*self.cloud_error.borrow_mut()=Some(error.to_string());} else {self.cloud_error.borrow_mut().take();}}
    }
    pub fn set_opacity(&mut self,value:f32){if !value.is_finite(){return;}let value=value.clamp(0.,1.);if self.opacity.get()==value{return;}self.opacity.set(value);self.cancel_pick();self.pyramid_revision=self.pyramid_revision.wrapping_add(1);for mesh in &mut self.meshes{mesh.set_xray(&self.queue,value==0.);}for lane in [&self.mesh_ink,&self.strokes,&self.paths,&self.markers,&self.grid]{lane.set_opacity(value);}self.highlight.set_opacity(value);if let Some(o)=self.outline.borrow_mut().as_mut(){o.invalidate();}if let Some(o)=self.global_outline.borrow_mut().as_mut(){o.invalidate();}}
    pub fn opacity(&self)->f32{self.opacity.get()}
    pub fn set_outlines(&self,on:bool){self.outlines_enabled.set(on);if !on{self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();}}
    pub fn global_outline_usage(&self)->[u64;4]{self.global_outline.borrow().as_ref().map_or([0;4],|o|o.usage())}
```

Replace it with:

```rust
    fn prepare_clouds(&self,transform:[f32;16]) {
        if let Some(lane)=self.clouds.borrow_mut().as_mut() {lane.set_clipping(self.clipping.borrow().writes);if let Err(error)=lane.prepare(&self.text_context(),transform,self.size,self.density,self.cloud_view.get()) {lane.failed();*self.cloud_error.borrow_mut()=Some(error.to_string());} else {self.cloud_error.borrow_mut().take();}}
    }
    pub fn set_opacity(&mut self,value:f32){if !value.is_finite(){return;}let value=value.clamp(0.,1.);if self.opacity.get()==value{return;}self.opacity.set(value);self.cancel_pick();self.pyramid_revision=self.pyramid_revision.wrapping_add(1);for mesh in &mut self.meshes{mesh.set_xray(&self.queue,value==0.);}if let Some(o)=self.outline.borrow_mut().as_mut(){o.invalidate();}if let Some(o)=self.global_outline.borrow_mut().as_mut(){o.invalidate();}}
    pub fn opacity(&self)->f32{self.opacity.get()}
    pub fn set_outlines(&self,on:bool){self.outlines_enabled.set(on);if !on{self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();}}
    pub fn global_outline_usage(&self)->[u64;4]{self.global_outline.borrow().as_ref().map_or([0;4],|o|o.usage())}
```

**Complete updated block reference:**

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
    surface_options:std::cell::Cell<[bool;2]>,
    world_bounds:std::cell::RefCell<Option<crate::bounds::Bounds>>,
    space:crate::render_space::Origin,
    opacity:std::cell::Cell<f32>,
    ambient_enabled:std::cell::Cell<bool>,
    ambient:std::cell::RefCell<Option<crate::ambient_gpu::Ambient>>,
    ambient_rows:crate::ambient_rows::Rows,
    pyramid:std::cell::RefCell<Option<crate::pyramid_gpu::Pyramid>>,
    pyramid_revision:u64,
    caps:std::cell::RefCell<crate::caps_gpu::Caps>,
    clipping:std::cell::RefCell<crate::clipping_gpu::Clipping>,
    tiles:std::cell::RefCell<crate::tile_gpu::Tiles>,
    guides:crate::guide_gpu::Guide,
    measurement:crate::measure_gpu::Measurement,
    sheets:crate::sheet_gpu::Lane,
    clouds:std::cell::RefCell<Option<crate::cloud_gpu::Lane>>,
    cloud_view:std::cell::Cell<crate::cloud_gpu::View>,
    cloud_error:std::cell::RefCell<Option<String>>,
    outline:std::cell::RefCell<Option<crate::outline_gpu::Outline>>,
    global_outline:std::cell::RefCell<Option<crate::outline_gpu::Outline>>,
    outlines_enabled:std::cell::Cell<bool>,
    outline_composite:std::cell::RefCell<Option<crate::outline_composite::Composite>>,
    text:std::cell::RefCell<Option<crate::scene_text_gpu::Lane>>,
    text_error:std::cell::RefCell<Option<String>>,
    full_text_fonts:std::cell::RefCell<Option<Vec<Vec<u8>>>>,
    presentation_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    scene_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    format:wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    instances:crate::instance_gpu::Lane,
    instance_rows:std::collections::HashSet<ObjectId>,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
    highlight: crate::highlight::Highlight,
    preview: crate::tool_preview::Preview,
    widget: crate::widget::Widget,
    meshes: Vec<GpuMesh>,
    geometry: crate::geometry_cache::GeometryCache,
    settings_allocations: usize,
    settings_writes: usize,
    settings_bytes: usize,
    uniform: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    depth: wgpu::TextureView,
    primitive:crate::primitive_image::Image,
    strokes: crate::stroke_gpu::Lane,
    paths: crate::stroke_gpu::Lane,
    markers: crate::stroke_gpu::Lane,
    grid: crate::stroke_gpu::Lane,
    grid_settings: Option<crate::grid::Settings>,
    grid_plane:crate::cplane::CPlane,grid_anchor:[f64;3],
    size: [u32; 2],
    density: f32,
    picker: crate::pick_gpu::Picker,
    sources: crate::pick_sources::Sources,
    faces: crate::face_pick::Faces,
    mesh_ink: crate::stroke_gpu::Lane,
    pick_mode:crate::pick_sources::Mode,
    pick_cursor:Option<([f32;2],[u64;3])>,
}

impl Renderer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        crate::shader_check::check(&device, crate::clipping_gpu::source(include_str!("cloud_points.wgsl"),2,0), "Cloud points");
        crate::shader_check::check(&device, include_str!("cloud_resolve.wgsl").into(), "Cloud resolve");
        crate::shader_check::check(&device, crate::finite_project_source::source(), "Finite projection");
        crate::shader_check::check(&device, crate::finite_project_source::tiles(), "Finite tiles");
        crate::shader_check::check(&device, crate::finite_project_source::scan(), "Finite scan");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0).into()),
        });
        let view_layout = crate::surface_view::layout(&device);
        let model_layout = crate::gpu_layout::uniform(&device, crate::object_record::BYTES as u64);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"), bind_group_layouts: &[Some(&view_layout), Some(&model_layout),Some(&crate::clipping_gpu::layout(&device)),Some(&crate::gpu_geometry::GpuGeometry::storage_layout(&device))], immediate_size: 0 });
        let mesh_pipeline = |depth,primitive_write| device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[crate::draw_slots::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState{format,blend:Some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(primitive_write))],
            }),
            primitive: Default::default(),
            depth_stencil: Some(depth),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let pipeline = mesh_pipeline(crate::depth::surface(),true);
        let tiles=std::cell::RefCell::new(crate::tile_gpu::Tiles::new(&device,&view_layout));
        let instances=crate::instance_gpu::Lane::new(&device,&queue,format,&view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink(),false);
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
        let widget = crate::widget::Widget::new(&device, format);
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: crate::surface_view::BYTES,
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
        let primitive=crate::primitive_image::Image::new(&device,[640,480]);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
        let grid = crate::stroke_gpu::Lane::grid(&device, format);
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {surface_options:Default::default(),world_bounds:Default::default(),space:Default::default(),opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, primitive, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy,grid_anchor:[0.;3], size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode:Default::default(),pick_cursor:None };
        for lane in [&mut renderer.mesh_ink,&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers]{lane.enable_finite(&renderer.device,format);}
        renderer.highlight.enable_finite(&renderer.device,format);
        renderer.set_scene(scene, None);
        renderer
    }

    pub fn anchor(&self)->[f64;3] {self.space.anchor()}

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: crate::depth::FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        if self.size != [size.width, size.height] { self.cancel_pick(); }
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
        self.primitive=crate::primitive_image::Image::new(&self.device,self.size);
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 && density != self.density { self.cancel_pick(); self.density = density; }
    }

    pub fn stroke_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.strokes.usage();
        [count, bytes, self.strokes.uploads, self.strokes.uploaded_bytes]
    }

    pub fn path_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.paths.usage();
        [count, bytes, self.paths.uploads, self.paths.uploaded_bytes]
    }

    pub fn point_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.markers.usage();
        [count, bytes, self.markers.uploads, self.markers.uploaded_bytes]
    }

    #[cfg(test)]
    pub(crate) fn use_uniform_surface_records(&mut self) {self.instances.ordinary.clear();}

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        let anchor=self.anchor();
        self.ambient_rows.set(scene);if let Some(ambient)=self.ambient.borrow_mut().as_mut(){if self.ambient_enabled.get(){ambient.sources.set(&self.device,&self.queue,&self.ambient_rows.specs);}}
        self.pyramid_revision=self.pyramid_revision.wrapping_add(1);
        self.clipping.borrow_mut().set(scene);
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        self.cancel_pick(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, None).expect("Empty selection");
        self.set_source_ink(scene);
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let matching = old.iter().position(|row| row.id == object.id
                && std::rc::Rc::ptr_eq(&row.geometry.source, &object.mesh));
            let row = if let Some(index) = matching {
                let mut row = old.swap_remove(index);
                let [calls, bytes] = row.update(&self.queue, object,style(scene,object.id,selected==Some(object.id)));
                self.settings_writes += calls;
                self.settings_bytes += bytes;
                row
            } else {
                let geometry = self.geometry.get(&crate::text_gpu_support::GpuCtx::new(&self.device,&self.queue), &object.mesh);
                self.settings_allocations += 1;
                GpuMesh::with_geometry_at(&self.device,&layout,object,
                    style(scene,object.id,selected==Some(object.id)),geometry,anchor)
            };
            self.meshes.push(row);
        }
        drop(old);
        self.caps.borrow_mut().set_scene(&self.queue,scene,&mut self.meshes,&self.clipping.borrow().rows);
        for mesh in &mut self.meshes{mesh.set_xray(&self.queue,self.opacity.get()==0.);}
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,true).expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        self.sheets.set_anchored(&self.device,&self.queue,scene,selected.as_slice(),&self.sources.sheet_bases,anchor).expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        if let Err(error)=self.set_clouds(scene,selected.as_slice()) {*self.cloud_error.borrow_mut()=Some(error.to_string());}
        if let Err(error)=self.set_scene_text(scene,selected.as_slice()) {*self.text_error.borrow_mut()=Some(error.to_string());}
        *self.world_bounds.borrow_mut()=scene.bounds();
    }

    pub fn grow_camera(&self,camera:&mut crate::camera::Camera) {
        if let Some(bounds)=self.world_bounds.borrow().as_ref() {camera.grow_extent(bounds);}
    }

    pub fn camera_matrix(&self,camera:&crate::camera::Camera)->[f32;16] {camera.view_projection_anchored(self.anchor()).to_f32()}
    pub fn rebase_delay(&self,now:f64)->Option<u32> {self.space.retry_after_ms(now)}
    pub fn prepare_origin(&mut self,camera:&crate::camera::Camera,scene:&Scene,target:Option<crate::pick_sources::Target>,ids:&[ObjectId],now:f64)->Result<crate::render_space::Rebase,&'static str> {
        let previous=self.anchor();let mut frame=self.space.update(camera.target,camera.distance,now).ok_or("Invalid camera origin")?;
        frame.moved&=previous.map(f64::to_bits)!=frame.anchor.map(f64::to_bits);
        if frame.moved {
            self.pyramid_revision=self.pyramid_revision.wrapping_add(1);
            for mesh in &mut self.meshes {let[calls,bytes]=mesh.rebase(&self.queue,frame.anchor);self.settings_writes+=calls;self.settings_bytes+=bytes;}
            self.set_source_ink(scene);
            if !self.highlight.rebase_face(&self.queue,frame.anchor,target){self.set_highlight(scene,target)?;}
            self.select_objects(scene,ids)?;
            self.set_grid_on(self.grid_settings,self.grid_plane)?;
        }
        self.cloud_camera(camera);Ok(frame)
    }

    fn set_source_ink(&mut self,scene:&Scene) {
        let anchor=self.anchor();
        self.sources = crate::pick_sources::Sources::anchored(scene,anchor).expect("Prepared source topology is bounded");
        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().copied()).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().copied()).expect("Source edge IDs match");
        let planes=crate::clipping_rows::owners(scene);let flags=self.sources.mesh_ids.iter().map(|id|crate::sub_id::decode(id[1]).and_then(|(_,address)|self.sources.edges.get(address)).map_or(0,|target|if planes.contains(&target.owner){4096}else{0})).collect();self.mesh_ink.set_flags(&self.device,flags).expect("Original plane ink flags match source addresses");
        self.markers.set_points(&self.device, scene.points().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut marker=row.marker_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {marker.colour=colour;}marker}).chain(scene.control_markers_at(anchor).into_iter().map(|c| c.display)))
            .expect("Prepared point placement fits the display range");
        self.paths.set_chains(&self.device, scene.paths().iter().filter(|row| scene.visible(row.id)).flat_map(|row| {let mut segments=row.segments_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {for segment in &mut segments {segment.stroke.colour=colour;}}segments}))
            .expect("Prepared path placement fits the display range");
        self.strokes.set(&self.device, scene.lines().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut stroke=row.stroke_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {stroke.colour=colour;}stroke}))
            .expect("Prepared line placement fits the display range");
        self.markers.set_ids(&self.device, self.sources.marker_ids.iter().copied()).expect("Marker owners match");
        self.paths.set_ids(&self.device, self.sources.path_ids.iter().copied()).expect("Path owners match");
        self.strokes.set_ids(&self.device, self.sources.line_ids.iter().copied()).expect("Line owners match");
    }

    pub fn request_pick(&mut self, screen: [f32; 2], transform: [f32; 16]) -> Result<(), &'static str> {
        let window = crate::pick_window::Window::new(screen, self.size, self.density).ok_or("Invalid pick cursor")?;
        let origin=self.anchor();self.picker.queue_anchored(crate::pick_gpu::Request{window,transform,density:self.density,mode:self.pick_mode},origin)?;
        self.pick_cursor=Some((screen,origin.map(f64::to_bits)));self.pump_pick(transform);Ok(())
    }

    fn pump_pick(&mut self, transform: [f32; 16]) {
        let Ok(view) = crate::pick_view::View::anchored(transform,self.size,self.density,self.anchor()) else { self.cancel_pick(); return; };
        if self.pick_mode == crate::pick_sources::Mode::Face && !self.faces.ready(self.meshes.len()) {
            self.faces.set(&self.device, &self.meshes, &self.sources.mesh_bases,&self.clipping.borrow().buffer);
        }
        if let Some(lane)=self.text.borrow_mut().as_mut() {if let Err(error)=lane.prepare(&self.text_context(),&self.text_frame(transform)) {*self.text_error.borrow_mut()=Some(error.to_string());return;}}
        self.clipping.borrow_mut().frame_anchored(&self.queue,transform,self.anchor(),self.size[1],self.density);
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) {
            let mut clipping=self.clipping.borrow_mut();clipping.frame_anchored(&self.queue,transform,self.anchor(),self.size[1],self.density);
            let bytes=crate::surface_view::bytes(transform,[0.;4],self.size,self.density);self.queue.write_buffer(&self.uniform,0,&bytes);
            let tables={let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);let mut encoder=self.device.create_command_encoder(&Default::default());tiles.encode(&self.device,&mut encoder,&self.view_group,transform);self.queue.submit([encoder.finish()]);tiles.map_report();crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.primitive.view.clone()}};
            self.picker.submit_finite(&self.device, &self.queue, request,
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces,self.text.borrow().as_ref(),self.clouds.borrow().as_ref(),Some(&self.sheets),Some(&tables),&clipping.group,&clipping.buffer,Some(&mut self.caps.borrow_mut()),Some(&self.instances)); }
    }

    pub fn take_pick(&mut self, transform: [f32; 16]) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let view = match crate::pick_view::View::anchored(transform,self.size,self.density,self.anchor()) { Ok(view) => view, Err(error) => {
            self.cancel_pick(); return Some(Err(error));
        } };
        match self.picker.take(view) {
            Some(Ok(crate::pick_gpu::Reply::Hit(hit)))=>{self.pick_cursor=None;Some(Ok(hit))},
            Some(Ok(crate::pick_gpu::Reply::Stale))=>{
                if let Some((screen,_))=self.pick_cursor.filter(|(_,origin)|*origin!=self.anchor().map(f64::to_bits)) {
                    if let Err(error)=self.request_pick(screen,transform){self.cancel_pick();return Some(Err(error));}
                } else {
                    self.pump_pick(transform);
                    if self.picker.usage()[0]==0&&!self.picker.queued(){self.pick_cursor=None;}
                }
                None
            },
            Some(Err(error))=>{self.cancel_pick();Some(Err(error))},None=>None,
        }
    }

    pub fn set_pick_mode(&mut self, mode: crate::pick_sources::Mode) {
        if self.pick_mode != mode { self.cancel_pick(); self.pick_mode = mode; }
    }

    pub fn resolve_pick(&self, scene: &Scene, pixel: [u32; 2]) -> Option<crate::pick_sources::Target> {
        self.sources.resolve(scene, pixel, self.pick_mode)
    }

    pub fn part_usage(&self) -> [u64; 4] { let [count, bytes] = self.mesh_ink.usage(); [count, bytes, self.faces.bytes(), self.sources.bytes() as u64] }

    pub fn cancel_pick(&mut self) {self.pick_cursor=None;self.picker.cancel();}

    pub fn pick_usage(&self) -> [u64; 5] {
        let [active, pending, bytes] = self.picker.usage();
        [active, pending, bytes + self.strokes.pick_bytes() + self.paths.pick_bytes() + self.markers.pick_bytes() + self.mesh_ink.pick_bytes() + self.mesh_ink.usage()[1] + self.faces.bytes(),
            self.picker.submitted, self.picker.coalesced]
    }

    pub fn geometry_owners(&self) -> impl Iterator<Item = &std::rc::Rc<crate::gpu_geometry::GpuGeometry>> {
        self.meshes.iter().map(|row| &row.geometry)
    }

    pub fn usage(&self)->[u64;4]{let mut usage=crate::memory::gpu(&self.meshes);usage[1]+=self.geometry.allocated_bytes();usage}
    pub fn arena_cpu_bytes(&self)->usize{self.geometry.cpu_bytes()}

    pub fn uploaded_bytes(&self) -> u64 {
        self.instances.written_bytes + self.geometry.uploaded_bytes + self.settings_allocations as u64 * crate::object_record::BYTES as u64 + self.settings_bytes as u64
            + self.strokes.uploaded_bytes + self.paths.uploaded_bytes + self.markers.uploaded_bytes + self.grid.uploaded_bytes
    }

    pub fn set_grid(&mut self,settings:Option<crate::grid::Settings>)->Result<(),&'static str>{self.set_grid_on(settings,crate::cplane::CPlane::Xy)}
    pub fn set_grid_on(&mut self,settings:Option<crate::grid::Settings>,plane:crate::cplane::CPlane)->Result<(),&'static str>{
        let anchor=self.anchor();
        if settings==self.grid_settings&&plane==self.grid_plane&&anchor==self.grid_anchor{return Ok(())}
        let strokes=settings.map(|s|s.strokes_on_at(plane,anchor)).transpose()?.unwrap_or_default();
        self.grid.set(&self.device,strokes)?;self.grid_settings=settings;self.grid_plane=plane;self.grid_anchor=anchor;Ok(())
    }

    pub fn grid_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.grid.usage(); [count, bytes, self.grid.uploads, self.grid.uploaded_bytes]
    }

    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        let anchor=self.anchor();self.highlight.set_at(&self.device,&self.pipeline.get_bind_group_layout(1),scene,target,anchor)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
        let anchor=self.anchor();
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        for mesh in &mut self.meshes { if let Some(row) = scene.objects().iter().find(|r| r.id == mesh.id) {
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,false)?;
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;self.sheets.set_anchored(&self.device,&self.queue,scene,ids,&self.sources.sheet_bases,anchor)?;
        if !ids.is_empty() { self.highlight.objects_at(&self.device,scene,ids,anchor)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
    }

    pub fn set_guides(&mut self,overlay:&crate::guide_gpu::Overlay)->Result<(),&'static str>{if self.guides.set(&self.device,overlay)?{self.set_scene_measure_text().map_err(|_|"Cannot prepare editing guide label")?}Ok(())}
    pub fn guide_bytes(&self)->u64{self.guides.bytes()}
    pub fn set_measurement(&mut self,mark:Option<&crate::measure_mark::Mark>)->Result<(),&'static str>{let anchor=self.anchor();if self.measurement.set_at(&self.device,mark,anchor)?{self.set_scene_measure_text().map_err(|_|"Cannot prepare measurement annotation")?;}Ok(())}
    pub fn measurement_bytes(&self)->u64{self.measurement.bytes()}
    fn merge_measurement(&self,mut labels:Vec<crate::text::TextLabel>)->Vec<crate::text::TextLabel>{if let Some(mark)=&self.measurement.mark{labels.extend(mark.labels(&labels));}labels.extend(self.guides.overlay.labels(&labels));labels}
    fn set_scene_measure_text(&self)->anyhow::Result<()>{let mut labels=self.scene_text.borrow().clone();labels.extend(self.presentation_text.borrow().clone());self.adopt_text(self.merge_measurement(labels))}
    pub fn set_preview(&mut self,draft:Option<&crate::tool::Draft>)->Result<(),&'static str>{let anchor=self.anchor();self.preview.set_at(&self.device,draft,anchor)}
    pub fn preview_bytes(&self) -> u64 { self.preview.bytes() }

    pub fn set_gizmo(&mut self, camera:&crate::camera::Camera, origin:Option<[f64;3]>, active:f32) {
        let anchor=self.anchor();let matrix=camera.view_projection_anchored(anchor);
        let layout=origin.map(|p|std::array::from_fn(|i|p[i]-anchor[i])).and_then(|p|crate::widget_layout::world_per_pixel(&matrix,p,self.size[1] as f64/self.density as f64)
            .and_then(|scale|crate::widget_layout::layout(&matrix,p,scale,self.size,active)));
        self.widget.prepare(&self.device,&self.queue,layout);
    }
    pub fn gizmo_usage(&self) -> [u64;4] { self.widget.usage() }
    pub fn gizmo_extent(&self) -> Option<[u32;2]> {self.widget.extent()}

    fn text_context(&self)->crate::text_gpu_support::GpuCtx {crate::text_gpu_support::GpuCtx::new(&self.device,&self.queue)}
    fn text_frame(&self,transform:[f32;16])->crate::scene_text_gpu::TextFrame {crate::scene_text_gpu::TextFrame {mvp:transform,origin:self.anchor(),framebuffer:self.size,logical:[self.size[0] as f64/self.density as f64,self.size[1] as f64/self.density as f64],ortho_half_height:0.,clip:{let clip=self.clipping.borrow();let mut planes=[[0.;4];6];if clip.enabled {for (p,row) in planes.iter_mut().zip(&clip.rows){*p=[row.plane.normal[0],row.plane.normal[1],row.plane.normal[2],row.plane.offset];}}planes}}}
    fn adopt_text(&self,labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        if labels.is_empty() {self.text.borrow_mut().take();self.text_error.borrow_mut().take();return Ok(());}
        let mut lane=self.text.borrow_mut();if lane.is_none() {let mut new=crate::scene_text_gpu::Lane::new(&self.text_context(),crate::text_gpu_support::Target {format:self.format,samples:1});if let Some(fonts)=self.full_text_fonts.borrow().as_ref() {new.document.replace_fonts(fonts.clone())?;}*lane=Some(new);}lane.as_mut().unwrap().set_labels(labels)
    }
    pub fn set_text(&self,mut labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        for (i,label) in labels.iter_mut().enumerate() {label.id=u32::MAX-i as u32;label.object=None;}
        let mut merged=self.scene_text.borrow().clone();merged.extend(labels.clone());self.adopt_text(self.merge_measurement(merged))?;*self.presentation_text.borrow_mut()=labels;Ok(())
    }
    fn set_scene_text(&self,scene:&Scene,selected:&[ObjectId])->anyhow::Result<()> {
        let labels=scene.text_labels(selected).map_err(anyhow::Error::msg)?;let mut merged=labels.clone();merged.extend(self.presentation_text.borrow().clone());self.adopt_text(self.merge_measurement(merged))?;*self.scene_text.borrow_mut()=labels;Ok(())
    }
    pub fn refresh_text_bounds(&self,scene:&mut Scene)->bool {
        let mut changed=false;
        if let Some(lane)=self.text.borrow().as_ref() {for row in scene.texts_mut() {if let Some(run)=lane.document.runs.iter().find(|run|run.label.id==row.id.value()) {
            let extent=run.buffer.layout_runs().fold([0f64;2],|[w,h],line|[w.max(line.line_w as f64),h.max((line.line_top+line.line_height) as f64)]);
            if row.prepared.extent!=extent {row.prepared.extent=extent;changed=true;}
        }}}
        if changed {*self.world_bounds.borrow_mut()=scene.bounds();}
        changed
    }
    pub fn text_stats(&self)->Option<crate::scene_text_gpu::TextStats> {self.text.borrow().as_ref().map(|lane|lane.stats.clone())}
    pub fn text_scene_usage(&self)->[u64;4] {self.text.borrow().as_ref().map_or([0;4],|lane|[lane.allocated_bytes(),lane.texture_bytes(),lane.stats.world_plane_rasterizations,lane.document.runs.len() as u64])}
    pub fn replace_text_fonts(&self,fonts:Vec<Vec<u8>>)->anyhow::Result<()> {if let Some(lane)=self.text.borrow_mut().as_mut() {lane.document.replace_fonts(fonts.clone())?;} else {let mut document=crate::text::TextDocument::new();document.replace_fonts(fonts.clone())?;}*self.full_text_fonts.borrow_mut()=Some(fonts);Ok(())}
    pub fn clear_text_fonts(&self) {self.full_text_fonts.borrow_mut().take();}

    pub fn text_error(&self)->Option<String> {self.text_error.borrow().clone()}
    pub fn text_usage(&self)->[u64;6] {self.text.borrow().as_ref().map_or([0;6],|lane|{[lane.document.shape_count,lane.stats.preparations,lane.stats.skipped_preparations,lane.stats.raster_images as u64,lane.stats.raster_image_capacity_bytes as u64,lane.stats.atlas_resets]})}
    pub fn text_demo(&self)->anyhow::Result<()> {self.set_text(vec![crate::text_example::label()])}

    pub fn cloud_camera(&self,camera:&crate::camera::Camera) {self.cloud_view.set(crate::cloud_gpu::View::anchored(camera,self.anchor()));self.sheets.camera(camera);}
    pub fn cloud_usage(&self)->[u64;5] {self.clouds.borrow().as_ref().map_or([0;5],|lane|lane.usage())}
    pub fn cloud_error(&self)->Option<String> {self.cloud_error.borrow().clone()}
    fn set_clouds(&self,scene:&Scene,selected:&[ObjectId])->Result<(),&'static str> {
        if scene.clouds().is_empty()&&scene.streams().is_empty() {self.clouds.borrow_mut().take();self.cloud_error.borrow_mut().take();return Ok(());}
        let mut owner=self.clouds.borrow_mut();let lane=owner.get_or_insert_with(||crate::cloud_gpu::Lane::new(&self.text_context(),self.format,&self.clipping.borrow().group));lane.set_scene(&self.text_context(),scene,selected)
    }
    fn prepare_clouds(&self,transform:[f32;16]) {
        if let Some(lane)=self.clouds.borrow_mut().as_mut() {lane.set_clipping(self.clipping.borrow().writes);if let Err(error)=lane.prepare(&self.text_context(),transform,self.size,self.density,self.cloud_view.get()) {lane.failed();*self.cloud_error.borrow_mut()=Some(error.to_string());} else {self.cloud_error.borrow_mut().take();}}
    }
    pub fn set_opacity(&mut self,value:f32){if !value.is_finite(){return;}let value=value.clamp(0.,1.);if self.opacity.get()==value{return;}self.opacity.set(value);self.cancel_pick();self.pyramid_revision=self.pyramid_revision.wrapping_add(1);for mesh in &mut self.meshes{mesh.set_xray(&self.queue,value==0.);}if let Some(o)=self.outline.borrow_mut().as_mut(){o.invalidate();}if let Some(o)=self.global_outline.borrow_mut().as_mut(){o.invalidate();}}
    pub fn opacity(&self)->f32{self.opacity.get()}
    pub fn set_outlines(&self,on:bool){self.outlines_enabled.set(on);if !on{self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();}}
    pub fn global_outline_usage(&self)->[u64;4]{self.global_outline.borrow().as_ref().map_or([0;4],|o|o.usage())}
    pub fn outline_usage(&self)->[u64;4] {self.outline.borrow().as_ref().map_or([0;4],|outline|outline.usage())}

    pub fn highlight_bytes(&self) -> u64 { self.highlight.bytes() }

    pub fn sheet_camera(&self,camera:&crate::camera::Camera) {self.sheets.camera(camera);}
    #[cfg(test)]
    pub(crate) fn tile_buffers(&self)->[wgpu::Buffer;2]{let tiles=self.tiles.borrow();[tiles.projected.clone(),tiles.buffer.clone()]}
    pub fn set_clipping(&self,enabled:bool,solid:bool){let mut clip=self.clipping.borrow_mut();clip.enabled=enabled;clip.solid=solid;self.caps.borrow_mut().enabled=enabled;}
    pub fn set_lighting(&self,on:bool,backfaces:bool){self.surface_options.set([on,backfaces]);}
    pub fn set_ambient(&self,enabled:bool){if self.ambient_enabled.replace(enabled)==enabled{return;}let mut ambient=self.ambient.borrow_mut();if enabled{let owner=ambient.get_or_insert_with(||crate::ambient_gpu::Ambient::new(&self.device,self.format));owner.sources.set(&self.device,&self.queue,&self.ambient_rows.specs);}else if let Some(owner)=ambient.as_mut(){owner.clear(&self.device,&self.queue);}}
    pub fn ambient_usage(&self)->[u64;5]{self.ambient.borrow().as_ref().map_or([0;5],crate::ambient_gpu::Ambient::usage)}
    pub fn ambient_error(&self)->Option<&'static str>{self.ambient.borrow().as_ref().and_then(|owner|owner.error)}
    pub fn set_depth_pyramid(&self,enabled:bool){let mut owner=self.pyramid.borrow_mut();if enabled {if owner.is_none(){*owner=Some(crate::pyramid_gpu::Pyramid::new(&self.device));}}else{*owner=None;}}
    pub fn depth_pyramid_usage(&self)->[u64;4]{self.pyramid.borrow().as_ref().map_or([0;4],crate::pyramid_gpu::Pyramid::usage)}
    pub fn depth_pyramid_error(&self)->Option<&'static str>{self.pyramid.borrow().as_ref().and_then(|p|p.error)}
    pub fn cap_error(&self)->Option<&'static str>{self.caps.borrow().pick_error}
    pub fn cap_usage(&self)->[u64;5]{self.caps.borrow().usage()}
    pub fn clipping_usage(&self)->[u64;5]{let clip=self.clipping.borrow();[clip.rows.len()as u64,u64::from(clip.enabled),u64::from(clip.solid),clip.writes,clip.buffer.size()]}
    pub fn clipping_ink_bytes(&self)->[u64;2]{[&self.mesh_ink,&self.strokes,&self.paths,&self.markers,&self.grid].into_iter().map(crate::stroke_gpu::Lane::clipping_bytes).fold(self.highlight.clipping_bytes(),|a,b|[a[0]+b[0],a[1]+b[1]])}
    pub fn clipping_error(&self)->Option<&'static str>{self.clipping.borrow().error}
    pub fn tile_usage(&self)->[u64;7]{self.tiles.borrow().usage()}
    pub fn sheet_usage(&self)->[u64;5] {self.sheets.usage()}
    pub fn instance_usage(&self)->[u64;5] {self.instances.usage()}
    pub fn instance_allocations(&self)->[u64;3] {self.instances.allocation_parts()}
    pub fn stats(&self) -> [usize; 4] {
        [self.geometry.uploads, self.settings_allocations, self.settings_writes, self.settings_bytes]
    }

    pub fn draw(
        &self,
        view: &wgpu::TextureView,
        background: &crate::background::Background,
        transform: &[f32; 16],
    ) { self.draw_mode(view, background, transform, false); }

    pub fn draw_mode(&self, view: &wgpu::TextureView, background: &crate::background::Background,
        transform: &[f32; 16], normals: bool,
    ) {
        let (clip_group,clip_revision)={let mut clip=self.clipping.borrow_mut();clip.frame_anchored(&self.queue,*transform,self.anchor(),self.size[1],self.density);(clip.group.clone(),clip.writes)};
        self.caps.borrow_mut().prepare(&self.device,&self.queue,self.format,self.size,{let clip=self.clipping.borrow();clip.enabled&&!clip.rows.is_empty()});
        self.prepare_clouds(*transform);
        let options=self.surface_options.get();
        let display=[f32::from(normals),self.opacity.get(),if self.ambient_enabled.get(){2.}else{f32::from(options[0])},f32::from(options[1])];
        let bytes=crate::surface_view::bytes(*transform,display,self.size,self.density);
        self.queue.write_buffer(&self.uniform, 0, &bytes);
        self.sheets.view(&self.queue,transform,self.size,self.density);
        let [r, g, b] = if self.ambient_enabled.get() { [0.94; 3] } else { background.rgb() };
        let mut encoder = self.device.create_command_encoder(&Default::default());
        let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);tiles.encode(&self.device,&mut encoder,&self.view_group,*transform);
        let tables=crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.primitive.view.clone()};for lane in [&self.mesh_ink,&self.strokes,&self.paths,&self.markers]{lane.finite_frame(&self.device,&tables,&self.depth);}self.highlight.finite_frame(&self.device,&tables,&self.depth);
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(crate::depth::FAR as f32),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
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
                }),Some(wgpu::RenderPassColorAttachment{
                    view:&self.primitive.view,depth_slice:None,resolve_target:None,
                    ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store},
                })],
                ..Default::default()
            });
            self.grid.draw(&mut pass,&self.view_group);
            pass.set_bind_group(2,&clip_group,&[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {if !self.instances.draw_ordinary(&mut pass,mesh){pass.set_pipeline(&self.pipeline);self.instances.bind_own(&mut pass);mesh.draw_pulled(&mut pass);}}
            if self.opacity.get()>=1.{self.instances.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            self.highlight.mesh(&mut pass);
        }
        self.caps.borrow_mut().draw(&self.device,&mut encoder,view,&self.primitive.view,&self.depth,&self.view_group,&clip_group,&self.meshes,if self.clipping.borrow().enabled{self.clipping.borrow().rows.len()as u32}else{0},Some(&self.instances));
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("cloud depth composite"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});if let Some(clouds)=self.clouds.borrow().as_ref(){clouds.draw(&mut pass);}}
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite scene ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:None,stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});
            pass.set_bind_group(2,&clip_group,&[]);
            self.highlight.ink(&mut pass,&self.view_group);
            self.mesh_ink.draw(&mut pass,&self.view_group);
            self.strokes.draw(&mut pass,&self.view_group);self.sheets.draw(&mut pass);
            self.paths.draw(&mut pass,&self.view_group);
            self.markers.draw(&mut pass,&self.view_group);
            self.highlight.points(&mut pass,&self.view_group);
            self.preview.draw(&mut pass,&self.view_group);
        }
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("measurement overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});self.measurement.draw(&mut pass,&self.view_group);self.guides.draw(&mut pass,&self.view_group);}
        if self.outlines_enabled.get()&&!self.meshes.is_empty(){
            let ctx=self.text_context();
            let mut global=self.global_outline.borrow_mut();let global=global.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));
            global.encode_masks(&ctx,&mut encoder,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,true,Some(&self.primitive.view));
            let mut selected=self.outline.borrow_mut();
            if self.meshes.iter().any(GpuMesh::selected){let selected=selected.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));selected.encode_masks(&ctx,&mut encoder,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,false,Some(&self.primitive.view));}else{selected.take();}
            if let Some(alpha)=global.alpha_view(){self.outline_composite.borrow_mut().get_or_insert_with(||crate::outline_composite::Composite::new(&self.device,self.format)).draw(&self.device,&mut encoder,view,alpha,selected.as_ref().and_then(|s|s.alpha_view()));}
        }else{
            self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();
            if self.meshes.iter().any(GpuMesh::selected){let mut outline=self.outline.borrow_mut();let outline=outline.get_or_insert_with(||crate::outline_gpu::Outline::new(&self.text_context(),self.format));outline.encode(&self.text_context(),&mut encoder,view,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision);}else{self.outline.borrow_mut().take();}
        }
        self.widget.draw(&mut encoder,view);
        self.text_error.borrow_mut().take();
        if let Some(lane)=self.text.borrow_mut().as_mut() {
            match lane.prepare(&self.text_context(),&self.text_frame(*transform)) {
            Ok(())=>{
                let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("scene glyphs"),depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {view:&self.depth,depth_ops:Some(wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view,depth_slice:None,resolve_target:None,ops:wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});if let Err(error)=lane.draw(&mut pass) {*self.text_error.borrow_mut()=Some(error.to_string());}
            }
            Err(error)=>{*self.text_error.borrow_mut()=Some(error.to_string());}
            }
        }
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.depth,&self.view_group,&clip_group,&clip.buffer,&self.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground_at(self.anchor()),[self.pyramid_revision,clip_revision],Some(&self.instances));}}
        if let Some(pyramid)=self.pyramid.borrow_mut().as_mut(){pyramid.encode(&self.device,&self.queue,&mut encoder,&self.depth,*transform,self.size,self.density,[self.pyramid_revision,clip_revision]);}
        self.queue.submit([encoder.finish()]);tiles.map_report();
    }
}

fn style(scene:&Scene,id:ObjectId,selected:bool)->crate::gpu_mesh::Style {crate::gpu_mesh::Style {selected,pickable:scene.pick_id(id.value()).is_some(),colour:scene.face_rgba(id)}}

fn rgba(rgb:[u8;3])->[f32;4] {[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]}

#[cfg(all(test,not(target_arch="wasm32")))]
impl Renderer {
 pub fn outline_alpha(&self)->Vec<u8>{self.read_outline_alpha(false)}
 pub fn global_outline_alpha(&self)->Vec<u8>{self.read_outline_alpha(true)}
 fn read_outline_alpha(&self,global:bool)->Vec<u8> {
  let owner=if global{self.global_outline.borrow()}else{self.outline.borrow()};let Some(outline)=owner.as_ref() else {return Vec::new();};let Some(texture)=outline.alpha_texture() else {return Vec::new();};let [width,height]=self.size;let pitch=width.div_ceil(256)*256;
  let buffer=self.device.create_buffer(&wgpu::BufferDescriptor {label:Some("actual outline alpha readback"),size:pitch as u64*height as u64,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});let mut encoder=self.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo {buffer:&buffer,layout:wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},wgpu::Extent3d {width,height,depth_or_array_layers:1});self.queue.submit([encoder.finish()]);let (tx,rx)=std::sync::mpsc::channel();buffer.slice(..).map_async(wgpu::MapMode::Read,move|r|tx.send(r).unwrap());self.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();let bytes=buffer.slice(..).get_mapped_range();let pixels=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize].iter().copied()).collect();drop(bytes);buffer.unmap();pixels
 }
}

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="pyramid_gpu_tests.rs"]
mod pyramid_gpu_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="ambient_gpu_tests.rs"]
mod ambient_gpu_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="global_outline_tests.rs"]
mod global_outline_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="opacity_gpu_tests.rs"]
mod opacity_gpu_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="camera_extent_tests.rs"]
mod camera_extent_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="surface_lighting_tests.rs"]
mod surface_lighting_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="surface_controls_gpu_tests.rs"]
mod surface_controls_tests;
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 15. `src/surface_lighting_tests.rs`

Retain the exact camera/display test and extend its actual viewport-prefix contract.

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzi-share-one-frame-across-color-and-ink-61.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 16. `src/shared_view_tests.rs`

Supply real mixed-original color/request/pick restoration controls.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzi-share-one-frame-across-color-and-ink-62.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 17. `src/renderer.rs`

Register optional native GPU controls outside the browser graph.

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzi-share-one-frame-across-color-and-ink-63.rs" skip=27
```

**Result after `96zzzzzzi-share-one-frame-across-color-and-ink`:** Original surfaces and ink retain their actual pixels and owners while the ink lanes stop allocating and updating duplicate frame uniforms.

## <a id="checkpoint-96zzzzzzj-fill-original-contact-polygons"></a>Step 2 of 5 · Fill original contact polygons (`96zzzzzzj-fill-original-contact-polygons`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2/5: 96zzzzzzj-fill-or…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/source_mesh_topology.rs…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original Element contact</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">bounded color/ID surface…</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">original object owner</text>
</svg>
</div>

A contact outline is an area on its original Element. Reuse the existing full-ring normal and kernel polygon triangulation, retaining mesh Parts so the existing source-ink collector supplies the complete black one-pixel perimeter. Keep contact preparation in the existing feature module; another retained boundary Polyline is unnecessary. Apply the contact marker explicitly to the derived display, because ordinary kernel conversion makes an opaque mesh. Surface records carry the original integer owner through the existing feature adapter. Color and integer vertex entries share the bounded camera-depth layer, leaving screen positions and original coordinates unchanged. Degenerate outlines retain their point/path fallback. Boundary depth, current ambient inputs and multisampling remain unfinished.

> **Check:** *Why keep Parts instead of building a second boundary Polyline?* — The prepared mesh already retains its original polygon edges independently of triangulation. Existing mesh ink can draw those edges and remap their picks to the Element, avoiding duplicate boundary coordinates, display pieces and uploads.

### 1. `src/source_mesh_topology.rs`

Reuse the existing full-ring Newell sum without per-face coordinate allocations. Read either mesh slots or original outline coordinates through one accessor. Keep every existing topology caller on the same normal algorithm.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }
}

/// Sort key of a face.
fn face_key(face: &(usize, &Vec<usize>)) -> usize {
    face.0
```

Replace it with:

```rust
    }
}

fn face_normal(vs: &[usize], vpos: &[[f64; 3]], slots: &SlotMap) -> Option<[f64; 3]> {
    polygon_normal(vs.len(), |i| vpos[slots.slot(vs[i])])
}

/// Sort key of a face.
fn face_key(face: &(usize, &Vec<usize>)) -> usize {
    face.0
```

**Locate this block:**

```rust
}

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
```

Replace it with:

```rust
}

/// Unit normal of one face, None when degenerate.
pub fn polygon_normal(count: usize, point: impl Fn(usize) -> [f64; 3]) -> Option<[f64; 3]> {
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
```

**Complete updated block reference:**

```rust
use crate::source_topology_colour::{BLACK,pack_rgba};
use session_rust::{Mesh, Tolerance};

/// Vertex key to its position in the sorted key list.
pub struct SlotMap {
    dense: Vec<u32>,                               // key indexes directly
    sparse: std::collections::HashMap<usize, u32>, // used for sparse keys
}

impl SlotMap {
    /// Build from the sorted vertex keys.
    pub fn new(keys: &[usize]) -> Self {
        let max_key = keys.last().copied().unwrap_or(0);

        // dense when keys are not too spread out
        if max_key < 4 * keys.len().max(1) {
            let mut dense = vec![u32::MAX; max_key + 1];

            for (s, &k) in keys.iter().enumerate() {
                dense[k] = s as u32;
            }

            return Self {
                dense,
                sparse: std::collections::HashMap::new(),
            };
        }

        let mut sparse = std::collections::HashMap::with_capacity(keys.len());

        for (s, &k) in keys.iter().enumerate() {
            sparse.insert(k, s as u32);
        }

        Self {
            dense: Vec::new(),
            sparse,
        }
    }

    /// The slot of key `k`.
    pub fn slot(&self, k: usize) -> usize {
        if self.dense.is_empty() {
            self.sparse[&k] as usize
        } else {
            self.dense[k] as usize
        }
    }
}

/// Edges, faces and normals of one mesh.
pub struct MeshTopo {
    pub edges: Vec<(usize, usize, u32)>, // (low key, high key, pen colour)
    pub edge_faces: Vec<[u32; 2]>,       // two faces per edge, u32::MAX = none
    pub opposed: Vec<bool>,              // per edge: faces wind opposite ways
    pub normals: Vec<Option<[f64; 3]>>,  // per face, None when degenerate
    pub closed: bool,                    // every edge has two faces
}

/// Unit normal of one face, None when degenerate.
pub fn polygon_normal(count: usize, point: impl Fn(usize) -> [f64; 3]) -> Option<[f64; 3]> {
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

/// Sort key of a face.
fn face_key(face: &(usize, &Vec<usize>)) -> usize {
    face.0
}

struct EdgeTable {
    edges: Vec<(usize, usize, u32)>,
    edge_faces: Vec<[u32; 2]>,
    dir0: Vec<u8>,
    opposed: Vec<bool>,
    head: Vec<u32>,
    next: Vec<u32>,
}

impl EdgeTable {
    fn new(slots: usize) -> Self {
        Self { edges: Vec::new(), edge_faces: Vec::new(), dir0: Vec::new(),
            opposed: Vec::new(), head: vec![u32::MAX; slots], next: Vec::new() }
    }

    fn add(&mut self, low_slot: usize, lo: usize, hi: usize, dir: u8, face: usize, cols: &[session_rust::Color]) {
        let ls = low_slot;
        let mut ei = self.head[ls];

        // look for the edge among those at `lo`
        while ei != u32::MAX && self.edges[ei as usize].1 != hi {
            ei = self.next[ei as usize];
        }

        // new edge
        if ei == u32::MAX {
            ei = self.edges.len() as u32;
            let pen = match cols.get(self.edges.len()) {
                Some(color) => pack_rgba(color.to_f32()),
                None => BLACK,
            };
            self.edges.push((lo, hi, pen));
            self.edge_faces.push([u32::MAX; 2]);
            self.dir0.push(dir);
            self.opposed.push(true);
            self.next.push(self.head[ls]);
            self.head[ls] = ei;
        }

        // record this face on the edge, first free slot
        let ef = &mut self.edge_faces[ei as usize];

        if ef[0] == u32::MAX {
            ef[0] = face as u32;
            self.dir0[ei as usize] = dir;
        } else if ef[1] == u32::MAX && ef[0] != face as u32 {
            ef[1] = face as u32;
            self.opposed[ei as usize] = dir != self.dir0[ei as usize];
        }
    }
}

/// Collect unique edges, their faces and the face normals.
pub fn mesh_topology(m: &Mesh, keys: &[usize], vpos: &[[f64; 3]], slots: &SlotMap) -> MeshTopo {
    let mut faces: Vec<(usize, &Vec<usize>)> = Vec::with_capacity(m.face.len());

    for (k, v) in m.face.iter() {
        faces.push((*k, v));
    }

    faces.sort_unstable_by_key(face_key);
    let cols = m.get_linecolors();

    let normals: Vec<_> = faces
        .iter()
        .map(|(_, vs)| face_normal(vs, vpos, slots))
        .collect();
    let mut table = EdgeTable::new(keys.len());

    // Keep the kernel's outer-edge ordering for colors, widths and picking ids.
    for holes in [false, true] {
        for (fs, (key, vs)) in faces.iter().enumerate() {
            let rings: &[Vec<usize>] = if holes {
                m.face_holes.get(key).map_or(&[], Vec::as_slice)
            } else {
                std::slice::from_ref(*vs)
            };
            for ring in rings {
                // Hole rings may be stored in either direction; a face walks them opposite its border.
                let reverse = holes
                    && normals[fs]
                        .zip(face_normal(ring, vpos, slots))
                        .is_some_and(|(a, b)| {
                            a.iter().zip(b).map(|(x, y)| x * y).sum::<f64>() > 0.0
                        });
                let n = ring.len();

                for i in 0..n {
                    let (u, v) = (ring[i], ring[(i + 1) % n]);
                    let (lo, hi, dir) = if u < v { (u, v, 0) } else { (v, u, 1) }; // edge key
                    let dir = dir ^ u8::from(reverse);
                    table.add(slots.slot(lo), lo, hi, dir, fs, cols);
                }
            }
        }
    }

    // closed when no edge is missing its second face
    let mut closed = !m.vertex.is_empty();

    for f in table.edge_faces.iter() {
        if f[1] == u32::MAX {
            closed = false;
        }
    }

    MeshTopo {
        edges: table.edges,
        edge_faces: table.edge_faces,
        opposed: table.opposed,
        normals,
        closed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::{Point, Polyline};

    fn topology(mesh: &Mesh) -> MeshTopo {
        let keys = mesh.vertices();
        let positions: Vec<_> = keys
            .iter()
            .map(|key| {
                let p = &mesh.vertex[key];
                [p.x, p.y, p.z]
            })
            .collect();
        mesh_topology(mesh, &keys, &positions, &SlotMap::new(&keys))
    }

    #[test]
    fn hole_rims_carry_both_faces_without_reordering_source_edges() {
        let loops = |z| {
            [10.0, 3.0].map(|r| {
                Polyline::new(vec![
                    Point::new(-r, -r, z),
                    Point::new(r, -r, z),
                    Point::new(r, r, z),
                    Point::new(-r, r, z),
                    Point::new(-r, -r, z),
                ])
            })
        };
        let mesh = Mesh::loft(&loops(0.0), &loops(2.0), true, false);
        assert_eq!(mesh.face_holes.len(), 2);
        let topo = topology(&mesh);
        assert!(topo.closed);
        assert_eq!(topo.edges.len(), 24);
        assert!(
            topo.edge_faces
                .iter()
                .all(|faces| !faces.contains(&u32::MAX))
        );
        assert!(topo.opposed.iter().all(|opposed| *opposed));
        let original = mesh.edges_with_colors();
        for (i, (a, b, _)) in original.iter().enumerate() {
            assert_eq!((topo.edges[i].0, topo.edges[i].1), (*a, *b));
        }
        for (face, holes) in &mesh.face_holes {
            let face_slot = mesh.faces().iter().position(|key| key == face).unwrap() as u32;
            for ring in holes {
                for i in 0..ring.len() {
                    let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
                    let edge = topo
                        .edges
                        .iter()
                        .position(|(u, v, _)| (*u, *v) == (a.min(b), a.max(b)))
                        .unwrap();
                    assert!(topo.edge_faces[edge].contains(&face_slot));
                }
            }
        }
    }

    #[test]
    fn open_face_draws_its_hole_boundary() {
        let points = vec![
            Point::new(-10.0, -10.0, 0.0),
            Point::new(10.0, -10.0, 0.0),
            Point::new(10.0, 10.0, 0.0),
            Point::new(-10.0, 10.0, 0.0),
            Point::new(-3.0, -3.0, 0.0),
            Point::new(3.0, -3.0, 0.0),
            Point::new(3.0, 3.0, 0.0),
            Point::new(-3.0, 3.0, 0.0),
        ];
        let mut mesh = Mesh::from_vertices_and_faces(points, vec![vec![0, 1, 2, 3]]);
        mesh.set_face_holes(0, vec![vec![4, 5, 6, 7]]);
        let topo = topology(&mesh);
        assert_eq!(topo.edges.len(), 8);
        assert!(!topo.closed);
    }
}
```

### 2. `src/feature_display.rs`

Try actual area before the existing float-collapsed point/path fallback.

Edit the existing block to match this code; keep unchanged lines:

```rust
                    if feature.feature_type=="contact" && let Some(mut fill)=contact_fill(outline)? {
                        fill.model=model.clone();let id=scene.insert(fill)?;
                        outlines.push(FeatureTree::Piece(format!("piece {}",part+1),id));continue;
                    }
                    let point = outline.get_point(0).filter(|p| outline.coords.chunks_exact(3)
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/contact_feature_tests.rs`

Supply actual Element-source, concave perimeter, original-owner GPU and depth/occlusion controls.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzj-fill-original-contact-polygons-14.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/renderer.rs`

Keep optional native GPU validation outside the browser implementation.

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzj-fill-original-contact-polygons-15.rs" skip=22
```

### 5. `src/gpu_mesh.rs`

Carry the original integer owner, including zero for an unpickable display. Write the resolved owner into the existing object record. Keep standalone uploaded originals on their own checked identity.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }
    pub fn upload_at(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,object:&Object,selected:bool,anchor:[f64;3])->Self {
        let geometry=Rc::new(GpuGeometry::upload(device,Rc::clone(&object.mesh)));
        Self::with_geometry_at(device,layout,object,Style{selected,pickable:!object.metadata.is_locked,colour:None},geometry,anchor)
    }

    pub fn with_geometry(device: &wgpu::Device, layout: &wgpu::BindGroupLayout,
```

Replace it with:

```rust
    }
    pub fn upload_at(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,object:&Object,selected:bool,anchor:[f64;3])->Self {
        let geometry=Rc::new(GpuGeometry::upload(device,Rc::clone(&object.mesh)));
        Self::with_geometry_at(device,layout,object,Style{selected,owner:if object.metadata.is_locked {0}else{object.id.value()},colour:None},geometry,anchor)
    }

    pub fn with_geometry(device: &wgpu::Device, layout: &wgpu::BindGroupLayout,
```

**Locate this block:**

```rust
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&u32::from(style.selected).to_ne_bytes());
    let id=if style.pickable {object.id.value()} else {0};
    bytes[68..72].copy_from_slice(&id.to_ne_bytes());
    let flags=if matches!(&object.original,Some(session_rust::Geometry::Plane(p)) if crate::clipping_geometry::is_clipping(p)){4096u32}else{0};bytes[72..76].copy_from_slice(&(flags|cap_flags).to_ne_bytes());bytes[76..80].copy_from_slice(&cap_token.to_ne_bytes());
    let normals = crate::normals::matrix(&object.model).expect("Scene placements are checked before insertion or editing");
```

Replace it with:

```rust
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&u32::from(style.selected).to_ne_bytes());
    let id=style.owner;
    bytes[68..72].copy_from_slice(&id.to_ne_bytes());
    let flags=if matches!(&object.original,Some(session_rust::Geometry::Plane(p)) if crate::clipping_geometry::is_clipping(p)){4096u32}else{0};bytes[72..76].copy_from_slice(&(flags|cap_flags).to_ne_bytes());bytes[76..80].copy_from_slice(&cap_token.to_ne_bytes());
    let normals = crate::normals::matrix(&object.model).expect("Scene placements are checked before insertion or editing");
```

**Locate this block:**

```rust
}

#[derive(Clone,Copy)]
pub struct Style {pub selected:bool,pub pickable:bool,pub colour:Option<[f32;4]>}

fn settings(object: &Object, style:Style,cap_flags:u32,cap_token:u32,primitive_base:u32,index_start:u32,anchor:[f64;3]) -> Record {
    let mut bytes = [0; BYTES];
```

Replace it with:

```rust
}

#[derive(Clone,Copy)]
pub struct Style {pub selected:bool,pub owner:u32,pub colour:Option<[f32;4]>}

fn settings(object: &Object, style:Style,cap_flags:u32,cap_token:u32,primitive_base:u32,index_start:u32,anchor:[f64;3]) -> Record {
    let mut bytes = [0; BYTES];
```

**Complete updated block reference:**

```rust
use crate::{gpu_geometry::GpuGeometry, scene::Object};
use std::rc::Rc;
use crate::object_record::{BYTES, Record, RANGE};
use wgpu::util::DeviceExt;

pub struct GpuMesh {
    pub geometry: Rc<GpuGeometry>,
    model_group: wgpu::BindGroup,
    pub id: crate::scene::ObjectId,
    model: wgpu::Buffer,
    translation:[f64;3],anchor:[f64;3],
    previous: Record,primitive_base:u32,xray:bool,cap_flags:u32,cap_token:u32,
}

#[derive(Clone,Copy)]
pub struct Style {pub selected:bool,pub owner:u32,pub colour:Option<[f32;4]>}

fn settings(object: &Object, style:Style,cap_flags:u32,cap_token:u32,primitive_base:u32,index_start:u32,anchor:[f64;3]) -> Record {
    let mut bytes = [0; BYTES];
    let model=crate::render_space::placed(&object.model,anchor).to_f32();
    for (index, value) in model.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&u32::from(style.selected).to_ne_bytes());
    let id=style.owner;
    bytes[68..72].copy_from_slice(&id.to_ne_bytes());
    let flags=if matches!(&object.original,Some(session_rust::Geometry::Plane(p)) if crate::clipping_geometry::is_clipping(p)){4096u32}else{0};bytes[72..76].copy_from_slice(&(flags|cap_flags).to_ne_bytes());bytes[76..80].copy_from_slice(&cap_token.to_ne_bytes());
    let normals = crate::normals::matrix(&object.model).expect("Scene placements are checked before insertion or editing");
    for (index, value) in normals.iter().enumerate() {
        bytes[80 + index * 4..84 + index * 4].copy_from_slice(&value.to_ne_bytes());
    }
    if let Some(colour)=style.colour {for (index,value) in colour[..3].iter().copied().chain([1.]).enumerate() {bytes[144+index*4..148+index*4].copy_from_slice(&value.to_ne_bytes());}}
    bytes[RANGE..RANGE+4].copy_from_slice(&index_start.to_ne_bytes());
    bytes[RANGE+4..RANGE+8].copy_from_slice(&primitive_base.to_ne_bytes());
    bytes
}

impl GpuMesh {
    pub fn upload(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, object: &Object, selected: bool) -> Self {
        Self::upload_at(device,layout,object,selected,[0.;3])
    }
    pub fn upload_at(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,object:&Object,selected:bool,anchor:[f64;3])->Self {
        let geometry=Rc::new(GpuGeometry::upload(device,Rc::clone(&object.mesh)));
        Self::with_geometry_at(device,layout,object,Style{selected,owner:if object.metadata.is_locked {0}else{object.id.value()},colour:None},geometry,anchor)
    }

    pub fn with_geometry(device: &wgpu::Device, layout: &wgpu::BindGroupLayout,
        object: &Object, style:Style, geometry: Rc<GpuGeometry>) -> Self
    {
        Self::with_geometry_at(device,layout,object,style,geometry,[0.;3])
    }
    pub fn with_geometry_at(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,object:&Object,style:Style,geometry:Rc<GpuGeometry>,anchor:[f64;3])->Self {
        let bytes=settings(object,style,0,0,0,geometry.index_start,anchor);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let model_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("object placement"),
            layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: model.as_entire_binding() }],
        });
        Self { translation:object.model.m[12..15].try_into().unwrap(),anchor,geometry, model_group, id: object.id, model, previous: bytes,primitive_base:0,xray:false,cap_flags:0,cap_token:0 }
    }

    pub fn rebase(&mut self,queue:&wgpu::Queue,anchor:[f64;3])->[usize;2] {
        assert!(anchor.iter().all(|v|v.is_finite()));self.anchor=anchor;
        let bytes=crate::render_space::relative(self.translation,anchor).into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();
        if self.previous[48..60]==bytes {return [0,0];}
        self.previous[48..60].copy_from_slice(&bytes);queue.write_buffer(&self.model,48,&bytes);[1,12]
    }
    pub(crate) fn model_buffer(&self)->&wgpu::Buffer{&self.model}
    pub(crate) fn packed(&self)->Record {self.previous}
    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())&1!=0}

    pub fn set_ambient_radius(&mut self,queue:&wgpu::Queue,radius:f32){let bytes=radius.to_ne_bytes();if self.previous[RANGE+8..RANGE+12]!=bytes {self.previous[RANGE+8..RANGE+12].copy_from_slice(&bytes);queue.write_buffer(&self.model,(RANGE+8)as u64,&bytes);}}
    pub fn primitive_base(&self)->u32{self.primitive_base}
    pub fn set_primitive_base(&mut self,queue:&wgpu::Queue,base:u32)->Result<(),&'static str>{
        if base.checked_add(self.geometry.index_count/3).is_none_or(|end|end>=crate::primitive_image::CAP_START){return Err("Primitive range overlaps reserved caps");}
        if self.primitive_base!=base{
            self.previous[RANGE+4..RANGE+8].copy_from_slice(&base.to_ne_bytes());
            queue.write_buffer(&self.model,(RANGE+4)as u64,&self.previous[RANGE+4..RANGE+8]);self.primitive_base=base;
        }
        Ok(())
    }

    pub fn set_xray(&mut self,queue:&wgpu::Queue,on:bool){if self.xray==on{return;}self.xray=on;let flags=u32::from_ne_bytes(self.previous[72..76].try_into().unwrap());let flags=(flags&!32768)|if on&&flags&8192!=0{32768}else{0};self.previous[72..76].copy_from_slice(&flags.to_ne_bytes());queue.write_buffer(&self.model,72,&self.previous[72..76]);}
    pub fn closed(&self)->bool{!self.xray&&self.cap_flags&crate::solid_orientation::CLOSED!=0}
    pub fn pickable(&self)->bool{u32::from_ne_bytes(self.previous[68..72].try_into().unwrap())!=0}
    pub fn set_caps(&mut self,queue:&wgpu::Queue,flags:u32,token:u32){
        if self.cap_flags==flags&&self.cap_token==token{return;}let plane=u32::from_ne_bytes(self.previous[72..76].try_into().unwrap())&4096;self.previous[72..76].copy_from_slice(&(plane|flags|if self.xray&&flags&8192!=0{32768}else{0}).to_ne_bytes());self.previous[76..80].copy_from_slice(&token.to_ne_bytes());queue.write_buffer(&self.model,72,&self.previous[72..80]);self.cap_flags=flags;self.cap_token=token;
    }
    pub fn settings_bytes(&self) -> u64 { self.model.size() }

    pub fn update(&mut self, queue: &wgpu::Queue, object: &Object, style:Style) -> [usize; 2] {
        self.translation=object.model.m[12..15].try_into().unwrap();
        let mut next = settings(object, style,self.cap_flags|if self.xray&&self.cap_flags&8192!=0{32768}else{0},self.cap_token,self.primitive_base,self.geometry.index_start,self.anchor);
        next[RANGE+8..RANGE+12].copy_from_slice(&self.previous[RANGE+8..RANGE+12]);
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144), (144,RANGE),(RANGE,BYTES)] {
            if next[start..end] != self.previous[start..end] {
                queue.write_buffer(&self.model, start as u64, &next[start..end]);
                writes[0] += 1;
                writes[1] += end - start;
            }
        }
        self.previous = next;
        writes
    }

    pub fn bind_model(&self, pass: &mut wgpu::RenderPass<'_>) { pass.set_bind_group(1, &self.model_group, &[]); }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.bind_model(pass);
        self.geometry.draw_pulled(pass, 1);
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_bind_group(1, &self.model_group, &[]);
        self.geometry.draw(pass);
    }
}
```

### 6. `src/renderer.rs`

Use the existing feature adapter for color and request-specific surface records.

Edit the existing block to match this code; keep unchanged lines:

```rust
fn style(scene:&Scene,id:ObjectId,selected:bool)->crate::gpu_mesh::Style {crate::gpu_mesh::Style {selected,owner:crate::feature_owner::display_owner(scene,id).map_or(0,ObjectId::value),colour:scene.face_rgba(id)}}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/global_outline_tests.rs`

Retain every exact original record/pixel/anchor assertion with the explicit owner field.

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzj-fill-original-contact-polygons-owner-5.rs"
```

### 8. `src/contact_depth.wgsl`

Pull the contact toward the eye by the current bounded depth/slope layer, leaving screen position and source coordinates fixed.

Create the file and type:

```wgsl
--8<-- "journey/code/96zzzzzzj-fill-original-contact-polygons-09.rs"
```

### 9. `src/clipping_gpu.rs`

Expand the shared helper only in shaders which explicitly include it.

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn source(base:&str,group:u32,binding:u32)->String{let base=base.replace(r#"#include "contact_depth.wgsl""#,include_str!("contact_depth.wgsl"));format!("{}\n{}\n{}\n@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",crate::colour::selection_shader(),crate::draw_slots::sentinel_shader(),crate::object_record::shader(),concat!(include_str!("packed_vertex.wgsl"),"\n",include_str!("physical.wgsl"),"\n",include_str!("clip.wgsl")))}
```

### 10. `src/triangle.wgsl`

Share contact placement between ordinary, pulled and integer surface entries. Use the actual original-facing plane and full physical frame for the layer.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```wgsl
    output.world = world.xyz;
    output.clip_flags=select(object.selection.z,(object.selection.z&~(8192u|32768u))|CONTACT_FLAG,contact);
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
}
```

Replace it with:

```wgsl
    output.world = world.xyz;
    output.clip_flags=select(object.selection.z,(object.selection.z&~(8192u|32768u))|CONTACT_FLAG,contact);
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    if(contact) {output.position=contact_layer(output.position,world.xyz,output.normal,view.transform,view.viewport.xy,view.eye);}
    return output;
}
```

**Locate this block:**

```wgsl
    @location(5) @interpolate(flat) primitive:u32,
}

const CONTACT_FLAG:u32=65536u;
fn geometry(position: vec3<f32>, colour: vec4<f32>, normal: vec3<f32>, object: ObjectSettings) -> VertexOutput {
    var output: VertexOutput;
```

Replace it with:

```wgsl
    @location(5) @interpolate(flat) primitive:u32,
}

#include "contact_depth.wgsl"
const CONTACT_FLAG:u32=65536u;
fn geometry(position: vec3<f32>, colour: vec4<f32>, normal: vec3<f32>, object: ObjectSettings) -> VertexOutput {
    var output: VertexOutput;
```

**Complete updated block reference:**

```wgsl
struct ViewSettings { transform: mat4x4<f32>, viewport:vec4<f32>, display: vec4<f32>, eye:vec4<f32>, }
@group(0) @binding(0) var<uniform> view: ViewSettings;
@group(1) @binding(0) var<uniform> object_settings: ObjectSettings;
fn object_at(index: u32) -> ObjectSettings { return object_settings; }

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
    @location(1) world: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) @interpolate(flat) clip_flags:u32,
    @location(4) @interpolate(flat) mirrored:u32,
    @location(5) @interpolate(flat) primitive:u32,
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
}

fn shade(input: VertexOutput, front: bool) -> vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let derivative = cross(dpdx(input.world), dpdy(input.world));
    // Screen derivatives ignore vertex order; front-facing restores the original face orientation for reversed winding and world-normal inspection.
    let face = select(derivative, -derivative, front);
    if(clip_active()&&clip_cut(input.clip_flags,input.world)){discard;}
    let contact=(input.clip_flags&CONTACT_FLAG)!=0u;
    let closed=(input.clip_flags&8192u)!=0u;
    let source_front=(front!=(input.mirrored!=0u))!=((input.clip_flags&16384u)!=0u);
    if((input.clip_flags&32768u)!=0u || (closed&&!source_front&&view.display.y>0.&&view.display.y<1.)){discard;}
    if(contact){return vec4<f32>(input.colour,1.);}
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    if view.display.x > 0.5 { return vec4<f32>(normal * 0.5 + vec3<f32>(0.5), select(1.,view.display.y,closed)); }
    let m=view.transform;
    let v=select(normalize(view.eye.xyz-input.world),normalize(vec3<f32>(m[0].z,m[1].z,m[2].z)),view.eye.w>0.);
    let n=select(normal,-normal,dot(normal,v)<0.);
    let l=normalize(v+vec3<f32>(0.,0.,0.35));let h=normalize(l+v);
    let wrap=clamp((dot(n,l)+0.5)/1.5,0.,1.);
    let lit=min(0.40+0.55*wrap+pow(max(dot(n,h),0.),32.)*0.15,1.);
    let sky=mix(0.72,1.,0.5+0.5*n.z);
    let brightness=select(1.,select(lit,sky,view.display.z>1.5),view.display.z>0.5);
    let back=!source_front&&view.display.w>0.5;
    let base=select(input.colour,vec3<f32>(0.80,0.05,0.05),back);
    return vec4<f32>(base*brightness,select(1.,view.display.y,closed));
}

struct FragmentOutput{
    @location(0) color:vec4<f32>,
    @location(1) primitive:vec2<u32>,
}
@fragment
fn fragment(input:VertexOutput,@builtin(front_facing) front:bool)->FragmentOutput{
    return FragmentOutput(shade(input,front),physical_primitive(input.primitive));
}

@group(3) @binding(0) var<storage, read> mesh_values: array<PackedVertex>;
@group(3) @binding(2) var<storage, read> mesh_indices: array<u32>;
struct MeshInput { position: vec3<f32>, colour: vec4<f32>, normal: vec3<f32>, }
fn mesh_vertex(occurrence:u32,first:u32)->MeshInput{
 let vertex=mesh_values[mesh_indices[occurrence+first]];
 return MeshInput(packed_position(vertex),unpack4x8unorm(vertex.color),oct32_decode(vertex.normal));
}

@vertex
fn vertex(@builtin(vertex_index) occurrence: u32, @location(0) row:u32,@location(1) slot_base:u32) -> VertexOutput {
    let object=object_at(row);
    let primitive_base=select(slot_base,object.range.y,row==OWN_ROW);
    let input=mesh_vertex(occurrence,object.range.x);
    var output=geometry(input.position,input.colour,input.normal,object);
    output.primitive=primitive_base+occurrence/3u+1u;
    return output;
}
```

### 11. `src/face_pick.wgsl`

Supply the actual normal to the same source-face vertex function.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
 let placed=geometry(packed_position(vertex),unpack4x8unorm(vertex.color),oct32_decode(vertex.normal),object);
```

### 12. `src/feature_display.rs`

Keep contact display preparation in its existing feature module. Use the original kernel types already owned by feature preparation. Prepare one contact mesh, retaining its full normal, display marker and existing polygon Parts.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    Rc::make_mut(&mut scene.view).current = current;
    crate::layer_edit::attach_features(scene, id, prepared?)
}
```

Replace it with:

```rust
    Rc::make_mut(&mut scene.view).current = current;
    crate::layer_edit::attach_features(scene, id, prepared?)
}

pub(crate) fn contact_fill(outline:&Polyline)->Result<Option<PreparedMesh>,&'static str> {
  if outline.coords.len()%3!=0 {return Err("Contact needs complete points");}
  let Some(normal)=crate::source_mesh_topology::polygon_normal(outline.coords.len()/3,|i| {
    let p=&outline.coords[i*3..];[p[0],p[1],p[2]]
  }) else {return Ok(None)};
  let points=outline.coords.chunks_exact(3).map(|p|Point::new(p[0],p[1],p[2])).collect();
  let mut mesh=Mesh::from_polylines(vec![points],None);
  mesh.set_objectcolor(Color::new(0.9,0.1,0.1,1.));
  let mut fill=PreparedMesh::new(mesh)?;let count=fill.display.vertices().len();
  fill.display=fill.display.with_normals(vec![normal.map(|v|v as f32);count])?.with_alphas(vec![1./255.;count])?;
  Ok(Some(fill))
}
```

**Locate this block:**

```rust
use crate::{scene::{Scene, ObjectId}, layer_edit::FeatureTree};
use session_rust::{Element, Session};
use std::rc::Rc;

pub fn append(scene: &mut Scene, id: ObjectId, session: &Session, element: &Element) -> Result<(), &'static str> {
```

Replace it with:

```rust
use crate::{scene::{Scene, ObjectId}, layer_edit::FeatureTree};
use crate::prepared::PreparedMesh;
use session_rust::{Element, Session};
use session_rust::{Color, Mesh, Point, Polyline};
use std::rc::Rc;

pub fn append(scene: &mut Scene, id: ObjectId, session: &Session, element: &Element) -> Result<(), &'static str> {
```

**Complete updated block reference:**

```rust
use crate::{scene::{Scene, ObjectId}, layer_edit::FeatureTree};
use crate::prepared::PreparedMesh;
use session_rust::{Element, Session};
use session_rust::{Color, Mesh, Point, Polyline};
use std::rc::Rc;

pub fn append(scene: &mut Scene, id: ObjectId, session: &Session, element: &Element) -> Result<(), &'static str> {
    let model = scene.placement(id).ok_or("Element placement is missing")?;
    let forest = std::mem::take(&mut scene.forest);
    let created = scene.created.take();
    let current = Rc::make_mut(&mut scene.view).current.take();
    let prepared = (|| {
        let mut groups = Vec::new();
        let mut pieces = |indices: &[usize]| -> Result<Vec<FeatureTree>, &'static str> {
            let mut rows = Vec::new();
            for &index in indices {
                let feature = &element.features[index];
                if !feature.visible { continue; }
                let mut outlines = Vec::new();
                for (part, outline) in feature.outlines.iter().enumerate() {
                    if feature.feature_type=="contact" && let Some(mut fill)=contact_fill(outline)? {
                        fill.model=model.clone();let id=scene.insert(fill)?;
                        outlines.push(FeatureTree::Piece(format!("piece {}",part+1),id));continue;
                    }
                    let point = outline.get_point(0).filter(|p| outline.coords.chunks_exact(3)
                        .all(|q| [q[0] as f32, q[1] as f32, q[2] as f32] == p.to_f32()));
                    let piece = if let Some(mut point) = point {
                        point.set_guid(uuid::Uuid::new_v4().to_string());
                        point.name = format!("piece {}", part + 1);
                        point.pointcolor = outline.linecolor.clone();
                        point.width = 12.0;
                        point.is_visible = true;
                        point.is_locked = false;
                        let piece = scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(point))?)?;
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
                        let piece = scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(source))?)?;
                        scene.place_following(piece, model.clone())?;
                        piece
                    };
                    if !crate::feature_plan::interaction(feature) {
                        Rc::make_mut(&mut scene.view).hide(&[piece], true);
                    }
                    outlines.push(FeatureTree::Piece(format!("piece {}", part + 1), piece));
                }
                if !outlines.is_empty() {
                    rows.push(FeatureTree::Group(crate::feature_plan::label(feature), outlines));
                }
            }
            Ok(rows)
        };
        let own = pieces(&crate::feature_plan::own(element))?;
        if !own.is_empty() { groups.push(FeatureTree::Group("attributes".into(), own)); }
        let mut hosted = Vec::new();
        for relation in crate::feature_plan::hosted(session, element) {
            let children = pieces(&relation.features)?;
            if !children.is_empty() { hosted.push(FeatureTree::Group(relation.label, children)); }
        }
        if !hosted.is_empty() { groups.push(FeatureTree::Group("interactions".into(), hosted)); }
        Ok(groups)
    })();
    scene.forest = forest;
    scene.created = created;
    Rc::make_mut(&mut scene.view).current = current;
    crate::layer_edit::attach_features(scene, id, prepared?)
}

pub(crate) fn contact_fill(outline:&Polyline)->Result<Option<PreparedMesh>,&'static str> {
  if outline.coords.len()%3!=0 {return Err("Contact needs complete points");}
  let Some(normal)=crate::source_mesh_topology::polygon_normal(outline.coords.len()/3,|i| {
    let p=&outline.coords[i*3..];[p[0],p[1],p[2]]
  }) else {return Ok(None)};
  let points=outline.coords.chunks_exact(3).map(|p|Point::new(p[0],p[1],p[2])).collect();
  let mut mesh=Mesh::from_polylines(vec![points],None);
  mesh.set_objectcolor(Color::new(0.9,0.1,0.1,1.));
  let mut fill=PreparedMesh::new(mesh)?;let count=fill.display.vertices().len();
  fill.display=fill.display.with_normals(vec![normal.map(|v|v as f32);count])?.with_alphas(vec![1./255.;count])?;
  Ok(Some(fill))
}
```

**Result after `96zzzzzzj-fill-original-contact-polygons`:** The original concave contact has an opaque unlit red fill, its complete existing perimeter, an open notch and the original Element object owner. Coplanar host depth stays behind it and real nearer geometry still occludes it.

## <a id="checkpoint-96zzzzzzk-broadcast-ordinary-ink-metadata"></a>Step 3 of 5 · Broadcast ordinary ink metadata (`96zzzzzzk-broadcast-ordinary-ink-metadata`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3/5: 96zzzzzzk-broadca…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/stroke_gpu.rs, src/fini…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">One existing ink lane</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">zero or four-byte vertex…</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">unchanged color and GPU I…</text>
</svg>
</div>

Ordinary ink repeats a zero clipping flag for every instance. Choose metadata addressing when creating the lane: broadcast one record for lines, paths and markers; keep indexed records for mesh borders and mixed highlights. Color, picking and source geometry stay unchanged.

Every ordinary line, path segment and marker currently stores its own zero clipping flag on both CPU and GPU. The value is constant across those batches. Choose the metadata mode once when enabling a lane: zero vertex stride broadcasts one record to every instance; mesh borders and mixed highlighted strokes retain indexed records. Keep color and cropped integer layouts identical, strict count validation and empty-scene release. This reduces metadata without another pipeline per mode, new scene owner or copied source geometry. The next contact-boundary correction will extend this same explicit record only where the visual consumer needs it.

> **Check:** *Why keep indexed metadata for mesh borders and highlighted strokes?* — A batch can contain clipping-plane borders, ordinary mesh borders and highlighted originals with different flags. Those records vary by instance. Ordinary paths and markers share a constant zero flag and can broadcast one record.

### 1. `src/stroke_gpu.rs`

Keep one metadata mode on the existing lane. Reset only the records actually addressed by the pipeline. Allocate one default flag for a nonempty constant lane. Validate constant and indexed metadata counts. Choose a mode when creating the existing finite pipelines. Keep the selected stride in color and integer pipelines. Initialize the same bounded default. Start with the ordinary constant mode.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(),
            depth_stencil: Some(crate::depth::ink()), multisample: Default::default(), multiview_mask: None, cache: None,
        }));
        Self {flags:None,flag_data:Vec::new(),finite:None,strict_depth, pipeline, pick_pipeline, ids: None, id_data: Vec::new(),  vertices: None, data: Vec::new(), stride, vertices_per_instance, uploads: 0, uploaded_bytes: 0 }
    }

    fn layout_depth(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
```

Replace it with:

```rust
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(),
            depth_stencil: Some(crate::depth::ink()), multisample: Default::default(), multiview_mask: None, cache: None,
        }));
        Self {flags:None,flag_data:Vec::new(),varying:false,finite:None,strict_depth, pipeline, pick_pipeline, ids: None, id_data: Vec::new(),  vertices: None, data: Vec::new(), stride, vertices_per_instance, uploads: 0, uploaded_bytes: 0 }
    }

    fn layout_depth(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
```

**Locate this block:**

```rust
                label: Some("stroke instances"), contents: &data, usage: wgpu::BufferUsages::VERTEX,
            }))
        };
        self.data = data; if self.finite.is_some(){self.set_flags(device,vec![0;self.data.len()/self.stride as usize])?;} Ok(())
    }

    pub fn set_flags(&mut self,device:&wgpu::Device,flags:Vec<u32>)->Result<(),&'static str>{
        if flags.len()!=self.data.len()/self.stride as usize{return Err("Clipping flags must match original ink instances")}
        if flags==self.flag_data{return Ok(())}use wgpu::util::DeviceExt;self.flags=if flags.is_empty(){None}else{let bytes=flags.iter().copied().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("original ink clipping flags"),contents:&bytes,usage:wgpu::BufferUsages::VERTEX}))};self.flag_data=flags;Ok(())
    }
    pub fn clipping_bytes(&self)->[u64;2]{[self.flags.as_ref().map_or(0,wgpu::Buffer::size),self.flag_data.capacity()as u64*4]}
    pub fn enable_finite(&mut self,device:&wgpu::Device,format:wgpu::TextureFormat){self.finite=Some(crate::finite_ink::Ink::new(device,format,self.stride,self.strict_depth));self.set_flags(device,vec![0;self.data.len()/self.stride as usize]).expect("Original ink clipping flags");}
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){if let Some(ink)=&self.finite{ink.set(device,tables,depth)}}
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup) {
        let Some(vertices) = &self.vertices else { return; };
```

Replace it with:

```rust
                label: Some("stroke instances"), contents: &data, usage: wgpu::BufferUsages::VERTEX,
            }))
        };
        self.data = data; if self.finite.is_some(){self.reset_flags(device)?;} Ok(())
    }

    fn reset_flags(&mut self,device:&wgpu::Device)->Result<(),&'static str>{
        let count=self.data.len()/self.stride as usize;
        self.set_flags(device,vec![0;if self.varying{count}else{count.min(1)}])
    }
    pub fn set_flags(&mut self,device:&wgpu::Device,flags:Vec<u32>)->Result<(),&'static str>{
        let count=self.data.len()/self.stride as usize;
        if flags.len()!=if self.varying{count}else{count.min(1)}{return Err("Clipping flags must match original ink instances")}
        if flags==self.flag_data{return Ok(())}use wgpu::util::DeviceExt;self.flags=if flags.is_empty(){None}else{let bytes=flags.iter().copied().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("original ink clipping flags"),contents:&bytes,usage:wgpu::BufferUsages::VERTEX}))};self.flag_data=flags;Ok(())
    }
    pub fn clipping_bytes(&self)->[u64;2]{[self.flags.as_ref().map_or(0,wgpu::Buffer::size),self.flag_data.capacity()as u64*4]}
    pub fn enable_finite(&mut self,device:&wgpu::Device,format:wgpu::TextureFormat,varying:bool){self.varying=varying;self.finite=Some(crate::finite_ink::Ink::new(device,format,self.stride,self.strict_depth,varying));self.reset_flags(device).expect("Original ink clipping flags");}
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){if let Some(ink)=&self.finite{ink.set(device,tables,depth)}}
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup) {
        let Some(vertices) = &self.vertices else { return; };
```

**Locate this block:**

```rust
pub struct Lane {
    flags:Option<wgpu::Buffer>,flag_data:Vec<u32>,
    finite:Option<crate::finite_ink::Ink>,
    strict_depth:bool,
    pipeline: wgpu::RenderPipeline,
```

Replace it with:

```rust
pub struct Lane {
    flags:Option<wgpu::Buffer>,flag_data:Vec<u32>,varying:bool,
    finite:Option<crate::finite_ink::Ink>,
    strict_depth:bool,
    pipeline: wgpu::RenderPipeline,
```

**Complete updated block reference:**

```rust
pub struct Lane {
    flags:Option<wgpu::Buffer>,flag_data:Vec<u32>,varying:bool,
    finite:Option<crate::finite_ink::Ink>,
    strict_depth:bool,
    pipeline: wgpu::RenderPipeline,
    pick_pipeline: Option<wgpu::RenderPipeline>,
    ids: Option<wgpu::Buffer>,
    id_data: Vec<u8>,
    vertices: Option<wgpu::Buffer>,
    data: Vec<u8>,
    stride: u64,
    vertices_per_instance: u32,
    pub uploads: u64,
    pub uploaded_bytes: u64,
}

impl Lane {
    pub fn set(&mut self, device: &wgpu::Device, strokes: impl IntoIterator<Item = crate::stroke::Stroke>) -> Result<(), &'static str> {
        let mut data = Vec::new();
        for stroke in strokes {
            if stroke.width <= 0.0 || stroke.values().iter().any(|value| !value.is_finite()) {
                return Err("Invalid stroke display data");
            }
            data.extend(stroke.bytes());
        }
        if self.stride != 44 { return Err("Stroke data needs the stroke layout"); }
        self.set_bytes(device, data)
    }

    pub fn set_guides(&mut self,device:&wgpu::Device,segments:&[crate::guide_gpu::Segment])->Result<(),&'static str>{if self.stride!=48{return Err("Guide data needs the guide layout")}let mut bytes=Vec::with_capacity(segments.len()*48);for segment in segments{if segment.stroke.width<=0.||segment.stroke.values().iter().any(|v|!v.is_finite()){return Err("Invalid guide stroke")}bytes.extend(segment.stroke.bytes());bytes.extend(u32::from(segment.dashed).to_ne_bytes());}self.set_bytes(device,bytes)}
    pub fn set_chains(&mut self, device: &wgpu::Device, segments: impl IntoIterator<Item = crate::chain::Segment>) -> Result<(), &'static str> {
        if self.stride != 72 { return Err("Connected data needs the connected layout"); }
        let mut data = Vec::new();
        for segment in segments {
            if segment.stroke.width <= 0.0 || segment.heads > 3
                || segment.stroke.values().iter().chain(segment.previous.iter()).chain(segment.next.iter()).any(|v| !v.is_finite()) {
                return Err("Invalid connected display data");
            }
            data.extend(segment.bytes());
        }
        self.set_bytes(device, data)
    }

    pub fn set_points(&mut self, device: &wgpu::Device, markers: impl IntoIterator<Item = crate::marker::Marker>) -> Result<(), &'static str> {
        if self.stride != 32 { return Err("Marker data needs the marker layout"); }
        let mut data = Vec::new();
        for marker in markers {
            if marker.diameter <= 0.0 || marker.values().iter().any(|v| !v.is_finite()) { return Err("Invalid marker display data"); }
            data.extend(marker.bytes());
        }
        self.set_bytes(device, data)
    }

    fn set_bytes(&mut self, device: &wgpu::Device, data: Vec<u8>) -> Result<(), &'static str> {
        use wgpu::util::DeviceExt;
        u32::try_from(data.len() as u64 / self.stride).map_err(|_| "Too many stroke instances")?;
        if data == self.data { return Ok(()); }
        self.vertices = if data.is_empty() { None } else {
            self.uploads += 1; self.uploaded_bytes += data.len() as u64;
            Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("stroke instances"), contents: &data, usage: wgpu::BufferUsages::VERTEX,
            }))
        };
        self.data = data; if self.finite.is_some(){self.reset_flags(device)?;} Ok(())
    }

    fn reset_flags(&mut self,device:&wgpu::Device)->Result<(),&'static str>{
        let count=self.data.len()/self.stride as usize;
        self.set_flags(device,vec![0;if self.varying{count}else{count.min(1)}])
    }
    pub fn set_flags(&mut self,device:&wgpu::Device,flags:Vec<u32>)->Result<(),&'static str>{
        let count=self.data.len()/self.stride as usize;
        if flags.len()!=if self.varying{count}else{count.min(1)}{return Err("Clipping flags must match original ink instances")}
        if flags==self.flag_data{return Ok(())}use wgpu::util::DeviceExt;self.flags=if flags.is_empty(){None}else{let bytes=flags.iter().copied().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("original ink clipping flags"),contents:&bytes,usage:wgpu::BufferUsages::VERTEX}))};self.flag_data=flags;Ok(())
    }
    pub fn clipping_bytes(&self)->[u64;2]{[self.flags.as_ref().map_or(0,wgpu::Buffer::size),self.flag_data.capacity()as u64*4]}
    pub fn enable_finite(&mut self,device:&wgpu::Device,format:wgpu::TextureFormat,varying:bool){self.varying=varying;self.finite=Some(crate::finite_ink::Ink::new(device,format,self.stride,self.strict_depth,varying));self.reset_flags(device).expect("Original ink clipping flags");}
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){if let Some(ink)=&self.finite{ink.set(device,tables,depth)}}
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup) {
        let Some(vertices) = &self.vertices else { return; };
        if self.finite.as_ref().is_some_and(|ink|ink.draw(pass,view,vertices,self.flags.as_ref().expect("Finite original clipping flags"),(self.data.len()as u64/self.stride)as u32,self.vertices_per_instance)){return}
        pass.set_pipeline(&self.pipeline); pass.set_bind_group(0, view, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.draw(0..self.vertices_per_instance, 0..(self.data.len() as u64 / self.stride) as u32);
    }

    pub fn set_ids(&mut self, device: &wgpu::Device, ids: impl IntoIterator<Item = [u32; 2]>) -> Result<(), &'static str> {
        use wgpu::util::DeviceExt;
        let data: Vec<u8> = ids.into_iter().flatten().flat_map(u32::to_ne_bytes).collect();
        if data.len() as u64 / 8 != self.data.len() as u64 / self.stride { return Err("Pick IDs must match display instances"); }
        if data == self.id_data { return Ok(()); }
        self.ids = if data.is_empty() { None } else { Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("ink IDs"), contents: &data, usage: wgpu::BufferUsages::VERTEX })) };
        self.id_data = data; Ok(())
    }

    pub fn pick(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup, crop: &wgpu::BindGroup) {
        let (Some(vertices), Some(ids), Some(pipeline)) = (&self.vertices, &self.ids, &self.pick_pipeline) else { return; };
        pass.set_pipeline(pipeline); pass.set_bind_group(0, view, &[]); pass.set_bind_group(1, crop, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..)); pass.set_vertex_buffer(1, ids.slice(..));
        pass.draw(0..self.vertices_per_instance, 0..(self.data.len() as u64 / self.stride) as u32);
    }

    pub fn pick_finite(&self,device:&wgpu::Device,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,crop:&wgpu::BindGroup,tables:Option<&crate::finite_ink::Tables>,depth:&wgpu::TextureView){
        if let(Some(ink),Some(tables),Some(vertices),Some(ids))=(&self.finite,tables,&self.vertices,&self.ids){ink.pick(device,pass,view,crop,vertices,ids,self.flags.as_ref().expect("Finite original clipping flags"),(self.data.len()as u64/self.stride)as u32,self.vertices_per_instance,tables,depth)}else{self.pick(pass,view,crop)}
    }
    pub fn pick_bytes(&self) -> u64 { self.ids.as_ref().map_or(0, wgpu::Buffer::size) }

    pub fn usage(&self) -> [u64; 2] {
        [self.data.len() as u64 / self.stride, self.vertices.as_ref().map_or(0, wgpu::Buffer::size)]
    }

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout(device, format, &crate::stroke::shader(), 44, 6,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32])
    }

    pub fn joined(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout(device, format, &crate::chain::shader(), 72, 18,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32,
                4 => Float32x3, 5 => Float32x3, 6 => Uint32])
    }

    pub fn markers(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout(device, format, &crate::marker::shader(), 32, 6,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32, 2 => Float32x4])
    }

    fn layout(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
        stride: u64, vertices_per_instance: u32, attributes: &[wgpu::VertexAttribute]) -> Self {
        Self::layout_depth(device, format, source, stride, vertices_per_instance, attributes, crate::depth::ink(), true)
    }

    pub fn guides(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{let mut depth=crate::depth::ink();depth.depth_compare=Some(wgpu::CompareFunction::Always);Self::layout_depth(device,format,&crate::finite_project_source::expand(include_str!("guide.wgsl")),48,6,&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Uint32],depth,true)}
    pub fn overlay(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{
        let mut depth=crate::depth::ink();depth.depth_compare=Some(wgpu::CompareFunction::Always);
        Self::layout_depth(device,format,&crate::stroke::shader(),44,6,&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32],depth,true)
    }
    pub fn grid(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout_target(device, format, include_str!("grid.wgsl"), 44, 2,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32], crate::depth::grid(), false, true)
    }

    fn layout_target(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
        stride: u64, vertices_per_instance: u32, attributes: &[wgpu::VertexAttribute], depth: wgpu::DepthStencilState, pickable:bool, physical:bool,
    ) -> Self {
        let view_layout=crate::surface_view::layout(device);
        let crop_layout = crate::gpu_layout::uniform(device, 80);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ink settings"), bind_group_layouts: &[Some(&view_layout)], immediate_size: 0 });
        let pick_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ink pick settings"), bind_group_layouts: &[Some(&view_layout), Some(&crop_layout)], immediate_size: 0 });
        let pick_source = match stride { 48=>include_str!("guide_pick.wgsl"),44 => include_str!("stroke_pick.wgsl"), 72 => include_str!("chain_pick.wgsl"), 32 => include_str!("marker_pick.wgsl"), _ => unreachable!() };
        let pick_shader = pickable.then(||device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("ink IDs"),
            source: wgpu::ShaderSource::Wgsl(format!("{source}\n{}\n{pick_source}", include_str!("pick_crop.wgsl")).into()) }));
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("stroke"), source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let strict_depth=depth.depth_compare==Some(crate::depth::CLOSER);
        let mut targets=vec![Some(wgpu::ColorTargetState{format,blend:if physical{None}else{Some(wgpu::BlendState::ALPHA_BLENDING)},write_mask:wgpu::ColorWrites::ALL})];
        if physical{targets.push(Some(crate::primitive_image::target(true)));}
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("stroke"), layout: Some(&layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout { array_stride: stride, step_mode: wgpu::VertexStepMode::Instance, attributes }],
            },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &targets,
            }),
            primitive: wgpu::PrimitiveState{topology:if physical{wgpu::PrimitiveTopology::LineList}else{wgpu::PrimitiveTopology::TriangleList},..Default::default()},
            depth_stencil: Some(depth),
            multisample: Default::default(), multiview_mask: None, cache: None,
        });
        let pick_pipeline = pick_shader.as_ref().map(|pick_shader|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("ink IDs"), layout: Some(&pick_layout),
            vertex: wgpu::VertexState { module: &pick_shader, entry_point: Some("pick_vertex"), compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout { array_stride: stride, step_mode: wgpu::VertexStepMode::Instance, attributes },
                    wgpu::VertexBufferLayout { array_stride: 8, step_mode: wgpu::VertexStepMode::Instance, attributes: &wgpu::vertex_attr_array![7 => Uint32x2] }] },
            fragment: Some(wgpu::FragmentState { module: &pick_shader, entry_point: Some("pick_fragment"), compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(),
            depth_stencil: Some(crate::depth::ink()), multisample: Default::default(), multiview_mask: None, cache: None,
        }));
        Self {flags:None,flag_data:Vec::new(),varying:false,finite:None,strict_depth, pipeline, pick_pipeline, ids: None, id_data: Vec::new(),  vertices: None, data: Vec::new(), stride, vertices_per_instance, uploads: 0, uploaded_bytes: 0 }
    }

    fn layout_depth(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
        stride: u64, vertices_per_instance: u32, attributes: &[wgpu::VertexAttribute], depth: wgpu::DepthStencilState, pickable:bool,
    ) -> Self {
        Self::layout_target(device,format,source,stride,vertices_per_instance,attributes,depth,pickable,false)
    }
}

#[cfg(test)]
#[path="grid_pick_pipeline_tests.rs"]
mod pick_pipeline_tests;
```

### 2. `src/finite_ink.rs`

Use one selected layout per existing pipeline. Zero stride broadcasts the complete existing u32 record. Make metadata addressing explicit at construction. Preserve identical color and cropped integer addressing.

Edit the existing block to match this code; keep unchanged lines:

```rust
fn pipeline(device:&wgpu::Device,format:wgpu::TextureFormat,stride:u64,grid:bool,picking:bool,varying:bool,view:&wgpu::BindGroupLayout,crop:&wgpu::BindGroupLayout,layout:&wgpu::BindGroupLayout,attributes:&[wgpu::VertexAttribute])->wgpu::RenderPipeline{
   let source=crate::finite_sources::source_style(stride,picking,grid);let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("finite original ink"),source:wgpu::ShaderSource::Wgsl(source.into())});
   let clipping=crate::clipping_gpu::layout(device);let groups=if picking{vec![Some(view),Some(crop),Some(layout),Some(&clipping)]}else{vec![Some(view),Some(layout),Some(&clipping)]};let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("finite original ink"),bind_group_layouts:&groups,immediate_size:0});
   let mut buffers=vec![wgpu::VertexBufferLayout{array_stride:stride,step_mode:wgpu::VertexStepMode::Instance,attributes}];let ids=wgpu::vertex_attr_array![7=>Uint32x2];if picking{buffers.push(wgpu::VertexBufferLayout{array_stride:8,step_mode:wgpu::VertexStepMode::Instance,attributes:&ids});}
   let flags=wgpu::vertex_attr_array![8=>Uint32];buffers.push(wgpu::VertexBufferLayout{array_stride:if varying{4}else{0},step_mode:wgpu::VertexStepMode::Instance,attributes:&flags});
   let mut depth=crate::depth::ink();depth.depth_compare=Some(wgpu::CompareFunction::Always);
   device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("finite original ink"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some(if picking{"pick_vertex"}else{"vertex"}),compilation_options:Default::default(),buffers:&buffers},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(if picking{"pick_fragment"}else{"fragment"}),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:if picking{wgpu::TextureFormat::Rg32Uint}else{format},blend:if picking{None}else{Some(wgpu::BlendState::ALPHA_BLENDING)},write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(depth),multisample:Default::default(),multiview_mask:None,cache:None})
  }
use std::cell::RefCell;
pub struct Ink{color:wgpu::RenderPipeline,pick:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,bindings:RefCell<Option<(wgpu::Buffer,wgpu::Buffer,wgpu::TextureView,wgpu::TextureView,wgpu::BindGroup)>>}
#[derive(Clone)]
pub struct Tables{pub projected:wgpu::Buffer,pub tiles:wgpu::Buffer,pub primitive:wgpu::TextureView}
impl Ink{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat,stride:u64,grid:bool,varying:bool)->Self{
  let attributes=match stride{44=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32].to_vec(),72=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Float32x3,5=>Float32x3,6=>Uint32].to_vec(),32=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x4].to_vec(),_=>panic!("Unsupported finite ink layout")};
  let view=crate::surface_view::layout(device);let crop=crate::gpu_layout::uniform(device,80);
  let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("finite ink coverage"),entries:&[crate::tile_pipelines::entry(0,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Storage{read_only:true}),crate::tile_pipelines::entry(1,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Storage{read_only:true}),wgpu::BindGroupLayoutEntry{binding:2,visibility:wgpu::ShaderStages::FRAGMENT,ty:wgpu::BindingType::Texture{sample_type:wgpu::TextureSampleType::Depth,view_dimension:wgpu::TextureViewDimension::D2,multisampled:false},count:None},crate::text_gpu_support::texture_entry(4,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Uint,false)]});
let color=pipeline(device,format,stride,grid,false,varying,&view,&crop,&layout,&attributes);let pick=pipeline(device,format,stride,grid,true,varying,&view,&crop,&layout,&attributes);Self{color,pick,layout,bindings:RefCell::new(None)}
 }
 fn group(&self,device:&wgpu::Device,tables:&Tables,depth:&wgpu::TextureView)->wgpu::BindGroup{device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("finite original ink coverage"),layout:&self.layout,entries:&[wgpu::BindGroupEntry{binding:0,resource:tables.projected.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:tables.tiles.as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:wgpu::BindingResource::TextureView(depth)},wgpu::BindGroupEntry{binding:4,resource:wgpu::BindingResource::TextureView(&tables.primitive)}]})}
 pub fn set(&self,device:&wgpu::Device,tables:&Tables,depth:&wgpu::TextureView){let mut bindings=self.bindings.borrow_mut();if bindings.as_ref().is_some_and(|(p,t,d,i,_)|p==&tables.projected&&t==&tables.tiles&&d==depth&&i==&tables.primitive){return}let group=self.group(device,tables,depth);*bindings=Some((tables.projected.clone(),tables.tiles.clone(),depth.clone(),tables.primitive.clone(),group));}
 pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,vertices:&wgpu::Buffer,flags:&wgpu::Buffer,count:u32,verts:u32)->bool{let bindings=self.bindings.borrow();let Some((_,_,_,_,group))=bindings.as_ref()else{return false};pass.set_pipeline(&self.color);pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,group,&[]);pass.set_vertex_buffer(0,vertices.slice(..));pass.set_vertex_buffer(1,flags.slice(..));pass.draw(0..verts,0..count);true}
 pub fn pick(&self,device:&wgpu::Device,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,crop:&wgpu::BindGroup,vertices:&wgpu::Buffer,ids:&wgpu::Buffer,flags:&wgpu::Buffer,count:u32,verts:u32,tables:&Tables,depth:&wgpu::TextureView){let group=self.group(device,tables,depth);pass.set_pipeline(&self.pick);pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,crop,&[]);pass.set_bind_group(2,&group,&[]);pass.set_vertex_buffer(0,vertices.slice(..));pass.set_vertex_buffer(1,ids.slice(..));pass.set_vertex_buffer(2,flags.slice(..));pass.draw(0..verts,0..count);}
}
```

### 3. `src/renderer.rs`

Mesh borders retain source-dependent clipping flags. Ordinary original lines, paths and points broadcast one record.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        let mut renderer = Self {surface_options:Default::default(),world_bounds:Default::default(),space:Default::default(),opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, primitive, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy,grid_anchor:[0.;3], size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode:Default::default(),pick_cursor:None };
        for lane in [&mut renderer.mesh_ink,&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers]{lane.enable_finite(&renderer.device,format);}
        renderer.highlight.enable_finite(&renderer.device,format);
        renderer.set_scene(scene, None);
        renderer
```

Replace it with:

```rust
        let mut renderer = Self {surface_options:Default::default(),world_bounds:Default::default(),space:Default::default(),opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, primitive, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy,grid_anchor:[0.;3], size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode:Default::default(),pick_cursor:None };
        renderer.mesh_ink.enable_finite(&renderer.device,format,true);
        for lane in [&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers]{lane.enable_finite(&renderer.device,format,false);}
        renderer.highlight.enable_finite(&renderer.device,format);
        renderer.set_scene(scene, None);
        renderer
```

**Complete updated block reference:**

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
    surface_options:std::cell::Cell<[bool;2]>,
    world_bounds:std::cell::RefCell<Option<crate::bounds::Bounds>>,
    space:crate::render_space::Origin,
    opacity:std::cell::Cell<f32>,
    ambient_enabled:std::cell::Cell<bool>,
    ambient:std::cell::RefCell<Option<crate::ambient_gpu::Ambient>>,
    ambient_rows:crate::ambient_rows::Rows,
    pyramid:std::cell::RefCell<Option<crate::pyramid_gpu::Pyramid>>,
    pyramid_revision:u64,
    caps:std::cell::RefCell<crate::caps_gpu::Caps>,
    clipping:std::cell::RefCell<crate::clipping_gpu::Clipping>,
    tiles:std::cell::RefCell<crate::tile_gpu::Tiles>,
    guides:crate::guide_gpu::Guide,
    measurement:crate::measure_gpu::Measurement,
    sheets:crate::sheet_gpu::Lane,
    clouds:std::cell::RefCell<Option<crate::cloud_gpu::Lane>>,
    cloud_view:std::cell::Cell<crate::cloud_gpu::View>,
    cloud_error:std::cell::RefCell<Option<String>>,
    outline:std::cell::RefCell<Option<crate::outline_gpu::Outline>>,
    global_outline:std::cell::RefCell<Option<crate::outline_gpu::Outline>>,
    outlines_enabled:std::cell::Cell<bool>,
    outline_composite:std::cell::RefCell<Option<crate::outline_composite::Composite>>,
    text:std::cell::RefCell<Option<crate::scene_text_gpu::Lane>>,
    text_error:std::cell::RefCell<Option<String>>,
    full_text_fonts:std::cell::RefCell<Option<Vec<Vec<u8>>>>,
    presentation_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    scene_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    format:wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    instances:crate::instance_gpu::Lane,
    instance_rows:std::collections::HashSet<ObjectId>,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
    highlight: crate::highlight::Highlight,
    preview: crate::tool_preview::Preview,
    widget: crate::widget::Widget,
    meshes: Vec<GpuMesh>,
    geometry: crate::geometry_cache::GeometryCache,
    settings_allocations: usize,
    settings_writes: usize,
    settings_bytes: usize,
    uniform: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    depth: wgpu::TextureView,
    primitive:crate::primitive_image::Image,
    strokes: crate::stroke_gpu::Lane,
    paths: crate::stroke_gpu::Lane,
    markers: crate::stroke_gpu::Lane,
    grid: crate::stroke_gpu::Lane,
    grid_settings: Option<crate::grid::Settings>,
    grid_plane:crate::cplane::CPlane,grid_anchor:[f64;3],
    size: [u32; 2],
    density: f32,
    picker: crate::pick_gpu::Picker,
    sources: crate::pick_sources::Sources,
    faces: crate::face_pick::Faces,
    mesh_ink: crate::stroke_gpu::Lane,
    pick_mode:crate::pick_sources::Mode,
    pick_cursor:Option<([f32;2],[u64;3])>,
}

impl Renderer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        crate::shader_check::check(&device, crate::clipping_gpu::source(include_str!("cloud_points.wgsl"),2,0), "Cloud points");
        crate::shader_check::check(&device, include_str!("cloud_resolve.wgsl").into(), "Cloud resolve");
        crate::shader_check::check(&device, crate::finite_project_source::source(), "Finite projection");
        crate::shader_check::check(&device, crate::finite_project_source::tiles(), "Finite tiles");
        crate::shader_check::check(&device, crate::finite_project_source::scan(), "Finite scan");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0).into()),
        });
        let view_layout = crate::surface_view::layout(&device);
        let model_layout = crate::gpu_layout::uniform(&device, crate::object_record::BYTES as u64);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"), bind_group_layouts: &[Some(&view_layout), Some(&model_layout),Some(&crate::clipping_gpu::layout(&device)),Some(&crate::gpu_geometry::GpuGeometry::storage_layout(&device))], immediate_size: 0 });
        let mesh_pipeline = |depth,primitive_write| device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[crate::draw_slots::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState{format,blend:Some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(primitive_write))],
            }),
            primitive: Default::default(),
            depth_stencil: Some(depth),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let pipeline = mesh_pipeline(crate::depth::surface(),true);
        let tiles=std::cell::RefCell::new(crate::tile_gpu::Tiles::new(&device,&view_layout));
        let instances=crate::instance_gpu::Lane::new(&device,&queue,format,&view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink(),false);
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
        let widget = crate::widget::Widget::new(&device, format);
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: crate::surface_view::BYTES,
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
        let primitive=crate::primitive_image::Image::new(&device,[640,480]);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
        let grid = crate::stroke_gpu::Lane::grid(&device, format);
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {surface_options:Default::default(),world_bounds:Default::default(),space:Default::default(),opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, primitive, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy,grid_anchor:[0.;3], size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode:Default::default(),pick_cursor:None };
        renderer.mesh_ink.enable_finite(&renderer.device,format,true);
        for lane in [&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers]{lane.enable_finite(&renderer.device,format,false);}
        renderer.highlight.enable_finite(&renderer.device,format);
        renderer.set_scene(scene, None);
        renderer
    }

    pub fn anchor(&self)->[f64;3] {self.space.anchor()}

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: crate::depth::FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        if self.size != [size.width, size.height] { self.cancel_pick(); }
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
        self.primitive=crate::primitive_image::Image::new(&self.device,self.size);
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 && density != self.density { self.cancel_pick(); self.density = density; }
    }

    pub fn stroke_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.strokes.usage();
        [count, bytes, self.strokes.uploads, self.strokes.uploaded_bytes]
    }

    pub fn path_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.paths.usage();
        [count, bytes, self.paths.uploads, self.paths.uploaded_bytes]
    }

    pub fn point_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.markers.usage();
        [count, bytes, self.markers.uploads, self.markers.uploaded_bytes]
    }

    #[cfg(test)]
    pub(crate) fn use_uniform_surface_records(&mut self) {self.instances.ordinary.clear();}

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        let anchor=self.anchor();
        self.ambient_rows.set(scene);if let Some(ambient)=self.ambient.borrow_mut().as_mut(){if self.ambient_enabled.get(){ambient.sources.set(&self.device,&self.queue,&self.ambient_rows.specs);}}
        self.pyramid_revision=self.pyramid_revision.wrapping_add(1);
        self.clipping.borrow_mut().set(scene);
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        self.cancel_pick(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, None).expect("Empty selection");
        self.set_source_ink(scene);
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let matching = old.iter().position(|row| row.id == object.id
                && std::rc::Rc::ptr_eq(&row.geometry.source, &object.mesh));
            let row = if let Some(index) = matching {
                let mut row = old.swap_remove(index);
                let [calls, bytes] = row.update(&self.queue, object,style(scene,object.id,selected==Some(object.id)));
                self.settings_writes += calls;
                self.settings_bytes += bytes;
                row
            } else {
                let geometry = self.geometry.get(&crate::text_gpu_support::GpuCtx::new(&self.device,&self.queue), &object.mesh);
                self.settings_allocations += 1;
                GpuMesh::with_geometry_at(&self.device,&layout,object,
                    style(scene,object.id,selected==Some(object.id)),geometry,anchor)
            };
            self.meshes.push(row);
        }
        drop(old);
        self.caps.borrow_mut().set_scene(&self.queue,scene,&mut self.meshes,&self.clipping.borrow().rows);
        for mesh in &mut self.meshes{mesh.set_xray(&self.queue,self.opacity.get()==0.);}
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,true).expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        self.sheets.set_anchored(&self.device,&self.queue,scene,selected.as_slice(),&self.sources.sheet_bases,anchor).expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        if let Err(error)=self.set_clouds(scene,selected.as_slice()) {*self.cloud_error.borrow_mut()=Some(error.to_string());}
        if let Err(error)=self.set_scene_text(scene,selected.as_slice()) {*self.text_error.borrow_mut()=Some(error.to_string());}
        *self.world_bounds.borrow_mut()=scene.bounds();
    }

    pub fn grow_camera(&self,camera:&mut crate::camera::Camera) {
        if let Some(bounds)=self.world_bounds.borrow().as_ref() {camera.grow_extent(bounds);}
    }

    pub fn camera_matrix(&self,camera:&crate::camera::Camera)->[f32;16] {camera.view_projection_anchored(self.anchor()).to_f32()}
    pub fn rebase_delay(&self,now:f64)->Option<u32> {self.space.retry_after_ms(now)}
    pub fn prepare_origin(&mut self,camera:&crate::camera::Camera,scene:&Scene,target:Option<crate::pick_sources::Target>,ids:&[ObjectId],now:f64)->Result<crate::render_space::Rebase,&'static str> {
        let previous=self.anchor();let mut frame=self.space.update(camera.target,camera.distance,now).ok_or("Invalid camera origin")?;
        frame.moved&=previous.map(f64::to_bits)!=frame.anchor.map(f64::to_bits);
        if frame.moved {
            self.pyramid_revision=self.pyramid_revision.wrapping_add(1);
            for mesh in &mut self.meshes {let[calls,bytes]=mesh.rebase(&self.queue,frame.anchor);self.settings_writes+=calls;self.settings_bytes+=bytes;}
            self.set_source_ink(scene);
            if !self.highlight.rebase_face(&self.queue,frame.anchor,target){self.set_highlight(scene,target)?;}
            self.select_objects(scene,ids)?;
            self.set_grid_on(self.grid_settings,self.grid_plane)?;
        }
        self.cloud_camera(camera);Ok(frame)
    }

    fn set_source_ink(&mut self,scene:&Scene) {
        let anchor=self.anchor();
        self.sources = crate::pick_sources::Sources::anchored(scene,anchor).expect("Prepared source topology is bounded");
        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().copied()).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().copied()).expect("Source edge IDs match");
        let planes=crate::clipping_rows::owners(scene);let flags=self.sources.mesh_ids.iter().map(|id|crate::sub_id::decode(id[1]).and_then(|(_,address)|self.sources.edges.get(address)).map_or(0,|target|if planes.contains(&target.owner){4096}else{0})).collect();self.mesh_ink.set_flags(&self.device,flags).expect("Original plane ink flags match source addresses");
        self.markers.set_points(&self.device, scene.points().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut marker=row.marker_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {marker.colour=colour;}marker}).chain(scene.control_markers_at(anchor).into_iter().map(|c| c.display)))
            .expect("Prepared point placement fits the display range");
        self.paths.set_chains(&self.device, scene.paths().iter().filter(|row| scene.visible(row.id)).flat_map(|row| {let mut segments=row.segments_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {for segment in &mut segments {segment.stroke.colour=colour;}}segments}))
            .expect("Prepared path placement fits the display range");
        self.strokes.set(&self.device, scene.lines().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut stroke=row.stroke_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {stroke.colour=colour;}stroke}))
            .expect("Prepared line placement fits the display range");
        self.markers.set_ids(&self.device, self.sources.marker_ids.iter().copied()).expect("Marker owners match");
        self.paths.set_ids(&self.device, self.sources.path_ids.iter().copied()).expect("Path owners match");
        self.strokes.set_ids(&self.device, self.sources.line_ids.iter().copied()).expect("Line owners match");
    }

    pub fn request_pick(&mut self, screen: [f32; 2], transform: [f32; 16]) -> Result<(), &'static str> {
        let window = crate::pick_window::Window::new(screen, self.size, self.density).ok_or("Invalid pick cursor")?;
        let origin=self.anchor();self.picker.queue_anchored(crate::pick_gpu::Request{window,transform,density:self.density,mode:self.pick_mode},origin)?;
        self.pick_cursor=Some((screen,origin.map(f64::to_bits)));self.pump_pick(transform);Ok(())
    }

    fn pump_pick(&mut self, transform: [f32; 16]) {
        let Ok(view) = crate::pick_view::View::anchored(transform,self.size,self.density,self.anchor()) else { self.cancel_pick(); return; };
        if self.pick_mode == crate::pick_sources::Mode::Face && !self.faces.ready(self.meshes.len()) {
            self.faces.set(&self.device, &self.meshes, &self.sources.mesh_bases,&self.clipping.borrow().buffer);
        }
        if let Some(lane)=self.text.borrow_mut().as_mut() {if let Err(error)=lane.prepare(&self.text_context(),&self.text_frame(transform)) {*self.text_error.borrow_mut()=Some(error.to_string());return;}}
        self.clipping.borrow_mut().frame_anchored(&self.queue,transform,self.anchor(),self.size[1],self.density);
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) {
            let mut clipping=self.clipping.borrow_mut();clipping.frame_anchored(&self.queue,transform,self.anchor(),self.size[1],self.density);
            let bytes=crate::surface_view::bytes(transform,[0.;4],self.size,self.density);self.queue.write_buffer(&self.uniform,0,&bytes);
            let tables={let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);let mut encoder=self.device.create_command_encoder(&Default::default());tiles.encode(&self.device,&mut encoder,&self.view_group,transform);self.queue.submit([encoder.finish()]);tiles.map_report();crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.primitive.view.clone()}};
            self.picker.submit_finite(&self.device, &self.queue, request,
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces,self.text.borrow().as_ref(),self.clouds.borrow().as_ref(),Some(&self.sheets),Some(&tables),&clipping.group,&clipping.buffer,Some(&mut self.caps.borrow_mut()),Some(&self.instances)); }
    }

    pub fn take_pick(&mut self, transform: [f32; 16]) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let view = match crate::pick_view::View::anchored(transform,self.size,self.density,self.anchor()) { Ok(view) => view, Err(error) => {
            self.cancel_pick(); return Some(Err(error));
        } };
        match self.picker.take(view) {
            Some(Ok(crate::pick_gpu::Reply::Hit(hit)))=>{self.pick_cursor=None;Some(Ok(hit))},
            Some(Ok(crate::pick_gpu::Reply::Stale))=>{
                if let Some((screen,_))=self.pick_cursor.filter(|(_,origin)|*origin!=self.anchor().map(f64::to_bits)) {
                    if let Err(error)=self.request_pick(screen,transform){self.cancel_pick();return Some(Err(error));}
                } else {
                    self.pump_pick(transform);
                    if self.picker.usage()[0]==0&&!self.picker.queued(){self.pick_cursor=None;}
                }
                None
            },
            Some(Err(error))=>{self.cancel_pick();Some(Err(error))},None=>None,
        }
    }

    pub fn set_pick_mode(&mut self, mode: crate::pick_sources::Mode) {
        if self.pick_mode != mode { self.cancel_pick(); self.pick_mode = mode; }
    }

    pub fn resolve_pick(&self, scene: &Scene, pixel: [u32; 2]) -> Option<crate::pick_sources::Target> {
        self.sources.resolve(scene, pixel, self.pick_mode)
    }

    pub fn part_usage(&self) -> [u64; 4] { let [count, bytes] = self.mesh_ink.usage(); [count, bytes, self.faces.bytes(), self.sources.bytes() as u64] }

    pub fn cancel_pick(&mut self) {self.pick_cursor=None;self.picker.cancel();}

    pub fn pick_usage(&self) -> [u64; 5] {
        let [active, pending, bytes] = self.picker.usage();
        [active, pending, bytes + self.strokes.pick_bytes() + self.paths.pick_bytes() + self.markers.pick_bytes() + self.mesh_ink.pick_bytes() + self.mesh_ink.usage()[1] + self.faces.bytes(),
            self.picker.submitted, self.picker.coalesced]
    }

    pub fn geometry_owners(&self) -> impl Iterator<Item = &std::rc::Rc<crate::gpu_geometry::GpuGeometry>> {
        self.meshes.iter().map(|row| &row.geometry)
    }

    pub fn usage(&self)->[u64;4]{let mut usage=crate::memory::gpu(&self.meshes);usage[1]+=self.geometry.allocated_bytes();usage}
    pub fn arena_cpu_bytes(&self)->usize{self.geometry.cpu_bytes()}

    pub fn uploaded_bytes(&self) -> u64 {
        self.instances.written_bytes + self.geometry.uploaded_bytes + self.settings_allocations as u64 * crate::object_record::BYTES as u64 + self.settings_bytes as u64
            + self.strokes.uploaded_bytes + self.paths.uploaded_bytes + self.markers.uploaded_bytes + self.grid.uploaded_bytes
    }

    pub fn set_grid(&mut self,settings:Option<crate::grid::Settings>)->Result<(),&'static str>{self.set_grid_on(settings,crate::cplane::CPlane::Xy)}
    pub fn set_grid_on(&mut self,settings:Option<crate::grid::Settings>,plane:crate::cplane::CPlane)->Result<(),&'static str>{
        let anchor=self.anchor();
        if settings==self.grid_settings&&plane==self.grid_plane&&anchor==self.grid_anchor{return Ok(())}
        let strokes=settings.map(|s|s.strokes_on_at(plane,anchor)).transpose()?.unwrap_or_default();
        self.grid.set(&self.device,strokes)?;self.grid_settings=settings;self.grid_plane=plane;self.grid_anchor=anchor;Ok(())
    }

    pub fn grid_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.grid.usage(); [count, bytes, self.grid.uploads, self.grid.uploaded_bytes]
    }

    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        let anchor=self.anchor();self.highlight.set_at(&self.device,&self.pipeline.get_bind_group_layout(1),scene,target,anchor)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
        let anchor=self.anchor();
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        for mesh in &mut self.meshes { if let Some(row) = scene.objects().iter().find(|r| r.id == mesh.id) {
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,false)?;
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;self.sheets.set_anchored(&self.device,&self.queue,scene,ids,&self.sources.sheet_bases,anchor)?;
        if !ids.is_empty() { self.highlight.objects_at(&self.device,scene,ids,anchor)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
    }

    pub fn set_guides(&mut self,overlay:&crate::guide_gpu::Overlay)->Result<(),&'static str>{if self.guides.set(&self.device,overlay)?{self.set_scene_measure_text().map_err(|_|"Cannot prepare editing guide label")?}Ok(())}
    pub fn guide_bytes(&self)->u64{self.guides.bytes()}
    pub fn set_measurement(&mut self,mark:Option<&crate::measure_mark::Mark>)->Result<(),&'static str>{let anchor=self.anchor();if self.measurement.set_at(&self.device,mark,anchor)?{self.set_scene_measure_text().map_err(|_|"Cannot prepare measurement annotation")?;}Ok(())}
    pub fn measurement_bytes(&self)->u64{self.measurement.bytes()}
    fn merge_measurement(&self,mut labels:Vec<crate::text::TextLabel>)->Vec<crate::text::TextLabel>{if let Some(mark)=&self.measurement.mark{labels.extend(mark.labels(&labels));}labels.extend(self.guides.overlay.labels(&labels));labels}
    fn set_scene_measure_text(&self)->anyhow::Result<()>{let mut labels=self.scene_text.borrow().clone();labels.extend(self.presentation_text.borrow().clone());self.adopt_text(self.merge_measurement(labels))}
    pub fn set_preview(&mut self,draft:Option<&crate::tool::Draft>)->Result<(),&'static str>{let anchor=self.anchor();self.preview.set_at(&self.device,draft,anchor)}
    pub fn preview_bytes(&self) -> u64 { self.preview.bytes() }

    pub fn set_gizmo(&mut self, camera:&crate::camera::Camera, origin:Option<[f64;3]>, active:f32) {
        let anchor=self.anchor();let matrix=camera.view_projection_anchored(anchor);
        let layout=origin.map(|p|std::array::from_fn(|i|p[i]-anchor[i])).and_then(|p|crate::widget_layout::world_per_pixel(&matrix,p,self.size[1] as f64/self.density as f64)
            .and_then(|scale|crate::widget_layout::layout(&matrix,p,scale,self.size,active)));
        self.widget.prepare(&self.device,&self.queue,layout);
    }
    pub fn gizmo_usage(&self) -> [u64;4] { self.widget.usage() }
    pub fn gizmo_extent(&self) -> Option<[u32;2]> {self.widget.extent()}

    fn text_context(&self)->crate::text_gpu_support::GpuCtx {crate::text_gpu_support::GpuCtx::new(&self.device,&self.queue)}
    fn text_frame(&self,transform:[f32;16])->crate::scene_text_gpu::TextFrame {crate::scene_text_gpu::TextFrame {mvp:transform,origin:self.anchor(),framebuffer:self.size,logical:[self.size[0] as f64/self.density as f64,self.size[1] as f64/self.density as f64],ortho_half_height:0.,clip:{let clip=self.clipping.borrow();let mut planes=[[0.;4];6];if clip.enabled {for (p,row) in planes.iter_mut().zip(&clip.rows){*p=[row.plane.normal[0],row.plane.normal[1],row.plane.normal[2],row.plane.offset];}}planes}}}
    fn adopt_text(&self,labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        if labels.is_empty() {self.text.borrow_mut().take();self.text_error.borrow_mut().take();return Ok(());}
        let mut lane=self.text.borrow_mut();if lane.is_none() {let mut new=crate::scene_text_gpu::Lane::new(&self.text_context(),crate::text_gpu_support::Target {format:self.format,samples:1});if let Some(fonts)=self.full_text_fonts.borrow().as_ref() {new.document.replace_fonts(fonts.clone())?;}*lane=Some(new);}lane.as_mut().unwrap().set_labels(labels)
    }
    pub fn set_text(&self,mut labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        for (i,label) in labels.iter_mut().enumerate() {label.id=u32::MAX-i as u32;label.object=None;}
        let mut merged=self.scene_text.borrow().clone();merged.extend(labels.clone());self.adopt_text(self.merge_measurement(merged))?;*self.presentation_text.borrow_mut()=labels;Ok(())
    }
    fn set_scene_text(&self,scene:&Scene,selected:&[ObjectId])->anyhow::Result<()> {
        let labels=scene.text_labels(selected).map_err(anyhow::Error::msg)?;let mut merged=labels.clone();merged.extend(self.presentation_text.borrow().clone());self.adopt_text(self.merge_measurement(merged))?;*self.scene_text.borrow_mut()=labels;Ok(())
    }
    pub fn refresh_text_bounds(&self,scene:&mut Scene)->bool {
        let mut changed=false;
        if let Some(lane)=self.text.borrow().as_ref() {for row in scene.texts_mut() {if let Some(run)=lane.document.runs.iter().find(|run|run.label.id==row.id.value()) {
            let extent=run.buffer.layout_runs().fold([0f64;2],|[w,h],line|[w.max(line.line_w as f64),h.max((line.line_top+line.line_height) as f64)]);
            if row.prepared.extent!=extent {row.prepared.extent=extent;changed=true;}
        }}}
        if changed {*self.world_bounds.borrow_mut()=scene.bounds();}
        changed
    }
    pub fn text_stats(&self)->Option<crate::scene_text_gpu::TextStats> {self.text.borrow().as_ref().map(|lane|lane.stats.clone())}
    pub fn text_scene_usage(&self)->[u64;4] {self.text.borrow().as_ref().map_or([0;4],|lane|[lane.allocated_bytes(),lane.texture_bytes(),lane.stats.world_plane_rasterizations,lane.document.runs.len() as u64])}
    pub fn replace_text_fonts(&self,fonts:Vec<Vec<u8>>)->anyhow::Result<()> {if let Some(lane)=self.text.borrow_mut().as_mut() {lane.document.replace_fonts(fonts.clone())?;} else {let mut document=crate::text::TextDocument::new();document.replace_fonts(fonts.clone())?;}*self.full_text_fonts.borrow_mut()=Some(fonts);Ok(())}
    pub fn clear_text_fonts(&self) {self.full_text_fonts.borrow_mut().take();}

    pub fn text_error(&self)->Option<String> {self.text_error.borrow().clone()}
    pub fn text_usage(&self)->[u64;6] {self.text.borrow().as_ref().map_or([0;6],|lane|{[lane.document.shape_count,lane.stats.preparations,lane.stats.skipped_preparations,lane.stats.raster_images as u64,lane.stats.raster_image_capacity_bytes as u64,lane.stats.atlas_resets]})}
    pub fn text_demo(&self)->anyhow::Result<()> {self.set_text(vec![crate::text_example::label()])}

    pub fn cloud_camera(&self,camera:&crate::camera::Camera) {self.cloud_view.set(crate::cloud_gpu::View::anchored(camera,self.anchor()));self.sheets.camera(camera);}
    pub fn cloud_usage(&self)->[u64;5] {self.clouds.borrow().as_ref().map_or([0;5],|lane|lane.usage())}
    pub fn cloud_error(&self)->Option<String> {self.cloud_error.borrow().clone()}
    fn set_clouds(&self,scene:&Scene,selected:&[ObjectId])->Result<(),&'static str> {
        if scene.clouds().is_empty()&&scene.streams().is_empty() {self.clouds.borrow_mut().take();self.cloud_error.borrow_mut().take();return Ok(());}
        let mut owner=self.clouds.borrow_mut();let lane=owner.get_or_insert_with(||crate::cloud_gpu::Lane::new(&self.text_context(),self.format,&self.clipping.borrow().group));lane.set_scene(&self.text_context(),scene,selected)
    }
    fn prepare_clouds(&self,transform:[f32;16]) {
        if let Some(lane)=self.clouds.borrow_mut().as_mut() {lane.set_clipping(self.clipping.borrow().writes);if let Err(error)=lane.prepare(&self.text_context(),transform,self.size,self.density,self.cloud_view.get()) {lane.failed();*self.cloud_error.borrow_mut()=Some(error.to_string());} else {self.cloud_error.borrow_mut().take();}}
    }
    pub fn set_opacity(&mut self,value:f32){if !value.is_finite(){return;}let value=value.clamp(0.,1.);if self.opacity.get()==value{return;}self.opacity.set(value);self.cancel_pick();self.pyramid_revision=self.pyramid_revision.wrapping_add(1);for mesh in &mut self.meshes{mesh.set_xray(&self.queue,value==0.);}if let Some(o)=self.outline.borrow_mut().as_mut(){o.invalidate();}if let Some(o)=self.global_outline.borrow_mut().as_mut(){o.invalidate();}}
    pub fn opacity(&self)->f32{self.opacity.get()}
    pub fn set_outlines(&self,on:bool){self.outlines_enabled.set(on);if !on{self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();}}
    pub fn global_outline_usage(&self)->[u64;4]{self.global_outline.borrow().as_ref().map_or([0;4],|o|o.usage())}
    pub fn outline_usage(&self)->[u64;4] {self.outline.borrow().as_ref().map_or([0;4],|outline|outline.usage())}

    pub fn highlight_bytes(&self) -> u64 { self.highlight.bytes() }

    pub fn sheet_camera(&self,camera:&crate::camera::Camera) {self.sheets.camera(camera);}
    #[cfg(test)]
    pub(crate) fn tile_buffers(&self)->[wgpu::Buffer;2]{let tiles=self.tiles.borrow();[tiles.projected.clone(),tiles.buffer.clone()]}
    pub fn set_clipping(&self,enabled:bool,solid:bool){let mut clip=self.clipping.borrow_mut();clip.enabled=enabled;clip.solid=solid;self.caps.borrow_mut().enabled=enabled;}
    pub fn set_lighting(&self,on:bool,backfaces:bool){self.surface_options.set([on,backfaces]);}
    pub fn set_ambient(&self,enabled:bool){if self.ambient_enabled.replace(enabled)==enabled{return;}let mut ambient=self.ambient.borrow_mut();if enabled{let owner=ambient.get_or_insert_with(||crate::ambient_gpu::Ambient::new(&self.device,self.format));owner.sources.set(&self.device,&self.queue,&self.ambient_rows.specs);}else if let Some(owner)=ambient.as_mut(){owner.clear(&self.device,&self.queue);}}
    pub fn ambient_usage(&self)->[u64;5]{self.ambient.borrow().as_ref().map_or([0;5],crate::ambient_gpu::Ambient::usage)}
    pub fn ambient_error(&self)->Option<&'static str>{self.ambient.borrow().as_ref().and_then(|owner|owner.error)}
    pub fn set_depth_pyramid(&self,enabled:bool){let mut owner=self.pyramid.borrow_mut();if enabled {if owner.is_none(){*owner=Some(crate::pyramid_gpu::Pyramid::new(&self.device));}}else{*owner=None;}}
    pub fn depth_pyramid_usage(&self)->[u64;4]{self.pyramid.borrow().as_ref().map_or([0;4],crate::pyramid_gpu::Pyramid::usage)}
    pub fn depth_pyramid_error(&self)->Option<&'static str>{self.pyramid.borrow().as_ref().and_then(|p|p.error)}
    pub fn cap_error(&self)->Option<&'static str>{self.caps.borrow().pick_error}
    pub fn cap_usage(&self)->[u64;5]{self.caps.borrow().usage()}
    pub fn clipping_usage(&self)->[u64;5]{let clip=self.clipping.borrow();[clip.rows.len()as u64,u64::from(clip.enabled),u64::from(clip.solid),clip.writes,clip.buffer.size()]}
    pub fn clipping_ink_bytes(&self)->[u64;2]{[&self.mesh_ink,&self.strokes,&self.paths,&self.markers,&self.grid].into_iter().map(crate::stroke_gpu::Lane::clipping_bytes).fold(self.highlight.clipping_bytes(),|a,b|[a[0]+b[0],a[1]+b[1]])}
    pub fn clipping_error(&self)->Option<&'static str>{self.clipping.borrow().error}
    pub fn tile_usage(&self)->[u64;7]{self.tiles.borrow().usage()}
    pub fn sheet_usage(&self)->[u64;5] {self.sheets.usage()}
    pub fn instance_usage(&self)->[u64;5] {self.instances.usage()}
    pub fn instance_allocations(&self)->[u64;3] {self.instances.allocation_parts()}
    pub fn stats(&self) -> [usize; 4] {
        [self.geometry.uploads, self.settings_allocations, self.settings_writes, self.settings_bytes]
    }

    pub fn draw(
        &self,
        view: &wgpu::TextureView,
        background: &crate::background::Background,
        transform: &[f32; 16],
    ) { self.draw_mode(view, background, transform, false); }

    pub fn draw_mode(&self, view: &wgpu::TextureView, background: &crate::background::Background,
        transform: &[f32; 16], normals: bool,
    ) {
        let (clip_group,clip_revision)={let mut clip=self.clipping.borrow_mut();clip.frame_anchored(&self.queue,*transform,self.anchor(),self.size[1],self.density);(clip.group.clone(),clip.writes)};
        self.caps.borrow_mut().prepare(&self.device,&self.queue,self.format,self.size,{let clip=self.clipping.borrow();clip.enabled&&!clip.rows.is_empty()});
        self.prepare_clouds(*transform);
        let options=self.surface_options.get();
        let display=[f32::from(normals),self.opacity.get(),if self.ambient_enabled.get(){2.}else{f32::from(options[0])},f32::from(options[1])];
        let bytes=crate::surface_view::bytes(*transform,display,self.size,self.density);
        self.queue.write_buffer(&self.uniform, 0, &bytes);
        self.sheets.view(&self.queue,transform,self.size,self.density);
        let [r, g, b] = if self.ambient_enabled.get() { [0.94; 3] } else { background.rgb() };
        let mut encoder = self.device.create_command_encoder(&Default::default());
        let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);tiles.encode(&self.device,&mut encoder,&self.view_group,*transform);
        let tables=crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.primitive.view.clone()};for lane in [&self.mesh_ink,&self.strokes,&self.paths,&self.markers]{lane.finite_frame(&self.device,&tables,&self.depth);}self.highlight.finite_frame(&self.device,&tables,&self.depth);
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(crate::depth::FAR as f32),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
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
                }),Some(wgpu::RenderPassColorAttachment{
                    view:&self.primitive.view,depth_slice:None,resolve_target:None,
                    ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store},
                })],
                ..Default::default()
            });
            self.grid.draw(&mut pass,&self.view_group);
            pass.set_bind_group(2,&clip_group,&[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {if !self.instances.draw_ordinary(&mut pass,mesh){pass.set_pipeline(&self.pipeline);self.instances.bind_own(&mut pass);mesh.draw_pulled(&mut pass);}}
            if self.opacity.get()>=1.{self.instances.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            self.highlight.mesh(&mut pass);
        }
        self.caps.borrow_mut().draw(&self.device,&mut encoder,view,&self.primitive.view,&self.depth,&self.view_group,&clip_group,&self.meshes,if self.clipping.borrow().enabled{self.clipping.borrow().rows.len()as u32}else{0},Some(&self.instances));
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("cloud depth composite"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});if let Some(clouds)=self.clouds.borrow().as_ref(){clouds.draw(&mut pass);}}
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite scene ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:None,stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});
            pass.set_bind_group(2,&clip_group,&[]);
            self.highlight.ink(&mut pass,&self.view_group);
            self.mesh_ink.draw(&mut pass,&self.view_group);
            self.strokes.draw(&mut pass,&self.view_group);self.sheets.draw(&mut pass);
            self.paths.draw(&mut pass,&self.view_group);
            self.markers.draw(&mut pass,&self.view_group);
            self.highlight.points(&mut pass,&self.view_group);
            self.preview.draw(&mut pass,&self.view_group);
        }
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("measurement overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});self.measurement.draw(&mut pass,&self.view_group);self.guides.draw(&mut pass,&self.view_group);}
        if self.outlines_enabled.get()&&!self.meshes.is_empty(){
            let ctx=self.text_context();
            let mut global=self.global_outline.borrow_mut();let global=global.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));
            global.encode_masks(&ctx,&mut encoder,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,true,Some(&self.primitive.view));
            let mut selected=self.outline.borrow_mut();
            if self.meshes.iter().any(GpuMesh::selected){let selected=selected.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));selected.encode_masks(&ctx,&mut encoder,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,false,Some(&self.primitive.view));}else{selected.take();}
            if let Some(alpha)=global.alpha_view(){self.outline_composite.borrow_mut().get_or_insert_with(||crate::outline_composite::Composite::new(&self.device,self.format)).draw(&self.device,&mut encoder,view,alpha,selected.as_ref().and_then(|s|s.alpha_view()));}
        }else{
            self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();
            if self.meshes.iter().any(GpuMesh::selected){let mut outline=self.outline.borrow_mut();let outline=outline.get_or_insert_with(||crate::outline_gpu::Outline::new(&self.text_context(),self.format));outline.encode(&self.text_context(),&mut encoder,view,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision);}else{self.outline.borrow_mut().take();}
        }
        self.widget.draw(&mut encoder,view);
        self.text_error.borrow_mut().take();
        if let Some(lane)=self.text.borrow_mut().as_mut() {
            match lane.prepare(&self.text_context(),&self.text_frame(*transform)) {
            Ok(())=>{
                let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("scene glyphs"),depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {view:&self.depth,depth_ops:Some(wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view,depth_slice:None,resolve_target:None,ops:wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});if let Err(error)=lane.draw(&mut pass) {*self.text_error.borrow_mut()=Some(error.to_string());}
            }
            Err(error)=>{*self.text_error.borrow_mut()=Some(error.to_string());}
            }
        }
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.depth,&self.view_group,&clip_group,&clip.buffer,&self.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground_at(self.anchor()),[self.pyramid_revision,clip_revision],Some(&self.instances));}}
        if let Some(pyramid)=self.pyramid.borrow_mut().as_mut(){pyramid.encode(&self.device,&self.queue,&mut encoder,&self.depth,*transform,self.size,self.density,[self.pyramid_revision,clip_revision]);}
        self.queue.submit([encoder.finish()]);tiles.map_report();
    }
}

fn style(scene:&Scene,id:ObjectId,selected:bool)->crate::gpu_mesh::Style {crate::gpu_mesh::Style {selected,owner:crate::feature_owner::display_owner(scene,id).map_or(0,ObjectId::value),colour:scene.face_rgba(id)}}

fn rgba(rgb:[u8;3])->[f32;4] {[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]}

#[cfg(all(test,not(target_arch="wasm32")))]
impl Renderer {
 pub fn outline_alpha(&self)->Vec<u8>{self.read_outline_alpha(false)}
 pub fn global_outline_alpha(&self)->Vec<u8>{self.read_outline_alpha(true)}
 fn read_outline_alpha(&self,global:bool)->Vec<u8> {
  let owner=if global{self.global_outline.borrow()}else{self.outline.borrow()};let Some(outline)=owner.as_ref() else {return Vec::new();};let Some(texture)=outline.alpha_texture() else {return Vec::new();};let [width,height]=self.size;let pitch=width.div_ceil(256)*256;
  let buffer=self.device.create_buffer(&wgpu::BufferDescriptor {label:Some("actual outline alpha readback"),size:pitch as u64*height as u64,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});let mut encoder=self.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo {buffer:&buffer,layout:wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},wgpu::Extent3d {width,height,depth_or_array_layers:1});self.queue.submit([encoder.finish()]);let (tx,rx)=std::sync::mpsc::channel();buffer.slice(..).map_async(wgpu::MapMode::Read,move|r|tx.send(r).unwrap());self.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();let bytes=buffer.slice(..).get_mapped_range();let pixels=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize].iter().copied()).collect();drop(bytes);buffer.unmap();pixels
 }
}

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="pyramid_gpu_tests.rs"]
mod pyramid_gpu_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="ambient_gpu_tests.rs"]
mod ambient_gpu_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="global_outline_tests.rs"]
mod global_outline_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="opacity_gpu_tests.rs"]
mod opacity_gpu_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="camera_extent_tests.rs"]
mod camera_extent_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="surface_lighting_tests.rs"]
mod surface_lighting_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="surface_controls_gpu_tests.rs"]
mod surface_controls_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="shared_view_tests.rs"]
mod shared_view_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="contact_feature_tests.rs"]
mod contact_feature_tests;
```

### 4. `src/highlight.rs`

Mixed selected clipping-plane strokes retain indexed flags. Selected paths and points use constant flags.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        Self {target:None,mesh:None,strokes: Lane::new(device, format), paths: Lane::joined(device, format), markers: Lane::markers(device, format) }
    }

    pub fn enable_finite(&mut self,device:&wgpu::Device,format:wgpu::TextureFormat){for lane in [&mut self.strokes,&mut self.paths,&mut self.markers]{lane.enable_finite(device,format)}}
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){for lane in [&self.strokes,&self.paths,&self.markers]{lane.finite_frame(device,tables,depth)}}
    pub fn set(&mut self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout, scene: &Scene, target: Option<Target>) -> Result<(), &'static str> {self.set_at(device,layout,scene,target,[0.;3])}
    pub fn set_at(&mut self,device:&wgpu::Device,layout:&wgpu::BindGroupLayout,scene:&Scene,target:Option<Target>,anchor:[f64;3])->Result<(),&'static str>{
```

Replace it with:

```rust
        Self {target:None,mesh:None,strokes: Lane::new(device, format), paths: Lane::joined(device, format), markers: Lane::markers(device, format) }
    }

    pub fn enable_finite(&mut self,device:&wgpu::Device,format:wgpu::TextureFormat){self.strokes.enable_finite(device,format,true);for lane in [&mut self.paths,&mut self.markers]{lane.enable_finite(device,format,false)}}
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){for lane in [&self.strokes,&self.paths,&self.markers]{lane.finite_frame(device,tables,depth)}}
    pub fn set(&mut self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout, scene: &Scene, target: Option<Target>) -> Result<(), &'static str> {self.set_at(device,layout,scene,target,[0.;3])}
    pub fn set_at(&mut self,device:&wgpu::Device,layout:&wgpu::BindGroupLayout,scene:&Scene,target:Option<Target>,anchor:[f64;3])->Result<(),&'static str>{
```

**Complete updated block reference:**

```rust
use crate::{gpu_mesh::GpuMesh, pick_sources::{Part, Target}, scene::Scene, stroke_gpu::Lane};
use std::rc::Rc;

pub struct Highlight {target:Option<Target>,mesh:Option<GpuMesh>,strokes:Lane,paths:Lane,markers:Lane}

impl Highlight {
    #[cfg(test)]
    pub(crate) fn geometry_for_test(&self)->Option<&Rc<crate::gpu_geometry::GpuGeometry>> {self.mesh.as_ref().map(|m|&m.geometry)}
    pub fn rebase_face(&mut self,queue:&wgpu::Queue,anchor:[f64;3],target:Option<Target>)->bool {
        if self.target!=target||!target.is_some_and(|t|matches!(t.part,Part::Face(_))){return false;}
        let Some(mesh)=self.mesh.as_mut() else{return false;};mesh.rebase(queue,anchor);true
    }

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self {target:None,mesh:None,strokes: Lane::new(device, format), paths: Lane::joined(device, format), markers: Lane::markers(device, format) }
    }

    pub fn enable_finite(&mut self,device:&wgpu::Device,format:wgpu::TextureFormat){self.strokes.enable_finite(device,format,true);for lane in [&mut self.paths,&mut self.markers]{lane.enable_finite(device,format,false)}}
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){for lane in [&self.strokes,&self.paths,&self.markers]{lane.finite_frame(device,tables,depth)}}
    pub fn set(&mut self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout, scene: &Scene, target: Option<Target>) -> Result<(), &'static str> {self.set_at(device,layout,scene,target,[0.;3])}
    pub fn set_at(&mut self,device:&wgpu::Device,layout:&wgpu::BindGroupLayout,scene:&Scene,target:Option<Target>,anchor:[f64;3])->Result<(),&'static str>{
        self.target=target.filter(|t|t.valid(scene));self.mesh=None;let mut strokes = Vec::new(); let mut paths = Vec::new(); let mut markers = Vec::new();
        let yellow = crate::colour::selection();
        if let Some(target)=self.target {match target.part {
            Part::Face(key) => {
                let row = scene.objects().iter().find(|r| r.id == target.owner).unwrap();
                let face = row.mesh.parts.faces.iter().position(|&f| f == key).unwrap() as u32;
                // Highlights own derived display data; changing selection changes presentation without mutating source geometry or adding a document history transaction.
                let mut vertices = Vec::new(); let mut normals = Vec::new(); let mut indices = Vec::new();
                for (tri, &source) in row.mesh.indices().chunks_exact(3).zip(&row.mesh.parts.triangles) { if source == face {
                    for &index in tri { indices.push(vertices.len() as u32); vertices.push(row.mesh.vertices()[index as usize]); normals.push(row.mesh.normals()[index as usize]); }
                } }
                let display = crate::mesh::Mesh::new(vertices, indices)?.with_normals(normals)?;
                let mut object = row.clone(); object.mesh = Rc::new(display);
                self.mesh = Some(GpuMesh::upload_at(device,layout,&object,true,anchor));
            }
            Part::BRepEdge(edge)=>{
                let row=scene.objects().iter().find(|r|r.id==target.owner).unwrap();let points=&row.cad.as_ref().unwrap().edges.iter().find(|border|border.edge==edge).unwrap().points;
                for pair in points.windows(2) {let ends=[pair[0],pair[1]].map(|p|crate::render_space::placed(&row.model,anchor).transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());strokes.push(crate::stroke::Stroke {start:ends[0],end:ends[1],colour:yellow,width:3.});}
            }
            Part::SurfaceEdge(edge)=>{
                let row=scene.objects().iter().find(|r|r.id==target.owner).unwrap();let surface=row.surface.as_ref().unwrap();let boundary=surface.grid.boundaries.iter().find(|b|b.edge==edge).unwrap();
                for ends in &boundary.segments {let ends=ends.map(|index|{let p=surface.grid.samples[index as usize].point;crate::render_space::placed(&row.model,anchor).transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32()});strokes.push(crate::stroke::Stroke {start:ends[0],end:ends[1],colour:yellow,width:3.});}
            }
            Part::MeshEdge(a, b) => {
                let row = scene.objects().iter().find(|r| r.id == target.owner).unwrap();
                let index = row.mesh.parts.edges.iter().position(|&edge| edge == (a, b)).unwrap();
                let ends = row.mesh.parts.endpoints[index].map(|p| crate::render_space::placed(&row.model,anchor).transform_point(&session_rust::Point::new(p[0], p[1], p[2])).to_f32());
                strokes.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: yellow, width: 3. });
            }
            Part::PathEdge(index) => {
                let row = scene.paths().iter().find(|r| r.id == target.owner).unwrap(); let points=row.points_at(anchor);
                strokes.push(crate::stroke::Stroke { start: points[index].to_f32(), end: points[index + 1].to_f32(), colour: yellow, width: row.prepared.width + 2. });
            }
            Part::Curve => {
                if let Some(row) = scene.lines().iter().find(|r| r.id == target.owner) {
                    strokes.push(crate::stroke::Stroke { colour: yellow, width: row.prepared.display.width + 2., ..row.stroke_at(anchor) });
                } else if let Some(row) = scene.paths().iter().find(|r| r.id == target.owner) {
                    paths.extend(row.segments_at(anchor).into_iter().map(|mut segment| { segment.stroke.colour = yellow; segment.stroke.width += 2.; segment }));
                }
            }
            Part::Control(_) | Part::SourceControl(_) => {
                let marker = scene.control_markers_at(anchor).into_iter().find(|m| m.parent == target.owner && m.part == target.part).unwrap();
                markers.push(crate::marker::Marker { colour: [1., 1., 0., 1.], ..marker.display });
            }
            Part::CloudPoint(id)=>{
                let p=crate::cloud_pick::position_at(scene,target.owner,id,anchor).ok_or("Cloud point is missing")?;
                markers.push(crate::marker::Marker {center:p.to_f32(),diameter:11.,colour:yellow});
            }
            Part::SheetEntity(_)|Part::Object => {}
        } }
        let flag=self.target.map_or(0,|t|if crate::clipping_rows::owners(scene).contains(&t.owner){4096}else{0});let count=strokes.len();self.strokes.set(device, strokes)?;self.strokes.set_flags(device,vec![flag;count])?; self.paths.set_chains(device, paths)?; self.markers.set_points(device, markers)?; Ok(())
    }

    pub fn objects(&mut self, device: &wgpu::Device, scene: &Scene, ids: &[crate::scene::ObjectId]) -> Result<(), &'static str> {self.objects_at(device,scene,ids,[0.;3])}
    pub fn objects_at(&mut self,device:&wgpu::Device,scene:&Scene,ids:&[crate::scene::ObjectId],anchor:[f64;3])->Result<(),&'static str>{
        self.target=None;self.mesh=None;let yellow = crate::colour::selection();
        self.strokes.set(device, scene.lines().iter().filter(|r| ids.contains(&r.id) && scene.visible(r.id)).map(|r|
            crate::stroke::Stroke { colour: yellow, width: r.prepared.display.width + 2., ..r.stroke_at(anchor) }).chain(scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|r.mesh.parts.endpoints.iter().map(move|pair|{let ends=pair.map(|p|crate::render_space::placed(&r.model,anchor).transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());crate::stroke::Stroke{start:ends[0],end:ends[1],colour:yellow,width:3.}}))))?;
        let planes=crate::clipping_rows::owners(scene);let flags=scene.lines().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)).map(|_|0).chain(scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|std::iter::repeat_n(if planes.contains(&r.id){4096}else{0},r.mesh.parts.endpoints.len()))).collect();self.strokes.set_flags(device,flags)?;
        self.paths.set_chains(device, scene.paths().iter().filter(|r| ids.contains(&r.id) && scene.visible(r.id)).flat_map(|r|
            r.segments_at(anchor).into_iter().map(|mut s| { s.stroke.colour = yellow; s.stroke.width += 2.; s })))?;
        self.markers.set_points(device, scene.points().iter().filter(|r| ids.contains(&r.id) && scene.visible(r.id)).map(|r|
            crate::marker::Marker { colour: yellow, diameter: r.prepared.display.diameter + 4., ..r.marker_at(anchor) }))?;
        Ok(())
    }

    pub fn mesh(&self, pass: &mut wgpu::RenderPass<'_>) { if let Some(mesh) = &self.mesh { mesh.draw_pulled(pass); } }
    pub fn ink(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup) { self.strokes.draw(pass,view); self.paths.draw(pass,view); }
    pub fn points(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup) { self.markers.draw(pass,view); }
    pub fn clipping_bytes(&self)->[u64;2]{[&self.strokes,&self.paths,&self.markers].into_iter().map(Lane::clipping_bytes).fold([0;2],|a,b|[a[0]+b[0],a[1]+b[1]])}
    pub fn bytes(&self) -> u64 { self.mesh.as_ref().map_or(0, |m| m.geometry.allocated_bytes() + m.settings_bytes())
        + self.strokes.usage()[1] + self.paths.usage()[1] + self.markers.usage()[1] }
}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/ink_metadata_tests.rs`

Supply real zero-stride GPU draws/IDs,8192-instance allocations, mixed indexed flags and complete release controls.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzk-broadcast-ordinary-ink-metadata-18.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/renderer.rs`

Connect optional native validation outside the browser runtime.

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzk-broadcast-ordinary-ink-metadata-19.rs" skip=26
```

**Result after `96zzzzzzk-broadcast-ordinary-ink-metadata`:** Each nonempty ordinary batch owns four metadata bytes on CPU and GPU, regardless of instance count. Indexed clipping behavior, actual original picks and all existing drawing remain intact.

## <a id="checkpoint-96zzzzzzl-layer-original-contact-boundaries"></a>Step 4 of 5 · Layer original contact boundaries (`96zzzzzzl-layer-original-contact-boundaries`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4/5: 96zzzzzzl-layer-o…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/stroke_gpu.rs, src/fini…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original contact normal</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shared contact layer</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">matching finite visibilit…</text>
</svg>
</div>

At steep angles, the contact fill can cover its own black border. Carry the original contact normal through the existing indexed mesh-ink record, apply the same bounded depth layer, and include that rise in the existing visibility tolerance. Ordinary ink keeps its four-byte record.

The original polygon, parent identity and archive remain unchanged. Reuse the existing packed normal codec, shared frame, mesh perimeter and finite visibility owner. The added normal belongs only to indexed strokes; color and cropped integer picking use the same geometry. A nearby original occluder must still hide the border.

> **Check:** *Why must the border carry depth rise into its visibility test?* — Moving its vertices toward the eye aligns the border with the contact fill. The physical plane-fit fallback also compares the border against the original host plane. Its existing sag tolerance must include the bounded rise so the host does not reject half the border.

### 1. `src/stroke_gpu.rs`

Keep existing flag callers and flatten only indexed [flag,normal] records. Validate both indexed words; constant records stay one word.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        self.set_flags(device,vec![0;if self.varying{count}else{count.min(1)}])
    }
    pub fn set_flags(&mut self,device:&wgpu::Device,flags:Vec<u32>)->Result<(),&'static str>{
        let count=self.data.len()/self.stride as usize;
        if flags.len()!=if self.varying{count}else{count.min(1)}{return Err("Clipping flags must match original ink instances")}
        if flags==self.flag_data{return Ok(())}use wgpu::util::DeviceExt;self.flags=if flags.is_empty(){None}else{let bytes=flags.iter().copied().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("original ink clipping flags"),contents:&bytes,usage:wgpu::BufferUsages::VERTEX}))};self.flag_data=flags;Ok(())
    }
    pub fn clipping_bytes(&self)->[u64;2]{[self.flags.as_ref().map_or(0,wgpu::Buffer::size),self.flag_data.capacity()as u64*4]}
```

Replace it with:

```rust
        self.set_flags(device,vec![0;if self.varying{count}else{count.min(1)}])
    }
    pub fn set_flags(&mut self,device:&wgpu::Device,flags:Vec<u32>)->Result<(),&'static str>{
        self.set_records(device,if self.varying{flags.into_iter().flat_map(|f|[f,crate::packed_vertex::NO_NORMAL]).collect()}else{flags})
    }
    pub fn set_records(&mut self,device:&wgpu::Device,flags:Vec<u32>)->Result<(),&'static str>{
        let count=self.data.len()/self.stride as usize;
        if flags.len()!=if self.varying{count*2}else{count.min(1)}{return Err("Clipping flags must match original ink instances")}
        if flags==self.flag_data{return Ok(())}use wgpu::util::DeviceExt;self.flags=if flags.is_empty(){None}else{let bytes=flags.iter().copied().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("original ink clipping flags"),contents:&bytes,usage:wgpu::BufferUsages::VERTEX}))};self.flag_data=flags;Ok(())
    }
    pub fn clipping_bytes(&self)->[u64;2]{[self.flags.as_ref().map_or(0,wgpu::Buffer::size),self.flag_data.capacity()as u64*4]}
```

**Complete updated block reference:**

```rust
pub struct Lane {
    flags:Option<wgpu::Buffer>,flag_data:Vec<u32>,varying:bool,
    finite:Option<crate::finite_ink::Ink>,
    strict_depth:bool,
    pipeline: wgpu::RenderPipeline,
    pick_pipeline: Option<wgpu::RenderPipeline>,
    ids: Option<wgpu::Buffer>,
    id_data: Vec<u8>,
    vertices: Option<wgpu::Buffer>,
    data: Vec<u8>,
    stride: u64,
    vertices_per_instance: u32,
    pub uploads: u64,
    pub uploaded_bytes: u64,
}

impl Lane {
    pub fn set(&mut self, device: &wgpu::Device, strokes: impl IntoIterator<Item = crate::stroke::Stroke>) -> Result<(), &'static str> {
        let mut data = Vec::new();
        for stroke in strokes {
            if stroke.width <= 0.0 || stroke.values().iter().any(|value| !value.is_finite()) {
                return Err("Invalid stroke display data");
            }
            data.extend(stroke.bytes());
        }
        if self.stride != 44 { return Err("Stroke data needs the stroke layout"); }
        self.set_bytes(device, data)
    }

    pub fn set_guides(&mut self,device:&wgpu::Device,segments:&[crate::guide_gpu::Segment])->Result<(),&'static str>{if self.stride!=48{return Err("Guide data needs the guide layout")}let mut bytes=Vec::with_capacity(segments.len()*48);for segment in segments{if segment.stroke.width<=0.||segment.stroke.values().iter().any(|v|!v.is_finite()){return Err("Invalid guide stroke")}bytes.extend(segment.stroke.bytes());bytes.extend(u32::from(segment.dashed).to_ne_bytes());}self.set_bytes(device,bytes)}
    pub fn set_chains(&mut self, device: &wgpu::Device, segments: impl IntoIterator<Item = crate::chain::Segment>) -> Result<(), &'static str> {
        if self.stride != 72 { return Err("Connected data needs the connected layout"); }
        let mut data = Vec::new();
        for segment in segments {
            if segment.stroke.width <= 0.0 || segment.heads > 3
                || segment.stroke.values().iter().chain(segment.previous.iter()).chain(segment.next.iter()).any(|v| !v.is_finite()) {
                return Err("Invalid connected display data");
            }
            data.extend(segment.bytes());
        }
        self.set_bytes(device, data)
    }

    pub fn set_points(&mut self, device: &wgpu::Device, markers: impl IntoIterator<Item = crate::marker::Marker>) -> Result<(), &'static str> {
        if self.stride != 32 { return Err("Marker data needs the marker layout"); }
        let mut data = Vec::new();
        for marker in markers {
            if marker.diameter <= 0.0 || marker.values().iter().any(|v| !v.is_finite()) { return Err("Invalid marker display data"); }
            data.extend(marker.bytes());
        }
        self.set_bytes(device, data)
    }

    fn set_bytes(&mut self, device: &wgpu::Device, data: Vec<u8>) -> Result<(), &'static str> {
        use wgpu::util::DeviceExt;
        u32::try_from(data.len() as u64 / self.stride).map_err(|_| "Too many stroke instances")?;
        if data == self.data { return Ok(()); }
        self.vertices = if data.is_empty() { None } else {
            self.uploads += 1; self.uploaded_bytes += data.len() as u64;
            Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("stroke instances"), contents: &data, usage: wgpu::BufferUsages::VERTEX,
            }))
        };
        self.data = data; if self.finite.is_some(){self.reset_flags(device)?;} Ok(())
    }

    fn reset_flags(&mut self,device:&wgpu::Device)->Result<(),&'static str>{
        let count=self.data.len()/self.stride as usize;
        self.set_flags(device,vec![0;if self.varying{count}else{count.min(1)}])
    }
    pub fn set_flags(&mut self,device:&wgpu::Device,flags:Vec<u32>)->Result<(),&'static str>{
        self.set_records(device,if self.varying{flags.into_iter().flat_map(|f|[f,crate::packed_vertex::NO_NORMAL]).collect()}else{flags})
    }
    pub fn set_records(&mut self,device:&wgpu::Device,flags:Vec<u32>)->Result<(),&'static str>{
        let count=self.data.len()/self.stride as usize;
        if flags.len()!=if self.varying{count*2}else{count.min(1)}{return Err("Clipping flags must match original ink instances")}
        if flags==self.flag_data{return Ok(())}use wgpu::util::DeviceExt;self.flags=if flags.is_empty(){None}else{let bytes=flags.iter().copied().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("original ink clipping flags"),contents:&bytes,usage:wgpu::BufferUsages::VERTEX}))};self.flag_data=flags;Ok(())
    }
    pub fn clipping_bytes(&self)->[u64;2]{[self.flags.as_ref().map_or(0,wgpu::Buffer::size),self.flag_data.capacity()as u64*4]}
    pub fn enable_finite(&mut self,device:&wgpu::Device,format:wgpu::TextureFormat,varying:bool){self.varying=varying;self.finite=Some(crate::finite_ink::Ink::new(device,format,self.stride,self.strict_depth,varying));self.reset_flags(device).expect("Original ink clipping flags");}
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){if let Some(ink)=&self.finite{ink.set(device,tables,depth)}}
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup) {
        let Some(vertices) = &self.vertices else { return; };
        if self.finite.as_ref().is_some_and(|ink|ink.draw(pass,view,vertices,self.flags.as_ref().expect("Finite original clipping flags"),(self.data.len()as u64/self.stride)as u32,self.vertices_per_instance)){return}
        pass.set_pipeline(&self.pipeline); pass.set_bind_group(0, view, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.draw(0..self.vertices_per_instance, 0..(self.data.len() as u64 / self.stride) as u32);
    }

    pub fn set_ids(&mut self, device: &wgpu::Device, ids: impl IntoIterator<Item = [u32; 2]>) -> Result<(), &'static str> {
        use wgpu::util::DeviceExt;
        let data: Vec<u8> = ids.into_iter().flatten().flat_map(u32::to_ne_bytes).collect();
        if data.len() as u64 / 8 != self.data.len() as u64 / self.stride { return Err("Pick IDs must match display instances"); }
        if data == self.id_data { return Ok(()); }
        self.ids = if data.is_empty() { None } else { Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("ink IDs"), contents: &data, usage: wgpu::BufferUsages::VERTEX })) };
        self.id_data = data; Ok(())
    }

    pub fn pick(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup, crop: &wgpu::BindGroup) {
        let (Some(vertices), Some(ids), Some(pipeline)) = (&self.vertices, &self.ids, &self.pick_pipeline) else { return; };
        pass.set_pipeline(pipeline); pass.set_bind_group(0, view, &[]); pass.set_bind_group(1, crop, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..)); pass.set_vertex_buffer(1, ids.slice(..));
        pass.draw(0..self.vertices_per_instance, 0..(self.data.len() as u64 / self.stride) as u32);
    }

    pub fn pick_finite(&self,device:&wgpu::Device,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,crop:&wgpu::BindGroup,tables:Option<&crate::finite_ink::Tables>,depth:&wgpu::TextureView){
        if let(Some(ink),Some(tables),Some(vertices),Some(ids))=(&self.finite,tables,&self.vertices,&self.ids){ink.pick(device,pass,view,crop,vertices,ids,self.flags.as_ref().expect("Finite original clipping flags"),(self.data.len()as u64/self.stride)as u32,self.vertices_per_instance,tables,depth)}else{self.pick(pass,view,crop)}
    }
    pub fn pick_bytes(&self) -> u64 { self.ids.as_ref().map_or(0, wgpu::Buffer::size) }

    pub fn usage(&self) -> [u64; 2] {
        [self.data.len() as u64 / self.stride, self.vertices.as_ref().map_or(0, wgpu::Buffer::size)]
    }

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout(device, format, &crate::stroke::shader(), 44, 6,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32])
    }

    pub fn joined(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout(device, format, &crate::chain::shader(), 72, 18,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32,
                4 => Float32x3, 5 => Float32x3, 6 => Uint32])
    }

    pub fn markers(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout(device, format, &crate::marker::shader(), 32, 6,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32, 2 => Float32x4])
    }

    fn layout(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
        stride: u64, vertices_per_instance: u32, attributes: &[wgpu::VertexAttribute]) -> Self {
        Self::layout_depth(device, format, source, stride, vertices_per_instance, attributes, crate::depth::ink(), true)
    }

    pub fn guides(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{let mut depth=crate::depth::ink();depth.depth_compare=Some(wgpu::CompareFunction::Always);Self::layout_depth(device,format,&crate::finite_project_source::expand(include_str!("guide.wgsl")),48,6,&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Uint32],depth,true)}
    pub fn overlay(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{
        let mut depth=crate::depth::ink();depth.depth_compare=Some(wgpu::CompareFunction::Always);
        Self::layout_depth(device,format,&crate::stroke::shader(),44,6,&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32],depth,true)
    }
    pub fn grid(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout_target(device, format, include_str!("grid.wgsl"), 44, 2,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32], crate::depth::grid(), false, true)
    }

    fn layout_target(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
        stride: u64, vertices_per_instance: u32, attributes: &[wgpu::VertexAttribute], depth: wgpu::DepthStencilState, pickable:bool, physical:bool,
    ) -> Self {
        let view_layout=crate::surface_view::layout(device);
        let crop_layout = crate::gpu_layout::uniform(device, 80);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ink settings"), bind_group_layouts: &[Some(&view_layout)], immediate_size: 0 });
        let pick_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ink pick settings"), bind_group_layouts: &[Some(&view_layout), Some(&crop_layout)], immediate_size: 0 });
        let pick_source = match stride { 48=>include_str!("guide_pick.wgsl"),44 => include_str!("stroke_pick.wgsl"), 72 => include_str!("chain_pick.wgsl"), 32 => include_str!("marker_pick.wgsl"), _ => unreachable!() };
        let pick_shader = pickable.then(||device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("ink IDs"),
            source: wgpu::ShaderSource::Wgsl(format!("{source}\n{}\n{pick_source}", include_str!("pick_crop.wgsl")).into()) }));
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("stroke"), source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let strict_depth=depth.depth_compare==Some(crate::depth::CLOSER);
        let mut targets=vec![Some(wgpu::ColorTargetState{format,blend:if physical{None}else{Some(wgpu::BlendState::ALPHA_BLENDING)},write_mask:wgpu::ColorWrites::ALL})];
        if physical{targets.push(Some(crate::primitive_image::target(true)));}
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("stroke"), layout: Some(&layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout { array_stride: stride, step_mode: wgpu::VertexStepMode::Instance, attributes }],
            },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &targets,
            }),
            primitive: wgpu::PrimitiveState{topology:if physical{wgpu::PrimitiveTopology::LineList}else{wgpu::PrimitiveTopology::TriangleList},..Default::default()},
            depth_stencil: Some(depth),
            multisample: Default::default(), multiview_mask: None, cache: None,
        });
        let pick_pipeline = pick_shader.as_ref().map(|pick_shader|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("ink IDs"), layout: Some(&pick_layout),
            vertex: wgpu::VertexState { module: &pick_shader, entry_point: Some("pick_vertex"), compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout { array_stride: stride, step_mode: wgpu::VertexStepMode::Instance, attributes },
                    wgpu::VertexBufferLayout { array_stride: 8, step_mode: wgpu::VertexStepMode::Instance, attributes: &wgpu::vertex_attr_array![7 => Uint32x2] }] },
            fragment: Some(wgpu::FragmentState { module: &pick_shader, entry_point: Some("pick_fragment"), compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(),
            depth_stencil: Some(crate::depth::ink()), multisample: Default::default(), multiview_mask: None, cache: None,
        }));
        Self {flags:None,flag_data:Vec::new(),varying:false,finite:None,strict_depth, pipeline, pick_pipeline, ids: None, id_data: Vec::new(),  vertices: None, data: Vec::new(), stride, vertices_per_instance, uploads: 0, uploaded_bytes: 0 }
    }

    fn layout_depth(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
        stride: u64, vertices_per_instance: u32, attributes: &[wgpu::VertexAttribute], depth: wgpu::DepthStencilState, pickable:bool,
    ) -> Self {
        Self::layout_target(device,format,source,stride,vertices_per_instance,attributes,depth,pickable,false)
    }
}

#[cfg(test)]
#[path="grid_pick_pipeline_tests.rs"]
mod pick_pipeline_tests;
```

### 2. `src/finite_ink.rs`

Assemble only the lane’s chosen metadata interface. Use one or two explicit words in the existing attribute. Indexed strokes carry their contact normal; ordinary ink retains zero stride.

Edit the existing block to match this code; keep unchanged lines:

```rust
fn pipeline(device:&wgpu::Device,format:wgpu::TextureFormat,stride:u64,grid:bool,picking:bool,varying:bool,view:&wgpu::BindGroupLayout,crop:&wgpu::BindGroupLayout,layout:&wgpu::BindGroupLayout,attributes:&[wgpu::VertexAttribute])->wgpu::RenderPipeline{
   let source=crate::finite_sources::source_metadata(stride,picking,grid,varying);let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("finite original ink"),source:wgpu::ShaderSource::Wgsl(source.into())});
   let clipping=crate::clipping_gpu::layout(device);let groups=if picking{vec![Some(view),Some(crop),Some(layout),Some(&clipping)]}else{vec![Some(view),Some(layout),Some(&clipping)]};let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("finite original ink"),bind_group_layouts:&groups,immediate_size:0});
   let mut buffers=vec![wgpu::VertexBufferLayout{array_stride:stride,step_mode:wgpu::VertexStepMode::Instance,attributes}];let ids=wgpu::vertex_attr_array![7=>Uint32x2];if picking{buffers.push(wgpu::VertexBufferLayout{array_stride:8,step_mode:wgpu::VertexStepMode::Instance,attributes:&ids});}
   let flags=[wgpu::VertexAttribute{format:if varying{wgpu::VertexFormat::Uint32x2}else{wgpu::VertexFormat::Uint32},offset:0,shader_location:8}];buffers.push(wgpu::VertexBufferLayout{array_stride:if varying{8}else{0},step_mode:wgpu::VertexStepMode::Instance,attributes:&flags});
   let mut depth=crate::depth::ink();depth.depth_compare=Some(wgpu::CompareFunction::Always);
   device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("finite original ink"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some(if picking{"pick_vertex"}else{"vertex"}),compilation_options:Default::default(),buffers:&buffers},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(if picking{"pick_fragment"}else{"fragment"}),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:if picking{wgpu::TextureFormat::Rg32Uint}else{format},blend:if picking{None}else{Some(wgpu::BlendState::ALPHA_BLENDING)},write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(depth),multisample:Default::default(),multiview_mask:None,cache:None})
  }
use std::cell::RefCell;
pub struct Ink{color:wgpu::RenderPipeline,pick:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,bindings:RefCell<Option<(wgpu::Buffer,wgpu::Buffer,wgpu::TextureView,wgpu::TextureView,wgpu::BindGroup)>>}
#[derive(Clone)]
pub struct Tables{pub projected:wgpu::Buffer,pub tiles:wgpu::Buffer,pub primitive:wgpu::TextureView}
impl Ink{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat,stride:u64,grid:bool,varying:bool)->Self{
  let attributes=match stride{44=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32].to_vec(),72=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Float32x3,5=>Float32x3,6=>Uint32].to_vec(),32=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x4].to_vec(),_=>panic!("Unsupported finite ink layout")};
  let view=crate::surface_view::layout(device);let crop=crate::gpu_layout::uniform(device,80);
  let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("finite ink coverage"),entries:&[crate::tile_pipelines::entry(0,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Storage{read_only:true}),crate::tile_pipelines::entry(1,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Storage{read_only:true}),wgpu::BindGroupLayoutEntry{binding:2,visibility:wgpu::ShaderStages::FRAGMENT,ty:wgpu::BindingType::Texture{sample_type:wgpu::TextureSampleType::Depth,view_dimension:wgpu::TextureViewDimension::D2,multisampled:false},count:None},crate::text_gpu_support::texture_entry(4,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Uint,false)]});
let color=pipeline(device,format,stride,grid,false,varying,&view,&crop,&layout,&attributes);let pick=pipeline(device,format,stride,grid,true,varying,&view,&crop,&layout,&attributes);Self{color,pick,layout,bindings:RefCell::new(None)}
 }
 fn group(&self,device:&wgpu::Device,tables:&Tables,depth:&wgpu::TextureView)->wgpu::BindGroup{device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("finite original ink coverage"),layout:&self.layout,entries:&[wgpu::BindGroupEntry{binding:0,resource:tables.projected.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:tables.tiles.as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:wgpu::BindingResource::TextureView(depth)},wgpu::BindGroupEntry{binding:4,resource:wgpu::BindingResource::TextureView(&tables.primitive)}]})}
 pub fn set(&self,device:&wgpu::Device,tables:&Tables,depth:&wgpu::TextureView){let mut bindings=self.bindings.borrow_mut();if bindings.as_ref().is_some_and(|(p,t,d,i,_)|p==&tables.projected&&t==&tables.tiles&&d==depth&&i==&tables.primitive){return}let group=self.group(device,tables,depth);*bindings=Some((tables.projected.clone(),tables.tiles.clone(),depth.clone(),tables.primitive.clone(),group));}
 pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,vertices:&wgpu::Buffer,flags:&wgpu::Buffer,count:u32,verts:u32)->bool{let bindings=self.bindings.borrow();let Some((_,_,_,_,group))=bindings.as_ref()else{return false};pass.set_pipeline(&self.color);pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,group,&[]);pass.set_vertex_buffer(0,vertices.slice(..));pass.set_vertex_buffer(1,flags.slice(..));pass.draw(0..verts,0..count);true}
 pub fn pick(&self,device:&wgpu::Device,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,crop:&wgpu::BindGroup,vertices:&wgpu::Buffer,ids:&wgpu::Buffer,flags:&wgpu::Buffer,count:u32,verts:u32,tables:&Tables,depth:&wgpu::TextureView){let group=self.group(device,tables,depth);pass.set_pipeline(&self.pick);pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,crop,&[]);pass.set_bind_group(2,&group,&[]);pass.set_vertex_buffer(0,vertices.slice(..));pass.set_vertex_buffer(1,ids.slice(..));pass.set_vertex_buffer(2,flags.slice(..));pass.draw(0..verts,0..count);}
}
```

### 3. `src/finite_sources.rs`

Keep the existing constant shader validation entry meaningful. Declare explicit flag/facing accessors for the actual input type.

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn source(stride:u64,pick:bool)->String{source_style(stride,pick,false)}
pub fn source_style(stride:u64,pick:bool,grid:bool)->String{source_metadata(stride,pick,grid,false)}
pub fn source_metadata(stride:u64,pick:bool,grid:bool,varying:bool)->String{
 let(base,ids)=match stride{44=>(include_str!("finite_stroke.wgsl"),include_str!("finite_stroke_pick.wgsl")),72=>(include_str!("finite_chain.wgsl"),include_str!("finite_chain_pick.wgsl")),32=>(include_str!("finite_marker.wgsl"),include_str!("finite_marker_pick.wgsl")),_=>panic!("Unsupported finite ink layout")};
 let base=crate::finite_project_source::expand(base).replace("#include \"finite_visibility.wgsl\"",include_str!("finite_visibility.wgsl")).replace("#include \"ink_plane.wgsl\"",include_str!("ink_plane.wgsl"));
 let base=if grid{base.replace("const FINITE_GRID:bool=false;","const FINITE_GRID:bool=true;")}else{base};
 let(ty,words)=if varying{("vec2<u32>","m")}else{("u32","vec2<u32>(m,0x80008000u)")};
 let base=format!("alias InkRecord={ty};\nfn ink_words(m:InkRecord)->vec2<u32>{{return {words};}}\n{base}");
 let source=if pick{format!("{}\n{}\n{ids}",base.replace("@group(1)","@group(2)").replace("return vec2<i32>(pixel);","return vec2<i32>(pixel)-vec2<i32>(i32(crop._a),i32(crop._b));"),include_str!("pick_crop.wgsl"))}else{base};crate::clipping_gpu::source(&source,if pick{3}else{2},0)
}
```

### 4. `src/renderer.rs`

Move metadata to the existing lane without retaining a duplicate CPU table.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzzzzzl-layer-original-contact-boundaries-08.rs"
```

### 5. `src/pick_sources.rs`

Align explicit numeric records with the existing perimeter batch. Collect clipping owners once per source pass. Reuse the actual contact normal, inverse-transpose placement and existing packed codec. Append one explicit record per existing edge. Control-net edges have no contact facing. Account for actual retained records before upload.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn bytes(&self) -> usize {
        (self.faces.capacity() + self.edges.capacity() + self.controls.capacity()) * std::mem::size_of::<Target>()
            + self.mesh_bases.capacity() * std::mem::size_of::<(ObjectId, u32)>()
            + self.mesh_ink.capacity() * std::mem::size_of::<crate::stroke::Stroke>()
            + (self.mesh_ids.capacity() + self.line_ids.capacity() + self.path_ids.capacity() + self.marker_ids.capacity()) * 8
    }
```

Replace it with:

```rust
    pub fn bytes(&self) -> usize {
        (self.faces.capacity() + self.edges.capacity() + self.controls.capacity()) * std::mem::size_of::<Target>()
            + self.mesh_bases.capacity() * std::mem::size_of::<(ObjectId, u32)>()
            + self.mesh_metadata.capacity()*4
            + self.mesh_ink.capacity() * std::mem::size_of::<crate::stroke::Stroke>()
            + (self.mesh_ids.capacity() + self.line_ids.capacity() + self.path_ids.capacity() + self.marker_ids.capacity()) * 8
    }
```

**Locate this block:**

```rust
        for (owner, stroke) in scene.control_nets_at(anchor) {
            sources.mesh_ink.push(stroke);
            sources.mesh_ids.push([owner.value(), 0]);
        }
        for row in scene.control_markers_at(anchor) {
            let ids = append(&mut sources.controls, Kind::Control, Target { owner: row.parent, part: row.part })?;
```

Replace it with:

```rust
        for (owner, stroke) in scene.control_nets_at(anchor) {
            sources.mesh_ink.push(stroke);
            sources.mesh_ids.push([owner.value(), 0]);
            sources.mesh_metadata.extend([0,crate::packed_vertex::NO_NORMAL]);
        }
        for row in scene.control_markers_at(anchor) {
            let ids = append(&mut sources.controls, Kind::Control, Target { owner: row.parent, part: row.part })?;
```

**Locate this block:**

```rust
                let ends=ends.map(|p|model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: scene.view.edges.get(&row.id).map_or([0.,0.,0.,1.],|rgb|[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]), width: 1. });
                sources.mesh_ids.push(id(row.id, ids[1]));
            }
        }
        for row in scene.lines().iter().filter(|row| scene.visible(row.id)) {
```

Replace it with:

```rust
                let ends=ends.map(|p|model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: scene.view.edges.get(&row.id).map_or([0.,0.,0.,1.],|rgb|[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]), width: 1. });
                sources.mesh_ids.push(id(row.id, ids[1]));
                sources.mesh_metadata.extend([flag,facing]);
            }
        }
        for row in scene.lines().iter().filter(|row| scene.visible(row.id)) {
```

**Locate this block:**

```rust
        let mut sources = Self::default();
        let id = |owner: ObjectId, tag| [crate::feature_owner::display_owner(scene, owner).map_or(0, ObjectId::value),
            if scene.forest.derived(owner) { 0 } else { tag }];
        for row in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let model=crate::render_space::placed(&row.model,anchor);
            sources.mesh_bases.push((row.id, u32::try_from(sources.faces.len()).map_err(|_| "Too many source faces")?));
            for &key in &row.mesh.parts.faces { append(&mut sources.faces, Kind::Face, Target { owner: row.id, part: Part::Face(key) })?; }
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
            let mut surface_edges=row.surface.as_ref().map(|surface|surface.edge_ids());
            for (&(a, b), ends) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints) {
```

Replace it with:

```rust
        let mut sources = Self::default();
        let id = |owner: ObjectId, tag| [crate::feature_owner::display_owner(scene, owner).map_or(0, ObjectId::value),
            if scene.forest.derived(owner) { 0 } else { tag }];
        let planes=crate::clipping_rows::owners(scene);
        for row in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let model=crate::render_space::placed(&row.model,anchor);
            sources.mesh_bases.push((row.id, u32::try_from(sources.faces.len()).map_err(|_| "Too many source faces")?));
            for &key in &row.mesh.parts.faces { append(&mut sources.faces, Kind::Face, Target { owner: row.id, part: Part::Face(key) })?; }
            let mut facing=crate::packed_vertex::NO_NORMAL;
            if row.mesh.alphas().first().is_some_and(|a|(*a-1./255.).abs()<0.5/255.) {
                let n=row.mesh.normals()[0];let m=crate::normals::matrix(&row.model)?;
                facing=crate::packed_vertex::oct32(std::array::from_fn(|i|(0..3).map(|j|m[j*4+i]*n[j]).sum()));
            }
            let flag=if planes.contains(&row.id){4096}else{0};
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
            let mut surface_edges=row.surface.as_ref().map(|surface|surface.edge_ids());
            for (&(a, b), ends) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints) {
```

**Locate this block:**

```rust
    pub mesh_bases: Vec<(ObjectId, u32)>,
    pub mesh_ink: Vec<crate::stroke::Stroke>,
    pub mesh_ids: Vec<[u32; 2]>,
    pub line_ids: Vec<[u32; 2]>,
    pub path_ids: Vec<[u32; 2]>,
    pub marker_ids: Vec<[u32; 2]>,
```

Replace it with:

```rust
    pub mesh_bases: Vec<(ObjectId, u32)>,
    pub mesh_ink: Vec<crate::stroke::Stroke>,
    pub mesh_ids: Vec<[u32; 2]>,
    pub mesh_metadata:Vec<u32>,
    pub line_ids: Vec<[u32; 2]>,
    pub path_ids: Vec<[u32; 2]>,
    pub marker_ids: Vec<[u32; 2]>,
```

**Complete updated block reference:**

```rust
use crate::{scene::{ObjectId, Scene}, sub_id::{self, Kind}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part { Object, Face(usize), SurfaceEdge(usize), BRepEdge(usize), MeshEdge(usize, usize), PathEdge(usize), Curve, Control(usize), SourceControl(crate::source_controls::ControlId), CloudPoint(u32), SheetEntity(u32) }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Target { pub owner: ObjectId, pub part: Part }

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u32)]
pub enum Mode { #[default] Object, Face, Edge, Control }

#[derive(Default)]
pub struct Sources {
    pub sheet_bases:std::collections::HashMap<(ObjectId,usize),u32>,
    pub faces: Vec<Target>,
    pub edges: Vec<Target>,
    pub controls: Vec<Target>,
    pub mesh_bases: Vec<(ObjectId, u32)>,
    pub mesh_ink: Vec<crate::stroke::Stroke>,
    pub mesh_ids: Vec<[u32; 2]>,
    pub mesh_metadata:Vec<u32>,
    pub line_ids: Vec<[u32; 2]>,
    pub path_ids: Vec<[u32; 2]>,
    pub marker_ids: Vec<[u32; 2]>,
}

fn append(rows: &mut Vec<Target>, kind: Kind, target: Target) -> Result<[u32; 2], &'static str> {
    let tag = sub_id::encode(kind, rows.len())?; rows.push(target); Ok([target.owner.value(), tag])
}

impl Sources {
    // A source table translates compact GPU addresses to original keys; addresses belong to this scene snapshot, not the document's saved identity.
    pub fn new(scene:&Scene)->Result<Self,&'static str> {Self::anchored(scene,[0.;3])}
    pub fn anchored(scene:&Scene,anchor:[f64;3])->Result<Self,&'static str> {
        let mut sources = Self::default();
        let id = |owner: ObjectId, tag| [crate::feature_owner::display_owner(scene, owner).map_or(0, ObjectId::value),
            if scene.forest.derived(owner) { 0 } else { tag }];
        let planes=crate::clipping_rows::owners(scene);
        for row in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let model=crate::render_space::placed(&row.model,anchor);
            sources.mesh_bases.push((row.id, u32::try_from(sources.faces.len()).map_err(|_| "Too many source faces")?));
            for &key in &row.mesh.parts.faces { append(&mut sources.faces, Kind::Face, Target { owner: row.id, part: Part::Face(key) })?; }
            let mut facing=crate::packed_vertex::NO_NORMAL;
            if row.mesh.alphas().first().is_some_and(|a|(*a-1./255.).abs()<0.5/255.) {
                let n=row.mesh.normals()[0];let m=crate::normals::matrix(&row.model)?;
                facing=crate::packed_vertex::oct32(std::array::from_fn(|i|(0..3).map(|j|m[j*4+i]*n[j]).sum()));
            }
            let flag=if planes.contains(&row.id){4096}else{0};
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
            let mut surface_edges=row.surface.as_ref().map(|surface|surface.edge_ids());
            for (&(a, b), ends) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints) {
                let part=cad_edges.as_mut().and_then(|edges|edges.next()).map(|edge|Part::BRepEdge(*edge)).unwrap_or_else(||surface_edges.as_mut().and_then(|edges|edges.next()).map_or(Part::MeshEdge(a,b),Part::SurfaceEdge));
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                let ends=ends.map(|p|model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: scene.view.edges.get(&row.id).map_or([0.,0.,0.,1.],|rgb|[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]), width: 1. });
                sources.mesh_ids.push(id(row.id, ids[1]));
                sources.mesh_metadata.extend([flag,facing]);
            }
        }
        for row in scene.lines().iter().filter(|row| scene.visible(row.id)) {
            let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part: Part::Curve })?;
            sources.line_ids.push(id(row.id, ids[1]));
        }
        for row in scene.paths().iter().filter(|row| scene.visible(row.id)) {
            let points:Vec<_>=row.points_at(anchor).iter().map(session_rust::Point::to_f32).collect();
            for (index, _) in points.windows(2).enumerate().filter(|(_, pair)| pair[0] != pair[1]) {
                let part = if matches!(row.prepared.source, crate::chain::Source::Polyline(_)) { Part::PathEdge(index) } else { Part::Curve };
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                sources.path_ids.push(id(row.id, ids[1]));
            }
        }
        for row in scene.points().iter().filter(|row| scene.visible(row.id)) { sources.marker_ids.push(id(row.id, 1)); }
        for (owner, stroke) in scene.control_nets_at(anchor) {
            sources.mesh_ink.push(stroke);
            sources.mesh_ids.push([owner.value(), 0]);
            sources.mesh_metadata.extend([0,crate::packed_vertex::NO_NORMAL]);
        }
        for row in scene.control_markers_at(anchor) {
            let ids = append(&mut sources.controls, Kind::Control, Target { owner: row.parent, part: row.part })?;
            sources.marker_ids.push(id(row.parent, ids[1]));
        }
        for row in scene.sheets().iter().filter(|row|scene.visible(row.id)) {for page in row.prepared.pages() {sources.sheet_bases.insert((row.id,page.key()),u32::try_from(sources.edges.len()).map_err(|_|"Too many sheet pick addresses")?);for entity in &page.entities {append(&mut sources.edges,Kind::Edge,Target {owner:row.id,part:if *entity==u32::MAX {Part::Object} else {Part::SheetEntity(*entity)}})?;}}}
        Ok(sources)
    }

    pub fn bytes(&self) -> usize {
        (self.faces.capacity() + self.edges.capacity() + self.controls.capacity()) * std::mem::size_of::<Target>()
            + self.mesh_bases.capacity() * std::mem::size_of::<(ObjectId, u32)>()
            + self.mesh_metadata.capacity()*4
            + self.mesh_ink.capacity() * std::mem::size_of::<crate::stroke::Stroke>()
            + (self.mesh_ids.capacity() + self.line_ids.capacity() + self.path_ids.capacity() + self.marker_ids.capacity()) * 8
    }

    pub fn resolve(&self, scene: &Scene, pixel: [u32; 2], mode: Mode) -> Option<Target> {
        let owner = scene.pick_id(pixel[0])?;
        if mode == Mode::Object { return Some(Target { owner, part: Part::Object }); }
        let (kind, address) = sub_id::decode(pixel[1])?;
        if mode==Mode::Control && kind==Kind::Control && (scene.clouds().iter().any(|r|r.id==owner)||scene.streams().iter().any(|r|r.id==owner)) {
            return crate::cloud_pick::resolve(scene,owner,address);
        }
        let target = match (mode, kind) {
            (Mode::Face, Kind::Face) => self.faces.get(address),
            (Mode::Edge, Kind::Edge) => self.edges.get(address),
            (Mode::Control, Kind::Control) => self.controls.get(address), _ => None,
        }?;
        (target.owner == owner && target.valid(scene)).then_some(*target)
    }
}
```

### 6. `src/finite_stroke.wgsl`

Read the existing full shared frame. Reuse the surface depth owner; the normal decoder is already assembled. Read flags from the lane’s actual record. Layer both endpoints and retain the bounded depth rise before near clipping. Keep color and integer metadata layouts identical. Share geometry for cropped integer picking. Carry the same flat rise to color and pick fragments. Pass contact rise explicitly into existing visibility.

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
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

### 7. `src/finite_stroke_pick.wgsl`

Keep color and integer metadata layouts identical. Share geometry for cropped integer picking. Carry the same flat rise to color and pick fragments. Pass contact rise explicitly into existing visibility.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(0) var<uniform> crop: Crop;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
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
}
```

### 8. `src/finite_stroke.wgsl`

Hidden geometry carries no rise. Keep the actual bounded rise with the source axis.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```wgsl
    let extent = half_width + 0.70711;
    var position = select(a, b, corner.x > 0.5);
    position = vec4<f32>(position.xy + normal * extent * 2.0 / view.size * corner.y * position.w, position.zw);
    return Output(position, colour, corner.y * extent, half_width,flags,finite_axis(a,b),vec2<f32>(a.z/a.w,b.z/b.w),finite_screen(position));
}

#include "stroke_coverage.wgsl"
```

Replace it with:

```wgsl
    let extent = half_width + 0.70711;
    var position = select(a, b, corner.x > 0.5);
    position = vec4<f32>(position.xy + normal * extent * 2.0 / view.size * corner.y * position.w, position.zw);
    return Output(position, colour, corner.y * extent, half_width,flags,finite_axis(a,b),vec2<f32>(a.z/a.w,b.z/b.w),finite_screen(position),rise);
}

#include "stroke_coverage.wgsl"
```

**Locate this block:**

```wgsl
}

fn hidden(colour: vec4<f32>,flags:u32) -> Output {
    return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0));
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32,metadata:InkRecord) -> Output {
```

Replace it with:

```wgsl
}

fn hidden(colour: vec4<f32>,flags:u32) -> Output {
    return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0),0.);
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32,metadata:InkRecord) -> Output {
```

**Complete updated block reference:**

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

#include "stroke_coverage.wgsl"

@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32,@location(8) metadata:InkRecord) -> Output { return geometry(start, end, colour, width, index,metadata); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false,vec2<f32>(input.rise,0.));if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}

#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

### 9. `src/finite_stroke_pick.wgsl`

Copy the geometry’s rise into the cropped output. Preserve the complete shared coverage interface.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(0) var<uniform> crop: Crop;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
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
    output.rise=source.rise;output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.side, input.half_width,input.clip_flags,input.finite_axis_value,input.finite_depths,input.finite_pixel,input.rise);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true,vec2<f32>(input.rise,0.))<=0.){discard;}
    return input.id;
}
```

### 10. `src/finite_visibility.wgsl`

Expose explicit sag only to segment visibility. Forward the existing axis tolerance. Keep grid behavior unchanged. Reuse existing sag-aware plane and projected-triangle predicates.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```wgsl
 return finite_hidden_alpha(hidden,picking);
}

fn finite_visible(pixel:vec2<f32>,ends:vec4<f32>,depths:vec2<f32>,flags:u32)->bool{
 if(FINITE_GRID){return finite_grid_visible(pixel,ends,depths,flags);}
 let axis=finite_ink_axis(pixel,ends,depths);let tolerance=abs(axis.depth)*DEPTH_REL_TOL+abs(1.-axis.depth)*2.3841858e-7;
 if(!finite_section(axis.at,axis.depth,tolerance)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(axis.at,view.size,axis.depth))){return false;}
 var fit_at=pixel;let source_at=floor(axis.at)+fract(pixel);
```

Replace it with:

```wgsl
 return finite_hidden_alpha(hidden,picking);
}

fn finite_visible(pixel:vec2<f32>,ends:vec4<f32>,depths:vec2<f32>,flags:u32,sag:vec2<f32>)->bool{
 if(FINITE_GRID){return finite_grid_visible(pixel,ends,depths,flags);}
 var axis=finite_ink_axis(pixel,ends,depths);axis.sag=sag;let tolerance=abs(axis.depth)*DEPTH_REL_TOL+abs(1.-axis.depth)*2.3841858e-7;
 if(!finite_section(axis.at,axis.depth,tolerance)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(axis.at,view.size,axis.depth))){return false;}
 var fit_at=pixel;let source_at=floor(axis.at)+fract(pixel);
```

**Locate this block:**

```wgsl
}

fn finite_alpha(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32,picking:bool)->f32{
 let delta=axis.zw-axis.xy;let t=clamp(dot(pixel-axis.xy,delta)/max(dot(delta,delta),1e-20),0.,1.);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,depth))){return 0.;}
 let hidden=!finite_visible(pixel,axis,depths,flags);return finite_hidden_alpha(hidden,picking);
}

fn finite_disc_alpha(pixel:vec2<f32>,centre:vec2<f32>,depth:f32,flags:u32,picking:bool)->f32{
```

Replace it with:

```wgsl
}

fn finite_alpha(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32,picking:bool,sag:vec2<f32>)->f32{
 let delta=axis.zw-axis.xy;let t=clamp(dot(pixel-axis.xy,delta)/max(dot(delta,delta),1e-20),0.,1.);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,depth))){return 0.;}
 let hidden=!finite_visible(pixel,axis,depths,flags,sag);return finite_hidden_alpha(hidden,picking);
}

fn finite_disc_alpha(pixel:vec2<f32>,centre:vec2<f32>,depth:f32,flags:u32,picking:bool)->f32{
```

**Complete updated block reference:**

```wgsl
const FINITE_GRID:bool=false;
@group(1) @binding(0) var<storage,read> finite_projected:array<ProjectedTriangle>;
@group(1) @binding(1) var<storage,read> finite_tiles:array<vec4<u32>>;
@group(1) @binding(2) var finite_depth:texture_depth_2d;
@group(1) @binding(4) var finite_primitive:texture_2d<u32>;
#include "ink_plane.wgsl"
fn finite_section(pixel:vec2<f32>,depth:f32,tolerance:f32)->bool{
 let primitive=ink_primitive(pixel,0u);
 if(!clip_is_cap(primitive)){return true;}
 let plane=clip_cap_plane(primitive);
 if(plane>=clipping.count){return true;}
 let z=clip_plane_depth(plane,clip_ndc(pixel,view.size,0.).xy);
 return select(z<=depth+tolerance,z<depth-tolerance,FINITE_GRID);
}
fn finite_physical(pixel:vec2<f32>,depth:f32,tolerance:f32)->bool{let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_depth));if(any(at<vec2<i32>(0))||any(at>=size)){return false;}let z=textureLoad(finite_depth,at,0);return select(z<=depth+tolerance,z<depth-tolerance,FINITE_GRID);}
fn finite_screen(p:vec4<f32>)->vec2<f32>{return(p.xy/p.w*vec2<f32>(0.5,-0.5)+0.5)*view.size;}
fn finite_axis(a:vec4<f32>,b:vec4<f32>)->vec4<f32>{return vec4<f32>(finite_screen(a),finite_screen(b));}
fn finite_grid_visible(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32)->bool{
 let delta=axis.zw-axis.xy;let length2=dot(delta,delta);let t=clamp(dot(pixel-axis.xy,delta)/max(length2,1e-20),0.0,1.0);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);let relative=abs(depth)*1.9073486e-6+abs(1.0-depth)*2.3841858e-7;
 if(!finite_section(at,depth,relative)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,depth))){return false;}
 if(any(at<vec2<f32>(0.0))||any(at>=view.size)){return finite_physical(pixel,depth,relative);}
 if(finite_tiles[0].x==0u){return finite_physical(pixel,depth,relative);}
 let span=f32(visibility_tile_span_of(u32(view.size.x),u32(view.size.y)));let size=vec2<u32>(ceil(view.size/span));let cell=vec2<u32>(at/span);let head=finite_tiles[1u+cell.y*size.x+cell.x];
 if(head.w!=0u||head.z!=head.x){return finite_physical(pixel,depth,relative);}
 for(var i=0u;i<head.x;i++){
  let offset=head.y+i*2u;let nearest=bitcast<f32>(finite_tiles[(offset+1u)/4u][(offset+1u)%4u]);if(nearest<depth-relative||(!FINITE_GRID&&nearest<=depth+relative)){continue;}
  let primitive=finite_tiles[offset/4u][offset%4u];if(primitive==0u||primitive>arrayLength(&finite_projected)){continue;}
  let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,at);let tolerance=relative+0.00390625*(abs(triangle.gradient.x)+abs(triangle.gradient.y));
  if(hit.y>0.5&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(at,view.size,hit.x)))&&select(hit.x>depth+tolerance,hit.x>=depth-tolerance,FINITE_GRID)){return false;}
 }
 return true;
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
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(axis.at,view.size,axis.depth))){return false;}
 var fit_at=pixel;let source_at=floor(axis.at)+fract(pixel);
 if(clip_is_cap(ink_primitive(pixel,0u))){fit_at=source_at;}
 let plane_visible=ink_visible_plane(fit_at,axis,0u);
 if(finite_tiles[0].x==0u){return plane_visible;}
 if(finite_source_hidden(fit_at,axis)){return false;}
 if(plane_visible){return true;}
 if(any(axis.at<vec2<f32>(0.))||any(axis.at>=view.size)){return false;}
 let span=f32(visibility_tile_span_of(u32(view.size.x),u32(view.size.y)));let size_px=vec2<u32>(ceil(view.size/span));let cell=vec2<u32>(axis.at/span);let head=finite_tiles[1u+cell.y*size_px.x+cell.x];
 if(head.w!=0u||head.z!=head.x){return false;}
 for(var i=0u;i<head.x;i++){
  let offset=head.y+i*2u;let nearest=bitcast<f32>(finite_tiles[(offset+1u)/4u][(offset+1u)%4u]);
  if(nearest<=axis.depth+abs(axis.depth)*DEPTH_REL_TOL){continue;}
  let primitive=finite_tiles[offset/4u][offset%4u];if(primitive==0u||primitive>arrayLength(&finite_projected)){continue;}
  let bounds=finite_projected[primitive-1u].bounds;
  if(any(axis.at<bounds.xy-vec2<f32>(SLOPE_PX))||any(axis.at>bounds.zw+vec2<f32>(SLOPE_PX))){continue;}
  if(finite_triangle_hides(primitive,axis)){return false;}
 }
 return true;
}

fn finite_hidden_alpha(hidden:bool,picking:bool)->f32{
 if(picking){return select(1.,0.,hidden);}
 let glass=view.opacity>0.&&view.opacity<1.;
 return select(1.,select(0.,1.-view.opacity,glass),hidden&&view.opacity>0.);
}
```

### 11. `src/finite_chain.wgsl`

Ordinary connected paths have no contact rise.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false,vec2<f32>(0.));if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}
```

### 12. `src/finite_chain_pick.wgsl`

Ordinary connected paths have no contact rise.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true,vec2<f32>(0.))<=0.){discard;}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 13. `src/contact_feature_tests.rs`

Expose the unchanged original-source validation fixture.

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzl-layer-original-contact-boundaries-37.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 14. `src/contact_feature_tests.rs`

Share the unchanged fixture projection.

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzl-layer-original-contact-boundaries-38.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 15. `src/ink_metadata_tests.rs`

Indexed contact-capable records now carry two words; ordinary8192-instance assertions remain exactly4bytes.

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzl-layer-original-contact-boundaries-39.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 16. `src/contact_boundary_tests.rs`

Validate actual original contact pixels, complete projected fallback, transformed normals and owner picks.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzl-layer-original-contact-boundaries-40.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 17. `src/renderer.rs`

Connect optional native validation outside the browser runtime.

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzl-layer-original-contact-boundaries-41.rs" skip=23
```

**Result after `96zzzzzzl-layer-original-contact-boundaries`:** The real concave contact retains its perimeter in full and bounded rendering, with unchanged ordinary ink allocation and original ownership.

## <a id="checkpoint-96zzzzzzm-query-original-faces-under-contact-overlays"></a>Step 5 of 5 · Query original faces under contact overlays (`96zzzzzzm-query-original-faces-under-contact-overlays`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5/5: 96zzzzzzm-query-o…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/gpu_mesh.rs, src/render…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original versus derived r…</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original depth and source…</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">actual source key</text>
</svg>
</div>

The contact selects its original parent as an object, but its generated face address cannot resolve as an original face. Exclude derived display rows from the face query and its depth pass. Keep original locked geometry as an occluder and preserve real source keys.

A contact can span several original faces or extend outside its host. Aliasing every generated polygon to parent face zero would invent source identity. Query the actual original beneath it instead. Instances retain their original face identities; selection does not grant permission to edit a readonly definition. The readonly red overlay can remain above the selected original-face highlight.

> **Check:** *Why must the original depth pass also ignore derived contact rows in Face mode?* — The contact is layered nearer than its host. Skipping only its face-ID draw would leave that nearer generated depth blocking the original face beneath it. Original locked or missing-face-metadata geometry still retains depth and occlusion.

### 1. `src/gpu_mesh.rs`

Retain source-query eligibility on the existing CPU mesh. Classify original and derived display rows explicitly. Standalone original geometry and the existing highlight adapter remain valid. Keep the flag out of the packed GPU record. Update eligibility without a GPU setting write.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn settings_bytes(&self) -> u64 { self.model.size() }

    pub fn update(&mut self, queue: &wgpu::Queue, object: &Object, style:Style) -> [usize; 2] {
        self.translation=object.model.m[12..15].try_into().unwrap();
        let mut next = settings(object, style,self.cap_flags|if self.xray&&self.cap_flags&8192!=0{32768}else{0},self.cap_token,self.primitive_base,self.geometry.index_start,self.anchor);
        next[RANGE+8..RANGE+12].copy_from_slice(&self.previous[RANGE+8..RANGE+12]);
```

Replace it with:

```rust
    pub fn settings_bytes(&self) -> u64 { self.model.size() }

    pub fn update(&mut self, queue: &wgpu::Queue, object: &Object, style:Style) -> [usize; 2] {
        self.source_faces=style.source_faces;
        self.translation=object.model.m[12..15].try_into().unwrap();
        let mut next = settings(object, style,self.cap_flags|if self.xray&&self.cap_flags&8192!=0{32768}else{0},self.cap_token,self.primitive_base,self.geometry.index_start,self.anchor);
        next[RANGE+8..RANGE+12].copy_from_slice(&self.previous[RANGE+8..RANGE+12]);
```

**Locate this block:**

```rust
            layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: model.as_entire_binding() }],
        });
        Self { translation:object.model.m[12..15].try_into().unwrap(),anchor,geometry, model_group, id: object.id, model, previous: bytes,primitive_base:0,xray:false,cap_flags:0,cap_token:0 }
    }

    pub fn rebase(&mut self,queue:&wgpu::Queue,anchor:[f64;3])->[usize;2] {
```

Replace it with:

```rust
            layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: model.as_entire_binding() }],
        });
        Self { source_faces:style.source_faces,translation:object.model.m[12..15].try_into().unwrap(),anchor,geometry, model_group, id: object.id, model, previous: bytes,primitive_base:0,xray:false,cap_flags:0,cap_token:0 }
    }

    pub fn rebase(&mut self,queue:&wgpu::Queue,anchor:[f64;3])->[usize;2] {
```

**Locate this block:**

```rust
    }
    pub fn upload_at(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,object:&Object,selected:bool,anchor:[f64;3])->Self {
        let geometry=Rc::new(GpuGeometry::upload(device,Rc::clone(&object.mesh)));
        Self::with_geometry_at(device,layout,object,Style{selected,owner:if object.metadata.is_locked {0}else{object.id.value()},colour:None},geometry,anchor)
    }

    pub fn with_geometry(device: &wgpu::Device, layout: &wgpu::BindGroupLayout,
```

Replace it with:

```rust
    }
    pub fn upload_at(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,object:&Object,selected:bool,anchor:[f64;3])->Self {
        let geometry=Rc::new(GpuGeometry::upload(device,Rc::clone(&object.mesh)));
        Self::with_geometry_at(device,layout,object,Style{source_faces:true,selected,owner:if object.metadata.is_locked {0}else{object.id.value()},colour:None},geometry,anchor)
    }

    pub fn with_geometry(device: &wgpu::Device, layout: &wgpu::BindGroupLayout,
```

**Locate this block:**

```rust
    pub geometry: Rc<GpuGeometry>,
    model_group: wgpu::BindGroup,
    pub id: crate::scene::ObjectId,
    model: wgpu::Buffer,
    translation:[f64;3],anchor:[f64;3],
    previous: Record,primitive_base:u32,xray:bool,cap_flags:u32,cap_token:u32,
}

#[derive(Clone,Copy)]
pub struct Style {pub selected:bool,pub owner:u32,pub colour:Option<[f32;4]>}

fn settings(object: &Object, style:Style,cap_flags:u32,cap_token:u32,primitive_base:u32,index_start:u32,anchor:[f64;3]) -> Record {
    let mut bytes = [0; BYTES];
```

Replace it with:

```rust
    pub geometry: Rc<GpuGeometry>,
    model_group: wgpu::BindGroup,
    pub id: crate::scene::ObjectId,
    pub source_faces:bool,
    model: wgpu::Buffer,
    translation:[f64;3],anchor:[f64;3],
    previous: Record,primitive_base:u32,xray:bool,cap_flags:u32,cap_token:u32,
}

#[derive(Clone,Copy)]
pub struct Style {pub source_faces:bool,pub selected:bool,pub owner:u32,pub colour:Option<[f32;4]>}

fn settings(object: &Object, style:Style,cap_flags:u32,cap_token:u32,primitive_base:u32,index_start:u32,anchor:[f64;3]) -> Record {
    let mut bytes = [0; BYTES];
```

**Complete updated block reference:**

```rust
use crate::{gpu_geometry::GpuGeometry, scene::Object};
use std::rc::Rc;
use crate::object_record::{BYTES, Record, RANGE};
use wgpu::util::DeviceExt;

pub struct GpuMesh {
    pub geometry: Rc<GpuGeometry>,
    model_group: wgpu::BindGroup,
    pub id: crate::scene::ObjectId,
    pub source_faces:bool,
    model: wgpu::Buffer,
    translation:[f64;3],anchor:[f64;3],
    previous: Record,primitive_base:u32,xray:bool,cap_flags:u32,cap_token:u32,
}

#[derive(Clone,Copy)]
pub struct Style {pub source_faces:bool,pub selected:bool,pub owner:u32,pub colour:Option<[f32;4]>}

fn settings(object: &Object, style:Style,cap_flags:u32,cap_token:u32,primitive_base:u32,index_start:u32,anchor:[f64;3]) -> Record {
    let mut bytes = [0; BYTES];
    let model=crate::render_space::placed(&object.model,anchor).to_f32();
    for (index, value) in model.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&u32::from(style.selected).to_ne_bytes());
    let id=style.owner;
    bytes[68..72].copy_from_slice(&id.to_ne_bytes());
    let flags=if matches!(&object.original,Some(session_rust::Geometry::Plane(p)) if crate::clipping_geometry::is_clipping(p)){4096u32}else{0};bytes[72..76].copy_from_slice(&(flags|cap_flags).to_ne_bytes());bytes[76..80].copy_from_slice(&cap_token.to_ne_bytes());
    let normals = crate::normals::matrix(&object.model).expect("Scene placements are checked before insertion or editing");
    for (index, value) in normals.iter().enumerate() {
        bytes[80 + index * 4..84 + index * 4].copy_from_slice(&value.to_ne_bytes());
    }
    if let Some(colour)=style.colour {for (index,value) in colour[..3].iter().copied().chain([1.]).enumerate() {bytes[144+index*4..148+index*4].copy_from_slice(&value.to_ne_bytes());}}
    bytes[RANGE..RANGE+4].copy_from_slice(&index_start.to_ne_bytes());
    bytes[RANGE+4..RANGE+8].copy_from_slice(&primitive_base.to_ne_bytes());
    bytes
}

impl GpuMesh {
    pub fn upload(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, object: &Object, selected: bool) -> Self {
        Self::upload_at(device,layout,object,selected,[0.;3])
    }
    pub fn upload_at(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,object:&Object,selected:bool,anchor:[f64;3])->Self {
        let geometry=Rc::new(GpuGeometry::upload(device,Rc::clone(&object.mesh)));
        Self::with_geometry_at(device,layout,object,Style{source_faces:true,selected,owner:if object.metadata.is_locked {0}else{object.id.value()},colour:None},geometry,anchor)
    }

    pub fn with_geometry(device: &wgpu::Device, layout: &wgpu::BindGroupLayout,
        object: &Object, style:Style, geometry: Rc<GpuGeometry>) -> Self
    {
        Self::with_geometry_at(device,layout,object,style,geometry,[0.;3])
    }
    pub fn with_geometry_at(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,object:&Object,style:Style,geometry:Rc<GpuGeometry>,anchor:[f64;3])->Self {
        let bytes=settings(object,style,0,0,0,geometry.index_start,anchor);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let model_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("object placement"),
            layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: model.as_entire_binding() }],
        });
        Self { source_faces:style.source_faces,translation:object.model.m[12..15].try_into().unwrap(),anchor,geometry, model_group, id: object.id, model, previous: bytes,primitive_base:0,xray:false,cap_flags:0,cap_token:0 }
    }

    pub fn rebase(&mut self,queue:&wgpu::Queue,anchor:[f64;3])->[usize;2] {
        assert!(anchor.iter().all(|v|v.is_finite()));self.anchor=anchor;
        let bytes=crate::render_space::relative(self.translation,anchor).into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();
        if self.previous[48..60]==bytes {return [0,0];}
        self.previous[48..60].copy_from_slice(&bytes);queue.write_buffer(&self.model,48,&bytes);[1,12]
    }
    pub(crate) fn model_buffer(&self)->&wgpu::Buffer{&self.model}
    pub(crate) fn packed(&self)->Record {self.previous}
    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())&1!=0}

    pub fn set_ambient_radius(&mut self,queue:&wgpu::Queue,radius:f32){let bytes=radius.to_ne_bytes();if self.previous[RANGE+8..RANGE+12]!=bytes {self.previous[RANGE+8..RANGE+12].copy_from_slice(&bytes);queue.write_buffer(&self.model,(RANGE+8)as u64,&bytes);}}
    pub fn primitive_base(&self)->u32{self.primitive_base}
    pub fn set_primitive_base(&mut self,queue:&wgpu::Queue,base:u32)->Result<(),&'static str>{
        if base.checked_add(self.geometry.index_count/3).is_none_or(|end|end>=crate::primitive_image::CAP_START){return Err("Primitive range overlaps reserved caps");}
        if self.primitive_base!=base{
            self.previous[RANGE+4..RANGE+8].copy_from_slice(&base.to_ne_bytes());
            queue.write_buffer(&self.model,(RANGE+4)as u64,&self.previous[RANGE+4..RANGE+8]);self.primitive_base=base;
        }
        Ok(())
    }

    pub fn set_xray(&mut self,queue:&wgpu::Queue,on:bool){if self.xray==on{return;}self.xray=on;let flags=u32::from_ne_bytes(self.previous[72..76].try_into().unwrap());let flags=(flags&!32768)|if on&&flags&8192!=0{32768}else{0};self.previous[72..76].copy_from_slice(&flags.to_ne_bytes());queue.write_buffer(&self.model,72,&self.previous[72..76]);}
    pub fn closed(&self)->bool{!self.xray&&self.cap_flags&crate::solid_orientation::CLOSED!=0}
    pub fn pickable(&self)->bool{u32::from_ne_bytes(self.previous[68..72].try_into().unwrap())!=0}
    pub fn set_caps(&mut self,queue:&wgpu::Queue,flags:u32,token:u32){
        if self.cap_flags==flags&&self.cap_token==token{return;}let plane=u32::from_ne_bytes(self.previous[72..76].try_into().unwrap())&4096;self.previous[72..76].copy_from_slice(&(plane|flags|if self.xray&&flags&8192!=0{32768}else{0}).to_ne_bytes());self.previous[76..80].copy_from_slice(&token.to_ne_bytes());queue.write_buffer(&self.model,72,&self.previous[72..80]);self.cap_flags=flags;self.cap_token=token;
    }
    pub fn settings_bytes(&self) -> u64 { self.model.size() }

    pub fn update(&mut self, queue: &wgpu::Queue, object: &Object, style:Style) -> [usize; 2] {
        self.source_faces=style.source_faces;
        self.translation=object.model.m[12..15].try_into().unwrap();
        let mut next = settings(object, style,self.cap_flags|if self.xray&&self.cap_flags&8192!=0{32768}else{0},self.cap_token,self.primitive_base,self.geometry.index_start,self.anchor);
        next[RANGE+8..RANGE+12].copy_from_slice(&self.previous[RANGE+8..RANGE+12]);
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144), (144,RANGE),(RANGE,BYTES)] {
            if next[start..end] != self.previous[start..end] {
                queue.write_buffer(&self.model, start as u64, &next[start..end]);
                writes[0] += 1;
                writes[1] += end - start;
            }
        }
        self.previous = next;
        writes
    }

    pub fn bind_model(&self, pass: &mut wgpu::RenderPass<'_>) { pass.set_bind_group(1, &self.model_group, &[]); }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.bind_model(pass);
        self.geometry.draw_pulled(pass, 1);
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_bind_group(1, &self.model_group, &[]);
        self.geometry.draw(pass);
    }
}
```

### 2. `src/renderer.rs`

Instances and locked originals are not generated attribute display pieces.

Edit the existing block to match this code; keep unchanged lines:

```rust
fn style(scene:&Scene,id:ObjectId,selected:bool)->crate::gpu_mesh::Style {crate::gpu_mesh::Style {source_faces:!scene.forest.derived(id),selected,owner:crate::feature_owner::display_owner(scene,id).map_or(0,ObjectId::value),colour:scene.face_rgba(id)}}
```

### 3. `src/pick_sources.rs`

Classify the source row once, then append only its actual original faces.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzzzzzm-query-original-faces-under-contact-overlays-07.rs"
```

### 4. `src/face_pick.rs`

Do not allocate generated display-face metadata; preserve one row per mesh.

Edit the existing block to match this code; keep unchanged lines:

```rust
            if !mesh.source_faces||geometry.source.parts.triangles.is_empty() { self.rows.push(None); continue; }
```

### 5. `src/pick_encode.rs`

Ignore derived depth only for source-face queries; keep all original occluders.

Edit the existing block to match this code; keep unchanged lines:

```rust
            for mesh in meshes {if mode==crate::pick_sources::Mode::Face&&!mesh.source_faces{continue;}if !records.is_some_and(|(rows,shared)|rows.draw_with(&mut pass,mesh,shared)){pass.set_pipeline(pipeline);mesh.draw_pulled(&mut pass);}}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/global_outline_tests.rs`

Preserve the unchanged original-source uniform/reference test.

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzm-query-original-faces-under-contact-overlays-10.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/contact_feature_tests.rs`

Reuse the unchanged original contact ring validation fixture.

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzm-query-original-faces-under-contact-overlays-11.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/source_face_policy_tests.rs`

Supply actual original face GPU/source/archive, locked occlusion, instance and BRep controls.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzm-query-original-faces-under-contact-overlays-12.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/renderer.rs`

Connect optional Native validation outside the browser runtime.

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzm-query-original-faces-under-contact-overlays-13.rs" skip=27
```

**Result after `96zzzzzzm-query-original-faces-under-contact-overlays`:** Face picks resolve the original Mesh or BRep face under the contact. Beyond the host and the cursor pick radius there is no original face to select. Object picking and the contact drawing are unchanged.

## Build, Verify & Visual Result

```sh
npm --prefix ../session_tests run course -- dependencies 22.2
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 22.2
npm --prefix ../session_tests run course -- save 22.2
```

Open `http://127.0.0.1:8780/` to inspect the live WebGPU canvas, or use [checkpoint recovery](../../recovery.md).

Continue with [22.3 · Original boundary pens, joins and vertex markers](../../consolidation/96zzzzzzn-respect-original-boundary-pens.md).
