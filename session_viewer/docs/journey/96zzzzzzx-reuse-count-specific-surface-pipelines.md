# 22.5.4 · Reuse surface pipelines for each sample count

**Chapter 22 · Surface lighting, contact and display quality · Section 22.5 · Frame resources and multisampling** · [Contents](contents.md)

**Typing: 29–57 minutes.** [Estimate](typing-load.md).

Reuse the existing surface drawing code at one and four samples. Move each descriptor into a factory, make its count explicit, and retain the alternate compiled recipe so switching back does not compile it again.

## Type

Continue from [Keep resolved colours in the canvas format](96zzzzzzw-match-resolve-base-and-view-formats.md). [Save or recover your work](recovery.md).

### 1. `src/sample_pipelines.rs`

Retain only active and alternate pipeline/layout recipes for counts1 and4; never retain inactive frame images.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-01.rs"
```

### 2. `src/surface_pipelines.rs`

Declare the existing surface descriptor factory interface and pair return.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-02.rs"
```

### 3. `src/surface_pipelines.rs`

Move the exact previously typed shader constructor.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
// shader
```

</details>

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-03.rs"
```

</details>

### 4. `src/surface_pipelines.rs`

Move the exact previously typed layout and descriptor closure.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
// layout
```

</details>

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-04.rs"
```

</details>

### 5. `src/surface_pipelines.rs`

Assemble the count-specific surface inputs from one original shader. Borrow the existing caller-owned layouts. Use the existing device reference. Use the same shared geometry layout. Apply one explicit colour sample count. Use sample-qualified colour/world/normal inputs only for4x.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn pipelines(device:&wgpu::Device,format:wgpu::TextureFormat,view_layout:&wgpu::BindGroupLayout,model_layout:&wgpu::BindGroupLayout,samples:u32)->(wgpu::RenderPipeline,wgpu::RenderPipeline) {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0).into()),
        });
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
        (mesh_pipeline(crate::depth::surface(),true),mesh_pipeline(crate::depth::ink(),false))

}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn pipelines(device:&wgpu::Device,format:wgpu::TextureFormat,view_layout:&wgpu::BindGroupLayout,model_layout:&wgpu::BindGroupLayout,samples:u32)->(wgpu::RenderPipeline,wgpu::RenderPipeline) {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(sampled(crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0),samples).into()),
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"), bind_group_layouts: &[Some(view_layout), Some(model_layout),Some(&crate::clipping_gpu::layout(device)),Some(&crate::gpu_geometry::GpuGeometry::storage_layout(device))], immediate_size: 0 });
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
            multisample: wgpu::MultisampleState {count:samples,..Default::default()},
            multiview_mask: None,
            cache: None,
        });
        (mesh_pipeline(crate::depth::surface(),true),mesh_pipeline(crate::depth::ink(),false))

}

pub fn sampled(source:String,samples:u32)->String {
 if samples==1 {return source;}
 source.replace("@location(0) colour:","@location(0) @interpolate(perspective,sample) colour:")
 .replace("@location(1) world:","@location(1) @interpolate(perspective,sample) world:")
 .replace("@location(2) normal:","@location(2) @interpolate(perspective,sample) normal:")
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-05.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-06.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-07.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-08.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-09.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-10.rs"
```

</details>

### 6. `src/renderer.rs`

Remove the shader constructor after moving its exact bytes into the surface factory. Remove the descriptor closure after moving its exact bytes into the surface factory. Keep the existing ordinary and highlight surface recipes in the shared sample-count cache.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
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
```

</details>

Replace it with:

```rust
            });
            self.grid.draw(&mut pass,&self.view_group);
            pass.set_bind_group(2,&clip_group,&[]);
            pass.set_pipeline(&self.surfaces.active.0);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {if !self.instances.draw_ordinary(&mut pass,mesh){pass.set_pipeline(&self.surfaces.active.0);self.instances.bind_own(&mut pass);mesh.draw_pulled(&mut pass);}}
            if self.opacity.get()>=1.{self.instances.draw(&mut pass);}
            pass.set_pipeline(&self.surfaces.active.1); pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            self.highlight.mesh(&mut pass);
        }
```

<details>
<summary>Locate this block</summary>

```rust
    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        let anchor=self.anchor();self.highlight.set_at(&self.device,&self.pipeline.get_bind_group_layout(1),scene,target,anchor)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
```

</details>

Replace it with:

```rust
    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        let anchor=self.anchor();self.highlight.set_at(&self.device,&self.surfaces.active.0.get_bind_group_layout(1),scene,target,anchor)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
```

<details>
<summary>Locate this block</summary>

```rust
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        self.cancel_pick(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, None).expect("Empty selection");
        self.set_source_ink(scene);
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let matching = old.iter().position(|row| row.id == object.id
```

</details>

Replace it with:

```rust
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        self.cancel_pick(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.surfaces.active.0.get_bind_group_layout(1), scene, None).expect("Empty selection");
        self.set_source_ink(scene);
        let layout = self.surfaces.active.0.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let matching = old.iter().position(|row| row.id == object.id
```

<details>
<summary>Locate this block</summary>

```rust
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {surface_options:Default::default(),world_bounds:Default::default(),space:Default::default(),opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,output_format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, targets, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy,grid_anchor:[0.;3], size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode:Default::default(),pick_cursor:None };
        renderer.mesh_ink.enable_finite(&renderer.device,format,true);
```

</details>

Replace it with:

```rust
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {surface_options:Default::default(),world_bounds:Default::default(),space:Default::default(),opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,output_format,device, queue, surfaces, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, targets, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy,grid_anchor:[0.;3], size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode:Default::default(),pick_cursor:None };
        renderer.mesh_ink.enable_finite(&renderer.device,format,true);
```

<details>
<summary>Locate this block</summary>

```rust
        });
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view settings"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
```

</details>

Replace it with:

```rust
        });
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view settings"),
            layout: &surfaces.active.0.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
```

<details>
<summary>Locate this block</summary>

```rust
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
```

</details>

Replace it with:

```rust
        crate::shader_check::check(&device, crate::finite_project_source::source(), "Finite projection");
        crate::shader_check::check(&device, crate::finite_project_source::tiles(), "Finite tiles");
        crate::shader_check::check(&device, crate::finite_project_source::scan(), "Finite scan");
        let view_layout = crate::surface_view::layout(&device);
        let model_layout = crate::gpu_layout::uniform(&device, crate::object_record::BYTES as u64);
        let surfaces=crate::sample_pipelines::Variants::new(crate::surface_pipelines::pipelines(&device,format,&view_layout,&model_layout,1),1);
        let tiles=std::cell::RefCell::new(crate::tile_gpu::Tiles::new(&device,&view_layout));
        let instances=crate::instance_gpu::Lane::new(&device,&queue,format,&view_layout);
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
        let widget = crate::widget::Widget::new(&device, format);
```

<details>
<summary>Locate this block</summary>

```rust
    pub queue: wgpu::Queue,
    instances:crate::instance_gpu::Lane,
    instance_rows:std::collections::HashSet<ObjectId>,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
    highlight: crate::highlight::Highlight,
    preview: crate::tool_preview::Preview,
    widget: crate::widget::Widget,
```

</details>

Replace it with:

```rust
    pub queue: wgpu::Queue,
    instances:crate::instance_gpu::Lane,
    instance_rows:std::collections::HashSet<ObjectId>,
    surfaces:crate::sample_pipelines::Variants<(wgpu::RenderPipeline,wgpu::RenderPipeline)>,
    highlight: crate::highlight::Highlight,
    preview: crate::tool_preview::Preview,
    widget: crate::widget::Widget,
```

<details>
<summary>Complete updated block reference</summary>

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
    output_format:wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    instances:crate::instance_gpu::Lane,
    instance_rows:std::collections::HashSet<ObjectId>,
    surfaces:crate::sample_pipelines::Variants<(wgpu::RenderPipeline,wgpu::RenderPipeline)>,
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
    targets:crate::frame_targets::Targets,
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
        Self::with_formats(device, queue, format, format, scene)
    }

    pub fn with_formats(
        device: wgpu::Device,
        queue: wgpu::Queue,
        output_format: wgpu::TextureFormat,
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        crate::shader_check::check(&device, crate::clipping_gpu::source(include_str!("cloud_points.wgsl"),2,0), "Cloud points");
        crate::shader_check::check(&device, include_str!("cloud_resolve.wgsl").into(), "Cloud resolve");
        crate::shader_check::check(&device, crate::finite_project_source::source(), "Finite projection");
        crate::shader_check::check(&device, crate::finite_project_source::tiles(), "Finite tiles");
        crate::shader_check::check(&device, crate::finite_project_source::scan(), "Finite scan");
        let view_layout = crate::surface_view::layout(&device);
        let model_layout = crate::gpu_layout::uniform(&device, crate::object_record::BYTES as u64);
        let surfaces=crate::sample_pipelines::Variants::new(crate::surface_pipelines::pipelines(&device,format,&view_layout,&model_layout,1),1);
        let tiles=std::cell::RefCell::new(crate::tile_gpu::Tiles::new(&device,&view_layout));
        let instances=crate::instance_gpu::Lane::new(&device,&queue,format,&view_layout);
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
            layout: &surfaces.active.0.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        let targets = crate::frame_targets::Targets::with_formats(&device,[640,480],output_format,format,1);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
        let grid = crate::stroke_gpu::Lane::grid(&device, format);
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {surface_options:Default::default(),world_bounds:Default::default(),space:Default::default(),opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,output_format,device, queue, surfaces, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, targets, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy,grid_anchor:[0.;3], size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode:Default::default(),pick_cursor:None };
        renderer.mesh_ink.enable_finite(&renderer.device,format,true);
        for lane in [&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers]{lane.enable_finite(&renderer.device,format,false);}
        renderer.highlight.enable_finite(&renderer.device,format);
        renderer.set_scene(scene, None);
        renderer
    }

    pub fn anchor(&self)->[f64;3] {self.space.anchor()}

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        if self.size == [size.width, size.height] { return; }
        self.cancel_pick();
        self.size = [size.width, size.height];
        self.targets=crate::frame_targets::Targets::with_formats(&self.device,self.size,self.output_format,self.format,self.targets.samples);
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
        self.ambient_rows.set(scene);
        self.pyramid_revision=self.pyramid_revision.wrapping_add(1);
        self.clipping.borrow_mut().set(scene);
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        self.cancel_pick(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.surfaces.active.0.get_bind_group_layout(1), scene, None).expect("Empty selection");
        self.set_source_ink(scene);
        let layout = self.surfaces.active.0.get_bind_group_layout(1);
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
        self.mesh_ink.set_records(&self.device,std::mem::take(&mut self.sources.mesh_metadata)).expect("Original ink metadata");
        self.markers.set_points(&self.device, std::mem::take(&mut self.sources.mesh_markers).into_iter().chain(scene.points().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut marker=row.marker_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {marker.colour=colour;}marker})).chain(scene.control_markers_at(anchor).into_iter().map(|c| c.display)))
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
            let tables={let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);let mut encoder=self.device.create_command_encoder(&Default::default());tiles.encode(&self.device,&mut encoder,&self.view_group,transform);self.queue.submit([encoder.finish()]);tiles.map_report();crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.targets.primitive.view.clone()}};
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
        let anchor=self.anchor();self.highlight.set_at(&self.device,&self.surfaces.active.0.get_bind_group_layout(1),scene,target,anchor)
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
    pub fn set_ambient(&self,enabled:bool){if self.ambient_enabled.replace(enabled)==enabled{return;}let mut ambient=self.ambient.borrow_mut();if enabled{ambient.get_or_insert_with(||crate::ambient_gpu::Ambient::new(&self.device,self.format));}else if let Some(owner)=ambient.as_mut(){owner.clear();}}
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
        let output=view;let view=self.targets.color(output);
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
        let tables=crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.targets.primitive.view.clone()};for lane in [&self.mesh_ink,&self.strokes,&self.paths,&self.markers]{lane.finite_frame(&self.device,&tables,&self.targets.depth);}self.highlight.finite_frame(&self.device,&tables,&self.targets.depth);
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.targets.depth,
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
                    view:&self.targets.primitive.view,depth_slice:None,resolve_target:None,
                    ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store},
                })],
                ..Default::default()
            });
            self.grid.draw(&mut pass,&self.view_group);
            pass.set_bind_group(2,&clip_group,&[]);
            pass.set_pipeline(&self.surfaces.active.0);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {if !self.instances.draw_ordinary(&mut pass,mesh){pass.set_pipeline(&self.surfaces.active.0);self.instances.bind_own(&mut pass);mesh.draw_pulled(&mut pass);}}
            if self.opacity.get()>=1.{self.instances.draw(&mut pass);}
            pass.set_pipeline(&self.surfaces.active.1); pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            self.highlight.mesh(&mut pass);
        }
        self.caps.borrow_mut().draw(&self.device,&mut encoder,view,&self.targets.primitive.view,&self.targets.depth,&self.view_group,&clip_group,&self.meshes,if self.clipping.borrow().enabled{self.clipping.borrow().rows.len()as u32}else{0},Some(&self.instances));
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("cloud depth composite"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.targets.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});if let Some(clouds)=self.clouds.borrow().as_ref(){clouds.draw(&mut pass);}}
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.targets.depth,&self.view_group,&clip_group,&clip.buffer,&self.targets.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground_at(self.anchor()),[self.pyramid_revision,clip_revision],Some(&self.instances));}}
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite scene ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.targets.depth,depth_ops:None,stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});
            pass.set_bind_group(2,&clip_group,&[]);
            self.highlight.ink(&mut pass,&self.view_group);
            self.mesh_ink.draw(&mut pass,&self.view_group);
            self.strokes.draw(&mut pass,&self.view_group);self.sheets.draw(&mut pass);
            self.paths.draw(&mut pass,&self.view_group);
            self.markers.draw(&mut pass,&self.view_group);
            self.highlight.points(&mut pass,&self.view_group);
            self.preview.draw(&mut pass,&self.view_group);
        }
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("measurement overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.targets.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});self.measurement.draw(&mut pass,&self.view_group);self.guides.draw(&mut pass,&self.view_group);}
        if self.outlines_enabled.get()&&!self.meshes.is_empty(){
            let ctx=self.text_context();
            let mut global=self.global_outline.borrow_mut();let global=global.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));
            global.encode_masks(&ctx,&mut encoder,&self.targets.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,true,Some(&self.targets.primitive.view));
            let mut selected=self.outline.borrow_mut();
            if self.meshes.iter().any(GpuMesh::selected){let selected=selected.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));selected.encode_masks(&ctx,&mut encoder,&self.targets.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,false,Some(&self.targets.primitive.view));}else{selected.take();}
            if let Some(alpha)=global.alpha_view(){self.outline_composite.borrow_mut().get_or_insert_with(||crate::outline_composite::Composite::new(&self.device,self.format)).draw(&self.device,&mut encoder,view,alpha,selected.as_ref().and_then(|s|s.alpha_view()));}
        }else{
            self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();
            if self.meshes.iter().any(GpuMesh::selected){let mut outline=self.outline.borrow_mut();let outline=outline.get_or_insert_with(||crate::outline_gpu::Outline::new(&self.text_context(),self.format));outline.encode(&self.text_context(),&mut encoder,view,&self.targets.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision);}else{self.outline.borrow_mut().take();}
        }
        self.text_error.borrow_mut().take();
        if let Some(lane)=self.text.borrow_mut().as_mut() {
            match lane.prepare(&self.text_context(),&self.text_frame(*transform)) {
            Ok(())=>{
                let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("scene glyphs"),depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {view:&self.targets.depth,depth_ops:Some(wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view,depth_slice:None,resolve_target:None,ops:wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});if let Err(error)=lane.draw(&mut pass) {*self.text_error.borrow_mut()=Some(error.to_string());}
            }
            Err(error)=>{*self.text_error.borrow_mut()=Some(error.to_string());}
            }
        }
        self.targets.resolve(&mut encoder,output);
        self.widget.draw(&mut encoder,output);
        if let Some(pyramid)=self.pyramid.borrow_mut().as_mut(){pyramid.encode(&self.device,&self.queue,&mut encoder,&self.targets.depth,*transform,self.size,self.density,[self.pyramid_revision,clip_revision]);}
        self.queue.submit([encoder.finish()]);tiles.map_report();
    }
}

fn style(scene:&Scene,id:ObjectId,selected:bool)->crate::gpu_mesh::Style {crate::gpu_mesh::Style {source_faces:!scene.forest.derived(id),selected,owner:crate::feature_owner::display_owner(scene,id).map_or(0,ObjectId::value),colour:scene.face_rgba(id)}}

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

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="ink_metadata_tests.rs"]
mod ink_metadata_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="contact_boundary_tests.rs"]
mod contact_boundary_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="source_face_policy_tests.rs"]
mod source_face_policy_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="source_pen_tests.rs"]
mod source_pen_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="round_caps_tests.rs"]
mod round_caps_tests;
#[cfg(test)]
#[path="mesh_marker_tests.rs"]
mod mesh_marker_tests;
#[cfg(test)]
#[path="mesh_density_tests.rs"]
mod mesh_density_tests;
#[cfg(test)]
#[path="ambient_input_tests.rs"]
mod ambient_input_tests;
#[cfg(test)]
#[path="ambient_order_tests.rs"]
mod ambient_order_tests;
#[cfg(test)]
#[path="ambient_contact_tests.rs"]
mod ambient_contact_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="frame_target_tests.rs"]
mod frame_target_tests;
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-13.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-14.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-15.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-16.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-17.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-18.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-19.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-20.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-22.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-23.rs"
```

</details>

### 7. `src/instance_gpu.rs`

Extract the existing shared-surface descriptor behind the same count-specific factory interface.

<details>
<summary>Locate the existing block</summary>

```rust
impl Lane {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
fn pipelines(device:&wgpu::Device,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout,layout:&wgpu::BindGroupLayout,samples:u32)->(wgpu::RenderPipeline,wgpu::RenderPipeline) {
// descriptors
        (create(None),create(Some(wgpu::BlendState::ALPHA_BLENDING)))

}
impl Lane {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-24.rs"
```

</details>

### 8. `src/instance_gpu.rs`

Move the exact previously typed shared shader and pipeline descriptor.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
// descriptors
```

</details>

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-25.rs"
```

</details>

### 9. `src/instance_gpu.rs`

Remove the original descriptor copy after moving it. Reuse the shared pipeline pair without rebuilding source geometry, records or slots. Borrow the existing shared record layout in the extracted factory. Match the shared surface pipeline to its actual sample count.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
```

</details>

Replace it with:

```rust
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipelines.active.0);
        self.records.bind(pass);
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
```

<details>
<summary>Locate this block</summary>

```rust
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;self.ordinary=ordinary;Ok(())
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.ordinary_pipeline)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        if !self.bind_record(pass,mesh){return false;}pass.set_pipeline(pipeline);mesh.geometry.draw_pulled(pass,1);true
    }
```

</details>

Replace it with:

```rust
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;self.ordinary=ordinary;Ok(())
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.pipelines.active.1)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        if !self.bind_record(pass,mesh){return false;}pass.set_pipeline(pipeline);mesh.geometry.draw_pulled(pass,1);true
    }
```

<details>
<summary>Locate this block</summary>

```rust
 crate::clipping_gpu::source(&crate::object_rows::source(include_str!("triangle.wgsl")),2,0)
}
fn pipelines(device:&wgpu::Device,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout,layout:&wgpu::BindGroupLayout,samples:u32)->(wgpu::RenderPipeline,wgpu::RenderPipeline) {
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        (create(None),create(Some(wgpu::BlendState::ALPHA_BLENDING)))

}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=crate::object_rows::layout(device);
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let pipeline=create(None);let ordinary_pipeline=create(Some(wgpu::BlendState::ALPHA_BLENDING));
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {lookup:None,space:None,pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,meshes:&mut[GpuMesh],instances:&std::collections::HashSet<crate::scene::ObjectId>,geometry_changed:bool)->Result<(),&'static str> {
        let compatible=geometry_changed||self.space.as_ref().is_some_and(|space|meshes.iter().all(|mesh|self.ordinary.get(&mesh.id).is_some_and(|(slot,row)|*slot>0&&*slot<space.end()&&space.local_slot(*slot)==[*row,mesh.primitive_base()])));
```

</details>

Replace it with:

```rust
 crate::clipping_gpu::source(&crate::object_rows::source(include_str!("triangle.wgsl")),2,0)
}
fn pipelines(device:&wgpu::Device,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout,layout:&wgpu::BindGroupLayout,samples:u32)->(wgpu::RenderPipeline,wgpu::RenderPipeline) {
        let source=crate::surface_pipelines::sampled(source(),samples);
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None});
        (create(None),create(Some(wgpu::BlendState::ALPHA_BLENDING)))

}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=crate::object_rows::layout(device);
        let pipelines=crate::sample_pipelines::Variants::new(pipelines(device,format,view,&layout,1),1);
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {lookup:None,space:None,pipelines,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set_samples(&mut self,device:&wgpu::Device,format:wgpu::TextureFormat,samples:u32) {
        self.pipelines.select(samples,||pipelines(device,format,&crate::surface_view::layout(device),&self.layout,samples));
    }
    pub fn set(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,meshes:&mut[GpuMesh],instances:&std::collections::HashSet<crate::scene::ObjectId>,geometry_changed:bool)->Result<(),&'static str> {
        let compatible=geometry_changed||self.space.as_ref().is_some_and(|space|meshes.iter().all(|mesh|self.ordinary.get(&mesh.id).is_some_and(|(slot,row)|*slot>0&&*slot<space.end()&&space.local_slot(*slot)==[*row,mesh.primitive_base()])));
```

<details>
<summary>Locate this block</summary>

```rust
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pub(crate) lookup:Option<crate::primitive_slot_table::Table>,pub(crate) space:Option<crate::slot_space::SlotSpace>,pipeline:wgpu::RenderPipeline,ordinary_pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
```

</details>

Replace it with:

```rust
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pub(crate) lookup:Option<crate::primitive_slot_table::Table>,pub(crate) space:Option<crate::slot_space::SlotSpace>,pipelines:crate::sample_pipelines::Variants<(wgpu::RenderPipeline,wgpu::RenderPipeline)>,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::BTreeMap,rc::Rc};
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pub(crate) lookup:Option<crate::primitive_slot_table::Table>,pub(crate) space:Option<crate::slot_space::SlotSpace>,pipelines:crate::sample_pipelines::Variants<(wgpu::RenderPipeline,wgpu::RenderPipeline)>,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
 let candidates=rows.iter().filter(|(_,key,declared)|*declared||uses[key]>1).map(|(id,_,_)|*id).collect::<std::collections::HashSet<_>>();
 if candidates.len()<=limit {candidates}else{rows.into_iter().filter(|(_,_,declared)|*declared).map(|(id,_,_)|id).collect()}
}
pub(crate) fn stable_rows(previous:&std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,ids:&[crate::scene::ObjectId],limit:usize)->Result<std::collections::HashMap<crate::scene::ObjectId,u32>,&'static str> {
 let mut rows=std::collections::HashMap::new();let mut used=std::collections::HashSet::new();
 for id in ids {if let Some(&(_,row))=previous.get(id) {if row as usize>=limit||!used.insert(row){return Err("Invalid retained object row");}rows.insert(*id,row);}}
 let mut next=0usize;
 for id in ids {if rows.contains_key(id){continue;}while next<limit&&used.contains(&(next as u32)){next+=1;}if next>=limit||u32::try_from(next).is_err(){return Err("Object records exceed device limits");}let row=next as u32;rows.insert(*id,row);used.insert(row);}
 Ok(rows)
}
pub(crate) fn source()->String {
 crate::clipping_gpu::source(&crate::object_rows::source(include_str!("triangle.wgsl")),2,0)
}
fn pipelines(device:&wgpu::Device,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout,layout:&wgpu::BindGroupLayout,samples:u32)->(wgpu::RenderPipeline,wgpu::RenderPipeline) {
        let source=crate::surface_pipelines::sampled(source(),samples);
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None});
        (create(None),create(Some(wgpu::BlendState::ALPHA_BLENDING)))

}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=crate::object_rows::layout(device);
        let pipelines=crate::sample_pipelines::Variants::new(pipelines(device,format,view,&layout,1),1);
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {lookup:None,space:None,pipelines,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set_samples(&mut self,device:&wgpu::Device,format:wgpu::TextureFormat,samples:u32) {
        self.pipelines.select(samples,||pipelines(device,format,&crate::surface_view::layout(device),&self.layout,samples));
    }
    pub fn set(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,meshes:&mut[GpuMesh],instances:&std::collections::HashSet<crate::scene::ObjectId>,geometry_changed:bool)->Result<(),&'static str> {
        let compatible=geometry_changed||self.space.as_ref().is_some_and(|space|meshes.iter().all(|mesh|self.ordinary.get(&mesh.id).is_some_and(|(slot,row)|*slot>0&&*slot<space.end()&&space.local_slot(*slot)==[*row,mesh.primitive_base()])));
        let limit=crate::object_rows::capacity(device);
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<(crate::scene::ObjectId,Record)>)>=BTreeMap::new();
        for row in meshes.iter().filter(|row|instances.contains(&row.id)) {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push((row.id,row.packed()));}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let own=meshes.iter().filter(|row|!instances.contains(&row.id)).collect::<Vec<_>>();let complete=own.len()<=limit-count;let own=if complete {own}else{Vec::new()};
        let mut ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        ids.sort_unstable_by_key(|id|id.value());
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let mut change=None;let members=crate::instance_members::collect(groups.values().map(|(geometry,records)|(geometry,records.iter().map(|(id,_)|indices[id]).collect())).chain(own.iter().map(|mesh|(&mesh.geometry,vec![indices[&mesh.id]]))));
        if ids.is_empty(){self.space=None;}else if let Some((members,arena))=members.filter(|(members,arena)|complete&&compatible&&crate::instance_members::bounded(members,self.space.as_ref().and_then(|space|space.triangle_count()).unwrap_or(*arena).max(*arena))) {
            let definitions=members.iter().map(crate::instance_members::Members::definition).collect::<Vec<_>>();let space=self.space.get_or_insert_with(Default::default);let touched=space.touched(&definitions);change=Some(space.update(&definitions,&touched,arena));
        }else{self.space=None;}
        let mut batches=BTreeMap::new();let mut ordinary=std::collections::HashMap::new();let mut all_slots=Vec::new();let mut all_records=vec![[0;BYTES];indices.values().max().map_or(0,|row|*row as usize+1)];
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for (id,record) in &records {
                let row=indices[id];
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                ordinary.insert(*id,(u32::try_from(all_slots.len()).map_err(|_|"Instance slot range")?,row));
                all_records[row as usize]=*record;
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        for mesh in own {
            let first=u32::try_from(all_slots.len()+1).map_err(|_|"Ordinary slot range")?;let row=indices[&mesh.id];
            all_slots.push([row,mesh.primitive_base()]);all_records[row as usize]=mesh.packed();ordinary.insert(mesh.id,(first,row));
        }
        if let Some(space)=&self.space {let locations=space.draws().into_iter().flat_map(|draw|draw.slots).map(|slot|{let[row,base]=space.local_slot(slot);(row,(slot,base))}).collect::<std::collections::HashMap<_,_>>();for(first,row)in ordinary.values_mut(){*first=locations[&*row].0;}for batch in batches.values_mut(){batch.first_slot=locations[&all_slots[(batch.first_slot-1)as usize][0]].0;}}
        let mut dense=0u32;for mesh in meshes {let entry=ordinary.get(&mesh.id);let base=if let(Some(space),Some((first,_)))=(&self.space,entry){space.local_slot(*first)[1]}else if geometry_changed {dense}else{mesh.primitive_base()};mesh.set_primitive_base(queue,base)?;if let Some((first,row))=entry {all_records[*row as usize]=mesh.packed();if self.space.is_none(){all_slots[(*first-1)as usize]=[*row,base];}}dense=dense.checked_add(mesh.geometry.index_count/3).filter(|n|*n<crate::primitive_image::CAP_START).ok_or("Primitive range overlaps reserved caps")?;}
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
        if self.records.written_bytes>0{self.writes+=1;self.written_bytes+=self.records.written_bytes;}
        if let(Some(space),Some(change))=(&self.space,&change){let table=self.lookup.get_or_insert_with(||crate::primitive_slot_table::Table::new(&ctx));self.slots.publish(&ctx,space,change,table);let bytes=table.written_bytes.get();if bytes>0{self.writes+=1;self.written_bytes+=bytes;}}else{self.lookup=None;if all_slots.is_empty()&&self.slots.instances()>0{self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;self.ordinary=ordinary;Ok(())
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.pipelines.active.1)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        if !self.bind_record(pass,mesh){return false;}pass.set_pipeline(pipeline);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn current_row(&self,mesh:&GpuMesh)->Option<(u32,u32)> {
        let &(first,row)=self.ordinary.get(&mesh.id)?;self.records.matches(row,&mesh.packed()).then_some((first,row))
    }
    pub fn bind_record(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some((first,_))=self.current_row(mesh)else{return false};self.records.bind(pass);self.slots.bind(pass,0,first);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipelines.active.0);
        self.records.bind(pass);
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn allocation_parts(&self)->[u64;3] {[self.records.allocated_bytes(),if self.slots.instances()==0{0}else{self.slots.allocated_bytes()},self.lookup.as_ref().map_or(0,|table|table.allocated_bytes())]}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.allocation_parts().into_iter().sum(),self.writes,self.written_bytes]}
}

#[cfg(test)]
mod stable_row_tests {
 use super::*;
 #[test]
 fn retained_ids_reuse_holes_and_reject_capacity_overflow() {
  let mut scene=crate::scene::Scene::empty();let mesh=||crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap();let a=scene.insert(mesh()).unwrap();let b=scene.insert(mesh()).unwrap();let c=scene.insert(mesh()).unwrap();
  let old=std::collections::HashMap::from([(a,(1,0)),(b,(2,1)),(c,(3,2))]);let rows=stable_rows(&old,&[c,a,b],3).unwrap();assert_eq!((rows[&a],rows[&b],rows[&c]),(0,1,2));
  let d=scene.insert(mesh()).unwrap();let rows=stable_rows(&old,&[c,d,a],3).unwrap();assert_eq!((rows[&a],rows[&d],rows[&c]),(0,1,2));assert!(!rows.contains_key(&b));assert!(stable_rows(&old,&[a,b,c,d],3).is_err());assert!(stable_rows(&old,&[c],2).is_err());assert!(stable_rows(&old,&[],0).unwrap().is_empty());
 }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-26.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-27.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-28.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-29.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-30.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-31.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-32.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-33.rs"
```

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-34.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 10. `src/global_outline_tests.rs`

Keep existing assertions on the same active ordinary pipeline.

<details>
<summary>Locate the existing block</summary>

```text
r.pipeline
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-35.rs"
```

</details>

### 11. `src/lib.rs`

Register the two small pipeline recipe modules.

<details>
<summary>Locate the existing block</summary>

```rust
mod camera_navigation_tests;
```

</details>

After that block, type:

```rust

mod sample_pipelines;
mod surface_pipelines;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-36.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 12. `src/global_outline_tests.rs`

Retain all unchanged outline and arena assertions after the ordinary surface owner moves.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{scene::Scene,prepared::PreparedMesh,cap_gpu_tests::{renderer,pixels,matrix,wait}};
use std::rc::Rc;
fn width_pixels(r:&super::Renderer){let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:Some("global CSS width"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),&session_rust::Xform::identity().m.map(|v|v as f32));wait(r);}
#[test]
fn actual_global_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths=Vec::new();
    for curved in [false,true] {
        let mut scene=Scene::demo();scene.clear();let id=if curved {scene.insert_cad(Rc::new(session_rust::BRep::create_cylinder(0.4,0.8))).unwrap()} else {let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.8,0.8,0.2)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,0.5);scene.insert(prepared).unwrap()};let mut renderer=renderer(&scene);renderer.set_outlines(true);assert!(!renderer.meshes.iter().any(crate::gpu_mesh::GpuMesh::selected));let _=id;let geometry=renderer.stats();let mut css=Vec::new();
        for density in [1.,2.] {renderer.set_density(density);width_pixels(&renderer);let alpha=renderer.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));let row=&alpha[120*320..121*320];let width=row[160..].iter().map(|v|*v as f64/255.).sum::<f64>()/density as f64;assert!((0.7..1.7).contains(&width),"Actual outer silhouette CSS coverage remains bounded: {width}");css.push(width);assert_eq!(&renderer.stats()[..2],&geometry[..2]);}
        assert!((css[0]-css[1]).abs()<0.5,"DPI keeps the outline in CSS units: {css:?}");widths.push(css);
    }
    for i in 0..2 {assert!((widths[0][i]-widths[1][i]).abs()<0.25,"A smooth cylinder and flat box use one coverage width: {widths:?}");}
}

#[test]
fn actual_global_and_selected_outlines_share_alpha_and_one_black_blend(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();
 let original=scene.objects()[0].geometry().unwrap().to_proto();let mut r=renderer(&scene);let plain=pixels(&r);let uploads=r.stats()[0];r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().filter(|v|**v>0).count()>300);let usage=r.global_outline_usage();assert_eq!(pixels(&r),global);assert_eq!(r.global_outline_usage()[1],usage[1]);assert!(r.global_outline_usage()[2]>usage[2]);
 r.select_objects(&scene,&[owner]).unwrap();let both=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"same source and CSS width yield identical masks");
 let mut boundary=0;for(i,(&coverage,(a,b)))in alpha.iter().zip(global.chunks_exact(4).zip(both.chunks_exact(4))).enumerate(){if coverage>0&&plain[i*4..i*4+3]==[255,255,255]{boundary+=1;assert_eq!(a,b,"selected plus global uses max alpha in one blend");}}
 assert!(boundary>200);assert_eq!(r.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
 r.request_pick([0.,0.],matrix()).unwrap();wait(&r);let hit=r.take_pick(matrix()).expect("mapped original pick").unwrap();assert_eq!(hit.unwrap()[0],owner.value());
 r.select_objects(&scene,&[]).unwrap();r.set_outlines(false);assert_eq!(r.global_outline_usage(),[0;4]);assert_eq!(pixels(&r),plain);
 scene.clear();r.set_scene(&scene,None);r.set_outlines(true);pixels(&r);assert_eq!(r.global_outline_usage(),[0;4]);
}
#[test]
fn actual_visible_cap_coverage_keeps_selected_and_global_masks_equal(){
 let(mut scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=renderer(&scene);r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));
 r.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"selected visible cap marker joins original coverage");
 for(i,v)in alpha.iter().enumerate(){if *v==255{assert!(global[i*4..i*4+3].iter().all(|v|*v<12));assert!(selected[i*4..i*4+3].iter().all(|v|*v<12));}}
 r.set_clipping(false,true);pixels(&r);assert_eq!(r.global_outline_alpha(),alpha,"horizontal cut and uncut original share the same outer XY silhouette");assert_eq!(r.cap_usage()[0],16);
 let side=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Yz,&[[0.,0.,0.]],0.9).unwrap();crate::plane_original::insert(&mut scene,Rc::new(side)).unwrap();r.set_scene(&scene,Some(owner));r.set_clipping(true,true);pixels(&r);assert_ne!(r.global_outline_alpha(),alpha,"changed section inputs cannot reuse stale coverage");
}
#[test]
fn actual_global_outline_includes_locked_and_releases_hidden_originals(){
 let mut scene=Scene::empty();let mut source=crate::cap_gpu_tests::source(false);source.is_locked=true;let owner=scene.insert(PreparedMesh::new(source).unwrap()).unwrap();let mut r=renderer(&scene);r.set_outlines(true);pixels(&r);assert!(r.global_outline_alpha().iter().any(|v|*v==255));assert_eq!(scene.pick_id(owner.value()),None);
 Rc::make_mut(&mut scene.view).hidden.insert(owner);r.set_scene(&scene,None);pixels(&r);assert!(r.global_outline_alpha().is_empty());assert_eq!(r.global_outline_usage(),[0;4]);
}

#[test]
fn actual_triangle_ranges_keep_selection_updates_visibility_and_original_owners(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    let bases=|gpu:&super::Renderer|gpu.meshes.iter().map(|row|u32::from_ne_bytes(row.packed()[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())).collect::<Vec<_>>();assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes.iter().all(|row|!row.selected()));let baseline=pixels(&gpu);
    gpu.select_objects(&scene,&[b]).unwrap();assert_eq!(bases(&gpu),vec![1,2]);assert_eq!(gpu.meshes.iter().map(|row|row.selected()).collect::<Vec<_>>(),vec![false,true]);let selected=pixels(&gpu);assert_ne!(selected,baseline);
    scene.translate(b,[0.,0.1,0.]).unwrap();gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes[1].selected());for(position,owner)in[([-0.5,-0.1],a),([0.5,0.],b)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
    let before=gpu.meshes[1].packed();assert!(gpu.meshes[1].set_primitive_base(&gpu.queue,crate::primitive_image::CAP_START).is_err());assert_eq!(gpu.meshes[1].packed(),before);
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1]);assert_eq!(gpu.meshes[0].id,b);assert!(gpu.meshes[0].selected());pixels(&gpu);
}

fn primitive_pixels(gpu:&super::Renderer,texture:&wgpu::Texture)->Vec<u32>{
        let width=texture.width();let height=texture.height();let pitch=(width*4+255)/256*256;
        let read=gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("actual primitive addresses"),size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=gpu.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},texture.size());gpu.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());wait(gpu);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let values=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize*4].chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap()))).collect();drop(bytes);read.unmap();values
    }
#[test]
fn actual_primitive_mrt_keeps_placed_intervals_and_overlay_addresses(){

    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    for density in [1.,2.]{gpu.set_density(density);pixels(&gpu);let before=primitive_pixels(&gpu,&gpu.targets.primitive.texture);assert_eq!(before[0],0);assert_eq!(before[120*320+80],2);assert_eq!(before[120*320+240],3);assert_eq!(gpu.targets.primitive.texture.format(),crate::primitive_image::FORMAT);
        let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner:a,part:crate::pick_sources::Part::Face(key)})).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),before,"A derived selected-face mesh must retain original triangle addresses");
        gpu.set_highlight(&scene,None).unwrap();
    }
    let mut instances=Scene::empty();crate::scene_document::import(&mut instances,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();gpu.set_scene(&instances,None);pixels(&gpu);let placed=primitive_pixels(&gpu,&gpu.targets.primitive.texture);let ids=gpu.meshes.iter().map(|mesh|mesh.primitive_base()+1).collect::<Vec<_>>();assert_eq!(ids.len(),2);assert_ne!(ids[0],ids[1]);assert!(ids.iter().all(|id|*id>0&&*id<crate::primitive_image::CAP_START));assert_eq!(placed[120*320+64],ids[0]);assert_eq!(placed[120*320+256],ids[1]);
    gpu.resize(crate::viewport::Viewport{width:640,height:480});assert_eq!([gpu.targets.primitive.texture.width(),gpu.targets.primitive.texture.height()],[640,480]);gpu.resize(crate::viewport::Viewport{width:320,height:240});gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert!(primitive_pixels(&gpu,&gpu.targets.primitive.texture).iter().all(|id|*id==0));
}

#[test]
fn actual_first_pick_primitives_match_current_cropped_owners_before_color(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    assert!(primitive_pixels(&gpu,&gpu.targets.primitive.texture).iter().all(|word|*word==0));
    for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
        for(position,owner,primitive)in[([-0.5,0.],a,2u32),([0.5,0.],b,3)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();assert_eq!([texture.width(),texture.height()],window.draw[2..]);assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],primitive);}
    }
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,None);gpu.request_pick([0.5,0.],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],b.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],2,"Visible survivor uses its current retained run without a color frame");
}

fn cap_primitive_frame(gpu:&super::Renderer){
 let texture=gpu.device.create_texture(&wgpu::TextureDescriptor{label:Some("cap primitive fixture"),size:wgpu::Extent3d{width:gpu.size[0],height:gpu.size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});
 gpu.draw(&texture.create_view(&Default::default()),&Default::default(),&matrix());wait(gpu);
}
#[test]
fn actual_cap_color_primitives_keep_plane_selection_and_empty_pixels(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});cap_primitive_frame(&gpu);
  let words=primitive_pixels(&gpu,&gpu.targets.primitive.texture);let at=(96*density*320*density+208*density)as usize;
  assert_eq!(words[at],0xfffffff0);assert_eq!(words[0],0);
  gpu.select_objects(&scene,&[owner]).unwrap();cap_primitive_frame(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture)[at],0xfffffff1);
  gpu.select_objects(&scene,&[]).unwrap();
 }
 gpu.set_scene(&Scene::empty(),None);cap_primitive_frame(&gpu);assert!(primitive_pixels(&gpu,&gpu.targets.primitive.texture).iter().all(|word|*word==0));
}

#[test]
fn actual_first_cap_pick_has_reserved_primitive_and_exact_original_owner(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
  assert!(primitive_pixels(&gpu,&gpu.targets.primitive.texture).iter().all(|word|*word==0));
  gpu.request_pick([0.3,0.2],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
  let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let words=primitive_pixels(&gpu,&texture);
  let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];
  assert_eq!(words[(y*texture.width()+x)as usize],0xfffffff0);
  assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);
 }
}

#[test]
fn actual_compact_surface_upload_uses_twenty_bytes_per_vertex_and_releases(){
 let scene=Scene::demo();let mut gpu=renderer(&scene);let source=scene.objects()[0].mesh.clone();let original=source.vertices().to_vec();
 let buffers=gpu.meshes[0].geometry.buffers();
 assert_eq!(gpu.meshes[0].geometry.allocated_bytes(),0);
 assert_eq!(gpu.usage()[1],buffers.0.size()+buffers.1.size(),"One arena owner accounts for the actual shared buffers");
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let mut cache=crate::geometry_cache::GeometryCache::default();let uploaded=cache.get(&ctx,&source);
 assert_eq!(uploaded.buffers().0.size(),source.vertices().len()as u64*20);
 assert_eq!(cache.uploaded_bytes,cache.allocated_bytes(),"The first nonempty compact upload matches actual GPU allocation");
 let written=cache.uploaded_bytes;cache.get(&ctx,&source);assert_eq!(cache.uploads,1);assert_eq!(cache.uploaded_bytes,written);
 let empty=crate::gpu_geometry::GpuGeometry::upload(&gpu.device,std::rc::Rc::new(crate::mesh::Mesh::new(vec![],vec![]).unwrap()));assert_eq!(empty.allocated_bytes(),24);
 pixels(&gpu);assert_eq!(source.vertices(),original);let uploads=gpu.stats();pixels(&gpu);assert_eq!(gpu.stats(),uploads);
 gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.arena_cpu_bytes(),0);
}
#[test]
fn actual_nonzero_shared_ranges_keep_surface_face_picks_projection_outlines_and_caps() {
    use std::cell::RefCell;
    use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh,text_gpu_support::GpuCtx};
    for clipped in [false,true] {
        let scene=if clipped {crate::cap_gpu_tests::scene(false).0} else {
            let mut scene=Scene::empty();scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();scene
        };
        let mut r=renderer(&scene);r.set_outlines(true);
        let expected=pixels(&r);
        let mut hits=Vec::new();
        for mode in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face] {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            hits.push(r.take_pick(matrix()).unwrap().unwrap());
        }
        assert!(hits.iter().all(Option::is_some));
        if !clipped {assert_ne!(hits[1].unwrap()[1],0,"The uncut source returns an original face address");}
        let ctx=GpuCtx::new(&r.device,&r.queue);
        let arena=Rc::new(RefCell::new(crate::mesh_arena::Arena::new(&ctx)));
        let decoy=Rc::new(crate::mesh::Mesh::new(vec![[0.7,0.7,-0.6,0.,1.,0.],[0.9,0.7,-0.6,0.,1.,0.],[0.8,0.9,-0.6,0.,1.,0.]],vec![0,1,2]).unwrap());
        arena.borrow_mut().upload(&ctx,&decoy).unwrap();
        let layout=r.surfaces.active.0.get_bind_group_layout(1);
        for mesh in &mut r.meshes {
            let object=scene.objects().iter().find(|object|object.id==mesh.id).unwrap();
            let geometry=Rc::new(GpuGeometry::shared(&ctx,object.mesh.clone(),arena.clone()).unwrap());
            assert!(geometry.index_start>=3);
            *mesh=GpuMesh::with_geometry(&r.device,&layout,object,super::style(&scene,object.id,false),geometry);
        }
        r.caps.borrow_mut().set_scene(&r.queue,&scene,&mut r.meshes,&r.clipping.borrow().rows);
        let mut base=0;for mesh in &mut r.meshes {mesh.set_primitive_base(&r.queue,base).unwrap();base+=mesh.geometry.index_count/3;}
        r.tiles.borrow_mut().set(&r.device,&r.meshes);r.faces.clear();
        if let Some(outline)=r.global_outline.borrow_mut().as_mut(){outline.invalidate();}
        assert_eq!(pixels(&r),expected,"Nonzero shared ranges preserve actual color, clipping and outline pixels");
        for (mode,expected) in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face].into_iter().zip(hits) {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            assert_eq!(r.take_pick(matrix()).unwrap().unwrap(),expected,"Shared source addressing preserves original picks");
        }
        assert!(r.tiles.borrow().usage()[3]>0);
        if clipped {assert_eq!(r.cap_usage()[1],1);}
    }
}

#[test]
fn actual_contact_marker_is_opaque_unlit_selected_and_keeps_original_ownership(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();
 let vertices=prepared.display.vertices().len();prepared.display=prepared.display.with_alphas(vec![1./255.;vertices]).unwrap();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);
 let source=scene.objects()[0].mesh.clone();let color=source.vertices()[0][3..6].to_vec();let expected=color.iter().map(|v|(v.clamp(0.,1.)*255.).round()as u8).collect::<Vec<_>>();
 let uploads=gpu.stats()[0];for opacity in [1.,0.5,0.] {gpu.set_opacity(opacity);let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],expected.as_slice());gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
 gpu.select_objects(&scene,&[owner]).unwrap();let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],&[255,255,0]);
 assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);assert_eq!(gpu.stats()[0],uploads);assert_eq!(source.alphas(),vec![1./255.;vertices]);
 scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn current_palette_is_shared_by_surface_shader_and_preserves_original_picks_and_color(){
 use wgpu::naga;
 let rgba=crate::colour::selection();assert_eq!(rgba,session_rust::Color::yellow().to_unified_array());assert_eq!(rgba,[1.,1.,0.,1.]);
 let shader=crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0);assert!(shader.contains(&crate::colour::selection_shader()));naga::valid::Validator::new(naga::valid::ValidationFlags::all(),naga::valid::Capabilities::all()).validate(&naga::front::wgsl::parse_str(&shader).unwrap()).unwrap();
 let mut scene=Scene::empty();let prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let plain=pixels(&gpu);let uploads=gpu.stats()[0];
 gpu.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&gpu);assert!(selected.chunks_exact(4).any(|p|p[0]>10&&p[0]==p[1]&&p[2]==0));assert_ne!(plain,selected);gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
 gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),plain);assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn full_primitive_base_survives_surface_selection_and_reserved_range_checks(){
 assert!(include_str!("clip.wgsl").contains(&format!("const CAP_PRIMITIVE: u32 = 0x{:08x}u;",crate::primitive_image::CAP_START)));
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.4],[0.4,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let mut gpu=renderer(&scene);let original=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 for base in [0x8000_0000,crate::primitive_image::CAP_START-2]{
  gpu.meshes[0].set_primitive_base(&gpu.queue,base).unwrap();pixels(&gpu);let plain=primitive_pixels(&gpu,&gpu.targets.primitive.texture);assert_eq!(plain[120*320+160],base+1);
  gpu.select_objects(&scene,&[owner]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),plain);assert_eq!(u32::from_ne_bytes(gpu.meshes[0].packed()[64..68].try_into().unwrap()),1);
  gpu.select_objects(&scene,&[]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),plain);let before=gpu.meshes[0].packed();for invalid in [crate::primitive_image::CAP_START-1,crate::primitive_image::CAP_START,u32::MAX]{assert!(gpu.meshes[0].set_primitive_base(&gpu.queue,invalid).is_err());assert_eq!(gpu.meshes[0].packed(),before);}
 }
 assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}

#[test]
fn actual_ordinary_surfaces_keep_one_own_row_slot_through_instances_highlights_and_release(){
 let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=[crate::draw_slots::OWN_ROW,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert!(gpu.instances.slots.allocated_bytes()>=8);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);let originals=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let owner=scene.objects()[0].id;
 for opacity in [1.,0.5]{gpu.set_opacity(opacity);let plain=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.targets.primitive.texture);let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::Face(key)})).unwrap();assert_ne!(pixels(&gpu),plain);assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),addresses);gpu.set_highlight(&scene,None).unwrap();assert_eq!(pixels(&gpu),plain);}
 assert_eq!(scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>(),originals);gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);
}

#[test]
fn actual_surface_batches_share_one_slot_owner_and_restore_ordinary_drawing(){
 use prost::Message;let mut source=crate::instance_specimen::message();let mut second=source.definitions.as_ref().unwrap().meshes[0].clone();second.guid="other-definition".into();source.definitions.as_mut().unwrap().meshes.push(second);source.objects.as_mut().unwrap().instances[1].definition_guid="other-definition".into();source.objects.as_mut().unwrap().instances[0].xform=None;source.objects.as_mut().unwrap().instances[1].flags=0;source.xforms=vec![session_rust::proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-0.6,0.,0.).to_proto())},session_rust::proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(0.6,0.,0.).to_proto())}];
 let mut scene=crate::scene::Scene::empty();crate::scene_document::import(&mut scene,&source.encode_to_vec(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=crate::cap_gpu_tests::renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(&gpu.instance_usage()[..3],&[2,2,16760]);assert_eq!(gpu.instance_allocations(),[352,24,16384]);assert_eq!(gpu.instances.slots.instances(),2);let slots=crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,24);assert_eq!(&slots[..8],&[u32::MAX,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(u32::from_ne_bytes(slots[8..12].try_into().unwrap()),0);assert_eq!(u32::from_ne_bytes(slots[16..20].try_into().unwrap()),1);
 let records=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),2*crate::object_record::BYTES as u64);
 let mut actual=records.chunks_exact(crate::object_record::BYTES).map(|r|r.to_vec()).collect::<Vec<_>>();actual.sort_unstable();let mut expected=gpu.meshes.iter().map(|m|m.packed().to_vec()).collect::<Vec<_>>();expected.sort_unstable();assert_eq!(actual,expected,"global records retain both original placements, IDs and ranges");
 let baseline=crate::cap_gpu_tests::pixels(&gpu);
 for half in 0..2 {let blue=baseline.chunks_exact(4).enumerate().filter(|(i,p)|(*i%320)/160==half&&p[2]>p[0].saturating_add(50)&&p[2]>p[1].saturating_add(30)).count();assert!(blue>100,"placed original blue triangles in half {half}: {blue}");}
 let uploads=gpu.stats()[0];let owner=scene.objects()[0].id;gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 gpu.set_opacity(0.5);crate::cap_gpu_tests::pixels(&gpu);gpu.set_opacity(1.);assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.set_scene(&crate::scene::Scene::empty(),None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_projected_records_follow_physical_gaps_clear_old_ranges_and_bound_fallback(){
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[-0.4,-0.4,0.],[0.4,-0.4,0.],[0.,0.4,0.]],[0.2,0.4,0.8]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);let baseline=pixels(&gpu);let triangles=gpu.meshes[0].geometry.index_count/3;assert_eq!(gpu.meshes.len(),1);let initial=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],u64::from(triangles)*96);let uploads=gpu.stats()[0];
 gpu.meshes[0].set_primitive_base(&gpu.queue,7).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+7));let projected=gpu.tile_buffers()[0].clone();let data=crate::growing_buffer_tests::read(&ctx,&projected,u64::from(triangles+7)*96);assert!(data[..7*96].iter().all(|v|*v==0));assert_eq!(&data[7*96..],initial);let projections=gpu.tile_usage()[5];pixels(&gpu);assert_eq!(gpu.tile_usage()[5],projections);
 gpu.meshes[0].set_primitive_base(&gpu.queue,3).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+3));assert_eq!(gpu.tile_buffers()[0],projected);let data=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],projected.size());assert!(data[..3*96].iter().all(|v|*v==0));assert_eq!(&data[3*96..(triangles as usize+3)*96],initial);assert!(data[(triangles as usize+3)*96..].iter().all(|v|*v==0));
 gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles));let current=gpu.tile_buffers()[0].clone();assert_eq!(current.size(),initial.len()as u64);assert_eq!(crate::growing_buffer_tests::read(&ctx,&current,current.size()),initial);assert_eq!(gpu.stats()[0],uploads);
 gpu.meshes[0].set_primitive_base(&gpu.queue,0x80000000).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);pixels(&gpu);assert!(gpu.tiles.borrow().fallback);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
 gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_ordinary_storage_records_match_legacy_pixels_and_primitives_through_growth_selection_and_release() {
 let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(gpu.instances.ordinary.len(),1);assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64),gpu.meshes[0].packed());let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.targets.primitive.texture);
 gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),addresses);gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),1);
 let b=scene.insert(PreparedMesh::triangle([[-0.2,-0.2,-0.4],[0.2,-0.2,-0.4],[0.,0.2,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();scene.translate(b,[0.6,0.,0.]).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),2);let wanted=gpu.meshes.iter().flat_map(|m|m.packed()).collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),wanted.len()as u64),wanted);
 let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.targets.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[a]).unwrap();assert_ne!(pixels(&gpu),stored);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);gpu.set_opacity(0.5);pixels(&gpu);gpu.set_opacity(1.);assert_eq!(pixels(&gpu),stored);
 for opacity in [0.,0.5,0.95,1.] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.targets.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact ordinary blend at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),addresses);gpu.set_scene(&scene,None);}
 let copy=scene.copy_row(b,false).unwrap();scene.translate(copy,[0.3,0.1,0.]).unwrap();let source=scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),3);let uploads=gpu.stats()[0];
 for opacity in [0.,0.5,0.95] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.targets.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact transparent placements at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[copy]).unwrap();if opacity>0. {assert_ne!(pixels(&gpu),stored);}gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);assert_eq!(gpu.stats()[0],uploads);}
 assert_eq!(scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto(),source);gpu.set_opacity(1.);gpu.set_scene(&scene,None);pixels(&gpu);
 let location=|gpu:&super::Renderer,id:crate::scene::ObjectId|{let space=gpu.instances.space.as_ref().unwrap();let row=gpu.instances.ordinary[&id].1;space.draws().into_iter().flat_map(|draw|draw.slots).find(|slot|space.local_slot(*slot)[0]==row).unwrap()};let first_slot=location(&gpu,a);
 let retained=gpu.instances.ordinary.clone();assert!(scene.remove(b));gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 let fresh=scene.insert(PreparedMesh::triangle([[0.7,-0.3,-0.4],[0.9,-0.3,-0.4],[0.8,-0.1,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&fresh].1,retained[&b].1);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 for mesh in &gpu.meshes {let row=gpu.instances.ordinary[&mesh.id].1;let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),(row as u64+1)*crate::object_record::BYTES as u64);assert_eq!(&bytes[row as usize*crate::object_record::BYTES..],&mesh.packed());}pixels(&gpu);
 let projected=gpu.tiles.borrow().projected.clone();let expected=crate::growing_buffer_tests::read(&ctx,&projected,projected.size());let image=pixels(&gpu);gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),image);let uniform=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&uniform,uniform.size()),expected);gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);let restored=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&restored,restored.size()),expected);
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());assert!(gpu.instances.space.is_none());assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_lookup_projection_groups_reuse_current_inputs_and_replace_output_bindings() {
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let count=gpu.meshes[0].primitive_base()+1;assert_eq!(count,2);
 let mut state=crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).unwrap();assert_eq!(state.bytes(),16);assert_eq!(crate::growing_buffer_tests::read(&ctx,&state.parameters,16),[2u32,1,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
 let pipes=crate::tile_pipelines::Pipelines::new(&gpu.device,&gpu.pipeline.get_bind_group_layout(0));let output=|size|gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("lookup projection output witness"),size,usage:wgpu::BufferUsages::STORAGE,mapped_at_creation:false});let first=output(192);let group=state.group(&gpu.device,&pipes.lookup_project_layout,&first);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&first),group);let larger=output(384);let next=state.group(&gpu.device,&pipes.lookup_project_layout,&larger);assert_ne!(next,group);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&larger),next);
 assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,None,count).is_none());assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());gpu.instances.ordinary.clear();assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).is_none());scene.clear();gpu.set_scene(&scene,None);assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());
}

#[test]
fn actual_lookup_projection_matches_legacy_bytes_keeps_selection_cache_and_releases() {
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let other=scene.insert(PreparedMesh::triangle([[0.1,0.1,-0.3],[0.3,0.1,-0.3],[0.2,0.3,-0.3]],[0.1,0.4,0.9]).unwrap()).unwrap();let copy=scene.copy_row(other,false).unwrap();scene.translate(copy,[0.4,0.,0.]).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(0.95);gpu.set_scene(&scene,None);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 let image=pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_some());let output=gpu.tiles.borrow().projected.clone();let wanted=crate::growing_buffer_tests::read(&ctx,&output,output.size());let uploads=gpu.stats()[0];gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(pixels(&gpu),image);let output=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&output,output.size()),wanted);
 gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);assert!(gpu.tiles.borrow().lookup.is_some());let projections=gpu.tile_usage()[5];gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);assert_eq!(gpu.stats()[0],uploads);
 scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_arena_reclaims_history_retained_sources_and_keeps_survivor_pixels_and_owners(){
 let left=[[-0.8,-0.4,-0.4],[-0.4,-0.4,-0.4],[-0.6,0.4,-0.4]];let right=[[0.4,-0.4,-0.4],[0.8,-0.4,-0.4],[0.6,0.4,-0.4]];let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle(left,[0.9,0.1,0.1]).unwrap()).unwrap();let b=scene.insert(PreparedMesh::triangle(right,[0.1,0.8,0.2]).unwrap()).unwrap();let source=Rc::clone(&scene.objects()[0].mesh);let precise=scene.objects()[0].geometry().unwrap().to_proto();let mut history=crate::history::History::default();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let first=pixels(&gpu);let expired=Rc::downgrade(&gpu.meshes[0].geometry);let survivor=gpu.meshes[1].geometry.index_start;assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(2,144));
 history.edit(&mut scene,|scene|{assert!(scene.remove(a));});gpu.set_scene(&scene,None);assert!(expired.upgrade().is_none());assert!(history.scenes().any(|saved|saved.contains(a)));assert!(Rc::strong_count(&source)>0);let mut c=None;history.edit(&mut scene,|scene|{c=Some(scene.insert(PreparedMesh::triangle(left,[0.1,0.3,0.9]).unwrap()).unwrap());});let c=c.unwrap();gpu.set_scene(&scene,None);let reused=gpu.meshes.iter().find(|mesh|mesh.id==c).unwrap();assert_eq!(reused.geometry.index_start,0);assert_eq!(gpu.meshes.iter().find(|mesh|mesh.id==b).unwrap().geometry.index_start,survivor);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(3,216));let next=pixels(&gpu);let l=(120*320+64)*4;let r=(120*320+256)*4;assert!(next[l+2]>next[l].saturating_add(50));assert_eq!(&next[r..r+4],&first[r..r+4]);for(point,id)in[([-0.6,0.],c),([0.6,0.],b)]{gpu.request_pick(point,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],id.value());}
 assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(Rc::ptr_eq(&source,&scene.objects().iter().find(|row|row.id==a).unwrap().mesh));assert_eq!(scene.objects().iter().find(|row|row.id==a).unwrap().geometry().unwrap().to_proto(),precise);assert_eq!(pixels(&gpu),first);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(4,288));scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert_eq!(gpu.geometry.entries(),0);assert_eq!(gpu.geometry.allocated_bytes(),0);assert_eq!(gpu.usage(),[0;4]);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);assert!(history.scenes().count()>0);
}

#[test]
fn actual_precise_object_rebase_matches_local_pixels_records_selection_and_owners(){
 let mut local=Scene::empty();let owner=local.insert(PreparedMesh::triangle([[-0.2,-0.3,-0.4],[0.2,-0.3,-0.4],[0.,0.3,-0.4]],[0.1,0.3,0.9]).unwrap()).unwrap();local.translate(owner,[0.125,-0.25,0.]).unwrap();let mut gpu=renderer(&local);gpu.set_opacity(1.);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();gpu.markers.set_points(&gpu.device,std::iter::empty()).unwrap();let baseline=pixels(&gpu);assert!(baseline.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(50)).count()>100);let source=local.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 let anchor=[1e12,-1e12,1e12];let mut far=local.clone();far.translate(owner,anchor).unwrap();gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();gpu.markers.set_points(&gpu.device,std::iter::empty()).unwrap();assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[1,12]);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64);assert_eq!(&bytes[48..60],&[0.125f32,-0.25,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(&bytes[..crate::object_record::BYTES],&gpu.meshes[0].packed());
 gpu.select_objects(&far,&[owner]).unwrap();assert_ne!(pixels(&gpu),baseline);gpu.request_pick([0.125,-0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());gpu.select_objects(&far,&[]).unwrap();assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);
 far.translate(owner,[0.001,0.,0.]).unwrap();gpu.set_scene(&far,None);let expected=crate::render_space::relative(far.objects()[0].model.m[12..15].try_into().unwrap(),anchor);assert_eq!(&gpu.meshes[0].packed()[48..60],&expected.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert!(expected[0]>0.125);assert_eq!(gpu.stats()[0],uploads);
}

#[test]
fn actual_derived_frame_origin_keeps_cap_pixels_world_text_clips_and_precise_ground(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.5,0.5,0.5)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,-0.5);let owner=scene.insert(prepared).unwrap();let plane=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Xy,&[[0.,0.,-0.5]],1.).unwrap();let plane_id=crate::plane_original::insert(&mut scene,Rc::new(plane)).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();gpu.markers.set_points(&gpu.device,std::iter::empty()).unwrap();let baseline=pixels(&gpu);assert!(crate::cap_gpu_tests::at(&baseline,160,120)[..3].iter().all(|v|(150..=156).contains(v)));let ground=gpu.ambient_rows.ground_at([0.;3]);let source=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [owner,plane_id]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();gpu.markers.set_points(&gpu.device,std::iter::empty()).unwrap();assert!(gpu.space.update(anchor,1.,0.).unwrap().moved);for mesh in &mut gpu.meshes {mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(gpu.anchor(),anchor);assert_eq!(gpu.ambient_rows.ground_at(anchor),ground);assert_eq!(gpu.ambient_rows.ground[0]-anchor[2],-0.75);let frame=gpu.text_frame(matrix());assert_eq!(frame.origin,anchor);assert!(!frame.cut([anchor[0],anchor[1],anchor[2]-0.75]));assert!(frame.cut([anchor[0],anchor[1],anchor[2]-0.25]));assert_eq!(pixels(&gpu),baseline);let writes=gpu.clipping.borrow().writes;assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.clipping.borrow().writes,writes);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);
}

#[test]
fn actual_cloud_and_sheet_origins_reuse_uploads_and_invalidate_equal_matrix_views(){
 let mut scene=Scene::empty();let mut source=session_rust::PointCloud::from_coords(vec![-0.5,0.,-0.5],vec![255,0,0,255],vec![]);source.point_size=16.;let source=Rc::new(source);let cloud=scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();let sheet=Rc::new(session_rust::proto::Sheet{guid:crate::sheet_example::source().guid,coords:vec![0.25,0.,-0.5,0.75,0.,-0.5],widths:vec![0.0625],colors:vec![0xff00_0000],segment_count:1,entity_count:1,source_ids:vec![0],..Default::default()});let drawing=scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::new(sheet.clone()).unwrap())).unwrap();let original=source.to_proto();let mut gpu=renderer(&scene);let mut camera=crate::camera_defaults_tests::top_fixture();controlled_translation_plane(&mut camera,[0.;3]);camera.distance=2.;camera.aspect=4./3.;camera.projection=crate::camera::Projection::Orthographic;gpu.cloud_camera(&camera);let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(baseline.chunks_exact(4).any(|p|p[0]>200&&p[1]<20&&p[2]<20));assert!(baseline.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<20)).count()>20);let uploads=(gpu.cloud_usage()[2],gpu.sheet_usage()[3]);assert_eq!(uploads,(1,1));
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [cloud,drawing]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);assert!(gpu.space.update(anchor,2.,0.).unwrap().moved);let mut far_camera=crate::camera_defaults_tests::top_fixture();controlled_translation_plane(&mut far_camera,[0.;3]);far_camera.distance=camera.distance;far_camera.aspect=camera.aspect;far_camera.projection=camera.projection;far_camera.target=anchor;controlled_translation_plane(&mut far_camera,anchor);gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,anchor).unwrap();let local=far_camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);let prepared=gpu.cloud_usage()[3];assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!(gpu.cloud_usage()[3],prepared);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];far_camera.target=moved;assert!(gpu.space.update(moved,2.,300.).unwrap().moved);gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,moved).unwrap();let same_matrix=far_camera.view_projection_anchored(moved).to_f32();assert_eq!(same_matrix,local);let panned=crate::cap_gpu_tests::pixels_at(&gpu,same_matrix);assert!(panned.chunks_exact(4).all(|p|p==&[255;4]));assert!(gpu.cloud_usage()[3]>prepared,"Origin alone invalidates the equal-matrix cloud view");assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);
 far_camera.target=anchor;controlled_translation_plane(&mut far_camera,anchor);assert!(gpu.space.update(anchor,2.,600.).unwrap().moved);gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,anchor).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);for(point,id)in[([-0.5,0.,-0.5],cloud),([0.5,0.,-0.5],drawing)]{let point=camera.view_projection().transform_point(&session_rust::Point::new(point[0],point[1],point[2]));gpu.request_pick([point[0]as f32,point[1]as f32],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap().unwrap()[0],id.value());}assert_eq!(source.to_proto(),original);assert!(Rc::ptr_eq(&far.sheets()[0].prepared.original,&sheet));assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.cloud_usage(),[0;5]);assert_eq!(&gpu.sheet_usage()[..2],&[0,0]);
}

fn temporary_inputs(gpu:&mut super::Renderer,world:[f64;3]){
 let point=|p:[f64;3]|std::array::from_fn(|i|p[i]+world[i]);let mut guides=crate::guide_gpu::Overlay::at(gpu.anchor());guides.wire(&[point([-0.75,0.25,-0.5]),point([0.75,0.25,-0.5])],[210./255.,40./255.,40./255.,1.],true);guides.mark(point([0.5,0.25,-0.5]),6.,[0.,0.,1.,1.]);guides.label=Some((point([0.5,0.25,-0.5]),"guide".into()));gpu.set_guides(&guides).unwrap();let mark=crate::measure_mark::Mark{points:vec![point([-0.5,-0.25,-0.5]),point([0.5,-0.25,-0.5])],label:"distance".into()};gpu.set_measurement(Some(&mark)).unwrap();let mut draft=crate::tool::Draft::new(crate::tool::Shape::Line);draft.submit(point([0.,0.,-0.5])).unwrap();draft.hover(Some(point([0.5,0.,-0.5]))).unwrap();gpu.set_preview(Some(&draft)).unwrap();
}
#[test]
fn actual_temporary_ink_origins_restore_pixels_invalidate_caches_and_release(){
 let scene=Scene::empty();let mut gpu=renderer(&scene);let mut camera=crate::camera_defaults_tests::top_fixture();controlled_translation_plane(&mut camera,[0.;3]);camera.projection=crate::camera::Projection::Orthographic;camera.distance=2.;camera.aspect=4./3.;let transform=camera.uniform();temporary_inputs(&mut gpu,[0.;3]);let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(baseline.chunks_exact(4).any(|p|p[0]>200&&p[1]>200&&p[2]<30));assert!(baseline.chunks_exact(4).any(|p|p[0]>150&&p[1]<80&&p[2]<80));assert!(baseline.chunks_exact(4).filter(|p|p[..3]!=[255;3]).count()>200);let bytes=(gpu.guide_bytes(),gpu.preview_bytes(),gpu.measurement.bytes());assert!(bytes.0>0&&bytes.1>0&&bytes.2>0);assert_eq!(gpu.text_error(),None);
 let anchor=[1e9,-1e9,1e9];assert!(gpu.space.update(anchor,2.,0.).unwrap().moved);camera.target=anchor;controlled_translation_plane(&mut camera,anchor);temporary_inputs(&mut gpu,anchor);let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!((gpu.guide_bytes(),gpu.preview_bytes(),gpu.measurement.bytes()),bytes);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];camera.target=moved;assert!(gpu.space.update(moved,2.,300.).unwrap().moved);temporary_inputs(&mut gpu,anchor);assert_eq!(camera.view_projection_anchored(moved).to_f32(),local);assert!(crate::cap_gpu_tests::pixels_at(&gpu,local).chunks_exact(4).all(|p|p==&[255;4]));camera.target=anchor;controlled_translation_plane(&mut camera,anchor);assert!(gpu.space.update(anchor,2.,600.).unwrap().moved);temporary_inputs(&mut gpu,anchor);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert!(scene.all_ids().is_empty());gpu.request_pick([0.,0.],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap(),None);
 gpu.set_preview(None).unwrap();gpu.set_measurement(None).unwrap();gpu.set_guides(&crate::guide_gpu::Overlay::at(anchor)).unwrap();crate::cap_gpu_tests::pixels_at(&gpu,local);assert_eq!(gpu.preview_bytes(),0);assert_eq!(gpu.guide_bytes(),0);assert_eq!(gpu.measurement.bytes(),0);assert_eq!(gpu.text_usage()[0],0);
}

#[test]
fn actual_source_grid_origin_cache_keeps_surface_pixels_owners_and_release(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();let mut gpu=renderer(&scene);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();gpu.markers.set_points(&gpu.device,std::iter::empty()).unwrap();let settings=Some(crate::grid::Settings{spacing:0.1,half_cells:5});let plane=|origin|crate::cplane::CPlane::Frame{origin,x:[1.,0.,0.],y:[0.,1.,0.]};gpu.set_grid_on(settings,plane([0.,0.,-0.75])).unwrap();let mut camera=crate::camera_defaults_tests::top_fixture();controlled_translation_plane(&mut camera,[0.;3]);camera.projection=crate::camera::Projection::Orthographic;camera.distance=2.;camera.aspect=4./3.;let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);let uploads=gpu.stats()[0];let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();far.translate(owner,anchor).unwrap();gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();gpu.markers.set_points(&gpu.device,std::iter::empty()).unwrap();gpu.space.update(anchor,2.,0.).unwrap();for mesh in &mut gpu.meshes{mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));let far_plane=plane([anchor[0],anchor[1],anchor[2]-0.75]);gpu.set_grid_on(settings,far_plane).unwrap();camera.target=anchor;controlled_translation_plane(&mut camera,anchor);let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);let grid_uploads=gpu.grid_usage()[2];gpu.set_grid_on(settings,far_plane).unwrap();assert_eq!(gpu.grid_usage()[2],grid_uploads);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];gpu.space.update(moved,2.,300.).unwrap();camera.target=moved;for mesh in &mut gpu.meshes{mesh.rebase(&gpu.queue,moved);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_grid_on(settings,far_plane).unwrap();assert!(gpu.grid_usage()[2]>grid_uploads);assert_eq!(camera.view_projection_anchored(moved).to_f32(),local);assert!(crate::cap_gpu_tests::pixels_at(&gpu,local).chunks_exact(4).all(|p|p==&[255;4]));gpu.space.update(anchor,2.,600.).unwrap();for mesh in &mut gpu.meshes{mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_grid_on(settings,far_plane).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);gpu.request_pick([0.,0.],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap().unwrap()[0],owner.value());assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);gpu.set_grid(None).unwrap();gpu.set_scene(&Scene::empty(),None);assert_eq!(&gpu.grid_usage()[..2],&[0,0]);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_initial_object_records_keep_precise_local_pixels_updates_and_sources(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.2,-0.25,-0.5],[0.3,-0.25,-0.5],[0.1,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();let mut gpu=renderer(&scene);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let layout=gpu.pipeline.get_bind_group_layout(1);gpu.meshes[0]=crate::gpu_mesh::GpuMesh::upload(&gpu.device,&layout,&scene.objects()[0],false);gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));let mut camera=crate::camera_defaults_tests::top_fixture();controlled_translation_plane(&mut camera,[0.;3]);camera.projection=crate::camera::Projection::Orthographic;camera.distance=2.;camera.aspect=4./3.;let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);let original=gpu.meshes[0].packed();let uploads=gpu.stats()[0];let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();far.translate(owner,anchor).unwrap();let layout=gpu.pipeline.get_bind_group_layout(1);gpu.meshes[0]=crate::gpu_mesh::GpuMesh::upload_at(&gpu.device,&layout,&far.objects()[0],false,anchor);assert_eq!(gpu.meshes[0].packed(),original);let geometry=gpu.meshes[0].geometry.clone();let style=crate::gpu_mesh::Style{source_faces:true,selected:false,owner:owner.value(),colour:None};gpu.meshes[0]=crate::gpu_mesh::GpuMesh::with_geometry_at(&gpu.device,&layout,&far.objects()[0],style,geometry.clone(),anchor);assert_eq!(gpu.meshes[0].packed(),original);assert!(Rc::ptr_eq(&gpu.meshes[0].geometry,&geometry));gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));camera.target=anchor;controlled_translation_plane(&mut camera,anchor);let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!(gpu.meshes[0].update(&gpu.queue,&far.objects()[0],style),[0,0]);let selected=crate::gpu_mesh::Style{selected:true,..style};assert_eq!(gpu.meshes[0].update(&gpu.queue,&far.objects()[0],selected),[1,16]);assert_eq!(&gpu.meshes[0].packed()[..64],&original[..64]);assert_eq!(gpu.meshes[0].update(&gpu.queue,&far.objects()[0],style),[1,16]);assert_eq!(gpu.meshes[0].packed(),original);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);assert_eq!(gpu.stats()[0],uploads);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_original_highlights_keep_local_pixels_addresses_and_sources_after_rebasing(){
 use crate::pick_sources::{Part,Target};let mut scene=Scene::empty();let mesh=scene.insert(PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let line=scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(-0.4,0.5,-0.5,0.4,0.5,-0.5))).unwrap()).unwrap();let path=scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(session_rust::Polyline::new(vec![session_rust::Point::new(-0.4,-0.5,-0.5),session_rust::Point::new(0.,-0.3,-0.5),session_rust::Point::new(0.4,-0.5,-0.5)]))).unwrap()).unwrap();let point=scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(0.75,0.,-0.5))).unwrap()).unwrap();let mut proto=session_rust::PointCloud::from_coords(vec![-0.75,0.,-0.5],vec![],vec![]).to_proto();proto.point_ids=vec![u32::MAX];let cloud=scene.insert_cloud(crate::cloud::Prepared::new(Rc::new(session_rust::PointCloud::from_proto(proto))).unwrap()).unwrap();scene.owner_controls(path).unwrap();scene.cloud_controls(Some(cloud));let curve=scene.insert_path(crate::chain::PreparedChain::curve(Rc::new(crate::freeform_curve::curve(&[session_rust::Point::new(-0.4,0.8,-0.5),session_rust::Point::new(0.,0.65,-0.5),session_rust::Point::new(0.4,0.8,-0.5)]))).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();let edge=scene.objects()[0].mesh.parts.edges[0];let control=scene.control_markers().into_iter().find(|c|c.parent==path).unwrap().part;let targets=[Target{owner:mesh,part:Part::Face(key)},Target{owner:mesh,part:Part::MeshEdge(edge.0,edge.1)},Target{owner:line,part:Part::Curve},Target{owner:curve,part:Part::Curve},Target{owner:path,part:Part::PathEdge(0)},Target{owner:path,part:control},Target{owner:cloud,part:Part::CloudPoint(u32::MAX)}];let mut gpu=renderer(&scene);let mut camera=crate::camera_defaults_tests::top_fixture();controlled_translation_plane(&mut camera,[0.;3]);camera.projection=crate::camera::Projection::Orthographic;camera.distance=3.;camera.aspect=4./3.;gpu.cloud_camera(&camera);let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);let uploads=gpu.stats()[0];let addresses=primitive_pixels(&gpu,&gpu.targets.primitive.texture);let mut pixels=Vec::new();for target in targets{assert!(target.valid(&scene));gpu.set_highlight(&scene,Some(target)).unwrap();let selected=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(selected!=baseline,"A valid original highlight must change actual pixels: {target:?}");assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),addresses);pixels.push(selected);}gpu.set_highlight(&scene,None).unwrap();gpu.select_objects(&scene,&[line,path,point]).unwrap();let objects=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert_ne!(objects,baseline);gpu.select_objects(&scene,&[]).unwrap();gpu.set_highlight(&scene,None).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,transform),baseline);
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [mesh,line,path,point,cloud,curve]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);gpu.space.update(anchor,3.,0.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_source_ink(&far);gpu.set_clouds(&far,&[]).unwrap();camera.target=anchor;controlled_translation_plane(&mut camera,anchor);gpu.cloud_camera(&camera);let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);
 gpu.mesh_ink.set(&gpu.device,crate::pick_sources::Sources::anchored(&far,anchor).unwrap().mesh_ink).unwrap();for(target,expected)in targets.into_iter().zip(pixels){assert!(target.valid(&far));gpu.set_highlight(&far,Some(target)).unwrap();let actual=crate::cap_gpu_tests::pixels_at(&gpu,local);assert!(actual==expected,"Distant original highlight pixels must match exactly: {target:?}; differing bytes: {}",actual.iter().zip(&expected).filter(|(a,b)|a!=b).count());assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),addresses);}gpu.set_highlight(&far,None).unwrap();gpu.select_objects(&far,&[line,path,point]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),objects);gpu.select_objects(&far,&[]).unwrap();gpu.set_highlight(&far,None).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_distant_widgets_keep_local_pixels_scale_picks_and_tile_release(){
 let scene=Scene::empty();let anchor=[1e12,-1e12,1e12];let pivot=[0.125,0.25,-0.5];let mut renderer=renderer(&scene);for projection in [crate::camera::Projection::Perspective,crate::camera::Projection::Orthographic]{for density in [1u32,2]{let mut camera=crate::camera_defaults_tests::top_fixture();controlled_translation_plane(&mut camera,[0.;3]);camera.isometric();camera.projection=projection;camera.aspect=4./3.;let size=[320*density,240*density];renderer.resize(crate::viewport::Viewport{width:size[0],height:size[1]});renderer.set_density(density as f32);renderer.space.forget();let scale=crate::widget_layout::camera_scale(&camera,pivot,240.).unwrap();renderer.set_gizmo(&camera,Some(pivot),-1.);let usage=renderer.gizmo_usage();let extent=renderer.gizmo_extent();let transform=camera.uniform();let baseline=crate::widget_tests::pixels(&mut renderer,&transform,size);assert!(baseline.chunks_exact(4).any(|p|p[..3]!=[255;3]));renderer.space.update(anchor,3.,0.).unwrap();camera.target=anchor;controlled_translation_plane(&mut camera,anchor);let distant=std::array::from_fn(|i|anchor[i]+pivot[i]);assert_eq!(crate::widget_layout::camera_scale(&camera,distant,240.).unwrap(),scale);renderer.set_gizmo(&camera,Some(distant),-1.);let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);let actual=crate::widget_tests::pixels(&mut renderer,&local,size);assert!(actual==baseline,"Precise distant widget pixels differ at density {density}");assert_eq!(renderer.gizmo_extent(),extent);assert_eq!(renderer.gizmo_usage()[2],usage[2]);renderer.request_pick([0.,0.],local).unwrap();crate::cap_gpu_tests::wait(&renderer);assert!(renderer.take_pick(local).unwrap().unwrap().is_none());renderer.set_gizmo(&camera,None,-1.);assert_eq!(renderer.gizmo_usage()[1],0);assert!(renderer.gizmo_extent().is_none());}}
}

#[test]
fn actual_origin_only_pick_views_discard_old_hits_and_restore_owners(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.5],[0.4,-0.4,-0.5],[0.,0.4,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();let mut gpu=renderer(&scene);let transform=matrix();let uploads=gpu.stats()[0];gpu.request_pick([0.,0.],transform).unwrap();wait(&gpu);gpu.space.update([2000.,0.,0.],3.,0.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,gpu.space.anchor());}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert!(gpu.take_pick(transform).is_none(),"Identical matrix in a new origin must not accept the old owner");assert_eq!(&gpu.pick_usage()[..2],&[1,0]);wait(&gpu);assert!(gpu.take_pick(transform).unwrap().unwrap().is_none(),"The automatic current-frame retry is empty");gpu.request_pick([0.,0.],transform).unwrap();wait(&gpu);assert!(gpu.take_pick(transform).unwrap().unwrap().is_none());gpu.space.update([0.;3],3.,300.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,[0.;3]);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.request_pick([0.,0.],transform).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(transform).unwrap().unwrap().unwrap()[0],owner.value());assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),source);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_rebased_pick_finishes_a_released_drag_without_reuploading_sources(){
 use crate::editor::{Editor,Action,Change};let mut editor=crate::camera_defaults_tests::editor_fixture();editor.scene.clear();let owner=editor.scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.5],[0.4,-0.4,-0.5],[0.,0.4,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();editor.scene.translate(owner,[2000.,0.,0.]).unwrap();editor.camera.target=[2000.,0.,0.];let original=editor.scene.placement(owner).unwrap().m;let source=editor.scene.objects()[0].geometry().unwrap().clone();let mut gpu=renderer(&editor.scene);let uploads=gpu.stats()[0];let transform=editor.camera.uniform();let size=[320.,240.];editor.apply(Action::CanvasStart{from:[0.,0.],to:[0.2,0.],size,modified:false,pointer:1}).unwrap();gpu.request_pick([0.,0.],transform).unwrap();editor.apply(Action::CanvasRelease{screen:[0.4,0.],size,add:false,remove:false}).unwrap();assert_eq!(editor.scene.placement(owner).unwrap().m,original);gpu.space.update(editor.camera.target,editor.camera.distance,0.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,editor.camera.target);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));let local=editor.camera.view_projection_anchored(gpu.anchor()).to_f32();assert_ne!(local,transform);wait(&gpu);assert!(gpu.take_pick(local).is_none());assert_eq!(&gpu.pick_usage()[..2],&[1,0]);assert_eq!(editor.scene.placement(owner).unwrap().m,original);wait(&gpu);let hit=gpu.take_pick(local).unwrap().unwrap().and_then(|pixel|gpu.resolve_pick(&editor.scene,pixel));assert_eq!(hit.unwrap().owner,owner);assert_eq!(editor.apply(Action::CanvasHit(hit.map(|target|target.owner))).unwrap(),Change::Scene);assert_ne!(editor.scene.placement(owner).unwrap().m,original);assert!(Rc::ptr_eq(&source,editor.scene.objects()[0].geometry().unwrap()));gpu.set_scene(&editor.scene,editor.selected);assert_eq!(gpu.stats()[0],uploads);editor.apply(Action::Undo).unwrap();assert_eq!(editor.scene.placement(owner).unwrap().m,original);
}
#[test]
fn actual_rebased_pending_cursor_stays_newest_and_cancellation_never_revives_it(){
 let mut scene=Scene::empty();let make=||PreparedMesh::triangle([[-0.18,-0.18,-0.5],[0.18,-0.18,-0.5],[0.,0.18,-0.5]],[0.2,0.4,0.8]).unwrap();let a=scene.insert(make()).unwrap();let b=scene.insert(make()).unwrap();scene.translate(a,[1999.5,0.,0.]).unwrap();scene.translate(b,[2000.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);let mut old=matrix();old[12]=-2000.;let local=matrix();gpu.request_pick([-0.5,0.],old).unwrap();gpu.request_pick([0.5,0.],old).unwrap();assert_eq!(&gpu.pick_usage()[..2],&[1,1]);gpu.space.update([2000.,0.,0.],3.,0.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,[2000.,0.,0.]);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));wait(&gpu);assert!(gpu.take_pick(local).is_none());assert_eq!(&gpu.pick_usage()[..2],&[1,0]);wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap().unwrap()[0],b.value());assert_eq!(gpu.pick_usage()[3],2);assert!(gpu.take_pick(local).is_none());gpu.request_pick([-0.5,0.],local).unwrap();gpu.cancel_pick();gpu.space.update([0.;3],3.,300.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,[0.;3]);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));wait(&gpu);assert!(gpu.take_pick(old).is_none());assert_eq!(&gpu.pick_usage()[..2],&[0,0]);gpu.request_pick([-0.5,0.],old).unwrap();gpu.set_scene(&Scene::empty(),None);gpu.space.update([2000.,0.,0.],3.,600.).unwrap();wait(&gpu);assert!(gpu.take_pick(local).is_none());assert_eq!(&gpu.pick_usage()[..2],&[0,0]);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_origin_refresh_reuses_selected_face_geometry_and_original_ink(){
 use crate::pick_sources::{Part,Target};let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();scene.translate(owner,[2000.,0.,0.]).unwrap();let original=scene.objects()[0].geometry().unwrap().to_proto();let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();let target=Some(Target{owner,part:Part::Face(key)});let mut gpu=renderer(&scene);let mut old=matrix();old[12]=-2000.;gpu.set_highlight(&scene,target).unwrap();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,old);let addresses=primitive_pixels(&gpu,&gpu.targets.primitive.texture);let geometry=gpu.highlight.geometry_for_test().unwrap().clone();let uploads=gpu.stats()[0];gpu.space.update([2000.,0.,0.],3.,0.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,[2000.,0.,0.]);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_source_ink(&scene);assert!(gpu.highlight.rebase_face(&gpu.queue,gpu.anchor(),target));assert!(Rc::ptr_eq(gpu.highlight.geometry_for_test().unwrap(),&geometry));let actual=crate::cap_gpu_tests::pixels_at(&gpu,matrix());assert!(actual==baseline,"Rebased original face and ink pixels must match exactly");assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),addresses);assert!(gpu.highlight.rebase_face(&gpu.queue,gpu.anchor(),target));assert!(Rc::ptr_eq(gpu.highlight.geometry_for_test().unwrap(),&geometry));assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);drop(geometry);gpu.set_highlight(&scene,None).unwrap();assert!(gpu.highlight.geometry_for_test().is_none());assert!(!gpu.highlight.rebase_face(&gpu.queue,gpu.anchor(),target));gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_shared_origin_keeps_all_original_lanes_and_pending_picks_coherent(){
 let mut scene=Scene::empty();let mesh=scene.insert(PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let line=scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(-0.5,0.5,-0.5,0.5,0.5,-0.5))).unwrap()).unwrap();let path=scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(session_rust::Polyline::new(vec![session_rust::Point::new(-0.5,-0.5,-0.5),session_rust::Point::new(0.,-0.25,-0.5),session_rust::Point::new(0.5,-0.5,-0.5)]))).unwrap()).unwrap();let point=scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(0.75,0.,-0.5))).unwrap()).unwrap();let cloud=scene.insert_cloud(crate::cloud::Prepared::new(Rc::new(session_rust::PointCloud::from_coords(vec![-0.75,0.,-0.5],vec![255,0,0,255],vec![]))).unwrap()).unwrap();let sheet=Rc::new(session_rust::proto::Sheet{guid:crate::sheet_example::source().guid,coords:vec![0.25,0.25,-0.5,0.75,0.25,-0.5],widths:vec![0.0625],colors:vec![0xff00_0000],segment_count:1,entity_count:1,source_ids:vec![0],..Default::default()});let drawing=scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::new(sheet.clone()).unwrap())).unwrap();let text_source=Rc::new(crate::scene_label::example(crate::text::TextPlacement::Anchor{world:[-0.5,0.75,-0.5],offset:[0.;2]}));let text=scene.insert_text(crate::scene_label::Prepared::new(text_source.clone()).unwrap()).unwrap();let ids=[line,path,point,cloud,drawing,text];let original=scene.objects()[0].geometry().unwrap().to_proto();let mut camera=crate::camera_defaults_tests::top_fixture();controlled_translation_plane(&mut camera,[0.;3]);camera.projection=crate::camera::Projection::Orthographic;camera.distance=3.;camera.aspect=4./3.;let mut gpu=renderer(&scene);gpu.select_objects(&scene,&ids).unwrap();let settings=Some(crate::grid::Settings{spacing:0.125,half_cells:4});let plane=|origin|crate::cplane::CPlane::Frame{origin,x:[1.,0.,0.],y:[0.,1.,0.]};gpu.set_grid_on(settings,plane([0.,0.,-0.75])).unwrap();assert!(!gpu.prepare_origin(&camera,&scene,None,&ids,0.).unwrap().moved);let transform=gpu.camera_matrix(&camera);let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(baseline.chunks_exact(4).any(|p|p[..3]!=[255;3]));let ground=gpu.ambient_rows.ground_at([0.;3]);let uploads=gpu.stats()[0];let anchor=[2000.,-2000.,2000.];let mut far=scene.clone();for id in [mesh,line,path,point,cloud,drawing,text]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);camera.target=anchor;controlled_translation_plane(&mut camera,anchor);gpu.set_grid_on(settings,plane([anchor[0],anchor[1],anchor[2]-0.75])).unwrap();let frame=gpu.prepare_origin(&camera,&far,None,&ids,300.).unwrap();assert!(frame.moved&&!frame.pending);assert_eq!(gpu.camera_matrix(&camera),transform);let actual=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(actual==baseline,"Shared original lanes must retain exact local/distant pixels");assert_eq!(gpu.ambient_rows.ground_at(anchor),ground);let writes=gpu.stats();assert!(!gpu.prepare_origin(&camera,&far,None,&ids,301.).unwrap().moved);assert_eq!(gpu.stats(),writes);gpu.request_pick([0.,0.],transform).unwrap();camera.target[0]+=2000.;let pending=gpu.prepare_origin(&camera,&far,None,&ids,350.).unwrap();assert!(!pending.moved&&pending.pending);assert_eq!(gpu.rebase_delay(350.),Some(151));assert_eq!(&gpu.pick_usage()[..2],&[1,0]);let moved=gpu.prepare_origin(&camera,&far,None,&ids,501.).unwrap();assert!(moved.moved&&!moved.pending);assert_eq!(gpu.camera_matrix(&camera),transform);assert_eq!(&gpu.pick_usage()[..2],&[1,0]);wait(&gpu);assert!(gpu.take_pick(transform).is_none());assert_eq!(&gpu.pick_usage()[..2],&[1,0]);wait(&gpu);assert!(gpu.take_pick(transform).unwrap().unwrap().is_none());camera.target=anchor;controlled_translation_plane(&mut camera,anchor);assert!(gpu.prepare_origin(&camera,&far,None,&ids,802.).unwrap().moved);let restored=crate::cap_gpu_tests::pixels_at(&gpu,gpu.camera_matrix(&camera));assert!(restored==baseline,"Restored shared origin must restore exact pixels");assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),original);assert!(Rc::ptr_eq(&far.texts()[0].prepared.source,&text_source));assert!(Rc::ptr_eq(&far.sheets()[0].prepared.original,&sheet));gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}

fn controlled_translation_plane(camera:&mut crate::camera::Camera,origin:[f64;3]){camera.grid_plane=Some(crate::cplane::CPlane::Frame{origin,x:[1.,0.,0.],y:[0.,1.,0.]});camera.radius=20000.;}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-37.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 13. `src/sample_surface_tests.rs`

Render actual ordinary and shared surfaces at both counts, inspect every physical depth/address, require exact return and cached pipeline identity, preserve original records and GPU geometry, then freshly query both original shared owners.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-38.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 14. `src/sample_test_proof.rs`

Export actual Native diagnostic pixels only when the caller opts into a proof directory; run all assertions without filesystem output by default.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-39.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 15. `src/renderer.rs`

Register the actual Native surface witness.

<details>
<summary>Locate the existing block</summary>

```text
nt_contact_tests.rs"]
mod ambient_contact_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="frame_target_tests.rs"]
mod frame_target_tests;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-40.rs" skip=150
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 16. `src/lib.rs`

Keep optional diagnostic exports Native-only.

<details>
<summary>Locate the existing block</summary>

```text
ach_tests;

#[cfg(test)]
mod camera_navigation_tests;

mod sample_pipelines;
mod surface_pipelines;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-41.rs" skip=100
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 22.5.4
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Ordinary and shared surface pipelines have matching one- and four-sample recipes with cached return switches. Source geometry, records, uploads and independent single-sample original-owner queries keep their existing roles. The public main frame still uses one sample.

### Native surface check

These actual GPU pictures show only the surface pass: two shared blue triangles and one ordinary green triangle. The left triangle reaches the viewport boundary. Compare silhouette coverage at one and four samples; the complete browser frame remains at one sample here.

**One sample — actual Native surface pass.**

![Actual Native isolated surface pass at one sample](../screenshots/journey/review/96zzzzzzx-reuse-count-specific-surface-pipelines-surface-one.png)

**Four samples — actual Native surface pass.**

![Actual Native isolated surface pass at four samples](../screenshots/journey/review/96zzzzzzx-reuse-count-specific-surface-pipelines-surface-four.png)

**Verified checkpoint in Chrome.**

**Critical browser-scene review accepted.**

Actual single-sample browser contact scene after extracting cached ordinary and shared surface recipes. Both densities retain original contact shading, differently scaled source instances, fresh original face picks, full sources and archives, geometry uploads and exact resize restoration. This browser picture is not a complete four-sample-frame demonstration. Two separate actual Native pictures show the isolated surface pass at 1x and 4x. Complete four-sample consumer integration, remaining view controls, runtime efficiency, Cargo independence and final whole-course visual validation remain unfinished. Broader facing and NURBS quality review is still required.

![Actual single-sample browser contact scene after extracting cached ordinary and shared surface recipes. Both densities retain original contact shading, differently scaled source instances, fresh original face picks, full sources and archives, geometry uploads and exact resize restoration. This browser picture is not a complete four-sample-frame demonstration. Two separate actual Native pictures show the isolated surface pass at 1x and 4x. Complete four-sample consumer integration, remaining view controls, runtime efficiency, Cargo independence and final whole-course visual validation remain unfinished. Broader facing and NURBS quality review is still required.](../screenshots/journey/96zzzzzzt-keep-contact-ambient-inputs-on-originals-browser.png)

[Verification scope](release.md).

This reference image is shared with [22.4.3 · Keep contact ambient inputs on originals](96zzzzzzt-keep-contact-ambient-inputs-on-originals.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

A small generic owner keeps the active pipeline pair, an optional alternate pair and their current count. Repeating the same count returns immediately; the first change creates the other recipe and later changes swap the two. It retains pipelines and their layouts, not inactive colour, depth or address images.

Move the exact existing ordinary surface shader constructor and descriptor closure before changing their inputs. The shared-instance factory follows the same pattern. Both factories use the requested multisample count. Only the four-sample shader qualifies interpolated colour, world position and normal at the actual sample; the single-sample source remains unchanged.

Ordinary and shared placements continue using the original GPU geometry, object records, slots and source owners. No tessellation or record upload is needed to change a pipeline recipe. The main renderer stays at one sample here: ink, caps, drawing sheets, text and other consumers must be connected before a complete four-sample frame is safe.

Existing surface descriptor → count-specific shader inputs → lazy pipeline pair → unchanged original geometry and records.



Why retain alternate pipelines but release inactive frame images?

Pipeline recipes can be reused across many frames and extents. Full-sized colour, depth and address images scale with the viewport; only the currently active coherent image set is required.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Run the supplied isolated surface witness. Compare ordinary uniform drawing with the same original shared records at four samples, inspect physical depth and integer addresses, and switch4→1→4. The returned pipeline identities and every returned colour pixel must match without rebuilding geometry.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 22.5.4
npm --prefix ../session_tests run course -- save 22.5.4
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The existing ordinary and shared surface descriptors move into count-specific factories. One small pipeline owner keeps the active pair and at most one alternate pair; it never retains inactive frame images. Only four-sample colour/world/normal inputs use sample interpolation. The main browser remains single-sample until all consumers are connected. The fresh actual Native isolated surface pass checks ordinary/shared colour equality, 364 mixed physical depth/address pixels, cached pipeline identity, exact 4→1→4 pixels, unchanged original records and GPU geometry, and fresh single-sample original-owner queries. Both Native surface images were individually inspected and appear in this lesson. All 993 Native tests, WASM/optimized builds, frame, canonical Chrome original-contact/resize checks and stored evidence pass; ten additional current canonical Native/browser pictures were individually inspected. Complete four-sample consumer integration, remaining view controls, runtime efficiency, Cargo independence and final whole-course visual validation remain unfinished. Broader facing and NURBS quality review is still required.

The new actual GPU witness checks silhouette changes, ordinary/shared colour equality, independently read physical sample depth and addresses, exact return pixels, cached pipeline identity, complete original mesh records, retained GPU geometry and fresh original-owner queries. Existing frame, resize and outline assertions remain intact.

Actual single-sample browser contact scene after extracting cached ordinary and shared surface recipes. Both densities retain original contact shading, differently scaled source instances, fresh original face picks, full sources and archives, geometry uploads and exact resize restoration. This browser picture is not a complete four-sample-frame demonstration. Two separate actual Native pictures show the isolated surface pass at 1x and 4x. Complete four-sample consumer integration, remaining view controls, runtime efficiency, Cargo independence and final whole-course visual validation remain unfinished. Broader facing and NURBS quality review is still required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 22.5.4
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
