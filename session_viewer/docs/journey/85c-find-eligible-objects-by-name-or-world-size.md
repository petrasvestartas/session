# 16.5.4 · Find eligible objects by name or world size

**Chapter 16 · Geometry editing and measurement · Section 16.5 · Text, styles and command integration** · [Contents](contents.md)

**Typing: 12–24 minutes.** [Estimate](typing-load.md).

Find eligible objects by name or world size.

## Type

Continue from [Retain original Plane and Text edit eligibility](85b-retain-original-plane-and-text-edit-eligibility.md). [Save or recover your work](recovery.md).

### 1. `src/selection_query.rs`

Create the file and type:

```rust
--8<-- "journey/code/85c-find-eligible-objects-by-name-or-world-size-01.rs"
```

### 2. `src/lib.rs`

Register this command owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod plane_original;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/85c-find-eligible-objects-by-name-or-world-size-02.rs" skip=25
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.5.4
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

![Native renderer output for find eligible objects by name or world size.](../screenshots/journey/84m-exercise-original-boxes-and-temporary-distance-annotations.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.4.14 · Exercise original boxes and temporary distance annotations](84m-exercise-original-boxes-and-temporary-distance-annotations.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



query → visible unlocked originals → unchanged selection on misses.

![Find eligible objects by name or world size.](../illustrations/85c-find-eligible-objects-by-name-or-world-size.svg)

Does Select Small depend on zoom?

No. It compares the world bounding-box diagonal with the requested positive length.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Try a quoted Unicode name and a strict size threshold. A miss must keep the previous selection.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.5.4
npm --prefix ../session_tests run course -- save 16.5.4
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Find eligible objects by name or world size.

The inherited measurement browser contract remains authored and unrun. The current-command browser contract connects at85ay and still requires real browser execution.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 16.5.4
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
