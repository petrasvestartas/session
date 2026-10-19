# 3.13.2 · Write a snapshot from the editable sources

**Chapter 3 · Documents, editing and ownership · Section 3.13 · Stable identity and saving** · [Contents](contents.md)

**Typing: 13–26 minutes.** [Estimate](typing-load.md).

Build Save bytes from live scene objects, including generated and edited objects and excluding deleted rows.

## Type

Continue from [Give each saved object a stable identity](29-identity.md). [Save or recover your work](recovery.md).

### 1. `src/document.rs`

Copy live sources into one bounded flat document, write placements separately and return bytes without editor mutation.

<details>
<summary>Locate the existing block</summary>

```rust
fn validate(message: &proto::Session) -> Result<(), &'static str> {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/29a-snapshot-01.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/snapshot_tests.rs`

Optional authoring fixture. Check live row membership, source topology/attributes, separate placement and untouched history.

Copy this supplied file:

```text
--8<-- "journey/code/29a-snapshot-02.rs"
```

</details>

### 3. `src/lib.rs`

Compile the read-only snapshot checks.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod file_identity_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/29a-snapshot-03.rs" skip=38
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Run the snapshot checks below. Serialization must read original source coordinates and separate placements without changing the live editor. Browser Save is connected in lesson 29d.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for write a snapshot from the editable sources.](../screenshots/journey/29a-snapshot.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>

Copy each source mesh's protobuf message, preserving doubles, topology, colours, names and flags. Replace only the copy's GUID with the stored object identity. Keep geometry local and write placement separately, keyed by that same GUID; baking both would move it twice.

This snapshot writes a flat tree. Camera, selection, display arrays and history are application state and stay outside the file.

Match the current loader's bounds: 1–64 meshes and at most 4 MiB encoded. Reject unsupported source data. Today Rust checks inspect the snapshot; placement loading and browser downloading follow.

Live objects → source protobuf copies + object GUIDs → flat tree and placements → bytes.

![Current objects supply copied mesh messages; stored object GUIDs connect those messages to a flat tree and placement records.](../illustrations/journey-29a.svg)

Why must saving copy source messages instead of modifying the retained session?

The retained file can contain objects that have since been deleted, and imported source allocations are shared through history. Saving traverses current objects and copies their source messages. It applies the inserted object GUID to each copy and writes placement separately, leaving the original geometry, history and file provenance unchanged.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Change the box placement in the check, then predict whether mesh vertices or the XformEntry changes. Confirm the retained source GUID stays unchanged and restore the check.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 3.13.2
npm --prefix ../session_tests run course -- save 3.13.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The production serializer combines editable source objects and placements rather than reading GPU buffers. This checkpoint covers a deliberately bounded flat-mesh document; complete trees, geometry families and metadata arrive later.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 3.13.2
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
