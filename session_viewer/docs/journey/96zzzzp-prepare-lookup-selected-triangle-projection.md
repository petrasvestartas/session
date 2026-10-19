# 20.11.8 · Prepare lookup-selected triangle projection

**Chapter 20 · Shared GPU rendering · Section 20.11 · Retained allocation and lookup projection** · [Contents](contents.md)

**Typing: 6–11 minutes.** [Estimate](typing-load.md).

Resolve physical primitive IDs to their shared source triangle and object row.

## Type

Continue from [Separate projection rows and arena offsets](96zzzzo-separate-projection-rows-and-arena-offsets.md). [Save or recover your work](recovery.md).

### 1. `src/finite_project_source.rs`

Assemble lookup-selected projection from the existing shader and unsigned lookup helper; keep one clipping and projection algorithm.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn lookup()->String {
 let source=records().replace("let index = local + live_count.y;","let index = local;").replace("let polygon = project_physical_triangle(local+1u, live_count.z, object_at(live_count.z).range.x);","let source=placed_triangle(index+1u,live_count.y);if(source.x==NO_TRIANGLE||source.y==SLOT_OWN_ROW){return;}let polygon=project_physical_triangle(source.x+1u,source.y,0u);");
 format!("@group(2) @binding(5) var slot_table:texture_2d<u32>;\n{}\n{}",include_str!("primitive_slot_lookup.wgsl"),source)
}

#[cfg(test)]
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzp-prepare-lookup-selected-triangle-projection-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/finite_project_source.rs`

Supply validation of uniform, shared per-mesh and lookup-selected compute interfaces.

<details>
<summary>Locate the existing block</summary>

```text
[super::source(),super::records()]
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzp-prepare-lookup-selected-triangle-projection-02.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.11.8
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The lookup-selected compute source is prepared and interface-checked; live projection still uses its existing dispatches.

**Verified checkpoint in Chrome.**

**Feature-specific visual review pending.** Scripted acceptance and the final main-page capture are separate evidence. A checker may restore its general fixture after closing the feature-test page; this image does not establish the feature’s visual correctness.

![General browser fixture after Prepare lookup-selected triangle projection; feature-scene visual acceptance pending.](../screenshots/journey/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-browser.png)

[Verification scope](release.md).

This reference image is shared with [20.11.6 · Adopt retained GPU slots and physical bases](96zzzzn-adopt-retained-gpu-slots-and-physical-bases.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Assemble a third compute source from the projection algorithm already written. The unsigned inverse lookup resolves a physical ID into an arena triangle and object row. Skip direct-arena and inactive entries, then project the resolved triangle with a zero extra offset. The existing uniform and per-mesh record sources remain available.

Physical primitive ID → exact unsigned lookup → arena triangle and object row → shared projection.



Why skip direct-arena entries in this owned slot path?

Visible objects use explicit slot rows. A direct-arena entry has the own-row sentinel and no shared object row to index.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace two placed copies to the same arena triangle and different object rows. Trace an inactive slot and confirm it produces no projected triangle.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.11.8
npm --prefix ../session_tests run course -- save 20.11.8
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Lookup-selected projection is prepared. Binding and dispatch adoption, reclamation, precise coordinates and broader viewer parity remain open.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.11.8
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
