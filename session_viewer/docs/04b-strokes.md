# 04b · Strokes and arrows

Lines, curves and mesh edges become ribbons: quads the vertex shader lays around each segment on screen, so they keep their pixel width at any zoom. Arrows get a lane of their own, whose tip lands exactly on the end point.

![Six vertices place a quad around the projected segment, and each pixel's coverage is the area of its square inside the band.](illustrations/ribbon.svg)

## Step 1 · src/engine/gpu/segments.rs

The 40-byte segment row the shaders read, and the tables one upload fills: pipes for mesh edges, ribbons for free curves.

`lessons/04b/src/engine/gpu/segments.rs` · type this, new file

```rust
--8<-- "lessons/04b/src/engine/gpu/segments.rs:segment-rows"
```

## Step 2 · src/engine/gpu/segments.rs

Drawing sheets stream in ordered batches; these helpers record where each batch landed on the GPU.

`lessons/04b/src/engine/gpu/segments.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/segments.rs:sheets"
```

## Step 3 · src/engine/gpu/segments.rs

Link each segment to its neighbours, so joints draw once, and mark ends that stop under an arrowhead.

`lessons/04b/src/engine/gpu/segments.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/segments.rs:joints"
```

## Step 4 · src/engine/gpu/segments.rs

One buffer per kind of segment with its bind group, the nine pipelines, and the lane that owns them.

`lessons/04b/src/engine/gpu/segments.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/segments.rs:tables"
```

## Step 5 · src/engine/gpu/segments.rs

Open `impl SegmentLane`: create the lane, and append an upload, rebuilding a bind group whenever its buffer was replaced.

`lessons/04b/src/engine/gpu/segments.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/segments.rs:lane-new"
```

## Step 6 · src/engine/gpu/segments.rs

Still inside the impl: hide rows for undo, overwrite an edited object's rows, and remember the selection.

`lessons/04b/src/engine/gpu/segments.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/segments.rs:lane-edit"
```

## Step 7 · src/engine/gpu/segments.rs

Draw unselected lines first and selected ones after, so the highlight sits on top.

`lessons/04b/src/engine/gpu/segments.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/segments.rs:lane-draw"
```

## Step 8 · src/engine/gpu/segments.rs

Draw the edges into the one-channel masks the outline pass reads.

`lessons/04b/src/engine/gpu/segments.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/segments.rs:lane-masks"
```

## Step 9 · src/engine/gpu/segments.rs

Colour and pick-id draws, all through `draw_table`, which also repeats each instanced copy.

`lessons/04b/src/engine/gpu/segments.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/segments.rs:lane-ids"
```

## Step 10 · src/engine/gpu/segments.rs

Reset, release and the row counts; the closing brace ends the impl block.

`lessons/04b/src/engine/gpu/segments.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/segments.rs:lane-counts"
```

## Step 11 · src/engine/gpu/segments.rs

Build the nine pipelines from one shader; each picks its entry points, target and blending.

`lessons/04b/src/engine/gpu/segments.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/segments.rs:pipelines"
```

## Step 12 · src/engine/gpu/segments.rs

Tests: the Rust and WGSL rows agree, sheet batches map to rows, and headed ends stop under the head.

`lessons/04b/src/engine/gpu/segments.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/segments.rs:tests"
```

## Step 13 · src/engine/gpu/segments.rs

Implement the `Lane` trait, so the Gpu retargets, resets and releases this lane with the others.

`lessons/04b/src/engine/gpu/segments.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/segments.rs:lane-trait"
```

## Step 14 · src/engine/gpu/segments.rs

A second `impl SegRows` block merges two uploads, shifting chains and ids by the rows already there.

`lessons/04b/src/engine/gpu/segments.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/segments.rs:merge"
```

## Step 15 · src/shaders/ribbon.wgsl

The shader's segment row, the arrowhead constants, and the three bindings of group 3.

`lessons/04b/src/shaders/ribbon.wgsl` · type this, new file

```wgsl
--8<-- "lessons/04b/src/shaders/ribbon.wgsl:stroke-row"
```

## Step 16 · src/shaders/ribbon.wgsl

Whether a mesh edge faces the camera, and the radius field turned into a half width in pixels.

`lessons/04b/src/shaders/ribbon.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04b/src/shaders/ribbon.wgsl:width"
```

## Step 17 · src/shaders/ribbon.wgsl

The arrowhead's length and tuck, the same lines as the vector shader, so curve and head meet without a seam.

`lessons/04b/src/shaders/ribbon.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04b/src/shaders/ribbon.wgsl:head-size"
```

## Step 18 · src/shaders/ribbon.wgsl

Walk the chain to a headed end and return the straight line where the curve stops under the head.

`lessons/04b/src/shaders/ribbon.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04b/src/shaders/ribbon.wgsl:head-cut"
```

## Step 19 · src/shaders/ribbon.wgsl

Exact pixel coverage of a band, and hairlines that never get thinner than a pixel but fade instead.

`lessons/04b/src/shaders/ribbon.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04b/src/shaders/ribbon.wgsl:coverage-math"
```

## Step 20 · src/shaders/ribbon.wgsl

What each ribbon corner hands the fragment shader, and which of the six vertices is which corner.

`lessons/04b/src/shaders/ribbon.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04b/src/shaders/ribbon.wgsl:varyings"
```

## Step 21 · src/shaders/ribbon.wgsl

Back edges hide unless the faces are see-through; an instanced draw takes its object row from the slot.

`lessons/04b/src/shaders/ribbon.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04b/src/shaders/ribbon.wgsl:hidden-edges"
```

## Step 22 · src/shaders/ribbon.wgsl

The bisector at a joint, where one segment's ribbon hands over to the next.

`lessons/04b/src/shaders/ribbon.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04b/src/shaders/ribbon.wgsl:joint-plane"
```

## Step 23 · src/shaders/ribbon.wgsl

Place one ribbon corner: clip at the near plane, project both ends, widen by the half width.

`lessons/04b/src/shaders/ribbon.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04b/src/shaders/ribbon.wgsl:stroke-vertex"
```

## Step 24 · src/shaders/ribbon.wgsl

Six vertex entry points: colour passes see through glass, pick and mask passes keep hiding back edges.

`lessons/04b/src/shaders/ribbon.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04b/src/shaders/ribbon.wgsl:entry-points"
```

## Step 25 · src/shaders/ribbon.wgsl

Coverage of one pixel: cut at joints and under heads, flat at a headed curve's bare ends.

`lessons/04b/src/shaders/ribbon.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04b/src/shaders/ribbon.wgsl:coverage"
```

## Step 26 · src/shaders/ribbon.wgsl

The colour fragment: coverage times visibility, faded behind see-through faces.

`lessons/04b/src/shaders/ribbon.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04b/src/shaders/ribbon.wgsl:color"
```

## Step 27 · src/shaders/ribbon.wgsl

Mask and pick-id fragments reuse the same coverage and visibility tests.

`lessons/04b/src/shaders/ribbon.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04b/src/shaders/ribbon.wgsl:masks-ids"
```

## Step 28 · src/shaders/ink_visibility.wgsl

The shared ink test every ink shader appends: is this pixel of a stroke hidden behind a face?

`lessons/04b/src/shaders/ink_visibility.wgsl` · copy the file

```wgsl
--8<-- "lessons/04b/src/shaders/ink_visibility.wgsl"
```

## Step 29 · src/shaders/projected_triangle.wgsl

The projected triangles that ink test reads.

`lessons/04b/src/shaders/projected_triangle.wgsl` · copy the file

```wgsl
--8<-- "lessons/04b/src/shaders/projected_triangle.wgsl"
```

## Step 30 · src/engine/gpu/vectors.rs

The arrow row: start, end, radius, head length, and which ends carry heads.

`lessons/04b/src/engine/gpu/vectors.rs` · type this, new file

```rust
--8<-- "lessons/04b/src/engine/gpu/vectors.rs:vector-row"
```

## Step 31 · src/engine/gpu/vectors.rs

The vector lane, its registry entry, and one instanced draw for every arrow.

`lessons/04b/src/engine/gpu/vectors.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/vectors.rs:vector-lane"
```

## Step 32 · src/engine/gpu/vectors.rs

`Lane` and `RowLane`: append, draw colour and ids, overwrite rows, and hide them for undo.

`lessons/04b/src/engine/gpu/vectors.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/vectors.rs:vector-traits"
```

## Step 33 · src/engine/gpu/vectors.rs

Two pipelines, colour and pick id, neither with a vertex buffer.

`lessons/04b/src/engine/gpu/vectors.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/vectors.rs:vector-pipelines"
```

## Step 34 · src/engine/gpu/vectors.rs

Tests: the row layout matches the WGSL, and a rendered arrow's tip lands on its end point.

`lessons/04b/src/engine/gpu/vectors.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/vectors.rs:vector-tests"
```

## Step 35 · src/shaders/vector.wgsl

The arrow row in WGSL, the head constants, and what the vertex shader passes on.

`lessons/04b/src/shaders/vector.wgsl` · type this, new file

```wgsl
--8<-- "lessons/04b/src/shaders/vector.wgsl:vector-struct"
```

## Step 36 · src/shaders/vector.wgsl

Half width, head length and tuck, the same lines as in ribbon.wgsl.

`lessons/04b/src/shaders/vector.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04b/src/shaders/vector.wgsl:vector-helpers"
```

## Step 37 · src/shaders/vector.wgsl

Place 18 corners: a shaft quad that stops under each head's base, and one quad per head.

`lessons/04b/src/shaders/vector.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04b/src/shaders/vector.wgsl:vector-vertex"
```

## Step 38 · src/shaders/vector.wgsl

Coverage: a flat-ended band for the shaft, a filtered triangle for each head.

`lessons/04b/src/shaders/vector.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04b/src/shaders/vector.wgsl:vector-coverage"
```

## Step 39 · src/shaders/vector.wgsl

Colour and pick-id fragments, with the same visibility test as the ribbons.

`lessons/04b/src/shaders/vector.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04b/src/shaders/vector.wgsl:vector-fragments"
```

## Step 40 · src/engine/pipelines/mod.rs

An ink shader is its own code plus the shared ink test and the scene prelude.

`lessons/04b/src/engine/pipelines/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/pipelines/mod.rs:ink"
```

## Step 41 · src/engine/gpu/objects.rs

Bind group 2 for ink: the object rows plus the depth textures the ink test reads.

`lessons/04b/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/objects.rs:ink-group"
```

## Step 42 · src/engine/gpu/mod.rs

Live row counts for pipes and ribbons, and a rebuild of the ink bind group.

`lessons/04b/src/engine/gpu/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/mod.rs:ink"
```

## Step 43 · src/engine/gpu/render.rs

The ink pass: lines, selection, other passes' overlays, registered lanes, then text.

`lessons/04b/src/engine/gpu/render.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/render.rs:ink-pass"
```

## Step 44 · registration lines

Each lane joins the viewer through lines tagged `register:`; copy the ones tagged `strokes`, `segments`, `vectors` and `ink` from these files of `lessons/04b/`:

- `src/engine/gpu/mod.rs`: the modules, the lane fields, their creation, append, reset and shader lists.
- `src/engine/gpu/lane.rs`: the vector lane's entry in the lane registry.
- `src/engine/gpu/objects.rs`, `patch.rs`, `present.rs`, `upload.rs`, `render.rs`: the ink group field, the pipe and ribbon counts, the segment rows of an upload, and the ink pass call.

Run `cargo check` in `lessons/04b/`.

## Check

Run `cargo xtest --lib -- segments vectors` in `lessons/04b/`. The arrow test draws eleven arrows (thin, thick, selected, two heads, head only, shorter than their head) in perspective and parallel views, at 1x and 2x pixel scale, with and without MSAA. It passes only when every tip lands within a quarter pixel of its end point, no ink of the shaft or the head reaches more than half a pixel past it, and an end without a head is cut flat at its point; it prints the worst tip error it saw.
