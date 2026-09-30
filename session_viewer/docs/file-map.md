# Where every file is taught

These links open the first and last additions inside their buildable tutorial step.

| File | Responsibility | First addition | Last addition |
| --- | --- | --- | --- |
| `.gitignore` | Keep source and build output apart | [00-001](00-environment.md#code-00-001) | [00-001](00-environment.md#code-00-001) |
| `.cargo/config.toml` | Choose where Rust runs | [00-002](00-environment.md#code-00-002) | [00-002](00-environment.md#code-00-002) |
| `Cargo.toml` | Describe the program to Cargo | [00-003](00-environment.md#code-00-003) | [00-003](00-environment.md#code-00-003) |
| `Trunk.toml` | Build a browser application | [00-004](00-environment.md#code-00-004) | [00-004](00-environment.md#code-00-004) |
| `index.html` | Give the browser a home for the viewer | [00-005](00-environment.md#code-00-005) | [12-001](12-picking.md#code-12-001) |
| `src/lib.rs` | Connect modules to the browser | [00-006](00-environment.md#code-00-006) | [37-001](37-command-dock.md#code-37-001) |
| `build.rs` | Prepare shader source before compiling | [01-003](01-first-frame.md#code-01-003) | [01-003](01-first-frame.md#code-01-003) |
| `examples/course_frame.rs` | Construct a reproducible specimen | [01-004](01-first-frame.md#code-01-004) | [01-004](01-first-frame.md#code-01-004) |
| `src/engine/gpu/backdrop.rs` | Paint the background | [01-005](01-first-frame.md#code-01-005) | [02-011](02-camera.md#code-02-011) |
| `src/engine/gpu/buffers.rs` | Give GPU bytes a meaning | [01-006](01-first-frame.md#code-01-006) | [01-006](01-first-frame.md#code-01-006) |
| `src/engine/gpu/device.rs` | Ask for a GPU connection | [01-007](01-first-frame.md#code-01-007) | [01-007](01-first-frame.md#code-01-007) |
| `src/engine/gpu/frame.rs` | Own one frame of work | [01-008](01-first-frame.md#code-01-008) | [01-008](01-first-frame.md#code-01-008) |
| `src/engine/gpu/hull.rs` | Bound geometry cheaply | [01-009](01-first-frame.md#code-01-009) | [04d-002](04d-clouds.md#code-04d-002) |
| `src/engine/gpu/instance.rs` | Describe one draw instance | [01-010](01-first-frame.md#code-01-010) | [18-003](18-finite-visibility.md#code-18-003) |
| `src/engine/gpu/lane.rs` | Group compatible drawing work | [01-011](01-first-frame.md#code-01-011) | [04b-001](04b-strokes.md#code-04b-001) |
| `src/engine/gpu/mod.rs` | Assemble the renderer | [01-012](01-first-frame.md#code-01-012) | [32-006](32-colors-lighting.md#code-32-006) |
| `src/engine/gpu/objects.rs` | Connect scene objects to shader records | [01-013](01-first-frame.md#code-01-013) | [04b-028](04b-strokes.md#code-04b-028) |
| `src/engine/gpu/pass.rs` | Record a render pass | [01-014](01-first-frame.md#code-01-014) | [32-007](32-colors-lighting.md#code-32-007) |
| `src/engine/gpu/present.rs` | Finish the image | [01-015](01-first-frame.md#code-01-015) | [32-010](32-colors-lighting.md#code-32-010) |
| `src/engine/gpu/render.rs` | Order the drawing passes | [01-016](01-first-frame.md#code-01-016) | [32-017](32-colors-lighting.md#code-32-017) |
| `src/engine/gpu/targets.rs` | Allocate the pictures a frame needs | [01-017](01-first-frame.md#code-01-017) | [01-017](01-first-frame.md#code-01-017) |
| `src/engine/gpu/upload.rs` | Send CPU data to the GPU | [01-018](01-first-frame.md#code-01-018) | [20-020](20-history.md#code-20-020) |
| `src/engine/gpu/view.rs` | Keep frame-wide values together | [01-019](01-first-frame.md#code-01-019) | [01-019](01-first-frame.md#code-01-019) |
| `src/engine/mod.rs` | Assemble the renderer | [01-020](01-first-frame.md#code-01-020) | [10-001](10-text-layout.md#code-10-001) |
| `src/engine/performance.rs` | Account for a frame | [01-021](01-first-frame.md#code-01-021) | [01-021](01-first-frame.md#code-01-021) |
| `src/engine/pipelines/bindings.rs` | Make the Rust–shader contract explicit | [01-022](01-first-frame.md#code-01-022) | [01-022](01-first-frame.md#code-01-022) |
| `src/engine/pipelines/layouts.rs` | Make the Rust–shader contract explicit | [01-023](01-first-frame.md#code-01-023) | [01-023](01-first-frame.md#code-01-023) |
| `src/engine/pipelines/mod.rs` | Make the Rust–shader contract explicit | [01-024](01-first-frame.md#code-01-024) | [04b-045](04b-strokes.md#code-04b-045) |
| `src/shaders/background.wgsl` | Read the GPU side of the contract | [01-025](01-first-frame.md#code-01-025) | [01-025](01-first-frame.md#code-01-025) |
| `src/shaders/clip.wgsl` | Read the GPU side of the contract | [01-026](01-first-frame.md#code-01-026) | [01-026](01-first-frame.md#code-01-026) |
| `src/shaders/normals.wgsl` | Keep a normal perpendicular after scaling | [01-027](01-first-frame.md#code-01-027) | [01-027](01-first-frame.md#code-01-027) |
| `src/shaders/physical.wgsl` | Store identity beside colour | [01-028](01-first-frame.md#code-01-028) | [01-028](01-first-frame.md#code-01-028) |
| `src/shaders/scene.wgsl` | Share exact scene records with Rust | [01-029](01-first-frame.md#code-01-029) | [01-029](01-first-frame.md#code-01-029) |
| `src/camera.rs` | Move the eye, not the model | [02-002](02-camera.md#code-02-002) | [02-002](02-camera.md#code-02-002) |
| `src/shaders/grid.wgsl` | Read the GPU side of the contract | [02-014](02-camera.md#code-02-014) | [02-014](02-camera.md#code-02-014) |
| `src/engine/gpu/patch.rs` | Change a small part of GPU state | [03-007](03-identity.md#code-03-007) | [04c-022](04c-markers.md#code-04c-022) |
| `src/engine/gpu/arena.rs` | Place many objects in shared storage | [04a-001](04a-meshes.md#code-04a-001) | [18-002](18-finite-visibility.md#code-18-002) |
| `src/engine/gpu/faces.rs` | Draw surface triangles | [04a-002](04a-meshes.md#code-04a-002) | [17-014](17-source-presentation.md#code-17-014) |
| `src/engine/gpu/slots.rs` | Reuse space without losing identity | [04a-025](04a-meshes.md#code-04a-025) | [04a-025](04a-meshes.md#code-04a-025) |
| `src/engine/gpu/text_outline.rs` | Place text in the scene | [04a-026](04a-meshes.md#code-04a-026) | [12-035](12-picking.md#code-12-035) |
| `src/engine/gpu/triangle_tiles.rs` | Look only at nearby triangles | [04a-027](04a-meshes.md#code-04a-027) | [18-014](18-finite-visibility.md#code-18-014) |
| `src/shaders/project_triangles.wgsl` | Read the GPU side of the contract | [04a-034](04a-meshes.md#code-04a-034) | [04a-034](04a-meshes.md#code-04a-034) |
| `src/shaders/projected_triangle.wgsl` | Read the GPU side of the contract | [04a-035](04a-meshes.md#code-04a-035) | [04a-035](04a-meshes.md#code-04a-035) |
| `src/shaders/scan_triangle_tiles.wgsl` | Read the GPU side of the contract | [04a-036](04a-meshes.md#code-04a-036) | [04a-036](04a-meshes.md#code-04a-036) |
| `src/shaders/slot_table.wgsl` | Read the GPU side of the contract | [04a-037](04a-meshes.md#code-04a-037) | [04a-037](04a-meshes.md#code-04a-037) |
| `src/shaders/text_outline.wgsl` | Read the GPU side of the contract | [04a-038](04a-meshes.md#code-04a-038) | [04a-038](04a-meshes.md#code-04a-038) |
| `src/shaders/triangle.wgsl` | Read the GPU side of the contract | [04a-039](04a-meshes.md#code-04a-039) | [04a-039](04a-meshes.md#code-04a-039) |
| `src/shaders/triangle_tiles.wgsl` | Read the GPU side of the contract | [04a-040](04a-meshes.md#code-04a-040) | [04a-040](04a-meshes.md#code-04a-040) |
| `src/engine/gpu/segments.rs` | Draw readable strokes | [04b-039](04b-strokes.md#code-04b-039) | [04b-039](04b-strokes.md#code-04b-039) |
| `src/engine/gpu/vectors.rs` | Draw direction as geometry | [04b-044](04b-strokes.md#code-04b-044) | [12-036](12-picking.md#code-12-036) |
| `src/shaders/ink_visibility.wgsl` | Read the GPU side of the contract | [04b-046](04b-strokes.md#code-04b-046) | [04b-046](04b-strokes.md#code-04b-046) |
| `src/shaders/ribbon.wgsl` | Read the GPU side of the contract | [04b-047](04b-strokes.md#code-04b-047) | [04b-047](04b-strokes.md#code-04b-047) |
| `src/shaders/vector.wgsl` | Read the GPU side of the contract | [04b-048](04b-strokes.md#code-04b-048) | [04b-048](04b-strokes.md#code-04b-048) |
| `src/engine/gpu/glyphs.rs` | Draw point markers | [04c-001](04c-markers.md#code-04c-001) | [04c-001](04c-markers.md#code-04c-001) |
| `src/shaders/glyph.wgsl` | Read the GPU side of the contract | [04c-031](04c-markers.md#code-04c-031) | [04c-031](04c-markers.md#code-04c-031) |
| `src/shaders/sphere.wgsl` | Read the GPU side of the contract | [04c-032](04c-markers.md#code-04c-032) | [04c-032](04c-markers.md#code-04c-032) |
| `src/engine/gpu/cloud.rs` | Render many points together | [04d-001](04d-clouds.md#code-04d-001) | [04d-001](04d-clouds.md#code-04d-001) |
| `src/engine/gpu/lod.rs` | Choose useful detail | [04d-003](04d-clouds.md#code-04d-003) | [04d-003](04d-clouds.md#code-04d-003) |
| `src/engine/gpu/splat.rs` | Give each point a footprint | [04d-032](04d-clouds.md#code-04d-032) | [04d-032](04d-clouds.md#code-04d-032) |
| `src/shaders/splat.wgsl` | Read the GPU side of the contract | [04d-037](04d-clouds.md#code-04d-037) | [04d-037](04d-clouds.md#code-04d-037) |
| `src/shaders/splat_resolve.wgsl` | Read the GPU side of the contract | [04d-038](04d-clouds.md#code-04d-038) | [04d-038](04d-clouds.md#code-04d-038) |
| `src/app/knobs.rs` | Connect interaction with the scene | [06-002](06-cad-contract.md#code-06-002) | [06-002](06-cad-contract.md#code-06-002) |
| `src/app/mod.rs` | Connect interaction with the scene | [06-003](06-cad-contract.md#code-06-003) | [31-004](31-splitting.md#code-31-004) |
| `src/app/walk/bounds.rs` | Translate geometry into renderable records | [06-004](06-cad-contract.md#code-06-004) | [06-004](06-cad-contract.md#code-06-004) |
| `src/app/walk/brep.rs` | Respect CAD face boundaries | [06-005](06-cad-contract.md#code-06-005) | [06-005](06-cad-contract.md#code-06-005) |
| `src/app/walk/brep_edges.rs` | Respect CAD face boundaries | [06-006](06-cad-contract.md#code-06-006) | [06-006](06-cad-contract.md#code-06-006) |
| `src/app/walk/brep_orient.rs` | Respect CAD face boundaries | [06-007](06-cad-contract.md#code-06-007) | [06-007](06-cad-contract.md#code-06-007) |
| `src/app/walk/cloud.rs` | Translate geometry into renderable records | [06-008](06-cad-contract.md#code-06-008) | [15-023](15-publication.md#code-15-023) |
| `src/app/walk/curves.rs` | Translate geometry into renderable records | [06-009](06-cad-contract.md#code-06-009) | [06-009](06-cad-contract.md#code-06-009) |
| `src/app/walk/encode.rs` | Translate geometry into renderable records | [06-010](06-cad-contract.md#code-06-010) | [06-010](06-cad-contract.md#code-06-010) |
| `src/app/walk/frames.rs` | Translate geometry into renderable records | [06-011](06-cad-contract.md#code-06-011) | [06-011](06-cad-contract.md#code-06-011) |
| `src/app/walk/mesh.rs` | Translate geometry into renderable records | [06-012](06-cad-contract.md#code-06-012) | [06-012](06-cad-contract.md#code-06-012) |
| `src/app/walk/mesh_ink.rs` | Select meaningful mesh edges | [06-013](06-cad-contract.md#code-06-013) | [06-013](06-cad-contract.md#code-06-013) |
| `src/app/walk/mesh_topology.rs` | Recover mesh relationships | [06-014](06-cad-contract.md#code-06-014) | [06-014](06-cad-contract.md#code-06-014) |
| `src/app/walk/mod.rs` | Translate geometry into renderable records | [06-015](06-cad-contract.md#code-06-015) | [19-020](19-sheets.md#code-19-020) |
| `src/app/walk/plane.rs` | Translate geometry into renderable records | [06-016](06-cad-contract.md#code-06-016) | [06-016](06-cad-contract.md#code-06-016) |
| `src/app/walk/points.rs` | Translate geometry into renderable records | [06-017](06-cad-contract.md#code-06-017) | [06-017](06-cad-contract.md#code-06-017) |
| `src/engine/text.rs` | Shape letters before drawing | [10-002](10-text-layout.md#code-10-002) | [23-071](23-geometry-commands.md#code-23-071) |
| `assets/text-quality.html` | Inspect a text specimen in the browser | [11-003](11-text-rendering.md#code-11-003) | [11-003](11-text-rendering.md#code-11-003) |
| `src/engine/gpu/text.rs` | Place text in the scene | [11-012](11-text-rendering.md#code-11-012) | [11-012](11-text-rendering.md#code-11-012) |
| `src/engine/gpu/text_plane.rs` | Place text in the scene | [11-013](11-text-rendering.md#code-11-013) | [11-013](11-text-rendering.md#code-11-013) |
| `src/engine/gpu/text_plate.rs` | Place text in the scene | [11-014](11-text-rendering.md#code-11-014) | [11-014](11-text-rendering.md#code-11-014) |
| `src/shaders/text_plane.wgsl` | Read the GPU side of the contract | [11-015](11-text-rendering.md#code-11-015) | [11-015](11-text-rendering.md#code-11-015) |
| `src/shaders/text_plate.wgsl` | Read the GPU side of the contract | [11-016](11-text-rendering.md#code-11-016) | [11-016](11-text-rendering.md#code-11-016) |
| `src/text_quality.rs` | Make text defects visible | [11-017](11-text-rendering.md#code-11-017) | [11-017](11-text-rendering.md#code-11-017) |
| `assets/view_local.yaml` | Choose the local course scene | [12-004](12-picking.md#code-12-004) | [12-004](12-picking.md#code-12-004) |
| `src/app/feedback.rs` | Connect interaction with the scene | [12-005](12-picking.md#code-12-005) | [30-008](30-layer-tree.md#code-30-008) |
| `src/app/gesture/mod.rs` | Give a drag one owner | [12-006](12-picking.md#code-12-006) | [21-012](21-editing.md#code-21-012) |
| `src/app/input.rs` | Route input to the right action | [12-007](12-picking.md#code-12-007) | [25-001](25-gumball.md#code-25-001) |
| `src/app/inspection.rs` | Connect interaction with the scene | [12-008](12-picking.md#code-12-008) | [31-003](31-splitting.md#code-31-003) |
| `src/app/keys.rs` | Connect interaction with the scene | [12-009](12-picking.md#code-12-009) | [30-010](30-layer-tree.md#code-30-010) |
| `src/app/route.rs` | Connect interaction with the scene | [12-012](12-picking.md#code-12-012) | [12-012](12-picking.md#code-12-012) |
| `src/app/scene.rs` | Represent the displayed scene | [12-013](12-picking.md#code-12-013) | [21-057](21-editing.md#code-21-057) |
| `src/app/scene_rows.rs` | Represent the displayed scene | [12-014](12-picking.md#code-12-014) | [12-014](12-picking.md#code-12-014) |
| `src/app/selection.rs` | Store what the user chose | [12-015](12-picking.md#code-12-015) | [12-015](12-picking.md#code-12-015) |
| `src/app/touch.rs` | Interpret fingers as gestures | [12-016](12-picking.md#code-12-016) | [12-016](12-picking.md#code-12-016) |
| `src/engine/gpu/pick.rs` | Turn a pixel into an identity | [12-028](12-picking.md#code-12-028) | [12-028](12-picking.md#code-12-028) |
| `src/state.rs` | Coordinate application state | [12-037](12-picking.md#code-12-037) | [36-005](36-translucent-faces.md#code-36-005) |
| `src/state/features.rs` | Coordinate application state | [12-038](12-picking.md#code-12-038) | [36-006](36-translucent-faces.md#code-36-006) |
| `tests/selection.cjs` | Specify behavior with a controlled experiment | [12-039](12-picking.md#code-12-039) | [12-039](12-picking.md#code-12-039) |
| `src/app/decode.rs` | Read serialized scene data | [14-005](14-loading.md#code-14-005) | [14-005](14-loading.md#code-14-005) |
| `src/app/fetch.rs` | Request bytes asynchronously | [14-006](14-loading.md#code-14-006) | [14-006](14-loading.md#code-14-006) |
| `src/app/fonts.rs` | Load the fonts text actually needs | [14-007](14-loading.md#code-14-007) | [14-007](14-loading.md#code-14-007) |
| `src/app/live.rs` | Connect interaction with the scene | [14-008](14-loading.md#code-14-008) | [14-008](14-loading.md#code-14-008) |
| `src/app/loader.rs` | Turn loading into explicit state | [14-009](14-loading.md#code-14-009) | [23-049](23-geometry-commands.md#code-23-049) |
| `src/app/manifest.rs` | Describe which scene files to load | [14-010](14-loading.md#code-14-010) | [14-010](14-loading.md#code-14-010) |
| `src/app/range_gate.rs` | Control partial reads | [14-015](14-loading.md#code-14-015) | [14-015](14-loading.md#code-14-015) |
| `src/app/validate.rs` | Connect interaction with the scene | [14-016](14-loading.md#code-14-016) | [14-016](14-loading.md#code-14-016) |
| `tests/lifecycle.cjs` | Specify behavior with a controlled experiment | [14-018](14-loading.md#code-14-018) | [14-018](14-loading.md#code-14-018) |
| `tests/loading.cjs` | Specify behavior with a controlled experiment | [14-019](14-loading.md#code-14-019) | [14-019](14-loading.md#code-14-019) |
| `src/app/cloud_query.rs` | Query a large cloud selectively | [15-004](15-publication.md#code-15-004) | [15-004](15-publication.md#code-15-004) |
| `src/app/stream.rs` | Publish data as it arrives | [15-022](15-publication.md#code-15-022) | [19-019](19-sheets.md#code-19-019) |
| `src/state/cloud_query.rs` | Coordinate application state | [15-034](15-publication.md#code-15-034) | [15-034](15-publication.md#code-15-034) |
| `tests/publication.py` | Specify behavior with a controlled experiment | [15-037](15-publication.md#code-15-037) | [15-037](15-publication.md#code-15-037) |
| `tests/streamed-controls.cjs` | Specify behavior with a controlled experiment | [15-038](15-publication.md#code-15-038) | [15-038](15-publication.md#code-15-038) |
| `assets/pb/.gitkeep` | Keep an empty asset directory | [16-001](16-accounting.md#code-16-001) | [16-001](16-accounting.md#code-16-001) |
| `examples/add_lod.rs` | Construct a reproducible specimen | [16-002](16-accounting.md#code-16-002) | [16-002](16-accounting.md#code-16-002) |
| `examples/cad_boundary_audit.rs` | Construct a reproducible specimen | [16-003](16-accounting.md#code-16-003) | [16-003](16-accounting.md#code-16-003) |
| `examples/cad_fixture.rs` | Construct a reproducible specimen | [16-004](16-accounting.md#code-16-004) | [16-004](16-accounting.md#code-16-004) |
| `examples/census_plates.rs` | Construct a reproducible specimen | [16-005](16-accounting.md#code-16-005) | [16-005](16-accounting.md#code-16-005) |
| `examples/check_determinism.rs` | Construct a reproducible specimen | [16-006](16-accounting.md#code-16-006) | [16-006](16-accounting.md#code-16-006) |
| `examples/interaction_fixture.rs` | Construct a reproducible specimen | [16-007](16-accounting.md#code-16-007) | [16-007](16-accounting.md#code-16-007) |
| `examples/mk_brep_probe.rs` | Construct a reproducible specimen | [16-008](16-accounting.md#code-16-008) | [16-008](16-accounting.md#code-16-008) |
| `examples/mk_cylinder_hidden_probe.rs` | Construct a reproducible specimen | [16-009](16-accounting.md#code-16-009) | [16-009](16-accounting.md#code-16-009) |
| `examples/mk_hidden_line_probe.rs` | Construct a reproducible specimen | [16-010](16-accounting.md#code-16-010) | [16-010](16-accounting.md#code-16-010) |
| `examples/mk_joint_probe.rs` | Construct a reproducible specimen | [16-011](16-accounting.md#code-16-011) | [16-011](16-accounting.md#code-16-011) |
| `examples/mk_mixed_solids.rs` | Construct a reproducible specimen | [16-012](16-accounting.md#code-16-012) | [16-012](16-accounting.md#code-16-012) |
| `examples/mk_plate_outline.rs` | Construct a reproducible specimen | [16-013](16-accounting.md#code-16-013) | [16-013](16-accounting.md#code-16-013) |
| `examples/mk_shade_probe.rs` | Construct a reproducible specimen | [16-014](16-accounting.md#code-16-014) | [16-014](16-accounting.md#code-16-014) |
| `examples/mk_teapot.rs` | Construct a reproducible specimen | [16-015](16-accounting.md#code-16-015) | [16-015](16-accounting.md#code-16-015) |
| `src/app/inspection/source_memory.rs` | Connect interaction with the scene | [16-020](16-accounting.md#code-16-020) | [16-020](16-accounting.md#code-16-020) |
| `src/app/scene_release.rs` | Represent the displayed scene | [16-027](16-accounting.md#code-16-027) | [21-060](21-editing.md#code-21-060) |
| `tests/README.md` | Specify behavior with a controlled experiment | [16-030](16-accounting.md#code-16-030) | [16-030](16-accounting.md#code-16-030) |
| `tests/cad-boundary-plot.py` | Specify behavior with a controlled experiment | [16-031](16-accounting.md#code-16-031) | [16-031](16-accounting.md#code-16-031) |
| `tests/cad-quality.py` | Specify behavior with a controlled experiment | [16-032](16-accounting.md#code-16-032) | [16-032](16-accounting.md#code-16-032) |
| `tests/depth/_closeup_box.py` | Specify behavior with a controlled experiment | [16-033](16-accounting.md#code-16-033) | [16-033](16-accounting.md#code-16-033) |
| `tests/depth/_count_colors.py` | Specify behavior with a controlled experiment | [16-034](16-accounting.md#code-16-034) | [16-034](16-accounting.md#code-16-034) |
| `tests/depth/_gate.sh` | Specify behavior with a controlled experiment | [16-035](16-accounting.md#code-16-035) | [16-035](16-accounting.md#code-16-035) |
| `tests/depth/_hidden_line_matrix.py` | Specify behavior with a controlled experiment | [16-036](16-accounting.md#code-16-036) | [16-036](16-accounting.md#code-16-036) |
| `tests/depth/_ink_suite.sh` | Specify behavior with a controlled experiment | [16-037](16-accounting.md#code-16-037) | [16-037](16-accounting.md#code-16-037) |
| `tests/depth/_orbit_check.py` | Specify behavior with a controlled experiment | [16-038](16-accounting.md#code-16-038) | [16-038](16-accounting.md#code-16-038) |
| `tests/depth/_probe_matrix.py` | Specify behavior with a controlled experiment | [16-039](16-accounting.md#code-16-039) | [16-039](16-accounting.md#code-16-039) |
| `tests/depth/_shade_scanline.py` | Specify behavior with a controlled experiment | [16-040](16-accounting.md#code-16-040) | [16-040](16-accounting.md#code-16-040) |
| `tests/depth/_stroke_weight.py` | Specify behavior with a controlled experiment | [16-041](16-accounting.md#code-16-041) | [16-041](16-accounting.md#code-16-041) |
| `tests/format.py` | Specify behavior with a controlled experiment | [16-042](16-accounting.md#code-16-042) | [16-042](16-accounting.md#code-16-042) |
| `tests/interaction.cjs` | Specify behavior with a controlled experiment | [16-043](16-accounting.md#code-16-043) | [16-043](16-accounting.md#code-16-043) |
| `tests/nameplate-scene.cjs` | Specify behavior with a controlled experiment | [16-044](16-accounting.md#code-16-044) | [16-044](16-accounting.md#code-16-044) |
| `tests/nameplate.cjs` | Specify behavior with a controlled experiment | [16-045](16-accounting.md#code-16-045) | [16-045](16-accounting.md#code-16-045) |
| `tests/teapot.cjs` | Specify behavior with a controlled experiment | [16-046](16-accounting.md#code-16-046) | [16-046](16-accounting.md#code-16-046) |
| `tests/text-quality.cjs` | Specify behavior with a controlled experiment | [16-047](16-accounting.md#code-16-047) | [16-047](16-accounting.md#code-16-047) |
| `tests/world-text.cjs` | Specify behavior with a controlled experiment | [16-048](16-accounting.md#code-16-048) | [16-048](16-accounting.md#code-16-048) |
| `examples/mk_selection_overlap.rs` | Construct a reproducible specimen | [17-004](17-source-presentation.md#code-17-004) | [17-004](17-source-presentation.md#code-17-004) |
| `examples/mk_stroke_joins.rs` | Construct a reproducible specimen | [17-005](17-source-presentation.md#code-17-005) | [17-005](17-source-presentation.md#code-17-005) |
| `src/app/scene_text.rs` | Represent the displayed scene | [17-013](17-source-presentation.md#code-17-013) | [21-073](21-editing.md#code-21-073) |
| `src/engine/gpu/surface_outline.rs` | Recover the visible silhouette | [17-018](17-source-presentation.md#code-17-018) | [32-023](32-colors-lighting.md#code-32-023) |
| `src/engine/gpu/surface_outline/tests.rs` | Specify behavior with a controlled experiment | [17-019](17-source-presentation.md#code-17-019) | [17-019](17-source-presentation.md#code-17-019) |
| `src/shaders/face_coverage.wgsl` | Read the GPU side of the contract | [17-020](17-source-presentation.md#code-17-020) | [17-020](17-source-presentation.md#code-17-020) |
| `src/shaders/surface_outline.wgsl` | Read the GPU side of the contract | [17-021](17-source-presentation.md#code-17-021) | [17-021](17-source-presentation.md#code-17-021) |
| `src/state/text.rs` | Coordinate application state | [17-034](17-source-presentation.md#code-17-034) | [17-034](17-source-presentation.md#code-17-034) |
| `tests/selection-overlap.py` | Specify behavior with a controlled experiment | [17-035](17-source-presentation.md#code-17-035) | [17-035](17-source-presentation.md#code-17-035) |
| `tests/stroke-joins.py` | Specify behavior with a controlled experiment | [17-036](17-source-presentation.md#code-17-036) | [17-036](17-source-presentation.md#code-17-036) |
| `examples/mk_triangle_visibility.rs` | Construct a reproducible specimen | [18-001](18-finite-visibility.md#code-18-001) | [18-001](18-finite-visibility.md#code-18-001) |
| `tests/triangle-visibility.py` | Specify behavior with a controlled experiment | [18-015](18-finite-visibility.md#code-18-015) | [18-015](18-finite-visibility.md#code-18-015) |
| `src/engine/gpu/instanced.rs` | Reuse geometry at several placements | [18a-001](18a-instancing.md#code-18a-001) | [18a-001](18a-instancing.md#code-18a-001) |
| `tests/instancing.cjs` | Specify behavior with a controlled experiment | [18a-004](18a-instancing.md#code-18a-004) | [18a-004](18a-instancing.md#code-18a-004) |
| `src/app/clipping.rs` | Connect interaction with the scene | [18b-001](18b-clipping.md#code-18b-001) | [23-009](23-geometry-commands.md#code-23-009) |
| `src/engine/gpu/clip.rs` | Cut display geometry with a plane | [18b-004](18b-clipping.md#code-18b-004) | [32-002](32-colors-lighting.md#code-32-002) |
| `src/engine/gpu/clip/pipelines.rs` | Cut display geometry with a plane | [18b-005](18b-clipping.md#code-18b-005) | [18b-005](18b-clipping.md#code-18b-005) |
| `src/engine/gpu/clip/tests.rs` | Specify behavior with a controlled experiment | [18b-006](18b-clipping.md#code-18b-006) | [18b-006](18b-clipping.md#code-18b-006) |
| `src/shaders/cap.wgsl` | Read the GPU side of the contract | [18b-009](18b-clipping.md#code-18b-009) | [18b-009](18b-clipping.md#code-18b-009) |
| `src/state/clipping.rs` | Coordinate application state | [18b-011](18b-clipping.md#code-18b-011) | [23-075](23-geometry-commands.md#code-23-075) |
| `tests/clipping-mixed.cjs` | Specify behavior with a controlled experiment | [18b-014](18b-clipping.md#code-18b-014) | [18b-014](18b-clipping.md#code-18b-014) |
| `examples/mk_sheet.rs` | Construct a reproducible specimen | [19-004](19-sheets.md#code-19-004) | [19-004](19-sheets.md#code-19-004) |
| `src/app/sheet_query.rs` | Request drawing metadata when needed | [19-018](19-sheets.md#code-19-018) | [19-018](19-sheets.md#code-19-018) |
| `src/app/walk/sheet.rs` | Translate geometry into renderable records | [19-021](19-sheets.md#code-19-021) | [19-021](19-sheets.md#code-19-021) |
| `src/state/sheet_query.rs` | Coordinate application state | [19-028](19-sheets.md#code-19-028) | [19-028](19-sheets.md#code-19-028) |
| `src/app/scene_sync.rs` | Coordinate document-to-display updates | [20-012](20-history.md#code-20-012) | [31-005](31-splitting.md#code-31-005) |
| `src/app/scene_sync/allocation.rs` | Reserve display storage | [20-013](20-history.md#code-20-013) | [20-013](20-history.md#code-20-013) |
| `src/app/scene_sync/compaction.rs` | Close holes in shared storage | [20-014](20-history.md#code-20-014) | [21-067](21-editing.md#code-21-067) |
| `src/app/scene_sync/nodes.rs` | Keep display state consistent with edits | [20-015](20-history.md#code-20-015) | [20-015](20-history.md#code-20-015) |
| `src/app/scene_sync/notes.rs` | Keep display state consistent with edits | [20-016](20-history.md#code-20-016) | [20-016](20-history.md#code-20-016) |
| `src/app/scene_sync/testing.rs` | Keep display state consistent with edits | [20-017](20-history.md#code-20-017) | [21-070](21-editing.md#code-21-070) |
| `src/app/scene_sync/tests.rs` | Specify behavior with a controlled experiment | [20-018](20-history.md#code-20-018) | [20-018](20-history.md#code-20-018) |
| `src/app/scene_sync/tombs.rs` | Remember removed display records | [20-019](20-history.md#code-20-019) | [21-071](21-editing.md#code-21-071) |
| `src/engine/gpu/upload_padding.rs` | Upload complete storage units | [20-021](20-history.md#code-20-021) | [20-021](20-history.md#code-20-021) |
| `examples/mk_extension_fixture.rs` | Construct a reproducible specimen | [21-004](21-editing.md#code-21-004) | [21-004](21-editing.md#code-21-004) |
| `src/app/cplane.rs` | Choose a plane for construction | [21-005](21-editing.md#code-21-005) | [21-005](21-editing.md#code-21-005) |
| `src/app/deform.rs` | Transform editable geometry | [21-006](21-editing.md#code-21-006) | [21-006](21-editing.md#code-21-006) |
| `src/app/edit.rs` | Commit an edit deliberately | [21-007](21-editing.md#code-21-007) | [21-007](21-editing.md#code-21-007) |
| `src/app/gesture/control.rs` | Give a drag one owner | [21-008](21-editing.md#code-21-008) | [21-008](21-editing.md#code-21-008) |
| `src/app/gesture/gizmo.rs` | Give a drag one owner | [21-009](21-editing.md#code-21-009) | [21-009](21-editing.md#code-21-009) |
| `src/app/gesture/object.rs` | Give a drag one owner | [21-013](21-editing.md#code-21-013) | [21-013](21-editing.md#code-21-013) |
| `src/app/gizmo.rs` | Constrain movement with handles | [21-014](21-editing.md#code-21-014) | [21-014](21-editing.md#code-21-014) |
| `src/app/layers.rs` | Organize document visibility | [21-031](21-editing.md#code-21-031) | [23-048](23-geometry-commands.md#code-23-048) |
| `src/app/mesh_preview.rs` | Connect interaction with the scene | [21-033](21-editing.md#code-21-033) | [21-033](21-editing.md#code-21-033) |
| `src/app/scene_instances.rs` | Represent the displayed scene | [21-058](21-editing.md#code-21-058) | [21-058](21-editing.md#code-21-058) |
| `src/app/scene_sync/editing_tests.rs` | Specify behavior with a controlled experiment | [21-068](21-editing.md#code-21-068) | [21-068](21-editing.md#code-21-068) |
| `src/app/scene_sync/preview.rs` | Keep display state consistent with edits | [21-069](21-editing.md#code-21-069) | [21-069](21-editing.md#code-21-069) |
| `src/app/snap.rs` | Choose a meaningful nearby point | [21-074](21-editing.md#code-21-074) | [21-074](21-editing.md#code-21-074) |
| `src/app/surface_preview.rs` | Connect interaction with the scene | [21-075](21-editing.md#code-21-075) | [21-075](21-editing.md#code-21-075) |
| `src/engine/gpu/clip/editing_tests.rs` | Specify behavior with a controlled experiment | [21-077](21-editing.md#code-21-077) | [21-077](21-editing.md#code-21-077) |
| `src/state/drag.rs` | Coordinate application state | [21-093](21-editing.md#code-21-093) | [31-012](31-splitting.md#code-31-012) |
| `src/state/edit.rs` | Coordinate application state | [21-094](21-editing.md#code-21-094) | [31-016](31-splitting.md#code-31-016) |
| `src/state/hydrate.rs` | Coordinate application state | [21-102](21-editing.md#code-21-102) | [30-031](30-layer-tree.md#code-30-031) |
| `src/state/number_box.rs` | Coordinate application state | [21-103](21-editing.md#code-21-103) | [25-021](25-gumball.md#code-25-021) |
| `tests/editing-extensions.cjs` | Specify behavior with a controlled experiment | [21-104](21-editing.md#code-21-104) | [21-104](21-editing.md#code-21-104) |
| `tests/large-object-dragging.cjs` | Specify behavior with a controlled experiment | [21-105](21-editing.md#code-21-105) | [21-105](21-editing.md#code-21-105) |
| `tests/live-shell-editing.cjs` | Specify behavior with a controlled experiment | [21-106](21-editing.md#code-21-106) | [21-106](21-editing.md#code-21-106) |
| `tests/source-editing.cjs` | Specify behavior with a controlled experiment | [21-107](21-editing.md#code-21-107) | [21-107](21-editing.md#code-21-107) |
| `tests/streamed-editing.cjs` | Specify behavior with a controlled experiment | [21-108](21-editing.md#code-21-108) | [21-108](21-editing.md#code-21-108) |
| `src/app/ui/mod.rs` | Build controls from state | [22-011](22-runtime-helpers.md#code-22-011) | [30-015](30-layer-tree.md#code-30-015) |
| `src/app/ui/number_box.rs` | Build controls from state | [22-012](22-runtime-helpers.md#code-22-012) | [23-069](23-geometry-commands.md#code-23-069) |
| `src/app/ui/overlay.rs` | Build controls from state | [22-013](22-runtime-helpers.md#code-22-013) | [23d-014](23d-annotate-measure.md#code-23d-014) |
| `src/app/ui/pointer.rs` | Build controls from state | [22-014](22-runtime-helpers.md#code-22-014) | [22-014](22-runtime-helpers.md#code-22-014) |
| `src/app/ui/theme.rs` | Build controls from state | [22-015](22-runtime-helpers.md#code-22-015) | [22-015](22-runtime-helpers.md#code-22-015) |
| `src/engine/gpu/ui.rs` | Composite the user interface | [22-021](22-runtime-helpers.md#code-22-021) | [22-021](22-runtime-helpers.md#code-22-021) |
| `tests/docked-workspace.cjs` | Specify behavior with a controlled experiment | [22-023](22-runtime-helpers.md#code-22-023) | [22-023](22-runtime-helpers.md#code-22-023) |
| `src/app/agent.rs` | Bridge browser text input | [23-008](23-geometry-commands.md#code-23-008) | [23-008](23-geometry-commands.md#code-23-008) |
| `src/app/command/mod.rs` | Make commands discoverable | [23-010](23-geometry-commands.md#code-23-010) | [23-010](23-geometry-commands.md#code-23-010) |
| `src/app/command/tests.rs` | Specify behavior with a controlled experiment | [23-011](23-geometry-commands.md#code-23-011) | [36-001](36-translucent-faces.md#code-36-001) |
| `src/app/command/tool.rs` | Represent an active tool | [23-012](23-geometry-commands.md#code-23-012) | [23c-002](23c-surfacing.md#code-23c-002) |
| `src/app/command/verbs/arrow.rs` | Turn a command into an action | [23-013](23-geometry-commands.md#code-23-013) | [23-013](23-geometry-commands.md#code-23-013) |
| `src/app/command/verbs/clipping_plane.rs` | Turn a command into an action | [23-014](23-geometry-commands.md#code-23-014) | [23-014](23-geometry-commands.md#code-23-014) |
| `src/app/command/verbs/close.rs` | Turn a command into an action | [23-015](23-geometry-commands.md#code-23-015) | [23-015](23-geometry-commands.md#code-23-015) |
| `src/app/command/verbs/curve.rs` | Turn a command into an action | [23-016](23-geometry-commands.md#code-23-016) | [23-016](23-geometry-commands.md#code-23-016) |
| `src/app/command/verbs/delete.rs` | Turn a command into an action | [23-017](23-geometry-commands.md#code-23-017) | [23-017](23-geometry-commands.md#code-23-017) |
| `src/app/command/verbs/escape.rs` | Turn a command into an action | [23-018](23-geometry-commands.md#code-23-018) | [23-018](23-geometry-commands.md#code-23-018) |
| `src/app/command/verbs/explode.rs` | Turn a command into an action | [23-019](23-geometry-commands.md#code-23-019) | [23-019](23-geometry-commands.md#code-23-019) |
| `src/app/command/verbs/fit.rs` | Turn a command into an action | [23-020](23-geometry-commands.md#code-23-020) | [23-020](23-geometry-commands.md#code-23-020) |
| `src/app/command/verbs/geometry.rs` | Turn a command into an action | [23-021](23-geometry-commands.md#code-23-021) | [23-021](23-geometry-commands.md#code-23-021) |
| `src/app/command/verbs/hide.rs` | Turn a command into an action | [23-022](23-geometry-commands.md#code-23-022) | [23-022](23-geometry-commands.md#code-23-022) |
| `src/app/command/verbs/line.rs` | Turn a command into an action | [23-023](23-geometry-commands.md#code-23-023) | [23-023](23-geometry-commands.md#code-23-023) |
| `src/app/command/verbs/mod.rs` | Turn a command into an action | [23-024](23-geometry-commands.md#code-23-024) | [36-002](36-translucent-faces.md#code-36-002) |
| `src/app/command/verbs/open.rs` | Turn a command into an action | [23-025](23-geometry-commands.md#code-23-025) | [23-025](23-geometry-commands.md#code-23-025) |
| `src/app/command/verbs/point.rs` | Turn a command into an action | [23-026](23-geometry-commands.md#code-23-026) | [23-026](23-geometry-commands.md#code-23-026) |
| `src/app/command/verbs/polyline.rs` | Turn a command into an action | [23-027](23-geometry-commands.md#code-23-027) | [23-027](23-geometry-commands.md#code-23-027) |
| `src/app/command/verbs/redo.rs` | Turn a command into an action | [23-028](23-geometry-commands.md#code-23-028) | [23-028](23-geometry-commands.md#code-23-028) |
| `src/app/command/verbs/save.rs` | Turn a command into an action | [23-029](23-geometry-commands.md#code-23-029) | [23-029](23-geometry-commands.md#code-23-029) |
| `src/app/command/verbs/show.rs` | Turn a command into an action | [23-030](23-geometry-commands.md#code-23-030) | [23-030](23-geometry-commands.md#code-23-030) |
| `src/app/command/verbs/undo.rs` | Turn a command into an action | [23-031](23-geometry-commands.md#code-23-031) | [23-031](23-geometry-commands.md#code-23-031) |
| `src/app/coords.rs` | Read coordinates from text | [23-032](23-geometry-commands.md#code-23-032) | [23-032](23-geometry-commands.md#code-23-032) |
| `src/app/modeling.rs` | Apply modeling operations to the document | [23-054](23-geometry-commands.md#code-23-054) | [23-054](23-geometry-commands.md#code-23-054) |
| `src/app/scene_sync/commands_tests.rs` | Specify behavior with a controlled experiment | [23-056](23-geometry-commands.md#code-23-056) | [23-056](23-geometry-commands.md#code-23-056) |
| `src/app/session_io.rs` | Save the document, not its picture | [23-057](23-geometry-commands.md#code-23-057) | [23-057](23-geometry-commands.md#code-23-057) |
| `src/app/ui/command_line.rs` | Build controls from state | [23-058](23-geometry-commands.md#code-23-058) | [23-058](23-geometry-commands.md#code-23-058) |
| `src/app/ui/command_line/tests.rs` | Specify behavior with a controlled experiment | [23-059](23-geometry-commands.md#code-23-059) | [35-004](35-attributes.md#code-35-004) |
| `src/app/ui/phone.rs` | Build controls from state | [23-070](23-geometry-commands.md#code-23-070) | [23-070](23-geometry-commands.md#code-23-070) |
| `src/state/drawing.rs` | Coordinate application state | [23-077](23-geometry-commands.md#code-23-077) | [31-014](31-splitting.md#code-31-014) |
| `src/state/tool.rs` | Coordinate application state | [23-085](23-geometry-commands.md#code-23-085) | [31-025](31-splitting.md#code-31-025) |
| `tests/command-workspace.cjs` | Specify behavior with a controlled experiment | [23-086](23-geometry-commands.md#code-23-086) | [23-086](23-geometry-commands.md#code-23-086) |
| `tests/drawing-large-scene.cjs` | Specify behavior with a controlled experiment | [23-087](23-geometry-commands.md#code-23-087) | [23-087](23-geometry-commands.md#code-23-087) |
| `src/app/command/tool/cut.rs` | Continue a command across several inputs | [23a-003](23a-tools.md#code-23a-003) | [23a-003](23a-tools.md#code-23a-003) |
| `src/app/command/tool/gather.rs` | Continue a command across several inputs | [23a-004](23a-tools.md#code-23a-004) | [23a-004](23a-tools.md#code-23a-004) |
| `src/app/command/tool/options.rs` | Give options one interpretation | [23a-005](23a-tools.md#code-23a-005) | [23a-005](23a-tools.md#code-23a-005) |
| `src/app/command/verbs/controls.rs` | Turn a command into an action | [23a-006](23a-tools.md#code-23a-006) | [23a-006](23a-tools.md#code-23a-006) |
| `src/app/command/verbs/copy.rs` | Turn a command into an action | [23a-007](23a-tools.md#code-23a-007) | [23a-007](23a-tools.md#code-23a-007) |
| `src/app/command/verbs/edge.rs` | Turn a command into an action | [23a-008](23a-tools.md#code-23a-008) | [23a-008](23a-tools.md#code-23a-008) |
| `src/app/command/verbs/extend.rs` | Turn a command into an action | [23a-009](23a-tools.md#code-23a-009) | [23a-009](23a-tools.md#code-23a-009) |
| `src/app/command/verbs/extend/reach.rs` | Turn a command into an action | [23a-010](23a-tools.md#code-23a-010) | [23a-010](23a-tools.md#code-23a-010) |
| `src/app/command/verbs/face.rs` | Turn a command into an action | [23a-011](23a-tools.md#code-23a-011) | [23a-011](23a-tools.md#code-23a-011) |
| `src/app/command/verbs/move.rs` | Turn a command into an action | [23a-016](23a-tools.md#code-23a-016) | [23a-016](23a-tools.md#code-23a-016) |
| `src/app/command/verbs/object.rs` | Turn a command into an action | [23a-017](23a-tools.md#code-23a-017) | [23a-017](23a-tools.md#code-23a-017) |
| `src/app/command/verbs/orient_3_points.rs` | Turn a command into an action | [23a-018](23a-tools.md#code-23a-018) | [23a-018](23a-tools.md#code-23a-018) |
| `src/app/command/verbs/rotate.rs` | Turn a command into an action | [23a-019](23a-tools.md#code-23a-019) | [23a-019](23a-tools.md#code-23a-019) |
| `src/app/command/verbs/scale.rs` | Turn a command into an action | [23a-020](23a-tools.md#code-23a-020) | [23a-020](23a-tools.md#code-23a-020) |
| `src/app/command/verbs/select_by_name.rs` | Turn a command into an action | [23a-021](23a-tools.md#code-23a-021) | [23a-021](23a-tools.md#code-23a-021) |
| `src/app/command/verbs/select_lasso.rs` | Turn a command into an action | [23a-022](23a-tools.md#code-23a-022) | [23a-022](23a-tools.md#code-23a-022) |
| `src/app/command/verbs/select_small.rs` | Turn a command into an action | [23a-023](23a-tools.md#code-23a-023) | [23a-023](23a-tools.md#code-23a-023) |
| `src/app/command/verbs/selecting.rs` | Turn a command into an action | [23a-024](23a-tools.md#code-23a-024) | [23a-024](23a-tools.md#code-23a-024) |
| `src/app/command/verbs/trim.rs` | Turn a command into an action | [23a-025](23a-tools.md#code-23a-025) | [23a-025](23a-tools.md#code-23a-025) |
| `src/app/command/verbs/trim/parts.rs` | Turn a command into an action | [23a-026](23a-tools.md#code-23a-026) | [23a-026](23a-tools.md#code-23a-026) |
| `tests/transforms.cjs` | Specify behavior with a controlled experiment | [23a-043](23a-tools.md#code-23a-043) | [23a-043](23a-tools.md#code-23a-043) |
| `tests/trim-extend.cjs` | Specify behavior with a controlled experiment | [23a-044](23a-tools.md#code-23a-044) | [23a-044](23a-tools.md#code-23a-044) |
| `src/app/command/tool/shape.rs` | Continue a command across several inputs | [23b-003](23b-shapes.md#code-23b-003) | [23c-003](23c-surfacing.md#code-23c-003) |
| `src/app/command/verbs/block_with_hole.rs` | Turn a command into an action | [23b-004](23b-shapes.md#code-23b-004) | [23b-004](23b-shapes.md#code-23b-004) |
| `src/app/command/verbs/box.rs` | Turn a command into an action | [23b-005](23b-shapes.md#code-23b-005) | [23b-005](23b-shapes.md#code-23b-005) |
| `src/app/command/verbs/capsule.rs` | Turn a command into an action | [23b-006](23b-shapes.md#code-23b-006) | [23b-006](23b-shapes.md#code-23b-006) |
| `src/app/command/verbs/cone.rs` | Turn a command into an action | [23b-007](23b-shapes.md#code-23b-007) | [23b-007](23b-shapes.md#code-23b-007) |
| `src/app/command/verbs/cylinder.rs` | Turn a command into an action | [23b-008](23b-shapes.md#code-23b-008) | [23b-008](23b-shapes.md#code-23b-008) |
| `src/app/command/verbs/dodecahedron.rs` | Turn a command into an action | [23b-009](23b-shapes.md#code-23b-009) | [23b-009](23b-shapes.md#code-23b-009) |
| `src/app/command/verbs/icosahedron.rs` | Turn a command into an action | [23b-010](23b-shapes.md#code-23b-010) | [23b-010](23b-shapes.md#code-23b-010) |
| `src/app/command/verbs/octahedron.rs` | Turn a command into an action | [23b-012](23b-shapes.md#code-23b-012) | [23b-012](23b-shapes.md#code-23b-012) |
| `src/app/command/verbs/pyramid.rs` | Turn a command into an action | [23b-013](23b-shapes.md#code-23b-013) | [23b-013](23b-shapes.md#code-23b-013) |
| `src/app/command/verbs/quad_sphere.rs` | Turn a command into an action | [23b-014](23b-shapes.md#code-23b-014) | [23b-014](23b-shapes.md#code-23b-014) |
| `src/app/command/verbs/sphere.rs` | Turn a command into an action | [23b-015](23b-shapes.md#code-23b-015) | [23b-015](23b-shapes.md#code-23b-015) |
| `src/app/command/verbs/tetrahedron.rs` | Turn a command into an action | [23b-016](23b-shapes.md#code-23b-016) | [23b-016](23b-shapes.md#code-23b-016) |
| `src/app/command/verbs/torus.rs` | Turn a command into an action | [23b-017](23b-shapes.md#code-23b-017) | [23b-017](23b-shapes.md#code-23b-017) |
| `src/app/command/tool/surfacing.rs` | Continue a command across several inputs | [23c-004](23c-surfacing.md#code-23c-004) | [23c-004](23c-surfacing.md#code-23c-004) |
| `src/app/command/verbs/extrude.rs` | Turn a command into an action | [23c-005](23c-surfacing.md#code-23c-005) | [23c-005](23c-surfacing.md#code-23c-005) |
| `src/app/command/verbs/loft.rs` | Turn a command into an action | [23c-006](23c-surfacing.md#code-23c-006) | [23c-006](23c-surfacing.md#code-23c-006) |
| `src/app/command/verbs/nurbs_curve_arc.rs` | Turn a command into an action | [23c-008](23c-surfacing.md#code-23c-008) | [23c-008](23c-surfacing.md#code-23c-008) |
| `src/app/command/verbs/nurbs_curve_circle.rs` | Turn a command into an action | [23c-009](23c-surfacing.md#code-23c-009) | [23c-009](23c-surfacing.md#code-23c-009) |
| `src/app/command/verbs/nurbs_curve_ellipse.rs` | Turn a command into an action | [23c-010](23c-surfacing.md#code-23c-010) | [23c-010](23c-surfacing.md#code-23c-010) |
| `src/app/command/verbs/nurbs_curve_parabola.rs` | Turn a command into an action | [23c-011](23c-surfacing.md#code-23c-011) | [23c-011](23c-surfacing.md#code-23c-011) |
| `src/app/command/verbs/nurbs_surface_4_points.rs` | Turn a command into an action | [23c-012](23c-surfacing.md#code-23c-012) | [23c-012](23c-surfacing.md#code-23c-012) |
| `src/app/command/verbs/nurbs_surface_loft.rs` | Turn a command into an action | [23c-013](23c-surfacing.md#code-23c-013) | [23c-013](23c-surfacing.md#code-23c-013) |
| `src/app/command/verbs/nurbs_surface_network.rs` | Turn a command into an action | [23c-014](23c-surfacing.md#code-23c-014) | [23c-014](23c-surfacing.md#code-23c-014) |
| `src/app/command/verbs/nurbs_surface_revolve.rs` | Turn a command into an action | [23c-015](23c-surfacing.md#code-23c-015) | [23c-015](23c-surfacing.md#code-23c-015) |
| `src/app/command/verbs/nurbs_surface_sweep1.rs` | Turn a command into an action | [23c-016](23c-surfacing.md#code-23c-016) | [23c-016](23c-surfacing.md#code-23c-016) |
| `src/app/command/verbs/nurbs_surface_sweep2.rs` | Turn a command into an action | [23c-017](23c-surfacing.md#code-23c-017) | [23c-017](23c-surfacing.md#code-23c-017) |
| `src/app/command/verbs/area.rs` | Turn a command into an action | [23d-002](23d-annotate-measure.md#code-23d-002) | [23d-002](23d-annotate-measure.md#code-23d-002) |
| `src/app/command/verbs/arrowhead.rs` | Turn a command into an action | [23d-003](23d-annotate-measure.md#code-23d-003) | [23d-003](23d-annotate-measure.md#code-23d-003) |
| `src/app/command/verbs/length.rs` | Turn a command into an action | [23d-004](23d-annotate-measure.md#code-23d-004) | [23d-004](23d-annotate-measure.md#code-23d-004) |
| `src/app/command/verbs/measure.rs` | Turn a command into an action | [23d-005](23d-annotate-measure.md#code-23d-005) | [23d-005](23d-annotate-measure.md#code-23d-005) |
| `src/app/command/verbs/measure_distance.rs` | Turn a command into an action | [23d-006](23d-annotate-measure.md#code-23d-006) | [33-002](33-contact-shadows.md#code-33-002) |
| `src/app/command/verbs/project_to_plane.rs` | Turn a command into an action | [23d-010](23d-annotate-measure.md#code-23d-010) | [23d-010](23d-annotate-measure.md#code-23d-010) |
| `src/app/command/verbs/text.rs` | Turn a command into an action | [23d-011](23d-annotate-measure.md#code-23d-011) | [23d-011](23d-annotate-measure.md#code-23d-011) |
| `src/app/command/verbs/volume.rs` | Turn a command into an action | [23d-012](23d-annotate-measure.md#code-23d-012) | [23d-012](23d-annotate-measure.md#code-23d-012) |
| `src/app/command/verbs/snap.rs` | Turn a command into an action | [24-003](24-placed-controls.md#code-24-003) | [24-003](24-placed-controls.md#code-24-003) |
| `src/engine/gpu/widget.rs` | Draw the transform handles | [25-011](25-gumball.md#code-25-011) | [25-011](25-gumball.md#code-25-011) |
| `src/engine/gpu/widget_mesh.rs` | Draw the transform handles | [25-012](25-gumball.md#code-25-012) | [25-012](25-gumball.md#code-25-012) |
| `src/shaders/widget.wgsl` | Read the GPU side of the contract | [25-013](25-gumball.md#code-25-013) | [25-013](25-gumball.md#code-25-013) |
| `src/app/hierarchy.rs` | Walk a nested document | [26-001](26-nested-panel.md#code-26-001) | [26-001](26-nested-panel.md#code-26-001) |
| `src/app/scene_sync/panel_tests.rs` | Specify behavior with a controlled experiment | [26-004](26-nested-panel.md#code-26-004) | [26-004](26-nested-panel.md#code-26-004) |
| `src/app/command/verbs/add_edge.rs` | Turn a command into an action | [30-003](30-layer-tree.md#code-30-003) | [30-003](30-layer-tree.md#code-30-003) |
| `src/app/command/verbs/add_group.rs` | Turn a command into an action | [30-004](30-layer-tree.md#code-30-004) | [30-004](30-layer-tree.md#code-30-004) |
| `src/app/command/verbs/layers.rs` | Turn a command into an action | [30-005](30-layer-tree.md#code-30-005) | [30-005](30-layer-tree.md#code-30-005) |
| `src/app/ui/graph.rs` | Build controls from state | [30-011](30-layer-tree.md#code-30-011) | [30-011](30-layer-tree.md#code-30-011) |
| `src/app/ui/layers.rs` | Build controls from state | [30-012](30-layer-tree.md#code-30-012) | [30-012](30-layer-tree.md#code-30-012) |
| `src/state/panel.rs` | Coordinate application state | [30-032](30-layer-tree.md#code-30-032) | [31-020](31-splitting.md#code-31-020) |
| `tests/layer-workspace.cjs` | Specify behavior with a controlled experiment | [30-033](30-layer-tree.md#code-30-033) | [30-033](30-layer-tree.md#code-30-033) |
| `src/app/command/verbs/split.rs` | Turn a command into an action | [31-002](31-splitting.md#code-31-002) | [31-002](31-splitting.md#code-31-002) |
| `src/app/scene_sync/split_tests.rs` | Specify behavior with a controlled experiment | [31-006](31-splitting.md#code-31-006) | [31-006](31-splitting.md#code-31-006) |
| `src/app/splitting.rs` | Replace geometry with its pieces | [31-007](31-splitting.md#code-31-007) | [31-007](31-splitting.md#code-31-007) |
| `src/state/splitting.rs` | Coordinate application state | [31-021](31-splitting.md#code-31-021) | [31-021](31-splitting.md#code-31-021) |
| `tests/splitting.cjs` | Specify behavior with a controlled experiment | [31-026](31-splitting.md#code-31-026) | [31-026](31-splitting.md#code-31-026) |
| `src/engine/gpu/ambient_warm.rs` | Prepare expensive shading work | [32-001](32-colors-lighting.md#code-32-001) | [32-001](32-colors-lighting.md#code-32-001) |
| `src/engine/gpu/ssao.rs` | Estimate nearby occlusion | [32-018](32-colors-lighting.md#code-32-018) | [32-018](32-colors-lighting.md#code-32-018) |
| `src/engine/gpu/ssao/pipelines.rs` | Estimate nearby occlusion | [32-019](32-colors-lighting.md#code-32-019) | [32-019](32-colors-lighting.md#code-32-019) |
| `src/engine/gpu/ssao/tests.rs` | Specify behavior with a controlled experiment | [32-020](32-colors-lighting.md#code-32-020) | [32-020](32-colors-lighting.md#code-32-020) |
| `src/engine/gpu/timing.rs` | Measure GPU work on the GPU | [32-024](32-colors-lighting.md#code-32-024) | [32-024](32-colors-lighting.md#code-32-024) |
| `src/shaders/ambient_composite.wgsl` | Read the GPU side of the contract | [32-025](32-colors-lighting.md#code-32-025) | [32-025](32-colors-lighting.md#code-32-025) |
| `src/shaders/ambient_depth.wgsl` | Read the GPU side of the contract | [32-026](32-colors-lighting.md#code-32-026) | [32-026](32-colors-lighting.md#code-32-026) |
| `src/shaders/ambient_geometry.wgsl` | Read the GPU side of the contract | [32-027](32-colors-lighting.md#code-32-027) | [32-027](32-colors-lighting.md#code-32-027) |
| `src/shaders/ssao.wgsl` | Read the GPU side of the contract | [32-028](32-colors-lighting.md#code-32-028) | [32-028](32-colors-lighting.md#code-32-028) |
| `tests/ambient-lighting.cjs` | Specify behavior with a controlled experiment | [32-029](32-colors-lighting.md#code-32-029) | [32-029](32-colors-lighting.md#code-32-029) |
| `src/app/command/verbs/arctic.rs` | Turn a command into an action | [33-001](33-contact-shadows.md#code-33-001) | [33-001](33-contact-shadows.md#code-33-001) |
| `src/app/command/verbs/outline.rs` | Turn a command into an action | [33-004](33-contact-shadows.md#code-33-004) | [33-004](33-contact-shadows.md#code-33-004) |
| `tests/ambient-details.cjs` | Specify behavior with a controlled experiment | [33-005](33-contact-shadows.md#code-33-005) | [33-005](33-contact-shadows.md#code-33-005) |
| `tests/ambient-floor.cjs` | Specify behavior with a controlled experiment | [33-006](33-contact-shadows.md#code-33-006) | [33-006](33-contact-shadows.md#code-33-006) |
| `tests/ambient-motion.cjs` | Specify behavior with a controlled experiment | [33-007](33-contact-shadows.md#code-33-007) | [33-007](33-contact-shadows.md#code-33-007) |
| `tests/ambient-scenes.cjs` | Specify behavior with a controlled experiment | [33-008](33-contact-shadows.md#code-33-008) | [33-008](33-contact-shadows.md#code-33-008) |
| `src/app/command/verbs/attributes.rs` | Turn a command into an action | [35-002](35-attributes.md#code-35-002) | [35-002](35-attributes.md#code-35-002) |
| `src/app/command/verbs/opacity.rs` | Turn a command into an action | [36-003](36-translucent-faces.md#code-36-003) | [36-003](36-translucent-faces.md#code-36-003) |
| `examples/check_cad_fixture.rs` | Construct a reproducible specimen | [37-002](37-command-dock.md#code-37-002) | [37-002](37-command-dock.md#code-37-002) |
| `examples/check_hidden_line_lifecycle.rs` | Construct a reproducible specimen | [37-003](37-command-dock.md#code-37-003) | [37-003](37-command-dock.md#code-37-003) |
| `examples/selftest.rs` | Construct a reproducible specimen | [37-004](37-command-dock.md#code-37-004) | [37-004](37-command-dock.md#code-37-004) |
| `src/selftest.rs` | Check a real rendered result | [37-005](37-command-dock.md#code-37-005) | [37-005](37-command-dock.md#code-37-005) |
| `src/selftest/lifecycle.rs` | Check a real rendered result | [37-006](37-command-dock.md#code-37-006) | [37-006](37-command-dock.md#code-37-006) |
| `tests/color-channels.cjs` | Specify behavior with a controlled experiment | [37-007](37-command-dock.md#code-37-007) | [37-007](37-command-dock.md#code-37-007) |
| `tests/final-review.md` | Specify behavior with a controlled experiment | [37-008](37-command-dock.md#code-37-008) | [37-008](37-command-dock.md#code-37-008) |

Supplied inputs:

- `Cargo.lock`
- `assets/text/NotoSans-Regular.subset.ttf`
- `assets/text/NotoSans-Regular.ttf`
- `assets/text/NotoSansSymbols-Regular.subset.ttf`
- `assets/text/NotoSansSymbols-Regular.ttf`
- `assets/text/NotoSansSymbols2-Regular.subset.ttf`
- `assets/text/NotoSansSymbols2-Regular.ttf`
- `assets/text/OFL.txt`
- `assets/text/README.md`
- `assets/course-boxes.pb`
- `assets/pb/view_mixed_teapot.pb`
