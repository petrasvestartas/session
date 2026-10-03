# Build a viewer you can explain

Start with a picture you understand, then grow it into the viewer we already use. You will type the implementation yourself. We will revisit earlier code when a new responsibility appears, explain the change and keep a working checkpoint.

**Current release: 79 cumulative lessons, about 127–231 active study hours.** Installation is extra. These estimates include reading, typing and experiments. Check the [release evidence](journey/release.md) before starting. The complete feature course is still being written.

The [complete draft lesson checklist](journey/roadmap.md) has **150 proposed slots: 79 current checkpoints and 71 later slots planned**. This is a teaching plan, not a fixed final count or percentage of engineering work.

**Command-line revision:** [Use the viewer’s real command dock](journey/command-line.md) from the early lessons. Features are exercised by typing commands; later lessons retain canvas picking and gestures.

The white canvas fills the browser window from lesson 01. The runtime shows your drawing and the command dock; lesson titles and explanations stay in this documentation.

**One-hour typing target:** the [typing-load audit](journey/typing-load.md) separates actual code changes from reading and experiments. Seventeen existing checkpoints still need splitting; working builds alone do not make those long sections finished lessons.

## Start small, keep the destination

| Lesson | Time | Working result |
| --- | --- | --- |
| [01 · A page that Rust can reach](journey/01-canvas.md) | 1–2 hours | Fill the browser window with a canvas and let Rust signal that it has started. |
| [02 · Ask the GPU to paint](journey/02-clear.md) | 2–3 hours | Paint the whole canvas white with a real GPU command. |
| [03 · Give the GPU three corners](journey/03-triangle.md) | 1–2 hours | Draw a pink triangle on the white background. |
| [03a · Draw our command line](journey/03a-panel.md) | 4–6 hours | Draw the production command panel and its Noto text over the triangle, using the same GPU. |
| [03b · Give the command line its memory](journey/03b-state.md) | 3–5 hours | The text field and history have one owner. |
| [03c · Draw completion and history](journey/03c-layout.md) | 8–12 hours | Lay out the production command dock from its model. |
| [03d · Type into the real command dock](journey/03d-input.md) | 5–8 hours | Send browser events to the dock and submit Help. |
| [04 · Make a choice change the picture](journey/04-input.md) | 1–2 hours | Use a command to switch backgrounds without changing the triangle. |
| [05 · Let Rust supply the corners](journey/05-vertices.md) | 1–2 hours | Draw a rectangle from six positions stored in a GPU buffer. |
| [06 · Share a corner between triangles](journey/06-indices.md) | 1–2 hours | Draw a diamond from four positions and six small index numbers. |
| [07 · Send one view setting to every corner](journey/07-uniforms.md) | 2–3 hours | Scale and shift the diamond without changing its stored positions. |
| [08 · Move the view, keep the geometry](journey/08-camera.md) | 2–3 hours | Pan, zoom and reset a flat view through camera state. |
| [09 · Let one matrix describe the view](journey/09-matrices.md) | 2–4 hours | Rotate the flat view using a matrix, ready for the third dimension. |
| [10 · Keep the nearest surface](journey/10-depth.md) | 2–4 hours | Draw two overlapping triangles in depth, keeping the nearer one visible even when it is drawn first. |
| [11 · Give the scene an owner](journey/11-scene.md) | 3–5 hours | Add and remove a mesh through scene data while reusing the renderer and camera. |
| [12 · Name objects without depending on their row](journey/12-identity.md) | 3–5 hours | Select and delete objects using stable identities, then highlight the selected object. |
| [13 · Ask which object is under the pointer](journey/13-picking.md) | 2–4 hours | Click a visible triangle to select its stable object ID, including after camera movement. |
| [14 · Make document changes reversible](journey/14-history.md) | 2–4 hours | Undo and redo adding or deleting an object without changing the camera or losing identity. |
| [15 · Look through a perspective camera](journey/15-perspective.md) | 4–7 hours | View the scene in perspective and select surfaces with a ray that agrees with the camera. |
| [16 · Walk around the model](journey/16-orbit.md) | 2–4 hours | Orbit and tilt a perspective camera while keeping its target in place. |
| [17 · Bring a solid into the scene](journey/17-solid.md) | 3–5 hours | Create a kernel box, convert it to display data, and add it as one undoable scene object. |
| [18 · Read the shape through light](journey/18-light.md) | 1–2 hours | Shade the box faces according to their direction, using the same mesh and renderer. |
| [19 · Give every action the same route](journey/19-actions.md) | 3–5 hours | Move document actions into a browser-independent editor while keeping picking, undo and drawing working. |
| [20 · Keep a changing window in proportion](journey/20-resize.md) | 3–5 hours | Resize the drawing buffer, depth attachment and camera together, including on dense displays. |
| [21 · Remember a press until it ends](journey/21-gestures.md) | 3–5 hours | Orbit with a right drag, pick with a left click, and stop safely when the pointer or window loses focus. |
| [22 · Keep navigation on the mouse and commands in the dock](journey/22-shortcuts.md) | 2–4 hours | Normalize wheel input and keep every keyboard feature command in the command dock. |
| [23 · Keep the document behind the picture](journey/23-import.md) | 5–8 hours | Import a real mesh-session file, keep its source identity, and undo the whole import as one action. |
| [24 · Find the whole scene](journey/24-fit.md) | 3–5 hours | Frame all current objects without rotating them or changing the document. |
| [25 · Choose how depth changes size](journey/25-projection.md) | 3–5 hours | Switch between perspective and orthographic views while keeping fitting, zoom and picking coherent. |
| [26 · Frame one object without changing its size](journey/26-selected.md) | 1–2 hours | Run Fit Selected while keeping geometry, selection and document history unchanged. |
| [27 · Give each object a placement](journey/27-placement.md) | 1–2 hours | Keep local mesh coordinates and an independent object placement. |
| [27a · Ask geometry questions in world coordinates](journey/27a-world.md) | 1–2 hours | Use placement for scene bounds, selected bounds and ray picking. |
| [27b · Apply object placement on the GPU](journey/27b-model.md) | 1–2 hours | Send a separate model matrix for each draw while retaining local vertex buffers. |
| [27c · Move a placed object with a typed offset](journey/27c-move.md) | 1–2 hours | Translate selected geometry through the existing command and history route. |
| [27d · Prove placement and history agree](journey/27d-history.md) | 1–2 hours | Check that Move changes world placement, preserves local geometry, and remains one reversible transaction. |
| [28 · Prepare a display from an owned source mesh](journey/28-record.md) | 1–2 hours | Give source geometry a shared owner and prepare its display before committing an object. |
| [28a · Retain the imported mesh behind each row](journey/28a-imported.md) | 1–2 hours | Insert imported source geometry and its prepared display together. |
| [28b · Give generated objects the same source owner](journey/28b-generated.md) | 1–2 hours | Build demo triangles and Example Box from kernel geometry; derive all displays on insertion. |
| [28c · Prove source ownership survives editing](journey/28c-ownership.md) | 1–2 hours | Connect local source geometry, display caches, provenance and document history. |
| [29 · Give each saved object a stable identity](journey/29-identity.md) | 1–2 hours | Distinguish a scene object from the original geometry it shares. |
| [29a · Write a snapshot from the editable sources](journey/29a-snapshot.md) | 1–2 hours | Serialize live source meshes and their placements without changing the editor. |
| [29b · Reopen source geometry with its placement](journey/29b-placements.md) | 1–2 hours | Validate stored placements before reconstructing source-backed objects. |
| [29c · Prove the saved document reopens faithfully](journey/29c-roundtrip.md) | 1–2 hours | Check exact local source data, attributes, identities and placement through saving and loading. |
| [29d · Download the editable document from the command line](journey/29d-save.md) | 1–2 hours | Connect Save to a browser download and reopen the actual downloaded document. |
| [30 · Report the result that actually committed](journey/30-feedback.md) | 1–2 hours | Keep failed reads and imports visible in command history without claiming success. |
| [30a · Give a pending read an explicit ticket](journey/30a-tickets.md) | 1–2 hours | Describe latest-read ownership, cancellation and one-shot completion in native Rust. |
| [30b · Cancel reads without accepting their late result](journey/30b-cancel.md) | 1–2 hours | Adopt one-shot tickets in the browser and give asynchronous results their own history entry. |
| [30c · Replace a document as one reversible change](journey/30c-replace.md) | 1–2 hours | Prepare a whole replacement before changing live rows, then commit it through existing history. |
| [30d · Choose append or replace before opening the picker](journey/30d-bridge.md) | 1–2 hours | Carry the selected Open operation through asynchronous file reading and cancel older work immediately. |
| [30e · Prove recovery at the transaction and display boundaries](journey/30e-recovery.md) | 1–2 hours | Test partial replacement failure, preserved selection/Redo and placement upload limits. |
| [31 · Keep selection out of the vertex data](journey/31-settings.md) | 1–2 hours | Put placement and selection in an explicitly packed object uniform. |
| [31a · Give immutable GPU geometry one owner](journey/31a-geometry.md) | 1–2 hours | Separate vertex/index storage from each object’s uniform and retain its CPU source. |
| [31b · Reuse uploads while their geometry is alive](journey/31b-cache.md) | 1–2 hours | Cache GPU geometry by its retained CPU allocation and release dead cache entries. |
| [31c · Update only changed object settings](journey/31c-incremental.md) | 1–2 hours | Retain GPU rows by object and geometry identity, then write only changed uniform ranges. |
| [32 · Find the owners retained by history](journey/32-history.md) | 1–2 hours | Inspect undo/redo roots and release both branches explicitly. |
| [32a · Count shared CPU displays once](journey/32a-cpu.md) | 1–2 hours | Account for active/history rows, retained sources/documents and unique display payloads. |
| [32b · Count each shared GPU buffer once](journey/32b-gpu.md) | 1–2 hours | Measure live document buffer sizes separately from cumulative allocation counters. |
| [32c · Close the document without resetting the view](journey/32c-close.md) | 1–2 hours | Drop active rows and both history branches while preserving the local ID counter and camera. |
| [32d · Close through the command line and revoke reads](journey/32d-command.md) | 1–2 hours | Wire Close into the dock and cancel pending delivery before closing the editor. |
| [32e · Prove release does not retain the old document](journey/32e-release.md) | 1–2 hours | Check imported history owners, the CPU/GPU release boundary, late read failures and reopening. |
| [32f · Keep row metadata separate from editable geometry](journey/32f-metadata.md) | 1–2 hours | Retain names, original source GUIDs and visibility/locking flags without keeping kernel owners alive. |
| [32fa · Ask whether an editable source is available](journey/32fa-access.md) | 1–2 hours | Route saving, source accounting and owner checks through borrowed source accessors. |
| [32fb · Make editable ownership a private row boundary](journey/32fb-boundary.md) | 1–2 hours | Finish migrating owner checks and prevent other modules from bypassing source availability. |
| [32fc · Record a reload version without retaining geometry](journey/32fc-version.md) | 1–2 hours | Own a small document header, a distinct import ID and an exact file fingerprint. |
| [32fd · Attach one origin to an imported document](journey/32fd-origin.md) | 1–2 hours | Share one geometry-free origin across imported rows and history. |
| [32fe · Give a reload URL an explicit owner](journey/32fe-location.md) | 1–2 hours | Retain an owned Blob URL through imports and history, then release it with its last owner. |
| [32ff · Adopt the selected file as a reloadable source](journey/32ff-bridge.md) | 1–2 hours | Transfer the accepted File through synchronous delivery and create its URL at adoption. |
| [32fg · Separate loaded and released editable ownership](journey/32fg-state.md) | 1–2 hours | Represent source residency without changing retained display, identity or placement. |
| [32fh · Protect sources that cannot be unloaded faithfully](journey/32fh-policy.md) | 1–2 hours | Require a reload location, original kernel allocation and unchanged row metadata. |
| [32fi · Unload sources across active and history roots](journey/32fi-history.md) | 1–2 hours | Release a validated whole-origin set without clearing Undo/Redo or changing placements. |
| [32fj · Prove unloading preserves placed history](journey/32fj-proof.md) | 1–2 hours | Observe imported kernel expiration while retained displays and placements survive Undo/Redo. |
| [32fk · Unload editable sources through the command line](journey/32fk-command.md) | 1–2 hours | Keep the drawing and GPU allocations while the dock unloads eligible imported sources. |
| [32fl · Protect history and future reload tickets](journey/32fl-guards.md) | 1–2 hours | Verify whole-origin protection, independent duplicate imports and checked release epochs. |
| [32g · Identify the source release a reload belongs to](journey/32g-keys.md) | 1–2 hours | Borrow the current imported release identity and epoch before asking for source data. |
| [32ga · Prepare original kernel data without rebuilding its display](journey/32ga-prepare.md) | 1–2 hours | Validate an immutable source version and restore its Session into a private reload candidate. |
| [32gb · Adopt restored source owners as one residency change](journey/32gb-adopt.md) | 1–2 hours | Validate current releases and all matching history rows before adopting any kernel candidate. |
| [32gc · Prove source restoration preserves display and history](journey/32gc-roundtrip.md) | 1–2 hours | Restore exact source coordinates and editing without reallocating retained display owners. |
| [32gd · Reject stale or inconsistent source batches atomically](journey/32gd-rejections.md) | 1–2 hours | Keep rows cold on changed versions, missing source identity and obsolete release keys. |
| [32ge · Ignore old reload results before decoding them](journey/32ge-stale.md) | 1–2 hours | Reject results for closed imports, loaded rows, duplicate keys and previous release epochs. |

The opening lessons separate browser events, the command dock, application state and GPU drawing. Before adding a camera or editable objects, follow this path without the listing: **typed command → state change → drawing commands → picture**.

Each lesson has one visible goal, a diagram, exact code changes, an experiment and a question. Do not rush through code you cannot connect to that goal. [Rust foundations](foundations.md) are available when a language idea needs more practice; you do not have to complete eight separate exercises before seeing a canvas.

## The destination

The final project must retain the current viewer’s features and crafted appearance. The small triangle is the first milestone. It is not the final product, and the later courses must continue the same handwritten project.

| Course | Destination | Availability |
| --- | --- | --- |
| 1 · A small viewer you understand | Mesh, camera, input and object identity. | Available through lit solids, orbit, pointer gestures, focused shortcuts, resizing, mesh import, scene fitting and projection choices; advanced picking still pending. |
| 2 · Geometry that reads clearly | Curves, points, CAD faces, boundaries, normals and text. | Pending conversion. |
| 3 · Documents that stay reliable | Loading, streaming, instancing, sheets and resource lifetimes. | Pending conversion. |
| 4 · Editing that can be undone | Selection, gestures, snapping, gumball, panels and transactions. | Basic selection, undo/redo and fallible transactions available; advanced tools pending. |
| 5 · The modeling toolbox | Every existing command, shape, surface, annotation and splitting tool. | Pending conversion. |
| 6 · The rendering quality we have now | Finite visibility, clipping caps, outlines, ambient occlusion and transparency. | Pending conversion. |
| 7 · Prove the complete viewer | Behavior, visual quality, responsiveness and resource checks. | Pending conversion. |

[The feature and quality contract](journey/destination.md) maps all 43 existing sections to these courses. A source inventory records all 271 current implementation/test files and the command implementations. No feature is considered delivered merely because it appears in this table.

The [complete implementation reference](reference-course.md) remains available during conversion. Its large sections have not become short lessons by being regrouped. They use a different starting point; do not jump from the current last lesson into reference section 00 and overwrite your project. The continuous bridge is still work to be done.

## Keep your place

Before an experiment, save your files. After it, write three lines: what changed; which values crossed a file boundary; what still puzzles me. You should be able to return tomorrow without reconstructing the whole lesson in your head.

[Checkpoint and recovery instructions](journey/recovery.md) preserve your own work. A corrected lesson should arrive as a focused change with a reason and a verification step, not a demand to restart. Releases keep their dependency lock and source fingerprints; the documentation labels unverified behavior.

[Prepare your computer](README.md#prepare-your-computer) · [Begin lesson 01](journey/01-canvas.md) · [Release evidence](journey/release.md)
