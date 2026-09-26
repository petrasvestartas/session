# 03 · Object rows and identity

The same triangle drawn twice, each with its own placement and colour.

![The 96-byte Instance layout and the vec3 alignment trap.](illustrations/vertex-layout.svg)

`vertex_index` walks the corners, `instance_index` walks the rows.

![A draw call carries two ranges: vertex_index walks the three corners, instance_index walks the object rows, and every invocation reads only the row its instance_index names - so a hundred objects are one call and one buffer.](illustrations/instancing.svg)

## Step 1 · `src/engine/gpu/instance.rs`

New file: one 96-byte row per object, matrix, colour, flags.

`lessons/03/src/engine/gpu/instance.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/03/src/engine/gpu/instance.rs:step-1a"
```

The rest of the file is a test; copy it.

`lessons/03/src/engine/gpu/instance.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/03/src/engine/gpu/instance.rs:step-1b"
```

## Step 2 · `src/engine/gpu/mod.rs` and `src/engine/mod.rs`

Two one-line files that put the new folder into the build.

`lessons/03/src/engine/gpu/mod.rs` · 1 lines · type this, new file

```rust
--8<-- "lessons/03/src/engine/gpu/mod.rs"
```

`lessons/03/src/engine/mod.rs` · 1 lines · type this, new file

```rust
--8<-- "lessons/03/src/engine/mod.rs"
```

## Step 3 · `src/scene.rs`

New file: two objects that share one triangle, each with its own row: placement and colour.

`lessons/03/src/scene.rs` · type this, new file

```rust
--8<-- "lessons/03/src/scene.rs"
```

Run `cargo check` in `lessons/03/`.

## Step 4 · `src/shaders/first.wgsl`

Replace the shader: it reads its row from binding 1 and applies the model matrix.

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

`lessons/03/src/shaders/first.wgsl` · edit · type this

Replaces the `fn vs_main` lines in `lessons/02/src/shaders/first.wgsl`

```wgsl
--8<-- "lessons/03/src/shaders/first.wgsl:step-4"
```

## Step 5 · `src/lib.rs`

Six edits: new modules, the rows uploaded to a storage buffer, one draw per row.

`lessons/03/src/lib.rs` · edit · type this

Added below

```rust
pub mod camera;
```

```rust
--8<-- "lessons/03/src/lib.rs:step-5a"
```

Added below

```rust
    camera: camera::Camera,
```

```rust
--8<-- "lessons/03/src/lib.rs:step-5b"
```

Replaces the lines from `entries: &[wgpu::BindGroupLayoutEntry {` to `}],` in `lessons/02/src/lib.rs`

```rust
--8<-- "lessons/03/src/lib.rs:step-5c"
```

Added after the camera setup, before `Ok(Self {`, in `lessons/03/src/lib.rs`

```rust
--8<-- "lessons/03/src/lib.rs:step-5d"
```

Added below

```rust
            camera,
```

```rust
--8<-- "lessons/03/src/lib.rs:step-5e"
```

Replaces the lines from `pass.draw(0..3, 0..1);` to `Ok(serde_json::json!({"stage": 2, "objects": 1, "width":w…` in `lessons/02/src/lib.rs`

```rust
--8<-- "lessons/03/src/lib.rs:step-5f"
```

## Step 6 · `index.html`

Three edits: the checkpoint number in the title, the starting status and the status text.

`lessons/03/index.html` · edit · copy the file

Replaces the line `<title>Session checkpoint 02</title>` in `lessons/02/index.html`

```html
--8<-- "lessons/03/index.html:step-6a"
```

Replaces the line `<output id="status">Starting checkpoint 02</output>` in `lessons/02/index.html`

```html
--8<-- "lessons/03/index.html:step-6b"
```

Replaces the line `document.getElementById('status').textContent = 'Checkpoi…` in `lessons/02/index.html`

```html
--8<-- "lessons/03/index.html:step-6c"
```

## Check

Run `trunk serve` in `lessons/03/` and open <http://127.0.0.1:8770/>.

Expected: two triangles, orange left, blue right, status **Checkpoint 03 · 2 objects**.

![Checkpoint 03: one triangle geometry drawn twice through two object rows, each with its own placement and tint.](screenshots/03.png)

If it fails:

- The first triangle is right and the second is garbage: the row stride is wrong. Compare `Instance` with the table in step 4.
- Only one triangle: the draw loop still says `draw(0..3, 0..1)`, so `instance_index` never reaches 1.
- A validation error naming binding 1: the layout entry, the bind group entry and the `@binding(1)` line do not agree.

## What changed

```text
lessons/03/src/
├── engine/
│   ├── gpu/
│   │   ├── instance.rs  +
│   │   └── mod.rs  +
│   └── mod.rs  +
├── shaders/
│   └── first.wgsl  ~
├── camera.rs
├── lib.rs  ~
└── scene.rs  +
```

`+` new in this lesson · `~` changed in this lesson

Every file at this point: `lessons/03/`.

## Next

[04a · Meshes on the GPU](04a-meshes.md)

## Expected viewer result

Checkpoint 03: one triangle geometry drawn twice through two object rows, each with its own placement and tint.

[![Full viewer result for 03 identity](screenshots/03.png)](screenshots/03.png)
