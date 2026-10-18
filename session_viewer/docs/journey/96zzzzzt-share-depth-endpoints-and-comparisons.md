# 21.4.1 · Share depth endpoints and comparisons

**Chapter 21 · Precise camera and browser coordinates · Section 21.4 · Reversed depth and near clipping** · [Contents](contents.md)

**Typing: 27–53 minutes.** [Estimate](typing-load.md).

Give frame depth state one explicit owner before changing its convention.

## Type

Continue from [Adopt the shared origin in the browser](96zzzzzs-adopt-the-shared-origin-in-the-browser.md). [Save or recover your work](recovery.md).

### 1. `src/depth.rs`

Own the endpoints and the two comparison predicates. Reuse the shared strict or equal comparison in this pipeline.

<details>
<summary>Locate the existing block</summary>

```rust
pub const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

pub fn surface() -> wgpu::DepthStencilState { state(true, wgpu::CompareFunction::Less) }
pub fn ink() -> wgpu::DepthStencilState { state(false, wgpu::CompareFunction::LessEqual) }
pub fn grid() -> wgpu::DepthStencilState { state(false, wgpu::CompareFunction::Less) }

fn state(write: bool, compare: wgpu::CompareFunction) -> wgpu::DepthStencilState {
    wgpu::DepthStencilState { format: FORMAT, depth_write_enabled: Some(write), depth_compare: Some(compare),
        stencil: Default::default(), bias: Default::default() }
}

pub fn state_for_ambient()->wgpu::DepthStencilState{state(false,wgpu::CompareFunction::Equal)}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
pub const NEAR:f64=0.;
pub const FAR:f64=1.;
pub const CLOSER:wgpu::CompareFunction=wgpu::CompareFunction::Less;
pub const CLOSER_EQUAL:wgpu::CompareFunction=wgpu::CompareFunction::LessEqual;

pub fn surface() -> wgpu::DepthStencilState { state(true, crate::depth::CLOSER) }
pub fn ink() -> wgpu::DepthStencilState { state(false, crate::depth::CLOSER_EQUAL) }
pub fn grid() -> wgpu::DepthStencilState { state(false, crate::depth::CLOSER) }

fn state(write: bool, compare: wgpu::CompareFunction) -> wgpu::DepthStencilState {
    wgpu::DepthStencilState { format: FORMAT, depth_write_enabled: Some(write), depth_compare: Some(compare),
        stencil: Default::default(), bias: Default::default() }
}

pub fn state_for_ambient()->wgpu::DepthStencilState{state(false,wgpu::CompareFunction::Equal)}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzt-share-depth-endpoints-and-comparisons-01.rs"
```

```text
--8<-- "journey/code/96zzzzzt-share-depth-endpoints-and-comparisons-02.rs"
```

</details>

### 2. `src/text_plate.rs`

Reuse the shared strict or equal comparison in this pipeline.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(false),
                depth_compare: Some(wgpu::CompareFunction::LessEqual),
                stencil: Default::default(),
                bias: Default::default(),
            }),
```

</details>

Replace it with:

```rust
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(false),
                depth_compare: Some(crate::depth::CLOSER_EQUAL),
                stencil: Default::default(),
                bias: Default::default(),
            }),
```

<details>
<summary>Complete updated block reference</summary>

```rust
--8<-- "journey/code/96zzzzzt-share-depth-endpoints-and-comparisons-03.rs"
```

</details>

### 3. `src/widget_pipelines.rs`

Reuse the shared strict or equal comparison in this pipeline.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn texture_layout(device:&wgpu::Device)->wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label:Some("handle tile"),entries:&[
            wgpu::BindGroupLayoutEntry { binding:0,visibility:wgpu::ShaderStages::FRAGMENT,ty:wgpu::BindingType::Texture { sample_type:wgpu::TextureSampleType::Float {filterable:true},view_dimension:wgpu::TextureViewDimension::D2,multisampled:false },count:None },
            wgpu::BindGroupLayoutEntry { binding:1,visibility:wgpu::ShaderStages::FRAGMENT,ty:wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),count:None },
        ] })
}

pub fn mesh(device:&wgpu::Device,layout:&wgpu::BindGroupLayout)->wgpu::RenderPipeline {
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("handles"),source:wgpu::ShaderSource::Wgsl(include_str!("widget.wgsl").into())});
        let mesh_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("handle mesh"),bind_group_layouts:&[Some(&layout)],immediate_size:0});
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label:Some("handle mesh"),layout:Some(&mesh_layout),
            vertex:wgpu::VertexState { module:&shader,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[wgpu::VertexBufferLayout {array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&wgpu::vertex_attr_array![0=>Float32x3,1=>Uint32,2=>Uint32]}] },
            fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fs_main"),compilation_options:Default::default(),targets:&[Some(wgpu::TextureFormat::Rgba8Unorm.into())]}),
            primitive:Default::default(),depth_stencil:Some(wgpu::DepthStencilState {format:wgpu::TextureFormat::Depth32Float,depth_write_enabled:Some(true),depth_compare:Some(wgpu::CompareFunction::LessEqual),stencil:Default::default(),bias:Default::default()}),
            multisample:wgpu::MultisampleState {count:4,..Default::default()},multiview_mask:None,cache:None,
        })
}

pub fn composite(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,texture_layout:&wgpu::BindGroupLayout,format:wgpu::TextureFormat)->wgpu::RenderPipeline {
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("handles"),source:wgpu::ShaderSource::Wgsl(include_str!("widget.wgsl").into())});
        let composite_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("handle composite"),bind_group_layouts:&[Some(&layout),Some(&texture_layout)],immediate_size:0});
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label:Some("handle composite"),layout:Some(&composite_layout),
            vertex:wgpu::VertexState {module:&shader,entry_point:Some("vs_composite"),compilation_options:Default::default(),buffers:&[]},
            fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fs_composite"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend:Some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL})]}),
            primitive:Default::default(),depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None,
        })
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzzzzt-share-depth-endpoints-and-comparisons-04.rs"
```

### 4. `src/cloud_gpu.rs`

Reuse the shared strict or equal comparison in this pipeline.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    depth_stencil: Some(wgpu::DepthStencilState {
      format: crate::depth::FORMAT,
      depth_write_enabled: Some(true),
      depth_compare: Some(wgpu::CompareFunction::LessEqual),
      stencil: Default::default(),
      bias: Default::default(),
    }),
```

</details>

Replace it with:

```rust
    depth_stencil: Some(wgpu::DepthStencilState {
      format: crate::depth::FORMAT,
      depth_write_enabled: Some(true),
      depth_compare: Some(crate::depth::CLOSER_EQUAL),
      stencil: Default::default(),
      bias: Default::default(),
    }),
```

<details>
<summary>Complete updated block reference</summary>

```rust
--8<-- "journey/code/96zzzzzt-share-depth-endpoints-and-comparisons-05.rs"
```

</details>

### 5. `src/outline_gpu.rs`

Reuse the shared strict or equal comparison in this pipeline.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    depth_stencil: Some(wgpu::DepthStencilState {
      format: crate::depth::FORMAT,
      depth_write_enabled: Some(false),
      depth_compare: Some(wgpu::CompareFunction::LessEqual),
      stencil: Default::default(),
      bias: Default::default(),
    }),
```

</details>

Replace it with:

```rust
    depth_stencil: Some(wgpu::DepthStencilState {
      format: crate::depth::FORMAT,
      depth_write_enabled: Some(false),
      depth_compare: Some(crate::depth::CLOSER_EQUAL),
      stencil: Default::default(),
      bias: Default::default(),
    }),
```

<details>
<summary>Complete updated block reference</summary>

```rust
--8<-- "journey/code/96zzzzzt-share-depth-endpoints-and-comparisons-06.rs"
```

</details>

### 6. `src/scene_text_gpu.rs`

Reuse the shared strict or equal comparison in this pipeline.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            ctx,
            &mut self.atlas,
            target,
            wgpu::CompareFunction::LessEqual,
        );
        self.prepared = None;
    }
```

</details>

Replace it with:

```rust
            ctx,
            &mut self.atlas,
            target,
            crate::depth::CLOSER_EQUAL,
        );
        self.prepared = None;
    }
```

<details>
<summary>Locate this block</summary>

```rust
            ctx,
            &mut self.atlas,
            self.target,
            wgpu::CompareFunction::LessEqual,
        );
        self.raster = SwashCache::new();
        self.raster_keys.clear();
```

</details>

Replace it with:

```rust
            ctx,
            &mut self.atlas,
            self.target,
            crate::depth::CLOSER_EQUAL,
        );
        self.raster = SwashCache::new();
        self.raster_keys.clear();
```

<details>
<summary>Locate this block</summary>

```rust
        let viewport = Viewport::new(&ctx.device, &cache);
        let mut atlas = make_atlas(ctx, &cache, target.format);
        let overlay = renderer(ctx, &mut atlas, target, wgpu::CompareFunction::Always);
        let anchored = renderer(ctx, &mut atlas, target, wgpu::CompareFunction::LessEqual);
        Self {
            document: TextDocument::new(),
            stats: TextStats::default(),
```

</details>

Replace it with:

```rust
        let viewport = Viewport::new(&ctx.device, &cache);
        let mut atlas = make_atlas(ctx, &cache, target.format);
        let overlay = renderer(ctx, &mut atlas, target, wgpu::CompareFunction::Always);
        let anchored = renderer(ctx, &mut atlas, target, crate::depth::CLOSER_EQUAL);
        Self {
            document: TextDocument::new(),
            stats: TextStats::default(),
```

<details>
<summary>Complete updated block reference</summary>

```rust
--8<-- "journey/code/96zzzzzt-share-depth-endpoints-and-comparisons-07.rs"
```

</details>

### 7. `src/outline_caps.rs`

Reuse the shared strict or equal comparison in this pipeline.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Caps{
 pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,
 input:Option<wgpu::TextureView>,group:Option<wgpu::BindGroup>,
}
impl Caps{
 pub fn new(device:&wgpu::Device)->Self{
  use wgpu::{ShaderStages as Stages,TextureSampleType as Sample};
  let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("visible cap coverage"),entries:&[
   crate::text_gpu_support::texture_entry(0,Stages::FRAGMENT,Sample::Uint,false),
   wgpu::BindGroupLayoutEntry{binding:1,visibility:Stages::FRAGMENT,ty:wgpu::BindingType::Buffer{ty:wgpu::BufferBindingType::Uniform,has_dynamic_offset:false,min_binding_size:wgpu::BufferSize::new(16)},count:None},
  ]});
  let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("visible cap coverage"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("outline_caps.wgsl"),1,0).into())});
  let clipping=crate::clipping_gpu::layout(device);
  let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("visible cap coverage"),bind_group_layouts:&[Some(&layout),Some(&clipping)],immediate_size:0});
  let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
   label:Some("visible cap coverage"),layout:Some(&pipeline_layout),
   vertex:wgpu::VertexState{module:&shader,entry_point:Some("vs_main"),buffers:&[],compilation_options:Default::default()},
   fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some("fs_main"),targets:&[Some(wgpu::TextureFormat::R8Unorm.into())],compilation_options:Default::default()}),
   primitive:Default::default(),depth_stencil:Some(wgpu::DepthStencilState{format:crate::depth::FORMAT,depth_write_enabled:Some(false),depth_compare:Some(wgpu::CompareFunction::LessEqual),stencil:Default::default(),bias:Default::default()}),multisample:Default::default(),multiview_mask:None,cache:None,
  });Self{pipeline,layout,input:None,group:None}
 }
 pub fn prepare(&mut self,device:&wgpu::Device,input:&wgpu::TextureView,radius:&wgpu::Buffer){
  if self.input.as_ref()==Some(input){return;}
  self.group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("visible cap coverage"),layout:&self.layout,entries:&[
   wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(input)},wgpu::BindGroupEntry{binding:1,resource:radius.as_entire_binding()},
  ]}));self.input=Some(input.clone());
 }
 pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>,clipping:&wgpu::BindGroup){if let Some(group)=&self.group{pass.set_pipeline(&self.pipeline);pass.set_bind_group(0,group,&[]);pass.set_bind_group(1,clipping,&[]);pass.draw(0..3,0..1);}}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzzzzt-share-depth-endpoints-and-comparisons-08.rs"
```

### 8. `src/stroke_gpu.rs`

Reuse the shared strict or equal comparison in this pipeline.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("stroke"), source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let strict_depth=depth.depth_compare==Some(wgpu::CompareFunction::Less);
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("stroke"), layout: Some(&layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some("vertex"),
```

</details>

Replace it with:

```rust
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("stroke"), source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let strict_depth=depth.depth_compare==Some(crate::depth::CLOSER);
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("stroke"), layout: Some(&layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some("vertex"),
```

<details>
<summary>Complete updated block reference</summary>

```rust
--8<-- "journey/code/96zzzzzt-share-depth-endpoints-and-comparisons-09.rs"
```

</details>

### 9. `src/text_plane.rs`

Reuse the shared strict or equal comparison in this pipeline.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            vertex: wgpu::VertexState { module: &shader, entry_point: Some(if ids { "vs_id" } else { "vs_main" }), buffers: &[wgpu::VertexBufferLayout { array_stride: 64, step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x2, 2 => Float32x4, 3 => Float32x4, 4 => Uint32, 5 => Float32] }], compilation_options: Default::default() },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some(if ids { "fs_id" } else { "fs_main" }), targets: &[Some(wgpu::ColorTargetState { format: target.format, blend: if ids { None } else { Some(wgpu::BlendState::ALPHA_BLENDING) }, write_mask: wgpu::ColorWrites::ALL })], compilation_options: Default::default() }),
            primitive: Default::default(), depth_stencil: Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth32Float, depth_write_enabled: Some(false), depth_compare: Some(wgpu::CompareFunction::LessEqual), stencil: Default::default(), bias: Default::default() }),
            multisample: wgpu::MultisampleState { count: target.samples, ..Default::default() }, multiview_mask: None, cache: None,
        })
    }
```

</details>

Replace it with:

```rust
            vertex: wgpu::VertexState { module: &shader, entry_point: Some(if ids { "vs_id" } else { "vs_main" }), buffers: &[wgpu::VertexBufferLayout { array_stride: 64, step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x2, 2 => Float32x4, 3 => Float32x4, 4 => Uint32, 5 => Float32] }], compilation_options: Default::default() },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some(if ids { "fs_id" } else { "fs_main" }), targets: &[Some(wgpu::ColorTargetState { format: target.format, blend: if ids { None } else { Some(wgpu::BlendState::ALPHA_BLENDING) }, write_mask: wgpu::ColorWrites::ALL })], compilation_options: Default::default() }),
            primitive: Default::default(), depth_stencil: Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth32Float, depth_write_enabled: Some(false), depth_compare: Some(crate::depth::CLOSER_EQUAL), stencil: Default::default(), bias: Default::default() }),
            multisample: wgpu::MultisampleState { count: target.samples, ..Default::default() }, multiview_mask: None, cache: None,
        })
    }
```

<details>
<summary>Complete updated block reference</summary>

```rust
--8<-- "journey/code/96zzzzzt-share-depth-endpoints-and-comparisons-10.rs"
```

</details>

### 10. `src/pick_encode.rs`

Clear to the shared far endpoint.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{gpu_mesh::GpuMesh, pick_image::Image, pick_window::Window, stroke_gpu::Lane};

pub(crate) fn draw(device: &wgpu::Device, image: &Image, window: Window, pipeline: &wgpu::RenderPipeline,
    view_group: &wgpu::BindGroup, crop_group: &wgpu::BindGroup, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces, mode: crate::pick_sources::Mode,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::sheet_gpu::Lane>,ctx:&crate::text_gpu_support::GpuCtx,tables:Option<&crate::finite_ink::Tables>,clipping:&wgpu::BindGroup,caps:Option<&mut crate::caps_gpu::Caps>,crop:&wgpu::Buffer,clipping_buffer:&wgpu::Buffer,records:Option<(&crate::instance_gpu::Lane,&wgpu::RenderPipeline)>) -> wgpu::CommandBuffer {
        let mut encoder = device.create_command_encoder(&Default::default());
        let mesh_crop=crate::pick_pipeline::crop_clipping_group(device,crop,clipping_buffer);
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("IDs"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &image.colour, depth_slice: None, resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store } }),Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &image.depth,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }), stencil_ops: None }), ..Default::default() });
            pass.set_pipeline(pipeline); pass.set_bind_group(0, view_group, &[]); pass.set_bind_group(2, &mesh_crop, &[]);
            for mesh in meshes {if !records.is_some_and(|(rows,shared)|rows.draw_with(&mut pass,mesh,shared)){pass.set_pipeline(pipeline);mesh.draw_pulled(&mut pass);}}

        }
        if mode==crate::pick_sources::Mode::Face{
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original source-face IDs"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:None,stencil_ops:None}),..Default::default()});
            faces.draw(&mut pass,meshes,view_group,crop_group,records.map(|(rows,_)|rows));
        }
        if let Some(caps)=caps{caps.encode_pick(device,&ctx.queue,&mut encoder,image,window,view_group,crop,clipping,meshes,mode,records.map(|(rows,_)|rows));}
        let tables=tables.map(|tables|crate::finite_ink::Tables{projected:tables.projected.clone(),tiles:tables.tiles.clone(),primitive:image.primitive.view.clone()});
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite integer ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:None,stencil_ops:None}),..Default::default()});
            pass.set_bind_group(3,clipping,&[]);
            for lane in ink {lane.pick_finite(device,&mut pass,crop_group,tables.as_ref(),&image.depth);}
        }
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("other original integer owners"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});
            if let Some(clouds)=clouds {clouds.pick(ctx,&mut pass,window,mode);}
            if let Some(sheets)=sheets {sheets.pick(&mut pass,crop_group,clipping);}
            if let Some(text)=text {text.draw_ids(&mut pass,crop_group);}
        }
        // The surrounding halo supports rasterization; only the smaller cursor window is copied back for hit testing.
        encoder.copy_texture_to_buffer(wgpu::TexelCopyTextureInfo { texture: &image.texture, mip_level: 0,
            origin: wgpu::Origin3d { x: window.copy[0] - window.draw[0], y: window.copy[1] - window.draw[1], z: 0 }, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &image.readback, layout: wgpu::TexelCopyBufferLayout { offset: 0,
                bytes_per_row: Some(window.row_bytes()), rows_per_image: Some(window.copy[3]) } },
            wgpu::Extent3d { width: window.copy[2], height: window.copy[3], depth_or_array_layers: 1 });
        encoder.finish()
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzzzzt-share-depth-endpoints-and-comparisons-11.rs"
```

### 11. `src/widget.rs`

Clear to the shared far endpoint.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("handle tile"),
                color_attachments:&[Some(wgpu::RenderPassColorAttachment {view:&tile.color,resolve_target:Some(&tile.resolved),depth_slice:None,ops:wgpu::Operations {load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Discard}})],
                depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {view:&tile.depth,depth_ops:Some(wgpu::Operations {load:wgpu::LoadOp::Clear(1.),store:wgpu::StoreOp::Discard}),stencil_ops:None}),..Default::default()});
            pass.set_pipeline(&self.pipeline); pass.set_bind_group(0,&self.group,&[]); pass.set_vertex_buffer(0,self.vertices.slice(..)); pass.draw(0..self.count,0..1);
        }
        let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("handle overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view,resolve_target:None,depth_slice:None,ops:wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});
```

</details>

Replace it with:

```rust
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("handle tile"),
                color_attachments:&[Some(wgpu::RenderPassColorAttachment {view:&tile.color,resolve_target:Some(&tile.resolved),depth_slice:None,ops:wgpu::Operations {load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Discard}})],
                depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {view:&tile.depth,depth_ops:Some(wgpu::Operations {load:wgpu::LoadOp::Clear(crate::depth::FAR as f32),store:wgpu::StoreOp::Discard}),stencil_ops:None}),..Default::default()});
            pass.set_pipeline(&self.pipeline); pass.set_bind_group(0,&self.group,&[]); pass.set_vertex_buffer(0,self.vertices.slice(..)); pass.draw(0..self.count,0..1);
        }
        let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("handle overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view,resolve_target:None,depth_slice:None,ops:wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});
```

<details>
<summary>Complete updated block reference</summary>

```rust
--8<-- "journey/code/96zzzzzt-share-depth-endpoints-and-comparisons-12.rs"
```

</details>

### 12. `src/cloud_gpu.rs`

Clear to the shared far endpoint.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        color_attachments: &[color(&t.color.view), color(&t.ids.view)],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
          view: &t.depth.view,
          depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.), store: wgpu::StoreOp::Store }),
          stencil_ops: None,
        }),
        ..Default::default()
```

</details>

Replace it with:

```rust
        color_attachments: &[color(&t.color.view), color(&t.ids.view)],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
          view: &t.depth.view,
          depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(crate::depth::FAR as f32), store: wgpu::StoreOp::Store }),
          stencil_ops: None,
        }),
        ..Default::default()
```

<details>
<summary>Complete updated block reference</summary>

```rust
--8<-- "journey/code/96zzzzzt-share-depth-endpoints-and-comparisons-13.rs"
```

</details>

### 13. `src/renderer.rs`

Clear to the shared far endpoint.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
```

</details>

Replace it with:

```rust
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(crate::depth::FAR as f32),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
```

<details>
<summary>Complete updated block reference</summary>

```rust
--8<-- "journey/code/96zzzzzt-share-depth-endpoints-and-comparisons-14.rs"
```

</details>

### 14. `src/camera.rs`

Construct CPU rays from the same near and far endpoints.

<details>
<summary>Locate the existing block</summary>

```rust
        let origin = inverse.transform_point(&Point::new(screen[0] as f64, screen[1] as f64, 0.0));
        let end = inverse.transform_point(&Point::new(screen[0] as f64, screen[1] as f64, 1.0));
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        let origin = inverse.transform_point(&Point::new(screen[0] as f64, screen[1] as f64, crate::depth::NEAR));
        let end = inverse.transform_point(&Point::new(screen[0] as f64, screen[1] as f64, crate::depth::FAR));
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzt-share-depth-endpoints-and-comparisons-15.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 15. `src/depth.rs`

Supply exact shared-state assertions while retaining every original GPU and camera fixture.

<details>
<summary>Locate the existing block</summary>

```text
pub const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
pub const NEAR:f64=0.;
pub const FAR:f64=1.;
pub const CLOSER:wgpu::CompareFunction=wgpu::CompareFunction::Less;
pub const CLOSER_EQUAL:wgpu::CompareFunction=wgpu::CompareFunction::LessEqual;

pub fn surface() -> wgpu::DepthStencilState { state(true, crate::depth::CLOSER) }
pub fn ink() -> wgpu::DepthStencilState { state(false, crate::depth::CLOSER_EQUAL) }
pub fn grid() -> wgpu::DepthStencilState { state(false, crate::depth::CLOSER) }

fn state(write: bool, compare: wgpu::CompareFunction) -> wgpu::DepthStencilState {
    wgpu::DepthStencilState { format: FORMAT, depth_write_enabled: Some(write), depth_compare: Some(compare),
        stencil: Default::default(), bias: Default::default() }
}

pub fn state_for_ambient()->wgpu::DepthStencilState{state(false,wgpu::CompareFunction::Equal)}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzt-share-depth-endpoints-and-comparisons-16.rs" skip=871
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 21.4.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Depth clears, comparisons and CPU ray endpoints have one owner.

**Verified checkpoint in Chrome.**

**Feature-specific visual review pending.** Scripted acceptance and the final main-page capture are separate evidence. A checker may restore its general fixture after closing the feature-test page; this image does not establish the feature’s visual correctness.

![General browser fixture after Share depth endpoints and comparisons; feature-scene visual acceptance pending.](../screenshots/journey/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-browser.png)

[Verification scope](release.md).

This reference image is shared with [20.11.6 · Adopt retained GPU slots and physical bases](96zzzzn-adopt-retained-gpu-slots-and-physical-bases.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Color, original picks, clouds, text and widget tiles must agree about an empty depth buffer and the closest fragment. Keep their strict or equal comparisons distinct, but take both from the existing depth module. CPU rays use the same near/far endpoints. This step preserves the current picture; the projection and shader readers still use forward depth until the coordinated switch.

Depth owner → clear and comparison → original pipelines and CPU rays.



Why keep strict and equal comparisons?

A grid needs strict separation from its supporting surface, while ink and text may intentionally share surface depth. One direction does not make these predicates interchangeable.

Study estimate, including typing and experiments: 0.6–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Overlap boxes, select the front original, toggle the grid and text, and orbit. Check exact clearing of selection, distant placement and Save/reopen. The depth convention has not changed yet.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 21.4.1
npm --prefix ../session_tests run course -- save 21.4.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Shared depth state is preparatory. Reversed projection, near-plane ink clipping and shader readers must switch together before this completion item can be checked.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 21.4.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
