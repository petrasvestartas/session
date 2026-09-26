# 11 · Text rendering

Screen-sized nameplates and a foreshortened scene label appear above the model.

![Five placements of one shaped line, and the same label rasterized once per device scale.](illustrations/text-placement.svg)

Copy each file from the lesson folder to the path shown.

Copy from `lessons/11/` (tooling this checkpoint needs but the course does not teach):

- `lessons/11/src/text_quality.rs`

## Step 1 · src/engine/gpu/text_plate.rs

New file: the plate behind a nameplate, two triangles per plate, cut to the label's clip box.

`lessons/11/src/engine/gpu/text_plate.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/11/src/engine/gpu/text_plate.rs:step-1a"
```

`lessons/11/src/engine/gpu/text_plate.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plate.rs:step-1b"
```

`lessons/11/src/engine/gpu/text_plate.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plate.rs:step-1c"
```

## Step 2 · src/shaders/text_plate.wgsl

New shader: a rounded-box distance gives each plate a soft edge one pixel wide at any size.

`lessons/11/src/shaders/text_plate.wgsl` · 28 lines · type this, new file

```wgsl
--8<-- "lessons/11/src/shaders/text_plate.wgsl"
```

## Step 3 · src/engine/gpu/text_plane.rs

New file: text lying on a world plane, rasterized once into its own texture and drawn as one quad.

`lessons/11/src/engine/gpu/text_plane.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/11/src/engine/gpu/text_plane.rs:step-3a"
```

`lessons/11/src/engine/gpu/text_plane.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plane.rs:step-3b"
```

`lessons/11/src/engine/gpu/text_plane.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plane.rs:step-3c"
```

`lessons/11/src/engine/gpu/text_plane.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plane.rs:step-3d"
```

`lessons/11/src/engine/gpu/text_plane.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plane.rs:step-3e"
```

`lessons/11/src/engine/gpu/text_plane.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plane.rs:step-3f"
```

`lessons/11/src/engine/gpu/text_plane.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plane.rs:step-3g"
```

`lessons/11/src/engine/gpu/text_plane.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plane.rs:step-3h"
```

`lessons/11/src/engine/gpu/text_plane.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plane.rs:step-3i"
```

## Step 4 · src/shaders/text_plane.wgsl

New shader: read the label's coverage texture and cut it to the clip box.

`lessons/11/src/shaders/text_plane.wgsl` · 31 lines · type this, new file

```wgsl
--8<-- "lessons/11/src/shaders/text_plane.wgsl"
```

## Step 5 · src/engine/gpu/text_plane.rs

Copy the GPU test for plane text to the end of the file.

`lessons/11/src/engine/gpu/text_plane.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text_plane.rs:step-5"
```

## Step 6 · src/engine/gpu/text.rs

New file: the text lane places every label each frame, then draws planes, depth-tested text, plates and overlays.

`lessons/11/src/engine/gpu/text.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/11/src/engine/gpu/text.rs:step-6a"
```

`lessons/11/src/engine/gpu/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text.rs:step-6b"
```

`lessons/11/src/engine/gpu/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text.rs:step-6c"
```

`lessons/11/src/engine/gpu/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text.rs:step-6d"
```

`lessons/11/src/engine/gpu/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text.rs:step-6e"
```

`lessons/11/src/engine/gpu/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text.rs:step-6f"
```

`lessons/11/src/engine/gpu/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text.rs:step-6g"
```

`lessons/11/src/engine/gpu/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text.rs:step-6h"
```

`lessons/11/src/engine/gpu/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text.rs:step-6i"
```

`lessons/11/src/engine/gpu/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text.rs:step-6j"
```

Copy this part from the lesson folder to the path shown.

`lessons/11/src/engine/gpu/text.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/11/src/engine/gpu/text.rs:step-6k"
```

Run `cargo check` in `lessons/11/`.

## Step 7 · src/engine/gpu/mod.rs

The GPU owner connects buffers, pipelines and frame resources.

`lessons/11/src/engine/gpu/mod.rs` · edit · type this

Added after the `pub mod targets;` line of `lessons/10/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/11/src/engine/gpu/mod.rs:step-7a"
```

Added after the `pub glyphs: glyphs::GlyphLane,` line in `struct Gpu` of `lessons/10/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/11/src/engine/gpu/mod.rs:step-7b"
```

Added after the `let glyphs = glyphs::GlyphLane::new(&ctx, &la…` line in `fn new` of `lessons/10/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/11/src/engine/gpu/mod.rs:step-7c"
```

Added after the `glyphs,` line in `fn new` of `lessons/10/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/11/src/engine/gpu/mod.rs:step-7d"
```

Added after the `self.glyphs.retarget(&self.ctx, &self.layouts…` line in `fn retarget` of `lessons/10/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/11/src/engine/gpu/mod.rs:step-7e"
```

Replaces `fn write_frame_uniforms` in `lessons/10/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/11/src/engine/gpu/mod.rs:step-7f"
```

Replaces the 5 lines from `}` in `impl Gpu` of `lessons/10/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/11/src/engine/gpu/mod.rs:step-7g"
```

Added after the `self.arena.draw_text(&mut pass, &basic);` line in `fn render` of `lessons/10/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/11/src/engine/gpu/mod.rs:step-7h"
```

## Step 8 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners.

`lessons/11/src/lib.rs` · edit · type this

Replaces `mod app` in `lessons/10/src/lib.rs`

```rust
--8<-- "lessons/11/src/lib.rs:step-8a"
```

Added after the `fixture.upload.drop_uploaded();` line in `fn create` of `lessons/10/src/lib.rs`

```rust
--8<-- "lessons/11/src/lib.rs:step-8b"
```

Replaces the `Ok(serde_json::json!({"stage":10,"objects":se…` line in `fn render` of `lessons/10/src/lib.rs`

```rust
--8<-- "lessons/11/src/lib.rs:step-8c"
```

Added after the `}` line of `lessons/10/src/lib.rs`

```rust
--8<-- "lessons/11/src/lib.rs:step-8d"
```

## Step 9 · src/text_layout.rs

Delete lesson 10's shaping check; the text-quality page below replaces it.

Delete `src/text_layout.rs` (it exists in `lessons/10/`, not in `lessons/11/`).

## Step 10 · assets/text-layout.html

Delete its check page too.

Delete `assets/text-layout.html` (it exists in `lessons/10/`, not in `lessons/11/`).

## Step 11 · assets/text-quality.html

Copy the test page that draws GPU text beside browser text at five sizes and any device scale.

`lessons/11/assets/text-quality.html` · 167 lines · copy the file, new file

```html
--8<-- "lessons/11/assets/text-quality.html"
```

## Step 12 · index.html

Copy this file from the lesson folder to the path shown.

`lessons/11/index.html` · edit · copy the file

Replaces the `<title>Session checkpoint 10</title>` line of `lessons/10/index.html`

```html
--8<-- "lessons/11/index.html:step-12a"
```

Replaces the 7 lines from `<link data-trunk rel="copy-file" href="assets…` of `lessons/10/index.html`

```html
--8<-- "lessons/11/index.html:step-12b"
```

Replaces the `document.getElementById('status').textContent…` line of `lessons/10/index.html`

```html
--8<-- "lessons/11/index.html:step-12c"
```

## Check

Run `trunk serve` in `lessons/11/` and open <http://127.0.0.1:8770/>.

Expected: Screen-sized nameplates and a foreshortened scene label appear above the model; status: **3 objects**.

![Checkpoint 11: two nameplates above the sphere, one rounded, and a fixed-plane label foreshortened on its own plane.](screenshots/11.png)

If it fails:

- Letters blur at one zoom level: the text frame scale disagrees with the framebuffer.
- The last glyph clips: plate padding excludes the glyph overhang.

## What changed

```text
lessons/11/src/engine/gpu/
├── arena.rs
├── backdrop.rs
├── buffers.rs
├── cloud.rs
├── frame.rs
├── glyphs.rs
├── instance.rs
├── lod.rs
├── mod.rs  ~
├── objects.rs
├── segments.rs
├── splat.rs
├── targets.rs
├── text.rs  +
├── text_outline.rs
├── text_plane.rs  +
├── text_plate.rs  +
├── upload.rs
└── view.rs
```

`+` new in this lesson · `~` changed in this lesson

Every file at this point: `lessons/11/`.

## Next

[12 · maintained viewer shell and picking](12-picking.md)

## Expected viewer result

Checkpoint 11: two nameplates above the sphere, one rounded, and a fixed-plane label foreshortened on its own plane.

[![Full viewer result for 11 text rendering](screenshots/11.png)](screenshots/11.png)
