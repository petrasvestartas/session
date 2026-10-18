# 16.1.31 · Validate original Split inputs and name pieces

**Chapter 16 · Geometry editing and measurement · Section 16.1 · Projection and Split inputs** · [Contents](contents.md)

**Typing: 27–54 minutes.** [Estimate](typing-load.md).

Accept original cutter curves and a valid selected BRep face. Convert cutters through the Session kernel and assign distinct names only when a curve produces sibling objects.

## Type

Continue from [Exercise original projection retry and repeated extension](81v-exercise-original-projection-retry-and-repeated-extension.md). [Save or recover your work](recovery.md).

### 1. `src/split_geometry.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/81za-validate-original-split-inputs-and-name-pieces-1.rs"
```

### 2. `src/lib.rs`

Register this original source-splitting owner.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod editing_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/81za-validate-original-split-inputs-and-name-pieces-2.rs" skip=32
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.1.31
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Split input validation and original piece naming compile before their editing connection.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for validate original split inputs and name pieces.](../screenshots/journey/81v-exercise-original-projection-retry-and-repeated-extension.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.1.30 · Exercise original projection retry and repeated extension](81v-exercise-original-projection-retry-and-repeated-extension.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original family → pure calculation → result or refusal.



Why keep a selected solid face inside its BRep?

Splitting that face changes the BRep topology while the solid remains one joined object. It does not explode the solid into face objects.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a multi-face BRep with no selected face. Compare a line cutter with an unsupported geometry kind.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.1.31
npm --prefix ../session_tests run course -- save 16.1.31
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original command helpers prepare the later editor bridge. The existing drawing stays unchanged.



[Full validation scope](release.md).

</details>
