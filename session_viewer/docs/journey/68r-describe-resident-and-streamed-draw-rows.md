# 12.3.20 · Describe resident and streamed draw rows

**Chapter 12 · Files and streaming · Section 12.3 · Streaming page budgets and rendering** · [Contents](contents.md)

**Typing: 16–32 minutes.** [Estimate](typing-load.md).

Represent either original source family with one draw descriptor while keeping row models and source ownership separate.

## Type

Continue from [Retain page owners beside GPU buffers](68q-retain-page-owners-beside-gpu-buffers.md). [Save or recover your work](recovery.md).

### 1. `src/cloud_gpu.rs`

Describe source ownership and compact retained-page addresses independently.

<details>
<summary>Locate the existing block</summary>

```rust
    usage: wgpu::BufferUsages::STORAGE,
  })
}
struct Draw {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/68r-describe-resident-and-streamed-draw-rows-01.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.3.20
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

![Native renderer output for describe resident and streamed draw rows.](../screenshots/journey/66af-connect-browser-save-open-url-adoption.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [12.1.44 · Connect browser save open and URL adoption](66af-connect-browser-save-open-url-adoption.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



source owner → shared node table → placed draw row → compact page address.

![Describe resident and streamed draw rows.](../illustrations/68r-describe-resident-and-streamed-draw-rows.svg)

Must the original full source row index fit the GPU sub-object tag?

No. Each retained page has a compact base address. Resolve it through the original page ID column.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare resident row0 with a later streamed source interval and its independent compact base.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.3.20
npm --prefix ../session_tests run course -- save 12.3.20
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The existing mixed original archive browser exercise remains authored and unrun. The final cache endpoint adds real ranged URL loading, response validation, cancellation and cleanup exercises, also awaiting browser execution.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 12.3.20
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
