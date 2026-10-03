# 26 · Frame one object without changing its size

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 22–44 minutes.** 54 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Run Fit Selected while keeping geometry, selection and document history unchanged.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Fit Selected → Editor → stable ObjectId → selected bounds → Camera::fit → frame.

**Before you finish, explain:** Does fitting an object change its coordinates or only the camera?

A small component can be lost inside a large assembly. Fit shows the whole scene; Fit Selected should show just the object you chose. We reuse the fitting policy from lesson 24 and change only the source of its bounds.

Scene searches by ObjectId, rather than treating a vector index as identity. The question mark after find returns None immediately if that ID is absent. A second question mark handles an empty vertex list. Starting from the first real vertex avoids accidentally including the origin in every box.

Editor holds an Option<ObjectId>. and_then asks Scene for bounds only when that option contains an ID. Camera then receives the same Bounds type it already understands. No duplicated fitting mathematics is needed.

![The selected ID chooses bounds; fitting changes the camera and leaves the shared mesh alone.](../illustrations/journey-26.svg)

A model coordinate and a screen pixel are different quantities. If an object spans ten model units, fitting it does not make it one unit wide. It makes those same ten units occupy more pixels. We have not introduced a unit-conversion command or claimed millimetres: the document coordinate convention still determines the unit. Later placements and source-document lessons will preserve that distinction.

## Type the change

Continue [Choose how depth changes size](25-projection.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-26-selected`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/scene.rs`

Find one stable ID and collect only its displayed vertices. An absent ID or empty mesh returns None.

Find this exact block:

```rust
    pub fn objects(&self) -> &[Object] {
```

Replace that block with:

```rust
--8<-- "journey/code/26-selected-01.rs"
```

### 2. `src/editor.rs`

Name a separate action for fitting the selection.

Find this exact block:

```rust
    Fit,
    Projection(crate::camera::Projection),
```

Replace that block with:

```rust
--8<-- "journey/code/26-selected-02.rs"
```

### 3. `src/editor.rs`

Borrow the selected ID, ask Scene for its bounds, and reuse Camera::fit. Neither the mesh nor document history changes.

Find this exact block:

```rust
                    Action::ResetView => {
```

Replace that block with:

```rust
--8<-- "journey/code/26-selected-03.rs"
```

### 4. `src/browser.rs`

Expose Fit Selected in the existing command vocabulary.

Find this exact block:

```rust
            "Fit",
            "View Reset",
```

Replace that block with:

```rust
--8<-- "journey/code/26-selected-04.rs"
```

### 5. `src/browser.rs`

Translate the named command into the new action.

Find this exact block:

```rust
                    "fit" => Action::Fit,
                    "view reset" => Action::ResetView,
```

Replace that block with:

```rust
--8<-- "journey/code/26-selected-05.rs"
```

### 6. `src/lib.rs`

Register the focused state checks.

Find this exact block:

```rust
#[cfg(test)]
mod projection_tests;
```

Replace that block with:

```rust
--8<-- "journey/code/26-selected-06.rs"
```

### 7. `src/selected_fit_tests.rs`

Check both projection modes, selection, shared mesh identity, history and the empty-selection case.

Create the file and type:

```rust
--8<-- "journey/code/26-selected-07.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run Example Box, View Isometric, Select Next and Fit Selected. The first triangle becomes the fitting target while the other objects remain in the scene. Run Fit to compare the whole-scene view. Type Fit Selected after clicking the drawing: it must reach the command dock. Run Undo: it removes the added box, not the camera move.

**Actual Chrome screenshot.**

Fit Selected frames the first triangle without deleting the added box or changing object coordinates.

![Actual browser result: Frame one object without changing its size.](../screenshots/journey/26-selected-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

## Try one small experiment

Select Next repeatedly and run Fit Selected after each selection. Predict the camera target from that object’s minimum and maximum coordinates. Then compare perspective and orthographic fits. Explain why neither operation should create an undo entry.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

Only the camera target and distance change. The selected object keeps the same shared mesh and stable ID. Bounds are measured in existing model coordinates; fitting changes their screen coverage, not their physical dimensions.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 26-selected
npm --prefix ../session_tests run course -- save 26-selected
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

The maintained viewer fits selected display rows and their world-space bounds. This checkpoint establishes stable selection and camera-only fitting. Local placements, inherited transforms and multiple selected objects will extend the bounds query in later lessons.

[Validation status and course release](release.md).
