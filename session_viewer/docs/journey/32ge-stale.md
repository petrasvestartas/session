# 3.16.25 · Ignore old reload results before decoding them

**Chapter 3 · Documents, editing and ownership · Section 3.16 · Source ownership and restoration** · [Contents](contents.md)

**Typing: 0–0 minutes.** [Estimate](typing-load.md).

Close and reimport the same file with the same original GUIDs. The import Origin is a new allocation, so the previous key must be ignored. Then hydrate the current release, deliver it twice, release again and deliver the previous epoch. Every obsolete completion returns false.

## Type

Continue from [Reject stale or inconsistent source batches atomically](32gd-rejections.md). [Save or recover your work](recovery.md).

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/rehydrate_tests.rs`

Optional authoring fixture. Show that obsolete identities are rejected before version checks, decoding or mutation.

<details>
<summary>Locate the existing block</summary>

```text
use crate::editor::{Action, Editor};
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/32ge-stale-01.rs" skip=37
```

</details>

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The supplied cases distinguish an obsolete reply from an invalid current candidate. They add no browser request or restoration behavior at this checkpoint.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for ignore old reload results before decoding them.](../screenshots/journey/32ge-stale.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>

A request must contain at most one candidate per origin. Duplicate keys are rejected before preparing either candidate. An empty batch also has no effect. Keep the current rows and their display identities throughout.

These checks establish the native adoption contract. Browser request cancellation and automatic Move/Delete/Save replay are still pending; the browser at this endpoint continues to demonstrate Unload Sources.

`Ok(false)` means this result no longer belongs to a current request; it is not a document error. `Err(message)` means a current candidate failed validation. Keeping those outcomes separate lets a later browser callback silently discard obsolete completions while reporting failures for the current reload. A captured key owns its Origin, so the next request-lifetime lesson must also drop that owner when cancelling work.

Old import/release key → active-key check → false before payload decoding.

![Reject results for closed imports, loaded rows, duplicate keys and previous release epochs.](../illustrations/journey-32ge.svg)

Why deliver deliberately invalid bytes with an obsolete reload key?

A false result instead of a decode error proves the identity check happens first. Invalid stale payloads cannot revive a closed import or disturb the current source release.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Read the stale-key example with the request owner from the preceding lesson. Identify which identity changes after Close/reimport and which epoch changes after another release. Then contrast an obsolete reply with invalid bytes for the current key.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 3.16.25
npm --prefix ../session_tests run course -- save 3.16.25
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Request ownership, import identity and release epochs must all survive asynchronous timing. The next lessons add that browser boundary to this checked state API.

The browser checks the existing command-only unload behavior and retained drawing. Source hydration at this endpoint is verified by the native state/GPU checks; browser fetch and automatic command replay are still pending.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 3.16.25
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
