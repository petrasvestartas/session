# Production source and course coverage map

The production viewer under `session_viewer/src` is the source of the course. This initial audit maps the actual cumulative endpoint, not just similarly named lesson files. It does not accept pending CAD evidence or private proposals.

**22 chapters · 1361 preserved checkpoints · 111 authored reader lessons.** All 22 chapters are integrated in the reader route, which contains 111 lessons. Runtime and visual acceptance are tracked separately.

[Machine-readable map](production-course-map.json) records every old step, its proposed retained lesson, runnable checkpoint, production source hashes and per-module provenance. Original source snippets, URLs, history and checkpoints remain intact.

## Latest integrated source (2026-10-23)

All 16 repositories were updated from GitHub (root95a14e08/sessiona391b104). The pull preserved all25,829 tutorial files byte for byte; a complete backup and hash manifests remain in Downloads/viewer-course-source-sync-20261020. Later intentional tutorial edits integrate the pushed Matrix renderer into existing lesson22.6, without new IDs.

Matrix now includes exact production WGSL composition and packing, component/archive identity, whole-object picking, selected borders and an independent plate-less world title. All five native Matrix tests pass and four unselected/selected one/four-sample frames were inspected. The shared cloud/Line visibility correction passes the existing regression and12focused production/course controls. These focused results do not close full display or browser acceptance.

The module hashes and six shader correspondences below describe the initial audit. They are not a new whole-tree source freeze after this pull; current integrated proof is recorded in consolidation/current-state.json and per-run manifests.

## Source findings

Six complete shader files match production after removing comments and whitespace; this is code correspondence, not visual acceptance:

- `src/shaders/ambient_depth.wgsl` → `src/ambient_depth.wgsl`

- `src/shaders/surface_outline.wgsl` → `src/outline.wgsl`

- `src/shaders/widget.wgsl` → `src/widget.wgsl`

- `src/shaders/projected_triangle.wgsl` → `src/projected_triangle.wgsl`

- `src/shaders/slot_table.wgsl` → `src/primitive_slot_lookup.wgsl`

- `src/shaders/ambient_composite.wgsl` → `src/ambient_composite.wgsl`

Standalone lines and joined/vector paths now use complete production programs with separate compact ownership adapters. Eight matched Point frames and all eight mesh frames, including selection, agree exactly across DPR/sample combinations. The disabled-outline policy now follows production. Ordinary Plane, printed fills and mesh/BRep border algorithms are integrated. Focused Matrix, glyph, cloud, source-identity and retained-owner checks pass; complete placed/selected/CAD/browser acceptance remains open.

Both implementations now contain the reload-camera and held-grid corrections. Four course regressions and three maintained native grid checks pass, including exact fractional world-grid pixels at a 10^12-unit origin. Browser route/live checks are still pending. Normal native dependencies exclude the maintained viewer, and presentation coalescing passes its three scheduler tests; browser measurements remain pending.

The feature rows below retain the initial audit classifications. Current execution status is in `session/docs/plans/2026-10-18-viewer-course-progress.md` and [integration state](consolidation/current-state.json); an initial `missing` row does not describe the subsequently integrated source.

## Feature coverage

| Feature | Existing chapters / steps | Proposed lesson | Runnable checkpoint | Representative evidence and remaining defect |
| --- | --- | --- | --- | --- |
| Startup, dock and frame ownership | 1, 4; exact IDs in map | `01-canvas` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` (pending CAD) | **staged**. Early clear/triangle and Background command are dependency exercises, not final production surface shading. Device recovery remains a later integration audit. |
| Mesh, materials, normals and lighting | 2, 3, 5, 18, 20, 22; exact IDs in map | `05-vertices` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` (pending CAD) | **staged**. Actual GPU layouts/pipeline/lighting reconciliation still needs complete source and feature review; a passing selected mesh fixture is not full parity. |
| Cloud octree, LOD, splats, depth and EDL | 11, 12, 22; exact IDs in map | `63-radius` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` (pending CAD) | **extracted; verification pending**. Actual-camera 221-point WARP baseline matches DPR1 but shows half-radius course splats at DPR2. Production splat/resolve sources and records are now extracted, including DPR, strict depth and world clipping; fresh pixel/query checks remain pending. The fixture is below the 2,000,000-point LOD threshold; large-source LOD and placement remain open. |
| Lines, joined strokes, widths and AA | 5, 17, 22; exact IDs in map | `35-stroke` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` (pending CAD) | **divergent**. Ordinary lines use the production ribbon: eight matched scenarios agree exactly; near-camera cuts, selection/rebasing, mixed-family visibility and opacity checks pass. Coplanar section ownership, sloping-plane width and fresh placed/rebased line queries now pass native checks. Joins, vectors and other ink families remain staged. |
| Markers, mesh dots and normals | 5, 22; exact IDs in map | `35-stroke` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` (pending CAD) | **staged**. Point sizing/crowding and pen controls still require production-path verification. Missing listed filenames must be resolved in source inventory, not read as absent functionality. |
| BRep tessellation, trims, holes, borders and NURBS isolines | 9, 15, 22; exact IDs in map | `55-validation` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` (pending CAD) | **staged**. CAD wire endpoint is pending browser/visual acceptance. Real holes, isoline legibility, family visibility and mirrored/nonuniform placement require independent evidence; existing Native/WASM evidence is retained. |
| Clipping, cap winding and source sections | 17, 20; exact IDs in map | `86-copy-finite-tile-allocation-rules` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` (pending CAD) | **staged**. Check production pass ordering, clipping policy, caps with holes and color/pick identity against actual source. Existing supplied cap fixtures alone are not an extraction proof. |
| Selection, integer IDs and async picks | 6, 7, 19, 20, 21; exact IDs in map | `42-window` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` (pending CAD) | **reused**. Widget WGSL matches after removing comments/whitespace. Other pick/crop/rebase owners are staged and still need semantic provenance closure. |
| Shared instances, packed vertices and GPU arenas | 13, 19, 20; exact IDs in map | `69-validate-original-definition-references` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` (pending CAD) | **reused**. Slot-table WGSL matches after comments/whitespace removal. Validate remaining layouts, draw ranges and ownership separately; do not infer all instance parity from this match. |
| Text, glyphs and drawing sheets | 10, 13, 14; exact IDs in map | `60-fonts` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` (pending CAD) | **staged**. Audit shaping, physical raster sizes, placements, clipping, source queries and retained resources. |
| Outlines, depth pyramids and ambient occlusion | 11, 18, 22; exact IDs in map | `63-radius` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` (pending CAD) | **reused**. Three listed WGSL files match after comments/whitespace removal. Remaining GTAO inputs, pass integration and physical multisample targets need independent verification. |
| Finite visibility and triangle tiles | 17, 20; exact IDs in map | `86-copy-finite-tile-allocation-rules` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` (pending CAD) | **reused**. Projected-triangle WGSL matches after comments/whitespace removal. Finite visibility integration remains staged; preserve production sag/tolerance/physical addresses. |
| Transparency and Xray | 18, 20, 22; exact IDs in map | `89-copy-density-scaled-depth-reduction` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` (pending CAD) | **staged**. Production face_list uses view opacity and shared arena/splat resolve. Verify transparent materials, locked/source selection, copied objects and clipping with actual fixtures. |
| History, hierarchy, tools and persistence | 3, 7, 8, 14, 15, 16, 19; exact IDs in map | `17-solid` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` (pending CAD) | **staged**. Preserve user hierarchy/panel/deform edits. Original save/open and edit owners are authoritative; course command algorithms must trace to them. |
| Streaming and GPU resource lifetime | 4, 12, 20; exact IDs in map | `33-owner` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` (pending CAD) | **staged**. Check cancellation, single-flight readbacks, retained buffer reuse, release and idle behavior before optimizing. |
| Camera, original Plane and reload behavior | 7, 14, 21, 22; exact IDs in map | `46-tool-panel` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` (pending CAD) | **missing**. Original Plane display/control coverage is incomplete. Camera-on-reload and fixed-base-grid requirements remain unimplemented in maintained/course runtimes; include motion during load and floating-origin world-plane checks. |
| Frame resources, multisampling and performance | 4, 18, 22; exact IDs in map | `33-owner` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` (pending CAD) | **staged**. Preserve actual MSAA/pipeline/target policy. Measure cold load, input-to-frame, steady frame time, uploads/allocations and idle before runtime changes; software GPU is not hardware FPS. |

## Finite editorial batches

| Chapter | Preserved steps | Reader lessons | Status |
| --- | ---: | ---: | --- |
| 1. Browser, Rust and the GPU | 29 | 4 | Consolidated; runtime evidence tracked separately |
| 2. Geometry, camera and selection | 19 | 6 | Consolidated; runtime evidence tracked separately |
| 3. Documents, editing and ownership | 96 | 7 | Consolidated; runtime evidence tracked separately |
| 4. Browser lifetime and recovery | 40 | 5 | Consolidated; runtime evidence tracked separately |
| 5. Lines, points and curves | 42 | 7 | Consolidated; runtime evidence tracked separately |
| 6. Picking and selection | 34 | 4 | Consolidated; runtime evidence tracked separately |
| 7. Drawing, snapping and transform tools | 57 | 6 | Consolidated; runtime evidence tracked separately |
| 8. Hierarchy, layers and direct editing | 53 | 4 | Consolidated; runtime evidence tracked separately |
| 9. CAD surfaces and topology | 26 | 5 | Consolidated; runtime evidence tracked separately |
| 10. Text and labels | 52 | 4 | Consolidated; runtime evidence tracked separately |
| 11. Outlines and point clouds | 41 | 4 | Consolidated; runtime evidence tracked separately |
| 12. Files and streaming | 102 | 4 | Consolidated; runtime evidence tracked separately |
| 13. Shared definitions and drawing sheets | 30 | 4 | Consolidated; runtime evidence tracked separately |
| 14. Coordinates and geometry tools | 118 | 6 | Consolidated; runtime evidence tracked separately |
| 15. Transforms and surface construction | 81 | 4 | Consolidated; runtime evidence tracked separately |
| 16. Geometry editing and measurement | 132 | 5 | Consolidated; runtime evidence tracked separately |
| 17. GPU visibility, clipping and sections | 122 | 4 | Consolidated; runtime evidence tracked separately |
| 18. Lighting, render quality and comparison | 89 | 7 | Consolidated; runtime evidence tracked separately |
| 19. Integration and original ownership | 34 | 4 | Consolidated; runtime evidence tracked separately |
| 20. Shared GPU rendering | 95 | 6 | Consolidated; runtime evidence tracked separately |
| 21. Precise camera and browser coordinates | 30 | 5 | Consolidated; runtime evidence tracked separately |
| 22. Surface lighting, contact and display quality | 38 | 6 | Consolidated; runtime evidence tracked separately |

## Evidence boundaries and next work

Stored chapter images remain historical unless their runtime, fixtures, harness and environment match the recorded proof. The current 22-chapter evidence audit confirms every checkpoint and snippet is present; accepted chapter-local feature images are still missing for 19 chapters. Historical capture candidates must be checked against their recorded source before acceptance.

Ordinary Plane, printed fills, Matrix, glyphs, mesh/BRep borders and reload/grid scheduling are integrated and compiled. Native source ownership, clipping, visibility and selection checks are being reconciled with the active production paths. The full native run found 29 failures after 1,030 passes; focused repair results are recorded in the current progress plan. Eight mesh and two Windows grid full-frame production/course comparisons match exactly; these are native results, separate from browser acceptance.

Remaining acceptance covers matched Plane/CAD/joined/sheet ingress, current browser fixtures, presentation lifecycle and all five measured performance categories, four route/live reload-grid combinations, then final source, checkpoint, visual, tooling and docs checks. Preserve the 22 chapters, 111 lessons and all existing checkpoint IDs. Nothing has been committed, pushed or externally published.
