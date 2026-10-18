# 16.5.5 · Plan sibling pieces from original owners

**Chapter 16 · Geometry editing and measurement · Section 16.5 · Text, styles and command integration** · [Contents](contents.md)

**Typing: 15–29 minutes.** [Estimate](typing-load.md).

Plan sibling pieces from original owners.

## Type

Continue from [Find eligible objects by name or world size](85c-find-eligible-objects-by-name-or-world-size.md). [Save or recover your work](recovery.md).

### 1. `src/explode_edit.rs`

Create the file and type:

```rust
--8<-- "journey/code/85d-plan-sibling-pieces-from-original-owners-01.rs"
```

### 2. `src/lib.rs`

Register this command owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod selection_query;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/85d-plan-sibling-pieces-from-original-owners-02.rs" skip=26
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.5.5
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

![Native renderer output for plan sibling pieces from original owners.](../screenshots/journey/84m-exercise-original-boxes-and-temporary-distance-annotations.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.4.14 · Exercise original boxes and temporary distance annotations](84m-exercise-original-boxes-and-temporary-distance-annotations.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



fresh original selection → bounded pieces → sibling adoption.

![Plan sibling pieces from original owners.](../illustrations/85d-plan-sibling-pieces-from-original-owners.svg)

Does Explode flatten world coordinates into the source?

No. Pieces retain their parent placement and original style; the plan validates all owners first.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect the 4096-piece limit, child refusal and mixed unsupported selection.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.5.5
npm --prefix ../session_tests run course -- save 16.5.5
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Plan sibling pieces from original owners.

The inherited measurement browser contract remains authored and unrun. The current-command browser contract connects at85ay and still requires real browser execution.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 16.5.5
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
