# 17 · Bring a solid into the scene

**Plan about 3–5 hours.** 99 lines to type, including comments and blank lines. Allow time to read, predict and experiment; this is an estimate, not a deadline.

**Today:** Create a kernel box, convert it to display data, and add it as one undoable scene object.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Kernel box → triangle extraction → validated display Mesh → document transaction → GPU upload → visible solid.

**Before you finish, explain:** Why does a failed mesh conversion belong outside the GPU draw loop?

The viewer can draw and select triangles. A solid adds topology: its faces meet to enclose a volume. We will create a box with the geometry kernel, then adapt that mesh to the display layout we already understand.

The kernel keeps geometric points in double precision and has its own face representation. `to_render` extracts float vertices and triangle indices. Our adapter selects position and colour from each vertex, then passes the result through Mesh's existing validation. Rendering still reads six floats per vertex; adding a box does not require a second renderer.

![A kernel box becomes a validated display mesh; a successful document transaction inserts it before the renderer uploads its triangles.](../illustrations/journey-17.svg)

A box has eight geometric corners, six quadrilateral faces and twelve triangles. The kernel's triangle extraction handles those convex box faces. This example does not establish correctness for every imported concave polygon; the CAD and topology lessons will address the additional contracts those sources need.

Our present index buffer uses 16-bit numbers. The kernel uses 32-bit indices, so conversion must be checked. `u16::try_from` returns an error if an index does not fit. The adapter reports that limitation rather than silently truncating a number and connecting the wrong vertices. We will widen the storage before large-scene work.

Creation also gives us a reason to strengthen history. `try_edit` records a successful change, but restores the previous scene if the action returns an error. Only a successful action clears redo. The test deliberately removes an object and then fails, proving that a partially changed scene does not escape the transaction.

The box is grey and has no lighting yet. In an isometric view you can recognise its silhouette, but adjoining faces share the same colour. Keep that observation: the next lesson will make the surface directions visible through light.

## Type the change

Continue [Walk around the model](16-orbit.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-17-solid`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/mesh.rs`

Adapt kernel triangle data to the current vertex and index layout with a checked narrowing conversion.

Find this exact block:

```rust
    pub fn vertices(&self) -> &[[f32; 6]] {
```

Replace that block with:

```rust
--8<-- "journey/code/17-solid-01.rs"
```

### 2. `src/mesh.rs`

Check the adapter using a solid with known topology.

Find this exact block:

```rust
    #[test]
    fn invalid_connections_and_nonfinite_vertices_are_rejected() {
```

Replace that block with:

```rust
--8<-- "journey/code/17-solid-02.rs"
```

### 3. `src/scene.rs`

Construct and place a box, then insert it only after its display mesh is valid.

Find this exact block:

```rust
    pub fn toggle_extra(&mut self) {
```

Replace that block with:

```rust
--8<-- "journey/code/17-solid-03.rs"
```

### 4. `src/history.rs`

Make transaction recording depend on success. The existing edit wrapper remains convenient for actions that cannot return an error.

Find this exact block:

```rust
    pub fn edit(&mut self, scene: &mut Scene, action: impl FnOnce(&mut Scene)) {
        let before = scene.clone();
        action(scene);
        if self.undo.len() == LIMIT {
            self.undo.remove(0);
        }
        self.undo.push(before);
        self.redo.clear();
    }
```

Replace that block with:

```rust
--8<-- "journey/code/17-solid-04.rs"
```

### 5. `src/history.rs`

Prove that a partial failed change cannot destroy either document contents or the redo branch.

Find this exact block:

```rust
    #[test]
    fn retained_history_has_a_fixed_count_limit() {
```

Replace that block with:

```rust
--8<-- "journey/code/17-solid-05.rs"
```

### 6. `src/browser.rs`

Connect bring a solid into the scene to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
        config.format.add_srgb_suffix(),
        &[
            "Help",
            "Example Triangle",
            "Select Next",
            "Delete",
```

Replace that block with:

```rust
--8<-- "journey/code/17-solid-window-1.rs"
```

### 7. `src/browser.rs`

Connect bring a solid into the scene to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
                        .and_then(|ray| crate::picking::pick(&scene, &ray));
                    renderer.set_scene(&scene, selected);
                }
                "example triangle" => {
                    history.edit(&mut scene, Scene::toggle_extra);
                    selected = selected.filter(|id| scene.contains(*id));
```

Replace that block with:

```rust
--8<-- "journey/code/17-solid-window-2.rs"
```

### 8. `src/browser.rs`

Connect bring a solid into the scene to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Orbit moves the eye around a fixed target.");
    Ok(())
}
```

Replace that block with:

```rust
--8<-- "journey/code/17-solid-window-3.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run `Example Box`, then `View Isometric`. A grey solid appears to the right of the triangles. Click it, run `Delete`, then `Undo`. Its identity and geometry return while the camera stays put.

**Actual Chrome screenshot.**

Example Box inserts the grey solid behind the triangles. Its faces have one flat colour at this checkpoint.

![Actual browser result: Bring a solid into the scene.](../screenshots/journey/17-solid-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

## Try one small experiment

Add a box and undo. In the rollback test, predict whether a failed new edit should erase the available redo. Run the test: it should not. Then redo the successful box addition. Separate three questions in your notes: did creation succeed, did the document change, and was the change uploaded?

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

Conversion establishes the display data contract before an object enters the scene. A failure should leave the document and history unchanged. The draw loop should read already valid GPU resources; it cannot repair an invalid index or decide whether a failed document edit should be kept.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 17-solid
npm --prefix ../session_tests run course -- save 17-solid
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

The final viewer converts kernel geometry into renderable lanes while preserving document identity and transactional edits. This adapter is the first real kernel-to-display connection; later lessons retain face and edge provenance and share allocations across many objects.

[Validation status and course release](release.md).
