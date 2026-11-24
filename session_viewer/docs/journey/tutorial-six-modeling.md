# Tutorial 6 · Object creation and transformations

## Goal

Register declarative `Spec` commands and interactive multi-point `Tool`s on construction planes (`CPlane`) with object snaps (`OSnap`), live zero-history previews, and atomic undoable creation, transform, surfacing, cutting, and measurement verbs.

## Diagram

![Command `Spec` parsing into `Action` and `Tool`, `CPlane` + `OSnap` point resolution, zero-history preview, and atomic `Undo` transactions (native wgpu readback of stage deck)](../../examples/stage/tutorials-interaction/06-modeling.png)

Stage deck source: [`06-modeling.yaml`](../../examples/stage/tutorials-interaction/06-modeling.yaml).

## Prerequisites and files

Requires [Tutorial 5](tutorial-five-camera.md). Owns the command registry core (`viewer-core/src/command/{mod,tool*,verbs/mod,verbs/geometry}.rs`), `cplane`, `snap`, `edit`, `modeling`, `deform`, `splitting`, previews, and 59 modeling/analysis verbs.

Owned files in build order (`79` files):

1. `viewer-core/src/command/mod.rs`
2. `viewer-core/src/command/tool.rs`
3. `viewer-core/src/command/tool/options.rs`
4. `viewer-core/src/command/tool/choices.rs`
5. `viewer-core/src/command/tool/gather.rs`
6. `viewer-core/src/command/tool/shape.rs`
7. `viewer-core/src/command/tool/surfacing.rs`
8. `viewer-core/src/command/tool/cut.rs`
9. `viewer-core/src/command/verbs/mod.rs`
10. `viewer-core/src/command/verbs/geometry.rs`
11. `src/state/tool.rs`
12. `src/state/drawing.rs`
13. `src/state/edit.rs`
14. `src/state/splitting.rs`
15. `viewer-core/src/cplane.rs`
16. `viewer-core/src/snap.rs`
17. `viewer-core/src/edit.rs`
18. `viewer-core/src/modeling.rs`
19. `viewer-core/src/deform.rs`
20. `viewer-core/src/splitting.rs`
21. `viewer-core/src/mesh_preview.rs`
22. `viewer-core/src/surface_preview.rs`
23. `viewer-core/src/scene_sync/preview.rs`
24. `viewer-core/src/scene_sync/commands_tests.rs`
25. `viewer-core/src/scene_sync/editing_tests.rs`
26. `viewer-core/src/scene_sync/split_tests.rs`
27. `viewer-core/src/command/verbs/construction_plane.rs`
28. `viewer-core/src/command/verbs/grid_snap.rs`
29. `viewer-core/src/command/verbs/snap.rs`
30. `viewer-core/src/command/verbs/point.rs`
31. `viewer-core/src/command/verbs/line.rs`
32. `viewer-core/src/command/verbs/polyline.rs`
33. `viewer-core/src/command/verbs/curve.rs`
34. `viewer-core/src/command/verbs/arrow.rs`
35. `viewer-core/src/command/verbs/box.rs`
36. `viewer-core/src/command/verbs/sphere.rs`
37. `viewer-core/src/command/verbs/cone.rs`
38. `viewer-core/src/command/verbs/cylinder.rs`
39. `viewer-core/src/command/verbs/capsule.rs`
40. `viewer-core/src/command/verbs/torus.rs`
41. `viewer-core/src/command/verbs/pyramid.rs`
42. `viewer-core/src/command/verbs/quad_sphere.rs`
43. `viewer-core/src/command/verbs/tetrahedron.rs`
44. `viewer-core/src/command/verbs/octahedron.rs`
45. `viewer-core/src/command/verbs/dodecahedron.rs`
46. `viewer-core/src/command/verbs/icosahedron.rs`
47. `viewer-core/src/command/verbs/block_with_hole.rs`
48. `viewer-core/src/command/verbs/bounding_box.rs`
49. `viewer-core/src/command/verbs/length_bounding_box.rs`
50. `viewer-core/src/command/verbs/nurbs_curve_arc.rs`
51. `viewer-core/src/command/verbs/nurbs_curve_circle.rs`
52. `viewer-core/src/command/verbs/nurbs_curve_ellipse.rs`
53. `viewer-core/src/command/verbs/nurbs_curve_parabola.rs`
54. `viewer-core/src/command/verbs/move.rs`
55. `viewer-core/src/command/verbs/rotate.rs`
56. `viewer-core/src/command/verbs/scale.rs`
57. `viewer-core/src/command/verbs/copy.rs`
58. `viewer-core/src/command/verbs/orient_3_points.rs`
59. `viewer-core/src/command/verbs/project_to_plane.rs`
60. `viewer-core/src/command/verbs/extrude.rs`
61. `viewer-core/src/command/verbs/loft.rs`
62. `viewer-core/src/command/verbs/nurbs_surface_4_points.rs`
63. `viewer-core/src/command/verbs/nurbs_surface_loft.rs`
64. `viewer-core/src/command/verbs/nurbs_surface_network.rs`
65. `viewer-core/src/command/verbs/nurbs_surface_revolve.rs`
66. `viewer-core/src/command/verbs/nurbs_surface_sweep1.rs`
67. `viewer-core/src/command/verbs/nurbs_surface_sweep2.rs`
68. `viewer-core/src/command/verbs/trim.rs`
69. `viewer-core/src/command/verbs/trim/parts.rs`
70. `viewer-core/src/command/verbs/extend.rs`
71. `viewer-core/src/command/verbs/extend/reach.rs`
72. `viewer-core/src/command/verbs/split.rs`
73. `viewer-core/src/command/verbs/explode.rs`
74. `viewer-core/src/command/verbs/area.rs`
75. `viewer-core/src/command/verbs/length.rs`
76. `viewer-core/src/command/verbs/measure.rs`
77. `viewer-core/src/command/verbs/measure_distance.rs`
78. `viewer-core/src/command/verbs/volume.rs`
79. `viewer-core/src/command/verbs/arrowhead.rs`

## Build

### Step 1 · `viewer-core/src/command/mod.rs`

`src/app/command/mod.rs` defines `Spec`, `Action` and `Draw`, matching longest space/case-insensitive command prefixes, canonicalizing verbs/options, rejecting non-finite numbers and returning `Result<Box<dyn Action>, String>` without mutating `State`. Spanning `430` lines, this module stays free of duplicate state by exposing `Action`, `Spec`, `fn`, `Verb` without introducing UI/GPU coupling cycles. Spanning `430` lines, this module stays free of duplicate state by exposing `Action`, `Spec`, `fn`, `Verb` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/mod.rs"
```

### Step 2 · `viewer-core/src/command/tool.rs`

Defines `Tool` (`Next::More`, `Repeat`, `Done`), temporarily moving out of `State` during callbacks to satisfy exclusive mutable borrows. Spanning `184` lines, this module stays free of duplicate state by exposing `Next`, `Tool`, `Stroke`, `Overlay` without introducing UI/GPU coupling cycles. Spanning `184` lines, this module stays free of duplicate state by exposing `Next`, `Tool`, `Stroke`, `Overlay` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/tool.rs"
```

### Step 3 · `viewer-core/src/command/tool/options.rs`

Matches multi-word options case- and space-insensitively while reserving empty `Enter` and `Escape` for tool continuation. Spanning `39` lines, this module stays free of duplicate state by exposing `choices`, `find`, `compact` without introducing UI/GPU coupling cycles. Spanning `39` lines, this module stays free of duplicate state by exposing `choices`, `find`, `compact` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/tool/options.rs"
```

### Step 4 · `viewer-core/src/command/tool/choices.rs`

Turns a bare command's options into clickable dock chips that dispatch full canonical command lines (cancelling on `Enter`). Spanning `86` lines, this module stays free of duplicate state by exposing `Choices` and verified by `options_read_without_the_verb`. Spanning `86` lines, this module stays free of duplicate state by exposing `Choices` and verified by `options_read_without_the_verb`.

```rust
--8<-- "session_viewer/viewer-core/src/command/tool/choices.rs"
```

### Step 5 · `viewer-core/src/command/tool/gather.rs`

Runs `Recipe` step queues (points, numbers, distance vectors, ordered curve groups and inline options) and commits multi-object `Made` results in one transaction. Spanning `1031` lines, this module stays free of duplicate state by exposing `Step`, `Answer`, `Input`, `along` and verified by `a_curves_step_keeps_click_order_and_toggles` (+6 more tests). Spanning `1031` lines, this module stays free of duplicate state by exposing `Step`, `Answer`, `Input`, `along` and verified by `a_curves_step_keeps_click_order_and_toggles` (+6 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/tool/gather.rs"
```

### Step 6 · `viewer-core/src/command/tool/shape.rs`

Provides the shared local-frame primitive builder (`Brep`/`Mesh` toggle, rubber-band distances and signed-height upright frame reversal). Spanning `983` lines, this module stays free of duplicate state by exposing `MESH_QUALITY`, `RING`, `BREP_MESH`, `MESH_ONLY` and verified by `frames_are_right_handed_on_every_plane` (+4 more tests). Spanning `983` lines, this module stays free of duplicate state by exposing `MESH_QUALITY`, `RING`, `BREP_MESH`, `MESH_ONLY` and verified by `frames_are_right_handed_on_every_plane` (+4 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/tool/shape.rs"
```

### Step 7 · `viewer-core/src/command/tool/surfacing.rs`

Checks scale-relative Newell area and planarity, aligns open/closed section directions and seams, and requires `>= 3` sections for closed lofts. Spanning `292` lines, this module stays free of duplicate state by exposing `loop_normal`, `diagonal`, `planar`, `align_sections` and verified by `world_curve_applies_the_row_placement` (+4 more tests). Spanning `292` lines, this module stays free of duplicate state by exposing `loop_normal`, `diagonal`, `planar`, `align_sections` and verified by `world_curve_applies_the_row_placement` (+4 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/tool/surfacing.rs"
```

### Step 8 · `viewer-core/src/command/tool/cut.rs`

Converts lines/polylines to degree-`1` NURBS and transforms cutting planes/rays by `3` points / `2` points so non-uniform target scales stay exact. Spanning `240` lines, this module stays free of duplicate state by exposing `Cutter`, `moved`, `as_curve`, `as_plane` and verified by `planes_come_from_flat_objects` (+4 more tests). Spanning `240` lines, this module stays free of duplicate state by exposing `Cutter`, `moved`, `as_curve`, `as_plane` and verified by `planes_come_from_flat_objects` (+4 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/tool/cut.rs"
```

### Step 9 · `viewer-core/src/command/verbs/mod.rs`

`src/app/command/verbs/mod.rs` expands `verbs!` into `REGISTRY` so every command registers completion, option chips, help and parsing in one place without editing the dock widget. Spanning `106` lines, this module stays free of duplicate state by exposing `REGISTRY` without introducing UI/GPU coupling cycles. Spanning `106` lines, this module stays free of duplicate state by exposing `REGISTRY` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/mod.rs"
```

### Step 10 · `viewer-core/src/command/verbs/geometry.rs`

`src/app/command/verbs/geometry.rs` (`Create::run`) validates finite coordinates within scene bounds, invokes the verb's constructor, commits the created object to `Scene`, syncs GPU rows, and selects the new row (leaving document and row counts untouched on error). Spanning `211` lines, this module stays free of duplicate state by exposing `Draw`, `open`, `geometry`, `Create` and verified by `a_drawing_verb_registers_in_one_file`. Spanning `211` lines, this module stays free of duplicate state by exposing `Draw`, `open`, `geometry`, `Create` and verified by `a_drawing_verb_registers_in_one_file`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/geometry.rs"
```

### Step 11 · `src/state/tool.rs`

Snapshots original row placements before tool previews so `Escape` restores exact GPU placements with zero history entries. Spanning `486` lines, this module stays free of duplicate state by exposing `start_tool`, `open_tool`, `tool_picks`, `tool_click` without introducing UI/GPU coupling cycles. Spanning `486` lines, this module stays free of duplicate state by exposing `start_tool`, `open_tool`, `tool_picks`, `tool_click` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/state/tool.rs"
```

### Step 12 · `src/state/drawing.rs`

Owns `Draft` points, hover preview and `CPlane`, serializing completed picks with explicit `w` world prefixes into `run_command` so typed and picked geometry share one undo path. Spanning `885` lines, this module stays free of duplicate state by exposing `Draft`, `drafting`, `drawing_command`, `ask_points` and verified by `a_draft_switches_to_a_tilted_plane_and_draws_on_it` (+1 more tests). Spanning `885` lines, this module stays free of duplicate state by exposing `Draft`, `drafting`, `drawing_command`, `ask_points` and verified by `a_draft_switches_to_a_tilted_plane_and_draws_on_it` (+1 more tests).

```rust
--8<-- "session_viewer/src/state/drawing.rs"
```

### Step 13 · `src/state/edit.rs`

Coordinates command dispatch, gumball drag preview/restore, whole-object vs subobject commits and single-click `number_box` activation. Spanning `1267` lines, this module stays free of duplicate state by exposing `GizmoDrag`, `place_gizmo`, `begin_gizmo_with_radius`, `drag_gizmo` and verified by `a_css_pixel_is_worth_more_world_on_a_denser_display` (+1 more tests). Spanning `1267` lines, this module stays free of duplicate state by exposing `GizmoDrag`, `place_gizmo`, `begin_gizmo_with_radius`, `drag_gizmo` and verified by `a_css_pixel_is_worth_more_world_on_a_denser_display` (+1 more tests).

```rust
--8<-- "session_viewer/src/state/edit.rs"
```

### Step 14 · `src/state/splitting.rs`

`app/splitting.rs` validates `1..64` unlocked cutters transformed into the target's frame; `state/splitting.rs` retains pending target/cutters across picks and restores highlights on error; `verbs/split.rs` confirms or starts `split_command` while preserving the pending split. Spanning `193` lines, this module stays free of duplicate state by exposing `Pending`, `split_status`, `split_command`, `cancel_split` without introducing UI/GPU coupling cycles. Spanning `193` lines, this module stays free of duplicate state by exposing `Pending`, `split_status`, `split_command`, `cancel_split` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/state/splitting.rs"
```

### Step 15 · `viewer-core/src/cplane.rs`

`src/app/cplane.rs` (`CPlane::from_3_points`) builds right-handed orthonormal frames (rejecting collinear triples) and maps square-on named views to their facing planes with grazing-ray fallbacks. Spanning `359` lines, this module stays free of duplicate state by exposing `CPlane`, `from_3_points`, `origin`, `axes` and verified by `view_mode_draws_on_the_ground_unless_square_on` (+6 more tests). Spanning `359` lines, this module stays free of duplicate state by exposing `CPlane`, `from_3_points`, `origin`, `axes` and verified by `view_mode_draws_on_the_ground_unless_square_on` (+6 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/cplane.rs"
```

### Step 16 · `viewer-core/src/snap.rs`

`src/app/snap.rs` evaluates prioritized `SnapKind` candidates (endpoints, vertices, midpoints, centres, perpendiculars, near curves) and rounds local `CPlane` `(u, v)` grid coordinates under per-move CSS-pixel and time budgets. Spanning `919` lines, this module stays free of duplicate state by exposing `SnapKind`, `END`, `NEAR`, `MID` and verified by `an_open_polyline_offers_ends_vertices_and_midpoints` (+13 more tests). Spanning `919` lines, this module stays free of duplicate state by exposing `SnapKind`, `END`, `NEAR`, `MID` and verified by `an_open_polyline_offers_ends_vertices_and_midpoints` (+13 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/snap.rs"
```

### Step 17 · `viewer-core/src/edit.rs`

`src/app/edit.rs` (`Scene::transform_rows`) converts world-space `Xform` deltas into each writable object's local parent frame, detaching shared `Rc<Session>` copies on write and skipping selected descendants of selected ancestors. Spanning `1236` lines, this module stays free of duplicate state by exposing `transform_rows`, `with_descendants`, `local_xform_of`, `set_row_xform` and verified by `group_transform_undo_restores_every_member_in_one_document` (+14 more tests). Spanning `1236` lines, this module stays free of duplicate state by exposing `transform_rows`, `with_descendants`, `local_xform_of`, `set_row_xform` and verified by `group_transform_undo_restores_every_member_in_one_document` (+14 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/edit.rs"
```

### Step 18 · `viewer-core/src/modeling.rs`

Inserts constructed `Made` objects into the active editable layer (creating `Created` on demand) and compensates for parent/document transforms. Spanning `351` lines, this module stays free of duplicate state by exposing `MAX_POINTS`, `PointBuilder`, `check_point`, `Interval` and verified by `creation_undo_redo_and_invalid_input` (+3 more tests). Spanning `351` lines, this module stays free of duplicate state by exposing `MAX_POINTS`, `PointBuilder`, `check_point`, `Interval` and verified by `creation_undo_redo_and_invalid_input` (+3 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/modeling.rs"
```

### Step 19 · `viewer-core/src/deform.rs`

Deforms selected mesh vertices, first-appearance edges (including hole rims) and checked `(u, v)` NURBS control points without altering whole-object placement. Spanning `576` lines, this module stays free of duplicate state by exposing `Target`, `selected`, `mesh_keys`, `points` and verified by `rational_curve_control_moves_without_changing_weights_or_other_controls` (+4 more tests). Spanning `576` lines, this module stays free of duplicate state by exposing `Target`, `selected`, `mesh_keys`, `points` and verified by `rational_curve_control_moves_without_changing_weights_or_other_controls` (+4 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/deform.rs"
```

### Step 20 · `viewer-core/src/splitting.rs`

`app/splitting.rs` validates `1..64` unlocked cutters transformed into the target's frame; `state/splitting.rs` retains pending target/cutters across picks and restores highlights on error; `verbs/split.rs` confirms or starts `split_command` while preserving the pending split. Spanning `405` lines, this module stays free of duplicate state by exposing `is_cutter`, `face_index`, `split_rows` and verified by `split_preserves_tree_placement_and_other_shared_documents_and_undo` (+1 more tests). Spanning `405` lines, this module stays free of duplicate state by exposing `is_cutter`, `face_index`, `split_rows` and verified by `split_preserves_tree_placement_and_other_shared_documents_and_undo` (+1 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/splitting.rs"
```

### Step 21 · `viewer-core/src/mesh_preview.rs`

Maps GPU arena vertices, pipes and markers back to source mesh vertex keys for in-place flat-shaded drag previews. Spanning `325` lines, this module stays free of duplicate state by exposing `MeshPreview`, `Gesture`, `mesh`, `capture` and verified by `large_mesh_gesture_only_patches_local_neighborhood`. Spanning `325` lines, this module stays free of duplicate state by exposing `MeshPreview`, `Gesture`, `mesh`, `capture` and verified by `large_mesh_gesture_only_patches_local_neighborhood`.

```rust
--8<-- "session_viewer/viewer-core/src/mesh_preview.rs"
```

### Step 22 · `viewer-core/src/surface_preview.rs`

Retains per-vertex `(surface_idx, u, v)` and pipe endpoint indices so CV drags re-evaluate only modified NURBS faces in place. Spanning `377` lines, this module stays free of duplicate state by exposing `SurfacePreview`, `capture`, `evaluate`, `allocated_bytes` and verified by `coincident_boundary_samples_keep_their_own_uv_after_edit` (+2 more tests). Spanning `377` lines, this module stays free of duplicate state by exposing `SurfacePreview`, `capture`, `evaluate`, `allocated_bytes` and verified by `coincident_boundary_samples_keep_their_own_uv_after_edit` (+2 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/surface_preview.rs"
```

### Step 23 · `viewer-core/src/scene_sync/preview.rs`

Captures single-object vertex/pipe spans during interactive drags so surface/mesh edits patch GPU buffers in place without full rewalks. Spanning `119` lines, this module stays free of duplicate state by exposing `Previews`, `capture_preview`, `mesh_preview`, `drop_preview` without introducing UI/GPU coupling cycles. Spanning `119` lines, this module stays free of duplicate state by exposing `Previews`, `capture_preview`, `mesh_preview`, `drop_preview` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/scene_sync/preview.rs"
```

### Step 24 · `viewer-core/src/scene_sync/commands_tests.rs`

Bounds `200`-frame growing-curve previews to `<= 16` reallocations and checks `100,000`-object vs `1,000`-object incremental edit scaling. Spanning `635` lines, this module stays free of duplicate state by exposing `drag` and verified by `notes_come_from_the_committed_transaction` (+15 more tests). Spanning `635` lines, this module stays free of duplicate state by exposing `drag` and verified by `notes_come_from_the_committed_transaction` (+15 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/scene_sync/commands_tests.rs"
```

### Step 25 · `viewer-core/src/scene_sync/editing_tests.rs`

Verifies `100`-point cloud tomb burial/unbury, forced tomb-cap release, post-compaction `Undo` rebuild and multi-document single-step deletion. Spanning `205` lines, this module stays free of duplicate state by wiring internal module helpers and verified by `a_deleted_cloud_waits_in_its_tomb` (+6 more tests). Spanning `205` lines, this module stays free of duplicate state by wiring internal module helpers and verified by `a_deleted_cloud_waits_in_its_tomb` (+6 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/scene_sync/editing_tests.rs"
```

### Step 26 · `viewer-core/src/scene_sync/split_tests.rs`

Verifies incremental row synchronization, cutter preservation and `Undo` restoration across BRep/mesh split edits. Spanning `454` lines, this module stays free of duplicate state by wiring internal module helpers and verified by `incremental_matches_fresh_oracle` (+8 more tests). Spanning `454` lines, this module stays free of duplicate state by wiring internal module helpers and verified by `incremental_matches_fresh_oracle` (+8 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/scene_sync/split_tests.rs"
```

### Step 27 · `viewer-core/src/command/verbs/construction_plane.rs`

Switches between `XY`, `XZ`, `YZ`, `3Point` and `View` planes while preserving active drawing `Draft` points. Spanning `230` lines, this module stays free of duplicate state by exposing `NAME` and verified by `construction_plane_parses_its_planes`. Spanning `230` lines, this module stays free of duplicate state by exposing `NAME` and verified by `construction_plane_parses_its_planes`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/construction_plane.rs"
```

### Step 28 · `viewer-core/src/command/verbs/grid_snap.rs`

`snap.rs` toggles master `OSnap` or individual snap bits; `grid_snap.rs` updates positive construction-grid spacing while preserving any active drawing `Draft`. Spanning `93` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `grid_snap_rounds_along_the_plane`. Spanning `93` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `grid_snap_rounds_along_the_plane`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/grid_snap.rs"
```

### Step 29 · `viewer-core/src/command/verbs/snap.rs`

`snap.rs` toggles master `OSnap` or individual snap bits; `grid_snap.rs` updates positive construction-grid spacing while preserving any active drawing `Draft`. Spanning `68` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `68` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/snap.rs"
```

### Step 30 · `viewer-core/src/command/verbs/point.rs`

Declares a `1`-point `Draw` spec wrapping the resolved world `Point` in `Rc` for immediate scene insertion. Spanning `22` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `22` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/point.rs"
```

### Step 31 · `viewer-core/src/command/verbs/line.rs`

Requires `2` endpoints separated by `> 1e-12`, rejecting coincident endpoints before constructing `session_rust::Line`. Spanning `37` lines, this module stays free of duplicate state by exposing `segment` without introducing UI/GPU coupling cycles. Spanning `37` lines, this module stays free of duplicate state by exposing `segment` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/line.rs"
```

### Step 32 · `viewer-core/src/command/verbs/polyline.rs`

Collects variable point counts (`Enter` to finish) and exposes planar rectangle and polygon sub-modes through the `Tool` path. Spanning `36` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `36` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/polyline.rs"
```

### Step 33 · `viewer-core/src/command/verbs/curve.rs`

Builds clamped NURBS curves (`degree <= 3`) from control points, switching to periodic closed construction when `>= 4` points repeat the start. Spanning `73` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `a_curve_ending_on_its_start_is_closed`. Spanning `73` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `a_curve_ending_on_its_start_is_closed`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/curve.rs"
```

### Step 34 · `viewer-core/src/command/verbs/arrow.rs`

Reuses the `2`-point `Line` validation and sets the end-arrowhead display attribute on the constructed curve. Spanning `28` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `28` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/arrow.rs"
```

### Step 35 · `viewer-core/src/command/verbs/box.rs`

`box.rs` treats picked corners as symmetric half-widths and translates `abs(h)` by `h/2` for negative heights; `block_with_hole.rs` adds a central through-hole bounded below half the shorter base side; `pyramid.rs` derives square edge `r * sqrt(2)` rotated `-45°` with upright signed height. Spanning `217` lines, this module stays free of duplicate state by exposing `ask`, `read`, `width_of`, `outline` and verified by `typed_sizes_and_a_corner_give_the_box` (+2 more tests). Spanning `217` lines, this module stays free of duplicate state by exposing `ask`, `read`, `width_of`, `outline` and verified by `typed_sizes_and_a_corner_give_the_box` (+2 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/box.rs"
```

### Step 36 · `viewer-core/src/command/verbs/sphere.rs`

`sphere.rs` aligns poles with the `CPlane` normal; `quad_sphere.rs` welds `6` `8×8` patches into a `384`-quad mesh; `cylinder.rs` and `cone.rs` flip frames on negative height for positive kernel solids; `capsule.rs` requires `abs(h) > 2r` across hemisphere caps (`Mesh` only); `torus.rs` enforces `0 < minor < major`. Spanning `79` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_sphere_has_its_poles_on_the_normal`. Spanning `79` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_sphere_has_its_poles_on_the_normal`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/sphere.rs"
```

### Step 37 · `viewer-core/src/command/verbs/cone.rs`

`sphere.rs` aligns poles with the `CPlane` normal; `quad_sphere.rs` welds `6` `8×8` patches into a `384`-quad mesh; `cylinder.rs` and `cone.rs` flip frames on negative height for positive kernel solids; `capsule.rs` requires `abs(h) > 2r` across hemisphere caps (`Mesh` only); `torus.rs` enforces `0 < minor < major`. Spanning `90` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_apex_is_on_the_normal`. Spanning `90` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_apex_is_on_the_normal`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/cone.rs"
```

### Step 38 · `viewer-core/src/command/verbs/cylinder.rs`

`sphere.rs` aligns poles with the `CPlane` normal; `quad_sphere.rs` welds `6` `8×8` patches into a `384`-quad mesh; `cylinder.rs` and `cone.rs` flip frames on negative height for positive kernel solids; `capsule.rs` requires `abs(h) > 2r` across hemisphere caps (`Mesh` only); `torus.rs` enforces `0 < minor < major`. Spanning `129` lines, this module stays free of duplicate state by exposing `ask`, `read` and verified by `the_cylinder_stands_on_its_base`. Spanning `129` lines, this module stays free of duplicate state by exposing `ask`, `read` and verified by `the_cylinder_stands_on_its_base`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/cylinder.rs"
```

### Step 39 · `viewer-core/src/command/verbs/capsule.rs`

`sphere.rs` aligns poles with the `CPlane` normal; `quad_sphere.rs` welds `6` `8×8` patches into a `384`-quad mesh; `cylinder.rs` and `cone.rs` flip frames on negative height for positive kernel solids; `capsule.rs` requires `abs(h) > 2r` across hemisphere caps (`Mesh` only); `torus.rs` enforces `0 < minor < major`. Spanning `111` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_capsule_spans_the_height`. Spanning `111` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_capsule_spans_the_height`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/capsule.rs"
```

### Step 40 · `viewer-core/src/command/verbs/torus.rs`

`sphere.rs` aligns poles with the `CPlane` normal; `quad_sphere.rs` welds `6` `8×8` patches into a `384`-quad mesh; `cylinder.rs` and `cone.rs` flip frames on negative height for positive kernel solids; `capsule.rs` requires `abs(h) > 2r` across hemisphere caps (`Mesh` only); `torus.rs` enforces `0 < minor < major`. Spanning `130` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_torus_spans_its_radii`. Spanning `130` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_torus_spans_its_radii`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/torus.rs"
```

### Step 41 · `viewer-core/src/command/verbs/pyramid.rs`

`box.rs` treats picked corners as symmetric half-widths and translates `abs(h)` by `h/2` for negative heights; `block_with_hole.rs` adds a central through-hole bounded below half the shorter base side; `pyramid.rs` derives square edge `r * sqrt(2)` rotated `-45°` with upright signed height. Spanning `148` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `a_clicked_corner_lands_on_the_click`. Spanning `148` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `a_clicked_corner_lands_on_the_click`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/pyramid.rs"
```

### Step 42 · `viewer-core/src/command/verbs/quad_sphere.rs`

`sphere.rs` aligns poles with the `CPlane` normal; `quad_sphere.rs` welds `6` `8×8` patches into a `384`-quad mesh; `cylinder.rs` and `cone.rs` flip frames on negative height for positive kernel solids; `capsule.rs` requires `abs(h) > 2r` across hemisphere caps (`Mesh` only); `torus.rs` enforces `0 < minor < major`. Spanning `71` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_quad_sphere_is_closed_and_round`. Spanning `71` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_quad_sphere_is_closed_and_round`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/quad_sphere.rs"
```

### Step 43 · `viewer-core/src/command/verbs/tetrahedron.rs`

Each Platonic verb (`tetrahedron.rs`, `octahedron.rs`, `icosahedron.rs`, `dodecahedron.rs`) scales edge length from unit circumradius, caches `OnceLock` wireframe previews, and rejects `Brep` radii `< 0.001`. Spanning `72` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_corners_lie_at_the_radius`. Spanning `72` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_corners_lie_at_the_radius`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/tetrahedron.rs"
```

### Step 44 · `viewer-core/src/command/verbs/octahedron.rs`

Each Platonic verb (`tetrahedron.rs`, `octahedron.rs`, `icosahedron.rs`, `dodecahedron.rs`) scales edge length from unit circumradius, caches `OnceLock` wireframe previews, and rejects `Brep` radii `< 0.001`. Spanning `72` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_corners_lie_at_the_radius`. Spanning `72` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_corners_lie_at_the_radius`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/octahedron.rs"
```

### Step 45 · `viewer-core/src/command/verbs/dodecahedron.rs`

Each Platonic verb (`tetrahedron.rs`, `octahedron.rs`, `icosahedron.rs`, `dodecahedron.rs`) scales edge length from unit circumradius, caches `OnceLock` wireframe previews, and rejects `Brep` radii `< 0.001`. Spanning `72` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_corners_lie_at_the_radius`. Spanning `72` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_corners_lie_at_the_radius`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/dodecahedron.rs"
```

### Step 46 · `viewer-core/src/command/verbs/icosahedron.rs`

Each Platonic verb (`tetrahedron.rs`, `octahedron.rs`, `icosahedron.rs`, `dodecahedron.rs`) scales edge length from unit circumradius, caches `OnceLock` wireframe previews, and rejects `Brep` radii `< 0.001`. Spanning `72` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_corners_lie_at_the_radius`. Spanning `72` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_corners_lie_at_the_radius`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/icosahedron.rs"
```

### Step 47 · `viewer-core/src/command/verbs/block_with_hole.rs`

`box.rs` treats picked corners as symmetric half-widths and translates `abs(h)` by `h/2` for negative heights; `block_with_hole.rs` adds a central through-hole bounded below half the shorter base side; `pyramid.rs` derives square edge `r * sqrt(2)` rotated `-45°` with upright signed height. Spanning `121` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_block_stands_on_the_plane`. Spanning `121` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_block_stands_on_the_plane`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/block_with_hole.rs"
```

### Step 48 · `viewer-core/src/command/verbs/bounding_box.rs`

`measure.rs` resolves placed geometry (requesting hydration if released) and tags temporary marks with `row_revision`; `measure_distance.rs` reports Euclidean + `Δx/Δy/Δz` without document edits; `length.rs` transforms curves into world space first for non-uniform scale accuracy; `area.rs` sums placed mesh/BRep triangles or `8×8` Gauss-quadrature NURBS knot spans; `volume.rs` sums origin-shifted signed tetrahedra on closed solids only; `bounding_box.rs` commits an axis-aligned world box while `length_bounding_box.rs` only displays `X × Y × Z` dimensions. Spanning `115` lines, this module stays free of duplicate state by exposing `selected_bounds` and verified by `boxes_keep_world_position_and_flat_extents` (+1 more tests). Spanning `115` lines, this module stays free of duplicate state by exposing `selected_bounds` and verified by `boxes_keep_world_position_and_flat_extents` (+1 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/bounding_box.rs"
```

### Step 49 · `viewer-core/src/command/verbs/length_bounding_box.rs`

`measure.rs` resolves placed geometry (requesting hydration if released) and tags temporary marks with `row_revision`; `measure_distance.rs` reports Euclidean + `Δx/Δy/Δz` without document edits; `length.rs` transforms curves into world space first for non-uniform scale accuracy; `area.rs` sums placed mesh/BRep triangles or `8×8` Gauss-quadrature NURBS knot spans; `volume.rs` sums origin-shifted signed tetrahedra on closed solids only; `bounding_box.rs` commits an axis-aligned world box while `length_bounding_box.rs` only displays `X × Y × Z` dimensions. Spanning `39` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `39` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/length_bounding_box.rs"
```

### Step 50 · `viewer-core/src/command/verbs/nurbs_curve_arc.rs`

`nurbs_curve_circle.rs` places an exact rational circle on `CPlane`; `nurbs_curve_ellipse.rs` aligns major axis to a picked point and measures perpendicular minor reach; `nurbs_curve_arc.rs` (`Center` or `3Points`) splits sweeps `<= 90°` with middle weight `cos(theta/2)`; `nurbs_curve_parabola.rs` sets middle control `2*apex - 0.5*(a + b)` so `C(0.5) == apex`. Spanning `330` lines, this module stays free of duplicate state by exposing `arc` and verified by `the_arc_is_exactly_round` (+1 more tests). Spanning `330` lines, this module stays free of duplicate state by exposing `arc` and verified by `the_arc_is_exactly_round` (+1 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/nurbs_curve_arc.rs"
```

### Step 51 · `viewer-core/src/command/verbs/nurbs_curve_circle.rs`

`nurbs_curve_circle.rs` places an exact rational circle on `CPlane`; `nurbs_curve_ellipse.rs` aligns major axis to a picked point and measures perpendicular minor reach; `nurbs_curve_arc.rs` (`Center` or `3Points`) splits sweeps `<= 90°` with middle weight `cos(theta/2)`; `nurbs_curve_parabola.rs` sets middle control `2*apex - 0.5*(a + b)` so `C(0.5) == apex`. Spanning `91` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_circle_lies_in_the_plane`. Spanning `91` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_circle_lies_in_the_plane`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/nurbs_curve_circle.rs"
```

### Step 52 · `viewer-core/src/command/verbs/nurbs_curve_ellipse.rs`

`nurbs_curve_circle.rs` places an exact rational circle on `CPlane`; `nurbs_curve_ellipse.rs` aligns major axis to a picked point and measures perpendicular minor reach; `nurbs_curve_arc.rs` (`Center` or `3Points`) splits sweeps `<= 90°` with middle weight `cos(theta/2)`; `nurbs_curve_parabola.rs` sets middle control `2*apex - 0.5*(a + b)` so `C(0.5) == apex`. Spanning `114` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_ellipse_follows_its_axes`. Spanning `114` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_ellipse_follows_its_axes`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/nurbs_curve_ellipse.rs"
```

### Step 53 · `viewer-core/src/command/verbs/nurbs_curve_parabola.rs`

`nurbs_curve_circle.rs` places an exact rational circle on `CPlane`; `nurbs_curve_ellipse.rs` aligns major axis to a picked point and measures perpendicular minor reach; `nurbs_curve_arc.rs` (`Center` or `3Points`) splits sweeps `<= 90°` with middle weight `cos(theta/2)`; `nurbs_curve_parabola.rs` sets middle control `2*apex - 0.5*(a + b)` so `C(0.5) == apex`. Spanning `152` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_parabola_passes_its_three_points`. Spanning `152` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_parabola_passes_its_three_points`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/nurbs_curve_parabola.rs"
```

### Step 54 · `viewer-core/src/command/verbs/move.rs`

`move.rs` previews and commits `2`-point or typed translations; `rotate.rs` computes `atan2` angles around a picked `CPlane` pivot or typed world axis; `scale.rs` applies `1D`/`2D`/`3D` orthonormal reach ratios; `copy.rs` loops with `Next::Repeat` minting fresh GUIDs in parent frames; `orient_3_points.rs` composes rigid `6`-point orthonormal frame alignment without scaling. Spanning `125` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `moving_previews_the_offset_from_the_base_point`. Spanning `125` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `moving_previews_the_offset_from_the_base_point`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/move.rs"
```

### Step 55 · `viewer-core/src/command/verbs/rotate.rs`

`move.rs` previews and commits `2`-point or typed translations; `rotate.rs` computes `atan2` angles around a picked `CPlane` pivot or typed world axis; `scale.rs` applies `1D`/`2D`/`3D` orthonormal reach ratios; `copy.rs` loops with `Next::Repeat` minting fresh GUIDs in parent frames; `orient_3_points.rs` composes rigid `6`-point orthonormal frame alignment without scaling. Spanning `242` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `a_turn_is_counter_clockwise_in_the_construction_plane` (+1 more tests). Spanning `242` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `a_turn_is_counter_clockwise_in_the_construction_plane` (+1 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/rotate.rs"
```

### Step 56 · `viewer-core/src/command/verbs/scale.rs`

`move.rs` previews and commits `2`-point or typed translations; `rotate.rs` computes `atan2` angles around a picked `CPlane` pivot or typed world axis; `scale.rs` applies `1D`/`2D`/`3D` orthonormal reach ratios; `copy.rs` loops with `Next::Repeat` minting fresh GUIDs in parent frames; `orient_3_points.rs` composes rigid `6`-point orthonormal frame alignment without scaling. Spanning `323` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `each_mode_scales_its_own_directions` (+2 more tests). Spanning `323` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `each_mode_scales_its_own_directions` (+2 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/scale.rs"
```

### Step 57 · `viewer-core/src/command/verbs/copy.rs`

`move.rs` previews and commits `2`-point or typed translations; `rotate.rs` computes `atan2` angles around a picked `CPlane` pivot or typed world axis; `scale.rs` applies `1D`/`2D`/`3D` orthonormal reach ratios; `copy.rs` loops with `Next::Repeat` minting fresh GUIDs in parent frames; `orient_3_points.rs` composes rigid `6`-point orthonormal frame alignment without scaling. Spanning `371` lines, this module stays free of duplicate state by exposing `copy_rows` and verified by `a_copy_keeps_parent_name_and_colours_and_undoes_alone` (+2 more tests). Spanning `371` lines, this module stays free of duplicate state by exposing `copy_rows` and verified by `a_copy_keeps_parent_name_and_colours_and_undoes_alone` (+2 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/copy.rs"
```

### Step 58 · `viewer-core/src/command/verbs/orient_3_points.rs`

`move.rs` previews and commits `2`-point or typed translations; `rotate.rs` computes `atan2` angles around a picked `CPlane` pivot or typed world axis; `scale.rs` applies `1D`/`2D`/`3D` orthonormal reach ratios; `copy.rs` loops with `Next::Repeat` minting fresh GUIDs in parent frames; `orient_3_points.rs` composes rigid `6`-point orthonormal frame alignment without scaling. Spanning `274` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `three_references_land_on_three_targets` (+3 more tests). Spanning `274` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `three_references_land_on_three_targets` (+3 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/orient_3_points.rs"
```

### Step 59 · `viewer-core/src/command/verbs/project_to_plane.rs`

`project_to_plane.rs` flattens points, curves and meshes via `place^-1 * P_world * place`, clearing invalidated mesh normals and skipping degenerate collapses. Spanning `468` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_planes_and_points_parse` (+4 more tests). Spanning `468` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `the_planes_and_points_parse` (+4 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/project_to_plane.rs"
```

### Step 60 · `viewer-core/src/command/verbs/extrude.rs`

`explode.rs` replaces leaf objects (refusing group parents) with bounded child parts in one undo step; `extrude.rs` caps closed planar profiles topologically along shared top, bottom and seam edges while leaving open/bent profiles uncapped. Spanning `521` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `polylines_become_boxes_and_sheets` (+3 more tests). Spanning `521` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `polylines_become_boxes_and_sheets` (+3 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/extrude.rs"
```

### Step 61 · `viewer-core/src/command/verbs/loft.rs`

`nurbs_surface_4_points.rs` orders bilinear controls `(a, d, b, c)`; `loft.rs` (`Open`/`Closed`) and `nurbs_surface_loft.rs` (`Smooth` deg `3` / `Straight` deg `1`) loft aligned sections; `nurbs_surface_revolve.rs` rotates profiles `(0, 360]`° right-handedly; `nurbs_surface_sweep1.rs` blends compatible knots along perpendicular rail stations; `nurbs_surface_sweep2.rs` aligns two rails and shapes by endpoint proximity; `nurbs_surface_network.rs` `2`-colours `4..32` crossing curves into Coons or sampled Gordon patches. Spanning `111` lines, this module stays free of duplicate state by exposing `no_axis` and verified by `three_arcs_loft_through_the_middle_section` (+1 more tests). Spanning `111` lines, this module stays free of duplicate state by exposing `no_axis` and verified by `three_arcs_loft_through_the_middle_section` (+1 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/loft.rs"
```

### Step 62 · `viewer-core/src/command/verbs/nurbs_surface_4_points.rs`

`nurbs_surface_4_points.rs` orders bilinear controls `(a, d, b, c)`; `loft.rs` (`Open`/`Closed`) and `nurbs_surface_loft.rs` (`Smooth` deg `3` / `Straight` deg `1`) loft aligned sections; `nurbs_surface_revolve.rs` rotates profiles `(0, 360]`° right-handedly; `nurbs_surface_sweep1.rs` blends compatible knots along perpendicular rail stations; `nurbs_surface_sweep2.rs` aligns two rails and shapes by endpoint proximity; `nurbs_surface_network.rs` `2`-colours `4..32` crossing curves into Coons or sampled Gordon patches. Spanning `95` lines, this module stays free of duplicate state by exposing `patch` and verified by `the_corners_are_reproduced`. Spanning `95` lines, this module stays free of duplicate state by exposing `patch` and verified by `the_corners_are_reproduced`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/nurbs_surface_4_points.rs"
```

### Step 63 · `viewer-core/src/command/verbs/nurbs_surface_loft.rs`

`nurbs_surface_4_points.rs` orders bilinear controls `(a, d, b, c)`; `loft.rs` (`Open`/`Closed`) and `nurbs_surface_loft.rs` (`Smooth` deg `3` / `Straight` deg `1`) loft aligned sections; `nurbs_surface_revolve.rs` rotates profiles `(0, 360]`° right-handedly; `nurbs_surface_sweep1.rs` blends compatible knots along perpendicular rail stations; `nurbs_surface_sweep2.rs` aligns two rails and shapes by endpoint proximity; `nurbs_surface_network.rs` `2`-colours `4..32` crossing curves into Coons or sampled Gordon patches. Spanning `49` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `49` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/nurbs_surface_loft.rs"
```

### Step 64 · `viewer-core/src/command/verbs/nurbs_surface_network.rs`

`nurbs_surface_4_points.rs` orders bilinear controls `(a, d, b, c)`; `loft.rs` (`Open`/`Closed`) and `nurbs_surface_loft.rs` (`Smooth` deg `3` / `Straight` deg `1`) loft aligned sections; `nurbs_surface_revolve.rs` rotates profiles `(0, 360]`° right-handedly; `nurbs_surface_sweep1.rs` blends compatible knots along perpendicular rail stations; `nurbs_surface_sweep2.rs` aligns two rails and shapes by endpoint proximity; `nurbs_surface_network.rs` `2`-colours `4..32` crossing curves into Coons or sampled Gordon patches. Spanning `425` lines, this module stays free of duplicate state by exposing `network` and verified by `a_saddle_from_its_rulings` (+2 more tests). Spanning `425` lines, this module stays free of duplicate state by exposing `network` and verified by `a_saddle_from_its_rulings` (+2 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/nurbs_surface_network.rs"
```

### Step 65 · `viewer-core/src/command/verbs/nurbs_surface_revolve.rs`

`nurbs_surface_4_points.rs` orders bilinear controls `(a, d, b, c)`; `loft.rs` (`Open`/`Closed`) and `nurbs_surface_loft.rs` (`Smooth` deg `3` / `Straight` deg `1`) loft aligned sections; `nurbs_surface_revolve.rs` rotates profiles `(0, 360]`° right-handedly; `nurbs_surface_sweep1.rs` blends compatible knots along perpendicular rail stations; `nurbs_surface_sweep2.rs` aligns two rails and shapes by endpoint proximity; `nurbs_surface_network.rs` `2`-colours `4..32` crossing curves into Coons or sampled Gordon patches. Spanning `116` lines, this module stays free of duplicate state by exposing `revolve` and verified by `a_line_revolves_into_a_cylinder`. Spanning `116` lines, this module stays free of duplicate state by exposing `revolve` and verified by `a_line_revolves_into_a_cylinder`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/nurbs_surface_revolve.rs"
```

### Step 66 · `viewer-core/src/command/verbs/nurbs_surface_sweep1.rs`

`nurbs_surface_4_points.rs` orders bilinear controls `(a, d, b, c)`; `loft.rs` (`Open`/`Closed`) and `nurbs_surface_loft.rs` (`Smooth` deg `3` / `Straight` deg `1`) loft aligned sections; `nurbs_surface_revolve.rs` rotates profiles `(0, 360]`° right-handedly; `nurbs_surface_sweep1.rs` blends compatible knots along perpendicular rail stations; `nurbs_surface_sweep2.rs` aligns two rails and shapes by endpoint proximity; `nurbs_surface_network.rs` `2`-colours `4..32` crossing curves into Coons or sampled Gordon patches. Spanning `224` lines, this module stays free of duplicate state by exposing `sweep1` and verified by `one_profile_keeps_its_place_on_the_rail` (+1 more tests). Spanning `224` lines, this module stays free of duplicate state by exposing `sweep1` and verified by `one_profile_keeps_its_place_on_the_rail` (+1 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/nurbs_surface_sweep1.rs"
```

### Step 67 · `viewer-core/src/command/verbs/nurbs_surface_sweep2.rs`

`nurbs_surface_4_points.rs` orders bilinear controls `(a, d, b, c)`; `loft.rs` (`Open`/`Closed`) and `nurbs_surface_loft.rs` (`Smooth` deg `3` / `Straight` deg `1`) loft aligned sections; `nurbs_surface_revolve.rs` rotates profiles `(0, 360]`° right-handedly; `nurbs_surface_sweep1.rs` blends compatible knots along perpendicular rail stations; `nurbs_surface_sweep2.rs` aligns two rails and shapes by endpoint proximity; `nurbs_surface_network.rs` `2`-colours `4..32` crossing curves into Coons or sampled Gordon patches. Spanning `135` lines, this module stays free of duplicate state by exposing `sweep2` and verified by `shape_ends_lie_on_the_rails`. Spanning `135` lines, this module stays free of duplicate state by exposing `sweep2` and verified by `shape_ends_lie_on_the_rails`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/nurbs_surface_sweep2.rs"
```

### Step 68 · `viewer-core/src/command/verbs/trim.rs`

`trim.rs` supports interactive target/cutter/part clicking plus `2`-parameter curve domain trims; `trim/parts.rs` partitions curve intervals (wrapping closed seams), BRep faces and plane-cut meshes in the target frame. Spanning `721` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `trim_parses_both_forms`. Spanning `721` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `trim_parses_both_forms`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/trim.rs"
```

### Step 69 · `viewer-core/src/command/verbs/trim/parts.rs`

`trim.rs` supports interactive target/cutter/part clicking plus `2`-parameter curve domain trims; `trim/parts.rs` partitions curve intervals (wrapping closed seams), BRep faces and plane-cut meshes in the target frame. Spanning `1475` lines, this module stays free of duplicate state by exposing `TOLERANCE`, `Parts`, `Blade`, `count` and verified by `a_line_keeps_what_is_not_removed` (+10 more tests). Spanning `1475` lines, this module stays free of duplicate state by exposing `TOLERANCE`, `Parts`, `Blade`, `count` and verified by `a_line_keeps_what_is_not_removed` (+10 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/trim/parts.rs"
```

### Step 70 · `viewer-core/src/command/verbs/extend.rs`

`extend.rs` runs `Repeat` endpoint clicks (each its own undo step) or `2`-parameter domain growth; `extend/reach.rs` extends open line/polyline tangents or NURBS ends to positive distances or nearest boundary crossings. Spanning `781` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `a_click_near_a_box_on_screen` (+2 more tests). Spanning `781` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `a_click_near_a_box_on_screen` (+2 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/extend.rs"
```

### Step 71 · `viewer-core/src/command/verbs/extend/reach.rs`

`extend.rs` runs `Repeat` endpoint clicks (each its own undo step) or `2`-parameter domain growth; `extend/reach.rs` extends open line/polyline tangents or NURBS ends to positive distances or nearest boundary crossings. Spanning `500` lines, this module stays free of duplicate state by exposing `End`, `Reach`, `Grown`, `tip` and verified by `a_line_reaches_the_boundary_ahead` (+2 more tests). Spanning `500` lines, this module stays free of duplicate state by exposing `End`, `Reach`, `Grown`, `tip` and verified by `a_line_reaches_the_boundary_ahead` (+2 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/extend/reach.rs"
```

### Step 72 · `viewer-core/src/command/verbs/split.rs`

`app/splitting.rs` validates `1..64` unlocked cutters transformed into the target's frame; `state/splitting.rs` retains pending target/cutters across picks and restores highlights on error; `verbs/split.rs` confirms or starts `split_command` while preserving the pending split. Spanning `31` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `31` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/split.rs"
```

### Step 73 · `viewer-core/src/command/verbs/explode.rs`

`explode.rs` replaces leaf objects (refusing group parents) with bounded child parts in one undo step; `extrude.rs` caps closed planar profiles topologically along shared top, bottom and seam edges while leaving open/bent profiles uncapped. Spanning `540` lines, this module stays free of duplicate state by exposing `explode_rows` and verified by `explode_is_one_transaction_and_preserves_placement` (+4 more tests). Spanning `540` lines, this module stays free of duplicate state by exposing `explode_rows` and verified by `explode_is_one_transaction_and_preserves_placement` (+4 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/explode.rs"
```

### Step 74 · `viewer-core/src/command/verbs/area.rs`

`measure.rs` resolves placed geometry (requesting hydration if released) and tags temporary marks with `row_revision`; `measure_distance.rs` reports Euclidean + `Δx/Δy/Δz` without document edits; `length.rs` transforms curves into world space first for non-uniform scale accuracy; `area.rs` sums placed mesh/BRep triangles or `8×8` Gauss-quadrature NURBS knot spans; `volume.rs` sums origin-shifted signed tetrahedra on closed solids only; `bounding_box.rs` commits an axis-aligned world box while `length_bounding_box.rs` only displays `X × Y × Z` dimensions. Spanning `243` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `meshes_and_breps_measure_in_world_units` (+1 more tests). Spanning `243` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `meshes_and_breps_measure_in_world_units` (+1 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/area.rs"
```

### Step 75 · `viewer-core/src/command/verbs/length.rs`

`measure.rs` resolves placed geometry (requesting hydration if released) and tags temporary marks with `row_revision`; `measure_distance.rs` reports Euclidean + `Δx/Δy/Δz` without document edits; `length.rs` transforms curves into world space first for non-uniform scale accuracy; `area.rs` sums placed mesh/BRep triangles or `8×8` Gauss-quadrature NURBS knot spans; `volume.rs` sums origin-shifted signed tetrahedra on closed solids only; `bounding_box.rs` commits an axis-aligned world box while `length_bounding_box.rs` only displays `X × Y × Z` dimensions. Spanning `100` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `curves_measure_in_world_units`. Spanning `100` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `curves_measure_in_world_units`.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/length.rs"
```

### Step 76 · `viewer-core/src/command/verbs/measure.rs`

`measure.rs` resolves placed geometry (requesting hydration if released) and tags temporary marks with `row_revision`; `measure_distance.rs` reports Euclidean + `Δx/Δy/Δz` without document edits; `length.rs` transforms curves into world space first for non-uniform scale accuracy; `area.rs` sums placed mesh/BRep triangles or `8×8` Gauss-quadrature NURBS knot spans; `volume.rs` sums origin-shifted signed tetrahedra on closed solids only; `bounding_box.rs` commits an axis-aligned world box while `length_bounding_box.rs` only displays `X × Y × Z` dimensions. Spanning `301` lines, this module stays free of duplicate state by exposing `Mark`, `Target`, `selected`, `loading` and verified by `values_print_short` (+3 more tests). Spanning `301` lines, this module stays free of duplicate state by exposing `Mark`, `Target`, `selected`, `loading` and verified by `values_print_short` (+3 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/measure.rs"
```

### Step 77 · `viewer-core/src/command/verbs/measure_distance.rs`

`measure.rs` resolves placed geometry (requesting hydration if released) and tags temporary marks with `row_revision`; `measure_distance.rs` reports Euclidean + `Δx/Δy/Δz` without document edits; `length.rs` transforms curves into world space first for non-uniform scale accuracy; `area.rs` sums placed mesh/BRep triangles or `8×8` Gauss-quadrature NURBS knot spans; `volume.rs` sums origin-shifted signed tetrahedra on closed solids only; `bounding_box.rs` commits an axis-aligned world box while `length_bounding_box.rs` only displays `X × Y × Z` dimensions. Spanning `191` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `two_points_answer_and_fewer_are_picked` (+1 more tests). Spanning `191` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `two_points_answer_and_fewer_are_picked` (+1 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/measure_distance.rs"
```

### Step 78 · `viewer-core/src/command/verbs/volume.rs`

`measure.rs` resolves placed geometry (requesting hydration if released) and tags temporary marks with `row_revision`; `measure_distance.rs` reports Euclidean + `Δx/Δy/Δz` without document edits; `length.rs` transforms curves into world space first for non-uniform scale accuracy; `area.rs` sums placed mesh/BRep triangles or `8×8` Gauss-quadrature NURBS knot spans; `volume.rs` sums origin-shifted signed tetrahedra on closed solids only; `bounding_box.rs` commits an axis-aligned world box while `length_bounding_box.rs` only displays `X × Y × Z` dimensions. Spanning `222` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `closed_meshes_measure_and_open_ones_are_refused` (+1 more tests). Spanning `222` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `closed_meshes_measure_and_open_ones_are_refused` (+1 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/volume.rs"
```

### Step 79 · `viewer-core/src/command/verbs/arrowhead.rs`

`attributes.rs` toggles owned element features; `interactions.rs` toggles joint/cut/contact features (rewalking when needed); `arrowhead.rs` (`None`/`Start`/`End`/`Both`) replaces unlocked selected curves in one transaction. Spanning `205` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `options_parse` (+2 more tests). Spanning `205` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `options_parse` (+2 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/arrowhead.rs"
```

## Integration

`viewer-core/src/command/verbs/mod.rs` expands the `verbs!` macro into `REGISTRY` in strict alphabetical order, and every `Action::run` takes `&mut dyn CommandHost` so identical verb code executes inside browser `State` (`src/state/host.rs`), `HeadlessHost` (`viewer-core`), and `session-viewer-cli`.

## Run and observe

Run `./target/x86_64-unknown-linux-gnu/debug/session-viewer-cli exec --cmd "Box 0,0,0 100 200 50" --cmd "Select All" --cmd "Move 25,50,10" --cmd "Volume"`: the CLI creates a solid BRep box on `Created`, translates it in one undo step, and prints its exact world volume (`1,000,000 mm^3`).

## Predict and try

Predict what happens when two placements share one `Rc<Session>` document and you `Move` an object in only the first placement (`an_edit_must_split_a_session_two_placements_share`): `Scene` clones-on-write (`Rc::make_mut`) only the edited document so the second placement stays untouched at its original coordinates.

## Question and answer

**Why does `ControlDrag` (`state/edit.rs`) cache `from_local` and patch local mesh neighborhoods through `mesh_preview.rs` during pointer motion instead of calling `Scene::replace_rows` every frame?**

**How to work it out:** Measure what `Scene::replace_rows` does on a `100,000`-vertex mesh: cloning the whole `Mesh`, rebuilding topology, walking all edges, and recording an undo transaction on every `16 ms` mouse move.

**The answer:** `mesh_preview.rs` identifies the `1`-ring neighborhood incident to the dragged vertex/edge/face once on press and patches only those GPU vertex slots per frame (`O(local valence)`), recording zero undo steps during the drag and committing a single `Session` transaction on mouse release (or restoring cleanly on `Escape`).

## Validation

Verified via `cargo xtest -p viewer-core command::` (`160+` verb, tool, surfacing, and trim tests), `modeling::tests`, `deform::tests`, `snap::tests`, `cplane::tests`, and `edit::tests`.
