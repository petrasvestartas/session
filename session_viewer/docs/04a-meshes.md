# 04a · Meshes on the GPU

A blue triangle draws from mesh buffers while the camera still orbits and zooms.

![One growable arena holds every mesh's vertices and a parallel table gives every vertex its object row; a mesh is a range of indices, and a draw binds both vertex buffers, binds one index run and calls draw_indexed.](illustrations/arena.svg)

## Step 1 · src/engine/gpu/buffers.rs

New file: a GPU buffer that grows when full.

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

New file: the shader prelude every scene shader includes.

`lessons/04a/src/shaders/scene.wgsl` · 57 lines · type this, new file

```wgsl
--8<-- "lessons/04a/src/shaders/scene.wgsl"
```

## Step 5 · src/engine/gpu/instance.rs

Move the object row into its own file.

`lessons/04a/src/engine/gpu/instance.rs` · edit · type this

Replaces the `struct Instance` lines in `lessons/03/src/engine/gpu/instance.rs`

```rust
--8<-- "lessons/04a/src/engine/gpu/instance.rs:step-5"
```

## Step 6 · src/shaders/normals.wgsl

New file: shader helpers that rotate normals correctly.

`lessons/04a/src/shaders/normals.wgsl` · 26 lines · type this, new file

```wgsl
--8<-- "lessons/04a/src/shaders/normals.wgsl"
```

Run `cargo check` in `lessons/04a/`.

## Step 7 · src/engine/gpu/targets.rs

New file: the colour and depth textures of one frame.

`lessons/04a/src/engine/gpu/targets.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/04a/src/engine/gpu/targets.rs:step-7a"
```

`lessons/04a/src/engine/gpu/targets.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/targets.rs:step-7b"
```

`lessons/04a/src/engine/gpu/targets.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/targets.rs:step-7c"
```

## Step 8 · src/engine/gpu/frame.rs

New file: the per-frame uniform with the camera and pen scale.

`lessons/04a/src/engine/gpu/frame.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/04a/src/engine/gpu/frame.rs:step-8a"
```

`lessons/04a/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/frame.rs:step-8b"
```

`lessons/04a/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/frame.rs:step-8c"
```

`lessons/04a/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/frame.rs:step-8d"
```

`lessons/04a/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/frame.rs:step-8e"
```

`lessons/04a/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/frame.rs:step-8f"
```

`lessons/04a/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/frame.rs:step-8g"
```

`lessons/04a/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/frame.rs:step-8h"
```

`lessons/04a/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/frame.rs:step-8i"
```

Copy this part from the lesson folder to the path shown.

`lessons/04a/src/engine/gpu/frame.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/frame.rs:step-8j"
```

## Step 9 · src/engine/gpu/view.rs

New file: the anchored view matrix as f32.
Copy this file from the lesson folder to the path shown.

`lessons/04a/src/engine/gpu/view.rs` · 74 lines · copy the file, new file

```rust
--8<-- "lessons/04a/src/engine/gpu/view.rs"
```

## Step 10 · src/app/mod.rs

New file: the app module, one line for now.

`lessons/04a/src/app/mod.rs` · 1 lines · type this, new file

```rust
--8<-- "lessons/04a/src/app/mod.rs"
```

## Step 11 · src/app/route.rs

New file: read one value from the page URL.

`lessons/04a/src/app/route.rs` · 14 lines · type this, new file

```rust
--8<-- "lessons/04a/src/app/route.rs"
```

## Step 12 · src/engine/gpu/objects.rs

New file: the object table on the GPU.

`lessons/04a/src/engine/gpu/objects.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/04a/src/engine/gpu/objects.rs:step-12a"
```

`lessons/04a/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/objects.rs:step-12b"
```

`lessons/04a/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/objects.rs:step-12c"
```

`lessons/04a/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/objects.rs:step-12d"
```

`lessons/04a/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/objects.rs:step-12e"
```

`lessons/04a/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/objects.rs:step-12f"
```

`lessons/04a/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/objects.rs:step-12g"
```

`lessons/04a/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/objects.rs:step-12h"
```

Copy this part from the lesson folder to the path shown.

`lessons/04a/src/engine/gpu/objects.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/objects.rs:step-12i"
```

Run `cargo check` in `lessons/04a/`.

## Step 13 · src/shaders/triangle.wgsl

New file: the mesh shader, position and flat colour per face.

`lessons/04a/src/shaders/triangle.wgsl` · type this, new file, start with these lines

```wgsl
--8<-- "lessons/04a/src/shaders/triangle.wgsl:step-13a"
```

`lessons/04a/src/shaders/triangle.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04a/src/shaders/triangle.wgsl:step-13b"
```

`lessons/04a/src/shaders/triangle.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04a/src/shaders/triangle.wgsl:step-13c"
```

`lessons/04a/src/shaders/triangle.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04a/src/shaders/triangle.wgsl:step-13d"
```

## Step 14 · src/engine/gpu/arena.rs

New file: the mesh arena, vertices and indices of every mesh.

`lessons/04a/src/engine/gpu/arena.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/04a/src/engine/gpu/arena.rs:step-14a"
```

`lessons/04a/src/engine/gpu/arena.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/arena.rs:step-14b"
```

`lessons/04a/src/engine/gpu/arena.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/arena.rs:step-14c"
```

`lessons/04a/src/engine/gpu/arena.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/arena.rs:step-14d"
```

## Step 15 · src/shaders/text_outline.wgsl

New file: the outline text shader.
Copy this file from the lesson folder to the path shown.

`lessons/04a/src/shaders/text_outline.wgsl` · 44 lines · copy the file, new file

```wgsl
--8<-- "lessons/04a/src/shaders/text_outline.wgsl"
```

## Step 16 · src/engine/gpu/text_outline.rs

New file: outline text drawn as vector glyphs.

`lessons/04a/src/engine/gpu/text_outline.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/04a/src/engine/gpu/text_outline.rs:step-16a"
```

`lessons/04a/src/engine/gpu/text_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/text_outline.rs:step-16b"
```

`lessons/04a/src/engine/gpu/text_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/text_outline.rs:step-16c"
```

Copy this part from the lesson folder to the path shown.

`lessons/04a/src/engine/gpu/text_outline.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/text_outline.rs:step-16d"
```

## Step 17 · src/engine/gpu/upload.rs

New file: everything one scene uploads, collected before the GPU sees it.

`lessons/04a/src/engine/gpu/upload.rs` · 37 lines · type this, new file

```rust
--8<-- "lessons/04a/src/engine/gpu/upload.rs"
```

## Step 18 · src/fixture.rs

New file: a small test scene built in code.
Copy this file from the lesson folder to the path shown.

`lessons/04a/src/fixture.rs` · 35 lines · copy the file, new file

```rust
--8<-- "lessons/04a/src/fixture.rs"
```

## Step 19 · src/engine/gpu/mod.rs

New file: the GPU owner, one field per drawing lane.

`lessons/04a/src/engine/gpu/mod.rs` · type this, replace the whole file, start with these lines

```rust
--8<-- "lessons/04a/src/engine/gpu/mod.rs:step-19a"
```

`lessons/04a/src/engine/gpu/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/mod.rs:step-19b"
```

`lessons/04a/src/engine/gpu/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/mod.rs:step-19c"
```

`lessons/04a/src/engine/gpu/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/engine/gpu/mod.rs:step-19d"
```

## Step 20 · src/engine/mod.rs

Put the gpu and pipelines folders into the build.

`lessons/04a/src/engine/mod.rs` · edit · type this

Added below

```rust
pub mod gpu;
```

```rust
--8<-- "lessons/04a/src/engine/mod.rs:step-20"
```

## Step 21 · src/lib.rs

Replace the entry point: it now owns a camera and the GPU.

`lessons/04a/src/lib.rs` · type this, replace the whole file, start with these lines

```rust
--8<-- "lessons/04a/src/lib.rs:step-21a"
```

`lessons/04a/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04a/src/lib.rs:step-21b"
```

## Step 22 · src/scene.rs

Delete the old scene file.

Delete `src/scene.rs` (it exists in `lessons/03/`, not in `lessons/04a/`).

## Step 23 · src/shaders/first.wgsl

Delete the first triangle shader.

Delete `src/shaders/first.wgsl` (it exists in `lessons/03/`, not in `lessons/04a/`).

## Step 24 · index.html

Copy the page: the status now reports mesh vertices.
Copy this file from the lesson folder to the path shown.

`lessons/04a/index.html` · edit · copy the file

Replaces the line `<title>Session checkpoint 03</title>` in `lessons/03/index.html`

```html
--8<-- "lessons/04a/index.html:step-24a"
```

Replaces the line `<output id="status">Starting checkpoint 03</output>` in `lessons/03/index.html`

```html
--8<-- "lessons/04a/index.html:step-24b"
```

Replaces the line `document.getElementById('status').textContent = 'Checkpoi…` in `lessons/03/index.html`

```html
--8<-- "lessons/04a/index.html:step-24c"
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
