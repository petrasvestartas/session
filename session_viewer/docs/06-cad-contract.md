# 06 · CAD face rules

A shaded planar face shows four black boundaries with retained source identities.

![One face, three representations: BRep source in f64, kernel mesh with u,v, normals and boundary tags, viewer rows in f32 that keep the face and edge identities.](illustrations/cad-contract.svg)

Copy each file from the lesson folder to the path shown.

## Step 1 · session_rust/src/remesh_nurbssurface_grid.rs

Read the kernel mesher; the lesson uses it, never changes it.

??? example "`session_rust/src/remesh_nurbssurface_grid.rs` · read only"

    [Open the full listing](kernel/remesh_nurbssurface_grid.md)
    

Run `cargo check` in `lessons/06/`.

## Step 2 · src/app/walk/encode.rs

New file: pack pen width, colour and face normals into the few bytes the shaders read.

`lessons/06/src/app/walk/encode.rs` · 78 lines · type this, new file

```rust
--8<-- "lessons/06/src/app/walk/encode.rs"
```

## Step 3 · src/app/walk/mod.rs

New file: the walk that turns each source object into GPU rows.

`lessons/06/src/app/walk/mod.rs` · 38 lines · type this, new file

```rust
--8<-- "lessons/06/src/app/walk/mod.rs"
```

## Step 4 · src/app/walk/bounds.rs

New file: bounds collected from the rows one document adds.

`lessons/06/src/app/walk/bounds.rs` · 74 lines · type this, new file

```rust
--8<-- "lessons/06/src/app/walk/bounds.rs"
```

## Step 5 · src/app/walk/mesh_topology.rs

New file: find each mesh edge once, with the faces beside it and one normal per face.

`lessons/06/src/app/walk/mesh_topology.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/06/src/app/walk/mesh_topology.rs:step-5a"
```

`lessons/06/src/app/walk/mesh_topology.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh_topology.rs:step-5b"
```

`lessons/06/src/app/walk/mesh_topology.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh_topology.rs:step-5c"
```

## Step 6 · src/app/walk/mesh_ink.rs

New file: turn mesh edges into pipes and vertices into dots, skipping flat diagonals and smooth seams.

`lessons/06/src/app/walk/mesh_ink.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/06/src/app/walk/mesh_ink.rs:step-6a"
```

`lessons/06/src/app/walk/mesh_ink.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh_ink.rs:step-6b"
```

`lessons/06/src/app/walk/mesh_ink.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh_ink.rs:step-6c"
```

`lessons/06/src/app/walk/mesh_ink.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh_ink.rs:step-6d"
```

`lessons/06/src/app/walk/mesh_ink.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh_ink.rs:step-6e"
```

`lessons/06/src/app/walk/mesh_ink.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh_ink.rs:step-6f"
```

Copy this part from the lesson folder to the path shown.

`lessons/06/src/app/walk/mesh_ink.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh_ink.rs:step-6g"
```

## Step 7 · src/app/walk/mesh.rs

New file: walk one mesh into arena triangles, then hand its edges and dots to the ink.

`lessons/06/src/app/walk/mesh.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/06/src/app/walk/mesh.rs:step-7a"
```

`lessons/06/src/app/walk/mesh.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh.rs:step-7b"
```

Copy this part from the lesson folder to the path shown.

`lessons/06/src/app/walk/mesh.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh.rs:step-7c"
```

`lessons/06/src/app/walk/mesh.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh.rs:step-7d"
```

## Step 8 · src/app/walk/curves.rs

New file: curves sampled into connected strokes.

`lessons/06/src/app/walk/curves.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/06/src/app/walk/curves.rs:step-8a"
```

`lessons/06/src/app/walk/curves.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/curves.rs:step-8b"
```

`lessons/06/src/app/walk/curves.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/curves.rs:step-8c"
```

## Step 9 · src/app/walk/brep_edges.rs

New file: boundary chains shared between faces.

`lessons/06/src/app/walk/brep_edges.rs` · 18 lines · type this, new file

```rust
--8<-- "lessons/06/src/app/walk/brep_edges.rs"
```

## Step 10 · src/app/walk/brep.rs

New file: the BRep walk, shaded faces and their edges.

`lessons/06/src/app/walk/brep.rs` · 99 lines · type this, new file

```rust
--8<-- "lessons/06/src/app/walk/brep.rs"
```

## Step 11 · src/app/knobs.rs

Copy the file: debug switches read once from environment variables such as VIEWER_PROFILE.

`lessons/06/src/app/knobs.rs` · 54 lines · copy the file, new file

```rust
--8<-- "lessons/06/src/app/knobs.rs"
```

## Step 12 · src/app/mod.rs

The app module declares the walk.

`lessons/06/src/app/mod.rs` · edit · type this

Replaces the line `pub mod route;` in `lessons/05/src/app/mod.rs`

```rust
--8<-- "lessons/06/src/app/mod.rs:step-12"
```

Run `cargo check` in `lessons/06/`.

## Step 13 · src/fixture.rs

Copy the test scene: CAD surfaces from the kernel.
Copy this file from the lesson folder to the path shown.

`lessons/06/src/fixture.rs` · edit · copy the file

Replaces the `fn scene` lines in `lessons/05/src/fixture.rs`

```rust
--8<-- "lessons/06/src/fixture.rs:step-13"
```

## Step 14 · src/lib.rs

The entry point runs the walk instead of the fixture.

`lessons/06/src/lib.rs` · edit · type this

Replaces the lines from `let mut upload = fixture::scene();` to `camera.unit = camera::Unit::Meters;` in `lessons/05/src/lib.rs`

```rust
--8<-- "lessons/06/src/lib.rs:step-14a"
```

Added below

```rust
            scale: 1.0,
```

```rust
--8<-- "lessons/06/src/lib.rs:step-14b"
```

Replaces the line `Ok(serde_json::json!({"stage":5,"objects":self.gpu.object…` in `lessons/05/src/lib.rs`

```rust
--8<-- "lessons/06/src/lib.rs:step-14c"
```

## Step 15 · src/shaders/triangle.wgsl

The mesh shader reads the packed facing.

`lessons/06/src/shaders/triangle.wgsl` · edit · type this

Replaces the line `o.normal = face_normal(inst.model, in.normal);` in `lessons/05/src/shaders/triangle.wgsl`

```wgsl
--8<-- "lessons/06/src/shaders/triangle.wgsl:step-15"
```

## Step 16 · index.html

Copy the page: the status says checkpoint 06.
Copy this file from the lesson folder to the path shown.

`lessons/06/index.html` · edit · copy the file

Replaces the line `<title>Session checkpoint 05</title>` in `lessons/05/index.html`

```html
--8<-- "lessons/06/index.html:step-16a"
```

Replaces the line `<output id="status">Starting checkpoint 05</output>` in `lessons/05/index.html`

```html
--8<-- "lessons/06/index.html:step-16b"
```

Replaces the line `document.getElementById('status').textContent = 'Checkpoi…` in `lessons/05/index.html`

```html
--8<-- "lessons/06/index.html:step-16c"
```

## Check

Run `trunk serve` in `lessons/06/` and open <http://127.0.0.1:8770/>.

Expected: A shaded planar face shows four black boundaries with retained source identities; status: **1 object**.

![Checkpoint 06: the CAD fixture shaded flat, boundaries drawn as pipes from the face mesh nodes.](screenshots/06.png)

If it fails:

- The face is missing: the producer, upload or arena draw range is empty.
- Boundaries float off the face: the two f64 to f32 conversions differ.

## What changed

```text
lessons/06/src/app/
├── walk/
│   ├── bounds.rs  +
│   ├── brep.rs  +
│   ├── brep_edges.rs  +
│   ├── curves.rs  +
│   ├── encode.rs  +
│   ├── mesh.rs  +
│   ├── mesh_ink.rs  +
│   ├── mesh_topology.rs  +
│   └── mod.rs  +
├── knobs.rs  +
├── mod.rs  ~
└── route.rs
```

`+` new in this lesson · `~` changed in this lesson

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: `lessons/06/`.

## Next

[07 · Shared boundaries](07-boundaries.md)

## Expected viewer result

Checkpoint 06: the CAD fixture shaded flat, boundaries drawn as pipes from the face mesh nodes.

[![Full viewer result for 06 cad contract](screenshots/06.png)](screenshots/06.png)
