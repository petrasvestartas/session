# 17.3.12 · Return original integer section owners

**Chapter 17 · GPU visibility, clipping and sections · Section 17.3 · Closed sections and cap rendering** · [Contents](contents.md)

**Typing: 18–35 minutes.** [Estimate](typing-load.md).

Return original integer section owners.

## Type

Continue from [Adapt color sections to forward depth](88h-adapt-color-sections-to-forward-depth.md). [Save or recover your work](recovery.md).

### 1. `src/cap_pick.wgsl`

Adapt the maintained section integer recipe to exact original owner rows.

Create the file and type:

```wgsl
--8<-- "journey/code/88i-return-original-integer-section-owners-01.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.3.12
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

Actual color clipping removes the right halves while retaining original geometry on the left.

![Native renderer output for return original integer section owners.](../screenshots/journey/87ab-exercise-original-clipping-color-and-picking.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.2.38 · Exercise original clipping color and picking](87ab-exercise-original-clipping-color-and-picking.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original closed source → derived section → original color and integer owner.

![Return original integer section owners.](../illustrations/88i-return-original-integer-section-owners.svg)

Does a cap invent a new face or edge ID?

No. A whole-element pick returns its original u32 owner and part0. A locked cap writes depth with owner0.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare a single interior token, overlapping nearest exits and original pick eligibility.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.3.12
npm --prefix ../session_tests run course -- save 17.3.12
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The supplied browser exercise requires an actual browser. Native software-GPU rendering does not measure Lenovo performance.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 17.3.12
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
