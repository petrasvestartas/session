# 03 · Object rows and identity

<!-- locator: off -->

The same triangle drawn twice, each copy with its own placement and colour. The GPU gets one 96-byte row per object; the object's name stays on the CPU.

![The 96-byte Instance layout and the vec3 alignment trap.](illustrations/vertex-layout.svg)

A draw call carries two counters: `vertex_index` walks the three corners, `instance_index` walks the rows. Every vertex reads only the row its `instance_index` names, so a hundred objects are one buffer and one loop of draws.

![A draw call carries two ranges: vertex_index walks the three corners, instance_index walks the object rows, and every invocation reads only the row its instance_index names - so a hundred objects are one call and one buffer.](illustrations/instancing.svg)

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

## Step 1 · `src/engine/gpu/instance.rs`

New file, in a new folder. One row per object: a 4×4 matrix, a colour, flag bits and two scalars. `#[repr(C)]` keeps the fields in this order so the bytes match the WGSL struct in step 4; `bytemuck::Pod` lets the row be cast to bytes. The size assertion fails `cargo check` if the row is not 96 bytes. Only `FLAG_SELECTED` and `placeholder` are used before lesson 12; the other flags are listed once so the file never changes.

<!-- file: 03 session_viewer/src/engine/gpu/instance.rs type lines=1-58 -->

The rest of the file is a test that parses every shader with naga and checks its member offsets against the Rust ones. Copy it from the link; the browser build never compiles it.

<!-- file: 03 session_viewer/src/engine/gpu/instance.rs copy lines=59-245 -->

## Step 2 · `src/engine/gpu/mod.rs` and `src/engine/mod.rs`

Two one-line files. Rust compiles a file only when a `mod` line names it; these put the row into the build.

<!-- file: 03 session_viewer/src/engine/gpu/mod.rs type -->

<!-- file: 03 session_viewer/src/engine/mod.rs type -->

## Step 3 · `src/scene.rs`

New file. A `SourceObject` is what the object *is*: a guid and a revision. Its `row` is how it is drawn. Two objects, same geometry, placed left and right with different tints. `model[12]` is the x translation of a column-major matrix.

<!-- file: 03 session_viewer/src/scene.rs type -->

<!-- check: 03 -->

## Step 4 · `src/shaders/first.wgsl`

Replace the whole file. The shader declares the same 96 bytes as `Instance`, reads the rows from binding 1 of group 0, and takes `instance_index` as a second argument. The position is now `mvp × model × point`, and the colour comes from the row.

```text
Rust `Instance`            offset   WGSL `struct Instance`
model: [f32; 16]              0     model: mat4x4<f32>
color: [f32; 4]              64     color: vec4<f32>
flags: u32                   80     flags: u32
_pad0: f32                   84     thickness: f32
spacing: f32                 88     spacing: f32
_pad: u32                    92     pad: u32
size                         96     array stride
```

<!-- file: 03 session_viewer/src/shaders/first.wgsl type -->

## Step 5 · `src/lib.rs`

Six edits. Declare the two new modules; add the objects to the struct; give the bind group layout a second entry, a read-only storage buffer for the vertex stage; upload the two rows into that buffer and put it at binding 1; keep the objects; draw once per row with `draw(0..3, row..row + 1)`, which is what sets `instance_index`.

<!-- file: 03 session_viewer/src/lib.rs type -->

## Step 6 · `index.html`

Two edits: the checkpoint number in the title and the status line.

<!-- file: 03 session_viewer/index.html copy -->

## Check

<!-- checkpoint: 03 -->

Expected: two triangles, orange on the left, blue on the right, and a status line **Checkpoint 03 · 2 objects**. Orbit and zoom: they keep their relative placement.

![Checkpoint 03: one triangle geometry drawn twice through two object rows, each with its own placement and tint.](screenshots/03.png)

If it fails:

- The first triangle is right and the second is garbage: the row stride is wrong. Compare `Instance` with the table in step 4.
- Only one triangle: the draw loop still says `draw(0..3, 0..1)`, so `instance_index` never reaches 1.
- A validation error naming binding 1: the layout entry, the bind group entry and the `@binding(1)` line do not agree.

## What changed

<!-- tree: 03 session_viewer/src -->

Data flow: `SourceObject.row` → storage buffer → `instances[instance_index]` → placed, tinted vertex. Every file at this point: [source at checkpoint 03](../lessons/03/index.md). The maintained viewer keeps `instance.rs` unchanged and the scene in `src/app/scene.rs`.

## Next

[04a · Meshes on the GPU](04a-meshes.md): vertex and index buffers, the mesh arena, the first real drawing module.

## Expected viewer result

Checkpoint 03: one triangle geometry drawn twice through two object rows, each with its own placement and tint.

[![Full viewer result for 03 identity](screenshots/03.png)](screenshots/03.png)
