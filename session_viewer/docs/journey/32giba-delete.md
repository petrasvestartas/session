# 32giba · Delete the original target while keeping later selection

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 26–52 minutes.** 49 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Delete a captured target by identity without redirecting the command to a later selection.

**Follow:** Captured ObjectId → current editable row → one history removal → preserve other selection.

Extract Editor::delete_object with an explicit ObjectId. Look up the current row and require editable geometry before creating history. A missing or cold target returns an error and preserves Redo. Normal Delete with no selection changes only the view.

History::edit records one removal. Clear selection only if it still names the deleted target; otherwise keep the newer selection. The camera is untouched.

The immutable borrow of row is last used in the geometry guard. Rust ends that borrow before History mutably borrows scene. Do not take selection to choose the replay target: taking it would clear a later selection before the command succeeds.

This lesson adds the native Delete operation. Automatic asynchronous replay remains a later lesson.

![Delete the captured target](../illustrations/journey-32giba.svg)

## Type the change

Continue from [Move the original target from its current placement](32gib-move.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-32giba-delete` (from `session_viewer`).

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

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the captured-Delete checks below. Later selection must not change the deleted target. One Undo restores the captured object.

**Verified checkpoint in Chrome.**

![Actual browser result: Delete the original target while keeping later selection.](../screenshots/journey/32giba-delete-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Select the other row before calling delete_object in the native test. Replace the identity comparison with unconditional selection clearing and explain the lost user interaction.

</details>

## Explain the change

Why does a delayed Delete compare selection with its captured target before clearing it?

<details>
<summary>Compare your explanation</summary>

The user may have selected another row while loading the target. The captured identity decides what to delete; current selection is cleared only when that exact row disappears.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32giba-delete
npm --prefix ../session_tests run course -- save 32giba-delete
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

Delayed edits resolve their captured target from current state, then mutate history once. Current selection is independent of the requested target.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

The native test captures Delete, selects another row and orbits before replaying the captured identity. One Undo restores the row; another Undo does nothing, and Redo removes the same target. A second test covers cold/missing targets, empty selection and clearing the deleted selection.

Chrome checks normal Delete/Undo/Redo through the explicit-target helper. Native tests prove delayed target identity, preserved later selection/camera and refusal without losing Redo. Automatic browser replay remains pending.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32giba-delete
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
