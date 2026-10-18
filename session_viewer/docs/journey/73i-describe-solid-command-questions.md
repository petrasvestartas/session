# 14.3.10 · Describe solid command questions

**Chapter 14 · Coordinates and geometry tools · Section 14.3 · Planar shapes and regular solids** · [Contents](contents.md)

**Typing: 21–41 minutes.** [Estimate](typing-load.md).

Represent typed coordinates and numbers explicitly. Give each question its interpretation and optional default.

## Type

Continue from [Match solid command options](73h-match-solid-command-options.md). [Save or recover your work](recovery.md).

### 1. `src/primitive_shape.rs`

Describe the solid answers and questions using kernel points.

Create the file and type:

```rust
--8<-- "journey/code/73i-describe-solid-command-questions-1.rs"
```

### 2. `src/lib.rs`

Register the foundation as it is written.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod primitive_options;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/73i-describe-solid-command-questions-2.rs" skip=28
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.3.10
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

![Native renderer output for describe solid command questions.](../screenshots/journey/73h-match-solid-command-options.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.3.9 · Match solid command options](73h-match-solid-command-options.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



Describe solid command questions → original f64 kernel coordinates → later temporary preview and accepted geometry.



Why keep a numeric answer separate from a clicked point?

The same number can mean a size or a signed height; the question decides how to interpret it.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Describe Center, Radius and Height using the three constructors. Predict which answers Enter can accept.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.3.10
npm --prefix ../session_tests run course -- save 14.3.10
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This codes the shared solid-command foundation before connecting the individual builders. The browser scene is unchanged at this checkpoint.



[Full validation scope](release.md).

</details>
