# Build a viewer you can explain

Learn the existing `session_viewer` implementation using the Session geometry kernel. Introduce production Rust and WGSL in dependency order, retaining its developed rendering algorithms. Early checkpoints establish the owners needed by later production paths. Work through coherent lessons and save at the retained intermediate checkpoints when you need a stopping point.

**Current release: 111 reader-facing lessons and 1,361 preserved implementation checkpoints.** All 22 chapters are consolidated, retaining 1,360 original coding checkpoint IDs. The maintained-runtime comparison remains a separate reference. The [source and coverage map](journey/production-course-map.md) tracks remaining implementation coverage. Check the [release evidence](journey/release.md) before starting; consolidation does not revalidate historical GPU evidence.

The [complete lesson checklist](journey/roadmap.md) lists **1361 authored checkpoints**. Use the [daily coding route](journey/daily-plan.md) to group small checkpoints. Its lesson checkboxes track fresh build and browser acceptance.

[Use the command dock](journey/command-line.md) from the early lessons to exercise features. Later lessons add canvas picking and gestures.

The white canvas fills the browser window from lesson 01. The runtime shows your drawing and the command dock; lesson titles and explanations stay in this documentation.

The [typing-load audit](journey/typing-load.md) measures the edits marked for typing. Those edits fit the one-hour limit. Supplied viewer implementation is still being converted into bounded coding lessons; excluding copied code from that estimate does not make the course complete.

Follow each lesson's purpose, code change and browser exercise. Experiments, supplied authoring fixtures and verification details expand when needed. The [course checklist](journey/roadmap.md) tracks the cumulative order and execution evidence.

## Find the part you need

For everyday use, read [commands and shortcuts](journey/command-line.md), the [current command grammar](journey/current-command-grammar.md) and the [floor/dome measurement recipe](journey/large-scenes.md). For implementation, begin at lesson 01 and follow its cumulative successors. These groups help you find a subject; later checkpoints retain earlier dependencies.

| Part | Subjects | First checkpoint |
| --- | --- | --- |
| GPU setup and command dock | 01–04 | [Rust and the first GPU frame](journey/consolidation/01-canvas.md) |
| Geometry, camera, picking and history | 05–26 | [Vertices](journey/05-vertices.md) |
| Placement, source loading and browser lifetime | 27–35 | [Placement](journey/27-placement.md) |
| Selection, tree and document panels | 36–50 | [Selection chain](journey/36-chain.md) |
| Tool input, handles and widgets | 51–65 | [Handles](journey/51-handles.md) |
| Large sources, shared instances and sheets | 66–71 | [Scene manifests](journey/66-read-scene-manifests.md) |
| Drawing, modelling, editing and measurements | 72–85 | [Construction planes](journey/72-copy-current-construction-planes-and-coordinate-forms.md) |
| Finite visibility, clipping and rendering quality | 86–95 | [Finite tile rules](journey/86-copy-finite-tile-allocation-rules.md) |
| Current source audit and maintained runtime | 96–97 | [Original-data inventory](journey/96-retain-non-drawing-originals-and-validate-raw-planes.md) |

For a maintenance review, use [current viewer behavior](journey/current-viewer.md), the [source inventory](journey/inventory.md), [latest commit audit](journey/latest-commit-audit.md) and [final handoff](journey/final-handoff.md). The final maintained-runtime comparison still has [implementation gaps](journey/rendering-differences.md).

## Cumulative checkpoints

<details>
<summary>Show all 1361 cumulative checkpoints</summary>

| Lesson | Working result |
| --- | --- |
| [1.1.1 · A page that Rust can reach](journey/01-canvas.md) | Fill the browser window with a canvas and let Rust signal that it has started. |
| [1.1.2 · Paint the first GPU frame](journey/01a-gpu.md) | Open a browser GPU and paint the whole canvas white. |
| [1.2.1 · Give browser presentation its own function](journey/02-clear.md) | Separate browser presentation from setup and add bounded sizing and error feedback. |
| [1.3.1 · Give the GPU three corners](journey/03-triangle.md) | Draw a pink triangle on the white background. |
| [1.3.2 · Prepare the command fonts and painter](journey/03a-fonts.md) | Create the font and GPU painter owner while keeping the triangle. |
| [1.3.3 · Paint command text over the scene](journey/03a-paint.md) | Draw a Command label over the triangle using the same GPU. |
| [1.3.4 · Lay out the command field](journey/03a-panel.md) | Draw the production command field beside its label. |
| [1.3.5 · Give the command field its memory](journey/03b-memory.md) | Store the field text and history in one owned model. |
| [1.3.6 · Prepare the dock completion helpers](journey/03b-state.md) | Give the upcoming layout a vocabulary contract and caret helpers. |
| [1.3.7 · Draw retained command history](journey/03c-history.md) | Draw retained history above the field. |
| [1.3.8 · Type directly into the command field](journey/03ca-typing.md) | Send the first printable key directly into the field. |
| [1.3.9 · Submit command text once](journey/03cb-submit.md) | Clear submitted text and retain one history entry. |
| [1.3.10 · Edit the command text](journey/03cc-edit.md) | Keep text selection, deletion and Escape in the field. |
| [1.3.11 · Recognize Help](journey/03cd-vocabulary.md) | Connect known names and useful unknown-command replies. |
| [1.3.12 · Complete a command name](journey/03ce-complete.md) | Select only the suggested suffix while retaining typed text. |
| [1.3.13 · Accept or cancel a completion](journey/03cf-accept.md) | Use Tab, Enter and Escape without losing input ownership. |
| [1.3.14 · Read editing keys before the field](journey/03cg-keys.md) | Consume command editing once and restore field focus. |
| [1.3.15 · Show matching commands](journey/03ch-popup.md) | Draw the matching names above the field. |
| [1.3.16 · Browse matching names](journey/03ci-browse.md) | Move through matching command names without changing the scene. |
| [1.3.17 · Prepare the dock before painting](journey/03cj-prepare.md) | Keep UI output owned until one GPU submission. |
| [1.3.18 · Remember where the dock is drawn](journey/03ck-rects.md) | Record actual widget bounds for mouse ownership. |
| [1.3.19 · Click the command dock](journey/03ck-pointer.md) | Use pointer ownership to edit, choose a name and fold history. |
| [1.3.20 · Scroll through command names](journey/03cl-wheel.md) | Keep scrolling over the dock inside the command list. |
| [1.3.21 · Share the production dock layout](journey/03c-layout.md) | Connect the existing row and history to one shared layout. |
| [1.3.22 · Align and measure the command row](journey/03co-align.md) | Align label and field text, and record the dock edge for later fitting. |
| [1.3.23 · Reserve command-row space for active tool values](journey/03cp-options.md) | Own active-tool value buttons beside the command field. |
| [1.3.24 · Return accepted application commands](journey/03cn-handoff.md) | Keep editing local and return accepted names to the caller. |
| [1.3.25 · Hand commands to the application](journey/03d-input.md) | Return accepted commands while keeping editing inside Panel. |
| [1.4.1 · Make a choice change the picture](journey/04-input.md) | Use a command to switch backgrounds without changing the triangle. |
| [2.1.1 · Let Rust supply the corners](journey/05-vertices.md) | Draw a rectangle from six positions stored in a GPU buffer. |
| [2.2.1 · Share a corner between triangles](journey/06-indices.md) | Draw a diamond from four positions and six small index numbers. |
| [2.3.1 · Send one view setting to every corner](journey/07-uniforms.md) | Scale and shift the diamond without changing its stored positions. |
| [2.4.1 · Move the view, keep the geometry](journey/08-camera.md) | Pan, zoom and reset a flat view through camera state. |
| [2.5.1 · Let one matrix describe the view](journey/09-matrices.md) | Rotate the flat view using a matrix, ready for the third dimension. |
| [2.6.1 · Keep the nearest surface](journey/10-depth.md) | Draw two overlapping triangles in depth, keeping the nearer one visible even when it is drawn first. |
| [2.6.2 · Give geometry a validated CPU owner](journey/10a-mesh.md) | Own validated mesh values and a scene collection before uploading them. |
| [2.7.1 · Draw the scene through GPU mesh owners](journey/11-scene.md) | Add and remove a mesh through scene data while reusing the renderer and camera. |
| [2.7.2 · Name objects independently of their rows](journey/11a-identity.md) | Keep object identity stable when removal changes vector order. |
| [2.8.1 · Display selection by object identity](journey/12-identity.md) | Select and delete objects using stable identities, then highlight the selected object. |
| [2.8.2 · Convert screen positions back to the scene](journey/12a-coordinates.md) | Undo the camera transform before querying CPU geometry. |
| [2.8.3 · Find the nearest triangle at a scene point](journey/12b-picking.md) | Query triangle coverage and depth without changing the scene. |
| [2.9.1 · Select the visible object with a mouse click](journey/13-picking.md) | Click a visible triangle to select its stable object ID, including after camera movement. |
| [2.9.2 · Retain reversible scene snapshots](journey/13a-history.md) | Restore scene contents without copying immutable mesh arrays or reusing IDs. |
| [2.10.1 · Run Undo and Redo from the command line](journey/14-history.md) | Undo and redo adding or deleting an object without changing the camera or losing identity. |
| [2.10.2 · Draw through a perspective camera](journey/14a-perspective.md) | Draw nearer geometry through a perspective view while keeping typed commands working. |
| [2.10.3 · Turn a screen point into a bounded ray](journey/14b-ray.md) | Recover a finite ray through the near-to-far segment from a screen position. |
| [2.11.1 · Pick the nearest surface through the view](journey/15-perspective.md) | View the scene in perspective and select surfaces with a ray that agrees with the camera. |
| [2.12.1 · Walk around the model](journey/16-orbit.md) | Orbit and tilt a perspective camera while keeping its target in place. |
| [3.1.1 · Bring a solid into the scene](journey/17-solid.md) | Create a kernel box, convert it to display data, and add it as one undoable scene object. |
| [3.2.1 · Read the shape through light](journey/18-light.md) | Shade the box faces according to their direction, using the same mesh and renderer. |
| [3.2.2 · Give application state one owner](journey/18a-editor.md) | Own scene, selection, camera, background and history in one browser-independent value. |
| [3.2.3 · Apply document and view actions in Rust](journey/18b-actions.md) | Apply actions through one editor while preserving view settings during history and reset. |
| [3.3.1 · Route browser input through the editor](journey/19-actions.md) | Move document actions into a browser-independent editor while keeping picking, undo and drawing working. |
| [3.3.2 · Measure a safe drawing size](journey/19a-viewport.md) | Convert CSS dimensions and display density to finite, bounded GPU pixel dimensions. |
| [3.4.1 · Resize canvas, depth and camera together](journey/20-resize.md) | Resize the drawing buffer, depth attachment and camera together, including on dense displays. |
| [3.4.2 · Remember the pointer that starts a drag](journey/20a-press.md) | Retain one pointer and produce movement relative to its previous position. |
| [3.4.3 · Finish or cancel a drag](journey/20b-release.md) | Release an owned pointer or cancel it without inventing a click. |
| [3.4.4 · Convert pointer motion to editor actions](journey/20c-motion.md) | Convert CSS pointer coordinates and movement into existing Pick and Orbit actions. |
| [3.5.1 · Connect captured pointers to the editor](journey/21-gestures.md) | Orbit with a right drag, pick with a left click, and stop safely when the pointer or window loses focus. |
| [3.5.2 · Convert wheel units to camera zoom](journey/21a-wheel.md) | Normalize pixels, lines and pages to bounded camera zoom without adding document history. |
| [3.6.1 · Deliver wheel input without keyboard feature shortcuts](journey/22-shortcuts.md) | Normalize wheel input and keep every keyboard feature command in the command dock. |
| [3.6.2 · Validate a raw mesh record](journey/22a-records.md) | Run the state checks. A valid box passes; non-finite coordinates and missing vertex references are rejected. |
| [3.6.3 · Validate session identity and build a sample file](journey/22b-session.md) | Validate a real Session and its source identities before browser import. |
| [3.6.4 · Decode and prepare the whole import](journey/22c-decode.md) | Run the state checks. Invalid bytes, oversized data and unsupported records are refused before scene insertion. |
| [3.6.5 · Commit an import as one undoable action](journey/22d-import.md) | Run the state checks. One Undo removes the whole file; Redo restores its IDs and shared source; a failed import preserves selection and redo. |
| [3.6.6 · Prepare asynchronous file delivery](journey/22e-file-reader.md) | Build and run the checkpoint. The current scene and commands remain available; file selection is connected next. |
| [3.7.1 · Open a file through the command line](journey/23-import.md) | Import a real mesh-session file, keep its source identity, and undo the whole import as one action. |
| [3.7.2 · Measure the displayed scene bounds](journey/23a-bounds.md) | Run the state checks. Known corners give the expected centre and radius; a scene with no objects has no bounds. |
| [3.7.3 · Fit the camera around the bounds](journey/23b-fit-camera.md) | Run the state checks. Every corner remains inside the view across scales and aspect ratios, and fitting preserves orientation. |
| [3.8.1 · Run Fit through the command line](journey/24-fit.md) | Frame all current objects without rotating them or changing the document. |
| [3.8.2 · Build perspective and orthographic camera matrices](journey/24a-projection.md) | Run the state checks. Switching preserves target-plane scale; orthographic size ignores depth and its pick rays are parallel. |
| [3.8.3 · Fit and change projection through the editor](journey/24b-projection-actions.md) | Run the state checks. Orthographic fitting contains every corner in tall and wide views; close picking and document Undo keep the chosen view coherent. |
| [3.9.1 · Run the named projection commands](journey/25-projection.md) | Switch between perspective and orthographic views while keeping fitting, zoom and picking coherent. |
| [3.10.1 · Frame one object without changing its size](journey/26-selected.md) | Run Fit Selected while keeping geometry, selection and document history unchanged. |
| [3.10.2 · Fit and step history from the keyboard](journey/26a-keys.md) | Let canvas F fit the selection and platform shortcuts run Undo or Redo through existing actions. |
| [3.11.1 · Give each object a placement](journey/27-placement.md) | Keep local mesh coordinates and an independent object placement. |
| [3.11.2 · Ask geometry questions in world coordinates](journey/27a-world.md) | Use placement for scene bounds, selected bounds and ray picking. |
| [3.11.3 · Apply object placement on the GPU](journey/27b-model.md) | Send a separate model matrix for each draw while retaining local vertex buffers. |
| [3.11.4 · Move a placed object with a typed offset](journey/27c-move.md) | Translate selected geometry through the existing command and history route. |
| [3.11.5 · Prove placement and history agree](journey/27d-history.md) | Check that Move changes world placement, preserves local geometry, and remains one reversible transaction. |
| [3.12.1 · Prepare a display from an owned source mesh](journey/28-record.md) | Give source geometry a shared owner and prepare its display before committing an object. |
| [3.12.2 · Retain the imported mesh behind each row](journey/28a-imported.md) | Insert imported source geometry and its prepared display together. |
| [3.12.3 · Give generated objects the same source owner](journey/28b-generated.md) | Build demo triangles and Example Box from kernel geometry; derive all displays on insertion. |
| [3.12.4 · Prove source ownership survives editing](journey/28c-ownership.md) | Connect local source geometry, display caches, provenance and document history. |
| [3.13.1 · Give each saved object a stable identity](journey/29-identity.md) | Distinguish a scene object from the original geometry it shares. |
| [3.13.2 · Write a snapshot from the editable sources](journey/29a-snapshot.md) | Serialize live source meshes and their placements without changing the editor. |
| [3.13.3 · Reopen source geometry with its placement](journey/29b-placements.md) | Validate stored placements before reconstructing source-backed objects. |
| [3.13.4 · Prove the saved document reopens faithfully](journey/29c-roundtrip.md) | Check exact local source data, attributes, identities and placement through saving and loading. |
| [3.13.5 · Download the editable document from the command line](journey/29d-save.md) | Connect Save to a browser download and reopen the actual downloaded document. |
| [3.14.1 · Report the result that actually committed](journey/30-feedback.md) | Keep failed reads and imports visible in command history without claiming success. |
| [3.14.2 · Give a pending read an explicit ticket](journey/30a-tickets.md) | Describe latest-read ownership, cancellation and one-shot completion in ordinary Rust. |
| [3.14.3 · Cancel reads without accepting their late result](journey/30b-cancel.md) | Adopt one-shot tickets in the browser and give asynchronous results their own history entry. |
| [3.14.4 · Replace a document as one reversible change](journey/30c-replace.md) | Prepare a whole replacement before changing live rows, then commit it through existing history. |
| [3.14.5 · Choose append or replace before opening the picker](journey/30d-bridge.md) | Carry the selected Open operation through asynchronous file reading and cancel older work immediately. |
| [3.14.6 · Prove recovery at the transaction and display boundaries](journey/30e-recovery.md) | Test partial replacement failure, preserved selection/Redo and placement upload limits. |
| [3.15.1 · Keep selection out of the vertex data](journey/31-settings.md) | Put placement and selection in an explicitly packed object uniform. |
| [3.15.2 · Give immutable GPU geometry one owner](journey/31a-geometry.md) | Separate vertex/index storage from each object’s uniform and retain its CPU source. |
| [3.15.3 · Reuse uploads while their geometry is alive](journey/31b-cache.md) | Cache GPU geometry by its retained CPU allocation and release dead cache entries. |
| [3.15.4 · Update only changed object settings](journey/31c-incremental.md) | Retain GPU rows by object and geometry identity, then write only changed uniform ranges. |
| [3.16.1 · Find the owners retained by history](journey/32-history.md) | Inspect undo/redo roots and release both branches explicitly. |
| [3.16.2 · Count shared CPU displays once](journey/32a-cpu.md) | Account for active/history rows, retained sources/documents and unique display payloads. |
| [3.16.3 · Count each shared GPU buffer once](journey/32b-gpu.md) | Measure live document buffer sizes separately from cumulative allocation counters. |
| [3.16.4 · Close the document without resetting the view](journey/32c-close.md) | Drop active rows and both history branches while preserving the local ID counter and camera. |
| [3.16.5 · Close through the command line and revoke reads](journey/32d-command.md) | Wire Close into the dock and cancel pending delivery before closing the editor. |
| [3.16.6 · Prove release does not retain the old document](journey/32e-release.md) | Check imported history owners, the CPU/GPU release boundary, late read failures and reopening. |
| [3.16.7 · Keep row metadata separate from editable geometry](journey/32f-metadata.md) | Retain names, original source GUIDs and visibility/locking flags without keeping kernel owners alive. |
| [3.16.8 · Ask whether an editable source is available](journey/32fa-access.md) | Route saving, source accounting and owner checks through borrowed source accessors. |
| [3.16.9 · Make editable ownership a private row boundary](journey/32fb-boundary.md) | Finish migrating owner checks and prevent other modules from bypassing source availability. |
| [3.16.10 · Record a reload version without retaining geometry](journey/32fc-version.md) | Own a small document header, a distinct import ID and an exact file fingerprint. |
| [3.16.11 · Attach one origin to an imported document](journey/32fd-origin.md) | Share one geometry-free origin across imported rows and history. |
| [3.16.12 · Give a reload URL an explicit owner](journey/32fe-location.md) | Retain an owned Blob URL through imports and history, then release it with its last owner. |
| [3.16.13 · Adopt the selected file as a reloadable source](journey/32ff-bridge.md) | Transfer the accepted File through synchronous delivery and create its URL at adoption. |
| [3.16.14 · Separate loaded and released editable ownership](journey/32fg-state.md) | Represent source residency without changing retained display, identity or placement. |
| [3.16.15 · Protect sources that cannot be unloaded faithfully](journey/32fh-policy.md) | Require a reload location, original kernel allocation and unchanged row metadata. |
| [3.16.16 · Unload sources across active and history roots](journey/32fi-history.md) | Release a validated whole-origin set without clearing Undo/Redo or changing placements. |
| [3.16.17 · Prove unloading preserves placed history](journey/32fj-proof.md) | Observe imported kernel expiration while retained displays and placements survive Undo/Redo. |
| [3.16.18 · Unload editable sources through the command line](journey/32fk-command.md) | Keep the drawing and GPU allocations while the dock unloads eligible imported sources. |
| [3.16.19 · Protect history and future reload tickets](journey/32fl-guards.md) | Verify whole-origin protection, independent duplicate imports and checked release epochs. |
| [3.16.20 · Identify the source release a reload belongs to](journey/32g-keys.md) | Borrow the current imported release identity and epoch before asking for source data. |
| [3.16.21 · Prepare original kernel data without rebuilding its display](journey/32ga-prepare.md) | Validate an immutable source version and restore its Session into a private reload candidate. |
| [3.16.22 · Adopt restored source owners as one residency change](journey/32gb-adopt.md) | Validate current releases and all matching history rows before adopting any kernel candidate. |
| [3.16.23 · Prove source restoration preserves display and history](journey/32gc-roundtrip.md) | Restore exact source coordinates and editing without reallocating retained display owners. |
| [3.16.24 · Reject stale or inconsistent source batches atomically](journey/32gd-rejections.md) | Keep rows cold on changed versions, missing source identity and obsolete release keys. |
| [3.16.25 · Ignore old reload results before decoding them](journey/32ge-stale.md) | Reject results for closed imports, loaded rows, duplicate keys and previous release epochs. |
| [3.16.26 · Give each source request its own owner](journey/32gf-request.md) | Give each source request its own owner. |
| [3.16.27 · Prove cancelled work releases its source owners](journey/32gfa-ownership.md) | Prove cancelled work releases its source owners. |
| [3.16.28 · Read a source response within its byte limit](journey/32gg-fetch.md) | Read a source response within its byte limit. |
| [3.16.29 · Pair reload ownership with a browser abort controller](journey/32gga-flight.md) | Pair reload ownership with a browser abort controller. |
| [3.16.30 · Deliver only the current completed reload batch](journey/32ggb-delivery.md) | Deliver only the current completed reload batch. |
| [3.16.31 · Restore editable sources through the command line](journey/32gh-command.md) | Restore editable sources through the command line. |
| [3.16.32 · Cancel reloads when their document context changes](journey/32gha-cancel.md) | Cancel reloads when their document context changes. |
| [3.16.33 · Capture the requested edit before waiting](journey/32gi-capture.md) | Retain the original object and arguments before any source request starts. |
| [3.16.34 · Load only the sources the requested edit needs](journey/32gia-scope.md) | Select source requests from the original edit target, with Save covering the active document. |
| [3.16.35 · Move the original target from its current placement](journey/32gib-move.md) | Apply a captured Move to its original target, composing with the placement that exists at replay time. |
| [3.16.36 · Delete the original target while keeping later selection](journey/32giba-delete.md) | Delete a captured target by identity without redirecting the command to a later selection. |
| [3.16.37 · Route captured Move, Delete and Save results](journey/32gibb-reply.md) | Return a scene change or original-precision save bytes from a captured intent. |
| [3.16.38 · Own the intent with its pending source ticket](journey/32gic-owner.md) | Store source keys and captured intent in one pending owner that current completion can consume once. |
| [3.16.39 · Pair captured intent with complete source bodies](journey/32gica-reply.md) | Build a completion value that retains captured intent and rejects incomplete source-body pairings. |
| [3.16.40 · Carry captured intent through browser completion](journey/32gicb-bridge.md) | Carry optional captured intent through the abortable flight and deliver it beside complete source bodies or a current failure. |
| [3.16.41 · Validate restoration before replaying the command](journey/32gid-complete.md) | Complete source restoration and replay the captured operation only after current document validation. |
| [3.16.42 · Deliver restored edit and Save results to the dock](journey/32gie-response.md) | Route validated scene edits and original-precision downloads back to the captured command history. |
| [3.16.43 · Automatically restore sources for Move, Delete and Save](journey/32gif-auto.md) | Connect cold-source edits and Save to captured reload and validated replay without requiring Reload Sources first. |
| [3.16.44 · Prove restored Save keeps the source doubles](journey/32gj-precision.md) | Verify the automatic browser Save download retains exact source coordinates after display-only unloading. |
| [3.16.45 · Prove failed restoration cannot partly commit](journey/32gja-failures.md) | Verify automatic command failures preserve cold drawing, placement and history, including all-or-nothing multiple-source Save. |
| [4.1.1 · Own browser listeners instead of forgetting callbacks](journey/33-owner.md) | Build a browser listener owner that detaches every registered event before freeing the Rust callback and its captured data. |
| [4.1.2 · Dispose the viewer without leaving pending work alive](journey/33a-runtime.md) | Bind real viewer input to owned callbacks and cancel pending file/source authority when the page hides. |
| [4.1.3 · Keep a cached viewer ready for Back navigation](journey/33b-cache.md) | Distinguish a reusable cached page from a final exit so Back navigation cannot restore an already-disposed viewer. |
| [4.2.1 · Keep the first GPU failure with its device](journey/34-fault.md) | Give asynchronous GPU callbacks one shared first-failure value without confusing an older device with its replacement. |
| [4.2.2 · Stop the viewer when its GPU device fails](journey/34a-stop.md) | Connect actual GPU failure callbacks, stop new UI and GPU work, and dispose only the runtime that belongs to the failed device. |
| [4.2.3 · Describe a viewer run without keeping its document](journey/34b-report.md) | Define owned diagnostic context and a serializable run outcome, without retaining scene or GPU owners. |
| [4.2.4 · Keep recent events without losing the first failure](journey/34ba-events.md) | Bound diagnostic observations while keeping the original failure independently of recent event rotation. |
| [4.2.5 · Read live diagnostic context outside the GPU runtime](journey/34c-browser.md) | Start a bounded page report and read real browser context independently of the renderer’s lifetime. |
| [4.2.6 · Download diagnostics through the real command line](journey/34d-download.md) | Download a current ready-run report with Report and attempt one independent first-failure download after GPU disposal. |
| [4.2.7 · Check the bounded shape of a report](journey/34e-schema.md) | Validate a typed report’s limits and failure/outcome invariants before the later storage decoder adopts it. |
| [4.2.8 · Admit only supported bounded saved JSON](journey/34ea-decode.md) | Reject oversized, malformed or unsupported saved JSON before it can become a diagnostic candidate. |
| [4.2.9 · Choose a recent failure without blaming active tabs](journey/34eb-recency.md) | Select recent failed or interrupted runs using actual failure time, valid chronology and tab identity. |
| [4.2.10 · Prove saved-run exclusions before adopting storage](journey/34ec-proof.md) | Complete timestamp-policy acceptance before a stored candidate can create a notice or previous-report download. |
| [4.2.11 · Read previous reports from real browser storage](journey/34f-storage.md) | Read admitted saved reports using stable tab identity, the real browser timestamp parser and bounded scanning. |
| [4.2.12 · Retain only three diagnostic runs](journey/34fa-retain.md) | Write admitted report metadata to an independent run key and retain a bounded set without disturbing viewer state. |
| [4.2.13 · Retrieve saved failure evidence through the command line](journey/34fb-store.md) | Persist current diagnostics independently of GPU lifetime and retrieve eligible previous evidence with a typed command. |
| [4.2.14 · Preserve unsupported telemetry while pruning](journey/34fc-retention.md) | Separate trusted-report admission from raw storage retention so an older viewer cannot strip or prematurely delete newer telemetry. |
| [4.2.15 · Refresh heartbeat without rewriting failure evidence](journey/34g-heartbeat.md) | Refresh actual last-seen metadata independently of first failure, event history and GPU ownership. |
| [4.2.16 · Own the periodic diagnostic heartbeat](journey/34ga-timer.md) | Schedule a real 15-second metadata heartbeat independently of GPU lifetime and cancel its callback through an owner. |
| [4.2.17 · Mark a final healthy run closed](journey/34gb-close.md) | Persist a healthy final exit as closed while preserving first-failure evidence and every observation. |
| [4.2.18 · Own diagnostic page transitions](journey/34gba-lifecycle.md) | Connect real pagehide/pageshow to independent diagnostics, cached suspension and safe final callback cleanup. |
| [4.2.19 · Keep each heartbeat pause reason separate](journey/34gbb-suspension.md) | Check that clearing one pause reason cannot resume a still-suspended heartbeat. |
| [4.2.20 · Connect visibility and freezing to heartbeat scheduling](journey/34gbba-browser.md) | Pause the actual diagnostic timer while any browser suspension reason remains. |
| [4.2.21 · Give pending startup a revocable ticket](journey/34gbc-authority.md) | Prevent a waiting task from regaining startup permission after closure. |
| [4.2.22 · Refuse GPU results after final page exit](journey/34gbca-startup.md) | Stop delayed adapter or device delivery from installing a renderer after final exit. |
| [4.2.23 · Bound browser failure messages](journey/34gc-messages.md) | Keep browser failure text readable and small enough for the existing report. |
| [4.2.24 · Observe uncaught browser failures](journey/34gca-errors.md) | Retain the first uncaught browser failure while later events remain bounded and drawing stays independently owned. |
| [4.2.25 · Retain adapter identity in the report](journey/34gd-adapter.md) | Preserve bounded adapter identity even when recent events rotate out of the report. |
| [4.2.26 · Read the viewer’s adapter identity](journey/34gda-browser.md) | Retain vendor and architecture from the same adapter request used for drawing. |
| [4.2.27 · Retain measured loading phases](journey/34gdb-phases.md) | Keep measured load operations after their recent event messages rotate away. |
| [4.2.28 · Measure GPU startup and the first frame](journey/34gdc-startup.md) | Download actual adapter, device, renderer and first completed frame timings. |
| [4.2.29 · Measure the selected file read](journey/34gdd-read.md) | Retain the duration and byte count of the selected file’s actual read. |
| [4.2.30 · Measure a completed CPU operation](journey/34gde-measure.md) | Measure CPU work without changing its success or failure. |
| [4.2.31 · Measure each document preparation phase](journey/34gdea-import.md) | Record decode, validation, kernel construction and display preparation at their actual boundaries. |
| [4.2.32 · Measure upload preparation and GPU completion](journey/34gdf-upload.md) | Distinguish CPU buffer preparation from completion on the drawing queue. |
| [4.2.33 · Measure accepted source fetches](journey/34gdg-network.md) | Retain real fetch and Resource Timing measurements only for the accepted reload ticket. |
| [4.2.34 · Record accepted file revisions](journey/34gdga-revision.md) | Identify accepted source versions without recording rejected or stale documents. |
| [4.2.35 · Keep the document outside the GPU runtime](journey/34ge-owner.md) | Give GPU runtimes shared access to a separately owned CPU document. |
| [4.2.36 · Permit one conservative recovery reload](journey/34gea-policy.md) | Allow one device-loss recovery reload and retain reduced quality until an ordinary reload. |
| [4.2.37 · Recover once at reduced canvas density](journey/34geb-reload.md) | Save device-loss diagnostics, reload once and draw conservatively until the next ordinary reload. |
| [5.1.1 · Prepare a stroke without changing its source](journey/35-stroke.md) | Prepare finite line endpoints, colour and screen width while retaining the original kernel line. |
| [5.1.2 · Retain line owners in document history](journey/35a-document.md) | Give prepared lines stable document IDs, history ownership and world-space bounds. |
| [5.1.3 · Extrude a stroke in screen pixels](journey/35b-extrude.md) | Expand a projected segment into two triangles with a width that survives camera zoom. |
| [5.1.4 · Create the shared stroke pipeline](journey/35c-lane.md) | Create one pipeline and view uniform for a batch of stroke instances. |
| [5.1.5 · Upload changed stroke instances only](journey/35ca-sync.md) | Synchronize a stroke batch, draw it and retain vertex buffers through camera-only changes. |
| [5.1.6 · Connect the stroke lane to the document renderer](journey/35cb-render.md) | Draw document strokes with shared depth, correct device density and complete Close accounting. |
| [5.1.7 · Insert a line through the real command dock](journey/35d-input.md) | Insert an editable line as one undoable command and prove its width in actual browser pixels. |
| [5.2.1 · Retain connected source geometry](journey/36-chain.md) | Retain exact line or polyline owners while preparing checked shared display points. |
| [5.2.2 · Link adjacent spans and headed ends](journey/36a-links.md) | Link every nonzero span to its neighbours and identify free ends carrying arrowheads. |
| [5.2.3 · Project neighbours for bounded stroke joins](journey/36b-join.md) | Calculate a shared bounded cross-section from projected neighbour directions. |
| [5.2.4 · Extrude the joined stroke body](journey/36ba-body.md) | Build joined stroke vertices and analytic coverage from the prepared projected cross-sections. |
| [5.2.5 · Extrude free-end arrowheads](journey/36bb-heads.md) | Draw flagged arrowheads with stable screen dimensions and limit their share of short spans. |
| [5.2.6 · Share instance drawing with connected strokes](journey/36c-lane.md) | Reuse the stroke lane lifetime and view code with the connected 72-byte instance layout. |
| [5.2.7 · Retain connected paths in document history](journey/36d-document.md) | Retain exact connected source owners as independent document rows and history values. |
| [5.2.8 · Place original connected coordinates and count shared owners](journey/36db-placement.md) | Place original path coordinates before display conversion and count shared source/display owners once. |
| [5.2.9 · Draw connected document rows](journey/36da-render.md) | Synchronize placed connected paths into one shared lane and release every instance on Close. |
| [5.2.10 · Insert connected paths and arrow examples](journey/36e-input.md) | Insert source-backed joined paths and arrowheads through the real dock, then check their actual browser pixels. |
| [5.3.1 · Prepare an original point for a screen marker](journey/37-point.md) | Retain exact kernel Point coordinates while preparing a finite 32-byte screen-marker record. |
| [5.3.2 · Extrude a circular marker in screen pixels](journey/37a-disc.md) | Extrude a point into a depth-tested screen disc with analytic circular coverage. |
| [5.3.3 · Draw markers with the shared instance lane](journey/37b-lane.md) | Bind actual 32-byte marker instances and verify their projected size, alpha and lifetime on the GPU. |
| [5.3.4 · Retain original point rows and placement](journey/37c-document.md) | Give original point sources checked document identities, precise placement and undoable lifetimes. |
| [5.3.5 · Draw original point rows and count their resources](journey/37d-render.md) | Draw placed original point rows with truthful CPU/GPU counters and complete Close release. |
| [5.3.6 · Insert original points through the real dock](journey/37da-input.md) | Insert one original Point through actual command input and prove its screen diameter, history and resource release. |
| [5.3.7 · Identify controls on the original curve owner](journey/37e-controls.md) | Refer to an original curve control by its shared curve owner and stable control index, preserving homogeneous source coordinates. |
| [5.4.1 · Validate an original NURBS curve before sampling](journey/38-layout.md) | Validate public kernel curve layout and domain before evaluating an original source. |
| [5.4.2 · Sample a validated curve without replacing its source](journey/38a-samples.md) | Build a bounded display approximation from an original curve and retain exact source controls separately. |
| [5.4.3 · Retain original curves beside their display samples](journey/38b-source.md) | Retain original NURBS ownership, cached double samples and placed curve bounds in the existing path document. |
| [5.4.4 · Draw controls with their original curve parent and index](journey/38c-controls.md) | Draw enabled curve controls through the real marker lane while retaining original parent IDs and control indices. |
| [5.4.5 · Insert an original curve and inspect its controls](journey/38d-input.md) | Insert an original NURBS curve through real input and distinguish its actual curve pixels from its indexed control markers. |
| [5.5.1 · Resolve original visibility for every drawing kind](journey/39-visible.md) | Respect original source visibility consistently in drawing, Fit, control markers and mesh picking while retaining hidden sources. |
| [5.5.2 · Share surface depth across all ink lanes](journey/39a-depth.md) | Use one explicit depth format and policy for opaque surfaces and screen-space ink, then prove actual occlusion. |
| [5.6.1 · Carry the original mesh colour mode into display data](journey/40-colour.md) | Preserve original object, point and face colour modes through the kernel render adapter, including shared face boundaries. |
| [5.6.2 · Compare original colour modes through actual input](journey/40a-input.md) | Insert original object/point/face colour specimens through real input and preserve their modes through Save/reopen. |
| [5.6.3 · Draw meshes beyond the sixteen-bit index limit](journey/40b-indices.md) | Preserve kernel u32 indices through display and GPU drawing, with truthful payload and allocation accounting. |
| [5.7.1 · Retain checked source normals beside display positions](journey/41-normals.md) | Retain a checked normal stream from the original kernel adapter while preserving source attributes and the existing position/colour layout. |
| [5.7.2 · Transform normals with nonuniform surface placement](journey/41a-placement.md) | Prepare a finite oriented normal matrix that stays perpendicular under nonuniform scale, shear, reflection and flattening. |
| [5.7.3 · Draw smooth normals and sharp surface faces](journey/41b-drawing.md) | Upload the retained source normal stream and its oriented placement matrix, with real smooth interpolation and explicit flat-face fallback. |
| [5.7.4 · Reveal oriented world normals in the renderer](journey/41c-view.md) | Draw an oriented world-normal diagnostic with sharp-face winding and finite source smoothing, independently of source history. |
| [5.7.5 · Inspect smooth normals and face winding through commands](journey/41ca-input.md) | Compare sharp/smooth source surfaces and actual face winding through real commands, retaining source history and editable normal attributes. |
| [5.7.6 · Bound the ground grid and its camera depth range](journey/41d-grid.md) | Prepare a finite bounded ground grid with current BRG axis colours and include its distant corners in the camera depth range. |
| [5.7.7 · Draw a depth-correct grid with shared stroke buffers](journey/41e-drawing.md) | Draw the bounded grid and BRG axes using the shared CSS stroke layout, with actual surface depth and exact independent grid-resource accounting. |
| [5.7.8 · Inspect the ground grid through actual commands](journey/41f-input.md) | Enable the finite depth-correct grid through actual dock input, preserving grid view state, source history and unchanged geometry during camera navigation. |
| [6.1.1 · Bound the cursor image and decode a hit](journey/42-window.md) | Bound picking work around the cursor independently of the canvas size. |
| [6.1.2 · Keep exact object IDs in shared GPU settings](journey/42a-id.md) | Share explicit uniform layouts between visible drawing and integer picking. |
| [6.1.3 · Share point and stroke coverage with picking](journey/42b-ink.md) | Use the same marker and stroke geometry in visible and ID shaders. |
| [6.1.4 · Pick connected paths and arrowheads with their drawn shape](journey/42c-path.md) | Keep joined strokes and shaped arrowheads consistent in both passes. |
| [6.1.5 · Attach original owners to visible ink instances](journey/42d-owners.md) | Keep picking identities separate from display coordinates and GPU geometry. |
| [6.1.6 · Draw integer ink with the shared geometry buffers](journey/42e-pass.md) | Add integer pipelines without another geometry upload. |
| [6.1.7 · Draw depth-tested mesh owners into an integer attachment](journey/42f-mesh.md) | Use original mesh buffers and placements for exact visible owner IDs. |
| [6.1.8 · Own a small ID image, depth target and readback buffer](journey/42g-image.md) | Allocate bounded picking resources with checked padded rows. |
| [6.1.9 · Bound asynchronous picking ownership](journey/42h-owner.md) | Own one active request and one latest pending cursor. |
| [6.1.10 · Record the ID draw and aligned image copy](journey/42i-copy.md) | Submit surfaces and ink to a bounded image before copying its cursor region. |
| [6.1.11 · Map the integer hit without blocking browser input](journey/42j-read.md) | Read completed GPU bytes asynchronously and reuse bounded resources. |
| [6.1.12 · Select original rows through actual GPU canvas clicks](journey/42k-input.md) | Connect real canvas input to bounded integer GPU picking for meshes and ink. |
| [6.2.1 · Name the request and view that produced a hit](journey/43-ticket.md) | Name the request and view that produced a hit. |
| [6.2.2 · Carry checked tickets through GPU mapping](journey/43a-flight.md) | Carry checked tickets through GPU mapping. |
| [6.2.3 · Reject results from an old view or interrupted input](journey/43b-view.md) | Reject results from an old view or interrupted input. |
| [6.3.1 · Preserve source faces and edges](journey/44-topology.md) | Preserve source faces and edges. |
| [6.3.2 · Tag the kind of a picked part](journey/44a-tags.md) | Tag the kind of a picked part. |
| [6.3.3 · Map visible ink back to original sources](journey/44b-sources.md) | Map visible ink back to original sources. |
| [6.3.4 · Validate a part against the current source](journey/44c-resolve.md) | Validate a part against the current source. |
| [6.3.5 · Reuse visible geometry for face identities](journey/44d-pulling.md) | Reuse visible geometry for face identities. |
| [6.3.6 · Create the integer source-face pipeline](journey/44e-pipeline.md) | Create the integer source-face pipeline. |
| [6.3.7 · Upload only original-face metadata](journey/44f-metadata.md) | Upload only original-face metadata. |
| [6.3.8 · Draw original boundary edges consistently](journey/44g-edge-ink.md) | Draw original boundary edges consistently. |
| [6.3.9 · Pick source parts with visible coverage](journey/44h-mode-gpu.md) | Pick source parts with visible coverage. |
| [6.3.10 · Highlight original edges and controls](journey/44i-ink-highlight.md) | Highlight original edges and controls. |
| [6.3.11 · Build highlights from original ink targets](journey/44ia-original-ink.md) | Build highlights from original ink targets. |
| [6.3.12 · Highlight one original face without recolouring its parent](journey/44j-face-highlight.md) | Highlight one original face without recolouring its parent. |
| [6.3.13 · Keep part selection outside document history](journey/44k-selection.md) | Keep part selection outside document history. |
| [6.3.14 · Select and inspect original faces, edges and controls](journey/44l-input.md) | Select and inspect original faces, edges and controls. |
| [6.4.1 · Preserve unique selections in pick order](journey/45-order.md) | Preserve unique selections in pick order. |
| [6.4.2 · Commit mixed selection edits atomically](journey/45a-batch.md) | Commit mixed selection edits atomically. |
| [6.4.3 · Connect multi-hit state and highlight every selected row](journey/45b-membership.md) | Connect multi-hit state and highlight every selected row. |
| [6.4.4 · Cycle every eligible original geometry owner](journey/45bb-cycle.md) | Cycle visible unlocked mesh, line, path and point rows in stable owner order. |
| [6.4.5 · Collect multiple objects with Shift-click and commands](journey/45c-input.md) | Collect multiple objects with Shift-click and commands. |
| [7.1.1 · Give active tools ownership of the command panel](journey/46-tool-panel.md) | Keep active values and tool controls inside the existing GPU command panel. |
| [7.2.1 · Separate browser requests from editor actions](journey/47-registry.md) | Separate browser requests from editor actions. |
| [7.2.2 · Register the existing editor command factories](journey/47a-actions.md) | Register the existing editor command factories. |
| [7.2.3 · Parse the longest command and validate its arguments](journey/47b-parse.md) | Parse the longest command and validate its arguments. |
| [7.2.4 · Route the command field through the typed registry](journey/47c-route.md) | Route the command field through the typed registry. |
| [7.3.1 · Resolve drawing points on the construction plane](journey/48-coordinates.md) | Resolve drawing points on the construction plane. |
| [7.3.2 · Keep unfinished point tools outside the document](journey/48a-draft.md) | Keep unfinished point tools outside the document. |
| [7.3.3 · Draw a temporary rubber band and cursor](journey/48b-preview.md) | Draw a temporary rubber band and cursor. |
| [7.3.4 · Commit completed tools in one Undo transaction](journey/48c-commit.md) | Commit completed tools in one Undo transaction. |
| [7.3.5 · Draw points and lines through the shared command dock](journey/48d-input.md) | Draw points and lines through the shared command dock. |
| [7.4.1 · Separate object and grid snapping](journey/49-settings.md) | Separate object and grid snapping. |
| [7.4.2 · Keep snap math in world and plane coordinates](journey/49a-math.md) | Keep snap math in world and plane coordinates. |
| [7.4.3 · Collect targets from original placed geometry](journey/49b-sources.md) | Collect targets from original placed geometry. |
| [7.4.4 · Search nearby screen cells](journey/49c-bins.md) | Search nearby screen cells. |
| [7.4.5 · Choose snap kinds before grid fallback](journey/49d-index.md) | Choose snap kinds before grid fallback. |
| [7.4.6 · Invalidate snap caches after source edits](journey/49e-cache.md) | Invalidate snap caches after source edits. |
| [7.4.7 · Preserve drafts while snapping](journey/49ea-state.md) | Preserve drafts while snapping. |
| [7.4.8 · Snap actual drawing input through the command dock](journey/49f-input.md) | Snap actual drawing input through the command dock. |
| [7.5.1 · Address placements across original geometry lanes](journey/50-placements.md) | Address placements across original geometry lanes. |
| [7.5.2 · Keep temporary placements outside the document](journey/50a-preview.md) | Keep temporary placements outside the document. |
| [7.5.3 · Present temporary edits through a separate scene view](journey/50b-display.md) | Present temporary edits through a separate scene view. |
| [7.5.4 · Preview Move from a base point to a target](journey/50c-move.md) | Preview Move from a base point to a target. |
| [7.5.5 · Cancel previews without consuming document history](journey/50d-ownership.md) | Cancel previews without consuming document history. |
| [7.6.1 · Name move, rotation and scale handles](journey/51-handles.md) | Name move, rotation and scale handles. |
| [7.6.2 · Measure handle rays in original world coordinates](journey/51a-rays.md) | Measure handle rays in original world coordinates. |
| [7.6.3 · Keep handle state at one original-world pivot](journey/51b-pivot.md) | Keep handle state at one original-world pivot. |
| [7.6.4 · Pick handles in a stable priority order](journey/51c-hit.md) | Pick handles in a stable priority order. |
| [7.6.5 · Capture the handle’s original grab](journey/51d-grab.md) | Capture the handle’s original grab. |
| [7.6.6 · Derive handle transforms and quarter-turn snapping](journey/51e-transform.md) | Derive handle transforms and quarter-turn snapping. |
| [7.6.7 · Pack and sample local handle vertices](journey/51ez-pack-and-sample-local-handle-vertices.md) | Write the twenty-byte widget vertex format, explicit byte packing and one shared rectangular surface sampler. |
| [7.6.8 · Build local arrows balls and rotation arcs](journey/51f-mesh.md) | Reuse the surface sampler for the original lathed shafts, cones, balls, rotation arcs and central hub. |
| [7.6.9 · Keep handles constant in CSS pixels](journey/51g-layout.md) | Keep handles constant in CSS pixels. |
| [7.6.10 · Draw a local handle tile and its canvas rectangle](journey/51h-shader.md) | Draw a local handle tile and its canvas rectangle. |
| [7.6.11 · Bind the widget’s own sampled image](journey/51i-image.md) | Bind the widget’s own sampled image. |
| [7.6.12 · Separate handle depth from scene depth](journey/51j-pipelines.md) | Separate handle depth from scene depth. |
| [7.6.13 · Own color, depth and resolve textures together](journey/51k-tile.md) | Own color, depth and resolve textures together. |
| [7.6.14 · Upload handle geometry once](journey/51l-upload.md) | Upload handle geometry once. |
| [7.6.15 · Write only current handle placement](journey/51m-prepare.md) | Write only current handle placement. |
| [7.6.16 · Composite handles after source geometry](journey/51n-draw.md) | Composite handles after source geometry. |
| [7.6.17 · Register the widget outside source picking](journey/51o-renderer.md) | Register the widget outside source picking. |
| [7.6.18 · Locate original mesh parts and curve controls](journey/51p-part-points.md) | Locate original mesh parts and curve controls. |
| [7.6.19 · Deform selected original points through their placement](journey/51q-deform.md) | Deform selected original points through their placement. |
| [7.6.20 · Keep part deformation outside saved geometry](journey/51r-part-preview.md) | Keep part deformation outside saved geometry. |
| [7.6.21 · Share one handle interface across whole and part edits](journey/51s-edit-owner.md) | Share one handle interface across whole and part edits. |
| [7.6.22 · Keep pointer ownership through click or drag](journey/51t-capture.md) | Keep pointer ownership through click or drag. |
| [7.6.23 · Place handles on the current editable selection](journey/51u-selection.md) | Place handles on the current editable selection. |
| [7.6.24 · Capture a valid handle without editing the document](journey/51v-press.md) | Capture a valid handle without editing the document. |
| [7.6.25 · Preview captured motion and commit one release](journey/51w-release.md) | Preview captured motion and commit one release. |
| [7.6.26 · Own one handle’s number-field state](journey/51x-number-model.md) | Own one handle’s number-field state. |
| [7.6.27 · Draw the number field beside its handle](journey/51y-number-field.md) | Draw the number field beside its handle. |
| [7.6.28 · Apply precise typed handle values once](journey/51z-number-submit.md) | Apply precise typed handle values once. |
| [7.6.29 · Route field input through the shared canvas panel](journey/51za-panel.md) | Route field input through the shared canvas panel. |
| [7.6.30 · Update selected handles with the actual camera](journey/51zb-presentation.md) | Update selected handles with the actual camera. |
| [7.6.31 · Capture actual browser handle drags](journey/51zc-pointer.md) | Capture actual browser handle drags. |
| [7.6.32 · Restore previews when input ownership ends](journey/51zd-interruption.md) | Restore previews when input ownership ends. |
| [7.6.33 · Count temporary source owners without saving them](journey/51ze-ledger.md) | Count temporary source owners without saving them. |
| [7.6.34 · Verify complete handle editing and input](journey/51zf-handles.md) | Verify complete handle editing and input. |
| [8.1.1 · Validate and place a nested source tree](journey/52-tree.md) | Validate and place a nested source tree. |
| [8.1.2 · Keep lightweight tree records outside kernel ownership](journey/52a-records.md) | Keep lightweight tree records outside kernel ownership. |
| [8.1.3 · Index descendants and read-only attribute groups](journey/52b-index.md) | Index descendants and read-only attribute groups. |
| [8.1.4 · Import nested source identities and precise placements](journey/52c-load.md) | Import nested source identities and precise placements. |
| [8.1.5 · Carry parent-owned descendants in one preview](journey/52d-preview.md) | Carry parent-owned descendants in one preview. |
| [8.1.6 · Commit parent moves and deletions as one transaction](journey/52e-edits.md) | Commit parent moves and deletions as one transaction. |
| [8.1.7 · Save nested rows and their local placements](journey/52f-save.md) | Save nested rows and their local placements. |
| [8.1.8 · Expose stable tree rows across geometry lanes](journey/52g-rows.md) | Expose stable tree rows across geometry lanes. |
| [8.1.9 · Keep expansion and paging as presentation state](journey/52h-expansion.md) | Keep expansion and paging as presentation state. |
| [8.1.10 · Draw indented original-source tree controls](journey/52i-panel.md) | Draw indented original-source tree controls. |
| [8.1.11 · Connect tree controls before canvas picking](journey/52j-input.md) | Connect tree controls before canvas picking. |
| [8.1.12 · Exercise nested source ownership and exact undo](journey/52k-tree.md) | Exercise nested source ownership and exact undo. |
| [8.2.1 · Separate layer presentation from original sources](journey/53-flags.md) | Separate layer presentation from original sources. |
| [8.2.2 · Index live identities and initialise view flags](journey/53a-ids.md) | Index live identities and initialise view flags. |
| [8.2.3 · Update surface colour and pickability in small GPU records](journey/53b-uniform.md) | Update surface colour and pickability in small GPU records. |
| [8.2.4 · Resolve live visibility and selection eligibility](journey/53bb-eligibility.md) | Resolve live visibility and selection eligibility. |
| [8.2.5 · Use current visibility and colours in every derived pass](journey/53c-presentation.md) | Use current visibility and colours in every derived pass. |
| [8.2.6 · Connect Hide Show and canvas Delete shortcuts](journey/53d-hide-show.md) | Connect Hide Show and canvas Delete shortcuts. |
| [8.2.7 · Apply eye lock and independent colour properties](journey/53e-properties.md) | Apply eye lock and independent colour properties. |
| [8.2.8 · Keep tree identities stable across edits and saves](journey/53f-portable.md) | Keep tree identities stable across edits and saves. |
| [8.2.9 · Represent group edits with temporary branches](journey/53g-branches.md) | Represent group edits with temporary branches. |
| [8.2.10 · Validate current layers and protected ancestors](journey/53h-layer-targets.md) | Validate current layers and protected ancestors. |
| [8.2.11 · Copy row identities while sharing source allocations](journey/53i-copy-records.md) | Copy row identities while sharing source allocations. |
| [8.2.12 · Move and copy objects between layer trees](journey/53j-transfers.md) | Move and copy objects between layer trees. |
| [8.2.13 · Create and rename uniquely named layers](journey/53k-new-rename.md) | Create and rename uniquely named layers. |
| [8.2.14 · Delete and duplicate complete layer contents](journey/53l-delete-duplicate.md) | Delete and duplicate complete layer contents. |
| [8.2.15 · Commit group edits through one history boundary](journey/53m-actions.md) | Commit group edits through one history boundary. |
| [8.2.16 · Create geometry on the current layer or Created](journey/53n-routing.md) | Create geometry on the current layer or Created. |
| [8.2.17 · Supply nine colour swatches and RGB sliders](journey/53o-palette.md) | Supply nine colour swatches and RGB sliders. |
| [8.2.18 · Resolve layer presentation for real tree controls](journey/53p-panel-state.md) | Resolve layer presentation for real tree controls. |
| [8.2.19 · Draw eye lock and independent colour menus](journey/53q-properties-ui.md) | Draw eye lock and independent colour menus. |
| [8.2.20 · Own rename text and populated deletion confirmation](journey/53r-name.md) | Own rename text and populated deletion confirmation. |
| [8.2.21 · Connect group menus to the wider tree panel](journey/53s-tree-controls.md) | Connect group menus to the wider tree panel. |
| [8.2.22 · Route layer controls before handles and canvas input](journey/53t-input.md) | Route layer controls before handles and canvas input. |
| [8.2.23 · Exercise layers shortcuts colours and exact source ownership](journey/53u-layers.md) | Exercise layers shortcuts colours and exact source ownership. |
| [8.3.1 · Navigate and fit inside the free canvas](journey/54-camera.md) | Navigate and fit inside the free canvas. |
| [8.3.2 · Route camera changes without consuming document history](journey/54aa-view-actions.md) | Route camera changes without consuming document history. |
| [8.3.3 · Apply visible selection and modifier precedence](journey/54a-selection.md) | Apply visible selection and modifier precedence. |
| [8.3.4 · Keep touch handles reachable without enlarging them](journey/54b-finger.md) | Keep touch handles reachable without enlarging them. |
| [8.3.5 · Distinguish window and crossing rectangles](journey/54c-rectangle.md) | Distinguish window and crossing rectangles. |
| [8.3.6 · Select original projected geometry in one source pass](journey/54d-samples.md) | Select original projected geometry in one source pass. |
| [8.3.7 · Freeze a released drag until its pick answers](journey/54e-drag.md) | Freeze a released drag until its pick answers. |
| [8.3.8 · Own drag previews outside document history](journey/54f-drag-state.md) | Own drag previews outside document history. |
| [8.3.9 · Apply held released and cancelled drag outcomes](journey/54g-drag-actions.md) | Apply held released and cancelled drag outcomes. |
| [8.3.10 · Capture mouse gestures in CSS pixels](journey/54h-mouse.md) | Capture mouse gestures in CSS pixels. |
| [8.3.11 · Orbit with one finger and pan pinch with two](journey/54i-touch.md) | Orbit with one finger and pan pinch with two. |
| [8.3.12 · Finish taps double taps and touch cancellation](journey/54j-taps.md) | Finish taps double taps and touch cancellation. |
| [8.3.13 · Draw a thin selection rectangle above the canvas](journey/54k-overlay.md) | Draw a thin selection rectangle above the canvas. |
| [8.3.14 · Route mouse touch and wheel after panel input](journey/54l-navigation.md) | Route mouse touch and wheel after panel input. |
| [8.3.15 · Retain the press pick through a quick release](journey/54m-pick-release.md) | Retain the press pick through a quick release. |
| [8.3.16 · Cancel owned gestures and preserve normal capture release](journey/54n-cancellation.md) | Cancel owned gestures and preserve normal capture release. |
| [8.3.17 · Expose drag ownership and centre the available drawing](journey/54o-frame.md) | Expose drag ownership and centre the available drawing. |
| [8.3.18 · Exercise current navigation selection and delayed answers](journey/54p-navigation.md) | Exercise current navigation selection and delayed answers. |
| [9.1.1 · Validate original surface control storage](journey/55-validation.md) | Validate original surface control storage. |
| [9.1.2 · Keep UV coordinates beside original world samples](journey/55a-grid.md) | Keep UV coordinates beside original world samples. |
| [9.1.3 · Use analytic normals and finite singular-point fallbacks](journey/55b-normals.md) | Use analytic normals and finite singular-point fallbacks. |
| [9.1.4 · Prepare derived surface display without losing its source](journey/55c-display.md) | Prepare derived surface display without losing its source. |
| [9.1.5 · Attach original surface ownership to existing scene rows](journey/55d-source-row.md) | Attach original surface ownership to existing scene rows. |
| [9.1.6 · Resolve original surface face and domain edges](journey/55e-identities.md) | Resolve original surface face and domain edges. |
| [9.1.7 · Account for shared source and UV cache owners](journey/55f-ownership.md) | Account for shared source and UV cache owners. |
| [9.1.8 · Draw and pick the original parametric roof](journey/55g-surface.md) | Draw and pick the original parametric roof. |
| [9.1.9 · Read original edge-use samples](journey/55za-read-original-edge-use-samples.md) | Write edge-use and chain records, parameter ordering and exact linear pcurve endpoints. |
| [9.1.10 · Follow original isoparametric boundaries](journey/55zb-follow-original-isoparametric-boundaries.md) | Follow an exact constant-u or constant-v boundary through the existing face mesh. |
| [9.1.11 · Follow original constrained trim labels](journey/55zc-follow-original-constrained-trim-labels.md) | Read ordered edge-use and interval labels from mesh vertices. |
| [9.2.1 · Connect original CAD edge chains](journey/56-chains.md) | Reuse constrained-label and isoparametric lookup for each original edge. |
| [9.2.2 · Mesh incident faces with shared boundary constraints](journey/56a-shared.md) | Mesh incident faces with shared boundary constraints. |
| [9.2.3 · Prove straight and circular shared boundaries](journey/56b-boundaries.md) | Prove straight and circular shared boundaries. |
| [9.3.1 · Keep the trimmed face connected to its surface](journey/57-parameters.md) | Keep the trimmed face connected to its surface. |
| [9.3.2 · Prove trimmed curved faces and open holes](journey/57a-trims.md) | Prove trimmed curved faces and open holes. |
| [9.4.1 · Keep both parameter-space uses of a seam](journey/58-uses.md) | Keep both parameter-space uses of a seam. |
| [9.4.2 · Prove seam continuity and suppress false open borders](journey/58a-periodic.md) | Prove seam continuity and suppress false open borders. |
| [9.4.3 · Match original face edge directions](journey/58za-match-original-face-edge-directions.md) | Match shared BRep edges across independently keyed face meshes. |
| [9.5.1 · Orient connected original face groups](journey/59-orientation.md) | Propagate coherent signs and turn each closed face group outward. |
| [9.5.2 · Map each display triangle to its original CAD face](journey/59a-face-data.md) | Map each display triangle to its original CAD face. |
| [9.5.3 · Retain source UVs and draw only original CAD boundaries](journey/59b-display.md) | Retain source UVs and draw only original CAD boundaries. |
| [9.5.4 · Attach original BRep ownership to existing rows](journey/59c-source-row.md) | Attach original BRep ownership to existing rows. |
| [9.5.5 · Pick and highlight an original CAD border](journey/59d-identities.md) | Pick and highlight an original CAD border. |
| [9.5.6 · Inspect shared CAD owners and compact cache counts](journey/59e-cache.md) | Inspect shared CAD owners and compact cache counts. |
| [9.5.7 · Draw the original bore and prove its empty hole](journey/59f-cad.md) | Draw the original bore and prove its empty hole. |
| [10.1.1 · Pin scene-text shaping and supply the viewer fonts](journey/60-fonts.md) | Pin scene-text shaping and supply the viewer fonts. |
| [10.1.2 · Keep source words metrics and placement together](journey/60a-labels.md) | Keep source words metrics and placement together. |
| [10.1.3 · Load explicit fonts and inspect missing coverage](journey/60b-coverage.md) | Load explicit fonts and inspect missing coverage. |
| [10.1.4 · Validate a complete label replacement before changing it](journey/60ba-validation.md) | Validate a complete label replacement before changing it. |
| [10.1.5 · Shape words into glyph clusters and baselines](journey/60bb-shaping.md) | Shape words into glyph clusters and baselines. |
| [10.1.6 · Own the shaped runs with their font revisions](journey/60c-document.md) | Own the shaped runs with their font revisions. |
| [10.1.7 · Reshape only when the source layout changes](journey/60d-reuse.md) | Reshape only when the source layout changes. |
| [10.1.8 · Replace usable fonts without corrupting current text](journey/60e-font-reload.md) | Replace usable fonts without corrupting current text. |
| [10.1.9 · Inspect glyph origins clusters and multiline metrics](journey/60f-diagnostics.md) | Inspect glyph origins clusters and multiline metrics. |
| [10.1.10 · Prove the current fonts shaping and reuse rules](journey/60g-text-shaping.md) | Prove the current fonts shaping and reuse rules. |
| [10.2.1 · Own glyph coverage in a shared GPU atlas](journey/61-atlas.md) | Own glyph coverage in a shared GPU atlas. |
| [10.2.2 · Prepare DPI-scaled glyph bounds and cache unchanged frames](journey/61a-coverage.md) | Prepare DPI-scaled glyph bounds and cache unchanged frames. |
| [10.2.3 · Supply a real-font CAD text specimen](journey/61b-specimen.md) | Supply a real-font CAD text specimen. |
| [10.2.4 · Draw lazy text coverage after the scene](journey/61c-renderer.md) | Draw lazy text coverage after the scene. |
| [10.2.5 · Draw the specimen and prove real coverage reuse](journey/61d-glyphs.md) | Draw the specimen and prove real coverage reuse. |
| [10.3.1 · Own original scene words and font metrics](journey/62-source.md) | Own original scene words and font metrics. |
| [10.3.2 · Transform the label frame without changing its source](journey/62a-placements.md) | Transform the label frame without changing its source. |
| [10.3.3 · Measure the world-sized label and its padded plate](journey/62b-bounds.md) | Measure the world-sized label and its padded plate. |
| [10.3.4 · Register text rows and release them with the scene](journey/62c-rows.md) | Register text rows and release them with the scene. |
| [10.3.5 · Include label owners in selection layers and history](journey/62d-identity.md) | Include label owners in selection layers and history. |
| [10.3.6 · Select a label through its projected plate](journey/62e-rectangles.md) | Select a label through its projected plate. |
| [10.3.7 · Supply the text buffer and target adapters](journey/62f-gpu-support.md) | Supply the text buffer and target adapters. |
| [10.3.8 · Keep plane clipping correct in a small pick window](journey/62g-plane-shader.md) | Keep plane clipping correct in a small pick window. |
| [10.3.9 · Draw the current rounded nameplate and its original ID](journey/62h-plate-shader.md) | Draw the current rounded nameplate and its original ID. |
| [10.3.10 · Keep CSS pixels separate from framebuffer pixels](journey/62i-coverage.md) | Validate one uniform device scale before allocating or positioning text. Keep world origins in f64 and GPU matrices in f32. |
| [10.3.11 · Project an original label into framebuffer coordinates](journey/62ia-project-an-original-label-into-framebuffer-coordinates.md) | Subtract the declared scene origin before projection. Cull invalid depth and apply the CSS/device scale once. |
| [10.3.12 · Give nameplate vertices one GPU owner](journey/62ib-give-nameplate-vertices-one-gpu-owner.md) | Keep the rectangle input separate from the retained vertex buffer and its color/ID pipelines. |
| [10.3.13 · Build nameplate color and object-ID pipelines](journey/62ic-build-nameplate-color-and-object-id-pipelines.md) | Prepare color and original-ID pipelines for nameplate geometry. |
| [10.3.14 · Own and release nameplate draw resources](journey/62id-own-and-release-nameplate-draw-resources.md) | Construct the buffers and pipelines once, draw physical and overlay plates in their declared phases, and reset logical vertex counts independently of capacity. |
| [10.3.15 · Upload nameplate rectangles in one physical coordinate system](journey/62ie-upload-nameplate-rectangles-in-one-physical-coordinate-system.md) | Generate the packed rectangle vertices once and retain the split between depth-tested plates and screen overlays. |
| [10.3.16 · Place the plate around the shaped label](journey/62if-place-the-plate-around-the-shaped-label.md) | Compute padding and rounded bounds in the same coordinate system as glyph placement. Define text statistics as observations, not source data. |
| [10.3.17 · Give world text a bounded coverage cache](journey/62ig-give-world-text-a-bounded-coverage-cache.md) | A cached plane owns its texture and bind group. Drop destroys the texture; a changed placement may reuse the raster while font/text changes invalidate it. |
| [10.3.18 · Choose bounded glyph raster resolution](journey/62ih-choose-bounded-glyph-raster-resolution.md) | Relate projected world height to framebuffer pixels, fit the shaped extent within the texture limit, and read mask/color/subpixel coverage consistently. |
| [10.3.19 · Rasterize the shaped run into owned coverage bytes](journey/62ii-rasterize-the-shaped-run-into-owned-coverage-bytes.md) | Allocate bounded coverage storage from the shaped glyph bounds. |
| [10.3.20 · Upload and bind one world-text coverage texture](journey/62ij-upload-and-bind-one-world-text-coverage-texture.md) | Create the R8 coverage texture, upload rows with the declared byte layout, and bind the texture/sampler into one cached owner. |
| [10.3.21 · Build the world-plane text quad](journey/62ik-build-the-world-plane-text-quad.md) | Transform the original text plane into rebased GPU vertices. Keep original label placement and color independent of the generated quad. |
| [10.3.22 · Build world-text color and ID pipelines](journey/62il-build-world-text-color-and-id-pipelines.md) | The plane shader reads coverage while color and integer targets retain their distinct fragment contracts. Declare the same vertex layout for both passes. |
| [10.3.23 · Own the world-text buffer and pipeline lifetime](journey/62im-own-the-world-text-buffer-and-pipeline-lifetime.md) | Create retained GPU resources, replace target-dependent pipelines on retarget, and distinguish reset, release and capacity accounting. |
| [10.3.24 · Rebuild only invalid world-text cache entries](journey/62in-rebuild-only-invalid-world-text-cache-entries.md) | Match a label to its cache entry, validate the total texture budget before upload, and replace or append one owned entry. |
| [10.3.25 · Prepare and draw the retained world-text planes](journey/62io-prepare-and-draw-the-retained-world-text-planes.md) | Prune expired labels, skip clipped/empty planes, reuse valid rasters, build the current quads and issue draws from retained cache indices. |
| [10.3.26 · Collect anchored and overlay glyph work separately](journey/62ip-collect-anchored-and-overlay-glyph-work-separately.md) | Return borrow-scoped TextAreas alongside plate inputs and per-label depths. The borrowed runs survive through preparation; no new source owner is created. |
| [10.3.27 · Give the text lane its retained resource owners](journey/62iq-give-the-text-lane-its-retained-resource-owners.md) | Declare the lane and its caches, atlas, viewport, pipelines and preparation witness. Build glyph renderers against the appropriate depth predicates. |
| [10.3.28 · Construct and rebuild the text lane once](journey/62ir-construct-and-rebuild-the-text-lane-once.md) | Initialize font/raster resources and world/plate owners. Rebuild atlas-dependent resources when fonts change while keeping original labels in the document. |
| [10.3.29 · Reset the text lane and record scoped preparation costs](journey/62is-reset-the-text-lane-and-record-scoped-preparation-costs.md) | Retarget pipelines, clear logical work and release owned world/plate resources. Report raster images and buffer capacities with their actual scope. |
| [10.3.30 · Prepare glyphs only when the witness changes](journey/62it-prepare-glyphs-only-when-the-witness-changes.md) | Skip text preparation when its complete frame witness is unchanged. |
| [10.3.31 · Draw the prepared text passes in order](journey/62iu-draw-the-prepared-text-passes-in-order.md) | Draw world planes, depth-tested plates and anchored glyphs before overlay plates/glyphs. Connect this completed lane to the frame in the next existing lesson. |
| [10.3.32 · Prepare the full text frame and load the scene depth](journey/62j-frame-pass.md) | Prepare the full text frame and load the scene depth. |
| [10.3.33 · Merge original scene labels with the screen specimen](journey/62k-lane-labels.md) | Merge original scene labels with the screen specimen. |
| [10.3.34 · Draw original text IDs through bounded readback](journey/62l-picking.md) | Draw original text IDs through bounded readback. |
| [10.3.35 · Fetch missing whole fonts as one checked set](journey/62m-whole-fonts.md) | Fetch missing whole fonts as one checked set. |
| [10.3.36 · Show current text sources metrics and preparation errors](journey/62n-diagnostics.md) | Show current text sources metrics and preparation errors. |
| [10.3.37 · Create pick move and delete the original scene labels](journey/62o-labels.md) | Create pick move and delete the original scene labels. |
| [11.1.1 · Measure outline width in CSS pixels](journey/63-radius.md) | Measure outline width in CSS pixels. |
| [11.1.2 · Convolve solid coverage into external contour ink](journey/63a-coverage.md) | Write bounded contour sampling and black-alpha composition. |
| [11.1.3 · Pool and dilate a conservative outline mask](journey/63aa-pool-and-dilate-a-conservative-outline-mask.md) | Accelerate bounded contour sampling with conservative coarse coverage. |
| [11.1.4 · Own and count the derived outline textures](journey/63b-resources.md) | Own and count the derived outline textures. |
| [11.1.5 · Create the mask and fullscreen pipeline layouts](journey/63c-pipelines.md) | Create the mask and fullscreen pipeline layouts. |
| [11.1.6 · Build bounded mask and tap bindings](journey/63d-bindings.md) | Build bounded mask and tap bindings. |
| [11.1.7 · Reuse masks while camera selection and density stay unchanged](journey/63e-reuse.md) | Reuse masks while camera selection and density stay unchanged. |
| [11.1.8 · Outline the visible union without internal seams](journey/63f-union.md) | Outline the visible union without internal seams. |
| [11.1.9 · Connect a lazy silhouette owner to the renderer](journey/63g-composite.md) | Connect a lazy silhouette owner to the renderer. |
| [11.1.10 · Check real contours widths reuse picks and cleanup](journey/63h-outline.md) | Check real contours widths reuse picks and cleanup. |
| [11.2.1 · Check cloud cubes against their original point rows](journey/64-nodes.md) | Check cloud cubes against their original point rows. |
| [11.2.2 · Validate one rooted tree and disjoint original ranges](journey/64a-tree.md) | Validate one rooted tree and disjoint original ranges. |
| [11.2.3 · Prepare bounds and spacing without replacing source words](journey/64b-source.md) | Prepare bounds and spacing without replacing source words. |
| [11.2.4 · Place the cloud through an independent f64 instance model](journey/64c-placements.md) | Place the cloud through an independent f64 instance model. |
| [11.2.5 · Insert a checked cloud on the current layer](journey/64d-scene.md) | Insert a checked cloud on the current layer. |
| [11.2.6 · Include clouds in whole selection history and rectangle checks](journey/64e-identity.md) | Include clouds in whole selection history and rectangle checks. |
| [11.2.7 · Measure point spacing in the current projection](journey/64f-projection.md) | Measure point spacing in the current projection. |
| [11.2.8 · Walk only detail whose screen spacing needs refinement](journey/64g-detail.md) | Walk only detail whose screen spacing needs refinement. |
| [11.2.9 · Declare point-cloud records and decode source normals](journey/64h-shaders.md) | Match the existing forty-word row layout and decode packed normals with a guarded inverse-transpose transform. |
| [11.2.10 · Project original cloud rows to bounded point splats](journey/64ha-project-original-cloud-rows-to-bounded-point-splats.md) | Find the owning row by its original point interval and project one source position to a bounded screen disc. |
| [11.2.11 · Rasterize source point discs with integer ownership](journey/64hb-rasterize-source-point-discs-with-integer-ownership.md) | Expand each projected point into a bounded quad, reject fragments outside its disc and preserve original owner addresses. |
| [11.2.12 · Encode original cloud normals and resolve point targets](journey/64hc-encode-original-cloud-normals-and-resolve-point-targets.md) | Pack source normals once and resolve color, depth and original owner IDs with one pixel convention. |
| [11.2.13 · Check cloud shaders on the actual browser device](journey/64hd-check-cloud-shaders-on-the-actual-browser-device.md) | Await real shader compilation info before connecting the point passes. |
| [11.2.14 · Upload original point columns once per shared source](journey/64i-columns.md) | Upload original point columns once per shared source. |
| [11.2.15 · Own the cached cloud images and frame identity](journey/64j-targets.md) | Own the cached cloud images and frame identity. |
| [11.2.16 · Bind source storage and cached pixels explicitly](journey/64k-layouts.md) | Bind source storage and cached pixels explicitly. |
| [11.2.17 · Create a lazy cloud lane with reusable pipelines](journey/64l-pipelines.md) | Create a lazy cloud lane with reusable pipelines. |
| [11.2.18 · Share column uploads across placed cloud rows](journey/64m-cache.md) | Share column uploads across placed cloud rows. |
| [11.2.19 · Encode each source run in the exact 160-byte layout](journey/64n-records.md) | Encode each source run in the exact 160-byte layout. |
| [11.2.20 · Keep one shared record budget for the whole view](journey/64o-runs.md) | Keep one shared record budget for the whole view. |
| [11.2.21 · Cache the nearest point frame until its inputs change](journey/64p-frame.md) | Cache the nearest point frame until its inputs change. |
| [11.2.22 · Create and release the cloud renderer only when needed](journey/64q-owner.md) | Create and release the cloud renderer only when needed. |
| [11.2.23 · Use the cached cloud depth in colour and integer passes](journey/64r-draw.md) | Use the cached cloud depth in colour and integer passes. |
| [11.2.24 · Expose cloud source payload detail and actual GPU resources](journey/64s-diagnostics.md) | Expose cloud source payload detail and actual GPU resources. |
| [11.2.25 · Exercise original clouds and real view-dependent detail](journey/64t-clouds.md) | Exercise original clouds and real view-dependent detail. |
| [11.3.1 · Resolve source point identities and precise positions](journey/65-originals.md) | Resolve source point identities and precise positions. |
| [11.3.2 · Recover original IDs from integer cloud pixels](journey/65a-addresses.md) | Recover original IDs from integer cloud pixels. |
| [11.3.3 · Enter source controls with the current command or F10](journey/65b-controls.md) | Enter source controls with the current command or F10. |
| [11.3.4 · Pick original points and prove source query reuse](journey/65c-cloud-point.md) | Pick original points and prove source query reuse. |
| [11.3.5 · Read original manifest records and placements](journey/65za-read-original-manifest-records-and-placements.md) | Pin the parser dependencies and write geometry/text records with their defaults. |
| [11.3.6 · Validate original manifest text and source keys](journey/65zb-validate-original-manifest-text-and-source-keys.md) | Validate world text, heights and plane axes. |
| [12.1.1 · Parse and bound original scene manifests](journey/66-read-scene-manifests.md) | Parse supported manifest forms, then bound bytes, record counts and text content. |
| [12.1.2 · Compare live byte revisions](journey/66a-compare-live-byte-revisions.md) | Write per-URL ETag and byte-hash records. |
| [12.1.3 · Bound finite coordinates and packed integer reads](journey/66b-validate-source-records-before-allocation.md) | Guard scalar finiteness, coordinate triples and arithmetic before allocating or indexing. Decode a bounded varint without shifts past sixty-three bits. |
| [12.1.4 · Validate one NURBS knot axis](journey/66ba-validate-one-nurbs-knot-axis.md) | Relate order, control count and knot count, then reject nonfinite or decreasing knots before a kernel constructor. |
| [12.1.5 · Read bounded JSON count and knot fields](journey/66bb-read-bounded-json-count-and-knot-fields.md) | Read unsigned JSON counts with defaults and explicit caps; reuse the knot-axis contract instead of writing a second validator. |
| [12.1.6 · Match JSON control layout to declared counts](journey/66bc-match-json-control-layout-to-declared-counts.md) | Check rational coordinate width, rows, columns and curve stride against the actual arrays before constructor allocation. |
| [12.1.7 · Walk JSON records within a depth limit](journey/66bd-walk-json-records-within-a-depth-limit.md) | Use an explicit bounded stack to find original control records; reject excess nesting and reuse the already-written record validators. |
| [12.1.8 · Validate original points lines and placements](journey/66be-validate-original-points-lines-and-placements.md) | Check original coordinate-bearing records and affine placement without changing source values or identity. |
| [12.1.9 · Validate original mesh indices and vertex attributes](journey/66bf-validate-original-mesh-indices-and-vertex-attributes.md) | Check source vertex keys, finite attributes, face rings and triangulation before constructing a mesh. |
| [12.1.10 · Validate curve and surface control addresses](journey/66bg-validate-curve-and-surface-control-addresses.md) | Use checked stride arithmetic to locate the final addressed control and reuse the shared knot and scalar contracts. |
| [12.1.11 · Validate cloud columns and octree addresses](journey/66bh-validate-cloud-columns-and-octree-addresses.md) | Match optional original cloud columns and octree ranges to actual point and node counts. |
| [12.1.12 · Validate BRep Element payloads and shared definitions](journey/66bi-validate-brep-element-payloads-and-shared-definitions.md) | Check BRep records and encoded Element geometry, then validate every shared definition without expanding instances into copied geometry. |
| [12.1.13 · Validate the complete original protobuf session](journey/66bj-validate-the-complete-original-protobuf-session.md) | Apply the existing per-record validators and enforce total original-object and hierarchy-depth budgets before allocation. |
| [12.1.14 · Connect retained-session validation and independent witnesses](journey/66bk-connect-retained-session-validation-and-independent-witnesses.md) | Reuse the protobuf validators for already-decoded original kernel objects and register independent malformed-record witnesses. |
| [12.1.15 · Describe the saved scene metadata](journey/66c-describe-the-saved-scene-metadata.md) | Describe the saved scene metadata. |
| [12.1.16 · Encode decode the viewer file](journey/66d-encode-decode-the-viewer-file.md) | Encode decode the viewer file. |
| [12.1.17 · Give copied lines independent identities](journey/66e-give-copied-lines-independent-identities.md) | Give copied lines independent identities. |
| [12.1.18 · Snapshot original geometry types](journey/66f-snapshot-geometry-types.md) | Snapshot original geometry types. |
| [12.1.19 · Retain geometry free document headers](journey/66g-retain-geometry-free-document-headers.md) | Retain geometry free document headers. |
| [12.1.20 · Inventory saved document namespaces](journey/66h-inventory-saved-document-namespaces.md) | Inventory saved document namespaces. |
| [12.1.21 · Map saved source identities](journey/66i-map-saved-source-identities.md) | Map saved source identities. |
| [12.1.22 · Store cloud size as presentation](journey/66j-store-cloud-size-as-presentation.md) | Store cloud size as presentation. |
| [12.1.23 · Prepare original saved documents](journey/66k-prepare-saved-documents.md) | Prepare original saved documents. |
| [12.1.24 · Save document display settings](journey/66l-save-document-display-settings.md) | Save document display settings. |
| [12.1.25 · Save the complete supported scene](journey/66m-save-the-complete-supported-scene.md) | Save the complete supported scene. |
| [12.1.26 · Prepare original document sources](journey/66n-prepare-document-sources.md) | Prepare original document sources. |
| [12.1.27 · Prepare every supported geometry row](journey/66o-prepare-every-supported-geometry-row.md) | Prepare every supported geometry row. |
| [12.1.28 · Adopt a complete source document](journey/66p-adopt-a-complete-source-document.md) | Adopt a complete source document. |
| [12.1.29 · Restore world labels and layer links](journey/66q-restore-world-labels-layer-links.md) | Restore world labels and layer links. |
| [12.1.30 · Restore scoped presentation settings](journey/66r-restore-scoped-presentation-settings.md) | Restore scoped presentation settings. |
| [12.1.31 · Reopen a complete viewer file](journey/66s-reopen-a-complete-viewer-file.md) | Reopen a complete viewer file. |
| [12.1.32 · Prepare file adoption and live history](journey/66t-prepare-file-adoption-live-history.md) | Prepare file adoption and live history. |
| [12.1.33 · Share repeated source placements](journey/66u-share-repeated-source-placements.md) | Share repeated source placements. |
| [12.1.34 · Start an empty scene without examples](journey/66v-start-an-empty-scene-without-examples.md) | Start an empty scene without examples. |
| [12.1.35 · Assemble placed manifest entries](journey/66w-assemble-placed-manifest-entries.md) | Assemble placed manifest entries. |
| [12.1.36 · Resolve scene and live URLs](journey/66x-resolve-scene-live-urls.md) | Resolve scene and live URLs. |
| [12.1.37 · Retain validated live source revisions](journey/66y-retain-validated-live-source-revisions.md) | Retain validated live source revisions. |
| [12.1.38 · Fetch bounded source bodies](journey/66z-fetch-bounded-source-bodies.md) | Fetch bounded source bodies. |
| [12.1.39 · Read a complete URL source revision](journey/66aa-read-a-complete-url-source-revision.md) | Read a complete URL source revision. |
| [12.1.40 · Own and cancel the URL loader](journey/66ab-own-cancel-the-url-loader.md) | Own and cancel the URL loader. |
| [12.1.41 · Accept only current URL completions](journey/66ac-accept-only-current-url-completions.md) | Accept only current URL completions. |
| [12.1.42 · Watch published source changes](journey/66ad-watch-published-source-changes.md) | Watch published source changes. |
| [12.1.43 · Inspect published source identities](journey/66ae-inspect-published-source-identities.md) | Inspect published source identities. |
| [12.1.44 · Connect browser save open and URL adoption](journey/66af-connect-browser-save-open-url-adoption.md) | Connect browser save open and URL adoption. |
| [12.2.1 · Bound scalar headers and byte addresses](journey/67-supply-checked-array-layout-readers.md) | Bound scalar headers and byte addresses. |
| [12.2.2 · Locate original cloud and sheet array fields](journey/67sa-locate-original-cloud-and-sheet-array-fields.md) | Locate original cloud and sheet array fields. |
| [12.2.3 · Convert bounded doubles and cache metadata windows](journey/67sb-convert-bounded-doubles-and-cache-metadata-windows.md) | Convert bounded doubles and cache metadata windows. |
| [12.2.4 · Decode packed arrays for existing GPU owners](journey/67sc-decode-packed-arrays-for-existing-gpu-owners.md) | Decode packed arrays for existing GPU owners. |
| [12.2.5 · Descend bounded original protobuf messages](journey/67sd-descend-bounded-original-protobuf-messages.md) | Descend bounded original protobuf messages. |
| [12.2.6 · Find coordinate arrays without downloading their bodies](journey/67se-find-coordinate-arrays-without-downloading-their-bodies.md) | Find coordinate arrays without downloading their bodies. |
| [12.2.7 · Validate sheet side-array lengths](journey/67sf-validate-sheet-side-array-lengths.md) | Validate sheet side-array lengths. |
| [12.2.8 · Collect original octree node columns](journey/67sg-collect-original-octree-node-columns.md) | Collect original octree node columns. |
| [12.2.9 · Check octree bounds and unique child ownership](journey/67sh-check-octree-bounds-and-unique-child-ownership.md) | Check octree bounds and unique child ownership. |
| [12.2.10 · Validate complete octrees before range streaming](journey/67si-validate-complete-octrees-before-range-streaming.md) | Validate complete octrees before range streaming. |
| [12.2.11 · Own original range-read permits](journey/66za-own-original-range-read-permits.md) | Write the per-scene gate, queued turn, waiting-read and permit types. |
| [12.2.12 · Poll and cancel original queued range reads](journey/67a-queue-range-reads-in-arrival-order.md) | Complete the waiting-read Future and its Drop behavior. |
| [12.2.13 · Validate exact HTTP range replies](journey/67b-validate-exact-http-range-replies.md) | Validate exact HTTP range replies. |
| [12.2.14 · Retain original point page values](journey/67c-retain-original-point-page-values.md) | Retain original point page values. |
| [12.2.15 · Own cancellable range requests](journey/67d-own-cancellable-range-requests.md) | Own cancellable range requests. |
| [12.2.16 · Time out stalled source reads](journey/67e-time-out-stalled-source-reads.md) | Time out stalled source reads. |
| [12.2.17 · Bound incoming range body chunks](journey/67f-bound-incoming-range-body-chunks.md) | Bound incoming range body chunks. |
| [12.2.18 · Request and accept one source interval](journey/67g-request-and-accept-one-source-interval.md) | Request and accept one source interval. |
| [12.2.19 · Keep range revisions and background results](journey/67h-keep-range-revisions-and-background-results.md) | Keep range revisions and background results. |
| [12.2.20 · Queue original source range reads](journey/67hza-queue-original-source-range-reads.md) | Write the browser range-read owner and cached metadata-window read. |
| [12.2.21 · Probe original source array layouts](journey/67hzb-probe-original-source-array-layouts.md) | Start owned asynchronous range tasks and inspect the bounded file probe. |
| [12.2.22 · Read original cloud node tables](journey/67hzc-read-original-cloud-node-tables.md) | Read original node-table fields through the cached metadata window. |
| [12.2.23 · Fetch original coordinates normals and colours](journey/67i-read-source-arrays-through-the-bounded-queue.md) | Fetch bounded source-array slices through the queue already written. |
| [12.2.24 · Keep original ranged source identity](journey/67j-keep-original-ranged-source-identity.md) | Keep original ranged source identity. |
| [12.2.25 · Prepare a revision bound remote source](journey/67k-prepare-a-revision-bound-remote-source.md) | Prepare a revision bound remote source. |
| [12.2.26 · Fetch precise coordinates and original IDs](journey/67l-fetch-precise-coordinates-and-original-ids.md) | Fetch precise coordinates and original IDs. |
| [12.2.27 · Cancel range work when its runtime closes](journey/67m-cancel-range-work-when-its-runtime-closes.md) | Cancel range work when its runtime closes. |
| [12.3.1 · Reserve page bytes before reading](journey/68-reserve-page-bytes-before-reading.md) | Reserve page bytes before reading. |
| [12.3.2 · Keep shared pages alive during eviction](journey/68a-keep-shared-pages-alive-during-eviction.md) | Keep shared pages alive during eviction. |
| [12.3.3 · Evict unpinned pages in access order](journey/68b-evict-unpinned-pages-in-access-order.md) | Evict unpinned pages in access order. |
| [12.3.4 · Reject late source generations](journey/68c-reject-late-source-generations.md) | Reject late source generations. |
| [12.3.5 · Bound source prefixes and decode work](journey/68d-bound-source-prefixes-and-decode-work.md) | Bound source prefixes and decode work. |
| [12.3.6 · Validate original streamed presentation](journey/68e-validate-original-streamed-presentation.md) | Validate original streamed presentation. |
| [12.3.7 · Reuse a checked source probe](journey/68f-reuse-a-checked-source-probe.md) | Reuse a checked source probe. |
| [12.3.8 · Validate complete source metadata](journey/68fa-validate-complete-source-metadata.md) | Refuse mixed or placed originals before choosing sampled range display. |
| [12.3.9 · Slice original pages without losing attributes](journey/68g-slice-original-pages-without-losing-attributes.md) | Slice original pages without losing attributes. |
| [12.3.10 · Prepare shared remote source owners](journey/68h-prepare-shared-remote-source-owners.md) | Prepare shared remote source owners. |
| [12.3.11 · Keep independent streamed placements](journey/68i-keep-independent-streamed-placements.md) | Keep independent streamed placements. |
| [12.3.12 · Pin coarse pages and adopt checked ranges](journey/68j-pin-coarse-pages-and-adopt-checked-ranges.md) | Pin coarse pages and adopt checked ranges. |
| [12.3.13 · Supply coarse and detailed page specimens](journey/68k-supply-coarse-and-detailed-page-specimens.md) | Supply coarse and detailed page specimens. |
| [12.3.14 · Insert streamed rows on the current layer](journey/68l-insert-streamed-rows-on-the-current-layer.md) | Insert streamed rows on the current layer. |
| [12.3.15 · Carry source placement through scene operations](journey/68m-carry-source-placement-through-scene-operations.md) | Carry source placement through scene operations. |
| [12.3.16 · Resolve compact picks to original point IDs](journey/68n-resolve-compact-picks-to-original-point-ids.md) | Resolve compact picks to original point IDs. |
| [12.3.17 · Account for original and retained source data](journey/68o-account-for-original-and-retained-source-data.md) | Account for original and retained source data. |
| [12.3.18 · Keep original attributes in ranged pages](journey/68p-keep-original-attributes-in-ranged-pages.md) | Keep original attributes in ranged pages. |
| [12.3.19 · Retain page owners beside GPU buffers](journey/68q-retain-page-owners-beside-gpu-buffers.md) | Retain page owners beside GPU buffers. |
| [12.3.20 · Describe resident and streamed draw rows](journey/68r-describe-resident-and-streamed-draw-rows.md) | Describe resident and streamed draw rows. |
| [12.3.21 · Draw shared source pages at independent placements](journey/68s-draw-shared-source-pages-at-independent-placements.md) | Draw shared source pages at independent placements. |
| [12.3.22 · Validate encoded source metadata](journey/68t-validate-encoded-source-metadata.md) | Validate encoded source metadata. |
| [12.3.23 · Decode bounded cancellable source streams](journey/68u-decode-bounded-cancellable-source-streams.md) | Decode bounded cancellable source streams. |
| [12.3.24 · Refuse conflicting source encoding revisions](journey/68v-refuse-conflicting-source-encoding-revisions.md) | Refuse conflicting source encoding revisions. |
| [12.3.25 · Adopt remote original revisions independently](journey/68w-adopt-remote-original-revisions-independently.md) | Adopt remote original revisions independently. |
| [12.3.26 · Prepare a coarse pin and bounded prefix](journey/68x-prepare-a-coarse-pin-and-bounded-prefix.md) | Prepare a coarse pin and bounded prefix. |
| [12.3.27 · Redraw before reusing evicted page addresses](journey/68y-redraw-before-reusing-evicted-page-addresses.md) | Redraw before reusing evicted page addresses. |
| [12.3.28 · Continue pages only for current source owners](journey/68z-continue-pages-only-for-current-source-owners.md) | Continue pages only for current source owners. |
| [12.3.29 · Own cancellation for each published source](journey/68aa-own-cancellation-for-each-published-source.md) | Own cancellation for each published source. |
| [12.3.30 · Choose bounded range loading for large originals](journey/68ab-choose-bounded-range-loading-for-large-originals.md) | Choose bounded range loading for large originals. |
| [12.3.31 · Connect current pages to the browser renderer](journey/68ac-connect-current-pages-to-the-browser-renderer.md) | Connect current pages to the browser renderer. |
| [13.1.1 · Validate original definition references](journey/69-validate-original-definition-references.md) | Validate original definition references. |
| [13.1.2 · Prepare each used definition once](journey/69b-prepare-each-used-definition-once.md) | Prepare each used definition once. |
| [13.1.3 · Keep instance bindings through scene history](journey/69c-keep-instance-bindings-through-scene-history.md) | Keep instance bindings through scene history. |
| [13.1.4 · Append independently placed shared rows](journey/69d-append-independently-placed-shared-rows.md) | Append independently placed shared rows. |
| [13.1.5 · Import canonical original instance placements](journey/69e-import-canonical-original-instance-placements.md) | Import canonical original instance placements. |
| [13.1.6 · Protect shared originals during part edits](journey/69f-protect-shared-originals-during-part-edits.md) | Protect shared originals during part edits. |
| [13.1.7 · Collect shared definitions without flattening](journey/69g-collect-shared-definitions-without-flattening.md) | Collect shared definitions without flattening. |
| [13.1.8 · Save original instance references and namespaces](journey/69h-save-original-instance-references-and-namespaces.md) | Save original instance references and namespaces. |
| [13.1.9 · Check sharing precision history and release](journey/69i-check-sharing-precision-history-and-release.md) | Check sharing precision history and release. |
| [13.1.10 · Preserve per instance original colours](journey/69j-preserve-per-instance-original-colours.md) | Preserve per instance original colours. |
| [13.1.11 · Create a shared definition face pipeline](journey/69k-create-a-shared-definition-face-pipeline.md) | Create a shared definition face pipeline. |
| [13.1.12 · Reuse bounded independent draw records](journey/69l-reuse-bounded-independent-draw-records.md) | Reuse bounded independent draw records. |
| [13.1.13 · Draw batched faces with independent integer owners](journey/69m-draw-batched-faces-with-independent-integer-owners.md) | Draw batched faces with independent integer owners. |
| [13.1.14 · Inspect and exercise original definition instances](journey/69n-inspect-and-exercise-original-definition-instances.md) | Inspect and exercise original definition instances. |
| [13.1.15 · Move between original rigid frames](journey/69zza-move-between-original-rigid-frames.md) | Write local coordinates and the rotation between two right-handed frames. |
| [13.1.16 · Find an original frame and pack key words](journey/69zzb-find-an-original-frame-and-pack-key-words.md) | Choose well-separated original surface control points for a right-handed frame. |
| [13.1.17 · Encode original parameters and topology](journey/69zzc-encode-original-parameters-and-topology.md) | Encode knots, local points, pcurve values and curve descriptors. |
| [13.1.18 · Assemble an original rigid shape key](journey/69a-describe-a-rigid-frame-and-original-shape-key.md) | Assemble the existing encoders for every original surface, curve, vertex and topology record. |
| [13.1.19 · Retain cloneable derived CAD values](journey/69aa-retain-cloneable-derived-cad-values.md) | Retain cloneable derived CAD values. |
| [13.1.20 · Replay sampled geometry in the new rigid frame](journey/69ab-replay-sampled-geometry-in-the-new-rigid-frame.md) | Replay sampled geometry in the new rigid frame. |
| [13.1.21 · Guard reusable walks with source validity and size](journey/69ac-guard-reusable-walks-with-source-validity-and-size.md) | Guard reusable walks with source validity and size. |
| [13.1.22 · Share recordings only during document preparation](journey/69ad-share-recordings-only-during-document-preparation.md) | Share recordings only during document preparation. |
| [13.1.23 · Compare actual replay pixels and integer owners](journey/69ae-compare-actual-replay-pixels-and-integer-owners.md) | Compare actual replay pixels and integer owners. |
| [13.1.24 · Exercise independent original CAD reuse](journey/69af-exercise-independent-original-cad-reuse.md) | Exercise independent original CAD reuse. |
| [13.2.1 · Evaluate source ancestry once per document](journey/70-evaluate-source-ancestry-once-per-document.md) | Evaluate source ancestry once per document. |
| [13.2.2 · Keep original local parent and world matrices](journey/70a-keep-original-local-parent-and-world-matrices.md) | Keep original local parent and world matrices. |
| [13.2.3 · Connect canonical cached models to every source owner](journey/70b-connect-canonical-cached-models-to-every-source-owner.md) | Connect canonical cached models to every source owner. |
| [13.2.4 · Save nested local frames without flattening](journey/70c-save-nested-local-frames-without-flattening.md) | Save nested local frames without flattening. |
| [13.2.5 · Exercise rotated and mirrored original references](journey/70d-exercise-rotated-and-mirrored-original-references.md) | Exercise rotated and mirrored original references. |
| [13.2.6 · Read original sheet metadata records](journey/70za-read-original-sheet-metadata-records.md) | Describe the original sheet side-table layout and entity identity fields. |
| [14.1.1 · Own cancellable sheet-entity lookups](journey/71-read-original-sheet-entity-metadata.md) | Complete the original lookup owner with its query id, sheet row and entity index. |
| [14.1.2 · Bound original entity records before reading](journey/71a-bound-original-entity-records-before-reading.md) | Bound original entity records before reading. |
| [14.1.3 · Give metadata reads their own cancellation signal](journey/71b-give-metadata-reads-their-own-cancellation-signal.md) | Give metadata reads their own cancellation signal. |
| [14.1.4 · Fetch one revision matched metadata answer](journey/71c-fetch-one-revision-matched-metadata-answer.md) | Fetch one revision matched metadata answer. |
| [14.1.5 · Decode precise original sheet columns](journey/71d-decode-precise-original-sheet-columns.md) | Decode precise original sheet columns. |
| [14.1.6 · Measure original sheet pages and world pens](journey/71e-measure-original-sheet-pages-and-world-pens.md) | Measure original sheet pages and world pens. |
| [14.1.7 · Retain charged pages beside resident pages](journey/71f-retain-charged-pages-beside-resident-pages.md) | Retain charged pages beside resident pages. |
| [14.1.8 · Own the complete original drawing source](journey/71g-own-the-complete-original-drawing-source.md) | Own the complete original drawing source. |
| [14.1.9 · Place drawing rows without changing endpoints](journey/71h-place-drawing-rows-without-changing-endpoints.md) | Place drawing rows without changing endpoints. |
| [14.1.10 · Register original drawing rows on the current layer](journey/71i-register-original-drawing-rows-on-the-current-layer.md) | Register original drawing rows on the current layer. |
| [14.1.11 · Carry drawing identity through scene operations](journey/71j-carry-drawing-identity-through-scene-operations.md) | Carry drawing identity through scene operations. |
| [14.1.12 · Save and reopen complete original sheets](journey/71k-save-and-reopen-complete-original-sheets.md) | Save and reopen complete original sheets. |
| [14.1.13 · Locate original drawing identity and fixed ranges](journey/71l-locate-original-drawing-identity-and-fixed-ranges.md) | Locate original drawing identity and fixed ranges. |
| [14.1.14 · Validate the bounded drawing envelope tail](journey/71m-validate-the-bounded-drawing-envelope-tail.md) | Validate the bounded drawing envelope tail. |
| [14.1.15 · Scan drawing columns under one revision](journey/71n-scan-drawing-columns-under-one-revision.md) | Scan drawing columns under one revision. |
| [14.1.16 · Read drawing page columns through the shared queue](journey/71o-read-drawing-page-columns-through-the-shared-queue.md) | Read drawing page columns through the shared queue. |
| [14.1.17 · Adopt nonoverlapping charged drawing pages](journey/71p-adopt-nonoverlapping-charged-drawing-pages.md) | Adopt nonoverlapping charged drawing pages. |
| [14.1.18 · Keep partial drawing sources explicit](journey/71q-keep-partial-drawing-sources-explicit.md) | Keep partial drawing sources explicit. |
| [14.1.19 · Reserve memory before the drawing prefix](journey/71r-reserve-memory-before-the-drawing-prefix.md) | Reserve memory before the drawing prefix. |
| [14.1.20 · Continue drawing reads without reviving closed sources](journey/71s-continue-drawing-reads-without-reviving-closed-sources.md) | Continue drawing reads without reviving closed sources. |
| [14.1.21 · Resolve sheet picks to original entity IDs](journey/71t-resolve-sheet-picks-to-original-entity-ids.md) | Resolve sheet picks to original entity IDs. |
| [14.1.22 · Project authored world pens into drawing quads](journey/71u-project-authored-world-pens-into-drawing-quads.md) | Project authored world pens into drawing quads. |
| [14.1.23 · Use the same sheet coverage for color and picks](journey/71v-use-the-same-sheet-coverage-for-color-and-picks.md) | Use the same sheet coverage for color and picks. |
| [14.1.24 · Build sheet color and integer pipelines](journey/71w-build-sheet-color-and-integer-pipelines.md) | Build sheet color and integer pipelines. |
| [14.1.25 · Own the shared drawing lane resources](journey/71x-own-the-shared-drawing-lane-resources.md) | Own the shared drawing lane resources. |
| [14.1.26 · Bound drawing buffers and pack independent settings](journey/71y-bound-drawing-buffers-and-pack-independent-settings.md) | Bound drawing buffers and pack independent settings. |
| [14.1.27 · Upload each original drawing page once](journey/71z-upload-each-original-drawing-page-once.md) | Upload each original drawing page once. |
| [14.1.28 · Reuse independent drawing placement records](journey/71aa-reuse-independent-drawing-placement-records.md) | Reuse independent drawing placement records. |
| [14.1.29 · Draw world pens with the current camera lens](journey/71ab-draw-world-pens-with-the-current-camera-lens.md) | Draw world pens with the current camera lens. |
| [14.1.30 · Connect sheet batches to rendering and bounded picks](journey/71ac-connect-sheet-batches-to-rendering-and-bounded-picks.md) | Connect sheet batches to rendering and bounded picks. |
| [14.1.31 · Own metadata delivery without pinning drawings](journey/71ad-own-metadata-delivery-without-pinning-drawings.md) | Own metadata delivery without pinning drawings. |
| [14.1.32 · Query metadata only for the current selected entity](journey/71ae-query-metadata-only-for-the-current-selected-entity.md) | Query metadata only for the current selected entity. |
| [14.1.33 · Keep published drawing sources and URL bases independent](journey/71af-keep-published-drawing-sources-and-url-bases-independent.md) | Keep published drawing sources and URL bases independent. |
| [14.1.34 · Retire staged drawing reads with their publication owner](journey/71ag-retire-staged-drawing-reads-with-their-publication-owner.md) | Retire staged drawing reads with their publication owner. |
| [14.1.35 · Stage bounded drawings through publication loading](journey/71ah-stage-bounded-drawings-through-publication-loading.md) | Stage bounded drawings through publication loading. |
| [14.1.36 · Show current sheet resources and selected entity answers](journey/71ai-show-current-sheet-resources-and-selected-entity-answers.md) | Show current sheet resources and selected entity answers. |
| [14.1.37 · Exercise complete and ranged original drawings](journey/71aj-exercise-complete-and-ranged-original-drawings.md) | Exercise complete and ranged original drawings. |
| [14.1.38 · Build original construction-plane frames](journey/71za-build-original-construction-plane-frames.md) | Describe the original world planes and a movable frame. |
| [14.1.39 · Map original plane and world coordinates](journey/71zb-map-original-plane-and-world-coordinates.md) | Reuse the existing origin and axes to map between plane and world coordinates. |
| [14.1.40 · Guard original plane and ray projections](journey/71zc-guard-original-plane-and-ray-projections.md) | Complete the original plane queries for view choice, nearest projection, facing and ray intersection. |
| [14.1.41 · Parse original drawing coordinate forms](journey/71zd-parse-original-drawing-coordinate-forms.md) | Describe the original absolute, relative, polar and distance forms. |
| [14.1.42 · Resolve original relative and polar coordinates](journey/71ze-resolve-original-relative-and-polar-coordinates.md) | Recognize the existing world prefix and resolve parsed coordinates from the plane origin, previous point and current direction. |
| [14.2.1 · Place original coordinates in the selected plane](journey/72-copy-current-construction-planes-and-coordinate-forms.md) | Complete absolute coordinates, offsets and point placement with the original construction-plane rules. |
| [14.2.2 · Resolve bounded typed batches in the current plane](journey/72a-resolve-bounded-typed-batches-in-the-current-plane.md) | Resolve bounded typed batches in the current plane. |
| [14.2.3 · Place clicks on stable construction planes](journey/72b-place-clicks-on-stable-construction-planes.md) | Place clicks on stable construction planes. |
| [14.2.4 · Name the construction plane chooser options](journey/72c-name-the-construction-plane-chooser-options.md) | Name the construction plane chooser options. |
| [14.2.5 · Add original polylines to the point tool](journey/72d-add-original-polylines-to-the-point-tool.md) | Add original polylines to the point tool. |
| [14.2.6 · Keep construction frames and plane points in drafts](journey/72e-keep-construction-frames-and-plane-points-in-drafts.md) | Keep construction frames and plane points in drafts. |
| [14.2.7 · Replay finished drafts with explicit world prefixes](journey/72f-replay-finished-drafts-with-explicit-world-prefixes.md) | Replay finished drafts with explicit world prefixes. |
| [14.2.8 · Own the fixed construction plane in the editor](journey/72g-own-the-fixed-construction-plane-in-the-editor.md) | Own the fixed construction plane in the editor. |
| [14.2.9 · Commit completed original geometry exactly once](journey/72h-commit-completed-original-geometry-exactly-once.md) | Commit completed original geometry exactly once. |
| [14.2.10 · Change fixed planes without rewriting accepted points](journey/72i-change-fixed-planes-without-rewriting-accepted-points.md) | Change fixed planes without rewriting accepted points. |
| [14.2.11 · Open the plane chooser and collect its frame](journey/72j-open-the-plane-chooser-and-collect-its-frame.md) | Open the plane chooser and collect its frame. |
| [14.2.12 · Resolve typed input against the current draft](journey/72k-resolve-typed-input-against-the-current-draft.md) | Resolve typed input against the current draft. |
| [14.2.13 · Finish open tools with Enter and route typed forms](journey/72l-finish-open-tools-with-enter-and-route-typed-forms.md) | Finish open tools with Enter and route typed forms. |
| [14.2.14 · Validate complete drawing commands before adoption](journey/72m-validate-complete-drawing-commands-before-adoption.md) | Validate complete drawing commands before adoption. |
| [14.2.15 · Close original polylines as one transaction](journey/72n-close-original-polylines-as-one-transaction.md) | Close original polylines as one transaction. |
| [14.2.16 · Snap draft starts and use the construction frame grid](journey/72o-snap-draft-starts-and-use-the-construction-frame-grid.md) | Snap draft starts and use the construction frame grid. |
| [14.2.17 · Register current drawing commands and plane choices](journey/72p-register-current-drawing-commands-and-plane-choices.md) | Register current drawing commands and plane choices. |
| [14.2.18 · Preview accepted segments without committing originals](journey/72q-preview-accepted-segments-without-committing-originals.md) | Preview accepted segments without committing originals. |
| [14.2.19 · Expose original drawing coordinates and cancellation state](journey/72r-expose-original-drawing-coordinates-and-cancellation-state.md) | Expose original drawing coordinates and cancellation state. |
| [14.2.20 · Exercise current drawing tools and original persistence](journey/72s-exercise-current-drawing-tools-and-original-persistence.md) | Exercise current drawing tools and original persistence. |
| [14.3.1 · Construct rectangles and polygons in plane axes](journey/73-construct-rectangles-and-polygons-in-plane-axes.md) | Construct rectangles and polygons in plane axes. |
| [14.3.2 · Bound generated original construction points](journey/73a-bound-generated-original-construction-points.md) | Bound generated original construction points. |
| [14.3.3 · Give point tools a plane aware construction hook](journey/73b-give-point-tools-a-plane-aware-construction-hook.md) | Give point tools a plane aware construction hook. |
| [14.3.4 · Build original rectangle and polygon tool results](journey/73c-build-original-rectangle-and-polygon-tool-results.md) | Build original rectangle and polygon tool results. |
| [14.3.5 · Preserve world replay for multiword construction names](journey/73d-preserve-world-replay-for-multiword-construction-names.md) | Preserve world replay for multiword construction names. |
| [14.3.6 · Keep polygon sides in the current draft](journey/73e-keep-polygon-sides-in-the-current-draft.md) | Keep polygon sides in the current draft. |
| [14.3.7 · Register current planar options and temporary outlines](journey/73f-register-current-planar-options-and-temporary-outlines.md) | Register current planar options and temporary outlines. |
| [14.3.8 · Exercise oriented original planar constructions](journey/73g-exercise-oriented-original-planar-constructions.md) | Exercise oriented original planar constructions. |
| [14.3.9 · Match solid command options](journey/73h-match-solid-command-options.md) | Compare normalized option words while keeping Enter and Escape outside shape choices.. |
| [14.3.10 · Describe solid command questions](journey/73i-describe-solid-command-questions.md) | Represent typed coordinates and numbers explicitly. |
| [14.3.11 · Build a right-handed solid frame](journey/73j-build-a-right-handed-solid-frame.md) | Place kernel coordinates in a construction-plane frame. |
| [14.3.12 · Sample solid preview wires](journey/73k-sample-solid-preview-wires.md) | Sample circles and boxes in the existing frame. |
| [14.3.13 · Describe solid builders and dimensions](journey/73l-describe-solid-builders-and-dimensions.md) | Read sizes and signed heights from answers, then describe one creation command through question, preview and kernel-build functions.. |
| [14.3.14 · Validate solid sizes and radius](journey/73m-validate-solid-sizes-and-radius.md) | Reject nonfinite and degenerate dimensions. |
| [14.3.15 · Weld solid tessellation once](journey/73n-weld-solid-tessellation-once.md) | Join kernel face meshes by reusing nearby corners, then retain either a placed BRep or its mesh representation. |
| [14.3.16 · Scale and cache polyhedron faces](journey/73o-scale-and-cache-polyhedron-faces.md) | Cache unit polyhedron face outlines, then apply the requested circumradius and the existing placement frame. |
| [14.3.17 · Build a placed kernel box](journey/73p-build-a-placed-kernel-box.md) | Build a box through the Session kernel and place its center half a signed height above the construction plane. |
| [14.3.18 · Read box questions and corner clicks](journey/73q-read-box-questions-and-corner-clicks.md) | Connect the box builder to its question sequence. |
| [14.3.19 · Build an oriented kernel pyramid](journey/73r-build-an-oriented-kernel-pyramid.md) | Read a base diagonal and signed height. |
| [14.3.20 · Build a block with a kernel hole](journey/73s-build-a-block-with-a-kernel-hole.md) | Reuse the existing box questions and dimension reader. |
| [14.3.21 · Build regular kernel polyhedra](journey/73t-build-regular-kernel-polyhedra.md) | Connect tetrahedron and octahedron constructors to the shared radius questions, unit-face cache and placed builder. |
| [14.4.1 · Add the remaining regular kernel solids](journey/74-copy-current-solid-geometry-and-question-definitions.md) | Build icosahedron and dodecahedron through the shared kernel solid helpers. |
| [14.4.2 · Own current shape questions and answer state](journey/74a-own-current-shape-questions-and-answer-state.md) | Own current shape questions and answer state. |
| [14.4.3 · Validate a new answer before changing its original state](journey/74b-validate-a-new-answer-before-changing-its-original-state.md) | Validate a new answer before changing its original state. |
| [14.4.4 · Build only after the last shape question](journey/74c-build-only-after-the-last-shape-question.md) | Build only after the last shape question. |
| [14.4.5 · Answer shape dimensions with atomic typed batches](journey/74d-answer-shape-dimensions-with-atomic-typed-batches.md) | Answer shape dimensions with atomic typed batches. |
| [14.4.6 · Preview solid wires and measure normal axis clicks](journey/74e-preview-solid-wires-and-measure-normal-axis-clicks.md) | Preview solid wires and measure normal axis clicks. |
| [14.4.7 · Adopt original primitive geometry through existing rows](journey/74f-adopt-original-primitive-geometry-through-existing-rows.md) | Adopt original primitive geometry through existing rows. |
| [14.4.8 · Keep primitive answers beside the existing drawing draft](journey/74g-keep-primitive-answers-beside-the-existing-drawing-draft.md) | Keep primitive answers beside the existing drawing draft. |
| [14.4.9 · Route clicked points to the current shape question](journey/74h-route-clicked-points-to-the-current-shape-question.md) | Route clicked points to the current shape question. |
| [14.4.10 · Route typed shape input and preserve detailed errors](journey/74i-route-typed-shape-input-and-preserve-detailed-errors.md) | Route typed shape input and preserve detailed errors. |
| [14.4.11 · Synchronize shape frames before accepting plane changes](journey/74j-synchronize-shape-frames-before-accepting-plane-changes.md) | Synchronize shape frames before accepting plane changes. |
| [14.4.12 · Resolve height clicks without closing an open shape collector](journey/74k-resolve-height-clicks-without-closing-an-open-shape-collector.md) | Resolve height clicks without closing an open shape collector. |
| [14.4.13 · Prepare complete solid commands before replacing a draft](journey/74l-prepare-complete-solid-commands-before-replacing-a-draft.md) | Prepare complete solid commands before replacing a draft. |
| [14.4.14 · Register every current solid name and representation](journey/74m-register-every-current-solid-name-and-representation.md) | Register every current solid name and representation. |
| [14.4.15 · Draw temporary solid wire sets and report current input failures](journey/74n-draw-temporary-solid-wire-sets-and-report-current-input-failures.md) | Draw temporary solid wire sets and report current input failures. |
| [14.4.16 · Keep quick mouse drift as a click using current event time](journey/74o-keep-quick-mouse-drift-as-a-click-using-current-event-time.md) | Keep quick mouse drift as a click using current event time. |
| [14.4.17 · Exercise current solid input and original persistence](journey/74p-exercise-current-solid-input-and-original-persistence.md) | Exercise current solid input and original persistence. |
| [14.4.18 · Build original cylinders with signed height](journey/74za-build-original-cylinders-with-signed-height.md) | Build cylinders with positive or negative height using the solid frame and radius checks already taught. |
| [14.4.19 · Build original cones and spheres](journey/74zb-build-original-cones-and-spheres.md) | Reuse the cylinder’s existing center, radius and height questions for a cone. |
| [14.5.1 · Build original tori with checked radii](journey/75-reuse-current-round-solid-builders.md) | Collect a center and two radii with the existing question owner. |
| [14.5.2 · Build original capsules with shared dimensions](journey/75aa-build-original-capsules-with-shared-dimensions.md) | Reuse the cylinder’s existing center, radius and signed-height questions. |
| [14.5.3 · Mesh original quad-sphere patches](journey/75a-retain-mesh-only-round-families.md) | Ask the kernel for six original quad-sphere patches. |
| [14.5.4 · Register current round names and choices](journey/75b-register-current-round-names-and-choices.md) | Register current round names and choices. |
| [14.5.5 · Exercise round originals and one transaction](journey/75c-exercise-round-originals-and-one-transaction.md) | Exercise round originals and one transaction. |
| [14.5.6 · Build original planar circles](journey/75za-build-original-planar-circles.md) | Reuse the checked frame and positive-radius helpers from 73h–74. |
| [14.5.7 · Build original planar ellipses](journey/75zb-build-original-planar-ellipses.md) | Reuse the same checked frame and radius owner. |
| [14.5.8 · Build exact rational arc spans](journey/75zc-build-exact-rational-arc-spans.md) | Build the original circular arc from rational quadratic spans. |
| [14.5.9 · Read original arcs from center and angle](journey/75zd-read-original-arcs-from-center-and-angle.md) | Read the existing center-based arc inputs using the checked frame. |
| [14.5.10 · Read three-point arcs and connect the recipe](journey/75ze-read-three-point-arcs-and-connect-the-recipe.md) | Complete the existing arc recipe with its two input modes. |
| [14.5.11 · Build original parabola previews and sources](journey/75zf-build-original-parabola-previews-and-sources.md) | Read the prepared start, apex and end from the checked local frame. |
| [14.6.1 · Read original parabola inputs in a checked frame](journey/76-copy-exact-planar-curve-definitions.md) | Complete the original parabola questions and input reader. |
| [14.6.2 · Expose exact curve choices in the existing registry](journey/76a-expose-exact-curve-choices-in-the-existing-registry.md) | Expose exact curve choices in the existing registry. |
| [14.6.3 · Create editable freeform curves](journey/76b-retain-editable-freeform-controls-in-the-point-collector.md) | Write the original degree and closure rules for collected control points. |
| [14.6.4 · Inspect original curve data through the browser contract](journey/76c-inspect-original-curve-data-through-the-browser-contract.md) | Inspect original curve data through the browser contract. |
| [14.6.5 · Exercise exact curves and rational control edits](journey/76d-exercise-exact-curves-and-rational-control-edits.md) | Exercise exact curves and rational control edits. |
| [15.1.1 · Build directional scale transforms](journey/77-copy-current-transformation-and-source-edit-algorithms.md) | Build directional scale transforms. |
| [15.1.2 · Compute signed construction-plane rotation](journey/77sa-compute-signed-construction-plane-rotation.md) | Compute signed construction-plane rotation. |
| [15.1.3 · Address original controls and exclusive part selections](journey/77sb-address-original-controls-and-exclusive-part-selections.md) | Address original controls and exclusive part selections. |
| [15.1.4 · Collect finite original controls and mesh vertices](journey/77sc-collect-finite-original-controls-and-mesh-vertices.md) | Collect finite original controls and mesh vertices. |
| [15.1.5 · Connect curve polygons and surface control nets](journey/77sd-connect-curve-polygons-and-surface-control-nets.md) | Connect curve polygons and surface control nets. |
| [15.1.6 · Reuse control collectors across original geometry families](journey/77se-reuse-control-collectors-across-original-geometry-families.md) | Reuse control collectors across original geometry families. |
| [15.1.7 · Represent original geometry edit targets](journey/77sf-represent-original-geometry-edit-targets.md) | Represent original geometry edit targets. |
| [15.1.8 · Resolve original mesh vertices for part edits](journey/77sg-resolve-original-mesh-vertices-for-part-edits.md) | Resolve original mesh vertices for part edits. |
| [15.1.9 · Resolve surface grids and individual source controls](journey/77sh-resolve-surface-grids-and-individual-source-controls.md) | Resolve surface grids and individual source controls. |
| [15.1.10 · Collect affected points across CAD geometry families](journey/77si-collect-affected-points-across-cad-geometry-families.md) | Collect affected points across CAD geometry families. |
| [15.1.11 · Preserve rational weights and validate BRep boundaries](journey/77sj-preserve-rational-weights-and-validate-brep-boundaries.md) | Preserve rational weights and validate BRep boundaries. |
| [15.1.12 · Transform original mesh and surface controls](journey/77sk-transform-original-mesh-and-surface-controls.md) | Transform original mesh and surface controls. |
| [15.1.13 · Transform original line polyline point and curve controls](journey/77sl-transform-original-line-polyline-point-and-curve-controls.md) | Transform original line polyline point and curve controls. |
| [15.1.14 · Transform shared BRep boundary positions coherently](journey/77sm-transform-shared-brep-boundary-positions-coherently.md) | Transform shared BRep boundary positions coherently. |
| [15.1.15 · Dispatch original part edits and retain Element ownership](journey/77sn-dispatch-original-part-edits-and-retain-element-ownership.md) | Dispatch original part edits and retain Element ownership. |
| [15.1.16 · Map sparse original vertex keys to display slots](journey/77so-map-sparse-original-vertex-keys-to-display-slots.md) | Map sparse original vertex keys to display slots. |
| [15.1.17 · Compute polygon normals from original boundaries](journey/77sp-compute-polygon-normals-from-original-boundaries.md) | Compute polygon normals from original boundaries. |
| [15.1.18 · Build one original edge table and record incident faces](journey/77sq-build-one-original-edge-table-and-record-incident-faces.md) | Build one original edge table and record incident faces. |
| [15.1.19 · Preserve hole winding and complete original mesh topology](journey/77sr-preserve-hole-winding-and-complete-original-mesh-topology.md) | Preserve hole winding and complete original mesh topology. |
| [15.1.20 · Own transformation questions and the selected original targets](journey/77a-own-transformation-questions-and-the-selected-original-targets.md) | Own transformation questions and the selected original targets. |
| [15.1.21 · Validate point and number transformation answers](journey/77b-validate-point-and-number-transformation-answers.md) | Validate point and number transformation answers. |
| [15.1.22 · Resolve typed batches and temporary transformation previews](journey/77c-resolve-typed-batches-and-temporary-transformation-previews.md) | Resolve typed batches and temporary transformation previews. |
| [15.1.23 · Resolve original CAD parts and complete selected bounds](journey/77d-resolve-original-cad-parts-and-complete-selected-bounds.md) | Resolve original CAD parts and complete selected bounds. |
| [15.1.24 · Rebuild an edited original CAD source before adoption](journey/77e-rebuild-an-edited-original-cad-source-before-adoption.md) | Rebuild an edited original CAD source before adoption. |
| [15.1.25 · Copy original rows beside their existing parents](journey/77f-copy-original-rows-beside-their-existing-parents.md) | Copy original rows beside their existing parents. |
| [15.1.26 · Carry transformation state through the existing drawing owner](journey/77g-carry-transformation-state-through-the-existing-drawing-owner.md) | Carry transformation state through the existing drawing owner. |
| [15.1.27 · Validate selected originals before beginning or committing a transform](journey/77h-validate-selected-originals-before-beginning-or-committing-a-transform.md) | Validate selected originals before beginning or committing a transform. |
| [15.1.28 · Adopt validated outcomes and preserve repeat copy transactions](journey/77i-adopt-validated-outcomes-and-preserve-repeat-copy-transactions.md) | Adopt validated outcomes and preserve repeat copy transactions. |
| [15.1.29 · Route transform input cancellation and current plane changes](journey/77j-route-transform-input-cancellation-and-current-plane-changes.md) | Route transform input cancellation and current plane changes. |
| [15.1.30 · Connect transformation lifecycle and part Move dispatch](journey/77k-connect-transformation-lifecycle-and-part-move-dispatch.md) | Connect transformation lifecycle and part Move dispatch. |
| [15.1.31 · Register Copy Scale Rotate and their current aliases](journey/77l-register-copy-scale-rotate-and-their-current-aliases.md) | Register Copy Scale Rotate and their current aliases. |
| [15.1.32 · Apply direct Scale and Rotate about the current selection center](journey/77m-apply-direct-scale-and-rotate-about-the-current-selection-center.md) | Apply direct Scale and Rotate about the current selection center. |
| [15.1.33 · Retain earlier selection checks with source part support](journey/77n-retain-earlier-selection-checks-with-source-part-support.md) | Retain earlier selection checks with source part support. |
| [15.1.34 · Exercise original copy transforms history and exact persistence](journey/77o-exercise-original-copy-transforms-history-and-exact-persistence.md) | Exercise original copy transforms history and exact persistence. |
| [15.1.35 · Describe curve gathering answers](journey/77zza-describe-curve-gathering-answers.md) | Describe ordered curve questions and typed answers. |
| [15.1.36 · Read curve gathering values](journey/77zzb-read-curve-gathering-values.md) | Read finite numbers, absolute points and distance vectors through the existing coordinate reader. |
| [15.1.37 · Connect gathering recipes](journey/77zzc-connect-gathering-recipes.md) | Connect typed input to a recipe, its questions and kernel builder. |
| [15.1.38 · Parse gathering command words](journey/77zzd-parse-gathering-command-words.md) | Read options anywhere in a command and consume other answers in order. |
| [15.1.39 · Track ordered curve picks](journey/77zze-track-ordered-curve-picks.md) | Track preselected curve rows and skip answers already typed. |
| [15.1.40 · Advance and cancel curve picks](journey/77zzf-advance-and-cancel-curve-picks.md) | Toggle a picked curve, reject duplicates across groups, and advance on a full group or Enter. |
| [15.1.41 · Prepare placed kernel curves](journey/77zzg-prepare-placed-kernel-curves.md) | Convert lines, polylines and NURBS curves to placed kernel curves. |
| [15.1.42 · Check original loops and surface controls](journey/77zzh-check-original-loops-and-surface-controls.md) | Find a loop’s unit normal and scale-aware planarity tolerance. |
| [15.1.43 · Align original sections and loft through them](journey/77zzi-align-original-sections-and-loft-through-them.md) | Align open curve directions and closed curve seams before asking the Session kernel for a loft. |
| [15.1.44 · Find extrusion axes and cap eligibility](journey/77zzj-find-extrusion-axes-and-cap-eligibility.md) | Reuse the original loop normal and planarity helpers to choose an extrusion axis. |
| [15.1.45 · Map cap curves to surface parameters](journey/77zzk-map-cap-curves-to-surface-parameters.md) | Build a planar cap patch around the original control points. |
| [15.1.46 · Build closed curved extrusion topology](journey/77zzl-build-closed-curved-extrusion-topology.md) | Build the side surface from an oriented original curve. |
| [15.1.47 · Extrude original corners and curves](journey/77zzm-extrude-original-corners-and-curves.md) | Dispatch original corner chains and curved sections through the shared extrusion helpers. |
| [15.1.48 · Connect the original extrusion recipe](journey/77zzn-connect-the-original-extrusion-recipe.md) | Complete the original recipe with curve selection, signed distance and cap choices. |
| [15.1.49 · Revolve original profiles around an axis](journey/77zzo-revolve-original-profiles-around-an-axis.md) | Turn original curves into exact kernel surfaces around a chosen axis. |
| [15.2.1 · Copy current curve collection and surfacing recipes](journey/78-copy-current-curve-collection-and-surfacing-recipes.md) | Copy current curve collection and surfacing recipes. |
| [15.2.2 · Own picked originals and world profile copies](journey/78a-own-picked-originals-and-world-profile-copies.md) | Own picked originals and world profile copies. |
| [15.2.3 · Resolve a recipe input and guard picked source freshness](journey/78b-resolve-a-recipe-input-and-guard-picked-source-freshness.md) | Resolve a recipe input and guard picked source freshness. |
| [15.2.4 · Accept curve picks and geometric answers atomically](journey/78c-accept-curve-picks-and-geometric-answers-atomically.md) | Accept curve picks and geometric answers atomically. |
| [15.2.5 · Read options and typed batches without partial results](journey/78d-read-options-and-typed-batches-without-partial-results.md) | Read options and typed batches without partial results. |
| [15.2.6 · Constrain distance input and draw lifted profile wires](journey/78e-constrain-distance-input-and-draw-lifted-profile-wires.md) | Constrain distance input and draw lifted profile wires. |
| [15.2.7 · Carry gather state through the current tool owner](journey/78f-carry-gather-state-through-the-current-tool-owner.md) | Carry gather state through the current tool owner. |
| [15.2.8 · Adopt every generated original in one history transaction](journey/78g-adopt-every-generated-original-in-one-history-transaction.md) | Adopt every generated original in one history transaction. |
| [15.2.9 · Start and advance the recipe through the existing Editor](journey/78h-start-and-advance-the-recipe-through-the-existing-editor.md) | Start and advance the recipe through the existing Editor. |
| [15.2.10 · Route typed answers and axis cursor input](journey/78i-route-typed-answers-and-axis-cursor-input.md) | Route typed answers and axis cursor input. |
| [15.2.11 · Restore selection and preserve the gather lifecycle](journey/78j-restore-selection-and-preserve-the-gather-lifecycle.md) | Restore selection and preserve the gather lifecycle. |
| [15.2.12 · Register extrusion revolution and browser curve picking](journey/78k-register-extrusion-revolution-and-browser-curve-picking.md) | Register extrusion revolution and browser curve picking. |
| [15.2.13 · Pick original profiles through the browser integer pass](journey/78l-pick-original-profiles-through-the-browser-integer-pass.md) | Pick original profiles through the browser integer pass. |
| [15.2.14 · Exercise profiles exact surfaces and one undo transaction](journey/78m-exercise-profiles-exact-surfaces-and-one-undo-transaction.md) | Exercise profiles exact surfaces and one undo transaction. |
| [15.3.1 · Retain current loft section order and degree options](journey/79-retain-current-loft-section-order-and-degree-options.md) | Retain current loft section order and degree options. |
| [15.3.2 · Frame original profiles on their rail](journey/79aa-frame-original-profiles-on-their-rail.md) | Find each profile’s station from its sampled center and the nearest rail parameter. |
| [15.3.3 · Carry and blend original profiles along the rail](journey/79ab-carry-and-blend-original-profiles-along-the-rail.md) | Sort original profile stations, match their control rows with a straight kernel loft, and include their stations in the sampled rail frames. |
| [15.3.4 · Connect original one-rail sweep inputs](journey/79a-carry-original-profiles-along-one-rail.md) | Complete the existing gather recipe with one rail and one or more profiles. |
| [15.3.5 · Align shape curves between two original rails](journey/79b-align-shape-curves-between-two-original-rails.md) | Align shape curves between two original rails. |
| [15.3.6 · Register loft sweep names options and aliases](journey/79c-register-loft-sweep-names-options-and-aliases.md) | Register loft sweep names options and aliases. |
| [15.3.7 · Exercise original sections rail groups and exact persistence](journey/79d-exercise-original-sections-rail-groups-and-exact-persistence.md) | Exercise original sections rail groups and exact persistence. |
| [15.3.8 · Sample original network curve stations](journey/79zza-sample-original-network-curve-stations.md) | Write control bounds, tolerant crossing checks and interpolation helpers. |
| [15.3.9 · Classify two original network directions](journey/79zzb-classify-two-original-network-directions.md) | Find original curve crossings and assign opposite directions through their graph. |
| [15.3.10 · Order and orient original network curves](journey/79zzc-order-and-orient-original-network-curves.md) | Sort each family along the other and orient its crossing parameters consistently. |
| [15.3.11 · Sample original network lofts and tensor](journey/79zzd-sample-original-network-lofts-and-tensor.md) | Sample both curve-family lofts and the tensor through their shared crossings. |
| [15.3.12 · Fit the original Gordon surface](journey/79zze-fit-the-original-gordon-surface.md) | Combine the two lofts minus their shared tensor. |
| [15.4.1 · Build a surface through original curve networks](journey/80-build-a-surface-through-two-directions-of-original-curves.md) | Connect the complete network helpers and retain the four-boundary Coons path. |
| [15.4.2 · Construct the patch through four ordered corners](journey/80a-construct-the-patch-through-four-ordered-corners.md) | Construct the patch through four ordered corners. |
| [15.4.3 · Guide corner questions without retaining unrelated profiles](journey/80b-guide-corner-questions-without-retaining-unrelated-profiles.md) | Guide corner questions without retaining unrelated profiles. |
| [15.4.4 · Register network patch aliases and initial corner arity](journey/80c-register-network-patch-aliases-and-initial-corner-arity.md) | Register network patch aliases and initial corner arity. |
| [15.4.5 · Exercise networks corner retries and exact original archives](journey/80d-exercise-networks-corner-retries-and-exact-original-archives.md) | Exercise networks corner retries and exact original archives. |
| [15.4.6 · Read original projection planes](journey/80za-read-original-projection-planes.md) | Reuse absolute coordinate parsing for named construction planes and three-point input. |
| [16.1.1 · Project originals in their placed frame](journey/81-project-originals-through-the-current-plane-algorithms.md) | Conjugate the world projection through the original placement transform. |
| [16.1.2 · Read original cutters in object frames](journey/81zza-read-original-cutters-in-object-frames.md) | Write the cutter records and convert supported originals to curves or planar boundaries. |
| [16.1.3 · Find original cutters near the pointer](journey/81zzb-find-original-cutters-near-the-pointer.md) | Build fence planes and unit rays in the original object frame. |
| [16.1.4 · Grow original domains and extract the tail](journey/81zzc-grow-original-domains-and-extract-the-tail.md) | Define end and reach records. |
| [16.1.5 · Find original end directions and length reach](journey/81zzd-find-original-end-directions-and-length-reach.md) | Find outward directions for supported original curve families. |
| [16.1.6 · Stop straight growth at the first boundary](journey/81zze-stop-straight-growth-at-the-first-boundary.md) | Find the nearest positive crossing of original planes, curves and transformed meshes along a straight end.. |
| [16.1.7 · Stop curved growth at an original boundary](journey/81zzf-stop-curved-growth-at-an-original-boundary.md) | Grow the original NURBS domain and inspect curve or plane crossings beyond its old end. |
| [16.1.8 · Dispatch original extension families](journey/81zzg-dispatch-original-extension-families.md) | Connect the existing reach helpers for lines, polylines and NURBS curves. |
| [16.1.9 · Edit original curve intervals](journey/81a-find-source-cutters-and-current-extension-reach.md) | Trim or extend supported original curves by a normalized physical-length interval. |
| [16.1.10 · Capture eligible original identity and placement](journey/81b-capture-eligible-original-identity-and-placement.md) | Capture eligible original identity and placement. |
| [16.1.11 · Replace an original without replacing its row](journey/81c-replace-an-original-without-replacing-its-row.md) | Replace an original without replacing its row. |
| [16.1.12 · Own the originals selected for projection](journey/81d-own-the-originals-selected-for-projection.md) | Own the originals selected for projection. |
| [16.1.13 · Validate candidate plane answers before accepting them](journey/81e-validate-candidate-plane-answers-before-accepting-them.md) | Validate candidate plane answers before accepting them. |
| [16.1.14 · Prepare projection results as one transaction](journey/81f-prepare-projection-results-as-one-transaction.md) | Prepare projection results as one transaction. |
| [16.1.15 · Own extension candidates and screen bounds](journey/81g-own-extension-candidates-and-screen-bounds.md) | Own extension candidates and screen bounds. |
| [16.1.16 · Gather boundaries and sample eligible curve owners](journey/81h-gather-boundaries-and-sample-eligible-curve-owners.md) | Gather boundaries and sample eligible curve owners. |
| [16.1.17 · Cache current boundary meshes and start the end phase](journey/81i-cache-current-boundary-meshes-and-start-the-end-phase.md) | Cache current boundary meshes and start the end phase. |
| [16.1.18 · Resolve distance and boundary reach in source coordinates](journey/81j-resolve-distance-and-boundary-reach-in-source-coordinates.md) | Resolve distance and boundary reach in source coordinates. |
| [16.1.19 · Find the nearest visible curve end in CSS pixels](journey/81k-find-the-nearest-visible-curve-end-in-css-pixels.md) | Find the nearest visible curve end in CSS pixels. |
| [16.1.20 · Preview extension and refresh only changed originals](journey/81l-preview-extension-and-refresh-only-changed-originals.md) | Preview extension and refresh only changed originals. |
| [16.1.21 · Keep projection and extension in the pending tool owner](journey/81m-keep-projection-and-extension-in-the-pending-tool-owner.md) | Keep projection and extension in the pending tool owner. |
| [16.1.22 · Connect checked projection to one editor transaction](journey/81n-connect-checked-projection-to-one-editor-transaction.md) | Connect checked projection to one editor transaction. |
| [16.1.23 · Connect boundary picks repeated end clicks and cancellation](journey/81o-connect-boundary-picks-repeated-end-clicks-and-cancellation.md) | Connect boundary picks repeated end clicks and cancellation. |
| [16.1.24 · Expose editing actions and normalized interval extension](journey/81p-expose-editing-actions-and-normalized-interval-extension.md) | Expose editing actions and normalized interval extension. |
| [16.1.25 · Route answers to the active editing owner](journey/81q-route-accepted-editing-input-and-owner-lifecycle-1.md) | Dispatch typed answers and world points to the current draft. |
| [16.1.26 · Finish a pending edit through one commit path](journey/81r-route-accepted-editing-input-and-owner-lifecycle-2.md) | Keep navigation available while an edit is pending, then finish through its owner. |
| [16.1.27 · Restore source selection when an edit is cancelled](journey/81s-route-accepted-editing-input-and-owner-lifecycle-3.md) | Discard temporary previews and restore the captured selection on cancellation. |
| [16.1.28 · Register current projection and extension commands](journey/81t-register-current-projection-and-extension-commands.md) | Register current projection and extension commands. |
| [16.1.29 · Route whole boundary picks and observe pending edits](journey/81u-route-whole-boundary-picks-and-observe-pending-edits.md) | Route whole boundary picks and observe pending edits. |
| [16.1.30 · Exercise original projection retry and repeated extension](journey/81v-exercise-original-projection-retry-and-repeated-extension.md) | Exercise original projection retry and repeated extension. |
| [16.1.31 · Validate original Split inputs and name pieces](journey/81za-validate-original-split-inputs-and-name-pieces.md) | Accept original cutter curves and a valid selected BRep face. |
| [16.2.1 · Divide original curves and joined faces](journey/82-divide-current-original-curve-families.md) | Bound the cutter count and dispatch original lines, polylines, curves, surfaces and selected BRep faces to the Session split algorithms. |
| [16.2.2 · Own the split target and toggle original cutters](journey/82a-own-the-split-target-and-toggle-original-cutters.md) | Own the split target and toggle original cutters. |
| [16.2.3 · Transform cutters into the target frame before dividing](journey/82b-transform-cutters-into-the-target-frame-before-dividing.md) | Transform cutters into the target frame before dividing. |
| [16.2.4 · Adopt sibling pieces beside the original target](journey/82c-adopt-sibling-pieces-beside-the-original-target.md) | Adopt sibling pieces beside the original target. |
| [16.2.5 · Connect pending split completion to one history change](journey/82d-connect-pending-split-completion-to-one-history-change.md) | Connect pending split completion to one history change. |
| [16.2.6 · Route cutter picks cancellation and temporary highlights](journey/82e-route-cutter-picks-cancellation-and-temporary-highlights.md) | Route cutter picks cancellation and temporary highlights. |
| [16.2.7 · Register Split and route mouse touch and confirmation](journey/82f-register-split-and-route-mouse-touch-and-confirmation.md) | Register Split and route mouse touch and confirmation. |
| [16.2.8 · Exercise curve pieces source inheritance and one-step Undo](journey/82g-exercise-curve-pieces-source-inheritance-and-one-step-undo.md) | Exercise curve pieces source inheritance and one-step Undo. |
| [16.3.1 · Split original faces without separating the solid](journey/83-split-original-faces-without-separating-the-solid.md) | Divide a selected original surface face while preserving joined topology and one-step history. |
| [16.3.2 · Visit original mesh triangles and format measures](journey/83za-visit-original-mesh-triangles-and-format-measures.md) | Write one visitor for the original mesh triangles in world coordinates. |
| [16.3.3 · Compute original mesh measures and curve length](journey/83zb-compute-original-mesh-measures-and-curve-length.md) | Reuse the triangle visitor for face area, mesh area and signed swept volume. |
| [16.3.4 · Integrate original surface area](journey/83ze-integrate-original-surface-area.md) | Reuse mesh area for meshes, meshed BReps and Elements. |
| [16.4.1 · Measure original closed-solid volume](journey/84-measure-original-curves-surfaces-and-closed-solids.md) | Reject open geometry, choose a world point near the mesh and reuse the shared swept-volume helper. |
| [16.4.2 · Validate an original box and derive wire-only display](journey/84a-validate-an-original-box-and-derive-wire-only-display.md) | Validate an original box and derive wire-only display. |
| [16.4.3 · Adopt original boxes and retain precise placed bounds](journey/84b-adopt-original-boxes-and-retain-precise-placed-bounds.md) | Adopt original boxes and retain precise placed bounds. |
| [16.4.4 · Save and reopen the original box frame](journey/84c-save-and-reopen-the-original-box-frame.md) | Save and reopen the original box frame. |
| [16.4.5 · Own temporary measurement labels without scene identities](journey/84d-own-temporary-measurement-labels-without-scene-identities.md) | Own temporary measurement labels without scene identities. |
| [16.4.6 · Report distance and world bounding lengths](journey/84e-report-distance-and-world-bounding-lengths.md) | Report distance and world bounding lengths. |
| [16.4.7 · Collect only supported finite original measurements](journey/84f-collect-only-supported-finite-original-measurements.md) | Collect only supported finite original measurements. |
| [16.4.8 · Format totals refusals and three-axis box extents](journey/84g-format-totals-refusals-and-three-axis-box-extents.md) | Format totals refusals and three-axis box extents. |
| [16.4.9 · Keep measurements outside geometry history](journey/84h-keep-measurements-outside-geometry-history.md) | Keep measurements outside geometry history. |
| [16.4.10 · Accept two distance points through the drawing tool](journey/84i-accept-two-distance-points-through-the-drawing-tool.md) | Accept two distance points through the drawing tool. |
| [16.4.11 · Register measurements and end temporary marks predictably](journey/84j-register-measurements-and-end-temporary-marks-predictably.md) | Register measurements and end temporary marks predictably. |
| [16.4.12 · Draw cached unpickable measurement strokes](journey/84k-draw-cached-unpickable-measurement-strokes.md) | Draw cached unpickable measurement strokes. |
| [16.4.13 · Merge measurement chips with cached scene text](journey/84l-merge-measurement-chips-with-cached-scene-text.md) | Merge measurement chips with cached scene text. |
| [16.4.14 · Exercise original boxes and temporary distance annotations](journey/84m-exercise-original-boxes-and-temporary-distance-annotations.md) | Exercise original boxes and temporary distance annotations. |
| [16.4.15 · Describe removable CAD parts](journey/84zza-describe-removable-cad-parts.md) | Represent curve intervals, BRep face groups and mesh sides without replacing the original CAD geometry. |
| [16.4.16 · Read trim part previews](journey/84zzb-read-trim-part-previews.md) | Expose part counts, sampled curve outlines, section edges and meshes through one Parts owner. |
| [16.4.17 · Collect curve crossings](journey/84zzc-collect-curve-crossings.md) | Collect kernel curve intersections, filter open endpoints and merge a closed seam. |
| [16.4.18 · Preserve trimmed source types](journey/84zzd-preserve-trimmed-source-types.md) | Rebuild kept curves in their original source family with their presentation. |
| [16.4.19 · Group faces by cutting planes](journey/84zze-group-faces-by-cutting-planes.md) | Extend a plane-plane intersection past surface control points. |
| [16.4.20 · Cut planar BRep faces](journey/84zzf-cut-planar-brep-faces.md) | Find crossed planar faces, split them through the kernel and collect section edges. |
| [16.4.21 · Assemble removable face groups](journey/84zzg-assemble-removable-face-groups.md) | Combine curve regions, fallback fence planes and crossed plane sections into one Parts value. |
| [16.4.22 · Mark retained BRep topology](journey/84zzh-mark-retained-brep-topology.md) | Walk retained faces through wires and edges to mark their surfaces, vertices and curve tables. |
| [16.4.23 · Rebuild retained BRep tables](journey/84zzi-rebuild-retained-brep-tables.md) | Compact marked BRep tables and rewrite every reference. |
| [16.4.24 · Split mesh faces at a plane](journey/84zzj-split-mesh-faces-at-a-plane.md) | Classify mesh vertices, reuse one cut vertex per original edge and build both clipped face rings. |
| [16.4.25 · Compact both trimmed mesh sides](journey/84zzk-compact-both-trimmed-mesh-sides.md) | Compact each clipped mesh side to only its used vertices, rewrite its faces and preserve source name and color. |
| [16.4.26 · Rebuild kept CAD parts](journey/84zzl-rebuild-kept-cad-parts.md) | Merge adjacent kept curve intervals, remove BRep face groups and return the retained mesh half. |
| [16.4.27 · Connect original trim recipes](journey/84zzm-connect-original-trim-recipes.md) | Route original curves, surfaces, BReps and meshes through the shared part builders. |
| [16.4.28 · Index selection loops by pixel row](journey/84zna-index-selection-loops-by-pixel-row.md) | Build a row index for the even-odd point test. |
| [16.4.29 · Test window membership and segment crossings](journey/84znb-test-window-membership-and-segment-crossings.md) | Complete Region with point membership and segment clipping against the rectangle. |
| [16.4.30 · Sample original CAD families for selection](journey/84znc-sample-original-cad-families-for-selection.md) | Reuse original curve corners, curved domains, mesh vertices and BRep references as bounded selection samples. |
| [16.4.31 · Collect original boundary strokes](journey/84znd-collect-original-boundary-strokes.md) | Build surface grids and closed mesh outlines, then combine BRep vertices, edges and curved faces. |
| [16.4.32 · Test crossing selection on original strokes](journey/84zne-test-crossing-selection-on-original-strokes.md) | Dispatch original geometry families to their shared stroke builders. |
| [16.4.33 · Connect window and bounds selection tests](journey/84znf-connect-window-and-bounds-selection-tests.md) | Add projection, conservative screen bounds and complete-enclosure dispatch. |
| [16.4.34 · Classify originals and isolate one mesh face](journey/84zoa-classify-originals-and-isolate-one-mesh-face.md) | Name each original geometry kind once. |
| [16.4.35 · Copy original BRep edges and pcurves](journey/84zob-copy-original-brep-edges-and-pcurves.md) | Copy an edge, its endpoint vertices and its surface pcurves into a new BRep. |
| [16.4.36 · Build one original BRep face](journey/84zoc-build-one-original-brep-face.md) | Reuse the copied-edge helper to rebuild one face surface, ordered wire references and one oriented shell. |
| [16.4.37 · Explode supported original families](journey/84zod-explode-supported-original-families.md) | Count derived pieces before allocating them. |
| [16.5.1 · Write original text placement and curve heads](journey/85-copy-current-original-command-recipes.md) | Build world-plane labels with readable axes and the original rounded scale steps. |
| [16.5.2 · Validate original Plane display and precise bounds](journey/85a-validate-original-plane-display-and-precise-bounds.md) | Validate original Plane display and precise bounds. |
| [16.5.3 · Retain original Plane and Text edit eligibility](journey/85b-retain-original-plane-and-text-edit-eligibility.md) | Retain original Plane and Text edit eligibility. |
| [16.5.4 · Find eligible objects by name or world size](journey/85c-find-eligible-objects-by-name-or-world-size.md) | Find eligible objects by name or world size. |
| [16.5.5 · Plan sibling pieces from original owners](journey/85d-plan-sibling-pieces-from-original-owners.md) | Plan sibling pieces from original owners. |
| [16.5.6 · Own Text height and words before placement](journey/85e-own-text-height-and-words-before-placement.md) | Own Text height and words before placement. |
| [16.5.7 · Own unpickable cached guide data](journey/85f-own-unpickable-cached-guide-data.md) | Own unpickable cached guide data. |
| [16.5.8 · Shade explicit dashed guides](journey/85g-shade-explicit-dashed-guides.md) | Shade explicit dashed guides. |
| [16.5.9 · Cache guide buffers and actual allocation](journey/85h-cache-guide-buffers-and-actual-allocation.md) | Cache guide buffers and actual allocation. |
| [16.5.10 · Draw cached guides through the overlay owner](journey/85i-draw-cached-guides-through-the-overlay-owner.md) | Draw cached guides through the overlay owner. |
| [16.5.11 · Own Trim targets cutters and part state](journey/85j-own-trim-targets-cutters-and-part-state.md) | Own Trim targets cutters and part state. |
| [16.5.12 · Prepare local cuts from world cutters](journey/85k-prepare-local-cuts-from-world-cutters.md) | Prepare local cuts from world cutters. |
| [16.5.13 · Hit remaining trim parts in canvas pixels](journey/85l-hit-remaining-trim-parts-in-canvas-pixels.md) | Hit remaining trim parts in canvas pixels. |
| [16.5.14 · Preview cuts without changing originals](journey/85m-preview-cuts-without-changing-originals.md) | Preview cuts without changing originals. |
| [16.5.15 · Show remaining cuts and removed-part guides](journey/85n-show-remaining-cuts-and-removed-part-guides.md) | Show remaining cuts and removed-part guides. |
| [16.5.16 · Sample bounded physical-pixel lasso loops](journey/85o-sample-bounded-physical-pixel-lasso-loops.md) | Sample bounded physical-pixel lasso loops. |
| [16.5.17 · Own document-scoped groups and links](journey/85p-own-document-scoped-groups-and-links.md) | Own document-scoped groups and links. |
| [16.5.18 · Resolve current graph and group identities](journey/85q-resolve-current-graph-and-group-identities.md) | Resolve current graph and group identities. |
| [16.5.19 · Connect two original owners and save their links](journey/85r-connect-two-original-owners-and-save-their-links.md) | Connect two original owners and save their links. |
| [16.5.20 · Remap appended links and remove deleted owners](journey/85s-remap-appended-links-and-remove-deleted-owners.md) | Remap appended links and remove deleted owners. |
| [16.5.21 · Plan world-preserving original groups](journey/85t-plan-world-preserving-original-groups.md) | Plan world-preserving original groups. |
| [16.5.22 · Adopt group branches with one reversible edit](journey/85u-adopt-group-branches-with-one-reversible-edit.md) | Adopt group branches with one reversible edit. |
| [16.5.23 · Append namespaced graphs and preserve original row identities](journey/85v-append-namespaced-graphs-and-preserve-original-row-identities.md) | Append namespaced graphs and preserve original row identities. |
| [16.5.24 · Replace original families without changing world placement](journey/85w-replace-original-families-without-changing-world-placement.md) | Replace original families without changing world placement. |
| [16.5.25 · Save group namespaces and original planes](journey/85x-save-group-namespaces-and-original-planes.md) | Save group namespaces and original planes. |
| [16.5.26 · Keep named views and grid on the drawing plane](journey/85y-keep-named-views-and-grid-on-the-drawing-plane.md) | Keep named views and grid on the drawing plane. |
| [16.5.27 · Connect cached guide and placed grid lanes](journey/85z-connect-cached-guide-and-placed-grid-lanes.md) | Connect cached guide and placed grid lanes. |
| [16.5.28 · Expose graph links and the lasso overlay](journey/85aa-expose-graph-links-and-the-lasso-overlay.md) | Expose graph links and the lasso overlay. |
| [16.5.29 · Own current pending command state](journey/85ab-own-current-pending-command-state.md) | Own current pending command state. |
| [16.5.30 · Retain Text answers inside the existing point draft](journey/85ac-retain-text-answers-inside-the-existing-point-draft.md) | Retain Text answers inside the existing point draft. |
| [16.5.31 · Complete original Text and headed Arrow tools](journey/85ad-complete-original-text-and-headed-arrow-tools.md) | Complete original Text and headed Arrow tools. |
| [16.5.32 · Present cached extension guides and error endpoints](journey/85ae-present-cached-extension-guides-and-error-endpoints.md) | Present cached extension guides and error endpoints. |
| [16.5.33 · Commit prepared trims once](journey/85af-commit-prepared-trims-once.md) | Commit prepared trims once. |
| [16.5.34 · Advance Trim through targets cutters and parts](journey/85ag-advance-trim-through-targets-cutters-and-parts.md) | Advance Trim through targets cutters and parts. |
| [16.5.35 · Adopt original pieces and arrowhead changes atomically](journey/85ah-adopt-original-pieces-and-arrowhead-changes-atomically.md) | Adopt original pieces and arrowhead changes atomically. |
| [16.5.36 · Dispatch current original commands through checked owners](journey/85ai-dispatch-current-original-commands-through-checked-owners.md) | Dispatch current original commands through checked owners. |
| [16.5.37 · Preserve previews selection and accepted tool answers](journey/85aj-preserve-previews-selection-and-accepted-tool-answers.md) | Preserve previews selection and accepted tool answers. |
| [16.5.38 · Keep annotations during touch navigation and capture lasso input](journey/85ak-keep-annotations-during-touch-navigation-and-capture-lasso-input.md) | Keep annotations during touch navigation and capture lasso input. |
| [16.5.39 · Preserve pending originals during view changes](journey/85al-preserve-pending-originals-during-view-changes.md) | Preserve pending originals during view changes. |
| [16.5.40 · Route named views and whole trim picks](journey/85am-route-named-views-and-whole-trim-picks.md) | Route named views and whole trim picks. |
| [16.5.41 · Capture source-backed command arguments](journey/85an-capture-source-backed-command-arguments.md) | Capture source-backed command arguments. |
| [16.5.42 · Restore typed requests and their selected owners](journey/85ao-restore-typed-requests-and-their-selected-owners.md) | Restore typed requests and their selected owners. |
| [16.5.43 · Witness identities and placement during source loading](journey/85ap-witness-identities-and-placement-during-source-loading.md) | Witness identities and placement during source loading. |
| [16.5.44 · Deduplicate original fetches and replay safely](journey/85aq-deduplicate-original-fetches-and-replay-safely.md) | Deduplicate original fetches and replay safely. |
| [16.5.45 · Route source-backed edits through guarded replay](journey/85ar-route-source-backed-edits-through-guarded-replay.md) | Route source-backed edits through guarded replay. |
| [16.5.46 · Register current names views and command aliases](journey/85as-register-current-names-views-and-command-aliases.md) | Register current names views and command aliases. |
| [16.5.47 · Validate current command words and options](journey/85at-validate-current-command-words-and-options.md) | Validate current command words and options. |
| [16.5.48 · Capture lasso pointer ownership in the browser](journey/85au-capture-lasso-pointer-ownership-in-the-browser.md) | Capture lasso pointer ownership in the browser. |
| [16.5.49 · Route trim and lasso pointer input](journey/85av-route-trim-and-lasso-pointer-input.md) | Route trim and lasso pointer input. |
| [16.5.50 · Refresh command panels source loading and cached previews](journey/85aw-refresh-command-panels-source-loading-and-cached-previews.md) | Refresh command panels source loading and cached previews. |
| [16.5.51 · Observe current original command state](journey/85ax-observe-current-original-command-state.md) | Observe current original command state. |
| [16.5.52 · Exercise current original commands together](journey/85ay-exercise-current-original-commands-together.md) | Exercise current original commands together. |
| [17.1.1 · Bound the physical tile grid and reference storage](journey/86-copy-finite-tile-allocation-rules.md) | Bound the physical tile grid and reference storage. |
| [17.1.2 · Grow projection and reference pools from demand](journey/86ma-grow-projection-and-reference-pools-from-demand.md) | Grow projection and reference pools from demand. |
| [17.1.3 · Define one shared projected-triangle coverage contract](journey/86a-copy-maintained-projection-and-scan-recipes.md) | Store stable screen-space triangle records and reuse one coverage and tile-span owner. |
| [17.1.4 · Declare finite projection buffers and original vertex access](journey/86b-copy-the-project-recipe.md) | Bind existing original mesh buffers and transform each indexed corner into the projection convention. |
| [17.1.5 · Clip original triangles before dividing by depth](journey/86ba-clip-original-triangles-before-dividing-by-depth.md) | Clip against the homogeneous near plane, retain up to four corners and measure signed screen area. |
| [17.1.6 · Write bounded projected triangle records on the GPU](journey/86bb-write-bounded-projected-triangle-records-on-the-gpu.md) | Dispatch one invocation per original triangle and write screen bounds, inward edge equations and a conditioned depth plane. |
| [17.1.7 · Assemble and validate the shared finite projection shader](journey/86c-project-existing-original-mesh-buffers.md) | Reuse shared WGSL declarations and validate the completed compute entry point. |
| [17.1.8 · Declare atomic tile records and flat triangle inputs](journey/86d-copy-the-triangle-tiles-recipe.md) | Bind projected triangle records and a shared bounded reference pool with atomic count and cursor words. |
| [17.1.9 · Count conservative triangle coverage per tile](journey/86da-count-conservative-triangle-coverage-per-tile.md) | Rasterize one tile-grid quad per triangle and reject tiles outside any inward edge before counting. |
| [17.1.10 · Fill bounded original triangle reference lists](journey/86db-fill-bounded-original-triangle-reference-lists.md) | Atomically reserve a tile slot, store the original projected triangle address and depth, and flag overflow instead of writing past the pool. |
| [17.1.11 · Assemble and validate the bounded tiles shader](journey/86e-count-and-fill-original-triangle-references.md) | Reuse shared WGSL source and validate the complete tiles pass interfaces. |
| [17.1.12 · Scan bounded tile counts in workgroup memory](journey/86f-copy-the-scan-triangle-tiles-recipe.md) | Build a saturating exclusive prefix sum with uniform barriers and shared scratch storage. |
| [17.1.13 · Scan tile blocks and their block totals](journey/86fa-scan-tile-blocks-and-their-block-totals.md) | Run the same prefix owner first over tile counts and then over their block totals. |
| [17.1.14 · Finalize bounded tile list offsets and demand](journey/86fb-finalize-bounded-tile-list-offsets-and-demand.md) | Combine block and local offsets, reset fill cursors, and record actual pool demand in the shared header. |
| [17.1.15 · Assemble and validate the bounded scan shader](journey/86fc-assemble-and-validate-the-bounded-scan-shader.md) | Reuse shared WGSL source and validate the complete scan pass interfaces. |
| [17.1.16 · Wake only after a tile report arrives](journey/86g-wake-only-after-a-tile-report-arrives.md) | Wake only after a tile report arrives. |
| [17.1.17 · Read actual pool demand asynchronously](journey/86h-read-actual-pool-demand-asynchronously.md) | Read GPU tile-pool demand asynchronously, with one map in flight and one reusable readback buffer. |
| [17.1.18 · Describe explicit finite GPU layouts](journey/86i-describe-explicit-finite-gpu-layouts.md) | Compile the finite pipeline resource types before their helper functions. |
| [17.1.19 · Build compute and count pipeline helpers](journey/86j-build-compute-and-count-pipeline-helpers.md) | Build compute and count pipeline helpers. |
| [17.1.20 · Connect projection count scan and fill pipelines](journey/86k-connect-projection-count-scan-and-fill-pipelines.md) | Connect projection count scan and fill pipelines. |
| [17.1.21 · Own bounded finite tile resources](journey/86l-own-bounded-finite-tile-resources.md) | Compile bounded tile resource types before allocation and population. |
| [17.1.22 · Allocate empty tile placeholders](journey/86m-allocate-empty-tile-placeholders.md) | Allocate empty tile placeholders. |
| [17.1.23 · Witness original buffers and world placement](journey/86n-witness-original-buffers-and-world-placement.md) | Witness original buffers and world placement. |
| [17.1.24 · Release obsolete tile lists and bindings](journey/86o-release-obsolete-tile-lists-and-bindings.md) | Release obsolete tile lists and bindings. |
| [17.1.25 · Grow lists only from actual demand](journey/86p-grow-lists-only-from-actual-demand.md) | Grow lists only from actual demand. |
| [17.1.26 · Rasterize bounded tile counts and references](journey/86q-rasterize-bounded-tile-counts-and-references.md) | Rasterize bounded tile counts and references. |
| [17.1.27 · Cache projection and finish current reports](journey/86r-cache-projection-and-finish-current-reports.md) | Cache projection and finish current reports. |
| [17.1.28 · Test visibility at the original center line](journey/86s-test-visibility-at-the-original-center-line.md) | Test visibility at the original center line. |
| [17.1.29 · Reuse original stroke coverage](journey/86t-reuse-original-stroke-coverage.md) | Reuse original stroke coverage. |
| [17.1.30 · Carry stroke axis and depth](journey/86u-carry-stroke-axis-and-depth.md) | Carry stroke axis and depth. |
| [17.1.31 · Reuse original stroke integer coverage](journey/86v-reuse-original-stroke-integer-coverage.md) | Reuse original stroke integer coverage. |
| [17.1.32 · Keep stroke crop ownership aligned](journey/86w-keep-stroke-crop-ownership-aligned.md) | Keep stroke crop ownership aligned. |
| [17.1.33 · Reuse original chain coverage](journey/86x-reuse-original-chain-coverage.md) | Reuse original chain coverage. |
| [17.1.34 · Carry chain axis and depth](journey/86y-carry-chain-axis-and-depth.md) | Carry chain axis and depth. |
| [17.1.35 · Reuse original chain integer coverage](journey/86z-reuse-original-chain-integer-coverage.md) | Reuse original chain integer coverage. |
| [17.1.36 · Keep chain crop ownership aligned](journey/86aa-keep-chain-crop-ownership-aligned.md) | Keep chain crop ownership aligned. |
| [17.1.37 · Reuse original marker coverage](journey/86ab-reuse-original-marker-coverage.md) | Reuse original marker coverage. |
| [17.1.38 · Carry marker axis and depth](journey/86ac-carry-marker-axis-and-depth.md) | Carry marker axis and depth. |
| [17.1.39 · Reuse original marker integer coverage](journey/86ad-reuse-original-marker-integer-coverage.md) | Reuse original marker integer coverage. |
| [17.1.40 · Keep marker crop ownership aligned](journey/86ae-keep-marker-crop-ownership-aligned.md) | Keep marker crop ownership aligned. |
| [17.1.41 · Assemble one shared finite shader predicate](journey/86af-assemble-one-shared-finite-shader-predicate.md) | Assemble one shared finite shader predicate. |
| [17.1.42 · Own finite color and integer pipelines](journey/86ag-own-finite-color-and-integer-pipelines.md) | Compile the finite pipeline and cached-binding types before construction. |
| [17.1.43 · Build finite shader pipelines with original layouts](journey/86ah-build-finite-shader-pipelines-with-original-layouts.md) | Build finite shader pipelines with original layouts. |
| [17.1.44 · Bind finite coverage for original families](journey/86ai-bind-finite-coverage-for-original-families.md) | Bind finite coverage for original families. |
| [17.1.45 · Reuse bindings and draw original owners](journey/86aj-reuse-bindings-and-draw-original-owners.md) | Reuse bindings and draw original owners. |
| [17.1.46 · Expose original GPU models to projection](journey/86ak-expose-original-gpu-models-to-projection.md) | Expose original GPU models to projection. |
| [17.1.47 · Connect finite visibility to all original ink lanes](journey/86al-connect-finite-visibility-to-all-original-ink-lanes.md) | Connect finite visibility to all original ink lanes. |
| [17.1.48 · Separate integer depth writers and sampled ink](journey/86am-separate-integer-depth-writers-and-sampled-ink.md) | Separate integer depth writers and sampled ink. |
| [17.1.49 · Own tile state before color and first picks](journey/86an-own-tile-state-before-color-and-first-picks.md) | Own tile state before color and first picks. |
| [17.1.50 · Split opaque finite ink and cloud composition](journey/86ao-split-opaque-finite-ink-and-cloud-composition.md) | Split opaque finite ink and cloud composition. |
| [17.1.51 · Observe actual derived tile resources](journey/86ap-observe-actual-derived-tile-resources.md) | Observe actual derived tile resources. |
| [17.1.52 · Exercise finite color and original picking](journey/86aq-exercise-finite-color-and-original-picking.md) | Exercise finite color and original picking. |
| [17.2.1 · Describe original clipping half-spaces](journey/87-copy-current-original-plane-modes.md) | Describe original clipping half-spaces. |
| [17.2.2 · Describe clipping construction choices once](journey/87sa-describe-clipping-construction-choices-once.md) | Describe clipping construction choices once. |
| [17.2.3 · Construct and reverse original clipping planes](journey/87sb-construct-and-reverse-original-clipping-planes.md) | Construct and reverse original clipping planes. |
| [17.2.4 · Derive placed world half-spaces from original planes](journey/87sc-derive-placed-world-half-spaces-from-original-planes.md) | Derive placed world half-spaces from original planes. |
| [17.2.5 · Gather six visible original world planes](journey/87a-gather-six-visible-original-world-planes.md) | Gather six visible original world planes. |
| [17.2.6 · Pack the clipping uniform with explicit byte offsets](journey/87b-copy-the-maintained-clipping-uniform-layout.md) | Match six-plane CPU storage to the later WGSL layout without losing original f64 data. |
| [17.2.7 · Condition the camera inverse for clipping](journey/87c-copy-conditioned-screen-and-hatch-projections.md) | Scale matrix rows before inversion and undo that scaling on the inverse. |
| [17.2.8 · Project a stable hatch coordinate onto a plane](journey/87ca-project-a-stable-hatch-coordinate-onto-a-plane.md) | Build homogeneous hatch numerator and denominator from the projected plane. |
| [17.2.9 · Fill one bounded clipping uniform from original planes](journey/87cb-fill-one-bounded-clipping-uniform-from-original-planes.md) | Populate world and screen half-spaces and reuse the hatch projection helper. |
| [17.2.10 · Match the clipping uniform and sample conventions](journey/87cc-match-the-clipping-uniform-and-sample-conventions.md) | Match the Rust layout and define shared specialization and sample constants. |
| [17.2.11 · Apply shared original half-space tests](journey/87cd-apply-shared-original-half-space-tests.md) | Reject points or segments on the cut side while preserving plane display rows. |
| [17.2.12 · Compute clipping coverage in framebuffer sample space](journey/87ce-compute-clipping-coverage-in-framebuffer-sample-space.md) | Convert framebuffer pixels to NDC and form per-sample half-space masks. |
| [17.2.13 · Address section depth and original cap records](journey/87cf-address-section-depth-and-original-cap-records.md) | Solve section depth and preserve the reserved cap and pick address conventions. |
| [17.2.14 · Parse strict clipping modes and presentation switches](journey/87d-parse-strict-clipping-modes-and-presentation-switches.md) | Parse strict clipping modes and presentation switches. |
| [17.2.15 · Preserve mode prefixes in generic point drafts](journey/87e-preserve-mode-prefixes-in-generic-point-drafts.md) | Preserve mode prefixes in generic point drafts. |
| [17.2.16 · Build original clipping planes through the shared tool](journey/87f-build-original-clipping-planes-through-the-shared-tool.md) | Build original clipping planes through the shared tool. |
| [17.2.17 · Own clipping presentation and one history transaction](journey/87g-own-clipping-presentation-and-one-history-transaction.md) | Own clipping presentation and one history transaction. |
| [17.2.18 · Route Clipping Plane and preserve pending points](journey/87h-route-clipping-plane-and-preserve-pending-points.md) | Route Clipping Plane and preserve pending points. |
| [17.2.19 · Own one shared clipping GPU uniform](journey/87i-own-one-shared-clipping-gpu-uniform.md) | Own one shared clipping GPU uniform. |
| [17.2.20 · Retain clipping state beside the renderer](journey/87j-retain-clipping-state-beside-the-renderer.md) | Retain clipping state beside the renderer. |
| [17.2.21 · Prepare shared clipping shader declarations](journey/87k-prepare-shared-clipping-shader-declarations.md) | Prepare shared clipping shader declarations. |
| [17.2.22 · Connect mesh and instance color half-spaces](journey/87l-connect-mesh-and-instance-color-half-spaces.md) | Connect mesh and instance color half-spaces. |
| [17.2.23 · Prepare explicit original mesh pick clipping](journey/87m-prepare-explicit-original-mesh-pick-clipping.md) | Prepare explicit original mesh pick clipping. |
| [17.2.24 · Bind shared clipping before the first integer request](journey/87n-bind-shared-clipping-before-the-first-integer-request.md) | Bind shared clipping before the first integer request. |
| [17.2.25 · Preserve sparse source face IDs across the cut](journey/87o-preserve-sparse-source-face-ids-across-the-cut.md) | Preserve sparse source face IDs across the cut. |
| [17.2.26 · Invalidate selected masks on clipping changes](journey/87p-invalidate-selected-masks-on-clipping-changes.md) | Invalidate selected masks on clipping changes. |
| [17.2.27 · Share clipping at finite original center lines](journey/87q-share-clipping-at-finite-original-center-lines.md) | Share clipping at finite original center lines. |
| [17.2.28 · Cache explicit original-plane ink flags](journey/87r-cache-explicit-original-plane-ink-flags.md) | Cache explicit original-plane ink flags. |
| [17.2.29 · Bind original flags without changing instance strides](journey/87s-bind-original-flags-without-changing-instance-strides.md) | Bind original flags without changing instance strides. |
| [17.2.30 · Keep stroke color and pick exemptions aligned](journey/87t-keep-stroke-color-and-pick-exemptions-aligned.md) | Keep stroke color and pick exemptions aligned. |
| [17.2.31 · Keep chain color and pick exemptions aligned](journey/87u-keep-chain-color-and-pick-exemptions-aligned.md) | Keep chain color and pick exemptions aligned. |
| [17.2.32 · Keep marker color and pick exemptions aligned](journey/87v-keep-marker-color-and-pick-exemptions-aligned.md) | Keep marker color and pick exemptions aligned. |
| [17.2.33 · Resolve locked and selected original plane boundaries](journey/87w-resolve-locked-and-selected-original-plane-boundaries.md) | Resolve locked and selected original plane boundaries. |
| [17.2.34 · Clip cached original point cloud discs](journey/87x-clip-cached-original-point-cloud-discs.md) | Clip cached original point cloud discs. |
| [17.2.35 · Clip original sheet pens and integer entities](journey/87y-clip-original-sheet-pens-and-integer-entities.md) | Clip original sheet pens and integer entities. |
| [17.2.36 · Clip text anchors with all six original planes](journey/87z-clip-text-anchors-with-all-six-original-planes.md) | Clip text anchors with all six original planes. |
| [17.2.37 · Observe actual clipping resources and errors](journey/87aa-observe-actual-clipping-resources-and-errors.md) | Observe actual clipping resources and errors. |
| [17.2.38 · Exercise original clipping color and picking](journey/87ab-exercise-original-clipping-color-and-picking.md) | Exercise original clipping color and picking. |
| [17.3.1 · Classify original closedness and winding](journey/88-copy-original-closedness-and-winding.md) | Pair each original mesh edge with the opposite face direction to detect a closed shell. |
| [17.3.2 · Cache verified original solids weakly](journey/88a-cache-verified-original-solids-weakly.md) | Cache verified original solids weakly. |
| [17.3.3 · Bound conservative plane crossings](journey/88b-copy-conservative-plane-crossing-bounds.md) | Project the original box half-extents onto the clipping-plane normal to bound its reach. |
| [17.3.4 · Cache local bounds and apply placement once](journey/88c-cache-local-bounds-and-apply-placement-once.md) | Cache local bounds and apply placement once. |
| [17.3.5 · Carry closedness and dense cap addresses](journey/88d-carry-closedness-and-dense-cap-addresses.md) | Carry closedness and dense cap addresses. |
| [17.3.6 · Count original closed-solid crossings](journey/88e-count-original-closed-solid-crossings.md) | Count original closed-solid crossings. |
| [17.3.7 · Count exact original cap pick candidates](journey/88f-count-exact-original-cap-pick-candidates.md) | Count exact original cap pick candidates. |
| [17.3.8 · Filter section bands over a pixel footprint](journey/88g-copy-section-coverage-and-hatch-filtering.md) | Write a fullscreen section vertex and integrate a line band over the pixel square. |
| [17.3.9 · Fade hatch detail and detect section boundaries](journey/88ga-fade-hatch-detail-and-detect-section-boundaries.md) | Use power-of-two hatch spacing and the existing solid crossing samples. |
| [17.3.10 · Shade sections with shared original depth and sample ownership](journey/88gb-shade-sections-with-shared-original-depth-and-sample-ownership.md) | Intersect cap samples with other planes and return color, primitive, depth and coverage together. |
| [17.3.11 · Adapt color sections to forward depth](journey/88h-adapt-color-sections-to-forward-depth.md) | Adapt color sections to forward depth. |
| [17.3.12 · Return original integer section owners](journey/88i-return-original-integer-section-owners.md) | Return original integer section owners. |
| [17.3.13 · Own lazy section images](journey/88j-own-lazy-section-images.md) | Own lazy section images. |
| [17.3.14 · Build compatible section render pipelines](journey/88k-build-compatible-section-render-pipelines.md) | Build compatible section render pipelines. |
| [17.3.15 · Add signed counts and grey hatch color](journey/88l-add-signed-counts-and-grey-hatch-color.md) | Add signed counts and grey hatch color. |
| [17.3.16 · Build bounded integer section pipelines](journey/88m-build-bounded-integer-section-pipelines.md) | Build bounded integer section pipelines. |
| [17.3.17 · Own bounded cap pick storage](journey/88n-own-bounded-cap-pick-storage.md) | Own bounded cap pick storage. |
| [17.3.18 · Prepare exact cap crop and owners](journey/88o-prepare-exact-cap-crop-and-owners.md) | Prepare exact cap crop and owners. |
| [17.3.19 · Draw cap count nearest owner and integer ID](journey/88p-draw-cap-count-nearest-owner-and-integer-id.md) | Draw cap count nearest owner and integer ID. |
| [17.3.20 · Own section caches and placeholders](journey/88q-own-section-caches-and-placeholders.md) | Own section caches and placeholders. |
| [17.3.21 · Gather crossing original owners](journey/88r-gather-crossing-original-owners.md) | Gather crossing original owners. |
| [17.3.22 · Prepare active section images](journey/88s-prepare-active-section-images.md) | Prepare active section images. |
| [17.3.23 · Draw the closed union and section depth](journey/88t-draw-the-closed-union-and-section-depth.md) | Draw the closed union and section depth. |
| [17.3.24 · Route exact original cap picks and allocation](journey/88u-route-exact-original-cap-picks-and-allocation.md) | Route exact original cap picks and allocation. |
| [17.3.25 · Connect sections after opaque originals](journey/88v-connect-sections-after-opaque-originals.md) | Connect sections after opaque originals. |
| [17.3.26 · Expose section data to finite ink tables](journey/88x-expose-section-data-to-finite-ink-tables.md) | Expose section data to finite ink tables. |
| [17.3.27 · Cache finite section bindings](journey/88y-cache-finite-section-bindings.md) | Cache finite section bindings. |
| [17.3.28 · Retain ink exactly on sloping sections](journey/88z-retain-ink-exactly-on-sloping-sections.md) | Retain ink exactly on sloping sections. |
| [17.3.29 · Hide original ink behind visible caps](journey/88aa-hide-original-ink-behind-visible-caps.md) | Hide original ink behind visible caps. |
| [17.3.30 · Pick sections before any color frame](journey/88ab-pick-sections-before-any-color-frame.md) | Pick sections before any color frame. |
| [17.3.31 · Observe actual section owners and errors](journey/88ac-observe-actual-section-owners-and-errors.md) | Observe actual section owners and errors. |
| [17.3.32 · Exercise original sections bores and ink](journey/88ad-exercise-original-sections-bores-and-ink.md) | Exercise original sections bores and ink. |
| [18.1.1 · Reduce depth and radius at a bounded display resolution](journey/89-copy-density-scaled-depth-reduction.md) | Write the density policy and nearest-positive depth/radius reduction. |
| [18.1.2 · Prepare linear original depth and radius addresses](journey/89a-prepare-linear-original-depth-and-radius-addresses.md) | Prepare linear original depth and radius addresses. |
| [18.1.3 · Own six real depth and radius levels](journey/89b-own-six-real-depth-and-radius-levels.md) | Own six real depth and radius levels. |
| [18.1.4 · Build prepare and nearest-depth pipelines](journey/89c-build-prepare-and-nearest-depth-pipelines.md) | Build prepare and nearest-depth pipelines. |
| [18.1.5 · Prepare cached pyramid inputs and images](journey/89d-prepare-cached-pyramid-inputs-and-images.md) | Prepare cached pyramid inputs and images. |
| [18.1.6 · Encode six levels only when inputs change](journey/89e-encode-six-levels-only-when-inputs-change.md) | Encode six levels only when inputs change. |
| [18.1.7 · Expose actual pyramid allocation and levels](journey/89f-expose-actual-pyramid-allocation-and-levels.md) | Expose actual pyramid allocation and levels. |
| [18.1.8 · Route original depth into the pyramid owner](journey/89g-route-original-depth-into-the-pyramid-owner.md) | Route original depth into the pyramid owner. |
| [18.1.9 · Exercise actual depth levels and cache release](journey/89h-exercise-actual-depth-levels-and-cache-release.md) | Exercise actual depth levels and cache release. |
| [18.2.1 · Derive original contact radius](journey/90-copy-maintained-original-contact-radius.md) | Derive the existing contact radius from the original scene bounds. |
| [18.2.2 · Write current viewing rays](journey/90a-copy-current-viewing-rays.md) | Unproject pixel neighbors into aligned world-space ray rows. |
| [18.2.3 · Adapt the earlier forward camera](journey/90b-adapt-the-earlier-forward-camera.md) | Adapt the earlier forward camera. |
| [18.2.4 · Gather weak original ambient properties](journey/90c-gather-weak-original-ambient-properties.md) | Gather weak original ambient properties. |
| [18.2.5 · Declare the ambient pass resource contracts](journey/90d-copy-current-horizon-and-bilateral-recipes.md) | Keep ray, depth, normal, radius and history inputs in explicit bind groups. |
| [18.2.6 · Reconstruct original surfaces from depth](journey/90da-reconstruct-original-surfaces-from-depth.md) | Unproject framebuffer depth and recover ray positions without changing original CAD geometry. |
| [18.2.7 · Encode normals and read original contact metadata](journey/90db-encode-normals-and-read-original-contact-metadata.md) | Read original radii, section normals and geometric fallback normals through the existing property textures. |
| [18.2.8 · Prepare a compact depth radius and normal target](journey/90dc-prepare-a-compact-depth-radius-and-normal-target.md) | Write ray distance, contact radius and octahedral normal into separate attachments. |
| [18.2.9 · Measure ground contact along bounded horizons](journey/90dd-measure-ground-contact-along-bounded-horizons.md) | Sample bounded directions and steps using the retained mip levels and physical contact radius. |
| [18.2.10 · Integrate cosine weighted surface visibility](journey/90de-integrate-cosine-weighted-surface-visibility.md) | Accumulate the occluded part of each projected normal slice, with bounded horizon sampling. |
| [18.2.11 · Filter contact without crossing unrelated surfaces](journey/90df-filter-contact-without-crossing-unrelated-surfaces.md) | Weight neighboring occlusion by surface separation, normal and contact radius. |
| [18.2.12 · Reject invalid ambient history before blending](journey/90dg-reject-invalid-ambient-history-before-blending.md) | Reproject into the prior frame, validate signed depth and clamp reused visibility to the current neighborhood. |
| [18.2.13 · Reconstruct full resolution contact from low resolution samples](journey/90dh-reconstruct-full-resolution-contact-from-low-resolution-samples.md) | Use surface-aware weights to combine the four nearby low-resolution values. |
| [18.2.14 · Collect bounded sample corrections with GPU atomics](journey/90di-collect-bounded-sample-corrections-with-gpu-atomics.md) | Pack per-sample corrections and claim each affected tile once for indirect edge draws. |
| [18.2.15 · Composite contact as black alpha over the original scene](journey/90dj-composite-contact-as-black-alpha-over-the-original-scene.md) | Load the current contact amount and use per-sample masks for corrected edge draws. |
| [18.2.16 · Validate the assembled original ambient shader contracts](journey/90e-bind-real-original-normals-and-radii.md) | Read optional shader-interface validation before runtime pipeline creation. |
| [18.2.17 · Write original surface contact properties](journey/90f-write-original-surface-contact-properties.md) | Write original surface contact properties. |
| [18.2.18 · Build the depth-equal original property pass](journey/90g-build-the-depth-equal-original-property-pass.md) | Build the depth-equal original property pass. |
| [18.2.19 · Cache original radius buffers and draw properties](journey/90h-cache-original-radius-buffers-and-draw-properties.md) | Cache original radius buffers and draw properties. |
| [18.2.20 · Own actual ambient image allocation](journey/90i-own-actual-ambient-image-allocation.md) | Own actual ambient image allocation. |
| [18.2.21 · Prepare reduced and full ambient targets](journey/90j-prepare-reduced-and-full-ambient-targets.md) | Prepare reduced and full ambient targets. |
| [18.2.22 · Build explicit ambient fullscreen pipelines](journey/90k-build-explicit-ambient-fullscreen-pipelines.md) | Build explicit ambient fullscreen pipelines. |
| [18.2.23 · Match current ambient sampling layouts](journey/90l-match-current-ambient-sampling-layouts.md) | Match current ambient sampling layouts. |
| [18.2.24 · Cache reduced levels and filtered samples](journey/90m-cache-reduced-levels-and-filtered-samples.md) | Cache reduced levels and filtered samples. |
| [18.2.25 · Bind exact current ambient source images](journey/90n-bind-exact-current-ambient-source-images.md) | Bind exact current ambient source images. |
| [18.2.26 · Record explicit ambient passes](journey/90o-record-explicit-ambient-passes.md) | Record explicit ambient passes. |
| [18.2.27 · Own cached ambient pipelines and release images](journey/90p-own-cached-ambient-pipelines-and-release-images.md) | Own cached ambient pipelines and release images. |
| [18.2.28 · Prepare changed ambient framebuffer inputs](journey/90q-prepare-changed-ambient-framebuffer-inputs.md) | Prepare changed ambient framebuffer inputs. |
| [18.2.29 · Render a complete ambient result in one frame](journey/90r-render-a-complete-ambient-result-in-one-frame.md) | Render a complete ambient result in one frame. |
| [18.2.30 · Count actual ambient resources and work](journey/90s-count-actual-ambient-resources-and-work.md) | Count actual ambient resources and work. |
| [18.2.31 · Route original scene properties into ambient shading](journey/90t-route-original-scene-properties-into-ambient-shading.md) | Route original scene properties into ambient shading. |
| [18.2.32 · Enable Arctic as a presentation change](journey/90u-enable-arctic-as-a-presentation-change.md) | Enable Arctic as a presentation change. |
| [18.2.33 · Exercise actual contacts cache and source ownership](journey/90v-exercise-actual-contacts-cache-and-source-ownership.md) | Exercise actual contacts cache and source ownership. |
| [18.3.1 · Include visible section coverage](journey/91-include-visible-section-coverage.md) | Include visible section coverage. |
| [18.3.2 · Cache visible section bindings](journey/91a-cache-visible-section-bindings.md) | Cache visible section bindings. |
| [18.3.3 · Combine outline alpha once](journey/91b-combine-outline-alpha-once.md) | Combine outline alpha once. |
| [18.3.4 · Cache one black outline composite](journey/91c-cache-one-black-outline-composite.md) | Cache one black outline composite. |
| [18.3.5 · Separate source scope from outline selection](journey/91d-separate-source-scope-from-outline-selection.md) | Separate source scope from outline selection. |
| [18.3.6 · Prepare masks without a second blend](journey/91e-prepare-masks-without-a-second-blend.md) | Prepare masks without a second blend. |
| [18.3.7 · Own independent global outline resources](journey/91f-own-independent-global-outline-resources.md) | Own independent global outline resources. |
| [18.3.8 · Draw all-solid and selected coverage consistently](journey/91g-draw-all-solid-and-selected-coverage-consistently.md) | Draw all-solid and selected coverage consistently. |
| [18.3.9 · Connect Outline and Arctic presentation commands](journey/91h-connect-outline-and-arctic-presentation-commands.md) | Connect Outline and Arctic presentation commands. |
| [18.3.10 · Prove actual outlines and original ownership](journey/91i-prove-actual-outlines-and-original-ownership.md) | Prove actual outlines and original ownership. |
| [18.4.1 · Link current Wood element builders](journey/92-link-current-wood-element-builders.md) | Link current Wood element builders. |
| [18.4.2 · Write original beam column and plate builders](journey/91zza-write-original-beam-column-and-plate-builders.md) | Parse positive finite lengths while retaining omitted defaults. |
| [18.4.3 · Write original support variable beam and block builders](journey/92a-retain-current-positive-lengths-and-six-builders.md) | Add the remaining original Support, Beam Variable and Block wrappers. |
| [18.4.4 · Own original Element geometry](journey/92b-own-original-element-geometry.md) | Own original Element geometry. |
| [18.4.5 · Read current Element command defaults](journey/92c-read-current-element-command-defaults.md) | Read current Element command defaults. |
| [18.4.6 · Build typed Elements from ordered world picks](journey/92d-build-typed-elements-from-ordered-world-picks.md) | Build typed Elements from ordered world picks. |
| [18.4.7 · Capture original picks while the tool runs](journey/92e-capture-original-picks-while-the-tool-runs.md) | Capture original picks while the tool runs. |
| [18.4.8 · Teach named and bare Element values](journey/92f-teach-named-and-bare-element-values.md) | Teach named and bare Element values. |
| [18.4.9 · Make Element input batches atomic](journey/92g-make-element-input-batches-atomic.md) | Make Element input batches atomic. |
| [18.4.10 · Attach readonly Element attributes](journey/92h-attach-readonly-element-attributes.md) | Attach readonly Element attributes. |
| [18.4.11 · Adopt Elements and consume sources once](journey/92i-adopt-elements-and-consume-sources-once.md) | Adopt Elements and consume sources once. |
| [18.4.12 · Save and open original Element records](journey/92j-save-and-open-original-element-records.md) | Save and open original Element records. |
| [18.4.13 · Own independent Element tool state](journey/92k-own-independent-element-tool-state.md) | Own independent Element tool state. |
| [18.4.14 · Commit one Element creation and retry errors](journey/92l-commit-one-element-creation-and-retry-errors.md) | Commit one Element creation and retry errors. |
| [18.4.15 · Route Element commands and independent visibility](journey/92m-route-current-element-commands-and-feature-visibility.md) | Toggle Element Attributes and Element Interactions independently. |
| [18.4.16 · Pick Elements through the real command dock](journey/92n-pick-elements-through-the-real-command-dock.md) | Pick Elements through the real command dock. |
| [18.4.17 · Prove Element recipes source history and archives](journey/92o-prove-element-recipes-source-history-and-archives.md) | Prove Element recipes source history and archives. |
| [18.5.1 · Classify original solids independently of clipping](journey/93-classify-original-solids-independently-of-clipping.md) | Classify original solids independently of clipping. |
| [18.5.2 · Own opacity in the existing surface frame](journey/93a-own-opacity-in-the-existing-surface-frame.md) | Own opacity in the existing surface frame. |
| [18.5.3 · Blend closed faces and retain opaque open surfaces](journey/93b-blend-closed-faces-and-retain-opaque-open-surfaces.md) | Blend closed faces and retain opaque open surfaces. |
| [18.5.4 · Exclude x-ray faces from original picks and masks](journey/93c-exclude-x-ray-faces-from-original-picks-and-masks.md) | Exclude x-ray faces from original picks and masks. |
| [18.5.5 · Carry opacity into finite original ink](journey/93d-carry-opacity-into-finite-original-ink.md) | Carry opacity into finite original ink. |
| [18.5.6 · Fade hidden ink without showing clipped geometry](journey/93e-fade-hidden-ink-without-showing-clipped-geometry.md) | Fade hidden ink without showing clipped geometry. |
| [18.5.7 · Invalidate presentation and preserve cached geometry](journey/93f-invalidate-presentation-and-preserve-cached-geometry.md) | Invalidate presentation and preserve cached geometry. |
| [18.5.8 · Connect strict Opacity presentation commands](journey/93g-connect-strict-opacity-presentation-commands.md) | Connect strict Opacity presentation commands. |
| [18.5.9 · Prove actual glass x-ray and source ownership](journey/93h-prove-actual-glass-x-ray-and-source-ownership.md) | Prove actual glass x-ray and source ownership. |
| [18.6.1 · Declare the rendering comparison contract](journey/94-declare-the-rendering-comparison-contract.md) | Declare the rendering comparison contract. |
| [18.6.2 · Link the maintained native rendering reference](journey/94a-link-the-maintained-native-rendering-reference.md) | Link the maintained native rendering reference. |
| [18.6.3 · Read actual maintained frames without source mutation](journey/94b-read-actual-maintained-frames-without-source-mutation.md) | Read actual maintained frames without source mutation. |
| [18.6.4 · Prove the original comparison and inspect differences](journey/94c-prove-the-original-comparison-and-inspect-differences.md) | Prove the original comparison and inspect differences. |
| [18.7.1 · Record comparable large scene conditions](journey/95-record-comparable-large-scene-conditions.md) | Record comparable large scene conditions. |
| [18.7.2 · Measure warmed frame times and the slow tail](journey/95a-measure-warmed-frame-times-and-the-slow-tail.md) | Measure warmed frame times and the slow tail. |
| [18.7.3 · Reject incomparable before and after runs](journey/95b-reject-incomparable-before-and-after-runs.md) | Reject incomparable before and after runs. |
| [18.7.4 · Witness immutable tree inputs across camera frames](journey/95c-witness-immutable-tree-inputs-across-camera-frames.md) | Witness immutable tree inputs across camera frames. |
| [18.7.5 · Reuse layer rows while rotating and zooming](journey/95d-reuse-layer-rows-while-rotating-and-zooming.md) | Reuse layer rows while rotating and zooming. |
| [18.7.6 · Borrow graph edges and index their labels](journey/95e-borrow-graph-edges-and-index-their-labels.md) | Borrow graph edges and index their labels. |
| [18.7.7 · Prove cache invalidation and honest performance reports](journey/95f-prove-cache-invalidation-and-honest-performance-reports.md) | Prove cache invalidation and honest performance reports. |
| [19.1.1 · Retain non drawing originals and validate raw planes](journey/96-retain-non-drawing-originals-and-validate-raw-planes.md) | Retain non drawing originals and validate raw planes. |
| [19.1.2 · Merge unused definitions with displayed instances](journey/96a-merge-unused-definitions-with-displayed-instances.md) | Merge unused definitions with displayed instances. |
| [19.1.3 · Preserve component and interaction metadata in archives](journey/96b-preserve-component-and-interaction-metadata-in-archives.md) | Preserve component and interaction metadata in archives. |
| [19.1.4 · Keep current Element and frame definition families](journey/96c-keep-current-element-and-frame-definition-families.md) | Keep current Element and frame definition families. |
| [19.1.5 · Observe the real current registry and parser](journey/96d-observe-the-real-current-registry-and-parser.md) | Observe the real current registry and parser. |
| [19.1.6 · Verify the complete declared current source closure](journey/96e-verify-the-complete-declared-current-source-closure.md) | Verify the complete declared current source closure. |
| [19.1.7 · Report retained original metadata in the real viewer](journey/96f-report-retained-original-metadata-in-the-real-viewer.md) | Report retained original metadata in the real viewer. |
| [19.1.8 · Prove exact metadata and definition round trips](journey/96g-prove-exact-metadata-and-definition-round-trips.md) | Prove exact metadata and definition round trips. |
| [19.1.9 · Prove current source grammar and explicit differences](journey/96h-prove-current-source-grammar-and-explicit-differences.md) | Prove current source grammar and explicit differences. |
| [19.2.1 · Separate own attributes from hosted features](journey/96i-separate-own-and-hosted-element-features.md) | Identify own attributes and interaction features by original feature type. Check readable names and face labels without changing any geometry. |
| [19.2.2 · Group pieces by their original interaction](journey/96j-group-features-by-original-interaction.md) | Walk graph edges in their original order. Group contact and joint pieces under the full interaction type, using the relation name or source Element name, and retain unmatched features. |
| [19.2.3 · Keep nested Elements editable and projections unsaved](journey/96k-separate-derived-rows-from-editable-elements.md) | Mark generated display rows separately from original objects. Nested child Elements remain editable even inside attributes, and saved documents omit derived geometry. |
| [19.2.4 · Attach a bounded feature tree to its host](journey/96l-attach-a-bounded-derived-feature-tree.md) | Build read-only attributes, interaction and piece rows under the owning Element. Validate the complete tree before replacing its saved structure. |
| [19.2.5 · Project original outlines without changing their coordinates](journey/96m-project-original-feature-outlines.md) | Create black display paths from original feature outlines, apply the host placement once, and keep attributes Off and interactions On by default. |
| [19.2.6 · Connect imported Elements to their feature hierarchy](journey/96n-connect-imported-interaction-hierarchies.md) | Import a document atomically, classify its editable child Elements, then attach feature projections from its original graph and retained Element payloads. |
| [19.2.7 · Use the same projection path for created Elements](journey/96o-reuse-projections-for-created-elements.md) | Created Elements and imported Elements now share feature display ownership. Visibility commands stop at independent child Elements and collect each feature piece once. |
| [19.2.8 · Build an original interaction specimen](journey/96p-build-an-imported-interaction-specimen.md) | Build a small original document with a host, an editable nested child, and a connector. Its contact, unnamed joint and orphan feature make the hierarchy inspectable without a large floor model. |
| [19.2.9 · Prove imported feature ownership and round trips](journey/96q-prove-imported-feature-ownership-and-roundtrips.md) | Verify that feature projections follow their owner once, preserve original payloads, survive Delete/Undo, and rebuild after Save/Open without serializing display copies. Reject invalid outlines without changing the current document. |
| [19.2.10 · Prove exact commands across the expanded vocabulary](journey/96r-preserve-exact-command-completions.md) | Share the existing dictionary wrapper with the real widget regression and browser typing proof. |
| [19.3.1 · Resolve feature rows to original Element owners](journey/96s-resolve-feature-rows-to-original-element-owners.md) | Trace each derived feature back to the nearest original Element without crossing an independent child Element. |
| [19.3.2 · Select and highlight original feature owners](journey/96t-select-and-highlight-original-feature-owners.md) | Connect feature-row clicks to original-owner selection and the existing editing/history path. |
| [19.3.3 · Fit original box corners to the free canvas](journey/96u-fit-original-box-corners-to-the-free-canvas.md) | Replace sphere fitting with the current projected-corner constraint and five-percent margin. |
| [19.3.4 · Project definition and per-instance features](journey/96v-project-definition-and-per-instance-features.md) | Reuse feature projection while keeping shared definitions and separate instance ownership. |
| [19.3.5 · Detach copied original source identity](journey/96w-detach-copied-original-source-identity.md) | Detach copied original source identity |
| [19.3.6 · Regenerate display for copied feature owners](journey/96x-regenerate-display-for-copied-feature-owners.md) | Regenerate display for copied feature owners |
| [19.3.7 · Address feature pixels to original CAD owners](journey/96y-address-feature-pixels-to-original-cad-owners.md) | Address feature pixels to original CAD owners |
| [19.3.8 · Preserve source feature visibility and pixel styles](journey/96z-preserve-source-feature-visibility-and-pixel-styles.md) | Project only visible original features, with two-pixel lines and twelve-pixel point discs. |
| [19.4.1 · Address original controls with family keys](journey/96za-address-original-controls-with-family-keys.md) | Address original controls with family keys. |
| [19.4.2 · Draw and pick original CAD control points](journey/96zb-draw-and-pick-original-cad-control-points.md) | Draw and pick original CAD control points. |
| [19.4.3 · Share the current control-point style](journey/96zc-share-the-current-control-point-style.md) | Use seven CSS pixel blue controls and same-size yellow selection. |
| [19.4.4 · Reuse world controls for their display nets](journey/96zd-reuse-world-controls-for-their-display-nets.md) | Draw placed control nets through the existing stroke lane. |
| [19.4.5 · Scope controls to the selected original](journey/96ze-scope-controls-to-the-selected-original.md) | Enter original control selection without competing object handles. |
| [19.4.6 · Edit original controls through the source preview](journey/96zf-edit-original-controls-through-the-source-preview.md) | Deform original control geometry without editing GPU records. |
| [19.4.7 · Rebuild an edited Element without replacing its owner](journey/96zg-rebuild-an-edited-element-without-replacing-its-owner.md) | Replace Element display geometry while retaining its original CAD identity. |
| [20.1.1 · Bind the shared geometry as storage](journey/96zh-bind-the-shared-geometry-as-storage.md) | Reuse the existing geometry buffers for shader vertex reads. |
| [20.1.2 · Read source vertices through one shader helper](journey/96zi-read-source-vertices-through-one-shader-helper.md) | Define the same vertex reader for ordinary and instanced drawing. |
| [20.1.3 · Activate shared vertex pulling for color](journey/96zj-activate-shared-vertex-pulling-for-color.md) | Rasterize ordinary and instanced triangles from their shared storage buffers. |
| [20.1.4 · Assign frame triangle ranges](journey/96zk-assign-frame-triangle-ranges.md) | Give every placed color triangle the address of its projected record. |
| [20.1.5 · Own the integer primitive image](journey/96zl-own-the-integer-primitive-image.md) | Allocate the frame image that will name rasterized triangles. |
| [20.1.6 · Write triangle addresses beside color](journey/96zm-write-triangle-addresses-beside-color.md) | Write color and a lossless triangle address in the same depth-tested pass. |
| [20.2.1 · Bind visible triangle addresses for ink](journey/96zn-bind-visible-triangle-addresses-for-ink.md) | Make the exact triangle image available to the existing shared ink shader. |
| [20.2.2 · Describe ink depth and surface slack](journey/96zo-describe-ink-depth-and-surface-slack.md) | Type the current axis record and depth-carrying error budget. |
| [20.2.3 · Fit a neighboring surface along the stroke](journey/96zp-fit-a-neighboring-surface-along-the-stroke.md) | Type the current fallback that carries a neighboring plane to the stroke axis. |
| [20.2.4 · Own a cropped primitive image for picks](journey/96zq-own-a-cropped-primitive-image-for-picks.md) | Retain a bounded triangle image beside integer owners and depth. |
| [20.2.5 · Share crop and clipping bindings for pulled picks](journey/96zr-share-crop-and-clipping-bindings-for-pulled-picks.md) | Prepare a combined uniform group without copying either uniform. |
| [20.2.6 · Write first-pick primitives from shared geometry](journey/96zs-write-first-pick-primitives-from-shared-geometry.md) | Rasterize bounded integer owners and primitive addresses together. |
| [20.2.7 · Fit surface depth beneath a point disc](journey/96zt-fit-surface-depth-beneath-a-point-disc.md) | Type the current two-axis surface carry for a disc fragment. |
| [20.2.8 · Test the disc center against a surface](journey/96zu-test-the-disc-center-against-a-surface.md) | Type the current center check that prevents a hidden point leaking at a fringe. |
| [20.2.9 · Carry the rasterized triangle gradient to ink](journey/96zv-carry-the-rasterized-triangle-gradient-to-ink.md) | Read the lossless primitive address and use its projected surface gradient. |
| [20.2.10 · Connect surface-fitted point visibility](journey/96zw-connect-surface-fitted-point-visibility.md) | Use the disc fragment and center tests for original point color and integer picks. |
| [20.2.11 · Test rasterized neighbors at the stroke source](journey/96zx-test-the-rasterized-neighbors-at-the-stroke-source.md) | Prepare exact source tests using the matching primitive image. |
| [20.2.12 · Connect fitted surfaces and exact stroke occlusion](journey/96zy-connect-fitted-surfaces-and-exact-stroke-occlusion.md) | Activate surface fitting, source-neighbor tests and bounded tile fallback for original strokes. |
| [20.2.13 · Separate hidden ink color from pick ownership](journey/96zz-separate-hidden-ink-color-from-pick-ownership.md) | Reveal hidden ink through glass without selecting an occluded original. |
| [20.2.14 · Connect section primitives to color and first picking](journey/96zza-connect-section-primitive-images.md) | Share lossless cap addresses with mesh surfaces in color and first-pick images. |
| [20.2.15 · Read section visibility from one image](journey/96zzb-read-section-visibility-from-one-image.md) | Use one physical primitive address for mesh and section ink visibility. |
| [20.3.1 · Share section addresses with outlines and ambient shading](journey/96zzc-share-section-addresses-with-outlines-and-ambient.md) | Read section identity once across ink, outline and ambient passes. |
| [20.3.2 · Use compact vertices in every surface pass](journey/96zzd-pack-surface-position-normal-and-color.md) | Use one compact surface vertex buffer throughout rendering and picking. |
| [20.3.3 · Retain RGBA in prepared displays](journey/96zze-retain-rgba-in-prepared-displays.md) | Preserve display RGBA through preparation and the compact upload. |
| [20.4.1 · Start with current browser display defaults](journey/96zzf-start-with-current-browser-display-defaults.md) | Apply current device and URL display defaults to browser startup and viewport sizing. |
| [20.5.1 · Grow buffers without losing rows](journey/96zzg-grow-buffers-without-losing-rows.md) | Preserve uploaded GPU records when a shared buffer grows. |
| [20.5.2 · Prepare shared compact mesh ranges](journey/96zzh-prepare-shared-compact-mesh-ranges.md) | Prepare shared packed records and stable ranges without pinning display sources. |
| [20.5.3 · Upload compact ranges to shared GPU storage](journey/96zzi-upload-compact-ranges-to-shared-gpu-storage.md) | Upload shared compact ranges and refresh immutable bindings after buffer growth. |
| [20.5.4 · Share the object record layout](journey/96zzj-share-the-object-record-layout.md) | Share the host object-record size across uniform and instance consumers. |
| [20.5.5 · Borrow the current arena buffers](journey/96zzk-borrow-the-current-arena-buffers.md) | Use one geometry interface for standalone and current shared storage. |
| [20.5.6 · Read shared ranges through object records](journey/96zzl-read-shared-ranges-through-object-records.md) | Read an arena range without changing original owners or local triangle identity. |
| [20.5.7 · Connect the renderer to one mesh arena](journey/96zzm-connect-the-renderer-to-one-mesh-arena.md) | Render prepared source meshes through one shared arena and release its owner when unused. |
| [20.5.8 · Describe eight-byte draw slots](journey/96zzn-describe-eight-byte-draw-slots.md) | Store an object row and primitive ID base in one instance-step vertex record. |
| [20.5.9 · Read instance properties through draw slots](journey/96zzo-read-instance-properties-through-draw-slots.md) | Connect compact draw slots to the existing instance color pipeline. |
| [20.6.1 · Classify contact fill shading](journey/96zzp-classify-contact-fill-shading.md) | Read the contact marker from packed RGBA and keep that display fill opaque and unlit. |
| [20.6.2 · Open files through one measured Session reader](journey/96zzq-open-files-through-one-measured-session-reader.md) | Prepare browser file imports once through the measured full original-document reader. |
| [20.6.3 · Route current Close and Opacity commands](journey/96zzr-route-current-close-and-opacity-commands.md) | Finish an active curve with Close and read the current Opacity argument convention. |
| [20.6.4 · Reuse full source preparation for editor and reload](journey/96zzs-reuse-full-source-preparation-for-editor-and-reload.md) | Reuse the full Session reader for byte actions and released mesh sources. |
| [20.6.5 · Share the current selection yellow](journey/96zzt-share-the-current-selection-yellow.md) | Use the current selection yellow for surfaces and temporary highlights without changing original colors. |
| [20.7.1 · Separate primitive addresses from selection](journey/96zzu-separate-primitive-addresses-from-selection.md) | Keep full primitive addresses in the range record while selection remains an independent flag. |
| [20.7.2 · Bind the own-row slot for ordinary surfaces](journey/96zzv-bind-the-own-row-slot-for-ordinary-surfaces.md) | Use one integer vertex input for ordinary surfaces and placed-instance surfaces. |
| [20.7.3 · Reload scenes through the existing publication owner](journey/96zzw-reload-scenes-through-the-existing-publication-owner.md) | Expose scene replacement through the existing reader, including empty-manifest document release. |
| [20.7.4 · Update only changed slot runs](journey/96zzx-update-only-changed-slot-runs.md) | Patch changed instance slots without rewriting their unchanged neighbours. |
| [20.7.5 · Share one slot buffer across surface draws](journey/96zzy-share-one-slot-buffer-across-surface-draws.md) | Bind ordinary surfaces and every instance batch from one slot buffer. |
| [20.7.6 · Prepare the integer primitive-slot texture](journey/96zzz-prepare-the-integer-primitive-slot-texture.md) | Store slot lookup records in an integer texture with bounded partial uploads. |
| [20.7.7 · Describe slot and triangle runs](journey/96zzza-describe-slot-and-triangle-runs.md) | Represent reserved slot capacity separately from occupied members and triangle IDs. |
| [20.7.8 · Read slot runs and lookup texels](journey/96zzzb-read-slot-runs-and-lookup-texels.md) | Derive GPU slot values and lookup texels from the same CPU run. |
| [20.7.9 · Reuse and merge free slot ranges](journey/96zzzc-reuse-and-merge-free-slot-ranges.md) | Allocate fitting holes, retain unused room and merge released neighbours. |
| [20.7.10 · Follow arena growth with slot headroom](journey/96zzzd-follow-arena-growth-with-slot-headroom.md) | Preserve instance IDs while arena growth remains inside reserved headroom. |
| [20.7.11 · Report only changed instance slots](journey/96zzze-report-only-changed-instance-slots.md) | Update touched members and growing definitions while reporting bounded dirty ranges. |
| [20.7.12 · Project triangles at their physical addresses](journey/96zzzf-project-triangles-at-their-physical-addresses.md) | Keep finite triangle records at the same addresses written into the primitive image. |
| [20.8.1 · Clear the current Arctic background](journey/96zzzg-clear-the-current-arctic-background.md) | Use the current Arctic background without another draw or geometry upload. |
| [20.8.2 · Share changed record runs](journey/96zzzh-share-changed-record-runs.md) | Reuse the existing contiguous-change scan for slot and object records. |
| [20.8.3 · Prepare one shared object record table](journey/96zzzi-prepare-one-shared-object-record-table.md) | Prepare one growable storage table without duplicating slot upload logic. |
| [20.8.4 · Share instance object records across definitions](journey/96zzzj-share-instance-object-records-across-definitions.md) | Draw distinct definitions from one shared placement-record buffer. |
| [20.8.5 · Reuse checked curve control coordinates](journey/96zzzk-reuse-checked-curve-control-coordinates.md) | Use one checked rational-control decoder for sampling and control markers. |
| [20.8.6 · Publish allocator texels through the integer table](journey/96zzzl-publish-allocator-texels-through-the-integer-table.md) | Connect allocator changes to the existing exact integer texture writer. |
| [20.8.7 · Resolve physical triangles with integer slots](journey/96zzzm-resolve-physical-triangles-with-integer-slots.md) | Resolve a physical triangle ID back to its definition triangle and object row. |
| [20.8.8 · Batch shared geometry with original ownership](journey/96zzzn-batch-shared-geometry-with-original-ownership.md) | Reuse instance drawing for copied originals sharing GPU geometry. |
| [20.8.9 · Shrink record ranges without rewriting live prefixes](journey/96zzzo-shrink-record-ranges-without-rewriting-live-prefixes.md) | Reuse one logical buffer truncate operation for object records and draw slots. |
| [20.8.10 · Share opaque ordinary surface records](journey/96zzzp-share-opaque-ordinary-surface-records.md) | Draw ordinary opaque surfaces from the same object-record table as instances. |
| [20.8.11 · Blend ordinary surface records](journey/96zzzq-blend-ordinary-surface-records.md) | Use the shared ordinary record table at the current translucent default. |
| [20.8.12 · Share transparent instance records](journey/96zzzr-share-transparent-instance-records.md) | Draw transparent placements from the same shared records in scene order. |
| [20.9.1 · Allocate stable object record rows](journey/96zzzs-allocate-stable-object-record-rows.md) | Prepare bounded row reuse without moving surviving originals. |
| [20.9.2 · Retain live surface record rows](journey/96zzzt-retain-live-surface-record-rows.md) | Connect stable row reuse to the existing surface record table. |
| [20.9.3 · Prepare shared record pick consumers](journey/96zzzu-prepare-shared-record-pick-consumers.md) | Share the storage layout and shader adaptation before connecting first picks. |
| [20.9.4 · Pick originals through shared records](journey/96zzzv-pick-originals-through-shared-records.md) | Connect first surface picks to current shared object records. |
| [20.9.5 · Read source faces through shared records](journey/96zzzw-read-source-faces-through-shared-records.md) | Connect original source-face picking to the same current object records. |
| [20.9.6 · Pack existing ambient radius in object records](journey/96zzzx-pack-existing-ambient-radius-in-object-records.md) | Carry the existing source radius in a spare aligned record word. |
| [20.9.7 · Read ambient properties through shared records](journey/96zzzy-read-ambient-properties-through-shared-records.md) | Connect source normal and radius drawing to the existing object table. |
| [20.10.1 · Share the object record shader schema](journey/96zzzz-share-the-object-record-shader-schema.md) | Keep one aligned object record declaration across drawing, caps and projection. |
| [20.10.2 · Prepare shared record triangle projection](journey/96zzzza-prepare-shared-record-triangle-projection.md) | Prepare the compute shader to select the same current object row. |
| [20.10.3 · Project triangles through shared records](journey/96zzzzb-project-triangles-through-shared-records.md) | Connect finite triangle projection to the current shared object table. |
| [20.10.4 · Prepare shared record cap counting](journey/96zzzzc-prepare-shared-record-cap-counting.md) | Separate cap crossing geometry from the object resource and vertex format. |
| [20.10.5 · Prepare cap plane and arena bindings](journey/96zzzzd-prepare-cap-plane-and-arena-bindings.md) | Prepare explicit plane parameters with current immutable arena bindings. |
| [20.10.6 · Count cap color through shared records](journey/96zzzze-count-cap-color-through-shared-records.md) | Connect cap crossing color to current object records and explicit planes. |
| [20.10.7 · Prepare shared record cap picks](journey/96zzzzf-prepare-shared-record-cap-picks.md) | Prepare both cap pick crossing states without changing original ownership. |
| [20.10.8 · Own shared record cap pick resources](journey/96zzzzg-own-shared-record-cap-pick-resources.md) | Reuse explicit plane and arena bindings under the existing pick lifetime. |
| [20.10.9 · Pick caps through shared records](journey/96zzzzh-pick-caps-through-shared-records.md) | Connect both cap crossing passes to current records and retain original owners. |
| [20.11.1 · Adapt incremental slots to pulled draws](journey/96zzzzi-adapt-incremental-slots-to-pulled-draws.md) | Keep physical triangle bases correct when pulled draws begin at local occurrence zero. |
| [20.11.2 · Prepare exact arena slot definitions](journey/96zzzzj-prepare-exact-arena-slot-definitions.md) | Describe allocator members using exact arena ranges and stable object rows. |
| [20.11.3 · Retain live allocator member runs](journey/96zzzzk-retain-live-allocator-member-runs.md) | Update retained source runs from the live stable object rows. |
| [20.11.4 · Prepare one plan for draw slots and lookup](journey/96zzzzl-prepare-one-plan-for-draw-slots-and-lookup.md) | Publish one retained plan through the existing draw-slot and integer-texture owners. |
| [20.11.5 · Prepare one owner for live address updates](journey/96zzzzm-prepare-one-owner-for-live-address-updates.md) | Prepare synchronized mesh and record updates without changing primitive addresses. |
| [20.11.6 · Adopt retained GPU slots and physical bases](journey/96zzzzn-adopt-retained-gpu-slots-and-physical-bases.md) | Draw retained sparse slots with synchronized original uniforms and shared records. |
| [20.11.7 · Separate projection rows and arena offsets](journey/96zzzzo-separate-projection-rows-and-arena-offsets.md) | Make source addressing explicit before projection consumes the inverse lookup. |
| [20.11.8 · Prepare lookup-selected triangle projection](journey/96zzzzp-prepare-lookup-selected-triangle-projection.md) | Resolve physical primitive IDs to their shared source triangle and object row. |
| [20.11.9 · Prepare the lookup projection pipeline](journey/96zzzzq-prepare-the-lookup-projection-pipeline.md) | Add the inverse lookup texture to the existing compute binding recipe. |
| [20.11.10 · Own current lookup projection bindings](journey/96zzzzr-own-current-lookup-projection-bindings.md) | Keep current shared inputs and replace immutable projection bindings when needed. |
| [20.11.11 · Project live triangles through the lookup](journey/96zzzzs-project-live-triangles-through-the-lookup.md) | Resolve shared source triangles and object rows on the GPU for live projection. |
| [20.12.1 · Plan reusable arena spans](journey/96zzzzt-plan-reusable-arena-spans.md) | Prepare bounded hole reuse without moving any surviving geometry. |
| [20.12.2 · Reuse retired arena record ranges](journey/96zzzzu-reuse-retired-arena-record-ranges.md) | Connect bounded span plans to CPU arena records while preserving live offsets. |
| [20.12.3 · Upload reused arena spans in place](journey/96zzzzv-upload-reused-arena-spans-in-place.md) | Write reused geometry ranges without replacing unchanged GPU buffers. |
| [20.12.4 · Reclaim ranges after the last GPU lease](journey/96zzzzw-reclaim-ranges-after-the-last-gpu-lease.md) | Retire GPU ranges independently of precise CPU history ownership. |
| [20.12.5 · Retire empty arena ranges after tail trimming](journey/96zzzzx-retire-empty-arena-ranges-after-tail-trimming.md) | Release empty source records independently of the current arena extent. |
| [21.1.1 · Prepare unit-aware anchored camera space](journey/96zzzzy-prepare-unit-aware-anchored-camera-space.md) | Keep scene coordinates precise while expressing the camera projection in physical metres. |
| [21.1.2 · Prepare precise anchor-relative object records](journey/96zzzzz-prepare-precise-anchor-relative-object-records.md) | Retain precise placements while updating only their GPU translation. |
| [21.1.3 · Prepare anchor-relative original ink](journey/96zzzzza-prepare-anchor-relative-original-ink.md) | Pack original ink positions only after subtracting the shared scene origin. |
| [21.1.4 · Prepare shared-origin frame inputs](journey/96zzzzzb-prepare-shared-origin-frame-inputs.md) | Keep derived rendering inputs consistent with one precise scene origin. |
| [21.1.5 · Prepare cloud and sheet frame placements](journey/96zzzzzc-prepare-cloud-and-sheet-frame-placements.md) | Preserve cached cloud and sheet sources while their frame origin changes. |
| [21.1.6 · Prepare local temporary overlays](journey/96zzzzzd-prepare-local-temporary-overlays.md) | Keep temporary ink and cached measurement positions precise across origin changes. |
| [21.1.7 · Preserve local precision before source placement](journey/96zzzzze-preserve-local-precision-before-source-placement.md) | Avoid a large world-coordinate intermediate when packing placed source data. |
| [21.1.8 · Prepare precise construction-grid origins](journey/96zzzzzf-prepare-precise-construction-grid-origins.md) | Pack a construction grid in the same precise local frame as original geometry. |
| [21.1.9 · Initialize object records in the local frame](journey/96zzzzzg-initialize-object-records-in-the-local-frame.md) | Create precise records before their first GPU draw. |
| [21.1.10 · Prepare original highlights in the shared local frame](journey/96zzzzzh-prepare-anchor-relative-original-highlights.md) | Keep original face, edge, curve and control highlights aligned after rebasing. |
| [21.1.11 · Verify the shared unit through original history](journey/96zzzzzi-share-one-camera-and-measurement-unit.md) | Check that measurement, projection and original history use the shared camera unit. |
| [21.2.1 · Project CPU queries around the precise target](journey/96zzzzzj-project-cpu-queries-around-the-precise-target.md) | Keep CPU screen queries precise before converting world points to pixels. |
| [21.2.2 · Prepare precise text selection polygons](journey/96zzzzzk-prepare-precise-text-selection-polygons.md) | Project original text bounds in a precise local query frame. |
| [21.2.3 · Connect precise rectangle selection queries](journey/96zzzzzl-connect-precise-rectangle-selection-queries.md) | Select original geometry and text through one precise CPU frame. |
| [21.2.4 · Finish precise lasso and handle queries](journey/96zzzzzm-finish-precise-lasso-and-handle-queries.md) | Keep lasso selection and handle sizing in precise local frames. |
| [21.2.5 · Stamp asynchronous picks with the scene origin](journey/96zzzzzn-stamp-asynchronous-picks-with-the-origin.md) | Reject pick readbacks from a different local coordinate frame. |
| [21.2.6 · Retry rebased picks with the latest cursor](journey/96zzzzzo-retry-rebased-picks-with-the-latest-cursor.md) | Finish a released gesture when its asynchronous pick frame is rebased. |
| [21.2.7 · Share derived inputs for origin refresh](journey/96zzzzzp-share-derived-inputs-for-origin-refresh.md) | Refresh local draw inputs without replacing the document or cancelling its gesture. |
| [21.3.1 · Prepare one shared render origin](journey/96zzzzzq-prepare-one-shared-render-origin.md) | Adopt one origin and synchronize original draw inputs before rendering or picking. |
| [21.3.2 · Prepare browser frame matrices and owned wakes](journey/96zzzzzr-prepare-browser-frame-matrices-and-owned-wakes.md) | Use one browser render matrix and prepare a wake owned by the runtime. |
| [21.3.3 · Adopt the shared origin in the browser](journey/96zzzzzs-adopt-the-shared-origin-in-the-browser.md) | Keep browser drawing and asynchronous interaction in one precise local frame. |
| [21.4.1 · Share depth endpoints and comparisons](journey/96zzzzzt-share-depth-endpoints-and-comparisons.md) | Give frame depth state one explicit owner before changing its convention. |
| [21.4.2 · Share homogeneous near-plane ink clipping](journey/96zzzzzu-share-homogeneous-near-plane-ink-clipping.md) | Clip expanded original ink through one homogeneous recipe. |
| [21.4.3 · Use reversed depth across the frame](journey/96zzzzzz-use-reversed-depth-across-the-frame.md) | Give every depth reader and writer the same near-one convention. |
| [21.5.1 · Match physical camera defaults and limits](journey/96zzzzzza-match-physical-camera-defaults-and-limits.md) | Keep camera defaults and numeric limits in physical units. |
| [21.5.2 · Omit unused background pick pipelines](journey/96zzzzzzb-omit-unused-background-pick-pipelines.md) | Create an integer pipeline only for lanes that can own a pick. |
| [21.5.3 · Draw the construction grid as physical lines](journey/96zzzzzzc-draw-the-construction-grid-as-physical-lines.md) | Reuse the grid owner with a physical background pipeline. |
| [21.5.4 · Build viewing rays without matrix inversion](journey/96zzzzzzd-build-viewing-rays-without-matrix-inversion.md) | Keep drawing rays and rendered depth consistent in both projections, including when the construction grid is hidden. |
| [21.5.5 · Grow camera depth from cached world bounds](journey/96zzzzzze-grow-camera-depth-from-cached-world-bounds.md) | Keep later or moved originals inside the depth range without refitting the view or walking their vertices on every camera frame. |
| [21.5.6 · Match wheel input and framed projection](journey/96zzzzzzf-match-wheel-input-and-framed-projection.md) | Use current wheel units and preserve the visible region when changing from orthographic to perspective. |
| [22.1.1 · Share flat and Arctic surface frames](journey/96zzzzzzg-share-current-surface-lighting-and-camera-uniforms.md) | Keep original fill colors and Arctic sky shading in one surface shader, with stable ambient cameras during wide zoom. |
| [22.1.2 · Connect headlight and backface controls](journey/96zzzzzzh-connect-headlight-and-backface-controls.md) | Expose current headlight and backface presentation through the existing surface shader, editor commands and browser settings. |
| [22.2.1 · Share one frame across color and ink](journey/96zzzzzzi-share-one-frame-across-color-and-ink.md) | Remove repeated ink-frame buffers and updates while keeping one explicit surface/ink layout and independent request-specific picking. |
| [22.2.2 · Fill original contact polygons](journey/96zzzzzzj-fill-original-contact-polygons.md) | Connect original contact polygons, their existing perimeter and original object owner through one derived mesh with bounded shared surface depth. |
| [22.2.3 · Broadcast ordinary ink metadata](journey/96zzzzzzk-broadcast-ordinary-ink-metadata.md) | Replace repeated zero clipping flags with one broadcast record in ordinary line, path and point batches. |
| [22.2.4 · Layer original contact boundaries](journey/96zzzzzzl-layer-original-contact-boundaries.md) | Keep original contact borders complete at steep angles when the projection budget falls back to physical depth. |
| [22.2.5 · Query original faces under contact overlays](journey/96zzzzzzm-query-original-faces-under-contact-overlays.md) | Select the actual original face beneath a readonly contact, without inventing an editable face for its display polygon. |
| [22.3.1 · Respect original boundary pens](journey/96zzzzzzn-respect-original-boundary-pens.md) | Render original edge widths and colors, hide authored zero-width mesh edges, and omit coplanar seams without changing source face keys. |
| [22.3.2 · Close original boundary joins](journey/96zzzzzzo-close-original-boundary-joins.md) | Close wide original boundary corners and preserve actual source queries at round ends and flat near-plane cuts. |
| [22.3.3 · Draw default mesh vertex dots](journey/96zzzzzzp-draw-default-mesh-vertex-dots.md) | Render the maintained default and authored mesh vertex dots without duplicating source meshes or endpoint positions. |
| [22.3.4 · Taper crowded original mesh vertices](journey/96zzzzzzq-taper-crowded-original-mesh-vertices.md) | Keep dense source vertices distinguishable while retaining explicit point sizes, editing controls and original ownership. |
| [22.4.1 · Share geometric ambient inputs](journey/96zzzzzzr-share-geometric-ambient-inputs.md) | Shade contacts from actual face geometry while preserving authored lighting normals and one contact-radius owner. |
| [22.4.2 · Shade surfaces before drawing ink](journey/96zzzzzzs-shade-surfaces-before-drawing-ink.md) | Preserve authored drawing colours and independent widget pixels by ordering existing passes. |
| [22.4.3 · Keep contact ambient inputs on originals](journey/96zzzzzzt-keep-contact-ambient-inputs-on-originals.md) | Match the displayed contact depth and original visual radius without adding GPU resources. |
| [22.5.1 · Keep depth and address samples separate](journey/96zzzzzzu-keep-depth-and-address-samples-separate.md) | Define matching frame attachments without averaging depth or physical primitive addresses. |
| [22.5.2 · Reuse coherent frame targets](journey/96zzzzzzv-reuse-coherent-frame-targets.md) | Retain frame attachments at equal extent and replace them coherently during real resizing. |
| [22.5.3 · Keep resolved colours in the canvas format](journey/96zzzzzzw-match-resolve-base-and-view-formats.md) | Preserve flat colours by matching the base texture formats and rendering view formats at both ends of a resolve. |
| [22.5.4 · Reuse surface pipelines for each sample count](journey/96zzzzzzx-reuse-count-specific-surface-pipelines.md) | Match ordinary and shared surface shaders and pipeline descriptors to the requested sample count while retaining original geometry and previously compiled variants. |
| [22.5.5 · Generate ink shaders for physical samples](journey/96zzzzzzy-generate-ink-for-physical-samples.md) | Carry an explicit physical sample through finite-ink visibility and generate colour shaders whose coordinates, depth, primitive addresses and pipeline descriptor use the same sample count. |
| [22.5.6 · Cache matching ink pipelines and bindings](journey/96zzzzzzz-cache-ink-pipelines-with-independent-queries.md) | Retain matching finite-ink colour pipelines and binding layouts for both sample counts while keeping integer queries independent and sharing the original vertex-layout constants. |
| [22.5.7 · Retarget existing overlay lanes](journey/96zzzzzzza-retarget-existing-overlay-lanes.md) | Apply one requested sample count through existing overlay lanes while retaining their original vertices, IDs, query pipelines and previously compiled colour recipes. |
| [22.5.8 · Read section crossings at each sample](journey/96zzzzzzzb-read-section-crossings-at-each-sample.md) | Match section crossing images and cap colour/depth to each physical sample while retaining original solid geometry, cached recipes and independent single-sample original-owner queries. |
| [22.5.9 · Compose cloud colour, depth and queries](journey/96zzzzzzzc-compose-cloud-depth-with-physical-addresses.md) | Make nearer original clouds agree across visible colour, physical depth/addresses and independent original-owner queries, including ink in front and behind. |
| [22.5.10 · Cache cloud and sheet sample recipes](journey/96zzzzzzzca-cache-cloud-and-sheet-samples.md) | Cache count-specific original cloud and sheet colour recipes while retaining original source geometry, point images/raster, drawing placements/IDs and independent one-sample owner queries. |
| [22.5.11 · Retarget cached text colour](journey/96zzzzzzzd-retarget-cached-text.md) | Match world text, nameplates and screen glyph colour to physical sample counts while retaining original sources, shaping, fonts, atlas contents, world-plane raster images and independent original-owner queries. |
| [22.5.12 · Cache section masks and the final outline blend](journey/96zzzzzzze-cache-outline-section-recipes.md) | Prepare count-specific section coverage and outline colour recipes while retaining original addresses, one-sample search images and one final black blend. |
| [22.5.13 · Resolve float outline coverage](journey/96zzzzzzzf-resolve-float-outline-coverage.md) | Feed physical face and section coverage into the retained one-sample outline search and blend the selected/global result once, without resolving integer addresses or depth. |
| [22.5.14 · Retarget original ambient properties](journey/96zzzzzzzg-retarget-original-ambient-properties.md) | Prepare cached physical-sample original radius and normal sources while retaining original geometry and the one-sample ambient hierarchy, filters and history images. |
| [22.5.15 · Share ambient shader recipes](journey/96zzzzzzzh-share-ambient-shader-recipes.md) | Reuse ambient shader modules and one pipeline factory, prepare matching physical input layouts and connect retained composition bindings before activating sparse sample corrections. |
| [22.5.16 · Correct ambient at physical samples](journey/96zzzzzzzi-correct-ambient-at-physical-samples.md) | Connect matching original depth/address/radius/normal samples to bounded sparse ambient correction while retaining coarse images, source geometry and cached recipes. |
| [22.5.17 · Cache physical depth-pyramid inputs](journey/96zzzzzzzj-cache-physical-depth-pyramid-inputs.md) | Cache physical depth inputs while retaining the six one-sample coarse images and their encoded radius/pixel addresses. |
| [22.5.18 · Retarget the complete colour frame](journey/96zzzzzzzk-retarget-the-complete-colour-frame.md) | Retarget every active colour consumer and frame attachment through one production API while preserving independent original-owner queries. |
| [22.5.19 · Choose bounded frame quality](journey/96zzzzzzzl-choose-bounded-frame-quality.md) | Choose coherent colour samples from actual geometry, physical viewport, device budget and recovery without changing original sources or owner queries. |
| [22.6.1 · Retain visible instance ranges](journey/96zzzzzzzm-retain-visible-instance-ranges.md) | Keep absolute original query addresses while filtering existing instance buffers with merged CPU ranges. |
| [22.6.2 · Prepare original ink families](journey/96zzzzzzzn-prepare-original-ink-families.md) | Classify retained original borders, points and controls once while preserving source query keys and complete buffer bindings. |
| [22.6.3 · Show original ink families](journey/96zzzzzzzo-show-original-ink-families.md) | Apply one retained visibility owner to actual colour, original queries and selected highlights while preserving active controls and shared sources. |
| [22.6.4 · Apply semantic view controls](journey/96zzzzzzzp-apply-semantic-view-controls.md) | Apply one retained editor presentation state before the first frame and after view commands while preserving original edits and history. |
| [22.6.5 · Draw original CAD wire curves](journey/96zzzzzzzq-draw-original-cad-wire-curves.md) | Display original CAD fallback curves through the existing joined path lane while retaining coherent source borders and independent family controls. |
| [Reference comparison · Compare with the maintained browser runtime](journey/97-finish-with-the-maintained-browser-runtime.md) | Run the latest maintained viewer and distinguish its behavior from the remaining handwritten implementation gaps. |

The opening lessons separate browser events, the command dock, application state and GPU drawing. Before adding a camera or editable objects, follow this path without the listing: **typed command → state change → drawing commands → picture**.

</details>

Each lesson has one visible goal, a diagram, exact code changes, an experiment and a question. Do not rush through code you cannot connect to that goal. [Rust foundations](foundations.md) are available when a language idea needs more practice; you do not have to complete eight separate exercises before seeing a canvas.

## The destination

All seven course areas below are authored. The [destination contract](journey/destination.md) maps the 43 reference sections to these areas; the [roadmap](journey/roadmap.md) records acceptance separately.

| Course | Destination |
| --- | --- |
| 1 · A small viewer you understand | Mesh, camera, input and object identity. |
| 2 · Geometry that reads clearly | Curves, points, CAD faces, boundaries, normals and text. |
| 3 · Documents that stay reliable | Loading, streaming, instancing, sheets and resource lifetimes. |
| 4 · Editing that can be undone | Selection, gestures, snapping, gumball, panels and transactions. |
| 5 · The modeling toolbox | Current commands, shapes, surfaces, annotations and splitting tools. |
| 6 · The rendering quality we have now | Finite visibility, clipping caps, outlines, ambient occlusion and transparency. |
| 7 · Prove the complete viewer | Behavior, visual quality, responsiveness and resource checks. |

The [current inventory](journey/inventory.md) accounts for the maintained source and command implementations. [Subject 97](journey/97-finish-with-the-maintained-browser-runtime.md) completes the continuous bridge to that runtime. Actual browser acceptance and matched Lenovo measurements remain pending. The [implementation reference](reference-course.md) stays available until replacement acceptance passes; its separate starting point is not a continuation of your learner project.

## Keep your place

Before an experiment, save your files. After it, write three lines: what changed; which values crossed a file boundary; what still puzzles me. You should be able to return tomorrow without reconstructing the whole lesson in your head.

[Checkpoint and recovery instructions](journey/recovery.md) preserve your own work. A corrected lesson should arrive as a focused change with a reason and a verification step, not a demand to restart. Releases keep their dependency lock and source fingerprints; the documentation labels unverified behavior.

[Prepare your computer](README.md#prepare-your-computer) · [Begin lesson 01](journey/01-canvas.md) · [Release evidence](journey/release.md)
