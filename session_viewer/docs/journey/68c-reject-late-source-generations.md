# 12.3.4 · Reject late source generations

**Chapter 12 · Files and streaming · Section 12.3 · Streaming page budgets and rendering** · [Contents](contents.md)

**Typing: 12–24 minutes.** [Estimate](typing-load.md).

Give concurrent reads distinct tickets. Close invalidates every outstanding ticket without wrapping counters.

## Type

Continue from [Evict unpinned pages in access order](68b-evict-unpinned-pages-in-access-order.md). [Save or recover your work](recovery.md).

### 1. `src/stream_generation.rs`

Check source generation and exactly one completion per request.

Create the file and type:

```rust
--8<-- "journey/code/68c-reject-late-source-generations-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod page_cache;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/68c-reject-late-source-generations-02.rs" skip=21
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/stream_generation.rs`

Copy concurrent completion, cancellation and overflow tests.

<details>
<summary>Locate the existing block</summary>

```text
    pub fn close(&mut self) {self.active=false;self.pending.clear();}
    pub fn pending(&self)->usize {self.pending.len()}
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/68c-reject-late-source-generations-03.rs" skip=125
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.3.4
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

The mixed original CAD, surface and coloured cloud scene remains visible.

![Native renderer output for reject late source generations.](../screenshots/journey/66af-connect-browser-save-open-url-adoption.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [12.1.44 · Connect browser save open and URL adoption](66af-connect-browser-save-open-url-adoption.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



source generation → request ticket → current completion → one adoption.

![Reject late source generations.](../illustrations/68c-reject-late-source-generations.svg)

Can an old read finish into a reopened source?

No. Reopening changes the generation; completion must still belong to the active generation and pending request.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Complete concurrent reads out of order, close, reopen and refuse old tickets or exhausted counters.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.3.4
npm --prefix ../session_tests run course -- save 12.3.4
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The existing mixed original archive browser exercise remains authored and unrun. The final cache endpoint adds real ranged URL loading, response validation, cancellation and cleanup exercises, also awaiting browser execution.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 12.3.4
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
