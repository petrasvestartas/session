# 12.3.5 · Bound source prefixes and decode work

**Chapter 12 · Files and streaming · Section 12.3 · Streaming page budgets and rendering** · [Contents](contents.md)

**Typing: 7–14 minutes.** [Estimate](typing-load.md).

Use explicit point ceilings and reserve temporary original/derived decode work before reading arrays.

## Type

Continue from [Reject late source generations](68c-reject-late-source-generations.md). [Save or recover your work](recovery.md).

### 1. `src/stream_budget.rs`

Limit point ranges and reserve checked decode work.

Create the file and type:

```rust
--8<-- "journey/code/68d-bound-source-prefixes-and-decode-work-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod stream_generation;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/68d-bound-source-prefixes-and-decode-work-02.rs" skip=28
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/stream_budget.rs`

Copy range ceiling and allocation overflow fixtures.

<details>
<summary>Locate the existing block</summary>

```text
}
pub fn temporary_page_bytes(count:u32)->Option<usize> {usize::try_from(count).ok()?.checked_mul(160)}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/68d-bound-source-prefixes-and-decode-work-03.rs" skip=104
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.3.5
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

![Native renderer output for bound source prefixes and decode work.](../screenshots/journey/66af-connect-browser-save-open-url-adoption.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [12.1.44 · Connect browser save open and URL adoption](66af-connect-browser-save-open-url-adoption.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



source count → point ceiling → checked temporary bytes → bounded page.

![Bound source prefixes and decode work.](../illustrations/68d-bound-source-prefixes-and-decode-work.svg)

Does the page budget measure all browser memory?

No. It bounds charged CPU page work. GPU allocations, metadata, old/new staged scenes and browser overhead require separate measurement.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare a2m page estimate with the256MiB budget and reduce the read before allocation.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.3.5
npm --prefix ../session_tests run course -- save 12.3.5
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The existing mixed original archive browser exercise remains authored and unrun. The final cache endpoint adds real ranged URL loading, response validation, cancellation and cleanup exercises, also awaiting browser execution.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 12.3.5
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
