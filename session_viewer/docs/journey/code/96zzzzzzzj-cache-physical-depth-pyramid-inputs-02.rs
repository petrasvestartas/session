use crate::pyramid_image::Image;
struct Images{size:[u32;2],pub linear:Image,pub radii:Image,groups:[wgpu::BindGroup;5]}
pub struct Pyramid{pipes:crate::pyramid_pipeline::Pipelines,frame:wgpu::Buffer,images:Option<Images>,input:Option<(wgpu::TextureView,wgpu::BindGroup)>,cached:Option<([u32;16],[u32;2],u32,[u64;2])>,pub preparations:u64,pub writes:u64,pub passes:u64,pub error:Option<&'static str>}
impl Pyramid{
 pub fn new(device:&wgpu::Device)->Self{Self::with_samples(device,1)}
 pub fn with_samples(device:&wgpu::Device,samples:u32)->Self{Self{pipes:crate::pyramid_pipeline::Pipelines::with_samples(device,samples),frame:device.create_buffer(&wgpu::BufferDescriptor{label:Some("ambient earlier camera"),size:112,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false}),images:None,input:None,cached:None,preparations:0,writes:0,passes:0,error:None}}
 pub fn set_samples(&mut self,device:&wgpu::Device,samples:u32){if self.pipes.set_samples(device,samples){self.input=None;self.cached=None;}}
 fn prepare(&mut self,device:&wgpu::Device,depth:&wgpu::TextureView,full:[u32;2],density:f32){
  let (width,height)=crate::pyramid_size::resolution((full[0],full[1]),f64::from(density));let size=[width.max(32),height.max(32)];
  if self.images.as_ref().is_none_or(|images|images.size!=size){let linear=Image::new(device,size,wgpu::TextureFormat::R32Float);let radii=Image::new(device,size,wgpu::TextureFormat::R16Uint);let groups=std::array::from_fn(|i|device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient previous level"),layout:&self.pipes.level,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&linear.levels[i])},wgpu::BindGroupEntry{binding:1,resource:wgpu::BindingResource::TextureView(&radii.levels[i])}]}));self.images=Some(Images{size,linear,radii,groups});self.cached=None;self.preparations+=1;}
  if self.input.as_ref().is_none_or(|(old,_)|old!=depth){let group=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient source depth"),layout:&self.pipes.prepare.active.1,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(depth)},wgpu::BindGroupEntry{binding:1,resource:self.frame.as_entire_binding()}]});self.input=Some((depth.clone(),group));self.cached=None;}
 }
 pub fn encode(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,depth:&wgpu::TextureView,matrix:[f32;16],full:[u32;2],density:f32,revision:[u64;2]){
  if full.contains(&0)||!density.is_finite()||density<=0.||matrix.iter().any(|v|!v.is_finite()){self.error=Some("Invalid ambient depth view");self.cached=None;return;}
  self.prepare(device,depth,full,density);let key=(matrix.map(f32::to_bits),full,density.to_bits(),revision);if self.cached==Some(key){return;}
  let Some(inverse)=crate::clipping_uniform::inverse(&session_rust::Xform::from_matrix(matrix.map(f64::from)))else{self.error=Some("Ambient depth camera is singular");self.cached=None;return;};if inverse.m.iter().any(|v|!(*v as f32).is_finite()){self.error=Some("Ambient inverse exceeds display range");self.cached=None;return;}let images=self.images.as_ref().unwrap();let mut bytes=inverse.m.into_iter().flat_map(|v|(v as f32).to_ne_bytes()).collect::<Vec<_>>();bytes.extend([full[0]as f32,full[1]as f32,images.size[0]as f32,images.size[1]as f32,0.05,0.,0.,0.,0.,0.,0.,0.].into_iter().flat_map(f32::to_ne_bytes));queue.write_buffer(&self.frame,0,&bytes);self.writes+=1;
  for level in 0..6{let attachments=[&images.linear.levels[level],&images.radii.levels[level]].map(|view|Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}}));let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("ambient nearest positive depth"),color_attachments:&attachments,..Default::default()});if level==0{pass.set_pipeline(&self.pipes.prepare.active.0);pass.set_bind_group(0,&self.input.as_ref().unwrap().1,&[]);}else{pass.set_pipeline(&self.pipes.reduce);pass.set_bind_group(0,&images.groups[level-1],&[]);}pass.draw(0..3,0..1);self.passes+=1;}
  self.cached=Some(key);self.error=None;
 }
 pub fn usage(&self)->[u64;4]{[self.frame.size()+self.images.as_ref().map_or(0,|i|i.linear.bytes+i.radii.bytes),self.preparations,self.writes,self.passes]}
 pub fn linear(&self)->Option<&Image>{self.images.as_ref().map(|i|&i.linear)}
 pub fn radii(&self)->Option<&Image>{self.images.as_ref().map(|i|&i.radii)}
}
