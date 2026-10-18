# 16.3.2 · Visit original mesh triangles and format measures

**Chapter 16 · Geometry editing and measurement · Section 16.3 · Surface and curve measurement** · [Contents](contents.md)

**Typing: 18–36 minutes.** [Estimate](typing-load.md).

Write one visitor for the original mesh triangles in world coordinates. Reuse the cached triangulation when present and the existing face fan otherwise. Keep short numeric and count formatting under the same measurement helper owner.

## Type

Continue from [Split original faces without separating the solid](83-split-original-faces-without-separating-the-solid.md). [Save or recover your work](recovery.md).

### 1. `src/measure_math.rs`

Type the whole original mesh measurement helpers using the shared visitor.

Create the file and type:

```rust
--8<-- "journey/code/83za-visit-original-mesh-triangles-and-format-measures-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original measurement module.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod split_face_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/83za-visit-original-mesh-triangles-and-format-measures-2.rs" skip=35
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.3.2
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The earlier drawing remains visible while mesh measurement helpers are prepared.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for visit original mesh triangles and format measures.](../screenshots/journey/83za-visit-original-mesh-triangles-and-format-measures.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>



original triangles → world transform → area or signed swept volume.



Why measure the cached triangles when they exist?

They are the triangles the viewer draws. Reuse their geometry instead of inventing another triangulation for measurements.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a cached face and a fan face through one transform. Compare a small nonzero value with rounded negative zero.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.3.2
npm --prefix ../session_tests run course -- save 16.3.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original mesh measurement helpers are preparatory until the existing measurement bridge connects them. The earlier drawing stays unchanged.



[Full validation scope](release.md).

</details>
