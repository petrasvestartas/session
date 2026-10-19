# 15.1.16 · Map sparse original vertex keys to display slots

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 18–35 minutes.** [Estimate](typing-load.md).

Choose a dense slot lookup only when sorted original keys are close enough together; otherwise use a sparse map.

## Type

Continue from [Dispatch original part edits and retain Element ownership](77sn-dispatch-original-part-edits-and-retain-element-ownership.md). [Save or recover your work](recovery.md).

### 1. `src/source_topology_colour.rs`

Pack original edge pen colours into the existing RGBA word convention.

Create the file and type:

```rust
--8<-- "journey/code/77so-map-sparse-original-vertex-keys-to-display-slots-typed-01.rs"
```

### 2. `src/source_mesh_topology.rs`

Map sparse original vertex keys to display slots.

Create the file and type:

```rust
--8<-- "journey/code/77so-map-sparse-original-vertex-keys-to-display-slots-typed-02.rs"
```

### 3. `src/lib.rs`

Register the shared source colour helper and original edge topology owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod source_deform;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/77so-map-sparse-original-vertex-keys-to-display-slots-typed-03.rs" skip=24
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.16
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

Original Circle, Ellipse, Arc, Parabola and freeform Curve remain visible.

![Native renderer output for map sparse original vertex keys to display slots.](../screenshots/journey/77-copy-current-transformation-and-source-edit-algorithms.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.1.1 · Build directional scale transforms](77-copy-current-transformation-and-source-edit-algorithms.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The lookup maps original identity to contiguous display storage without changing the source keys. Pack source pen colours once with the small shared RGBA helper; edge topology will reuse that packed format.

original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.



Why not allocate a GPU slot array up to the largest original vertex key?

Sparse original keys can be enormous even when the mesh has few vertices.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare original keys[0,1,2] with[2,1000000]. Predict which lookup stores each mesh and which GPU slot belongs to its last original vertex.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.16
npm --prefix ../session_tests run course -- save 15.1.16
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Original edit preparation; the later connected command and control-picking browser exercises establish interaction behavior.

[Full validation scope](release.md).

</details>
