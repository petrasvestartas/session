# 08 · Trims, holes and periodic seams

A trimmed patch has an empty hole and a torus keeps its periodic seams attached.

![Left: outer and inner loops select the face in u,v and the hole stays empty. Right: a cylinder's seam is one XYZ curve used at u=0 and u=1.](illustrations/trims-seams.svg)

## Step 1 · src/app/walk/brep.rs

The BRep walk uses the cached trim mesh when the surface has one.

`lessons/08/src/app/walk/brep.rs` · edit · type this

Replaces the line `use session_rust::{BRep, Color, NurbsSurface, RenderMesh};` in `lessons/07/src/app/walk/brep.rs`

```rust
--8<-- "lessons/08/src/app/walk/brep.rs:step-1a"
```

`lessons/08/src/app/walk/brep.rs` · edit · type this

Replaces the `fn walk_surface` lines in `lessons/07/src/app/walk/brep.rs`

```rust
--8<-- "lessons/08/src/app/walk/brep.rs:step-1b"
```

Run `cargo check` in `lessons/08/`.

## Step 2 · src/fixture.rs

Copy the test scene: a trimmed patch and a torus.
Copy this file from the lesson folder to the path shown.

`lessons/08/src/fixture.rs` · edit · copy the file

Replaces the line `use session_rust::{BRep, Color, Geometry, Xform};` in `lessons/07/src/fixture.rs`

```rust
--8<-- "lessons/08/src/fixture.rs:step-2a"
```

Replaces the lines from `let mut cylinder = BRep::create_cylinder(100.0, 220.0);` to `Xform::translation(220.0, 0.0, 0.0),` in `lessons/07/src/fixture.rs`

```rust
--8<-- "lessons/08/src/fixture.rs:step-2b"
```

## Step 3 · src/lib.rs

The entry point reports the trimmed faces.

`lessons/08/src/lib.rs` · edit · type this

Replaces the line `Ok(serde_json::json!({"stage":7,"objects":self.gpu.object…` in `lessons/07/src/lib.rs`

```rust
--8<-- "lessons/08/src/lib.rs:step-3"
```

## Step 4 · index.html

Copy the page: the status says checkpoint 08.
Copy this file from the lesson folder to the path shown.

`lessons/08/index.html` · edit · copy the file

Replaces the line `<title>Session checkpoint 07</title>` in `lessons/07/index.html`

```html
--8<-- "lessons/08/index.html:step-4a"
```

Replaces the line `<output id="status">Starting checkpoint 07</output>` in `lessons/07/index.html`

```html
--8<-- "lessons/08/index.html:step-4b"
```

Replaces the line `document.getElementById('status').textContent = 'Checkpoi…` in `lessons/07/index.html`

```html
--8<-- "lessons/08/index.html:step-4c"
```

## Check

Run `trunk serve` in `lessons/08/` and open <http://127.0.0.1:8770/>.

Expected: A trimmed patch has an empty hole and a torus keeps its periodic seams attached; status: **2 objects**.

![Checkpoint 08: a trimmed patch with its hole left empty and a torus whose seams are drawn once.](screenshots/08.png)

If it fails:

- A periodic boundary crosses the patch: its UV branch or oriented use mapping is wrong.
- The hole stays filled: trimming changes ink without removing covered triangles.

## What changed

```text
lessons/08/src/app/
├── walk/
│   ├── bounds.rs
│   ├── brep.rs  ~
│   ├── brep_edges.rs
│   ├── brep_orient.rs
│   ├── curves.rs
│   ├── encode.rs
│   ├── mesh.rs
│   ├── mesh_ink.rs
│   ├── mesh_topology.rs
│   └── mod.rs
├── knobs.rs
├── mod.rs
└── route.rs
```

`+` new in this lesson · `~` changed in this lesson

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: `lessons/08/`.

## Next

[09 · Normals and shading](09-normals.md): analytic normals, singular fallbacks, C0 splits and the affine normal transform.

## Expected viewer result

Checkpoint 08: a trimmed patch with its hole left empty and a torus whose seams are drawn once.

[![Full viewer result for 08 trimming](screenshots/08.png)](screenshots/08.png)
