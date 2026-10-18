# 12.3.26 · Prepare a coarse pin and bounded prefix

**Chapter 12 · Files and streaming · Section 12.3 · Streaming page budgets and rendering** · [Contents](contents.md)

**Typing: 20–40 minutes.** [Estimate](typing-load.md).

Read the original coarse node and an initial prefix after reserving checked temporary memory.

## Type

Continue from [Adopt remote original revisions independently](68w-adopt-remote-original-revisions-independently.md). [Save or recover your work](recovery.md).

### 1. `src/stream_load.rs`

Prepare a pinned original coarse page and a budgeted initial prefix.

Create the file and type:

```rust
--8<-- "journey/code/68x-prepare-a-coarse-pin-and-bounded-prefix-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(target_arch = "wasm32")]
pub mod source_unpack;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/68x-prepare-a-coarse-pin-and-bounded-prefix-02.rs" skip=54
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.3.26
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

![Native renderer output for prepare a coarse pin and bounded prefix.](../screenshots/journey/66af-connect-browser-save-open-url-adoption.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [12.1.44 · Connect browser save open and URL adoption](66af-connect-browser-save-open-url-adoption.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



source metadata → coarse pin → bounded prefix → initial source owner.

![Prepare a coarse pin and bounded prefix.](../illustrations/68x-prepare-a-coarse-pin-and-bounded-prefix.svg)

May a huge coarse node bypass the initial read limit?

No. Refuse a coarse node that exceeds the initial page or memory limit instead of downloading the complete source.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace prefix reduction before allocation and inspect original IDs and colour cursors in the returned owner.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.3.26
npm --prefix ../session_tests run course -- save 12.3.26
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The existing mixed original archive browser exercise remains authored and unrun. The final cache endpoint adds real ranged URL loading, response validation, cancellation and cleanup exercises, also awaiting browser execution.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 12.3.26
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
