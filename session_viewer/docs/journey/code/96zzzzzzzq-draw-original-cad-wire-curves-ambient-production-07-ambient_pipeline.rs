use crate::{tile_pipelines::entry,text_gpu_support::texture_entry};
use wgpu::{ShaderStages as S,TextureSampleType as T,BufferBindingType as B};
pub struct Pipelines{pub source:wgpu::BindGroupLayout,pub level:wgpu::BindGroupLayout,pub sample:wgpu::BindGroupLayout,pub history:wgpu::BindGroupLayout,pub prepare:wgpu::RenderPipeline,pub reduce:wgpu::RenderPipeline,pub occupancy:wgpu::RenderPipeline,pub ground:wgpu::RenderPipeline,pub raw:wgpu::RenderPipeline,pub filter:[wgpu::RenderPipeline;2],pub upsample:wgpu::RenderPipeline,pub composite:wgpu::RenderPipeline,pub edges:Option<wgpu::RenderPipeline>,pub composite_layout:wgpu::BindGroupLayout}
fn pipeline(device:&wgpu::Device,module:&wgpu::ShaderModule,vertex:&str,fragment:&str,groups:&[Option<&wgpu::BindGroupLayout>],formats:&[wgpu::TextureFormat],blend:bool,samples:u32)->wgpu::RenderPipeline{
 let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("current ambient recipe"),bind_group_layouts:groups,immediate_size:0});let targets=formats.iter().map(|format|Some(wgpu::ColorTargetState{format:*format,blend:if blend{Some(wgpu::BlendState::ALPHA_BLENDING)}else{None},write_mask:wgpu::ColorWrites::ALL})).collect::<Vec<_>>();
 device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("current ambient recipe"),layout:Some(&layout),vertex:wgpu::VertexState{module,entry_point:Some(vertex),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState{module,entry_point:Some(fragment),compilation_options:Default::default(),targets:&targets}),primitive:Default::default(),depth_stencil:None,multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None})
}
impl Pipelines{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{Self::with_samples(device,format,1)}
 pub fn with_samples(device:&wgpu::Device,format:wgpu::TextureFormat,samples:u32)->Self{assert!(matches!(samples,1|4));
  let texture=|binding,sample|texture_entry(binding,S::FRAGMENT,sample,false);let uniform=|binding|entry(binding,S::FRAGMENT,B::Uniform);
  let source=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient source images and precise camera"),entries:&[texture_entry(0,S::FRAGMENT,T::Depth,samples==4),uniform(1),texture_entry(2,S::FRAGMENT,T::Float{filterable:false},samples==4),texture_entry(3,S::FRAGMENT,T::Float{filterable:false},samples==4),texture_entry(4,S::FRAGMENT,T::Uint,samples==4),uniform(5),texture_entry(6,S::FRAGMENT,T::Float{filterable:false},samples==4),texture_entry(7,S::FRAGMENT,T::Float{filterable:false},samples==4)]});
  let level=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient reduced original properties"),entries:&[texture(0,T::Float{filterable:false}),texture(1,T::Uint),texture(2,T::Float{filterable:false})]});
  let sample=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient filtered samples"),entries:&[texture(0,T::Float{filterable:true}),wgpu::BindGroupLayoutEntry{binding:1,visibility:S::FRAGMENT,ty:wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),count:None}]});
  let mut entries=(0..4).map(|i|entry(i,S::FRAGMENT,B::Storage{read_only:false})).collect::<Vec<_>>();entries.push(texture(4,T::Float{filterable:true}));entries.push(texture(5,T::Float{filterable:false}));let history=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient reserved current history bindings"),entries:&entries});
  let shader_source=shader_source(samples);let module=|label,source:String|device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some(label),source:wgpu::ShaderSource::Wgsl(source.into())});
  let shader=module("current ambient recipe",shader_source);let depth_shader=module("ambient depth recipe",include_str!("ambient_depth.wgsl").into());let composite_shader=module("ambient composition",include_str!("ambient_composite.wgsl").into());
  let groups=[Some(&source),Some(&level),Some(&sample),Some(&history)];let prepare=pipeline(device,&shader,"vs_main","fs_prepare",&[Some(&source)],&[wgpu::TextureFormat::R32Float,wgpu::TextureFormat::R16Uint,wgpu::TextureFormat::Rg8Unorm],false,1);let make=|fragment|pipeline(device,&shader,"vs_main",fragment,&groups,&[wgpu::TextureFormat::R8Unorm],false,1);
  let ground=make("fs_ground");let raw=make("fs_main");let filter=[make("fs_filter_x"),make("fs_filter_y")];let upsample=make("fs_upsample");
  let composite_layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient sparse sample corrections"),entries:&[entry(0,S::VERTEX_FRAGMENT,B::Uniform),entry(1,S::FRAGMENT,B::Storage{read_only:true}),entry(2,S::VERTEX,B::Storage{read_only:true})]});
  let composite=pipeline(device,&composite_shader,"vs_main","fs_main",&[Some(&composite_layout),Some(&sample)],&[format],true,samples);
  let edges=(samples==4).then(||pipeline(device,&composite_shader,"vs_edges","fs_edges",&[Some(&composite_layout),Some(&sample)],&[format],true,samples));
  let reduce=pipeline(device,&depth_shader,"vs_main","fs_reduce",&[Some(&level)],&[wgpu::TextureFormat::R32Float,wgpu::TextureFormat::R16Uint],false,1);let occupancy=pipeline(device,&depth_shader,"vs_main","fs_occupancy",&[Some(&level)],&[wgpu::TextureFormat::R8Unorm],false,1);
  Self{source,level,sample,history,prepare,reduce,occupancy,ground,raw,filter,upsample,composite,edges,composite_layout}
 }
}
pub(crate) fn shader_source(samples:u32)->String {
 let source=crate::clipping_gpu::source(include_str!("ambient.wgsl"),0,5);if samples==1 {return source;}
 source.replace("const MSAA: bool = false;","const MSAA: bool = true;")
 .replace("var depth: texture_depth_2d;","var depth: texture_depth_multisampled_2d;")
 .replace("var source_radii:texture_2d<f32>;","var source_radii:texture_multisampled_2d<f32>;")
 .replace("var source_normals:texture_2d<f32>;","var source_normals:texture_multisampled_2d<f32>;")
 .replace("var source_normal_y:texture_2d<f32>;","var source_normal_y:texture_multisampled_2d<f32>;")
 .replace("var source_normal_z:texture_2d<f32>;","var source_normal_z:texture_multisampled_2d<f32>;")
 .replace("var source_primitives:texture_2d<u32>;","var source_primitives:texture_multisampled_2d<u32>;")
}
