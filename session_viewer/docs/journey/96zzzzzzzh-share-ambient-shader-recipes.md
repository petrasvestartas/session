# 22.5.15 · Share ambient shader recipes

**Chapter 22 · Surface lighting, contact and display quality · Section 22.5 · Frame resources and multisampling** · [Contents](contents.md)

**Typing: 30–60 minutes.** [Estimate](typing-load.md).

Create each distinct ambient shader module once per recipe set and reuse it across the passes that need it. Give the common pipeline factory explicit vertex and fragment entries, and connect retained composition inputs before activating the physical-sample owner.

## Type

Continue from [Retarget original ambient properties](96zzzzzzzg-retarget-original-ambient-properties.md). [Save or recover your work](recovery.md).

### 1. `src/ambient_pipeline.rs`

Share actual ambient shader modules, match physical inputs and prepare bounded sparse sample corrections while retaining the coarse one-sample recipes.

<details>
<summary>Locate the existing block</summary>

```rust
use wgpu::{ShaderStages as S,TextureSampleType as T,BufferBindingType as B};
pub struct Pipelines{pub source:wgpu::BindGroupLayout,pub level:wgpu::BindGroupLayout,pub sample:wgpu::BindGroupLayout,pub history:wgpu::BindGroupLayout,pub prepare:wgpu::RenderPipeline,pub reduce:wgpu::RenderPipeline,pub occupancy:wgpu::RenderPipeline,pub ground:wgpu::RenderPipeline,pub raw:wgpu::RenderPipeline,pub filter:[wgpu::RenderPipeline;2],pub upsample:wgpu::RenderPipeline,pub composite:wgpu::RenderPipeline}
fn pipeline(device:&wgpu::Device,source:&str,fragment:&str,groups:&[Option<&wgpu::BindGroupLayout>],formats:&[wgpu::TextureFormat],blend:bool)->wgpu::RenderPipeline{
 let module=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("current ambient recipe"),source:wgpu::ShaderSource::Wgsl(source.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("current ambient recipe"),bind_group_layouts:groups,immediate_size:0});let targets=formats.iter().map(|format|Some(wgpu::ColorTargetState{format:*format,blend:if blend{Some(wgpu::BlendState::ALPHA_BLENDING)}else{None},write_mask:wgpu::ColorWrites::ALL})).collect::<Vec<_>>();
 device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("current ambient recipe"),layout:Some(&layout),vertex:wgpu::VertexState{module:&module,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState{module:&module,entry_point:Some(fragment),compilation_options:Default::default(),targets:&targets}),primitive:Default::default(),depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None})
}
impl Pipelines{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{
  let texture=|binding,sample|crate::text_gpu_support::texture_entry(binding,S::FRAGMENT,sample,false);let uniform=|binding|crate::tile_pipelines::entry(binding,S::FRAGMENT,B::Uniform);
  let source=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient source images and precise camera"),entries:&[texture(0,T::Depth),uniform(1),texture(2,T::Float{filterable:false}),texture(3,T::Float{filterable:false}),texture(4,T::Uint),uniform(5)]});
  let level=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient reduced original properties"),entries:&[texture(0,T::Float{filterable:false}),texture(1,T::Uint),texture(2,T::Float{filterable:false})]});
  let sample=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient filtered samples"),entries:&[texture(0,T::Float{filterable:true}),wgpu::BindGroupLayoutEntry{binding:1,visibility:S::FRAGMENT,ty:wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),count:None}]});
  let mut entries=(0..4).map(|i|crate::tile_pipelines::entry(i,S::FRAGMENT,B::Storage{read_only:false})).collect::<Vec<_>>();entries.push(texture(4,T::Float{filterable:true}));entries.push(texture(5,T::Float{filterable:false}));let history=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient reserved current history bindings"),entries:&entries});
  let shader=shader_source(1);let groups=[Some(&source),Some(&level),Some(&sample),Some(&history)];let prepare=pipeline(device,&shader,"fs_prepare",&[Some(&source)],&[wgpu::TextureFormat::R32Float,wgpu::TextureFormat::R16Uint,wgpu::TextureFormat::Rg8Unorm],false);let make=|fragment|pipeline(device,&shader,fragment,&groups,&[wgpu::TextureFormat::R8Unorm],false);
  let ground=make("fs_ground");let raw=make("fs_main");let filter=[make("fs_filter_x"),make("fs_filter_y")];let upsample=make("fs_upsample");let composite=pipeline(device,include_str!("ambient_composite.wgsl"),"fs_main",&[None,Some(&sample)],&[format],true);
  let reduce=pipeline(device,include_str!("ambient_depth.wgsl"),"fs_reduce",&[Some(&level)],&[wgpu::TextureFormat::R32Float,wgpu::TextureFormat::R16Uint],false);let occupancy=pipeline(device,include_str!("ambient_depth.wgsl"),"fs_occupancy",&[Some(&level)],&[wgpu::TextureFormat::R8Unorm],false);
  Self{source,level,sample,history,prepare,reduce,occupancy,ground,raw,filter,upsample,composite}
 }
}

pub(crate) fn shader_source(samples:u32)->String {
 let source=crate::clipping_gpu::source(include_str!("ambient.wgsl"),0,5);if samples==1 {return source;}
 source.replace("const MSAA: bool = false;","const MSAA: bool = true;")
 .replace("var depth: texture_depth_2d;","var depth: texture_depth_multisampled_2d;")
 .replace("var source_radii:texture_2d<f32>;","var source_radii:texture_multisampled_2d<f32>;")
 .replace("var source_normals:texture_2d<f32>;","var source_normals:texture_multisampled_2d<f32>;")
 .replace("var source_primitives:texture_2d<u32>;","var source_primitives:texture_multisampled_2d<u32>;")
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
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
  let source=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient source images and precise camera"),entries:&[texture_entry(0,S::FRAGMENT,T::Depth,samples==4),uniform(1),texture_entry(2,S::FRAGMENT,T::Float{filterable:false},samples==4),texture_entry(3,S::FRAGMENT,T::Float{filterable:false},samples==4),texture_entry(4,S::FRAGMENT,T::Uint,samples==4),uniform(5)]});
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
 .replace("var source_primitives:texture_2d<u32>;","var source_primitives:texture_multisampled_2d<u32>;")
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-01.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-02.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-03.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-04.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-05.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-06.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-07.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-08.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-09.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-10.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-11.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-12.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-13.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-14.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-15.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-16.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-17.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-18.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-19.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-20.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-21.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-22.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-23.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-24.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-25.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-26.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-27.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-28.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-29.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-30.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-31.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-32.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-33.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-34.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-35.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-36.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-37.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-38.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-39.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-40.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-41.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-42.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-43.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-44.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-45.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-46.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-47.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-48.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-49.rs"
```

</details>

### 2. `src/ambient_groups.rs`

Share actual ambient shader modules, match physical inputs and prepare bounded sparse sample corrections while retaining the coarse one-sample recipes.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Groups{pub levels:[wgpu::BindGroup;6],pub occupancy:wgpu::BindGroup,pub ground:wgpu::BindGroup,pub ao:[wgpu::BindGroup;3],pub history:wgpu::BindGroup,pub buffers:[wgpu::Buffer;4]}
fn sampled(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,view:&wgpu::TextureView,sampler:&wgpu::Sampler)->wgpu::BindGroup{device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient retained sample"),layout,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(view)},wgpu::BindGroupEntry{binding:1,resource:wgpu::BindingResource::Sampler(sampler)}]})}
impl Groups{
 pub fn new(device:&wgpu::Device,pipes:&crate::ambient_pipeline::Pipelines,images:&crate::ambient_images::Images)->Self{
  let levels=std::array::from_fn(|i|device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient retained depth and properties"),layout:&pipes.level,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&images.linear.levels[i])},wgpu::BindGroupEntry{binding:1,resource:wgpu::BindingResource::TextureView(&images.radii.levels[i])},wgpu::BindGroupEntry{binding:2,resource:wgpu::BindingResource::TextureView(&images.normals.view)}]}));
  let sampler=device.create_sampler(&wgpu::SamplerDescriptor{mag_filter:wgpu::FilterMode::Linear,min_filter:wgpu::FilterMode::Linear,..Default::default()});let occupancy=sampled(device,&pipes.sample,&images.occupancy.view,&sampler);let ground=sampled(device,&pipes.sample,&images.ground.view,&sampler);let ao=std::array::from_fn(|i|sampled(device,&pipes.sample,&images.ao[i].view,&sampler));
  let buffers=std::array::from_fn(|_|device.create_buffer(&wgpu::BufferDescriptor{label:Some("ambient reserved single-sample work"),size:16,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false}));let mut entries=buffers.iter().enumerate().map(|(i,b)|wgpu::BindGroupEntry{binding:i as u32,resource:b.as_entire_binding()}).collect::<Vec<_>>();entries.push(wgpu::BindGroupEntry{binding:4,resource:wgpu::BindingResource::TextureView(&images.history.view)});entries.push(wgpu::BindGroupEntry{binding:5,resource:wgpu::BindingResource::TextureView(&images.history_depth.view)});let history=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient inactive single-sample history"),layout:&pipes.history,entries:&entries});Self{levels,occupancy,ground,ao,history,buffers}
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

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-50.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-51.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-52.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-53.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-54.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-55.rs"
```

</details>

### 3. `src/ambient_gpu.rs`

Connect the retained camera and bounded one-sample work buffers to the current ambient groups. Bind the existing sparse-composition input group while the current ambient owner remains single-sample.

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
  if self.state.as_ref().is_none_or(|s|s.images.full!=full||s.images.size!=size){let images=Images::new(device,full,density);let groups=Groups::new(device,&self.pipes,&images);let group=Groups::source(device,&self.pipes,&images,depth,sections,&self.frame,clip);self.state=Some(State{images,groups,input:(depth.clone(),sections.clone(),group)});self.cached=None;self.preparations+=1;}
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
  draw(encoder,&self.pipes.composite,&[color],&[(1,&state.groups.ao[2])],false);self.passes+=1;self.error=None;
 }
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.state.as_ref().map_or(0,|s|s.images.bytes()+s.groups.buffers.iter().map(wgpu::Buffer::size).sum::<u64>()),1,self.preparations,self.writes,self.passes]}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

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

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-56.rs"
```

```text
--8<-- "journey/code/96zzzzzzzh-share-ambient-shader-recipes-57.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 22.5.15
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Ambient passes reuse three distinct source modules and one descriptor factory. Physical input layouts and final composition bindings are prepared coherently; coarse recipes remain single-sample. The current ambient owner and complete browser remain one-sample until bounded corrections are connected.

**Verified checkpoint in Chrome.**

**Critical browser-scene review accepted.**

Actual single-sample original contact browser fixture. Both densities preserve original owners, complete source archives, shared geometry, contact shading and exact resize restoration. Complete physical-sample browser integration and whole-course feature review remain pending.

![Actual single-sample original contact browser fixture. Both densities preserve original owners, complete source archives, shared geometry, contact shading and exact resize restoration. Complete physical-sample browser integration and whole-course feature review remain pending.](../screenshots/journey/96zzzzzzt-keep-contact-ambient-inputs-on-originals-browser.png)

[Verification scope](release.md).

This reference image is shared with [22.4.3 · Keep contact ambient inputs on originals](96zzzzzzt-keep-contact-ambient-inputs-on-originals.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The ambient, depth-reduction and final-composition sources each get one module. Reuse those modules across preparation, hierarchy reduction, occupancy, ground, raw shading, filtering and upsampling. The same descriptor factory also creates sparse edge correction from its different vertex/fragment entries, avoiding a second nearly identical pipeline descriptor.

The source layout distinguishes physical depth, original radius/normal and integer address inputs from the retained one-sample coarse and filtered inputs. Only the final colour composition uses the requested physical count; hierarchy and filtering recipes stay single-sample. The final layout binds the existing camera and reserved work buffers alongside filtered ambient. Its eventual indirect-draw buffer gains the required usage, while work storage still retains its small current one-sample size here.

The current ambient owner and canonical browser still use one sample. Its composition now supplies both required groups without changing the intended pixels. Allocation of bounded physical corrections, clearing changed-frame work and the actual sparse draw follow together in the next lesson, with independent source/property and work-buffer readback.

Three distinct shader sources → shared modules → explicit vertex/fragment/count recipes → retained composition inputs.



Why should the pipeline factory accept shader modules and entry names?

Many ambient passes share one source module but choose different entry points and output formats. Passing the existing module avoids recreating it for every pass, and explicit vertex/fragment entries let the same descriptor build fullscreen and sparse edge recipes.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace every ambient pass back to one of the three source modules. Compare fullscreen and sparse edge construction: only entries, formats, bindings, blending and count differ. Run current actual GPU contact/cache and geometric-normal regressions, and inspect that canonical contact shading stays correct before activating sparse corrections.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 22.5.15
npm --prefix ../session_tests run course -- save 22.5.15
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The ambient, depth-reduction and composition sources each create one module reused across their passes. A common descriptor factory accepts existing modules and explicit vertex/fragment entries, including sparse edge recipes, without duplicating a pipeline descriptor. Physical input layouts distinguish original sample-specific properties from retained single-sample coarse inputs. Current composition binds the camera and reserved work buffers alongside filtered ambient. The eventual draw buffer has indirect usage; work storage remains at its small current one-sample size until the bounded correction owner is connected. The active ambient owner and canonical browser remain single-sample. No hardware frame-rate improvement is claimed. All 1003 Native tests, WASM/optimized builds, Native frame, canonical Chrome contact/resize checks and stored evidence pass. Ten current canonical Native/browser pictures were individually inspected. Complete physical-sample viewer integration, adaptive quality, remaining view controls, runtime efficiency, Cargo independence and final whole-course feature-picture review remain unfinished. Maintained Session reload camera/base-grid fixes follow the completed course.

Keep full original and assembled Naga validation, actual geometric-normal/original-property assertions, cached contact shading and release checks. Require correct current composition bindings and unchanged default output before activating the physical-sample owner.

Actual single-sample original contact browser fixture. Both densities preserve original owners, complete source archives, shared geometry, contact shading and exact resize restoration. Complete physical-sample browser integration and whole-course feature review remain pending.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 22.5.15
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
