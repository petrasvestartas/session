# 14 · Make document changes reversible

**Combined study estimate: 2–4 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 35–70 minutes.** 112 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Splitting required:** this verified checkpoint exceeds the one-hour typing target. Smaller runnable lessons are still being prepared.

**Today:** Undo and redo adding or deleting an object without changing the camera or losing identity.

**Follow:** Document edit → prior scene snapshot → undo or redo → restored scene → selection repair → GPU synchronization.

Make edits reversible with scene snapshots. Undo moves the current scene to the redo stack and restores the previous one. Redo reverses that move; a new edit clears redo.

Objects hold `Rc<Mesh>`, sharing immutable vertex data. Cloning a scene copies its object list and shared handles, not every vertex array. The pointer-equality test checks that sharing.

`History::edit` accepts `FnOnce`: an action called once with the scene. It records the prior snapshot before applying the action. Keep the latest 64 edits.

History restores document contents, leaving camera and background alone. Retain selection only if its ID exists in the restored scene, then upload through the normal scene-change path. Error rollback is added in lesson 17.

![An edit saves the previous scene; undo and redo move snapshots between two stacks while the camera stays outside the history.](../illustrations/journey-14.svg)

## Type the change

Continue from [Ask which object is under the pointer](13-picking.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-14-history` (from `session_viewer`).

### 1. `src/scene.rs`

Use shared ownership for immutable mesh data.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::mesh::Mesh;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/14-history-01.rs"
```

### 2. `src/scene.rs`

Allow scene snapshots to clone the object record.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Object {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/14-history-02.rs"
```

### 3. `src/scene.rs`

Share the mesh allocation between object snapshots.

<details>
<summary>Locate the existing block</summary>

```rust
    pub mesh: Mesh,
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/14-history-03.rs"
```

### 4. `src/scene.rs`

Clone the small scene records for a snapshot. Rc prevents a deep copy of each mesh.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Scene {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/14-history-04.rs"
```

### 5. `src/scene.rs`

Give a newly inserted mesh its first shared owner.

<details>
<summary>Locate the existing block</summary>

```rust
        self.objects.push(Object { id, mesh });
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/14-history-05.rs"
```

### 6. `src/scene.rs`

Restore document contents while preserving the highest ID counter reached.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn objects(&self) -> &[Object] {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/14-history-06.rs"
```

### 7. `src/history.rs`

Create bounded undo and redo stacks, plus tests for identity, shared data and a new editing branch.

Create the file and type:

```rust
--8<-- "journey/code/14-history-07.rs"
```

### 8. `src/lib.rs`

Register history as a document operation, independent of the browser and GPU.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod mesh;
pub mod scene;
pub mod picking;
pub mod gpu_mesh;
pub mod renderer;
#[cfg(target_arch = "wasm32")]
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/14-history-fullscreen-1.rs"
```

### 9. `src/browser.rs`

Import the history owner.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::background::Background;
use crate::camera::Camera;
use crate::renderer::Renderer;
use crate::scene::Scene;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/14-history-window-1.rs"
```

### 10. `src/browser.rs`

Add Undo and Redo to the vocabulary.

<details>
<summary>Locate the existing block</summary>

```rust
            "Example Triangle",
            "Select Next",
            "Delete",
            "Background",
            "Zoom In",
            "Zoom Out",
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/14-history-window-2.rs"
```

### 11. `src/browser.rs`

Create document history beside the scene.

<details>
<summary>Locate the existing block</summary>

```rust
        ],
    );
    panel.update(None, &canvas)?;
    let mut selected = None;
    let mut background = Background::default();
    let mut camera = Camera::default();
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/14-history-window-3.rs"
```

### 12. `src/browser.rs`

Record the example toggle as a history edit.

<details>
<summary>Locate the existing block</summary>

```rust
                    renderer.set_scene(&scene, selected);
                }
                "example triangle" => {
                    scene.toggle_extra();
                    selected = selected.filter(|id| scene.contains(*id));
                    renderer.set_scene(&scene, selected);
                }
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/14-history-window-4.rs"
```

### 13. `src/browser.rs`

Record deletion and route Undo/Redo through history; retain only valid selection.

<details>
<summary>Locate the existing block</summary>

```rust
                }
                "delete" => {
                    if let Some(id) = selected.take() {
                        scene.remove(id);
                        renderer.set_scene(&scene, selected);
                    }
                }
                "background" => background.toggle(),
                "zoom in" => camera.zoom(2.0),
                "zoom out" => camera.zoom(0.5),
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/14-history-window-5.rs"
```

### 14. `src/browser.rs`

Report the undo result.

<details>
<summary>Locate the existing block</summary>

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Click a triangle; the visible object is selected.");
    Ok(())
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/14-history-window-6.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Type `Example Triangle`, `Undo`, then `Redo`. The added triangle disappears and returns as one document change; the camera stays fixed.

**Verified checkpoint in Chrome.**

![Actual browser result: Make document changes reversible.](../screenshots/journey/14-history-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Add the third triangle, undo that addition, then add it again. It looks similar, but the new addition must receive a new ID. Read the branching-history test and explain why. Next pan the camera, delete an object and undo: the camera should stay where you put it.

</details>

## Explain the change

Why must restoring an old scene not reset the object-ID counter to its old value?

<details>
<summary>Compare your explanation</summary>

IDs created after that snapshot may still be remembered by a tool or earlier result. If the counter moved backward, a new object could receive one of those names. We restore the old scene contents but keep the highest counter reached, so an undo followed by a new edit cannot silently reuse an ID.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 14-history
npm --prefix ../session_tests run course -- save 14-history
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

The full viewer groups document operations into transactions and coordinates restoration with displayed rows and tool state. This lesson establishes reversible document changes, shared immutable data, preserved identity and the boundary between an edit and a view change.

</details>
