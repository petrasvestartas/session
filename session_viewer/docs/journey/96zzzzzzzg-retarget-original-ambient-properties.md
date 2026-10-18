# 22.5.14 · Retarget original ambient properties

**Chapter 22 · Surface lighting, contact and display quality · Section 22.5 · Frame resources and multisampling** · [Contents](contents.md)

**Typing: 29–57 minutes.** [Estimate](typing-load.md).

Match original ambient radius and normal sources to the frame sample count. Cache the ordinary and shared source recipes, and replace only their full-resolution source images when the count changes. Keep the coarse hierarchy and filters single-sample.

## Type

Continue from [Resolve float outline coverage](96zzzzzzzf-resolve-float-outline-coverage.md). [Save or recover your work](recovery.md).

### 1. `src/ambient_source_pipeline.rs`

Prepare cached original-radius/normal sample recipes and retarget only their full-resolution source images; retain the one-sample ambient hierarchy and filters.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Pipeline{pub draw:wgpu::RenderPipeline,pub record_draw:wgpu::RenderPipeline}
impl Pipeline{
 pub fn new(device:&wgpu::Device)->Self{
  let create=|shared| {
  let view=crate::surface_view::layout(device);let model=if shared {crate::object_rows::layout(device)}else{crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64)};let clip=crate::clipping_gpu::layout(device);let geometry=crate::gpu_geometry::GpuGeometry::storage_layout(device);let groups=[Some(&view),Some(&model),Some(&clip),Some(&geometry)];let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("ambient original source metadata"),bind_group_layouts:&groups[..if shared{4}else{3}],immediate_size:0});let source=concat!(include_str!("triangle.wgsl"),"
",include_str!("ambient_sources.wgsl"));let source=if shared {crate::object_rows::source(source)}else{source.into()};let module=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("ambient original normals and radii"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(&source,2,0).into())});
  let attributes=wgpu::vertex_attr_array![0=>Float32x3,1=>Uint32,2=>Uint32];let draw=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("ambient original normals and radii"),layout:Some(&layout),vertex:wgpu::VertexState{module:&module,entry_point:Some(if shared {"vs_record"}else{"vs_main"}),compilation_options:Default::default(),buffers:&[if shared {crate::draw_slots::layout()}else{wgpu::VertexBufferLayout{array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&attributes}}]},fragment:Some(wgpu::FragmentState{module:&module,entry_point:Some("fs_main"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R32Float,blend:None,write_mask:wgpu::ColorWrites::RED}),Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::state_for_ambient()),multisample:Default::default(),multiview_mask:None,cache:None});draw};let draw=create(false);let record_draw=create(true);Self{draw,record_draw}
 }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Pipeline{pub draw:wgpu::RenderPipeline,pub record_draw:wgpu::RenderPipeline}
impl Pipeline{
 pub fn new(device:&wgpu::Device)->Self{Self::with_samples(device,1)}
 pub fn with_samples(device:&wgpu::Device,samples:u32)->Self{
  let create=|shared| {
  let view=crate::surface_view::layout(device);let model=if shared {crate::object_rows::layout(device)}else{crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64)};let clip=crate::clipping_gpu::layout(device);let geometry=crate::gpu_geometry::GpuGeometry::storage_layout(device);let groups=[Some(&view),Some(&model),Some(&clip),Some(&geometry)];let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("ambient original source metadata"),bind_group_layouts:&groups[..if shared{4}else{3}],immediate_size:0});let source=concat!(include_str!("triangle.wgsl"),"
",include_str!("ambient_sources.wgsl"));let source=if shared {crate::object_rows::source(source)}else{source.into()};let module=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("ambient original normals and radii"),source:wgpu::ShaderSource::Wgsl(if samples==4{crate::clipping_gpu::source(&source,2,0).replace("@location(0) world:","@location(0) @interpolate(perspective,sample) world:").into()}else{crate::clipping_gpu::source(&source,2,0).into()})});
  let attributes=wgpu::vertex_attr_array![0=>Float32x3,1=>Uint32,2=>Uint32];let draw=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("ambient original normals and radii"),layout:Some(&layout),vertex:wgpu::VertexState{module:&module,entry_point:Some(if shared {"vs_record"}else{"vs_main"}),compilation_options:Default::default(),buffers:&[if shared {crate::draw_slots::layout()}else{wgpu::VertexBufferLayout{array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&attributes}}]},fragment:Some(wgpu::FragmentState{module:&module,entry_point:Some("fs_main"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R32Float,blend:None,write_mask:wgpu::ColorWrites::RED}),Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::state_for_ambient()),multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None});draw};let draw=create(false);let record_draw=create(true);Self{draw,record_draw}
 }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-01.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-02.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-03.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-04.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-05.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-06.rs"
```

</details>

### 2. `src/ambient_source_gpu.rs`

Prepare cached original-radius/normal sample recipes and retarget only their full-resolution source images; retain the one-sample ambient hierarchy and filters.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Sources{pipes:crate::ambient_source_pipeline::Pipeline}
impl Sources{
 pub fn new(device:&wgpu::Device)->Self{Self{pipes:crate::ambient_source_pipeline::Pipeline::new(device)}}
 pub fn encode(&mut self,encoder:&mut wgpu::CommandEncoder,radius:&wgpu::TextureView,normals:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clip:&wgpu::BindGroup,meshes:&[crate::gpu_mesh::GpuMesh],records:Option<&crate::instance_gpu::Lane>){
  let attachments=[radius,normals].map(|view|Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}}));let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original ambient source properties"),color_attachments:&attachments,depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&self.pipes.draw);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clip,&[]);for mesh in meshes{if records.is_some_and(|rows|rows.bind_record(&mut pass,mesh)){pass.set_pipeline(&self.pipes.record_draw);mesh.geometry.draw_pulled(&mut pass,1);}else{pass.set_pipeline(&self.pipes.draw);mesh.bind_model(&mut pass);mesh.geometry.draw(&mut pass);}}
 }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{sample_pipelines::Variants,ambient_source_pipeline::Pipeline};
pub struct Sources{pipes:Variants<Pipeline>}
impl Sources{
 pub fn new(device:&wgpu::Device)->Self{Self::with_samples(device,1)}
 pub fn with_samples(device:&wgpu::Device,samples:u32)->Self{Self{pipes:Variants::new(Pipeline::with_samples(device,samples),samples)}}
 pub fn set_samples(&mut self,device:&wgpu::Device,samples:u32){self.pipes.select(samples,||Pipeline::with_samples(device,samples));}
 pub fn encode(&mut self,encoder:&mut wgpu::CommandEncoder,radius:&wgpu::TextureView,normals:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clip:&wgpu::BindGroup,meshes:&[crate::gpu_mesh::GpuMesh],records:Option<&crate::instance_gpu::Lane>){
  let attachments=[radius,normals].map(|view|Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}}));let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original ambient source properties"),color_attachments:&attachments,depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&self.pipes.active.draw);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clip,&[]);for mesh in meshes{if records.is_some_and(|rows|rows.bind_record(&mut pass,mesh)){pass.set_pipeline(&self.pipes.active.record_draw);mesh.geometry.draw_pulled(&mut pass,1);}else{pass.set_pipeline(&self.pipes.active.draw);mesh.bind_model(&mut pass);mesh.geometry.draw(&mut pass);}}
 }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-07.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-08.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-09.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-10.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-11.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-12.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-13.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-14.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-15.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-16.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-17.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-18.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-19.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-20.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-21.rs"
```

</details>

### 3. `src/ambient_image.rs`

Prepare cached original-radius/normal sample recipes and retarget only their full-resolution source images; retain the one-sample ambient hierarchy and filters.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Image{pub texture:wgpu::Texture,pub view:wgpu::TextureView,pub bytes:u64,pub size:[u32;2]}
impl Image{
 pub fn new(device:&wgpu::Device,size:[u32;2],format:wgpu::TextureFormat)->Self{let texture=device.create_texture(&wgpu::TextureDescriptor{label:Some("ambient actual image"),size:wgpu::Extent3d{width:size[0],height:size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format,usage:wgpu::TextureUsages::TEXTURE_BINDING|wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::COPY_SRC,view_formats:&[]});let view=texture.create_view(&Default::default());let bytes=u64::from(size[0])*u64::from(size[1])*u64::from(format.block_copy_size(None).unwrap());Self{texture,view,bytes,size}}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Image{pub texture:wgpu::Texture,pub view:wgpu::TextureView,pub bytes:u64,pub size:[u32;2]}
impl Image{
 pub fn new(device:&wgpu::Device,size:[u32;2],format:wgpu::TextureFormat)->Self{Self::with_samples(device,size,format,1)}
 pub fn with_samples(device:&wgpu::Device,size:[u32;2],format:wgpu::TextureFormat,samples:u32)->Self{assert!(matches!(samples,1|4));let texture=device.create_texture(&wgpu::TextureDescriptor{label:Some("ambient actual image"),size:wgpu::Extent3d{width:size[0],height:size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:samples,dimension:wgpu::TextureDimension::D2,format,usage:wgpu::TextureUsages::TEXTURE_BINDING|wgpu::TextureUsages::RENDER_ATTACHMENT|if samples==1 {wgpu::TextureUsages::COPY_SRC}else{wgpu::TextureUsages::empty()},view_formats:&[]});let view=texture.create_view(&Default::default());let bytes=u64::from(size[0])*u64::from(size[1])*u64::from(format.block_copy_size(None).unwrap())*u64::from(samples);Self{texture,view,bytes,size}}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-22.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-23.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-24.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-25.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-26.rs"
```

</details>

### 4. `src/ambient_images.rs`

Prepare cached original-radius/normal sample recipes and retarget only their full-resolution source images; retain the one-sample ambient hierarchy and filters.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{ambient_image::Image,pyramid_image::Image as Pyramid};
pub struct Images{pub full:[u32;2],pub size:[u32;2],pub source_radius:Image,pub source_normals:Image,pub linear:Pyramid,pub radii:Pyramid,pub normals:Image,pub occupancy:Image,pub ground:Image,pub ao:[Image;3],pub history:Image,pub history_depth:Image}
impl Images{
 pub fn new(device:&wgpu::Device,full:[u32;2],density:f32)->Self{
  let(w,h)=crate::pyramid_size::resolution((full[0],full[1]),f64::from(density));let size=[w.max(32),h.max(32)];let image=|size,format|Image::new(device,size,format);
  let source_radius=image(full,wgpu::TextureFormat::R32Float);let source_normals=image(full,wgpu::TextureFormat::Rg8Unorm);let linear=Pyramid::new(device,size,wgpu::TextureFormat::R32Float);let radii=Pyramid::new(device,size,wgpu::TextureFormat::R16Uint);let normals=image(size,wgpu::TextureFormat::Rg8Unorm);let occupancy=image(size,wgpu::TextureFormat::R8Unorm);let ground=image(size,wgpu::TextureFormat::R8Unorm);let ao=std::array::from_fn(|i|image(if i==2{full}else{size},wgpu::TextureFormat::R8Unorm));let history=image(size,wgpu::TextureFormat::R8Unorm);let history_depth=image(size,wgpu::TextureFormat::R32Float);Self{full,size,source_radius,source_normals,linear,radii,normals,occupancy,ground,ao,history,history_depth}
 }
 pub fn bytes(&self)->u64{self.source_radius.bytes+self.source_normals.bytes+self.linear.bytes+self.radii.bytes+self.normals.bytes+self.occupancy.bytes+self.ground.bytes+self.ao.iter().map(|i|i.bytes).sum::<u64>()+self.history.bytes+self.history_depth.bytes}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{ambient_image::Image,pyramid_image::Image as Pyramid};
pub struct Images{pub full:[u32;2],pub size:[u32;2],pub source_radius:Image,pub source_normals:Image,pub linear:Pyramid,pub radii:Pyramid,pub normals:Image,pub occupancy:Image,pub ground:Image,pub ao:[Image;3],pub history:Image,pub history_depth:Image}
impl Images{
 pub fn new(device:&wgpu::Device,full:[u32;2],density:f32)->Self{Self::with_samples(device,full,density,1)}
 pub fn with_samples(device:&wgpu::Device,full:[u32;2],density:f32,samples:u32)->Self{
  let(w,h)=crate::pyramid_size::resolution((full[0],full[1]),f64::from(density));let size=[w.max(32),h.max(32)];let image=|size,format|Image::new(device,size,format);
  let source_radius=Image::with_samples(device,full,wgpu::TextureFormat::R32Float,samples);let source_normals=Image::with_samples(device,full,wgpu::TextureFormat::Rg8Unorm,samples);let linear=Pyramid::new(device,size,wgpu::TextureFormat::R32Float);let radii=Pyramid::new(device,size,wgpu::TextureFormat::R16Uint);let normals=image(size,wgpu::TextureFormat::Rg8Unorm);let occupancy=image(size,wgpu::TextureFormat::R8Unorm);let ground=image(size,wgpu::TextureFormat::R8Unorm);let ao=std::array::from_fn(|i|image(if i==2{full}else{size},wgpu::TextureFormat::R8Unorm));let history=image(size,wgpu::TextureFormat::R8Unorm);let history_depth=image(size,wgpu::TextureFormat::R32Float);Self{full,size,source_radius,source_normals,linear,radii,normals,occupancy,ground,ao,history,history_depth}
 }
 pub fn retarget_sources(&mut self,device:&wgpu::Device,samples:u32){
  self.source_radius=Image::with_samples(device,self.full,wgpu::TextureFormat::R32Float,samples);self.source_normals=Image::with_samples(device,self.full,wgpu::TextureFormat::Rg8Unorm,samples);
 }
 pub fn bytes(&self)->u64{self.source_radius.bytes+self.source_normals.bytes+self.linear.bytes+self.radii.bytes+self.normals.bytes+self.occupancy.bytes+self.ground.bytes+self.ao.iter().map(|i|i.bytes).sum::<u64>()+self.history.bytes+self.history_depth.bytes}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-27.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-28.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-29.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-30.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-31.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-32.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-33.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-34.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-35.rs"
```

</details>

### 5. `src/ambient.wgsl`

Prepare cached original-radius/normal sample recipes and retarget only their full-resolution source images; retain the one-sample ambient hierarchy and filters.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```wgsl
    }
    if all(values==vec4<f32>(0.0)) { return 0.0; }
    let center = depth_sample(pixel.xy,0);
    let first = reconstruct(pixel.xy,0,center,values);
    if !MSAA { return first; }
    var ao = vec4<f32>(first);
    var different = false;
    for (var i=1; i<4; i++) {
        let z = depth_sample(pixel.xy,i);
        if (z>0.0)!=(center>0.0) || abs(z-center)>max(center*0.001,1e-7) {
            ao[i] = reconstruct(pixel.xy,i,z,values);
            different = different || ao[i]!=first;
        }
```

</details>

Replace it with:

```wgsl
    }
    if all(values==vec4<f32>(0.0)) { return 0.0; }
    let center = depth_sample(pixel.xy,0);
    let primitive=source_primitive(source_at(pixel.xy),0);
    let first = reconstruct(pixel.xy,0,center,values);
    if !MSAA { return first; }
    var ao = vec4<f32>(first);
    var different = false;
    for (var i=1; i<4; i++) {
        let z = depth_sample(pixel.xy,i);
        if (z>0.0)!=(center>0.0) || abs(z-center)>max(center*0.001,1e-7) || source_primitive(source_at(pixel.xy),i)!=primitive {
            ao[i] = reconstruct(pixel.xy,i,z,values);
            different = different || ao[i]!=first;
        }
```

<details>
<summary>Locate this block</summary>

```wgsl
    var radius = ambient.params.y;
    if geometry {
        n=normal_of(at,p.xyz,sample);
        radius=source_radius(at);
    }
    let origin = full_pixel(vec2<f32>(base)+0.5);
    let blend = (at-origin)/(full_pixel(vec2<f32>(base)+1.5)-origin);
```

</details>

Replace it with:

```wgsl
    var radius = ambient.params.y;
    if geometry {
        n=normal_of(at,p.xyz,sample);
        radius=source_radius(at,sample);
    }
    let origin = full_pixel(vec2<f32>(base)+0.5);
    let blend = (at-origin)/(full_pixel(vec2<f32>(base)+1.5)-origin);
```

<details>
<summary>Locate this block</summary>

```wgsl
    }
    let p = world(at,z);
    let along = dot(p-origin,ray)/dot(ray,ray);
    let radius=source_radius(at)/max(ambient.params.y,1e-20);
    let bits = (pack2x16float(vec2<f32>(radius,0.0))+2u)>>2u;
    let coord = vec2<u32>(pixel.xy);
    let encoded = (bits<<4u)|(coord.x&3u)|((coord.y&3u)<<2u);
```

</details>

Replace it with:

```wgsl
    }
    let p = world(at,z);
    let along = dot(p-origin,ray)/dot(ray,ray);
    let radius=source_radius(at,0)/max(ambient.params.y,1e-20);
    let bits = (pack2x16float(vec2<f32>(radius,0.0))+2u)>>2u;
    let coord = vec2<u32>(pixel.xy);
    let encoded = (bits<<4u)|(coord.x&3u)|((coord.y&3u)<<2u);
```

<details>
<summary>Locate this block</summary>

```wgsl
    return decode_normal(textureLoad(normals,coord,0).xy*2.0-1.0);
}
fn source_at(pixel:vec2<f32>)->vec2<i32>{return clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1);}
fn source_primitive(at:vec2<i32>)->u32{
 let size=vec2<i32>(textureDimensions(source_primitives));
 let halves=textureLoad(source_primitives,clamp(at,vec2<i32>(0),size-1),0).xy;
 return physical_address(halves);
}
fn source_radius(pixel:vec2<f32>)->f32{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,0).x;
 let primitive=source_primitive(at);
 return select(radius,ambient.params.y,radius==0.0&&clip_is_cap(primitive));
}
fn normal_of(pixel:vec2<f32>,p:vec3<f32>,sample:i32)->vec3<f32>{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,0).x;
 if(radius>0.0){let n=decode_normal(textureLoad(source_normals,at,0).xy*2.0-1.0);return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
 let primitive=source_primitive(at);
 if(clip_is_cap(primitive)){
  let plane=clip_cap_plane(primitive);
  if(plane<clipping.count){let n=-clipping.planes[plane].xyz;return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
```

</details>

Replace it with:

```wgsl
    return decode_normal(textureLoad(normals,coord,0).xy*2.0-1.0);
}
fn source_at(pixel:vec2<f32>)->vec2<i32>{return clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1);}
fn source_primitive(at:vec2<i32>,sample:i32)->u32{
 let size=vec2<i32>(textureDimensions(source_primitives));
 let halves=textureLoad(source_primitives,clamp(at,vec2<i32>(0),size-1),sample).xy;
 return physical_address(halves);
}
fn source_radius(pixel:vec2<f32>,sample:i32)->f32{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,sample).x;
 let primitive=source_primitive(at,sample);
 return select(radius,ambient.params.y,radius==0.0&&clip_is_cap(primitive));
}
fn normal_of(pixel:vec2<f32>,p:vec3<f32>,sample:i32)->vec3<f32>{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,sample).x;
 if(radius>0.0){let n=decode_normal(textureLoad(source_normals,at,sample).xy*2.0-1.0);return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
 let primitive=source_primitive(at,sample);
 if(clip_is_cap(primitive)){
  let plane=clip_cap_plane(primitive);
  if(plane<clipping.count){let n=-clipping.planes[plane].xyz;return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
```

<details>
<summary>Complete updated block reference</summary>

```wgsl
const MSAA: bool = false;
@group(3) @binding(0) var<storage,read_write> corrections: array<u32>;
@group(3) @binding(1) var<storage,read_write> edge_flags: array<atomic<u32>>;
@group(3) @binding(2) var<storage,read_write> edge_tiles: array<u32>;
@group(3) @binding(4) var history_ao: texture_2d<f32>;
@group(3) @binding(5) var history_depth: texture_2d<f32>;
struct Draw { vertices: u32, instances: atomic<u32>, first_vertex: u32, first_instance: u32 };
@group(3) @binding(3) var<storage,read_write> draw_edges: Draw;
struct Ambient {
    inverse_mvp: mat4x4<f32>,
    mvp: mat4x4<f32>,
    params: vec4<f32>,
    extent: vec4<f32>,
    near: vec4<f32>,
    near_x: vec4<f32>,
    near_y: vec4<f32>,
    ray: vec4<f32>,
    ray_x: vec4<f32>,
    ray_y: vec4<f32>,
    previous_mvp: mat4x4<f32>,
};
@group(0) @binding(0) var depth: texture_depth_2d;
@group(0) @binding(1) var<uniform> ambient: Ambient;
@group(0) @binding(2) var source_radii:texture_2d<f32>;
@group(0) @binding(3) var source_normals:texture_2d<f32>;
@group(0) @binding(4) var source_primitives:texture_2d<u32>;
// Geometry access: ambient_geometry.wgsl, appended by ssao.rs
@group(1) @binding(0) var linear: texture_2d<f32>;
@group(1) @binding(1) var radii: texture_2d<u32>;
@group(1) @binding(2) var normals: texture_2d<f32>;
@group(2) @binding(0) var occlusion: texture_2d<f32>;
@group(2) @binding(1) var linear_sampler: sampler;

struct DepthRadius {
    @location(0) depth: f32,
    @location(1) radius: u32,
    @location(2) normal: vec2<f32>,
};
struct Hit {
    point: vec3<f32>,
    along: f32,
    radius: f32,
};

fn fullscreen(i: u32) -> vec4<f32> {
    let p = array<vec2<f32>, 3>(vec2<f32>(-1.0,-1.0), vec2<f32>(3.0,-1.0), vec2<f32>(-1.0,3.0));
    return vec4<f32>(p[i],0.0,1.0);
}
@vertex fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> { return fullscreen(i); }
fn world(pixel: vec2<f32>, z: f32) -> vec3<f32> {
    let uv = pixel / ambient.params.zw;
    let p = ambient.inverse_mvp * vec4<f32>(uv.x*2.0-1.0, 1.0-uv.y*2.0, z, 1.0);
    return p.xyz / p.w;
}
fn ray_origin(pixel: vec2<f32>) -> vec3<f32> {
    return ambient.near.xyz+ambient.near_x.xyz*pixel.x+ambient.near_y.xyz*pixel.y;
}
fn ray_direction(pixel: vec2<f32>) -> vec3<f32> {
    return ambient.ray.xyz+ambient.ray_x.xyz*pixel.x+ambient.ray_y.xyz*pixel.y;
}
fn depth_sample(pixel: vec2<f32>, sample: i32) -> f32 {
    return textureLoad(depth,clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1),sample);
}
fn surface_at_depth(pixel: vec2<f32>, z: f32) -> vec4<f32> {
    if z>0.0 { return vec4<f32>(world(pixel,z),1.0); }
    let near = ray_origin(pixel);
    let direction = ray_direction(pixel);
    if direction.z>=-1e-7 || near.z<ambient.params.x { return vec4<f32>(0.0); }
    return vec4<f32>(near+direction*((ambient.params.x-near.z)/direction.z),1.0);
}
fn surface(pixel: vec2<f32>, sample: i32) -> vec4<f32> {
    return surface_at_depth(pixel,depth_sample(pixel,sample));
}
fn full_pixel(pixel: vec2<f32>) -> vec2<f32> {
    return floor(pixel/ambient.extent.yz*ambient.params.zw)+0.5;
}
fn position_at(pixel: vec2<f32>, along: f32) -> vec3<f32> {
    return ray_origin(pixel)+ray_direction(pixel)*abs(along);
}

fn encode_normal(n: vec3<f32>) -> vec2<f32> {
    let p = n.xy/(abs(n.x)+abs(n.y)+abs(n.z));
    let folded = (1.0-abs(p.yx))*select(vec2<f32>(-1.0),vec2<f32>(1.0),p>=vec2<f32>(0.0));
    return select(folded,p,n.z>=0.0);
}
fn decode_normal(e: vec2<f32>) -> vec3<f32> {
    let z = 1.0-abs(e.x)-abs(e.y);
    let t = max(-z,0.0);
    let xy = e-select(vec2<f32>(-t),vec2<f32>(t),e>=vec2<f32>(0.0));
    return normalize(vec3<f32>(xy,z));
}
fn normal_half(coord: vec2<i32>) -> vec3<f32> {
    return decode_normal(textureLoad(normals,coord,0).xy*2.0-1.0);
}
fn source_at(pixel:vec2<f32>)->vec2<i32>{return clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1);}
fn source_primitive(at:vec2<i32>,sample:i32)->u32{
 let size=vec2<i32>(textureDimensions(source_primitives));
 let halves=textureLoad(source_primitives,clamp(at,vec2<i32>(0),size-1),sample).xy;
 return physical_address(halves);
}
fn source_radius(pixel:vec2<f32>,sample:i32)->f32{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,sample).x;
 let primitive=source_primitive(at,sample);
 return select(radius,ambient.params.y,radius==0.0&&clip_is_cap(primitive));
}
fn normal_of(pixel:vec2<f32>,p:vec3<f32>,sample:i32)->vec3<f32>{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,sample).x;
 if(radius>0.0){let n=decode_normal(textureLoad(source_normals,at,sample).xy*2.0-1.0);return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
 let primitive=source_primitive(at,sample);
 if(clip_is_cap(primitive)){
  let plane=clip_cap_plane(primitive);
  if(plane<clipping.count){let n=-clipping.planes[plane].xyz;return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
 }
 let a=surface(pixel+vec2<f32>(1.,0.),sample);let b=surface(pixel-vec2<f32>(1.,0.),sample);let c=surface(pixel+vec2<f32>(0.,1.),sample);let d=surface(pixel-vec2<f32>(0.,1.),sample);
 let dx=select(p-b.xyz,a.xyz-p,a.w>0.&& (b.w==0.||distance(a.xyz,p)<distance(b.xyz,p)));let dy=select(p-d.xyz,c.xyz-p,c.w>0.&&(d.w==0.||distance(c.xyz,p)<distance(d.xyz,p)));let n=cross(dx,dy);let normal=n/max(length(n),1e-12);return select(-normal,normal,dot(normal,-ray_direction(pixel))>0.);
}

@fragment fn fs_prepare(@builtin(position) pixel: vec4<f32>) -> DepthRadius {
    let at = full_pixel(pixel.xy);
    let ray = ray_direction(at);
    let origin = ray_origin(at);
    let z = depth_sample(at,0);
    if z==0.0 {
        if ray.z>=-1e-7 || origin.z<ambient.params.x { return DepthRadius(0.0,0u,vec2<f32>(0.5)); }
        return DepthRadius(-(ambient.params.x-origin.z)/ray.z,0u,vec2<f32>(0.5));
    }
    let p = world(at,z);
    let along = dot(p-origin,ray)/dot(ray,ray);
    let radius=source_radius(at,0)/max(ambient.params.y,1e-20);
    let bits = (pack2x16float(vec2<f32>(radius,0.0))+2u)>>2u;
    let coord = vec2<u32>(pixel.xy);
    let encoded = (bits<<4u)|(coord.x&3u)|((coord.y&3u)<<2u);
    return DepthRadius(along,encoded,encode_normal(normal_of(at,p,0))*0.5+0.5);
}
fn radius_at(coord: vec2<i32>) -> f32 {
    return unpack2x16float((textureLoad(radii,coord,0).x>>4u)<<2u).x*ambient.params.y;
}
fn sample_hit(at: vec2<f32>, offset: vec2<f32>, level: i32) -> Hit {
    let dims = vec2<i32>(textureDimensions(linear,level));
    let coord = vec2<i32>(floor((at+offset)/ambient.params.zw*ambient.extent.yz))>>vec2<u32>(u32(level));
    if any(coord<vec2<i32>(0)) || any(coord>=dims) { return Hit(vec3<f32>(0.0),0.0,0.0); }
    let along = textureLoad(linear,coord,level).x;
    let encoded = textureLoad(radii,coord,level).x;
    let mask = (1u<<u32(level))-1u;
    let offset_coord = vec2<u32>(encoded&3u,(encoded>>2u)&3u)&vec2<u32>(mask);
    let source_coord = (vec2<u32>(coord)<<vec2<u32>(u32(level)))+offset_coord;
    let pixel = full_pixel(vec2<f32>(source_coord)+0.5);
    return Hit(position_at(pixel,along),along,unpack2x16float((encoded>>4u)<<2u).x*ambient.params.y);
}
fn stride() -> u32 {
    return select(1u,2u,ambient.near.w!=0.0);
}
fn phase(pixel: vec2<f32>, stride: u32) -> u32 {
    return (u32(pixel.x)+u32(pixel.y))%stride;
}

fn ground_contact(at: vec2<f32>, p: vec3<f32>, pixel_world: f32, noise: f32, phase: u32) -> f32 {
    let dims = vec2<i32>(textureDimensions(occlusion));
    let tile = clamp(vec2<i32>(at/ambient.params.zw*vec2<f32>(dims)),vec2<i32>(0),dims-1);
    if textureLoad(occlusion,tile,0).x==0.0 { return 0.0; }
    let radius_px = min(ambient.params.y*6.0/max(pixel_world,1e-6),64.0);
    let stride = stride();
    var sum = 0.0;
    for (var index=0u; index<24u/stride; index++) {
        let direction = index*stride+phase;
        let angle = (f32(direction)+noise)*0.261799388;
        let axis = vec2<f32>(cos(angle),sin(angle));
        var horizon = 0.0;
        for (var step=0u; step<6u; step++) {
            let jitter = fract(noise+f32(direction)*0.618034);
            let fraction = (f32(step)+0.5+jitter*0.5)/6.0;
            let span = max(1.5,radius_px*fraction*fraction);
            let scaled = span*ambient.extent.y/ambient.params.z;
            let level = i32(scaled>=8.0)+i32(scaled>=16.0);
            let hit = sample_hit(at,axis*span,level);
            let radius = hit.radius*6.0;
            if hit.along<=0.0 || radius<=0.0 { continue; }
            let delta = hit.point-p;
            if delta.z<=0.0 || delta.z>=radius { continue; }
            let elevation = max(delta.z/max(length(delta),1e-6)-0.04,0.0);
            let radial = 1.0-smoothstep(radius*0.1,radius,length(delta.xy));
            let vertical = 1.0-smoothstep(0.0,radius,delta.z);
            horizon = max(horizon,elevation*radial*vertical*vertical);
        }
        sum += horizon;
    }
    return clamp(sum*(2.6*f32(stride)/24.0),0.0,0.65);
}
@fragment fn fs_ground(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    let at = pixel.xy/ambient.extent.xw*ambient.params.zw;
    let ray = ray_direction(at);
    let origin = ray_origin(at);
    if ray.z>=-1e-7 || origin.z<ambient.params.x { return 0.0; }
    let along = (ambient.params.x-origin.z)/ray.z;
    let p = origin+ray*along;
    let pixel_world = max(length(ambient.near_x.xyz+ambient.ray_x.xyz*along),1e-6);
    let noise = fract(52.9829189*fract(dot(floor(pixel.xy),vec2<f32>(0.06711056,0.00583715))));
    return ground_contact(at,p,pixel_world,noise,phase(pixel.xy,stride()));
}
fn filtered_ground(at: vec2<f32>) -> f32 {
    return textureSampleLevel(occlusion,linear_sampler,at/ambient.params.zw,0.0).x;
}

fn arc(h: f32, n: f32) -> f32 {
    return (cos(n)+2.0*h*sin(n)-cos(2.0*h-n))*0.25;
}
@fragment fn fs_main(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    let coord = vec2<i32>(pixel.xy);
    let along = textureLoad(linear,coord,0).x;
    if along==0.0 { return 0.0; }
    let at = full_pixel(pixel.xy);
    let p = position_at(at,along);
    let step_x = ambient.near_x.xyz+ambient.ray_x.xyz*abs(along);
    let step_y = ambient.near_y.xyz+ambient.ray_y.xyz*abs(along);
    let pixel_world = max(length(step_x),1e-6);
    let noise = fract(52.9829189*fract(dot(floor(pixel.xy),vec2<f32>(0.06711056,0.00583715))));
    if along<0.0 { return filtered_ground(at); }
    let radius = radius_at(coord);
    if radius<=0.0 { return 0.0; }
    let normal = normal_half(coord);
    let view = -normalize(ray_direction(at));
    let radius_px = min(radius*8.0/pixel_world,128.0);
    let bias = radius*0.025;
    let stride = stride();
    let phase = phase(pixel.xy,stride);
    var occluded = 0.0;
    for (var index=0u; index<4u/stride; index++) {
        let slice = index*stride+phase;
        let angle = (f32(slice)+noise)*0.785398163;
        let axis = vec2<f32>(cos(angle),sin(angle));
        let tangent = step_x*axis.x+step_y*axis.y;
        let side = normalize(tangent-view*dot(tangent,view));
        let plane_normal = cross(side,view);
        let projected = normal-plane_normal*dot(normal,plane_normal);
        let normal_length = length(projected);
        let n = atan2(dot(projected,side),dot(projected,view));
        var slice_occ = 0.0;
        for (var sign=-1; sign<=1; sign+=2) {
            let signed_n = n*f32(sign);
            let low = cos(signed_n+1.570796327);
            var horizon = low;
            for (var step=0u; step<6u; step++) {
                let fraction = (f32(step)+0.5)/6.0;
                let step_length = max(1.5,radius_px*fraction*fraction);
                let scaled = step_length*ambient.extent.y/ambient.params.z;
                let level = i32(scaled>=8.0)+i32(scaled>=16.0);
                let hit = sample_hit(at,axis*f32(sign)*step_length,level);
                if hit.along<=0.0 { continue; }
                let delta = hit.point-p;
                let span = max(length(delta),1e-6);
                if dot(delta,normal)<=span*0.07+bias { continue; }
                let weight = max(1.0-smoothstep(radius*0.5,radius*2.0,span),0.75*(1.0-smoothstep(radius*4.0,radius*16.0,span)));
                horizon = max(horizon,mix(low,dot(delta/span,view),weight));
            }
            let h = acos(clamp(horizon,-1.0,1.0));
            slice_occ += arc(signed_n+1.570796327,signed_n)-arc(h,signed_n);
        }
        occluded += normal_length*slice_occ;
    }
    return clamp(occluded*(1.08*f32(stride)/4.0),0.0,0.65);
}

fn weight_at(q: vec2<i32>, p: vec3<f32>, n: vec3<f32>, geometry: bool, radius: f32) -> f32 {
    let along = textureLoad(linear,q,0).x;
    if along==0.0 || (along>0.0)!=geometry { return 0.0; }
    let delta = position_at(full_pixel(vec2<f32>(q)+0.5),along)-p;
    let separation = abs(dot(delta,n));
    let tolerance = max(radius*0.024,length(delta)*0.08);
    let ground_weight = select(1.0-smoothstep(0.0,radius,length(delta.xy)),1.0,geometry);
    return exp(-separation/max(tolerance,1e-6))*ground_weight;
}
fn denoise(pixel: vec2<f32>, axis: vec2<i32>) -> vec3<f32> {
    let coord = vec2<i32>(pixel);
    var values: array<f32, 7>;
    var total = 0.0;
    for (var i=0; i<7; i++) {
        let q = clamp(coord+axis*(i-3),vec2<i32>(0),vec2<i32>(ambient.extent.yz)-1);
        values[i] = textureLoad(occlusion,q,0).x;
        total += values[i];
    }
    if total==0.0 { return vec3<f32>(0.0); }
    let along = textureLoad(linear,coord,0).x;
    if along==0.0 { return vec3<f32>(0.0); }
    let at = full_pixel(pixel);
    let p = position_at(at,along);
    var n = vec3<f32>(0.0,0.0,1.0);
    if along>0.0 { n = normal_half(coord); }
    let radius = select(ambient.params.y,radius_at(coord),along>0.0);
    var sum = 0.0;
    var weights = 0.0;
    var low = 1.0;
    var high = 0.0;
    for (var i=0; i<7; i++) {
        let q = clamp(coord+axis*(i-3),vec2<i32>(0),vec2<i32>(ambient.extent.yz)-1);
        let weight = exp(-f32((i-3)*(i-3))*0.12)*weight_at(q,p,n,along>0.0,radius);
        if weight>0.05 { low=min(low,values[i]); high=max(high,values[i]); }
        sum += values[i]*weight;
        weights += weight;
    }
    return vec3<f32>(sum/max(weights,1e-6),low,high);
}

@fragment fn fs_filter_x(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    return denoise(pixel.xy,vec2<i32>(1,0)).x;
}
@fragment fn fs_filter_y(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    let current = denoise(pixel.xy,vec2<i32>(0,1));
    if ambient.near.w==0.0 || current.z==0.0 { return current.x; }
    let along = textureLoad(linear,vec2<i32>(pixel.xy),0).x;
    if along==0.0 { return current.x; }
    let p = position_at(full_pixel(pixel.xy),along);
    let clip = ambient.previous_mvp*vec4<f32>(p,1.0);
    let uv = vec2<f32>(clip.x,-clip.y)/clip.w*0.5+0.5;
    if clip.w<=0.0 || any(uv<=vec2<f32>(0.0)) || any(uv>=vec2<f32>(1.0)) { return current.x; }
    let expected = clip.z/clip.w;
    let dims = vec2<i32>(textureDimensions(history_depth));
    let base = vec2<i32>(floor(uv*vec2<f32>(dims)-0.5));
    var valid = false;
    for(var i=0; i<4; i++) {
        let q = clamp(base+vec2<i32>(i%2,i/2),vec2<i32>(0),dims-1);
        let depth = textureLoad(history_depth,q,0).x;
        valid = valid || (depth!=0.0 && (depth>0.0)==(along>0.0)
            && abs(abs(depth)-expected)<max(abs(expected)*0.0025,1e-6));
    }
    if !valid { return current.x; }
    let previous = textureSampleLevel(history_ao,linear_sampler,uv,0.0).x;
    return mix(current.x,clamp(previous,current.y,current.z),0.8);
}
@fragment fn fs_history(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    let coord = min(vec2<i32>(pixel.xy)*2+1,vec2<i32>(ambient.extent.yz)-1);
    let along = textureLoad(linear,coord,0).x;
    if along==0.0 { return 0.0; }
    let p = position_at(full_pixel(vec2<f32>(coord)+0.5),along);
    let clip = ambient.mvp*vec4<f32>(p,1.0);
    return select(-clip.z/clip.w,clip.z/clip.w,along>0.0);
}

fn source_base(at: vec2<f32>) -> vec2<i32> {
    let base = vec2<i32>(floor(at/ambient.params.zw*ambient.extent.yz-0.5));
    return base-select(vec2<i32>(0),vec2<i32>(1),at<full_pixel(vec2<f32>(base)+0.5));
}
fn reconstruct(at: vec2<f32>, sample: i32, z: f32, values: vec4<f32>) -> f32 {
    let base = source_base(at);
    let p = surface_at_depth(at,z);
    if p.w==0.0 { return 0.0; }
    let geometry = z>0.0;
    var n = vec3<f32>(0.0,0.0,1.0);
    var radius = ambient.params.y;
    if geometry {
        n=normal_of(at,p.xyz,sample);
        radius=source_radius(at,sample);
    }
    let origin = full_pixel(vec2<f32>(base)+0.5);
    let blend = (at-origin)/(full_pixel(vec2<f32>(base)+1.5)-origin);
    var sum = 0.0;
    var weights = 0.0;
    for (var i=0; i<4; i++) {
        let offset = vec2<i32>(i%2,i/2);
        let q = clamp(base+offset,vec2<i32>(0),vec2<i32>(ambient.extent.yz)-1);
        let spatial = mix(1.0-blend,blend,vec2<f32>(offset));
        let w = spatial.x*spatial.y*weight_at(q,p.xyz,n,geometry,radius);
        sum += values[i]*w;
        weights += w;
    }
    return sum/max(weights,1e-6);
}

@fragment fn fs_upsample(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    let xy = vec2<u32>(pixel.xy);
    if MSAA { corrections[xy.y*u32(ambient.params.z)+xy.x] = 0u; }
    let base_coord = source_base(pixel.xy);
    var values = vec4<f32>(0.0);
    for (var i=0; i<4; i++) {
        let q = clamp(base_coord+vec2<i32>(i%2,i/2),vec2<i32>(0),vec2<i32>(ambient.extent.yz)-1);
        values[i] = textureLoad(occlusion,q,0).x;
    }
    if all(values==vec4<f32>(0.0)) { return 0.0; }
    let center = depth_sample(pixel.xy,0);
    let primitive=source_primitive(source_at(pixel.xy),0);
    let first = reconstruct(pixel.xy,0,center,values);
    if !MSAA { return first; }
    var ao = vec4<f32>(first);
    var different = false;
    for (var i=1; i<4; i++) {
        let z = depth_sample(pixel.xy,i);
        if (z>0.0)!=(center>0.0) || abs(z-center)>max(center*0.001,1e-7) || source_primitive(source_at(pixel.xy),i)!=primitive {
            ao[i] = reconstruct(pixel.xy,i,z,values);
            different = different || ao[i]!=first;
        }
    }
    if !different { return first; }
    let base = round(min(min(ao.x,ao.y),min(ao.z,ao.w))*255.0)/255.0;
    let packed = pack4x8unorm((ao-vec4<f32>(base))/max(1.0-base,1e-6));
    if packed!=0u {
        corrections[xy.y*u32(ambient.params.z)+xy.x] = packed;
        let tile_width = (u32(ambient.params.z)+15u)/16u;
        let tile = (xy.y/16u)*tile_width+xy.x/16u;
        if atomicOr(&edge_flags[tile],1u)==0u {
            let index = atomicAdd(&draw_edges.instances,4u)/4u;
            edge_tiles[index] = tile;
        }
    }
    return base;
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-36.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-37.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-38.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-39.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-40.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-41.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-42.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-43.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-44.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-45.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-46.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-47.rs"
```

</details>

### 6. `src/ambient_pipeline.rs`

Assemble the matching original-property/depth input declarations while keeping the coarse hierarchy single-sample. Use the assembled single-sample source without changing the current ambient owner.

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
  let shader=crate::clipping_gpu::source(include_str!("ambient.wgsl"),0,5);let groups=[Some(&source),Some(&level),Some(&sample),Some(&history)];let prepare=pipeline(device,&shader,"fs_prepare",&[Some(&source)],&[wgpu::TextureFormat::R32Float,wgpu::TextureFormat::R16Uint,wgpu::TextureFormat::Rg8Unorm],false);let make=|fragment|pipeline(device,&shader,fragment,&groups,&[wgpu::TextureFormat::R8Unorm],false);
  let ground=make("fs_ground");let raw=make("fs_main");let filter=[make("fs_filter_x"),make("fs_filter_y")];let upsample=make("fs_upsample");let composite=pipeline(device,include_str!("ambient_composite.wgsl"),"fs_main",&[None,Some(&sample)],&[format],true);
  let reduce=pipeline(device,include_str!("ambient_depth.wgsl"),"fs_reduce",&[Some(&level)],&[wgpu::TextureFormat::R32Float,wgpu::TextureFormat::R16Uint],false);let occupancy=pipeline(device,include_str!("ambient_depth.wgsl"),"fs_occupancy",&[Some(&level)],&[wgpu::TextureFormat::R8Unorm],false);
  Self{source,level,sample,history,prepare,reduce,occupancy,ground,raw,filter,upsample,composite}
 }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

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

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-48.rs"
```

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-49.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/ambient_shader_contract.rs`

Retain both existing full Naga contracts and additionally validate both assembled original-property sample modes before their consumer is connected.

<details>
<summary>Locate the existing block</summary>

```text
        include_str!("ambient_composite.wgsl").to_string(),
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-50.rs" skip=59
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 22.5.14
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Original radius/normal source recipes, full-resolution images and assembled shader inputs support matching physical samples. Coarse ambient images retain their existing one-sample ownership. The current ambient producer remains one-sample until the matching shaders and sparse correction path are connected.

**Verified checkpoint in Chrome.**

**Critical browser-scene review accepted.**

Actual single-sample original contact fixture after preparing matching original ambient source recipes and image retargeting. Both densities preserve original owners, complete source archives, shared geometry, contact shading and exact resize restoration. Complete physical-sample ambient shading remains pending.

![Actual single-sample original contact fixture after preparing matching original ambient source recipes and image retargeting. Both densities preserve original owners, complete source archives, shared geometry, contact shading and exact resize restoration. Complete physical-sample ambient shading remains pending.](../screenshots/journey/96zzzzzzt-keep-contact-ambient-inputs-on-originals-browser.png)

[Verification scope](release.md).

This reference image is shared with [22.4.3 · Keep contact ambient inputs on originals](96zzzzzzt-keep-contact-ambient-inputs-on-originals.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Ambient inputs describe the original source face, not authored display-lighting normals. The source pass uses the existing equal-depth policy and sample-qualified original world positions. Both ordinary and shared object paths retain their source geometry, placement records and original properties. The shared two-count cache selects the matching recipe without another source upload.

The image owner accounts for the physical sample count in actual byte usage. Four-sample source images remain renderable and bindable; their original integer/depth inputs are not resolved. One-sample images retain the existing readback capability used by the actual Native witnesses. A dedicated retarget operation replaces only full-resolution radius/normal images and leaves the hierarchy, coarse normals, occupancy, filtered ambient and history images intact.

Carry the same explicit physical sample through original radius, normal, primitive and depth reads. Sample zero remains the representative coarse input; sparse per-sample reconstruction also compares original primitive identity. Assemble matching texture declarations and retain full Naga validation of the original and both assembled recipes.

The active ambient owner still creates one-sample resources until its matching shader bindings and sparse corrections are connected. Existing actual GPU regressions verify source geometric normals, contact radii, shared placements, released source records, cached contact shading and image release here. The complete four-sample source/property readback is required after the remaining ambient pieces are integrated.

Original face radii/normals → matching physical source recipes/images → retained one-sample ambient hierarchy.



Which ambient resources need the physical sample count?

The full-resolution original radius/normal inputs and their source recipes must match physical depth samples. Coarse hierarchy, occupancy, filtering and history images remain single-sample; a colour count change should retain them.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Run the actual source-normal, shared contact-radius and cached-contact regressions. Compare the new retarget operation with full image preparation: it changes only the two full-resolution original-property images. Check that physical sample count contributes to byte accounting and no four-sample source image is averaged into an original property.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 22.5.14
npm --prefix ../session_tests run course -- save 22.5.14
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Original ambient radius/normal source recipes use the shared two-count cache and matching sample-qualified world positions. Ordinary/shared geometry, original property records and placements remain retained. Only the two full-resolution radius/normal images are retargeted; the one-sample hierarchy, coarse normals, occupancy, filters and history images remain retained. Byte usage includes the actual physical count and four-sample source images do not add readback flags. Current source-normal, original radius, shared/released-source, cached contact and image-release GPU regressions pass. The active ambient owner remains single-sample until its matching consumers and sparse corrections are connected. Matching assembled shader declarations carry the physical sample through radius, normal, primitive and depth reads; coarse preparation retains representative sample zero and sparse reconstruction also compares original primitive identity. Existing full Naga contracts retain their assertions and additionally validate both assembled modes. All 1003 Native tests, WASM/optimized builds, Native frame, canonical Chrome contact/resize checks and stored evidence pass. Ten current canonical Native/browser pictures were individually inspected. Complete physical-sample ambient shaders/corrections and viewer integration, remaining view controls, runtime efficiency, Cargo independence and final whole-course feature-picture review remain unfinished. Maintained Session reload camera/base-grid fixes follow the completed course.

Retain every actual geometric-normal, authored-lighting, original contact-radius, shared-placement, released-source, contact shading, cache and image-release assertion. Require unchanged default rendering before integrating the sample-aware ambient consumer.

Actual single-sample original contact fixture after preparing matching original ambient source recipes and image retargeting. Both densities preserve original owners, complete source archives, shared geometry, contact shading and exact resize restoration. Complete physical-sample ambient shading remains pending.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 22.5.14
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
