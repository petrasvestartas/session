# Tutorial 3 · Rendering engine and shared shaders

## Goal

Preprocess WGSL `#include` headers at build time through `build.rs`, establish bind-group layouts `0..=3` and lazy `OnceLock` pipeline compilation, and execute the ordered frame render passes with background and construction-plane grid shaders.

![Shaded BRep teapot with octahedral normals and GTAO crease darkening](../screenshots/tutorials/t03-shaded.png) ![Wireframe pass isolating screen-space extruded curve ribbons](../screenshots/tutorials/t03-wireframe.png)

**What we achieve:** Compile-time WGSL `#include` preprocessing (`build.rs`), shared bind-group layouts `@group(0..=3)`, `Lazy<T>` on-demand pipeline specialization, and the ordered frame pass encoder (`render.rs`) with `background.wgsl` and `grid.wgsl`.

## How it works

`build.rs` expands `#include` headers (`scene.wgsl`, `clip.wgsl`, `physical.wgsl`, `normals.wgsl`, `projected_triangle.wgsl`, `ink_visibility.wgsl`) into `OUT_DIR/shaders/`, `scene.wgsl` places vertices via split `instances[i].model` + rebased `translations[i]`, `normals.wgsl` decodes octahedral normals through the cofactor matrix, and `ink_visibility.wgsl` adds `ink_sag_slack(sag, gradient)` so curved BRep chords stay unbroken over grazing mesh facets.

![build.rs WGSL #include expansion, shared @group(0..3) layouts, Lazy<T> pipeline cache, octahedral normals, and curved-edge ink_sag_slack](../screenshots/tutorials/explain-03-rendering.png)

## Compilable checkpoint slice

`build.rs` and the shared `@group(0..=3)` shader prelude compile inside the `rows` assembly (`80` files) once `painting`, `mathematics`, `dock`, and `camera` are registered.

Lines marked `+` show the exact module declarations unlocked between `camera` (`50` files) and `rows` (`80` files):

```rust
// viewer-core/src/lib.rs (camera -> rows)
  pub mod camera;
+ pub mod rows;
+ pub mod sink;
+ pub use crate as app;
+ pub use sink as core;
+ pub use sink::GpuSink;

// src/lib.rs (camera -> rows)
  /// A WGSL file from src/shaders as build.rs wrote it: no comments, indentation or blank lines.
  macro_rules! shader {
      ($name:literal) => {
          include_str!(concat!(env!("OUT_DIR"), "/shaders/", $name))
      };
  }
  mod engine;
+ pub use viewer_core as core;
+ pub use viewer_core::rows;
  pub use viewer_core::camera;
  pub mod command_dock;
  pub mod app;
  #[cfg(all(target_arch = "wasm32", feature = "tutorial"))]
  #[path = "../examples/course/entry.rs"]
  mod course;

// src/engine/mod.rs (camera -> rows)
+ pub mod diagnostics;
  pub mod gpu;
+ pub mod performance;
+ pub mod pipelines;

// src/engine/gpu/mod.rs (camera -> rows)
+ pub mod buffers; // register:buffers
  pub mod device; // register:device
+ pub mod hull; // register:hull
+ pub mod instance; // register:instance
+ pub mod objects;
  pub mod ui; // register:ui

// examples/course/registration.rs (camera -> rows)
  // --8<-- [start:course-registration]
  #[path = "canvas.rs"]
  mod canvas;
  #[path = "frame.rs"]
  mod frame;
  #[path = "painting.rs"]
  mod painting;
  #[path = "mathematics.rs"]
  mod mathematics;
  #[path = "dock.rs"]
  mod dock;
  #[path = "camera.rs"]
  mod camera;
+ #[path = "rows.rs"]
+ mod rows;
  type Start = fn() -> Result<(), wasm_bindgen::JsValue>;
  pub(super) const STARTS: &[(&str, Start)] = &[
      ("frame", frame::start),
      ("painting", painting::start),
      ("mathematics", mathematics::start),
      ("dock", dock::start),
      ("camera", camera::start),
+     ("rows", rows::start),
  ];
  // --8<-- [end:course-registration]
```

## Core code walkthrough

### Compile-time WGSL `#include` expansion (`build.rs:21:61`)

```rust
--8<-- "session_viewer/build.rs:21:61"
```

Standard WGSL has no native `#include` directive. `expand()` (lines `22–45`) recursively splices `#include "file.wgsl"` headers (`scene.wgsl`, `clip.wgsl`, `normals.wgsl`, `ink_visibility.wgsl`) with a `seen` guard so diamond includes emit once per shader, while `minify()` (lines `48–61`) strips `//` comments, rejects `/*` block comments up front, and collapses whitespace into `OUT_DIR/shaders/`.

### Cofactor normal transform & Cigolle `oct32` decode (`src/shaders/normals.wgsl:1:59`)

```wgsl
--8<-- "session_viewer/src/shaders/normals.wgsl:1:59"
```

Non-uniform scaling shears surface normals if multiplied by `model` directly. `transform_normal` (lines `2–22`) computes the cofactor matrix `[y × z, z × x, x × y] * sign(det)`—which equals `det(M) · M⁻ᵀ` without risking division by a tiny determinant—and returns `0` on degenerate matrices. Below it, `oct32_decode` (lines `44–59`) unpacks two `snorm16` numbers (`0x80008000` sentinel = no normal) onto the `L¹` octahedron `|x| + |y| + |z| = 1` and folds the `z < 0` lower hemisphere across the diagonals.

## Prerequisites and files

Requires [Tutorial 2](tutorial-two-wgpu.md). Followed immediately by [Tutorial 3.1 (`egui` painting)](tutorial-three-painting.md) and [Tutorial 3.2 (2D mathematics stage)](tutorial-three-mathematics.md).

**Core files to type inline (6 files):**

1. `build.rs`
2. `src/engine/pipelines/layouts.rs`
3. `src/engine/pipelines/core.rs`
4. `src/shaders/scene.wgsl`
5. `src/shaders/normals.wgsl`
6. `src/engine/gpu/render.rs`

**Reference catalog & test files (13 files, folded below):** `src/engine/pipelines/mod.rs`, `src/engine/pipelines/bindings.rs`, `src/engine/pipelines/geometry.rs`, `src/shaders/clip.wgsl`, `src/shaders/physical.wgsl`, `src/shaders/projected_triangle.wgsl`, `src/shaders/ink_visibility.wgsl`, `src/engine/gpu/lane.rs`, `src/engine/gpu/pass.rs`, `src/engine/gpu/backdrop.rs`, `src/engine/gpu/backdrop/application_tests.rs`, `src/shaders/background.wgsl`, `src/shaders/grid.wgsl`.

## Build

### Step 1 · `build.rs`

`build.rs` expands `#include "..."` directives across `src/shaders/*.wgsl` into `OUT_DIR/shaders/`, strips comments and redundant whitespace, and emits `cargo:rerun-if-changed=src/shaders`. Trap: `build.rs` rejects `/*` block comments explicitly so nested shader includes stay deterministic across Naga validation.

```rust
--8<-- "session_viewer/build.rs"
```

### Step 2 · `src/engine/pipelines/layouts.rs`

`src/engine/pipelines/layouts.rs` assembles `Layouts`: camera `mvp` (`@group(0)`), `LineUniform` + `ClipUniform` (`@group(1)`), and `instances` + `translations` (`@group(2)`), plus depth/ID/tile bindings for ink visibility.

```rust
--8<-- "session_viewer/src/engine/pipelines/layouts.rs"
```

### Step 3 · `src/engine/pipelines/core.rs`

`src/engine/pipelines/core.rs` wraps shaders and pipelines in `Lazy<T>` (`Rc<LazyCell<...>>`) inside `Cache`, keying render pipelines by entry points, layouts, topology, sample count, target format and `CLIPPING` specialization so unused variants never compile.

```rust
--8<-- "session_viewer/src/engine/pipelines/core.rs"
```

### Step 4 · `src/shaders/scene.wgsl`

`src/shaders/scene.wgsl` mirrors the `112 B` `Instance` and `80 B` `LineUniform` structs, resolves selection/tint/edge colours, offsets coplanar contact layers along the view ray (`depth_layer`), and places vertices via `place(i, p) = (instances[i].model * vec4(p, 1.0)).xyz + translations[i].xyz`.

```wgsl
--8<-- "session_viewer/src/shaders/scene.wgsl"
```

### Step 5 · `src/shaders/normals.wgsl`

`src/shaders/normals.wgsl` transforms normals by the cofactor inverse-transpose matrix (preserving perpendicularity under non-uniform scale and flipping sign when `det(M) < 0`) and decodes octahedral `16`-bit and `32`-bit normals while reserving `0x80008000` for absent normals.

```wgsl
--8<-- "session_viewer/src/shaders/normals.wgsl"
```

### Step 6 · `src/engine/gpu/render.rs`

Encodes one frame in strict order: pass prep + triangle tiles + splats -> backdrop, caps, faces & splat resolve -> SSAO & outlines -> ink, lanes, markers, text & gizmo.

```rust
--8<-- "session_viewer/src/engine/gpu/render.rs"
```

## Reference catalog and test files (13 files)

The 6 core files above implement the complete mechanism for this lesson. Expand this reference section whenever you want to inspect or copy the remaining 13 sibling verbs, adapters, and unit-test suites (`Step 7`–`Step 19`).

### Step 7 · `src/engine/pipelines/mod.rs`

`src/engine/pipelines/mod.rs` (7 lines) declares and re-exports module tree `bindings`, `layouts`, `core`, `geometry`. It keeps `mod` deterministic across native `cargo xtest` and `wasm32-unknown-unknown` builds.

```rust
--8<-- "session_viewer/src/engine/pipelines/mod.rs"
```

### Step 8 · `src/engine/pipelines/bindings.rs`

`src/engine/pipelines/bindings.rs` constructs typed uniform, read-only storage and `2D`/multisampled texture `BindGroupLayoutEntry` descriptors without allocating GPU memory.

```rust
--8<-- "session_viewer/src/engine/pipelines/bindings.rs"
```

### Step 9 · `src/engine/pipelines/geometry.rs`

`src/engine/pipelines/geometry.rs` (71 lines) defines `vertex_layout()`. It connects `created`, `gpu`, `session_rust` within `pipelines` across native and `wasm32` targets.

```rust
--8<-- "session_viewer/src/engine/pipelines/geometry.rs"
```

### Step 10 · `src/shaders/clip.wgsl`

`src/shaders/clip.wgsl` declares the `448 B` `ClipUniform` (`6` world/screen section planes, hatch numerators/denominators, `CLIP_OFFSETS` for `4×` MSAA, scale-aware `CLIP_SLACK = 2^-17` for faces, `8`-epsilon dot-product rounding for on-plane ink, and `override CLIPPING: bool`).

```wgsl
--8<-- "session_viewer/src/shaders/clip.wgsl"
```

### Step 11 · `src/shaders/physical.wgsl`

`src/shaders/physical.wgsl` splits `32`-bit triangle IDs across two `16`-bit halves (`rg16uint`) alongside colour and `(object, subobject)` pick outputs because WebGPU forbids `4×` MSAA on `r32uint` attachments.

```wgsl
--8<-- "session_viewer/src/shaders/physical.wgsl"
```

### Step 12 · `src/shaders/projected_triangle.wgsl`

`src/shaders/projected_triangle.wgsl` defines the `96 B` `ProjectedTriangle` screen-space record (`edge0..3` inward line equations, reference point, linear depth gradient and pixel bounds) and evaluates exact point-in-triangle depth across at most `262,144` adaptive screen tiles.

```wgsl
--8<-- "session_viewer/src/shaders/projected_triangle.wgsl"
```

### Step 13 · `src/shaders/ink_visibility.wgsl`

`src/shaders/ink_visibility.wgsl` carries neighbour depth along verified planar surfaces (`ink_pair_planar`) and adds `ink_sag_slack(sag, gradient)` when testing tile triangles so curved BRep edges rising above grazing facets stay unbroken while rear edges disappear cleanly.

```wgsl
--8<-- "session_viewer/src/shaders/ink_visibility.wgsl"
```

### Step 14 · `src/engine/gpu/lane.rs`

`src/engine/gpu/lane.rs` defines `Lane` (`reset` vs `release`, `retarget`, `append`, `draw`, `bytes`) and `RowLane` (`Registered` table descriptors retiring edited rows to the hidden sink instead of shifting offsets).

```rust
--8<-- "session_viewer/src/engine/gpu/lane.rs"
```

### Step 15 · `src/engine/gpu/pass.rs`

Defines `Pass` lifecycle hooks (`PASSES`: instancing, clipping, SSAO, outlines), swapping each pass with `Nothing` during mutable `Gpu` borrows.

```rust
--8<-- "session_viewer/src/engine/gpu/pass.rs"
```

### Step 16 · `src/engine/gpu/backdrop.rs`

Renders the fullscreen background triangle and anchor-rebased `f64` construction grid without rebuilding vertex buffers on pan/orbit.

```rust
--8<-- "session_viewer/src/engine/gpu/backdrop.rs"
```

### Step 17 · `src/engine/gpu/backdrop/application_tests.rs`

`src/engine/gpu/backdrop/application_tests.rs` (165 lines) implements `application_tests` (`grey_from_above()`, `actual_grid_pixels_keep_far_fractions_and_return_to_the_same_near_frame()`, `the_grid_lies_on_the_construction_plane()`) inside `backdrop`. It connects `CPlane`, `scene`, `camera`, `gpu` within `backdrop` across native and `wasm32` targets.

```rust
--8<-- "session_viewer/src/engine/gpu/backdrop/application_tests.rs"
```

### Step 18 · `src/shaders/background.wgsl`

`src/shaders/background.wgsl`: One vertex of the fullscreen triangle. Defines entry points `@vertex fn vs_main`, `@fragment fn fs_main`; keep struct layouts and binding indices aligned with their Rust `bytemuck` mirrors and Naga validation tests.

```wgsl
--8<-- "session_viewer/src/shaders/background.wgsl"
```

### Step 19 · `src/shaders/grid.wgsl`

`src/shaders/grid.wgsl`: Floor line color. Defines entry points `@vertex fn vs_main`, `@fragment fn fs_main`; bindings `@group(2) @binding(0) grid_frame`; keep struct layouts and binding indices aligned with their Rust `bytemuck` mirrors and Naga validation tests.

```wgsl
--8<-- "session_viewer/src/shaders/grid.wgsl"
```

## Integration

`src/engine/mod.rs` adds `pub mod pipelines;`, while `src/engine/gpu/mod.rs` registers `lane`, `pass`, `render`, and `backdrop`. Every geometry and ink lane in Tutorials 4–12 compiles against `pipelines::core::PRELUDE` (`scene.wgsl` + `clip.wgsl`) and registers in `lane::REGISTRY` and `pass::PASSES` without reopening `render.rs`.

## Run and observe

Compile all preprocessed WGSL shaders through `build.rs` and run the Naga validation, lazy-pipeline, and anchored-grid precision tests:

```sh
cargo xtest -p session_viewer lazy_objects_are_made_once_on_first_use
cargo xtest -p session_viewer only_stroke_picks_read_the_tables
cargo xtest -p session_viewer grid_uniform_keeps_far_world_fractions_across_anchor_returns
```

`build.rs` expands every `#include` into `OUT_DIR/shaders/*.wgsl`, Naga validates all bind groups `@group(0..=3)`, and `Lazy<Pipeline>` compiles each pipeline at most once per sample count.

## Predict and try

Predict what happens if you edit a `.wgsl` header (`src/shaders/clip.wgsl`) without `cargo:rerun-if-changed` on `src/shaders`: `OUT_DIR/shaders/` would keep stale expanded WGSL. Inspect `build.rs` to verify it watches both `src/shaders` and everyindividual `.wgsl` file while stripping comments and blank lines.

## Question and answer

**Why compile `scene.wgsl` and `clip.wgsl` into every lane through `PRELUDE` instead of giving each shader its own camera and clipping uniforms?**

??? tip "Think first — click to reveal how to work it out and the answer"
    **How to work it out:** Count the stages that must agree on world-to-clip projection and section-plane discarding: solid faces, ribbons, vector arrowheads, marker spheres/discs, point-cloud splats, world-plane text, silhouette coverage masks, and the GPU ID picking pass.

    **The answer:** Sharing one `PRELUDE` and fixed bind groups (`group(0)` frame camera, `group(1)` line/clip uniform, `group(2)` `Instance` + anchored `translations`) makes divergence impossible: `place(i, p)` rebases every vertex identically, and `clip_discard(world)` cuts faces, strokes, masks, and ID picks on the exact same fragment boundary.

## Validation

Verified via `cargo xtest -p session_viewer pipelines::core::tests::lazy_objects_are_made_once_on_first_use`, `render::tests::only_stroke_picks_read_the_tables`, `backdrop::precision_tests::grid_uniform_keeps_far_world_fractions_across_anchor_returns`, and `instance::validation::tiles_tests::shader_validation_and_layouts`.
