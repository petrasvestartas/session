# The complete draft lesson checklist

**Draft plan: 238 lesson slots; 172 current checkpoints have fresh build and Chrome evidence. 66 planned lessons remain.** The early dock steps introduce the actual command line before camera and document commands. This plan may grow when a topic needs splitting. Published IDs remain stable.

The endpoint is fixed: the features and quality in [the destination contract](destination.md), including the later browser-recovery and interaction-ownership fixes. The current lessons are the opening of the course; this count is not a percentage of the implementation. Lesson count is not a measure of remaining engineering work.

A checked box means the implementation exists, the displayed code builds, its Rust checks pass where applicable, and its scripted Chrome checkpoint passed. It does not mean every production acceptance case has been covered. An unchecked box is a proposed tutorial, not content you can follow yet. [Read the exact verification scope](release.md).

Each future lesson will get its own typing estimate, architecture diagram, experiment, buildable endpoint and screenshot when it is authored. No reliable total study-time estimate exists for the unpublished lessons.

## Main task: finish a direct, step-by-step viewer course

- [x] Remove the extra opening paragraph from every lesson; keep one short outcome before Type.
- [x] Replace browser-acceptance directions with immediate learner checks in the diagnostic lessons.
- [x] Replace accumulated command tours with one focused check in all current lessons.
- [x] Rewrite all 18 chapter-34 explanations around the current change and its Rust concepts.
- [x] Shorten 49 further explanations; remove stale HTML-control instructions and misplaced proof tours.
- [x] Split the first GPU lesson into a working clear and a separate presentation step.
- [x] Move detailed validation into expandable notes; make experiments optional.
- [x] Put the to-do link on every lesson and in the course navigation.
- [x] Put a Latest lesson link above every lesson and on the course index, so new tutorials are visible from an older page.
- [x] Check the revised page layout and visible progress links in the same Chrome tab.
- [x] Review every current main explanation against the same direct teaching style.
- [x] Replace 100 long or generic code instructions with the exact action, in at most 25 words.
- [x] Replace 48 further repeated instructions with the specific action at each edit.
- [x] Give all 17 new dock steps a concrete optional exercise and a diagram of their actual data or event flow.
- [x] Remove two formatting-only typing steps; put their spacing in the first declarations.
- [x] Apply a direct page layout to all 172 lessons: short introduction, typed changes and one result check; fold extended explanations, experiments and verification notes.
- [x] Remove repeated goal/trace banners and separate explanation/save sections; keep the first lesson’s setup visible.
- [x] Split command-state ownership into two short endpoints, connecting the field immediately and removing unrelated GPU reformatting.
- [x] Split the final 3 long checkpoints into 12 complete runnable steps; no current checkpoint exceeds one hour of estimated typing.
- [ ] Write and verify the 66 currently planned feature lessons below.
- [ ] Verify the final course against the complete production viewer, then retire the old reference course.

Every lesson opens with one outcome paragraph, at most 45 words. The main page goes directly to Type, then Run and check. Keep extra explanation and acceptance evidence folded.

Each lesson should answer: what changes, what Rust is needed, what to type, and what one check proves it works. Each code instruction names its exact action in at most 25 words. Keep the small concepts that explain the code; put extended test evidence in supporting notes. Split by a useful responsibility, and connect it as soon as its dependencies exist.

**Typing limit:** each lesson must fit within one hour of human typing, separately from reading and experiments. The [source-edit audit](typing-load.md) finds no current checkpoint exceeds the conservative planning limit. All 172 checkpoints build and have current Chrome evidence. Future topics will be split when necessary.

- [x] Audit the added/changed code in all 172 current checkpoints, excluding unchanged context.
- [x] Show typing time at the top; keep the combined study estimate in the expandable code explanation.
- [x] Split every current over-limit checkpoint into complete runnable lessons; do not split inside functions.
- [x] Recheck explanations, diagrams, commands, experiments, Rust builds, Chrome input and screenshots for each new endpoint.
- [x] Make every displayed state-check command select the native host, so the repository's WebAssembly default cannot prevent the tests from running.
- [x] Publish each completed lesson and refresh the same existing Chrome tutorial tab, preserving the reader’s current lesson.

## Typing splits completed

- [x] Split the former 03a panel checkpoint into font/painter ownership, text painting and field layout; each fits one hour of typing.
- [x] Split 03b into a connected state model and the completion helpers; remove unrelated GPU formatting changes.
- [x] Split 03c into history, typing, submission, editing, vocabulary, completion, input ownership, preparation and shared layout.
- [x] Split 03d into accepted-command handoff and canvas event delivery. Every new dock step fits one hour of typing.
- [x] Verify the scene split: CPU owners (24–47 minutes) and GPU drawing (23–46); existing code endpoints are unchanged.
- [x] Verify the identity split: stable names (29–57 minutes) and selection display (16–32); existing code endpoints are unchanged.
- [x] Verify the picking split: inverse coordinates (10–19 minutes), triangle query (28–56) and actual clicks (14–27). Remove dependency-feature reordering; independently recheck 13–20.
- [x] Verify the history split: shared snapshots (28–55 minutes) and command routing (8–16); application behavior is unchanged. The earlier coordinate lesson adds a native round-trip test.
- [x] Verify the perspective split into drawing (20–40 minutes), screen rays (14–28) and actual picking (30–59). Existing endpoints are unchanged; native/WebAssembly/Trunk/Chrome checks pass, including actual click, Delete, Undo and Redo after zoom.
- [x] Verify the action split: state owner (14–27 minutes), Rust actions (27–53) and browser routing (29–57). Move the aspect-preserving reset and its test before browser routing; all 20 and later endpoints are unchanged. Native/WebAssembly/Trunk/Chrome checks pass.
- [x] Verify resize through bounded pixel sizing (19–37 minutes) and one canvas/surface/depth/aspect transaction (29–57). Source endpoints are unchanged; native/WebAssembly/Trunk/Chrome checks pass, including wide/tall windows and density two.
- [x] Verify gestures through pointer memory (20–39 minutes), release/cancellation (23–46), action conversion (18–35) and browser delivery (23–46). All existing code endpoints are unchanged. Native/WebAssembly/Trunk/Chrome checks pass, including actual capture outside the canvas, release, lost capture, cancellation, blur, resize and left-click classification.
- [x] Verify wheel normalization (25–50 minutes) and browser delivery (17–34). All existing code endpoints are unchanged. Native/WebAssembly/Trunk/Chrome checks pass, including all 26 typed letters, editing keys, first-character command focus, wheel units and prevented page scrolling.
- [x] Split import into raw records (26–51 minutes), session identity (29–58), decoding (18–35), atomic history (28–55), asynchronous delivery (18–36) and [typed Open](23-import.md) (22–44). Native validation and held-read Chrome checks pass; all earlier source endpoints remain unchanged.
- [x] Split camera fitting into bounds (18–36 minutes), camera calculation (23–46) and [typed Fit](24-fit.md) (18–35). Native scale/corner checks, GPU frames and wide/tall Chrome checks pass.
- [x] Split projection into camera matrices (26–51 minutes), editor actions and mode-aware fitting (25–49), then [named commands](25-projection.md) (9–18). Native size/ray/picking tests, GPU frames and actual Chrome command round trips pass.

## Published checkpoints and current work

- [x] Move the work into this foreground session on GPT-6.1 Sol High.
- [x] Production typing focus, named View commands and right-click repeat verified in Chrome.
- [x] Phone orbit, two-finger pan, pinch zoom and touch cancellation verified in Chrome.
- [x] Production attributes/features default off and opacity 0.95 verified in Chrome.
- [x] Drawing options and snapping use typed commands; broader command-workspace regression passed.
- [x] Production phone edge, loading and navigation corrections accepted: appearance correct, loading faster and rotation smooth; the numerical phone loading target remains unmeasured.
- [ ] Replace remaining production failure/recovery HTML buttons with the command-only flow; preserve usable report retrieval when the GPU stops. The runnable course checkpoints already reject feature buttons.
- [ ] Complete remaining production robustness: adjacent-face ownership, idle tap/selection preparation and fewer ribbon variants. Teach and verify these in the rendering/performance chapters; CPU restructuring remains conditional on phone diagnostics.
- [x] Revise 03d and 22 for immediate typing and command-only keyboard features; recheck 23–26.
- [x] Refresh all changed earlier endpoints and their screenshot evidence.
- [x] [Review this week’s commits](weekly-changes.md) and map visibility, locking, tree colors and curve sampling to lessons.
- [x] [Fix missing ink at close zoom and sharp angles](line-visibility.md); geometric oracle and Chrome zoom checks pass. Teach this in the later rendering chapters.
- [x] Publish the verified tutorial and viewer input batches; refresh the same existing tutorial tab.

- [x] 01 · [A page that Rust can reach](01-canvas.md).

- [x] 01a · [Paint the first GPU frame](01a-gpu.md): first half of the former 87-minute typing step.

- [x] 02 · [Give browser presentation its own function](02-clear.md): bounded sizing, colour view and error feedback.

- [x] 03 · [Give the GPU three corners](03-triangle.md).

- [x] 03a · [Prepare the command fonts and painter](03a-fonts.md).

- [x] 03a · [Paint command text over the scene](03a-paint.md).

- [x] 03a · [Lay out the command field](03a-panel.md).

- [x] 03b · [Give the command field its memory](03b-memory.md).

- [x] 03b · [Prepare the dock completion helpers](03b-state.md).

- [x] 03c · [Draw retained command history](03c-history.md).

- [x] 03ca · [Type directly into the command field](03ca-typing.md).

- [x] 03cb · [Submit command text once](03cb-submit.md).

- [x] 03cc · [Edit the command text](03cc-edit.md).

- [x] 03cd · [Recognize Help](03cd-vocabulary.md).

- [x] 03ce · [Complete a command name](03ce-complete.md).

- [x] 03cf · [Accept or cancel a completion](03cf-accept.md).

- [x] 03cg · [Read editing keys before the field](03cg-keys.md).

- [x] 03ch · [Show matching commands](03ch-popup.md).

- [x] 03ci · [Browse matching names](03ci-browse.md).

- [x] 03cj · [Prepare the dock before painting](03cj-prepare.md).

- [x] 03ck · [Remember where the dock is drawn](03ck-rects.md).

- [x] 03ck · [Click the command dock](03ck-pointer.md).

- [x] 03cl · [Scroll through command names](03cl-wheel.md).

- [x] 03c · [Share the production dock layout](03c-layout.md).

- [x] 03cn · [Return accepted application commands](03cn-handoff.md).

- [x] 03d · [Hand commands to the application](03d-input.md).

- [x] 04 · [Make a choice change the picture](04-input.md).

- [x] 05 · [Let Rust supply the corners](05-vertices.md).

- [x] 06 · [Share a corner between triangles](06-indices.md).

- [x] 07 · [Send one view setting to every corner](07-uniforms.md).

- [x] 08 · [Move the view, keep the geometry](08-camera.md).

- [x] 09 · [Let one matrix describe the view](09-matrices.md).

- [x] 10 · [Keep the nearest surface](10-depth.md).

- [x] 10a · [Give geometry a validated CPU owner](10a-mesh.md).

- [x] 11 · [Draw the scene through GPU mesh owners](11-scene.md).

- [x] 11a · [Name objects independently of their rows](11a-identity.md).

- [x] 12 · [Display selection by object identity](12-identity.md).

- [x] 12a · [Convert screen positions back to the scene](12a-coordinates.md).

- [x] 12b · [Find the nearest triangle at a scene point](12b-picking.md).

- [x] 13 · [Select the visible object with a mouse click](13-picking.md).

- [x] 13a · [Retain reversible scene snapshots](13a-history.md).

- [x] 14 · [Run Undo and Redo from the command line](14-history.md).

- [x] 14a · [Draw through a perspective camera](14a-perspective.md).

- [x] 14b · [Turn a screen point into a bounded ray](14b-ray.md).

- [x] 15 · [Pick the nearest surface through the view](15-perspective.md).

- [x] 16 · [Walk around the model](16-orbit.md).

- [x] 17 · [Bring a solid into the scene](17-solid.md).

- [x] 18 · [Read the shape through light](18-light.md).

- [x] 18a · [Give application state one owner](18a-editor.md).

- [x] 18b · [Apply document and view actions in Rust](18b-actions.md).

- [x] 19 · [Route browser input through the editor](19-actions.md).

- [x] 19a · [Measure a safe drawing size](19a-viewport.md).

- [x] 20 · [Resize canvas, depth and camera together](20-resize.md).
- [x] 20a · [Remember the pointer that starts a drag](20a-press.md).
- [x] 20b · [Finish or cancel a drag](20b-release.md).
- [x] 20c · [Convert pointer motion to editor actions](20c-motion.md).

- [x] 21 · [Connect captured pointers to the editor](21-gestures.md).
- [x] 21a · [Convert wheel units to camera zoom](21a-wheel.md).

- [x] 22 · [Deliver wheel input without keyboard feature shortcuts](22-shortcuts.md).

- [x] 22a · [Validate a raw mesh record](22a-records.md).
- [x] 22b · [Validate session identity and build a sample file](22b-session.md).
- [x] 22c · [Decode and prepare the whole import](22c-decode.md).
- [x] 22d · [Commit an import as one undoable action](22d-import.md).
- [x] 22e · [Prepare asynchronous file delivery](22e-file-reader.md).

- [x] 23 · [Open a file through the command line](23-import.md).

- [x] 23a · [Measure the displayed scene bounds](23a-bounds.md).
- [x] 23b · [Fit the camera around the bounds](23b-fit-camera.md).

- [x] 24 · [Run Fit through the command line](24-fit.md).

- [x] 24a · [Build perspective and orthographic camera matrices](24a-projection.md).
- [x] 24b · [Fit and change projection through the editor](24b-projection-actions.md).

- [x] 25 · [Run the named projection commands](25-projection.md).

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

- [x] 30 · [Report the result that actually committed](30-feedback.md) — file errors stay in the dock; a failed import cannot claim success.

- [x] 30a · [Give a pending read an explicit ticket](30a-tickets.md) — native checks cover stale, cancelled, duplicate and exhausted requests.

- [x] 30b · [Cancel reads without accepting their late result](30b-cancel.md) — Cancel Open revokes delivery; asynchronous results retain later command history.

- [x] 30c · [Replace a document as one reversible change](30c-replace.md) — prepare sources first; preserve owners, camera and issued local IDs.

- [x] 30d · [Choose append or replace before opening the picker](30d-bridge.md) — Open Replace starts a new request immediately and carries its mode through reading.

- [x] 30e · [Prove recovery at the transaction and display boundaries](30e-recovery.md) — partial rollback, selection/Redo, float upload range and startup fallback/reload.

- [x] 31 · [Keep selection out of the vertex data](31-settings.md) — separate immutable colours from eighty-byte object settings.

- [x] 31a · [Give immutable GPU geometry one owner](31a-geometry.md) — retain the exact CPU display owner and share vertex/index buffers between draw rows.

- [x] 31b · [Reuse uploads while their geometry is alive](31b-cache.md) — weak cache, source allocation identity and upload/allocation counters.

- [x] 31c · [Update only changed object settings](31c-incremental.md) — retain rows, queue sixteen-byte selection or sixty-four-byte matrix writes, and verify deletion/reupload.

- [x] 32 · [Find the owners retained by history](32-history.md) — borrowed Undo/Redo roots and explicit release of both branches.

- [x] 32a · [Count shared CPU displays once](32a-cpu.md) — source/document identities, history copies, GPU-held CPU displays and vector-capacity payload.

- [x] 32b · [Count each shared GPU buffer once](32b-gpu.md) — allocated vertex/index sizes and independent object settings.

- [x] 32c · [Close the document without resetting the view](32c-close.md) — release row/history storage, preserve camera and issued IDs.

- [x] 32d · [Close through the command line and revoke reads](32d-command.md) — actual dock command, cancelled delivery, zero live document ledgers and reopening.

- [x] 32e · [Prove release does not retain the old document](32e-release.md) — weak imported/history owner checks, late success/failure, empty Save and fresh reopened IDs.

- [x] 32f · [Keep row metadata separate from editable geometry](32f-metadata.md) — independent names, original source GUIDs, visibility/locking values and shared history metadata; no kernel owner retained by metadata.

- [x] 32fa · [Ask whether an editable source is available](32fa-access.md) — borrowed geometry/provenance access, explicit Save requirements and resident-source accounting.

- [x] 32fb · [Make editable ownership a private row boundary](32fb-boundary.md) — all external owner checks use accessors; preparation and GPU geometry keep their separate contracts.

- [x] 32fc · [Record a reload version without retaining geometry](32fc-version.md) — owned geometry-free header, unique import identity and exact immutable-file SHA-256 version.

- [x] 32fd · [Attach one origin to an imported document](32fd-origin.md) — share import origin across rows/history without pinning kernel Mesh or Session values.

- [x] 32fe · [Give a reload URL an explicit owner](32fe-location.md) — successful/history and failed-import URL ownership; last-owner revocation.

- [x] 32ff · [Adopt the selected file as a reloadable source](32ff-bridge.md) — accepted File handoff, no URL for cancelled reads, adoption errors and actual browser URL cleanup.

- [x] 32fg · [Separate loaded and released editable ownership](32fg-state.md) — total source accessors; retain display, identity, metadata and placement.

- [x] 32fh · [Protect sources that cannot be unloaded faithfully](32fh-policy.md) — generated/unlocated data, source flags and replacement kernel allocations.

- [x] 32fi · [Unload sources across active and history roots](32fi-history.md) — whole-origin validation across active/Undo/Redo, checked epochs and no cleared history.

- [x] 32fj · [Prove unloading preserves placed history](32fj-proof.md) — Weak kernel/document expiration, retained display/GPU owners and cold placement Undo/Redo.

- [x] 32fk · [Unload editable sources through the command line](32fk-command.md) — actual dock command, identical Chrome scene pixels, unchanged GPU counters and cold edit/save errors.

- [x] 32fl · [Protect history and future reload tickets](32fl-guards.md) — modified Redo-only source protection, independent duplicate origins, exhaustion and epochs retained across Close.

- [x] 32g · [Identify the source release a reload belongs to](32g-keys.md) — Borrow the current imported release identity and epoch before asking for source data.

- [x] 32ga · [Prepare original kernel data without rebuilding its display](32ga-prepare.md) — Validate an immutable source version and restore its Session into a private reload candidate.

- [x] 32gb · [Adopt restored source owners as one residency change](32gb-adopt.md) — Validate current releases and all matching history rows before adopting any kernel candidate.

- [x] 32gc · [Prove source restoration preserves display and history](32gc-roundtrip.md) — Restore exact source coordinates and editing without reallocating retained display owners.

- [x] 32gd · [Reject stale or inconsistent source batches atomically](32gd-rejections.md) — Keep rows cold on changed versions, missing source identity and obsolete release keys.

- [x] 32ge · [Ignore old reload results before decoding them](32ge-stale.md) — Reject results for closed imports, loaded rows, duplicate keys and previous release epochs.

- [x] 32gf · [Give each source request its own owner](32gf-request.md) — Active release keys → ReloadJob → ticket and URL strings → one accepted completion.

- [x] 32gfa · [Prove cancelled work releases its source owners](32gfa-ownership.md) — Pending job holds keys → Close drops rows → cancel drops keys → abandoned request is inert.

- [x] 32gg · [Read a source response within its byte limit](32gg-fetch.md) — Recorded URL → fetch with AbortSignal → HTTP/length checks → bounded stream chunks.

- [x] 32gga · [Pair reload ownership with a browser abort controller](32gga-flight.md) — ReloadJob + AbortController → captured signal → cancel or current finish.

- [x] 32ggb · [Deliver only the current completed reload batch](32ggb-delivery.md) — Flight begin → await URL reads → finish ticket once → scoped Rust reply → viewer event.

- [x] 32gh · [Restore editable sources through the command line](32gh-command.md) — Reload Sources → active keys → owned fetch → atomic hydrate → same GPU display.

- [x] 32gha · [Cancel reloads when their document context changes](32gha-cancel.md) — Close / replacement / Undo / release → cancel flight → late result is inert.

- [x] 32gi · [Capture the requested edit before waiting](32gi-capture.md) — Record the original ObjectId and Move arguments; retain no source owner. Native target/release checks and the existing explicit browser reload are verified.

- [x] 32gia · [Load only the sources the requested edit needs](32gia-scope.md) — Original target → its one cold origin; Save → unique active cold origins. Loaded targets fetch nothing; missing targets fail.

- [x] 32gib · [Move the original target from its current placement](32gib-move.md) — Original ObjectId + offset compose with the current model; preserve later selection and camera; one document history transaction.

- [x] 32giba · [Delete the original target while keeping later selection](32giba-delete.md) — Resolve the captured ObjectId, require editable geometry, preserve other selected rows and refuse a missing target without changing history.

- [x] 32gibb · [Route captured Move, Delete and Save results](32gibb-reply.md) — Return the edit change or original-precision snapshot bytes; Save leaves history unchanged. Pending-owner integration follows next.

- [x] 32gic · [Own the intent with its pending source ticket](32gic-owner.md) — Keep intent and keys in one owner; consume current completion once, preserve newer work on stale replies and drop both on cancellation. Browser context revocation follows with automatic commands.

- [x] 32gica · [Pair captured intent with complete source bodies](32gica-reply.md) — Keep the operation beside complete body pairs or a failure; reject missing and extra bodies.

- [x] 32gicb · [Carry captured intent through browser completion](32gicb-bridge.md) — Bind the pending owner to fetch/abort/delivery; retain explicit Reload Sources. Automatic commands follow next.

- [x] 32gid · [Validate restoration before replaying the command](32gid-complete.md) — Hydrate only current source keys and versions, then replay the captured operation; stale or failed restoration cannot edit or download.

- [x] 32gie · [Deliver restored edit and Save results to the dock](32gie-response.md) — Consume Changed/Saved replies and associate completion or failure with the original command name.

- [x] 32gif · [Automatically restore sources for Move, Delete and Save](32gif-auto.md) — Cold commands fetch their required sources and replay once. Later camera/selection remain available; new edits, Cancel, Replace, history travel and Close revoke pending authority.

- [x] 32gj · [Prove restored Save keeps the source doubles](32gj-precision.md) — Use a non-f32-representable source coordinate and compare actual warm and restored Save downloads with uploaded source doubles; verify placement/history and download/source URL cleanup.

- [x] 32gja · [Prove failed restoration cannot partly commit](32gja-failures.md) — Automatic Move/Delete/Save reject HTTP, length/body/read/network/type/version failures without changing drawing or state. Multiple-source Save restores nothing if its second source fails; native missing-body/stale/duplicate-key checks and late-Close URL release remain explicit.

- [x] 33 · [Own browser listeners instead of forgetting callbacks](33-owner.md) — One Closure owns successful bindings. Real EventTargets verify handler detachment and captured-value release after Drop.

- [x] 33a · [Dispose the viewer without leaving pending work alive](33a-runtime.md) — Bind eighteen live listeners to a runtime owner; pagehide defers safe disposal, cancels read/fetch authority and releases captures. Chrome checks late source/file replies, URL release, no further GPU submissions and ordinary document Close/reopen.

- [x] 33b · [Keep a cached viewer ready for Back navigation](33b-cache.md) — Cancel unfinished gestures but retain the runtime for persisted pagehide; final exits still dispose. Chrome checks retained state and command input, then real same-tab Back navigation and reports cached versus fresh loading.

- [x] 34 · [Keep the first GPU failure with its device](34-fault.md) — Arc/Mutex first-reason ownership, independent replacement devices and native concurrent callback checks. Browser callback integration follows next.

- [x] 34a · [Stop the viewer when its GPU device fails](34a-stop.md) — Real uncaptured/device-lost callbacks, immediate input/GPU gate, deferred identity-checked disposal; Chrome destroys a real device and verifies eighteen detached listeners, aborted source work, URL release and zero further GPU calls.

- [x] 34b · [Describe a viewer run without keeping its document](34b-report.md) — Owned metadata, explicit outcome and flat JSON header; native round-trip/snapshot checks and inherited Chrome loss acceptance pass.

- [x] 34ba · [Keep recent events without losing the first failure](34ba-events.md) — Last 24 events, capped Unicode text, finite timing, retained first failure and failed outcome; native and inherited Chrome acceptance pass.

- [x] 34c · [Read live diagnostic context outside the GPU runtime](34c-browser.md) — Real sanitized browser/page/dimensions/density/time metadata, independent report lifetime, read-only snapshots and fresh run identity on same-tab reload verified.

- [x] 34d · [Download diagnostics through the real command line](34d-download.md) — Typed actual JSON download, ready milestone, real GPU-loss and rejected-startup reports; history/state/counters/late-input and download URL release checks pass.

- [x] 34e · [Check the bounded shape of a diagnostic report](34e-schema.md) — Typed metadata/events/failure validation; native, WebAssembly, GPU and Chrome checks pass.

- [x] 34ea · [Admit only supported bounded saved JSON](34ea-decode.md) — Byte/schema admission; native, WebAssembly, GPU and Chrome checks pass.

- [x] 34eb · [Choose a recent failure without blaming active tabs](34eb-recency.md) — Actual failure/heartbeat chronology and eligibility; native, WebAssembly, GPU and Chrome checks pass.

- [x] 34ec · [Prove saved-run exclusions before adopting storage](34ec-proof.md) — Invalid clocks, healthy states and chronology acceptance; native, WebAssembly, GPU and Chrome checks pass.

- [x] 34f · [Read previous reports from real browser storage](34f-storage.md) — Stable identity, bounded real storage reads and Date parsing; builds and actual Chrome checks pass.

- [x] 34fa · [Retain only three diagnostic runs](34fa-retain.md) — Actual writes, metadata-only ownership and denied-storage behavior; builds and actual Chrome checks pass.

- [x] 34fb · [Retrieve saved failure evidence through the command line](34fb-store.md) — Live persistence and typed previous-report download; builds and actual Chrome checks pass.

- [x] 34fc · [Preserve unsupported telemetry while pruning](34fc-retention.md) — Separate raw retention from strict adoption; native, WebAssembly, rendering and actual Chrome checks pass.

- [x] 34g · [Refresh heartbeat without rewriting failure evidence](34g-heartbeat.md) — Actual stored heartbeat, unchanged scene/history and retained first failure after real GPU loss; native, WebAssembly, GPU and Chrome pass.

- [x] 34ga · [Own the periodic diagnostic heartbeat](34ga-timer.md) — Actual interval registration, independent post-loss metadata, exact cancellation/replacement and denied-scheduler startup/download; native, WebAssembly, GPU and Chrome pass.

- [x] 34gb · [Mark a final healthy run closed](34gb-close.md) — Stored healthy/failure outcomes and late-milestone preservation; native, WebAssembly, GPU and Chrome pass.

- [x] 34gba · [Own diagnostic page transitions](34gba-lifecycle.md) — Cached pause/resume, independent post-loss metadata, guarded final cleanup and denied-registration acceptance; all build/render/Chrome checks pass.

- [x] 34gbb · [Keep each heartbeat pause reason separate](34gbb-suspension.md) — Independent hidden/frozen/cached reasons and permanent final closure; 135 native checks, WebAssembly, GPU and retained Chrome route pass. Typing 22–43 minutes.

- [x] 34gbba · [Connect visibility and freezing to heartbeat scheduling](34gbba-browser.md) — Genuine tab visibility, actual Chrome freezing, hidden startup, duplicate resumption, post-loss metadata and final/denied-owner guards pass. Typing 20–40 minutes.

- [x] 34gbc · [Give pending startup a revocable ticket](34gbc-authority.md) — Shared revocation, permanent closure and independent allocations; 138 native tests, WebAssembly, GPU and retained Chrome route pass. Typing 14–27 minutes.

- [x] 34gbca · [Refuse GPU results after final page exit](34gbca-startup.md) — Actual delayed adapter/device success and rejection, cached return, absent metadata, denied mandatory binding and complete lifecycle route pass. Typing 15–30 minutes.

- [x] 34gc · [Bound browser failure messages](34gc-messages.md) — Unicode/detail/source budgets, positions and readable fallbacks; 141 native tests, WebAssembly, GPU and Chrome pass. Typing 19–38 minutes.

- [x] 34gca · [Observe uncaught browser failures](34gca-errors.md) — Real uncaught errors and rejected promises, automatic/manual downloads, bounded safe reasons, healthy camera, post-loss metadata and partial-registration cleanup pass. Typing 18–35 minutes.

- [x] 34gd · [Retain adapter identity in the report](34gd-adapter.md) — Bounded strings, legacy decoding, strict fields, event rotation and atomic refusal; 144 native tests, WebAssembly, GPU and Chrome pass. Typing 14–28 minutes; copy the check file.

- [x] 34gda · [Read the viewer’s adapter identity](34gda-browser.md) — Actual drawing-device identity, one request, descriptor restoration, unavailable metadata, replacement ownership and the complete preceding browser route pass. Typing 29–57 minutes.

- [x] 34gdb · [Retain measured loading phases](34gdb-phases.md) — Strict phases, legacy decoding, 256 entries, encoded byte retention and first failure preservation; 148 native tests and Chrome pass. Typing 27–54 minutes.

- [x] 34gdc · [Measure GPU startup and the first frame](34gdc-startup.md) — Actual adapter/device/renderer timings, delayed completion, exited/lost/replaced device guards and the complete preceding browser route pass. Typing 23–46 minutes.

- [ ] 34gdd · Record file loading and resource diagnostics — File read/decode/walk/upload, resource timing and live replacements; adapter and GPU startup are taught above.

- [ ] 34ge · Recover from GPU loss — Retain diagnostics, bound retries and restore normal quality after conservative recovery.

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

- [ ] 69a · Reuse display walks of congruent BReps — compare rigid-frame shape keys, replay moved/rotated copies with their own row/edge IDs and facing data, independently walk mirrors and changed geometry, release per-document recordings, and prove circle-chord and rendered-shape equivalence. Split into manageable runnable endpoints when authored.

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
