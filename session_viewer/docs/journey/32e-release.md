# 32e · Prove release does not retain the old document

**Typing: 14–28 minutes.** [Estimate](typing-load.md).

Finish the ownership argument with imported documents and retained history, not just generated demo triangles. The source-only state check records Weak observers across active and undo/redo roots, closes, and verifies that none can recover an old value. Reopening must issue fresh local IDs.

## Type

Continue from [Close through the command line and revoke reads](32d-command.md). [Save or recover your work](recovery.md).

### 1. `src/editor.rs`

Verify imported source/document release across history and fresh identities after reopening.

<details>
<summary>Locate the existing block</summary>

```rust
    #[test]
    fn resetting_the_view_preserves_its_current_aspect() {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32e-release-01.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Run the release checks below. After Close and empty GPU synchronization, weak observers must find no old document or display owner. The viewer can still open another file.

**Verified checkpoint in Chrome.**

![Actual browser result: Prove release does not retain the old document.](../screenshots/journey/32e-release-browser.png)

[Verification scope](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Code explanation and diagram</summary>

The native GPU proof makes the two release moments explicit. Before synchronization, old GPU geometry still owns the CPU displays used by its buffers. After synchronizing the empty scene, its Weak observers cannot upgrade, live document buffer usage is zero, and the cumulative upload counters have not been reset.

Weak old owners → Close → editor roots gone → empty GPU synchronization → fresh document IDs.

![Old source and document values disappear at editor close; GPU-held displays disappear at empty synchronization.](../illustrations/journey-32e.svg)

What does a zero live document ledger prove, and what does it not prove?

It proves the scoped roots and document buffer owners counted by this viewer are gone. Weak checks separately prove the old managed source/display values are dropped. It does not establish process RSS or immediate driver-memory reclamation: other renderer resources, queued GPU work, allocator caches and weak control allocations have separate lifetimes.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Keep only Weak observers of an old imported document, then close it. Explain why they can prove the value was dropped while still retaining small control allocations themselves. Do not use process memory graphs as a substitute for ownership checks.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 32e-release
npm --prefix ../session_tests run course -- save 32e-release
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Next we retain display rows while unloading editable source data, then rehydrate that source for editing. Browser listener shutdown, device recovery and all production source/resource families remain separate required lessons.

Chrome checks both a late successful read and a late rejected read after Close. Neither may change scene rows, history or result text. A new file request must still succeed afterwards. It also refuses Save while empty without downloading a file.

The CPU ledger remains scoped to mesh ownership and derived display-vector capacity. The GPU ledger counts document vertex/index/settings buffers. Production source categories, arena/texture accounting and source-only unload with rehydration remain explicitly planned; these checks do not replace those later acceptance requirements.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32e-release
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
