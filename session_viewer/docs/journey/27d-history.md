# 27d · Prove placement and history agree

**Typing: 22–44 minutes.** [Estimate](typing-load.md).

The visible result is useful evidence, but a moved picture alone does not prove which data changed. These checks follow the values that future editing tools must preserve.

## Type

Continue from [Move a placed object with a typed offset](27c-move.md). [Save or recover your work](recovery.md).

### 1. `src/lib.rs`

Register the direct Move and history checks.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod placement_tests;
```

</details>

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

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Run the placement/history checks below. Move and Undo must change only placement, preserve shared local geometry, and treat a zero offset as no document edit.

**Verified checkpoint in Chrome.**

![Actual browser result: Prove placement and history agree.](../screenshots/journey/27d-history-browser.png)

[Verification scope](release.md).

Run the state checks on your computer from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked --target host-tuple -j4
```

<details>
<summary>Code explanation and diagram</summary>

First parse valid and invalid lines. Then start with a scaled placement. A world-axis Move must shift its world point by the exact requested offset, regardless of that existing scale. Retain an Rc handle and compare local vertices as well as world coordinates.

Scene::place takes ownership of its Xform. Save the sixteen-value array before that call when a later assertion needs it. The array is Copy; the kernel Xform is not.

Undo must restore the complete old placement, and Redo must restore the moved placement. Insert invalid and zero moves between Undo and Redo: neither may discard the redo record. This is why a history transaction belongs in Editor rather than in a browser key handler.

Parsed offset → world translation → shared mesh → Undo/Redo → identical placement.

![World placement changes; local mesh and camera survive history travel.](../illustrations/journey-27d.svg)

What evidence distinguishes moving a placement from rewriting a mesh?

The world point shifts by the requested offset, the Rc allocation and local vertices stay identical, and Undo/Redo restore the placement while leaving the camera alone. Invalid input and zero movement must not consume the document’s redo or undo history.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Change the existing x scale in the world-axis test from 2 to 3. Predict the local-axis error you would see if the multiplication order were reversed. Confirm the existing implementation still shifts the world point by the same amount, then restore the test.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 27d-history
npm --prefix ../session_tests run course -- save 27d-history
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These invariants carry into picked-point Move, Copy, Rotate, Scale, previews and cancellation. The production course must keep the same source/placement distinction while adding large scenes, instancing and incremental uploads.

The native frame also projects a known point on the moved box, casts its picking ray, and samples the resulting GPU pixels. Chrome repeats the typed move/Undo/Redo picture round trip and checks visible errors. These are separate checks of state, shader presentation and event delivery. An exact edge click compares two different calculations—the CPU ray and GPU raster. Here we test an interior fragment; the later GPU-ID lesson covers exact raster ownership.

Prove placement and history agree. The actual command dock drives this checkpoint; the selected object is highlighted.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 27d-history
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
