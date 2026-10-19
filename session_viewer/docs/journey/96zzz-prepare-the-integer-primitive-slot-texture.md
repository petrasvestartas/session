# 20.7.6 · Prepare the integer primitive-slot texture

**Chapter 20 · Shared GPU rendering · Section 20.7 · Draw slots and primitive lookup** · [Contents](contents.md)

**Typing: 22–44 minutes.** [Estimate](typing-load.md).

Store slot lookup records in an integer texture with bounded partial uploads.

## Type

Continue from [Share one slot buffer across surface draws](96zzy-share-one-slot-buffer-across-surface-draws.md). [Save or recover your work](recovery.md).

### 1. `src/primitive_slot_table.rs`

Prepare one RGBA32Uint lookup owner, full replacement and bounded row writes.

Create the file and type:

```rust
--8<-- "journey/code/96zzz-prepare-the-integer-primitive-slot-texture-1.rs"
```

### 2. `src/lib.rs`

Register the lookup resource without changing the existing draw path.

<details>
<summary>Locate the existing block</summary>

```rust
mod draw_slots;
```

</details>

After that block, type:

```rust
mod primitive_slot_table;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzz-prepare-the-integer-primitive-slot-texture-2.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/primitive_slot_table_tests.rs`

Supply actual integer texture readback, row-crossing, bounds and release checks.

Copy this supplied file:

```text
--8<-- "journey/code/96zzz-prepare-the-integer-primitive-slot-texture-3.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/lib.rs`

Register the optional GPU validation fixture.

<details>
<summary>Locate the existing block</summary>

```text
#[cfg(test)] mod draw_slots_tests;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzz-prepare-the-integer-primitive-slot-texture-4.rs" skip=34
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.7.6
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

![Native renderer output for prepare the integer primitive-slot texture.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The vertex buffer selects placements. A lookup texture will let fragment passes find their original row and triangle interval. Prepare four integer words per texel, a fixed row width and partial writes that split at texture-row boundaries. The drawing remains unchanged until this table is connected.

slot index → texture row and column → four exact integer words.



Why split a partial upload at each texture-row boundary?

A rectangular texture write cannot wrap from the end of one row to the beginning of the next. Each run must fit its destination row.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Predict the columns and rows for slots 1023, 1024 and 1025 using index % WIDTH and index / WIDTH. Trace a three-texel upload starting at 1023. Keep the existing scene running: this preparatory table does not change its drawing.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.7.6
npm --prefix ../session_tests run course -- save 20.7.6
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This prepares the current four-word integer lookup resource. Allocation, record generation and fragment consumers still need connection; no browser rendering parity is claimed by this preparatory checkpoint.



[Full validation scope](release.md).

</details>
