# 30e · Prove recovery at the transaction and display boundaries

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 26–51 minutes.** 54 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Test partial replacement failure, preserved selection/Redo and placement upload limits.

**Follow:** Malformed source or failed insertion → retained old owners and history; double matrix → finite float upload check.

Test a replacement failure after insertion starts, not only malformed bytes before mutation. Put the object-ID counter one below its limit: one insertion succeeds and the next fails.

History must restore every old row and allocation while retaining the highest issued counter. IDs from failed work must not be reused. A separate malformed-replacement test preserves selection and Redo.

Also reject placement coefficients that are finite as doubles but become infinity in the current float upload. `f64::MAX` demonstrates this boundary for direct placement and encoded files. Keep the previous placement untouched. This limits the display adapter without reducing retained source precision.

![Decode errors occur before mutation; insertion errors trigger rollback; upload range errors are rejected before drawing.](../illustrations/journey-30e.svg)

## Type the change

Continue from [Choose append or replace before opening the picker](30d-bridge.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-30e-recovery` (from `session_viewer`).

### 1. `src/placement.rs`

Reject a finite double coefficient that cannot enter the current float uniform.

<details>
<summary>Locate the existing block</summary>

```rust
matrix.iter().all(|value| value.is_finite())
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30e-recovery-01.rs"
```

### 2. `src/scene.rs`

State the full placement boundary in its error.

<details>
<summary>Locate the existing block</summary>

```rust
return Err("Placement must be finite and affine");
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30e-recovery-02.rs"
```

### 3. `src/scene.rs`

Fail during the second insertion, then prove rollback restores the old allocations without recycling issued IDs.

<details>
<summary>Locate the existing block</summary>

```rust
    fn removal_moves_a_row_without_changing_its_identity() {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30e-recovery-03.rs"
```

### 4. `src/replace_tests.rs`

Test refusal before mutation, selection/Redo retention and the double-to-float upload boundary.

<details>
<summary>Locate the existing block</summary>

```rust
    assert_eq!((editor.camera.distance, editor.camera.aspect), (7.0, 2.0));
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30e-recovery-04.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the recovery checks below. A replacement that fails on its second mesh must preserve the whole previous scene and its Redo branch.

**Verified checkpoint in Chrome.**

![Actual browser result: Prove recovery at the transaction and display boundaries.](../screenshots/journey/30e-recovery-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Temporarily remove History::try_edit from the partial-replacement check. Predict why the old scene disappears when the second insertion fails. Restore the transaction. Then compare f64::MAX with its float cast before reading the range-check answer.

</details>

## Explain the change

Why do we need a failure after the first new row was inserted?

<details>
<summary>Compare your explanation</summary>

Malformed decoding fails before a transaction starts. It does not test rollback after mutation. Exhausting the ID counter during the second insertion proves History::try_edit restores old rows and shared owners after a partial replacement, while retaining the highest issued counter so failed work cannot cause ID reuse.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 30e-recovery
npm --prefix ../session_tests run course -- save 30e-recovery
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

Production recovery preserves the last valid scene and checks display representations separately from editable source data. This endpoint proves those boundaries for the current flat mesh editor; full geometry, streaming and coordinate rebasing remain later lessons.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Chrome repeats the file errors, cancelled newer picker, replacement and Undo/Redo checks against this final endpoint. It also forces adapter unavailability before startup, checks the visible failure fallback, then reloads and restores a working viewer. Its screenshot shows the replaced frame, with an independently moved right post. Native frame checks distinguish selected gold from timber brown rather than treating both as selection.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 30e-recovery
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
