# 32gibb · Route captured Move, Delete and Save results

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 25–50 minutes.** 51 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Return a scene change or original-precision save bytes from a captured intent.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Intent → explicit-target edit or original source snapshot → Reply::Changed or Reply::Saved.

**Before you finish, explain:** Why must Save return bytes without adding a document history entry?

Add edit_replay as a separate module. A second impl Intent block adds replay without changing capture or scope discovery. Reply carries either a Change or saved bytes, letting the future browser completion handler decide whether to update the renderer or start a download.

Move and Delete call the explicit-target editor methods already proved. Save calls document::snapshot on the current scene; that function reads original kernel doubles and current placements. It neither changes selection/camera nor adds history. Result::map wraps successful values while preserving existing errors. The enum is a Rust tagged choice: matching Saved exposes bytes; Changed exposes a renderer change.

Native checks replay all three intents, compare saved source coordinates exactly and prove Save does not consume Move Undo. Cold Save fails while preserving Redo. Chrome checks the existing normal Save download and Move Undo path; browser completion has not yet been connected to this replay method.

Replay itself does not authorize an asynchronous completion. The next lesson pairs the intent with its pending ticket; only a current completion may take and execute it.

![Route an intent reply](../illustrations/journey-32gibb.svg)

## Type the change

Continue [Delete the original target while keeping later selection](32giba-delete.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-32gibb-reply`. A save keeps your own work; it does not fill in the next lesson.

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

From your project, run `REGEN_PROTO=0 cargo run --example sample --locked -j4`. Type Open Replace, choose sample.pb, Select Next twice, Move 0.35,0,0.25, View Isometric, Orbit Right, Orbit Up, Move 1.92,0,0.93, Move 0.25,0,-0.15 and Fit. Unload Sources and Reload Sources must retain this drawing. Automatic Move/Delete/Save reload is still pending. Then type Move 0,0.25,0 and Fit. Inspect the native scope tests to compare original-target requests with Save requests. Then type Move 0.25,0,0.15, Undo and Redo. Undo restores the preceding drawing; Redo restores the move. Type Fit afterward. Type Delete, Undo, Select Next twice and Redo. Undo restores the geometry; selection must be chosen again after deleting the selected row. Finally type Select Next and Fit to inspect a remaining object. Move 0.15,0,0, Save, Undo and Redo; Save must leave the Move available to Undo. Finally type Fit.

**Actual Chrome screenshot.**

Chrome downloads through normal Save and proves Move Undo remains available. Native tests execute all three captured intents and compare saved doubles. Owned asynchronous completion/replay remains pending.

![Actual browser result: Route captured Move, Delete and Save results.](../screenshots/journey/32gibb-reply-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Call History::edit during Save, then run the native test and explain why Undo now observes an extra history step.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

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

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

The pending owner will consume an intent once, then route changed-state feedback or save bytes. This lesson defines only execution and result values.

[Validation status and course release](release.md).
