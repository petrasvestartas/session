# 14.1.38 · Build original construction-plane frames

**Chapter 14 · Coordinates and geometry tools · Section 14.1 · Sheet queries and construction planes** · [Contents](contents.md)

**Typing: 27–53 minutes.** [Estimate](typing-load.md).

Describe the original world planes and a movable frame. Build a right-handed frame from three points using shared subtraction, cross-product and normalization helpers. Reject collinear points before constructing the axes.

## Type

Continue from [Exercise complete and ranged original drawings](71aj-exercise-complete-and-ranged-original-drawings.md). [Save or recover your work](recovery.md).

### 1. `src/cplane.rs`

Type the whole original frame or coordinate helpers using their shared owners.

Create the file and type:

```rust
--8<-- "journey/code/71za-build-original-construction-plane-frames-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original frame or coordinate module.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod sheet_scene_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/71za-build-original-construction-plane-frames-2.rs" skip=36
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.1.38
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Construction-plane frames can be built without changing the current browser view.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for build original construction-plane frames.](../screenshots/journey/71aj-exercise-complete-and-ranged-original-drawings.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.1.37 · Exercise complete and ranged original drawings](71aj-exercise-complete-and-ranged-original-drawings.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original frame → parsed coordinates → world drawing point.



Why take two cross products when constructing the frame?

The first finds its normal. The second derives a perpendicular in-plane y axis while retaining the chosen x direction.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace three valid points and three collinear points through the same frame builder.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.1.38
npm --prefix ../session_tests run course -- save 14.1.38
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original frame and coordinate helpers are preparatory until the existing input bridge connects them. The earlier drawing stays unchanged.



[Full validation scope](release.md).

</details>
