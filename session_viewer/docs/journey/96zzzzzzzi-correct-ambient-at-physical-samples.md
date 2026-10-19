# 22.5.16 · Correct ambient at physical samples

**Chapter 22 · Surface lighting, contact and display quality · Section 22.5 · Frame resources and multisampling** · [Contents](contents.md)

**Typing: 30–60 minutes.** [Estimate](typing-load.md).

Connect matching original ambient properties to every physical sample and draw corrections only for samples that differ. Retain the existing coarse hierarchy and images, cache colour recipes and bound correction work by the viewport and its tile count.

## Type

Continue from [Share ambient shader recipes](96zzzzzzzh-share-ambient-shader-recipes.md). [Save or recover your work](recovery.md).

### 1. `src/ambient_gpu.rs`

Retain coarse ambient images while matching physical inputs and execute bounded sparse per-sample corrections without runtime readback flags.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{ambient_images::Images,ambient_groups::Groups,ambient_pass::draw};
struct State{images:Images,groups:Groups,input:(wgpu::TextureView,wgpu::TextureView,wgpu::BindGroup)}
pub struct Ambient{pipes:crate::ambient_pipeline::Pipelines,pub sources:crate::ambient_source_gpu::Sources,frame:wgpu::Buffer,state:Option<State>,cached:Option<([f32;80],[u64;2])>,pub preparations:u64,pub writes:u64,pub passes:u64,pub error:Option<&'static str>}
impl Ambient{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{Self{pipes:crate::ambient_pipeline::Pipelines::new(device,format),sources:crate::ambient_source_gpu::Sources::new(device),frame:device.create_buffer(&wgpu::BufferDescriptor{label:Some("current ambient camera"),size:320,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false}),state:None,cached:None,preparations:0,writes:0,passes:0,error:None}}
 pub fn clear(&mut self){self.state=None;self.cached=None;self.error=None;}
 fn prepare(&mut self,device:&wgpu::Device,full:[u32;2],density:f32,depth:&wgpu::TextureView,sections:&wgpu::TextureView,clip:&wgpu::Buffer){
  let(w,h)=crate::pyramid_size::resolution((full[0],full[1]),f64::from(density));let size=[w.max(32),h.max(32)];
  if self.state.as_ref().is_none_or(|s|s.images.full!=full||s.images.size!=size){let images=Images::new(device,full,density);let groups=Groups::new(device,&self.pipes,&images,&self.frame);let group=Groups::source(device,&self.pipes,&images,depth,sections,&self.frame,clip);self.state=Some(State{images,groups,input:(depth.clone(),sections.clone(),group)});self.cached=None;self.preparations+=1;}
  let state=self.state.as_mut().unwrap();if state.input.0!=*depth||state.input.1!=*sections{state.input=(depth.clone(),sections.clone(),Groups::source(device,&self.pipes,&state.images,depth,sections,&self.frame,clip));self.cached=None;}
 }
 pub fn draw(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,clip:&wgpu::Buffer,sections:&wgpu::TextureView,meshes:&[crate::gpu_mesh::GpuMesh],matrix:[f32;16],full:[u32;2],density:f32,ground:[f32;2],revision:[u64;2],records:Option<&crate::instance_gpu::Lane>){
  if ground[1]<=0.0{self.clear();return;}if matrix.iter().any(|v|!v.is_finite())||full.contains(&0)||!density.is_finite()||density<=0.{self.error=Some("Invalid ambient view");return;}
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
  draw(encoder,&self.pipes.composite,&[color],&[(0,&state.groups.composite),(1,&state.groups.ao[2])],false);self.passes+=1;self.error=None;
 }
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.state.as_ref().map_or(0,|s|s.images.bytes()+s.groups.buffers.iter().map(wgpu::Buffer::size).sum::<u64>()),1,self.preparations,self.writes,self.passes]}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::sample_pipelines::Variants;
use crate::{ambient_images::Images,ambient_groups::Groups,ambient_pass::draw};
struct State{images:Images,groups:Groups,input:(wgpu::TextureView,wgpu::TextureView,wgpu::BindGroup)}
pub struct Ambient{pipes:Variants<crate::ambient_pipeline::Pipelines>,format:wgpu::TextureFormat,pub sources:crate::ambient_source_gpu::Sources,frame:wgpu::Buffer,state:Option<State>,cached:Option<([f32;80],[u64;2])>,pub preparations:u64,pub writes:u64,pub passes:u64,pub error:Option<&'static str>}
impl Ambient{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{Self::with_samples(device,format,1)}
 pub fn with_samples(device:&wgpu::Device,format:wgpu::TextureFormat,count:u32)->Self{Self{pipes:Variants::new(crate::ambient_pipeline::Pipelines::with_samples(device,format,count),count),format,sources:crate::ambient_source_gpu::Sources::with_samples(device,count),frame:device.create_buffer(&wgpu::BufferDescriptor{label:Some("current ambient camera"),size:320,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false}),state:None,cached:None,preparations:0,writes:0,passes:0,error:None}}
 pub fn set_samples(&mut self,device:&wgpu::Device,count:u32){if self.pipes.select(count,||crate::ambient_pipeline::Pipelines::with_samples(device,self.format,count)){self.sources.set_samples(device,count);self.cached=None;}}
 pub fn clear(&mut self){self.state=None;self.cached=None;self.error=None;}
 fn prepare(&mut self,device:&wgpu::Device,full:[u32;2],density:f32,depth:&wgpu::TextureView,sections:&wgpu::TextureView,clip:&wgpu::Buffer){
  let(w,h)=crate::pyramid_size::resolution((full[0],full[1]),f64::from(density));let size=[w.max(32),h.max(32)];
  if self.state.as_ref().is_none_or(|s|s.images.full!=full||s.images.size!=size){let images=Images::with_samples(device,full,density,self.pipes.samples);let groups=Groups::new(device,&self.pipes.active,&images,&self.frame,self.pipes.samples);let group=Groups::source(device,&self.pipes.active,&images,depth,sections,&self.frame,clip);self.state=Some(State{images,groups,input:(depth.clone(),sections.clone(),group)});self.cached=None;self.preparations+=1;}
  let state=self.state.as_mut().unwrap();
  if state.images.source_radius.texture.sample_count()!=self.pipes.samples {state.images.retarget_sources(device,self.pipes.samples);state.groups=Groups::new(device,&self.pipes.active,&state.images,&self.frame,self.pipes.samples);state.input=(depth.clone(),sections.clone(),Groups::source(device,&self.pipes.active,&state.images,depth,sections,&self.frame,clip));self.cached=None;self.preparations+=1;}
if state.input.0!=*depth||state.input.1!=*sections{state.input=(depth.clone(),sections.clone(),Groups::source(device,&self.pipes.active,&state.images,depth,sections,&self.frame,clip));self.cached=None;}
 }
 pub fn draw(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,clip:&wgpu::Buffer,sections:&wgpu::TextureView,meshes:&[crate::gpu_mesh::GpuMesh],matrix:[f32;16],full:[u32;2],density:f32,ground:[f32;2],revision:[u64;2],records:Option<&crate::instance_gpu::Lane>){
  if ground[1]<=0.0{self.clear();return;}if matrix.iter().any(|v|!v.is_finite())||full.contains(&0)||!density.is_finite()||density<=0.{self.error=Some("Invalid ambient view");return;}
  self.prepare(device,full,density,depth,sections,clip);let state=self.state.as_ref().unwrap();let Some(uniform)=crate::ambient_camera::uniform(matrix,full,state.images.size,ground)else{self.error=Some("Invalid ambient camera inverse");return;};
  if self.cached!=Some((uniform,revision)){
   queue.write_buffer(&self.frame,0,&uniform.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());self.writes+=1;
   if self.pipes.samples==4 {encoder.clear_buffer(&state.groups.buffers[1],0,None);encoder.clear_buffer(&state.groups.buffers[3],4,Some(4));}
   let images=&state.images;let groups=&state.groups;let source=&state.input.2;self.sources.encode(encoder,&images.source_radius.view,&images.source_normals.view,depth,view,clipping,meshes,records);self.passes+=1;
   draw(encoder,&self.pipes.active.prepare,&[&images.linear.levels[0],&images.radii.levels[0],&images.normals.view],&[(0,source)],true);
   for level in 1..6{draw(encoder,&self.pipes.active.reduce,&[&images.linear.levels[level],&images.radii.levels[level]],&[(0,&groups.levels[level-1])],true);}self.passes+=6;
   draw(encoder,&self.pipes.active.occupancy,&[&images.occupancy.view],&[(0,&groups.levels[0])],true);
   draw(encoder,&self.pipes.active.ground,&[&images.ground.view],&[(0,source),(1,&groups.levels[0]),(2,&groups.occupancy),(3,&groups.history)],true);
   draw(encoder,&self.pipes.active.raw,&[&images.ao[0].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ground),(3,&groups.history)],true);
   draw(encoder,&self.pipes.active.filter[0],&[&images.ao[1].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[0]),(3,&groups.history)],true);
   draw(encoder,&self.pipes.active.filter[1],&[&images.ao[0].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[1]),(3,&groups.history)],true);
   draw(encoder,&self.pipes.active.upsample,&[&images.ao[2].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[0]),(3,&groups.history)],true);self.passes+=6;
   self.cached=Some((uniform,revision));
  }
  {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("ambient sample-preserving composition"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:color,resolve_target:None,depth_slice:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&self.pipes.active.composite);pass.set_bind_group(0,&state.groups.composite,&[]);pass.set_bind_group(1,&state.groups.ao[2],&[]);pass.draw(0..3,0..1);if let Some(edges)=&self.pipes.active.edges{pass.set_pipeline(edges);pass.draw_indirect(&state.groups.buffers[3],0);}}
  self.passes+=1+u64::from(self.pipes.samples==4);self.error=None;
 }
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.state.as_ref().map_or(0,|s|s.images.bytes()+s.groups.buffers.iter().map(wgpu::Buffer::size).sum::<u64>()),1,self.preparations,self.writes,self.passes]}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-01.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-02.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-03.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-04.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-05.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-06.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-07.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-08.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-09.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-10.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-11.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-12.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-13.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-14.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-15.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-16.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-17.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-18.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-19.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-20.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-21.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-22.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-23.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-24.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-25.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-26.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-27.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-28.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-29.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-30.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-31.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-32.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-33.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-34.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-35.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-36.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-37.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-38.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-39.rs"
```

</details>

### 2. `src/ambient_groups.rs`

Retain coarse ambient images while matching physical inputs and execute bounded sparse per-sample corrections without runtime readback flags.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Groups{pub levels:[wgpu::BindGroup;6],pub occupancy:wgpu::BindGroup,pub ground:wgpu::BindGroup,pub ao:[wgpu::BindGroup;3],pub history:wgpu::BindGroup,pub composite:wgpu::BindGroup,pub buffers:[wgpu::Buffer;4]}
fn sampled(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,view:&wgpu::TextureView,sampler:&wgpu::Sampler)->wgpu::BindGroup{device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient retained sample"),layout,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(view)},wgpu::BindGroupEntry{binding:1,resource:wgpu::BindingResource::Sampler(sampler)}]})}
impl Groups{
 pub fn new(device:&wgpu::Device,pipes:&crate::ambient_pipeline::Pipelines,images:&crate::ambient_images::Images,frame:&wgpu::Buffer)->Self{
  let levels=std::array::from_fn(|i|device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient retained depth and properties"),layout:&pipes.level,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&images.linear.levels[i])},wgpu::BindGroupEntry{binding:1,resource:wgpu::BindingResource::TextureView(&images.radii.levels[i])},wgpu::BindGroupEntry{binding:2,resource:wgpu::BindingResource::TextureView(&images.normals.view)}]}));
  let sampler=device.create_sampler(&wgpu::SamplerDescriptor{mag_filter:wgpu::FilterMode::Linear,min_filter:wgpu::FilterMode::Linear,..Default::default()});let occupancy=sampled(device,&pipes.sample,&images.occupancy.view,&sampler);let ground=sampled(device,&pipes.sample,&images.ground.view,&sampler);let ao=std::array::from_fn(|i|sampled(device,&pipes.sample,&images.ao[i].view,&sampler));
  let buffers=std::array::from_fn(|i|device.create_buffer(&wgpu::BufferDescriptor{label:Some("ambient reserved single-sample work"),size:16,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST|if i==3{wgpu::BufferUsages::INDIRECT}else{wgpu::BufferUsages::empty()},mapped_at_creation:false}));let mut entries=buffers.iter().enumerate().map(|(i,b)|wgpu::BindGroupEntry{binding:i as u32,resource:b.as_entire_binding()}).collect::<Vec<_>>();entries.push(wgpu::BindGroupEntry{binding:4,resource:wgpu::BindingResource::TextureView(&images.history.view)});entries.push(wgpu::BindGroupEntry{binding:5,resource:wgpu::BindingResource::TextureView(&images.history_depth.view)});let history=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient inactive single-sample history"),layout:&pipes.history,entries:&entries});let composite=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient sparse sample corrections"),layout:&pipes.composite_layout,entries:&[wgpu::BindGroupEntry{binding:0,resource:frame.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:buffers[0].as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:buffers[2].as_entire_binding()}]});Self{levels,occupancy,ground,ao,history,composite,buffers}
 }
 pub fn source(device:&wgpu::Device,pipes:&crate::ambient_pipeline::Pipelines,images:&crate::ambient_images::Images,depth:&wgpu::TextureView,sections:&wgpu::TextureView,frame:&wgpu::Buffer,clip:&wgpu::Buffer)->wgpu::BindGroup{
  let entries=[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(depth)},wgpu::BindGroupEntry{binding:1,resource:frame.as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:wgpu::BindingResource::TextureView(&images.source_radius.view)},wgpu::BindGroupEntry{binding:3,resource:wgpu::BindingResource::TextureView(&images.source_normals.view)},wgpu::BindGroupEntry{binding:4,resource:wgpu::BindingResource::TextureView(sections)},wgpu::BindGroupEntry{binding:5,resource:clip.as_entire_binding()}];device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient exact current input"),layout:&pipes.source,entries:&entries})
 }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Groups{pub levels:[wgpu::BindGroup;6],pub occupancy:wgpu::BindGroup,pub ground:wgpu::BindGroup,pub ao:[wgpu::BindGroup;3],pub history:wgpu::BindGroup,pub composite:wgpu::BindGroup,pub buffers:[wgpu::Buffer;4]}
fn sampled(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,view:&wgpu::TextureView,sampler:&wgpu::Sampler)->wgpu::BindGroup{device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient retained sample"),layout,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(view)},wgpu::BindGroupEntry{binding:1,resource:wgpu::BindingResource::Sampler(sampler)}]})}
impl Groups{
 pub fn new(device:&wgpu::Device,pipes:&crate::ambient_pipeline::Pipelines,images:&crate::ambient_images::Images,frame:&wgpu::Buffer,samples:u32)->Self{
  let levels=std::array::from_fn(|i|device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient retained depth and properties"),layout:&pipes.level,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&images.linear.levels[i])},wgpu::BindGroupEntry{binding:1,resource:wgpu::BindingResource::TextureView(&images.radii.levels[i])},wgpu::BindGroupEntry{binding:2,resource:wgpu::BindingResource::TextureView(&images.normals.view)}]}));
  let sampler=device.create_sampler(&wgpu::SamplerDescriptor{mag_filter:wgpu::FilterMode::Linear,min_filter:wgpu::FilterMode::Linear,..Default::default()});let occupancy=sampled(device,&pipes.sample,&images.occupancy.view,&sampler);let ground=sampled(device,&pipes.sample,&images.ground.view,&sampler);let ao=std::array::from_fn(|i|sampled(device,&pipes.sample,&images.ao[i].view,&sampler));
  let pixels=u64::from(images.full[0])*u64::from(images.full[1]);let tiles=u64::from(images.full[0].div_ceil(16))*u64::from(images.full[1].div_ceil(16));
  let buffers=std::array::from_fn(|i|{let buffer=device.create_buffer(&wgpu::BufferDescriptor{label:Some("ambient bounded sample work"),size:if i==3||samples==1{16}else if i==0{pixels*4}else{tiles*4},usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST|if i==3{wgpu::BufferUsages::INDIRECT}else{wgpu::BufferUsages::empty()},mapped_at_creation:i==3});if i==3{buffer.slice(0..4).get_mapped_range_mut().copy_from_slice(&6u32.to_ne_bytes());buffer.unmap();}buffer});let mut entries=buffers.iter().enumerate().map(|(i,b)|wgpu::BindGroupEntry{binding:i as u32,resource:b.as_entire_binding()}).collect::<Vec<_>>();entries.push(wgpu::BindGroupEntry{binding:4,resource:wgpu::BindingResource::TextureView(&images.history.view)});entries.push(wgpu::BindGroupEntry{binding:5,resource:wgpu::BindingResource::TextureView(&images.history_depth.view)});let history=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient inactive single-sample history"),layout:&pipes.history,entries:&entries});let composite=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient sparse sample corrections"),layout:&pipes.composite_layout,entries:&[wgpu::BindGroupEntry{binding:0,resource:frame.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:buffers[0].as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:buffers[2].as_entire_binding()}]});Self{levels,occupancy,ground,ao,history,composite,buffers}
 }
 pub fn source(device:&wgpu::Device,pipes:&crate::ambient_pipeline::Pipelines,images:&crate::ambient_images::Images,depth:&wgpu::TextureView,sections:&wgpu::TextureView,frame:&wgpu::Buffer,clip:&wgpu::Buffer)->wgpu::BindGroup{
  let entries=[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(depth)},wgpu::BindGroupEntry{binding:1,resource:frame.as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:wgpu::BindingResource::TextureView(&images.source_radius.view)},wgpu::BindGroupEntry{binding:3,resource:wgpu::BindingResource::TextureView(&images.source_normals.view)},wgpu::BindGroupEntry{binding:4,resource:wgpu::BindingResource::TextureView(sections)},wgpu::BindGroupEntry{binding:5,resource:clip.as_entire_binding()}];device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient exact current input"),layout:&pipes.source,entries:&entries})
 }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-40.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-41.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-42.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-43.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-44.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-45.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-46.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-47.rs"
```

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-48.rs"
```

</details>

### 3. `src/renderer.rs`

Late-enabled ambient sources and colour recipes adopt the current frame count.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn set_ambient(&self,enabled:bool){if self.ambient_enabled.replace(enabled)==enabled{return;}let mut ambient=self.ambient.borrow_mut();if enabled{ambient.get_or_insert_with(||crate::ambient_gpu::Ambient::new(&self.device,self.format));}else if let Some(owner)=ambient.as_mut(){owner.clear();}}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn set_ambient(&self,enabled:bool){if self.ambient_enabled.replace(enabled)==enabled{return;}let mut ambient=self.ambient.borrow_mut();if enabled{let owner=ambient.get_or_insert_with(||crate::ambient_gpu::Ambient::with_samples(&self.device,self.format,self.targets.samples));owner.set_samples(&self.device,self.targets.samples);}else if let Some(owner)=ambient.as_mut(){owner.clear();}}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-49.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/ambient_sample_inspect.rs`

Supplied Native actual GPU resource handles expose existing inputs for independent compute; production images/buffers do not gain readback flags.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-50.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/ambient_gpu.rs`

Register only the optional Native original-property/sparse-work observer.

<details>
<summary>Locate the existing block</summary>

```text
uffers.iter().map(wgpu::Buffer::size).sum::<u64>()),1,self.preparations,self.writes,self.passes]}
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-51.rs" skip=100
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/sample_surface_tests.rs`

Extend the supplied Native count fixture to enabled ambient resources while retaining every prior owner.

<details>
<summary>Locate the existing block</summary>

```text
fn retarget_basic_fixture(r:&mut Renderer,count:u32) {
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-52.rs" skip=55
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/sample_surface_tests.rs`

Read all actual physical depth/address/radius/normal samples, bounded sparse tiles/indirect work, retained coarse images and exact current output through repeated counts and ordinary/shared source paths.

<details>
<summary>Locate the existing block</summary>

```text
,"Actual4x horizontal section and full solid share the same outer silhouette");crate::sample_test_proof::write(root.join("uncut-four.rgba"),uncut).unwrap();
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-53.rs" skip=160
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 22.5.16
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Ambient rendering supports matching physical original properties with retained coarse images and bounded sparse correction draws. Real GPU readback verifies every physical property, unique tile work and indirect counts through exact sample returns. Complete browser rendering remains one-sample until all consumers are integrated.

### Native ambient check

These actual Native GPU pictures show the original ambient scene at one and four samples and its plain four-sample control. Independent GPU readback checks every physical original property, unique bounded correction tiles and actual indirect counts. Coarse images, original sources and geometry remain retained, exact count returns and ordinary/shared equality pass. Complete browser rendering remains single-sample until all remaining consumers are connected.

**original ambient scene, one sample — actual ambient correction check.**

![Actual Native component: original ambient scene, one sample](../screenshots/journey/review/96zzzzzzzi-correct-ambient-at-physical-samples-ambient-0.png)

**original ambient scene, four samples — actual ambient correction check.**

![Actual Native component: original ambient scene, four samples](../screenshots/journey/review/96zzzzzzzi-correct-ambient-at-physical-samples-ambient-1.png)

**plain control without ambient, four samples — actual ambient correction check.**

![Actual Native component: plain control without ambient, four samples](../screenshots/journey/review/96zzzzzzzi-correct-ambient-at-physical-samples-ambient-2.png)


**Verified checkpoint in Chrome.**

**Critical browser-scene review accepted.**

Actual single-sample original contact browser fixture. Both densities preserve original owners, complete source archives, shared geometry, contact shading and exact resize restoration. Separate labelled Native pictures show actual one/four-sample ambient output and its plain four-sample control. Complete physical-sample browser integration and whole-course feature review remain pending.

![Actual single-sample original contact browser fixture. Both densities preserve original owners, complete source archives, shared geometry, contact shading and exact resize restoration. Separate labelled Native pictures show actual one/four-sample ambient output and its plain four-sample control. Complete physical-sample browser integration and whole-course feature review remain pending.](../screenshots/journey/96zzzzzzt-keep-contact-ambient-inputs-on-originals-browser.png)

[Verification scope](release.md).

This reference image is shared with [22.4.3 · Keep contact ambient inputs on originals](96zzzzzzt-keep-contact-ambient-inputs-on-originals.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

A real count change selects the cached recipes, retargets only the two full-resolution source images and refreshes incompatible bindings. Coarse hierarchy, normal and filtered ambient images stay retained. Late-enabled ambient rendering adopts the current frame count. Repeating a count keeps the current resources.

Reserve one correction word per physical pixel and bounded tile flags/list storage for four-sample frames. Keep the indirect command four words long, initialize its six-vertex count once and clear only changed-frame flags and its instance count before generating work. Single-sample frames retain the small reserved buffers. Production resources do not gain readback flags.

Compose the retained filtered result across the current colour target, then use the actual indirect command to draw sparse per-sample alpha corrections. The supplied Native observer reads the existing resources through independent GPU compute: every original depth/address/radius/normal sample must agree with source geometry, correction tiles must be unique and bounded, and the real draw count must match the list. It also checks exact return pixels, retained coarse image identities, unchanged uploads and identical ordinary/shared inputs.

Matching original physical inputs → representative coarse shading → changed-sample corrections → unique bounded tiles → indirect colour draw.



What prevents correction work from accumulating across changed frames?

Clear the bounded tile flags and indirect instance count before generating changed-frame work, while retaining the fixed six-vertex count and cached images. Unique flagged tiles produce a bounded list and actual indirect instance count; unchanged frames reuse the prepared result.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Run the actual ambient witness at4→1→4. Inspect both shaded frames and the separate plain four-sample control. Require exact return pixels, retained coarse image identities and source geometry, and unchanged source uploads. Read the real correction JSON and require covered samples, mixed original radii/addresses, nonzero corrections, unique bounded tiles and repeatable indirect counts.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 22.5.16
npm --prefix ../session_tests run course -- save 22.5.16
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Original physical depth, integer addresses, radius and geometric normals stay matched at every sample. The cached ambient owner retargets only full-resolution original-property images and incompatible bindings/work storage; coarse images and source geometry remain retained. Per-pixel correction words and unique tile flags/list storage are bounded by the actual viewport. The four-word indirect command retains six vertices and clears only flags and its instance count before changed-frame work. Actual indirect sparse per-sample alpha corrects the retained filtered composition; production resources gain no readback flags. Independent Native GPU compute verifies every original property, bounded unique tiles, nonzero and repeatable real indirect work, exact count-return pixels, coarse image identities, ordinary/shared equality and full source/geometry/upload retention. Late-enabled ambient adopts the frame count. Complete browser rendering remains single-sample until every consumer is integrated. All 1004 Native tests, WASM/optimized builds, Native frame, canonical Chrome contact/resize checks and stored evidence pass. Ten current canonical Native/browser pictures were individually inspected. Every unique current Native component figure was also individually inspected. Complete physical-sample viewer integration, adaptive quality, remaining view controls, runtime efficiency, Cargo independence and final whole-course feature-picture review remain unfinished. Maintained Session reload camera/base-grid fixes follow the completed course.

Retain every prior source, contact, outline and shader contract assertion. Add actual all-sample original depth/address/radius/normal readback, bounded unique sparse tile/indirect work, per-change resets, exact return pixels, coarse image identities, ordinary/shared equality and original source/geometry/upload retention.

Actual single-sample original contact browser fixture. Both densities preserve original owners, complete source archives, shared geometry, contact shading and exact resize restoration. Separate labelled Native pictures show actual one/four-sample ambient output and its plain four-sample control. Complete physical-sample browser integration and whole-course feature review remain pending.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 22.5.16
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
