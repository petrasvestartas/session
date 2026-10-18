# 20.9.1 · Allocate stable object record rows

**Chapter 20 · Shared GPU rendering · Section 20.9 · Stable records and original picking** · [Contents](contents.md)

**Typing: 8–16 minutes.** [Estimate](typing-load.md).

Prepare bounded row reuse without moving surviving originals.

## Type

Continue from [Share transparent instance records](96zzzr-share-transparent-instance-records.md). [Save or recover your work](recovery.md).

### 1. `src/instance_gpu.rs`

Retain surviving original IDs, reuse the lowest free row and enforce the shared device bound.

<details>
<summary>Locate the existing block</summary>

```rust
pub(crate) fn source()->String {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub(crate) fn stable_rows(previous:&std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,ids:&[crate::scene::ObjectId],limit:usize)->Result<std::collections::HashMap<crate::scene::ObjectId,u32>,&'static str> {
 let mut rows=std::collections::HashMap::new();let mut used=std::collections::HashSet::new();
 for id in ids {if let Some(&(_,row))=previous.get(id) {if row as usize>=limit||!used.insert(row){return Err("Invalid retained object row");}rows.insert(*id,row);}}
 let mut next=0usize;
 for id in ids {if rows.contains_key(id){continue;}while next<limit&&used.contains(&(next as u32)){next+=1;}if next>=limit||u32::try_from(next).is_err(){return Err("Object records exceed device limits");}let row=next as u32;rows.insert(*id,row);used.insert(row);}
 Ok(rows)
}
pub(crate) fn source()->String {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzs-allocate-stable-object-record-rows-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/instance_gpu.rs`

Supply original-ID reorder, retired-hole reuse, bounded growth, overflow and empty-release checks.

<details>
<summary>Locate the existing block</summary>

```text
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.records.allocated_bytes()+if self.slots.instances()==0{0}else{self.slots.allocated_bytes()},self.writes,self.written_bytes]}
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzs-allocate-stable-object-record-rows-02.rs" skip=248
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.9.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The bounded stable-row allocator is written; drawing still uses the earlier sequential rows.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for allocate stable object record rows.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

A packed table can retain a live object at the same row across edits. Preserve rows belonging to surviving IDs, then assign new objects to the lowest free rows. Retired holes are reusable. Bound every row by the same device capacity already used by the record writer. This lesson prepares the allocator; the next lesson connects it to drawing.

Original IDs plus previous rows → retained rows and reused holes.



Why retain surviving rows before assigning new IDs?

Assigning a new ID first could steal a row still owned by a live object. Reserve survivors, then reuse only free rows.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect the supplied reorder, deletion, insertion and capacity fixture. In the browser, use the existing copied-object selection and opacity exercise; allocation is not connected yet.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.9.1
npm --prefix ../session_tests run course -- save 20.9.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Preparatory allocator only. Stable surface drawing, remaining pick/property uniforms, primitive lookup and full viewer parity still require integration.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.9.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
