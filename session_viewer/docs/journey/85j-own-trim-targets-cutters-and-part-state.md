# 16.5.11 · Own Trim targets cutters and part state

**Chapter 16 · Geometry editing and measurement · Section 16.5 · Text, styles and command integration** · [Contents](contents.md)

**Typing: 29–58 minutes.** [Estimate](typing-load.md).

Own Trim targets cutters and part state.

## Type

Continue from [Draw cached guides through the overlay owner](85i-draw-cached-guides-through-the-overlay-owner.md). [Save or recover your work](recovery.md).

### 1. `src/trim_draft.rs`

Create the file and type:

```rust
--8<-- "journey/code/85j-own-trim-targets-cutters-and-part-state-01.rs"
```

### 2. `src/lib.rs`

Register this command owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod guide_gpu;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/85j-own-trim-targets-cutters-and-part-state-02.rs" skip=20
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.5.11
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

The preceding solid, wire bounding box and temporary distance annotation remain visible.

![Native renderer output for own trim targets cutters and part state.](../screenshots/journey/84m-exercise-original-boxes-and-temporary-distance-annotations.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.4.14 · Exercise original boxes and temporary distance annotations](84m-exercise-original-boxes-and-temporary-distance-annotations.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



fresh originals → target and cutter toggles → pending state.

![Own Trim targets cutters and part state.](../illustrations/85j-own-trim-targets-cutters-and-part-state.svg)

Can a target cut itself?

No. Targets and cutters remain separate, and all target edits belong to one document.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect whole-owner capture, toggled picks and the 4096-owner guard.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.5.11
npm --prefix ../session_tests run course -- save 16.5.11
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Own Trim targets cutters and part state.

The inherited measurement browser contract remains authored and unrun. The current-command browser contract connects at85ay and still requires real browser execution.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 16.5.11
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
