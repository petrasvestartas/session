# 12.2.13 · Validate exact HTTP range replies

**Chapter 12 · Files and streaming · Section 12.2 · Bounded read scheduling and cancellation** · [Contents](contents.md)

**Typing: 14–28 minutes.** [Estimate](typing-load.md).

Compute checked inclusive HTTP intervals, then match response offsets, lengths and total file size.

## Type

Continue from [Poll and cancel original queued range reads](67a-queue-range-reads-in-arrival-order.md). [Save or recover your work](recovery.md).

### 1. `src/range_rules.rs`

Validate the source interval before issuing or accepting a read.

Create the file and type:

```rust
--8<-- "journey/code/67b-validate-exact-http-range-replies-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod range_gate;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67b-validate-exact-http-range-replies-02.rs" skip=21
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/range_rules.rs`

Copy exact-response, EOF-probe and overflow regression fixtures.

<details>
<summary>Locate the existing block</summary>

```text
    Ok(total)
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/67b-validate-exact-http-range-replies-03.rs" skip=16
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.2.13
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

The mixed source archive scene remains unchanged.

![Native renderer output for validate exact http range replies.](../screenshots/journey/66af-connect-browser-save-open-url-adoption.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [12.1.44 · Connect browser save open and URL adoption](66af-connect-browser-save-open-url-adoption.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



requested interval → HTTP206 → matching Content-Range → accepted bytes.

![Validate exact HTTP range replies.](../illustrations/67b-validate-exact-http-range-replies.svg)

May a server return the whole file instead of the requested range?

No. Refuse HTTP200 without downloading its unwanted body. Only the initial probe may stop at an earlier file end.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Check wrong offsets, missing headers, incomplete chunks, invalid totals and u64 overflow.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.2.13
npm --prefix ../session_tests run course -- save 12.2.13
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The mixed-source Save/Open browser contract remains authored and unrun. These preparatory range helpers connect to actual browser streaming and page drawing in68.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 12.2.13
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
