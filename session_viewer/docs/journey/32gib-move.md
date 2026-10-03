# 32gib · Move the original target from its current placement

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 27–53 minutes.** 45 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Apply a captured Move to its original target, composing with the placement that exists at replay time.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Captured ObjectId + offset → current row.model → world shift × model → one history edit.

**Before you finish, explain:** Why must a delayed Move compose with the current placement rather than a matrix saved when the request began?

Extract Editor::move_object with an explicit ObjectId. It validates arguments, finds that current row, composes the world shift with its current model and makes one History::try_edit transaction. It leaves selection and camera untouched. Normal Action::Translate calls the same method after resolving current selection.

The native test captures a Move, changes the target’s placement and selects another object before executing the captured values. The result is based on the newer placement; selection and camera stay unchanged. One Undo restores the immediate prior model, another Undo does nothing, and Redo restores the result. No-op or refused moves preserve an existing Redo. Existing source tests still reject moving a cold row until hydration.

This is the native Move replay operation. One-shot asynchronous authority is added in the ticket/intent lesson, and automatic browser interception follows it. Chrome checks the normal Move/Undo/Redo path using this same method. Delete and Save are taught separately to keep each change small.

![Replay the original Move target](../illustrations/journey-32gib.svg)

## Type the change

Continue [Load only the sources the requested edit needs](32gia-scope.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-32gib-move`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/editor.rs`

Resolve an explicit target from current state and commit its world-space delta once.

Find this exact block:

```rust
    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
```

Replace that block with:

```rust
--8<-- "journey/code/32gib-move-01.rs"
```

### 2. `src/editor.rs`

Keep ordinary Move on the same implementation used by captured edits.

Find this exact block:

```rust
            Action::Translate(offset) => {
                let id = self.selected.ok_or("Select an object before Move")?;
                if offset.iter().all(|&v| v == 0.0) { return Ok(Change::View); }
                let object = self.scene.objects().iter().find(|o| o.id == id).ok_or("Object not found")?;
                let shift = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
                let model = &shift * &object.model;
                self.history.try_edit(&mut self.scene, |scene| scene.place(id, model))?;
            }
```

Replace that block with:

```rust
--8<-- "journey/code/32gib-move-02.rs"
```

### 3. `src/lib.rs`

Register the original-target Move checks.

Find this exact block:

```rust
mod edit_scope_tests;
```

Replace that block with:

```rust
--8<-- "journey/code/32gib-move-03.rs"
```

### 4. `src/edit_move_tests.rs`

Prove current placement, later selection, camera and one history transaction survive replay.

Create the file and type:

```rust
--8<-- "journey/code/32gib-move-04.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

From your project, run `REGEN_PROTO=0 cargo run --example sample --locked -j4`. Type Open Replace, choose sample.pb, Select Next twice, Move 0.35,0,0.25, View Isometric, Orbit Right, Orbit Up, Move 1.92,0,0.93, Move 0.25,0,-0.15 and Fit. Unload Sources and Reload Sources must retain this drawing. Automatic Move/Delete/Save reload is still pending. Then type Move 0,0.25,0 and Fit. Inspect the native scope tests to compare original-target requests with Save requests. Then type Move 0.25,0,0.15, Undo and Redo. Undo restores the preceding drawing; Redo restores the move. Type Fit afterward.

**Actual Chrome screenshot.**

Chrome checks normal Move/Undo/Redo through the extracted explicit-target method. Delayed target identity, current-placement composition and later selection are proved natively; automatic browser replay is still pending.

![Actual browser result: Move the original target from its current placement.](../screenshots/journey/32gib-move-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

In the native test, replace the newer model with the model that existed at capture time. Explain which placement that stale replacement would lose.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

The request describes an offset, not a replacement matrix. Reading the current model preserves any placement that now exists. The captured ObjectId chooses the target while current selection remains available for other interaction.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32gib-move
npm --prefix ../session_tests run course -- save 32gib-move
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Captured intent names the target and requested delta. Replay reads current state; a separate ticket owner limits which completion may execute it.

[Validation status and course release](release.md).
