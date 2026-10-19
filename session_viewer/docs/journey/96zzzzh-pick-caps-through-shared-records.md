# 20.10.9 · Pick caps through shared records

**Chapter 20 · Shared GPU rendering · Section 20.10 · Projection and cap consumers** · [Contents](contents.md)

**Typing: 18–35 minutes.** [Estimate](typing-load.md).

Connect both cap crossing passes to current records and retain original owners.

## Type

Continue from [Own shared record cap pick resources](96zzzzg-own-shared-record-cap-pick-resources.md). [Save or recover your work](recovery.md).

### 1. `src/cap_pick_gpu.rs`

Prepare immutable current arena groups before encoding either crossing pass. Use the existing renderer record owner without introducing another table. Prepare exact target-plane groups once and reuse them across count and nearest-owner passes. Count and resolve owners from current records, retaining the original uniform path for changed packed records.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{cap_image::Image,pick_window::Window};
pub struct Pick{geometry:crate::cap_geometry::Geometry,crop:Option<wgpu::Buffer>,pipes:crate::cap_pick_pipeline::Pipelines,records:wgpu::Buffer,owners:wgpu::Buffer,frame:wgpu::Buffer,frame_group:wgpu::BindGroup,record_group:Option<wgpu::BindGroup>,owner_group:Option<wgpu::BindGroup>,target:Option<Image>,pub preparations:u64}
impl Pick{
 pub fn new(device:&wgpu::Device)->Self{let pipes=crate::cap_pick_pipeline::Pipelines::new(device);let buffer=|label,size,usage|device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size,usage:usage|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let records=buffer("bounded original cap crossings",16,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_SRC);let owners=buffer("exact original cap owners",8,wgpu::BufferUsages::STORAGE);let frame=buffer("cap physical crop frame",32,wgpu::BufferUsages::UNIFORM);let frame_group=crate::gpu_layout::group(device,&pipes.frame,&frame);Self{geometry:Default::default(),crop:None,pipes,records,owners,frame,frame_group,record_group:None,owner_group:None,target:None,preparations:0}}
 fn capacity(device:&wgpu::Device,needed:u64,buffer:&mut wgpu::Buffer,label:&str)->Result<(),&'static str>{if needed>u64::from(device.limits().max_storage_buffer_binding_size){return Err("Cap picking exceeds the device storage binding limit")}if buffer.size()<needed{*buffer=device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size:needed,usage:buffer.usage(),mapped_at_creation:false});}Ok(())}
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,window:Window,crop:&wgpu::Buffer,rows:&[[u32;2]],planes:u32,mode:crate::pick_sources::Mode)->Result<(),&'static str>{
  self.geometry.invalidate();self.geometry.set_planes(device,planes)?;self.crop=Some(crop.clone());
  let size=[window.draw[2],window.draw[3]];let bytes=u64::from(size[0]).checked_mul(u64::from(size[1])).and_then(|n|n.checked_mul(u64::from(planes))).and_then(|n|n.checked_mul(16)).ok_or("Cap pick storage overflow")?;Self::capacity(device,bytes.max(16),&mut self.records,"bounded cap crossings")?;let owners=rows.iter().flatten().copied().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();Self::capacity(device,(owners.len()as u64).max(8),&mut self.owners,"original cap owner map")?;if !owners.is_empty(){queue.write_buffer(&self.owners,0,&owners);}
  let words=[window.full[0],window.full[1],size[0],size[1],window.draw[0],window.draw[1],mode as u32,0];queue.write_buffer(&self.frame,0,&words.into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
  self.record_group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("cap record frame"),layout:&self.pipes.records,entries:&[wgpu::BindGroupEntry{binding:0,resource:self.records.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:crop.as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:self.frame.as_entire_binding()}]}));
  self.owner_group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("original cap owners"),layout:&self.pipes.owners,entries:&[wgpu::BindGroupEntry{binding:0,resource:self.records.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:self.owners.as_entire_binding()}]}));if self.target.as_ref().is_none_or(|t|t.size!=size){self.target=Some(Image::new(device,size,wgpu::TextureFormat::R8Unorm,1));}self.preparations+=1;Ok(())
 }
 pub fn encode(&self,encoder:&mut wgpu::CommandEncoder,image:&crate::pick_image::Image,window:Window,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,meshes:&[crate::gpu_mesh::GpuMesh],targets:&std::collections::HashMap<u32,u32>,planes:u32){
  let Some(target)=&self.target else{return};encoder.clear_buffer(&self.records,0,None);
  for pipeline in [&self.pipes.count,&self.pipes.nearest]{let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section count and nearest owner"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&target.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(pipeline);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);pass.set_bind_group(3,self.record_group.as_ref().unwrap(),&[]);for plane in 0..planes{for mesh in meshes.iter().filter(|m|m.closed()&&targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)){mesh.bind_model(&mut pass);mesh.geometry.draw_range(&mut pass,plane..plane+1);}}}
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section integer owners"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&self.pipes.ids);pass.set_bind_group(0,&self.frame_group,&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,self.owner_group.as_ref().unwrap(),&[]);pass.draw(0..3,0..planes);let _=window;
 }
 pub fn bytes(&self)->u64{self.geometry.bytes()+self.records.size()+self.owners.size()+self.frame.size()+self.target.as_ref().map_or(0,|t|t.bytes)}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{cap_image::Image,pick_window::Window};
pub struct Pick{geometry:crate::cap_geometry::Geometry,crop:Option<wgpu::Buffer>,pipes:crate::cap_pick_pipeline::Pipelines,records:wgpu::Buffer,owners:wgpu::Buffer,frame:wgpu::Buffer,frame_group:wgpu::BindGroup,record_group:Option<wgpu::BindGroup>,owner_group:Option<wgpu::BindGroup>,target:Option<Image>,pub preparations:u64}
impl Pick{
 pub fn new(device:&wgpu::Device)->Self{let pipes=crate::cap_pick_pipeline::Pipelines::new(device);let buffer=|label,size,usage|device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size,usage:usage|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let records=buffer("bounded original cap crossings",16,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_SRC);let owners=buffer("exact original cap owners",8,wgpu::BufferUsages::STORAGE);let frame=buffer("cap physical crop frame",32,wgpu::BufferUsages::UNIFORM);let frame_group=crate::gpu_layout::group(device,&pipes.frame,&frame);Self{geometry:Default::default(),crop:None,pipes,records,owners,frame,frame_group,record_group:None,owner_group:None,target:None,preparations:0}}
 fn capacity(device:&wgpu::Device,needed:u64,buffer:&mut wgpu::Buffer,label:&str)->Result<(),&'static str>{if needed>u64::from(device.limits().max_storage_buffer_binding_size){return Err("Cap picking exceeds the device storage binding limit")}if buffer.size()<needed{*buffer=device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size:needed,usage:buffer.usage(),mapped_at_creation:false});}Ok(())}
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,window:Window,crop:&wgpu::Buffer,rows:&[[u32;2]],planes:u32,mode:crate::pick_sources::Mode)->Result<(),&'static str>{
  self.geometry.invalidate();self.geometry.set_planes(device,planes)?;self.crop=Some(crop.clone());
  let size=[window.draw[2],window.draw[3]];let bytes=u64::from(size[0]).checked_mul(u64::from(size[1])).and_then(|n|n.checked_mul(u64::from(planes))).and_then(|n|n.checked_mul(16)).ok_or("Cap pick storage overflow")?;Self::capacity(device,bytes.max(16),&mut self.records,"bounded cap crossings")?;let owners=rows.iter().flatten().copied().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();Self::capacity(device,(owners.len()as u64).max(8),&mut self.owners,"original cap owner map")?;if !owners.is_empty(){queue.write_buffer(&self.owners,0,&owners);}
  let words=[window.full[0],window.full[1],size[0],size[1],window.draw[0],window.draw[1],mode as u32,0];queue.write_buffer(&self.frame,0,&words.into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
  self.record_group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("cap record frame"),layout:&self.pipes.records,entries:&[wgpu::BindGroupEntry{binding:0,resource:self.records.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:crop.as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:self.frame.as_entire_binding()}]}));
  self.owner_group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("original cap owners"),layout:&self.pipes.owners,entries:&[wgpu::BindGroupEntry{binding:0,resource:self.records.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:self.owners.as_entire_binding()}]}));if self.target.as_ref().is_none_or(|t|t.size!=size){self.target=Some(Image::new(device,size,wgpu::TextureFormat::R8Unorm,1));}self.preparations+=1;Ok(())
 }
 pub fn encode(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,image:&crate::pick_image::Image,window:Window,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,meshes:&[crate::gpu_mesh::GpuMesh],targets:&std::collections::HashMap<u32,u32>,planes:u32,records:Option<&crate::instance_gpu::Lane>){
  let Some(target)=&self.target else{return};encoder.clear_buffer(&self.records,0,None);
  let extra=[wgpu::BindGroupEntry{binding:0,resource:self.records.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:self.crop.as_ref().unwrap().as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:self.frame.as_entire_binding()}];let rows=(0..planes).map(|plane|meshes.iter().filter(|m|m.closed()&&targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)).map(|mesh|{let group=records.and_then(|records|records.current_row(mesh)).map(|_|self.geometry.group_with(device,&self.pipes.record_geometry,&mesh.geometry,plane,3,&extra).expect("Prepared cap pick plane"));(mesh,group)}).collect::<Vec<_>>()).collect::<Vec<_>>();
  for(pipeline,shared)in [(&self.pipes.count,&self.pipes.record_count),(&self.pipes.nearest,&self.pipes.record_nearest)]{let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section count and nearest owner"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&target.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(pipeline);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);pass.set_bind_group(3,self.record_group.as_ref().unwrap(),&[]);for(plane,rows)in rows.iter().enumerate(){for(mesh,group)in rows{if let Some(group)=group.as_ref().filter(|_|records.is_some_and(|records|records.bind_record(&mut pass,mesh))){pass.set_pipeline(shared);pass.set_bind_group(3,group,&[]);pass.draw(0..mesh.geometry.index_count,0..1);}else{pass.set_pipeline(pipeline);pass.set_bind_group(3,self.record_group.as_ref().unwrap(),&[]);mesh.bind_model(&mut pass);mesh.geometry.draw_range(&mut pass,plane as u32..plane as u32+1);}}}}
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section integer owners"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&self.pipes.ids);pass.set_bind_group(0,&self.frame_group,&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,self.owner_group.as_ref().unwrap(),&[]);pass.draw(0..3,0..planes);let _=window;
 }
 pub fn bytes(&self)->u64{self.geometry.bytes()+self.records.size()+self.owners.size()+self.frame.size()+self.target.as_ref().map_or(0,|t|t.bytes)}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzh-pick-caps-through-shared-records-01.rs"
```

```text
--8<-- "journey/code/96zzzzh-pick-caps-through-shared-records-02.rs"
```

```text
--8<-- "journey/code/96zzzzh-pick-caps-through-shared-records-03.rs"
```

```text
--8<-- "journey/code/96zzzzh-pick-caps-through-shared-records-04.rs"
```

</details>

### 2. `src/caps_gpu.rs`

Thread the same optional object owner through cap picking. Keep preparation and encoding under the existing bounded cap pick owner.

<details>
<summary>Locate the existing block</summary>

```rust
use wgpu::util::DeviceExt;
use crate::{gpu_mesh::GpuMesh,solid_orientation::CLOSED};
use crate::cap_image::Image;
struct Counts{image:Image,group:wgpu::BindGroup,geometry:crate::cap_geometry::Geometry}
pub struct Caps{pub enabled:bool,planes:u32,pick:Option<crate::cap_pick_gpu::Pick>,pub pick_error:Option<&'static str>,bounds:crate::cap_source_bounds::Bounds,targets:std::collections::HashMap<u32,u32>,pub solids:crate::solid_cache::Solids,pipes:Option<crate::cap_pipeline::Pipelines>,counts:Option<Counts>,frame:wgpu::Buffer,frame_group:Option<wgpu::BindGroup>,pub rows:Vec<[u32;2]>,pub draws:u64,pub passes:u64}
impl Caps{
 pub fn new(device:&wgpu::Device)->Self{let frame=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("section canvas frame"),contents:&[0;16],usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST});Self{enabled:true,planes:0,pick:None,pick_error:None,bounds:Default::default(),targets:Default::default(),solids:Default::default(),pipes:None,counts:None,frame,frame_group:None,rows:Vec::new(),draws:0,passes:0}}
 pub fn set_scene(&mut self,queue:&wgpu::Queue,scene:&crate::scene::Scene,meshes:&mut[GpuMesh],planes:&[crate::clipping_rows::Row]){
  if let Some(counts)=self.counts.as_mut(){counts.geometry.invalidate();}
  self.planes=planes.len()as u32;self.rows.clear();self.targets.clear();for(mesh,object)in meshes.iter_mut().zip(scene.objects().iter().filter(|r|scene.visible(r.id))){assert_eq!(mesh.id,object.id);let flags=self.solids.flags(object);let token=if !planes.is_empty()&&flags&CLOSED!=0{self.rows.push([mesh.id.value(),u32::from(mesh.pickable())]);self.rows.len()as u32}else{0};mesh.set_caps(queue,flags,token);if token>0{if let Some(bounds)=self.bounds.placed(&object.mesh,&object.model){let mask=planes.iter().enumerate().fold(0,|mask,(i,row)|mask|if row.plane.crosses(&bounds,[0.;3]){1<<i}else{0});self.targets.insert(mesh.id.value(),mask);}}}
  self.solids.prune();self.bounds.prune();if self.rows.is_empty(){self.rows.shrink_to_fit();self.targets.shrink_to_fit();}
 }
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,size:[u32;2],active:bool){
  let size=if active&&!self.rows.is_empty(){size}else{[1,1]};
  if size==[1,1]{self.counts=None;return;}
  if self.pipes.is_none(){let pipes=crate::cap_pipeline::Pipelines::new(device,format);self.frame_group=Some(crate::gpu_layout::group(device,&pipes.frame,&self.frame));self.pipes=Some(pipes);}
  let pipes=self.pipes.as_ref().unwrap();if self.counts.as_ref().is_none_or(|c|c.image.size!=size){let image=Image::new(device,size,wgpu::TextureFormat::R16Float,2);let group=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("original section crossings"),layout:&pipes.counts,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&image.view)}]});self.counts=Some(Counts{image,group,geometry:Default::default()});}
  self.counts.as_mut().unwrap().geometry.set_planes(device,self.planes).expect("Validated clipping plane count");
  let bytes=[size[0]as f32,size[1]as f32,0.,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();queue.write_buffer(&self.frame,0,&bytes);
 }
 pub fn draw(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,primitive:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],planes:u32,records:Option<&crate::instance_gpu::Lane>){
  let(Some(pipes),Some(counts))=(&self.pipes,&mut self.counts)else{return};
  for plane in 0..planes{
   let rows=meshes.iter().filter(|m|m.closed()&&self.targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)).map(|mesh|{let group=records.and_then(|records|records.current_row(mesh)).map(|_|counts.geometry.group(device,&pipes.geometry,&mesh.geometry,plane).expect("Prepared cap plane"));(mesh,group)}).collect::<Vec<_>>();
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original closed-solid crossings"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&counts.image.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&pipes.count);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);for(mesh,group)in &rows{if let Some(group)=group.as_ref().filter(|_|records.is_some_and(|records|records.bind_record(&mut pass,mesh))){pass.set_pipeline(&pipes.record_count);pass.set_bind_group(3,group,&[]);pass.draw(0..mesh.geometry.index_count,0..1);}else{pass.set_pipeline(&pipes.count);mesh.bind_model(&mut pass);mesh.geometry.draw_range(&mut pass,plane..plane+1);}self.draws+=1;}}
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section color and depth"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:color,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:primitive,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&pipes.color);pass.set_bind_group(0,self.frame_group.as_ref().unwrap(),&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,&counts.group,&[]);pass.draw(0..3,plane..plane+1);self.draws+=1;}
   self.passes+=2;
  }
 }
 pub fn encode_pick(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,image:&crate::pick_image::Image,window:crate::pick_window::Window,view:&wgpu::BindGroup,crop:&wgpu::Buffer,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],mode:crate::pick_sources::Mode){
  if !self.enabled||self.planes==0||self.rows.is_empty(){self.pick=None;self.pick_error=None;return;}
  let pick=self.pick.get_or_insert_with(||crate::cap_pick_gpu::Pick::new(device));if let Err(error)=pick.prepare(device,queue,window,crop,&self.rows,self.planes,mode){self.pick_error=Some(error);self.pick=None;return;}self.pick_error=None;pick.encode(encoder,image,window,view,clipping,meshes,&self.targets,self.planes);
 }
 pub fn pick_bytes(&self)->u64{self.pick.as_ref().map_or(0,crate::cap_pick_gpu::Pick::bytes)}
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.pick_bytes()+self.counts.as_ref().map_or(0,|c|c.image.bytes+c.geometry.bytes()),self.rows.len()as u64,self.solids.checks,self.draws,self.passes]}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use wgpu::util::DeviceExt;
use crate::{gpu_mesh::GpuMesh,solid_orientation::CLOSED};
use crate::cap_image::Image;
struct Counts{image:Image,group:wgpu::BindGroup,geometry:crate::cap_geometry::Geometry}
pub struct Caps{pub enabled:bool,planes:u32,pick:Option<crate::cap_pick_gpu::Pick>,pub pick_error:Option<&'static str>,bounds:crate::cap_source_bounds::Bounds,targets:std::collections::HashMap<u32,u32>,pub solids:crate::solid_cache::Solids,pipes:Option<crate::cap_pipeline::Pipelines>,counts:Option<Counts>,frame:wgpu::Buffer,frame_group:Option<wgpu::BindGroup>,pub rows:Vec<[u32;2]>,pub draws:u64,pub passes:u64}
impl Caps{
 pub fn new(device:&wgpu::Device)->Self{let frame=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("section canvas frame"),contents:&[0;16],usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST});Self{enabled:true,planes:0,pick:None,pick_error:None,bounds:Default::default(),targets:Default::default(),solids:Default::default(),pipes:None,counts:None,frame,frame_group:None,rows:Vec::new(),draws:0,passes:0}}
 pub fn set_scene(&mut self,queue:&wgpu::Queue,scene:&crate::scene::Scene,meshes:&mut[GpuMesh],planes:&[crate::clipping_rows::Row]){
  if let Some(counts)=self.counts.as_mut(){counts.geometry.invalidate();}
  self.planes=planes.len()as u32;self.rows.clear();self.targets.clear();for(mesh,object)in meshes.iter_mut().zip(scene.objects().iter().filter(|r|scene.visible(r.id))){assert_eq!(mesh.id,object.id);let flags=self.solids.flags(object);let token=if !planes.is_empty()&&flags&CLOSED!=0{self.rows.push([mesh.id.value(),u32::from(mesh.pickable())]);self.rows.len()as u32}else{0};mesh.set_caps(queue,flags,token);if token>0{if let Some(bounds)=self.bounds.placed(&object.mesh,&object.model){let mask=planes.iter().enumerate().fold(0,|mask,(i,row)|mask|if row.plane.crosses(&bounds,[0.;3]){1<<i}else{0});self.targets.insert(mesh.id.value(),mask);}}}
  self.solids.prune();self.bounds.prune();if self.rows.is_empty(){self.rows.shrink_to_fit();self.targets.shrink_to_fit();}
 }
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,size:[u32;2],active:bool){
  let size=if active&&!self.rows.is_empty(){size}else{[1,1]};
  if size==[1,1]{self.counts=None;return;}
  if self.pipes.is_none(){let pipes=crate::cap_pipeline::Pipelines::new(device,format);self.frame_group=Some(crate::gpu_layout::group(device,&pipes.frame,&self.frame));self.pipes=Some(pipes);}
  let pipes=self.pipes.as_ref().unwrap();if self.counts.as_ref().is_none_or(|c|c.image.size!=size){let image=Image::new(device,size,wgpu::TextureFormat::R16Float,2);let group=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("original section crossings"),layout:&pipes.counts,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&image.view)}]});self.counts=Some(Counts{image,group,geometry:Default::default()});}
  self.counts.as_mut().unwrap().geometry.set_planes(device,self.planes).expect("Validated clipping plane count");
  let bytes=[size[0]as f32,size[1]as f32,0.,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();queue.write_buffer(&self.frame,0,&bytes);
 }
 pub fn draw(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,primitive:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],planes:u32,records:Option<&crate::instance_gpu::Lane>){
  let(Some(pipes),Some(counts))=(&self.pipes,&mut self.counts)else{return};
  for plane in 0..planes{
   let rows=meshes.iter().filter(|m|m.closed()&&self.targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)).map(|mesh|{let group=records.and_then(|records|records.current_row(mesh)).map(|_|counts.geometry.group(device,&pipes.geometry,&mesh.geometry,plane).expect("Prepared cap plane"));(mesh,group)}).collect::<Vec<_>>();
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original closed-solid crossings"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&counts.image.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&pipes.count);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);for(mesh,group)in &rows{if let Some(group)=group.as_ref().filter(|_|records.is_some_and(|records|records.bind_record(&mut pass,mesh))){pass.set_pipeline(&pipes.record_count);pass.set_bind_group(3,group,&[]);pass.draw(0..mesh.geometry.index_count,0..1);}else{pass.set_pipeline(&pipes.count);mesh.bind_model(&mut pass);mesh.geometry.draw_range(&mut pass,plane..plane+1);}self.draws+=1;}}
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section color and depth"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:color,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:primitive,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&pipes.color);pass.set_bind_group(0,self.frame_group.as_ref().unwrap(),&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,&counts.group,&[]);pass.draw(0..3,plane..plane+1);self.draws+=1;}
   self.passes+=2;
  }
 }
 pub fn encode_pick(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,image:&crate::pick_image::Image,window:crate::pick_window::Window,view:&wgpu::BindGroup,crop:&wgpu::Buffer,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],mode:crate::pick_sources::Mode,records:Option<&crate::instance_gpu::Lane>){
  if !self.enabled||self.planes==0||self.rows.is_empty(){self.pick=None;self.pick_error=None;return;}
  let pick=self.pick.get_or_insert_with(||crate::cap_pick_gpu::Pick::new(device));if let Err(error)=pick.prepare(device,queue,window,crop,&self.rows,self.planes,mode){self.pick_error=Some(error);self.pick=None;return;}self.pick_error=None;pick.encode(device,encoder,image,window,view,clipping,meshes,&self.targets,self.planes,records);
 }
 pub fn pick_bytes(&self)->u64{self.pick.as_ref().map_or(0,crate::cap_pick_gpu::Pick::bytes)}
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.pick_bytes()+self.counts.as_ref().map_or(0,|c|c.image.bytes+c.geometry.bytes()),self.rows.len()as u64,self.solids.checks,self.draws,self.passes]}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzh-pick-caps-through-shared-records-05.rs"
```

```text
--8<-- "journey/code/96zzzzh-pick-caps-through-shared-records-06.rs"
```

</details>

### 3. `src/pick_encode.rs`

Share object records with first picks, source faces and cap picks in the existing submission.

<details>
<summary>Locate the existing block</summary>

```rust
        if let Some(caps)=caps{caps.encode_pick(device,&ctx.queue,&mut encoder,image,window,view_group,crop,clipping,meshes,mode);}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        if let Some(caps)=caps{caps.encode_pick(device,&ctx.queue,&mut encoder,image,window,view_group,crop,clipping,meshes,mode,records.map(|(rows,_)|rows));}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzh-pick-caps-through-shared-records-07.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/cap_pick_tests.rs`

Supply full-u32 shared/fallback owner comparison and restoration before a color frame, retaining all original geometry assertions.

<details>
<summary>Locate the existing block</summary>

```text
assert!(r.cap_usage()[0]>24);pixels(&r);
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzh-pick-caps-through-shared-records-08.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.10.9
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Cap picking consumes shared object records and retains original full integer identities, clipping and crop behavior.

### Recorded visual result

**Native render check — not a browser screenshot.**

![Native renderer output for pick caps through shared records.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*Read directly from this checkpoint’s GPU texture. Browser controls and event delivery remain unverified until the browser check passes.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Prepare each eligible object and plane group once. Both counting and nearest-owner passes pull current arena positions through the object slot, with an explicit plane independent of instance numbering. Keep the atomic crossing algorithms and final integer-owner pass. Changed packed records use the existing uniform fallback.

Current record and explicit plane → atomic crossings → original integer cap owner.



Why share geometry groups between both crossing passes?

They read the same object and plane during one pick; only the fragment operation changes.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Pick a cap before drawing a color frame, then pick the corner shared by two clipping planes. Repeat after selection and scene replacement.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.10.9
npm --prefix ../session_tests run course -- save 20.10.9
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Surface, projection, ambient and cap consumers now share records. Primitive lookup, allocation, reclamation, precise coordinates and broader viewer parity remain open.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.10.9
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
