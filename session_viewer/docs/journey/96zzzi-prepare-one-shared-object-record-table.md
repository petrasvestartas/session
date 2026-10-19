# 20.8.3 · Prepare one shared object record table

**Chapter 20 · Shared GPU rendering · Section 20.8 · Background and shared object records** · [Contents](contents.md)

**Typing: 22–44 minutes.** [Estimate](typing-load.md).

Prepare one growable storage table without duplicating slot upload logic.

## Type

Continue from [Share changed record runs](96zzzh-share-changed-record-runs.md). [Save or recover your work](recovery.md).

### 1. `src/object_rows.rs`

Reuse the aligned record, growing buffer and shared changed-run helper.

Create the file and type:

```rust
--8<-- "journey/code/96zzzi-prepare-one-shared-object-record-table-01.rs"
```

### 2. `src/lib.rs`

Expose the preparatory owner.

<details>
<summary>Locate the existing block</summary>

```rust
mod draw_slots;
```

</details>

After that block, type:

```rust
mod object_rows;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzi-prepare-one-shared-object-record-table-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/object_rows.rs`

Supply mapped GPU checks for unchanged neighbors, high-bit fields, growth, binding replacement and release.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{object_record::{BYTES,Record},text_gpu_support::{GpuCtx,GrowBuf}};
pub struct Rows {
 pub(crate) buffer:GrowBuf,
 group:wgpu::BindGroup,
 previous:Vec<Record>,
 pub written_bytes:u64,
}
impl Rows {
 pub fn new(ctx:&GpuCtx,layout:&wgpu::BindGroupLayout)->Self {
  let buffer=GrowBuf::new(ctx,"shared object records",BYTES as u64,wgpu::BufferUsages::STORAGE);
  let group=crate::gpu_layout::group(&ctx.device,layout,&buffer.buf);
  Self {buffer,group,previous:Vec::new(),written_bytes:0}
 }
 pub fn set(&mut self,ctx:&GpuCtx,layout:&wgpu::BindGroupLayout,rows:&[Record])->Result<bool,&'static str> {
  let limits=ctx.device.limits();let limit=limits.max_buffer_size.min(u64::from(limits.max_storage_buffer_binding_size));
  if rows.len()as u64>limit/BYTES as u64||u32::try_from(rows.len()).is_err(){return Err("Object records exceed device limits");}
  self.written_bytes=0;
  if rows.is_empty() {
   if !self.previous.is_empty(){self.buffer.release(ctx);self.group=crate::gpu_layout::group(&ctx.device,layout,&self.buffer.buf);self.previous=Vec::new();}
   return Ok(false);
  }
  let bytes=|rows:&[Record]|rows.iter().flatten().copied().collect::<Vec<_>>();
  let mut grew=false;
  if rows.len()<self.previous.len() {
   self.buffer.reset();let data=bytes(rows);grew=self.buffer.append_bytes(ctx,&data);self.written_bytes=data.len()as u64;
  }else {
   for run in crate::draw_slots::changed_runs(&self.previous,rows) {
    let data=bytes(&rows[run.clone()]);self.buffer.write_bytes_at(ctx,run.start as u32,&data);self.written_bytes+=data.len()as u64;
   }
   let tail=bytes(&rows[self.previous.len()..]);
   if !tail.is_empty(){grew=self.buffer.append_bytes(ctx,&tail);self.written_bytes+=tail.len()as u64;}
  }
  if grew{self.group=crate::gpu_layout::group(&ctx.device,layout,&self.buffer.buf);}
  self.previous.clear();self.previous.extend_from_slice(rows);Ok(self.written_bytes>0)
 }
 pub fn bind(&self,pass:&mut wgpu::RenderPass<'_>){pass.set_bind_group(1,&self.group,&[]);}
 pub fn len(&self)->u32{self.previous.len()as u32}
 pub fn allocated_bytes(&self)->u64{if self.previous.is_empty(){0}else{self.buffer.buf.size()}}
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzi-prepare-one-shared-object-record-table-03.rs" skip=2140
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.8.3
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The browser scene is unchanged. The growing object-record table is ready for its first drawing consumer.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for prepare one shared object record table.](../screenshots/journey/96zzzg-clear-the-current-arctic-background.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.8.1 · Clear the current Arctic background](96zzzg-clear-the-current-arctic-background.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Use the existing aligned object record and growing GPU buffer. Update changed records through the shared scan, append new records, rebuild the immutable binding after buffer growth and release scene capacity when empty. Validate device limits before mutating either owner. This table is preparatory; the next lesson connects instance drawing.

packed object records → changed runs → growing storage buffer → current immutable binding.



Why replace a bind group after buffer growth?

A bind group retains the buffer it was created with. Growing the owner does not change that retained resource.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Keep the original browser scene running. This preparatory table adds no drawing or allocation until it is connected; inspect the supplied GPU readback showing retained neighbors, high-bit ID bytes and changed binding after growth.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.8.3
npm --prefix ../session_tests run course -- save 20.8.3
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The maintained viewer shares object records across draws. This prepares that owner; existing instance batches still draw through their earlier buffers.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.8.3
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
