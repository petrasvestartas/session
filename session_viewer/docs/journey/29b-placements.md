# 29b · Reopen source geometry with its placement

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 24–48 minutes.** 48 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Validate stored placements before reconstructing source-backed objects.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Bytes → validated mesh IDs and affine matrices → prepared local sources plus placement → inserted objects.

**Before you finish, explain:** Where must a saved transform be applied when reopening?

The snapshot contains local geometry and separate placement records. Extend the loader to accept exactly that contract. A transform is trusted only after its GUID belongs to a mesh, it occurs once, and its message contains sixteen finite values with an affine final row.

Put the matrix rule in placement::valid so file loading and Scene::place enforce the same boundary. The kernel Xform constructor can accept a short message by leaving other entries at defaults; the file validator must reject that ambiguity before construction.

![The loader validates mesh identities and affine placements, then prepares local source/display pairs with one object matrix.](../illustrations/journey-29b.svg)

PreparedMesh now carries a model matrix, initially identity for generated geometry. During load, copy the validated placement for that mesh GUID from the retained session. Scene insertion takes the prepared matrix instead of replacing it with identity.

The snapshot can now call the same full validator as load. No temporary copy with placements removed is necessary. Both directions accept the same bounded flat-mesh subset.

The snapshot checks still compare unchanged local source data and separate placement. The next lesson checks exact save/reopen round trips. The malformed-message checks reject short, non-finite and projective matrices, orphan GUIDs and duplicate placement entries before any scene changes.

Save is still a Rust function today. The next lesson connects its bytes to the command dock and a browser download.

## Type the change

Continue [Write a snapshot from the editable sources](29a-snapshot.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-29b-placements`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/placement.rs`

Use one affine validation rule for decoded matrices and direct edits.

Create the file and type:

```rust
--8<-- "journey/code/29b-placements-01.rs"
```

### 2. `src/lib.rs`

Expose the shared matrix validator.

Find this exact block:

```rust
pub mod prepared;
```

Replace that block with:

```rust
--8<-- "journey/code/29b-placements-02.rs"
```

### 3. `src/scene.rs`

Make direct placement edits use the same rule as imported files.

Find this exact block:

```rust
        if !model.m.iter().all(|v| v.is_finite())
            || [model.m[3], model.m[7], model.m[11], model.m[15]] != [0.0, 0.0, 0.0, 1.0]
```

Replace that block with:

```rust
--8<-- "journey/code/29b-placements-03.rs"
```

### 4. `src/prepared.rs`

Carry the validated object placement beside local source geometry.

Find this exact block:

```rust
    pub source: Option<Source>,
```

Replace that block with:

```rust
--8<-- "journey/code/29b-placements-04.rs"
```

### 5. `src/prepared.rs`

Generated geometry begins with identity placement.

Find this exact block:

```rust
        Ok(Self { geometry, display, source: None })
```

Replace that block with:

```rust
--8<-- "journey/code/29b-placements-05.rs"
```

### 6. `src/document.rs`

Retain the validated stored placement without baking it into source vertices.

Find this exact block:

```rust
        let mut prepared = PreparedMesh::from_shared(Rc::clone(mesh))?;
```

Replace that block with:

```rust
--8<-- "journey/code/29b-placements-06.rs"
```

### 7. `src/scene.rs`

Insert each prepared model exactly once.

Find this exact block:

```rust
            model: session_rust::Xform::identity(), source: prepared.source });
```

Replace that block with:

```rust
--8<-- "journey/code/29b-placements-07.rs"
```

### 8. `src/document.rs`

Remove the temporary source-only validation comment.

Find this exact block:

```rust
    // Source validation; placements come next.
```

Delete this block.

### 9. `src/document.rs`

Validate the complete saved message now that the loader accepts placements.

Find this exact block:

```rust
    let mut geometry = message.clone(); geometry.xforms.clear();
    validate(&geometry)?;
```

Replace that block with:

```rust
--8<-- "journey/code/29b-placements-09.rs"
```

### 10. `src/document.rs`

Accept placements while keeping the other unsupported source families explicit.

Find this exact block:

```rust
    if !message.xforms.is_empty() || message.definitions.is_some() || !message.interactions.is_empty() {
        return Err("Placements, definitions and interactions arrive in later checkpoints");
```

Replace that block with:

```rust
--8<-- "journey/code/29b-placements-10.rs"
```

### 11. `src/document.rs`

Reject unknown, duplicate, incomplete, non-finite or projective placement records before kernel construction.

Find this exact block:

```rust
    if let Some(root) = message.tree.as_ref().and_then(|tree| tree.root.as_ref()) {
```

Replace that block with:

```rust
--8<-- "journey/code/29b-placements-11.rs"
```

### 12. `src/placement_load_tests.rs`

Reject all malformed placement families using actual encoded messages.

Create the file and type:

```rust
--8<-- "journey/code/29b-placements-12.rs"
```

### 13. `src/lib.rs`

Compile the malformed-input checks.

Find this exact block:

```rust
#[cfg(test)]
mod snapshot_tests;
```

Replace that block with:

```rust
--8<-- "journey/code/29b-placements-13.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the snapshot and malformed-placement checks. In the viewer, Open your specimen, Example Box, Select Next six times, Move 1,0.5,0.3, View Isometric and Fit. Existing import and Move still work; the new loader also accepts snapshots that carry placements.

**Actual Chrome screenshot.**

![Actual browser result: Reopen source geometry with its placement.](../screenshots/journey/29b-placements-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

## Try one small experiment

Change the malformed-placement check’s projective matrix into a finite translation. Predict why validation accepts it, then restore the rejection case. Explain why a fifteen-value matrix remains invalid even if every supplied value is finite.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

A saved transform belongs to the object placement. Load prepares the original local kernel mesh and attaches its validated matrix. Scene insertion uses that matrix once. It must not transform source vertices and also retain the placement, or the object moves twice.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 29b-placements
npm --prefix ../session_tests run course -- save 29b-placements
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

The production session stores local placements keyed by object GUID and composes nested tree transforms later. This checkpoint handles one flat level and rejects unsupported definitions and interactions.

[Validation status and course release](release.md).
