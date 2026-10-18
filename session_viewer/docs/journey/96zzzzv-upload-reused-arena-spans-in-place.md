# 20.12.3 · Upload reused arena spans in place

**Chapter 20 · Shared GPU rendering · Section 20.12 · Arena reclamation** · [Contents](contents.md)

**Typing: 19–38 minutes.** [Estimate](typing-load.md).

Write reused geometry ranges without replacing unchanged GPU buffers.

## Type

Continue from [Reuse retired arena record ranges](96zzzzu-reuse-retired-arena-record-ranges.md). [Save or recover your work](recovery.md).

### 1. `src/text_gpu_support.rs`

Use the same device-limit calculation for append and bounded overwrite operations. Validate alignment, start and complete end before any write, then reuse existing bounded overwrite and growth operations.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        }),stride,len:0,label,usage}
    }

    pub fn append_bytes(&mut self,ctx:&GpuCtx,bytes:&[u8])->bool {
        assert_eq!(bytes.len() as u64%self.stride,0);
        if bytes.is_empty(){return false;}
        let rows=u32::try_from(bytes.len() as u64/self.stride).expect("Bounded GPU records");
        let end=self.len.checked_add(rows).expect("GPU row count");
        let limits=ctx.device.limits();
        let limit=if self.usage.contains(wgpu::BufferUsages::STORAGE){
            limits.max_buffer_size.min(u64::from(limits.max_storage_buffer_binding_size))
        }else{limits.max_buffer_size};
        let most=limit/self.stride;assert!(u64::from(end)<=most);
        let grew=u64::from(end)*self.stride>self.buf.size();
        if grew {
            let cap=self.buf.size()/self.stride;
```

</details>

Replace it with:

```rust
        }),stride,len:0,label,usage}
    }

    fn max_rows(&self,ctx:&GpuCtx)->u64 {
        let limits=ctx.device.limits();let bytes=if self.usage.contains(wgpu::BufferUsages::STORAGE){limits.max_buffer_size.min(u64::from(limits.max_storage_buffer_binding_size))}else{limits.max_buffer_size};bytes/self.stride
    }
    pub fn write_or_append(&mut self,ctx:&GpuCtx,at:u32,bytes:&[u8])->bool {
        assert_eq!(bytes.len()as u64%self.stride,0);assert!(at<=self.len);
        let rows=u32::try_from(bytes.len()as u64/self.stride).expect("Bounded GPU records");let end=at.checked_add(rows).expect("GPU row count");assert!(u64::from(end)<=self.max_rows(ctx));
        let inside=usize::try_from(u64::from((self.len-at).min(rows))*self.stride).unwrap();
        self.write_bytes_at(ctx,at,&bytes[..inside]);self.append_bytes(ctx,&bytes[inside..])
    }
    pub fn append_bytes(&mut self,ctx:&GpuCtx,bytes:&[u8])->bool {
        assert_eq!(bytes.len() as u64%self.stride,0);
        if bytes.is_empty(){return false;}
        let rows=u32::try_from(bytes.len() as u64/self.stride).expect("Bounded GPU records");
        let end=self.len.checked_add(rows).expect("GPU row count");
        let most=self.max_rows(ctx);assert!(u64::from(end)<=most);
        let grew=u64::from(end)*self.stride>self.buf.size();
        if grew {
            let cap=self.buf.size()/self.stride;
```

<details>
<summary>Complete updated block reference</summary>

```rust
pub struct GpuCtx {pub device:wgpu::Device,pub queue:wgpu::Queue}
impl GpuCtx {pub fn new(device:&wgpu::Device,queue:&wgpu::Queue)->Self {Self {device:device.clone(),queue:queue.clone()}}}
pub type Pipeline=wgpu::RenderPipeline;
#[derive(Clone,Copy)]
pub struct Target {pub format:wgpu::TextureFormat,pub samples:u32}
impl Target {pub const ID:Self=Self {format:wgpu::TextureFormat::Rg32Uint,samples:1};}
pub const VERTS:wgpu::BufferUsages=wgpu::BufferUsages::VERTEX.union(wgpu::BufferUsages::COPY_DST);
pub struct GrowBuf {
    pub buf:wgpu::Buffer,
    stride:u64,len:u32,label:&'static str,usage:wgpu::BufferUsages,
}
impl GrowBuf {
    pub fn new(ctx:&GpuCtx,label:&'static str,stride:u64,usage:wgpu::BufferUsages)->Self {
        assert!(stride>0&&stride%4==0);
        let usage=usage|wgpu::BufferUsages::COPY_SRC|wgpu::BufferUsages::COPY_DST;
        Self {buf:ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label:Some(label),size:stride,usage,mapped_at_creation:false,
        }),stride,len:0,label,usage}
    }

    fn max_rows(&self,ctx:&GpuCtx)->u64 {
        let limits=ctx.device.limits();let bytes=if self.usage.contains(wgpu::BufferUsages::STORAGE){limits.max_buffer_size.min(u64::from(limits.max_storage_buffer_binding_size))}else{limits.max_buffer_size};bytes/self.stride
    }
    pub fn write_or_append(&mut self,ctx:&GpuCtx,at:u32,bytes:&[u8])->bool {
        assert_eq!(bytes.len()as u64%self.stride,0);assert!(at<=self.len);
        let rows=u32::try_from(bytes.len()as u64/self.stride).expect("Bounded GPU records");let end=at.checked_add(rows).expect("GPU row count");assert!(u64::from(end)<=self.max_rows(ctx));
        let inside=usize::try_from(u64::from((self.len-at).min(rows))*self.stride).unwrap();
        self.write_bytes_at(ctx,at,&bytes[..inside]);self.append_bytes(ctx,&bytes[inside..])
    }
    pub fn append_bytes(&mut self,ctx:&GpuCtx,bytes:&[u8])->bool {
        assert_eq!(bytes.len() as u64%self.stride,0);
        if bytes.is_empty(){return false;}
        let rows=u32::try_from(bytes.len() as u64/self.stride).expect("Bounded GPU records");
        let end=self.len.checked_add(rows).expect("GPU row count");
        let most=self.max_rows(ctx);assert!(u64::from(end)<=most);
        let grew=u64::from(end)*self.stride>self.buf.size();
        if grew {
            let cap=self.buf.size()/self.stride;
            let cap=(cap+cap/2).max(u64::from(end)).min(most);
            let next=ctx.device.create_buffer(&wgpu::BufferDescriptor {
                label:Some(self.label),size:cap*self.stride,usage:self.usage,mapped_at_creation:false,
            });
            if self.len>0 {
                let mut encoder=ctx.device.create_command_encoder(&Default::default());
                encoder.copy_buffer_to_buffer(&self.buf,0,&next,0,u64::from(self.len)*self.stride);
                ctx.queue.submit([encoder.finish()]);
            }
            self.buf=next;
        }
        ctx.queue.write_buffer(&self.buf,u64::from(self.len)*self.stride,bytes);
        self.len=end;grew
    }

    pub fn truncate(&mut self,len:u32){assert!(len<=self.len);self.len=len;}

    pub fn write_bytes_at(&self,ctx:&GpuCtx,at:u32,bytes:&[u8]) {
        assert_eq!(bytes.len()as u64%self.stride,0);
        let rows=bytes.len()as u64/self.stride;
        assert!(u64::from(at)+rows<=u64::from(self.len));
        if !bytes.is_empty(){ctx.queue.write_buffer(&self.buf,u64::from(at)*self.stride,bytes);}
    }

    pub fn append<const N:usize>(&mut self,ctx:&GpuCtx,vertices:&[[f32;N]]) {
        assert_eq!(self.stride,N as u64*4);
        let bytes:Vec<u8>=vertices.iter().flatten().flat_map(|v|v.to_ne_bytes()).collect();
        self.append_bytes(ctx,&bytes);
    }
    pub fn reset(&mut self){self.len=0;}
    pub fn release(&mut self,ctx:&GpuCtx){*self=Self::new(ctx,self.label,self.stride,self.usage);}
    pub fn len(&self)->u32{self.len}
    pub fn is_empty(&self)->bool{self.len==0}
}
pub fn texture_entry(binding:u32,visibility:wgpu::ShaderStages,sample_type:wgpu::TextureSampleType,multisampled:bool)->wgpu::BindGroupLayoutEntry {wgpu::BindGroupLayoutEntry {binding,visibility,ty:wgpu::BindingType::Texture {sample_type,view_dimension:wgpu::TextureViewDimension::D2,multisampled},count:None}}
pub fn resource_group<'a,const N:usize>(ctx:&GpuCtx,layout:&wgpu::BindGroupLayout,label:&str,entries:[(u32,wgpu::BindingResource<'a>);N])->wgpu::BindGroup {ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {label:Some(label),layout,entries:&entries.into_iter().map(|(binding,resource)|wgpu::BindGroupEntry {binding,resource}).collect::<Vec<_>>()})}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzv-upload-reused-arena-spans-in-place-01.rs"
```

```text
--8<-- "journey/code/96zzzzv-upload-reused-arena-spans-in-place-02.rs"
```

</details>

### 2. `src/mesh_arena.rs`

Measure real new geometry uploads and written source bytes under the arena owner. Start upload counters with the arena. Write exact reused ranges and count real writes; preserve current groups unless a buffer allocation changes.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{arena_rows::{Rows,MeshRange},mesh::Mesh,text_gpu_support::{GpuCtx,GrowBuf}};
use std::rc::Rc;

pub struct Arena {
    pub rows:Rows,pub vertices:GrowBuf,pub indices:GrowBuf,
    pub layout:wgpu::BindGroupLayout,pub group:wgpu::BindGroup,pub revision:u64,
}
impl Arena {
    pub fn new(ctx:&GpuCtx)->Self {
        let vertices=GrowBuf::new(ctx,"arena compact vertices",20,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::VERTEX);
        let indices=GrowBuf::new(ctx,"arena indices",4,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::INDEX);
        let layout=crate::gpu_geometry::GpuGeometry::storage_layout(&ctx.device);
        let group=Self::bind(ctx,&layout,&vertices,&indices);
        Self {rows:Rows::default(),vertices,indices,layout,group,revision:0}
    }
    fn bind(ctx:&GpuCtx,layout:&wgpu::BindGroupLayout,vertices:&GrowBuf,indices:&GrowBuf)->wgpu::BindGroup {
        ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:Some("shared arena reads"),layout,
            entries:&[(0,&vertices.buf),(2,&indices.buf)].map(|(binding,buffer)|wgpu::BindGroupEntry {binding,resource:buffer.as_entire_binding()}),
        })
    }
    pub fn upload(&mut self,ctx:&GpuCtx,source:&Rc<Mesh>)->Result<MeshRange,&'static str>{
        let vertex_start=self.rows.vertices.len();let index_start=self.rows.indices.len();
        let range=self.rows.append(source)?;
        let vertices=self.vertices.append_bytes(ctx,&self.rows.vertices[vertex_start..]);
        let indices=self.indices.append_bytes(ctx,&self.rows.index_bytes_from(index_start));
        if vertices||indices {
            self.group=Self::bind(ctx,&self.layout,&self.vertices,&self.indices);
            self.revision+=1;
        }
        Ok(range)
    }
    pub fn reset(&mut self){self.rows.reset();self.vertices.reset();self.indices.reset();}
    pub fn release(&mut self,ctx:&GpuCtx){*self=Self::new(ctx);}
    pub fn allocated_bytes(&self)->u64{self.vertices.buf.size()+self.indices.buf.size()}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{arena_rows::{Rows,MeshRange},mesh::Mesh,text_gpu_support::{GpuCtx,GrowBuf}};
use std::rc::Rc;

pub struct Arena {
    pub rows:Rows,pub vertices:GrowBuf,pub indices:GrowBuf,
    pub uploads:usize,pub uploaded_bytes:u64,
    pub layout:wgpu::BindGroupLayout,pub group:wgpu::BindGroup,pub revision:u64,
}
impl Arena {
    pub fn new(ctx:&GpuCtx)->Self {
        let vertices=GrowBuf::new(ctx,"arena compact vertices",20,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::VERTEX);
        let indices=GrowBuf::new(ctx,"arena indices",4,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::INDEX);
        let layout=crate::gpu_geometry::GpuGeometry::storage_layout(&ctx.device);
        let group=Self::bind(ctx,&layout,&vertices,&indices);
        Self {rows:Rows::default(),vertices,indices,layout,group,revision:0,uploads:0,uploaded_bytes:0}
    }
    fn bind(ctx:&GpuCtx,layout:&wgpu::BindGroupLayout,vertices:&GrowBuf,indices:&GrowBuf)->wgpu::BindGroup {
        ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:Some("shared arena reads"),layout,
            entries:&[(0,&vertices.buf),(2,&indices.buf)].map(|(binding,buffer)|wgpu::BindGroupEntry {binding,resource:buffer.as_entire_binding()}),
        })
    }
    pub fn upload(&mut self,ctx:&GpuCtx,source:&Rc<Mesh>)->Result<MeshRange,&'static str>{
        let sources=self.rows.source_count();let range=self.rows.append(source)?;
        if sources==self.rows.source_count(){return Ok(range);}
        let bytes=&self.rows.vertices[range.vertices.start as usize*20..range.vertices.end as usize*20];
        let index_bytes=self.rows.indices[range.indices.start as usize..range.indices.end as usize].iter().flat_map(|i|i.to_le_bytes()).collect::<Vec<_>>();
        let vertices=self.vertices.write_or_append(ctx,range.vertices.start,bytes);
        let indices=self.indices.write_or_append(ctx,range.indices.start,&index_bytes);
        self.uploads+=1;self.uploaded_bytes+=(bytes.len()+index_bytes.len())as u64;
        self.vertices.truncate((self.rows.vertices.len()/20)as u32);self.indices.truncate(self.rows.indices.len()as u32);
        if vertices||indices {self.group=Self::bind(ctx,&self.layout,&self.vertices,&self.indices);self.revision+=1;}
        Ok(range)
    }
    pub fn reset(&mut self){self.rows.reset();self.vertices.reset();self.indices.reset();}
    pub fn release(&mut self,ctx:&GpuCtx){*self=Self::new(ctx);}
    pub fn allocated_bytes(&self)->u64{self.vertices.buf.size()+self.indices.buf.size()}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzv-upload-reused-arena-spans-in-place-03.rs"
```

```text
--8<-- "journey/code/96zzzzv-upload-reused-arena-spans-in-place-04.rs"
```

```text
--8<-- "journey/code/96zzzzv-upload-reused-arena-spans-in-place-05.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/mesh_arena_tests.rs`

Supply mapped reused bytes, exact upload totals, retained CPU source, real blue/green/yellow pixels, old/fresh immutable bindings and refused-write preservation.

<details>
<summary>Locate the existing block</summary>

```text
use crate::text_gpu_support::GpuCtx;
use std::rc::Rc;
fn mesh(center:f32,color:[f32;3])->Rc<crate::mesh::Mesh>{
 Rc::new(crate::mesh::Mesh::new(vec![[center-0.4,-0.6,0.,color[0],color[1],color[2]],[center+0.4,-0.6,0.,color[0],color[1],color[2]],[center,0.6,0.,color[0],color[1],color[2]]],vec![0,1,2]).unwrap())
}
#[test]
fn actual_gpu_arena_appends_exact_records_reuses_identity_and_releases(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut arena=crate::mesh_arena::Arena::new(&ctx);
 let a=mesh(-0.45,[1.,0.,0.]);let b=mesh(0.45,[0.,1.,0.]);let first=arena.upload(&ctx,&a).unwrap();let revision=arena.revision;
 assert_eq!(arena.upload(&ctx,&a).unwrap(),first);assert_eq!(arena.revision,revision);
 let second=arena.upload(&ctx,&b).unwrap();assert_eq!(second.vertices,3..6);assert_eq!(second.indices,3..6);assert!(arena.revision>revision);
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.vertices.buf,120),[crate::packed_vertex::bytes(&a),crate::packed_vertex::bytes(&b)].concat());
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.indices.buf,24),arena.rows.index_bytes());
 let allocated=arena.allocated_bytes();let revision=arena.revision;arena.reset();assert_eq!(arena.rows.source_count(),0);assert!(arena.vertices.is_empty());assert!(arena.indices.is_empty());
 assert_eq!(arena.upload(&ctx,&b).unwrap().vertices,0..3);assert_eq!(arena.allocated_bytes(),allocated);assert_eq!(arena.revision,revision);
 arena.release(&ctx);assert_eq!(arena.allocated_bytes(),24);assert!(arena.vertices.is_empty());assert!(arena.indices.is_empty());
}
fn render(ctx:&GpuCtx,arena:&crate::mesh_arena::Arena,group:&wgpu::BindGroup)->Vec<u8>{
 let shader=ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("arena binding fixture"),source:wgpu::ShaderSource::Wgsl(format!("{}\n{}",include_str!("packed_vertex.wgsl"),r#"
 @group(0) @binding(0) var<storage,read> vertices:array<PackedVertex>;
 @group(0) @binding(2) var<storage,read> indices:array<u32>;
 struct Out{@builtin(position) position:vec4<f32>,@location(0) color:vec4<f32>}
 @vertex fn vertex(@builtin(vertex_index) occurrence:u32)->Out{
  let v=vertices[indices[occurrence]];return Out(vec4<f32>(packed_position(v),1.),unpack4x8unorm(v.color));
 }
 @fragment fn fragment(input:Out)->@location(0) vec4<f32>{return input.color;}
 "#).into())});
 let layout=ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:None,bind_group_layouts:&[Some(&arena.layout)],immediate_size:0});
 let pipeline=ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:None,layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::Rgba8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None});
 let texture=ctx.device.create_texture(&wgpu::TextureDescriptor{label:Some("actual arena colors"),size:wgpu::Extent3d{width:64,height:64,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::COPY_SRC,view_formats:&[]});let view=texture.create_view(&Default::default());
 let mut encoder=ctx.device.create_command_encoder(&Default::default());{
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::WHITE),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&pipeline);pass.set_bind_group(0,group,&[]);pass.draw(0..arena.indices.len(),0..1);
 }
 let output=ctx.device.create_buffer(&wgpu::BufferDescriptor{label:None,size:64*256,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::COPY_SRC,mapped_at_creation:false});
 encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&output,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(256),rows_per_image:Some(64)}},texture.size());ctx.queue.submit([encoder.finish()]);crate::growing_buffer_tests::read(ctx,&output,64*256)
}
#[test]
fn actual_arena_draw_rebuilds_immutable_bindings_after_growth(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut arena=crate::mesh_arena::Arena::new(&ctx);
 let a=mesh(-0.45,[1.,0.,0.]);let b=mesh(0.45,[0.,1.,0.]);arena.upload(&ctx,&a).unwrap();let old=arena.group.clone();arena.upload(&ctx,&b).unwrap();
 let fresh=render(&ctx,&arena,&arena.group);let left=(32*64+16)*4;let right=(32*64+48)*4;
 assert_eq!(&fresh[left..left+4],&[255,0,0,255]);assert_eq!(&fresh[right..right+4],&[0,255,0,255]);
 let stale=render(&ctx,&arena,&old);assert_eq!(&stale[left..left+4],&[255,0,0,255]);assert_eq!(&stale[right..right+4],&[255;4],"An old binding cannot read the appended right-hand range");
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzv-upload-reused-arena-spans-in-place-06.rs" skip=5276
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.12.3
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The arena can upload reused ranges correctly; live retirement remains unconnected until geometry leases are tracked.

**Verified checkpoint in Chrome.**

**Feature-specific visual review pending.** Scripted acceptance and the final main-page capture are separate evidence. A checker may restore its general fixture after closing the feature-test page; this image does not establish the feature’s visual correctness.

![General browser fixture after Upload reused arena spans in place; feature-scene visual acceptance pending.](../screenshots/journey/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-browser.png)

[Verification scope](release.md).

This reference image is shared with [20.11.6 · Adopt retained GPU slots and physical bases](96zzzzn-adopt-retained-gpu-slots-and-physical-bases.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

A reused span is inside the buffer, so appending its bytes would draw the old data. Add a bounded overwrite-and-tail operation to the growing buffer, sharing its existing device-limit calculation. Upload each arena range at its actual start, retain immutable groups for in-place writes and refresh them on growth. Count actual new geometry uploads and source bytes, including reused holes.

Current arena range → bounded overwrite or tail → retained/grown buffers → exact upload totals.



Why does hole reuse need explicit upload accounting?

The arena length may stay unchanged even though a new source writes vertex and index bytes.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Repeat original selection and restore it. Trace a retired left-hand triangle replaced by a blue triangle while the right-hand range stays fixed; buffer growth must replace the immutable group.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.12.3
npm --prefix ../session_tests run course -- save 20.12.3
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

In-place arena uploads and measured writes are prepared. Actual GPU geometry lease retirement is the remaining reclamation connection.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.12.3
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
