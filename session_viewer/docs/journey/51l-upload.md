# 7.6.14 · Upload handle geometry once

**Chapter 7 · Drawing, snapping and transform tools · Section 7.6 · Move, rotate and scale handles** · [Contents](contents.md)

**Typing: 19–38 minutes.** [Estimate](typing-load.md).

The widget owns one static vertex buffer and a96-byte placement uniform. Construct its pipelines once, count its buffer/tile allocations and drop tile resources when the selection is hidden.

## Type

Continue from [Own color, depth and resolve textures together](51k-tile.md). [Save or recover your work](recovery.md).

### 1. `src/widget.rs`

Upload one static handle mesh, construct pipelines and expose explicit allocation accounting.

<details>
<summary>Locate the existing block</summary>

```rust
struct Tile { size:[u32;2],color:wgpu::TextureView,depth:wgpu::TextureView,resolved:wgpu::TextureView,group:wgpu::BindGroup }

impl Tile {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/51l-upload-01.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

The earlier two gold boxes and temporary Move band remain.

![Native renderer output for upload handle geometry once.](../screenshots/journey/50d-ownership.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [7.5.5 · Cancel previews without consuming document history](50d-ownership.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



one packed mesh → persistent vertex buffer and uniform → current tile ledger → release.

![Upload handle geometry once](../illustrations/journey-51l.svg)

Should orbiting upload the procedural mesh again?

No. Geometry remains static. Orbiting changes the widget placement uniform; a changed extent alone may replace the tile.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare persistent buffer bytes with temporary tile bytes. Clearing a selection drops textures while reusable buffers remain owned by the renderer.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 7.6.14
npm --prefix ../session_tests run course -- save 7.6.14
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This preparation retains the earlier Move input. Current handle math and drawing are introduced in dependency order; selection and pointer input are connected later in this subject.

The earlier temporary Move remains while handle infrastructure is introduced.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 7.6.14
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
