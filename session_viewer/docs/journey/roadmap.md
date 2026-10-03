# The complete draft lesson checklist

**Draft plan: 112 lesson slots; 44 current checkpoints have fresh build and Chrome evidence.** The four early dock lessons introduce the actual command line before camera and document commands. This plan may grow when a topic needs splitting. Published IDs remain stable.

The endpoint is fixed: the features and quality in [the destination contract](destination.md), including the later browser-recovery and interaction-ownership fixes. The current lessons are the opening of the course; this count is not a percentage of the implementation. Lesson count is not a measure of remaining engineering work.

A checked box means the implementation exists, the displayed code builds, its Rust checks pass where applicable, and its scripted Chrome checkpoint passed. It does not mean every production acceptance case has been covered. An unchecked box is a proposed tutorial, not content you can follow yet. [Read the exact verification scope](release.md).

Each future lesson will get its own typing estimate, architecture diagram, experiment, buildable endpoint and screenshot when it is authored. No reliable total study-time estimate exists for the unpublished lessons.

**Typing limit:** each lesson must fit within one hour of human typing, separately from reading and experiments. The [source-edit audit](typing-load.md) finds 17 current checkpoints over the conservative planning limit. Their build/browser checks remain valid, but they still need splitting before the course meets this teaching requirement. The proposed total will change.

- [x] Audit the added/changed code in all 44 current checkpoints, excluding unchanged context.
- [x] Display separate typing and combined study estimates at each checkpoint’s top.
- [ ] Split every over-limit checkpoint into complete runnable lessons; do not split inside functions.
- [ ] Recheck explanations, diagrams, commands, experiments, Rust builds, Chrome input and screenshots for each new endpoint.
- [ ] Publish each completed lesson and refresh the same existing Chrome tutorial tab, preserving the reader’s current lesson.

## Published checkpoints and current work

- [x] Move the work into this foreground session on GPT-6.1 Sol High.
- [x] Production typing focus, named View commands and right-click repeat verified in Chrome.
- [x] Phone orbit, two-finger pan, pinch zoom and touch cancellation verified in Chrome.
- [x] Production attributes/features default off and opacity 0.95 verified in Chrome.
- [x] Drawing options and snapping use typed commands; broader command-workspace regression passed.
- [x] Revise 03d and 22 for immediate typing and command-only keyboard features; recheck 23–26.
- [x] Refresh all changed earlier endpoints and their screenshot evidence.
- [x] [Review this week’s commits](weekly-changes.md) and map visibility, locking, tree colors and curve sampling to lessons.
- [x] [Fix missing ink at close zoom and sharp angles](line-visibility.md); geometric oracle and Chrome zoom checks pass. Teach this in the later rendering chapters.
- [x] Publish the verified tutorial and viewer input batches; refresh the same existing tutorial tab.

- [x] 01 · [A page that Rust can reach](01-canvas.md).

- [x] 02 · [Ask the GPU to paint](02-clear.md).

- [x] 03 · [Give the GPU three corners](03-triangle.md).

- [x] 03a · [Draw our command line](03a-panel.md): actual production fonts, field and GPU painting.

- [x] 03b · [Give the command line its memory](03b-state.md).

- [x] 03c · [Draw completion and history](03c-layout.md).

- [x] 03d · [Type into the real command dock](03d-input.md).

- [x] 04 · [Make a choice change the picture](04-input.md).

- [x] 05 · [Let Rust supply the corners](05-vertices.md).

- [x] 06 · [Share a corner between triangles](06-indices.md).

- [x] 07 · [Send one view setting to every corner](07-uniforms.md).

- [x] 08 · [Move the view, keep the geometry](08-camera.md).

- [x] 09 · [Let one matrix describe the view](09-matrices.md).

- [x] 10 · [Keep the nearest surface](10-depth.md).

- [x] 11 · [Give the scene an owner](11-scene.md).

- [x] 12 · [Name objects without depending on their row](12-identity.md).

- [x] 13 · [Ask which object is under the pointer](13-picking.md).

- [x] 14 · [Make document changes reversible](14-history.md).

- [x] 15 · [Look through a perspective camera](15-perspective.md).

- [x] 16 · [Walk around the model](16-orbit.md).

- [x] 17 · [Bring a solid into the scene](17-solid.md).

- [x] 18 · [Read the shape through light](18-light.md).

- [x] 19 · [Give every action the same route](19-actions.md).

- [x] 20 · [Keep a changing window in proportion](20-resize.md).

- [x] 21 · [Remember a press until it ends](21-gestures.md).

- [x] 22 · [Keep navigation on the mouse and commands in the dock](22-shortcuts.md).

- [x] 23 · [Keep the document behind the picture](23-import.md).

- [x] 24 · [Find the whole scene](24-fit.md).

- [x] 25 · [Choose how depth changes size](25-projection.md).

## A camera and scene ready for real documents

Camera, identity, loading and browser shell: reference 02, 03, 12, 14.

- [x] 26 · [Frame one object without changing its size](26-selected.md) — camera-only fitting and model coordinates; Rust, WebAssembly, GPU and Chrome checks passed.

- [x] 27 · [Give each object a placement](27-placement.md) — local vertices, identity initialization and affine validation.

- [x] 27a · [Ask geometry questions in world coordinates](27a-world.md) — placed bounds and ray picking.

- [x] 27b · [Apply object placement on the GPU](27b-model.md) — per-object uniform binding, local box geometry and world-space lighting.

- [x] 27c · [Move a placed object with a typed offset](27c-move.md) — finite offsets, world axes and one history transaction.

- [x] 27d · [Prove placement and history agree](27d-history.md) — shared mesh identity, Undo/Redo, errors, zero offsets and actual rendered picking.

- [x] 28 · [Prepare a display from an owned source mesh](28-record.md) — preserve double coordinates, attributes and identity before display conversion.

- [x] 28a · [Retain the imported mesh behind each row](28a-imported.md) — share the original session allocation and retain file provenance.

- [x] 28b · [Give generated objects the same source owner](28b-generated.md) — source geometry is required for every scene object.

- [x] 28c · [Prove source ownership survives editing](28c-ownership.md) — Move, Undo/Redo and row removal preserve source and display owners.

- [x] 29 · [Give each saved object a stable identity](29-identity.md) — keep duplicate imports distinct without changing original source GUIDs.

- [x] 29a · [Write a snapshot from the editable sources](29a-snapshot.md) — save live local geometry and separate placements without editing history.

- [x] 29b · [Reopen source geometry with its placement](29b-placements.md) — reject invalid, duplicate and orphaned matrices before construction.

- [x] 29c · [Prove the saved document reopens faithfully](29c-roundtrip.md) — exact doubles, names, visibility/locking and independent imported identities.

- [x] 29d · [Download the editable document from the command line](29d-save.md) — real Chrome download, unchanged Undo history, empty-save error and pixel-identical reopening.

- [ ] 30 · Open and replace documents safely — cancel stale reads and leave a failed load recoverable.

- [ ] 31 · Share GPU storage across objects — update changed ranges instead of rebuilding every mesh.

- [ ] 32 · Release a closed document — account for CPU objects, GPU allocations and shared references.

- [ ] 33 · Give browser listeners a lifetime — detach callbacks and cancel work when the viewer closes.

- [ ] 34 · Recover from GPU loss — stop submissions, download diagnostics and recover the previous run.

## Geometry beyond solid triangles

Strokes, markers, clouds and normals: reference 04b, 04c, 04d, 05, 09. Cloud streaming comes later.

- [ ] 35 · Draw readable thick lines — give a stroke width in screen pixels.

- [ ] 36 · Join strokes and draw arrowheads — keep the line owner through generated triangles.

- [ ] 37 · Draw points and control markers — size them on screen and retain their source IDs.

- [ ] 38 · Sample a curve for display — connect a kernel curve to an approximation we can inspect.

- [ ] 39 · Resolve surface and line visibility — keep depth comparisons consistent at edges.

- [ ] 40 · Carry face and vertex colours — preserve the source colour rules in display data.

- [ ] 41 · Read smooth and sharp surfaces — make normals and face winding visible and testable.

## Selection, editing and the application interface

Picking, source controls, history, editing, egui, snapping and panels: reference 12, 13, 20, 21, 22, 24, 25, 26, 30.

- [ ] 42 · Pick an object through a GPU ID image — read a result without blocking the drawing loop.

- [ ] 43 · Reject an old pick result — connect scene revisions to asynchronous work.

- [ ] 44 · Select a face, edge or control — map display IDs back to the editable source.

- [ ] 45 · Collect multiple selections — keep object and subobject selection rules explicit.

- [ ] 46 · Draw an egui panel — share the GPU while keeping input ownership clear.

- [ ] 47 · Route commands through the editor — parse a verb and report errors in one place.

- [ ] 48 · Build a tool that asks for points — show a preview before accepting a result.

- [ ] 49 · Snap a point to existing geometry — compare candidate points in a common coordinate frame.

- [ ] 50 · Commit or cancel a direct edit — keep previews out of document history.

- [ ] 51 · Move, rotate and scale with handles — draw and pick the gumball consistently.

- [ ] 52 · Expand a nested session tree — make each row lead back to its source.

- [ ] 53 · Control layers and visibility — keep the tree, selection and drawing in agreement.

- [ ] 54 · Complete browser navigation — test touch, focus changes, pointer cancellation and high DPI.

## CAD faces and readable text

CAD contract, shared boundaries, trimming, normals, source presentation and text: reference 06–11 and 17.

- [ ] 55 · Draw a parametric surface — keep surface coordinates beside world positions.

- [ ] 56 · Share a boundary between faces — give adjacent patches compatible samples.

- [ ] 57 · Trim a face and preserve holes — separate a surface from the region we keep.

- [ ] 58 · Cross a periodic seam — keep loops and samples continuous around a closed surface.

- [ ] 59 · Preserve CAD face identity — connect shaded triangles and selected source faces.

- [ ] 60 · Shape a line of text — turn characters into positioned glyphs.

- [ ] 61 · Draw glyph coverage — build the atlas and sample it without losing fine strokes.

- [ ] 62 · Place labels in the scene — keep source text, size and alignment meaningful.

- [ ] 63 · Draw one selected silhouette — avoid outlines on internal tessellation edges.

## Large documents and shared geometry

Publication, byte ranges, accounting, instancing and sheets: reference 04d, 13, 15, 16, 18a, 19.

- [ ] 64 · Organize a point cloud — choose visible detail for the current view.

- [ ] 65 · Pick a cloud point — recover its original identity from the displayed subset.

- [ ] 66 · Publish a scene with metadata — separate the small index from large geometry blocks.

- [ ] 67 · Read only needed byte ranges — handle cancellation and failed partial reads.

- [ ] 68 · Bound the streaming cache — evict data without releasing resources still in use.

- [ ] 69 · Draw many placements of one definition — share geometry while preserving instance IDs.

- [ ] 70 · Keep nested instance transforms correct — apply placement once at each level.

- [ ] 71 · Open a drawing sheet — batch its display while loading entity metadata on demand.

## The existing modelling toolbox

Every command implementation is covered, including command errors, previews and cancellation: reference 23, 23a–23d, 31. These families will split into more lessons where necessary; no command may disappear inside a family label.

- [ ] 72 · Draw points, lines and polylines — reuse the same point-collection tool.

- [ ] 73 · Build planar shapes — collect dimensions and an orientation frame.

- [ ] 74 · Build box and prism families — connect parameters to solid geometry.

- [ ] 75 · Build round and curved solids — teach the existing cylinder, cone, sphere and torus families.

- [ ] 76 · Create and edit freeform curves — retain their editable controls.

- [ ] 77 · Copy and transform geometry — preserve source identity rules and undo grouping.

- [ ] 78 · Extrude and revolve a profile — turn a collected profile into a surface or solid.

- [ ] 79 · Loft and sweep curves — explain the inputs for the existing surfacing commands.

- [ ] 80 · Build a surface network and patch — connect boundary choices to the generated face.

- [ ] 81 · Project and extend geometry — preview the target and handle unsuccessful results.

- [ ] 82 · Split a curve — keep the pieces and undo the operation as one change.

- [ ] 83 · Split a face — follow the cutter from input through resulting trimmed faces.

- [ ] 84 · Measure and annotate — keep numerical results and displayed annotations together.

- [ ] 85 · Complete the command inventory — demonstrate every remaining registered verb, option and error case.

## The rendering quality of the current viewer

Finite visibility, clipping, ambient occlusion, outline modes, features and opacity: reference 18, 18b, 32, 33, 35, 36.

- [ ] 86 · Compare finite triangles in screen tiles — preserve precise CAD ink visibility.

- [ ] 87 · Clip with a plane — apply the same clipping rule to drawing and picking.

- [ ] 88 · Cap a cut solid — show a section surface with correct ownership.

- [ ] 89 · Build a depth pyramid — prepare the input for ambient occlusion.

- [ ] 90 · Add ambient and contact shading — tune against fixed reference scenes.

- [ ] 91 · Connect Arctic and Outline modes — keep presentation settings out of geometry.

- [ ] 92 · Show element features and interactions — make the first interaction argument own its attached geometry.

- [ ] 93 · Blend translucent faces — preserve depth and selection while controlling opacity.

## Prove the finished viewer and replace the old course

Full acceptance contract, all source files and every command: reference 37 plus the September 29 browser-recovery and interaction fixes.

- [ ] 94 · Compare the old and new viewer — fix cameras and scenes for visual and behaviour checks.

- [ ] 95 · Measure large-scene responsiveness — compare frame times, allocations and release on the same hardware.

- [ ] 96 · Audit the complete feature and source inventory — explain every remaining implementation difference.

- [ ] 97 · Run the full browser course and retire the old tutorials — remove obsolete pages only after replacement checks pass.

## How the plan stays complete

The [feature contract](destination.md) maps all 43 reference sections, 271 source/test files and 77 command implementation files. Those 77 files are not necessarily 77 independent user commands. The command lessons must enumerate the actual registered verbs and their options before that part is marked complete. Grouping several commands under a title does not satisfy their acceptance checks.

At each completed lesson we update this checklist and the course release evidence, publish the code, and provide its review URL. If a lesson becomes too large to understand as one connected change, we split its unchecked slot and explain the new sequence. We never reduce the feature target to make the count look smaller.

[Return to the available lessons](../journey.md)
