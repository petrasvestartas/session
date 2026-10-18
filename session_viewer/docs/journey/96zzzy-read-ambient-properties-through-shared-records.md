# 20.9.7 · Read ambient properties through shared records

**Chapter 20 · Shared GPU rendering · Section 20.9 · Stable records and original picking** · [Contents](contents.md)

**Typing: 17–33 minutes.** [Estimate](typing-load.md).

Connect source normal and radius drawing to the existing object table.

## Type

Continue from [Pack existing ambient radius in object records](96zzzx-pack-existing-ambient-radius-in-object-records.md). [Save or recover your work](recovery.md).

### 1. `src/ambient_source_pipeline.rs`

Own the shared property state beside its existing fallback. Move the fallback radius layout to the shared descriptor owner. Build both property states from one descriptor. Select the current shared record layout or the earlier uniform layout. Use the existing arena geometry binding for pulled properties. Shared properties read radius from their record; fallback properties retain their uniform. Adapt the same property shader through the existing record owner. Share the shader source across both property states. Select the slot-aware property entry for shared records. Keep the indexed vertex attributes alive while creating either property state. Read draw slots for shared properties and retain the earlier indexed vertex format for fallback. Create both states with the same property outputs and depth policy.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Pipeline{pub draw:wgpu::RenderPipeline,pub radius:wgpu::BindGroupLayout}
impl Pipeline{
 pub fn new(device:&wgpu::Device)->Self{
  let view=crate::gpu_layout::uniform(device,80);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);let clip=crate::clipping_gpu::layout(device);let radius=crate::gpu_layout::uniform(device,16);let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("ambient original source metadata"),bind_group_layouts:&[Some(&view),Some(&model),Some(&clip),Some(&radius)],immediate_size:0});let module=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("ambient original normals and radii"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(concat!(include_str!("triangle.wgsl"),"
",include_str!("ambient_sources.wgsl")),2,0).into())});
  let draw=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("ambient original normals and radii"),layout:Some(&layout),vertex:wgpu::VertexState{module:&module,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[wgpu::VertexBufferLayout{array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&wgpu::vertex_attr_array![0=>Float32x3,2=>Uint32]}]},fragment:Some(wgpu::FragmentState{module:&module,entry_point:Some("fs_main"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R32Float,blend:None,write_mask:wgpu::ColorWrites::RED}),Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::state_for_ambient()),multisample:Default::default(),multiview_mask:None,cache:None});Self{draw,radius}
 }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Pipeline{pub draw:wgpu::RenderPipeline,pub record_draw:wgpu::RenderPipeline,pub radius:wgpu::BindGroupLayout}
impl Pipeline{
 pub fn new(device:&wgpu::Device)->Self{
  let radius=crate::gpu_layout::uniform(device,16);let create=|shared| {
  let view=crate::gpu_layout::uniform(device,80);let model=if shared {crate::object_rows::layout(device)}else{crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64)};let clip=crate::clipping_gpu::layout(device);let geometry=crate::gpu_geometry::GpuGeometry::storage_layout(device);let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("ambient original source metadata"),bind_group_layouts:&[Some(&view),Some(&model),Some(&clip),Some(if shared {&geometry}else{&radius})],immediate_size:0});let source=concat!(include_str!("triangle.wgsl"),"
",include_str!("ambient_sources.wgsl"));let source=if shared {crate::object_rows::source(source)}else{source.into()};let module=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("ambient original normals and radii"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(&source,2,0).into())});
  let attributes=wgpu::vertex_attr_array![0=>Float32x3,2=>Uint32];let draw=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("ambient original normals and radii"),layout:Some(&layout),vertex:wgpu::VertexState{module:&module,entry_point:Some(if shared {"vs_record"}else{"vs_main"}),compilation_options:Default::default(),buffers:&[if shared {crate::draw_slots::layout()}else{wgpu::VertexBufferLayout{array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&attributes}}]},fragment:Some(wgpu::FragmentState{module:&module,entry_point:Some("fs_main"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R32Float,blend:None,write_mask:wgpu::ColorWrites::RED}),Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::state_for_ambient()),multisample:Default::default(),multiview_mask:None,cache:None});draw};let draw=create(false);let record_draw=create(true);Self{draw,record_draw,radius}
 }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-01.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-03.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-04.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-05.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-06.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-07.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-08.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-09.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-attributes.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-10.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-11.rs"
```

</details>

### 2. `src/ambient_source_gpu.rs`

Accept the existing renderer object table. Read current shared properties or the earlier exact uniform fallback.

<details>
<summary>Locate the existing block</summary>

```rust
use std::collections::HashMap;
pub struct Sources{pipes:crate::ambient_source_pipeline::Pipeline,rows:HashMap<u32,(wgpu::Buffer,wgpu::BindGroup,f32)>,pub writes:u64,pub draws:u64}
impl Sources{
 pub fn new(device:&wgpu::Device)->Self{Self{pipes:crate::ambient_source_pipeline::Pipeline::new(device),rows:Default::default(),writes:0,draws:0}}
 pub fn set(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,specs:&[(u32,f32)]){
  let live=specs.iter().map(|(id,_)|*id).collect::<std::collections::HashSet<_>>();self.rows.retain(|id,_|live.contains(id));for &(id,radius)in specs{let row=self.rows.entry(id).or_insert_with(||{let buffer=device.create_buffer(&wgpu::BufferDescriptor{label:Some("original ambient contact radius"),size:16,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let group=crate::gpu_layout::group(device,&self.pipes.radius,&buffer);(buffer,group,f32::NAN)});if row.2!=radius{queue.write_buffer(&row.0,0,&radius.to_ne_bytes());row.2=radius;self.writes+=1;}}if self.rows.is_empty(){self.rows.shrink_to_fit();}
 }
 pub fn encode(&mut self,encoder:&mut wgpu::CommandEncoder,radius:&wgpu::TextureView,normals:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clip:&wgpu::BindGroup,meshes:&[crate::gpu_mesh::GpuMesh]){
  let attachments=[radius,normals].map(|view|Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}}));let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original ambient source properties"),color_attachments:&attachments,depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&self.pipes.draw);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clip,&[]);for mesh in meshes{if let Some((_,group,_))=self.rows.get(&mesh.id.value()){mesh.bind_model(&mut pass);pass.set_bind_group(3,group,&[]);mesh.geometry.draw(&mut pass);self.draws+=1;}}
 }
 pub fn bytes(&self)->u64{self.rows.values().map(|r|r.0.size()).sum()}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use std::collections::HashMap;
pub struct Sources{pipes:crate::ambient_source_pipeline::Pipeline,rows:HashMap<u32,(wgpu::Buffer,wgpu::BindGroup,f32)>,pub writes:u64,pub draws:u64}
impl Sources{
 pub fn new(device:&wgpu::Device)->Self{Self{pipes:crate::ambient_source_pipeline::Pipeline::new(device),rows:Default::default(),writes:0,draws:0}}
 pub fn set(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,specs:&[(u32,f32)]){
  let live=specs.iter().map(|(id,_)|*id).collect::<std::collections::HashSet<_>>();self.rows.retain(|id,_|live.contains(id));for &(id,radius)in specs{let row=self.rows.entry(id).or_insert_with(||{let buffer=device.create_buffer(&wgpu::BufferDescriptor{label:Some("original ambient contact radius"),size:16,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let group=crate::gpu_layout::group(device,&self.pipes.radius,&buffer);(buffer,group,f32::NAN)});if row.2!=radius{queue.write_buffer(&row.0,0,&radius.to_ne_bytes());row.2=radius;self.writes+=1;}}if self.rows.is_empty(){self.rows.shrink_to_fit();}
 }
 pub fn encode(&mut self,encoder:&mut wgpu::CommandEncoder,radius:&wgpu::TextureView,normals:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clip:&wgpu::BindGroup,meshes:&[crate::gpu_mesh::GpuMesh],records:Option<&crate::instance_gpu::Lane>){
  let attachments=[radius,normals].map(|view|Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}}));let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original ambient source properties"),color_attachments:&attachments,depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&self.pipes.draw);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clip,&[]);for mesh in meshes{if let Some((_,group,_))=self.rows.get(&mesh.id.value()){if records.is_some_and(|rows|rows.bind_record(&mut pass,mesh)){pass.set_pipeline(&self.pipes.record_draw);mesh.geometry.draw_pulled(&mut pass,1);}else{pass.set_pipeline(&self.pipes.draw);mesh.bind_model(&mut pass);pass.set_bind_group(3,group,&[]);mesh.geometry.draw(&mut pass);}self.draws+=1;}}
 }
 pub fn bytes(&self)->u64{self.rows.values().map(|r|r.0.size()).sum()}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-12.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-13.rs"
```

</details>

### 3. `src/ambient_gpu.rs`

Keep the object table available while preparing original source properties. Pass the existing record owner through the unchanged ambient recipe.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{ambient_images::Images,ambient_groups::Groups,ambient_pass::draw};
struct State{images:Images,groups:Groups,input:(wgpu::TextureView,wgpu::TextureView,wgpu::BindGroup)}
pub struct Ambient{pipes:crate::ambient_pipeline::Pipelines,pub sources:crate::ambient_source_gpu::Sources,frame:wgpu::Buffer,state:Option<State>,cached:Option<([f32;80],[u64;2])>,pub preparations:u64,pub writes:u64,pub passes:u64,pub error:Option<&'static str>}
impl Ambient{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{Self{pipes:crate::ambient_pipeline::Pipelines::new(device,format),sources:crate::ambient_source_gpu::Sources::new(device),frame:device.create_buffer(&wgpu::BufferDescriptor{label:Some("current ambient camera"),size:320,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false}),state:None,cached:None,preparations:0,writes:0,passes:0,error:None}}
 pub fn clear(&mut self,device:&wgpu::Device,queue:&wgpu::Queue){self.state=None;self.cached=None;self.sources.set(device,queue,&[]);self.error=None;}
 fn prepare(&mut self,device:&wgpu::Device,full:[u32;2],density:f32,depth:&wgpu::TextureView,sections:&wgpu::TextureView,clip:&wgpu::Buffer){
  let(w,h)=crate::pyramid_size::resolution((full[0],full[1]),f64::from(density));let size=[w.max(32),h.max(32)];
  if self.state.as_ref().is_none_or(|s|s.images.full!=full||s.images.size!=size){let images=Images::new(device,full,density);let groups=Groups::new(device,&self.pipes,&images);let group=Groups::source(device,&self.pipes,&images,depth,sections,&self.frame,clip);self.state=Some(State{images,groups,input:(depth.clone(),sections.clone(),group)});self.cached=None;self.preparations+=1;}
  let state=self.state.as_mut().unwrap();if state.input.0!=*depth||state.input.1!=*sections{state.input=(depth.clone(),sections.clone(),Groups::source(device,&self.pipes,&state.images,depth,sections,&self.frame,clip));self.cached=None;}
 }
 pub fn draw(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,clip:&wgpu::Buffer,sections:&wgpu::TextureView,meshes:&[crate::gpu_mesh::GpuMesh],matrix:[f32;16],full:[u32;2],density:f32,ground:[f32;2],revision:[u64;2]){
  if ground[1]<=0.0{self.clear(device,queue);return;}if matrix.iter().any(|v|!v.is_finite())||full.contains(&0)||!density.is_finite()||density<=0.{self.error=Some("Invalid ambient view");return;}
  self.prepare(device,full,density,depth,sections,clip);let state=self.state.as_ref().unwrap();let Some(uniform)=crate::ambient_camera::uniform(matrix,full,state.images.size,ground)else{self.error=Some("Invalid ambient camera inverse");return;};
  if self.cached!=Some((uniform,revision)){
   queue.write_buffer(&self.frame,0,&uniform.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());self.writes+=1;
   let images=&state.images;let groups=&state.groups;let source=&state.input.2;self.sources.encode(encoder,&images.source_radius.view,&images.source_normals.view,depth,view,clipping,meshes);self.passes+=1;
   draw(encoder,&self.pipes.prepare,&[&images.linear.levels[0],&images.radii.levels[0],&images.normals.view],&[(0,source)],true);
   for level in 1..6{draw(encoder,&self.pipes.reduce,&[&images.linear.levels[level],&images.radii.levels[level]],&[(0,&groups.levels[level-1])],true);}self.passes+=6;
   draw(encoder,&self.pipes.occupancy,&[&images.occupancy.view],&[(0,&groups.levels[0])],true);
   draw(encoder,&self.pipes.ground,&[&images.ground.view],&[(0,source),(1,&groups.levels[0]),(2,&groups.occupancy),(3,&groups.history)],true);
   draw(encoder,&self.pipes.raw,&[&images.ao[0].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ground),(3,&groups.history)],true);
   draw(encoder,&self.pipes.filter[0],&[&images.ao[1].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[0]),(3,&groups.history)],true);
   draw(encoder,&self.pipes.filter[1],&[&images.ao[0].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[1]),(3,&groups.history)],true);
   draw(encoder,&self.pipes.upsample,&[&images.ao[2].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[0]),(3,&groups.history)],true);self.passes+=6;
   self.cached=Some((uniform,revision));
  }
  draw(encoder,&self.pipes.composite,&[color],&[(1,&state.groups.ao[2])],false);self.passes+=1;self.error=None;
 }
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.sources.bytes()+self.state.as_ref().map_or(0,|s|s.images.bytes()+s.groups.buffers.iter().map(wgpu::Buffer::size).sum::<u64>()),1,self.preparations,self.writes,self.passes]}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{ambient_images::Images,ambient_groups::Groups,ambient_pass::draw};
struct State{images:Images,groups:Groups,input:(wgpu::TextureView,wgpu::TextureView,wgpu::BindGroup)}
pub struct Ambient{pipes:crate::ambient_pipeline::Pipelines,pub sources:crate::ambient_source_gpu::Sources,frame:wgpu::Buffer,state:Option<State>,cached:Option<([f32;80],[u64;2])>,pub preparations:u64,pub writes:u64,pub passes:u64,pub error:Option<&'static str>}
impl Ambient{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{Self{pipes:crate::ambient_pipeline::Pipelines::new(device,format),sources:crate::ambient_source_gpu::Sources::new(device),frame:device.create_buffer(&wgpu::BufferDescriptor{label:Some("current ambient camera"),size:320,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false}),state:None,cached:None,preparations:0,writes:0,passes:0,error:None}}
 pub fn clear(&mut self,device:&wgpu::Device,queue:&wgpu::Queue){self.state=None;self.cached=None;self.sources.set(device,queue,&[]);self.error=None;}
 fn prepare(&mut self,device:&wgpu::Device,full:[u32;2],density:f32,depth:&wgpu::TextureView,sections:&wgpu::TextureView,clip:&wgpu::Buffer){
  let(w,h)=crate::pyramid_size::resolution((full[0],full[1]),f64::from(density));let size=[w.max(32),h.max(32)];
  if self.state.as_ref().is_none_or(|s|s.images.full!=full||s.images.size!=size){let images=Images::new(device,full,density);let groups=Groups::new(device,&self.pipes,&images);let group=Groups::source(device,&self.pipes,&images,depth,sections,&self.frame,clip);self.state=Some(State{images,groups,input:(depth.clone(),sections.clone(),group)});self.cached=None;self.preparations+=1;}
  let state=self.state.as_mut().unwrap();if state.input.0!=*depth||state.input.1!=*sections{state.input=(depth.clone(),sections.clone(),Groups::source(device,&self.pipes,&state.images,depth,sections,&self.frame,clip));self.cached=None;}
 }
 pub fn draw(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,clip:&wgpu::Buffer,sections:&wgpu::TextureView,meshes:&[crate::gpu_mesh::GpuMesh],matrix:[f32;16],full:[u32;2],density:f32,ground:[f32;2],revision:[u64;2],records:Option<&crate::instance_gpu::Lane>){
  if ground[1]<=0.0{self.clear(device,queue);return;}if matrix.iter().any(|v|!v.is_finite())||full.contains(&0)||!density.is_finite()||density<=0.{self.error=Some("Invalid ambient view");return;}
  self.prepare(device,full,density,depth,sections,clip);let state=self.state.as_ref().unwrap();let Some(uniform)=crate::ambient_camera::uniform(matrix,full,state.images.size,ground)else{self.error=Some("Invalid ambient camera inverse");return;};
  if self.cached!=Some((uniform,revision)){
   queue.write_buffer(&self.frame,0,&uniform.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());self.writes+=1;
   let images=&state.images;let groups=&state.groups;let source=&state.input.2;self.sources.encode(encoder,&images.source_radius.view,&images.source_normals.view,depth,view,clipping,meshes,records);self.passes+=1;
   draw(encoder,&self.pipes.prepare,&[&images.linear.levels[0],&images.radii.levels[0],&images.normals.view],&[(0,source)],true);
   for level in 1..6{draw(encoder,&self.pipes.reduce,&[&images.linear.levels[level],&images.radii.levels[level]],&[(0,&groups.levels[level-1])],true);}self.passes+=6;
   draw(encoder,&self.pipes.occupancy,&[&images.occupancy.view],&[(0,&groups.levels[0])],true);
   draw(encoder,&self.pipes.ground,&[&images.ground.view],&[(0,source),(1,&groups.levels[0]),(2,&groups.occupancy),(3,&groups.history)],true);
   draw(encoder,&self.pipes.raw,&[&images.ao[0].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ground),(3,&groups.history)],true);
   draw(encoder,&self.pipes.filter[0],&[&images.ao[1].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[0]),(3,&groups.history)],true);
   draw(encoder,&self.pipes.filter[1],&[&images.ao[0].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[1]),(3,&groups.history)],true);
   draw(encoder,&self.pipes.upsample,&[&images.ao[2].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[0]),(3,&groups.history)],true);self.passes+=6;
   self.cached=Some((uniform,revision));
  }
  draw(encoder,&self.pipes.composite,&[color],&[(1,&state.groups.ao[2])],false);self.passes+=1;self.error=None;
 }
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.sources.bytes()+self.state.as_ref().map_or(0,|s|s.images.bytes()+s.groups.buffers.iter().map(wgpu::Buffer::size).sum::<u64>()),1,self.preparations,self.writes,self.passes]}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-14.rs"
```

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-15.rs"
```

</details>

### 4. `src/renderer.rs`

Use the same current object records for rendering, picking and ambient properties.

<details>
<summary>Locate the existing block</summary>

```rust
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.depth,&self.view_group,&clip_group,&clip.buffer,&self.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground,[self.pyramid_revision,clip_revision]);}}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.depth,&self.view_group,&clip_group,&clip.buffer,&self.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground,[self.pyramid_revision,clip_revision],Some(&self.instances));}}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-16.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/ambient_gpu_tests.rs`

Supply an exact shared-versus-uniform ambient comparison while retaining every original cache, source, shading and release assertion.

<details>
<summary>Locate the existing block</summary>

```text
fn actual_ambient_darkens_original_contacts_in_one_frame_caches_and_releases_large_images(){
 let mut scene=Scene::empty();let owner=scene.insert(crate::prepared::PreparedMesh::new(source(false)).unwrap()).unwrap();let original=scene.objects()[0].geometry().unwrap().to_proto();let mut camera=crate::camera::Camera::default();camera.isometric();camera.aspect=4./3.;camera.target=[0.,0.,-0.5];camera.distance=3.;let matrix=camera.uniform();let r=renderer(&scene);let plain=pixels(&r,&matrix);let uploads=r.stats()[0];r.set_ambient(true);let shaded=pixels(&r,&matrix);assert_eq!(r.ambient_error(),None);let usage=r.ambient_usage();assert!(usage[0]>100_000);assert_eq!(usage[1],1);assert_eq!(usage[3],1);let second=pixels(&r,&matrix);assert_eq!(second,shaded,"one frame completes ambient shading");assert_eq!(&r.ambient_usage()[1..4],&usage[1..4]);assert_eq!(r.stats()[0],uploads);let darker=plain.chunks_exact(4).zip(shaded.chunks_exact(4)).filter(|(a,b)|a[..3]!=[255,255,255]&&a[..3].iter().zip(&b[..3]).any(|(a,b)|a.saturating_sub(*b)>3)).count();assert!(darker>100,"actual ambient shading: {darker} pixels");let shadow=plain.chunks_exact(4).zip(shaded.chunks_exact(4)).filter(|(a,b)|a[..3]==[255,255,255]&&b[..3].iter().any(|v|*v<235)).count();assert!(shadow>20,"actual virtual-ground contacts: {shadow}");assert_eq!(scene.objects()[0].id,owner);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);r.set_ambient(false);assert_eq!(r.ambient_usage()[0],320);assert_eq!(pixels(&r,&matrix),plain);r.set_ambient(true);assert_eq!(pixels(&r,&matrix),shaded);assert_eq!(r.ambient_usage()[1],1,"style toggles retain compiled pipelines");assert_eq!(r.stats()[0],uploads);
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzy-read-ambient-properties-through-shared-records-17.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.9.7
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Ambient source properties consume current shared records and retain the earlier ambient pixels.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for read ambient properties through shared records.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Read the normal transform, flags and radius from the current shared object row. Pull positions and packed normals from the same arena already used for color and picks. Keep one property shader and descriptor for shared and uniform fallback states. The ambient algorithm and its cache remain unchanged; only its source resource path changes.

Current object row and arena geometry → original source normals and radius → existing ambient recipe.



Why keep the ambient recipe unchanged during this connection?

Changing its source resource should preserve the earlier property values and pixels. The exact comparison isolates that resource change from later contact and pipeline-order work.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Toggle Arctic on an ordinary original and a copied placement. Select and restore them, then inspect the supplied exact shared-versus-uniform ambient comparison and full release checks.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.9.7
npm --prefix ../session_tests run course -- save 20.9.7
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Ambient source properties share object records. Projection, cap/property consumers, primitive lookup, current contact inputs and pipeline-order parity remain open.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.9.7
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
