# 15.3.2 · Frame original profiles on their rail

**Chapter 15 · Transforms and surface construction · Section 15.3 · Lofts, sweeps and Gordon surfaces** · [Contents](contents.md)

**Typing: 16–32 minutes.** [Estimate](typing-load.md).

Find each profile’s station from its sampled center and the nearest rail parameter. Use the kernel’s perpendicular rail frame to prepare a local transform. Write the original four-dimensional control-point blend for the matched profiles used in the next step.

## Type

Continue from [Retain current loft section order and degree options](79-retain-current-loft-section-order-and-degree-options.md). [Save or recover your work](recovery.md).

### 1. `src/sweep1_geometry.rs`

Type the whole original sweep helpers using the existing kernel and gather owners.

Create the file and type:

```rust
--8<-- "journey/code/79aa-frame-original-profiles-on-their-rail-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original sweep module.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod nurbs_loft_geometry;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/79aa-frame-original-profiles-on-their-rail-2.rs" skip=30
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.3.2
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Original profiles can be framed on their rail without changing the existing drawing.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for frame original profiles on their rail.](../screenshots/journey/79-retain-current-loft-section-order-and-degree-options.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.3.1 · Retain current loft section order and degree options](79-retain-current-loft-section-order-and-degree-options.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original rail and profiles → local section frames → kernel sweep surface.



Why transform a profile copy into the rail frame?

Its shape must be carried from that local frame to later rail frames. Keep the picked original unchanged.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a closed profile’s repeated end sample and its local frame. Compare endpoint and middle control weights in the blend.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.3.2
npm --prefix ../session_tests run course -- save 15.3.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original sweep helpers are preparatory until the existing sweep command registry connects them. The earlier drawing stays unchanged.



[Full validation scope](release.md).

</details>
