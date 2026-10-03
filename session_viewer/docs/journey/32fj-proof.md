# 32fj · Prove unloading preserves placed history

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 19–38 minutes.** 30 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Observe imported kernel expiration while retained displays and placements survive Undo/Redo.

**Follow:** Weak kernel/document observers → unload all roots → retained display → cold model-history travel.

Use a located imported document and a real placement edit. Retain its derived display Rc, record Weak observers of the kernel mesh and Session, and unload. The old editable values must disappear while the row’s local ID, saved GUID, display owner, placement and camera remain.

![Unloaded source values disappear while display and placement history remain.](../illustrations/journey-32fj.svg)

An unavailable-source Save or Move must refuse without consuming Undo. Undo then restores the earlier placement and Redo restores the moved placement; both keep the origin’s release epoch. This is residency travel across existing document history, not an undo command for unloading.

The next endpoint exposes the same operation in Chrome and adds GPU retention proof. Rehydration and automatic edit replay remain separate required work.

## Type the change

Continue from [Unload sources across active and history roots](32fi-history.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-32fj-proof` (from `session_viewer`).

### 1. `src/lib.rs`

Verify actual source release independently of GPU drawing.

<details>
<summary>Locate the existing block</summary>

```rust
mod origin_tests;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32fj-proof-01.rs"
```

### 2. `src/unload_tests.rs`

Prove old source values expire while placed drawing data and history remain available.

Create the file and type:

```rust
--8<-- "journey/code/32fj-proof-02.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the weak-owner checks below. After source release, the original source allocation must expire while its retained display still draws.

**Verified checkpoint in Chrome.**

![Actual browser result: Prove unloading preserves placed history.](../screenshots/journey/32fj-proof-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Retain only the Origin rather than the display Rc in the fixture. Predict which owners survive and which Weak values expire.

</details>

## Explain the change

What proves source unloading did not secretly clear the document history?

<details>
<summary>Compare your explanation</summary>

Record the number of scene roots and an earlier model matrix, unload, and Undo the prior Move. The roots remain and the earlier placement returns while that row still has no editable source. Weak observers separately prove the old kernel values are gone.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32fj-proof
npm --prefix ../session_tests run course -- save 32fj-proof
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

This proof covers the current immutable imported mesh subset. The full source reload flow and all production geometry/resource families remain required.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32fj-proof
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
