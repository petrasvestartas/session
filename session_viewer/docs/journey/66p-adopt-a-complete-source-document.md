# 12.1.28 · Adopt a complete source document

**Chapter 12 · Files and streaming · Section 12.1 · Scene manifests and range reads** · [Contents](contents.md)

**Typing: 17–33 minutes.** [Estimate](typing-load.md).

Use a temporary scene while preparing rows, then install its namespace once the entire document succeeds.

## Type

Continue from [Prepare every supported geometry row](66o-prepare-every-supported-geometry-row.md). [Save or recover your work](recovery.md).

### 1. `src/scene_document.rs`

Adopt original document rows atomically and preserve the existing source namespaces.

Create the file and type:

```rust
--8<-- "journey/code/66p-adopt-a-complete-source-document-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod document_geometry;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/66p-adopt-a-complete-source-document-02.rs" skip=28
```

### 3. `src/scene.rs`

Prepare a source-free candidate before reopening complete documents.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn pick_id(&self,value:u32)->Option<ObjectId> {
        let id=ObjectId(value);(self.visible(id) && !self.view.locked.contains(&id) && !self.forest.readonly(id)).then_some(id)
    }
```

</details>

After that block, type:

```rust
--8<-- "journey/code/66p-adopt-a-complete-source-document-03.rs" skip=191
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.1.28
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

The original cloud and local point marker remain unchanged.

![Native renderer output for adopt a complete source document.](../screenshots/journey/65c-cloud-point.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [11.3.4 · Pick original points and prove source query reuse](65c-cloud-point.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



temporary scene → complete preparation → atomic document adoption.

![Adopt a complete source document.](../illustrations/66p-adopt-a-complete-source-document.svg)

May a late display error replace half the scene?

No. Adopt the candidate only after all source rows and placements pass.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Prepare an overflowing point after valid meshes and keep the previous scene unchanged.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.1.28
npm --prefix ../session_tests run course -- save 12.1.28
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Inherited browser assertions are authored and unrun. Publication browser assertions connect at the final checkpoint; native software rendering does not establish Lenovo performance.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 12.1.28
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
