# Course contents

Read in chapter order. Consolidated lessons use **chapter.lesson** numbering, such as **1.1**. Original chapter.section.checkpoint numbers and URLs remain available for recovery and source comparison. The [daily plan](daily-plan.md) keeps stopping points within each lesson.

[Production source and coverage map](production-course-map.md) · **18 / 22 chapters consolidated; 287 reader-facing lessons on the current route**. All 1,361 old checkpoint IDs remain available; the final maintained-runtime comparison is a reference appendix.

[Progress and remaining work](roadmap.md) · [Visual review and known issues](visual-review.md). Existing lesson links and saved checkpoints remain valid.

Each chapter shows a recorded result from one linked checkpoint. **Current Chrome check** means that checkpoint’s source and browser checks match the recorded proof. **Recorded Chrome screenshot** means fresh acceptance is pending. **Native render** shows GPU output without proving browser interaction. These pictures do not establish that every lesson in a chapter works.

## 1 · Browser, Rust and the GPU

**Automated Chrome checks: 0 of 29 checkpoints current.** Remaining checkpoints need fresh Chrome acceptance.

**Critical browser-scene review: 0 of 29 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Critical visual review pending.** This recorded reference is not an accepted visual-quality result.

**Previous native render — fresh verification pending.** [1.4.1 · Make a choice change the picture](04-input.md).

![Chapter 1: Make a choice change the picture. Previous native render — fresh verification pending.](../screenshots/journey/04-input-light.png)

### 1.1 · Rust and the first GPU frame

- [1.1 · Rust and the first GPU frame](consolidation/01-canvas.md)

### 1.2 · Browser presentation

- [1.2 · Browser presentation](consolidation/02-clear.md)

### 1.3 · Shaders and the command dock

- [1.3 · Shaders and the command dock](consolidation/03-triangle.md)

### 1.4 · Commands that change the view

- [1.4 · Commands that change the view](consolidation/04-input.md)

## 2 · Geometry, camera and selection

**Automated Chrome checks: 0 of 19 checkpoints current.** Remaining checkpoints need fresh Chrome acceptance.

**Critical browser-scene review: 0 of 19 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Critical visual review pending.** This recorded reference is not an accepted visual-quality result.

**Previous native render — fresh verification pending.** [2.12.1 · Walk around the model](16-orbit.md).

![Chapter 2: Walk around the model. Previous native render — fresh verification pending.](../screenshots/journey/16-orbit.png)

### 2.1 · Buffers that describe a shared mesh

- [2.1 · Buffers that describe a shared mesh](consolidation/05-vertices.md)

### 2.2 · Camera state and the world-to-clip transform

- [2.2 · Camera state and the world-to-clip transform](consolidation/07-uniforms.md)

### 2.3 · Depth, validated geometry and stable objects

- [2.3 · Depth, validated geometry and stable objects](consolidation/10-depth.md)

### 2.4 · Selection that agrees with the displayed view

- [2.4 · Selection that agrees with the displayed view](consolidation/12-identity.md)

### 2.5 · Document history and perspective rays

- [2.5 · Document history and perspective rays](consolidation/13a-history.md)

### 2.6 · Perspective picking and orbit navigation

- [2.6 · Perspective picking and orbit navigation](consolidation/15-perspective.md)

## 3 · Documents, editing and ownership

**Automated Chrome checks: 0 of 96 checkpoints current.** Remaining checkpoints need fresh Chrome acceptance.

**Critical browser-scene review: 0 of 96 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Critical visual review pending.** This recorded reference is not an accepted visual-quality result.

**Previous native render — fresh verification pending.** [3.16.45 · Prove failed restoration cannot partly commit](32gja-failures.md).

![Chapter 3: Prove failed restoration cannot partly commit. Previous native render — fresh verification pending.](../screenshots/journey/32gja-failures.png)

### 3.1 · Kernel geometry and a single editor owner

- [3.1 · Kernel geometry and a single editor owner](consolidation/17-solid.md)

### 3.2 · One drawing size and one captured gesture

- [3.2 · One drawing size and one captured gesture](consolidation/19a-viewport.md)

### 3.3 · Validated files, bounds and useful projections

- [3.3 · Validated files, bounds and useful projections](consolidation/22a-records.md)

### 3.4 · Local sources, world placement and display caches

- [3.4 · Local sources, world placement and display caches](consolidation/27-placement.md)

### 3.5 · Save, replace and asynchronous transaction results

- [3.5 · Save, replace and asynchronous transaction results](consolidation/29-identity.md)

### 3.6 · Shared GPU geometry and releasable source ownership

- [3.6 · Shared GPU geometry and releasable source ownership](consolidation/31-settings.md)

### 3.7 · Restore exact sources and replay the captured edit

- [3.7 · Restore exact sources and replay the captured edit](consolidation/32g-keys.md)

## 4 · Browser lifetime and recovery

**Automated Chrome checks: 0 of 40 checkpoints current.** Remaining checkpoints need fresh Chrome acceptance.

**Critical browser-scene review: 0 of 40 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Critical visual review pending.** This recorded reference is not an accepted visual-quality result.

**Previous native render — fresh verification pending.** [4.2.37 · Recover once at reduced canvas density](34geb-reload.md).

![Chapter 4: Recover once at reduced canvas density. Previous native render — fresh verification pending.](../screenshots/journey/34geb-reload.png)

### 4.1 · Browser callbacks with a real lifetime

- [4.1 · Browser callbacks with a real lifetime](consolidation/33-owner.md)

### 4.2 · First failure and independent diagnostic reports

- [4.2 · First failure and independent diagnostic reports](consolidation/34-fault.md)

### 4.3 · Admit useful evidence and bound stored runs

- [4.3 · Admit useful evidence and bound stored runs](consolidation/34e-schema.md)

### 4.4 · Heartbeat suspension and revocable startup

- [4.4 · Heartbeat suspension and revocable startup](consolidation/34g-heartbeat.md)

### 4.5 · Measured phases and conservative GPU recovery

- [4.5 · Measured phases and conservative GPU recovery](consolidation/34gc-messages.md)

## 5 · Lines, points and curves

**Automated Chrome checks: 0 of 42 checkpoints current.** Remaining checkpoints need fresh Chrome acceptance.

**Critical browser-scene review: 0 of 42 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Critical visual review pending.** This recorded reference is not an accepted visual-quality result.

**Previous native render — fresh verification pending.** [5.7.8 · Inspect the ground grid through actual commands](41f-input.md).

![Chapter 5: Inspect the ground grid through actual commands. Previous native render — fresh verification pending.](../screenshots/journey/41f-input.png)

### 5.1 · Original lines and a screen-space stroke lane

- [5.1 · Original lines and a screen-space stroke lane](consolidation/35-stroke.md)

### 5.2 · Joined paths and headed ends

- [5.2 · Joined paths and headed ends](consolidation/36-chain.md)

### 5.3 · Original point markers and curve control identities

- [5.3 · Original point markers and curve control identities](consolidation/37-point.md)

### 5.4 · NURBS source ownership and derived samples

- [5.4 · NURBS source ownership and derived samples](consolidation/38-layout.md)

### 5.5 · One visibility policy and shared surface depth

- [5.5 · One visibility policy and shared surface depth](consolidation/39-visible.md)

### 5.6 · Original mesh colours and large index buffers

- [5.6 · Original mesh colours and large index buffers](consolidation/40-colour.md)

### 5.7 · Normals, placement and a depth-correct construction grid

- [5.7 · Normals, placement and a depth-correct construction grid](consolidation/41-normals.md)

## 6 · Picking and selection

**Automated Chrome checks: 0 of 34 checkpoints current.** Remaining checkpoints need fresh Chrome acceptance.

**Critical browser-scene review: 0 of 34 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Critical visual review pending.** This recorded reference is not an accepted visual-quality result.

**Previous native render — fresh verification pending.** [6.4.5 · Collect multiple objects with Shift-click and commands](45c-input.md).

![Chapter 6: Collect multiple objects with Shift-click and commands. Previous native render — fresh verification pending.](../screenshots/journey/45c-input.png)

### 6.1 · A bounded integer picking pass

- [6.1 · A bounded integer picking pass](consolidation/42-window.md)

### 6.2 · Accept a hit only for its current view

- [6.2 · Accept a hit only for its current view](consolidation/43-ticket.md)

### 6.3 · Pick original faces, edges and controls

- [6.3 · Pick original faces, edges and controls](consolidation/44-topology.md)

### 6.4 · Ordered multi-selection and atomic edits

- [6.4 · Ordered multi-selection and atomic edits](consolidation/45-order.md)

## 7 · Drawing, snapping and transform tools

**Automated Chrome checks: 0 of 57 checkpoints current.** Remaining checkpoints need fresh Chrome acceptance.

**Critical browser-scene review: 0 of 57 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Critical visual review pending.** This recorded reference is not an accepted visual-quality result.

**Previous native render — fresh verification pending.** [7.6.34 · Verify complete handle editing and input](51zf-handles.md).

![Chapter 7: Verify complete handle editing and input. Previous native render — fresh verification pending.](../screenshots/journey/51zf-handles.png)

### 7.1 · Active tools own their command input

- [7.1 · Active tools own their command input](consolidation/46-tool-panel.md)

### 7.2 · Typed commands and host requests

- [7.2 · Typed commands and host requests](consolidation/47-registry.md)

### 7.3 · Construction-plane drafts and one final commit

- [7.3 · Construction-plane drafts and one final commit](consolidation/48-coordinates.md)

### 7.4 · Precise snapping with derived search caches

- [7.4 · Precise snapping with derived search caches](consolidation/49-settings.md)

### 7.5 · Temporary transforms with a fixed baseline

- [7.5 · Temporary transforms with a fixed baseline](consolidation/50-placements.md)

### 7.6 · Handles, numeric edits and source-part deformation

- [7.6 · Handles, numeric edits and source-part deformation](consolidation/51-handles.md)

## 8 · Hierarchy, layers and direct editing

**Automated Chrome checks: 0 of 53 checkpoints current.** Remaining checkpoints need fresh Chrome acceptance.

**Critical browser-scene review: 0 of 53 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Critical visual review pending.** This recorded reference is not an accepted visual-quality result.

**Previous native render — fresh verification pending.** [8.3.18 · Exercise current navigation selection and delayed answers](54p-navigation.md).

![Chapter 8: Exercise current navigation selection and delayed answers. Previous native render — fresh verification pending.](../screenshots/journey/54p-navigation.png)

### 8.1 · Nested original sources and exact parent edits

- [8.1 · Nested original sources and exact parent edits](consolidation/52-tree.md)

### 8.2 · Tree presentation with explicit input ownership

- [8.2 · Tree presentation with explicit input ownership](consolidation/52g-rows.md)

### 8.3 · Layers, visibility and source-preserving properties

- [8.3 · Layers, visibility and source-preserving properties](consolidation/53-flags.md)

### 8.4 · Navigation, delayed picks and direct dragging

- [8.4 · Navigation, delayed picks and direct dragging](consolidation/54-camera.md)

## 9 · CAD surfaces and topology

**Automated Chrome checks: 0 of 26 checkpoints current.** Remaining checkpoints need fresh Chrome acceptance.

**Critical browser-scene review: 0 of 26 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Critical visual review pending.** This recorded reference is not an accepted visual-quality result.

**Previous native render — fresh verification pending.** [9.5.7 · Draw the original bore and prove its empty hole](59f-cad.md).

![Chapter 9: Draw the original bore and prove its empty hole. Previous native render — fresh verification pending.](../screenshots/journey/59f-cad.png)

### 9.1 · Original surfaces, analytic normals and trim boundaries

- [9.1 · Original surfaces, analytic normals and trim boundaries](consolidation/55-validation.md)

### 9.2 · Shared CAD boundaries across incident faces

- [9.2 · Shared CAD boundaries across incident faces](consolidation/56-chains.md)

### 9.3 · Trimmed curved faces with real open holes

- [9.3 · Trimmed curved faces with real open holes](consolidation/57-parameters.md)

### 9.4 · Periodic seams and face-edge direction

- [9.4 · Periodic seams and face-edge direction](consolidation/58-uses.md)

### 9.5 · Oriented CAD solids, original picks and empty bores

- [9.5 · Oriented CAD solids, original picks and empty bores](consolidation/59-orientation.md)

## 10 · Text and labels

**Automated Chrome checks: 0 of 52 checkpoints current.** Remaining checkpoints need fresh Chrome acceptance.

**Critical browser-scene review: 0 of 52 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Critical visual review pending.** This recorded reference is not an accepted visual-quality result.

**Previous native render — fresh verification pending.** [10.3.37 · Create pick move and delete the original scene labels](62o-labels.md).

![Chapter 10: Create pick move and delete the original scene labels. Previous native render — fresh verification pending.](../screenshots/journey/62o-labels.png)

### 10.1 · Explicit fonts and source label placement

- [10.1 · Explicit fonts and source label placement](consolidation/60-fonts.md)

### 10.2 · Validated shaping and revision-based reuse

- [10.2 · Validated shaping and revision-based reuse](consolidation/60ba-validation.md)

### 10.3 · A shared glyph atlas and lazy text drawing

- [10.3 · A shared glyph atlas and lazy text drawing](consolidation/61-atlas.md)

### 10.4 · Original world labels, plates and integer picks

- [10.4 · Original world labels, plates and integer picks](consolidation/62-source.md)

## 11 · Outlines and point clouds

**Automated Chrome checks: 0 of 41 checkpoints current.** Remaining checkpoints need fresh Chrome acceptance.

**Critical browser-scene review: 0 of 41 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Critical visual review pending.** This recorded reference is not an accepted visual-quality result.

**Previous native render — fresh verification pending.** [11.3.6 · Validate original manifest text and source keys](65zb-validate-original-manifest-text-and-source-keys.md).

![Chapter 11: Validate original manifest text and source keys. Previous native render — fresh verification pending.](../screenshots/journey/65zb-validate-original-manifest-text-and-source-keys.png)

### 11.1 · Coverage-based screen-space outlines

- [11.1 · Coverage-based screen-space outlines](consolidation/63-radius.md)

### 11.2 · Validated original cloud columns and placed owners

- [11.2 · Validated original cloud columns and placed owners](consolidation/64-nodes.md)

### 11.3 · View-dependent LOD and cached point splats

- [11.3 · View-dependent LOD and cached point splats](consolidation/64f-projection.md)

### 11.4 · Original point identity and manifest source records

- [11.4 · Original point identity and manifest source records](consolidation/65-originals.md)

## 12 · Files and streaming

**Automated Chrome checks: 1 of 102 checkpoints current.** Remaining checkpoints need fresh Chrome acceptance.

**Critical browser-scene review: 1 of 102 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Critical browser-scene review accepted.** Actual Chrome reference: open CAD bore, separate NURBS roof and two dense planar point-cloud instances sharing one 2,000,064-point original. The asserted scene was critically inspected at DPR 1 and 2. The red/blue four-point native example is separate test data.

**Current Chrome capture — critically reviewed scene.** [12.3.31 · Connect current pages to the browser renderer](68ac-connect-current-pages-to-the-browser-renderer.md). Captured 2026-10-15.

![Chapter 12: Connect current pages to the browser renderer. Current Chrome capture — critically reviewed scene.](../screenshots/journey/68ac-connect-current-pages-to-the-browser-renderer-browser.png)

### 12.1 · Complete viewer files and published scene sources

- [12.1 · Complete viewer files and published scene sources](consolidation/66-read-scene-manifests.md)

### 12.2 · Locate original arrays without downloading their bodies

- [12.2 · Locate original arrays without downloading their bodies](consolidation/67-supply-checked-array-layout-readers.md)

### 12.3 · Bounded scheduling, exact replies and cancellation

- [12.3 · Bounded scheduling, exact replies and cancellation](consolidation/66za-own-original-range-read-permits.md)

### 12.4 · Charged residency and current streamed pages

- [12.4 · Charged residency and current streamed pages](consolidation/68-reserve-page-bytes-before-reading.md)

## 13 · Shared definitions and drawing sheets

**Automated Chrome checks: 0 of 30 checkpoints current.** Remaining checkpoints need fresh Chrome acceptance.

**Critical browser-scene review: 0 of 30 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Critical visual review pending.** This recorded reference is not an accepted visual-quality result.

**Previous native render — fresh verification pending.** [13.2.6 · Read original sheet metadata records](70za-read-original-sheet-metadata-records.md).

![Chapter 13: Read original sheet metadata records. Previous native render — fresh verification pending.](../screenshots/journey/70za-read-original-sheet-metadata-records.png)

### 13.1 · Shared original definitions and independently placed instances

- [13.1 · Shared original definitions and independently placed instances](consolidation/69-validate-original-definition-references.md)

### 13.2 · Independent presentation and reusable CAD preparation

- [13.2 · Independent presentation and reusable CAD preparation](consolidation/69j-preserve-per-instance-original-colours.md)

### 13.3 · Feature ancestry and canonical local-to-world placement

- [13.3 · Feature ancestry and canonical local-to-world placement](consolidation/70-evaluate-source-ancestry-once-per-document.md)

### 13.4 · Original sheet entity metadata

- [13.4 · Original sheet entity metadata](consolidation/70za-read-original-sheet-metadata-records.md)

## 14 · Coordinates and geometry tools

**Automated Chrome checks: 0 of 118 checkpoints current.** Remaining checkpoints need fresh Chrome acceptance.

**Critical browser-scene review: 0 of 118 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Critical visual review pending.** This recorded reference is not an accepted visual-quality result.

**Previous native render — fresh verification pending.** [14.6.5 · Exercise exact curves and rational control edits](76d-exercise-exact-curves-and-rational-control-edits.md).

![Chapter 14: Exercise exact curves and rational control edits. Previous native render — fresh verification pending.](../screenshots/journey/76d-exercise-exact-curves-and-rational-control-edits.png)

### 14.1 · Original drawings, entity queries and construction frames

- [14.1 · Original drawings, entity queries and construction frames](consolidation/71-read-original-sheet-entity-metadata.md)

### 14.2 · Precise coordinates and original drawing drafts

- [14.2 · Precise coordinates and original drawing drafts](consolidation/72-copy-current-construction-planes-and-coordinate-forms.md)

### 14.3 · Planar boundaries and checked solid foundations

- [14.3 · Planar boundaries and checked solid foundations](consolidation/73-construct-rectangles-and-polygons-in-plane-axes.md)

### 14.4 · Solid questions, previews and one original commit

- [14.4 · Solid questions, previews and one original commit](consolidation/74-copy-current-solid-geometry-and-question-definitions.md)

### 14.5 · Round solids and exact planar conics

- [14.5 · Round solids and exact planar conics](consolidation/75-reuse-current-round-solid-builders.md)

### 14.6 · Exact parabola recipes and editable freeform controls

- [14.6 · Exact parabola recipes and editable freeform controls](consolidation/76-copy-exact-planar-curve-definitions.md)

## 15 · Transforms and surface construction

**Automated Chrome checks: 0 of 81 checkpoints current.** Remaining checkpoints need fresh Chrome acceptance.

**Critical browser-scene review: 0 of 81 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Visual review: issue found.** The inherited network-surface image contains six original input curves and two surfaces; it does not show a complete NURBS isocurve display. The interior curves are faint and discontinuous. The current pixel check counts aggregate dark pixels and does not prove curve continuity or contrast. Visual quality is not accepted; a focused comparison and correction are required.

**Previous native render — fresh verification pending.** [15.4.6 · Read original projection planes](80za-read-original-projection-planes.md).

![Chapter 15: Read original projection planes. Previous native render — fresh verification pending.](../screenshots/journey/80za-read-original-projection-planes.png)

### 15.1 · Original transforms, part edits and solid recipes

- [15.1 · Original transforms, part edits and solid recipes](consolidation/77-copy-current-transformation-and-source-edit-algorithms.md)

### 15.2 · Profile collection and one surfacing transaction

- [15.2 · Profile collection and one surfacing transaction](consolidation/78-copy-current-curve-collection-and-surfacing-recipes.md)

### 15.3 · Lofts, rail sweeps and network foundations

- [15.3 · Lofts, rail sweeps and network foundations](consolidation/79-retain-current-loft-section-order-and-degree-options.md)

### 15.4 · Two-direction networks and corner patches

- [15.4 · Two-direction networks and corner patches](consolidation/80-build-a-surface-through-two-directions-of-original-curves.md)

## 16 · Geometry editing and measurement

**Automated Chrome checks: 0 of 132 checkpoints current.** Remaining checkpoints need fresh Chrome acceptance.

**Critical browser-scene review: 0 of 132 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Critical visual review pending.** This recorded reference is not an accepted visual-quality result.

**Previous native render — fresh verification pending.** [16.5.52 · Exercise current original commands together](85ay-exercise-current-original-commands-together.md).

![Chapter 16: Exercise current original commands together. Previous native render — fresh verification pending.](../screenshots/journey/85ay-exercise-current-original-commands-together.png)

### 16.1 · Projection, extension and stable edit targets

- [16.1 · Projection, extension and stable edit targets](consolidation/81-project-originals-through-the-current-plane-algorithms.md)

### 16.2 · Split original curves and joined faces

- [16.2 · Split original curves and joined faces](consolidation/82-divide-current-original-curve-families.md)

### 16.3 · Joined-face topology and exact surface measures

- [16.3 · Joined-face topology and exact surface measures](consolidation/83-split-original-faces-without-separating-the-solid.md)

### 16.4 · Measurements, temporary marks and retained CAD pieces

- [16.4 · Measurements, temporary marks and retained CAD pieces](consolidation/84-measure-original-curves-surfaces-and-closed-solids.md)

### 16.5 · Text, Plane, Trim and integrated editing commands

- [16.5 · Text, Plane, Trim and integrated editing commands](consolidation/85-copy-current-original-command-recipes.md)

## 17 · GPU visibility, clipping and sections

**Automated Chrome checks: 0 of 122 checkpoints current.** Remaining checkpoints need fresh Chrome acceptance.

**Critical browser-scene review: 0 of 122 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Critical visual review pending.** This recorded reference is not an accepted visual-quality result.

**Previous native render — fresh verification pending.** [17.3.32 · Exercise original sections bores and ink](88ad-exercise-original-sections-bores-and-ink.md).

![Chapter 17: Exercise original sections bores and ink. Previous native render — fresh verification pending.](../screenshots/journey/88ad-exercise-original-sections-bores-and-ink.png)

### 17.1 · Bounded projected triangles and tile lists

- [17.1 · Bounded projected triangles and tile lists](consolidation/86-copy-finite-tile-allocation-rules.md)

### 17.2 · Finite ink visibility and original colour/pick ownership

- [17.2 · Finite ink visibility and original colour/pick ownership](consolidation/86s-test-visibility-at-the-original-center-line.md)

### 17.3 · Original clipping half-spaces across drawing families

- [17.3 · Original clipping half-spaces across drawing families](consolidation/87-copy-current-original-plane-modes.md)

### 17.4 · Verified closed sections and exact cap picks

- [17.4 · Verified closed sections and exact cap picks](consolidation/88-copy-original-closedness-and-winding.md)

## 18 · Lighting, render quality and comparison

**Automated Chrome checks: 0 of 89 checkpoints current.** Remaining checkpoints need fresh Chrome acceptance.

**Critical browser-scene review: 0 of 89 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Critical visual review pending.** This recorded reference is not an accepted visual-quality result.

**Previous native render — fresh verification pending.** [18.7.7 · Prove cache invalidation and honest performance reports](95f-prove-cache-invalidation-and-honest-performance-reports.md).

![Chapter 18: Prove cache invalidation and honest performance reports. Previous native render — fresh verification pending.](../screenshots/journey/95f-prove-cache-invalidation-and-honest-performance-reports.png)

### 18.1 · A retained depth and radius pyramid

- [18.1 · A retained depth and radius pyramid](consolidation/89-copy-density-scaled-depth-reduction.md)

### 18.2 · Original contact metadata and ambient light

- [18.2 · Original contact metadata and ambient light](consolidation/90-copy-maintained-original-contact-radius.md)

### 18.3 · Section-aware outlines and one presentation composite

- [18.3 · Section-aware outlines and one presentation composite](consolidation/91-include-visible-section-coverage.md)

### 18.4 · Original Wood Elements and portable source records

- [18.4 · Original Wood Elements and portable source records](consolidation/92-link-current-wood-element-builders.md)

### 18.5 · Glass, X-ray and shared visibility rules

- [18.5 · Glass, X-ray and shared visibility rules](consolidation/93-classify-original-solids-independently-of-clipping.md)

### 18.6 · Matched rendering comparisons from original sources

- [18.6 · Matched rendering comparisons from original sources](consolidation/94-declare-the-rendering-comparison-contract.md)

### 18.7 · Measured frame costs and dependency-aware cache reports

- [18.7 · Measured frame costs and dependency-aware cache reports](consolidation/95-record-comparable-large-scene-conditions.md)

## 19 · Integration and original ownership

**Automated Chrome checks: 0 of 34 checkpoints current.** Remaining checkpoints need fresh Chrome acceptance.

**Critical browser-scene review: 0 of 34 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Critical visual review pending.** This recorded reference is not an accepted visual-quality result.

**Previous native render — fresh verification pending.** [19.4.7 · Rebuild an edited Element without replacing its owner](96zg-rebuild-an-edited-element-without-replacing-its-owner.md).

![Chapter 19: Rebuild an edited Element without replacing its owner. Previous native render — fresh verification pending.](../screenshots/journey/96zg-rebuild-an-edited-element-without-replacing-its-owner.png)

### 19.1 · Metadata and current command contracts

- [19.1.1 · Retain non drawing originals and validate raw planes](96-retain-non-drawing-originals-and-validate-raw-planes.md)
- [19.1.2 · Merge unused definitions with displayed instances](96a-merge-unused-definitions-with-displayed-instances.md)
- [19.1.3 · Preserve component and interaction metadata in archives](96b-preserve-component-and-interaction-metadata-in-archives.md)
- [19.1.4 · Keep current Element and frame definition families](96c-keep-current-element-and-frame-definition-families.md)
- [19.1.5 · Observe the real current registry and parser](96d-observe-the-real-current-registry-and-parser.md)
- [19.1.6 · Verify the complete declared current source closure](96e-verify-the-complete-declared-current-source-closure.md)
- [19.1.7 · Report retained original metadata in the real viewer](96f-report-retained-original-metadata-in-the-real-viewer.md)
- [19.1.8 · Prove exact metadata and definition round trips](96g-prove-exact-metadata-and-definition-round-trips.md)
- [19.1.9 · Prove current source grammar and explicit differences](96h-prove-current-source-grammar-and-explicit-differences.md)

### 19.2 · Elements and feature hierarchies

- [19.2.1 · Separate own attributes from hosted features](96i-separate-own-and-hosted-element-features.md)
- [19.2.2 · Group pieces by their original interaction](96j-group-features-by-original-interaction.md)
- [19.2.3 · Keep nested Elements editable and projections unsaved](96k-separate-derived-rows-from-editable-elements.md)
- [19.2.4 · Attach a bounded feature tree to its host](96l-attach-a-bounded-derived-feature-tree.md)
- [19.2.5 · Project original outlines without changing their coordinates](96m-project-original-feature-outlines.md)
- [19.2.6 · Connect imported Elements to their feature hierarchy](96n-connect-imported-interaction-hierarchies.md)
- [19.2.7 · Use the same projection path for created Elements](96o-reuse-projections-for-created-elements.md)
- [19.2.8 · Build an original interaction specimen](96p-build-an-imported-interaction-specimen.md)
- [19.2.9 · Prove imported feature ownership and round trips](96q-prove-imported-feature-ownership-and-roundtrips.md)
- [19.2.10 · Prove exact commands across the expanded vocabulary](96r-preserve-exact-command-completions.md)

### 19.3 · Selecting, copying and presenting originals

- [19.3.1 · Resolve feature rows to original Element owners](96s-resolve-feature-rows-to-original-element-owners.md)
- [19.3.2 · Select and highlight original feature owners](96t-select-and-highlight-original-feature-owners.md)
- [19.3.3 · Fit original box corners to the free canvas](96u-fit-original-box-corners-to-the-free-canvas.md)
- [19.3.4 · Project definition and per-instance features](96v-project-definition-and-per-instance-features.md)
- [19.3.5 · Detach copied original source identity](96w-detach-copied-original-source-identity.md)
- [19.3.6 · Regenerate display for copied feature owners](96x-regenerate-display-for-copied-feature-owners.md)
- [19.3.7 · Address feature pixels to original CAD owners](96y-address-feature-pixels-to-original-cad-owners.md)
- [19.3.8 · Preserve source feature visibility and pixel styles](96z-preserve-source-feature-visibility-and-pixel-styles.md)

### 19.4 · Editing original controls

- [19.4.1 · Address original controls with family keys](96za-address-original-controls-with-family-keys.md)
- [19.4.2 · Draw and pick original CAD control points](96zb-draw-and-pick-original-cad-control-points.md)
- [19.4.3 · Share the current control-point style](96zc-share-the-current-control-point-style.md)
- [19.4.4 · Reuse world controls for their display nets](96zd-reuse-world-controls-for-their-display-nets.md)
- [19.4.5 · Scope controls to the selected original](96ze-scope-controls-to-the-selected-original.md)
- [19.4.6 · Edit original controls through the source preview](96zf-edit-original-controls-through-the-source-preview.md)
- [19.4.7 · Rebuild an edited Element without replacing its owner](96zg-rebuild-an-edited-element-without-replacing-its-owner.md)

## 20 · Shared GPU rendering

**Automated Chrome checks: 11 of 95 checkpoints current.** Remaining checkpoints need fresh Chrome acceptance.

**Critical browser-scene review: 0 of 95 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Critical visual review pending.** This recorded reference is not an accepted visual-quality result.

**Current supporting native render — critical image review pending.** [20.12.5 · Retire empty arena ranges after tail trimming](96zzzzx-retire-empty-arena-ranges-after-tail-trimming.md).

![Chapter 20: Retire empty arena ranges after tail trimming. Current supporting native render — critical image review pending.](../screenshots/journey/96zzzzx-retire-empty-arena-ranges-after-tail-trimming.png)

### 20.1 · Shared geometry and primitive addresses

- [20.1.1 · Bind the shared geometry as storage](96zh-bind-the-shared-geometry-as-storage.md)
- [20.1.2 · Read source vertices through one shader helper](96zi-read-source-vertices-through-one-shader-helper.md)
- [20.1.3 · Activate shared vertex pulling for color](96zj-activate-shared-vertex-pulling-for-color.md)
- [20.1.4 · Assign frame triangle ranges](96zk-assign-frame-triangle-ranges.md)
- [20.1.5 · Own the integer primitive image](96zl-own-the-integer-primitive-image.md)
- [20.1.6 · Write triangle addresses beside color](96zm-write-triangle-addresses-beside-color.md)

### 20.2 · Surface-fitted finite ink

- [20.2.1 · Bind visible triangle addresses for ink](96zn-bind-visible-triangle-addresses-for-ink.md)
- [20.2.2 · Describe ink depth and surface slack](96zo-describe-ink-depth-and-surface-slack.md)
- [20.2.3 · Fit a neighboring surface along the stroke](96zp-fit-a-neighboring-surface-along-the-stroke.md)
- [20.2.4 · Own a cropped primitive image for picks](96zq-own-a-cropped-primitive-image-for-picks.md)
- [20.2.5 · Share crop and clipping bindings for pulled picks](96zr-share-crop-and-clipping-bindings-for-pulled-picks.md)
- [20.2.6 · Write first-pick primitives from shared geometry](96zs-write-first-pick-primitives-from-shared-geometry.md)
- [20.2.7 · Fit surface depth beneath a point disc](96zt-fit-surface-depth-beneath-a-point-disc.md)
- [20.2.8 · Test the disc center against a surface](96zu-test-the-disc-center-against-a-surface.md)
- [20.2.9 · Carry the rasterized triangle gradient to ink](96zv-carry-the-rasterized-triangle-gradient-to-ink.md)
- [20.2.10 · Connect surface-fitted point visibility](96zw-connect-surface-fitted-point-visibility.md)
- [20.2.11 · Test rasterized neighbors at the stroke source](96zx-test-the-rasterized-neighbors-at-the-stroke-source.md)
- [20.2.12 · Connect fitted surfaces and exact stroke occlusion](96zy-connect-fitted-surfaces-and-exact-stroke-occlusion.md)
- [20.2.13 · Separate hidden ink color from pick ownership](96zz-separate-hidden-ink-color-from-pick-ownership.md)
- [20.2.14 · Connect section primitives to color and first picking](96zza-connect-section-primitive-images.md)
- [20.2.15 · Read section visibility from one image](96zzb-read-section-visibility-from-one-image.md)

### 20.3 · Shared physical inputs and packed formats

- [20.3.1 · Share section addresses with outlines and ambient shading](96zzc-share-section-addresses-with-outlines-and-ambient.md)
- [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md)
- [20.3.3 · Retain RGBA in prepared displays](96zze-retain-rgba-in-prepared-displays.md)

### 20.4 · Browser display defaults

- [20.4.1 · Start with current browser display defaults](96zzf-start-with-current-browser-display-defaults.md)

### 20.5 · Shared mesh arenas

- [20.5.1 · Grow buffers without losing rows](96zzg-grow-buffers-without-losing-rows.md)
- [20.5.2 · Prepare shared compact mesh ranges](96zzh-prepare-shared-compact-mesh-ranges.md)
- [20.5.3 · Upload compact ranges to shared GPU storage](96zzi-upload-compact-ranges-to-shared-gpu-storage.md)
- [20.5.4 · Share the object record layout](96zzj-share-the-object-record-layout.md)
- [20.5.5 · Borrow the current arena buffers](96zzk-borrow-the-current-arena-buffers.md)
- [20.5.6 · Read shared ranges through object records](96zzl-read-shared-ranges-through-object-records.md)
- [20.5.7 · Connect the renderer to one mesh arena](96zzm-connect-the-renderer-to-one-mesh-arena.md)
- [20.5.8 · Describe eight-byte draw slots](96zzn-describe-eight-byte-draw-slots.md)
- [20.5.9 · Read instance properties through draw slots](96zzo-read-instance-properties-through-draw-slots.md)

### 20.6 · Current file and command behavior

- [20.6.1 · Classify contact fill shading](96zzp-classify-contact-fill-shading.md)
- [20.6.2 · Open files through one measured Session reader](96zzq-open-files-through-one-measured-session-reader.md)
- [20.6.3 · Route current Close and Opacity commands](96zzr-route-current-close-and-opacity-commands.md)
- [20.6.4 · Reuse full source preparation for editor and reload](96zzs-reuse-full-source-preparation-for-editor-and-reload.md)
- [20.6.5 · Share the current selection yellow](96zzt-share-the-current-selection-yellow.md)

### 20.7 · Draw slots and primitive lookup

- [20.7.1 · Separate primitive addresses from selection](96zzu-separate-primitive-addresses-from-selection.md)
- [20.7.2 · Bind the own-row slot for ordinary surfaces](96zzv-bind-the-own-row-slot-for-ordinary-surfaces.md)
- [20.7.3 · Reload scenes through the existing publication owner](96zzw-reload-scenes-through-the-existing-publication-owner.md)
- [20.7.4 · Update only changed slot runs](96zzx-update-only-changed-slot-runs.md)
- [20.7.5 · Share one slot buffer across surface draws](96zzy-share-one-slot-buffer-across-surface-draws.md)
- [20.7.6 · Prepare the integer primitive-slot texture](96zzz-prepare-the-integer-primitive-slot-texture.md)
- [20.7.7 · Describe slot and triangle runs](96zzza-describe-slot-and-triangle-runs.md)
- [20.7.8 · Read slot runs and lookup texels](96zzzb-read-slot-runs-and-lookup-texels.md)
- [20.7.9 · Reuse and merge free slot ranges](96zzzc-reuse-and-merge-free-slot-ranges.md)
- [20.7.10 · Follow arena growth with slot headroom](96zzzd-follow-arena-growth-with-slot-headroom.md)
- [20.7.11 · Report only changed instance slots](96zzze-report-only-changed-instance-slots.md)
- [20.7.12 · Project triangles at their physical addresses](96zzzf-project-triangles-at-their-physical-addresses.md)

### 20.8 · Background and shared object records

- [20.8.1 · Clear the current Arctic background](96zzzg-clear-the-current-arctic-background.md)
- [20.8.2 · Share changed record runs](96zzzh-share-changed-record-runs.md)
- [20.8.3 · Prepare one shared object record table](96zzzi-prepare-one-shared-object-record-table.md)
- [20.8.4 · Share instance object records across definitions](96zzzj-share-instance-object-records-across-definitions.md)
- [20.8.5 · Reuse checked curve control coordinates](96zzzk-reuse-checked-curve-control-coordinates.md)
- [20.8.6 · Publish allocator texels through the integer table](96zzzl-publish-allocator-texels-through-the-integer-table.md)
- [20.8.7 · Resolve physical triangles with integer slots](96zzzm-resolve-physical-triangles-with-integer-slots.md)
- [20.8.8 · Batch shared geometry with original ownership](96zzzn-batch-shared-geometry-with-original-ownership.md)
- [20.8.9 · Shrink record ranges without rewriting live prefixes](96zzzo-shrink-record-ranges-without-rewriting-live-prefixes.md)
- [20.8.10 · Share opaque ordinary surface records](96zzzp-share-opaque-ordinary-surface-records.md)
- [20.8.11 · Blend ordinary surface records](96zzzq-blend-ordinary-surface-records.md)
- [20.8.12 · Share transparent instance records](96zzzr-share-transparent-instance-records.md)

### 20.9 · Stable records and original picking

- [20.9.1 · Allocate stable object record rows](96zzzs-allocate-stable-object-record-rows.md)
- [20.9.2 · Retain live surface record rows](96zzzt-retain-live-surface-record-rows.md)
- [20.9.3 · Prepare shared record pick consumers](96zzzu-prepare-shared-record-pick-consumers.md)
- [20.9.4 · Pick originals through shared records](96zzzv-pick-originals-through-shared-records.md)
- [20.9.5 · Read source faces through shared records](96zzzw-read-source-faces-through-shared-records.md)
- [20.9.6 · Pack existing ambient radius in object records](96zzzx-pack-existing-ambient-radius-in-object-records.md)
- [20.9.7 · Read ambient properties through shared records](96zzzy-read-ambient-properties-through-shared-records.md)

### 20.10 · Projection and cap consumers

- [20.10.1 · Share the object record shader schema](96zzzz-share-the-object-record-shader-schema.md)
- [20.10.2 · Prepare shared record triangle projection](96zzzza-prepare-shared-record-triangle-projection.md)
- [20.10.3 · Project triangles through shared records](96zzzzb-project-triangles-through-shared-records.md)
- [20.10.4 · Prepare shared record cap counting](96zzzzc-prepare-shared-record-cap-counting.md)
- [20.10.5 · Prepare cap plane and arena bindings](96zzzzd-prepare-cap-plane-and-arena-bindings.md)
- [20.10.6 · Count cap color through shared records](96zzzze-count-cap-color-through-shared-records.md)
- [20.10.7 · Prepare shared record cap picks](96zzzzf-prepare-shared-record-cap-picks.md)
- [20.10.8 · Own shared record cap pick resources](96zzzzg-own-shared-record-cap-pick-resources.md)
- [20.10.9 · Pick caps through shared records](96zzzzh-pick-caps-through-shared-records.md)

### 20.11 · Retained allocation and lookup projection

- [20.11.1 · Adapt incremental slots to pulled draws](96zzzzi-adapt-incremental-slots-to-pulled-draws.md)
- [20.11.2 · Prepare exact arena slot definitions](96zzzzj-prepare-exact-arena-slot-definitions.md)
- [20.11.3 · Retain live allocator member runs](96zzzzk-retain-live-allocator-member-runs.md)
- [20.11.4 · Prepare one plan for draw slots and lookup](96zzzzl-prepare-one-plan-for-draw-slots-and-lookup.md)
- [20.11.5 · Prepare one owner for live address updates](96zzzzm-prepare-one-owner-for-live-address-updates.md)
- [20.11.6 · Adopt retained GPU slots and physical bases](96zzzzn-adopt-retained-gpu-slots-and-physical-bases.md)
- [20.11.7 · Separate projection rows and arena offsets](96zzzzo-separate-projection-rows-and-arena-offsets.md)
- [20.11.8 · Prepare lookup-selected triangle projection](96zzzzp-prepare-lookup-selected-triangle-projection.md)
- [20.11.9 · Prepare the lookup projection pipeline](96zzzzq-prepare-the-lookup-projection-pipeline.md)
- [20.11.10 · Own current lookup projection bindings](96zzzzr-own-current-lookup-projection-bindings.md)
- [20.11.11 · Project live triangles through the lookup](96zzzzs-project-live-triangles-through-the-lookup.md)

### 20.12 · Arena reclamation

- [20.12.1 · Plan reusable arena spans](96zzzzt-plan-reusable-arena-spans.md)
- [20.12.2 · Reuse retired arena record ranges](96zzzzu-reuse-retired-arena-record-ranges.md)
- [20.12.3 · Upload reused arena spans in place](96zzzzv-upload-reused-arena-spans-in-place.md)
- [20.12.4 · Reclaim ranges after the last GPU lease](96zzzzw-reclaim-ranges-after-the-last-gpu-lease.md)
- [20.12.5 · Retire empty arena ranges after tail trimming](96zzzzx-retire-empty-arena-ranges-after-tail-trimming.md)

## 21 · Precise camera and browser coordinates

**Automated Chrome checks: 30 of 30 checkpoints current.**

**Critical browser-scene review: 3 of 30 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Critical browser-scene review accepted.** Actual asserted projection scene: the original line endpoints visible in the cropped orthographic Box remain visible after the current perspective refit. Separate trusted-wheel and explicit logical-pixel/line/page/overflow controls pass at actual DPR 1, 2 and 3, including render cap 2 at DPR 3. Original geometry and exact repeated frames are retained.

**Current Chrome capture — critically reviewed scene.** [21.5.6 · Match wheel input and framed projection](96zzzzzzf-match-wheel-input-and-framed-projection.md). Captured 2026-10-15.

![Chapter 21: Match wheel input and framed projection. Current Chrome capture — critically reviewed scene.](../screenshots/journey/96zzzzzzf-match-wheel-input-and-framed-projection-browser.png)

### 21.1 · Units and shared local coordinates

- [21.1.1 · Prepare unit-aware anchored camera space](96zzzzy-prepare-unit-aware-anchored-camera-space.md)
- [21.1.2 · Prepare precise anchor-relative object records](96zzzzz-prepare-precise-anchor-relative-object-records.md)
- [21.1.3 · Prepare anchor-relative original ink](96zzzzza-prepare-anchor-relative-original-ink.md)
- [21.1.4 · Prepare shared-origin frame inputs](96zzzzzb-prepare-shared-origin-frame-inputs.md)
- [21.1.5 · Prepare cloud and sheet frame placements](96zzzzzc-prepare-cloud-and-sheet-frame-placements.md)
- [21.1.6 · Prepare local temporary overlays](96zzzzzd-prepare-local-temporary-overlays.md)
- [21.1.7 · Preserve local precision before source placement](96zzzzze-preserve-local-precision-before-source-placement.md)
- [21.1.8 · Prepare precise construction-grid origins](96zzzzzf-prepare-precise-construction-grid-origins.md)
- [21.1.9 · Initialize object records in the local frame](96zzzzzg-initialize-object-records-in-the-local-frame.md)
- [21.1.10 · Prepare original highlights in the shared local frame](96zzzzzh-prepare-anchor-relative-original-highlights.md)
- [21.1.11 · Verify the shared unit through original history](96zzzzzi-share-one-camera-and-measurement-unit.md)

### 21.2 · CPU queries and asynchronous picks

- [21.2.1 · Project CPU queries around the precise target](96zzzzzj-project-cpu-queries-around-the-precise-target.md)
- [21.2.2 · Prepare precise text selection polygons](96zzzzzk-prepare-precise-text-selection-polygons.md)
- [21.2.3 · Connect precise rectangle selection queries](96zzzzzl-connect-precise-rectangle-selection-queries.md)
- [21.2.4 · Finish precise lasso and handle queries](96zzzzzm-finish-precise-lasso-and-handle-queries.md)
- [21.2.5 · Stamp asynchronous picks with the scene origin](96zzzzzn-stamp-asynchronous-picks-with-the-origin.md)
- [21.2.6 · Retry rebased picks with the latest cursor](96zzzzzo-retry-rebased-picks-with-the-latest-cursor.md)
- [21.2.7 · Share derived inputs for origin refresh](96zzzzzp-share-derived-inputs-for-origin-refresh.md)

### 21.3 · Browser origin lifetime

- [21.3.1 · Prepare one shared render origin](96zzzzzq-prepare-one-shared-render-origin.md)
- [21.3.2 · Prepare browser frame matrices and owned wakes](96zzzzzr-prepare-browser-frame-matrices-and-owned-wakes.md)
- [21.3.3 · Adopt the shared origin in the browser](96zzzzzs-adopt-the-shared-origin-in-the-browser.md)

### 21.4 · Reversed depth and near clipping

- [21.4.1 · Share depth endpoints and comparisons](96zzzzzt-share-depth-endpoints-and-comparisons.md)
- [21.4.2 · Share homogeneous near-plane ink clipping](96zzzzzu-share-homogeneous-near-plane-ink-clipping.md)
- [21.4.3 · Use reversed depth across the frame](96zzzzzz-use-reversed-depth-across-the-frame.md)

### 21.5 · Physical camera, grid and navigation

- [21.5.1 · Match physical camera defaults and limits](96zzzzzza-match-physical-camera-defaults-and-limits.md)
- [21.5.2 · Omit unused background pick pipelines](96zzzzzzb-omit-unused-background-pick-pipelines.md)
- [21.5.3 · Draw the construction grid as physical lines](96zzzzzzc-draw-the-construction-grid-as-physical-lines.md)
- [21.5.4 · Build viewing rays without matrix inversion](96zzzzzzd-build-viewing-rays-without-matrix-inversion.md)
- [21.5.5 · Grow camera depth from cached world bounds](96zzzzzze-grow-camera-depth-from-cached-world-bounds.md)
- [21.5.6 · Match wheel input and framed projection](96zzzzzzf-match-wheel-input-and-framed-projection.md)

## 22 · Surface lighting, contact and display quality

**Automated Chrome checks: 37 of 38 checkpoints current.** Remaining checkpoints need fresh Chrome acceptance.

**Critical browser-scene review: 37 of 38 checkpoints recorded.** A supporting image is a reference, not approval of the entire chapter.

**Critical browser-scene review accepted.** Actual controlled single-sample contact regression fixture. Separately labelled actual Browser figures and independent first-frame GPU/command/query controls are required for the new presentation adapter. Whole-course feature review and later camera/grid fixes remain pending.

**Current Chrome capture — critically reviewed scene.** [22.6.4 · Apply semantic view controls](96zzzzzzzp-apply-semantic-view-controls.md). Captured 2026-10-17.

![Chapter 22: Apply semantic view controls. Current Chrome capture — critically reviewed scene.](../screenshots/journey/96zzzzzzzp-apply-semantic-view-controls-browser.png)

### 22.1 · Surface colors and lighting

- [22.1.1 · Share flat and Arctic surface frames](96zzzzzzg-share-current-surface-lighting-and-camera-uniforms.md)
- [22.1.2 · Connect headlight and backface controls](96zzzzzzh-connect-headlight-and-backface-controls.md)

### 22.2 · Contact rendering and shared frames

- [22.2.1 · Share one frame across color and ink](96zzzzzzi-share-one-frame-across-color-and-ink.md)
- [22.2.2 · Fill original contact polygons](96zzzzzzj-fill-original-contact-polygons.md)
- [22.2.3 · Broadcast ordinary ink metadata](96zzzzzzk-broadcast-ordinary-ink-metadata.md)
- [22.2.4 · Layer original contact boundaries](96zzzzzzl-layer-original-contact-boundaries.md)
- [22.2.5 · Query original faces under contact overlays](96zzzzzzm-query-original-faces-under-contact-overlays.md)

### 22.3 · Original edge pens, joins and facing

- [22.3.1 · Respect original boundary pens](96zzzzzzn-respect-original-boundary-pens.md)
- [22.3.2 · Close original boundary joins](96zzzzzzo-close-original-boundary-joins.md)
- [22.3.3 · Draw default mesh vertex dots](96zzzzzzp-draw-default-mesh-vertex-dots.md)
- [22.3.4 · Taper crowded original mesh vertices](96zzzzzzq-taper-crowded-original-mesh-vertices.md)

### 22.4 · Geometric ambient inputs and pass order

- [22.4.1 · Share geometric ambient inputs](96zzzzzzr-share-geometric-ambient-inputs.md)
- [22.4.2 · Shade surfaces before drawing ink](96zzzzzzs-shade-surfaces-before-drawing-ink.md)
- [22.4.3 · Keep contact ambient inputs on originals](96zzzzzzt-keep-contact-ambient-inputs-on-originals.md)

### 22.5 · Frame resources and multisampling

- [22.5.1 · Keep depth and address samples separate](96zzzzzzu-keep-depth-and-address-samples-separate.md)
- [22.5.2 · Reuse coherent frame targets](96zzzzzzv-reuse-coherent-frame-targets.md)
- [22.5.3 · Keep resolved colours in the canvas format](96zzzzzzw-match-resolve-base-and-view-formats.md)
- [22.5.4 · Reuse surface pipelines for each sample count](96zzzzzzx-reuse-count-specific-surface-pipelines.md)
- [22.5.5 · Generate ink shaders for physical samples](96zzzzzzy-generate-ink-for-physical-samples.md)
- [22.5.6 · Cache matching ink pipelines and bindings](96zzzzzzz-cache-ink-pipelines-with-independent-queries.md)
- [22.5.7 · Retarget existing overlay lanes](96zzzzzzza-retarget-existing-overlay-lanes.md)
- [22.5.8 · Read section crossings at each sample](96zzzzzzzb-read-section-crossings-at-each-sample.md)
- [22.5.9 · Compose cloud colour, depth and queries](96zzzzzzzc-compose-cloud-depth-with-physical-addresses.md)
- [22.5.10 · Cache cloud and sheet sample recipes](96zzzzzzzca-cache-cloud-and-sheet-samples.md)
- [22.5.11 · Retarget cached text colour](96zzzzzzzd-retarget-cached-text.md)
- [22.5.12 · Cache section masks and the final outline blend](96zzzzzzze-cache-outline-section-recipes.md)
- [22.5.13 · Resolve float outline coverage](96zzzzzzzf-resolve-float-outline-coverage.md)
- [22.5.14 · Retarget original ambient properties](96zzzzzzzg-retarget-original-ambient-properties.md)
- [22.5.15 · Share ambient shader recipes](96zzzzzzzh-share-ambient-shader-recipes.md)
- [22.5.16 · Correct ambient at physical samples](96zzzzzzzi-correct-ambient-at-physical-samples.md)
- [22.5.17 · Cache physical depth-pyramid inputs](96zzzzzzzj-cache-physical-depth-pyramid-inputs.md)
- [22.5.18 · Retarget the complete colour frame](96zzzzzzzk-retarget-the-complete-colour-frame.md)
- [22.5.19 · Choose bounded frame quality](96zzzzzzzl-choose-bounded-frame-quality.md)

### 22.6 · Original display controls

- [22.6.1 · Retain visible instance ranges](96zzzzzzzm-retain-visible-instance-ranges.md)
- [22.6.2 · Prepare original ink families](96zzzzzzzn-prepare-original-ink-families.md)
- [22.6.3 · Show original ink families](96zzzzzzzo-show-original-ink-families.md)
- [22.6.4 · Apply semantic view controls](96zzzzzzzp-apply-semantic-view-controls.md)
- [22.6.5 · Draw original CAD wire curves](96zzzzzzzq-draw-original-cad-wire-curves.md)

## Reference comparison

[Compare with the maintained browser runtime](97-finish-with-the-maintained-browser-runtime.md). This comparison remains outside the learner coding route while implementation is unfinished.
