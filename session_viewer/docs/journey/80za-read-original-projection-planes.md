# 15.4.6 · Read original projection planes

**Chapter 15 · Transforms and surface construction · Section 15.4 · Curve networks and projection planes** · [Contents](contents.md)

**Typing: 20–39 minutes.** [Estimate](typing-load.md).

Reuse absolute coordinate parsing for named construction planes and three-point input. Reject unknown names, wrong point counts and collinear frame points.

## Type

Continue from [Exercise networks corner retries and exact original archives](80d-exercise-networks-corner-retries-and-exact-original-archives.md). [Save or recover your work](recovery.md).

### 1. `src/project_geometry.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/80za-read-original-projection-planes-1.rs"
```

### 2. `src/lib.rs`

Register the original projection helpers.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod network_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/80za-read-original-projection-planes-2.rs" skip=32
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.4.6
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Projection-plane parsing is prepared while the existing command behavior remains available.

**Visual review: issue found.** The inherited network-surface image contains six original input curves and two surfaces; it does not show a complete NURBS isocurve display. The interior curves are faint and discontinuous. The current pixel check counts aggregate dark pixels and does not prove curve continuity or contrast. Visual quality is not accepted; a focused comparison and correction are required.

[Visual review findings](visual-review.md).

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for read original projection planes.](../screenshots/journey/80za-read-original-projection-planes.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>



original family → pure calculation → result or refusal.



Why validate the three points before projecting?

They must define a stable plane. Collinear or repeated points cannot supply its normal.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare XY, the current construction plane and three picked points. Trace the wrong-arity and collinearity refusals.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.4.6
npm --prefix ../session_tests run course -- save 15.4.6
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original command helpers prepare the later editor bridge. The existing drawing stays unchanged.



[Full validation scope](release.md).

</details>
