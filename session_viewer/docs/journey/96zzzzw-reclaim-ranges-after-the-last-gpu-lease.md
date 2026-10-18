# 20.12.4 · Reclaim ranges after the last GPU lease

**Chapter 20 · Shared GPU rendering · Section 20.12 · Arena reclamation** · [Contents](contents.md)

**Typing: 19–38 minutes.** [Estimate](typing-load.md).

Retire GPU ranges independently of precise CPU history ownership.

## Type

Continue from [Upload reused arena spans in place](96zzzzv-upload-reused-arena-spans-in-place.md). [Save or recover your work](recovery.md).

### 1. `src/mesh_arena.rs`

Use weak range tokens independently of CPU source ownership. Keep weak range leases under the existing arena owner. Begin without live range leases. Acquire one shared range token and retire only expired GPU leases; synchronize logical GPU lengths after free-tail trimming. Reset lease metadata with the existing explicit arena reset.

<details>
<summary>Locate the existing block</summary>

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

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{arena_rows::{Rows,MeshRange},mesh::Mesh,text_gpu_support::{GpuCtx,GrowBuf}};
use std::{collections::HashMap,rc::{Rc,Weak}};

pub struct Arena {
    leases:HashMap<usize,Weak<MeshRange>>,
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
        Self {leases:HashMap::new(),rows:Rows::default(),vertices,indices,layout,group,revision:0,uploads:0,uploaded_bytes:0}
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
    pub fn acquire(&mut self,ctx:&GpuCtx,source:&Rc<Mesh>)->Result<Rc<MeshRange>,&'static str>{
        self.prune();let key=Rc::as_ptr(source)as usize;
        if let Some(range)=self.leases.get(&key).and_then(Weak::upgrade){return Ok(range);}
        let range=Rc::new(self.upload(ctx,source)?);self.leases.insert(key,Rc::downgrade(&range));Ok(range)
    }
    pub fn prune(&mut self){
        let retired=self.leases.iter().filter(|(_,range)|range.strong_count()==0).map(|(&key,_)|key).collect::<Vec<_>>();
        for key in retired {self.leases.remove(&key);self.rows.retire(key);}
        self.vertices.truncate((self.rows.vertices.len()/20)as u32);self.indices.truncate(self.rows.indices.len()as u32);
    }
    pub fn reset(&mut self){self.leases.clear();self.rows.reset();self.vertices.reset();self.indices.reset();}
    pub fn release(&mut self,ctx:&GpuCtx){*self=Self::new(ctx);}
    pub fn allocated_bytes(&self)->u64{self.vertices.buf.size()+self.indices.buf.size()}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-01.rs"
```

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-02.rs"
```

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-03.rs"
```

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-04.rs"
```

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-05.rs"
```

</details>

### 2. `src/gpu_geometry.rs`

Keep the range reserved for every shared GPU geometry owner, including direct construction outside the cache. Acquire a range lease before publishing its immutable draw offsets. Hold the range token for the geometry lifetime. Read the same arena buffers/groups through leased storage without changing standalone preview ownership.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    }

    pub fn shared(ctx:&crate::text_gpu_support::GpuCtx,source:Rc<Mesh>,arena:Rc<RefCell<crate::mesh_arena::Arena>>)->Result<Self,&'static str>{
        let range=arena.borrow_mut().upload(ctx,&source)?;
        Ok(Self {source,index_count:range.indices.len()as u32,index_start:range.indices.start,storage:Storage::Shared(arena)})
    }
    pub(crate) fn buffers(&self)->(wgpu::Buffer,wgpu::Buffer){
        match &self.storage {
            Storage::Owned {vertices,indices,..}=>(vertices.clone(),indices.clone()),
            Storage::Shared(arena)=>{let arena=arena.borrow();(arena.vertices.buf.clone(),arena.indices.buf.clone())},
        }
    }
    fn group(&self)->wgpu::BindGroup{
        match &self.storage {Storage::Owned {group,..}=>group.clone(),Storage::Shared(arena)=>arena.borrow().group.clone()}
    }
    pub(crate) fn arena(&self)->Option<&Rc<RefCell<crate::mesh_arena::Arena>>> {match &self.storage {Storage::Shared(arena)=>Some(arena),Storage::Owned{..}=>None}}
    pub fn allocated_bytes(&self)->u64{
        match &self.storage {Storage::Owned {vertices,indices,..}=>vertices.size()+indices.size(),Storage::Shared(_)=>0}
    }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
```

</details>

Replace it with:

```rust
    }

    pub fn shared(ctx:&crate::text_gpu_support::GpuCtx,source:Rc<Mesh>,arena:Rc<RefCell<crate::mesh_arena::Arena>>)->Result<Self,&'static str>{
        let range=arena.borrow_mut().acquire(ctx,&source)?;
        Ok(Self {source,index_count:range.indices.len()as u32,index_start:range.indices.start,storage:Storage::Shared {arena,_lease:range}})
    }
    pub(crate) fn buffers(&self)->(wgpu::Buffer,wgpu::Buffer){
        match &self.storage {
            Storage::Owned {vertices,indices,..}=>(vertices.clone(),indices.clone()),
            Storage::Shared {arena,..}=>{let arena=arena.borrow();(arena.vertices.buf.clone(),arena.indices.buf.clone())},
        }
    }
    fn group(&self)->wgpu::BindGroup{
        match &self.storage {Storage::Owned {group,..}=>group.clone(),Storage::Shared {arena,..}=>arena.borrow().group.clone()}
    }
    pub(crate) fn arena(&self)->Option<&Rc<RefCell<crate::mesh_arena::Arena>>> {match &self.storage {Storage::Shared {arena,..}=>Some(arena),Storage::Owned{..}=>None}}
    pub fn allocated_bytes(&self)->u64{
        match &self.storage {Storage::Owned {vertices,indices,..}=>vertices.size()+indices.size(),Storage::Shared {..}=>0}
    }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
```

<details>
<summary>Locate this block</summary>

```rust

enum Storage {
    Owned {vertices:wgpu::Buffer,indices:wgpu::Buffer,group:wgpu::BindGroup},
    Shared(Rc<RefCell<crate::mesh_arena::Arena>>),
}

impl GpuGeometry {
```

</details>

Replace it with:

```rust

enum Storage {
    Owned {vertices:wgpu::Buffer,indices:wgpu::Buffer,group:wgpu::BindGroup},
    Shared {arena:Rc<RefCell<crate::mesh_arena::Arena>>,_lease:Rc<crate::arena_rows::MeshRange>},
}

impl GpuGeometry {
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::mesh::Mesh;
use std::{cell::RefCell,rc::Rc};
use wgpu::util::DeviceExt;

pub struct GpuGeometry {
    pub source: Rc<Mesh>,
    pub(crate) index_count: u32,
    pub(crate) index_start: u32,
    storage: Storage,
}

enum Storage {
    Owned {vertices:wgpu::Buffer,indices:wgpu::Buffer,group:wgpu::BindGroup},
    Shared {arena:Rc<RefCell<crate::mesh_arena::Arena>>,_lease:Rc<crate::arena_rows::MeshRange>},
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
        Self {source,index_count,index_start:0,storage:Storage::Owned {vertices,indices,group:pull_group}}
    }

    pub fn shared(ctx:&crate::text_gpu_support::GpuCtx,source:Rc<Mesh>,arena:Rc<RefCell<crate::mesh_arena::Arena>>)->Result<Self,&'static str>{
        let range=arena.borrow_mut().acquire(ctx,&source)?;
        Ok(Self {source,index_count:range.indices.len()as u32,index_start:range.indices.start,storage:Storage::Shared {arena,_lease:range}})
    }
    pub(crate) fn buffers(&self)->(wgpu::Buffer,wgpu::Buffer){
        match &self.storage {
            Storage::Owned {vertices,indices,..}=>(vertices.clone(),indices.clone()),
            Storage::Shared {arena,..}=>{let arena=arena.borrow();(arena.vertices.buf.clone(),arena.indices.buf.clone())},
        }
    }
    fn group(&self)->wgpu::BindGroup{
        match &self.storage {Storage::Owned {group,..}=>group.clone(),Storage::Shared {arena,..}=>arena.borrow().group.clone()}
    }
    pub(crate) fn arena(&self)->Option<&Rc<RefCell<crate::mesh_arena::Arena>>> {match &self.storage {Storage::Shared {arena,..}=>Some(arena),Storage::Owned{..}=>None}}
    pub fn allocated_bytes(&self)->u64{
        match &self.storage {Storage::Owned {vertices,indices,..}=>vertices.size()+indices.size(),Storage::Shared {..}=>0}
    }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
        if self.index_count == 0 || count == 0 { return; }
        pass.set_bind_group(3, &self.group(), &[]);
        pass.draw(0..self.index_count, 0..count);
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {self.draw_instances(pass,1);}

    pub fn draw_instances(&self,pass:&mut wgpu::RenderPass<'_>,count:u32){self.draw_range(pass,0..count);}
    pub(crate) fn draw_range(&self,pass:&mut wgpu::RenderPass<'_>,instances:std::ops::Range<u32>){
        if self.index_count==0||instances.is_empty(){return;}
        let(vertices,indices)=self.buffers();
        pass.set_vertex_buffer(0,vertices.slice(..));pass.set_index_buffer(indices.slice(..),wgpu::IndexFormat::Uint32);
        pass.draw_indexed(self.index_start..self.index_start+self.index_count,0,instances);
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-06.rs"
```

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-07.rs"
```

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-08.rs"
```

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-09.rs"
```

</details>

### 3. `src/geometry_cache.rs`

Capture real arena upload counters before acquisition, including hole reuse. Count actual new geometry uploads and written bytes once instead of inferring them from arena growth. Retire expired leases after scene replacement and retain the existing empty-owner release.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{gpu_geometry::GpuGeometry, mesh::Mesh};
use std::{cell::RefCell,collections::HashMap, rc::{Rc, Weak}};

#[derive(Default)]
pub struct GeometryCache {
    entries: HashMap<*const Mesh, Weak<GpuGeometry>>,
    arena:Option<Rc<RefCell<crate::mesh_arena::Arena>>>,
    pub uploads: usize,
    pub uploaded_bytes: u64,
}

impl GeometryCache {
    pub fn get(&mut self, ctx:&crate::text_gpu_support::GpuCtx, source: &Rc<Mesh>) -> Rc<GpuGeometry> {
        let key = Rc::as_ptr(source);
        if let Some(geometry) = self.entries.get(&key).and_then(Weak::upgrade) {
            return geometry;
        }
        let arena=self.arena.get_or_insert_with(||Rc::new(RefCell::new(crate::mesh_arena::Arena::new(ctx)))).clone();
        let (sources,bytes)={let arena=arena.borrow();(arena.rows.source_count(),arena.rows.vertices.len()+arena.rows.indices.len()*4)};
        let geometry=Rc::new(GpuGeometry::shared(ctx,Rc::clone(source),arena.clone()).expect("Prepared shared mesh ranges"));
        let arena=arena.borrow();
        self.uploads+=usize::from(arena.rows.source_count()!=sources);
        self.uploaded_bytes+=(arena.rows.vertices.len()+arena.rows.indices.len()*4-bytes)as u64;
        self.entries.insert(key, Rc::downgrade(&geometry));

        geometry
    }

    pub fn prune(&mut self) {
        self.entries.retain(|_, geometry| geometry.strong_count() > 0);
        if self.entries.is_empty(){self.arena=None;}
    }

    pub fn entries(&self) -> usize { self.entries.len() }
    pub fn allocated_bytes(&self)->u64{self.arena.as_ref().map_or(0,|arena|arena.borrow().allocated_bytes())}
    pub fn cpu_bytes(&self)->usize{self.arena.as_ref().map_or(0,|arena|{let arena=arena.borrow();arena.rows.vertices.capacity()+arena.rows.indices.capacity()*4})}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{gpu_geometry::GpuGeometry, mesh::Mesh};
use std::{cell::RefCell,collections::HashMap, rc::{Rc, Weak}};

#[derive(Default)]
pub struct GeometryCache {
    entries: HashMap<*const Mesh, Weak<GpuGeometry>>,
    arena:Option<Rc<RefCell<crate::mesh_arena::Arena>>>,
    pub uploads: usize,
    pub uploaded_bytes: u64,
}

impl GeometryCache {
    pub fn get(&mut self, ctx:&crate::text_gpu_support::GpuCtx, source: &Rc<Mesh>) -> Rc<GpuGeometry> {
        let key = Rc::as_ptr(source);
        if let Some(geometry) = self.entries.get(&key).and_then(Weak::upgrade) {
            return geometry;
        }
        let arena=self.arena.get_or_insert_with(||Rc::new(RefCell::new(crate::mesh_arena::Arena::new(ctx)))).clone();
        let (uploads,bytes)={let arena=arena.borrow();(arena.uploads,arena.uploaded_bytes)};
        let geometry=Rc::new(GpuGeometry::shared(ctx,Rc::clone(source),arena.clone()).expect("Prepared shared mesh ranges"));
        let arena=arena.borrow();
        self.uploads+=arena.uploads-uploads;
        self.uploaded_bytes+=arena.uploaded_bytes-bytes;
        self.entries.insert(key, Rc::downgrade(&geometry));

        geometry
    }

    pub fn prune(&mut self) {
        self.entries.retain(|_, geometry| geometry.strong_count() > 0);
        if let Some(arena)=&self.arena {arena.borrow_mut().prune();}
        if self.entries.is_empty(){self.arena=None;}
    }

    pub fn entries(&self) -> usize { self.entries.len() }
    pub fn allocated_bytes(&self)->u64{self.arena.as_ref().map_or(0,|arena|arena.borrow().allocated_bytes())}
    pub fn cpu_bytes(&self)->usize{self.arena.as_ref().map_or(0,|arena|{let arena=arena.borrow();arena.rows.vertices.capacity()+arena.rows.indices.capacity()*4})}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-10.rs"
```

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-11.rs"
```

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-12.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/mesh_arena_tests.rs`

Supply direct shared-construction aliases, last-owner retirement despite retained CPU history, exact reused offsets/bytes, safe restoration and logical empty release.

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

#[test]
fn actual_gpu_arena_reuses_ranges_preserves_bindings_and_counts_exact_writes(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut arena=crate::mesh_arena::Arena::new(&ctx);let a=mesh(-0.45,[1.,0.,0.]);let b=mesh(0.45,[0.,1.,0.]);let history=Rc::clone(&a);let first=arena.upload(&ctx,&a).unwrap();let survivor=arena.upload(&ctx,&b).unwrap();let group=arena.group.clone();let buffers=[arena.vertices.buf.clone(),arena.indices.buf.clone()];let revision=arena.revision;assert_eq!((arena.uploads,arena.uploaded_bytes),(2,144));
 assert!(arena.rows.retire(Rc::as_ptr(&a)as usize));let c=mesh(-0.45,[0.,0.,1.]);assert_eq!(arena.upload(&ctx,&c).unwrap(),first);assert_eq!(arena.upload(&ctx,&b).unwrap(),survivor);assert_eq!(arena.group,group);assert_eq!(arena.revision,revision);assert_eq!([arena.vertices.buf.clone(),arena.indices.buf.clone()],buffers);assert_eq!((arena.uploads,arena.uploaded_bytes),(3,216));assert_eq!(history.indices(),&[0,1,2]);
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.vertices.buf,120),[crate::packed_vertex::bytes(&c),crate::packed_vertex::bytes(&b)].concat());assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.indices.buf,24),[0u32,1,2,3,4,5].into_iter().flat_map(u32::to_le_bytes).collect::<Vec<_>>());let image=render(&ctx,&arena,&group);let left=(32*64+16)*4;let right=(32*64+48)*4;let center=(32*64+32)*4;assert_eq!(&image[left..left+4],&[0,0,255,255]);assert_eq!(&image[right..right+4],&[0,255,0,255]);
 let d=mesh(0.,[1.,1.,0.]);arena.upload(&ctx,&d).unwrap();assert_ne!(arena.group,group);assert!(arena.revision>revision);assert_eq!((arena.uploads,arena.uploaded_bytes),(4,288));let fresh=render(&ctx,&arena,&arena.group);let stale=render(&ctx,&arena,&group);assert_eq!(&fresh[center..center+4],&[255,255,0,255]);assert_eq!(&stale[center..center+4],&[255;4]);assert_eq!(&stale[left..left+4],&[0,0,255,255]);
 let before=crate::growing_buffer_tests::read(&ctx,&arena.vertices.buf,180);assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||arena.vertices.write_or_append(&ctx,u32::MAX,&[0;20]))).is_err());assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.vertices.buf,180),before);assert!(!arena.vertices.write_or_append(&ctx,9,&[]));
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-13.rs" skip=7560
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/global_outline_tests.rs`

Supply actual scene deletion/addition/Undo with retained precise CPU history, reused GPU ranges, stable survivor pixels/owners, exact uploads, original identity and empty release.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{scene::Scene,prepared::PreparedMesh,cap_gpu_tests::{renderer,pixels,matrix,wait}};
use std::rc::Rc;
fn width_pixels(r:&super::Renderer){let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:Some("global CSS width"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),&session_rust::Xform::identity().m.map(|v|v as f32));wait(r);}
#[test]
fn actual_global_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths=Vec::new();
    for curved in [false,true] {
        let mut scene=Scene::demo();scene.clear();let id=if curved {scene.insert_cad(Rc::new(session_rust::BRep::create_cylinder(0.4,0.8))).unwrap()} else {let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.8,0.8,0.2)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,0.5);scene.insert(prepared).unwrap()};let mut renderer=renderer(&scene);renderer.set_outlines(true);assert!(!renderer.meshes.iter().any(crate::gpu_mesh::GpuMesh::selected));let _=id;let geometry=renderer.stats();let mut css=Vec::new();
        for density in [1.,2.] {renderer.set_density(density);width_pixels(&renderer);let alpha=renderer.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));let row=&alpha[120*320..121*320];let width=row[160..].iter().map(|v|*v as f64/255.).sum::<f64>()/density as f64;assert!((0.7..1.7).contains(&width),"Actual outer silhouette CSS coverage remains bounded: {width}");css.push(width);assert_eq!(&renderer.stats()[..2],&geometry[..2]);}
        assert!((css[0]-css[1]).abs()<0.5,"DPI keeps the outline in CSS units: {css:?}");widths.push(css);
    }
    for i in 0..2 {assert!((widths[0][i]-widths[1][i]).abs()<0.25,"A smooth cylinder and flat box use one coverage width: {widths:?}");}
}

#[test]
fn actual_global_and_selected_outlines_share_alpha_and_one_black_blend(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();
 let original=scene.objects()[0].geometry().unwrap().to_proto();let mut r=renderer(&scene);let plain=pixels(&r);let uploads=r.stats()[0];r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().filter(|v|**v>0).count()>300);let usage=r.global_outline_usage();assert_eq!(pixels(&r),global);assert_eq!(r.global_outline_usage()[1],usage[1]);assert!(r.global_outline_usage()[2]>usage[2]);
 r.select_objects(&scene,&[owner]).unwrap();let both=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"same source and CSS width yield identical masks");
 let mut boundary=0;for(i,(&coverage,(a,b)))in alpha.iter().zip(global.chunks_exact(4).zip(both.chunks_exact(4))).enumerate(){if coverage>0&&plain[i*4..i*4+3]==[255,255,255]{boundary+=1;assert_eq!(a,b,"selected plus global uses max alpha in one blend");}}
 assert!(boundary>200);assert_eq!(r.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
 r.request_pick([0.,0.],matrix()).unwrap();wait(&r);let hit=r.take_pick(matrix()).expect("mapped original pick").unwrap();assert_eq!(hit.unwrap()[0],owner.value());
 r.select_objects(&scene,&[]).unwrap();r.set_outlines(false);assert_eq!(r.global_outline_usage(),[0;4]);assert_eq!(pixels(&r),plain);
 scene.clear();r.set_scene(&scene,None);r.set_outlines(true);pixels(&r);assert_eq!(r.global_outline_usage(),[0;4]);
}
#[test]
fn actual_visible_cap_coverage_keeps_selected_and_global_masks_equal(){
 let(mut scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=renderer(&scene);r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));
 r.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"selected visible cap marker joins original coverage");
 for(i,v)in alpha.iter().enumerate(){if *v==255{assert!(global[i*4..i*4+3].iter().all(|v|*v<12));assert!(selected[i*4..i*4+3].iter().all(|v|*v<12));}}
 r.set_clipping(false,true);pixels(&r);assert_eq!(r.global_outline_alpha(),alpha,"horizontal cut and uncut original share the same outer XY silhouette");assert_eq!(r.cap_usage()[0],16);
 let side=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Yz,&[[0.,0.,0.]],0.9).unwrap();crate::plane_original::insert(&mut scene,Rc::new(side)).unwrap();r.set_scene(&scene,Some(owner));r.set_clipping(true,true);pixels(&r);assert_ne!(r.global_outline_alpha(),alpha,"changed section inputs cannot reuse stale coverage");
}
#[test]
fn actual_global_outline_includes_locked_and_releases_hidden_originals(){
 let mut scene=Scene::empty();let mut source=crate::cap_gpu_tests::source(false);source.is_locked=true;let owner=scene.insert(PreparedMesh::new(source).unwrap()).unwrap();let mut r=renderer(&scene);r.set_outlines(true);pixels(&r);assert!(r.global_outline_alpha().iter().any(|v|*v==255));assert_eq!(scene.pick_id(owner.value()),None);
 Rc::make_mut(&mut scene.view).hidden.insert(owner);r.set_scene(&scene,None);pixels(&r);assert!(r.global_outline_alpha().is_empty());assert_eq!(r.global_outline_usage(),[0;4]);
}

#[test]
fn actual_triangle_ranges_keep_selection_updates_visibility_and_original_owners(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    let bases=|gpu:&super::Renderer|gpu.meshes.iter().map(|row|u32::from_ne_bytes(row.packed()[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())).collect::<Vec<_>>();assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes.iter().all(|row|!row.selected()));let baseline=pixels(&gpu);
    gpu.select_objects(&scene,&[b]).unwrap();assert_eq!(bases(&gpu),vec![1,2]);assert_eq!(gpu.meshes.iter().map(|row|row.selected()).collect::<Vec<_>>(),vec![false,true]);let selected=pixels(&gpu);assert_ne!(selected,baseline);
    scene.translate(b,[0.,0.1,0.]).unwrap();gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes[1].selected());for(position,owner)in[([-0.5,-0.1],a),([0.5,0.],b)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
    let before=gpu.meshes[1].packed();assert!(gpu.meshes[1].set_primitive_base(&gpu.queue,crate::primitive_image::CAP_START).is_err());assert_eq!(gpu.meshes[1].packed(),before);
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1]);assert_eq!(gpu.meshes[0].id,b);assert!(gpu.meshes[0].selected());pixels(&gpu);
}

fn primitive_pixels(gpu:&super::Renderer,texture:&wgpu::Texture)->Vec<u32>{
        let width=texture.width();let height=texture.height();let pitch=(width*4+255)/256*256;
        let read=gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("actual primitive addresses"),size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=gpu.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},texture.size());gpu.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());wait(gpu);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let values=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize*4].chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap()))).collect();drop(bytes);read.unmap();values
    }
#[test]
fn actual_primitive_mrt_keeps_placed_intervals_and_overlay_addresses(){

    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    for density in [1.,2.]{gpu.set_density(density);pixels(&gpu);let before=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(before[0],0);assert_eq!(before[120*320+80],2);assert_eq!(before[120*320+240],3);assert_eq!(gpu.primitive.texture.format(),crate::primitive_image::FORMAT);
        let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner:a,part:crate::pick_sources::Part::Face(key)})).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),before,"A derived selected-face mesh must retain original triangle addresses");
        gpu.set_highlight(&scene,None).unwrap();
    }
    let mut instances=Scene::empty();crate::scene_document::import(&mut instances,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();gpu.set_scene(&instances,None);pixels(&gpu);let placed=primitive_pixels(&gpu,&gpu.primitive.texture);let ids=gpu.meshes.iter().map(|mesh|mesh.primitive_base()+1).collect::<Vec<_>>();assert_eq!(ids.len(),2);assert_ne!(ids[0],ids[1]);assert!(ids.iter().all(|id|*id>0&&*id<crate::primitive_image::CAP_START));assert_eq!(placed[120*320+64],ids[0]);assert_eq!(placed[120*320+256],ids[1]);
    gpu.resize(crate::viewport::Viewport{width:640,height:480});assert_eq!([gpu.primitive.texture.width(),gpu.primitive.texture.height()],[640,480]);gpu.resize(crate::viewport::Viewport{width:320,height:240});gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|id|*id==0));
}

#[test]
fn actual_first_pick_primitives_match_current_cropped_owners_before_color(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
    for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
        for(position,owner,primitive)in[([-0.5,0.],a,2u32),([0.5,0.],b,3)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();assert_eq!([texture.width(),texture.height()],window.draw[2..]);assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],primitive);}
    }
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,None);gpu.request_pick([0.5,0.],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],b.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],2,"Visible survivor uses its current retained run without a color frame");
}

fn cap_primitive_frame(gpu:&super::Renderer){
 let texture=gpu.device.create_texture(&wgpu::TextureDescriptor{label:Some("cap primitive fixture"),size:wgpu::Extent3d{width:gpu.size[0],height:gpu.size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});
 gpu.draw(&texture.create_view(&Default::default()),&Default::default(),&matrix());wait(gpu);
}
#[test]
fn actual_cap_color_primitives_keep_plane_selection_and_empty_pixels(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});cap_primitive_frame(&gpu);
  let words=primitive_pixels(&gpu,&gpu.primitive.texture);let at=(96*density*320*density+208*density)as usize;
  assert_eq!(words[at],0xfffffff0);assert_eq!(words[0],0);
  gpu.select_objects(&scene,&[owner]).unwrap();cap_primitive_frame(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture)[at],0xfffffff1);
  gpu.select_objects(&scene,&[]).unwrap();
 }
 gpu.set_scene(&Scene::empty(),None);cap_primitive_frame(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
}

#[test]
fn actual_first_cap_pick_has_reserved_primitive_and_exact_original_owner(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
  assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
  gpu.request_pick([0.3,0.2],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
  let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let words=primitive_pixels(&gpu,&texture);
  let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];
  assert_eq!(words[(y*texture.width()+x)as usize],0xfffffff0);
  assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);
 }
}

#[test]
fn actual_compact_surface_upload_uses_twenty_bytes_per_vertex_and_releases(){
 let scene=Scene::demo();let mut gpu=renderer(&scene);let source=scene.objects()[0].mesh.clone();let original=source.vertices().to_vec();
 let buffers=gpu.meshes[0].geometry.buffers();
 assert_eq!(gpu.meshes[0].geometry.allocated_bytes(),0);
 assert_eq!(gpu.usage()[1],buffers.0.size()+buffers.1.size(),"One arena owner accounts for the actual shared buffers");
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let mut cache=crate::geometry_cache::GeometryCache::default();let uploaded=cache.get(&ctx,&source);
 assert_eq!(uploaded.buffers().0.size(),source.vertices().len()as u64*20);
 assert_eq!(cache.uploaded_bytes,cache.allocated_bytes(),"The first nonempty compact upload matches actual GPU allocation");
 let written=cache.uploaded_bytes;cache.get(&ctx,&source);assert_eq!(cache.uploads,1);assert_eq!(cache.uploaded_bytes,written);
 let empty=crate::gpu_geometry::GpuGeometry::upload(&gpu.device,std::rc::Rc::new(crate::mesh::Mesh::new(vec![],vec![]).unwrap()));assert_eq!(empty.allocated_bytes(),24);
 pixels(&gpu);assert_eq!(source.vertices(),original);let uploads=gpu.stats();pixels(&gpu);assert_eq!(gpu.stats(),uploads);
 gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.arena_cpu_bytes(),0);
}
#[test]
fn actual_nonzero_shared_ranges_keep_surface_face_picks_projection_outlines_and_caps() {
    use std::cell::RefCell;
    use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh,text_gpu_support::GpuCtx};
    for clipped in [false,true] {
        let scene=if clipped {crate::cap_gpu_tests::scene(false).0} else {
            let mut scene=Scene::empty();scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();scene
        };
        let mut r=renderer(&scene);r.set_outlines(true);
        let expected=pixels(&r);
        let mut hits=Vec::new();
        for mode in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face] {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            hits.push(r.take_pick(matrix()).unwrap().unwrap());
        }
        assert!(hits.iter().all(Option::is_some));
        if !clipped {assert_ne!(hits[1].unwrap()[1],0,"The uncut source returns an original face address");}
        let ctx=GpuCtx::new(&r.device,&r.queue);
        let arena=Rc::new(RefCell::new(crate::mesh_arena::Arena::new(&ctx)));
        let decoy=Rc::new(crate::mesh::Mesh::new(vec![[0.7,0.7,-0.6,0.,1.,0.],[0.9,0.7,-0.6,0.,1.,0.],[0.8,0.9,-0.6,0.,1.,0.]],vec![0,1,2]).unwrap());
        arena.borrow_mut().upload(&ctx,&decoy).unwrap();
        let layout=r.pipeline.get_bind_group_layout(1);
        for mesh in &mut r.meshes {
            let object=scene.objects().iter().find(|object|object.id==mesh.id).unwrap();
            let geometry=Rc::new(GpuGeometry::shared(&ctx,object.mesh.clone(),arena.clone()).unwrap());
            assert!(geometry.index_start>=3);
            *mesh=GpuMesh::with_geometry(&r.device,&layout,object,super::style(&scene,object.id,false),geometry);
        }
        r.caps.borrow_mut().set_scene(&r.queue,&scene,&mut r.meshes,&r.clipping.borrow().rows);
        let mut base=0;for mesh in &mut r.meshes {mesh.set_primitive_base(&r.queue,base).unwrap();base+=mesh.geometry.index_count/3;}
        r.tiles.borrow_mut().set(&r.device,&r.meshes);r.faces.clear();
        if let Some(outline)=r.global_outline.borrow_mut().as_mut(){outline.invalidate();}
        assert_eq!(pixels(&r),expected,"Nonzero shared ranges preserve actual color, clipping and outline pixels");
        for (mode,expected) in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face].into_iter().zip(hits) {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            assert_eq!(r.take_pick(matrix()).unwrap().unwrap(),expected,"Shared source addressing preserves original picks");
        }
        assert!(r.tiles.borrow().usage()[3]>0);
        if clipped {assert_eq!(r.cap_usage()[1],1);}
    }
}

#[test]
fn actual_contact_marker_is_opaque_unlit_selected_and_keeps_original_ownership(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();
 let vertices=prepared.display.vertices().len();prepared.display=prepared.display.with_alphas(vec![1./255.;vertices]).unwrap();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);
 let source=scene.objects()[0].mesh.clone();let color=source.vertices()[0][3..6].to_vec();let expected=color.iter().map(|v|(v.clamp(0.,1.)*255.).round()as u8).collect::<Vec<_>>();
 let uploads=gpu.stats()[0];for opacity in [1.,0.5,0.] {gpu.set_opacity(opacity);let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],expected.as_slice());gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
 gpu.select_objects(&scene,&[owner]).unwrap();let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],&[255,255,0]);
 assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);assert_eq!(gpu.stats()[0],uploads);assert_eq!(source.alphas(),vec![1./255.;vertices]);
 scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn current_palette_is_shared_by_surface_shader_and_preserves_original_picks_and_color(){
 use wgpu::naga;
 let rgba=crate::colour::selection();assert_eq!(rgba,session_rust::Color::yellow().to_unified_array());assert_eq!(rgba,[1.,1.,0.,1.]);
 let shader=crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0);assert!(shader.contains(&crate::colour::selection_shader()));naga::valid::Validator::new(naga::valid::ValidationFlags::all(),naga::valid::Capabilities::all()).validate(&naga::front::wgsl::parse_str(&shader).unwrap()).unwrap();
 let mut scene=Scene::empty();let prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let plain=pixels(&gpu);let uploads=gpu.stats()[0];
 gpu.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&gpu);assert!(selected.chunks_exact(4).any(|p|p[0]>10&&p[0]==p[1]&&p[2]==0));assert_ne!(plain,selected);gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
 gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),plain);assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn full_primitive_base_survives_surface_selection_and_reserved_range_checks(){
 assert!(include_str!("clip.wgsl").contains(&format!("const CAP_PRIMITIVE: u32 = 0x{:08x}u;",crate::primitive_image::CAP_START)));
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.4],[0.4,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let mut gpu=renderer(&scene);let original=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 for base in [0x8000_0000,crate::primitive_image::CAP_START-2]{
  gpu.meshes[0].set_primitive_base(&gpu.queue,base).unwrap();pixels(&gpu);let plain=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(plain[120*320+160],base+1);
  gpu.select_objects(&scene,&[owner]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);assert_eq!(u32::from_ne_bytes(gpu.meshes[0].packed()[64..68].try_into().unwrap()),1);
  gpu.select_objects(&scene,&[]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);let before=gpu.meshes[0].packed();for invalid in [crate::primitive_image::CAP_START-1,crate::primitive_image::CAP_START,u32::MAX]{assert!(gpu.meshes[0].set_primitive_base(&gpu.queue,invalid).is_err());assert_eq!(gpu.meshes[0].packed(),before);}
 }
 assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}

#[test]
fn actual_ordinary_surfaces_keep_one_own_row_slot_through_instances_highlights_and_release(){
 let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=[crate::draw_slots::OWN_ROW,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert!(gpu.instances.slots.allocated_bytes()>=8);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);let originals=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let owner=scene.objects()[0].id;
 for opacity in [1.,0.5]{gpu.set_opacity(opacity);let plain=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::Face(key)})).unwrap();assert_ne!(pixels(&gpu),plain);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_highlight(&scene,None).unwrap();assert_eq!(pixels(&gpu),plain);}
 assert_eq!(scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>(),originals);gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);
}

#[test]
fn actual_surface_batches_share_one_slot_owner_and_restore_ordinary_drawing(){
 use prost::Message;let mut source=crate::instance_specimen::message();let mut second=source.definitions.as_ref().unwrap().meshes[0].clone();second.guid="other-definition".into();source.definitions.as_mut().unwrap().meshes.push(second);source.objects.as_mut().unwrap().instances[1].definition_guid="other-definition".into();source.objects.as_mut().unwrap().instances[0].xform=None;source.objects.as_mut().unwrap().instances[1].flags=0;source.xforms=vec![session_rust::proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-0.6,0.,0.).to_proto())},session_rust::proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(0.6,0.,0.).to_proto())}];
 let mut scene=crate::scene::Scene::empty();crate::scene_document::import(&mut scene,&source.encode_to_vec(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=crate::cap_gpu_tests::renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(&gpu.instance_usage()[..3],&[2,2,16760]);assert_eq!(gpu.instance_allocations(),[352,24,16384]);assert_eq!(gpu.instances.slots.instances(),2);let slots=crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,24);assert_eq!(&slots[..8],&[u32::MAX,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(u32::from_ne_bytes(slots[8..12].try_into().unwrap()),0);assert_eq!(u32::from_ne_bytes(slots[16..20].try_into().unwrap()),1);
 let records=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),2*crate::object_record::BYTES as u64);
 let mut actual=records.chunks_exact(crate::object_record::BYTES).map(|r|r.to_vec()).collect::<Vec<_>>();actual.sort_unstable();let mut expected=gpu.meshes.iter().map(|m|m.packed().to_vec()).collect::<Vec<_>>();expected.sort_unstable();assert_eq!(actual,expected,"global records retain both original placements, IDs and ranges");
 let baseline=crate::cap_gpu_tests::pixels(&gpu);
 for half in 0..2 {let blue=baseline.chunks_exact(4).enumerate().filter(|(i,p)|(*i%320)/160==half&&p[2]>p[0].saturating_add(50)&&p[2]>p[1].saturating_add(30)).count();assert!(blue>100,"placed original blue triangles in half {half}: {blue}");}
 let uploads=gpu.stats()[0];let owner=scene.objects()[0].id;gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 gpu.set_opacity(0.5);crate::cap_gpu_tests::pixels(&gpu);gpu.set_opacity(1.);assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.set_scene(&crate::scene::Scene::empty(),None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_projected_records_follow_physical_gaps_clear_old_ranges_and_bound_fallback(){
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[-0.4,-0.4,0.],[0.4,-0.4,0.],[0.,0.4,0.]],[0.2,0.4,0.8]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);let baseline=pixels(&gpu);let triangles=gpu.meshes[0].geometry.index_count/3;assert_eq!(gpu.meshes.len(),1);let initial=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],u64::from(triangles)*96);let uploads=gpu.stats()[0];
 gpu.meshes[0].set_primitive_base(&gpu.queue,7).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+7));let projected=gpu.tile_buffers()[0].clone();let data=crate::growing_buffer_tests::read(&ctx,&projected,u64::from(triangles+7)*96);assert!(data[..7*96].iter().all(|v|*v==0));assert_eq!(&data[7*96..],initial);let projections=gpu.tile_usage()[5];pixels(&gpu);assert_eq!(gpu.tile_usage()[5],projections);
 gpu.meshes[0].set_primitive_base(&gpu.queue,3).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+3));assert_eq!(gpu.tile_buffers()[0],projected);let data=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],projected.size());assert!(data[..3*96].iter().all(|v|*v==0));assert_eq!(&data[3*96..(triangles as usize+3)*96],initial);assert!(data[(triangles as usize+3)*96..].iter().all(|v|*v==0));
 gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles));let current=gpu.tile_buffers()[0].clone();assert_eq!(current.size(),initial.len()as u64);assert_eq!(crate::growing_buffer_tests::read(&ctx,&current,current.size()),initial);assert_eq!(gpu.stats()[0],uploads);
 gpu.meshes[0].set_primitive_base(&gpu.queue,0x80000000).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);pixels(&gpu);assert!(gpu.tiles.borrow().fallback);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
 gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_ordinary_storage_records_match_legacy_pixels_and_primitives_through_growth_selection_and_release() {
 let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(gpu.instances.ordinary.len(),1);assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64),gpu.meshes[0].packed());let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);
 gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),1);
 let b=scene.insert(PreparedMesh::triangle([[-0.2,-0.2,-0.4],[0.2,-0.2,-0.4],[0.,0.2,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();scene.translate(b,[0.6,0.,0.]).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),2);let wanted=gpu.meshes.iter().flat_map(|m|m.packed()).collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),wanted.len()as u64),wanted);
 let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[a]).unwrap();assert_ne!(pixels(&gpu),stored);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);gpu.set_opacity(0.5);pixels(&gpu);gpu.set_opacity(1.);assert_eq!(pixels(&gpu),stored);
 for opacity in [0.,0.5,0.95,1.] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact ordinary blend at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);}
 let copy=scene.copy_row(b,false).unwrap();scene.translate(copy,[0.3,0.1,0.]).unwrap();let source=scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),3);let uploads=gpu.stats()[0];
 for opacity in [0.,0.5,0.95] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact transparent placements at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[copy]).unwrap();if opacity>0. {assert_ne!(pixels(&gpu),stored);}gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);assert_eq!(gpu.stats()[0],uploads);}
 assert_eq!(scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto(),source);gpu.set_opacity(1.);gpu.set_scene(&scene,None);pixels(&gpu);
 let location=|gpu:&super::Renderer,id:crate::scene::ObjectId|{let space=gpu.instances.space.as_ref().unwrap();let row=gpu.instances.ordinary[&id].1;space.draws().into_iter().flat_map(|draw|draw.slots).find(|slot|space.local_slot(*slot)[0]==row).unwrap()};let first_slot=location(&gpu,a);
 let retained=gpu.instances.ordinary.clone();assert!(scene.remove(b));gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 let fresh=scene.insert(PreparedMesh::triangle([[0.7,-0.3,-0.4],[0.9,-0.3,-0.4],[0.8,-0.1,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&fresh].1,retained[&b].1);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 for mesh in &gpu.meshes {let row=gpu.instances.ordinary[&mesh.id].1;let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),(row as u64+1)*crate::object_record::BYTES as u64);assert_eq!(&bytes[row as usize*crate::object_record::BYTES..],&mesh.packed());}pixels(&gpu);
 let projected=gpu.tiles.borrow().projected.clone();let expected=crate::growing_buffer_tests::read(&ctx,&projected,projected.size());let image=pixels(&gpu);gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),image);let uniform=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&uniform,uniform.size()),expected);gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);let restored=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&restored,restored.size()),expected);
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());assert!(gpu.instances.space.is_none());assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_lookup_projection_groups_reuse_current_inputs_and_replace_output_bindings() {
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let count=gpu.meshes[0].primitive_base()+1;assert_eq!(count,2);
 let mut state=crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).unwrap();assert_eq!(state.bytes(),16);assert_eq!(crate::growing_buffer_tests::read(&ctx,&state.parameters,16),[2u32,1,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
 let pipes=crate::tile_pipelines::Pipelines::new(&gpu.device,&gpu.pipeline.get_bind_group_layout(0));let output=|size|gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("lookup projection output witness"),size,usage:wgpu::BufferUsages::STORAGE,mapped_at_creation:false});let first=output(192);let group=state.group(&gpu.device,&pipes.lookup_project_layout,&first);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&first),group);let larger=output(384);let next=state.group(&gpu.device,&pipes.lookup_project_layout,&larger);assert_ne!(next,group);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&larger),next);
 assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,None,count).is_none());assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());gpu.instances.ordinary.clear();assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).is_none());scene.clear();gpu.set_scene(&scene,None);assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());
}

#[test]
fn actual_lookup_projection_matches_legacy_bytes_keeps_selection_cache_and_releases() {
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let other=scene.insert(PreparedMesh::triangle([[0.1,0.1,-0.3],[0.3,0.1,-0.3],[0.2,0.3,-0.3]],[0.1,0.4,0.9]).unwrap()).unwrap();let copy=scene.copy_row(other,false).unwrap();scene.translate(copy,[0.4,0.,0.]).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(0.95);gpu.set_scene(&scene,None);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 let image=pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_some());let output=gpu.tiles.borrow().projected.clone();let wanted=crate::growing_buffer_tests::read(&ctx,&output,output.size());let uploads=gpu.stats()[0];gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(pixels(&gpu),image);let output=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&output,output.size()),wanted);
 gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);assert!(gpu.tiles.borrow().lookup.is_some());let projections=gpu.tile_usage()[5];gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);assert_eq!(gpu.stats()[0],uploads);
 scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-14.rs" skip=37938
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.12.4
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Live shared geometry reclaims retired arena ranges while retaining original CAD ownership and safe history restoration.

**Verified checkpoint in Chrome.**

**Feature-specific visual review pending.** Scripted acceptance and the final main-page capture are separate evidence. A checker may restore its general fixture after closing the feature-test page; this image does not establish the feature’s visual correctness.

![General browser fixture after Reclaim ranges after the last GPU lease; feature-scene visual acceptance pending.](../screenshots/journey/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-browser.png)

[Verification scope](release.md).

This reference image is shared with [20.11.6 · Adopt retained GPU slots and physical bases](96zzzzn-adopt-retained-gpu-slots-and-physical-bases.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

A source mesh can remain alive in Undo while no GPU geometry uses its range. Give every shared GPU geometry a strong range token; the arena retains weak tokens only. Prune expired leases, remove their source associations and return their spans. Keep surviving ranges fixed, reuse holes for later uploads and derive cache statistics from actual arena writes.

Last shared GPU owner drops → weak range lease expires → retire source/span → later geometry reuses the hole.



Why is a weak source mesh insufficient for retirement?

History may keep the precise mesh alive after its GPU geometry is gone. The range follows GPU leases, while source and history ownership remain independent.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Create two original meshes, delete the first while it remains in Undo, then add another mesh. The new mesh should reuse the hole; the survivor must keep its placement and pick owner. Undo twice and inspect exact restoration, then reload an empty scene.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.12.4
npm --prefix ../session_tests run course -- save 20.12.4
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Live arena reclamation is connected. Current-viewer parity still requires precise coordinates/depth, rendering quality, remaining tools and browser lifetime acceptance.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.12.4
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
