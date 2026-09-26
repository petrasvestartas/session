# 04a · Meshes on the GPU

Every mesh of a scene lands in one arena: a shared vertex buffer, a parallel table of object rows, and index runs drawn with `draw_indexed`.
The mesh shader is written in its final form here, including the clipping test and the instance slot that later lessons fill.

![One growable arena holds every mesh's vertices and a parallel table gives every vertex its object row; a mesh is a range of indices, and a draw binds both vertex buffers, binds one index run and calls draw_indexed.](illustrations/arena.svg)

## Step 1 · src/shaders/clip.wgsl

The clipping prelude: the plane uniform, the `CLIPPING` override and `clip_active`; with zero planes nothing is cut.

`lessons/04a/src/shaders/clip.wgsl` · type this, new file

```wgsl
--8<-- "lessons/04a/src/shaders/clip.wgsl:clip-uniform"
```

## Step 2 · src/shaders/clip.wgsl

The signed distance to a plane and three cut tests: for a scene point, a segment, and a screen point.

`lessons/04a/src/shaders/clip.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04a/src/shaders/clip.wgsl:clip-tests"
```

## Step 3 · src/shaders/clip.wgsl

Cut per MSAA sample: a mask of the samples that lie on the kept side of every plane.

`lessons/04a/src/shaders/clip.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04a/src/shaders/clip.wgsl:clip-samples"
```

## Step 4 · src/shaders/clip.wgsl

Plane helpers the section caps of lesson 18b read: eye side, depth and slope on screen, cap ids, pick words.

`lessons/04a/src/shaders/clip.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04a/src/shaders/clip.wgsl:clip-planes"
```

## Step 5 · src/shaders/triangle.wgsl

The mesh vertex shader: unpack, place and colour one vertex, with its object row taken through slot 0.

`lessons/04a/src/shaders/triangle.wgsl` · type this, new file

```wgsl
--8<-- "lessons/04a/src/shaders/triangle.wgsl:triangle-vertex"
```

## Step 6 · src/shaders/triangle.wgsl

Vertex pulling: read each vertex from storage by its index, so a triangle knows its own id and source face.

`lessons/04a/src/shaders/triangle.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04a/src/shaders/triangle.wgsl:triangle-pulled"
```

## Step 7 · src/shaders/triangle.wgsl

Shading: a headlight, red back faces, and translucent solids that drop their far side; lesson 36 turns opacity on.

`lessons/04a/src/shaders/triangle.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04a/src/shaders/triangle.wgsl:triangle-shade"
```

## Step 8 · src/shaders/triangle.wgsl

The fragment entry points: shaded colour, pick ids, outline masks, and colour cut sample by sample.

`lessons/04a/src/shaders/triangle.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04a/src/shaders/triangle.wgsl:triangle-fragments"
```

## Step 9 · src/shaders/triangle.wgsl

Crossing counts behind a clipping plane, which tell lesson 18b where to paint section caps.

`lessons/04a/src/shaders/triangle.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04a/src/shaders/triangle.wgsl:triangle-count"
```

## Step 10 · src/shaders/triangle.wgsl

Picking a section cap: atomic counters name the solid under the clicked pixel.

`lessons/04a/src/shaders/triangle.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04a/src/shaders/triangle.wgsl:triangle-pick-caps"
```

## Step 11 · src/shaders/text_outline.wgsl

The print shader for sheet fills and lettering: flat colour, yellow when selected, cut in screen space.

`lessons/04a/src/shaders/text_outline.wgsl` · type this, new file

```wgsl
--8<-- "lessons/04a/src/shaders/text_outline.wgsl:outline-shader"
```

## Step 12 · src/engine/gpu/slots.rs

Slot values, the `Draw` record, and three layouts that step the slot buffer once per instance.

`lessons/04a/src/engine/gpu/slots.rs` · type this, new file

```rust
--8<-- "lessons/04a/src/engine/gpu/slots.rs:slots-layouts"
```

## Step 13 · src/engine/gpu/slots.rs

The slot buffer, which holds slot 0 alone until lesson 18a writes instances into it.

`lessons/04a/src/engine/gpu/slots.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/slots.rs:slots"
```

## Step 14 · src/engine/gpu/slots.rs

The slot table: a texture of four u32 per texel that finds an instance from a triangle id.

`lessons/04a/src/engine/gpu/slots.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/slots.rs:slot-table"
```

## Step 15 · src/engine/gpu/slots.rs

The slot allocator that lesson 18a drives; copy it now so the arena compiles.

`lessons/04a/src/engine/gpu/slots.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/slots.rs:slot-space"
```

## Step 16 · src/engine/gpu/slots.rs

The slot tests.

`lessons/04a/src/engine/gpu/slots.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/slots.rs:slots-tests"
```

## Step 17 · src/engine/gpu/text_outline.rs

The print lane: one shader, three pipelines; the methods call `draw`, written in the next step.

`lessons/04a/src/engine/gpu/text_outline.rs` · type this, new file

```rust
--8<-- "lessons/04a/src/engine/gpu/text_outline.rs:outline-lane"
```

## Step 18 · src/engine/gpu/text_outline.rs

The shared indexed draw and the three pipelines: colour blended on top, ids at equal depth.

`lessons/04a/src/engine/gpu/text_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/text_outline.rs:outline-draw"
```

## Step 19 · src/engine/gpu/upload.rs

Register the mesh rows in `Upload`: four lines, one in the struct, `default`, `drop_uploaded` and `merge`.

`lessons/04a/src/engine/gpu/upload.rs` · type this, one line in `pub struct Upload`

```rust
--8<-- "lessons/04a/src/engine/gpu/upload.rs:upload-arena-field"
```

`lessons/04a/src/engine/gpu/upload.rs` · type this, one line in `fn default`

```rust
--8<-- "lessons/04a/src/engine/gpu/upload.rs:upload-arena-default"
```

`lessons/04a/src/engine/gpu/upload.rs` · type this, one line in `fn drop_uploaded`

```rust
--8<-- "lessons/04a/src/engine/gpu/upload.rs:upload-arena-drop"
```

`lessons/04a/src/engine/gpu/upload.rs` · type this, one line in `fn merge`

```rust
--8<-- "lessons/04a/src/engine/gpu/upload.rs:upload-arena-merge"
```

## Step 20 · src/engine/gpu/upload.rs

A second `impl Upload` block: merge another file's meshes after these, shifting its indices and face ids.

`lessons/04a/src/engine/gpu/upload.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/upload.rs:upload-meshes"
```

## Step 21 · src/engine/gpu/arena.rs

The packed 20-byte arena vertex and the vertex layout that tells the GPU where each field sits.

`lessons/04a/src/engine/gpu/arena.rs` · type this, new file

```rust
--8<-- "lessons/04a/src/engine/gpu/arena.rs:arena-vertex"
```

## Step 22 · src/engine/gpu/arena.rs

Pack a normal into one u32, the inverse of `oct32_decode`.

`lessons/04a/src/engine/gpu/arena.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/arena.rs:arena-octahedral"
```

## Step 23 · src/engine/gpu/arena.rs

The CPU lists one upload fills before the GPU sees them.

`lessons/04a/src/engine/gpu/arena.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/arena.rs:arena-rows"
```

## Step 24 · src/engine/gpu/arena.rs

The arena lane with its buffers, created empty; its `impl` block stays open until step 28.

`lessons/04a/src/engine/gpu/arena.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/arena.rs:arena-lane"
```

## Step 25 · src/engine/gpu/arena.rs

Write rows: append a file, overwrite an object in place, hide rows for undo, empty a triangle.

`lessons/04a/src/engine/gpu/arena.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/arena.rs:arena-write"
```

## Step 26 · src/engine/gpu/arena.rs

The draw methods, ending in `draw_run`: bind three vertex buffers and an index run, then call `draw_indexed`.

`lessons/04a/src/engine/gpu/arena.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/arena.rs:arena-draw"
```

## Step 27 · src/engine/gpu/arena.rs

Draw chosen runs of solid faces for later passes, plain or once per placed instance.

`lessons/04a/src/engine/gpu/arena.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/arena.rs:arena-instanced"
```

## Step 28 · src/engine/gpu/arena.rs

Reset for a new scene, release the memory, count the rows; the `impl` block closes here.

`lessons/04a/src/engine/gpu/arena.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/arena.rs:arena-reset"
```

## Step 29 · src/engine/gpu/arena.rs

The two mask pipelines, and the `Lane` hooks that let the GPU treat the arena like every other lane.

`lessons/04a/src/engine/gpu/arena.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/arena.rs:arena-pipelines"
```

## Step 30 · src/engine/gpu/arena.rs

The arena tests: vertex size, normal packing, colour packing.

`lessons/04a/src/engine/gpu/arena.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/arena.rs:arena-tests"
```

## Step 31 · src/engine/gpu/arena.rs

Where each triangle and vertex came from: the face tag, the source face, the surface sample; lesson 17 uses them.

`lessons/04a/src/engine/gpu/arena.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/arena.rs:arena-faces"
```

## Step 32 · src/engine/pipelines/mod.rs

Every mesh pipeline takes the arena vertex as its vertex buffer 0.

`lessons/04a/src/engine/pipelines/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/pipelines/mod.rs:arena-layout"
```

## Step 33 · src/engine/pipelines/mod.rs

The pipeline compile test.

`lessons/04a/src/engine/pipelines/mod.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/pipelines/mod.rs:compile-tests"
```

## Step 34 · src/engine/gpu/mod.rs

A second `impl Gpu` block: live face counts and the row edits that undo and editing call.

`lessons/04a/src/engine/gpu/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/mod.rs:editable-rows"
```

Run cargo check in lessons/04a/.

## Check

`cargo xtest` in `lessons/04a/` runs the arena and slot tests natively: a vertex packs into 20 bytes, and a normal survives packing within a ten-thousandth of a radian.
