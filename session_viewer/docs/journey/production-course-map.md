# Production source and course coverage map

The production viewer under `session_viewer/src` is the source of the course. This initial audit maps the actual cumulative endpoint, not just similarly named lesson files. It does not accept pending CAD evidence or private proposals.

**22 chapters · 1361 preserved checkpoints · 111 authored reader lessons.** 18 chapters are integrated in the reader route, which currently contains 287 lessons. Runtime and visual acceptance are tracked separately.

[Machine-readable map](production-course-map.json) records every old step, its proposed retained lesson, runnable checkpoint, production source hashes and per-module provenance. Original source snippets, URLs, history and checkpoints remain intact.

## Source findings

Six complete shader files match production after removing comments and whitespace; this is code correspondence, not visual acceptance:

- `src/shaders/ambient_depth.wgsl` → `src/ambient_depth.wgsl`

- `src/shaders/surface_outline.wgsl` → `src/outline.wgsl`

- `src/shaders/widget.wgsl` → `src/widget.wgsl`

- `src/shaders/projected_triangle.wgsl` → `src/projected_triangle.wgsl`

- `src/shaders/slot_table.wgsl` → `src/primitive_slot_lookup.wgsl`

- `src/shaders/ambient_composite.wgsl` → `src/ambient_composite.wgsl`

Cloud resolve retains the actual four-neighbor log-depth EDL formula, strength factor 75 and floor 0.25. Its bindings and pick output differ. Course stroke coverage currently uses capsule-distance clamping; production uses the developed ribbon extrusion/coverage path. Cloud density and fine stroke ends/AA remain known reconciliation work. No replacement shader is introduced by this audit.

## Feature coverage

| Feature | Existing chapters / steps | Proposed lesson | Runnable checkpoint | Representative evidence and remaining defect |
| --- | --- | --- | --- | --- |
| Startup, dock and frame ownership | 1, 4; exact IDs in map | `01-canvas` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` (pending CAD) | **staged**. Early clear/triangle and Background command are dependency exercises, not final production surface shading. Device recovery remains a later integration audit. |
| Mesh, materials, normals and lighting | 2, 3, 5, 18, 20, 22; exact IDs in map | `05-vertices` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` (pending CAD) | **staged**. Actual GPU layouts/pipeline/lighting reconciliation still needs complete source and feature review; a passing selected mesh fixture is not full parity. |
| Cloud octree, LOD, splats, depth and EDL | 11, 12, 22; exact IDs in map | `63-radius` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` (pending CAD) | **divergent**. Observed course DPR1/2 cloud density differs; matched production evidence is pending, so no production mismatch is established. The 1,681-point witness is below the 2,000,000-point LOD threshold. EDL equations correspond; composed splat/LOD parity remains unverified. |
| Lines, joined strokes, widths and AA | 5, 17, 22; exact IDs in map | `35-stroke` and mapped lessons | `96zzzzzzzq-draw-original-cad-wire-curves` (pending CAD) | **divergent**. Course stroke coverage is a capsule-distance clamp; production ribbon has its own joined extrusion and coverage path. Known fine terminal/AA mismatch needs matched source/camera/DPR/samples/encoding witness, not an invented final replacement. |
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
| 19. Integration and original ownership | 34 | 4 | Authored; reader integration and runtime acceptance pending |
| 20. Shared GPU rendering | 95 | 6 | Authored; reader integration and runtime acceptance pending |
| 21. Precise camera and browser coordinates | 30 | 5 | Authored; reader integration and runtime acceptance pending |
| 22. Surface lighting, contact and display quality | 38 | 6 | Authored; reader integration and runtime acceptance pending |

## Evidence boundaries and next work

Stored chapter images remain historical unless their runtime, fixtures, harness and environment match the recorded proof. The representative image manifest is `visual-review.json`, alongside `screenshots/journey/browser.json`; neither file is re-signed by this audit. Current CAD source and partial evidence remain at `/tmp/viewer-cad-wire-1361-pilot`; its stopped browser job must not restart as part of the prose pilot.

Private Plane/runtime proposals are uncompiled and unaccepted until staged and checked. Some earlier display drafts have since landed; the current cumulative endpoint and its source hash, not stale private-plan status text, determine what is present.

Remaining acceptance: reconcile cloud density and ribbon AA against production; close CAD holes/isolines/mirrored placement/family visibility; finish original Plane and pen/cloud controls; measure runtime before optimization and dependency removal; implement reload camera retention and explicit-only base-grid changes; then freeze sources and run final required checks. No new chapter or microtutorial is needed.

The pilot changes prose/navigation only. Preserve valid Native/WASM/GPU results; run source-replay, alias/link/docs checks and inspect the published chapter. No commits, pushes or external publication.
