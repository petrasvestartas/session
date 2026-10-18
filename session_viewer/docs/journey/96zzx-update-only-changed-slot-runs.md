# 20.7.4 · Update only changed slot runs

**Chapter 20 · Shared GPU rendering · Section 20.7 · Draw slots and primitive lookup** · [Contents](contents.md)

**Typing: 15–30 minutes.** [Estimate](typing-load.md).

A selection change can update one slot while its shared geometry remains untouched. Add bounded writes to the existing growing buffer. Compare slot rows, group adjacent changes and upload those runs. Keep the sentinel and reuse capacity when the active count shrinks.

## Type

Continue from [Reload scenes through the existing publication owner](96zzw-reload-scenes-through-the-existing-publication-owner.md). [Save or recover your work](recovery.md).

### 1. `src/text_gpu_support.rs`

Write aligned existing rows without changing capacity or the active count.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn append<const N:usize>(&mut self,ctx:&GpuCtx,vertices:&[[f32;N]]) {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn write_bytes_at(&self,ctx:&GpuCtx,at:u32,bytes:&[u8]) {
        assert_eq!(bytes.len()as u64%self.stride,0);
        let rows=bytes.len()as u64/self.stride;
        assert!(u64::from(at)+rows<=u64::from(self.len));
        if !bytes.is_empty(){ctx.queue.write_buffer(&self.buf,u64::from(at)*self.stride,bytes);}
    }

    pub fn append<const N:usize>(&mut self,ctx:&GpuCtx,vertices:&[[f32;N]]) {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzx-update-only-changed-slot-runs-1.rs"
```

</details>

### 2. `src/draw_slots.rs`

Report the bytes actually uploaded by this slot update. Initialize upload accounting. Patch changed contiguous runs; append new rows and retain capacity on shrink.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::text_gpu_support::{GpuCtx,GrowBuf};
pub const OWN_ROW:u32=u32::MAX;
pub fn sentinel_shader()->String{format!("const OWN_ROW:u32={OWN_ROW}u;")}
pub type Slot=[u32;2];
const ATTRIBUTES:[wgpu::VertexAttribute;2]=[
 wgpu::VertexAttribute {offset:0,shader_location:0,format:wgpu::VertexFormat::Uint32},
 wgpu::VertexAttribute {offset:4,shader_location:1,format:wgpu::VertexFormat::Uint32},
];
pub fn layout()->wgpu::VertexBufferLayout<'static>{wgpu::VertexBufferLayout {array_stride:8,step_mode:wgpu::VertexStepMode::Instance,attributes:&ATTRIBUTES}}
pub struct Slots {pub(crate) buffer:GrowBuf,previous:Vec<Slot>}
impl Slots {
 pub fn new(ctx:&GpuCtx)->Self {
  let mut slots=Self {buffer:GrowBuf::new(ctx,"instance draw slots",8,wgpu::BufferUsages::VERTEX),previous:Vec::new()};
  slots.set(ctx,&[]);slots
 }
 pub fn set(&mut self,ctx:&GpuCtx,rows:&[Slot])->bool{
  if self.buffer.len()>0&&self.previous==rows{return false;}
  let bytes:Vec<u8>=std::iter::once([OWN_ROW,0]).chain(rows.iter().copied()).flatten().flat_map(u32::to_ne_bytes).collect();
  self.buffer.reset();self.buffer.append_bytes(ctx,&bytes);
  self.previous.clear();self.previous.extend_from_slice(rows);true
 }
 pub fn clear(&mut self,ctx:&GpuCtx,free:bool){if free {self.buffer.release(ctx);}self.set(ctx,&[]);}
 pub fn bind(&self,pass:&mut wgpu::RenderPass<'_>,slot:u32,first:u32){
  assert!(first<self.buffer.len());pass.set_vertex_buffer(slot,self.buffer.buf.slice(u64::from(first)*8..));
 }
 pub fn instances(&self)->u32{self.buffer.len()-1}
 pub fn allocated_bytes(&self)->u64{self.buffer.buf.size()}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::text_gpu_support::{GpuCtx,GrowBuf};
pub const OWN_ROW:u32=u32::MAX;
pub fn sentinel_shader()->String{format!("const OWN_ROW:u32={OWN_ROW}u;")}
pub type Slot=[u32;2];
const ATTRIBUTES:[wgpu::VertexAttribute;2]=[
 wgpu::VertexAttribute {offset:0,shader_location:0,format:wgpu::VertexFormat::Uint32},
 wgpu::VertexAttribute {offset:4,shader_location:1,format:wgpu::VertexFormat::Uint32},
];
pub fn layout()->wgpu::VertexBufferLayout<'static>{wgpu::VertexBufferLayout {array_stride:8,step_mode:wgpu::VertexStepMode::Instance,attributes:&ATTRIBUTES}}
pub struct Slots {pub(crate) buffer:GrowBuf,previous:Vec<Slot>,pub written_bytes:u64}
impl Slots {
 pub fn new(ctx:&GpuCtx)->Self {
  let mut slots=Self {buffer:GrowBuf::new(ctx,"instance draw slots",8,wgpu::BufferUsages::VERTEX),previous:Vec::new(),written_bytes:0};
  slots.set(ctx,&[]);slots
 }
 pub fn set(&mut self,ctx:&GpuCtx,rows:&[Slot])->bool{
  self.written_bytes=0;
  let bytes=|rows:&[Slot]|rows.iter().flatten().flat_map(|v|v.to_ne_bytes()).collect::<Vec<_>>();
  if self.buffer.len()==0 {
   self.buffer.append_bytes(ctx,&bytes(&[[OWN_ROW,0]]));self.written_bytes=8;
  }
  let inside=self.previous.len().min(rows.len());let mut at=0;
  while at<inside {
   if self.previous[at]==rows[at]{at+=1;continue;}
   let first=at;while at<inside&&self.previous[at]!=rows[at]{at+=1;}
   self.buffer.write_bytes_at(ctx,(first+1)as u32,&bytes(&rows[first..at]));
   self.written_bytes+=(at-first)as u64*8;
  }
  if rows.len()>self.previous.len(){
   let tail=bytes(&rows[inside..]);self.buffer.append_bytes(ctx,&tail);self.written_bytes+=tail.len()as u64;
  }else if rows.len()<self.previous.len(){
   self.buffer.reset();self.buffer.append_bytes(ctx,&bytes(&[[OWN_ROW,0]]));self.buffer.append_bytes(ctx,&bytes(rows));
   self.written_bytes=(rows.len()as u64+1)*8;
  }
  self.previous.clear();self.previous.extend_from_slice(rows);self.written_bytes>0
 }
 pub fn clear(&mut self,ctx:&GpuCtx,free:bool){if free {self.buffer.release(ctx);}self.set(ctx,&[]);}
 pub fn bind(&self,pass:&mut wgpu::RenderPass<'_>,slot:u32,first:u32){
  assert!(first<self.buffer.len());pass.set_vertex_buffer(slot,self.buffer.buf.slice(u64::from(first)*8..));
 }
 pub fn instances(&self)->u32{self.buffer.len()-1}
 pub fn allocated_bytes(&self)->u64{self.buffer.buf.size()}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzx-update-only-changed-slot-runs-2.rs"
```

```text
--8<-- "journey/code/96zzx-update-only-changed-slot-runs-3.rs"
```

```text
--8<-- "journey/code/96zzx-update-only-changed-slot-runs-4.rs"
```

</details>

### 3. `src/instance_gpu.rs`

Count partial slot uploads rather than every slot in the batch.

<details>
<summary>Locate the existing block</summary>

```rust
                    if batch.slots.set(&ctx,&slots){self.writes+=1;self.written_bytes+=(slots.len()as u64+1)*8;}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
                    if batch.slots.set(&ctx,&slots){self.writes+=1;self.written_bytes+=batch.slots.written_bytes;}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzx-update-only-changed-slot-runs-5.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/draw_slots_tests.rs`

Supply real GPU readback checks for partial writes, unchanged rows, growth, shrink and bounds.

<details>
<summary>Locate the existing block</summary>

```text
(slots.instances(),0);assert_eq!(slots.allocated_bytes(),8);
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&slots.buffer.buf,8),bytes(&[[u32::MAX,0]]));
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzx-update-only-changed-slot-runs-6.rs" skip=160
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.7.4
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for update only changed slot runs.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



changed rows → contiguous runs → queue.write_buffer → unchanged draw slots.



Why compare rows before writing the buffer?

An unchanged row needs no upload. Grouping adjacent changes also avoids one queue write per changed row.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Select and deselect one placed object. Its drawing changes without uploading its source geometry. Repeating the same selection should add no slot writes.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.7.4
npm --prefix ../session_tests run course -- save 20.7.4
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This adds partial slot uploads to the existing per-batch owner. A shared allocator and stable global slot ranges remain separate work.



[Full validation scope](release.md).

</details>
