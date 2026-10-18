# 20.5.3 · Upload compact ranges to shared GPU storage

**Chapter 20 · Shared GPU rendering · Section 20.5 · Shared mesh arenas** · [Contents](contents.md)

**Typing: 23–46 minutes.** [Estimate](typing-load.md).

A shared arena binds one vertex buffer and one index buffer.

## Type

Continue from [Prepare shared compact mesh ranges](96zzh-prepare-shared-compact-mesh-ranges.md). [Save or recover your work](recovery.md).

### 1. `src/arena_rows.rs`

Share one index serializer for full and appended ranges.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn index_bytes(&self)->Vec<u8>{self.indices.iter().flat_map(|i|i.to_le_bytes()).collect()}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn index_bytes(&self)->Vec<u8>{self.index_bytes_from(0)}
    pub fn index_bytes_from(&self,start:usize)->Vec<u8>{self.indices[start..].iter().flat_map(|i|i.to_le_bytes()).collect()}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzi-upload-compact-ranges-to-shared-gpu-storage-1.rs"
```

</details>

### 2. `src/mesh_arena.rs`

Upload appended records through the existing owners and rebuild bindings only when growth replaces resources.

Create the file and type:

```rust
--8<-- "journey/code/96zzi-upload-compact-ranges-to-shared-gpu-storage-2.rs"
```

### 3. `src/lib.rs`

Register the preparatory shared GPU storage owner.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod arena_rows;
```

</details>

After that block, type:

```rust
pub mod mesh_arena;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzi-upload-compact-ranges-to-shared-gpu-storage-3.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/growing_buffer_tests.rs`

Reuse the existing supplied mapped-buffer reader in the arena fixtures.

<details>
<summary>Locate the existing block</summary>

```text
fn read(ctx:
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzi-upload-compact-ranges-to-shared-gpu-storage-4.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/mesh_arena_tests.rs`

Supply real packed-byte reads and two rendered arena triangles through fresh and stale bindings.

Copy this supplied file:

```text
--8<-- "journey/code/96zzi-upload-compact-ranges-to-shared-gpu-storage-5.rs"
```

</details>

### 6. `src/lib.rs`

Register the supplied arena GPU fixtures.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod mesh_arena;
```

</details>

After that block, type:

```rust
#[cfg(test)]
mod mesh_arena_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzi-upload-compact-ranges-to-shared-gpu-storage-6.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.5.3
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

One shared GPU owner uploads appended records, preserves earlier ranges and rebuilds bindings when resources change.

### Recorded visual result

**Previous native render — current verification pending.**

This supporting frame has no instances; use the supplied original Session specimen for the browser exercise.

![Native renderer output for upload compact ranges to shared gpu storage.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Upload only the new ranges through the existing growth owner. Rebuild the shared bind group when growth replaces either buffer: bind groups retain the resources they were created with. Reuse the CPU range cache and index encoder. The application renderer still uses per-geometry uploads at this preparatory checkpoint.

Follow an appended range into buffer growth and the replacement shared bind group..



Why does growing a buffer require a new bind group?

The old bind group refers to the old GPU buffer. Updating a Rust field does not retarget that resource binding.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect existing placed CAD copies and their upload counters; the application remains unchanged here. In the supplied GPU exercise, omit rebuilding the group and predict which of the two arena triangles disappears. Restore the rebuild before continuing.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.5.3
npm --prefix ../session_tests run course -- save 20.5.3
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This prepares the shared mesh GPU owner using the existing CPU range cache and growth helper. It does not yet replace renderer geometry uploads or connect draw slots, physical projection and source-face reads.

Supplied GPU fixtures read exact packed bytes and render both arena ranges after nonempty growth. They verify shared identity, replacement bindings, reset and release.

[Full validation scope](release.md).

</details>
