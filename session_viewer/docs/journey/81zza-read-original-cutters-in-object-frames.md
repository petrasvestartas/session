# 16.1.2 · Read original cutters in object frames

**Chapter 16 · Geometry editing and measurement · Section 16.1 · Projection and Split inputs** · [Contents](contents.md)

**Typing: 27–53 minutes.** [Estimate](typing-load.md).

Write the cutter records and convert supported originals to curves or planar boundaries. Rebuild a moved plane from three transformed points.

## Type

Continue from [Project originals in their placed frame](81-project-originals-through-the-current-plane-algorithms.md). [Save or recover your work](recovery.md).

### 1. `src/source_cut.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/81zza-read-original-cutters-in-object-frames-1.rs"
```

### 2. `src/lib.rs`

Register this original curve-editing owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod project_geometry;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/81zza-read-original-cutters-in-object-frames-2.rs" skip=27
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.1.2
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Original cutters can be read in an object's frame before extension input is connected.

**Visual review: issue found.** This inherited network reference has faint or interrupted original input curves. Its aggregate pixel checks do not establish curve continuity or contrast. These are input curves, not a full NURBS isocurve display; visual-quality acceptance is incomplete.

[Visual review findings](visual-review.md).

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for read original cutters in object frames.](../screenshots/journey/80za-read-original-projection-planes.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.4.6 · Read original projection planes](80za-read-original-projection-planes.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original cutter → object frame → physical reach → edited curve.



Why rebuild a plane after a stretch?

Its transformed axes may cease to be orthogonal. Three transformed points define the actual transformed plane.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a nonuniform stretch and a curved surface through plane conversion.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.1.2
npm --prefix ../session_tests run course -- save 16.1.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original cutter and reach helpers prepare cancellable editing of the original curve.



[Full validation scope](release.md).

</details>
