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

Standalone lines and joined/vector paths now use complete production programs with separate compact ownership adapters. Eight Point and eight mesh full-frame pairs match after the initial GPU translation correction. Sixteen mirrored/nonuniform placed Plane pairs also match across DPR/sample/display-size/selection combinations. These are bounded native results; plain/covered/printed-fill Plane controls remain pending. The disabled-outline policy now follows production. Ordinary Plane, printed fills and mesh/BRep border algorithms are integrated. Focused Matrix, glyph, cloud, source-identity and retained-owner checks pass; complete placed/selected/CAD/browser acceptance remains open.

Both implementations now contain the reload-camera and held-grid corrections. Four course regressions and three maintained native grid checks pass, including exact fractional world-grid pixels at a 10^12-unit origin. Browser route/live checks are still pending. Normal native dependencies exclude the maintained viewer, and presentation coalescing passes its three scheduler tests; browser measurements remain pending.

The feature rows below distinguish integrated bounded checks from remaining feature acceptance. Current execution status is in `session/docs/plans/2026-10-18-viewer-course-progress.md` and [integration state](consolidation/current-state.json); an initial `missing` row does not describe the subsequently integrated source.

## Feature coverage

| Feature | Existing chapters / steps | Proposed lesson | Runnable checkpoint | Evidence and remaining work |
| --- | --- | --- | --- | --- |
| Startup, dock and frame ownership | 1, 4; exact IDs in map | `01-canvas` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` | **staged**. Early clear/triangle and Background command are dependency exercises, not final production surface shading. Device recovery remains a later integration audit. |
| Mesh, materials, normals and lighting | 2, 3, 5, 18, 20, 22; exact IDs in map | `05-vertices` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` | **staged**. Actual GPU layouts/pipeline/lighting reconciliation still needs complete source and feature review; a passing selected mesh fixture is not full parity. |
| Cloud octree, LOD, splats, depth and EDL | 11, 12, 22; exact IDs in map | `63-radius` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` | **staged**. Production splat/resolve extraction passed eight DPR/sample/EDL pairs; 18 actual production/course LOD pairs used the original 2,000,064-point source and placed owners. Twelve focused cloud/Line controls pass after the shared visibility correction. These bounded results replace the earlier unresolved DPR diagnosis; complete current display-group and browser acceptance remain open. |
| Lines, joined strokes, widths and AA | 5, 17, 22; exact IDs in map | `35-stroke` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` | **staged**. Eight ordinary non-sheet production/course pairs matched all pixels with actual GPU pen inputs verified. Actual resident Sheet ingress passed all32 projection/placement/DPR/sample/selection pairs after correcting witness camera setup. Joined/CAD families and complete current display controls still require their own source-bound visual and browser evidence. |
| Markers, mesh dots and normals | 5, 22; exact IDs in map | `35-stroke` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` | **staged**. Corrected actual-production Point8/8 and mesh8/8 native full-RGBA pairs match at the recorded translated orthographic pose, including DPR1/2, samples1/4 and selection. These do not establish arbitrary occlusion or all marker/normal/control toggles; complete display controls and browser coverage remain pending. |
| BRep tessellation, trims, holes, borders and NURBS isolines | 9, 15, 22; exact IDs in map | `55-validation` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` | **staged**. CAD wire endpoint is pending browser/visual acceptance. Real holes, isoline legibility, family visibility and mirrored/nonuniform placement require independent evidence; existing Native/WASM evidence is retained. |
| Clipping, cap winding and source sections | 17, 20; exact IDs in map | `86-copy-finite-tile-allocation-rules` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` | **staged**. Check production pass ordering, clipping policy, caps with holes and color/pick identity against actual source. Existing supplied cap fixtures alone are not an extraction proof. |
| Selection, integer IDs and async picks | 6, 7, 19, 20, 21; exact IDs in map | `42-window` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` | **reused**. Widget WGSL matches after removing comments/whitespace. Other pick/crop/rebase owners are staged and still need semantic provenance closure. |
| Shared instances, packed vertices and GPU arenas | 13, 19, 20; exact IDs in map | `69-validate-original-definition-references` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` | **reused**. Slot-table WGSL matches after comments/whitespace removal. Validate remaining layouts, draw ranges and ownership separately; do not infer all instance parity from this match. |
| Text, glyphs and drawing sheets | 10, 13, 14; exact IDs in map | `60-fonts` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` | **staged**. Finite Sheet/text criteria complete: separately recorded native shaping/placement/clipping/query/lifetime assertions and current Chapter 14 inherited Sheet/drawing browser checks, with fresh root-reviewed Sheet state before replacement. [Finite proof](release/sheet-text-finite-review.json). Other source-specific callbacks and final release remain separate. |
| Outlines, depth pyramids and ambient occlusion | 11, 18, 22; exact IDs in map | `63-radius` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` | **reused**. Three listed WGSL files match after comments/whitespace removal. Remaining GTAO inputs, pass integration and physical multisample targets need independent verification. |
| Finite visibility and triangle tiles | 17, 20; exact IDs in map | `86-copy-finite-tile-allocation-rules` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` | **reused**. Projected-triangle WGSL matches after comments/whitespace removal. Finite visibility integration remains staged; preserve production sag/tolerance/physical addresses. |
| Transparency and Xray | 18, 20, 22; exact IDs in map | `89-copy-density-scaled-depth-reduction` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` | **staged**. Production face_list uses view opacity and shared arena/splat resolve. Verify transparent materials, locked/source selection, copied objects and clipping with actual fixtures. |
| History, hierarchy, tools and persistence | 3, 7, 8, 14, 15, 16, 19; exact IDs in map | `17-solid` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` | **staged**. Preserve user hierarchy/panel/deform edits. Original save/open and edit owners are authoritative; course command algorithms must trace to them. |
| Streaming and GPU resource lifetime | 4, 12, 20; exact IDs in map | `33-owner` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` | **staged**. Check cancellation, single-flight readbacks, retained buffer reuse, release and idle behavior before optimizing. |
| Camera, original Plane and reload behavior | 7, 14, 21, 22; exact IDs in map | `46-tool-panel` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` | **staged**. Latest-camera publication and held world-grid behavior are implemented in maintained/course runtimes; focused native checks and actual grid near/far/rebase frames pass. Sixteen corrected-witness mirrored/nonuniform placed Plane pairs match all RGBA pixels across DPR1/2, samples1/4, halfsize50/100 and selection. Plain/covered/printed-fill Plane controls and all four deferred-load browser reload/grid combinations remain unaccepted. |
| Frame resources, multisampling and performance | 4, 18, 22; exact IDs in map | `33-owner` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` | **staged**. Preserve actual MSAA/pipeline/target policy. Measure cold load, input-to-frame, steady frame time, uploads/allocations and idle before runtime changes; software GPU is not hardware FPS. |
| Matrix cells, procedural digits and independent world titles | 22; exact IDs in map | `96zzzzzzzm-retain-visible-instance-ranges` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` | **staged**. Five Matrix native tests pass, including actual GPU cells/digits/selection/object-picking at1/4/1 samples, archive identity/title and shader provenance. Four rendered images individually inspected. Browser and full production/course image parity remain pending. |

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
