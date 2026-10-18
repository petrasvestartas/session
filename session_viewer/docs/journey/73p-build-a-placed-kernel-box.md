# 14.3.17 · Build a placed kernel box

**Chapter 14 · Coordinates and geometry tools · Section 14.3 · Planar shapes and regular solids** · [Contents](contents.md)

**Typing: 17–34 minutes.** [Estimate](typing-load.md).

Build a box through the Session kernel and place its center half a signed height above the construction plane. Draw its line, base and full-box preview from the shared frame.

## Type

Continue from [Scale and cache polyhedron faces](73o-scale-and-cache-polyhedron-faces.md). [Save or recover your work](recovery.md).

### 1. `src/primitive_box.rs`

Build and place the kernel box, with its width and preview helpers.

Create the file and type:

```rust
--8<-- "journey/code/73p-build-a-placed-kernel-box-1.rs"
```

### 2. `src/lib.rs`

Register the box helpers before connecting their question sequence.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod primitive_shape;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/73p-build-a-placed-kernel-box-2.rs" skip=26
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.3.17
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

![Native renderer output for build a placed kernel box.](../screenshots/journey/73h-match-solid-command-options.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.3.9 · Match solid command options](73h-match-solid-command-options.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



Build a placed kernel box → Session kernel geometry → existing preview and placement helpers.



Why translate by half the signed height?

The kernel box is centered. The translation places its bottom on the construction plane; a negative height puts it below that plane.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace positive and negative heights. Predict the preview coordinates on the Front plane.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.3.17
npm --prefix ../session_tests run course -- save 14.3.17
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This writes original solid builders before the later creation-tool registration. The current browser scene remains unchanged.



[Full validation scope](release.md).

</details>
