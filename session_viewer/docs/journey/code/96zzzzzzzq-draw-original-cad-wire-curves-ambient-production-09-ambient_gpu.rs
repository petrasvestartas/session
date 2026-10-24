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
  self.prepare(device,full,density,depth,sections,clip);let state=self.state.as_ref().unwrap();let Some(uniform)=crate::ambient_camera::uniform_with_ground(matrix,full,state.images.size,state.images.ground.size,ground)else{self.error=Some("Invalid ambient camera inverse");return;};
  if self.cached!=Some((uniform,revision)){
   queue.write_buffer(&self.frame,0,&uniform.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());self.writes+=1;
   if self.pipes.samples==4 {encoder.clear_buffer(&state.groups.buffers[1],0,None);encoder.clear_buffer(&state.groups.buffers[3],4,Some(4));}
   let images=&state.images;let groups=&state.groups;let source=&state.input.2;self.sources.encode(encoder,&images.source_radius.view,images.source_normals.each_ref().map(|image|&image.view),depth,view,clipping,meshes,records);self.passes+=1;
   draw(encoder,&self.pipes.active.prepare,&[&images.linear.levels[0],&images.radii.levels[0],&images.normals.view],&[(0,source)],true);
   for level in 1..6{draw(encoder,&self.pipes.active.reduce,&[&images.linear.levels[level],&images.radii.levels[level]],&[(0,&groups.levels[level-1])],true);}self.passes+=6;
   draw(encoder,&self.pipes.active.occupancy,&[&images.occupancy.view],&[(0,&groups.levels[4])],true);
   draw(encoder,&self.pipes.active.ground,&[&images.ground.view],&[(0,source),(1,&groups.depth),(2,&groups.occupancy),(3,&groups.history)],true);
   draw(encoder,&self.pipes.active.raw,&[&images.ao[0].view],&[(0,source),(1,&groups.depth),(2,&groups.ground),(3,&groups.history)],true);
   draw(encoder,&self.pipes.active.filter[0],&[&images.ao[1].view],&[(0,source),(1,&groups.depth),(2,&groups.ao[0]),(3,&groups.history)],true);
   draw(encoder,&self.pipes.active.filter[1],&[&images.ao[0].view],&[(0,source),(1,&groups.depth),(2,&groups.ao[1]),(3,&groups.history)],true);
   draw(encoder,&self.pipes.active.upsample,&[&images.ao[2].view],&[(0,source),(1,&groups.depth),(2,&groups.ao[0]),(3,&groups.history)],true);self.passes+=6;
   self.cached=Some((uniform,revision));
  }
  {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("ambient sample-preserving composition"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:color,resolve_target:None,depth_slice:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&self.pipes.active.composite);pass.set_bind_group(0,&state.groups.composite,&[]);pass.set_bind_group(1,&state.groups.ao[2],&[]);pass.draw(0..3,0..1);if let Some(edges)=&self.pipes.active.edges{pass.set_pipeline(edges);pass.draw_indirect(&state.groups.buffers[3],0);}}
  self.passes+=1+u64::from(self.pipes.samples==4);self.error=None;
 }
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.state.as_ref().map_or(0,|s|s.images.bytes()+s.groups.buffers.iter().map(wgpu::Buffer::size).sum::<u64>()),1,self.preparations,self.writes,self.passes]}
}

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="ambient_sample_inspect.rs"]
mod sample_inspect;
