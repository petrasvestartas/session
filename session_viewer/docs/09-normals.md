# 09 · Normals and shading

The sphere shades smoothly and sharp rims retain separate normals under transformed placements.

![Analytic normal or finite fallback at a pole; two shading normals at a C0 crease; the cofactor transform keeps a normal perpendicular under nonuniform scale.](illustrations/normals.svg)

## Step 1 · session_rust/src/nurbssurface_trimmed.rs

Read the kernel's trimmed surface normals; used as is.

??? example "`session_rust/src/nurbssurface_trimmed.rs` · read only · 3335 lines"

    ```rust
    --8<-- "session_rust/src/nurbssurface_trimmed.rs"
    ```
    

Copy each file from the lesson folder to the path shown.

Copy from `lessons/09/` (tooling this checkpoint needs but the course does not teach):

- `lessons/09/assets/pb/view_mixed_teapot.pb` (binary)

Run `cargo check` in `lessons/09/`.

## Step 2 · src/shaders/normals.wgsl

Normals are rotated with the inverse transpose of the model matrix.

`lessons/09/src/shaders/normals.wgsl` · edit · type this

Replaces the line `let transformed = model * normal;` in `lessons/08/src/shaders/normals.wgsl`

```wgsl
--8<-- "lessons/09/src/shaders/normals.wgsl:step-2a"
```

Added above

```wgsl
fn face_normal(model: mat4x4<f32>, normal: vec3<f32>) -> vec3<f32> {
```

```wgsl
--8<-- "lessons/09/src/shaders/normals.wgsl:step-2b"
```

## Step 3 · src/shaders/triangle.wgsl

The mesh shader shades with the rotated normal.

`lessons/09/src/shaders/triangle.wgsl` · edit · type this

Replaces the line `o.normal = vec3<f32>(0.0);` in `lessons/08/src/shaders/triangle.wgsl`

```wgsl
--8<-- "lessons/09/src/shaders/triangle.wgsl:step-3"
```

## Step 4 · src/app/walk/brep_edges.rs

Edge pipes take their facing from the two face normals.

`lessons/09/src/app/walk/brep_edges.rs` · edit · type this

Replaces the `fn mean_normal` lines in `lessons/08/src/app/walk/brep_edges.rs`

```rust
--8<-- "lessons/09/src/app/walk/brep_edges.rs:step-4a"
```

Delete the lines from `let n0 = scaled_normal(mean_normal(a.normal(), b.normal()…` to `};` from `lessons/08/src/app/walk/brep_edges.rs`.

Replaces the line `facing: pack_facing(n0.as_ref(), n1.as_ref()),` in `lessons/08/src/app/walk/brep_edges.rs`

```rust
--8<-- "lessons/09/src/app/walk/brep_edges.rs:step-4d"
```

Copy this part from the lesson folder to the path shown.

`lessons/09/src/app/walk/brep_edges.rs` · edit · copy the file

Added after the last test in `lessons/08/src/app/walk/brep_edges.rs`

```rust
--8<-- "lessons/09/src/app/walk/brep_edges.rs:step-4e"
```

Replaces the lines from `let ep = EdgePen {` to `assert_ne!(p.facing, FACING_UNKNOWN);` in `lessons/08/src/app/walk/brep_edges.rs`

```rust
--8<-- "lessons/09/src/app/walk/brep_edges.rs:step-4f"
```

Replaces the line `assert_eq!(p.facing & 0xffff, p.facing >> 16);` in `lessons/08/src/app/walk/brep_edges.rs`

```rust
--8<-- "lessons/09/src/app/walk/brep_edges.rs:step-4g"
```

Replaces the lines from `let ep = EdgePen {` to `};` in `lessons/08/src/app/walk/brep_edges.rs`

```rust
--8<-- "lessons/09/src/app/walk/brep_edges.rs:step-4h"
```

## Step 5 · src/app/walk/brep.rs

The BRep walk uploads source normals.

`lessons/09/src/app/walk/brep.rs` · edit · type this

Replaces the lines from `let ep = EdgePen {` to `};` in `lessons/08/src/app/walk/brep.rs`

```rust
--8<-- "lessons/09/src/app/walk/brep.rs:step-5a"
```

Copy this part from the lesson folder to the path shown.

`lessons/09/src/app/walk/brep.rs` · edit · copy the file

Added below

```rust
            );
        }
    }
```

```rust
--8<-- "lessons/09/src/app/walk/brep.rs:step-5b"
```

Run `cargo check` in `lessons/09/`.

## Step 6 · src/engine/gpu/mod.rs

The GPU owner uploads normals with the vertices.

`lessons/09/src/engine/gpu/mod.rs` · edit · type this

Replaces the line `self.segments.draw_pipes(&mut pass, &ink);` in `lessons/08/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/09/src/engine/gpu/mod.rs:step-6"
```

## Step 7 · src/lib.rs

The entry point reports the normal count.

`lessons/09/src/lib.rs` · edit · type this

Added below

```rust
        gpu.view.show_grid = false;
```

```rust
--8<-- "lessons/09/src/lib.rs:step-7a"
```

Replaces the line `Ok(serde_json::json!({"stage":8,"objects":self.gpu.object…` in `lessons/08/src/lib.rs`

```rust
--8<-- "lessons/09/src/lib.rs:step-7b"
```

## Step 8 · src/fixture.rs

Copy the test scene: a folded surface with two normals at the crease.
Copy this file from the lesson folder to the path shown.

`lessons/09/src/fixture.rs` · edit · copy the file

Added above

```rust
/// A curved source patch carrying the producer's constrained hole mesh in its supported cache.
```

```rust
--8<-- "lessons/09/src/fixture.rs:step-8a"
```

Replaces the lines from `scene.add(` to `);` in `lessons/08/src/fixture.rs`

```rust
--8<-- "lessons/09/src/fixture.rs:step-8b"
```

## Step 9 · index.html

Copy the page: the status says checkpoint 09.
Copy this file from the lesson folder to the path shown.

`lessons/09/index.html` · edit · copy the file

Replaces the line `<title>Session checkpoint 08</title>` in `lessons/08/index.html`

```html
--8<-- "lessons/09/index.html:step-9a"
```

Replaces the line `<output id="status">Starting checkpoint 08</output>` in `lessons/08/index.html`

```html
--8<-- "lessons/09/index.html:step-9b"
```

Replaces the line `document.getElementById('status').textContent = 'Checkpoi…` in `lessons/08/index.html`

```html
--8<-- "lessons/09/index.html:step-9c"
```

## Check

Run `trunk serve` in `lessons/09/` and open <http://127.0.0.1:8770/>.

Expected: The sphere shades smoothly and sharp rims retain separate normals under transformed placements; status: **1 object**.

![Checkpoint 09 at `?cad=sphere&lit=1`: the sphere's interior shades smoothly, with no facet pattern.](screenshots/09.png)

If it fails:

- A sphere looks faceted: triangle normals replace surface normals.
- Shading crosses a sharp rim: coincident positions incorrectly share a normal.
- A mirrored copy shades differently: the normal transform ignores the inverse transpose.

## What changed

```text
lessons/09/src/
├── app/
│   ├── walk/
│   │   ├── bounds.rs
│   │   ├── brep.rs  ~
│   │   ├── brep_edges.rs  ~
│   │   ├── brep_orient.rs
│   │   ├── curves.rs
│   │   ├── encode.rs
│   │   ├── mesh.rs
│   │   ├── mesh_ink.rs
│   │   ├── mesh_topology.rs
│   │   └── mod.rs
│   ├── knobs.rs
│   ├── mod.rs
│   └── route.rs
├── engine/
│   ├── gpu/
│   │   ├── arena.rs
│   │   ├── backdrop.rs
│   │   ├── buffers.rs
│   │   ├── cloud.rs
│   │   ├── frame.rs
│   │   ├── glyphs.rs
│   │   ├── instance.rs
│   │   ├── lod.rs
│   │   ├── mod.rs  ~
│   │   ├── objects.rs
│   │   ├── segments.rs
│   │   ├── splat.rs
│   │   ├── targets.rs
│   │   ├── text_outline.rs
│   │   ├── upload.rs
│   │   └── view.rs
│   ├── pipelines/
│   │   ├── layouts.rs
│   │   └── mod.rs
│   └── mod.rs
├── shaders/
│   ├── background.wgsl
│   ├── glyph.wgsl
│   ├── grid.wgsl
│   ├── ink_visibility.wgsl
│   ├── normals.wgsl  ~
│   ├── physical.wgsl
│   ├── ribbon.wgsl
│   ├── scene.wgsl
│   ├── sphere.wgsl
│   ├── splat.wgsl
│   ├── splat_resolve.wgsl
│   ├── text_outline.wgsl
│   └── triangle.wgsl  ~
├── camera.rs
├── fixture.rs  ~
└── lib.rs  ~
```

`+` new in this lesson · `~` changed in this lesson

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: `lessons/09/`.

## Next

[10 · Text shaping](10-text-layout.md): fonts, glyph advances and clusters before any pixel is drawn.

## Expected viewer result

Checkpoint 09 at `?cad=sphere&lit=1`: the sphere's interior shades smoothly, with no facet pattern.

[![Full viewer result for 09 normals](screenshots/09.png)](screenshots/09.png)
