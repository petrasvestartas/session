# 32ge · Ignore old reload results before decoding them

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 16–31 minutes.** 23 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Reject results for closed imports, loaded rows, duplicate keys and previous release epochs.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Old import/release key → active-key check → false before payload decoding.

**Before you finish, explain:** Why deliver deliberately invalid bytes with an obsolete reload key?

Close and reimport the same file with the same original GUIDs. The import Origin is a new allocation, so the previous key must be ignored. Then hydrate the current release, deliver it twice, release again and deliver the previous epoch. Every obsolete completion returns false.

A request must contain at most one candidate per origin. Duplicate keys are rejected before preparing either candidate. An empty batch also has no effect. Keep the current rows and their display identities throughout.

These checks establish the native adoption contract. Browser request cancellation and automatic Move/Delete/Save replay are still pending; the browser at this endpoint continues to demonstrate Unload Sources.

`Ok(false)` means this result no longer belongs to a current request; it is not a document error. `Err(message)` means a current candidate failed validation. Keeping those outcomes separate lets a later browser callback silently discard obsolete completions while reporting failures for the current reload. A captured key owns its Origin, so the next request-lifetime lesson must also drop that owner when cancelling work.

![Reject results for closed imports, loaded rows, duplicate keys and previous release epochs.](../illustrations/journey-32ge.svg)

## Type the change

Continue [Reject stale or inconsistent source batches atomically](32gd-rejections.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-32ge-stale`. A save keeps your own work; it does not fill in the next lesson.

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

When Trunk reloads after a code change, the viewer starts with its generated demo again. From `workspace/journey`, make the sample using the example you already typed:

```sh
REGEN_PROTO=0 cargo run --example sample --locked -j4
```

Type `Open Replace`, choose `sample.pb`, then type `Select Next` twice, `Move 0.35,0,0.25`, `View Isometric` and `Fit`. You now have a moved object from a retained, reloadable file.

Run the stale-result checks with invalid bytes and verify that no stale key triggers decoding or changes current row residency.

**Actual Chrome screenshot.**

The browser checks the existing command-only unload behavior and retained drawing. Source hydration at this endpoint is verified by the native state/GPU checks; browser fetch and automatic command replay are still pending.

![Actual browser result: Ignore old reload results before decoding them.](../screenshots/journey/32ge-stale-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Use the current key with invalid bytes instead. Predict why the error now concerns the source version rather than an obsolete request.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

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

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Request ownership, import identity and release epochs must all survive asynchronous timing. The next lessons add that browser boundary to this checked state API.

[Validation status and course release](release.md).
