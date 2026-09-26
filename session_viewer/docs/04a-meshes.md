# 04a · Meshes on the GPU

A blue triangle draws from mesh buffers while the camera still orbits and zooms.

![One growable arena holds every mesh's vertices and a parallel table gives every vertex its object row; a mesh is a range of indices, and a draw binds both vertex buffers, binds one index run and calls draw_indexed.](illustrations/arena.svg)

## Step 1 · src/engine/gpu/buffers.rs

New file: buffer usage flags, a GPU buffer that grows by half when full, and small buffer helpers.

`lessons/04a/src/engine/gpu/buffers.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/04a/src/engine/gpu/buffers.rs:step-1a"
```

`lessons/04a/src/engine/gpu/buffers.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/buffers.rs:step-1b"
```

`lessons/04a/src/engine/gpu/buffers.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/buffers.rs:step-1c"
```

`lessons/04a/src/engine/gpu/buffers.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/buffers.rs:step-1d"
```

`lessons/04a/src/engine/gpu/buffers.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/buffers.rs:step-1e"
```

## Step 2 · src/engine/pipelines/layouts.rs

New file: the bind group layouts every pipeline shares.

`lessons/04a/src/engine/pipelines/layouts.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/04a/src/engine/pipelines/layouts.rs:step-2a"
```

`lessons/04a/src/engine/pipelines/layouts.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/pipelines/layouts.rs:step-2b"
```

## Step 3 · src/engine/pipelines/mod.rs

New file: one builder for every render pipeline.

`lessons/04a/src/engine/pipelines/mod.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/04a/src/engine/pipelines/mod.rs:step-3a"
```

`lessons/04a/src/engine/pipelines/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/pipelines/mod.rs:step-3b"
```

`lessons/04a/src/engine/pipelines/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/pipelines/mod.rs:step-3c"
```

`lessons/04a/src/engine/pipelines/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/pipelines/mod.rs:step-3d"
```

`lessons/04a/src/engine/pipelines/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/pipelines/mod.rs:step-3e"
```

`lessons/04a/src/engine/pipelines/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/pipelines/mod.rs:step-3f"
```

## Step 4 · src/shaders/scene.wgsl

New file: the bindings, structs and helpers every scene shader starts with.

`lessons/04a/src/shaders/scene.wgsl` · type this, new file

```wgsl
--8<-- "lessons/04a/src/shaders/scene.wgsl"
```

## Step 5 · src/shaders/normals.wgsl

New file: two helpers that turn a normal with its object, and one that unpacks a 2-byte normal.

`lessons/04a/src/shaders/normals.wgsl` · type this, new file

```wgsl
--8<-- "lessons/04a/src/shaders/normals.wgsl"
```

Run `cargo check` in `lessons/04a/`.

## Step 6 · src/engine/gpu/targets.rs

New file: the colour and depth textures of one frame.

`lessons/04a/src/engine/gpu/targets.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/04a/src/engine/gpu/targets.rs:step-6a"
```

`lessons/04a/src/engine/gpu/targets.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/targets.rs:step-6b"
```

`lessons/04a/src/engine/gpu/targets.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/targets.rs:step-6c"
```

## Step 7 · src/engine/gpu/frame.rs

New file: what the app hands the renderer each frame, the uniforms it writes, and the three bind groups every draw sets first.

`lessons/04a/src/engine/gpu/frame.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/04a/src/engine/gpu/frame.rs:step-7a"
```

`lessons/04a/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/frame.rs:step-7b"
```

`lessons/04a/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/frame.rs:step-7c"
```

`lessons/04a/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/frame.rs:step-7d"
```

`lessons/04a/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/frame.rs:step-7e"
```

`lessons/04a/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/frame.rs:step-7f"
```

`lessons/04a/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/frame.rs:step-7g"
```

`lessons/04a/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/frame.rs:step-7h"
```

`lessons/04a/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/frame.rs:step-7i"
```

Copy this part from the lesson folder to the path shown.

`lessons/04a/src/engine/gpu/frame.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/frame.rs:step-7j"
```

## Step 8 · src/engine/gpu/view.rs

New file: the display settings, each read once from the page URL or an env variable.
Copy this file from the lesson folder to the path shown.

`lessons/04a/src/engine/gpu/view.rs` · copy the file, new file

```rust
--8<-- "lessons/04a/src/engine/gpu/view.rs"
```

## Step 9 · src/app/mod.rs

New file: the app module, one line for now.

`lessons/04a/src/app/mod.rs` · 1 lines · type this, new file

```rust
--8<-- "lessons/04a/src/app/mod.rs"
```

## Step 10 · src/app/route.rs

New file: read one value from the page URL.

`lessons/04a/src/app/route.rs` · 14 lines · type this, new file

```rust
--8<-- "lessons/04a/src/app/route.rs"
```

## Step 11 · src/engine/gpu/objects.rs

New file: the object table on the GPU.

`lessons/04a/src/engine/gpu/objects.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/04a/src/engine/gpu/objects.rs:step-11a"
```

`lessons/04a/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/objects.rs:step-11b"
```

`lessons/04a/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/objects.rs:step-11c"
```

`lessons/04a/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/objects.rs:step-11d"
```

`lessons/04a/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/objects.rs:step-11e"
```

`lessons/04a/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/objects.rs:step-11f"
```

`lessons/04a/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/objects.rs:step-11g"
```

`lessons/04a/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/objects.rs:step-11h"
```

Copy this part from the lesson folder to the path shown.

`lessons/04a/src/engine/gpu/objects.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/objects.rs:step-11i"
```

Run `cargo check` in `lessons/04a/`.

## Step 12 · src/shaders/triangle.wgsl

New file: the mesh shader, position and flat colour per face.

`lessons/04a/src/shaders/triangle.wgsl` · type this, new file, start with these lines

```wgsl
--8<-- "lessons/04a/src/shaders/triangle.wgsl:step-12a"
```

`lessons/04a/src/shaders/triangle.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04a/src/shaders/triangle.wgsl:step-12b"
```

`lessons/04a/src/shaders/triangle.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04a/src/shaders/triangle.wgsl:step-12c"
```

`lessons/04a/src/shaders/triangle.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04a/src/shaders/triangle.wgsl:step-12d"
```

## Step 13 · src/engine/gpu/arena.rs

New file: the mesh arena, vertices and indices of every mesh.

`lessons/04a/src/engine/gpu/arena.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/04a/src/engine/gpu/arena.rs:step-13a"
```

`lessons/04a/src/engine/gpu/arena.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/arena.rs:step-13b"
```

`lessons/04a/src/engine/gpu/arena.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/arena.rs:step-13c"
```

`lessons/04a/src/engine/gpu/arena.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/arena.rs:step-13d"
```

## Step 14 · src/shaders/text_outline.wgsl

New file: the shader for sheet fills and lettering: placed, flat-coloured, yellow when selected.
Copy this file from the lesson folder to the path shown.

`lessons/04a/src/shaders/text_outline.wgsl` · copy the file, new file

```wgsl
--8<-- "lessons/04a/src/shaders/text_outline.wgsl"
```

## Step 15 · src/engine/gpu/text_outline.rs

New file: the lane that draws sheet fills and lettering, in colour and as pick ids.

`lessons/04a/src/engine/gpu/text_outline.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/04a/src/engine/gpu/text_outline.rs:step-15a"
```

`lessons/04a/src/engine/gpu/text_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/text_outline.rs:step-15b"
```

`lessons/04a/src/engine/gpu/text_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/text_outline.rs:step-15c"
```

## Step 16 · src/engine/gpu/upload.rs

New file: everything one scene uploads, collected before the GPU sees it.

`lessons/04a/src/engine/gpu/upload.rs` · 37 lines · type this, new file

```rust
--8<-- "lessons/04a/src/engine/gpu/upload.rs"
```

## Step 17 · src/fixture.rs

New file: a small test scene built in code.
Copy this file from the lesson folder to the path shown.

`lessons/04a/src/fixture.rs` · 35 lines · copy the file, new file

```rust
--8<-- "lessons/04a/src/fixture.rs"
```

## Step 18 · src/engine/gpu/mod.rs

New file: the GPU owner, one field per drawing lane.

`lessons/04a/src/engine/gpu/mod.rs` · type this, replace the whole file, start with these lines

```rust
--8<-- "lessons/04a/src/engine/gpu/mod.rs:step-18a"
```

`lessons/04a/src/engine/gpu/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/mod.rs:step-18b"
```

`lessons/04a/src/engine/gpu/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/mod.rs:step-18c"
```

`lessons/04a/src/engine/gpu/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/mod.rs:step-18d"
```

## Step 19 · src/engine/mod.rs

Put the gpu and pipelines folders into the build.

`lessons/04a/src/engine/mod.rs` · edit · type this

Added below

```rust
pub mod gpu;
```

```rust
--8<-- "lessons/04a/src/engine/mod.rs:step-19"
```

## Step 20 · src/lib.rs

Replace the entry point: it now owns a camera and the GPU.

`lessons/04a/src/lib.rs` · type this, replace the whole file, start with these lines

```rust
--8<-- "lessons/04a/src/lib.rs:step-20a"
```

`lessons/04a/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/lib.rs:step-20b"
```

## Step 21 · src/scene.rs

Delete the old scene file.

Delete `src/scene.rs` (it exists in `lessons/03/`, not in `lessons/04a/`).

## Step 22 · src/shaders/first.wgsl

Delete the first triangle shader.

Delete `src/shaders/first.wgsl` (it exists in `lessons/03/`, not in `lessons/04a/`).

## Step 23 · index.html

Copy the page: the status now reports mesh vertices.
Copy this file from the lesson folder to the path shown.

`lessons/04a/index.html` · edit · copy the file

Replaces the line `<title>Session checkpoint 03</title>` in `lessons/03/index.html`

```html
--8<-- "lessons/04a/index.html:step-23a"
```

Replaces the line `<output id="status">Starting checkpoint 03</output>` in `lessons/03/index.html`

```html
--8<-- "lessons/04a/index.html:step-23b"
```

Replaces the line `document.getElementById('status').textContent = 'Checkpoi…` in `lessons/03/index.html`

```html
--8<-- "lessons/04a/index.html:step-23c"
```

Run `cargo check` in `lessons/04a/`.

## Check

Run `trunk serve` in `lessons/04a/` and open <http://127.0.0.1:8770/>.

Expected: A blue triangle draws from mesh buffers while the camera still orbits and zooms; status: **Checkpoint 04a · 1 objects**.

![Checkpoint 04a: the first mesh drawn from arena buffers through the object table.](screenshots/04a.png)

If it fails:

- The canvas stays empty: the surface is not configured or the arena has no rows.
- Geometry is scrambled: the vertex stride or object row layout differs from the shader.

## What changed

```text
lessons/04a/src/
├── app/
│   ├── mod.rs  +
│   └── route.rs  +
├── engine/
│   ├── gpu/
│   │   ├── arena.rs  +
│   │   ├── buffers.rs  +
│   │   ├── frame.rs  +
│   │   ├── instance.rs  ~
│   │   ├── mod.rs  ~
│   │   ├── objects.rs  +
│   │   ├── targets.rs  +
│   │   ├── text_outline.rs  +
│   │   ├── upload.rs  +
│   │   └── view.rs  +
│   ├── pipelines/
│   │   ├── layouts.rs  +
│   │   └── mod.rs  +
│   └── mod.rs  ~
├── shaders/
│   ├── normals.wgsl  +
│   ├── scene.wgsl  +
│   ├── text_outline.wgsl  +
│   └── triangle.wgsl  +
├── camera.rs
├── fixture.rs  +
└── lib.rs  ~
```

`+` new in this lesson · `~` changed in this lesson

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: `lessons/04a/`.

## Next
[04b · Strokes](04b-strokes.md): the segment drawing module, `ribbon.wgsl`, and the shared ink visibility rule.

## Expected viewer result

Checkpoint 04a: the first mesh drawn from arena buffers through the object table.

[![Full viewer result for 04a meshes](screenshots/04a.png)](screenshots/04a.png)
