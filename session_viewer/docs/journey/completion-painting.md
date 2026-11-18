# Paint shapes and text

Composite an anti-aliased pink `egui` triangle and a bottom-left `Command:` label over Tutorial 1's cleared white WebGPU surface.

![Logical points vs WebGPU NDC coordinates, deferred texture free and two-pass compositing](../illustrations/stage/02-logical-vs-ndc.svg)

![Tutorial 2 output: egui pink triangle and bottom-left Command label at 1280x800](../screenshots/active/painting-triangle-1280.png)

## Prerequisites and ownership

Requires [1. Open a GPU frame](completion-frame.md). All `13` files from `frame` remain unchanged; checkpoint `painting` (`15` files) adds `2` files plus additive `mod` / `STARTS` entries in `src/engine/gpu/mod.rs` and `examples/course/registration.rs`:

| Added file | Role |
| --- | --- |
| [src/engine/gpu/ui.rs](../../src/engine/gpu/ui.rs) | Production `egui_wgpu::Renderer` wrapper: texture delta upload, 1-frame deferred free, tessellation and `LoadOp::Load` draw pass. |
| [examples/course/painting.rs](../../examples/course/painting.rs) | Loads `NotoSans-Subset.ttf`, converts physical texels to logical points, and paints the centred triangle and `Command:` label. |

## Build

Materialize checkpoint `painting` or add the two files below to your `frame` directory:

```sh
npm --prefix ../session_tests run course -- checkpoint painting ../session_viewer/target/learn-painting
```

### Step 1. Wrap `egui_wgpu::Renderer` in `src/engine/gpu/ui.rs`

Create `src/engine/gpu/ui.rs` and declare `pub mod ui;` in `src/engine/gpu/mod.rs`. `Ui::prepare` frees textures retired by the *previous* frame (`free_last_frame`), uploads `textures_delta.set`, tessellates `output.shapes` at `pixels_per_point`, and uploads vertex/index buffers before `Ui::draw` opens a second render pass with `LoadOp::Load` so Tutorial 1's cleared white canvas survives underneath:

```rust
--8<-- "session_viewer/src/engine/gpu/ui.rs"
```

### Step 2. Paint logical shapes and glyphs in `examples/course/painting.rs`

Divide `[config.width, config.height]` by `scale` (`devicePixelRatio`) so `egui` positions the triangle and bottom-left `Command:` label in density-independent logical points while rasterizing edges and `NotoSans-Subset.ttf` glyphs at native physical DPI, then append `("painting", painting::start)` to `examples/course/registration.rs`:

```rust
--8<-- "session_viewer/examples/course/painting.rs:course-painting"
```

## Run and observe

Serve the project and open `http://127.0.0.1:8780/?tutorial=painting`:

- A `#fa9ebc` pink triangle stays centred on `#ffffff`, and `Command:` sits cleanly at the bottom-left margin at both `1280×800` and `390×844`.
- Opening `?tutorial=frame` still renders the plain white surface from Tutorial 1.

## Predict and try

Replace `screen.center()` with `screen.left_top()` in `painting.rs`: predict which triangle vertices land at negative point coordinates and clip at the top-left border, verify in the browser, then restore `screen.center()`.

## Question and answer

**Why does `Ui::draw` open its render pass with `wgpu::LoadOp::Load` instead of `LoadOp::Clear`?**
`Ui::draw` overlays 2D shapes onto a surface already cleared (and in later tutorials, shaded in 3D) earlier in the same encoder; `LoadOp::Clear` would erase Pass 1's pixels before blending UI quads.

## Validation

Verified by locked `wasm32` compilation, native + wasm viewer typechecks, cumulative retention tests, and desktop/upright pixel checks in the recorded Windows Intel WebGPU environment ([receipt](release/completion-partials-windows-20261115.json)). Physical-phone verification remains separate.
