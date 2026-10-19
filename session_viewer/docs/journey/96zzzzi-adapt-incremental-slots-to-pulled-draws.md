# 20.11.1 · Adapt incremental slots to pulled draws

**Chapter 20 · Shared GPU rendering · Section 20.11 · Retained allocation and lookup projection** · [Contents](contents.md)

**Typing: 5–10 minutes.** [Estimate](typing-load.md).

Keep physical triangle bases correct when pulled draws begin at local occurrence zero.

## Type

Continue from [Pick caps through shared records](96zzzzh-pick-caps-through-shared-records.md). [Save or recover your work](recovery.md).

### 1. `src/slot_space.rs`

Adapt physical bases to local pulled occurrences and report only changed member positions to the existing allocator.

<details>
<summary>Locate the existing block</summary>

```rust
  pub fn texel(&self, slot: u32) -> [u32; 4] {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
  pub fn local_slot(&self,slot:u32)->Slot {let texel=self.texel(slot);[texel[0],texel[1]]}
  pub fn touched(&self,definitions:&[Definition])->Vec<(u32,u32)> {
    let mut out=Vec::new();for definition in definitions {let Some(run)=self.runs.iter().find(|run|run.key==Some(definition.key))else{continue};for(position,row)in definition.rows.iter().enumerate(){if position<run.cap as usize&&self.rows.get(run.start as usize+position)!=Some(row){out.push((definition.key,position as u32));}}}out
  }
  pub fn texel(&self, slot: u32) -> [u32; 4] {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzi-adapt-incremental-slots-to-pulled-draws-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/slot_space.rs`

Supply local/global occurrence equivalence, exact physical IDs, changed-member reporting, reorder and retired-hole checks while retaining all five original allocator fixtures.

<details>
<summary>Locate the existing block</summary>

```text
    /// Instances number their triangles one after another, past the arena's:
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzi-adapt-incremental-slots-to-pulled-draws-02.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.11.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The allocator supplies local pulled slot bases and changed-member positions; live drawing still uses its previous slots.

### Recorded visual result

**Native render check — not a browser screenshot.**

![Native renderer output for adapt incremental slots to pulled draws.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*Read directly from this checkpoint’s GPU texture. Browser controls and event delivery remain unverified until the browser check passes.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The allocator originally subtracts the definition first triangle because its draw uses global arena indices. Our pulled draw begins at local occurrence zero. Add a local slot accessor from the existing texel instead of copying the allocator. Compare new definition rows with retained runs to report only changed member positions. The next connection will publish those changes to the existing GPU owners.

Retained allocator run → changed members and local slot base → correct physical IDs.



Why does a local pulled draw use the full physical base?

Its occurrence starts at zero. Subtracting the definition first triangle would shift every primitive address backward.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace the second placement of a two-triangle definition: local triangle one and global definition triangle eleven must name the same physical primitive.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.11.1
npm --prefix ../session_tests run course -- save 20.11.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The adapter is preparatory. Live incremental slots, primitive lookup, arena reclamation and broader viewer parity remain open.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.11.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
