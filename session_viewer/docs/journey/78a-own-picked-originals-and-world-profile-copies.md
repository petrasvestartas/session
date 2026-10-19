# 15.2.2 · Own picked originals and world profile copies

**Chapter 15 · Transforms and surface construction · Section 15.2 · Profile collection and surfacing** · [Contents](contents.md)

**Typing: 24–47 minutes.** [Estimate](typing-load.md).

Retain the recipe, accepted answers, original selection, source allocations and placements beside derived world curve inputs.

## Type

Continue from [Copy current curve collection and surfacing recipes](78-copy-current-curve-collection-and-surfacing-recipes.md). [Save or recover your work](recovery.md).

### 1. `src/gather_draft.rs`

Own pending recipe answers, original allocations, placements and derived world profiles.

Create the file and type:

```rust
--8<-- "journey/code/78a-own-picked-originals-and-world-profile-copies-01.rs"
```

### 2. `src/lib.rs`

Register the gather component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod revolve_geometry;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/78a-own-picked-originals-and-world-profile-copies-02.rs" skip=27
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.2.2
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

Three original boxes retain independent placements.

![Native renderer output for own picked originals and world profile copies.](../screenshots/journey/78-copy-current-curve-collection-and-surfacing-recipes.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.2.1 · Copy current curve collection and surfacing recipes](78-copy-current-curve-collection-and-surfacing-recipes.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original selection and placement → world profile copies → pending recipe owner.

![Own picked originals and world profile copies.](../illustrations/78a-own-picked-originals-and-world-profile-copies.svg)

Can world curve conversion mutate a shared source?

No. Clone and place the builder input; retain the original source allocation and exact row placement for freshness checks.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare the clamped world input with its original periodic curve. A profile placed at x10 is transformed once before building.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.2.2
npm --prefix ../session_tests run course -- save 15.2.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The preceding transformation browser contract remains authored and unrun. The final extrusion/revolution contract exercises the registered names, failed input, cap options, one Undo/Redo and the same rendered exercise. Actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 15.2.2
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
