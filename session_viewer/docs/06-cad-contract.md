# 06 · CAD face rules

The walk turns each kernel object into rows of the tables from lessons 01 to 04d: triangles, pipes, ribbons, dots and cloud points. BRep faces follow one contract, written once at the top of brep.rs and used by lessons 07 to 09.

![One face, three representations: BRep source in f64, kernel mesh with u,v, normals and boundary tags, viewer rows in f32 that keep the face and edge identities.](illustrations/cad-contract.svg)

Kernel code the walk calls, read only: [remesh_nurbssurface_grid.rs](kernel/remesh_nurbssurface_grid.md), [brep.rs](kernel/brep.md), [nurbssurface_trimmed.rs](kernel/nurbssurface_trimmed.md).

## Step 1 · src/app/mod.rs

The app's module list: one line per module, each added by the lesson that teaches it.

`lessons/06/src/app/mod.rs` · type this, new file

```rust
--8<-- "lessons/06/src/app/mod.rs"
```

## Step 2 · src/app/knobs.rs

Debug switches read once from the environment, such as faces without edges.

`lessons/06/src/app/knobs.rs` · type this, new file

```rust
--8<-- "lessons/06/src/app/knobs.rs"
```

## Step 3 · src/app/walk/encode.rs

Pack pen width, colour and face normals into the few bytes the shaders read.

`lessons/06/src/app/walk/encode.rs` · type this, new file

```rust
--8<-- "lessons/06/src/app/walk/encode.rs"
```

## Step 4 · src/app/walk/mod.rs

The walk module: its files and the row tables it writes into.

`lessons/06/src/app/walk/mod.rs` · type this, new file

```rust
--8<-- "lessons/06/src/app/walk/mod.rs:walk-mods"
```

## Step 5 · src/app/walk/mod.rs

The tables one object writes into, where its rows go, and what it reports back.

`lessons/06/src/app/walk/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mod.rs:walk-tables"
```

## Step 6 · src/app/walk/mod.rs

An element's features, drawn thick into the element's own row.

`lessons/06/src/app/walk/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mod.rs:walk-features"
```

## Step 7 · src/app/walk/mod.rs

One match sends every kind of geometry to its walk.

`lessons/06/src/app/walk/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mod.rs:walk-geometry"
```

## Step 8 · src/app/walk/mod.rs

Test: visible features join the element's row, hidden ones add nothing.

`lessons/06/src/app/walk/mod.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mod.rs:walk-tests"
```

## Step 9 · src/app/walk/points.rs

A point becomes one dot, 3 px when it has no pen.

`lessons/06/src/app/walk/points.rs` · type this, new file

```rust
--8<-- "lessons/06/src/app/walk/points.rs"
```

## Step 10 · src/app/walk/curves.rs

A polyline becomes one ribbon per span, joined into one chain.

`lessons/06/src/app/walk/curves.rs` · type this, new file

```rust
--8<-- "lessons/06/src/app/walk/curves.rs:curve-segments"
```

## Step 11 · src/app/walk/curves.rs

Arrowheads: a head-only vector row at each headed end, and the ribbon marked to stop under it.

`lessons/06/src/app/walk/curves.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/curves.rs:curve-heads"
```

## Step 12 · src/app/walk/curves.rs

Lines and polylines, each with its heads.

`lessons/06/src/app/walk/curves.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/curves.rs:curve-walks"
```

## Step 13 · src/app/walk/curves.rs

A NURBS curve becomes a polyline, one chord per 5° of turning.

`lessons/06/src/app/walk/curves.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/curves.rs:curve-sampling"
```

## Step 14 · src/app/walk/curves.rs

Tests: heads aim along the end segments and land on the curve's end point.

`lessons/06/src/app/walk/curves.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/curves.rs:curve-tests"
```

## Step 15 · src/app/walk/frames.rs

A plane becomes a 1 m square, a box its 12 edges.

`lessons/06/src/app/walk/frames.rs` · type this, new file

```rust
--8<-- "lessons/06/src/app/walk/frames.rs"
```

## Step 16 · src/app/walk/plane.rs

A clipping plane: its name, the closedness switch, and its rectangle with an arrow; lesson 18b cuts with it.

`lessons/06/src/app/walk/plane.rs` · type this, new file

```rust
--8<-- "lessons/06/src/app/walk/plane.rs:plane-walk"
```

## Step 17 · src/app/walk/plane.rs

Whether a mesh is a closed solid and whether its faces wind inward, the two facts a section cap needs.

`lessons/06/src/app/walk/plane.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/plane.rs:solid-orientation"
```

## Step 18 · src/app/walk/plane.rs

Small vector helpers on three f64 numbers.

`lessons/06/src/app/walk/plane.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/plane.rs:plane-math"
```

## Step 19 · src/app/walk/bounds.rs

Table lengths before a file is walked, and the box of everything it added.

`lessons/06/src/app/walk/bounds.rs` · type this, new file

```rust
--8<-- "lessons/06/src/app/walk/bounds.rs:bounds-baselines"
```

## Step 20 · src/app/walk/bounds.rs

The z band of a flat drawing, and whether a row lies inside it.

`lessons/06/src/app/walk/bounds.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/bounds.rs:bounds-bands"
```

## Step 21 · src/app/walk/bounds.rs

Pipes, ribbons and heads of a sheet get a 1 mm pen where none was set.

`lessons/06/src/app/walk/bounds.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/bounds.rs:bounds-pens"
```

## Step 22 · src/app/walk/cloud.rs

A point cloud becomes points, octree nodes and one draw.

`lessons/06/src/app/walk/cloud.rs` · type this, new file

```rust
--8<-- "lessons/06/src/app/walk/cloud.rs:cloud-walk"
```

## Step 23 · src/app/walk/cloud.rs

Copy points and nodes, and estimate the point spacing from the cloud's size.

`lessons/06/src/app/walk/cloud.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/cloud.rs:cloud-copy"
```

## Step 24 · src/app/walk/mesh_topology.rs

Map sparse vertex keys to dense slots.

`lessons/06/src/app/walk/mesh_topology.rs` · type this, new file

```rust
--8<-- "lessons/06/src/app/walk/mesh_topology.rs:slot-map"
```

## Step 25 · src/app/walk/mesh_topology.rs

The edges, faces and normals of one mesh, and the normal of one face.

`lessons/06/src/app/walk/mesh_topology.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh_topology.rs:mesh-topo"
```

## Step 26 · src/app/walk/mesh_topology.rs

Find each edge once, with its two faces and whether they agree on winding, hole rings included.

`lessons/06/src/app/walk/mesh_topology.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh_topology.rs:mesh-edges"
```

## Step 27 · src/app/walk/mesh_topology.rs

Tests: hole rims carry both faces, and an open face keeps its hole boundary.

`lessons/06/src/app/walk/mesh_topology.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh_topology.rs:topology-tests"
```

## Step 28 · src/app/walk/mesh_ink.rs

Where a mesh's edges and dots go, and what the ink pass needs from the face pass.

`lessons/06/src/app/walk/mesh_ink.rs` · type this, new file

```rust
--8<-- "lessons/06/src/app/walk/mesh_ink.rs:ink-context"
```

## Step 29 · src/app/walk/mesh_ink.rs

Small rules: pen width per edge, the two outward normals, borders and creases.

`lessons/06/src/app/walk/mesh_ink.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh_ink.rs:ink-edge-rules"
```

## Step 30 · src/app/walk/mesh_ink.rs

One pipe per drawn edge; diagonals inside flat regions are skipped.

`lessons/06/src/app/walk/mesh_ink.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh_ink.rs:ink-pipes"
```

## Step 31 · src/app/walk/mesh_ink.rs

Which edges meet at each vertex, kept in one flat list.

`lessons/06/src/app/walk/mesh_ink.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh_ink.rs:ink-incidence"
```

## Step 32 · src/app/walk/mesh_ink.rs

One marker per vertex with a visible edge, carrying up to six face normals.

`lessons/06/src/app/walk/mesh_ink.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh_ink.rs:ink-markers"
```

## Step 33 · src/app/walk/mesh_ink.rs

Edges first, then vertex dots.

`lessons/06/src/app/walk/mesh_ink.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh_ink.rs:ink-entry"
```

## Step 34 · src/app/walk/mesh_ink.rs

Tests: a box has 12 edges and 8 dots; a smooth mesh inks only borders and creases.

`lessons/06/src/app/walk/mesh_ink.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh_ink.rs:ink-tests"
```

## Step 35 · src/app/walk/mesh.rs

The limits: raw meshes, black wireframes, flat regions and creases.

`lessons/06/src/app/walk/mesh.rs` · type this, new file

```rust
--8<-- "lessons/06/src/app/walk/mesh.rs:mesh-limits"
```

## Step 36 · src/app/walk/mesh.rs

How a plain mesh, a sampled surface and an element's mesh are walked differently.

`lessons/06/src/app/walk/mesh.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh.rs:mesh-opts"
```

## Step 37 · src/app/walk/mesh.rs

A lap timer natively, an empty one in the browser.

`lessons/06/src/app/walk/mesh.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh.rs:mesh-lap"
```

## Step 38 · src/app/walk/mesh.rs

A mesh: triangles into the arena, its flags, then edges and dots.

`lessons/06/src/app/walk/mesh.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh.rs:mesh-walk"
```

## Step 39 · src/app/walk/mesh.rs

One source face address per triangle.

`lessons/06/src/app/walk/mesh.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/mesh.rs:mesh-face-ids"
```

## Step 40 · src/app/walk/brep_edges.rs

Edge chains, facets and edge pipes; lessons 07, 08 and 09 read this file part by part.

`lessons/06/src/app/walk/brep_edges.rs` · copy the file

```rust
--8<-- "lessons/06/src/app/walk/brep_edges.rs"
```

## Step 41 · src/app/walk/brep_orient.rs

Face signs that turn every normal of a solid outward; lesson 09 reads this file.

`lessons/06/src/app/walk/brep_orient.rs` · copy the file

```rust
--8<-- "lessons/06/src/app/walk/brep_orient.rs"
```

## Step 42 · src/app/walk/brep.rs

The CAD contract, the mesh quality, and the running totals of one solid.

`lessons/06/src/app/walk/brep.rs` · type this, new file

```rust
--8<-- "lessons/06/src/app/walk/brep.rs:brep-solid"
```

## Step 43 · src/app/walk/brep.rs

Append one face mesh to the arena, with its source face.

`lessons/06/src/app/walk/brep.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/brep.rs:brep-face"
```

## Step 44 · src/app/walk/brep.rs

A BRep: mesh every face, turn inside-out faces, set the flags, then draw the edges.

`lessons/06/src/app/walk/brep.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/brep.rs:brep-walk"
```

## Step 45 · src/app/walk/brep.rs

Every BRep edge as pipes on the face meshes, or as a ribbon from its 3D curve.

`lessons/06/src/app/walk/brep.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/brep.rs:brep-edges"
```

## Step 46 · src/app/walk/brep.rs

A bare NURBS surface as a grid with its four border edges.

`lessons/06/src/app/walk/brep.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/brep.rs:brep-surface"
```

## Step 47 · src/app/walk/brep.rs

Tests: teapot patches, cylinder normals, open and reversed shells, poles and a pyramid apex.

`lessons/06/src/app/walk/brep.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/brep.rs:brep-tests"
```

## Step 48 · src/app/walk/brep.rs

Remember each vertex's (u, v), so a later edit can move it on its surface.

`lessons/06/src/app/walk/brep.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/app/walk/brep.rs:brep-samples"
```

## Step 49 · src/lib.rs

The crate gains the app module.

`lessons/06/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/06/src/lib.rs:app-mod"
```

## Step 50 · src/engine/gpu/vectors.rs

Copy the test module between the `06-curve-heads-test` markers at the end of the file: curve heads land on their end points.

`lessons/06/src/engine/gpu/vectors.rs` · copy this part, append at the end of the file

Run `cargo check` in `lessons/06/`.

## Check

Run `cargo xtest --lib app::walk` in `lessons/06/`: boxes, curves, clouds and solids walk into the expected pipes, dots and heads. The teapot test reads `assets/pb/view_mixed_teapot.pb`, which the crate gains in lesson 16.
