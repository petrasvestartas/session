# 32gha · Cancel reloads when their document context changes

**Typing: 16–31 minutes.** [Estimate](typing-load.md).

Cancel the reload before Close, another Unload Sources, Undo/Redo or replacement adoption. Open Replace also cancels at picker opening, matching the existing file-read boundary. Camera and selection remain available while a source request waits.

`matches!` recognizes actions that invalidate pending work. Borrow the reload owner, cancel it before applying the document action, then end that borrow. Late delivery cannot regain the old ticket’s authority.

## Type

Continue from [Restore editable sources through the command line](32gh-command.md). [Save or recover your work](recovery.md).

### 1. `src/browser.rs`

Invalidate the old reload as soon as replacement intent begins.

<details>
<summary>Locate the existing block</summary>

```rust
                read_mode.set(if line == "open replace" { crate::file_input::Mode::Replace }
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32gha-cancel-01.rs"
```

### 2. `src/browser.rs`

Remove reload keys and abort I/O before changing the current document context.

<details>
<summary>Locate the existing block</summary>

```rust
        if let Some(action) = action {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32gha-cancel-02.rs"
```

### 3. `examples/reload_check.rs`

Compare original geometry and flags with the real browser download using the typed protobuf reader.

Create the file and type:

```rust
--8<-- "journey/code/32gha-cancel-03.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Open `sample.pb` and unload its sources. Request `Reload Sources`, then `Cancel Reload` while it is pending. Its late reply must not restore the cancelled batch.

**Verified checkpoint in Chrome.**

![Actual browser result: Cancel reloads when their document context changes.](../screenshots/journey/32gha-cancel-browser.png)

[Verification scope](release.md).

Run the state checks on your computer from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked --target host-tuple -j4
```

<details>
<summary>Code explanation and diagram</summary>



Close / replacement / Undo / release → cancel flight → late result is inert.

![Cancel reloads when their document context changes](../illustrations/journey-32gha.svg)

Why cancel Open Replace at picker opening rather than after its file finishes reading?

The user has already chosen to replace the document context. Cancel the older reload before waiting for another picker or file read, so its delayed result cannot adopt into that context.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Cancel only after replacement adoption in a scratch copy. Explain the interval in which an older reload could still finish while the replacement picker or read is pending.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 32gha-cancel
npm --prefix ../session_tests run course -- save 32gha-cancel
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Context cancellation and source release-key checks complement one another. Neither a network abort nor a valid original file is enough to authorize an obsolete completion.

Chrome holds real fetch results while typing camera commands, Cancel Reload, Undo, Close and Open Replace. It releases both late successes and failures after invalidation and verifies no adopted source or new history reply. Closing drops the current source URL even if a delayed task still exists.

The same browser checker injects HTTP failures, oversized/stream-error bodies and changed versions, then performs a successful retry. These failures keep the display cold and pixel-identical. A real Blob retry restores source ownership without allocating geometry. Automatic requested-edit replay remains pending.

Type a small native `reload_check` example to compare the original file with the actual browser download. It compares original double vertices and topology by saved GUID, checks names/flags and requires separately stored placements. Run `REGEN_PROTO=0 cargo run --example reload_check -- sample.pb ~/Downloads/viewer.session` from your project. The Chrome checker invokes this same example on its actual saved file; it does not substitute a generated fixture.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32gha-cancel
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
