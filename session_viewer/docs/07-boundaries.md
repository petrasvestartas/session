# 07 · Shared boundaries

A cylinder and a block with a hole show continuous source boundary curves.

![Before: face A, face B and the ink each chord the same edge differently. After: one canonical chain constrains both meshes and the ink is drawn from those nodes.](illustrations/shared-boundary.svg)

Copy each file from the lesson folder to the path shown.

## Step 1 · session_rust/src/nurbssurface_trimmed.rs

Read the kernel's trimmed surface; the lesson uses it as is.

??? example "`session_rust/src/nurbssurface_trimmed.rs` · read only · 3335 lines"

    ```rust
    --8<-- "session_rust/src/nurbssurface_trimmed.rs"
    ```
    

## Step 2 · session_rust/src/brep.rs

Read the kernel's BRep; the lesson uses it as is.

??? example "`session_rust/src/brep.rs` · read only · 2950 lines"

    ```rust
    --8<-- "session_rust/src/brep.rs"
    ```
    

Run `cargo check` in `lessons/07/`.

## Step 3 · src/app/walk/brep_edges.rs

Chains are keyed by edge so two faces share one boundary.

`lessons/07/src/app/walk/brep_edges.rs` · type this, replace the whole file, start with these lines

```rust
--8<-- "lessons/07/src/app/walk/brep_edges.rs:step-3a"
```

`lessons/07/src/app/walk/brep_edges.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/07/src/app/walk/brep_edges.rs:step-3b"
```

`lessons/07/src/app/walk/brep_edges.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/07/src/app/walk/brep_edges.rs:step-3c"
```

`lessons/07/src/app/walk/brep_edges.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/07/src/app/walk/brep_edges.rs:step-3d"
```

`lessons/07/src/app/walk/brep_edges.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/07/src/app/walk/brep_edges.rs:step-3e"
```

`lessons/07/src/app/walk/brep_edges.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/07/src/app/walk/brep_edges.rs:step-3f"
```

`lessons/07/src/app/walk/brep_edges.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/07/src/app/walk/brep_edges.rs:step-3g"
```

Copy this part from the lesson folder to the path shown.

`lessons/07/src/app/walk/brep_edges.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/07/src/app/walk/brep_edges.rs:step-3h"
```

## Step 4 · src/app/walk/brep_orient.rs

New file: which way each face is walked, from its shared edges.

`lessons/07/src/app/walk/brep_orient.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/07/src/app/walk/brep_orient.rs:step-4a"
```

`lessons/07/src/app/walk/brep_orient.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/07/src/app/walk/brep_orient.rs:step-4b"
```

`lessons/07/src/app/walk/brep_orient.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/07/src/app/walk/brep_orient.rs:step-4c"
```

`lessons/07/src/app/walk/brep_orient.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/07/src/app/walk/brep_orient.rs:step-4d"
```

Copy this part from the lesson folder to the path shown.

`lessons/07/src/app/walk/brep_orient.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/07/src/app/walk/brep_orient.rs:step-4e"
```

## Step 5 · src/app/walk/mod.rs

The walk passes orientation to the BRep walk.

`lessons/07/src/app/walk/mod.rs` · edit · type this

Added below

```rust
pub mod brep_edges;
```

```rust
--8<-- "lessons/07/src/app/walk/mod.rs:step-5"
```

Run `cargo check` in `lessons/07/`.

## Step 6 · src/app/walk/brep.rs

The BRep walk draws each shared edge once.

`lessons/07/src/app/walk/brep.rs` · edit · type this

Replaces the line `use session_rust::{BRep, NurbsSurface, RenderMesh};` in `lessons/06/src/app/walk/brep.rs`

```rust
--8<-- "lessons/07/src/app/walk/brep.rs:step-6a"
```

Replaces the `fn walk_brep` lines in `lessons/06/src/app/walk/brep.rs`

```rust
--8<-- "lessons/07/src/app/walk/brep.rs:step-6b"
```

## Step 7 · src/fixture.rs

Copy the test scene: a block with a hole.
Copy this file from the lesson folder to the path shown.

`lessons/07/src/fixture.rs` · edit · copy the file

Replaces the line `use session_rust::{Color, Geometry, NurbsSurface, Point, …` in `lessons/06/src/fixture.rs`

```rust
--8<-- "lessons/07/src/fixture.rs:step-7a"
```

Replaces the `fn planar_surface` lines in `lessons/06/src/fixture.rs`

```rust
--8<-- "lessons/07/src/fixture.rs:step-7b"
```

## Step 8 · src/lib.rs

The entry point picks the scene from the URL.

`lessons/07/src/lib.rs` · edit · type this

Replaces the line `Ok(serde_json::json!({"stage":6,"objects":self.gpu.object…` in `lessons/06/src/lib.rs`

```rust
--8<-- "lessons/07/src/lib.rs:step-8"
```

## Step 9 · index.html

Copy the page: the status says checkpoint 07.
Copy this file from the lesson folder to the path shown.

`lessons/07/index.html` · edit · copy the file

Replaces the line `<title>Session checkpoint 06</title>` in `lessons/06/index.html`

```html
--8<-- "lessons/07/index.html:step-9a"
```

Replaces the line `<output id="status">Starting checkpoint 06</output>` in `lessons/06/index.html`

```html
--8<-- "lessons/07/index.html:step-9b"
```

Replaces the line `document.getElementById('status').textContent = 'Checkpoi…` in `lessons/06/index.html`

```html
--8<-- "lessons/07/index.html:step-9c"
```

## Check

Run `trunk serve` in `lessons/07/` and open <http://127.0.0.1:8770/>.

Expected: A cylinder and a block with a hole show continuous source boundary curves; status: **2 objects**.

![Checkpoint 07: a cylinder and a block with a hole; every CAD edge is ink drawn from the shared face-mesh nodes.](screenshots/07.png)

If it fails:

- A boundary floats or doubles: adjacent faces use different boundary samples.
- A diagonal appears as an edge: mesh topology is substituted for the source edge chain.

## What changed

```text
lessons/07/src/app/walk/
├── bounds.rs
├── brep.rs  ~
├── brep_edges.rs  ~
├── brep_orient.rs  +
├── curves.rs
├── encode.rs
├── mesh.rs
├── mesh_ink.rs
├── mesh_topology.rs
└── mod.rs  ~
```

`+` new in this lesson · `~` changed in this lesson

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: `lessons/07/`.

## Next

[08 · Trims and seams](08-trimming.md): holes, natural boundaries and repeated seam uses keep correct geometry and source IDs.

## Expected viewer result

Checkpoint 07: a cylinder and a block with a hole; every CAD edge is ink drawn from the shared face-mesh nodes.

[![Full viewer result for 07 boundaries](screenshots/07.png)](screenshots/07.png)
