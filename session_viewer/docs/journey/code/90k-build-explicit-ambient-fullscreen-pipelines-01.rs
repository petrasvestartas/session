use wgpu::{ShaderStages as S,TextureSampleType as T,BufferBindingType as B};
pub struct Pipelines{pub source:wgpu::BindGroupLayout,pub level:wgpu::BindGroupLayout,pub sample:wgpu::BindGroupLayout,pub history:wgpu::BindGroupLayout,pub prepare:wgpu::RenderPipeline,pub reduce:wgpu::RenderPipeline,pub occupancy:wgpu::RenderPipeline,pub ground:wgpu::RenderPipeline,pub raw:wgpu::RenderPipeline,pub filter:[wgpu::RenderPipeline;2],pub upsample:wgpu::RenderPipeline,pub composite:wgpu::RenderPipeline}
fn pipeline(device:&wgpu::Device,source:&str,fragment:&str,groups:&[Option<&wgpu::BindGroupLayout>],formats:&[wgpu::TextureFormat],blend:bool)->wgpu::RenderPipeline{
 let module=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("current ambient recipe"),source:wgpu::ShaderSource::Wgsl(source.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("current ambient recipe"),bind_group_layouts:groups,immediate_size:0});let targets=formats.iter().map(|format|Some(wgpu::ColorTargetState{format:*format,blend:if blend{Some(wgpu::BlendState::ALPHA_BLENDING)}else{None},write_mask:wgpu::ColorWrites::ALL})).collect::<Vec<_>>();
 device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("current ambient recipe"),layout:Some(&layout),vertex:wgpu::VertexState{module:&module,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState{module:&module,entry_point:Some(fragment),compilation_options:Default::default(),targets:&targets}),primitive:Default::default(),depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None})
}
impl Pipelines{

}
