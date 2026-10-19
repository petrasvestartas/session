pub struct Pipelines{pub prepare:wgpu::RenderPipeline,pub reduce:wgpu::RenderPipeline,pub input:wgpu::BindGroupLayout,pub level:wgpu::BindGroupLayout}
fn pipeline(device:&wgpu::Device,shader:&str,fragment:&str,layout:&wgpu::BindGroupLayout)->wgpu::RenderPipeline{
 let module=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("ambient linear-depth recipe"),source:wgpu::ShaderSource::Wgsl(shader.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:None,bind_group_layouts:&[Some(layout)],immediate_size:0});
 device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("ambient linear-depth pyramid"),layout:Some(&layout),vertex:wgpu::VertexState{module:&module,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState{module:&module,entry_point:Some(fragment),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R32Float,blend:None,write_mask:wgpu::ColorWrites::RED}),Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R16Uint,blend:None,write_mask:wgpu::ColorWrites::RED})]}),primitive:Default::default(),depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None})
}
impl Pipelines{
 pub fn new(device:&wgpu::Device)->Self{
  let texture=|binding,sample_type|crate::text_gpu_support::texture_entry(binding,wgpu::ShaderStages::FRAGMENT,sample_type,false);
  let input=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient original depth and camera"),entries:&[texture(0,wgpu::TextureSampleType::Depth),crate::tile_pipelines::entry(1,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Uniform)]});
  let level=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient depth and radius level"),entries:&[texture(0,wgpu::TextureSampleType::Float{filterable:false}),texture(1,wgpu::TextureSampleType::Uint)]});
  let prepare=pipeline(device,include_str!("pyramid_prepare.wgsl"),"fs_prepare",&input);let reduce=pipeline(device,include_str!("ambient_depth.wgsl"),"fs_reduce",&level);Self{prepare,reduce,input,level}
 }
}
