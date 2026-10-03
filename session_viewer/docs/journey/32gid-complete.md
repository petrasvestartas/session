# 32gid · Validate restoration before replaying the command

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 30–59 minutes.** 56 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Complete source restoration and replay the captured operation only after current document validation.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Reply → complete bodies → current keys and versions → hydrate → captured Intent → edit result.

**Before you finish, explain:** Why does a stale restoration return None instead of replaying its captured Save or edit?

The reply already pairs complete bodies with the operation captured before fetching. complete now gives it one synchronous editor boundary. Hydration validates current source identity, release epoch, file version and metadata before it restores any source. A stale context returns None; a failed fetch or invalid body returns an error. Neither path replays the operation.

After successful hydration, Some(intent) calls the explicit-target replay taught earlier. Move and Delete each create one edit transaction; Save produces the original-precision document bytes without creating history. None represents explicit Reload Sources and returns a scene result. The browser will consume these results in the next checkpoint.

Native checks complete all three cold-source operations, change selection and camera while waiting, verify the original target and one-step Undo, and reject a closed document and a network failure. Existing replay checks cover exact source doubles and Save history. Chrome still checks explicit browser reload because automatic command routing is connected next.

![Validate before replay](../illustrations/journey-32gid.svg)

## Type the change

Continue [Carry captured intent through browser completion](32gicb-bridge.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-32gid-complete`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/reload_reply.rs`

Validate every current source key and body before replay; a stale context produces no reply.

Find this exact block:

```rust
impl Reply {
```

Replace that block with:

```rust
--8<-- "journey/code/32gid-complete-01.rs"
```

### 2. `src/lib.rs`

Register native completion checks separately from the body-pairing checks.

Find this exact block:

```rust
mod reload_reply_tests;
```

Replace that block with:

```rust
--8<-- "journey/code/32gid-complete-02.rs"
```

### 3. `src/reload_complete_tests.rs`

Prove automatic native Move/Delete/Save completion, original target, later selection/camera, Undo, and stale or failed completion.

Create the file and type:

```rust
--8<-- "journey/code/32gid-complete-03.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

From your project, run `REGEN_PROTO=0 cargo run --example sample --locked -j4`. Type Open Replace, choose sample.pb, Select Next twice, Move 0.35,0,0.25, View Isometric, Orbit Right, Orbit Up, Move 1.92,0,0.93, Move 0.25,0,-0.15 and Fit. Unload Sources and Reload Sources must retain this drawing. Automatic Move/Delete/Save reload is still pending. Then type Move 0,0.25,0 and Fit. Inspect the native scope tests to compare original-target requests with Save requests. Then type Move 0.25,0,0.15, Undo and Redo. Undo restores the preceding drawing; Redo restores the move. Type Fit afterward. Type Delete, Undo, Select Next twice and Redo. Undo restores the geometry; selection must be chosen again after deleting the selected row. Finally type Select Next and Fit to inspect a remaining object. Move 0.15,0,0, Save, Undo and Redo; Save must leave the Move available to Undo. Finally type Fit. Type Move 0,0,0.15 and Fit. Inspect the native owner tests for stale, duplicate and cancelled completions. Type Orbit Right and Fit. Inspect the native completion checks before connecting automatic command replay. Type Move 0,0.15,0 and Fit. Automatic Move/Delete/Save reload remains the next checkpoint. Type Move 0.1,0,0 and Fit. Native tests now complete captured operations; browser automatic commands follow next.

**Actual Chrome screenshot.**

Native checks complete captured cold-source Move/Delete/Save and reject stale or failed restoration. Chrome retains explicit reload until automatic command routing is connected next.

![Actual browser result: Validate restoration before replaying the command.](../screenshots/journey/32gid-complete-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Move intent.replay above hydrate and run the failed/stale completion checks. Explain which edits or downloads could escape document validation.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

The source keys no longer match the current document and release epoch. The reply has lost authority; neither a scene change nor a download belongs to the replacement context.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32gid-complete
npm --prefix ../session_tests run course -- save 32gid-complete
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

One editor completion boundary validates restoration before applying a captured operation; fetch itself never mutates selection, camera or history.

[Validation status and course release](release.md).
