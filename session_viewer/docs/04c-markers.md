# 04c · Markers

Vertex markers and free dots are discs drawn by the fragment shader: a marker from one camera-facing quad, a dot from one triangle.

![A marker is four quad corners pushed out by the pixel radius plus half the feather; a dot is one triangle whose inner circle is the disc.](illustrations/markers.svg)

## Step 1 · src/engine/gpu/glyphs.rs

The 48-byte row of a marker or dot, and the two tables one upload fills.

`lessons/04c/src/engine/gpu/glyphs.rs` · type this, new file

```rust
--8<-- "lessons/04c/src/engine/gpu/glyphs.rs:glyph-rows"
```

## Step 2 · src/engine/gpu/glyphs.rs

One buffer per table with its bind group, the two shaders, the five pipelines, and the lane.

`lessons/04c/src/engine/gpu/glyphs.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04c/src/engine/gpu/glyphs.rs:glyph-lane"
```

## Step 3 · src/engine/gpu/glyphs.rs

Open `impl GlyphLane`: overwrite and hide rows, create the lane, append an upload.

`lessons/04c/src/engine/gpu/glyphs.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04c/src/engine/gpu/glyphs.rs:glyph-impl"
```

## Step 4 · src/engine/gpu/glyphs.rs

Markers draw one quad per row as instances; dots draw three vertices each; the impl block closes.

`lessons/04c/src/engine/gpu/glyphs.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04c/src/engine/gpu/glyphs.rs:glyph-draw"
```

## Step 5 · src/engine/gpu/glyphs.rs

The five pipelines, and the unit quad every marker is drawn from.

`lessons/04c/src/engine/gpu/glyphs.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04c/src/engine/gpu/glyphs.rs:glyph-pipelines"
```

## Step 6 · src/engine/gpu/glyphs.rs

Test: both shaders declare the same row as Rust.

`lessons/04c/src/engine/gpu/glyphs.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/04c/src/engine/gpu/glyphs.rs:glyph-tests"
```

## Step 7 · src/engine/gpu/glyphs.rs

The `Lane` trait, so the Gpu handles this lane in the same loop as the strokes.

`lessons/04c/src/engine/gpu/glyphs.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04c/src/engine/gpu/glyphs.rs:glyph-trait"
```

## Step 8 · src/shaders/sphere.wgsl

A marker is an impostor: the shader's row, and the pen radius in world units and in pixels.

`lessons/04c/src/shaders/sphere.wgsl` · type this, new file

```wgsl
--8<-- "lessons/04c/src/shaders/sphere.wgsl:marker-row"
```

## Step 9 · src/shaders/sphere.wgsl

Whether any face around the vertex looks at the camera.

`lessons/04c/src/shaders/sphere.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04c/src/shaders/sphere.wgsl:marker-facing"
```

## Step 10 · src/shaders/sphere.wgsl

Two entry points first, then the `marker_vertex` they share: place, size, cull and colour one quad corner.

`lessons/04c/src/shaders/sphere.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04c/src/shaders/sphere.wgsl:marker-vertex"
```

## Step 11 · src/shaders/sphere.wgsl

Cut the quad to a soft-edged disc, test it against the faces, write colour or pick id.

`lessons/04c/src/shaders/sphere.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04c/src/shaders/sphere.wgsl:marker-fragments"
```

## Step 12 · src/shaders/glyph.wgsl

The same row for dots, drawn as one triangle whose inner circle is the disc.

`lessons/04c/src/shaders/glyph.wgsl` · type this, new file

```wgsl
--8<-- "lessons/04c/src/shaders/glyph.wgsl:dot-row"
```

## Step 13 · src/shaders/glyph.wgsl

Place one corner of a dot's triangle, sized in pixels, fading when thinner than one.

`lessons/04c/src/shaders/glyph.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04c/src/shaders/glyph.wgsl:dot-vertex"
```

## Step 14 · src/shaders/glyph.wgsl

Disc coverage, colour and the two pick ids: the object, or the cloud point a dot stands for.

`lessons/04c/src/shaders/glyph.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04c/src/shaders/glyph.wgsl:dot-fragments"
```

## Step 15 · src/engine/gpu/mod.rs

Marker and dot rows still drawn, after undo hid some.

`lessons/04c/src/engine/gpu/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04c/src/engine/gpu/mod.rs:markers"
```

## Step 16 · src/engine/gpu/render.rs

The marker and dot draws of the ink pass, each behind its view switch.

`lessons/04c/src/engine/gpu/render.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04c/src/engine/gpu/render.rs:marker-draws"
```

## Step 17 · registration lines

Copy the lines tagged `register:glyphs` and `register:markers` from these files of `lessons/04c/`:

- `src/engine/gpu/mod.rs`: the module, the lane field, its creation, append, patch, kill, release and shader list.
- `src/engine/gpu/patch.rs`, `present.rs`, `upload.rs`, `render.rs`: the marker and dot counts, the glyph rows of an upload, and the two draw calls.

Run `cargo check` in `lessons/04c/`.

## Check

Run `cargo xtest --lib glyphs` in `lessons/04c/`: the test confirms `sphere.wgsl` and `glyph.wgsl` declare `GlyphPoint` with the Rust fields in the Rust order, 48 bytes each.
