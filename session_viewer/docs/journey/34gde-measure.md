# 4.2.30 · Measure a completed CPU operation

**Chapter 4 · Browser lifetime and recovery · Section 4.2 · GPU diagnostics and recovery** · [Contents](contents.md)

**Typing: 11–22 minutes.** [Estimate](typing-load.md).

Wrap one synchronous operation with two clock samples. Pass only completed metadata to an observer and return the original result.

## Type

Continue from [Measure the selected file read](34gdd-read.md). [Save or recover your work](recovery.md).

### 1. `src/load_measure.rs`

Keep timing metadata separate from the operation’s owned data and result.

Create the file and type:

```rust
--8<-- "journey/code/34gde-measure-01.rs"
```

### 2. `src/lib.rs`

Register the portable clock boundary and its tests.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod report_recency;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod load_measure;
#[cfg(test)]
mod load_measure_tests;
pub mod report_recency;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34gde-measure-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/load_measure_tests.rs`

Copy deterministic checks for the clock boundary.

Copy this supplied file:

```text
--8<-- "journey/code/34gde-measure-03.rs"
```

</details>

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Run the checks. Success and failure both keep their original result while valid clocks record one completed phase.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for measure a completed cpu operation.](../screenshots/journey/34gba-lifecycle.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [4.2.18 · Own diagnostic page transitions](34gba-lifecycle.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The observer receives no document or source handle. Invalid clock values suppress diagnostics while the operation keeps its normal outcome. Native tests inject the clock; the browser uses its existing performance clock.

clock → operation result → valid duration → metadata observer → original result.

![A clock surrounds the operation; diagnostics cannot replace its result.](../illustrations/journey-34gde.svg)

Why return the operation result even when its clock is invalid?

Timing is diagnostic. A backwards or unavailable clock must never turn a successful edit into a failure.

Study estimate, including typing and experiments: 0.5–0.75 hours.

</details>

<details>
<summary>Optional experiment</summary>

Give the test clock a backwards pair of samples. The result stays unchanged and the observer stays quiet.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 4.2.30
npm --prefix ../session_tests run course -- save 4.2.30
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This portable measurement boundary prepares the document phases in the next lesson.

Native tests check success, failure, exact boundaries and invalid clocks. Chrome repeats the real selected-file read checks.

Chrome keeps the existing file-read and import behavior while the portable timing boundary is introduced.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 4.2.30
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
