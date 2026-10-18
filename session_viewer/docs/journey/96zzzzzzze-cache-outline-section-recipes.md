# 22.5.12 · Cache section masks and the final outline blend

**Chapter 22 · Surface lighting, contact and display quality · Section 22.5 · Frame resources and multisampling** · [Contents](contents.md)

**Typing: 30–60 minutes.** [Estimate](typing-load.md).

Cache the section coverage and final outline blend for their requested sample count. Retain original integer addresses and the existing one-sample search images. This prepares the recipes; the outline coverage owner is connected in the next step.

## Type

Continue from [Retarget cached text colour](96zzzzzzzd-retarget-cached-text.md). [Save or recover your work](recovery.md).

### 1. `src/outline_caps.rs`

Match physical face/section coverage and colour recipes while retaining one-sample outline search images and a single final blend.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Caps{
 pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,
 input:Option<wgpu::TextureView>,group:Option<wgpu::BindGroup>,
}
impl Caps{
 pub fn new(device:&wgpu::Device)->Self{
  use wgpu::{ShaderStages as Stages,TextureSampleType as Sample};
  let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("visible cap coverage"),entries:&[
   crate::text_gpu_support::texture_entry(0,Stages::FRAGMENT,Sample::Uint,false),
   wgpu::BindGroupLayoutEntry{binding:1,visibility:Stages::FRAGMENT,ty:wgpu::BindingType::Buffer{ty:wgpu::BufferBindingType::Uniform,has_dynamic_offset:false,min_binding_size:wgpu::BufferSize::new(16)},count:None},
  ]});
  let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("visible cap coverage"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("outline_caps.wgsl"),1,0).into())});
  let clipping=crate::clipping_gpu::layout(device);
  let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("visible cap coverage"),bind_group_layouts:&[Some(&layout),Some(&clipping)],immediate_size:0});
  let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
   label:Some("visible cap coverage"),layout:Some(&pipeline_layout),
   vertex:wgpu::VertexState{module:&shader,entry_point:Some("vs_main"),buffers:&[],compilation_options:Default::default()},
   fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some("fs_main"),targets:&[Some(wgpu::TextureFormat::R8Unorm.into())],compilation_options:Default::default()}),
   primitive:Default::default(),depth_stencil:Some(wgpu::DepthStencilState{format:crate::depth::FORMAT,depth_write_enabled:Some(false),depth_compare:Some(crate::depth::CLOSER_EQUAL),stencil:Default::default(),bias:Default::default()}),multisample:Default::default(),multiview_mask:None,cache:None,
  });Self{pipeline,layout,input:None,group:None}
 }
 pub fn prepare(&mut self,device:&wgpu::Device,input:&wgpu::TextureView,radius:&wgpu::Buffer){
  if self.input.as_ref()==Some(input){return;}
  self.group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("visible cap coverage"),layout:&self.layout,entries:&[
   wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(input)},wgpu::BindGroupEntry{binding:1,resource:radius.as_entire_binding()},
  ]}));self.input=Some(input.clone());
 }
 pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>,clipping:&wgpu::BindGroup){if let Some(group)=&self.group{pass.set_pipeline(&self.pipeline);pass.set_bind_group(0,group,&[]);pass.set_bind_group(1,clipping,&[]);pass.draw(0..3,0..1);}}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Caps{
 pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,
 input:Option<wgpu::TextureView>,group:Option<wgpu::BindGroup>,
}
impl Caps{
 pub fn new(device:&wgpu::Device)->Self{
Self{pipeline,layout,input:None,group:None}
 }
 pub fn prepare(&mut self,device:&wgpu::Device,input:&wgpu::TextureView,radius:&wgpu::Buffer){
  if self.input.as_ref()==Some(input){return;}
  self.group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("visible cap coverage"),layout:&self.layout,entries:&[
   wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(input)},wgpu::BindGroupEntry{binding:1,resource:radius.as_entire_binding()},
  ]}));self.input=Some(input.clone());
 }
 pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>,clipping:&wgpu::BindGroup){if let Some(group)=&self.group{pass.set_pipeline(&self.pipeline);pass.set_bind_group(0,group,&[]);pass.set_bind_group(1,clipping,&[]);pass.draw(0..3,0..1);}}
}

fn pipelines(device:&wgpu::Device,samples:u32)->(wgpu::RenderPipeline,wgpu::BindGroupLayout) {
// existing section coverage descriptor
(pipeline,layout)
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-02.rs"
```

</details>

### 2. `src/outline_caps.rs`

Reuse the previously typed immutable outline descriptor.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
// existing section coverage descriptor
```

</details>

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-03.rs"
```

</details>

### 3. `src/outline_caps.rs`

Match physical face/section coverage and colour recipes while retaining one-sample outline search images and a single final blend.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Caps{
 pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,
 input:Option<wgpu::TextureView>,group:Option<wgpu::BindGroup>,
}
impl Caps{
 pub fn new(device:&wgpu::Device)->Self{
Self{pipeline,layout,input:None,group:None}
 }
 pub fn prepare(&mut self,device:&wgpu::Device,input:&wgpu::TextureView,radius:&wgpu::Buffer){
  if self.input.as_ref()==Some(input){return;}
  self.group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("visible cap coverage"),layout:&self.layout,entries:&[
   wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(input)},wgpu::BindGroupEntry{binding:1,resource:radius.as_entire_binding()},
  ]}));self.input=Some(input.clone());
 }
 pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>,clipping:&wgpu::BindGroup){if let Some(group)=&self.group{pass.set_pipeline(&self.pipeline);pass.set_bind_group(0,group,&[]);pass.set_bind_group(1,clipping,&[]);pass.draw(0..3,0..1);}}
}

fn pipelines(device:&wgpu::Device,samples:u32)->(wgpu::RenderPipeline,wgpu::BindGroupLayout) {
  use wgpu::{ShaderStages as Stages,TextureSampleType as Sample};
  let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("visible cap coverage"),entries:&[
   crate::text_gpu_support::texture_entry(0,Stages::FRAGMENT,Sample::Uint,false),
   wgpu::BindGroupLayoutEntry{binding:1,visibility:Stages::FRAGMENT,ty:wgpu::BindingType::Buffer{ty:wgpu::BufferBindingType::Uniform,has_dynamic_offset:false,min_binding_size:wgpu::BufferSize::new(16)},count:None},
  ]});
  let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("visible cap coverage"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("outline_caps.wgsl"),1,0).into())});
  let clipping=crate::clipping_gpu::layout(device);
  let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("visible cap coverage"),bind_group_layouts:&[Some(&layout),Some(&clipping)],immediate_size:0});
  let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
   label:Some("visible cap coverage"),layout:Some(&pipeline_layout),
   vertex:wgpu::VertexState{module:&shader,entry_point:Some("vs_main"),buffers:&[],compilation_options:Default::default()},
   fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some("fs_main"),targets:&[Some(wgpu::TextureFormat::R8Unorm.into())],compilation_options:Default::default()}),
   primitive:Default::default(),depth_stencil:Some(wgpu::DepthStencilState{format:crate::depth::FORMAT,depth_write_enabled:Some(false),depth_compare:Some(crate::depth::CLOSER_EQUAL),stencil:Default::default(),bias:Default::default()}),multisample:Default::default(),multiview_mask:None,cache:None,
  });
(pipeline,layout)
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::sample_pipelines::Variants;
pub struct Caps{
 pipelines:Option<Variants<(wgpu::RenderPipeline,wgpu::BindGroupLayout)>>,
 input:Option<wgpu::TextureView>,group:Option<wgpu::BindGroup>,
}
impl Caps{
 pub fn new(_:&wgpu::Device)->Self {Self {pipelines:None,input:None,group:None}}
 pub fn clear(&mut self){self.input=None;self.group=None;}
 pub fn prepare(&mut self,device:&wgpu::Device,input:&wgpu::TextureView,radius:&wgpu::Buffer,samples:u32){
  if self.pipelines.is_none(){self.pipelines=Some(Variants::new(pipelines(device,samples),samples));}
  if self.pipelines.as_mut().unwrap().select(samples,||pipelines(device,samples)){self.clear();}
  if self.input.as_ref()==Some(input){return;}
  self.group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("visible cap coverage"),layout:&self.pipelines.as_ref().unwrap().active.1,entries:&[
   wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(input)},wgpu::BindGroupEntry{binding:1,resource:radius.as_entire_binding()},
  ]}));self.input=Some(input.clone());
 }
 pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>,clipping:&wgpu::BindGroup){if let Some(group)=&self.group{pass.set_pipeline(&self.pipelines.as_ref().unwrap().active.0);pass.set_bind_group(0,group,&[]);pass.set_bind_group(1,clipping,&[]);pass.draw(0..3,0..1);}}
}

fn pipelines(device:&wgpu::Device,samples:u32)->(wgpu::RenderPipeline,wgpu::BindGroupLayout) {
  use wgpu::{ShaderStages as Stages,TextureSampleType as Sample};
  let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("visible cap coverage"),entries:&[
   crate::text_gpu_support::texture_entry(0,Stages::FRAGMENT,Sample::Uint,samples==4),
   wgpu::BindGroupLayoutEntry{binding:1,visibility:Stages::FRAGMENT,ty:wgpu::BindingType::Buffer{ty:wgpu::BufferBindingType::Uniform,has_dynamic_offset:false,min_binding_size:wgpu::BufferSize::new(16)},count:None},
  ]});
  let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("visible cap coverage"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(&source(samples),1,0).into())});
  let clipping=crate::clipping_gpu::layout(device);
  let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("visible cap coverage"),bind_group_layouts:&[Some(&layout),Some(&clipping)],immediate_size:0});
  let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
   label:Some("visible cap coverage"),layout:Some(&pipeline_layout),
   vertex:wgpu::VertexState{module:&shader,entry_point:Some("vs_main"),buffers:&[],compilation_options:Default::default()},
   fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some("fs_main"),targets:&[Some(wgpu::TextureFormat::R8Unorm.into())],compilation_options:Default::default()}),
   primitive:Default::default(),depth_stencil:Some(wgpu::DepthStencilState{format:crate::depth::FORMAT,depth_write_enabled:Some(false),depth_compare:Some(crate::depth::CLOSER_EQUAL),stencil:Default::default(),bias:Default::default()}),multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None,
  });(pipeline,layout)
}

fn source(samples:u32)->String {
 let source=include_str!("outline_caps.wgsl");if samples==1{return source.into();}
 source.replace("var primitives:texture_2d<u32>;","var primitives:texture_multisampled_2d<u32>;")
 .replace(r#"@vertex fn vs_main(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32>{
 let p=array<vec2<f32>,3>(vec2<f32>(-1.,-1.),vec2<f32>(3.,-1.),vec2<f32>(-1.,3.));return vec4<f32>(p[i],0.,1.);
}
"#,r#"struct Vertex{@builtin(position) position:vec4f,@location(0) @interpolate(linear,sample) pixel:vec2f}
@vertex fn vs_main(@builtin(vertex_index) i:u32)->Vertex {
 let p=array<vec2f,3>(vec2f(-1,-1),vec2f(3,-1),vec2f(-1,3))[i];return Vertex(vec4f(p,0,1),p*vec2f(0.5,-0.5)+0.5);
}
"#)
 .replace("fn fs_main(@builtin(position) p:vec4<f32>)->Coverage{","fn fs_main(input:Vertex,@builtin(sample_index) sample:u32)->Coverage{let p=vec4f(input.pixel*vec2f(textureDimensions(primitives)),0,1);")
 .replace("textureLoad(primitives,vec2<i32>(p.xy),0)","textureLoad(primitives,vec2<i32>(p.xy),i32(sample))")
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-04.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-05.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-06.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-07.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-08.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-09.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-10.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-11.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-12.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-13.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-14.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-15.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-16.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-17.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-18.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-19.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-20.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-21.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-22.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-23.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-24.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-25.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-26.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-27.rs"
```

</details>

### 4. `src/outline_composite.rs`

Match physical face/section coverage and colour recipes while retaining one-sample outline search images and a single final blend.

<details>
<summary>Locate the existing block</summary>

```rust
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
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Composite{pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,inputs:Option<[wgpu::TextureView;2]>,group:Option<wgpu::BindGroup>}
impl Composite{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{
  let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("max outline alpha"),entries:&[0,1].map(|n|crate::text_gpu_support::texture_entry(n,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Float{filterable:false},false))});
  let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("max outline alpha"),source:wgpu::ShaderSource::Wgsl(include_str!("outline_composite.wgsl").into())});
  let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("max outline alpha"),bind_group_layouts:&[Some(&layout)],immediate_size:0});
  let pipeline=Variants::new(pipeline(device,&pipeline_layout,&shader,format,samples),samples);Self{pipeline,layout,inputs:None,group:None}
 }
 pub fn draw(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,colour:&wgpu::TextureView,global:&wgpu::TextureView,selected:Option<&wgpu::TextureView>){
  let inputs=[global.clone(),selected.unwrap_or(global).clone()];if self.inputs.as_ref()!=Some(&inputs){self.group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("max outline alpha"),layout:&self.layout,entries:&[0,1].map(|n|wgpu::BindGroupEntry{binding:n,resource:wgpu::BindingResource::TextureView(&inputs[n as usize])})}));self.inputs=Some(inputs);}
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("one black outline blend"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&self.pipeline);pass.set_bind_group(0,self.group.as_ref().unwrap(),&[]);pass.draw(0..3,0..1);
 }
}

fn pipeline(device:&wgpu::Device,pipeline_layout:&wgpu::PipelineLayout,shader:&wgpu::ShaderModule,format:wgpu::TextureFormat,samples:u32)->wgpu::RenderPipeline {
// existing outline descriptor
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-28.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-29.rs"
```

</details>

### 5. `src/outline_composite.rs`

Reuse the previously typed immutable outline descriptor.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
// existing outline descriptor
```

</details>

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-30.rs"
```

</details>

### 6. `src/outline_composite.rs`

Match physical face/section coverage and colour recipes while retaining one-sample outline search images and a single final blend.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Composite{pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,inputs:Option<[wgpu::TextureView;2]>,group:Option<wgpu::BindGroup>}
impl Composite{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{
  let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("max outline alpha"),entries:&[0,1].map(|n|crate::text_gpu_support::texture_entry(n,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Float{filterable:false},false))});
  let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("max outline alpha"),source:wgpu::ShaderSource::Wgsl(include_str!("outline_composite.wgsl").into())});
  let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("max outline alpha"),bind_group_layouts:&[Some(&layout)],immediate_size:0});
  let pipeline=Variants::new(pipeline(device,&pipeline_layout,&shader,format,samples),samples);Self{pipeline,layout,inputs:None,group:None}
 }
 pub fn draw(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,colour:&wgpu::TextureView,global:&wgpu::TextureView,selected:Option<&wgpu::TextureView>){
  let inputs=[global.clone(),selected.unwrap_or(global).clone()];if self.inputs.as_ref()!=Some(&inputs){self.group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("max outline alpha"),layout:&self.layout,entries:&[0,1].map(|n|wgpu::BindGroupEntry{binding:n,resource:wgpu::BindingResource::TextureView(&inputs[n as usize])})}));self.inputs=Some(inputs);}
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("one black outline blend"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&self.pipeline);pass.set_bind_group(0,self.group.as_ref().unwrap(),&[]);pass.draw(0..3,0..1);
 }
}

fn pipeline(device:&wgpu::Device,pipeline_layout:&wgpu::PipelineLayout,shader:&wgpu::ShaderModule,format:wgpu::TextureFormat,samples:u32)->wgpu::RenderPipeline {
device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("one black outline blend"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some("vs_main"),buffers:&[],compilation_options:Default::default()},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some("fs_main"),targets:&[Some(wgpu::ColorTargetState{format,blend:Some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL})],compilation_options:Default::default()}),primitive:Default::default(),depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None})
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::sample_pipelines::Variants;
pub struct Composite{pipeline:Variants<wgpu::RenderPipeline>,shader:wgpu::ShaderModule,format:wgpu::TextureFormat,layout:wgpu::BindGroupLayout,inputs:Option<[wgpu::TextureView;2]>,group:Option<wgpu::BindGroup>}
impl Composite{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{Self::with_samples(device,format,1)}
 pub fn with_samples(device:&wgpu::Device,format:wgpu::TextureFormat,samples:u32)->Self{
  let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("max outline alpha"),entries:&[0,1].map(|n|crate::text_gpu_support::texture_entry(n,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Float{filterable:false},false))});
  let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("max outline alpha"),source:wgpu::ShaderSource::Wgsl(include_str!("outline_composite.wgsl").into())});
  let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("max outline alpha"),bind_group_layouts:&[Some(&layout)],immediate_size:0});
  let pipeline=Variants::new(pipeline(device,&pipeline_layout,&shader,format,samples),samples);Self{pipeline,shader,format,layout,inputs:None,group:None}
 }
 pub fn set_samples(&mut self,device:&wgpu::Device,samples:u32){self.pipeline.select(samples,||{let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("max outline alpha"),bind_group_layouts:&[Some(&self.layout)],immediate_size:0});pipeline(device,&layout,&self.shader,self.format,samples)});}
 pub fn draw(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,colour:&wgpu::TextureView,global:&wgpu::TextureView,selected:Option<&wgpu::TextureView>){
  let inputs=[global.clone(),selected.unwrap_or(global).clone()];if self.inputs.as_ref()!=Some(&inputs){self.group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("max outline alpha"),layout:&self.layout,entries:&[0,1].map(|n|wgpu::BindGroupEntry{binding:n,resource:wgpu::BindingResource::TextureView(&inputs[n as usize])})}));self.inputs=Some(inputs);}
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("one black outline blend"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&self.pipeline.active);pass.set_bind_group(0,self.group.as_ref().unwrap(),&[]);pass.draw(0..3,0..1);
 }
}

fn pipeline(device:&wgpu::Device,pipeline_layout:&wgpu::PipelineLayout,shader:&wgpu::ShaderModule,format:wgpu::TextureFormat,samples:u32)->wgpu::RenderPipeline {
 device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("one black outline blend"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some("vs_main"),buffers:&[],compilation_options:Default::default()},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some("fs_main"),targets:&[Some(wgpu::ColorTargetState{format,blend:Some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL})],compilation_options:Default::default()}),primitive:Default::default(),depth_stencil:None,multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None})
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-31.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-32.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-33.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-34.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-35.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-36.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-37.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-38.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-39.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-40.rs"
```

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-41.rs"
```

</details>

### 7. `src/outline_gpu.rs`

Keep this prerequisite independently compilable: its current coverage producer is still one-sample. The next lesson passes the active physical count.

<details>
<summary>Locate the existing block</summary>

```rust
    if let Some(sections)=sections{self.caps.prepare(&ctx.device,sections,&self.radius);}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    if let Some(sections)=sections{self.caps.prepare(&ctx.device,sections,&self.radius,1);}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-42.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 22.5.12
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Section coverage and final outline blending have lazy or cached count-specific recipes. Original source addresses and existing one-sample search resources retain their meaning. The current coverage producer and complete browser frame remain single-sample until their next integration step.

**Verified checkpoint in Chrome.**

**Critical browser-scene review accepted.**

Actual single-sample contact fixture after preparing lazy section masks and a cached final outline blend. Both densities preserve original owners, complete source archives, shared geometry, contact shading and exact resize restoration. Complete four-sample outline coverage is connected next.

![Actual single-sample contact fixture after preparing lazy section masks and a cached final outline blend. Both densities preserve original owners, complete source archives, shared geometry, contact shading and exact resize restoration. Complete four-sample outline coverage is connected next.](../screenshots/journey/96zzzzzzt-keep-contact-ambient-inputs-on-originals-browser.png)

[Verification scope](release.md).

This reference image is shared with [22.4.3 · Keep contact ambient inputs on originals](96zzzzzzt-keep-contact-ambient-inputs-on-originals.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

An empty section owner now waits until first use before creating a recipe. Its bind group refers to the active original-address image, and a real count change invalidates that binding. The four-sample shader reads the matching integer sample and uses actual sample-qualified coordinates to reconstruct the section depth. Integer addresses and depth are never resolved or averaged.

Move the previously typed section and final-blend descriptors into small factories. Reuse those exact descriptors and change only the count-dependent parts. Both owners use the shared two-count recipe cache. The final blend retains its source shader and immutable layout and chooses the cached colour recipe matching the output pass. Its input alpha images remain single-sample.

The current outline coverage producer still uses one sample. Existing actual GPU regressions therefore verify unchanged selected/global masks, section silhouettes, CSS widths and the single black blend here. Actual four-sample outline coverage, resource bounds and original-owner checks follow when the producer is connected.

Original section address → matching physical coverage sample → retained outline search → one cached final blend.



Which outline inputs can be resolved, and which must remain sample-specific?

Float coverage may be resolved for the one-sample search. Original integer addresses and depth remain sample-specific; read the matching physical sample when constructing section coverage. Blend the resulting outline only once.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Run the actual selected/global outline, section-mask and two-density CSS-width regressions. Inspect that default one-sample silhouettes are unchanged. Follow the four-sample section shader from its explicit sample coordinate to its original integer address and reconstructed plane depth; do not mistake this prerequisite for a complete four-sample frame.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 22.5.12
npm --prefix ../session_tests run course -- save 22.5.12
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Section coverage recipes are created lazily and cached for each physical count. They read the matching original integer sample and reconstruct section depth at actual sample coordinates. Original address and depth images are never resolved. The final outline blend caches its count-specific colour recipe and retains its immutable shader/layout and single-sample alpha inputs. Both factories reuse the exact previously typed descriptors. The current coverage producer remains single-sample until the next wiring step; this checkpoint claims current default-rendering regression coverage and prepared count-aware recipes, not a complete four-sample outline frame. All 1002 Native tests, WASM/optimized builds, Native frame, canonical Chrome contact/resize checks and stored evidence pass. Ten current canonical Native/browser pictures were individually inspected. Complete physical-sample outline/AO/viewer integration, remaining view controls, runtime efficiency, Cargo independence and final whole-course feature-picture review remain unfinished. Maintained Session reload camera/base-grid fixes follow the completed course.

Retain all existing actual section-mask, selected/global outline, single-blend, original ownership and CSS-width assertions. Require fresh current default rendering before connecting the multisample coverage producer.

Actual single-sample contact fixture after preparing lazy section masks and a cached final outline blend. Both densities preserve original owners, complete source archives, shared geometry, contact shading and exact resize restoration. Complete four-sample outline coverage is connected next.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 22.5.12
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
