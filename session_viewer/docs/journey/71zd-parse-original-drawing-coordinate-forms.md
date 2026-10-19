# 14.1.41 · Parse original drawing coordinate forms

**Chapter 14 · Coordinates and geometry tools · Section 14.1 · Sheet queries and construction planes** · [Contents](contents.md)

**Typing: 26–52 minutes.** [Estimate](typing-load.md).

Describe the original absolute, relative, polar and distance forms. Keep the selected construction plane in its existing owner. Parse only finite numbers and retain missing height as a distinct two-dimensional coordinate.

## Type

Continue from [Guard original plane and ray projections](71zc-guard-original-plane-and-ray-projections.md). [Save or recover your work](recovery.md).

### 1. `src/drawing_coords.rs`

Type the whole original frame or coordinate helpers using their shared owners.

Create the file and type:

```rust
--8<-- "journey/code/71zd-parse-original-drawing-coordinate-forms-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original frame or coordinate module.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod cplane;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/71zd-parse-original-drawing-coordinate-forms-2.rs" skip=17
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.1.41
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The coordinate grammar can distinguish supported drawing forms; the existing input remains available.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for parse original drawing coordinate forms.](../screenshots/journey/71aj-exercise-complete-and-ranged-original-drawings.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.1.37 · Exercise complete and ranged original drawings](71aj-exercise-complete-and-ranged-original-drawings.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original frame → parsed coordinates → world drawing point.



Why retain missing height instead of immediately inserting zero?

A two-dimensional input lies on the drawing plane. An explicit third coordinate asks for a three-dimensional position.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace 1,2, @1,2, 5<45 and a single distance through the same parser.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.1.41
npm --prefix ../session_tests run course -- save 14.1.41
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original frame and coordinate helpers are preparatory until the existing input bridge connects them. The earlier drawing stays unchanged.



[Full validation scope](release.md).

</details>
