# 18.4.7 · Capture original picks while the tool runs

**Chapter 18 · Lighting, render quality and comparison · Section 18.4 · Wood Elements and archives** · [Contents](contents.md)

**Typing: 21–42 minutes.** [Estimate](typing-load.md).

Capture original picks while the tool runs.

## Type

Continue from [Build typed Elements from ordered world picks](92d-build-typed-elements-from-ordered-world-picks.md). [Save or recover your work](recovery.md).

### 1. `src/element_draft.rs`

Capture fitting original sources and ordered tool state.

Create the file and type:

```rust
--8<-- "journey/code/92e-capture-original-picks-while-the-tool-runs-01.rs"
```

### 2. `src/lib.rs`

Register the completed Element component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod element_input;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/92e-capture-original-picks-while-the-tool-runs-02.rs" skip=24
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.4.7
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

Actual original curved and flat surfaces use the same black outline width.

![Native renderer output for capture original picks while the tool runs.](../screenshots/journey/91i-prove-actual-outlines-and-original-ownership.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [18.3.10 · Prove actual outlines and original ownership](91i-prove-actual-outlines-and-original-ownership.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original ordered picks → current Wood builders → typed Element and attributes → one history edit.

![Capture original picks while the tool runs.](../illustrations/92e-capture-original-picks-while-the-tool-runs.svg)

Can a pending Element tool use a moved or replaced source?

No. Capture its shared original and exact placement, then validate both before creation. Picking again toggles its ordered membership.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Move a captured axis before Enter and verify the original remains present after rejection.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.4.7
npm --prefix ../session_tests run course -- save 18.4.7
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The supplied browser contract uses the command dock to create a beam from original lines, adjust named values, show features, measure bounds and Undo/Redo. It is syntaxchecked and requires an actual enabled browser. Native fixtures do not constitute browser acceptance.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 18.4.7
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
