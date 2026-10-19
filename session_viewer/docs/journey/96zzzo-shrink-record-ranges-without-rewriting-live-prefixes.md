# 20.8.9 · Shrink record ranges without rewriting live prefixes

**Chapter 20 · Shared GPU rendering · Section 20.8 · Background and shared object records** · [Contents](contents.md)

**Typing: 7–13 minutes.** [Estimate](typing-load.md).

Reuse one logical buffer truncate operation for object records and draw slots.

## Type

Continue from [Batch shared geometry with original ownership](96zzzn-batch-shared-geometry-with-original-ownership.md). [Save or recover your work](recovery.md).

### 1. `src/text_gpu_support.rs`

Shorten a logical record range without replacing its allocation or rewriting the live prefix.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn write_bytes_at(&self,ctx:&GpuCtx,at:u32,bytes:&[u8]) {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn truncate(&mut self,len:u32){assert!(len<=self.len);self.len=len;}

    pub fn write_bytes_at(&self,ctx:&GpuCtx,at:u32,bytes:&[u8]) {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzo-shrink-record-ranges-without-rewriting-live-prefixes-01.rs"
```

</details>

### 2. `src/object_rows.rs`

Reuse the existing changed-run scan on shrinking tables; write only changed live records.

<details>
<summary>Locate the existing block</summary>

```rust
  if rows.len()<self.previous.len() {
   self.buffer.reset();let data=bytes(rows);grew=self.buffer.append_bytes(ctx,&data);self.written_bytes=data.len()as u64;
  }else {
   for run in crate::draw_slots::changed_runs(&self.previous,rows) {
    let data=bytes(&rows[run.clone()]);self.buffer.write_bytes_at(ctx,run.start as u32,&data);self.written_bytes+=data.len()as u64;
   }
   let tail=bytes(&rows[self.previous.len()..]);
   if !tail.is_empty(){grew=self.buffer.append_bytes(ctx,&tail);self.written_bytes+=tail.len()as u64;}
  }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
  if rows.len()<self.previous.len(){self.buffer.truncate(rows.len()as u32);}
  for run in crate::draw_slots::changed_runs(&self.previous,rows) {
   let data=bytes(&rows[run.clone()]);self.buffer.write_bytes_at(ctx,run.start as u32,&data);self.written_bytes+=data.len()as u64;
  }
  let tail=bytes(&rows[self.previous.len().min(rows.len())..]);
  if !tail.is_empty(){grew=self.buffer.append_bytes(ctx,&tail);self.written_bytes+=tail.len()as u64;}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzo-shrink-record-ranges-without-rewriting-live-prefixes-02.rs"
```

</details>

### 3. `src/draw_slots.rs`

Keep slot zero and unchanged instance slots while shortening their logical range.

<details>
<summary>Locate the existing block</summary>

```rust
   self.buffer.reset();self.buffer.append_bytes(ctx,&bytes(&[[OWN_ROW,0]]));self.buffer.append_bytes(ctx,&bytes(rows));
   self.written_bytes=(rows.len()as u64+1)*8;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
   self.buffer.truncate(u32::try_from(rows.len()+1).expect("Bounded draw slots"));
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzo-shrink-record-ranges-without-rewriting-live-prefixes-03.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/object_rows.rs`

Supply mapped live prefixes, exact partial-write counts and current immutable bindings while shrinking.

<details>
<summary>Locate the existing block</summary>

```text
 rows.truncate(2);table.set(&ctx,&layout,&rows).unwrap();assert_eq!(table.allocated_bytes(),capacity);check(&table,&rows);
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzo-shrink-record-ranges-without-rewriting-live-prefixes-04.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/draw_slots_tests.rs`

Supply exact retained slot bytes and one changed high-bit slot through a second shrink.

<details>
<summary>Locate the existing block</summary>

```text
 rows.truncate(2);let capacity=slots.allocated_bytes();slots.set(&ctx,&rows);assert_eq!(slots.written_bytes,24);assert_eq!(slots.allocated_bytes(),capacity);check(&slots,&rows);
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzo-shrink-record-ranges-without-rewriting-live-prefixes-05.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.8.9
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The same original scene remains visible. Removing records can now preserve unchanged live bytes and current GPU bindings.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for shrink record ranges without rewriting live prefixes.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Removing a last record changes the live range without changing its prefix. Keep the allocation and immutable binding, shorten the logical count, and reuse the changed-run scan for records that really changed. An empty object table still follows the existing full-release path; ordinary slot zero remains available.

Shorter live range → same allocation → changed live runs only → current draw count.



Does logical truncation erase the reserved tail?

No. It changes the live count. Drawing must use that count; complete scene release still drops the old allocation through the existing release path.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Hide and show a copied original, restore selection and compare the scene. Inspect the supplied mapped-byte checks for zero writes on an unchanged shortened prefix and one changed high-bit record.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.8.9
npm --prefix ../session_tests run course -- save 20.8.9
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Shared record and slot tables now shrink through one logical buffer operation. Stable object-row allocation, primitive slot drawing and lookup consumers remain unfinished.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.8.9
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
