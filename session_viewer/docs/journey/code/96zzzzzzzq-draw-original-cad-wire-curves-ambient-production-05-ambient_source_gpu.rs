use crate::{sample_pipelines::Variants,ambient_source_pipeline::Pipeline};
pub struct Sources{pipes:Variants<Pipeline>}
impl Sources{
 pub fn new(device:&wgpu::Device)->Self{Self::with_samples(device,1)}
 pub fn with_samples(device:&wgpu::Device,samples:u32)->Self{Self{pipes:Variants::new(Pipeline::with_samples(device,samples),samples)}}
 pub fn set_samples(&mut self,device:&wgpu::Device,samples:u32){self.pipes.select(samples,||Pipeline::with_samples(device,samples));}
 pub fn encode(&mut self,encoder:&mut wgpu::CommandEncoder,radius:&wgpu::TextureView,normals:[&wgpu::TextureView;3],depth:&wgpu::TextureView,view:&wgpu::BindGroup,clip:&wgpu::BindGroup,meshes:&[crate::gpu_mesh::GpuMesh],records:Option<&crate::instance_gpu::Lane>){
  let attachments=[radius,normals[0],normals[1],normals[2]].map(|view|Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}}));let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original ambient source properties"),color_attachments:&attachments,depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&self.pipes.active.draw);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clip,&[]);for mesh in meshes.iter().filter(|mesh|!mesh.printed()){if records.is_some_and(|rows|rows.bind_record(&mut pass,mesh)){pass.set_pipeline(&self.pipes.active.record_draw);mesh.geometry.draw_pulled(&mut pass,1);}else{pass.set_pipeline(&self.pipes.active.draw);mesh.bind_model(&mut pass);mesh.geometry.draw_pulled(&mut pass,1);}}
 }
}
