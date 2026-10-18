# 20.9.5 · Read source faces through shared records

**Chapter 20 · Shared GPU rendering · Section 20.9 · Stable records and original picking** · [Contents](contents.md)

**Typing: 22–44 minutes.** [Estimate](typing-load.md).

Connect original source-face picking to the same current object records.

## Type

Continue from [Pick originals through shared records](96zzzv-pick-originals-through-shared-records.md). [Save or recover your work](recovery.md).

### 1. `src/instance_gpu.rs`

Reuse current-record validation and slot binding for consumers with different geometry layouts. Bind only the shared object record and slot; let each consumer draw its own geometry.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.ordinary_pipeline)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        let Some(&(first,row))=self.ordinary.get(&mesh.id)else{return false};
        if !self.records.matches(row,&mesh.packed()){return false;}
        pass.set_pipeline(pipeline);self.records.bind(pass);self.slots.bind(pass,0,first);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
```

</details>

Replace it with:

```rust
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.ordinary_pipeline)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        if !self.bind_record(pass,mesh){return false;}pass.set_pipeline(pipeline);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn bind_record(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some(&(first,row))=self.ordinary.get(&mesh.id)else{return false};
        if !self.records.matches(row,&mesh.packed()){return false;}
        self.records.bind(pass);self.slots.bind(pass,0,first);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::BTreeMap,rc::Rc};
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pipeline:wgpu::RenderPipeline,ordinary_pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
 let candidates=rows.iter().filter(|(_,key,declared)|*declared||uses[key]>1).map(|(id,_,_)|*id).collect::<std::collections::HashSet<_>>();
 if candidates.len()<=limit {candidates}else{rows.into_iter().filter(|(_,_,declared)|*declared).map(|(id,_,_)|id).collect()}
}
pub(crate) fn stable_rows(previous:&std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,ids:&[crate::scene::ObjectId],limit:usize)->Result<std::collections::HashMap<crate::scene::ObjectId,u32>,&'static str> {
 let mut rows=std::collections::HashMap::new();let mut used=std::collections::HashSet::new();
 for id in ids {if let Some(&(_,row))=previous.get(id) {if row as usize>=limit||!used.insert(row){return Err("Invalid retained object row");}rows.insert(*id,row);}}
 let mut next=0usize;
 for id in ids {if rows.contains_key(id){continue;}while next<limit&&used.contains(&(next as u32)){next+=1;}if next>=limit||u32::try_from(next).is_err(){return Err("Object records exceed device limits");}let row=next as u32;rows.insert(*id,row);used.insert(row);}
 Ok(rows)
}
pub(crate) fn source()->String {
 crate::clipping_gpu::source(&crate::object_rows::source(include_str!("triangle.wgsl")),2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=crate::object_rows::layout(device);
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let pipeline=create(None);let ordinary_pipeline=create(Some(wgpu::BlendState::ALPHA_BLENDING));
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>,ordinary_rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<(crate::scene::ObjectId,Record)>)>=BTreeMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push((row.id,row.packed()));}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let own=ordinary_rows.into_iter().collect::<Vec<_>>();let own=if own.len()<=limit-count {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let mut batches=BTreeMap::new();let mut ordinary=std::collections::HashMap::new();let mut all_slots=Vec::new();let mut all_records=vec![[0;BYTES];indices.values().max().map_or(0,|row|*row as usize+1)];
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for (id,record) in &records {
                let row=indices[id];
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                ordinary.insert(*id,(u32::try_from(all_slots.len()).map_err(|_|"Instance slot range")?,row));
                all_records[row as usize]=*record;
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        for mesh in own {
            let first=u32::try_from(all_slots.len()+1).map_err(|_|"Ordinary slot range")?;let row=indices[&mesh.id];
            all_slots.push([row,mesh.primitive_base()]);all_records[row as usize]=mesh.packed();ordinary.insert(mesh.id,(first,row));
        }
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
        if self.records.written_bytes>0{self.writes+=1;self.written_bytes+=self.records.written_bytes;}
        if all_slots.is_empty()&&self.slots.instances()>0{self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;self.ordinary=ordinary;Ok(())
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.ordinary_pipeline)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        if !self.bind_record(pass,mesh){return false;}pass.set_pipeline(pipeline);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn bind_record(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some(&(first,row))=self.ordinary.get(&mesh.id)else{return false};
        if !self.records.matches(row,&mesh.packed()){return false;}
        self.records.bind(pass);self.slots.bind(pass,0,first);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.records.allocated_bytes()+if self.slots.instances()==0{0}else{self.slots.allocated_bytes()},self.writes,self.written_bytes]}
}

#[cfg(test)]
mod stable_row_tests {
 use super::*;
 #[test]
 fn retained_ids_reuse_holes_and_reject_capacity_overflow() {
  let mut scene=crate::scene::Scene::empty();let mesh=||crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap();let a=scene.insert(mesh()).unwrap();let b=scene.insert(mesh()).unwrap();let c=scene.insert(mesh()).unwrap();
  let old=std::collections::HashMap::from([(a,(1,0)),(b,(2,1)),(c,(3,2))]);let rows=stable_rows(&old,&[c,a,b],3).unwrap();assert_eq!((rows[&a],rows[&b],rows[&c]),(0,1,2));
  let d=scene.insert(mesh()).unwrap();let rows=stable_rows(&old,&[c,d,a],3).unwrap();assert_eq!((rows[&a],rows[&d],rows[&c]),(0,1,2));assert!(!rows.contains_key(&b));assert!(stable_rows(&old,&[a,b,c,d],3).is_err());assert!(stable_rows(&old,&[c],2).is_err());assert!(stable_rows(&old,&[],0).unwrap().is_empty());
 }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-01.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-02.rs"
```

</details>

### 2. `src/face_pick.wgsl`

Carry original ownership into the source-face fragment as an unsigned value. Share one source-face geometry function between ordinary and slot-selected records. Read the original owner selected by the source-face vertex.

<details>
<summary>Locate the existing block</summary>

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
    let vertex = positions[indices[index+object_settings.range.x]];
    let placed = geometry(packed_position(vertex), unpack4x8unorm(vertex.color), vec3<f32>(0.0),object_settings);
    return FaceOutput(crop.transform * placed.position, faces[index / 3u] + source.base,placed.world,placed.clip_flags);
}
@fragment
fn face_fragment(input: FaceOutput) -> @location(0) vec2<u32> {
    if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}
    return vec2<u32>(object_settings.selection.y, (0x20000000u | input.face) + 1u);
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

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
    @location(1) world:vec3<f32>,@location(2) @interpolate(flat) flags:u32,@location(3) @interpolate(flat) owner:u32,
}
fn face_geometry(index:u32,object:ObjectSettings)->FaceOutput {
 let vertex=positions[indices[index+object.range.x]];
 let placed=geometry(packed_position(vertex),unpack4x8unorm(vertex.color),vec3<f32>(0.),object);
 return FaceOutput(crop.transform*placed.position,faces[index/3u]+source.base,placed.world,placed.clip_flags,object.selection.y);
}
@vertex fn face_vertex(@builtin(vertex_index) index:u32)->FaceOutput {return face_geometry(index,object_at(0u));}
@vertex fn face_record_vertex(@builtin(vertex_index) index:u32,@location(0) row:u32)->FaceOutput {return face_geometry(index,object_at(row));}
@fragment
fn face_fragment(input: FaceOutput) -> @location(0) vec2<u32> {
    if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}
    return vec2<u32>(input.owner, (0x20000000u | input.face) + 1u);
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-03.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-04.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-05.rs"
```

</details>

### 3. `src/face_pick.rs`

Keep a shared-record source-face state beside the existing fallback. Share the source-face pipeline descriptor across both record resources. Use the shared storage contract or the current uniform contract. Adapt the same source-face shader through the existing record owner. Preserve source-face clipping after shader adaptation. Select the source-face record entry and existing draw-slot layout. Build both source-face states from one descriptor and shader. Accept the existing renderer record owner for original source-face reads. Use current records or the exact uniform fallback, retaining original face metadata and crop bindings.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::gpu_mesh::GpuMesh;
use wgpu::util::DeviceExt;

struct Row { group: wgpu::BindGroup, bytes: u64 }

pub struct Faces { pipeline: wgpu::RenderPipeline, rows: Vec<Option<Row>> }

fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let mut entries = Vec::new();
    for binding in 0..4 { entries.push(wgpu::BindGroupLayoutEntry { binding, visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer { ty: if binding == 3 { wgpu::BufferBindingType::Uniform }
            else { wgpu::BufferBindingType::Storage { read_only: true } }, has_dynamic_offset: false, min_binding_size: None }, count: None }); }
    entries.push(crate::tile_pipelines::entry(4,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Uniform));
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("source face metadata"), entries: &entries })
}

impl Faces {
    pub fn new(device: &wgpu::Device) -> Self {
        let storage = layout(device); let view = crate::gpu_layout::uniform(device, 80);
        let model = crate::gpu_layout::uniform(device, crate::object_record::BYTES as u64); let crop = crate::gpu_layout::uniform(device, 80);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("source face IDs"),
            bind_group_layouts: &[Some(&view), Some(&model), Some(&storage), Some(&crop)], immediate_size: 0 });
        let source = concat!(include_str!("triangle.wgsl"), "\n", include_str!("pick_crop.wgsl"), "\n", include_str!("face_pick.wgsl"));
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("source faces"), source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(source,2,4).into()) });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor { label: Some("source face IDs"), layout: Some(&layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some("face_vertex"), buffers: &[], compilation_options: Default::default() },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("face_fragment"), compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(), depth_stencil: Some(crate::depth::ink()),
            multisample: Default::default(), multiview_mask: None, cache: None });
        Self { pipeline, rows: Vec::new() }
    }

    pub fn set(&mut self, device: &wgpu::Device, meshes: &[GpuMesh], bases: &[(crate::scene::ObjectId, u32)],clipping:&wgpu::Buffer) {
        self.rows.clear();
        for (mesh, &(owner, base)) in meshes.iter().zip(bases) {
            assert_eq!(mesh.id, owner); let geometry = &mesh.geometry;
            if geometry.source.parts.triangles.is_empty() { self.rows.push(None); continue; }
            // Vertex pulling reads the existing index and position buffers; one small per-triangle stream adds original face identity.
            let bytes: Vec<u8> = geometry.source.parts.triangles.iter().flat_map(|n| n.to_ne_bytes()).collect();
            let faces = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("source triangle faces"), contents: &bytes, usage: wgpu::BufferUsages::STORAGE });
            let data: Vec<u8> = [base, 0, 0, 0].into_iter().flat_map(u32::to_ne_bytes).collect();
            let settings = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("source face base"), contents: &data, usage: wgpu::BufferUsages::UNIFORM });
            let(vertices,indices)=geometry.buffers();
            let buffers = [&vertices, &indices, &faces, &settings,clipping];
            let entries: Vec<_> = buffers.iter().enumerate().map(|(binding, buffer)| wgpu::BindGroupEntry { binding: binding as u32, resource: buffer.as_entire_binding() }).collect();
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("source face metadata"), layout: &self.pipeline.get_bind_group_layout(2), entries: &entries });
            self.rows.push(Some(Row { group, bytes: faces.size() + settings.size() }));
        }
        assert_eq!(meshes.len(), bases.len());
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>, meshes: &[GpuMesh], view: &wgpu::BindGroup, crop: &wgpu::BindGroup) {
        pass.set_pipeline(&self.pipeline); pass.set_bind_group(0, view, &[]); pass.set_bind_group(3, crop, &[]);
        for (mesh, row) in meshes.iter().zip(&self.rows) { if let Some(row) = row {
            mesh.bind_model(pass); pass.set_bind_group(2, &row.group, &[]); pass.draw(0..mesh.geometry.index_count, 0..1);
        } }
    }

    pub fn clear(&mut self) { self.rows.clear(); }
    pub fn ready(&self, count: usize) -> bool { self.rows.len() == count }

    pub fn bytes(&self) -> u64 { self.rows.iter().flatten().map(|row| row.bytes).sum() }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::gpu_mesh::GpuMesh;
use wgpu::util::DeviceExt;

struct Row { group: wgpu::BindGroup, bytes: u64 }

pub struct Faces { pipeline: wgpu::RenderPipeline,record_pipeline:wgpu::RenderPipeline, rows: Vec<Option<Row>> }

fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let mut entries = Vec::new();
    for binding in 0..4 { entries.push(wgpu::BindGroupLayoutEntry { binding, visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer { ty: if binding == 3 { wgpu::BufferBindingType::Uniform }
            else { wgpu::BufferBindingType::Storage { read_only: true } }, has_dynamic_offset: false, min_binding_size: None }, count: None }); }
    entries.push(crate::tile_pipelines::entry(4,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Uniform));
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("source face metadata"), entries: &entries })
}

impl Faces {
    pub fn new(device: &wgpu::Device) -> Self {
        let create=|shared| {let storage = layout(device); let view = crate::gpu_layout::uniform(device, 80);
        let model=if shared {crate::object_rows::layout(device)}else{crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64)}; let crop = crate::gpu_layout::uniform(device, 80);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("source face IDs"),
            bind_group_layouts: &[Some(&view), Some(&model), Some(&storage), Some(&crop)], immediate_size: 0 });
        let source = concat!(include_str!("triangle.wgsl"), "\n", include_str!("pick_crop.wgsl"), "\n", include_str!("face_pick.wgsl"));
        let source=if shared {crate::object_rows::source(source)}else{source.into()};let slots=[crate::draw_slots::layout()];
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("source faces"), source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(&source,2,4).into()) });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor { label: Some("source face IDs"), layout: Some(&layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some(if shared {"face_record_vertex"}else{"face_vertex"}), buffers: if shared {&slots}else{&[]}, compilation_options: Default::default() },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("face_fragment"), compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(), depth_stencil: Some(crate::depth::ink()),
            multisample: Default::default(), multiview_mask: None, cache: None });
        pipeline};let pipeline=create(false);let record_pipeline=create(true);Self {pipeline,record_pipeline,rows:Vec::new()}
    }

    pub fn set(&mut self, device: &wgpu::Device, meshes: &[GpuMesh], bases: &[(crate::scene::ObjectId, u32)],clipping:&wgpu::Buffer) {
        self.rows.clear();
        for (mesh, &(owner, base)) in meshes.iter().zip(bases) {
            assert_eq!(mesh.id, owner); let geometry = &mesh.geometry;
            if geometry.source.parts.triangles.is_empty() { self.rows.push(None); continue; }
            // Vertex pulling reads the existing index and position buffers; one small per-triangle stream adds original face identity.
            let bytes: Vec<u8> = geometry.source.parts.triangles.iter().flat_map(|n| n.to_ne_bytes()).collect();
            let faces = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("source triangle faces"), contents: &bytes, usage: wgpu::BufferUsages::STORAGE });
            let data: Vec<u8> = [base, 0, 0, 0].into_iter().flat_map(u32::to_ne_bytes).collect();
            let settings = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("source face base"), contents: &data, usage: wgpu::BufferUsages::UNIFORM });
            let(vertices,indices)=geometry.buffers();
            let buffers = [&vertices, &indices, &faces, &settings,clipping];
            let entries: Vec<_> = buffers.iter().enumerate().map(|(binding, buffer)| wgpu::BindGroupEntry { binding: binding as u32, resource: buffer.as_entire_binding() }).collect();
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some("source face metadata"), layout: &self.pipeline.get_bind_group_layout(2), entries: &entries });
            self.rows.push(Some(Row { group, bytes: faces.size() + settings.size() }));
        }
        assert_eq!(meshes.len(), bases.len());
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>, meshes: &[GpuMesh], view: &wgpu::BindGroup, crop: &wgpu::BindGroup,records:Option<&crate::instance_gpu::Lane>) {
        pass.set_pipeline(&self.pipeline); pass.set_bind_group(0, view, &[]); pass.set_bind_group(3, crop, &[]);
        for (mesh, row) in meshes.iter().zip(&self.rows) { if let Some(row) = row {
            if records.is_some_and(|records|records.bind_record(pass,mesh)){pass.set_pipeline(&self.record_pipeline);}else{pass.set_pipeline(&self.pipeline);mesh.bind_model(pass);}pass.set_bind_group(2,&row.group,&[]); pass.draw(0..mesh.geometry.index_count, 0..1);
        } }
    }

    pub fn clear(&mut self) { self.rows.clear(); }
    pub fn ready(&self, count: usize) -> bool { self.rows.len() == count }

    pub fn bytes(&self) -> u64 { self.rows.iter().flatten().map(|row| row.bytes).sum() }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-06.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-07.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-08.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-09.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-10.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-11.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-12.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-13.rs"
```

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-14.rs"
```

</details>

### 4. `src/pick_encode.rs`

Share the original object table with both first and source-face pick passes.

<details>
<summary>Locate the existing block</summary>

```rust
            faces.draw(&mut pass,meshes,view_group,crop_group);
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
            faces.draw(&mut pass,meshes,view_group,crop_group,records.map(|(rows,_)|rows));
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzw-read-source-faces-through-shared-records-15.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.9.5
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Original source-face reads consume current shared object records with exact owner and face identity.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for read source faces through shared records.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

A source face needs its original face metadata as well as placement and owner. Bind the existing shared object row and slot, then retain the source-face buffers and crop group. Separate record binding from geometry drawing so each consumer can keep its own geometry layout. Both source-face pipeline states share one descriptor and shader; their owner travels through the same unsigned vertex result.

Current object record → original source face plus original owner → existing face readback.



Why separate binding an object record from drawing its geometry?

The source-face pass has its own face metadata and crop groups. It can reuse the current object row and slot while preserving those consumer-specific bindings.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Select Face on ordinary and copied surfaces. Move and restore a copied original, then repeat the face pick. Inspect the unchanged relocated-range and original-hole face fixtures.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.9.5
npm --prefix ../session_tests run course -- save 20.9.5
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

First and source-face picks share current object records. Source face metadata remains separate; property consumers, primitive lookup and wider viewer parity remain open.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.9.5
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
