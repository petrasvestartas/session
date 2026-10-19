# 20.8.4 · Share instance object records across definitions

**Chapter 20 · Shared GPU rendering · Section 20.8 · Background and shared object records** · [Contents](contents.md)

**Typing: 17–33 minutes.** [Estimate](typing-load.md).

Replace each definition batch’s storage buffer with the shared object table. Slot row numbers now address that table across all definitions. Keep geometry ownership, the ordinary sentinel and primitive bases unchanged. Check the complete table against the device storage limit before adopting a new drawing.

## Type

Continue from [Prepare one shared object record table](96zzzi-prepare-one-shared-object-record-table.md). [Save or recover your work](recovery.md).

### 1. `src/instance_gpu.rs`

Remove batch-specific buffer creation. A batch retains shared geometry and its draw interval only. The lane owns one object-record table. Bind the existing instance layout to the shared table. Number records across definitions before publishing any GPU change. Reuse changed-record writes, current bindings and empty release. Every definition reads the same current object table. Count the shared allocation once. One shared table removes the old per-definition chunk key. Retain each original geometry definition once. Expose the current shared buffer for the authoring GPU readback.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::BTreeMap,rc::Rc};
use wgpu::util::DeviceExt;
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,records:wgpu::Buffer,group:wgpu::BindGroup,bytes:Vec<u8>,count:u32,first_slot:u32}
pub struct Lane {pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<(usize,usize),Batch>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn source()->String {
 let source=include_str!("triangle.wgsl")
  .replace("var<uniform> object_settings: ObjectSettings;","var<storage, read> objects: array<ObjectSettings>;")
  .replace("return object_settings;","return objects[index];");
 crate::clipping_gpu::source(&source,2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("shared instance records"),entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(BYTES as u64)},count:None}]});
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(format.into()),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        Self {pipeline,layout,batches:BTreeMap::new(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=(u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size)/BYTES as u64) as usize;
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<Record>)>=BTreeMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push(row.packed());}
        let mut old=std::mem::take(&mut self.batches);let mut batches=BTreeMap::new();let mut all_slots=Vec::new();
        for (source,(geometry,records)) in groups {
            for (part,records) in records.chunks(limit).enumerate() {
                let slots:Vec<crate::draw_slots::Slot>=records.iter().enumerate().map(|(row,record)|[row as u32,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]).collect();
                let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;all_slots.extend_from_slice(&slots);
                let key=(source,part);let bytes=records.iter().flatten().copied().collect::<Vec<_>>();
                let batch=if let Some(mut batch)=old.remove(&key).filter(|b|Rc::ptr_eq(&b.geometry,&geometry)&&b.records.size()>=bytes.len() as u64) {
                    batch.first_slot=first_slot;
                    if batch.bytes!=bytes {queue.write_buffer(&batch.records,0,&bytes);self.writes+=1;self.written_bytes+=bytes.len() as u64;batch.bytes=bytes;}batch.count=records.len() as u32;batch
                } else {
                    let buffer=device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:Some("independent source placements"),contents:&bytes,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST});let group=crate::gpu_layout::group(device,&self.layout,&buffer);self.writes+=1;self.written_bytes+=bytes.len() as u64;Batch {geometry:geometry.clone(),records:buffer,group,bytes,count:records.len() as u32,first_slot}
                };batches.insert(key,batch);
            }
        }
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        if batches.is_empty()&&!old.is_empty(){self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;Ok(())
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        for batch in self.batches.values() {pass.set_bind_group(1,&batch.group,&[]);self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.batches.values().map(|b|b.records.size()).sum::<u64>()+if self.batches.is_empty(){0}else{self.slots.allocated_bytes()},self.writes,self.written_bytes]}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::BTreeMap,rc::Rc};
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn source()->String {
 let source=include_str!("triangle.wgsl")
  .replace("var<uniform> object_settings: ObjectSettings;","var<storage, read> objects: array<ObjectSettings>;")
  .replace("return object_settings;","return objects[index];");
 crate::clipping_gpu::source(&source,2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("shared instance records"),entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(BYTES as u64)},count:None}]});
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(format.into()),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {pipeline,layout,batches:BTreeMap::new(),records,slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=(u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size)/BYTES as u64) as usize;
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<Record>)>=BTreeMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push(row.packed());}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let mut batches=BTreeMap::new();let mut all_slots=Vec::new();let mut all_records=Vec::with_capacity(count);
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for record in &records {
                let row=u32::try_from(all_records.len()).map_err(|_|"Instance record row")?;
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                all_records.push(*record);
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
        if self.records.written_bytes>0{self.writes+=1;self.written_bytes+=self.records.written_bytes;}
        if batches.is_empty()&&!self.batches.is_empty(){self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;Ok(())
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.records.allocated_bytes()+if self.batches.is_empty(){0}else{self.slots.allocated_bytes()},self.writes,self.written_bytes]}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-02.rs"
```

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-03.rs"
```

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-04.rs"
```

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-05.rs"
```

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-06.rs"
```

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-07.rs"
```

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-08.rs"
```

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-09.rs"
```

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-10.rs"
```

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-11.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/global_outline_tests.rs`

The second definition now addresses global row one, while the ordinary sentinel stays unchanged.

<details>
<summary>Locate the existing block</summary>

```text
assert_eq!(u32::from_ne_bytes(slots[16..20].try_into().unwrap()),0);
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-12.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/global_outline_tests.rs`

Supply exact mapped shared records and actual original-colored pixels in both placed halves; retain selection, opacity restoration and release assertions.

<details>
<summary>Locate the existing block</summary>

```text
 let baseline=crate::cap_gpu_tests::pixels(&gpu);let uploads=gpu.stats()[0];let owner=scene.objects()[0].id;
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzj-share-instance-object-records-across-definitions-13.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.8.4
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Original placed copies retain their colours, selection and picking. Their drawing batches now share one object-record buffer.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for share instance object records across definitions.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original placements → shared object records → global slot row → unchanged pulled surface shader.



Why must the second definition stop numbering rows from zero?

All definitions now bind the same array. A local zero would read the first definition’s placement, original ID and selection state.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Open the original instance specimen, then a second original definition. Select and move one copy, Undo and change opacity. Confirm that each definition retains its own placement and original picking; empty scene adoption releases both record and slot capacity.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.8.4
npm --prefix ../session_tests run course -- save 20.8.4
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This connects one instance object-record owner and removes per-definition storage buffers. Ordinary surface uniforms, stable row allocation and primitive lookup consumers still need their later connections.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.8.4
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
