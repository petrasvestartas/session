# 20.2 · Physical formats, shared section inputs and display defaults

**Goal:** Apply current device and URL display defaults to browser startup and viewport sizing. · **Checkpoints:** `96zzc-share-section-addresses-with-outlines-and-ambient` → `96zzf-start-with-current-browser-display-defaults` (4 steps) · **Source:** [engine/gpu/arena.rs](../../../../src/engine/gpu/arena.rs) · [engine/gpu/buffers.rs](../../../../src/engine/gpu/buffers.rs) · [engine/gpu/objects.rs](../../../../src/engine/gpu/objects.rs) · [engine/gpu/slots.rs](../../../../src/engine/gpu/slots.rs) · [engine/gpu/instanced.rs](../../../../src/engine/gpu/instanced.rs) · [engine/gpu/faces.rs](../../../../src/engine/gpu/faces.rs) · [engine/gpu/clip.rs](../../../../src/engine/gpu/clip.rs) · [engine/gpu/triangle_tiles.rs](../../../../src/engine/gpu/triangle_tiles.rs) · [app/scene_sync/tombs.rs](../../../../src/app/scene_sync/tombs.rs) (`session_viewer/src/lib.rs`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 20.2 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/physical.wgsl</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/ink_plane.wgsl</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/outline_caps.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">96zzc-share-section-addresses-with-outlines-and-ambient -&gt; 96zzf-start-with-current-browser-display-defaults</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/device.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Share section addresses w…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. R8Unorm Glyph Atlas (B&amp;W)</text>
  <rect x="616" y="54" width="168" height="84" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="628" y="64" width="34" height="36" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.5"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="645" y="88" text-anchor="middle" font-family="monospace" font-size="15" font-weight="700" fill="#111111">A</text>
  <rect x="670" y="64" width="34" height="36" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.5"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="687" y="88" text-anchor="middle" font-family="monospace" font-size="15" font-weight="700" fill="#111111">B</text>
  <rect x="712" y="64" width="34" height="36" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.5"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="729" y="88" text-anchor="middle" font-family="monospace" font-size="15" font-weight="700" fill="#111111">7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="124" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">1px Padded Shelves</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Physical formats, shared se…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Procedural matrix.wgsl</text>
  <rect x="914" y="54" width="120" height="84" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <rect x="914" y="54" width="40" height="28" fill="#fdba74"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="72" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">+1.0</text>
  <rect x="954" y="82" width="40" height="28" fill="#f97316"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="100" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">+2.4</text>
  <rect x="994" y="54" width="40" height="28" fill="#93c5fd"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1014" y="72" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">-0.8</text>
  <rect x="994" y="110" width="40" height="28" fill="#fdba74"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1014" y="128" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">+1.0</text>
  <line x1="954" y1="54" x2="954" y2="138" stroke="#111111" stroke-width="1"/>
  <line x1="994" y1="54" x2="994" y2="138" stroke="#111111" stroke-width="1"/>
  <line x1="914" y1="82" x2="1034" y2="82" stroke="#111111" stroke-width="1"/>
  <line x1="914" y1="110" x2="1034" y2="110" stroke="#111111" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Start with current browse…</text>
</svg>
</div>

## <a id="checkpoint-96zzc-share-section-addresses-with-outlines-and-ambient"></a>Step 1 of 4 · Share section addresses with outlines and ambient shading (`96zzc-share-section-addresses-with-outlines-and-ambient`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1/4: 96zzc-share-secti…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/physical.wgsl, src/ink_…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Follow the same reserved…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Follow the same reserved…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Follow the same reserved…</text>
</svg>
</div>

**What this code does:** Read section identity once across ink, outline and ambient passes.

### 1. `src/physical.wgsl`

Keep one lossless decoder alongside the shared encoder.

**Locate this existing block:**

```wgsl
fn physical_primitive(primitive:u32)->vec2<u32>{
    return vec2<u32>(primitive&0xffffu,primitive>>16u);
}
```

After that block, type:

```wgsl
--8<-- "journey/code/96zzc-share-section-addresses-with-outlines-and-ambient-resource-1.wgsl" skip=107
```

### 2. `src/ink_plane.wgsl`

Reuse the shared physical address decoder.

**Locate this existing block:**

```wgsl
    return halves.x|(halves.y<<16u);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    return physical_address(halves);
```

### 3. `src/outline_caps.wgsl`

Bind the existing integer surface address. Recover cap selection and plane depth while retaining occlusion by later depth writers.

Replace the entire file with:

```wgsl
@group(0) @binding(0) var primitives:texture_2d<u32>;
@group(0) @binding(1) var<uniform> radius:vec4<f32>;
@vertex fn vs_main(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32>{
 let p=array<vec2<f32>,3>(vec2<f32>(-1.,-1.),vec2<f32>(3.,-1.),vec2<f32>(-1.,3.));return vec4<f32>(p[i],0.,1.);
}
struct Coverage{@location(0) value:f32,@builtin(frag_depth) depth:f32}
@fragment fn fs_main(@builtin(position) p:vec4<f32>)->Coverage{
 let size=textureDimensions(primitives);
 if(any(vec2<u32>(p.xy)>=size)){discard;}
 let halves=textureLoad(primitives,vec2<i32>(p.xy),0).xy;
 let primitive=physical_address(halves);
 if(!clip_is_cap(primitive)||(radius.y>0.5&&(primitive&1u)==0u)){discard;}
 let plane=clip_cap_plane(primitive);
 if(plane>=clipping.count){discard;}
 let z=clip_plane_depth(plane,clip_ndc(p.xy,vec2<f32>(size),0.).xy);
 return Coverage(1.,z);
}
```

### 4. `src/outline_caps.rs`

Match the integer texture binding. Assemble the shared clipping helpers once. Reuse the existing clipping uniform layout. Add the shared plane group to the mask pipeline. Accept the current frame’s clipping planes. Bind the same clipping group used by the surface pass.

Replace the entire file with:

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
   primitive:Default::default(),depth_stencil:Some(wgpu::DepthStencilState{format:crate::depth::FORMAT,depth_write_enabled:Some(false),depth_compare:Some(wgpu::CompareFunction::LessEqual),stencil:Default::default(),bias:Default::default()}),multisample:Default::default(),multiview_mask:None,cache:None,
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

### 5. `src/outline_gpu.rs`

Supply current plane depth to the cap mask.

**Locate this existing block:**

```rust
        if sections.is_some(){self.caps.draw(&mut pass);}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        if sections.is_some(){self.caps.draw(&mut pass,clipping);}
```

### 6. `src/renderer.rs`

Use the full physical primitive image for this outline mask. Remove the unused section-resource borrow.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        }
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("measurement overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});self.measurement.draw(&mut pass);self.guides.draw(&mut pass);}
        if self.outlines_enabled.get()&&!self.meshes.is_empty(){
            let ctx=self.text_context();let caps=self.caps.borrow();
            let mut global=self.global_outline.borrow_mut();let global=global.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));
            global.encode_masks(&ctx,&mut encoder,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,true,Some(&caps.depth.view));
            let mut selected=self.outline.borrow_mut();
            if self.meshes.iter().any(GpuMesh::selected){let selected=selected.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));selected.encode_masks(&ctx,&mut encoder,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,false,Some(&caps.depth.view));}else{selected.take();}
            if let Some(alpha)=global.alpha_view(){self.outline_composite.borrow_mut().get_or_insert_with(||crate::outline_composite::Composite::new(&self.device,self.format)).draw(&self.device,&mut encoder,view,alpha,selected.as_ref().and_then(|s|s.alpha_view()));}
        }else{
            self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();
```

Replace it with:

```rust
        }
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("measurement overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});self.measurement.draw(&mut pass);self.guides.draw(&mut pass);}
        if self.outlines_enabled.get()&&!self.meshes.is_empty(){
            let ctx=self.text_context();
            let mut global=self.global_outline.borrow_mut();let global=global.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));
            global.encode_masks(&ctx,&mut encoder,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,true,Some(&self.primitive.view));
            let mut selected=self.outline.borrow_mut();
            if self.meshes.iter().any(GpuMesh::selected){let selected=selected.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));selected.encode_masks(&ctx,&mut encoder,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,false,Some(&self.primitive.view));}else{selected.take();}
            if let Some(alpha)=global.alpha_view(){self.outline_composite.borrow_mut().get_or_insert_with(||crate::outline_composite::Composite::new(&self.device,self.format)).draw(&self.device,&mut encoder,view,alpha,selected.as_ref().and_then(|s|s.alpha_view()));}
        }else{
            self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();
```

### 7. `src/ambient.wgsl`

Read integer section identity during ambient preparation. Share one bounded integer reader between ambient radius and normal lookup. Keep the established section radius without a second surface record. Decode and bound the section plane before reading its normal.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```wgsl
    return decode_normal(textureLoad(normals,coord,0).xy*2.0-1.0);
}
fn source_at(pixel:vec2<f32>)->vec2<i32>{return clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1);}
fn source_radius(pixel:vec2<f32>)->f32{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,0).x;
 let cap=textureLoad(source_sections,clamp(at,vec2<i32>(0),vec2<i32>(textureDimensions(source_sections))-1),0).y;
 return select(radius,ambient.params.y,radius==0.0&&cap>=0.5);
}
fn normal_of(pixel:vec2<f32>,p:vec3<f32>,sample:i32)->vec3<f32>{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,0).x;
 if(radius>0.0){let n=decode_normal(textureLoad(source_normals,at,0).xy*2.0-1.0);return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
 let cap=textureLoad(source_sections,clamp(at,vec2<i32>(0),vec2<i32>(textureDimensions(source_sections))-1),0).y;
 if(cap>=0.5){let plane=(u32(cap)-1u)&7u;let n=-clipping.planes[plane].xyz;return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
 let a=surface(pixel+vec2<f32>(1.,0.),sample);let b=surface(pixel-vec2<f32>(1.,0.),sample);let c=surface(pixel+vec2<f32>(0.,1.),sample);let d=surface(pixel-vec2<f32>(0.,1.),sample);
 let dx=select(p-b.xyz,a.xyz-p,a.w>0.&& (b.w==0.||distance(a.xyz,p)<distance(b.xyz,p)));let dy=select(p-d.xyz,c.xyz-p,c.w>0.&&(d.w==0.||distance(c.xyz,p)<distance(d.xyz,p)));let n=cross(dx,dy);let normal=n/max(length(n),1e-12);return select(-normal,normal,dot(normal,-ray_direction(pixel))>0.);
}
```

Replace it with:

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
 }
 let a=surface(pixel+vec2<f32>(1.,0.),sample);let b=surface(pixel-vec2<f32>(1.,0.),sample);let c=surface(pixel+vec2<f32>(0.,1.),sample);let d=surface(pixel-vec2<f32>(0.,1.),sample);
 let dx=select(p-b.xyz,a.xyz-p,a.w>0.&& (b.w==0.||distance(a.xyz,p)<distance(b.xyz,p)));let dy=select(p-d.xyz,c.xyz-p,c.w>0.&&(d.w==0.||distance(c.xyz,p)<distance(d.xyz,p)));let n=cross(dx,dy);let normal=n/max(length(n),1e-12);return select(-normal,normal,dot(normal,-ray_direction(pixel))>0.);
}
```

**Locate this block:**

```wgsl
@group(0) @binding(1) var<uniform> ambient: Ambient;
@group(0) @binding(2) var source_radii:texture_2d<f32>;
@group(0) @binding(3) var source_normals:texture_2d<f32>;
@group(0) @binding(4) var source_sections:texture_2d<f32>;
// Geometry access: ambient_geometry.wgsl, appended by ssao.rs
@group(1) @binding(0) var linear: texture_2d<f32>;
@group(1) @binding(1) var radii: texture_2d<u32>;
```

Replace it with:

```wgsl
@group(0) @binding(1) var<uniform> ambient: Ambient;
@group(0) @binding(2) var source_radii:texture_2d<f32>;
@group(0) @binding(3) var source_normals:texture_2d<f32>;
@group(0) @binding(4) var source_primitives:texture_2d<u32>;
// Geometry access: ambient_geometry.wgsl, appended by ssao.rs
@group(1) @binding(0) var linear: texture_2d<f32>;
@group(1) @binding(1) var radii: texture_2d<u32>;
```

### 8. `src/ambient_pipeline.rs`

Match the shared integer source binding.

**Locate this existing block:**

```rust
  let source=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient source images and precise camera"),entries:&[texture(0,T::Depth),uniform(1),texture(2,T::Float{filterable:false}),texture(3,T::Float{filterable:false}),texture(4,T::Float{filterable:false}),uniform(5)]});
```

Edit the existing block to match this code; keep unchanged lines:

```rust
  let source=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient source images and precise camera"),entries:&[texture(0,T::Depth),uniform(1),texture(2,T::Float{filterable:false}),texture(3,T::Float{filterable:false}),texture(4,T::Uint),uniform(5)]});
```

### 9. `src/renderer.rs`

Bind the same physical primitive image to ambient shading.

**Locate this existing block:**

```rust
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.depth,&self.view_group,&clip_group,&clip.buffer,&self.caps.borrow().depth.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground,[self.pyramid_revision,clip_revision]);}}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.depth,&self.view_group,&clip_group,&clip.buffer,&self.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground,[self.pyramid_revision,clip_revision]);}}
```

### 10. `src/caps_gpu.rs`

Remove the unused float section image owner. Stop allocating its empty placeholder. Resize only the crossing-count image still used by cap rendering. Keep only color and physical primitive attachments. Report only allocated cap resources.

Replace the entire file with:

```rust
use wgpu::util::DeviceExt;
use crate::{gpu_mesh::GpuMesh,solid_orientation::CLOSED};
use crate::cap_image::Image;
struct Counts{image:Image,group:wgpu::BindGroup}
pub struct Caps{pub enabled:bool,planes:u32,pick:Option<crate::cap_pick_gpu::Pick>,pub pick_error:Option<&'static str>,bounds:crate::cap_source_bounds::Bounds,targets:std::collections::HashMap<u32,u32>,pub solids:crate::solid_cache::Solids,pipes:Option<crate::cap_pipeline::Pipelines>,counts:Option<Counts>,frame:wgpu::Buffer,frame_group:Option<wgpu::BindGroup>,pub rows:Vec<[u32;2]>,pub draws:u64,pub passes:u64}
impl Caps{
 pub fn new(device:&wgpu::Device)->Self{let frame=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("section canvas frame"),contents:&[0;16],usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST});Self{enabled:true,planes:0,pick:None,pick_error:None,bounds:Default::default(),targets:Default::default(),solids:Default::default(),pipes:None,counts:None,frame,frame_group:None,rows:Vec::new(),draws:0,passes:0}}
 pub fn set_scene(&mut self,queue:&wgpu::Queue,scene:&crate::scene::Scene,meshes:&mut[GpuMesh],planes:&[crate::clipping_rows::Row]){
  self.planes=planes.len()as u32;self.rows.clear();self.targets.clear();for(mesh,object)in meshes.iter_mut().zip(scene.objects().iter().filter(|r|scene.visible(r.id))){assert_eq!(mesh.id,object.id);let flags=self.solids.flags(object);let token=if !planes.is_empty()&&flags&CLOSED!=0{self.rows.push([mesh.id.value(),u32::from(mesh.pickable())]);self.rows.len()as u32}else{0};mesh.set_caps(queue,flags,token);if token>0{if let Some(bounds)=self.bounds.placed(&object.mesh,&object.model){let mask=planes.iter().enumerate().fold(0,|mask,(i,row)|mask|if row.plane.crosses(&bounds,[0.;3]){1<<i}else{0});self.targets.insert(mesh.id.value(),mask);}}}
  self.solids.prune();self.bounds.prune();if self.rows.is_empty(){self.rows.shrink_to_fit();self.targets.shrink_to_fit();}
 }
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,size:[u32;2],active:bool){
  let size=if active&&!self.rows.is_empty(){size}else{[1,1]};
  if size==[1,1]{self.counts=None;return;}
  if self.pipes.is_none(){let pipes=crate::cap_pipeline::Pipelines::new(device,format);self.frame_group=Some(crate::gpu_layout::group(device,&pipes.frame,&self.frame));self.pipes=Some(pipes);}
  let pipes=self.pipes.as_ref().unwrap();if self.counts.as_ref().is_none_or(|c|c.image.size!=size){let image=Image::new(device,size,wgpu::TextureFormat::R16Float,2);let group=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("original section crossings"),layout:&pipes.counts,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&image.view)}]});self.counts=Some(Counts{image,group});}
  let bytes=[size[0]as f32,size[1]as f32,0.,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();queue.write_buffer(&self.frame,0,&bytes);
 }
 pub fn draw(&mut self,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,primitive:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],planes:u32){
  let(Some(pipes),Some(counts))=(&self.pipes,&self.counts)else{return};
  for plane in 0..planes{
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original closed-solid crossings"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&counts.image.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&pipes.count);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);for mesh in meshes.iter().filter(|m|m.closed()&&self.targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)){mesh.bind_model(&mut pass);pass.set_vertex_buffer(0,mesh.geometry.vertices.slice(..));pass.set_index_buffer(mesh.geometry.indices.slice(..),wgpu::IndexFormat::Uint32);pass.draw_indexed(0..mesh.geometry.index_count,0,plane..plane+1);self.draws+=1;}}
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section color and depth"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:color,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:primitive,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&pipes.color);pass.set_bind_group(0,self.frame_group.as_ref().unwrap(),&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,&counts.group,&[]);pass.draw(0..3,plane..plane+1);self.draws+=1;}
   self.passes+=2;
  }
 }
 pub fn encode_pick(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,image:&crate::pick_image::Image,window:crate::pick_window::Window,view:&wgpu::BindGroup,crop:&wgpu::Buffer,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],mode:crate::pick_sources::Mode){
  if !self.enabled||self.planes==0||self.rows.is_empty(){self.pick=None;self.pick_error=None;return;}
  let pick=self.pick.get_or_insert_with(||crate::cap_pick_gpu::Pick::new(device));if let Err(error)=pick.prepare(device,queue,window,crop,&self.rows,self.planes,mode){self.pick_error=Some(error);self.pick=None;return;}self.pick_error=None;pick.encode(encoder,image,window,view,clipping,meshes,&self.targets,self.planes);
 }
 pub fn pick_bytes(&self)->u64{self.pick.as_ref().map_or(0,crate::cap_pick_gpu::Pick::bytes)}
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.pick_bytes()+self.counts.as_ref().map_or(0,|c|c.image.bytes),self.rows.len()as u64,self.solids.checks,self.draws,self.passes]}
}
```

### 11. `src/cap_color.wgsl`

Move the physical address beside color after removing the float output. Keep plane and selection only in the physical primitive marker.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```wgsl
    edge = max(edge, section_outline(in.pos.xy));
    let color = mix(select(hatched, solid, clipping.fill == 1u), vec3<f32>(0.0), edge);
    let marker = CAP_PRIMITIVE + i * 2u + select(0u, 1u, selected);
    return CapOut(vec4<f32>(color, 1.0), vec2<f32>(z,f32(i+1u)+select(0.0,8.0,selected)), physical_primitive(marker), z, mask);
}
```

Replace it with:

```wgsl
    edge = max(edge, section_outline(in.pos.xy));
    let color = mix(select(hatched, solid, clipping.fill == 1u), vec3<f32>(0.0), edge);
    let marker = CAP_PRIMITIVE + i * 2u + select(0u, 1u, selected);
    return CapOut(vec4<f32>(color, 1.0), physical_primitive(marker), z, mask);
}
```

**Locate this block:**

```wgsl
// A section cap fragment: color, triangle id, the plane's depth and the samples it covers.
struct CapOut {
    @location(0) color: vec4<f32>, // hatch or light grey, with a black boundary
    @location(1) section_depth: vec2<f32>, // visible section depth and presence
    @location(2) primitive: vec2<u32>,
    @builtin(frag_depth) depth: f32, // depth of the plane here
    @builtin(sample_mask) mask: u32, // samples inside a solid and kept by the other planes
}
```

Replace it with:

```wgsl
// A section cap fragment: color, triangle id, the plane's depth and the samples it covers.
struct CapOut {
    @location(0) color: vec4<f32>, // hatch or light grey, with a black boundary
    @location(1) primitive: vec2<u32>,
    @builtin(frag_depth) depth: f32, // depth of the plane here
    @builtin(sample_mask) mask: u32, // samples inside a solid and kept by the other planes
}
```

### 12. `src/cap_pipeline.rs`

Match the reduced fragment outputs.

**Locate this existing block:**

```rust
  let color=pipeline(device,crate::clipping_gpu::source(include_str!("cap_color.wgsl"),1,0),&[&frame,&clip,&counts],"vs_cap","fs_cap",&[wgpu::ColorTargetState{format,blend:None,write_mask:wgpu::ColorWrites::ALL},wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg32Float,blend:None,write_mask:wgpu::ColorWrites::ALL},crate::primitive_image::target(true)],false);Self{count,color,counts,frame}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
  let color=pipeline(device,crate::clipping_gpu::source(include_str!("cap_color.wgsl"),1,0),&[&frame,&clip,&counts],"vs_cap","fs_cap",&[wgpu::ColorTargetState{format,blend:None,write_mask:wgpu::ColorWrites::ALL},crate::primitive_image::target(true)],false);Self{count,color,counts,frame}
```

### 13. `src/cap_pick_pipeline.rs`

Match the reduced fragment outputs.

**Locate this existing block:**

```rust
  let ids=crate::cap_pipeline::pipeline(device,crate::clipping_gpu::source(include_str!("cap_pick.wgsl"),1,0),&[&frame,&clip,&owners],"vs_cap","fs_cap_id",&[wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg32Uint,blend:None,write_mask:wgpu::ColorWrites::ALL},wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg32Float,blend:None,write_mask:wgpu::ColorWrites::ALL},crate::primitive_image::target(true)],false);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
  let ids=crate::cap_pipeline::pipeline(device,crate::clipping_gpu::source(include_str!("cap_pick.wgsl"),1,0),&[&frame,&clip,&owners],"vs_cap","fs_cap_id",&[wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg32Uint,blend:None,write_mask:wgpu::ColorWrites::ALL},crate::primitive_image::target(true)],false);
```

### 14. `src/cap_pick.wgsl`

Keep exact original owner and physical address as the two integer outputs. Remove the unused float pick record.

Replace the entire file with:

```wgsl
struct PickFrame{full:vec2<u32>,size:vec2<u32>,origin:vec2<u32>,mode:u32,pad:u32}
@group(0) @binding(0) var<uniform> frame:PickFrame;
@group(2) @binding(0) var<storage,read_write> pick_caps:array<atomic<u32>>;
@group(2) @binding(1) var<storage,read> owners:array<vec2<u32>>;
struct CapVertex {
    @builtin(position) pos: vec4<f32>, // clip position
    @location(0) @interpolate(flat) plane: u32, // clipping plane; for the masks 1 = selected caps only
}

@vertex
// A triangle over the whole target.
fn vs_cap(@builtin(vertex_index) vertex: u32, @builtin(instance_index) plane: u32) -> CapVertex {
    let xy = vec2<f32>(f32((vertex << 1u) & 2u), f32(vertex & 2u));
    var out: CapVertex;
    out.pos = vec4<f32>(xy * 2.0 - 1.0, 0.0, 1.0);
    out.plane = plane;
    return out;
}

struct CapId{@location(0) id:vec2<u32>,@location(1) primitive:vec2<u32>,@builtin(frag_depth) depth:f32}
@fragment
fn fs_cap_id(in: CapVertex) -> CapId {
    let i = in.plane;
    let ndc = clip_ndc(in.pos.xy + vec2<f32>(frame.origin), vec2<f32>(frame.full), 0.0).xy;
    let z = clip_plane_depth(i, ndc);
    let word = clip_pick_word(i, in.pos.xy, frame.size);
    let count = bitcast<i32>(atomicLoad(&pick_caps[word]));
    let owner = select(atomicLoad(&pick_caps[word + 3u]), atomicLoad(&pick_caps[word + 1u]), count == 1);
    var kept = true;

    for (var j = 0u; j < clipping.count; j++) {
        kept = kept && (j == i || dot(clipping.screen[j], vec4<f32>(ndc, z, 1.0)) >= 0.0);
    }

    if (count < 1 || owner == 0u || owner>arrayLength(&owners) || !kept || !(z >= 0.0 && z <= 1.0)) {
        discard;
    }

    let marker=CAP_PRIMITIVE+i*2u;let source=owners[owner-1u];return CapId(vec2<u32>(select(0u,source.x,source.y!=0u),0u),physical_primitive(marker),z);
}
```

### 15. `src/cap_pick_gpu.rs`

Retain the existing crop’s primitive attachment.

**Locate this existing block:**

```rust
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section integer owners"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:&image.section,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&self.pipes.ids);pass.set_bind_group(0,&self.frame_group,&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,self.owner_group.as_ref().unwrap(),&[]);pass.draw(0..3,0..planes);let _=window;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section integer owners"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&self.pipes.ids);pass.set_bind_group(0,&self.frame_group,&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,self.owner_group.as_ref().unwrap(),&[]);pass.draw(0..3,0..planes);let _=window;
```

### 16. `src/pick_image.rs`

Remove the unused pick-image member. Stop allocating a duplicate eight-byte surface record per cropped pixel. Construct the smaller cropped image.

Replace the entire file with:

```rust
use crate::pick_window::Window;

pub(crate) struct Image {
    pub texture: wgpu::Texture,
    pub colour: wgpu::TextureView,
    pub depth: wgpu::TextureView,
    pub primitive:crate::primitive_image::Image,
    pub readback: wgpu::Buffer,
    pub window: Window,
}

impl Image {
    pub fn new(device: &wgpu::Device, window: Window) -> Self {
        let size = wgpu::Extent3d { width: window.draw[2], height: window.draw[3], depth_or_array_layers: 1 };
        let texture = device.create_texture(&wgpu::TextureDescriptor { label: Some("bounded pick image"), size,
            mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rg32Uint, usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC, view_formats: &[] });
        let colour = texture.create_view(&Default::default());
        let depth = device.create_texture(&wgpu::TextureDescriptor { label: Some("pick depth"), size,
            mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
            format: crate::depth::FORMAT, usage: wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[] }).create_view(&Default::default());
        // GPU image rows need 256-byte copy alignment; the decoder skips that padding and reads only the logical columns.
        let readback = device.create_buffer(&wgpu::BufferDescriptor { label: Some("bounded pick readback"), size: window.bytes(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
        let primitive=crate::primitive_image::Image::new(device,[size.width,size.height]);
        Self { texture, colour, depth, primitive, readback, window }
    }

}
```

### 17. `src/pick_encode.rs`

Remove the pass that only cleared the unused attachment.

**Locate this existing block:**

```rust
        {let _clear=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("clear bounded section depth"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.section,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});}
        {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        {
```

### 18. `src/pick_gpu.rs`

Account for eight owner bytes, four depth bytes and four primitive bytes.

**Locate this existing block:**

```rust
            image.readback.size() + u64::from(image.window.draw[2]) * u64::from(image.window.draw[3]) * 24) + 160+u64::from(self.legacy_clipping.is_some())*448]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            image.readback.size() + u64::from(image.window.draw[2]) * u64::from(image.window.draw[3]) * 16) + 160+u64::from(self.legacy_clipping.is_some())*448]
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 19. `src/pick_gpu_tests.rs`

Tighten the existing bounded-memory assertion to the actual resources.

**Locate this existing block:**

```text
31 * 31 * 24
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzc-share-section-addresses-with-outlines-and-ambient-resource-22.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 20. `src/cap_gpu_tests.rs`

Require exactly the remaining uniform and crossing-count allocation.

**Locate this existing block:**

```text
16+320*240*10
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzc-share-section-addresses-with-outlines-and-ambient-resource-23.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 21. `src/cap_gpu_tests.rs`

Require release down to the retained sixteen-byte frame uniform.

**Locate this existing block:**

```text
assert_eq!(r.cap_usage()[0],24)
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzc-share-section-addresses-with-outlines-and-ambient-resource-24.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 22. `src/global_outline_tests.rs`

Require the reduced inactive cap allocation in the existing mask regression.

**Locate this existing block:**

```text
assert_eq!(r.cap_usage()[0],24)
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzc-share-section-addresses-with-outlines-and-ambient-resource-25.rs"
```

## <a id="checkpoint-96zzd-pack-surface-position-normal-and-color"></a>Step 2 of 4 · Use compact vertices in every surface pass (`96zzd-pack-surface-position-normal-and-color`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2/4: 96zzd-pack-surfac…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/packed_vertex.rs, src/l…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Follow the same five word…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Follow the same five word…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Follow the same five word…</text>
</svg>
</div>

**What this code does:** Use one compact surface vertex buffer throughout rendering and picking.

### 1. `src/packed_vertex.rs`

Write the compact derived GPU representation while retaining source ownership.

Create the file and type:

```rust
--8<-- "journey/code/96zzd-pack-surface-position-normal-and-color-1.rs"
```

### 2. `src/lib.rs`

Expose the pure encoder before connecting its upload.

**Locate this existing block:**

```rust
pub mod gpu_geometry;
```

After that block, type:

```rust
pub mod packed_vertex;
#[cfg(test)]
mod packed_vertex_tests;
```

### 3. `src/packed_vertex.wgsl`

Move the oct32 decoder already taught for clouds into the shared shader source.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

Create the destination by moving the existing code:

```text
--8<-- "journey/code/96zzd-pack-surface-position-normal-and-color-3.wgsl"
```

### 4. `src/cloud_points.wgsl`

Use the shared decoder without a second implementation.

**Locate this existing block:**

```wgsl
fn oct32_decode(p: u32) -> vec3<f32> {
    if (p == 0x80008000u) {
        return vec3<f32>(0.0);
    }

    let e = unpack2x16snorm(p);
    var n = vec3<f32>(e, 1.0 - abs(e.x) - abs(e.y));

    if (n.z < 0.0) {
        let s = vec2<f32>(select(1.0, -1.0, n.x < 0.0), select(1.0, -1.0, n.y < 0.0));
        n = vec3<f32>((1.0 - abs(n.y)) * s.x, (1.0 - abs(n.x)) * s.y, n.z);
    }

    return normalize(n);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl

```

### 5. `src/clipping_gpu.rs`

Make the same decoder available to current cloud and upcoming mesh shaders.

**Locate this existing block:**

```rust
pub fn source(base:&str,group:u32,binding:u32)->String{format!("@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",concat!(include_str!("physical.wgsl"),"\n",include_str!("clip.wgsl")))}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn source(base:&str,group:u32,binding:u32)->String{format!("@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",concat!(include_str!("packed_vertex.wgsl"),"\n",include_str!("physical.wgsl"),"\n",include_str!("clip.wgsl")))}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/packed_vertex_tests.rs`

Supply independent position-bit, memory-size, quantization, sentinel and directional-error checks.

Copy this supplied file:

```text
--8<-- "journey/code/96zzd-pack-surface-position-normal-and-color-6.rs"
```

### 7. `src/packed_vertex.wgsl`

Declare one scalar-aligned twenty-byte shader layout and shared position reader.

**Locate this existing block:**

```wgsl
fn oct32_decode(p: u32) -> vec3<f32> {
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct PackedVertex{x:f32,y:f32,z:f32,normal:u32,color:u32}
fn packed_position(vertex:PackedVertex)->vec3<f32>{return vec3<f32>(vertex.x,vertex.y,vertex.z);}
fn oct32_decode(p: u32) -> vec3<f32> {
```

### 8. `src/gpu_geometry.rs`

Remove the separate normal buffer owner. Keep stable vertex and index bindings; no normal storage binding remains. Require at least one complete compact vertex or one index in the storage binding. Upload position, encoded normal and color in one buffer. Remove the duplicate upload. Bind the same compact storage used by every surface pass. Retain one source and one compact vertex buffer. Report actual vertex and index allocation. Indexed passes read their normal word from the same vertex buffer.

Replace the entire file with:

```rust
use crate::mesh::Mesh;
use std::rc::Rc;
use wgpu::util::DeviceExt;

pub struct GpuGeometry {
    pub source: Rc<Mesh>,
    pub(crate) vertices: wgpu::Buffer,
    pub(crate) indices: wgpu::Buffer,
    pub(crate) index_count: u32,
    pull_group: wgpu::BindGroup,
}

impl GpuGeometry {
    pub fn storage_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let entries = [0, 2].map(|binding| wgpu::BindGroupLayoutEntry {
            binding, visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false, min_binding_size: std::num::NonZeroU64::new(if binding==0 {20}else{4}),
            }, count: None,
        });
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("shared geometry storage"), entries: &entries,
        })
    }

    pub fn upload(device: &wgpu::Device, source: Rc<Mesh>) -> Self {
        fn buffer(device: &wgpu::Device, label: &str, mut bytes: Vec<u8>, usage: wgpu::BufferUsages) -> wgpu::Buffer {
            if bytes.is_empty() { bytes.resize(4, 0); }
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label), contents: &bytes, usage: usage | wgpu::BufferUsages::STORAGE,
            })
        }
        let mut bytes = crate::packed_vertex::bytes(&source);
        if bytes.is_empty() { bytes.resize(20, 0); }
        let vertices = buffer(device, "shared mesh vertices", bytes, wgpu::BufferUsages::VERTEX);
        let bytes = source.indices().iter().flat_map(|v| v.to_ne_bytes()).collect();
        let indices = buffer(device, "shared mesh indices", bytes, wgpu::BufferUsages::INDEX);
        let entries = [(0, &vertices), (2, &indices)].map(|(binding, buffer)| wgpu::BindGroupEntry {
            binding, resource: buffer.as_entire_binding(),
        });
        let pull_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("shared vertex reads"), layout: &Self::storage_layout(device), entries: &entries,
        });
        let index_count = source.indices().len() as u32;
        Self { source, vertices, indices, index_count, pull_group }
    }

    pub fn allocated_bytes(&self) -> u64 { self.vertices.size() + self.indices.size() }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
        if self.index_count == 0 || count == 0 { return; }
        pass.set_bind_group(3, &self.pull_group, &[]);
        pass.draw(0..self.index_count, 0..count);
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {self.draw_instances(pass,1);}

    pub fn draw_instances(&self, pass: &mut wgpu::RenderPass<'_>,count:u32) {
        if self.index_count==0||count==0{return;}
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.index_count, 0, 0..count);
    }
}
```

### 9. `src/triangle.wgsl`

Read packed words without numeric conversion. Remove the second normal binding. Decode compact vertices through the already shared oct32 helper for color and pulled integer picks.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```wgsl
    return FragmentOutput(shade(input,front),physical_primitive(input.primitive));
}

@group(3) @binding(0) var<storage, read> mesh_values: array<f32>;
@group(3) @binding(1) var<storage, read> mesh_normals: array<f32>;
@group(3) @binding(2) var<storage, read> mesh_indices: array<u32>;
struct MeshInput { position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, }
fn mesh_vertex(occurrence: u32) -> MeshInput {
    let index = mesh_indices[occurrence];
    let v = index * 6u;
    let n = index * 3u;
    return MeshInput(
        vec3<f32>(mesh_values[v], mesh_values[v+1u], mesh_values[v+2u]),
        vec3<f32>(mesh_values[v+3u], mesh_values[v+4u], mesh_values[v+5u]),
        vec3<f32>(mesh_normals[n], mesh_normals[n+1u], mesh_normals[n+2u]),
    );
}

@vertex
```

Replace it with:

```wgsl
    return FragmentOutput(shade(input,front),physical_primitive(input.primitive));
}

@group(3) @binding(0) var<storage, read> mesh_values: array<PackedVertex>;
@group(3) @binding(2) var<storage, read> mesh_indices: array<u32>;
struct MeshInput { position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, }
fn mesh_vertex(occurrence:u32)->MeshInput{
 let vertex=mesh_values[mesh_indices[occurrence]];
 return MeshInput(packed_position(vertex),unpack4x8unorm(vertex.color).rgb,oct32_decode(vertex.normal));
}

@vertex
```

### 10. `src/finite_project.wgsl`

Project positions from the actual compact upload. Use the shared vertex layout rather than a separate stride calculation. Preserve position bits when fitting physical triangle planes.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```wgsl
    depth: vec3<f32>, // screen gradient xy and depth at the viewport centre
};
fn physical_clip_corner(index:u32)->vec4<f32>{
 let base=physical_indices[index]*6u;
 let point=vec3<f32>(physical_vertices[base],physical_vertices[base+1u],physical_vertices[base+2u]);
 let clip=view.transform*object.model*vec4<f32>(point,1.0);
 return vec4<f32>(clip.xy,clip.w-clip.z,clip.w);
}
```

Replace it with:

```wgsl
    depth: vec3<f32>, // screen gradient xy and depth at the viewport centre
};
fn physical_clip_corner(index:u32)->vec4<f32>{
 let point=packed_position(physical_vertices[physical_indices[index]]);
 let clip=view.transform*object.model*vec4<f32>(point,1.0);
 return vec4<f32>(clip.xy,clip.w-clip.z,clip.w);
}
```

**Locate this block:**

```wgsl
struct ObjectSettings {model:mat4x4<f32>,selected:u32,id:u32,pad:vec2<u32>,normals:mat4x4<f32>,colour:vec4<f32>}
@group(0) @binding(0) var<uniform> view:View;
@group(1) @binding(0) var<uniform> line:TileLine;
@group(2) @binding(0) var<storage,read> physical_vertices:array<f32>;
@group(2) @binding(1) var<storage,read> physical_indices:array<u32>;
@group(2) @binding(2) var<uniform> object:ObjectSettings;
@group(2) @binding(3) var<storage,read_write> projected:array<ProjectedTriangle>;
```

Replace it with:

```wgsl
struct ObjectSettings {model:mat4x4<f32>,selected:u32,id:u32,pad:vec2<u32>,normals:mat4x4<f32>,colour:vec4<f32>}
@group(0) @binding(0) var<uniform> view:View;
@group(1) @binding(0) var<uniform> line:TileLine;
@group(2) @binding(0) var<storage,read> physical_vertices:array<PackedVertex>;
@group(2) @binding(1) var<storage,read> physical_indices:array<u32>;
@group(2) @binding(2) var<uniform> object:ObjectSettings;
@group(2) @binding(3) var<storage,read_write> projected:array<ProjectedTriangle>;
```

### 11. `src/finite_project_source.rs`

Assemble physical projection with the same packed layout and position reader.

**Locate this existing block:**

```rust
pub fn source() -> String { expand(include_str!("finite_project.wgsl")) }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn source() -> String { format!("{}\n{}",include_str!("packed_vertex.wgsl"),expand(include_str!("finite_project.wgsl"))) }
```

### 12. `src/face_pick.wgsl`

Remove the obsolete source-face layout; use the shared twenty-byte declaration. Read original face positions and colors through the same compact vertex layout.

Replace the entire file with:

```wgsl
struct FaceSettings { base: u32, _a: u32, _b: u32, _c: u32 }
@group(2) @binding(0) var<storage, read> positions: array<PackedVertex>;
@group(2) @binding(1) var<storage, read> indices: array<u32>;
@group(2) @binding(2) var<storage, read> faces: array<u32>;
@group(2) @binding(3) var<uniform> source: FaceSettings;
@group(3) @binding(0) var<uniform> crop: Crop;
struct FaceOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) face: u32,
    @location(1) world:vec3<f32>,@location(2) @interpolate(flat) flags:u32,
}
@vertex
fn face_vertex(@builtin(vertex_index) index: u32) -> FaceOutput {
    let vertex = positions[indices[index]];
    let placed = geometry(packed_position(vertex), unpack4x8unorm(vertex.color).rgb, vec3<f32>(0.0),object_settings);
    return FaceOutput(crop.transform * placed.position, faces[index / 3u] + source.base,placed.world,placed.clip_flags);
}
@fragment
fn face_fragment(input: FaceOutput) -> @location(0) vec2<u32> {
    if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}
    return vec2<u32>(object_settings.selection.y, (0x20000000u | input.face) + 1u);
}
```

### 13. `src/cap_pipeline.rs`

Count closed-solid crossings from the compact vertex positions.

**Locate this existing block:**

```rust
 let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("original closed-solid sections"),source:wgpu::ShaderSource::Wgsl(source.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("original closed-solid sections"),bind_group_layouts:&groups.iter().map(|g|Some(*g)).collect::<Vec<_>>(),immediate_size:0});let attributes=wgpu::vertex_attr_array![0=>Float32x3];let buffers=if count{vec![wgpu::VertexBufferLayout{array_stride:24,step_mode:wgpu::VertexStepMode::Vertex,attributes:&attributes}]}else{Vec::new()};
```

Edit the existing block to match this code; keep unchanged lines:

```rust
 let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("original closed-solid sections"),source:wgpu::ShaderSource::Wgsl(source.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("original closed-solid sections"),bind_group_layouts:&groups.iter().map(|g|Some(*g)).collect::<Vec<_>>(),immediate_size:0});let attributes=wgpu::vertex_attr_array![0=>Float32x3];let buffers=if count{vec![wgpu::VertexBufferLayout{array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&attributes}]}else{Vec::new()};
```

### 14. `src/outline_gpu.rs`

Use the same position stride for outline masks.

**Locate this existing block:**

```rust
        array_stride: 24,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        array_stride: 20,
```

### 15. `src/ambient_source_pipeline.rs`

Read position and the normal word from one twenty-byte vertex binding.

**Locate this existing block:**

```rust
  let draw=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("ambient original normals and radii"),layout:Some(&layout),vertex:wgpu::VertexState{module:&module,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[wgpu::VertexBufferLayout{array_stride:24,step_mode:wgpu::VertexStepMode::Vertex,attributes:&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3]},wgpu::VertexBufferLayout{array_stride:12,step_mode:wgpu::VertexStepMode::Vertex,attributes:&wgpu::vertex_attr_array![2=>Float32x3]}]},fragment:Some(wgpu::FragmentState{module:&module,entry_point:Some("fs_main"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R32Float,blend:None,write_mask:wgpu::ColorWrites::RED}),Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::state_for_ambient()),multisample:Default::default(),multiview_mask:None,cache:None});Self{draw,radius}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
  let draw=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("ambient original normals and radii"),layout:Some(&layout),vertex:wgpu::VertexState{module:&module,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[wgpu::VertexBufferLayout{array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&wgpu::vertex_attr_array![0=>Float32x3,2=>Uint32]}]},fragment:Some(wgpu::FragmentState{module:&module,entry_point:Some("fs_main"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R32Float,blend:None,write_mask:wgpu::ColorWrites::RED}),Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::state_for_ambient()),multisample:Default::default(),multiview_mask:None,cache:None});Self{draw,radius}
```

### 16. `src/ambient_sources.wgsl`

Receive the integer normal word in the ambient property pass. Use the same decoder and preserve the missing-normal derivative fallback.

Replace the entire file with:

```wgsl
struct View{transform:mat4x4<f32>,display:vec4<f32>}
struct Object{model:mat4x4<f32>,selection:vec4<u32>,normal_matrix:mat4x4<f32>,face_colour:vec4<f32>}
@group(0) @binding(0) var<uniform> view:View;
@group(1) @binding(0) var<uniform> object:Object;
@group(3) @binding(0) var<uniform> ambient_radius:vec4<f32>;
struct Vertex{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(1) normal:vec3<f32>,@location(2) @interpolate(flat) flags:u32}
@vertex fn vs_main(@location(0) position:vec3<f32>,@location(2) normal:u32)->Vertex{let world=object.model*vec4<f32>(position,1.);return Vertex(view.transform*world,world.xyz,(object.normal_matrix*vec4<f32>(oct32_decode(normal),0.)).xyz,object.selection.z);}
fn encode_normal(n:vec3<f32>)->vec2<f32>{let p=n.xy/(abs(n.x)+abs(n.y)+abs(n.z));let folded=(1.-abs(p.yx))*select(vec2<f32>(-1.),vec2<f32>(1.),p>=vec2<f32>(0.));return select(folded,p,n.z>=0.);}
struct Source{@location(0) radius:f32,@location(1) normal:vec2<f32>}
@fragment fn fs_main(in:Vertex,@builtin(front_facing) front:bool)->Source{
 let face=cross(dpdx(in.world),dpdy(in.world));if((in.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(in.flags,in.world)){discard;}
 let normal=normalize(select(select(face,-face,front),in.normal,dot(in.normal,in.normal)>0.));
 return Source(ambient_radius.x,encode_normal(normal)*0.5+0.5);
}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 17. `src/global_outline_tests.rs`

Supply actual GPU allocation, source retention, reuse and release checks alongside the existing color/primitive/pick fixtures.

**Locate this existing block:**

```text
().unwrap().unwrap()[0],owner.value());
  let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let words=primitive_pixels(&gpu,&texture);
  let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];
  assert_eq!(words[(y*texture.width()+x)as usize],0xfffffff0);
  assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);
 }
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzd-pack-surface-position-normal-and-color-upload-31.rs" skip=400
```

### 18. `src/geometry_cache.rs`

Count the compact vertex and index bytes actually uploaded; no separate float normal upload remains.

**Locate this existing block:**

```rust
        self.uploaded_bytes += (source.vertices().len() * 24 + source.normals().len() * 12 + source.indices().len() * 4) as u64;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.uploaded_bytes += (source.vertices().len() * 20 + source.indices().len() * 4) as u64;
```

## <a id="checkpoint-96zze-retain-rgba-in-prepared-displays"></a>Step 3 of 4 · Retain RGBA in prepared displays (`96zze-retain-rgba-in-prepared-displays`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3/4: 96zze-retain-rgba…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/mesh.rs, src/packed_ver…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Follow ordinary or derive…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Follow ordinary or derive…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Follow ordinary or derive…</text>
</svg>
</div>

**What this code does:** Preserve display RGBA through preparation and the compact upload.

### 1. `src/mesh.rs`

Retain alpha beside the existing derived vertex columns. Keep ordinary RGB-created display vertices opaque by default. Use one validated RenderMesh conversion for ordinary kernel displays and derived feature displays. Keep length and finiteness invariants with the display owner. Account for the retained CPU alpha column.

Replace the entire file with:

```rust
#[derive(Clone)]
pub struct Mesh {
    vertices: Vec<[f32; 6]>,
    normals: Vec<[f32; 3]>,
    alphas: Vec<f32>,
    indices: Vec<u32>,
    pub parts: crate::mesh_parts::Parts,
}

impl Mesh {
    pub fn new(vertices: Vec<[f32; 6]>, indices: Vec<u32>) -> Result<Self, &'static str> {
        if indices.len() % 3 != 0 || indices.len() > u32::MAX as usize {
            return Err("Indices must describe complete triangles within the draw limit");
        }
        if indices.iter().any(|&index| index as usize >= vertices.len()) {
            return Err("A triangle refers to a missing vertex");
        }
        if vertices.iter().flatten().any(|value| !value.is_finite()) {
            return Err("Vertex values must be finite");
        }
        let normals = vec![[0.0; 3]; vertices.len()];
        let alphas = vec![1.; vertices.len()];
        Ok(Self { vertices, normals, alphas, indices, parts: Default::default() })
    }

    pub fn from_kernel(mesh: &session_rust::Mesh) -> Result<Self, &'static str> {
        let render = mesh.to_render();
        let parts = crate::mesh_parts::Parts::new(mesh, render.indices.len() / 3)?;
        let mut display = Self::from_render(render)?;
        display.parts = parts; Ok(display)
    }

    pub fn from_render(render:session_rust::render_mesh::RenderMesh)->Result<Self,&'static str>{
        let vertices = render.vertices.iter().map(|vertex| {
            let [x, y, z] = vertex.position;
            let [r, g, b, _] = vertex.color;
            [x, y, z, r, g, b]
        }).collect();
        let normals = render.vertices.iter().map(|vertex| vertex.normal).collect();
        let alphas = render.vertices.iter().map(|vertex| vertex.color[3]).collect();
        Self::new(vertices, render.indices)?.with_normals(normals)?.with_alphas(alphas)
    }

    pub fn with_normals(mut self, normals: Vec<[f32; 3]>) -> Result<Self, &'static str> {
        if normals.len() != self.vertices.len() { return Err("Each display vertex needs one normal"); }
        self.normals = normals.into_iter().map(crate::normals::unit).collect::<Result<_, _>>()?; Ok(self)
    }

    pub fn with_alphas(mut self, alphas:Vec<f32>)->Result<Self,&'static str>{
        if alphas.len()!=self.vertices.len()||alphas.iter().any(|v|!v.is_finite()){
            return Err("Each display vertex needs finite alpha");
        }
        self.alphas=alphas;Ok(self)
    }
    pub fn alphas(&self)->&[f32]{&self.alphas}
    pub fn normals(&self) -> &[[f32; 3]] { &self.normals }

    pub fn payload_bytes(&self) -> usize {
        self.vertices.capacity() * std::mem::size_of::<[f32; 6]>()
            + self.indices.capacity() * std::mem::size_of::<u32>()
            + self.normals.capacity() * std::mem::size_of::<[f32; 3]>()
            + self.alphas.capacity() * std::mem::size_of::<f32>() + self.parts.bytes()
    }

    pub fn vertices(&self) -> &[[f32; 6]] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_kernel_box_becomes_eight_vertices_and_twelve_triangles() {
        let source = session_rust::Mesh::create_box(0.8, 0.8, 0.8);
        let mesh = Mesh::from_kernel(&source).unwrap();
        assert_eq!(mesh.vertices().len(), 8);
        assert_eq!(mesh.indices().len(), 36);
    }

    #[test]
    fn invalid_connections_and_nonfinite_vertices_are_rejected() {
        assert!(Mesh::new(vec![[0.0; 6]; 3], vec![0, 1, 3]).is_err());
        assert!(Mesh::new(vec![[0.0; 6]; 3], vec![0, 1]).is_err());
        assert!(Mesh::new(vec![[f32::NAN; 6]; 3], vec![0, 1, 2]).is_err());
        assert!(Mesh::new(vec![[0.0; 6]; 3], vec![0, 1, 2]).is_ok());
    }
}
```

### 2. `src/packed_vertex.rs`

Encode the actual display alpha instead of an opaque constant. Use the same clamped RGBA quantization for all four channels. Pack the validated columns together without another encoder.

Replace the entire file with:

```rust
pub const NO_NORMAL:u32=0x8000_8000;
pub fn oct32(normal:[f32;3])->u32{
 let length=normal.iter().map(|v|v.abs()).sum::<f32>();
 if length==0.||length.is_nan(){return NO_NORMAL;}
 let (x,y)=(normal[0]/length,normal[1]/length);
 let sign=|v:f32|if v<0.{-1.}else{1.};
 let (x,y)=if normal[2]<0.{((1.-y.abs())*sign(x),(1.-x.abs())*sign(y))}else{(x,y)};
 let half=|v:f32|u32::from((v*32767.).round().clamp(-32766.,32767.)as i16 as u16);
 half(x)|(half(y)<<16)
}
pub fn pack(vertex:[f32;6],normal:[f32;3],alpha:f32)->[u32;5]{
 let color=[vertex[3],vertex[4],vertex[5],alpha].map(|v|(v.clamp(0.,1.)*255.).round()as u8);
 [vertex[0].to_bits(),vertex[1].to_bits(),vertex[2].to_bits(),oct32(normal),u32::from_le_bytes(color)]
}
pub fn bytes(mesh:&crate::mesh::Mesh)->Vec<u8>{
 mesh.vertices().iter().zip(mesh.normals()).zip(mesh.alphas()).flat_map(|((v,n),a)|pack(*v,*n,*a)).flat_map(u32::to_ne_bytes).collect()
}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/packed_vertex_tests.rs`

Keep the existing opaque quantization contract explicit.

**Locate this existing block:**

```text
pack(v,[0.;3])
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zze-retain-rgba-in-prepared-displays-9.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/display_rgba_tests.rs`

Supply ordinary kernel and derived contact conversion, exact RGBA, unchanged source and column-validation fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/96zze-retain-rgba-in-prepared-displays-10.rs"
```

### 5. `src/lib.rs`

Connect the supplied alpha-retention fixtures to the authoring harness.

**Locate this existing block:**

```rust
pub mod packed_vertex;
```

After that block, type:

```rust
#[cfg(test)]
mod display_rgba_tests;
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/memory.rs`

Include the three retained alpha values in the exact shared-snapshot memory fixture.

**Locate this existing block:**

```text
baseline[4] + 16 * 24 + 16 * 4 + 3 * 12
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zze-retain-rgba-in-prepared-displays-memory-alpha.rs" skip=39
```

## <a id="checkpoint-96zzf-start-with-current-browser-display-defaults"></a>Step 4 of 4 · Start with current browser display defaults (`96zzf-start-with-current-browser-display-defaults`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4/4: 96zzf-start-with-…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/browser_defaults.rs, sr…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Follow pointer capability…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Follow pointer capability…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Follow pointer capability…</text>
</svg>
</div>

**What this code does:** Apply current device and URL display defaults to browser startup and viewport sizing.

### 1. `src/browser_defaults.rs`

Reuse existing opacity and grid state; detect pointer capability and read URL defaults once.

Create the file and type:

```rust
--8<-- "journey/code/96zzf-start-with-current-browser-display-defaults-1.rs"
```

### 2. `src/lib.rs`

Register the startup settings without a second renderer settings owner.

**Locate this existing block:**

```rust
pub mod packed_vertex;
```

After that block, type:

```rust
pub mod browser_defaults;
```

### 3. `Cargo.toml`

Enable the browser media-query return type in the existing pinned web-sys dependency.

**Locate this existing block:**

```toml
web-sys = { version = "=0.3.105", features = ["console", "Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "History", "UrlSearchParams", "Navigator", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "ReadableWritablePair", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "RequestMode", "RequestCache", "EventSource", "MessageEvent", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
```

Edit the existing block to match this code; keep unchanged lines:

```toml
web-sys = { version = "=0.3.105", features = ["console", "Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "History", "UrlSearchParams", "Navigator", "MediaQueryList", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "ReadableWritablePair", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "RequestMode", "RequestCache", "EventSource", "MessageEvent", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
```

### 4. `src/browser.rs`

Initialize a new browser document once before creating its renderer.

**Locate this existing block:**

```rust
pub async fn run() -> Result<(), JsValue> { run_with(crate::drawing_document::new()).await }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub async fn run() -> Result<(), JsValue> {
    let shared=crate::drawing_document::new();
    crate::browser_defaults::initialize(&mut shared.borrow_mut());
    run_with(shared).await
}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/browser_defaults_tests.rs`

Supply pointer-policy, finite override, clamping and existing-owner fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/96zzf-start-with-current-browser-display-defaults-5.rs"
```

### 6. `src/lib.rs`

Register the supplied startup policy fixture.

**Locate this existing block:**

```rust
pub mod browser_defaults;
```

After that block, type:

```rust
#[cfg(test)]
mod browser_defaults_tests;
```

### 7. `src/browser_defaults.rs`

Use one query reader for startup defaults and viewport scale. Cap requested pixels using the current finite f32 override policy.

**Locate this existing block:**

```rust
pub struct Settings { pub opacity:f32, pub grid:bool }

impl Settings {
    pub fn new(coarse:bool,opacity:Option<&str>,nogrid:Option<&str>)->Self {
        let default=if coarse {1.} else {0.95};
        let opacity=opacity.and_then(|s|s.parse::<f32>().ok())
            .filter(|v|v.is_finite()).unwrap_or(default).clamp(0.,1.);
        Self {opacity,grid:nogrid.is_none()}
    }

    pub fn apply(&self,editor:&mut crate::editor::Editor) {
        editor.opacity=self.opacity;
        editor.camera.grid=self.grid.then(Default::default);
    }
}

#[cfg(target_arch="wasm32")]
pub fn initialize(editor:&mut crate::editor::Editor) {
    let Some(window)=web_sys::window() else {return};
    let coarse=window.match_media("(pointer: coarse)").ok().flatten()
        .is_some_and(|media|media.matches());
    let search=window.location().search().unwrap_or_default();
    let query=web_sys::UrlSearchParams::new_with_str(&search).ok();
    let opacity=query.as_ref().and_then(|q|q.get("opacity"));
    let nogrid=query.as_ref().and_then(|q|q.get("nogrid"));
    Settings::new(coarse,opacity.as_deref(),nogrid.as_deref()).apply(editor);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Settings { pub opacity:f32, pub grid:bool }

impl Settings {
    pub fn new(coarse:bool,opacity:Option<&str>,nogrid:Option<&str>)->Self {
        let default=if coarse {1.} else {0.95};
        let opacity=opacity.and_then(|s|s.parse::<f32>().ok())
            .filter(|v|v.is_finite()).unwrap_or(default).clamp(0.,1.);
        Self {opacity,grid:nogrid.is_none()}
    }

    pub fn apply(&self,editor:&mut crate::editor::Editor) {
        editor.opacity=self.opacity;
        editor.camera.grid=self.grid.then(Default::default);
    }
}

pub fn density(ratio:f64,override_value:Option<&str>)->f64 {
    let ratio=if ratio>0. {ratio} else {1.};
    let cap=override_value.and_then(|s|s.parse::<f32>().ok())
        .filter(|v|v.is_finite()&&*v>=0.5).map(f64::from).unwrap_or(2.);
    ratio.min(cap)
}

#[cfg(target_arch="wasm32")]
fn query()->Option<web_sys::UrlSearchParams>{
    web_sys::UrlSearchParams::new_with_str(&web_sys::window()?.location().search().ok()?).ok()
}

#[cfg(target_arch="wasm32")]
pub fn scale(ratio:f64)->f64 {
    let value=query().and_then(|q|q.get("dpr"));
    density(ratio,value.as_deref())
}

#[cfg(target_arch="wasm32")]
pub fn initialize(editor:&mut crate::editor::Editor) {
    let Some(window)=web_sys::window() else {return};
    let coarse=window.match_media("(pointer: coarse)").ok().flatten()
        .is_some_and(|media|media.matches());
    let query=query();
    let opacity=query.as_ref().and_then(|q|q.get("opacity"));
    let nogrid=query.as_ref().and_then(|q|q.get("nogrid"));
    Settings::new(coarse,opacity.as_deref(),nogrid.as_deref()).apply(editor);
}
```

### 8. `src/browser.rs`

Give viewport images and CSS ink the same capped scale before recovery reduction.

**Locate this existing block:**

```rust
    let density = crate::browser_recovery::density(window.device_pixel_ratio());
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let density = crate::browser_recovery::density(crate::browser_defaults::scale(window.device_pixel_ratio()));
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/browser_defaults_tests.rs`

Supply high-density, finite override, lower-bound and fallback policy cases.

**Locate this existing block:**

```text
use crate::browser_defaults::Settings;
#[test]
fn pointer_defaults_and_url_overrides_share_existing_editor_state(){
 let mut editor=crate::editor::Editor::default();
 for (coarse,raw,nogrid,opacity,grid) in [
 (false,None,None,0.95,true),(true,None,None,1.,true),
 (true,Some("0.5"),Some("1"),0.5,false),
 (false,Some("NaN"),None,0.95,true),(true,Some("inf"),None,1.,true),
 (false,Some("2"),None,1.,true),(false,Some("-1"),None,0.,true)]{
  Settings::new(coarse,raw,nogrid).apply(&mut editor);
  assert_eq!(editor.opacity,opacity);assert_eq!(editor.camera.grid.is_some(),grid);
 }
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzg-cap-browser-device-scale-4.rs" skip=584
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 20.2
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 20.2
npm --prefix ../session_tests run course -- save 20.2
```

Continue with [20.3 · Shared arenas, object records and source preparation](../../consolidation/96zzg-grow-buffers-without-losing-rows.md).
