# 27d · Prove placement and history agree

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 22–44 minutes.** 52 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Check that Move changes world placement, preserves local geometry, and remains one reversible transaction.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Parsed offset → world translation → shared mesh → Undo/Redo → identical placement.

**Before you finish, explain:** What evidence distinguishes moving a placement from rewriting a mesh?

The visible result is useful evidence, but a moved picture alone does not prove which data changed. These checks follow the values that future editing tools must preserve.

First parse valid and invalid lines. Then start with a scaled placement. A world-axis Move must shift its world point by the exact requested offset, regardless of that existing scale. Retain an Rc handle and compare local vertices as well as world coordinates.

![World placement changes; local mesh and camera survive history travel.](../illustrations/journey-27d.svg)

Scene::place takes ownership of its Xform. Save the sixteen-value array before that call when a later assertion needs it. The array is Copy; the kernel Xform is not.

Undo must restore the complete old placement, and Redo must restore the moved placement. Insert invalid and zero moves between Undo and Redo: neither may discard the redo record. This is why a history transaction belongs in Editor rather than in a browser key handler.

The native frame also projects a known point on the moved box, casts its picking ray, and samples the resulting GPU pixels. Chrome repeats the typed move/Undo/Redo picture round trip and checks visible errors. These are separate checks of state, shader presentation and event delivery. An exact edge click compares two different calculations—the CPU ray and GPU raster. Here we test an interior fragment; the later GPU-ID lesson covers exact raster ownership.

## Type the change

Continue [Move a placed object with a typed offset](27c-move.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-27d-history`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/lib.rs`

Register the direct Move and history checks.

Find this exact block:

```rust
#[cfg(test)]
mod placement_tests;
```

Replace that block with:

```rust
--8<-- "journey/code/27d-history-01.rs"
```

### 2. `src/move_tests.rs`

Verify grammar, world-axis composition, shared geometry, failure/no-op history and selection.

Create the file and type:

```rust
--8<-- "journey/code/27d-history-02.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run Example Box, Select Next three times, View Orthographic, Move 0,0.6,0, and Fit. Then Undo and Redo the move. Run the Rust checks and compare the local mesh, model matrix, world point, selected ID and camera values at each stage.

**Actual Chrome screenshot.**

Prove placement and history agree. The actual command dock drives this checkpoint; the selected object is highlighted.

![Actual browser result: Prove placement and history agree.](../screenshots/journey/27d-history-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

## Try one small experiment

Change the existing x scale in the world-axis test from 2 to 3. Predict the local-axis error you would see if the multiplication order were reversed. Confirm the existing implementation still shifts the world point by the same amount, then restore the test.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

The world point shifts by the requested offset, the Rc allocation and local vertices stay identical, and Undo/Redo restore the placement while leaving the camera alone. Invalid input and zero movement must not consume the document’s redo or undo history.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 27d-history
npm --prefix ../session_tests run course -- save 27d-history
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

These invariants carry into picked-point Move, Copy, Rotate, Scale, previews and cancellation. The production course must keep the same source/placement distinction while adding large scenes, instancing and incremental uploads.

[Validation status and course release](release.md).
