pub struct Pipelines{pub prepare:crate::sample_pipelines::Variants<(wgpu::RenderPipeline,wgpu::BindGroupLayout)>,pub reduce:wgpu::RenderPipeline,pub level:wgpu::BindGroupLayout}
fn pipeline(device:&wgpu::Device,shader:&str,fragment:&str,layout:&wgpu::BindGroupLayout)->wgpu::RenderPipeline{
 let module=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("ambient linear-depth recipe"),source:wgpu::ShaderSource::Wgsl(shader.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:None,bind_group_layouts:&[Some(layout)],immediate_size:0});
 device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("ambient linear-depth pyramid"),layout:Some(&layout),vertex:wgpu::VertexState{module:&module,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState{module:&module,entry_point:Some(fragment),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R32Float,blend:None,write_mask:wgpu::ColorWrites::RED}),Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R16Uint,blend:None,write_mask:wgpu::ColorWrites::RED})]}),primitive:Default::default(),depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None})
}
fn input(device:&wgpu::Device,samples:u32)->(wgpu::RenderPipeline,wgpu::BindGroupLayout){
 let input=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient original depth and camera"),entries:&[crate::text_gpu_support::texture_entry(0,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Depth,samples==4),crate::tile_pipelines::entry(1,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Uniform)]});
 let source=include_str!("pyramid_prepare.wgsl");
 let source=if samples==4{source.replace("var depth:texture_depth_2d;","var depth:texture_depth_multisampled_2d;")}else{source.into()};
 (pipeline(device,&source,"fs_prepare",&input),input)
}
impl Pipelines{
 pub fn with_samples(device:&wgpu::Device,samples:u32)->Self{
  let texture=|binding,sample_type|crate::text_gpu_support::texture_entry(binding,wgpu::ShaderStages::FRAGMENT,sample_type,false);
  let level=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient depth and radius level"),entries:&[texture(0,wgpu::TextureSampleType::Float{filterable:false}),texture(1,wgpu::TextureSampleType::Uint)]});
  let prepare=crate::sample_pipelines::Variants::new(input(device,samples),samples);let reduce=pipeline(device,include_str!("ambient_depth.wgsl"),"fs_reduce",&level);Self{prepare,reduce,level}
 }
 pub fn set_samples(&mut self,device:&wgpu::Device,samples:u32)->bool{self.prepare.select(samples,||input(device,samples))}
}
