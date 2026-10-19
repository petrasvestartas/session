# 20.5.1 · Grow buffers without losing rows

**Chapter 20 · Shared GPU rendering · Section 20.5 · Shared mesh arenas** · [Contents](contents.md)

**Typing: 26–51 minutes.** [Estimate](typing-load.md).

Growing a shared GPU arena must keep rows already uploaded.

## Type

Continue from [Start with current browser display defaults](96zzf-start-with-current-browser-display-defaults.md). [Save or recover your work](recovery.md).

### 1. `src/text_gpu_support.rs`

Grow the existing owner in rows, preserve live bytes on the GPU and share one append algorithm.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct GrowBuf {pub buf:wgpu::Buffer,stride:u64,len:u32,label:&'static str,usage:wgpu::BufferUsages}
impl GrowBuf {
 pub fn new(ctx:&GpuCtx,label:&'static str,stride:u64,usage:wgpu::BufferUsages)->Self {Self {buf:ctx.device.create_buffer(&wgpu::BufferDescriptor {label:Some(label),size:stride,usage,mapped_at_creation:false}),stride,len:0,label,usage}}
 pub fn append<const N:usize>(&mut self,ctx:&GpuCtx,vertices:&[[f32;N]]) {assert_eq!(self.stride,N as u64*4);if vertices.is_empty() {return;}
  let count=u32::try_from(vertices.len()).expect("Bounded text vertices");let size=(self.len as u64+count as u64)*self.stride;
  if size>self.buf.size() {assert_eq!(self.len,0,"Text upload starts at an empty buffer");self.buf=ctx.device.create_buffer(&wgpu::BufferDescriptor {label:Some(self.label),size:size.next_power_of_two(),usage:self.usage,mapped_at_creation:false});}
  let bytes:Vec<u8>=vertices.iter().flatten().flat_map(|n|n.to_ne_bytes()).collect();ctx.queue.write_buffer(&self.buf,self.len as u64*self.stride,&bytes);self.len+=count;
 }
 pub fn reset(&mut self) {self.len=0;}
 pub fn release(&mut self,ctx:&GpuCtx) {*self=Self::new(ctx,self.label,self.stride,self.usage);}
 pub fn len(&self)->u32 {self.len}
 pub fn is_empty(&self)->bool {self.len==0}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzg-grow-buffers-without-losing-rows-1.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/growing_buffer_tests.rs`

Supply real mapped GPU bytes for nonempty growth, empty appends, reuse, release and the existing float adapter.

Copy this supplied file:

```text
--8<-- "journey/code/96zzg-grow-buffers-without-losing-rows-2.rs"
```

</details>

### 3. `src/lib.rs`

Register the supplied GPU-growth readback fixture.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod text_gpu_support;
```

</details>

After that block, type:

```rust
#[cfg(test)]
mod growing_buffer_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzg-grow-buffers-without-losing-rows-3.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.5.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

One existing GPU buffer owner preserves old records, supports aligned packed bytes and bounds growth by device limits.

### Recorded visual result

**Previous native render — current verification pending.**

This supporting frame has no instances; use the supplied original Session specimen for the browser exercise.

![Native renderer output for grow buffers without losing rows.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Extend the existing text buffer owner to append any aligned record bytes, preserve live rows with a GPU copy and respect device limits. Text keeps the same owner; the compact mesh arena will reuse it next. Mesh storage is still per geometry at this preparatory checkpoint.

Follow existing rows through buffer replacement, an ordered GPU copy and the next append..



Why submit the old-row copy before writing the appended rows?

Queue order preserves earlier writes before the copy and places the new append after it. No CPU readback is needed for growth.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Create and move original text repeatedly, then clear it and add it again. Check the existing text counters and pixels. The supplied GPU fixture separately verifies the exact bytes through multiple nonempty growths and reset/release.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.5.1
npm --prefix ../session_tests run course -- save 20.5.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This extends the existing text GrowBuf rather than introducing another growth owner. It prepares the shared compact mesh arena, which is not connected yet. Text continues to use its float-record adapter.

The supplied GPU fixture maps exact bytes after nonempty growth, reset, reuse and release. Chrome reuses complete focused original-text assertions at DPR1/2.

Actual selected original CAD text after complete focused text, display-default and density checks.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.5.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
