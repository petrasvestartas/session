# 20.2.2 · Describe ink depth and surface slack

**Chapter 20 · Shared GPU rendering · Section 20.2 · Surface-fitted finite ink** · [Contents](contents.md)

**Typing: 11–22 minutes.** [Estimate](typing-load.md).

A thick stroke covers pixels beside its center line.

## Type

Continue from [Bind visible triangle addresses for ink](96zn-bind-visible-triangle-addresses-for-ink.md). [Save or recover your work](recovery.md).

### 1. `src/ink_plane.wgsl`

Type the current axis and tolerance math once, with the declared forward-depth reader adapter.

Create the file and type:

```wgsl
--8<-- "journey/code/96zo-describe-ink-depth-and-surface-slack-1.wgsl"
```

### 2. `src/finite_visibility.wgsl`

Keep surface fitting as a shared shader helper file.

<details>
<summary>Locate the existing block</summary>

```wgsl
fn finite_section(pixel:vec2<f32>,depth:f32,tolerance:f32)->bool{
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
#include "ink_plane.wgsl"
fn finite_section(pixel:vec2<f32>,depth:f32,tolerance:f32)->bool{
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zo-describe-ink-depth-and-surface-slack-2.wgsl"
```

</details>

### 3. `src/finite_sources.rs`

Expand the helper before compiling either color or integer ink shaders.

<details>
<summary>Locate the existing block</summary>

```rust
 let base=crate::finite_project_source::expand(base).replace("#include \"finite_visibility.wgsl\"",include_str!("finite_visibility.wgsl"));
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
 let base=crate::finite_project_source::expand(base).replace("#include \"finite_visibility.wgsl\"",include_str!("finite_visibility.wgsl")).replace("#include \"ink_plane.wgsl\"",include_str!("ink_plane.wgsl"));
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zo-describe-ink-depth-and-surface-slack-3.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.2.2
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

This supporting frame has no instances; use the supplied original Session specimen for the browser exercise.

![Native renderer output for describe ink depth and surface slack.](../screenshots/journey/96u-fit-original-box-corners-to-the-free-canvas.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [19.3.3 · Fit original box corners to the free canvas](96u-fit-original-box-corners-to-the-free-canvas.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Comparing its center depth with an unrelated side pixel can clip a coplanar stroke. Prepare the current InkAxis record: center, depth, direction, slope and sag. Sag represents a curved source edge rising above its display facets. Use the current relative rounding and subpixel slope tolerances rather than a fixed world-space bias. Keep one helper for carrying a surface depth to the stroke. For now, the depth reader explicitly converts the teaching forward-depth image; reversed camera depth will remove that conversion. These helpers do not change visibility yet.

Follow gpu_geometry.rs, triangle.wgsl, gpu_mesh.rs, instance_gpu.rs and renderer.rs in this cumulative checkpoint; compare the maintained engine/gpu/ and shaders/triangle.wgsl ownership..



Why does the tolerance include a pixel lever?

Carrying a sampled plane farther amplifies depth error from subpixel rasterization. The error bound depends on slope and distance, not only the center depth.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compile and compare the unchanged CAD view. Follow the units of each InkAxis field before connecting the helpers.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.2.2
npm --prefix ../session_tests run course -- save 20.2.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Type the current ink depth/plane-fit helpers once. They are preparatory and remain inactive until all visibility callers and first-pick resources are connected. The forward-depth conversion remains an explicit camera adapter.

Chrome acceptance for shared vertex reads, original normal pixels, placed instances and selection is pending. Integer primitive readback has separate GPU fixture evidence.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.2.2
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
