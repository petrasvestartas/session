# What the finished course must preserve

The destination is the current working viewer, including its local changes. This inventory is an acceptance contract, not a statement that the short course is finished. A source fingerprint detects changes; it does not prove visual or behavioral equivalence.

Reference: `viewer-2026-09-28-local`. 271 source files and 77 command implementation files are inventoried. Every one has a destination; the complete source remains in the existing reference course.

The September 29 [browser-recovery fixes](../debugging.md) must also be taught and verified before this contract is complete. Their production code is ahead of the frozen reference. Acceptance includes a downloadable diagnostic after device loss, recovery of saved context after an interrupted run, no GPU submissions after loss, and feature ownership that follows the Wood interaction's first argument.

[Course route and availability](../journey.md) · [Complete draft lesson checklist](roadmap.md) · [Release checks](release.md)

## Current viewer changes to teach

The white canvas fills the browser window from the first lesson. Teaching text stays in the documentation, and feature requests use the viewer’s styled command line.

The completed course must also teach these current requirements:

- Typing starts a command without first clicking the field. Feature shortcuts give way to named commands such as `View Top`, `View Side`, `View Show Edges` and `View Outline`.
- Mouse and phone camera gestures remain. A right click repeats the last command; a right drag moves the camera.
- New scenes start with element features off and face opacity at `0.95`. `Element Features On` reveals the features; `Opacity` changes the shared face setting.
- Loading and saving preserve each object’s `is_visible` and `is_locked`. Hidden objects stay hidden, and locked objects cannot be edited through selection.
- Object colours follow their tree node or nearest coloured parent unless explicitly overridden. Curved edges retain their own curve geometry.
- Congruent BReps within one document reuse a prepared display walk after rigid placement. Replay preserves row/edge identities, normals and facing data; mirrors and different shapes remain distinct. Recordings end with document preparation, and a translated full circle keeps the same display chord count.
- Lines remain readable at close zoom and shallow viewing angles. A passing ordinary-distance screenshot is insufficient for this check.

These changes extend the frozen inventory below. Their cumulative teaching and browser checks remain tracked in the course work; listing a requirement does not mark it complete.

## Feature destinations

| Existing section | Course | What must survive |
| --- | --- | --- |
| [00 · Load Rust in the browser](../00-environment.md) | 01 | Create the browser page and the Rust entry point. |
| [01 · First WebGPU frame](../01-first-frame.md) | 01 | Connect the GPU, allocate its shared resources and draw the first offscreen frame. |
| [02 · Camera](../02-camera.md) | 01 | Implement orbit, pan and zoom, and turn a camera into a projection. |
| [03 · Object rows and identity](../03-identity.md) | 01 | Give displayed objects stable rows, placement and identity. |
| [04a · Meshes on the GPU](../04a-meshes.md) | 01 | Pack meshes into shared GPU storage and draw indexed triangles. |
| [04b · Strokes and arrows](../04b-strokes.md) | 02 | Draw readable strokes and arrows with width and visibility rules. |
| [04c · Markers](../04c-markers.md) | 02 | Render points and control markers with identifiable owners. |
| [04d · Point clouds](../04d-clouds.md) | 02 | Organize point clouds and draw the detail needed for the current view. |
| [05 · Depth and visible ink](../05-visibility.md) | 02 | Trace how surfaces decide whether nearby ink is visible. |
| [06 · CAD face rules](../06-cad-contract.md) | 02 | Translate editable CAD geometry into display records with source identity. |
| [07 · Shared boundaries](../07-boundaries.md) | 02 | Follow the sampling and ordering of a shared CAD boundary. |
| [08 · Trims, holes and periodic seams](../08-trimming.md) | 02 | Trace trims and holes from surface coordinates to displayed faces. |
| [09 · Normals and shading](../09-normals.md) | 02 | Check face winding and the normals used for shading. |
| [10 · Text shaping](../10-text-layout.md) | 02 | Turn text into shaped glyph runs and measured positions. |
| [11 · Text rendering](../11-text-rendering.md) | 02 | Render shaped text, its coverage and its placement in the scene. |
| [12 · The viewer shell and picking](../12-picking.md) | 01 | Connect browser events, redraw requests and asynchronous picking. |
| [13 · Source controls and cloud picks](../13-controls.md) | 01 | Connect control selection and cloud picks to original source identities. |
| [14 · Loading scenes](../14-loading.md) | 03 | Load a scene description and publish its geometry to the viewer. |
| [15 · Publication and streamed reads](../15-publication.md) | 03 | Read large published files through metadata and selected byte ranges. |
| [16 · Resource accounting and release](../16-accounting.md) | 03 | Track shared CPU/GPU allocations and release resources at the right time. |
| [17 · Source faces, text objects and one silhouette](../17-source-presentation.md) | 03 | Present selected source faces, labels and a consistent silhouette. |
| [18 · Finite-triangle visibility](../18-finite-visibility.md) | 06 | Build screen-tile lists for precise triangle visibility tests. |
| [18a · Instancing](../18a-instancing.md) | 03 | Prepare shared geometry for drawing at multiple placements. |
| [18b · Clipping planes and section caps](../18b-clipping.md) | 06 | Apply clipping planes and draw the cut surfaces of eligible solids. |
| [19 · Sheets: batched drawings with lazy metadata](../19-sheets.md) | 03 | Draw batched sheets while looking up detailed entity information on demand. |
| [20 · The document and its rows](../20-history.md) | 04 | Connect document transactions, undo/redo and display synchronization. |
| [21 · Direct editing](../21-editing.md) | 04 | Implement direct editing, previews, cancellation and final commits. |
| [22 · The egui layer](../22-runtime-helpers.md) | 04 | Draw the egui interface above the scene and manage its resources. |
| [23 · The command line: one verb per file](../23-geometry-commands.md) | 05 | Connect command words and options to application actions. |
| [23a · Tools that ask for points](../23a-tools.md) | 05 | Build interactive tools that ask for a sequence of points or choices. |
| [23b · Shapes](../23b-shapes.md) | 05 | Construct shapes from the dimensions and frame collected by a tool. |
| [23c · Surfacing](../23c-surfacing.md) | 05 | Turn selected curves and surfacing choices into new surface geometry. |
| [23d · Annotate and measure](../23d-annotate-measure.md) | 05 | Measure geometry and display the resulting annotations. |
| [24 · Snapping](../24-placed-controls.md) | 04 | Connect the Snap command to the snapping system introduced during direct editing. |
| [25 · Draw a solid, readable gumball](../25-gumball.md) | 04 | Render identifiable gumball handles with a clear on-screen shape. |
| [26 · The nested session panel](../26-nested-panel.md) | 04 | Represent a nested scene as expandable panel rows. |
| [30 · The layers panel and the layer tree](../30-layer-tree.md) | 04 | Apply layer visibility and selection through shared source identities. |
| [31 · Split curves and faces](../31-splitting.md) | 05 | Connect splitting tools, cutters and the resulting geometry edits. |
| [32 · Ambient occlusion](../32-colors-lighting.md) | 06 | Estimate ambient occlusion using depth data and a depth pyramid. |
| [33 · GTAO, Arctic and Outline](../33-contact-shadows.md) | 06 | Connect Arctic, outline and contact-shading controls to render state. |
| [35 · Element features](../35-attributes.md) | 06 | Show or hide element features through the shared display state. |
| [36 · Translucent faces and Opacity](../36-translucent-faces.md) | 06 | Expose face opacity through the command and state layers. |
| [37 · Self-test: the finished viewer](../37-command-dock.md) | 07 | Exercise the finished viewer from a scene file through GPU output. |


## Quality acceptance

All cases below still need an acceptance run for the completed new course. Existing source and native checks give useful evidence, but do not replace browser and scene comparisons.

**Camera and browser:** Orbit, pan, zoom, resize and high-DPI rendering; keyboard focus, mouse and touch; device errors are visible.

**Identity and selection:** Objects, source faces, edges, points and controls map back to the correct original; stale asynchronous picks are rejected.

**CAD ink:** Shared boundaries, trims, holes, periodic seams, normals and finite-triangle occlusion preserve the crafted edge appearance.

**Text:** Shaping, symbols, label placement, source text and outlines remain readable at the reference views.

**Large scenes:** Point clouds, streamed reads, sheets, instancing and released documents preserve IDs and bounded resource use.

**Transactions:** Preview, accept, cancel, undo and redo agree with the document and displayed geometry.

**Tools:** Every command implementation in the source inventory has a demonstrated invocation, result and cancellation/error case where applicable.

**Panels and input:** Layer trees, nested panels, command input, snapping and gumball agree on selection and respect pointer/keyboard ownership.

**Presentation:** Opaque and translucent faces, section caps, silhouettes, Arctic, ambient occlusion and feature controls match representative reference scenes.

**Persistence:** Save and open preserve supported geometry, placement, identity, layers and document data.

**Performance:** Compare frame time, peak GPU allocations and release-after-replacement on the same hardware, viewport and scene; record agreed tolerances before accepting differences.

**Release:** Browser gestures and visual comparisons pass, every feature is mapped, and any intentional source difference has a reviewed explanation.

## What “finished” means

Every feature above has cumulative lessons, every checkpoint builds and runs, and the final project passes the behavior and visual comparisons. Any source changes must be explained and evaluated against the reference. A smaller demonstration is an early milestone, never the final acceptance target.

The machine-readable `destination.json` records each source hash and its course owner. `npm --prefix ../session_tests run course -- structure` fails if a current source file is missing, changed or unmapped. Review an intentional change before updating the reference; never regenerate the baseline merely to silence the check.
