# 20.5.2 · Prepare shared compact mesh ranges

**Chapter 20 · Shared GPU rendering · Section 20.5 · Shared mesh arenas** · [Contents](contents.md)

**Typing: 19–37 minutes.** [Estimate](typing-load.md).

Shared geometry needs one range of vertices and indices, regardless of how many placed rows draw it.

## Type

Continue from [Grow buffers without losing rows](96zzg-grow-buffers-without-losing-rows.md). [Save or recover your work](recovery.md).

### 1. `src/arena_rows.rs`

Use the existing twenty-byte encoder, rebase validated local indices and cache weak display identities.

Create the file and type:

```rust
--8<-- "journey/code/96zzh-prepare-shared-compact-mesh-ranges-1.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/arena_rows_tests.rs`

Supply exact packed bytes, shared owner ranges, global indices, weak retention and reset/release fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/96zzh-prepare-shared-compact-mesh-ranges-2.rs"
```

</details>

### 3. `src/lib.rs`

Register the preparatory arena records and supplied fixtures.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod packed_vertex;
```

</details>

After that block, type:

```rust
pub mod arena_rows;
#[cfg(test)]
mod arena_rows_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzh-prepare-shared-compact-mesh-ranges-3.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.5.2
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Prepared arena records preserve source positions and share ranges by display identity.

### Recorded visual result

**Previous native render — current verification pending.**

This supporting frame has no instances; use the supplied original Session specimen for the browser exercise.

![Native renderer output for prepare shared compact mesh ranges.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Prepare compact mesh records with the existing encoder. Rebase local indices into their shared vertex range and reuse ranges for the same display owner. Keep weak source references so derived arena records do not keep removed sources alive. This is preparation; the renderer still uploads per geometry.

Follow a local mesh index into its arena vertex offset and a reused range for shared placements..



Why add the vertex offset to indices rather than changing source positions?

The range changes where storage lives. Positions remain in the original object frame; placements remain separate draw state.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect placed copies of an original CAD surface and compare their shared geometry counts. Rendering is unchanged at this preparatory checkpoint; the supplied fixture verifies exact compact bytes, index offsets and weak source retention.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.5.2
npm --prefix ../session_tests run course -- save 20.5.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This prepares compact vertex/index arena ranges for the shared GPU owner. It reuses the existing packed encoder and records weak display identities. Arena uploads, free ranges, draw slots and their shader connection remain required.

Supplied fixtures verify exact compact bytes, shared display ranges, global index offsets, weak source retention and reset/release. Mesh arena uploads are not connected yet.

[Full validation scope](release.md).

</details>
