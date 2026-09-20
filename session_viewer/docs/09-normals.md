# 09 · Normals and shading
<!-- locator: off -->

The sphere shades smoothly and sharp rims retain separate normals under transformed placements.

![Analytic normal or finite fallback at a pole; two shading normals at a C0 crease; the cofactor transform keeps a normal perpendicular under nonuniform scale.](illustrations/normals.svg)

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

## Step 1 · session_rust/src/nurbssurface_trimmed.rs

Read this source file from its link; the checkpoint already contains it.
<!-- listing: 09 session_rust/src/nurbssurface_trimmed.rs -->
Download each file to the path shown.
<!-- supplied: 09 -->
<!-- check: 09 -->
## Step 2 · src/shaders/normals.wgsl

Normal helpers keep lighting correct under transformed placements. Nonuniform scale requires the inverse transpose.
<!-- file: 09 session_viewer/src/shaders/normals.wgsl type -->
## Step 3 · src/shaders/triangle.wgsl

The mesh shader places vertices and shades visible faces. Its instance row must be the row uploaded with that vertex.
<!-- file: 09 session_viewer/src/shaders/triangle.wgsl type -->
## Step 4 · src/app/walk/brep_edges.rs

Boundary chains share samples between adjacent faces. Preserve oriented source edge IDs through the upload.
<!-- file: 09 session_viewer/src/app/walk/brep_edges.rs type hunks=1-4 -->
Download this part from its link to the path shown.
<!-- file: 09 session_viewer/src/app/walk/brep_edges.rs copy hunks=5-8 -->
## Step 5 · src/app/walk/brep.rs

The geometry walk uploads shaded faces and their source boundary edges. Tessellation diagonals must never become CAD edges.
<!-- file: 09 session_viewer/src/app/walk/brep.rs type hunks=1-1 -->
Download this part from its link to the path shown.
<!-- file: 09 session_viewer/src/app/walk/brep.rs copy hunks=2-2 -->
<!-- check: 09 -->
## Step 6 · src/engine/gpu/mod.rs

The GPU owner connects buffers, pipelines and frame resources. Create resources before building the bind groups that refer to them.
<!-- file: 09 session_viewer/src/engine/gpu/mod.rs type -->
## Step 7 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners. Wire initialization and frame updates together so a new module actually runs.
<!-- file: 09 session_viewer/src/lib.rs type -->
## Step 8 · src/fixture.rs

Download this file from its link to the path shown.
<!-- file: 09 session_viewer/src/fixture.rs copy -->
## Step 9 · index.html

Download this file from its link to the path shown.
<!-- file: 09 session_viewer/index.html copy -->
## Check

<!-- checkpoint: 09 -->

Expected: The sphere shades smoothly and sharp rims retain separate normals under transformed placements; status: **1 object**.

![Checkpoint 09 at `?cad=sphere&lit=1`: the sphere's interior shades smoothly, with no facet pattern.](screenshots/09.png)

If it fails:

- A sphere looks faceted: triangle normals replace surface normals.
- Shading crosses a sharp rim: coincident positions incorrectly share a normal.
- A mirrored copy shades differently: the normal transform ignores the inverse transpose.

## What changed

<!-- tree: 09 session_viewer/src -->

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: [source at checkpoint 09](../lessons/09/index.md).

## Next

[10 · Text shaping](10-text-layout.md): fonts, glyph advances and clusters before any pixel is drawn.

## Expected viewer result

Checkpoint 09 at `?cad=sphere&lit=1`: the sphere's interior shades smoothly, with no facet pattern.

[![Full viewer result for 09 normals](screenshots/09.png)](screenshots/09.png)
