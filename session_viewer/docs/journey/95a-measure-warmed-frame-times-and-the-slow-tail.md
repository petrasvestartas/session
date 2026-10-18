# 18.7.2 · Measure warmed frame times and the slow tail

**Chapter 18 · Lighting, render quality and comparison · Section 18.7 · Performance and cache reports** · [Contents](contents.md)

**Typing: 9–17 minutes.** [Estimate](typing-load.md).

Measure warmed frame times and the slow tail.

## Type

Continue from [Record comparable large scene conditions](95-record-comparable-large-scene-conditions.md). [Save or recover your work](recovery.md).

### 1. `src/bench_samples.rs`

Summarize real samples with an explicit warmup and slow-frame tail.

Create the file and type:

```rust
--8<-- "journey/code/95a-measure-warmed-frame-times-and-the-slow-tail-01.rs"
```

### 2. `src/lib.rs`

Register the completed performance owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod bench_config;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/95a-measure-warmed-frame-times-and-the-slow-tail-02.rs" skip=23
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.7.2
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

Four blue original solids retain continuous black curved and flat boundaries. The maintained native frame is inspected separately; exact pixel equality is not asserted.

![Native renderer output for measure warmed frame times and the slow tail.](../screenshots/journey/94c-prove-the-original-comparison-and-inspect-differences.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [18.6.4 · Prove the original comparison and inspect differences](94c-prove-the-original-comparison-and-inspect-differences.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



record comparable hardware and source → reuse unchanged scene work → measure rotation and zoom.

![Measure warmed frame times and the slow tail.](../illustrations/95a-measure-warmed-frame-times-and-the-slow-tail.svg)

Does average FPS describe occasional long stalls?

Keep raw frame durations, discard a declared warmup and report median, nearest-rank P95 and maximum. Reject missing, zero or nonfinite samples.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare a stable run with a few long frames; inspect P95 rather than reporting only its median.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.7.2
npm --prefix ../session_tests run course -- save 18.7.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The supplied browser contract exercises rotation, zoom, selection and visibility on the actual teaching comparison scene. It is syntaxchecked and remains unrun until a real browser is enabled. Lenovo Chrome/Firefox timings are not available.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 18.7.2
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
