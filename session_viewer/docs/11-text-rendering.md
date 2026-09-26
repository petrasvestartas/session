# 11 · Text rendering

Glyphon draws screen and anchored labels from one glyph atlas, rounded plates sit behind them, and a label lying on a plane gets a texture of its own. The camera moves labels every frame, but a label is shaped only once.

![Five placements of one shaped line, and the same label rasterized once per device scale.](illustrations/text-placement.svg)

## Step 1 · src/engine/gpu/text.rs

New file: the camera, canvas and clipping planes the text lane reads each frame, all planes zero until 18b, and its counters.

`lessons/11/src/engine/gpu/text.rs` · type this, new file

```rust
--8<-- "lessons/11/src/engine/gpu/text.rs:frame"
```

## Step 2 · src/engine/gpu/text.rs

The lane: one glyph atlas, two glyphon renderers that differ only in their depth test, the plates and the planes.

`lessons/11/src/engine/gpu/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text.rs:lane"
```

## Step 3 · src/engine/gpu/text.rs

Open `impl TextLane`: create the atlas and both renderers, and rebuild them when the MSAA sample count changes.

`lessons/11/src/engine/gpu/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text.rs:lane-new"
```

## Step 4 · src/engine/gpu/text.rs

Prepare a frame: skip it when nothing moved, otherwise place every label, collect its plate and hand glyphon the text areas.

`lessons/11/src/engine/gpu/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text.rs:prepare"
```

## Step 5 · src/engine/gpu/text.rs

Draw planes, plates and glyphs in layering order, and free everything on release; the impl block closes.

`lessons/11/src/engine/gpu/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text.rs:draw"
```

## Step 6 · src/engine/gpu/text.rs

Project a label's world point to pixels, drop it when clipped or behind the camera, and size world text by distance.

`lessons/11/src/engine/gpu/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text.rs:place"
```

## Step 7 · src/engine/gpu/text.rs

The plate behind a nameplate or an object's label, centred and padded, and a label's clip box in framebuffer pixels.

`lessons/11/src/engine/gpu/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text.rs:rectangle"
```

## Step 8 · src/engine/gpu/text.rs

A glyphon renderer with a given depth test, and an atlas that matches the canvas colour format.

`lessons/11/src/engine/gpu/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text.rs:glyphon"
```

## Step 9 · src/engine/gpu/text.rs

Tests: the device scale is applied once, anchored labels keep their depth, and three GPU tests draw real glyphs.

`lessons/11/src/engine/gpu/text.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text.rs:tests"
```

## Step 10 · src/engine/gpu/text.rs

The `Lane` trait, so the Gpu retargets, resets and releases the text lane together with every other lane.

`lessons/11/src/engine/gpu/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text.rs:lane-trait"
```

## Step 11 · src/engine/gpu/text_plate.rs

New file: a plate rectangle in screen pixels, and the buffer and two pipelines that draw the plates.

`lessons/11/src/engine/gpu/text_plate.rs` · type this, new file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plate.rs:plate-rows"
```

## Step 12 · src/engine/gpu/text_plate.rs

Six clip-space vertices per plate, depth-tested plates first and overlays after, drawn as two ranges of one buffer.

`lessons/11/src/engine/gpu/text_plate.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plate.rs:plate-impl"
```

## Step 13 · src/engine/gpu/text_plate.rs

The colour and id pipelines of the plates, compiled on first use; they read depth and never write it.

`lessons/11/src/engine/gpu/text_plate.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plate.rs:plate-pipeline"
```

## Step 14 · src/shaders/text_plate.wgsl

New file: the plate vertex, passed on as clip space, or mapped into the pick window for the id pass.

`lessons/11/src/shaders/text_plate.wgsl` · type this, new file

```wgsl
--8<-- "lessons/11/src/shaders/text_plate.wgsl:plate-vertex"
```

## Step 15 · src/shaders/text_plate.wgsl

A signed distance to the rounded edge gives a one-pixel soft border; the id pass keeps only pixels inside.

`lessons/11/src/shaders/text_plate.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/11/src/shaders/text_plate.wgsl:plate-fragment"
```

## Step 16 · src/engine/gpu/text_plane.rs

New file: a plane label cached as its own coverage texture, freed on drop, and the set of those labels.

`lessons/11/src/engine/gpu/text_plane.rs` · type this, new file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plane.rs:plane-cache"
```

## Step 17 · src/engine/gpu/text_plane.rs

Open `impl Planes`: the texture-and-sampler layout, a linear sampler and both pipelines.

`lessons/11/src/engine/gpu/text_plane.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plane.rs:plane-new"
```

## Step 18 · src/engine/gpu/text_plane.rs

Keep the textures of labels that still exist, repaint one only when its text, font or needed sharpness changed, then place the quads.

`lessons/11/src/engine/gpu/text_plane.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plane.rs:plane-prepare"
```

## Step 19 · src/engine/gpu/text_plane.rs

One draw per label, since each has its own texture; reset, release and byte counts close the impl block.

`lessons/11/src/engine/gpu/text_plane.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plane.rs:plane-draw"
```

## Step 20 · src/engine/gpu/text_plane.rs

Pick the texture's pixels per em from the label's size on screen, rounded up to a power of two.

`lessons/11/src/engine/gpu/text_plane.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plane.rs:plane-raster"
```

## Step 21 · src/engine/gpu/text_plane.rs

Paint every glyph of the label into one coverage image, padded for the plate around it.

`lessons/11/src/engine/gpu/text_plane.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plane.rs:plane-rasterize"
```

## Step 22 · src/engine/gpu/text_plane.rs

Six vertices spanning the label on its plane, projected on the CPU, with the ink made linear for an sRGB canvas.

`lessons/11/src/engine/gpu/text_plane.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plane.rs:plane-quad"
```

## Step 23 · src/engine/gpu/text_plane.rs

The colour and id pipelines of the plane labels: 64-byte vertices, depth read and never written.

`lessons/11/src/engine/gpu/text_plane.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plane.rs:plane-pipeline"
```

## Step 24 · src/engine/gpu/text_plane.rs

Tests: a long label fits the texture limit, and on a GPU a plane label hides behind a solid.

`lessons/11/src/engine/gpu/text_plane.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plane.rs:plane-tests"
```

## Step 25 · src/shaders/text_plane.wgsl

New file: the plane vertex with its texture coordinate and clip box, and the coverage texture it samples.

`lessons/11/src/shaders/text_plane.wgsl` · type this, new file

```wgsl
--8<-- "lessons/11/src/shaders/text_plane.wgsl:plane-vertex"
```

## Step 26 · src/shaders/text_plane.wgsl

Mix the ink over the plate colour by glyph coverage, cut to the clip box and the plate's rounded edge.

`lessons/11/src/shaders/text_plane.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/11/src/shaders/text_plane.wgsl:plane-fragment"
```

## Step 27 · src/engine/gpu/present.rs

A second `impl Gpu` block fills the frame facts from the camera and prepares the labels before the frame is drawn.

`lessons/11/src/engine/gpu/present.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/present.rs:prepare-text"
```

## Step 28 · src/text_quality.rs

Copy the check page's wasm export: it draws six sample lines at five sizes with the text lane.

`lessons/11/src/text_quality.rs` · copy the file, new file

```rust
--8<-- "lessons/11/src/text_quality.rs"
```

## Step 29 · src/lib.rs

Declare the check page's module, compiled for the browser only.

`lessons/11/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/lib.rs:text-quality-mod"
```

## Step 30 · assets/text-quality.html

Copy the check page: the text lane on the left, the browser's own text in the same font on the right.

`lessons/11/assets/text-quality.html` · copy the file, new file

```html
--8<-- "lessons/11/assets/text-quality.html"
```

## Step 31 · registration lines

Copy the lines tagged `register:text` from these files of `lessons/11/`:

- `src/engine/gpu/mod.rs`: the module, the lane field and its creation.
- `src/engine/gpu/present.rs`: the call that prepares the labels.
- `src/engine/gpu/render.rs`: the draw call.

Run `cargo check` in `lessons/11/`.

## Check

Run `cargo xtest --lib text` in `lessons/11/`, then `trunk serve` and open <http://127.0.0.1:8770/text-quality.html>: both columns show the same lines, and the status line reports their largest width difference in CSS pixels.

![The text-quality page: the text lane on the left, browser text in the same font on the right.](screenshots/11-text-quality.png)
