pub struct Composite{pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,inputs:Option<[wgpu::TextureView;2]>,group:Option<wgpu::BindGroup>}
impl Composite{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{
  let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("max outline alpha"),entries:&[0,1].map(|n|crate::text_gpu_support::texture_entry(n,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Float{filterable:false},false))});
  let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("max outline alpha"),source:wgpu::ShaderSource::Wgsl(include_str!("outline_composite.wgsl").into())});
  let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("max outline alpha"),bind_group_layouts:&[Some(&layout)],immediate_size:0});
  let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("one black outline blend"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some("vs_main"),buffers:&[],compilation_options:Default::default()},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some("fs_main"),targets:&[Some(wgpu::ColorTargetState{format,blend:Some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL})],compilation_options:Default::default()}),primitive:Default::default(),depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None});Self{pipeline,layout,inputs:None,group:None}
 }
 pub fn draw(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,colour:&wgpu::TextureView,global:&wgpu::TextureView,selected:Option<&wgpu::TextureView>){
  let inputs=[global.clone(),selected.unwrap_or(global).clone()];if self.inputs.as_ref()!=Some(&inputs){self.group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("max outline alpha"),layout:&self.layout,entries:&[0,1].map(|n|wgpu::BindGroupEntry{binding:n,resource:wgpu::BindingResource::TextureView(&inputs[n as usize])})}));self.inputs=Some(inputs);}
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("one black outline blend"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&self.pipeline);pass.set_bind_group(0,self.group.as_ref().unwrap(),&[]);pass.draw(0..3,0..1);
 }
}
