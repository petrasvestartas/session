# 32gi · Capture the requested edit before waiting

**Typing: 30–60 minutes.** [Estimate](typing-load.md).

A delayed edit needs a small record of the user’s request. Intent contains only a stable ObjectId and copied Move arguments, or Save. It owns no geometry, origin or URL. Changing selection cannot redirect that value.

## Type

Continue from [Cancel reloads when their document context changes](32gha-cancel.md). [Save or recover your work](recovery.md).

### 1. `src/lib.rs`

Register the value-only requested-edit module.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod reload_job;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32gi-capture-01.rs"
```

### 2. `src/edit_intent.rs`

Capture the requested target and arguments without retaining any source owners.

Create the file and type:

```rust
--8<-- "journey/code/32gi-capture-02.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Run the intent checks below. Capture Move for one selected object, change selection, and compare the stored target and offset. Automatic browser replay comes later.

**Verified checkpoint in Chrome.**

![Actual browser result: Capture the requested edit before waiting.](../screenshots/journey/32gi-capture-browser.png)

[Verification scope](release.md).

Run the state checks on your computer from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked --target host-tuple -j4
```

<details>
<summary>Code explanation and diagram</summary>

Unselected Delete, zero-offset Move and ordinary view commands do not create an edit intent; invalid Move values are refused. Save is represented explicitly because it will need every active cold origin instead of one selected target. Source selection and replay are added in the following checkpoints.

Command + original selection → value-only Intent → unchanged target while waiting.

![Capture identity before waiting](../illustrations/journey-32gi.svg)

Why store ObjectId and Move offset rather than selection or a precomputed model?

Selection can change while I/O waits. The original ObjectId names the requested target; the offset expresses the requested change. A later replay must read that target’s current placement rather than replace it with a stale matrix.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Capture Move for one object, change selection, then inspect the captured ObjectId in the native test. Explain which origin the request must load.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 32gi-capture
npm --prefix ../session_tests run course -- save 32gi-capture
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Separate requested intent from current interaction state. Source tickets authorize the completion; the captured target determines which edit may be replayed.

This checkpoint introduces and tests the request record. It does not yet intercept browser edits or replay them. Chrome checks the existing explicit reload path and its retained drawing; native tests prove captured target identity and that Close releases sources despite a retained intent.

The existing explicit source reload preserves this checkpoint’s placed drawing. Captured-intent identity and release ownership are checked natively; automatic browser replay is introduced next.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32gi-capture
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
