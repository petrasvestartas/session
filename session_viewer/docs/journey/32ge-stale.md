# 32ge · Ignore old reload results before decoding them

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 16–31 minutes.** 23 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Reject results for closed imports, loaded rows, duplicate keys and previous release epochs.

**Follow:** Old import/release key → active-key check → false before payload decoding.

Close and reimport the same file with the same original GUIDs. The import Origin is a new allocation, so the previous key must be ignored. Then hydrate the current release, deliver it twice, release again and deliver the previous epoch. Every obsolete completion returns false.

A request must contain at most one candidate per origin. Duplicate keys are rejected before preparing either candidate. An empty batch also has no effect. Keep the current rows and their display identities throughout.

These checks establish the native adoption contract. Browser request cancellation and automatic Move/Delete/Save replay are still pending; the browser at this endpoint continues to demonstrate Unload Sources.

`Ok(false)` means this result no longer belongs to a current request; it is not a document error. `Err(message)` means a current candidate failed validation. Keeping those outcomes separate lets a later browser callback silently discard obsolete completions while reporting failures for the current reload. A captured key owns its Origin, so the next request-lifetime lesson must also drop that owner when cancelling work.

![Reject results for closed imports, loaded rows, duplicate keys and previous release epochs.](../illustrations/journey-32ge.svg)

## Type the change

Continue from [Reject stale or inconsistent source batches atomically](32gd-rejections.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-32ge-stale` (from `session_viewer`).

### 1. `src/rehydrate_tests.rs`

Show that obsolete identities are rejected before version checks, decoding or mutation.

Find this exact block:

```rust
use crate::editor::{Action, Editor};
```

Replace that block with:

```rust
--8<-- "journey/code/32ge-stale-01.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the stale-key checks below. Closed imports, loaded rows, duplicate replies and previous release epochs must reject an obsolete restoration.

**Verified checkpoint in Chrome.**

![Actual browser result: Ignore old reload results before decoding them.](../screenshots/journey/32ge-stale-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Use the current key with invalid bytes instead. Predict why the error now concerns the source version rather than an obsolete request.

</details>

## Explain the change

Why deliver deliberately invalid bytes with an obsolete reload key?

<details>
<summary>Compare your explanation</summary>

A false result instead of a decode error proves the identity check happens first. Invalid stale payloads cannot revive a closed import or disturb the current source release.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32ge-stale
npm --prefix ../session_tests run course -- save 32ge-stale
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

Request ownership, import identity and release epochs must all survive asynchronous timing. The next lessons add that browser boundary to this checked state API.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

The browser checks the existing command-only unload behavior and retained drawing. Source hydration at this endpoint is verified by the native state/GPU checks; browser fetch and automatic command replay are still pending.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32ge-stale
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
