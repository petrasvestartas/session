# 20.11.4 · Prepare one plan for draw slots and lookup

**Chapter 20 · Shared GPU rendering · Section 20.11 · Retained allocation and lookup projection** · [Contents](contents.md)

**Typing: 9–18 minutes.** [Estimate](typing-load.md).

Publish one retained plan through the existing draw-slot and integer-texture owners.

## Type

Continue from [Retain live allocator member runs](96zzzzk-retain-live-allocator-member-runs.md). [Save or recover your work](recovery.md).

### 1. `src/draw_slots.rs`

Publish one retained plan through both owners. Merge slot boundary changes with allocator reports and refresh the lookup header after shrinking; reuse the existing range coalescer.

<details>
<summary>Locate the existing block</summary>

```rust
 pub fn clear(&mut self,ctx:&GpuCtx,free:bool){if free {self.buffer.release(ctx);}self.set(ctx,&[]);}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
 pub fn publish(&mut self,ctx:&GpuCtx,space:&crate::slot_space::SlotSpace,change:&crate::slot_space::Change,table:&mut crate::primitive_slot_table::Table)->bool {
  let rows=(1..space.end()).map(|slot|space.local_slot(slot)).collect::<Vec<_>>();
  assert!(change.slots.iter().all(|range|range.start<=range.end&&range.end<=space.end()));
  let mut ranges=change.slots.clone();ranges.extend(changed_runs(&self.previous,&rows).into_iter().map(|run|(run.start+1)as u32..(run.end+1)as u32));
  if rows.len()!=self.previous.len(){ranges.push(0..1);}
  let change=crate::slot_space::Change {all:change.all,slots:crate::slot_space::coalesce(ranges,space.end())};
  if rows.is_empty()&&self.instances()>0{self.clear(ctx,true);}else{self.set(ctx,&rows);}
  table.publish(ctx,space,&change)
 }
 pub fn clear(&mut self,ctx:&GpuCtx,free:bool){if free {self.buffer.release(ctx);}self.set(ctx,&[]);}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzl-prepare-one-plan-for-draw-slots-and-lookup-01.rs"
```

</details>

### 2. `src/slot_space.rs`

Expose the range coalescer already written so publication shares its bounds and merging instead of duplicating it.

<details>
<summary>Locate the existing block</summary>

```rust
fn coalesce(ranges: Vec<Range<u32>>, end: u32) -> Vec<Range<u32>> {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub(crate) fn coalesce(ranges: Vec<Range<u32>>, end: u32) -> Vec<Range<u32>> {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzl-prepare-one-plan-for-draw-slots-and-lookup-03.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/primitive_slot_table_tests.rs`

Supply exact mapped writes and actual lookup shader results through high IDs, hole splitting/reuse, unchanged neighbours, shrinking headers, arena following and release.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{primitive_slot_table::Table,text_gpu_support::GpuCtx};
fn read(ctx:&GpuCtx,table:&Table)->Vec<[u32;4]>{
 let size=table.texture.size();let pitch=(size.width*16+255)/256*256;let bytes=u64::from(pitch)*u64::from(size.height);
 let output=ctx.device.create_buffer(&wgpu::BufferDescriptor {label:Some("slot texture readback"),size:bytes,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});let mut encoder=ctx.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(table.texture.as_image_copy(),wgpu::TexelCopyBufferInfo {buffer:&output,layout:wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(size.height)}},size);ctx.queue.submit([encoder.finish()]);
 let(tx,rx)=std::sync::mpsc::channel();output.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());ctx.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();let mapped=output.slice(..).get_mapped_range();let result=mapped.chunks_exact(pitch as usize).flat_map(|row|row[..size.width as usize*16].chunks_exact(16).map(|texel|std::array::from_fn(|i|u32::from_le_bytes(texel[i*4..i*4+4].try_into().unwrap())))).collect();drop(mapped);output.unmap();result
}
#[test]
fn actual_integer_slot_texture_preserves_ids_across_partial_row_writes_and_release(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut table=Table::new(&ctx);assert_eq!(table.allocated_bytes(),16);assert_eq!(read(&ctx,&table),vec![[0;4]]);
 let mut wanted=(0..1050).map(|i|[i,0x80000000+i,16_777_217+i,3]).collect::<Vec<_>>();table.write(&ctx,&wanted);assert_eq!([table.texture.width(),table.texture.height()],[1024,2]);assert_eq!(table.texture.format(),wgpu::TextureFormat::Rgba32Uint);let texture=table.texture.clone();wanted.resize(2048,[0;4]);assert_eq!(read(&ctx,&table),wanted);
 let changed=[[7,u32::MAX-1,31,9],[11,0x80000000,37,12],[13,16_777_217,41,15]];assert!(table.write_at(&ctx,1023,&changed));wanted[1023..1026].copy_from_slice(&changed);assert_eq!(table.texture,texture);assert_eq!(read(&ctx,&table),wanted);
 assert!(!table.write_at(&ctx,u32::MAX,&changed));assert!(!table.write_at(&ctx,2047,&changed));assert!(table.write_at(&ctx,2048,&[]));assert_eq!(read(&ctx,&table),wanted);
 table.write(&ctx,&[]);assert_eq!(table.allocated_bytes(),16);assert_eq!(read(&ctx,&table),vec![[0;4]]);assert!(!table.write_at(&ctx,0,&changed));assert_eq!(read(&ctx,&table),vec![[0;4]]);
}

#[test]
fn actual_allocator_publication_keeps_high_rows_free_slots_neighbours_and_release() {
 use crate::slot_space::{Definition,SlotSpace};
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut table=Table::new(&ctx);let mut space=SlotSpace::default();
 let mut a=[7,0x80000001];let b=[u32::MAX-1];
 fn definitions<'a>(a:&'a[u32],b:&'a[u32])->Vec<Definition<'a>>{vec![Definition {key:1,rows:a,faces:30..36,pipes:0..0,ribbons:0..0},Definition {key:2,rows:b,faces:60..63,pipes:0..0,ribbons:0..0}]}
 let change=space.reset(&definitions(&a,&b),100);assert!(table.publish(&ctx,&space,&change));let texture=table.texture.clone();let bytes=read(&ctx,&table);assert_eq!(&bytes[..4],&[[3,100,0,0],[7,100,10,2],[0x80000001,102,10,2],[u32::MAX-1,104,20,1]]);
 a[0]=0x80000007;let change=space.update(&definitions(&a,&b),&[(1,0)],100);assert!(!change.all);assert!(!table.publish(&ctx,&space,&change));assert_eq!(table.texture,texture);let next=read(&ctx,&table);assert_eq!(next[1],[0x80000007,100,10,2]);assert_eq!(&next[2..],&bytes[2..]);
 let remaining=[Definition {key:2,rows:&b,faces:60..63,pipes:0..0,ribbons:0..0}];let change=space.update(&remaining,&[],100);table.publish(&ctx,&space,&change);let next=read(&ctx,&table);assert_eq!(next[1][3],0);assert_eq!(next[2][3],0);assert_eq!(next[3],[u32::MAX-1,104,20,1]);
 let change=space.reset(&[],0);assert!(table.publish(&ctx,&space,&change));assert_eq!(table.allocated_bytes(),16);assert_eq!(read(&ctx,&table),vec![[0;4]]);
}

#[test]
fn actual_slot_shader_resolves_arena_instances_holes_high_rows_and_empty_table() {
 use wgpu::util::DeviceExt;use crate::slot_space::{Definition,SlotSpace};
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut table=Table::new(&ctx);let mut space=SlotSpace::default();
 let rows=[7,0x80000001];let other=[u32::MAX-1];let definitions=[Definition {key:1,rows:&rows,faces:30..36,pipes:0..0,ribbons:0..0},Definition {key:2,rows:&other,faces:60..63,pipes:0..0,ribbons:0..0}];let change=space.reset(&definitions,100);table.publish(&ctx,&space,&change);
 let source=format!("@group(0) @binding(0) var slot_table: texture_2d<u32>;\n@group(0) @binding(1) var<storage,read_write> output: array<vec2<u32>>;\n{}\n@compute @workgroup_size(1) fn run(@builtin(global_invocation_id) p: vec3<u32>) {{ let ids=array<u32,8>(0u,100u,101u,102u,103u,104u,105u,106u);output[p.x]=placed_triangle(ids[p.x],100u); }}",include_str!("primitive_slot_lookup.wgsl"));
 let shader=ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("actual integer slot consumer"),source:wgpu::ShaderSource::Wgsl(source.into())});let pipeline=ctx.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {label:None,layout:None,module:&shader,entry_point:Some("run"),compilation_options:Default::default(),cache:None});
 let output=ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:None,contents:&[0;64],usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_SRC});let mapped=ctx.device.create_buffer(&wgpu::BufferDescriptor {label:None,size:64,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});
 let resolve=|table:&Table| {let group=ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {label:None,layout:&pipeline.get_bind_group_layout(0),entries:&[wgpu::BindGroupEntry {binding:0,resource:wgpu::BindingResource::TextureView(&table.view)},wgpu::BindGroupEntry {binding:1,resource:output.as_entire_binding()}]});let mut encoder=ctx.device.create_command_encoder(&Default::default());{let mut pass=encoder.begin_compute_pass(&Default::default());pass.set_pipeline(&pipeline);pass.set_bind_group(0,&group,&[]);pass.dispatch_workgroups(8,1,1);}encoder.copy_buffer_to_buffer(&output,0,&mapped,0,64);ctx.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();mapped.slice(..).map_async(wgpu::MapMode::Read,move|r|tx.send(r).unwrap());ctx.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();let bytes=mapped.slice(..).get_mapped_range();let result=bytes.chunks_exact(8).map(|v|[u32::from_le_bytes(v[..4].try_into().unwrap()),u32::from_le_bytes(v[4..].try_into().unwrap())]).collect::<Vec<_>>();drop(bytes);mapped.unmap();result};
 let none=[u32::MAX;2];assert_eq!(resolve(&table),vec![none,[99,u32::MAX],[10,7],[11,7],[10,0x80000001],[11,0x80000001],[20,u32::MAX-1],none]);
 let mut cross=vec![[0,100,0,0];1026];cross[0]=[1025,100,0,0];cross[1024]=[0x80000001,100,30,2];cross[1025]=[u32::MAX-1,104,40,1];table.write(&ctx,&cross);assert_eq!(resolve(&table),vec![none,[99,u32::MAX],[30,0x80000001],[31,0x80000001],none,none,[40,u32::MAX-1],none]);table.write(&ctx,&(0..space.end()).map(|i|space.texel(i)).collect::<Vec<_>>());
 let remaining=[Definition {key:2,rows:&other,faces:60..63,pipes:0..0,ribbons:0..0}];let change=space.update(&remaining,&[],100);table.publish(&ctx,&space,&change);let values=resolve(&table);assert_eq!(&values[2..6],&[none;4]);assert_eq!(values[6],[20,u32::MAX-1]);
 let change=space.reset(&[],0);table.publish(&ctx,&space,&change);let values=resolve(&table);assert_eq!(values[1],[99,u32::MAX]);assert_eq!(values[0],none);assert!(values[2..].iter().all(|v|*v==none));
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzl-prepare-one-plan-for-draw-slots-and-lookup-02.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.11.4
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

One publication operation prepares matching local slots and lookup texels; live rendering remains unchanged.

### Recorded visual result

**Native render check — not a browser screenshot.**

![Native renderer output for prepare one plan for draw slots and lookup.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*Read directly from this checkpoint’s GPU texture. Browser controls and event delivery remain unverified until the browser check passes.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The local slot accessor and lookup texel describe the same physical interval. Publish both through the existing changed-write helpers. A reused hole can change an inactive boundary that binary search still reads; merge those slot differences with the allocator report using its existing coalescer. Update the header when the retained range shrinks. Preserve the own-row sentinel and release empty plans. Live adoption follows in the next lesson.

One retained plan → local draw-slot bytes and integer lookup texels.



Why update an inactive hole boundary?

The lookup binary search reads interval starts before checking whether a slot is active. A stale boundary can hide a newly reused live interval.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a changed member, a retired hole and a replacement member. Compare the local base with the lookup start before and after arena growth.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.11.4
npm --prefix ../session_tests run course -- save 20.11.4
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

GPU publication is prepared and independently checked. Live adoption, primitive consumers, reclamation and broader viewer parity remain open.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.11.4
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
