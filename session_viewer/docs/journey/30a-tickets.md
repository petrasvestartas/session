# 30a · Give a pending read an explicit ticket

**Typing: 18–35 minutes.** [Estimate](typing-load.md).

The browser currently compares a loose counter before delivering a read. We need to express more than newest number: a cancelled or already completed read must no longer be allowed to publish a result.

Create ReadGate as ordinary Rust. begin issues a checked increasing ticket and makes it the sole pending read. finish returns true only for that pending ticket and consumes it. cancel removes pending ownership without reusing the issued number.

## Type

Continue from [Report the result that actually committed](30-feedback.md). [Save or recover your work](recovery.md).

### 1. `src/read_gate.rs`

Own one pending ticket and test stale, duplicate, cancelled and exhausted requests independently of browser APIs.

Create the file and type:

```rust
--8<-- "journey/code/30a-tickets-01.rs"
```

### 2. `src/lib.rs`

Expose the native request model without changing the browser adapter yet.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod document;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30a-tickets-02.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Run the ReadGate checks below. Start two tickets and complete the older one first: only the current ticket may be accepted. Browser integration follows next.

**Verified checkpoint in Chrome.**

![Actual browser result: Give a pending read an explicit ticket.](../screenshots/journey/30a-tickets-browser.png)

[Verification scope](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Code explanation and diagram</summary>

Option distinguishes no pending read from a particular ticket. The counter names requests, while pending determines which request may still commit. Those are separate responsibilities.

begin clears pending before checking the next number. If the counter is exhausted, it returns an error and does not leave an older read able to commit after a refused new request. It never wraps back to an ID a previous operation might still hold.

The checks simulate completions in the wrong order, cancellation, duplicate completion and counter exhaustion. They need no window or GPU. This checkpoint tests the ticket model; the next lesson adopts it in the running file adapter.

Begin read → pending ticket → supersede, cancel or finish once.

![One pending ticket can be superseded, cancelled or consumed once; a stale ticket cannot clear a newer one.](../illustrations/journey-30a.svg)

Why must a stale completion leave the newer pending ticket intact?

The old operation owns only its own ticket. If rejecting it also cleared pending state, it would cancel the newer operation. finish compares IDs first, consumes only the matching current ticket, and rejects duplicate completions after it has been consumed.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Start three tickets and finish them in reverse order. Predict why only the newest succeeds and why the rejected older completions cannot clear it. Restore the original checks.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 30a-tickets
npm --prefix ../session_tests run course -- save 30a-tickets
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Production load generations reject old asynchronous results. This small gate also models cancellation and one-shot delivery, preparing the same ownership rule for browser reads and later asynchronous picking.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 30a-tickets
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
