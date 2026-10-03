# 32gibb · Route captured Move, Delete and Save results

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 25–50 minutes.** 51 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Return a scene change or original-precision save bytes from a captured intent.

**Follow:** Intent → explicit-target edit or original source snapshot → Reply::Changed or Reply::Saved.

Add edit_replay as a separate module. A second impl Intent block adds replay without changing capture or scope discovery. Reply carries either a Change or saved bytes, letting the future browser completion handler decide whether to update the renderer or start a download.

Move and Delete call the explicit-target editor methods already proved. Save calls document::snapshot on the current scene; that function reads original kernel doubles and current placements. It neither changes selection/camera nor adds history. Result::map wraps successful values while preserving existing errors. The enum is a Rust tagged choice: matching Saved exposes bytes; Changed exposes a renderer change.

Replay itself does not authorize an asynchronous completion. The next lesson pairs the intent with its pending ticket; only a current completion may take and execute it.

![Route an intent reply](../illustrations/journey-32gibb.svg)

## Type the change

Continue from [Delete the original target while keeping later selection](32giba-delete.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-32gibb-reply` (from `session_viewer`).

### 1. `src/edit_replay.rs`

Return changed-state feedback or save bytes without confusing a download with a history edit.

Create the file and type:

```rust
--8<-- "journey/code/32gibb-reply-01.rs"
```

### 2. `src/lib.rs`

Register the captured-intent result module.

Find this exact block:

```rust
pub mod edit_intent;
```

Replace that block with:

```rust
--8<-- "journey/code/32gibb-reply-02.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the reply checks below. Move/Delete return a scene change; Save returns original-source bytes without adding an Undo step.

**Verified checkpoint in Chrome.**

![Actual browser result: Route captured Move, Delete and Save results.](../screenshots/journey/32gibb-reply-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Call History::edit during Save, then run the native test and explain why Undo now observes an extra history step.

</details>

## Explain the change

Why must Save return bytes without adding a document history entry?

<details>
<summary>Compare your explanation</summary>

Saving observes the current document. Undo must still undo the last modeling operation; returning original-source bytes preserves precision without turning a download into an edit.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32gibb-reply
npm --prefix ../session_tests run course -- save 32gibb-reply
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

The pending owner will consume an intent once, then route changed-state feedback or save bytes. This lesson defines only execution and result values.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Native checks replay all three intents, compare saved source coordinates exactly and prove Save does not consume Move Undo. Cold Save fails while preserving Redo. Chrome checks the existing normal Save download and Move Undo path; browser completion has not yet been connected to this replay method.

Chrome downloads through normal Save and proves Move Undo remains available. Native tests execute all three captured intents and compare saved doubles. Owned asynchronous completion/replay remains pending.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32gibb-reply
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
