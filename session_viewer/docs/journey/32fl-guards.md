# 32fl · Protect history and future reload tickets

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 25–50 minutes.** 42 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Verify whole-origin protection, independent duplicate imports and checked release epochs.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Modified history origin → protection; independent origin → release; exhausted epoch → no mutation.

**Before you finish, explain:** Why is a release epoch kept across Close rather than reset for each document?

Add the cases that a simple active-row check would miss. A modified metadata row exists only in history, while the active original still looks eligible. Protect that entire origin. A second import of identical bytes has its own Origin and can unload independently.

![Protected historical origin and checked epochs prevent wrong source adoption.](../illustrations/journey-32fl.svg)

The epoch test forces exhaustion and verifies that no row becomes cold. It then releases, closes, reopens and releases again: the new rows receive the next epoch rather than reusing one.

Chrome keeps the command-only unloading proof, checks that a second unload refuses without changing pixels, and reopens in the same page before unloading again. The native GPU proof checks old kernel expiration with live display/GPU owners. These checks prepare guarded reload delivery; the fetch, cancellation and automatic edit replay are still next.

## Type the change

Continue [Unload editable sources through the command line](32fk-command.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-32fl-guards`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/editor.rs`

Check modified history, independent imports and exhausted/reopened release issuance.

Find this exact block:

```rust
    #[test]
    fn close_ends_history_and_keeps_issued_ids_and_view_settings() {
```

Replace that block with:

```rust
--8<-- "journey/code/32fl-guards-01.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the history-protection and epoch checks, then use Unload Sources twice and Close/reopen in Chrome. Failed or repeated unloads must not alter the drawing or current release identity.

**Actual Chrome screenshot.**

![Actual browser result: Protect history and future reload tickets.](../screenshots/journey/32fl-guards-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

## Try one small experiment

Reset the issuance counter inside Close in a scratch copy and predict the reopened epoch. Explain why origin identity alone should not replace per-release ownership.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

A fetch can finish after Close. A later document must not reuse the old release token. Keep checked issuance in the viewer lifetime, and later compare the requested origin and its current released epoch before adopting a result.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32fl-guards
npm --prefix ../session_tests run course -- save 32fl-guards
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Each source release has both an import identity and a viewer-issued epoch. The rehydration flow must verify both and its own read ticket before adopting data.

[Validation status and course release](release.md).
