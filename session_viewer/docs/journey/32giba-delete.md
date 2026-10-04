# 32giba · Delete the original target while keeping later selection

**Typing: 26–52 minutes.** [Estimate](typing-load.md).

Extract Editor::delete_object with an explicit ObjectId. Look up the current row and require editable geometry before creating history. A missing or cold target returns an error and preserves Redo. Normal Delete with no selection changes only the view.

## Type

Continue from [Move the original target from its current placement](32gib-move.md). [Save or recover your work](recovery.md).

### 1. `src/editor.rs`

Remove the original target once and clear only its own selection.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32giba-delete-01.rs"
```

### 2. `src/editor.rs`

Use the same helper for ordinary Delete and preserve history when nothing is selected.

<details>
<summary>Locate the existing block</summary>

```rust
            Action::Delete => {
                if let Some(id) = self.selected {
                    let row = self.scene.objects().iter().find(|row| row.id == id).ok_or("Object not found")?;
                    if row.geometry().is_none() { return Err("Reload editable sources before Delete"); }
                }
                if let Some(id) = self.selected.take() {
                    self.history.edit(&mut self.scene, |scene| { scene.remove(id); });
                }
            }
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32giba-delete-02.rs"
```

### 3. `src/lib.rs`

Register the captured-target Delete checks.

<details>
<summary>Locate the existing block</summary>

```rust
mod edit_move_tests;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32giba-delete-03.rs"
```

### 4. `src/edit_delete_tests.rs`

Check delayed selection, one Undo and refusal without losing Redo.

Create the file and type:

```rust
--8<-- "journey/code/32giba-delete-04.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Run the captured-Delete checks below. Later selection must not change the deleted target. One Undo restores the captured object.

**Verified checkpoint in Chrome.**

![Actual browser result: Delete the original target while keeping later selection.](../screenshots/journey/32giba-delete-browser.png)

[Verification scope](release.md).

Run the state checks on your computer from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked --target host-tuple -j4
```

<details>
<summary>Code explanation and diagram</summary>

History::edit records one removal. Clear selection only if it still names the deleted target; otherwise keep the newer selection. The camera is untouched.

The immutable borrow of row is last used in the geometry guard. Rust ends that borrow before History mutably borrows scene. Do not take selection to choose the replay target: taking it would clear a later selection before the command succeeds.

This lesson adds the native Delete operation. Automatic asynchronous replay remains a later lesson.

Captured ObjectId → current editable row → one history removal → preserve other selection.

![Delete the captured target](../illustrations/journey-32giba.svg)

Why does a delayed Delete compare selection with its captured target before clearing it?

The user may have selected another row while loading the target. The captured identity decides what to delete; current selection is cleared only when that exact row disappears.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Select the other row before calling delete_object in the native test. Replace the identity comparison with unconditional selection clearing and explain the lost user interaction.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 32giba-delete
npm --prefix ../session_tests run course -- save 32giba-delete
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Delayed edits resolve their captured target from current state, then mutate history once. Current selection is independent of the requested target.

The native test captures Delete, selects another row and orbits before replaying the captured identity. One Undo restores the row; another Undo does nothing, and Redo removes the same target. A second test covers cold/missing targets, empty selection and clearing the deleted selection.

Chrome checks normal Delete/Undo/Redo through the explicit-target helper. Native tests prove delayed target identity, preserved later selection/camera and refusal without losing Redo. Automatic browser replay remains pending.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32giba-delete
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
