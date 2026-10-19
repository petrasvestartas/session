# 22.5.17 · Cache physical depth-pyramid inputs

**Chapter 22 · Surface lighting, contact and display quality · Section 22.5 · Frame resources and multisampling** · [Contents](contents.md)

**Typing: 30–60 minutes.** [Estimate](typing-load.md).

Read the matching physical depth texture through a cached input recipe. Retain the six coarse pyramid levels, encoded radius/pixel addresses and one-sample reductions. A count change invalidates only the incompatible input binding and its prepared result.

## Type

Continue from [Correct ambient at physical samples](96zzzzzzzi-correct-ambient-at-physical-samples.md). [Save or recover your work](recovery.md).

### 1. `src/pyramid_pipeline.rs`

Cache the matching physical-depth input recipe; retain the exact render descriptor, one-sample reductions and six coarse image levels.

<details>
<summary>Locate the existing block</summary>

```rust
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
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzzzzzzj-cache-physical-depth-pyramid-inputs-01.rs"
```

### 2. `src/pyramid_gpu.rs`

Retain coarse image identities and reduction groups; invalidate only an incompatible physical-depth input binding and its prepared result.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::pyramid_image::Image;
struct Images{size:[u32;2],pub linear:Image,pub radii:Image,groups:[wgpu::BindGroup;5]}
pub struct Pyramid{pipes:crate::pyramid_pipeline::Pipelines,frame:wgpu::Buffer,images:Option<Images>,input:Option<(wgpu::TextureView,wgpu::BindGroup)>,cached:Option<([u32;16],[u32;2],u32,[u64;2])>,pub preparations:u64,pub writes:u64,pub passes:u64,pub error:Option<&'static str>}
impl Pyramid{
 pub fn new(device:&wgpu::Device)->Self{Self{pipes:crate::pyramid_pipeline::Pipelines::new(device),frame:device.create_buffer(&wgpu::BufferDescriptor{label:Some("ambient earlier camera"),size:112,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false}),images:None,input:None,cached:None,preparations:0,writes:0,passes:0,error:None}}
 fn prepare(&mut self,device:&wgpu::Device,depth:&wgpu::TextureView,full:[u32;2],density:f32){
  let (width,height)=crate::pyramid_size::resolution((full[0],full[1]),f64::from(density));let size=[width.max(32),height.max(32)];
  if self.images.as_ref().is_none_or(|images|images.size!=size){let linear=Image::new(device,size,wgpu::TextureFormat::R32Float);let radii=Image::new(device,size,wgpu::TextureFormat::R16Uint);let groups=std::array::from_fn(|i|device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient previous level"),layout:&self.pipes.level,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&linear.levels[i])},wgpu::BindGroupEntry{binding:1,resource:wgpu::BindingResource::TextureView(&radii.levels[i])}]}));self.images=Some(Images{size,linear,radii,groups});self.cached=None;self.preparations+=1;}
  if self.input.as_ref().is_none_or(|(old,_)|old!=depth){let group=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient source depth"),layout:&self.pipes.input,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(depth)},wgpu::BindGroupEntry{binding:1,resource:self.frame.as_entire_binding()}]});self.input=Some((depth.clone(),group));self.cached=None;}
 }
 pub fn encode(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,depth:&wgpu::TextureView,matrix:[f32;16],full:[u32;2],density:f32,revision:[u64;2]){
  if full.contains(&0)||!density.is_finite()||density<=0.||matrix.iter().any(|v|!v.is_finite()){self.error=Some("Invalid ambient depth view");self.cached=None;return;}
  self.prepare(device,depth,full,density);let key=(matrix.map(f32::to_bits),full,density.to_bits(),revision);if self.cached==Some(key){return;}
  let Some(inverse)=crate::clipping_uniform::inverse(&session_rust::Xform::from_matrix(matrix.map(f64::from)))else{self.error=Some("Ambient depth camera is singular");self.cached=None;return;};if inverse.m.iter().any(|v|!(*v as f32).is_finite()){self.error=Some("Ambient inverse exceeds display range");self.cached=None;return;}let images=self.images.as_ref().unwrap();let mut bytes=inverse.m.into_iter().flat_map(|v|(v as f32).to_ne_bytes()).collect::<Vec<_>>();bytes.extend([full[0]as f32,full[1]as f32,images.size[0]as f32,images.size[1]as f32,0.05,0.,0.,0.,0.,0.,0.,0.].into_iter().flat_map(f32::to_ne_bytes));queue.write_buffer(&self.frame,0,&bytes);self.writes+=1;
  for level in 0..6{let attachments=[&images.linear.levels[level],&images.radii.levels[level]].map(|view|Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}}));let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("ambient nearest positive depth"),color_attachments:&attachments,..Default::default()});if level==0{pass.set_pipeline(&self.pipes.prepare);pass.set_bind_group(0,&self.input.as_ref().unwrap().1,&[]);}else{pass.set_pipeline(&self.pipes.reduce);pass.set_bind_group(0,&images.groups[level-1],&[]);}pass.draw(0..3,0..1);self.passes+=1;}
  self.cached=Some(key);self.error=None;
 }
 pub fn usage(&self)->[u64;4]{[self.frame.size()+self.images.as_ref().map_or(0,|i|i.linear.bytes+i.radii.bytes),self.preparations,self.writes,self.passes]}
 pub fn linear(&self)->Option<&Image>{self.images.as_ref().map(|i|&i.linear)}
 pub fn radii(&self)->Option<&Image>{self.images.as_ref().map(|i|&i.radii)}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzzzzzzj-cache-physical-depth-pyramid-inputs-02.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/pyramid_gpu_tests.rs`

Extend the existing actual six-level positive-depth/radius-address GPU assertions to both physical inputs. The isolated fixture retargets only its active consumers until the next production API step.

<details>
<summary>Locate the existing block</summary>

```text
use crate::cap_gpu_tests::{scene,renderer,pixels,wait};
fn level(r:&crate::renderer::Renderer,texture:&wgpu::Texture,mip:u32,bytes:u32)->Vec<u8>{
 let width=texture.width()>>mip;let height=texture.height()>>mip;let pitch=(width*bytes).div_ceil(256)*256;let buffer=r.device.create_buffer(&wgpu::BufferDescriptor{label:Some("actual pyramid fixture"),size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=r.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(wgpu::TexelCopyTextureInfo{mip_level:mip,..texture.as_image_copy()},wgpu::TexelCopyBufferInfo{buffer:&buffer,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},wgpu::Extent3d{width,height,depth_or_array_layers:1});r.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();buffer.slice(..).map_async(wgpu::MapMode::Read,move|v|tx.send(v).unwrap());wait(r);rx.recv().unwrap().unwrap();let view=buffer.slice(..).get_mapped_range();let data=view.chunks_exact(pitch as usize).flat_map(|r|r[..(width*bytes)as usize].iter().copied()).collect();drop(view);buffer.unmap();data
}
#[test]
fn actual_pyramid_uses_six_nearest_positive_depth_levels_and_keeps_radius_addresses(){
 let(scene,_)=scene(false);let r=renderer(&scene);r.set_depth_pyramid(true);pixels(&r);assert_eq!(r.depth_pyramid_error(),None);let owner=r.pyramid.borrow();let owner=owner.as_ref().unwrap();let linear=owner.linear().unwrap();let radii=owner.radii().unwrap();let mut previous:Option<(Vec<f32>,Vec<u16>,usize,usize)>=None;
 for mip in 0..6{let width=(linear.texture.width()>>mip)as usize;let height=(linear.texture.height()>>mip)as usize;let depths=level(&r,&linear.texture,mip,4).chunks_exact(4).map(|b|f32::from_le_bytes(b.try_into().unwrap())).collect::<Vec<_>>();let encoded=level(&r,&radii.texture,mip,2).chunks_exact(2).map(|b|u16::from_le_bytes(b.try_into().unwrap())).collect::<Vec<_>>();assert!(depths.iter().all(|z|z.is_finite()&&*z>=0.));if let Some((before,radius,w,h))=&previous{for y in 0..height{for x in 0..width{let mut nearest=1e30;let mut chosen=0;for i in 0..4{let q=(y*2+i/2).min(h-1)*w+(x*2+i%2).min(w-1);if before[q]>0.&&before[q]<nearest{nearest=before[q];chosen=radius[q];}}let expected=if nearest==1e30{0.}else{nearest};assert_eq!(depths[y*width+x],expected,"actual reduction mip{mip} at{x},{y}");assert_eq!(encoded[y*width+x],chosen);}}}else{let index=48*width+104;assert!((depths[index]-0.5).abs()<1e-5,"original section depth: {}",depths[index]);assert_eq!(depths[0],0.,"empty depth stays absent");assert!(encoded[index]>0);}previous=Some((depths,encoded,width,height));}
 assert_eq!(r.depth_pyramid_usage()[3],6);
}
#[test]
fn actual_pyramid_caches_resting_views_rebuilds_on_resize_and_releases_on_disable(){
 let(scene,_)=scene(false);let mut r=renderer(&scene);r.set_depth_pyramid(true);pixels(&r);let first=r.depth_pyramid_usage();pixels(&r);assert_eq!(r.depth_pyramid_usage(),first);r.resize(crate::viewport::Viewport{width:640,height:480});r.set_density(2.);let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:None,size:wgpu::Extent3d{width:640,height:480,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),&crate::cap_gpu_tests::matrix());assert_eq!(r.depth_pyramid_usage()[1],2);assert_eq!(r.depth_pyramid_usage()[3],12);r.set_depth_pyramid(false);assert_eq!(r.depth_pyramid_usage(),[0;4]);
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzzj-cache-physical-depth-pyramid-inputs-03.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 22.5.17
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Physical depth input recipes are cached for either count; six coarse image levels and reduction groups stay retained. Main browser integration follows in the next step.

**Verified checkpoint in Chrome.**

**Critical browser-scene review accepted.**

Actual single-sample original contact browser fixture after caching compatible physical-depth inputs. Original source archives, owners, geometry, contact shading and exact resize returns remain checked at both densities. Physical depth input recipes are cached for either count; six coarse image levels and reduction groups stay retained. Main browser integration follows in the next step.

![Actual single-sample original contact browser fixture after caching compatible physical-depth inputs. Original source archives, owners, geometry, contact shading and exact resize returns remain checked at both densities. Physical depth input recipes are cached for either count; six coarse image levels and reduction groups stay retained. Main browser integration follows in the next step.](../screenshots/journey/96zzzzzzt-keep-contact-ambient-inputs-on-originals-browser.png)

[Verification scope](release.md).

This reference image is shared with [22.4.3 · Keep contact ambient inputs on originals](96zzzzzzt-keep-contact-ambient-inputs-on-originals.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The pyramid deliberately reads representative sample zero into its existing one-sample hierarchy. Reduction selects each nearest-positive depth together with its encoded radius/pixel address; it does not resolve or average the pair. This coarse hierarchy is separate from the matching per-sample ambient corrections taught earlier.

Reuse the existing render descriptor and reduction recipe. The shared two-count cache holds compatible source layouts/pipelines, while existing image allocation depends only on the coarse dimensions. An unchanged count retains its binding and prepared result.

The supplied Native regression renders actual physical inputs at both counts and reads every pixel of all six depth/radius levels. It retains the nearest-positive-depth, empty-background and original section-depth assertions, plus resting-view cache, resize and release checks. Main browser rendering remains one-sample until the production switch is connected next.

Matching physical depth → cached source recipe → representative sample zero → six retained single-sample levels.



Why does a four-sample depth input not require four-sample pyramid images?

The coarse pyramid deliberately reads representative sample zero. Cache the compatible input layout/pipeline, then keep the existing one-sample reduction hierarchy. Reduction selects the nearest-positive depth and its encoded radius/pixel address together, without averaging.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Run both existing actual pyramid GPU regressions. Require all six nearest-positive-depth and radius-address reductions at both physical input counts, correct empty/section depths, unchanged resting counters, bounded resize allocation and release on disable.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 22.5.17
npm --prefix ../session_tests run course -- save 22.5.17
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The two-count cache selects a compatible physical-depth input recipe. It reuses the earlier render descriptor and single-sample reduction recipe. Representative sample zero feeds six retained coarse levels. Each nearest-positive depth carries its corresponding encoded radius/pixel address; reduction selects that pair and never averages it. Count changes release only the incompatible source binding and cached result. Actual Native full-field depth and encoded-address reductions cover both physical inputs; resting-view, resize and release assertions remain strict. All 1004 Native tests, WASM/optimized builds, Native frame, canonical Chrome contact/resize checks and stored evidence pass. Ten current canonical Native/browser pictures were individually inspected. Complete physical-sample viewer integration, adaptive quality, remaining view controls, runtime efficiency, Cargo independence and final whole-course feature-picture review remain unfinished. Maintained Session reload camera/base-grid fixes follow the completed course.

Run both existing actual pyramid GPU regressions. Require all six nearest-positive-depth and radius-address reductions at both physical input counts, correct empty/section depths, unchanged resting counters, bounded resize allocation and release on disable.

Actual single-sample original contact browser fixture after caching compatible physical-depth inputs. Original source archives, owners, geometry, contact shading and exact resize returns remain checked at both densities. Physical depth input recipes are cached for either count; six coarse image levels and reduction groups stay retained. Main browser integration follows in the next step.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 22.5.17
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
