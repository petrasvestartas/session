# 20.11.2 · Prepare exact arena slot definitions

**Chapter 20 · Shared GPU rendering · Section 20.11 · Retained allocation and lookup projection** · [Contents](contents.md)

**Typing: 14–27 minutes.** [Estimate](typing-load.md).

Describe allocator members using exact arena ranges and stable object rows.

## Type

Continue from [Adapt incremental slots to pulled draws](96zzzzi-adapt-incremental-slots-to-pulled-draws.md). [Save or recover your work](recovery.md).

### 1. `src/instance_members.rs`

Group stable object rows by exact shared arena ranges; reject standalone buffers and mixed arenas instead of narrowing pointers into IDs.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzj-prepare-exact-arena-slot-definitions-01.rs"
```

### 2. `src/lib.rs`

Declare the preparatory definition adapter.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod instance_gpu;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod instance_members;
mod instance_gpu;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzj-prepare-exact-arena-slot-definitions-02.rs"
```

</details>

### 3. `src/gpu_geometry.rs`

Expose shared arena identity as a borrowed owner without changing range or allocation ownership.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn allocated_bytes(&self)->u64{
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub(crate) fn arena(&self)->Option<&Rc<RefCell<crate::mesh_arena::Arena>>> {match &self.storage {Storage::Shared(arena)=>Some(arena),Storage::Owned{..}=>None}}
    pub fn allocated_bytes(&self)->u64{
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzj-prepare-exact-arena-slot-definitions-03.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/instance_members.rs`

Supply exact range keys, repeated-definition merging, stable rows, original arena count, standalone/mixed-arena refusal and empty checks.

<details>
<summary>Locate the existing block</summary>

```text
use std::{collections::BTreeMap,ops::Range,rc::Rc};
use crate::gpu_geometry::GpuGeometry;
pub struct Members {pub key:u32,pub faces:Range<u32>,pub rows:Vec<u32>}
impl Members {
 pub fn definition(&self)->crate::slot_space::Definition<'_> {
  crate::slot_space::Definition {key:self.key,rows:&self.rows,faces:self.faces.clone(),pipes:0..0,ribbons:0..0}
 }
}
pub fn collect<'a>(sources:impl IntoIterator<Item=(&'a Rc<GpuGeometry>,Vec<u32>)>)->Option<(Vec<Members>,u32)> {
 let mut arena=None;let mut count=0;let mut members=BTreeMap::<u32,Members>::new();
 for(geometry,rows)in sources {
  let current=geometry.arena()?;if arena.is_some_and(|first|!Rc::ptr_eq(first,current)){return None;}arena=Some(current);
  count=u32::try_from(current.borrow().rows.indices.len()/3).ok()?;
  let first=geometry.index_start;let end=first.checked_add(geometry.index_count)?;
  if first%3!=0||geometry.index_count%3!=0{return None;}
  let member=members.entry(first).or_insert_with(||Members {key:first,faces:first..end,rows:Vec::new()});
  if member.faces.end!=end{return None;}member.rows.extend(rows);
 }
 Some((members.into_values().collect(),count))
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzj-prepare-exact-arena-slot-definitions-04.rs" skip=1140
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.11.2
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Exact shared-arena definitions are available; live slots remain unchanged.

### Recorded visual result

**Native render check — not a browser screenshot.**

![Native renderer output for prepare exact arena slot definitions.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*Read directly from this checkpoint’s GPU texture. Browser controls and event delivery remain unverified until the browser check passes.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The allocator needs a stable integer definition key and a complete face range. A shared arena index offset already provides that exact identity. Group object rows at that offset and verify that every geometry belongs to the same arena. Standalone preview buffers retain the earlier drawing path. This adapter prepares the next live allocator connection without changing draws.

Shared arena range plus stable rows → one definition per source → existing slot allocator.



Why avoid converting a geometry pointer into a definition key?

A pointer can be wider than the integer key. Its truncated value can collide, while the existing bounded arena range is an exact identity.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace two originals sharing one definition and a second independent mesh. The first pair shares a face range while retaining separate object rows.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.11.2
npm --prefix ../session_tests run course -- save 20.11.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Definition preparation is complete. Live incremental slots, primitive lookup, reclamation and broader viewer parity remain open.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.11.2
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
