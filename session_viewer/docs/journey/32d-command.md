# 32d · Close through the command line and revoke reads

**Typing: 5–9 minutes.** [Estimate](typing-load.md).

Expose typed `Close` through the existing dock. Cancel `ReadGate` before returning `Action::Close`, so a pending read cannot refill the closed document.

## Type

Continue from [Close the document without resetting the view](32c-close.md). [Save or recover your work](recovery.md).

### 1. `src/browser.rs`

Offer closing only through the actual dock vocabulary.

<details>
<summary>Locate the existing block</summary>

```rust
            "Cancel Open",
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32d-command-01.rs"
```

### 2. `src/browser.rs`

Revoke pending file ownership before the editor closes.

<details>
<summary>Locate the existing block</summary>

```rust
            if line == "cancel open" {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32d-command-02.rs"
```

### 3. `src/browser.rs`

Retain the submitted operation kind across action consumption.

<details>
<summary>Locate the existing block</summary>

```rust
            let replacement = matches!(&action, Action::Replace(_));
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32d-command-03.rs"
```

### 4. `src/browser.rs`

Report close only after its scene operation and renderer synchronization succeed.

<details>
<summary>Locate the existing block</summary>

```rust
                    if event.type_() == "viewer-file" {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32d-command-04.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Open `sample.pb`, then type `Close`. The drawing becomes white and both history branches clear. Type `Open` again: the same page must remain usable.

**Verified checkpoint in Chrome.**

![Actual browser result: Close through the command line and revoke reads.](../screenshots/journey/32d-command-browser.png)

[Verification scope](release.md).

Run the state checks on your computer from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked --target host-tuple -j4
```

<details>
<summary>Code explanation and diagram</summary>

Remember the action kind before `apply` consumes it. Report “Document closed” only after the scene change and renderer synchronization succeed.

Close drops document and history owners while preserving camera, background and the GPU device. Page-exit disposal later ends the viewer itself; Close leaves it ready for another Open.

Typed Close → read-ticket cancellation → editor close → empty GPU rows → dock result.

![Close cancels delivery ownership before clearing editor and GPU document roots.](../illustrations/journey-32d.svg)

Why must Close revoke a pending read before it drops the document?

A read can finish after the old document was closed. Without ticket cancellation, its result would import old data into the now-empty scene. Revoke the request at command execution; the file task checks ownership before delivery, so both late success and late failure remain silent.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Start a file read, then close before it resolves. Predict why cancelling the ticket must happen before a late result can import into the empty scene. Compare this with merely clearing visible rows.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 32d-command
npm --prefix ../session_tests run course -- save 32d-command
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The browser owns asynchronous delivery while the editor owns document close. Keeping those responsibilities explicit will also protect later released-source reloads and viewer shutdown.

Chrome holds a real File.arrayBuffer result, types Close, and releases the old promise afterwards. The scene must stay empty and command history must not acquire a late result. It also tries Undo and Redo after close, checks live CPU/GPU document ledgers, and imports a new specimen without reloading the page.

The Chrome check verifies an empty drawing before reopening. Weak observers check source and GPU ownership release; the final screenshot shows the reopened specimen, not release itself.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32d-command
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
