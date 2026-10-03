# 32gd · Reject stale or inconsistent source batches atomically

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 25–49 minutes.** 38 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Keep rows cold on changed versions, missing source identity and obsolete release keys.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Prepare two origins → reject changed second source → no adoption → successful retry.

**Before you finish, explain:** Should a late result after Close be decoded before deciding that it is obsolete?

A successful round trip is not enough. Use duplicate imports with independent origins and validate all candidates before adoption. Change the second source version and confirm neither origin becomes loaded. A wrong original source GUID in a retained row must also reject the candidate.

Close and reimport the same bytes, then deliver an old key with malformed bytes: the stale result must be ignored without even trying to decode it. Restore one origin, release again and verify the old epoch no longer matches; a duplicate completion of an already adopted key is also ignored.

These are native acceptance checks for the atomic API. Fetch errors, aborts and queued edits belong to the next browser request boundary.

`Rc::make_mut` gives this row its own metadata value when another history row shares the original. That lets the check simulate an inconsistent retained row without changing the imported file. The changed byte version and the changed row metadata exercise separate rejection boundaries.

![Keep rows cold on changed versions, missing source identity and obsolete release keys.](../illustrations/journey-32gd.svg)

## Type the change

Continue [Prove source restoration preserves display and history](32gc-roundtrip.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-32gd-rejections`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/rehydrate_tests.rs`

Reject the entire candidate batch before adopting any source and preserve displayed owners.

Find this exact block:

```rust
use crate::editor::{Action, Editor};
```

Replace that block with:

```rust
--8<-- "journey/code/32gd-rejections-01.rs"
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

Run the rejection cases and verify unchanged row IDs, model data, display identities and source availability after each failed or stale batch.

**Actual Chrome screenshot.**

The browser checks the existing command-only unload behavior and retained drawing. Source hydration at this endpoint is verified by the native state/GPU checks; browser fetch and automatic command replay are still pending.

![Actual browser result: Reject stale or inconsistent source batches atomically.](../screenshots/journey/32gd-rejections-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Change the second candidate in a two-origin batch to malformed bytes. Predict why validating all candidates first keeps the first origin cold.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

No. First confirm the captured key still matches an active cold row. An obsolete result returns false before source construction or mutation, whether its payload is valid or malformed.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32gd-rejections
npm --prefix ../session_tests run course -- save 32gd-rejections
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Late source delivery must never revive a closed or replaced import. Immutable file-version checks and origin/epoch checks address different causes of stale data.

[Validation status and course release](release.md).
