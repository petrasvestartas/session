# 32gid · Validate restoration before replaying the command

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 30–59 minutes.** 56 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Complete source restoration and replay the captured operation only after current document validation.

**Follow:** Reply → complete bodies → current keys and versions → hydrate → captured Intent → edit result.

The reply already pairs complete bodies with the operation captured before fetching. complete now gives it one synchronous editor boundary. Hydration validates current source identity, release epoch, file version and metadata before it restores any source. A stale context returns None; a failed fetch or invalid body returns an error. Neither path replays the operation.

After successful hydration, Some(intent) calls the explicit-target replay taught earlier. Move and Delete each create one edit transaction; Save produces the original-precision document bytes without creating history. None represents explicit Reload Sources and returns a scene result. The browser will consume these results in the next checkpoint.

![Validate before replay](../illustrations/journey-32gid.svg)

## Type the change

Continue from [Carry captured intent through browser completion](32gicb-bridge.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-32gid-complete` (from `session_viewer`).

### 1. `src/reload_reply.rs`

Validate every current source key and body before replay; a stale context produces no reply.

<details>
<summary>Locate the existing block</summary>

```rust
impl Reply {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32gid-complete-01.rs"
```

### 2. `src/lib.rs`

Register native completion checks separately from the body-pairing checks.

<details>
<summary>Locate the existing block</summary>

```rust
mod reload_reply_tests;
```

</details>

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

Run the completion checks below. A valid batch restores sources before replaying its captured edit. Failed or stale batches must perform no edit or download.

**Verified checkpoint in Chrome.**

![Actual browser result: Validate restoration before replaying the command.](../screenshots/journey/32gid-complete-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Move intent.replay above hydrate and run the failed/stale completion checks. Explain which edits or downloads could escape document validation.

</details>

## Explain the change

Why does a stale restoration return None instead of replaying its captured Save or edit?

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

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

One editor completion boundary validates restoration before applying a captured operation; fetch itself never mutates selection, camera or history.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Native checks complete all three cold-source operations, change selection and camera while waiting, verify the original target and one-step Undo, and reject a closed document and a network failure. Existing replay checks cover exact source doubles and Save history. Chrome still checks explicit browser reload because automatic command routing is connected next.

Native checks complete captured cold-source Move/Delete/Save and reject stale or failed restoration. Chrome retains explicit reload until automatic command routing is connected next.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32gid-complete
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
