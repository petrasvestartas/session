# Tutorial 3 · Rendering engine and shared shaders

## Goal

Preprocess WGSL `#include` headers at build time through `build.rs`, establish bind-group layouts `0..=3` and lazy `OnceLock` pipeline compilation, and execute the ordered frame render passes with background and construction-plane grid shaders.

## Diagram

![`build.rs` `#include` assembly into `OUT_DIR`, shared `PRELUDE` bind groups `0..=3`, lazy pipeline cache, and ordered frame passes (native wgpu readback of stage deck)](../../examples/stage/tutorials-foundations/03-rendering.png)

Stage deck source: [`03-rendering.yaml`](../../examples/stage/tutorials-foundations/03-rendering.yaml).

## Prerequisites and files

Requires [Tutorial 2](tutorial-two-wgpu.md). Followed immediately by [Tutorial 3.1 (`egui` painting)](tutorial-three-painting.md) and [Tutorial 3.2 (2D mathematics stage)](tutorial-three-mathematics.md).

Owned files in build order (`19` files):

1. `build.rs`
2. `src/engine/pipelines/mod.rs`
3. `src/engine/pipelines/bindings.rs`
4. `src/engine/pipelines/layouts.rs`
5. `src/engine/pipelines/core.rs`
6. `src/engine/pipelines/geometry.rs`
7. `src/shaders/scene.wgsl`
8. `src/shaders/clip.wgsl`
9. `src/shaders/physical.wgsl`
10. `src/shaders/normals.wgsl`
11. `src/shaders/projected_triangle.wgsl`
12. `src/shaders/ink_visibility.wgsl`
13. `src/engine/gpu/lane.rs`
14. `src/engine/gpu/pass.rs`
15. `src/engine/gpu/render.rs`
16. `src/engine/gpu/backdrop.rs`
17. `src/engine/gpu/backdrop/application_tests.rs`
18. `src/shaders/background.wgsl`
19. `src/shaders/grid.wgsl`

## Build

### Step 1 · `build.rs`

Create `build.rs` at the crate root. `build.rs` expands `#include "file.wgsl"` directives once per shader, rejects `/*` block comments, strips `//` comments and redundant whitespace, and writes minified `.wgsl` sources into `OUT_DIR/shaders/` for `src/lib.rs`'s `shader!()` macro: Spanning `61` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `61` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/build.rs"
```

### Step 2 · `src/engine/pipelines/mod.rs`

`src/engine/gpu/hull.rs`, `src/engine/gpu/instance.rs`, `src/engine/gpu/objects.rs` and `src/engine/pipelines/mod.rs` re-export narrow public types (`Hull`, `Instance`, `ObjectRows`, `Layouts`, `Cache`) that Tutorial 7 extends additively. Spanning `7` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `7` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/pipelines/mod.rs"
```

### Step 3 · `src/engine/pipelines/bindings.rs`

`src/engine/pipelines/bindings.rs` constructs typed uniform, read-only storage and `2D`/multisampled texture `BindGroupLayoutEntry` descriptors without allocating GPU memory. Spanning `36` lines, this module stays free of duplicate state by exposing `buffer_entry`, `texture_entry` without introducing UI/GPU coupling cycles. Spanning `36` lines, this module stays free of duplicate state by exposing `buffer_entry`, `texture_entry` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/pipelines/bindings.rs"
```

### Step 4 · `src/engine/pipelines/layouts.rs`

`src/engine/pipelines/layouts.rs` assembles `Layouts`: camera `mvp` (`@group(0)`), `LineUniform` + `ClipUniform` (`@group(1)`), and `instances` + `translations` (`@group(2)`), plus depth/ID/tile bindings for ink visibility. Spanning `199` lines, this module stays free of duplicate state by exposing `Layouts` without introducing UI/GPU coupling cycles. Spanning `199` lines, this module stays free of duplicate state by exposing `Layouts` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/pipelines/layouts.rs"
```

### Step 5 · `src/engine/pipelines/core.rs`

`src/engine/pipelines/core.rs` wraps shaders and pipelines in `Lazy<T>` (`Rc<LazyCell<...>>`) inside `Cache`, keying render pipelines by entry points, layouts, topology, sample count, target format and `CLIPPING` specialization so unused variants never compile. Spanning `587` lines, this module stays free of duplicate state by exposing `count_pipeline`, `count_shader`, `created`, `Lazy` without introducing UI/GPU coupling cycles. Spanning `587` lines, this module stays free of duplicate state by exposing `count_pipeline`, `count_shader`, `created`, `Lazy` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/pipelines/core.rs"
```

### Step 6 · `src/engine/pipelines/geometry.rs`

`src/engine/pipelines/geometry.rs` binds `GpuVertex` (`20 B` stride: `pos: [f32; 3]`, `oct32` normal `u32`, `rgba8` `u32`) to arena pipelines and includes the adapter-gated lazy-compilation test verifying `1×`/`4×` MSAA target reuse without redundant shader builds, while `src/engine/gpu/sink.rs` implements `viewer_core::GpuSink` on `Gpu` so `Scene` flushes staged uploads, patches and compactions without depending on `wgpu`. Spanning `71` lines, this module stays free of duplicate state by exposing `vertex_layout` without introducing UI/GPU coupling cycles. Spanning `71` lines, this module stays free of duplicate state by exposing `vertex_layout` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/pipelines/geometry.rs"
```

### Step 7 · `src/shaders/scene.wgsl`

1. **`src/shaders/scene.wgsl`** mirrors the `112 B` `Instance` and `80 B` `LineUniform` structs, resolves selection/tint/edge colours, offsets coplanar contact layers along the view ray (`depth_layer`), and places vertices via `place(i, p) = (instances[i].model * vec4(p, 1.0)).xyz + translations[i].xyz`: Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `144`-line shader defines shared WGSL helper functions and uniform structs compiled directly through `build.rs` into `OUT_DIR`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `144`-line shader defines shared WGSL helper functions and uniform structs compiled directly through `build.rs` into `OUT_DIR`.

```wgsl
--8<-- "session_viewer/src/shaders/scene.wgsl"
```

### Step 8 · `src/shaders/clip.wgsl`

5. **`src/shaders/clip.wgsl`** declares the `448 B` `ClipUniform` (`6` world/screen section planes, hatch numerators/denominators, `CLIP_OFFSETS` for `4×` MSAA, scale-aware `CLIP_SLACK = 2^-17` for faces, `8`-epsilon dot-product rounding for on-plane ink, and `override CLIPPING: bool`): Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `188`-line shader defines shared WGSL helper functions and uniform structs compiled directly through `build.rs` into `OUT_DIR`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `188`-line shader defines shared WGSL helper functions and uniform structs compiled directly through `build.rs` into `OUT_DIR`.

```wgsl
--8<-- "session_viewer/src/shaders/clip.wgsl"
```

### Step 9 · `src/shaders/physical.wgsl`

3. **`src/shaders/physical.wgsl`** splits `32`-bit triangle IDs across two `16`-bit halves (`rg16uint`) alongside colour and `(object, subobject)` pick outputs because WebGPU forbids `4×` MSAA on `r32uint` attachments: Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `16`-line shader defines shared WGSL helper functions and uniform structs compiled directly through `build.rs` into `OUT_DIR`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `16`-line shader defines shared WGSL helper functions and uniform structs compiled directly through `build.rs` into `OUT_DIR`.

```wgsl
--8<-- "session_viewer/src/shaders/physical.wgsl"
```

### Step 10 · `src/shaders/normals.wgsl`

2. **`src/shaders/normals.wgsl`** transforms normals by the cofactor inverse-transpose matrix (preserving perpendicularity under non-uniform scale and flipping sign when $\det(\mathbf{M}) < 0$) and decodes octahedral `16`-bit and `32`-bit normals while reserving `0x80008000` for absent normals: Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `59`-line shader defines shared WGSL helper functions and uniform structs compiled directly through `build.rs` into `OUT_DIR`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `59`-line shader defines shared WGSL helper functions and uniform structs compiled directly through `build.rs` into `OUT_DIR`.

```wgsl
--8<-- "session_viewer/src/shaders/normals.wgsl"
```

### Step 11 · `src/shaders/projected_triangle.wgsl`

4. **`src/shaders/projected_triangle.wgsl`** defines the `96 B` `ProjectedTriangle` screen-space record (`edge0..3` inward line equations, reference point, linear depth gradient and pixel bounds) and evaluates exact point-in-triangle depth across at most `262,144` adaptive screen tiles: Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `52`-line shader defines shared WGSL helper functions and uniform structs compiled directly through `build.rs` into `OUT_DIR`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `52`-line shader defines shared WGSL helper functions and uniform structs compiled directly through `build.rs` into `OUT_DIR`.

```wgsl
--8<-- "session_viewer/src/shaders/projected_triangle.wgsl"
```

### Step 12 · `src/shaders/ink_visibility.wgsl`

3. **`src/shaders/ink_visibility.wgsl`** carries neighbour depth along verified planar surfaces (`ink_pair_planar`) and adds `ink_sag_slack(sag, gradient)` when testing tile triangles so curved BRep edges rising above grazing facets stay unbroken while rear edges disappear cleanly: Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `459`-line shader defines shared WGSL helper functions and uniform structs after `#include` expansion of `projected_triangle.wgsl`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `459`-line shader defines shared WGSL helper functions and uniform structs after `#include` expansion of `projected_triangle.wgsl`.

```wgsl
--8<-- "session_viewer/src/shaders/ink_visibility.wgsl"
```

### Step 13 · `src/engine/gpu/lane.rs`

`src/engine/gpu/lane.rs` defines `Lane` (`reset` vs `release`, `retarget`, `append`, `draw`, `bytes`) and `RowLane` (`Registered` table descriptors retiring edited rows to the hidden sink instead of shifting offsets): Spanning `80` lines, this module stays free of duplicate state by exposing `Lane`, `RowLane`, `Registered`, `REGISTRY` without introducing UI/GPU coupling cycles. Spanning `80` lines, this module stays free of duplicate state by exposing `Lane`, `RowLane`, `Registered`, `REGISTRY` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/lane.rs"
```

### Step 14 · `src/engine/gpu/pass.rs`

Defines `Pass` lifecycle hooks (`PASSES`: instancing, clipping, SSAO, outlines), swapping each pass with `Nothing` during mutable `Gpu` borrows. Spanning `153` lines, this module stays free of duplicate state by exposing `Frame`, `Pass`, `PASSES`, `Nothing` without introducing UI/GPU coupling cycles. Spanning `153` lines, this module stays free of duplicate state by exposing `Frame`, `Pass`, `PASSES`, `Nothing` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/pass.rs"
```

### Step 15 · `src/engine/gpu/render.rs`

Encodes one frame in strict order: pass prep + triangle tiles + splats -> backdrop, caps, faces & splat resolve -> SSAO & outlines -> ink, lanes, markers, text & gizmo. Spanning `552` lines, this module stays free of duplicate state by exposing `encode_frame`, `backdrop_list`, `point_pass`, `id_pass` and verified by `only_stroke_picks_read_the_tables`. Spanning `552` lines, this module stays free of duplicate state by exposing `encode_frame`, `backdrop_list`, `point_pass`, `id_pass` and verified by `only_stroke_picks_read_the_tables`.

```rust
--8<-- "session_viewer/src/engine/gpu/render.rs"
```

### Step 16 · `src/engine/gpu/backdrop.rs`

Renders the fullscreen background triangle and anchor-rebased `f64` construction grid without rebuilding vertex buffers on pan/orbit. Spanning `224` lines, this module stays free of duplicate state by exposing `SHADERS`, `BackdropLane`, `set_grid_frame`, `rebase` and verified by `grid_uniform_keeps_far_world_fractions_across_anchor_returns`. Spanning `224` lines, this module stays free of duplicate state by exposing `SHADERS`, `BackdropLane`, `set_grid_frame`, `rebase` and verified by `grid_uniform_keeps_far_world_fractions_across_anchor_returns`.

```rust
--8<-- "session_viewer/src/engine/gpu/backdrop.rs"
```

### Step 17 · `src/engine/gpu/backdrop/application_tests.rs`

Verifies `0` changed pixels on `WARP` when rebasing the construction grid from `(0, 0, 0)` to `10^12` units and back. Spanning `165` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `165` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/backdrop/application_tests.rs"
```

### Step 18 · `src/shaders/background.wgsl`

4. **`src/shaders/background.wgsl`** and **`src/shaders/grid.wgsl`** draw the paper backdrop (`1.0`, or `0.94` under SSAO) and `±5 m` construction grid without vertex buffers: Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `26`-line shader defines `@vertex fn vs_main`, `@fragment fn fs_main` compiled directly through `build.rs` into `OUT_DIR`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `26`-line shader defines `@vertex fn vs_main`, `@fragment fn fs_main` compiled directly through `build.rs` into `OUT_DIR`.

```wgsl
--8<-- "session_viewer/src/shaders/background.wgsl"
```

### Step 19 · `src/shaders/grid.wgsl`

4. **`src/shaders/background.wgsl`** and **`src/shaders/grid.wgsl`** draw the paper backdrop (`1.0`, or `0.94` under SSAO) and `±5 m` construction grid without vertex buffers: Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `78`-line shader defines `@vertex fn vs_main`, `@fragment fn fs_main` compiled directly through `build.rs` into `OUT_DIR`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `78`-line shader defines `@vertex fn vs_main`, `@fragment fn fs_main` compiled directly through `build.rs` into `OUT_DIR`.

```wgsl
--8<-- "session_viewer/src/shaders/grid.wgsl"
```

## Integration

`src/engine/mod.rs` adds `pub mod pipelines;`, while `src/engine/gpu/mod.rs` registers `lane`, `pass`, `render`, and `backdrop`. Every geometry and ink lane in Tutorials 4–12 compiles against `pipelines::core::PRELUDE` (`scene.wgsl` + `clip.wgsl`) and registers in `lane::REGISTRY` and `pass::PASSES` without reopening `render.rs`.

## Run and observe

Run `cargo xtest -p session_viewer lazy_objects_are_made_once_on_first_use`, `only_stroke_picks_read_the_tables`, and `grid_uniform_keeps_far_world_fractions_across_anchor_returns`: Naga validates every assembled shader in `OUT_DIR/shaders/`, lazy pipelines compile at most once per MSAA sample count, and the ground grid remains phase-locked across camera anchor rebases.

## Predict and try

Predict what happens if you edit a `.wgsl` header (`src/shaders/clip.wgsl`) without `cargo:rerun-if-changed` on `src/shaders`: `OUT_DIR/shaders/` would keep stale expanded WGSL. Inspect `build.rs` to verify it watches both `src/shaders` and everyindividual `.wgsl` file while stripping comments and blank lines.

## Question and answer

**Why compile `scene.wgsl` and `clip.wgsl` into every lane through `PRELUDE` instead of giving each shader its own camera and clipping uniforms?**

**How to work it out:** Count the stages that must agree on world-to-clip projection and section-plane discarding: solid faces, ribbons, vector arrowheads, marker spheres/discs, point-cloud splats, world-plane text, silhouette coverage masks, and the GPU ID picking pass.

**The answer:** Sharing one `PRELUDE` and fixed bind groups (`group(0)` frame camera, `group(1)` line/clip uniform, `group(2)` `Instance` + anchored `translations`) makes divergence impossible: `place(i, p)` rebases every vertex identically, and `clip_discard(world)` cuts faces, strokes, masks, and ID picks on the exact same fragment boundary.

## Validation

Verified via `cargo xtest -p session_viewer pipelines::core::tests::lazy_objects_are_made_once_on_first_use`, `render::tests::only_stroke_picks_read_the_tables`, `backdrop::precision_tests::grid_uniform_keeps_far_world_fractions_across_anchor_returns`, and `instance::validation::tiles_tests::shader_validation_and_layouts`.
