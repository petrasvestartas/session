# Keep each typing lesson under one hour

The complete viewer remains the destination. A working checkpoint can still be too large for a good typing lesson. The table below separates the code you must add or change from the reading and experiments around it.

This audit counts added or changed lines in each exact code edit. Unchanged context shown around an edit is excluded: keep those lines rather than retyping them. A changed line is counted in full, including indentation, comments and its line break.

For planning, the typing range assumes 10–20 five-character words per minute (50–100 characters per minute). This is a conservative estimate, not a measurement of your speed. Reading, reasoning, debugging and experiments take additional time. The existing study ranges combine those activities and must be reassessed as lessons are split.

**Acceptance target:** at most 60 minutes of estimated typing at the slower planning rate, with a complete buildable endpoint. Splits must follow a responsibility or a working behaviour, never stop midway through a Rust function. Some later checkpoints supply implementation files as well as fixtures. The typing column excludes copy-marked edits; read, trace and debug that supplied implementation separately. A low typing estimate is not a low study-time estimate.

## Current checkpoint audit

| Checkpoint | Added/changed lines | Characters | Typing estimate | Existing combined study estimate | Typing limit |
| --- | ---: | ---: | --- | --- | --- |
| [1.1.1 · A page that Rust can reach](01-canvas.md) | 49 | 1585 | 16–32 min | 1–2 h | Within planning limit |
| [1.1.2 · Paint the first GPU frame](01a-gpu.md) | 69 | 2944 | 30–59 min | 1–2 h | Within planning limit |
| [1.2.1 · Give browser presentation its own function](02-clear.md) | 55 | 2774 | 28–56 min | 1–2 h | Within planning limit |
| [1.3.1 · Give the GPU three corners](03-triangle.md) | 47 | 1866 | 19–38 min | 1–2 h | Within planning limit |
| [1.3.2 · Prepare the command fonts and painter](03a-fonts.md) | 82 | 2963 | 30–60 min | 1–2 h | Within planning limit |
| [1.3.3 · Paint command text over the scene](03a-paint.md) | 44 | 2428 | 25–49 min | 1–2 h | Within planning limit |
| [1.3.4 · Lay out the command field](03a-panel.md) | 76 | 2963 | 30–60 min | 1–2 h | Within planning limit |
| [1.3.5 · Give the command field its memory](03b-memory.md) | 42 | 2084 | 21–42 min | 1–2 h | Within planning limit |
| [1.3.6 · Prepare the dock completion helpers](03b-state.md) | 84 | 2845 | 29–57 min | 1–2 h | Within planning limit |
| [1.3.7 · Draw retained command history](03c-history.md) | 44 | 2146 | 22–43 min | 0.5–1 h | Within planning limit |
| [1.3.8 · Type directly into the command field](03ca-typing.md) | 42 | 1950 | 20–39 min | 0.5–1 h | Within planning limit |
| [1.3.9 · Submit command text once](03cb-submit.md) | 30 | 1303 | 14–27 min | 0.5–0.75 h | Within planning limit |
| [1.3.10 · Edit the command text](03cc-edit.md) | 26 | 1389 | 14–28 min | 0.5–0.75 h | Within planning limit |
| [1.3.11 · Recognize Help](03cd-vocabulary.md) | 57 | 2216 | 23–45 min | 0.5–1 h | Within planning limit |
| [1.3.12 · Complete a command name](03ce-complete.md) | 50 | 2173 | 22–44 min | 0.5–1 h | Within planning limit |
| [1.3.13 · Accept or cancel a completion](03cf-accept.md) | 73 | 2991 | 30–60 min | 0.75–1.25 h | Within planning limit |
| [1.3.14 · Read editing keys before the field](03cg-keys.md) | 71 | 2298 | 23–46 min | 0.5–1.25 h | Within planning limit |
| [1.3.15 · Show matching commands](03ch-popup.md) | 54 | 2625 | 27–53 min | 0.75–1.25 h | Within planning limit |
| [1.3.16 · Browse matching names](03ci-browse.md) | 62 | 2247 | 23–45 min | 0.5–1 h | Within planning limit |
| [1.3.17 · Prepare the dock before painting](03cj-prepare.md) | 18 | 1087 | 11–22 min | 0.5–0.75 h | Within planning limit |
| [1.3.18 · Remember where the dock is drawn](03ck-rects.md) | 41 | 1994 | 20–40 min | 0.5–1 h | Within planning limit |
| [1.3.19 · Click the command dock](03ck-pointer.md) | 37 | 2528 | 26–51 min | 0.5–1.25 h | Within planning limit |
| [1.3.20 · Scroll through command names](03cl-wheel.md) | 56 | 2455 | 25–50 min | 0.5–1.25 h | Within planning limit |
| [1.3.21 · Share the production dock layout](03c-layout.md) | 88 | 2877 | 29–58 min | 0.75–1.25 h | Within planning limit |
| [1.3.22 · Align and measure the command row](03co-align.md) | 6 | 416 | 5–9 min | 0.25–0.5 h | Within planning limit |
| [1.3.23 · Reserve command-row space for active tool values](03cp-options.md) | 82 | 2974 | 30–60 min | 1–1.5 h | Within planning limit |
| [1.3.24 · Return accepted application commands](03cn-handoff.md) | 60 | 2826 | 29–57 min | 0.75–1.25 h | Within planning limit |
| [1.3.25 · Hand commands to the application](03d-input.md) | 60 | 2166 | 22–44 min | 0.5–1 h | Within planning limit |
| [1.4.1 · Make a choice change the picture](04-input.md) | 38 | 1188 | 12–24 min | 1–2 h | Within planning limit |
| [2.1.1 · Let Rust supply the corners](05-vertices.md) | 26 | 1115 | 12–23 min | 1–2 h | Within planning limit |
| [2.2.1 · Share a corner between triangles](06-indices.md) | 14 | 737 | 8–15 min | 1–2 h | Within planning limit |
| [2.3.1 · Send one view setting to every corner](07-uniforms.md) | 35 | 1583 | 16–32 min | 2–3 h | Within planning limit |
| [2.4.1 · Move the view, keep the geometry](08-camera.md) | 93 | 2482 | 25–50 min | 2–3 h | Within planning limit |
| [2.5.1 · Let one matrix describe the view](09-matrices.md) | 30 | 1130 | 12–23 min | 2–4 h | Within planning limit |
| [2.6.1 · Keep the nearest surface](10-depth.md) | 63 | 2798 | 28–56 min | 2–4 h | Within planning limit |
| [2.6.2 · Give geometry a validated CPU owner](10a-mesh.md) | 76 | 2331 | 24–47 min | 0.75–1.25 h | Within planning limit |
| [2.7.1 · Draw the scene through GPU mesh owners](11-scene.md) | 60 | 2298 | 23–46 min | 0.75–1.25 h | Within planning limit |
| [2.7.2 · Name objects independently of their rows](11a-identity.md) | 84 | 2814 | 29–57 min | 0.75–1.5 h | Within planning limit |
| [2.8.1 · Display selection by object identity](12-identity.md) | 33 | 1551 | 16–32 min | 0.5–1 h | Within planning limit |
| [2.8.2 · Convert screen positions back to the scene](12a-coordinates.md) | 27 | 948 | 10–19 min | 0.5–0.75 h | Within planning limit |
| [2.8.3 · Find the nearest triangle at a scene point](12b-picking.md) | 81 | 2757 | 28–56 min | 0.75–1.5 h | Within planning limit |
| [2.9.1 · Select the visible object with a mouse click](13-picking.md) | 27 | 1308 | 14–27 min | 0.5–1 h | Within planning limit |
| [2.9.2 · Retain reversible scene snapshots](13a-history.md) | 94 | 2719 | 28–55 min | 0.75–1.5 h | Within planning limit |
| [2.10.1 · Run Undo and Redo from the command line](14-history.md) | 18 | 757 | 8–16 min | 0.25–0.5 h | Within planning limit |
| [2.10.2 · Draw through a perspective camera](14a-perspective.md) | 38 | 1984 | 20–40 min | 0.75–1.25 h | Within planning limit |
| [2.10.3 · Turn a screen point into a bounded ray](14b-ray.md) | 32 | 1367 | 14–28 min | 0.5–0.75 h | Within planning limit |
| [2.11.1 · Pick the nearest surface through the view](15-perspective.md) | 54 | 2918 | 30–59 min | 1–1.5 h | Within planning limit |
| [2.12.1 · Walk around the model](16-orbit.md) | 61 | 2785 | 28–56 min | 2–4 h | Within planning limit |
| [3.1.1 · Bring a solid into the scene](17-solid.md) | 67 | 2667 | 27–54 min | 3–5 h | Within planning limit |
| [3.2.1 · Read the shape through light](18-light.md) | 9 | 529 | 6–11 min | 1–2 h | Within planning limit |
| [3.2.2 · Give application state one owner](18a-editor.md) | 59 | 1328 | 14–27 min | 0.5–0.75 h | Within planning limit |
| [3.2.3 · Apply document and view actions in Rust](18b-actions.md) | 58 | 2650 | 27–53 min | 1–1.5 h | Within planning limit |
| [3.3.1 · Route browser input through the editor](19-actions.md) | 66 | 2826 | 29–57 min | 1–1.5 h | Within planning limit |
| [3.3.2 · Measure a safe drawing size](19a-viewport.md) | 52 | 1838 | 19–37 min | 0.75–1.25 h | Within planning limit |
| [3.4.1 · Resize canvas, depth and camera together](20-resize.md) | 83 | 2825 | 29–57 min | 1–1.5 h | Within planning limit |
| [3.4.2 · Remember the pointer that starts a drag](20a-press.md) | 46 | 1336 | 14–27 min | 0.75–1.25 h | Within planning limit |
| [3.4.3 · Finish or cancel a drag](20b-release.md) | 22 | 667 | 7–14 min | 0.75–1.25 h | Within planning limit |
| [3.4.4 · Convert pointer motion to editor actions](20c-motion.md) | 21 | 663 | 7–14 min | 0.75–1.25 h | Within planning limit |
| [3.5.1 · Connect captured pointers to the editor](21-gestures.md) | 60 | 2253 | 23–46 min | 1–1.5 h | Within planning limit |
| [3.5.2 · Convert wheel units to camera zoom](21a-wheel.md) | 18 | 493 | 5–10 min | 0.75–1.5 h | Within planning limit |
| [3.6.1 · Deliver wheel input without keyboard feature shortcuts](22-shortcuts.md) | 41 | 1685 | 17–34 min | 0.5–1 h | Within planning limit |
| [3.6.2 · Validate a raw mesh record](22a-records.md) | 61 | 2521 | 26–51 min | 0.75–1.25 h | Within planning limit |
| [3.6.3 · Validate session identity and build a sample file](22b-session.md) | 61 | 2789 | 28–56 min | 0.75–1.25 h | Within planning limit |
| [3.6.4 · Decode and prepare the whole import](22c-decode.md) | 17 | 655 | 7–14 min | 0.75–1.25 h | Within planning limit |
| [3.6.5 · Commit an import as one undoable action](22d-import.md) | 17 | 701 | 8–15 min | 0.75–1.25 h | Within planning limit |
| [3.6.6 · Prepare asynchronous file delivery](22e-file-reader.md) | 42 | 1770 | 18–36 min | 0.75–1.25 h | Within planning limit |
| [3.7.1 · Open a file through the command line](23-import.md) | 44 | 2156 | 22–44 min | 0.75–1.25 h | Within planning limit |
| [3.7.2 · Measure the displayed scene bounds](23a-bounds.md) | 43 | 1268 | 13–26 min | 0.75–1.25 h | Within planning limit |
| [3.7.3 · Fit the camera around the bounds](23b-fit-camera.md) | 17 | 791 | 8–16 min | 0.75–1.25 h | Within planning limit |
| [3.8.1 · Run Fit through the command line](24-fit.md) | 8 | 277 | 3–6 min | 0.75–1.25 h | Within planning limit |
| [3.8.2 · Build perspective and orthographic camera matrices](24a-projection.md) | 22 | 776 | 8–16 min | 0.75–1.25 h | Within planning limit |
| [3.8.3 · Fit and change projection through the editor](24b-projection-actions.md) | 7 | 374 | 4–8 min | 0.75–1.25 h | Within planning limit |
| [3.9.1 · Run the named projection commands](25-projection.md) | 22 | 899 | 9–18 min | 0.25–0.5 h | Within planning limit |
| [3.10.1 · Frame one object without changing its size](26-selected.md) | 25 | 988 | 10–20 min | 1–2 h | Within planning limit |
| [3.10.2 · Fit and step history from the keyboard](26a-keys.md) | 44 | 1890 | 19–38 min | 0.5–1 h | Within planning limit |
| [3.11.1 · Give each object a placement](27-placement.md) | 23 | 936 | 10–19 min | 1–2 h | Within planning limit |
| [3.11.2 · Ask geometry questions in world coordinates](27a-world.md) | 10 | 699 | 7–14 min | 1–2 h | Within planning limit |
| [3.11.3 · Apply object placement on the GPU](27b-model.md) | 30 | 1611 | 17–33 min | 1–2 h | Within planning limit |
| [3.11.4 · Move a placed object with a typed offset](27c-move.md) | 44 | 2066 | 21–42 min | 1–2 h | Within planning limit |
| [3.11.5 · Prove placement and history agree](27d-history.md) | 2 | 29 | 1–1 min | 1–2 h | Within planning limit |
| [3.12.1 · Prepare a display from an owned source mesh](28-record.md) | 24 | 703 | 8–15 min | 1–2 h | Within planning limit |
| [3.12.2 · Retain the imported mesh behind each row](28a-imported.md) | 16 | 765 | 8–16 min | 1–2 h | Within planning limit |
| [3.12.3 · Give generated objects the same source owner](28b-generated.md) | 23 | 1432 | 15–29 min | 1–2 h | Within planning limit |
| [3.12.4 · Prove source ownership survives editing](28c-ownership.md) | 2 | 31 | 1–1 min | 1–2 h | Within planning limit |
| [3.13.1 · Give each saved object a stable identity](29-identity.md) | 9 | 416 | 5–9 min | 1–2 h | Within planning limit |
| [3.13.2 · Write a snapshot from the editable sources](29a-snapshot.md) | 31 | 1288 | 13–26 min | 1–2 h | Within planning limit |
| [3.13.3 · Reopen source geometry with its placement](29b-placements.md) | 24 | 1153 | 12–24 min | 1–2 h | Within planning limit |
| [3.13.4 · Prove the saved document reopens faithfully](29c-roundtrip.md) | 2 | 39 | 1–1 min | 1–2 h | Within planning limit |
| [3.13.5 · Download the editable document from the command line](29d-save.md) | 46 | 2339 | 24–47 min | 1–2 h | Within planning limit |
| [3.14.1 · Report the result that actually committed](30-feedback.md) | 29 | 1498 | 15–30 min | 1–2 h | Within planning limit |
| [3.14.2 · Give a pending read an explicit ticket](30a-tickets.md) | 64 | 1728 | 18–35 min | 1–2 h | Within planning limit |
| [3.14.3 · Cancel reads without accepting their late result](30b-cancel.md) | 31 | 1546 | 16–31 min | 1–2 h | Within planning limit |
| [3.14.4 · Replace a document as one reversible change](30c-replace.md) | 13 | 442 | 5–9 min | 1–2 h | Within planning limit |
| [3.14.5 · Choose append or replace before opening the picker](30d-bridge.md) | 43 | 2645 | 27–53 min | 1–2 h | Within planning limit |
| [3.14.6 · Prove recovery at the transaction and display boundaries](30e-recovery.md) | 20 | 978 | 10–20 min | 1–2 h | Within planning limit |
| [3.15.1 · Keep selection out of the vertex data](31-settings.md) | 19 | 732 | 8–15 min | 1–2 h | Within planning limit |
| [3.15.2 · Give immutable GPU geometry one owner](31a-geometry.md) | 48 | 1861 | 19–38 min | 1–2 h | Within planning limit |
| [3.15.3 · Reuse uploads while their geometry is alive](31b-cache.md) | 51 | 1991 | 20–40 min | 1–2 h | Within planning limit |
| [3.15.4 · Update only changed object settings](31c-incremental.md) | 41 | 1942 | 20–39 min | 1–2 h | Within planning limit |
| [3.16.1 · Find the owners retained by history](32-history.md) | 38 | 1455 | 15–30 min | 1–2 h | Within planning limit |
| [3.16.2 · Count shared CPU displays once](32a-cpu.md) | 72 | 2979 | 30–60 min | 1–2 h | Within planning limit |
| [3.16.3 · Count each shared GPU buffer once](32b-gpu.md) | 19 | 700 | 7–14 min | 1–2 h | Within planning limit |
| [3.16.4 · Close the document without resetting the view](32c-close.md) | 41 | 1996 | 20–40 min | 1–2 h | Within planning limit |
| [3.16.5 · Close through the command line and revoke reads](32d-command.md) | 10 | 449 | 5–9 min | 1–2 h | Within planning limit |
| [3.16.6 · Prove release does not retain the old document](32e-release.md) | 23 | 1390 | 14–28 min | 1–2 h | Within planning limit |
| [3.16.7 · Keep row metadata separate from editable geometry](32f-metadata.md) | 62 | 2785 | 28–56 min | 1–2 h | Within planning limit |
| [3.16.8 · Ask whether an editable source is available](32fa-access.md) | 8 | 457 | 5–10 min | 1–2 h | Within planning limit |
| [3.16.9 · Make editable ownership a private row boundary](32fb-boundary.md) | 7 | 487 | 5–10 min | 1–2 h | Within planning limit |
| [3.16.10 · Record a reload version without retaining geometry](32fc-version.md) | 52 | 2009 | 21–41 min | 1–2 h | Within planning limit |
| [3.16.11 · Attach one origin to an imported document](32fd-origin.md) | 9 | 587 | 6–12 min | 1–2 h | Within planning limit |
| [3.16.12 · Give a reload URL an explicit owner](32fe-location.md) | 37 | 1612 | 17–33 min | 1–2 h | Within planning limit |
| [3.16.13 · Adopt the selected file as a reloadable source](32ff-bridge.md) | 31 | 2306 | 24–47 min | 1–2 h | Within planning limit |
| [3.16.14 · Separate loaded and released editable ownership](32fg-state.md) | 32 | 1712 | 18–35 min | 1–2 h | Within planning limit |
| [3.16.15 · Protect sources that cannot be unloaded faithfully](32fh-policy.md) | 35 | 2044 | 21–41 min | 1–2 h | Within planning limit |
| [3.16.16 · Unload sources across active and history roots](32fi-history.md) | 32 | 1590 | 16–32 min | 1–2 h | Within planning limit |
| [3.16.17 · Prove unloading preserves placed history](32fj-proof.md) | 2 | 31 | 1–1 min | 1–2 h | Within planning limit |
| [3.16.18 · Unload editable sources through the command line](32fk-command.md) | 13 | 767 | 8–16 min | 1–2 h | Within planning limit |
| [3.16.19 · Protect history and future reload tickets](32fl-guards.md) | 42 | 2498 | 25–50 min | 1–2 h | Within planning limit |
| [3.16.20 · Identify the source release a reload belongs to](32g-keys.md) | 47 | 1837 | 19–37 min | 1–2 h | Within planning limit |
| [3.16.21 · Prepare original kernel data without rebuilding its display](32ga-prepare.md) | 36 | 1529 | 16–31 min | 1–2 h | Within planning limit |
| [3.16.22 · Adopt restored source owners as one residency change](32gb-adopt.md) | 30 | 1568 | 16–32 min | 1–2 h | Within planning limit |
| [3.16.23 · Prove source restoration preserves display and history](32gc-roundtrip.md) | 2 | 34 | 1–1 min | 1–2 h | Within planning limit |
| [3.16.24 · Reject stale or inconsistent source batches atomically](32gd-rejections.md) | 0 | 0 | 0–0 min | 1–2 h | Within planning limit |
| [3.16.25 · Ignore old reload results before decoding them](32ge-stale.md) | 0 | 0 | 0–0 min | 1–2 h | Within planning limit |
| [3.16.26 · Give each source request its own owner](32gf-request.md) | 45 | 1447 | 15–29 min | 1–2 h | Within planning limit |
| [3.16.27 · Prove cancelled work releases its source owners](32gfa-ownership.md) | 54 | 2544 | 26–51 min | 1–2 h | Within planning limit |
| [3.16.28 · Read a source response within its byte limit](32gg-fetch.md) | 40 | 2331 | 24–47 min | 1–2 h | Within planning limit |
| [3.16.29 · Pair reload ownership with a browser abort controller](32gga-flight.md) | 35 | 1164 | 12–24 min | 1–2 h | Within planning limit |
| [3.16.30 · Deliver only the current completed reload batch](32ggb-delivery.md) | 30 | 1381 | 14–28 min | 1–2 h | Within planning limit |
| [3.16.31 · Restore editable sources through the command line](32gh-command.md) | 30 | 1729 | 18–35 min | 1–2 h | Within planning limit |
| [3.16.32 · Cancel reloads when their document context changes](32gha-cancel.md) | 3 | 275 | 3–6 min | 1–2 h | Within planning limit |
| [3.16.33 · Capture the requested edit before waiting](32gi-capture.md) | 72 | 2957 | 30–60 min | 1–2 h | Within planning limit |
| [3.16.34 · Load only the sources the requested edit needs](32gia-scope.md) | 13 | 531 | 6–11 min | 1–2 h | Within planning limit |
| [3.16.35 · Move the original target from its current placement](32gib-move.md) | 13 | 721 | 8–15 min | 1–2 h | Within planning limit |
| [3.16.36 · Delete the original target while keeping later selection](32giba-delete.md) | 14 | 649 | 7–13 min | 1–2 h | Within planning limit |
| [3.16.37 · Route captured Move, Delete and Save results](32gibb-reply.md) | 51 | 2495 | 25–50 min | 1–2 h | Within planning limit |
| [3.16.38 · Own the intent with its pending source ticket](32gic-owner.md) | 24 | 861 | 9–18 min | 1–2 h | Within planning limit |
| [3.16.39 · Pair captured intent with complete source bodies](32gica-reply.md) | 19 | 616 | 7–13 min | 1–2 h | Within planning limit |
| [3.16.40 · Carry captured intent through browser completion](32gicb-bridge.md) | 16 | 1120 | 12–23 min | 1–2 h | Within planning limit |
| [3.16.41 · Validate restoration before replaying the command](32gid-complete.md) | 11 | 486 | 5–10 min | 1–2 h | Within planning limit |
| [3.16.42 · Deliver restored edit and Save results to the dock](32gie-response.md) | 23 | 1316 | 14–27 min | 1–2 h | Within planning limit |
| [3.16.43 · Automatically restore sources for Move, Delete and Save](32gif-auto.md) | 36 | 2412 | 25–49 min | 1–2 h | Within planning limit |
| [3.16.44 · Prove restored Save keeps the source doubles](32gj-precision.md) | 10 | 349 | 4–7 min | 1–2 h | Within planning limit |
| [3.16.45 · Prove failed restoration cannot partly commit](32gja-failures.md) | 2 | 39 | 1–1 min | 1–2 h | Within planning limit |
| [4.1.1 · Own browser listeners instead of forgetting callbacks](33-owner.md) | 50 | 1978 | 20–40 min | 1–2 h | Within planning limit |
| [4.1.2 · Dispose the viewer without leaving pending work alive](33a-runtime.md) | 49 | 1847 | 19–37 min | 1–2 h | Within planning limit |
| [4.1.3 · Keep a cached viewer ready for Back navigation](33b-cache.md) | 3 | 202 | 3–5 min | 1–2 h | Within planning limit |
| [4.2.1 · Keep the first GPU failure with its device](34-fault.md) | 20 | 537 | 6–11 min | 1–2 h | Within planning limit |
| [4.2.2 · Stop the viewer when its GPU device fails](34a-stop.md) | 28 | 1468 | 15–30 min | 1–2 h | Within planning limit |
| [4.2.3 · Describe a viewer run without keeping its document](34b-report.md) | 38 | 1031 | 11–21 min | 1–2 h | Within planning limit |
| [4.2.4 · Keep recent events without losing the first failure](34ba-events.md) | 30 | 1340 | 14–27 min | 1–2 h | Within planning limit |
| [4.2.5 · Read live diagnostic context outside the GPU runtime](34c-browser.md) | 51 | 2341 | 24–47 min | 1–2 h | Within planning limit |
| [4.2.6 · Download diagnostics through the real command line](34d-download.md) | 26 | 1233 | 13–25 min | 1–2 h | Within planning limit |
| [4.2.7 · Check the bounded shape of a report](34e-schema.md) | 19 | 1244 | 13–25 min | 1–2 h | Within planning limit |
| [4.2.8 · Admit only supported bounded saved JSON](34ea-decode.md) | 16 | 678 | 7–14 min | 1–2 h | Within planning limit |
| [4.2.9 · Choose a recent failure without blaming active tabs](34eb-recency.md) | 18 | 843 | 9–17 min | 1–2 h | Within planning limit |
| [4.2.10 · Prove saved-run exclusions before adopting storage](34ec-proof.md) | 0 | 0 | 0–0 min | 1–2 h | Within planning limit |
| [4.2.11 · Read previous reports from real browser storage](34f-storage.md) | 51 | 1996 | 20–40 min | 1–2 h | Within planning limit |
| [4.2.12 · Retain only three diagnostic runs](34fa-retain.md) | 30 | 1729 | 18–35 min | 1–2 h | Within planning limit |
| [4.2.13 · Retrieve saved failure evidence through the command line](34fb-store.md) | 45 | 2366 | 24–48 min | 1–2 h | Within planning limit |
| [4.2.14 · Preserve unsupported telemetry while pruning](34fc-retention.md) | 20 | 1053 | 11–22 min | 1–2 h | Within planning limit |
| [4.2.15 · Refresh heartbeat without rewriting failure evidence](34g-heartbeat.md) | 18 | 711 | 8–15 min | 1–2 h | Within planning limit |
| [4.2.16 · Own the periodic diagnostic heartbeat](34ga-timer.md) | 41 | 1518 | 16–31 min | 1–2 h | Within planning limit |
| [4.2.17 · Mark a final healthy run closed](34gb-close.md) | 21 | 858 | 9–18 min | 1–2 h | Within planning limit |
| [4.2.18 · Own diagnostic page transitions](34gba-lifecycle.md) | 52 | 2485 | 25–50 min | 1–2 h | Within planning limit |
| [4.2.19 · Keep each heartbeat pause reason separate](34gbb-suspension.md) | 32 | 911 | 10–19 min | 0.75–1.25 h | Within planning limit |
| [4.2.20 · Connect visibility and freezing to heartbeat scheduling](34gbba-browser.md) | 35 | 1943 | 20–39 min | 0.75–1.25 h | Within planning limit |
| [4.2.21 · Give pending startup a revocable ticket](34gbc-authority.md) | 19 | 478 | 5–10 min | 0.5–1 h | Within planning limit |
| [4.2.22 · Refuse GPU results after final page exit](34gbca-startup.md) | 38 | 1491 | 15–30 min | 0.75–1.25 h | Within planning limit |
| [4.2.23 · Bound browser failure messages](34gc-messages.md) | 18 | 834 | 9–17 min | 0.5–1 h | Within planning limit |
| [4.2.24 · Observe uncaught browser failures](34gca-errors.md) | 38 | 1713 | 18–35 min | 0.75–1.25 h | Within planning limit |
| [4.2.25 · Retain adapter identity in the report](34gd-adapter.md) | 34 | 1384 | 14–28 min | 0.75–1.25 h | Within planning limit |
| [4.2.26 · Read the viewer’s adapter identity](34gda-browser.md) | 66 | 2801 | 29–57 min | 0.75–1.25 h | Within planning limit |
| [4.2.27 · Retain measured loading phases](34gdb-phases.md) | 61 | 2671 | 27–54 min | 0.75–1.25 h | Within planning limit |
| [4.2.28 · Measure GPU startup and the first frame](34gdc-startup.md) | 43 | 2257 | 23–46 min | 0.75–1.25 h | Within planning limit |
| [4.2.29 · Measure the selected file read](34gdd-read.md) | 5 | 332 | 4–7 min | 0.25–0.5 h | Within planning limit |
| [4.2.30 · Measure a completed CPU operation](34gde-measure.md) | 33 | 1075 | 11–22 min | 0.5–0.75 h | Within planning limit |
| [4.2.31 · Measure each document preparation phase](34gdea-import.md) | 46 | 2146 | 22–43 min | 0.75–1.25 h | Within planning limit |
| [4.2.32 · Measure upload preparation and GPU completion](34gdf-upload.md) | 28 | 1282 | 13–26 min | 0.5–1 h | Within planning limit |
| [4.2.33 · Measure accepted source fetches](34gdg-network.md) | 41 | 2118 | 22–43 min | 0.75–1.25 h | Within planning limit |
| [4.2.34 · Record accepted file revisions](34gdga-revision.md) | 34 | 1738 | 18–35 min | 0.5–1 h | Within planning limit |
| [4.2.35 · Keep the document outside the GPU runtime](34ge-owner.md) | 24 | 1291 | 13–26 min | 0.5–1 h | Within planning limit |
| [4.2.36 · Permit one conservative recovery reload](34gea-policy.md) | 15 | 447 | 5–9 min | 0.25–0.5 h | Within planning limit |
| [4.2.37 · Recover once at reduced canvas density](34geb-reload.md) | 40 | 2000 | 20–40 min | 0.75–1.25 h | Within planning limit |
| [5.1.1 · Prepare a stroke without changing its source](35-stroke.md) | 33 | 1339 | 14–27 min | 0.5–1 h | Within planning limit |
| [5.1.2 · Retain line owners in document history](35a-document.md) | 52 | 2128 | 22–43 min | 0.75–1.25 h | Within planning limit |
| [5.1.3 · Extrude a stroke in screen pixels](35b-extrude.md) | 44 | 1971 | 20–40 min | 0.75–1.25 h | Within planning limit |
| [5.1.4 · Create the shared stroke pipeline](35c-lane.md) | 42 | 2400 | 24–48 min | 0.75–1.25 h | Within planning limit |
| [5.1.5 · Upload changed stroke instances only](35ca-sync.md) | 37 | 1789 | 18–36 min | 0.75–1.25 h | Within planning limit |
| [5.1.6 · Connect the stroke lane to the document renderer](35cb-render.md) | 47 | 2409 | 25–49 min | 0.75–1.25 h | Within planning limit |
| [5.1.7 · Insert a line through the real command dock](35d-input.md) | 15 | 664 | 7–14 min | 0.75–1.25 h | Within planning limit |
| [5.2.1 · Retain connected source geometry](36-chain.md) | 53 | 2190 | 22–44 min | 0.75–1.25 h | Within planning limit |
| [5.2.2 · Link adjacent spans and headed ends](36a-links.md) | 27 | 1200 | 12–24 min | 0.75–1.25 h | Within planning limit |
| [5.2.3 · Project neighbours for bounded stroke joins](36b-join.md) | 23 | 1225 | 13–25 min | 0.75–1.25 h | Within planning limit |
| [5.2.4 · Extrude the joined stroke body](36ba-body.md) | 34 | 2045 | 21–41 min | 0.75–1.25 h | Within planning limit |
| [5.2.5 · Extrude free-end arrowheads](36bb-heads.md) | 33 | 2017 | 21–41 min | 0.75–1.25 h | Within planning limit |
| [5.2.6 · Share instance drawing with connected strokes](36c-lane.md) | 42 | 2456 | 25–50 min | 0.75–1.25 h | Within planning limit |
| [5.2.7 · Retain connected paths in document history](36d-document.md) | 41 | 1903 | 20–39 min | 0.75–1.25 h | Within planning limit |
| [5.2.8 · Place original connected coordinates and count shared owners](36db-placement.md) | 38 | 1782 | 18–36 min | 0.75–1.25 h | Within planning limit |
| [5.2.9 · Draw connected document rows](36da-render.md) | 20 | 1194 | 12–24 min | 0.75–1.25 h | Within planning limit |
| [5.2.10 · Insert connected paths and arrow examples](36e-input.md) | 26 | 1391 | 14–28 min | 0.75–1.25 h | Within planning limit |
| [5.3.1 · Prepare an original point for a screen marker](37-point.md) | 33 | 1223 | 13–25 min | 0.75–1.25 h | Within planning limit |
| [5.3.2 · Extrude a circular marker in screen pixels](37a-disc.md) | 28 | 1402 | 15–29 min | 0.75–1.25 h | Within planning limit |
| [5.3.3 · Draw markers with the shared instance lane](37b-lane.md) | 15 | 765 | 8–16 min | 0.75–1.25 h | Within planning limit |
| [5.3.4 · Retain original point rows and placement](37c-document.md) | 44 | 2384 | 24–48 min | 0.75–1.25 h | Within planning limit |
| [5.3.5 · Draw original point rows and count their resources](37d-render.md) | 29 | 1630 | 17–33 min | 0.75–1.25 h | Within planning limit |
| [5.3.6 · Insert original points through the real dock](37da-input.md) | 15 | 665 | 7–14 min | 0.75–1.25 h | Within planning limit |
| [5.3.7 · Identify controls on the original curve owner](37e-controls.md) | 40 | 1821 | 19–37 min | 0.75–1.25 h | Within planning limit |
| [5.4.1 · Validate an original NURBS curve before sampling](38-layout.md) | 31 | 1662 | 17–34 min | 0.75–1.25 h | Within planning limit |
| [5.4.2 · Sample a validated curve without replacing its source](38a-samples.md) | 37 | 1927 | 20–39 min | 0.75–1.25 h | Within planning limit |
| [5.4.3 · Retain original curves beside their display samples](38b-source.md) | 51 | 2663 | 27–54 min | 0.75–1.25 h | Within planning limit |
| [5.4.4 · Draw controls with their original curve parent and index](38c-controls.md) | 28 | 1632 | 17–33 min | 0.75–1.25 h | Within planning limit |
| [5.4.5 · Insert an original curve and inspect its controls](38d-input.md) | 22 | 1100 | 11–22 min | 0.75–1.25 h | Within planning limit |
| [5.5.1 · Resolve original visibility for every drawing kind](39-visible.md) | 37 | 2222 | 23–45 min | 0.75–1.25 h | Within planning limit |
| [5.5.2 · Share surface depth across all ink lanes](39a-depth.md) | 14 | 699 | 7–14 min | 0.75–1.25 h | Within planning limit |
| [5.6.1 · Carry the original mesh colour mode into display data](40-colour.md) | 22 | 898 | 9–18 min | 0.75–1.25 h | Within planning limit |
| [5.6.2 · Compare original colour modes through actual input](40a-input.md) | 26 | 1380 | 14–28 min | 0.75–1.25 h | Within planning limit |
| [5.6.3 · Draw meshes beyond the sixteen-bit index limit](40b-indices.md) | 10 | 240 | 3–5 min | 0.75–1.25 h | Within planning limit |
| [5.7.1 · Retain checked source normals beside display positions](41-normals.md) | 26 | 1298 | 13–26 min | 0.75–1.25 h | Within planning limit |
| [5.7.2 · Transform normals with nonuniform surface placement](41a-placement.md) | 17 | 865 | 9–18 min | 0.75–1.25 h | Within planning limit |
| [5.7.3 · Draw smooth normals and sharp surface faces](41b-drawing.md) | 40 | 2382 | 24–48 min | 0.75–1.25 h | Within planning limit |
| [5.7.4 · Reveal oriented world normals in the renderer](41c-view.md) | 39 | 2124 | 22–43 min | 0.75–1.25 h | Within planning limit |
| [5.7.5 · Inspect smooth normals and face winding through commands](41ca-input.md) | 40 | 1980 | 20–40 min | 0.75–1.25 h | Within planning limit |
| [5.7.6 · Bound the ground grid and its camera depth range](41d-grid.md) | 58 | 2604 | 27–53 min | 0.75–1.25 h | Within planning limit |
| [5.7.7 · Draw a depth-correct grid with shared stroke buffers](41e-drawing.md) | 34 | 1997 | 20–40 min | 0.75–1.25 h | Within planning limit |
| [5.7.8 · Inspect the ground grid through actual commands](41f-input.md) | 22 | 1122 | 12–23 min | 0.75–1.25 h | Within planning limit |
| [6.1.1 · Bound the cursor image and decode a hit](42-window.md) | 61 | 2954 | 30–60 min | 0.5–1 h | Within planning limit |
| [6.1.2 · Keep exact object IDs in shared GPU settings](42a-id.md) | 42 | 2555 | 26–52 min | 0.5–1 h | Within planning limit |
| [6.1.3 · Share point and stroke coverage with picking](42b-ink.md) | 68 | 2964 | 30–60 min | 0.5–1 h | Within planning limit |
| [6.1.4 · Pick connected paths and arrowheads with their drawn shape](42c-path.md) | 45 | 2239 | 23–45 min | 0.5–1 h | Within planning limit |
| [6.1.5 · Attach original owners to visible ink instances](42d-owners.md) | 39 | 2807 | 29–57 min | 0.5–1 h | Within planning limit |
| [6.1.6 · Draw integer ink with the shared geometry buffers](42e-pass.md) | 28 | 2691 | 27–54 min | 0.5–1 h | Within planning limit |
| [6.1.7 · Draw depth-tested mesh owners into an integer attachment](42f-mesh.md) | 28 | 2227 | 23–45 min | 0.5–1 h | Within planning limit |
| [6.1.8 · Own a small ID image, depth target and readback buffer](42g-image.md) | 29 | 1603 | 17–33 min | 0.5–1 h | Within planning limit |
| [6.1.9 · Bound asynchronous picking ownership](42h-owner.md) | 58 | 2439 | 25–49 min | 0.5–1 h | Within planning limit |
| [6.1.10 · Record the ID draw and aligned image copy](42i-copy.md) | 25 | 2054 | 21–42 min | 0.5–1 h | Within planning limit |
| [6.1.11 · Map the integer hit without blocking browser input](42j-read.md) | 43 | 2506 | 26–51 min | 0.5–1 h | Within planning limit |
| [6.1.12 · Select original rows through actual GPU canvas clicks](42k-input.md) | 56 | 2733 | 28–55 min | 0.5–1 h | Within planning limit |
| [6.2.1 · Name the request and view that produced a hit](43-ticket.md) | 48 | 1907 | 20–39 min | 0.5–1 h | Within planning limit |
| [6.2.2 · Carry checked tickets through GPU mapping](43a-flight.md) | 26 | 1956 | 20–40 min | 0.5–1 h | Within planning limit |
| [6.2.3 · Reject results from an old view or interrupted input](43b-view.md) | 31 | 2411 | 25–49 min | 0.5–1 h | Within planning limit |
| [6.3.1 · Preserve source faces and edges](44-topology.md) | 47 | 2371 | 24–48 min | 0.5–1 h | Within planning limit |
| [6.3.2 · Tag the kind of a picked part](44a-tags.md) | 55 | 2003 | 21–41 min | 0.5–1 h | Within planning limit |
| [6.3.3 · Map visible ink back to original sources](44b-sources.md) | 36 | 2493 | 25–50 min | 0.5–1 h | Within planning limit |
| [6.3.4 · Validate a part against the current source](44c-resolve.md) | 41 | 2399 | 24–48 min | 0.5–1 h | Within planning limit |
| [6.3.5 · Reuse visible geometry for face identities](44d-pulling.md) | 39 | 2005 | 21–41 min | 0.5–1 h | Within planning limit |
| [6.3.6 · Create the integer source-face pipeline](44e-pipeline.md) | 34 | 2299 | 23–46 min | 0.5–1 h | Within planning limit |
| [6.3.7 · Upload only original-face metadata](44f-metadata.md) | 29 | 2364 | 24–48 min | 0.5–1 h | Within planning limit |
| [6.3.8 · Draw original boundary edges consistently](44g-edge-ink.md) | 32 | 2399 | 24–48 min | 0.5–1 h | Within planning limit |
| [6.3.9 · Pick source parts with visible coverage](44h-mode-gpu.md) | 27 | 2434 | 25–49 min | 0.5–1 h | Within planning limit |
| [6.3.10 · Highlight original edges and controls](44i-ink-highlight.md) | 21 | 1132 | 12–23 min | 0.5–1 h | Within planning limit |
| [6.3.11 · Build highlights from original ink targets](44ia-original-ink.md) | 31 | 2357 | 24–48 min | 0.5–1 h | Within planning limit |
| [6.3.12 · Highlight one original face without recolouring its parent](44j-face-highlight.md) | 31 | 2332 | 24–47 min | 0.5–1 h | Within planning limit |
| [6.3.13 · Keep part selection outside document history](44k-selection.md) | 40 | 2573 | 26–52 min | 0.5–1 h | Within planning limit |
| [6.3.14 · Select and inspect original faces, edges and controls](44l-input.md) | 26 | 1944 | 20–39 min | 0.5–1 h | Within planning limit |
| [6.4.1 · Preserve unique selections in pick order](45-order.md) | 42 | 2202 | 23–45 min | 0.5–1 h | Within planning limit |
| [6.4.2 · Commit mixed selection edits atomically](45a-batch.md) | 45 | 2743 | 28–55 min | 0.5–1 h | Within planning limit |
| [6.4.3 · Connect multi-hit state and highlight every selected row](45b-membership.md) | 41 | 2933 | 30–59 min | 0.5–1 h | Within planning limit |
| [6.4.4 · Cycle every eligible original geometry owner](45bb-cycle.md) | 11 | 707 | 8–15 min | 0.5–1 h | Within planning limit |
| [6.4.5 · Collect multiple objects with Shift-click and commands](45c-input.md) | 16 | 1443 | 15–29 min | 0.5–1 h | Within planning limit |
| [7.1.1 · Give active tools ownership of the command panel](46-tool-panel.md) | 10 | 920 | 10–19 min | 0.5–1 h | Within planning limit |
| [7.2.1 · Separate browser requests from editor actions](47-registry.md) | 29 | 878 | 9–18 min | 0.5–1 h | Within planning limit |
| [7.2.2 · Register the existing editor command factories](47a-actions.md) | 46 | 2511 | 26–51 min | 0.5–1 h | Within planning limit |
| [7.2.3 · Parse the longest command and validate its arguments](47b-parse.md) | 37 | 1844 | 19–37 min | 0.5–1 h | Within planning limit |
| [7.2.4 · Route the command field through the typed registry](47c-route.md) | 35 | 1543 | 16–31 min | 0.5–1 h | Within planning limit |
| [7.3.1 · Resolve drawing points on the construction plane](48-coordinates.md) | 25 | 1157 | 12–24 min | 0.5–1 h | Within planning limit |
| [7.3.2 · Keep unfinished point tools outside the document](48a-draft.md) | 49 | 2387 | 24–48 min | 0.5–1 h | Within planning limit |
| [7.3.3 · Draw a temporary rubber band and cursor](48b-preview.md) | 35 | 2108 | 22–43 min | 0.5–1 h | Within planning limit |
| [7.3.4 · Commit completed tools in one Undo transaction](48c-commit.md) | 32 | 1962 | 20–40 min | 0.5–1 h | Within planning limit |
| [7.3.5 · Draw points and lines through the shared command dock](48d-input.md) | 37 | 2872 | 29–58 min | 0.5–1 h | Within planning limit |
| [7.4.1 · Separate object and grid snapping](49-settings.md) | 46 | 2129 | 22–43 min | 0.5–1 h | Within planning limit |
| [7.4.2 · Keep snap math in world and plane coordinates](49a-math.md) | 45 | 2644 | 27–53 min | 0.5–1 h | Within planning limit |
| [7.4.3 · Collect targets from original placed geometry](49b-sources.md) | 43 | 2888 | 29–58 min | 0.5–1 h | Within planning limit |
| [7.4.4 · Search nearby screen cells](49c-bins.md) | 25 | 1361 | 14–28 min | 0.5–1 h | Within planning limit |
| [7.4.5 · Choose snap kinds before grid fallback](49d-index.md) | 45 | 2820 | 29–57 min | 0.5–1 h | Within planning limit |
| [7.4.6 · Invalidate snap caches after source edits](49e-cache.md) | 26 | 1560 | 16–32 min | 0.5–1 h | Within planning limit |
| [7.4.7 · Preserve drafts while snapping](49ea-state.md) | 23 | 1677 | 17–34 min | 0.5–1 h | Within planning limit |
| [7.4.8 · Snap actual drawing input through the command dock](49f-input.md) | 26 | 2338 | 24–47 min | 0.5–1 h | Within planning limit |
| [7.5.1 · Address placements across original geometry lanes](50-placements.md) | 14 | 811 | 9–17 min | 0.5–1 h | Within planning limit |
| [7.5.2 · Keep temporary placements outside the document](50a-preview.md) | 37 | 2076 | 21–42 min | 0.5–1 h | Within planning limit |
| [7.5.3 · Present temporary edits through a separate scene view](50b-display.md) | 23 | 1905 | 20–39 min | 0.5–1 h | Within planning limit |
| [7.5.4 · Preview Move from a base point to a target](50c-move.md) | 33 | 2714 | 28–55 min | 0.5–1 h | Within planning limit |
| [7.5.5 · Cancel previews without consuming document history](50d-ownership.md) | 12 | 1167 | 12–24 min | 0.5–1 h | Within planning limit |
| [7.6.1 · Name move, rotation and scale handles](51-handles.md) | 101 | 2470 | 25–50 min | 0.5–1 h | Within planning limit |
| [7.6.2 · Measure handle rays in original world coordinates](51a-rays.md) | 93 | 2354 | 24–48 min | 0.5–1 h | Within planning limit |
| [7.6.3 · Keep handle state at one original-world pivot](51b-pivot.md) | 49 | 1179 | 12–24 min | 0.5–1 h | Within planning limit |
| [7.6.4 · Pick handles in a stable priority order](51c-hit.md) | 59 | 1738 | 18–35 min | 0.5–1 h | Within planning limit |
| [7.6.5 · Capture the handle’s original grab](51d-grab.md) | 48 | 1693 | 17–34 min | 0.5–1 h | Within planning limit |
| [7.6.6 · Derive handle transforms and quarter-turn snapping](51e-transform.md) | 61 | 2594 | 26–52 min | 0.5–1 h | Within planning limit |
| [7.6.7 · Pack and sample local handle vertices](51ez-pack-and-sample-local-handle-vertices.md) | 62 | 1542 | 16–31 min | 0.5–1 h | Within planning limit |
| [7.6.8 · Build local arrows balls and rotation arcs](51f-mesh.md) | 48 | 1773 | 18–36 min | 0.5–1 h | Within planning limit |
| [7.6.9 · Keep handles constant in CSS pixels](51g-layout.md) | 49 | 2800 | 28–56 min | 0.5–1 h | Within planning limit |
| [7.6.10 · Draw a local handle tile and its canvas rectangle](51h-shader.md) | 61 | 2060 | 21–42 min | 0.5–1 h | Within planning limit |
| [7.6.11 · Bind the widget’s own sampled image](51i-image.md) | 9 | 657 | 7–14 min | 0.5–1 h | Within planning limit |
| [7.6.12 · Separate handle depth from scene depth](51j-pipelines.md) | 22 | 2636 | 27–53 min | 0.5–1 h | Within planning limit |
| [7.6.13 · Own color, depth and resolve textures together](51k-tile.md) | 17 | 1454 | 15–30 min | 0.5–1 h | Within planning limit |
| [7.6.14 · Upload handle geometry once](51l-upload.md) | 31 | 1872 | 19–38 min | 0.5–1 h | Within planning limit |
| [7.6.15 · Write only current handle placement](51m-prepare.md) | 9 | 680 | 7–14 min | 0.5–1 h | Within planning limit |
| [7.6.16 · Composite handles after source geometry](51n-draw.md) | 11 | 1420 | 15–29 min | 0.5–1 h | Within planning limit |
| [7.6.17 · Register the widget outside source picking](51o-renderer.md) | 16 | 880 | 9–18 min | 0.5–1 h | Within planning limit |
| [7.6.18 · Locate original mesh parts and curve controls](51p-part-points.md) | 38 | 1970 | 20–40 min | 0.5–1 h | Within planning limit |
| [7.6.19 · Deform selected original points through their placement](51q-deform.md) | 33 | 2618 | 27–53 min | 0.5–1 h | Within planning limit |
| [7.6.20 · Keep part deformation outside saved geometry](51r-part-preview.md) | 32 | 2022 | 21–41 min | 0.5–1 h | Within planning limit |
| [7.6.21 · Share one handle interface across whole and part edits](51s-edit-owner.md) | 17 | 1170 | 12–24 min | 0.5–1 h | Within planning limit |
| [7.6.22 · Keep pointer ownership through click or drag](51t-capture.md) | 31 | 1893 | 19–38 min | 0.5–1 h | Within planning limit |
| [7.6.23 · Place handles on the current editable selection](51u-selection.md) | 27 | 1614 | 17–33 min | 0.5–1 h | Within planning limit |
| [7.6.24 · Capture a valid handle without editing the document](51v-press.md) | 15 | 1811 | 19–37 min | 0.5–1 h | Within planning limit |
| [7.6.25 · Preview captured motion and commit one release](51w-release.md) | 24 | 2543 | 26–51 min | 0.5–1 h | Within planning limit |
| [7.6.26 · Own one handle’s number-field state](51x-number-model.md) | 15 | 732 | 8–15 min | 0.5–1 h | Within planning limit |
| [7.6.27 · Draw the number field beside its handle](51y-number-field.md) | 70 | 2664 | 27–54 min | 0.5–1 h | Within planning limit |
| [7.6.28 · Apply precise typed handle values once](51z-number-submit.md) | 27 | 2712 | 28–55 min | 0.5–1 h | Within planning limit |
| [7.6.29 · Route field input through the shared canvas panel](51za-panel.md) | 21 | 2098 | 21–42 min | 0.5–1 h | Within planning limit |
| [7.6.30 · Update selected handles with the actual camera](51zb-presentation.md) | 11 | 1061 | 11–22 min | 0.5–1 h | Within planning limit |
| [7.6.31 · Capture actual browser handle drags](51zc-pointer.md) | 31 | 2075 | 21–42 min | 0.5–1 h | Within planning limit |
| [7.6.32 · Restore previews when input ownership ends](51zd-interruption.md) | 20 | 2087 | 21–42 min | 0.5–1 h | Within planning limit |
| [7.6.33 · Count temporary source owners without saving them](51ze-ledger.md) | 4 | 297 | 3–6 min | 0.5–1 h | Within planning limit |
| [7.6.34 · Verify complete handle editing and input](51zf-handles.md) | 5 | 71 | 1–2 min | 0.5–1 h | Within planning limit |
| [8.1.1 · Validate and place a nested source tree](52-tree.md) | 29 | 1397 | 14–28 min | 0.5–1 h | Within planning limit |
| [8.1.2 · Keep lightweight tree records outside kernel ownership](52a-records.md) | 7 | 382 | 4–8 min | 0.5–1 h | Within planning limit |
| [8.1.3 · Index descendants and read-only attribute groups](52b-index.md) | 24 | 2107 | 22–43 min | 0.5–1 h | Within planning limit |
| [8.1.4 · Import nested source identities and precise placements](52c-load.md) | 12 | 856 | 9–18 min | 0.5–1 h | Within planning limit |
| [8.1.5 · Carry parent-owned descendants in one preview](52d-preview.md) | 21 | 1623 | 17–33 min | 0.5–1 h | Within planning limit |
| [8.1.6 · Commit parent moves and deletions as one transaction](52e-edits.md) | 15 | 1593 | 16–32 min | 0.5–1 h | Within planning limit |
| [8.1.7 · Save nested rows and their local placements](52f-save.md) | 20 | 1703 | 18–35 min | 0.5–1 h | Within planning limit |
| [8.1.8 · Expose stable tree rows across geometry lanes](52g-rows.md) | 24 | 1870 | 19–38 min | 0.5–1 h | Within planning limit |
| [8.1.9 · Keep expansion and paging as presentation state](52h-expansion.md) | 19 | 985 | 10–20 min | 0.5–1 h | Within planning limit |
| [8.1.10 · Draw indented original-source tree controls](52i-panel.md) | 25 | 2501 | 26–51 min | 0.5–1 h | Within planning limit |
| [8.1.11 · Connect tree controls before canvas picking](52j-input.md) | 19 | 2383 | 24–48 min | 0.5–1 h | Within planning limit |
| [8.1.12 · Exercise nested source ownership and exact undo](52k-tree.md) | 8 | 292 | 3–6 min | 0.5–1 h | Within planning limit |
| [8.2.1 · Separate layer presentation from original sources](53-flags.md) | 25 | 1586 | 16–32 min | 0.5–1 h | Within planning limit |
| [8.2.2 · Index live identities and initialise view flags](53a-ids.md) | 19 | 1491 | 15–30 min | 0.5–1 h | Within planning limit |
| [8.2.3 · Update surface colour and pickability in small GPU records](53b-uniform.md) | 28 | 2094 | 21–42 min | 0.5–1 h | Within planning limit |
| [8.2.4 · Resolve live visibility and selection eligibility](53bb-eligibility.md) | 13 | 1256 | 13–26 min | 0.5–1 h | Within planning limit |
| [8.2.5 · Use current visibility and colours in every derived pass](53c-presentation.md) | 17 | 2368 | 24–48 min | 0.5–1 h | Within planning limit |
| [8.2.6 · Connect Hide Show and canvas Delete shortcuts](53d-hide-show.md) | 11 | 884 | 9–18 min | 0.5–1 h | Within planning limit |
| [8.2.7 · Apply eye lock and independent colour properties](53e-properties.md) | 35 | 2517 | 26–51 min | 0.5–1 h | Within planning limit |
| [8.2.8 · Keep tree identities stable across edits and saves](53f-portable.md) | 10 | 901 | 10–19 min | 0.5–1 h | Within planning limit |
| [8.2.9 · Represent group edits with temporary branches](53g-branches.md) | 28 | 1257 | 13–26 min | 0.5–1 h | Within planning limit |
| [8.2.10 · Validate current layers and protected ancestors](53h-layer-targets.md) | 39 | 2327 | 24–47 min | 0.5–1 h | Within planning limit |
| [8.2.11 · Copy row identities while sharing source allocations](53i-copy-records.md) | 21 | 1797 | 18–36 min | 0.5–1 h | Within planning limit |
| [8.2.12 · Move and copy objects between layer trees](53j-transfers.md) | 35 | 2982 | 30–60 min | 0.5–1 h | Within planning limit |
| [8.2.13 · Create and rename uniquely named layers](53k-new-rename.md) | 21 | 1840 | 19–37 min | 0.5–1 h | Within planning limit |
| [8.2.14 · Delete and duplicate complete layer contents](53l-delete-duplicate.md) | 18 | 1478 | 15–30 min | 0.5–1 h | Within planning limit |
| [8.2.15 · Commit group edits through one history boundary](53m-actions.md) | 16 | 922 | 10–19 min | 0.5–1 h | Within planning limit |
| [8.2.16 · Create geometry on the current layer or Created](53n-routing.md) | 20 | 2053 | 21–42 min | 0.5–1 h | Within planning limit |
| [8.2.17 · Supply nine colour swatches and RGB sliders](53o-palette.md) | 15 | 1257 | 13–26 min | 0.5–1 h | Within planning limit |
| [8.2.18 · Resolve layer presentation for real tree controls](53p-panel-state.md) | 8 | 1804 | 19–37 min | 0.5–1 h | Within planning limit |
| [8.2.19 · Draw eye lock and independent colour menus](53q-properties-ui.md) | 18 | 2148 | 22–43 min | 0.5–1 h | Within planning limit |
| [8.2.20 · Own rename text and populated deletion confirmation](53r-name.md) | 17 | 1572 | 16–32 min | 0.5–1 h | Within planning limit |
| [8.2.21 · Connect group menus to the wider tree panel](53s-tree-controls.md) | 22 | 2732 | 28–55 min | 0.5–1 h | Within planning limit |
| [8.2.22 · Route layer controls before handles and canvas input](53t-input.md) | 13 | 2002 | 21–41 min | 0.5–1 h | Within planning limit |
| [8.2.23 · Exercise layers shortcuts colours and exact source ownership](53u-layers.md) | 12 | 265 | 3–6 min | 0.5–1 h | Within planning limit |
| [8.3.1 · Navigate and fit inside the free canvas](54-camera.md) | 35 | 1954 | 20–40 min | 0.5–1 h | Within planning limit |
| [8.3.2 · Route camera changes without consuming document history](54aa-view-actions.md) | 7 | 1473 | 15–30 min | 0.5–1 h | Within planning limit |
| [8.3.3 · Apply visible selection and modifier precedence](54a-selection.md) | 21 | 1511 | 16–31 min | 0.5–1 h | Within planning limit |
| [8.3.4 · Keep touch handles reachable without enlarging them](54b-finger.md) | 13 | 1856 | 19–38 min | 0.5–1 h | Within planning limit |
| [8.3.5 · Distinguish window and crossing rectangles](54c-rectangle.md) | 35 | 1679 | 17–34 min | 0.5–1 h | Within planning limit |
| [8.3.6 · Select original projected geometry in one source pass](54d-samples.md) | 23 | 2104 | 22–43 min | 0.5–1 h | Within planning limit |
| [8.3.7 · Freeze a released drag until its pick answers](54e-drag.md) | 39 | 2738 | 28–55 min | 0.5–1 h | Within planning limit |
| [8.3.8 · Own drag previews outside document history](54f-drag-state.md) | 18 | 1628 | 17–33 min | 0.5–1 h | Within planning limit |
| [8.3.9 · Apply held released and cancelled drag outcomes](54g-drag-actions.md) | 16 | 2120 | 22–43 min | 0.5–1 h | Within planning limit |
| [8.3.10 · Capture mouse gestures in CSS pixels](54h-mouse.md) | 35 | 2932 | 30–59 min | 0.5–1 h | Within planning limit |
| [8.3.11 · Orbit with one finger and pan pinch with two](54i-touch.md) | 32 | 2242 | 23–45 min | 0.5–1 h | Within planning limit |
| [8.3.12 · Finish taps double taps and touch cancellation](54j-taps.md) | 9 | 760 | 8–16 min | 0.5–1 h | Within planning limit |
| [8.3.13 · Draw a thin selection rectangle above the canvas](54k-overlay.md) | 26 | 2171 | 22–44 min | 0.5–1 h | Within planning limit |
| [8.3.14 · Route mouse touch and wheel after panel input](54l-navigation.md) | 28 | 2599 | 26–52 min | 0.5–1 h | Within planning limit |
| [8.3.15 · Retain the press pick through a quick release](54m-pick-release.md) | 24 | 2272 | 23–46 min | 0.5–1 h | Within planning limit |
| [8.3.16 · Cancel owned gestures and preserve normal capture release](54n-cancellation.md) | 11 | 1492 | 15–30 min | 0.5–1 h | Within planning limit |
| [8.3.17 · Expose drag ownership and centre the available drawing](54o-frame.md) | 4 | 632 | 7–13 min | 0.5–1 h | Within planning limit |
| [8.3.18 · Exercise current navigation selection and delayed answers](54p-navigation.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [9.1.1 · Validate original surface control storage](55-validation.md) | 29 | 2241 | 23–45 min | 0.5–1 h | Within planning limit |
| [9.1.2 · Keep UV coordinates beside original world samples](55a-grid.md) | 26 | 1786 | 18–36 min | 0.5–1 h | Within planning limit |
| [9.1.3 · Use analytic normals and finite singular-point fallbacks](55b-normals.md) | 12 | 872 | 9–18 min | 0.5–1 h | Within planning limit |
| [9.1.4 · Prepare derived surface display without losing its source](55c-display.md) | 22 | 2348 | 24–47 min | 0.5–1 h | Within planning limit |
| [9.1.5 · Attach original surface ownership to existing scene rows](55d-source-row.md) | 10 | 886 | 9–18 min | 0.5–1 h | Within planning limit |
| [9.1.6 · Resolve original surface face and domain edges](55e-identities.md) | 10 | 1257 | 13–26 min | 0.5–1 h | Within planning limit |
| [9.1.7 · Account for shared source and UV cache owners](55f-ownership.md) | 13 | 1615 | 17–33 min | 0.5–1 h | Within planning limit |
| [9.1.8 · Draw and pick the original parametric roof](55g-surface.md) | 3 | 240 | 3–5 min | 0.5–1 h | Within planning limit |
| [9.1.9 · Read original edge-use samples](55za-read-original-edge-use-samples.md) | 91 | 2104 | 22–43 min | 0.5–1 h | Within planning limit |
| [9.1.10 · Follow original isoparametric boundaries](55zb-follow-original-isoparametric-boundaries.md) | 75 | 1827 | 19–37 min | 0.5–1 h | Within planning limit |
| [9.1.11 · Follow original constrained trim labels](55zc-follow-original-constrained-trim-labels.md) | 74 | 2291 | 23–46 min | 0.5–1 h | Within planning limit |
| [9.2.1 · Connect original CAD edge chains](56-chains.md) | 49 | 1347 | 14–27 min | 0.5–1 h | Within planning limit |
| [9.2.2 · Mesh incident faces with shared boundary constraints](56a-shared.md) | 26 | 1525 | 16–31 min | 0.5–1 h | Within planning limit |
| [9.2.3 · Prove straight and circular shared boundaries](56b-boundaries.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [9.3.1 · Keep the trimmed face connected to its surface](57-parameters.md) | 17 | 1198 | 12–24 min | 0.5–1 h | Within planning limit |
| [9.3.2 · Prove trimmed curved faces and open holes](57a-trims.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [9.4.1 · Keep both parameter-space uses of a seam](58-uses.md) | 14 | 931 | 10–19 min | 0.5–1 h | Within planning limit |
| [9.4.2 · Prove seam continuity and suppress false open borders](58a-periodic.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [9.4.3 · Match original face edge directions](58za-match-original-face-edge-directions.md) | 84 | 2192 | 22–44 min | 0.5–1 h | Within planning limit |
| [9.5.1 · Orient connected original face groups](59-orientation.md) | 75 | 1767 | 18–36 min | 0.5–1 h | Within planning limit |
| [9.5.2 · Map each display triangle to its original CAD face](59a-face-data.md) | 23 | 2221 | 23–45 min | 0.5–1 h | Within planning limit |
| [9.5.3 · Retain source UVs and draw only original CAD boundaries](59b-display.md) | 27 | 2072 | 21–42 min | 0.5–1 h | Within planning limit |
| [9.5.4 · Attach original BRep ownership to existing rows](59c-source-row.md) | 9 | 928 | 10–19 min | 0.5–1 h | Within planning limit |
| [9.5.5 · Pick and highlight an original CAD border](59d-identities.md) | 9 | 1193 | 12–24 min | 0.5–1 h | Within planning limit |
| [9.5.6 · Inspect shared CAD owners and compact cache counts](59e-cache.md) | 12 | 1018 | 11–21 min | 0.5–1 h | Within planning limit |
| [9.5.7 · Draw the original bore and prove its empty hole](59f-cad.md) | 3 | 224 | 3–5 min | 0.5–1 h | Within planning limit |
| [10.1.1 · Pin scene-text shaping and supply the viewer fonts](60-fonts.md) | 5 | 175 | 2–4 min | 0.5–1 h | Within planning limit |
| [10.1.2 · Keep source words metrics and placement together](60a-labels.md) | 79 | 1915 | 20–39 min | 0.5–1 h | Within planning limit |
| [10.1.3 · Load explicit fonts and inspect missing coverage](60b-coverage.md) | 35 | 1111 | 12–23 min | 0.5–1 h | Within planning limit |
| [10.1.4 · Validate a complete label replacement before changing it](60ba-validation.md) | 75 | 2251 | 23–46 min | 0.5–1 h | Within planning limit |
| [10.1.5 · Shape words into glyph clusters and baselines](60bb-shaping.md) | 21 | 653 | 7–14 min | 0.5–1 h | Within planning limit |
| [10.1.6 · Own the shaped runs with their font revisions](60c-document.md) | 34 | 608 | 7–13 min | 0.5–1 h | Within planning limit |
| [10.1.7 · Reshape only when the source layout changes](60d-reuse.md) | 49 | 1672 | 17–34 min | 0.5–1 h | Within planning limit |
| [10.1.8 · Replace usable fonts without corrupting current text](60e-font-reload.md) | 36 | 1296 | 13–26 min | 0.5–1 h | Within planning limit |
| [10.1.9 · Inspect glyph origins clusters and multiline metrics](60f-diagnostics.md) | 47 | 1435 | 15–29 min | 0.5–1 h | Within planning limit |
| [10.1.10 · Prove the current fonts shaping and reuse rules](60g-text-shaping.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [10.2.1 · Own glyph coverage in a shared GPU atlas](61-atlas.md) | 26 | 1019 | 11–21 min | 0.5–1 h | Within planning limit |
| [10.2.2 · Prepare DPI-scaled glyph bounds and cache unchanged frames](61a-coverage.md) | 15 | 1921 | 20–39 min | 0.5–1 h | Within planning limit |
| [10.2.3 · Supply a real-font CAD text specimen](61b-specimen.md) | 5 | 289 | 3–6 min | 0.5–1 h | Within planning limit |
| [10.2.4 · Draw lazy text coverage after the scene](61c-renderer.md) | 21 | 1845 | 19–37 min | 0.5–1 h | Within planning limit |
| [10.2.5 · Draw the specimen and prove real coverage reuse](61d-glyphs.md) | 6 | 615 | 7–13 min | 0.5–1 h | Within planning limit |
| [10.3.1 · Own original scene words and font metrics](62-source.md) | 19 | 1244 | 13–25 min | 0.5–1 h | Within planning limit |
| [10.3.2 · Transform the label frame without changing its source](62a-placements.md) | 21 | 2100 | 21–42 min | 0.5–1 h | Within planning limit |
| [10.3.3 · Measure the world-sized label and its padded plate](62b-bounds.md) | 9 | 897 | 9–18 min | 0.5–1 h | Within planning limit |
| [10.3.4 · Register text rows and release them with the scene](62c-rows.md) | 13 | 2158 | 22–44 min | 0.5–1 h | Within planning limit |
| [10.3.5 · Include label owners in selection layers and history](62d-identity.md) | 19 | 2597 | 26–52 min | 0.5–1 h | Within planning limit |
| [10.3.6 · Select a label through its projected plate](62e-rectangles.md) | 26 | 2848 | 29–57 min | 0.5–1 h | Within planning limit |
| [10.3.7 · Supply the text buffer and target adapters](62f-gpu-support.md) | 24 | 2461 | 25–50 min | 0.5–1 h | Within planning limit |
| [10.3.8 · Keep plane clipping correct in a small pick window](62g-plane-shader.md) | 74 | 2767 | 28–56 min | 0.5–1 h | Within planning limit |
| [10.3.9 · Draw the current rounded nameplate and its original ID](62h-plate-shader.md) | 58 | 2032 | 21–41 min | 0.5–1 h | Within planning limit |
| [10.3.10 · Keep CSS pixels separate from framebuffer pixels](62i-coverage.md) | 62 | 1782 | 18–36 min | 0.5–1 h | Within planning limit |
| [10.3.11 · Project an original label into framebuffer coordinates](62ia-project-an-original-label-into-framebuffer-coordinates.md) | 90 | 2707 | 28–55 min | 0.5–1 h | Within planning limit |
| [10.3.12 · Give nameplate vertices one GPU owner](62ib-give-nameplate-vertices-one-gpu-owner.md) | 19 | 488 | 5–10 min | 0.5–1 h | Within planning limit |
| [10.3.13 · Build nameplate color and object-ID pipelines](62ic-build-nameplate-color-and-object-id-pipelines.md) | 55 | 2433 | 25–49 min | 0.5–1 h | Within planning limit |
| [10.3.14 · Own and release nameplate draw resources](62id-own-and-release-nameplate-draw-resources.md) | 61 | 1652 | 17–34 min | 0.5–1 h | Within planning limit |
| [10.3.15 · Upload nameplate rectangles in one physical coordinate system](62ie-upload-nameplate-rectangles-in-one-physical-coordinate-system.md) | 66 | 2332 | 24–47 min | 0.5–1 h | Within planning limit |
| [10.3.16 · Place the plate around the shaped label](62if-place-the-plate-around-the-shaped-label.md) | 86 | 2389 | 24–48 min | 0.5–1 h | Within planning limit |
| [10.3.17 · Give world text a bounded coverage cache](62ig-give-world-text-a-bounded-coverage-cache.md) | 64 | 1818 | 19–37 min | 0.5–1 h | Within planning limit |
| [10.3.18 · Choose bounded glyph raster resolution](62ih-choose-bounded-glyph-raster-resolution.md) | 78 | 1756 | 18–36 min | 0.5–1 h | Within planning limit |
| [10.3.19 · Rasterize the shaped run into owned coverage bytes](62ii-rasterize-the-shaped-run-into-owned-coverage-bytes.md) | 79 | 2853 | 29–58 min | 0.5–1 h | Within planning limit |
| [10.3.20 · Upload and bind one world-text coverage texture](62ij-upload-and-bind-one-world-text-coverage-texture.md) | 56 | 1523 | 16–31 min | 0.5–1 h | Within planning limit |
| [10.3.21 · Build the world-plane text quad](62ik-build-the-world-plane-text-quad.md) | 95 | 2428 | 25–49 min | 0.5–1 h | Within planning limit |
| [10.3.22 · Build world-text color and ID pipelines](62il-build-world-text-color-and-id-pipelines.md) | 28 | 2136 | 22–43 min | 0.5–1 h | Within planning limit |
| [10.3.23 · Own the world-text buffer and pipeline lifetime](62im-own-the-world-text-buffer-and-pipeline-lifetime.md) | 74 | 2284 | 23–46 min | 0.5–1 h | Within planning limit |
| [10.3.24 · Rebuild only invalid world-text cache entries](62in-rebuild-only-invalid-world-text-cache-entries.md) | 67 | 1930 | 20–39 min | 0.5–1 h | Within planning limit |
| [10.3.25 · Prepare and draw the retained world-text planes](62io-prepare-and-draw-the-retained-world-text-planes.md) | 90 | 2538 | 26–51 min | 0.5–1 h | Within planning limit |
| [10.3.26 · Collect anchored and overlay glyph work separately](62ip-collect-anchored-and-overlay-glyph-work-separately.md) | 83 | 2391 | 24–48 min | 0.5–1 h | Within planning limit |
| [10.3.27 · Give the text lane its retained resource owners](62iq-give-the-text-lane-its-retained-resource-owners.md) | 55 | 1437 | 15–29 min | 0.5–1 h | Within planning limit |
| [10.3.28 · Construct and rebuild the text lane once](62ir-construct-and-rebuild-the-text-lane-once.md) | 65 | 2087 | 21–42 min | 0.5–1 h | Within planning limit |
| [10.3.29 · Reset the text lane and record scoped preparation costs](62is-reset-the-text-lane-and-record-scoped-preparation-costs.md) | 60 | 2274 | 23–46 min | 0.5–1 h | Within planning limit |
| [10.3.30 · Prepare glyphs only when the witness changes](62it-prepare-glyphs-only-when-the-witness-changes.md) | 78 | 2416 | 25–49 min | 0.5–1 h | Within planning limit |
| [10.3.31 · Draw the prepared text passes in order](62iu-draw-the-prepared-text-passes-in-order.md) | 30 | 787 | 8–16 min | 0.5–1 h | Within planning limit |
| [10.3.32 · Prepare the full text frame and load the scene depth](62j-frame-pass.md) | 7 | 1673 | 17–34 min | 0.5–1 h | Within planning limit |
| [10.3.33 · Merge original scene labels with the screen specimen](62k-lane-labels.md) | 16 | 2031 | 21–41 min | 0.5–1 h | Within planning limit |
| [10.3.34 · Draw original text IDs through bounded readback](62l-picking.md) | 16 | 2203 | 23–45 min | 0.5–1 h | Within planning limit |
| [10.3.35 · Fetch missing whole fonts as one checked set](62m-whole-fonts.md) | 27 | 1973 | 20–40 min | 0.5–1 h | Within planning limit |
| [10.3.36 · Show current text sources metrics and preparation errors](62n-diagnostics.md) | 11 | 1669 | 17–34 min | 0.5–1 h | Within planning limit |
| [10.3.37 · Create pick move and delete the original scene labels](62o-labels.md) | 6 | 826 | 9–17 min | 0.5–1 h | Within planning limit |
| [11.1.1 · Measure outline width in CSS pixels](63-radius.md) | 31 | 856 | 9–18 min | 0.5–1 h | Within planning limit |
| [11.1.2 · Convolve solid coverage into external contour ink](63a-coverage.md) | 87 | 2896 | 29–58 min | 0.5–1 h | Within planning limit |
| [11.1.3 · Pool and dilate a conservative outline mask](63aa-pool-and-dilate-a-conservative-outline-mask.md) | 37 | 1049 | 11–21 min | 0.5–1 h | Within planning limit |
| [11.1.4 · Own and count the derived outline textures](63b-resources.md) | 102 | 2719 | 28–55 min | 0.5–1 h | Within planning limit |
| [11.1.5 · Create the mask and fullscreen pipeline layouts](63c-pipelines.md) | 83 | 2807 | 29–57 min | 0.5–1 h | Within planning limit |
| [11.1.6 · Build bounded mask and tap bindings](63d-bindings.md) | 76 | 2792 | 28–56 min | 0.5–1 h | Within planning limit |
| [11.1.7 · Reuse masks while camera selection and density stay unchanged](63e-reuse.md) | 75 | 2946 | 30–59 min | 0.5–1 h | Within planning limit |
| [11.1.8 · Outline the visible union without internal seams](63f-union.md) | 69 | 2573 | 26–52 min | 0.5–1 h | Within planning limit |
| [11.1.9 · Connect a lazy silhouette owner to the renderer](63g-composite.md) | 12 | 1367 | 14–28 min | 0.5–1 h | Within planning limit |
| [11.1.10 · Check real contours widths reuse picks and cleanup](63h-outline.md) | 1 | 110 | 2–3 min | 0.5–1 h | Within planning limit |
| [11.2.1 · Check cloud cubes against their original point rows](64-nodes.md) | 89 | 2705 | 28–55 min | 0.5–1 h | Within planning limit |
| [11.2.2 · Validate one rooted tree and disjoint original ranges](64a-tree.md) | 46 | 1420 | 15–29 min | 0.5–1 h | Within planning limit |
| [11.2.3 · Prepare bounds and spacing without replacing source words](64b-source.md) | 51 | 1944 | 20–39 min | 0.5–1 h | Within planning limit |
| [11.2.4 · Place the cloud through an independent f64 instance model](64c-placements.md) | 25 | 696 | 7–14 min | 0.5–1 h | Within planning limit |
| [11.2.5 · Insert a checked cloud on the current layer](64d-scene.md) | 12 | 1663 | 17–34 min | 0.5–1 h | Within planning limit |
| [11.2.6 · Include clouds in whole selection history and rectangle checks](64e-identity.md) | 13 | 2367 | 24–48 min | 0.5–1 h | Within planning limit |
| [11.2.7 · Measure point spacing in the current projection](64f-projection.md) | 70 | 2356 | 24–48 min | 0.5–1 h | Within planning limit |
| [11.2.8 · Walk only detail whose screen spacing needs refinement](64g-detail.md) | 68 | 2027 | 21–41 min | 0.5–1 h | Within planning limit |
| [11.2.9 · Declare point-cloud records and decode source normals](64h-shaders.md) | 78 | 2200 | 22–44 min | 0.5–1 h | Within planning limit |
| [11.2.10 · Project original cloud rows to bounded point splats](64ha-project-original-cloud-rows-to-bounded-point-splats.md) | 96 | 2835 | 29–57 min | 0.5–1 h | Within planning limit |
| [11.2.11 · Rasterize source point discs with integer ownership](64hb-rasterize-source-point-discs-with-integer-ownership.md) | 45 | 1592 | 16–32 min | 0.5–1 h | Within planning limit |
| [11.2.12 · Encode original cloud normals and resolve point targets](64hc-encode-original-cloud-normals-and-resolve-point-targets.md) | 45 | 1645 | 17–33 min | 0.5–1 h | Within planning limit |
| [11.2.13 · Check cloud shaders on the actual browser device](64hd-check-cloud-shaders-on-the-actual-browser-device.md) | 27 | 1356 | 14–28 min | 0.5–1 h | Within planning limit |
| [11.2.14 · Upload original point columns once per shared source](64i-columns.md) | 87 | 2758 | 28–56 min | 0.5–1 h | Within planning limit |
| [11.2.15 · Own the cached cloud images and frame identity](64j-targets.md) | 83 | 1949 | 20–39 min | 0.5–1 h | Within planning limit |
| [11.2.16 · Bind source storage and cached pixels explicitly](64k-layouts.md) | 71 | 2596 | 26–52 min | 0.5–1 h | Within planning limit |
| [11.2.17 · Create a lazy cloud lane with reusable pipelines](64l-pipelines.md) | 67 | 2402 | 25–49 min | 0.5–1 h | Within planning limit |
| [11.2.18 · Share column uploads across placed cloud rows](64m-cache.md) | 78 | 2740 | 28–55 min | 0.5–1 h | Within planning limit |
| [11.2.19 · Encode each source run in the exact 160-byte layout](64n-records.md) | 72 | 2640 | 27–53 min | 0.5–1 h | Within planning limit |
| [11.2.20 · Keep one shared record budget for the whole view](64o-runs.md) | 58 | 1917 | 20–39 min | 0.5–1 h | Within planning limit |
| [11.2.21 · Cache the nearest point frame until its inputs change](64p-frame.md) | 81 | 2887 | 29–58 min | 0.5–1 h | Within planning limit |
| [11.2.22 · Create and release the cloud renderer only when needed](64q-owner.md) | 13 | 1484 | 15–30 min | 0.5–1 h | Within planning limit |
| [11.2.23 · Use the cached cloud depth in colour and integer passes](64r-draw.md) | 11 | 1484 | 15–30 min | 0.5–1 h | Within planning limit |
| [11.2.24 · Expose cloud source payload detail and actual GPU resources](64s-diagnostics.md) | 15 | 1453 | 15–30 min | 0.5–1 h | Within planning limit |
| [11.2.25 · Exercise original clouds and real view-dependent detail](64t-clouds.md) | 6 | 579 | 6–12 min | 0.5–1 h | Within planning limit |
| [11.3.1 · Resolve source point identities and precise positions](65-originals.md) | 35 | 2551 | 26–52 min | 0.5–1 h | Within planning limit |
| [11.3.2 · Recover original IDs from integer cloud pixels](65a-addresses.md) | 8 | 450 | 5–9 min | 0.5–1 h | Within planning limit |
| [11.3.3 · Enter source controls with the current command or F10](65b-controls.md) | 14 | 1660 | 17–34 min | 0.5–1 h | Within planning limit |
| [11.3.4 · Pick original points and prove source query reuse](65c-cloud-point.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [11.3.5 · Read original manifest records and placements](65za-read-original-manifest-records-and-placements.md) | 101 | 2897 | 29–58 min | 0.5–1 h | Within planning limit |
| [11.3.6 · Validate original manifest text and source keys](65zb-validate-original-manifest-text-and-source-keys.md) | 89 | 2763 | 28–56 min | 0.5–1 h | Within planning limit |
| [12.1.1 · Parse and bound original scene manifests](66-read-scene-manifests.md) | 83 | 2864 | 29–58 min | 0.5–1 h | Within planning limit |
| [12.1.2 · Compare live byte revisions](66a-compare-live-byte-revisions.md) | 35 | 978 | 10–20 min | 0.5–1 h | Within planning limit |
| [12.1.3 · Bound finite coordinates and packed integer reads](66b-validate-source-records-before-allocation.md) | 82 | 1973 | 20–40 min | 0.5–1 h | Within planning limit |
| [12.1.4 · Validate one NURBS knot axis](66ba-validate-one-nurbs-knot-axis.md) | 25 | 781 | 8–16 min | 0.5–1 h | Within planning limit |
| [12.1.5 · Read bounded JSON count and knot fields](66bb-read-bounded-json-count-and-knot-fields.md) | 48 | 1199 | 12–24 min | 0.5–1 h | Within planning limit |
| [12.1.6 · Match JSON control layout to declared counts](66bc-match-json-control-layout-to-declared-counts.md) | 44 | 1622 | 17–33 min | 0.5–1 h | Within planning limit |
| [12.1.7 · Walk JSON records within a depth limit](66bd-walk-json-records-within-a-depth-limit.md) | 41 | 1282 | 13–26 min | 0.5–1 h | Within planning limit |
| [12.1.8 · Validate original points lines and placements](66be-validate-original-points-lines-and-placements.md) | 44 | 1244 | 13–25 min | 0.5–1 h | Within planning limit |
| [12.1.9 · Validate original mesh indices and vertex attributes](66bf-validate-original-mesh-indices-and-vertex-attributes.md) | 50 | 1428 | 15–29 min | 0.5–1 h | Within planning limit |
| [12.1.10 · Validate curve and surface control addresses](66bg-validate-curve-and-surface-control-addresses.md) | 55 | 1712 | 18–35 min | 0.5–1 h | Within planning limit |
| [12.1.11 · Validate cloud columns and octree addresses](66bh-validate-cloud-columns-and-octree-addresses.md) | 48 | 1643 | 17–33 min | 0.5–1 h | Within planning limit |
| [12.1.12 · Validate BRep Element payloads and shared definitions](66bi-validate-brep-element-payloads-and-shared-definitions.md) | 91 | 1953 | 20–40 min | 0.5–1 h | Within planning limit |
| [12.1.13 · Validate the complete original protobuf session](66bj-validate-the-complete-original-protobuf-session.md) | 77 | 1771 | 18–36 min | 0.5–1 h | Within planning limit |
| [12.1.14 · Connect retained-session validation and independent witnesses](66bk-connect-retained-session-validation-and-independent-witnesses.md) | 25 | 558 | 6–12 min | 0.5–1 h | Within planning limit |
| [12.1.15 · Describe the saved scene metadata](66c-describe-the-saved-scene-metadata.md) | 61 | 2473 | 25–50 min | 0.5–1 h | Within planning limit |
| [12.1.16 · Encode decode the viewer file](66d-encode-decode-the-viewer-file.md) | 30 | 1955 | 20–40 min | 0.5–1 h | Within planning limit |
| [12.1.17 · Give copied lines independent identities](66e-give-copied-lines-independent-identities.md) | 5 | 554 | 6–12 min | 0.5–1 h | Within planning limit |
| [12.1.18 · Snapshot original geometry types](66f-snapshot-geometry-types.md) | 33 | 2107 | 22–43 min | 0.5–1 h | Within planning limit |
| [12.1.19 · Retain geometry free document headers](66g-retain-geometry-free-document-headers.md) | 13 | 1564 | 16–32 min | 0.5–1 h | Within planning limit |
| [12.1.20 · Inventory saved document namespaces](66h-inventory-saved-document-namespaces.md) | 15 | 1406 | 15–29 min | 0.5–1 h | Within planning limit |
| [12.1.21 · Map saved source identities](66i-map-saved-source-identities.md) | 21 | 1576 | 16–32 min | 0.5–1 h | Within planning limit |
| [12.1.22 · Store cloud size as presentation](66j-store-cloud-size-as-presentation.md) | 12 | 829 | 9–17 min | 0.5–1 h | Within planning limit |
| [12.1.23 · Prepare original saved documents](66k-prepare-saved-documents.md) | 20 | 2222 | 23–45 min | 0.5–1 h | Within planning limit |
| [12.1.24 · Save document display settings](66l-save-document-display-settings.md) | 16 | 1235 | 13–25 min | 0.5–1 h | Within planning limit |
| [12.1.25 · Save the complete supported scene](66m-save-the-complete-supported-scene.md) | 12 | 1520 | 16–31 min | 0.5–1 h | Within planning limit |
| [12.1.26 · Prepare original document sources](66n-prepare-document-sources.md) | 31 | 2388 | 24–48 min | 0.5–1 h | Within planning limit |
| [12.1.27 · Prepare every supported geometry row](66o-prepare-every-supported-geometry-row.md) | 22 | 2095 | 21–42 min | 0.5–1 h | Within planning limit |
| [12.1.28 · Adopt a complete source document](66p-adopt-a-complete-source-document.md) | 25 | 1623 | 17–33 min | 0.5–1 h | Within planning limit |
| [12.1.29 · Restore world labels and layer links](66q-restore-world-labels-layer-links.md) | 23 | 1620 | 17–33 min | 0.5–1 h | Within planning limit |
| [12.1.30 · Restore scoped presentation settings](66r-restore-scoped-presentation-settings.md) | 17 | 1802 | 19–37 min | 0.5–1 h | Within planning limit |
| [12.1.31 · Reopen a complete viewer file](66s-reopen-a-complete-viewer-file.md) | 9 | 684 | 7–14 min | 0.5–1 h | Within planning limit |
| [12.1.32 · Prepare file adoption and live history](66t-prepare-file-adoption-live-history.md) | 23 | 1473 | 15–30 min | 0.5–1 h | Within planning limit |
| [12.1.33 · Share repeated source placements](66u-share-repeated-source-placements.md) | 27 | 2696 | 27–54 min | 0.5–1 h | Within planning limit |
| [12.1.34 · Start an empty scene without examples](66v-start-an-empty-scene-without-examples.md) | 4 | 281 | 3–6 min | 0.5–1 h | Within planning limit |
| [12.1.35 · Assemble placed manifest entries](66w-assemble-placed-manifest-entries.md) | 20 | 1497 | 15–30 min | 0.5–1 h | Within planning limit |
| [12.1.36 · Resolve scene and live URLs](66x-resolve-scene-live-urls.md) | 26 | 2553 | 26–52 min | 0.5–1 h | Within planning limit |
| [12.1.37 · Retain validated live source revisions](66y-retain-validated-live-source-revisions.md) | 39 | 2927 | 30–59 min | 0.5–1 h | Within planning limit |
| [12.1.38 · Fetch bounded source bodies](66z-fetch-bounded-source-bodies.md) | 30 | 2690 | 27–54 min | 0.5–1 h | Within planning limit |
| [12.1.39 · Read a complete URL source revision](66aa-read-a-complete-url-source-revision.md) | 23 | 1711 | 18–35 min | 0.5–1 h | Within planning limit |
| [12.1.40 · Own and cancel the URL loader](66ab-own-cancel-the-url-loader.md) | 33 | 2567 | 26–52 min | 0.5–1 h | Within planning limit |
| [12.1.41 · Accept only current URL completions](66ac-accept-only-current-url-completions.md) | 17 | 1578 | 16–32 min | 0.5–1 h | Within planning limit |
| [12.1.42 · Watch published source changes](66ad-watch-published-source-changes.md) | 10 | 1097 | 11–22 min | 0.5–1 h | Within planning limit |
| [12.1.43 · Inspect published source identities](66ae-inspect-published-source-identities.md) | 2 | 721 | 8–15 min | 0.5–1 h | Within planning limit |
| [12.1.44 · Connect browser save open and URL adoption](66af-connect-browser-save-open-url-adoption.md) | 30 | 2930 | 30–59 min | 0.5–1 h | Within planning limit |
| [12.2.1 · Bound scalar headers and byte addresses](67-supply-checked-array-layout-readers.md) | 25 | 627 | 7–13 min | 0.5–1 h | Within planning limit |
| [12.2.2 · Locate original cloud and sheet array fields](67sa-locate-original-cloud-and-sheet-array-fields.md) | 82 | 2906 | 30–59 min | 0.5–1 h | Within planning limit |
| [12.2.3 · Convert bounded doubles and cache metadata windows](67sb-convert-bounded-doubles-and-cache-metadata-windows.md) | 62 | 1860 | 19–38 min | 0.5–1 h | Within planning limit |
| [12.2.4 · Decode packed arrays for existing GPU owners](67sc-decode-packed-arrays-for-existing-gpu-owners.md) | 95 | 2402 | 25–49 min | 0.5–1 h | Within planning limit |
| [12.2.5 · Descend bounded original protobuf messages](67sd-descend-bounded-original-protobuf-messages.md) | 42 | 1073 | 11–22 min | 0.5–1 h | Within planning limit |
| [12.2.6 · Find coordinate arrays without downloading their bodies](67se-find-coordinate-arrays-without-downloading-their-bodies.md) | 70 | 2034 | 21–41 min | 0.5–1 h | Within planning limit |
| [12.2.7 · Validate sheet side-array lengths](67sf-validate-sheet-side-array-lengths.md) | 36 | 1311 | 14–27 min | 0.5–1 h | Within planning limit |
| [12.2.8 · Collect original octree node columns](67sg-collect-original-octree-node-columns.md) | 50 | 1503 | 16–31 min | 0.5–1 h | Within planning limit |
| [12.2.9 · Check octree bounds and unique child ownership](67sh-check-octree-bounds-and-unique-child-ownership.md) | 64 | 1696 | 17–34 min | 0.5–1 h | Within planning limit |
| [12.2.10 · Validate complete octrees before range streaming](67si-validate-complete-octrees-before-range-streaming.md) | 35 | 926 | 10–19 min | 0.5–1 h | Within planning limit |
| [12.2.11 · Own original range-read permits](66za-own-original-range-read-permits.md) | 106 | 2669 | 27–54 min | 0.5–1 h | Within planning limit |
| [12.2.12 · Poll and cancel original queued range reads](67a-queue-range-reads-in-arrival-order.md) | 55 | 1674 | 17–34 min | 0.5–1 h | Within planning limit |
| [12.2.13 · Validate exact HTTP range replies](67b-validate-exact-http-range-replies.md) | 26 | 1398 | 14–28 min | 0.5–1 h | Within planning limit |
| [12.2.14 · Retain original point page values](67c-retain-original-point-page-values.md) | 26 | 1453 | 15–30 min | 0.5–1 h | Within planning limit |
| [12.2.15 · Own cancellable range requests](67d-own-cancellable-range-requests.md) | 25 | 1495 | 15–30 min | 0.5–1 h | Within planning limit |
| [12.2.16 · Time out stalled source reads](67e-time-out-stalled-source-reads.md) | 28 | 1695 | 17–34 min | 0.5–1 h | Within planning limit |
| [12.2.17 · Bound incoming range body chunks](67f-bound-incoming-range-body-chunks.md) | 16 | 1070 | 11–22 min | 0.5–1 h | Within planning limit |
| [12.2.18 · Request and accept one source interval](67g-request-and-accept-one-source-interval.md) | 19 | 2036 | 21–41 min | 0.5–1 h | Within planning limit |
| [12.2.19 · Keep range revisions and background results](67h-keep-range-revisions-and-background-results.md) | 15 | 1022 | 11–21 min | 0.5–1 h | Within planning limit |
| [12.2.20 · Queue original source range reads](67hza-queue-original-source-range-reads.md) | 84 | 2582 | 26–52 min | 0.5–1 h | Within planning limit |
| [12.2.21 · Probe original source array layouts](67hzb-probe-original-source-array-layouts.md) | 68 | 2212 | 23–45 min | 0.5–1 h | Within planning limit |
| [12.2.22 · Read original cloud node tables](67hzc-read-original-cloud-node-tables.md) | 87 | 2631 | 27–53 min | 0.5–1 h | Within planning limit |
| [12.2.23 · Fetch original coordinates normals and colours](67i-read-source-arrays-through-the-bounded-queue.md) | 72 | 2277 | 23–46 min | 0.5–1 h | Within planning limit |
| [12.2.24 · Keep original ranged source identity](67j-keep-original-ranged-source-identity.md) | 15 | 831 | 9–17 min | 0.5–1 h | Within planning limit |
| [12.2.25 · Prepare a revision bound remote source](67k-prepare-a-revision-bound-remote-source.md) | 16 | 918 | 10–19 min | 0.5–1 h | Within planning limit |
| [12.2.26 · Fetch precise coordinates and original IDs](67l-fetch-precise-coordinates-and-original-ids.md) | 11 | 915 | 10–19 min | 0.5–1 h | Within planning limit |
| [12.2.27 · Cancel range work when its runtime closes](67m-cancel-range-work-when-its-runtime-closes.md) | 2 | 277 | 3–6 min | 0.5–1 h | Within planning limit |
| [12.3.1 · Reserve page bytes before reading](68-reserve-page-bytes-before-reading.md) | 29 | 1197 | 12–24 min | 0.5–1 h | Within planning limit |
| [12.3.2 · Keep shared pages alive during eviction](68a-keep-shared-pages-alive-during-eviction.md) | 19 | 1109 | 12–23 min | 0.5–1 h | Within planning limit |
| [12.3.3 · Evict unpinned pages in access order](68b-evict-unpinned-pages-in-access-order.md) | 18 | 1369 | 14–28 min | 0.5–1 h | Within planning limit |
| [12.3.4 · Reject late source generations](68c-reject-late-source-generations.md) | 19 | 1170 | 12–24 min | 0.5–1 h | Within planning limit |
| [12.3.5 · Bound source prefixes and decode work](68d-bound-source-prefixes-and-decode-work.md) | 14 | 683 | 7–14 min | 0.5–1 h | Within planning limit |
| [12.3.6 · Validate original streamed presentation](68e-validate-original-streamed-presentation.md) | 20 | 909 | 10–19 min | 0.5–1 h | Within planning limit |
| [12.3.7 · Reuse a checked source probe](68f-reuse-a-checked-source-probe.md) | 19 | 1346 | 14–27 min | 0.5–1 h | Within planning limit |
| [12.3.8 · Validate complete source metadata](68fa-validate-complete-source-metadata.md) | 35 | 2350 | 24–47 min | 0.5–1 h | Within planning limit |
| [12.3.9 · Slice original pages without losing attributes](68g-slice-original-pages-without-losing-attributes.md) | 16 | 1433 | 15–29 min | 0.5–1 h | Within planning limit |
| [12.3.10 · Prepare shared remote source owners](68h-prepare-shared-remote-source-owners.md) | 18 | 1797 | 18–36 min | 0.5–1 h | Within planning limit |
| [12.3.11 · Keep independent streamed placements](68i-keep-independent-streamed-placements.md) | 9 | 586 | 6–12 min | 0.5–1 h | Within planning limit |
| [12.3.12 · Pin coarse pages and adopt checked ranges](68j-pin-coarse-pages-and-adopt-checked-ranges.md) | 22 | 2103 | 22–43 min | 0.5–1 h | Within planning limit |
| [12.3.13 · Supply coarse and detailed page specimens](68k-supply-coarse-and-detailed-page-specimens.md) | 2 | 24 | 1–1 min | 0.5–1 h | Within planning limit |
| [12.3.14 · Insert streamed rows on the current layer](68l-insert-streamed-rows-on-the-current-layer.md) | 18 | 2337 | 24–47 min | 0.5–1 h | Within planning limit |
| [12.3.15 · Carry source placement through scene operations](68m-carry-source-placement-through-scene-operations.md) | 12 | 2031 | 21–41 min | 0.5–1 h | Within planning limit |
| [12.3.16 · Resolve compact picks to original point IDs](68n-resolve-compact-picks-to-original-point-ids.md) | 15 | 1910 | 20–39 min | 0.5–1 h | Within planning limit |
| [12.3.17 · Account for original and retained source data](68o-account-for-original-and-retained-source-data.md) | 10 | 560 | 6–12 min | 0.5–1 h | Within planning limit |
| [12.3.18 · Keep original attributes in ranged pages](68p-keep-original-attributes-in-ranged-pages.md) | 12 | 984 | 10–20 min | 0.5–1 h | Within planning limit |
| [12.3.19 · Retain page owners beside GPU buffers](68q-retain-page-owners-beside-gpu-buffers.md) | 8 | 995 | 10–20 min | 0.5–1 h | Within planning limit |
| [12.3.20 · Describe resident and streamed draw rows](68r-describe-resident-and-streamed-draw-rows.md) | 20 | 1553 | 16–32 min | 0.5–1 h | Within planning limit |
| [12.3.21 · Draw shared source pages at independent placements](68s-draw-shared-source-pages-at-independent-placements.md) | 29 | 2996 | 30–60 min | 0.5–1 h | Within planning limit |
| [12.3.22 · Validate encoded source metadata](68t-validate-encoded-source-metadata.md) | 14 | 820 | 9–17 min | 0.5–1 h | Within planning limit |
| [12.3.23 · Decode bounded cancellable source streams](68u-decode-bounded-cancellable-source-streams.md) | 35 | 2239 | 23–45 min | 0.5–1 h | Within planning limit |
| [12.3.24 · Refuse conflicting source encoding revisions](68v-refuse-conflicting-source-encoding-revisions.md) | 22 | 2228 | 23–45 min | 0.5–1 h | Within planning limit |
| [12.3.25 · Adopt remote original revisions independently](68w-adopt-remote-original-revisions-independently.md) | 10 | 955 | 10–20 min | 0.5–1 h | Within planning limit |
| [12.3.26 · Prepare a coarse pin and bounded prefix](68x-prepare-a-coarse-pin-and-bounded-prefix.md) | 32 | 1976 | 20–40 min | 0.5–1 h | Within planning limit |
| [12.3.27 · Redraw before reusing evicted page addresses](68y-redraw-before-reusing-evicted-page-addresses.md) | 15 | 888 | 9–18 min | 0.5–1 h | Within planning limit |
| [12.3.28 · Continue pages only for current source owners](68z-continue-pages-only-for-current-source-owners.md) | 17 | 1264 | 13–26 min | 0.5–1 h | Within planning limit |
| [12.3.29 · Own cancellation for each published source](68aa-own-cancellation-for-each-published-source.md) | 11 | 1623 | 17–33 min | 0.5–1 h | Within planning limit |
| [12.3.30 · Choose bounded range loading for large originals](68ab-choose-bounded-range-loading-for-large-originals.md) | 24 | 2504 | 26–51 min | 0.5–1 h | Within planning limit |
| [12.3.31 · Connect current pages to the browser renderer](68ac-connect-current-pages-to-the-browser-renderer.md) | 24 | 2413 | 25–49 min | 0.5–1 h | Within planning limit |
| [13.1.1 · Validate original definition references](69-validate-original-definition-references.md) | 33 | 2326 | 24–47 min | 0.5–1 h | Within planning limit |
| [13.1.2 · Prepare each used definition once](69b-prepare-each-used-definition-once.md) | 32 | 1611 | 17–33 min | 0.5–1 h | Within planning limit |
| [13.1.3 · Keep instance bindings through scene history](69c-keep-instance-bindings-through-scene-history.md) | 7 | 848 | 9–17 min | 0.5–1 h | Within planning limit |
| [13.1.4 · Append independently placed shared rows](69d-append-independently-placed-shared-rows.md) | 23 | 2396 | 24–48 min | 0.5–1 h | Within planning limit |
| [13.1.5 · Import canonical original instance placements](69e-import-canonical-original-instance-placements.md) | 12 | 1528 | 16–31 min | 0.5–1 h | Within planning limit |
| [13.1.6 · Protect shared originals during part edits](69f-protect-shared-originals-during-part-edits.md) | 1 | 132 | 2–3 min | 0.5–1 h | Within planning limit |
| [13.1.7 · Collect shared definitions without flattening](69g-collect-shared-definitions-without-flattening.md) | 26 | 1867 | 19–38 min | 0.5–1 h | Within planning limit |
| [13.1.8 · Save original instance references and namespaces](69h-save-original-instance-references-and-namespaces.md) | 11 | 1694 | 17–34 min | 0.5–1 h | Within planning limit |
| [13.1.9 · Check sharing precision history and release](69i-check-sharing-precision-history-and-release.md) | 8 | 504 | 6–11 min | 0.5–1 h | Within planning limit |
| [13.1.10 · Preserve per instance original colours](69j-preserve-per-instance-original-colours.md) | 14 | 1789 | 18–36 min | 0.5–1 h | Within planning limit |
| [13.1.11 · Create a shared definition face pipeline](69k-create-a-shared-definition-face-pipeline.md) | 18 | 2810 | 29–57 min | 0.5–1 h | Within planning limit |
| [13.1.12 · Reuse bounded independent draw records](69l-reuse-bounded-independent-draw-records.md) | 23 | 2253 | 23–46 min | 0.5–1 h | Within planning limit |
| [13.1.13 · Draw batched faces with independent integer owners](69m-draw-batched-faces-with-independent-integer-owners.md) | 11 | 1387 | 14–28 min | 0.5–1 h | Within planning limit |
| [13.1.14 · Inspect and exercise original definition instances](69n-inspect-and-exercise-original-definition-instances.md) | 7 | 1050 | 11–21 min | 0.5–1 h | Within planning limit |
| [13.1.15 · Move between original rigid frames](69zza-move-between-original-rigid-frames.md) | 106 | 2956 | 30–60 min | 0.5–1 h | Within planning limit |
| [13.1.16 · Find an original frame and pack key words](69zzb-find-an-original-frame-and-pack-key-words.md) | 100 | 2886 | 29–58 min | 0.5–1 h | Within planning limit |
| [13.1.17 · Encode original parameters and topology](69zzc-encode-original-parameters-and-topology.md) | 88 | 2785 | 28–56 min | 0.5–1 h | Within planning limit |
| [13.1.18 · Assemble an original rigid shape key](69a-describe-a-rigid-frame-and-original-shape-key.md) | 56 | 1787 | 18–36 min | 0.5–1 h | Within planning limit |
| [13.1.19 · Retain cloneable derived CAD values](69aa-retain-cloneable-derived-cad-values.md) | 14 | 598 | 6–12 min | 0.5–1 h | Within planning limit |
| [13.1.20 · Replay sampled geometry in the new rigid frame](69ab-replay-sampled-geometry-in-the-new-rigid-frame.md) | 20 | 1831 | 19–37 min | 0.5–1 h | Within planning limit |
| [13.1.21 · Guard reusable walks with source validity and size](69ac-guard-reusable-walks-with-source-validity-and-size.md) | 10 | 1009 | 11–21 min | 0.5–1 h | Within planning limit |
| [13.1.22 · Share recordings only during document preparation](69ad-share-recordings-only-during-document-preparation.md) | 16 | 1686 | 17–34 min | 0.5–1 h | Within planning limit |
| [13.1.23 · Compare actual replay pixels and integer owners](69ae-compare-actual-replay-pixels-and-integer-owners.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [13.1.24 · Exercise independent original CAD reuse](69af-exercise-independent-original-cad-reuse.md) | 6 | 551 | 6–12 min | 0.5–1 h | Within planning limit |
| [13.2.1 · Evaluate source ancestry once per document](70-evaluate-source-ancestry-once-per-document.md) | 11 | 769 | 8–16 min | 0.5–1 h | Within planning limit |
| [13.2.2 · Keep original local parent and world matrices](70a-keep-original-local-parent-and-world-matrices.md) | 20 | 2235 | 23–45 min | 0.5–1 h | Within planning limit |
| [13.2.3 · Connect canonical cached models to every source owner](70b-connect-canonical-cached-models-to-every-source-owner.md) | 8 | 1084 | 11–22 min | 0.5–1 h | Within planning limit |
| [13.2.4 · Save nested local frames without flattening](70c-save-nested-local-frames-without-flattening.md) | 9 | 831 | 9–17 min | 0.5–1 h | Within planning limit |
| [13.2.5 · Exercise rotated and mirrored original references](70d-exercise-rotated-and-mirrored-original-references.md) | 5 | 749 | 8–15 min | 0.5–1 h | Within planning limit |
| [13.2.6 · Read original sheet metadata records](70za-read-original-sheet-metadata-records.md) | 70 | 2116 | 22–43 min | 0.5–1 h | Within planning limit |
| [14.1.1 · Own cancellable sheet-entity lookups](71-read-original-sheet-entity-metadata.md) | 35 | 898 | 9–18 min | 0.5–1 h | Within planning limit |
| [14.1.2 · Bound original entity records before reading](71a-bound-original-entity-records-before-reading.md) | 21 | 720 | 8–15 min | 0.5–1 h | Within planning limit |
| [14.1.3 · Give metadata reads their own cancellation signal](71b-give-metadata-reads-their-own-cancellation-signal.md) | 16 | 1283 | 13–26 min | 0.5–1 h | Within planning limit |
| [14.1.4 · Fetch one revision matched metadata answer](71c-fetch-one-revision-matched-metadata-answer.md) | 81 | 2801 | 29–57 min | 0.5–1 h | Within planning limit |
| [14.1.5 · Decode precise original sheet columns](71d-decode-precise-original-sheet-columns.md) | 67 | 2328 | 24–47 min | 0.5–1 h | Within planning limit |
| [14.1.6 · Measure original sheet pages and world pens](71e-measure-original-sheet-pages-and-world-pens.md) | 28 | 870 | 9–18 min | 0.5–1 h | Within planning limit |
| [14.1.7 · Retain charged pages beside resident pages](71f-retain-charged-pages-beside-resident-pages.md) | 33 | 890 | 9–18 min | 0.5–1 h | Within planning limit |
| [14.1.8 · Own the complete original drawing source](71g-own-the-complete-original-drawing-source.md) | 59 | 2077 | 21–42 min | 0.5–1 h | Within planning limit |
| [14.1.9 · Place drawing rows without changing endpoints](71h-place-drawing-rows-without-changing-endpoints.md) | 51 | 1740 | 18–35 min | 0.5–1 h | Within planning limit |
| [14.1.10 · Register original drawing rows on the current layer](71i-register-original-drawing-rows-on-the-current-layer.md) | 11 | 1862 | 19–38 min | 0.5–1 h | Within planning limit |
| [14.1.11 · Carry drawing identity through scene operations](71j-carry-drawing-identity-through-scene-operations.md) | 13 | 2403 | 25–49 min | 0.5–1 h | Within planning limit |
| [14.1.12 · Save and reopen complete original sheets](71k-save-and-reopen-complete-original-sheets.md) | 10 | 2142 | 22–43 min | 0.5–1 h | Within planning limit |
| [14.1.13 · Locate original drawing identity and fixed ranges](71l-locate-original-drawing-identity-and-fixed-ranges.md) | 75 | 2106 | 22–43 min | 0.5–1 h | Within planning limit |
| [14.1.14 · Validate the bounded drawing envelope tail](71m-validate-the-bounded-drawing-envelope-tail.md) | 24 | 703 | 8–15 min | 0.5–1 h | Within planning limit |
| [14.1.15 · Scan drawing columns under one revision](71n-scan-drawing-columns-under-one-revision.md) | 69 | 2708 | 28–55 min | 0.5–1 h | Within planning limit |
| [14.1.16 · Read drawing page columns through the shared queue](71o-read-drawing-page-columns-through-the-shared-queue.md) | 47 | 1697 | 17–34 min | 0.5–1 h | Within planning limit |
| [14.1.17 · Adopt nonoverlapping charged drawing pages](71p-adopt-nonoverlapping-charged-drawing-pages.md) | 52 | 1703 | 18–35 min | 0.5–1 h | Within planning limit |
| [14.1.18 · Keep partial drawing sources explicit](71q-keep-partial-drawing-sources-explicit.md) | 46 | 2423 | 25–49 min | 0.5–1 h | Within planning limit |
| [14.1.19 · Reserve memory before the drawing prefix](71r-reserve-memory-before-the-drawing-prefix.md) | 51 | 1671 | 17–34 min | 0.5–1 h | Within planning limit |
| [14.1.20 · Continue drawing reads without reviving closed sources](71s-continue-drawing-reads-without-reviving-closed-sources.md) | 58 | 1854 | 19–38 min | 0.5–1 h | Within planning limit |
| [14.1.21 · Resolve sheet picks to original entity IDs](71t-resolve-sheet-picks-to-original-entity-ids.md) | 74 | 2977 | 30–60 min | 0.5–1 h | Within planning limit |
| [14.1.22 · Project authored world pens into drawing quads](71u-project-authored-world-pens-into-drawing-quads.md) | 40 | 2538 | 26–51 min | 0.5–1 h | Within planning limit |
| [14.1.23 · Use the same sheet coverage for color and picks](71v-use-the-same-sheet-coverage-for-color-and-picks.md) | 10 | 967 | 10–20 min | 0.5–1 h | Within planning limit |
| [14.1.24 · Build sheet color and integer pipelines](71w-build-sheet-color-and-integer-pipelines.md) | 67 | 2800 | 28–56 min | 0.5–1 h | Within planning limit |
| [14.1.25 · Own the shared drawing lane resources](71x-own-the-shared-drawing-lane-resources.md) | 58 | 1561 | 16–32 min | 0.5–1 h | Within planning limit |
| [14.1.26 · Bound drawing buffers and pack independent settings](71y-bound-drawing-buffers-and-pack-independent-settings.md) | 41 | 1157 | 12–24 min | 0.5–1 h | Within planning limit |
| [14.1.27 · Upload each original drawing page once](71z-upload-each-original-drawing-page-once.md) | 51 | 2014 | 21–41 min | 0.5–1 h | Within planning limit |
| [14.1.28 · Reuse independent drawing placement records](71aa-reuse-independent-drawing-placement-records.md) | 66 | 2779 | 28–56 min | 0.5–1 h | Within planning limit |
| [14.1.29 · Draw world pens with the current camera lens](71ab-draw-world-pens-with-the-current-camera-lens.md) | 62 | 2455 | 25–50 min | 0.5–1 h | Within planning limit |
| [14.1.30 · Connect sheet batches to rendering and bounded picks](71ac-connect-sheet-batches-to-rendering-and-bounded-picks.md) | 16 | 2474 | 25–50 min | 0.5–1 h | Within planning limit |
| [14.1.31 · Own metadata delivery without pinning drawings](71ad-own-metadata-delivery-without-pinning-drawings.md) | 88 | 2283 | 23–46 min | 0.5–1 h | Within planning limit |
| [14.1.32 · Query metadata only for the current selected entity](71ae-query-metadata-only-for-the-current-selected-entity.md) | 73 | 2589 | 26–52 min | 0.5–1 h | Within planning limit |
| [14.1.33 · Keep published drawing sources and URL bases independent](71af-keep-published-drawing-sources-and-url-bases-independent.md) | 7 | 1174 | 12–24 min | 0.5–1 h | Within planning limit |
| [14.1.34 · Retire staged drawing reads with their publication owner](71ag-retire-staged-drawing-reads-with-their-publication-owner.md) | 9 | 2700 | 27–54 min | 0.5–1 h | Within planning limit |
| [14.1.35 · Stage bounded drawings through publication loading](71ah-stage-bounded-drawings-through-publication-loading.md) | 9 | 1398 | 14–28 min | 0.5–1 h | Within planning limit |
| [14.1.36 · Show current sheet resources and selected entity answers](71ai-show-current-sheet-resources-and-selected-entity-answers.md) | 9 | 1796 | 18–36 min | 0.5–1 h | Within planning limit |
| [14.1.37 · Exercise complete and ranged original drawings](71aj-exercise-complete-and-ranged-original-drawings.md) | 4 | 413 | 5–9 min | 0.5–1 h | Within planning limit |
| [14.1.38 · Build original construction-plane frames](71za-build-original-construction-plane-frames.md) | 79 | 2629 | 27–53 min | 0.5–1 h | Within planning limit |
| [14.1.39 · Map original plane and world coordinates](71zb-map-original-plane-and-world-coordinates.md) | 43 | 1417 | 15–29 min | 0.5–1 h | Within planning limit |
| [14.1.40 · Guard original plane and ray projections](71zc-guard-original-plane-and-ray-projections.md) | 85 | 2854 | 29–58 min | 0.5–1 h | Within planning limit |
| [14.1.41 · Parse original drawing coordinate forms](71zd-parse-original-drawing-coordinate-forms.md) | 85 | 2589 | 26–52 min | 0.5–1 h | Within planning limit |
| [14.1.42 · Resolve original relative and polar coordinates](71ze-resolve-original-relative-and-polar-coordinates.md) | 58 | 1927 | 20–39 min | 0.5–1 h | Within planning limit |
| [14.2.1 · Place original coordinates in the selected plane](72-copy-current-construction-planes-and-coordinate-forms.md) | 62 | 2364 | 24–48 min | 0.5–1 h | Within planning limit |
| [14.2.2 · Resolve bounded typed batches in the current plane](72a-resolve-bounded-typed-batches-in-the-current-plane.md) | 23 | 1431 | 15–29 min | 0.5–1 h | Within planning limit |
| [14.2.3 · Place clicks on stable construction planes](72b-place-clicks-on-stable-construction-planes.md) | 22 | 1432 | 15–29 min | 0.5–1 h | Within planning limit |
| [14.2.4 · Name the construction plane chooser options](72c-name-the-construction-plane-chooser-options.md) | 3 | 155 | 2–4 min | 0.5–1 h | Within planning limit |
| [14.2.5 · Add original polylines to the point tool](72d-add-original-polylines-to-the-point-tool.md) | 13 | 1087 | 11–22 min | 0.5–1 h | Within planning limit |
| [14.2.6 · Keep construction frames and plane points in drafts](72e-keep-construction-frames-and-plane-points-in-drafts.md) | 5 | 781 | 8–16 min | 0.5–1 h | Within planning limit |
| [14.2.7 · Replay finished drafts with explicit world prefixes](72f-replay-finished-drafts-with-explicit-world-prefixes.md) | 19 | 1412 | 15–29 min | 0.5–1 h | Within planning limit |
| [14.2.8 · Own the fixed construction plane in the editor](72g-own-the-fixed-construction-plane-in-the-editor.md) | 4 | 222 | 3–5 min | 0.5–1 h | Within planning limit |
| [14.2.9 · Commit completed original geometry exactly once](72h-commit-completed-original-geometry-exactly-once.md) | 15 | 1134 | 12–23 min | 0.5–1 h | Within planning limit |
| [14.2.10 · Change fixed planes without rewriting accepted points](72i-change-fixed-planes-without-rewriting-accepted-points.md) | 11 | 1539 | 16–31 min | 0.5–1 h | Within planning limit |
| [14.2.11 · Open the plane chooser and collect its frame](72j-open-the-plane-chooser-and-collect-its-frame.md) | 29 | 2549 | 26–51 min | 0.5–1 h | Within planning limit |
| [14.2.12 · Resolve typed input against the current draft](72k-resolve-typed-input-against-the-current-draft.md) | 12 | 749 | 8–15 min | 0.5–1 h | Within planning limit |
| [14.2.13 · Finish open tools with Enter and route typed forms](72l-finish-open-tools-with-enter-and-route-typed-forms.md) | 14 | 1975 | 20–40 min | 0.5–1 h | Within planning limit |
| [14.2.14 · Validate complete drawing commands before adoption](72m-validate-complete-drawing-commands-before-adoption.md) | 11 | 1572 | 16–32 min | 0.5–1 h | Within planning limit |
| [14.2.15 · Close original polylines as one transaction](72n-close-original-polylines-as-one-transaction.md) | 19 | 2073 | 21–42 min | 0.5–1 h | Within planning limit |
| [14.2.16 · Snap draft starts and use the construction frame grid](72o-snap-draft-starts-and-use-the-construction-frame-grid.md) | 14 | 1262 | 13–26 min | 0.5–1 h | Within planning limit |
| [14.2.17 · Register current drawing commands and plane choices](72p-register-current-drawing-commands-and-plane-choices.md) | 7 | 592 | 6–12 min | 0.5–1 h | Within planning limit |
| [14.2.18 · Preview accepted segments without committing originals](72q-preview-accepted-segments-without-committing-originals.md) | 5 | 562 | 6–12 min | 0.5–1 h | Within planning limit |
| [14.2.19 · Expose original drawing coordinates and cancellation state](72r-expose-original-drawing-coordinates-and-cancellation-state.md) | 10 | 2441 | 25–49 min | 0.5–1 h | Within planning limit |
| [14.2.20 · Exercise current drawing tools and original persistence](72s-exercise-current-drawing-tools-and-original-persistence.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [14.3.1 · Construct rectangles and polygons in plane axes](73-construct-rectangles-and-polygons-in-plane-axes.md) | 48 | 1513 | 16–31 min | 0.5–1 h | Within planning limit |
| [14.3.2 · Bound generated original construction points](73a-bound-generated-original-construction-points.md) | 6 | 600 | 6–12 min | 0.5–1 h | Within planning limit |
| [14.3.3 · Give point tools a plane aware construction hook](73b-give-point-tools-a-plane-aware-construction-hook.md) | 4 | 379 | 4–8 min | 0.5–1 h | Within planning limit |
| [14.3.4 · Build original rectangle and polygon tool results](73c-build-original-rectangle-and-polygon-tool-results.md) | 12 | 1286 | 13–26 min | 0.5–1 h | Within planning limit |
| [14.3.5 · Preserve world replay for multiword construction names](73d-preserve-world-replay-for-multiword-construction-names.md) | 6 | 330 | 4–7 min | 0.5–1 h | Within planning limit |
| [14.3.6 · Keep polygon sides in the current draft](73e-keep-polygon-sides-in-the-current-draft.md) | 15 | 1924 | 20–39 min | 0.5–1 h | Within planning limit |
| [14.3.7 · Register current planar options and temporary outlines](73f-register-current-planar-options-and-temporary-outlines.md) | 11 | 1131 | 12–23 min | 0.5–1 h | Within planning limit |
| [14.3.8 · Exercise oriented original planar constructions](73g-exercise-oriented-original-planar-constructions.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [14.3.9 · Match solid command options](73h-match-solid-command-options.md) | 41 | 1134 | 12–23 min | 0.8–1 h | Within planning limit |
| [14.3.10 · Describe solid command questions](73i-describe-solid-command-questions.md) | 73 | 2041 | 21–41 min | 0.8–1 h | Within planning limit |
| [14.3.11 · Build a right-handed solid frame](73j-build-a-right-handed-solid-frame.md) | 81 | 2241 | 23–45 min | 0.8–1 h | Within planning limit |
| [14.3.12 · Sample solid preview wires](73k-sample-solid-preview-wires.md) | 35 | 1288 | 13–26 min | 0.8–1 h | Within planning limit |
| [14.3.13 · Describe solid builders and dimensions](73l-describe-solid-builders-and-dimensions.md) | 73 | 2520 | 26–51 min | 0.8–1 h | Within planning limit |
| [14.3.14 · Validate solid sizes and radius](73m-validate-solid-sizes-and-radius.md) | 61 | 1761 | 18–36 min | 0.8–1 h | Within planning limit |
| [14.3.15 · Weld solid tessellation once](73n-weld-solid-tessellation-once.md) | 76 | 2409 | 25–49 min | 0.8–1 h | Within planning limit |
| [14.3.16 · Scale and cache polyhedron faces](73o-scale-and-cache-polyhedron-faces.md) | 65 | 1878 | 19–38 min | 0.8–1 h | Within planning limit |
| [14.3.17 · Build a placed kernel box](73p-build-a-placed-kernel-box.md) | 54 | 1660 | 17–34 min | 0.8–1 h | Within planning limit |
| [14.3.18 · Read box questions and corner clicks](73q-read-box-questions-and-corner-clicks.md) | 52 | 1684 | 17–34 min | 0.8–1 h | Within planning limit |
| [14.3.19 · Build an oriented kernel pyramid](73r-build-an-oriented-kernel-pyramid.md) | 90 | 2714 | 28–55 min | 0.8–1 h | Within planning limit |
| [14.3.20 · Build a block with a kernel hole](73s-build-a-block-with-a-kernel-hole.md) | 70 | 2214 | 23–45 min | 0.8–1 h | Within planning limit |
| [14.3.21 · Build regular kernel polyhedra](73t-build-regular-kernel-polyhedra.md) | 68 | 1848 | 19–37 min | 0.8–1 h | Within planning limit |
| [14.4.1 · Add the remaining regular kernel solids](74-copy-current-solid-geometry-and-question-definitions.md) | 68 | 1843 | 19–37 min | 0.8–1 h | Within planning limit |
| [14.4.2 · Own current shape questions and answer state](74a-own-current-shape-questions-and-answer-state.md) | 28 | 1146 | 12–23 min | 0.5–1 h | Within planning limit |
| [14.4.3 · Validate a new answer before changing its original state](74b-validate-a-new-answer-before-changing-its-original-state.md) | 16 | 1103 | 12–23 min | 0.5–1 h | Within planning limit |
| [14.4.4 · Build only after the last shape question](74c-build-only-after-the-last-shape-question.md) | 6 | 360 | 4–8 min | 0.5–1 h | Within planning limit |
| [14.4.5 · Answer shape dimensions with atomic typed batches](74d-answer-shape-dimensions-with-atomic-typed-batches.md) | 29 | 2038 | 21–41 min | 0.5–1 h | Within planning limit |
| [14.4.6 · Preview solid wires and measure normal axis clicks](74e-preview-solid-wires-and-measure-normal-axis-clicks.md) | 12 | 961 | 10–20 min | 0.5–1 h | Within planning limit |
| [14.4.7 · Adopt original primitive geometry through existing rows](74f-adopt-original-primitive-geometry-through-existing-rows.md) | 15 | 791 | 8–16 min | 0.5–1 h | Within planning limit |
| [14.4.8 · Keep primitive answers beside the existing drawing draft](74g-keep-primitive-answers-beside-the-existing-drawing-draft.md) | 10 | 1595 | 16–32 min | 0.5–1 h | Within planning limit |
| [14.4.9 · Route clicked points to the current shape question](74h-route-clicked-points-to-the-current-shape-question.md) | 10 | 926 | 10–19 min | 0.5–1 h | Within planning limit |
| [14.4.10 · Route typed shape input and preserve detailed errors](74i-route-typed-shape-input-and-preserve-detailed-errors.md) | 14 | 1467 | 15–30 min | 0.5–1 h | Within planning limit |
| [14.4.11 · Synchronize shape frames before accepting plane changes](74j-synchronize-shape-frames-before-accepting-plane-changes.md) | 3 | 518 | 6–11 min | 0.5–1 h | Within planning limit |
| [14.4.12 · Resolve height clicks without closing an open shape collector](74k-resolve-height-clicks-without-closing-an-open-shape-collector.md) | 3 | 742 | 8–15 min | 0.5–1 h | Within planning limit |
| [14.4.13 · Prepare complete solid commands before replacing a draft](74l-prepare-complete-solid-commands-before-replacing-a-draft.md) | 11 | 1964 | 20–40 min | 0.5–1 h | Within planning limit |
| [14.4.14 · Register every current solid name and representation](74m-register-every-current-solid-name-and-representation.md) | 32 | 1938 | 20–39 min | 0.5–1 h | Within planning limit |
| [14.4.15 · Draw temporary solid wire sets and report current input failures](74n-draw-temporary-solid-wire-sets-and-report-current-input-failures.md) | 3 | 365 | 4–8 min | 0.5–1 h | Within planning limit |
| [14.4.16 · Keep quick mouse drift as a click using current event time](74o-keep-quick-mouse-drift-as-a-click-using-current-event-time.md) | 20 | 1849 | 19–37 min | 0.5–1 h | Within planning limit |
| [14.4.17 · Exercise current solid input and original persistence](74p-exercise-current-solid-input-and-original-persistence.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [14.4.18 · Build original cylinders with signed height](74za-build-original-cylinders-with-signed-height.md) | 76 | 2219 | 23–45 min | 0.5–1 h | Within planning limit |
| [14.4.19 · Build original cones and spheres](74zb-build-original-cones-and-spheres.md) | 80 | 2102 | 22–43 min | 0.5–1 h | Within planning limit |
| [14.5.1 · Build original tori with checked radii](75-reuse-current-round-solid-builders.md) | 83 | 2250 | 23–45 min | 0.5–1 h | Within planning limit |
| [14.5.2 · Build original capsules with shared dimensions](75aa-build-original-capsules-with-shared-dimensions.md) | 71 | 2215 | 23–45 min | 0.5–1 h | Within planning limit |
| [14.5.3 · Mesh original quad-sphere patches](75a-retain-mesh-only-round-families.md) | 34 | 1015 | 11–21 min | 0.5–1 h | Within planning limit |
| [14.5.4 · Register current round names and choices](75b-register-current-round-names-and-choices.md) | 16 | 804 | 9–17 min | 0.5–1 h | Within planning limit |
| [14.5.5 · Exercise round originals and one transaction](75c-exercise-round-originals-and-one-transaction.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [14.5.6 · Build original planar circles](75za-build-original-planar-circles.md) | 55 | 1435 | 15–29 min | 0.5–1 h | Within planning limit |
| [14.5.7 · Build original planar ellipses](75zb-build-original-planar-ellipses.md) | 73 | 2231 | 23–45 min | 0.5–1 h | Within planning limit |
| [14.5.8 · Build exact rational arc spans](75zc-build-exact-rational-arc-spans.md) | 64 | 2150 | 22–43 min | 0.5–1 h | Within planning limit |
| [14.5.9 · Read original arcs from center and angle](75zd-read-original-arcs-from-center-and-angle.md) | 43 | 1208 | 13–25 min | 0.5–1 h | Within planning limit |
| [14.5.10 · Read three-point arcs and connect the recipe](75ze-read-three-point-arcs-and-connect-the-recipe.md) | 89 | 2920 | 30–59 min | 0.5–1 h | Within planning limit |
| [14.5.11 · Build original parabola previews and sources](75zf-build-original-parabola-previews-and-sources.md) | 56 | 1865 | 19–38 min | 0.5–1 h | Within planning limit |
| [14.6.1 · Read original parabola inputs in a checked frame](76-copy-exact-planar-curve-definitions.md) | 51 | 1482 | 15–30 min | 0.5–1 h | Within planning limit |
| [14.6.2 · Expose exact curve choices in the existing registry](76a-expose-exact-curve-choices-in-the-existing-registry.md) | 6 | 370 | 4–8 min | 0.5–1 h | Within planning limit |
| [14.6.3 · Create editable freeform curves](76b-retain-editable-freeform-controls-in-the-point-collector.md) | 28 | 2137 | 22–43 min | 0.5–1 h | Within planning limit |
| [14.6.4 · Inspect original curve data through the browser contract](76c-inspect-original-curve-data-through-the-browser-contract.md) | 1 | 377 | 4–8 min | 0.5–1 h | Within planning limit |
| [14.6.5 · Exercise exact curves and rational control edits](76d-exercise-exact-curves-and-rational-control-edits.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [15.1.1 · Build directional scale transforms](77-copy-current-transformation-and-source-edit-algorithms.md) | 55 | 1904 | 20–39 min | 0.5–1 h | Within planning limit |
| [15.1.2 · Compute signed construction-plane rotation](77sa-compute-signed-construction-plane-rotation.md) | 23 | 932 | 10–19 min | 0.5–1 h | Within planning limit |
| [15.1.3 · Address original controls and exclusive part selections](77sb-address-original-controls-and-exclusive-part-selections.md) | 82 | 2406 | 25–49 min | 0.5–1 h | Within planning limit |
| [15.1.4 · Collect finite original controls and mesh vertices](77sc-collect-finite-original-controls-and-mesh-vertices.md) | 50 | 1427 | 15–29 min | 0.5–1 h | Within planning limit |
| [15.1.5 · Connect curve polygons and surface control nets](77sd-connect-curve-polygons-and-surface-control-nets.md) | 61 | 1791 | 18–36 min | 0.5–1 h | Within planning limit |
| [15.1.6 · Reuse control collectors across original geometry families](77se-reuse-control-collectors-across-original-geometry-families.md) | 68 | 2455 | 25–50 min | 0.5–1 h | Within planning limit |
| [15.1.7 · Represent original geometry edit targets](77sf-represent-original-geometry-edit-targets.md) | 29 | 899 | 9–18 min | 0.5–1 h | Within planning limit |
| [15.1.8 · Resolve original mesh vertices for part edits](77sg-resolve-original-mesh-vertices-for-part-edits.md) | 43 | 1421 | 15–29 min | 0.5–1 h | Within planning limit |
| [15.1.9 · Resolve surface grids and individual source controls](77sh-resolve-surface-grids-and-individual-source-controls.md) | 34 | 1280 | 13–26 min | 0.5–1 h | Within planning limit |
| [15.1.10 · Collect affected points across CAD geometry families](77si-collect-affected-points-across-cad-geometry-families.md) | 52 | 2313 | 24–47 min | 0.5–1 h | Within planning limit |
| [15.1.11 · Preserve rational weights and validate BRep boundaries](77sj-preserve-rational-weights-and-validate-brep-boundaries.md) | 66 | 2282 | 23–46 min | 0.5–1 h | Within planning limit |
| [15.1.12 · Transform original mesh and surface controls](77sk-transform-original-mesh-and-surface-controls.md) | 36 | 1125 | 12–23 min | 0.5–1 h | Within planning limit |
| [15.1.13 · Transform original line polyline point and curve controls](77sl-transform-original-line-polyline-point-and-curve-controls.md) | 53 | 2022 | 21–41 min | 0.5–1 h | Within planning limit |
| [15.1.14 · Transform shared BRep boundary positions coherently](77sm-transform-shared-brep-boundary-positions-coherently.md) | 53 | 1848 | 19–37 min | 0.5–1 h | Within planning limit |
| [15.1.15 · Dispatch original part edits and retain Element ownership](77sn-dispatch-original-part-edits-and-retain-element-ownership.md) | 44 | 1933 | 20–39 min | 0.5–1 h | Within planning limit |
| [15.1.16 · Map sparse original vertex keys to display slots](77so-map-sparse-original-vertex-keys-to-display-slots.md) | 64 | 1739 | 18–35 min | 0.5–1 h | Within planning limit |
| [15.1.17 · Compute polygon normals from original boundaries](77sp-compute-polygon-normals-from-original-boundaries.md) | 41 | 1264 | 13–26 min | 0.5–1 h | Within planning limit |
| [15.1.18 · Build one original edge table and record incident faces](77sq-build-one-original-edge-table-and-record-incident-faces.md) | 52 | 1636 | 17–33 min | 0.5–1 h | Within planning limit |
| [15.1.19 · Preserve hole winding and complete original mesh topology](77sr-preserve-hole-winding-and-complete-original-mesh-topology.md) | 64 | 2118 | 22–43 min | 0.5–1 h | Within planning limit |
| [15.1.20 · Own transformation questions and the selected original targets](77a-own-transformation-questions-and-the-selected-original-targets.md) | 23 | 1698 | 17–34 min | 0.5–1 h | Within planning limit |
| [15.1.21 · Validate point and number transformation answers](77b-validate-point-and-number-transformation-answers.md) | 26 | 2959 | 30–60 min | 0.5–1 h | Within planning limit |
| [15.1.22 · Resolve typed batches and temporary transformation previews](77c-resolve-typed-batches-and-temporary-transformation-previews.md) | 12 | 1751 | 18–36 min | 0.5–1 h | Within planning limit |
| [15.1.23 · Resolve original CAD parts and complete selected bounds](77d-resolve-original-cad-parts-and-complete-selected-bounds.md) | 22 | 1857 | 19–38 min | 0.5–1 h | Within planning limit |
| [15.1.24 · Rebuild an edited original CAD source before adoption](77e-rebuild-an-edited-original-cad-source-before-adoption.md) | 12 | 1972 | 20–40 min | 0.5–1 h | Within planning limit |
| [15.1.25 · Copy original rows beside their existing parents](77f-copy-original-rows-beside-their-existing-parents.md) | 15 | 2246 | 23–45 min | 0.5–1 h | Within planning limit |
| [15.1.26 · Carry transformation state through the existing drawing owner](77g-carry-transformation-state-through-the-existing-drawing-owner.md) | 15 | 2894 | 29–58 min | 0.5–1 h | Within planning limit |
| [15.1.27 · Validate selected originals before beginning or committing a transform](77h-validate-selected-originals-before-beginning-or-committing-a-transform.md) | 15 | 1938 | 20–39 min | 0.5–1 h | Within planning limit |
| [15.1.28 · Adopt validated outcomes and preserve repeat copy transactions](77i-adopt-validated-outcomes-and-preserve-repeat-copy-transactions.md) | 19 | 2341 | 24–47 min | 0.5–1 h | Within planning limit |
| [15.1.29 · Route transform input cancellation and current plane changes](77j-route-transform-input-cancellation-and-current-plane-changes.md) | 8 | 932 | 10–19 min | 0.5–1 h | Within planning limit |
| [15.1.30 · Connect transformation lifecycle and part Move dispatch](77k-connect-transformation-lifecycle-and-part-move-dispatch.md) | 15 | 2674 | 27–54 min | 0.5–1 h | Within planning limit |
| [15.1.31 · Register Copy Scale Rotate and their current aliases](77l-register-copy-scale-rotate-and-their-current-aliases.md) | 21 | 2517 | 26–51 min | 0.5–1 h | Within planning limit |
| [15.1.32 · Apply direct Scale and Rotate about the current selection center](77m-apply-direct-scale-and-rotate-about-the-current-selection-center.md) | 7 | 1301 | 14–27 min | 0.5–1 h | Within planning limit |
| [15.1.33 · Retain earlier selection checks with source part support](77n-retain-earlier-selection-checks-with-source-part-support.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [15.1.34 · Exercise original copy transforms history and exact persistence](77o-exercise-original-copy-transforms-history-and-exact-persistence.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [15.1.35 · Describe curve gathering answers](77zza-describe-curve-gathering-answers.md) | 48 | 1710 | 18–35 min | 0.8–1 h | Within planning limit |
| [15.1.36 · Read curve gathering values](77zzb-read-curve-gathering-values.md) | 43 | 1758 | 18–36 min | 0.8–1 h | Within planning limit |
| [15.1.37 · Connect gathering recipes](77zzc-connect-gathering-recipes.md) | 65 | 2233 | 23–45 min | 0.8–1 h | Within planning limit |
| [15.1.38 · Parse gathering command words](77zzd-parse-gathering-command-words.md) | 46 | 1479 | 15–30 min | 0.8–1 h | Within planning limit |
| [15.1.39 · Track ordered curve picks](77zze-track-ordered-curve-picks.md) | 71 | 2171 | 22–44 min | 0.8–1 h | Within planning limit |
| [15.1.40 · Advance and cancel curve picks](77zzf-advance-and-cancel-curve-picks.md) | 59 | 2006 | 21–41 min | 0.8–1 h | Within planning limit |
| [15.1.41 · Prepare placed kernel curves](77zzg-prepare-placed-kernel-curves.md) | 48 | 1654 | 17–34 min | 0.8–1 h | Within planning limit |
| [15.1.42 · Check original loops and surface controls](77zzh-check-original-loops-and-surface-controls.md) | 87 | 2480 | 25–50 min | 0.8–1 h | Within planning limit |
| [15.1.43 · Align original sections and loft through them](77zzi-align-original-sections-and-loft-through-them.md) | 68 | 2226 | 23–45 min | 0.8–1 h | Within planning limit |
| [15.1.44 · Find extrusion axes and cap eligibility](77zzj-find-extrusion-axes-and-cap-eligibility.md) | 62 | 1816 | 19–37 min | 0.8–1 h | Within planning limit |
| [15.1.45 · Map cap curves to surface parameters](77zzk-map-cap-curves-to-surface-parameters.md) | 83 | 2993 | 30–60 min | 0.8–1 h | Within planning limit |
| [15.1.46 · Build closed curved extrusion topology](77zzl-build-closed-curved-extrusion-topology.md) | 72 | 2938 | 30–59 min | 0.8–1 h | Within planning limit |
| [15.1.47 · Extrude original corners and curves](77zzm-extrude-original-corners-and-curves.md) | 56 | 2143 | 22–43 min | 0.8–1 h | Within planning limit |
| [15.1.48 · Connect the original extrusion recipe](77zzn-connect-the-original-extrusion-recipe.md) | 74 | 1939 | 20–39 min | 0.8–1 h | Within planning limit |
| [15.1.49 · Revolve original profiles around an axis](77zzo-revolve-original-profiles-around-an-axis.md) | 72 | 1973 | 20–40 min | 0.8–1 h | Within planning limit |
| [15.2.1 · Copy current curve collection and surfacing recipes](78-copy-current-curve-collection-and-surfacing-recipes.md) | 6 | 74 | 1–2 min | 0.5–1 h | Within planning limit |
| [15.2.2 · Own picked originals and world profile copies](78a-own-picked-originals-and-world-profile-copies.md) | 21 | 2311 | 24–47 min | 0.5–1 h | Within planning limit |
| [15.2.3 · Resolve a recipe input and guard picked source freshness](78b-resolve-a-recipe-input-and-guard-picked-source-freshness.md) | 10 | 1587 | 16–32 min | 0.5–1 h | Within planning limit |
| [15.2.4 · Accept curve picks and geometric answers atomically](78c-accept-curve-picks-and-geometric-answers-atomically.md) | 6 | 1224 | 13–25 min | 0.5–1 h | Within planning limit |
| [15.2.5 · Read options and typed batches without partial results](78d-read-options-and-typed-batches-without-partial-results.md) | 14 | 1544 | 16–31 min | 0.5–1 h | Within planning limit |
| [15.2.6 · Constrain distance input and draw lifted profile wires](78e-constrain-distance-input-and-draw-lifted-profile-wires.md) | 8 | 1179 | 12–24 min | 0.5–1 h | Within planning limit |
| [15.2.7 · Carry gather state through the current tool owner](78f-carry-gather-state-through-the-current-tool-owner.md) | 10 | 2243 | 23–45 min | 0.5–1 h | Within planning limit |
| [15.2.8 · Adopt every generated original in one history transaction](78g-adopt-every-generated-original-in-one-history-transaction.md) | 7 | 1314 | 14–27 min | 0.5–1 h | Within planning limit |
| [15.2.9 · Start and advance the recipe through the existing Editor](78h-start-and-advance-the-recipe-through-the-existing-editor.md) | 9 | 1873 | 19–38 min | 0.5–1 h | Within planning limit |
| [15.2.10 · Route typed answers and axis cursor input](78i-route-typed-answers-and-axis-cursor-input.md) | 5 | 1420 | 15–29 min | 0.5–1 h | Within planning limit |
| [15.2.11 · Restore selection and preserve the gather lifecycle](78j-restore-selection-and-preserve-the-gather-lifecycle.md) | 9 | 2387 | 24–48 min | 0.5–1 h | Within planning limit |
| [15.2.12 · Register extrusion revolution and browser curve picking](78k-register-extrusion-revolution-and-browser-curve-picking.md) | 11 | 1241 | 13–25 min | 0.5–1 h | Within planning limit |
| [15.2.13 · Pick original profiles through the browser integer pass](78l-pick-original-profiles-through-the-browser-integer-pass.md) | 9 | 1872 | 19–38 min | 0.5–1 h | Within planning limit |
| [15.2.14 · Exercise profiles exact surfaces and one undo transaction](78m-exercise-profiles-exact-surfaces-and-one-undo-transaction.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [15.3.1 · Retain current loft section order and degree options](79-retain-current-loft-section-order-and-degree-options.md) | 76 | 2127 | 22–43 min | 0.5–1 h | Within planning limit |
| [15.3.2 · Frame original profiles on their rail](79aa-frame-original-profiles-on-their-rail.md) | 52 | 1587 | 16–32 min | 0.5–1 h | Within planning limit |
| [15.3.3 · Carry and blend original profiles along the rail](79ab-carry-and-blend-original-profiles-along-the-rail.md) | 69 | 2459 | 25–50 min | 0.5–1 h | Within planning limit |
| [15.3.4 · Connect original one-rail sweep inputs](79a-carry-original-profiles-along-one-rail.md) | 29 | 811 | 9–17 min | 0.5–1 h | Within planning limit |
| [15.3.5 · Align shape curves between two original rails](79b-align-shape-curves-between-two-original-rails.md) | 88 | 2368 | 24–48 min | 0.5–1 h | Within planning limit |
| [15.3.6 · Register loft sweep names options and aliases](79c-register-loft-sweep-names-options-and-aliases.md) | 4 | 901 | 10–19 min | 0.5–1 h | Within planning limit |
| [15.3.7 · Exercise original sections rail groups and exact persistence](79d-exercise-original-sections-rail-groups-and-exact-persistence.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [15.3.8 · Sample original network curve stations](79zza-sample-original-network-curve-stations.md) | 72 | 2259 | 23–46 min | 0.5–1 h | Within planning limit |
| [15.3.9 · Classify two original network directions](79zzb-classify-two-original-network-directions.md) | 73 | 2145 | 22–43 min | 0.5–1 h | Within planning limit |
| [15.3.10 · Order and orient original network curves](79zzc-order-and-orient-original-network-curves.md) | 42 | 1444 | 15–29 min | 0.5–1 h | Within planning limit |
| [15.3.11 · Sample original network lofts and tensor](79zzd-sample-original-network-lofts-and-tensor.md) | 78 | 2716 | 28–55 min | 0.5–1 h | Within planning limit |
| [15.3.12 · Fit the original Gordon surface](79zze-fit-the-original-gordon-surface.md) | 48 | 1685 | 17–34 min | 0.5–1 h | Within planning limit |
| [15.4.1 · Build a surface through original curve networks](80-build-a-surface-through-two-directions-of-original-curves.md) | 57 | 1631 | 17–33 min | 0.5–1 h | Within planning limit |
| [15.4.2 · Construct the patch through four ordered corners](80a-construct-the-patch-through-four-ordered-corners.md) | 48 | 1529 | 16–31 min | 0.5–1 h | Within planning limit |
| [15.4.3 · Guide corner questions without retaining unrelated profiles](80b-guide-corner-questions-without-retaining-unrelated-profiles.md) | 2 | 666 | 7–14 min | 0.5–1 h | Within planning limit |
| [15.4.4 · Register network patch aliases and initial corner arity](80c-register-network-patch-aliases-and-initial-corner-arity.md) | 4 | 1710 | 18–35 min | 0.5–1 h | Within planning limit |
| [15.4.5 · Exercise networks corner retries and exact original archives](80d-exercise-networks-corner-retries-and-exact-original-archives.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [15.4.6 · Read original projection planes](80za-read-original-projection-planes.md) | 56 | 1933 | 20–39 min | 0.5–1 h | Within planning limit |
| [16.1.1 · Project originals in their placed frame](81-project-originals-through-the-current-plane-algorithms.md) | 77 | 2647 | 27–53 min | 0.5–1 h | Within planning limit |
| [16.1.2 · Read original cutters in object frames](81zza-read-original-cutters-in-object-frames.md) | 71 | 2635 | 27–53 min | 0.5–1 h | Within planning limit |
| [16.1.3 · Find original cutters near the pointer](81zzb-find-original-cutters-near-the-pointer.md) | 67 | 2410 | 25–49 min | 0.5–1 h | Within planning limit |
| [16.1.4 · Grow original domains and extract the tail](81zzc-grow-original-domains-and-extract-the-tail.md) | 80 | 2230 | 23–45 min | 0.5–1 h | Within planning limit |
| [16.1.5 · Find original end directions and length reach](81zzd-find-original-end-directions-and-length-reach.md) | 75 | 2554 | 26–52 min | 0.5–1 h | Within planning limit |
| [16.1.6 · Stop straight growth at the first boundary](81zze-stop-straight-growth-at-the-first-boundary.md) | 58 | 2186 | 22–44 min | 0.5–1 h | Within planning limit |
| [16.1.7 · Stop curved growth at an original boundary](81zzf-stop-curved-growth-at-an-original-boundary.md) | 83 | 2980 | 30–60 min | 0.5–1 h | Within planning limit |
| [16.1.8 · Dispatch original extension families](81zzg-dispatch-original-extension-families.md) | 55 | 2080 | 21–42 min | 0.5–1 h | Within planning limit |
| [16.1.9 · Edit original curve intervals](81a-find-source-cutters-and-current-extension-reach.md) | 63 | 2157 | 22–44 min | 0.5–1 h | Within planning limit |
| [16.1.10 · Capture eligible original identity and placement](81b-capture-eligible-original-identity-and-placement.md) | 59 | 2279 | 23–46 min | 0.5–1 h | Within planning limit |
| [16.1.11 · Replace an original without replacing its row](81c-replace-an-original-without-replacing-its-row.md) | 19 | 3000 | 30–60 min | 0.5–1 h | Within planning limit |
| [16.1.12 · Own the originals selected for projection](81d-own-the-originals-selected-for-projection.md) | 39 | 1070 | 11–22 min | 0.5–1 h | Within planning limit |
| [16.1.13 · Validate candidate plane answers before accepting them](81e-validate-candidate-plane-answers-before-accepting-them.md) | 28 | 1034 | 11–21 min | 0.5–1 h | Within planning limit |
| [16.1.14 · Prepare projection results as one transaction](81f-prepare-projection-results-as-one-transaction.md) | 35 | 1397 | 14–28 min | 0.5–1 h | Within planning limit |
| [16.1.15 · Own extension candidates and screen bounds](81g-own-extension-candidates-and-screen-bounds.md) | 68 | 1796 | 18–36 min | 0.5–1 h | Within planning limit |
| [16.1.16 · Gather boundaries and sample eligible curve owners](81h-gather-boundaries-and-sample-eligible-curve-owners.md) | 50 | 1868 | 19–38 min | 0.5–1 h | Within planning limit |
| [16.1.17 · Cache current boundary meshes and start the end phase](81i-cache-current-boundary-meshes-and-start-the-end-phase.md) | 75 | 2707 | 28–55 min | 0.5–1 h | Within planning limit |
| [16.1.18 · Resolve distance and boundary reach in source coordinates](81j-resolve-distance-and-boundary-reach-in-source-coordinates.md) | 74 | 2825 | 29–57 min | 0.5–1 h | Within planning limit |
| [16.1.19 · Find the nearest visible curve end in CSS pixels](81k-find-the-nearest-visible-curve-end-in-css-pixels.md) | 38 | 1602 | 17–33 min | 0.5–1 h | Within planning limit |
| [16.1.20 · Preview extension and refresh only changed originals](81l-preview-extension-and-refresh-only-changed-originals.md) | 83 | 2804 | 29–57 min | 0.5–1 h | Within planning limit |
| [16.1.21 · Keep projection and extension in the pending tool owner](81m-keep-projection-and-extension-in-the-pending-tool-owner.md) | 10 | 2537 | 26–51 min | 0.5–1 h | Within planning limit |
| [16.1.22 · Connect checked projection to one editor transaction](81n-connect-checked-projection-to-one-editor-transaction.md) | 7 | 2122 | 22–43 min | 0.5–1 h | Within planning limit |
| [16.1.23 · Connect boundary picks repeated end clicks and cancellation](81o-connect-boundary-picks-repeated-end-clicks-and-cancellation.md) | 8 | 2702 | 28–55 min | 0.5–1 h | Within planning limit |
| [16.1.24 · Expose editing actions and normalized interval extension](81p-expose-editing-actions-and-normalized-interval-extension.md) | 6 | 1029 | 11–21 min | 0.5–1 h | Within planning limit |
| [16.1.25 · Route answers to the active editing owner](81q-route-accepted-editing-input-and-owner-lifecycle-1.md) | 9 | 1683 | 17–34 min | 0.5–1 h | Within planning limit |
| [16.1.26 · Finish a pending edit through one commit path](81r-route-accepted-editing-input-and-owner-lifecycle-2.md) | 6 | 2256 | 23–46 min | 0.5–1 h | Within planning limit |
| [16.1.27 · Restore source selection when an edit is cancelled](81s-route-accepted-editing-input-and-owner-lifecycle-3.md) | 1 | 644 | 7–13 min | 0.5–1 h | Within planning limit |
| [16.1.28 · Register current projection and extension commands](81t-register-current-projection-and-extension-commands.md) | 65 | 2692 | 27–54 min | 0.5–1 h | Within planning limit |
| [16.1.29 · Route whole boundary picks and observe pending edits](81u-route-whole-boundary-picks-and-observe-pending-edits.md) | 7 | 2254 | 23–46 min | 0.5–1 h | Within planning limit |
| [16.1.30 · Exercise original projection retry and repeated extension](81v-exercise-original-projection-retry-and-repeated-extension.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [16.1.31 · Validate original Split inputs and name pieces](81za-validate-original-split-inputs-and-name-pieces.md) | 73 | 2660 | 27–54 min | 0.5–1 h | Within planning limit |
| [16.2.1 · Divide original curves and joined faces](82-divide-current-original-curve-families.md) | 71 | 2681 | 27–54 min | 0.5–1 h | Within planning limit |
| [16.2.2 · Own the split target and toggle original cutters](82a-own-the-split-target-and-toggle-original-cutters.md) | 43 | 1478 | 15–30 min | 0.5–1 h | Within planning limit |
| [16.2.3 · Transform cutters into the target frame before dividing](82b-transform-cutters-into-the-target-frame-before-dividing.md) | 30 | 1133 | 12–23 min | 0.5–1 h | Within planning limit |
| [16.2.4 · Adopt sibling pieces beside the original target](82c-adopt-sibling-pieces-beside-the-original-target.md) | 35 | 1050 | 11–21 min | 0.5–1 h | Within planning limit |
| [16.2.5 · Connect pending split completion to one history change](82d-connect-pending-split-completion-to-one-history-change.md) | 15 | 2052 | 21–42 min | 0.5–1 h | Within planning limit |
| [16.2.6 · Route cutter picks cancellation and temporary highlights](82e-route-cutter-picks-cancellation-and-temporary-highlights.md) | 6 | 1904 | 20–39 min | 0.5–1 h | Within planning limit |
| [16.2.7 · Register Split and route mouse touch and confirmation](82f-register-split-and-route-mouse-touch-and-confirmation.md) | 10 | 2703 | 28–55 min | 0.5–1 h | Within planning limit |
| [16.2.8 · Exercise curve pieces source inheritance and one-step Undo](82g-exercise-curve-pieces-source-inheritance-and-one-step-undo.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [16.3.1 · Split original faces without separating the solid](83-split-original-faces-without-separating-the-solid.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [16.3.2 · Visit original mesh triangles and format measures](83za-visit-original-mesh-triangles-and-format-measures.md) | 63 | 1796 | 18–36 min | 0.5–1 h | Within planning limit |
| [16.3.3 · Compute original mesh measures and curve length](83zb-compute-original-mesh-measures-and-curve-length.md) | 71 | 2319 | 24–47 min | 0.5–1 h | Within planning limit |
| [16.3.4 · Integrate original surface area](83ze-integrate-original-surface-area.md) | 69 | 2737 | 28–55 min | 0.5–1 h | Within planning limit |
| [16.4.1 · Measure original closed-solid volume](84-measure-original-curves-surfaces-and-closed-solids.md) | 62 | 2314 | 24–47 min | 0.5–1 h | Within planning limit |
| [16.4.2 · Validate an original box and derive wire-only display](84a-validate-an-original-box-and-derive-wire-only-display.md) | 76 | 2330 | 24–47 min | 0.5–1 h | Within planning limit |
| [16.4.3 · Adopt original boxes and retain precise placed bounds](84b-adopt-original-boxes-and-retain-precise-placed-bounds.md) | 43 | 2623 | 27–53 min | 0.5–1 h | Within planning limit |
| [16.4.4 · Save and reopen the original box frame](84c-save-and-reopen-the-original-box-frame.md) | 8 | 1172 | 12–24 min | 0.5–1 h | Within planning limit |
| [16.4.5 · Own temporary measurement labels without scene identities](84d-own-temporary-measurement-labels-without-scene-identities.md) | 84 | 2354 | 24–48 min | 0.5–1 h | Within planning limit |
| [16.4.6 · Report distance and world bounding lengths](84e-report-distance-and-world-bounding-lengths.md) | 41 | 1216 | 13–25 min | 0.5–1 h | Within planning limit |
| [16.4.7 · Collect only supported finite original measurements](84f-collect-only-supported-finite-original-measurements.md) | 64 | 2130 | 22–43 min | 0.5–1 h | Within planning limit |
| [16.4.8 · Format totals refusals and three-axis box extents](84g-format-totals-refusals-and-three-axis-box-extents.md) | 85 | 2733 | 28–55 min | 0.5–1 h | Within planning limit |
| [16.4.9 · Keep measurements outside geometry history](84h-keep-measurements-outside-geometry-history.md) | 6 | 1856 | 19–38 min | 0.5–1 h | Within planning limit |
| [16.4.10 · Accept two distance points through the drawing tool](84i-accept-two-distance-points-through-the-drawing-tool.md) | 12 | 2518 | 26–51 min | 0.5–1 h | Within planning limit |
| [16.4.11 · Register measurements and end temporary marks predictably](84j-register-measurements-and-end-temporary-marks-predictably.md) | 9 | 991 | 10–20 min | 0.5–1 h | Within planning limit |
| [16.4.12 · Draw cached unpickable measurement strokes](84k-draw-cached-unpickable-measurement-strokes.md) | 49 | 1786 | 18–36 min | 0.5–1 h | Within planning limit |
| [16.4.13 · Merge measurement chips with cached scene text](84l-merge-measurement-chips-with-cached-scene-text.md) | 11 | 2454 | 25–50 min | 0.5–1 h | Within planning limit |
| [16.4.14 · Exercise original boxes and temporary distance annotations](84m-exercise-original-boxes-and-temporary-distance-annotations.md) | 6 | 1090 | 11–22 min | 0.5–1 h | Within planning limit |
| [16.4.15 · Describe removable CAD parts](84zza-describe-removable-cad-parts.md) | 78 | 2625 | 27–53 min | 0.8–1 h | Within planning limit |
| [16.4.16 · Read trim part previews](84zzb-read-trim-part-previews.md) | 55 | 1915 | 20–39 min | 0.8–1 h | Within planning limit |
| [16.4.17 · Collect curve crossings](84zzc-collect-curve-crossings.md) | 68 | 2069 | 21–42 min | 0.8–1 h | Within planning limit |
| [16.4.18 · Preserve trimmed source types](84zzd-preserve-trimmed-source-types.md) | 75 | 2615 | 27–53 min | 0.8–1 h | Within planning limit |
| [16.4.19 · Group faces by cutting planes](84zze-group-faces-by-cutting-planes.md) | 62 | 1942 | 20–39 min | 0.8–1 h | Within planning limit |
| [16.4.20 · Cut planar BRep faces](84zzf-cut-planar-brep-faces.md) | 73 | 2268 | 23–46 min | 0.8–1 h | Within planning limit |
| [16.4.21 · Assemble removable face groups](84zzg-assemble-removable-face-groups.md) | 95 | 2884 | 29–58 min | 0.8–1 h | Within planning limit |
| [16.4.22 · Mark retained BRep topology](84zzh-mark-retained-brep-topology.md) | 76 | 2141 | 22–43 min | 0.8–1 h | Within planning limit |
| [16.4.23 · Rebuild retained BRep tables](84zzi-rebuild-retained-brep-tables.md) | 92 | 2884 | 29–58 min | 0.8–1 h | Within planning limit |
| [16.4.24 · Split mesh faces at a plane](84zzj-split-mesh-faces-at-a-plane.md) | 92 | 2873 | 29–58 min | 0.8–1 h | Within planning limit |
| [16.4.25 · Compact both trimmed mesh sides](84zzk-compact-both-trimmed-mesh-sides.md) | 27 | 890 | 9–18 min | 0.8–1 h | Within planning limit |
| [16.4.26 · Rebuild kept CAD parts](84zzl-rebuild-kept-cad-parts.md) | 63 | 2129 | 22–43 min | 0.8–1 h | Within planning limit |
| [16.4.27 · Connect original trim recipes](84zzm-connect-original-trim-recipes.md) | 55 | 1922 | 20–39 min | 0.8–1 h | Within planning limit |
| [16.4.28 · Index selection loops by pixel row](84zna-index-selection-loops-by-pixel-row.md) | 76 | 2772 | 28–56 min | 0.8–1 h | Within planning limit |
| [16.4.29 · Test window membership and segment crossings](84znb-test-window-membership-and-segment-crossings.md) | 44 | 1317 | 14–27 min | 0.8–1 h | Within planning limit |
| [16.4.30 · Sample original CAD families for selection](84znc-sample-original-cad-families-for-selection.md) | 70 | 2376 | 24–48 min | 0.8–1 h | Within planning limit |
| [16.4.31 · Collect original boundary strokes](84znd-collect-original-boundary-strokes.md) | 48 | 1683 | 17–34 min | 0.8–1 h | Within planning limit |
| [16.4.32 · Test crossing selection on original strokes](84zne-test-crossing-selection-on-original-strokes.md) | 66 | 2374 | 24–48 min | 0.8–1 h | Within planning limit |
| [16.4.33 · Connect window and bounds selection tests](84znf-connect-window-and-bounds-selection-tests.md) | 63 | 2569 | 26–52 min | 0.8–1 h | Within planning limit |
| [16.4.34 · Classify originals and isolate one mesh face](84zoa-classify-originals-and-isolate-one-mesh-face.md) | 90 | 2670 | 27–54 min | 0.5–1 h | Within planning limit |
| [16.4.35 · Copy original BRep edges and pcurves](84zob-copy-original-brep-edges-and-pcurves.md) | 52 | 1772 | 18–36 min | 0.5–1 h | Within planning limit |
| [16.4.36 · Build one original BRep face](84zoc-build-one-original-brep-face.md) | 38 | 1542 | 16–31 min | 0.5–1 h | Within planning limit |
| [16.4.37 · Explode supported original families](84zod-explode-supported-original-families.md) | 66 | 2369 | 24–48 min | 0.5–1 h | Within planning limit |
| [16.5.1 · Write original text placement and curve heads](85-copy-current-original-command-recipes.md) | 88 | 2645 | 27–53 min | 0.5–1 h | Within planning limit |
| [16.5.2 · Validate original Plane display and precise bounds](85a-validate-original-plane-display-and-precise-bounds.md) | 23 | 2288 | 23–46 min | 0.5–1 h | Within planning limit |
| [16.5.3 · Retain original Plane and Text edit eligibility](85b-retain-original-plane-and-text-edit-eligibility.md) | 10 | 658 | 7–14 min | 0.5–1 h | Within planning limit |
| [16.5.4 · Find eligible objects by name or world size](85c-find-eligible-objects-by-name-or-world-size.md) | 13 | 1156 | 12–24 min | 0.5–1 h | Within planning limit |
| [16.5.5 · Plan sibling pieces from original owners](85d-plan-sibling-pieces-from-original-owners.md) | 21 | 1402 | 15–29 min | 0.5–1 h | Within planning limit |
| [16.5.6 · Own Text height and words before placement](85e-own-text-height-and-words-before-placement.md) | 11 | 1681 | 17–34 min | 0.5–1 h | Within planning limit |
| [16.5.7 · Own unpickable cached guide data](85f-own-unpickable-cached-guide-data.md) | 11 | 796 | 8–16 min | 0.5–1 h | Within planning limit |
| [16.5.8 · Shade explicit dashed guides](85g-shade-explicit-dashed-guides.md) | 52 | 2852 | 29–58 min | 0.5–1 h | Within planning limit |
| [16.5.9 · Cache guide buffers and actual allocation](85h-cache-guide-buffers-and-actual-allocation.md) | 3 | 1042 | 11–21 min | 0.5–1 h | Within planning limit |
| [16.5.10 · Draw cached guides through the overlay owner](85i-draw-cached-guides-through-the-overlay-owner.md) | 8 | 892 | 9–18 min | 0.5–1 h | Within planning limit |
| [16.5.11 · Own Trim targets cutters and part state](85j-own-trim-targets-cutters-and-part-state.md) | 34 | 2890 | 29–58 min | 0.5–1 h | Within planning limit |
| [16.5.12 · Prepare local cuts from world cutters](85k-prepare-local-cuts-from-world-cutters.md) | 23 | 2137 | 22–43 min | 0.5–1 h | Within planning limit |
| [16.5.13 · Hit remaining trim parts in canvas pixels](85l-hit-remaining-trim-parts-in-canvas-pixels.md) | 7 | 1291 | 13–26 min | 0.5–1 h | Within planning limit |
| [16.5.14 · Preview cuts without changing originals](85m-preview-cuts-without-changing-originals.md) | 9 | 1166 | 12–24 min | 0.5–1 h | Within planning limit |
| [16.5.15 · Show remaining cuts and removed-part guides](85n-show-remaining-cuts-and-removed-part-guides.md) | 1 | 860 | 9–18 min | 0.5–1 h | Within planning limit |
| [16.5.16 · Sample bounded physical-pixel lasso loops](85o-sample-bounded-physical-pixel-lasso-loops.md) | 41 | 2139 | 22–43 min | 0.5–1 h | Within planning limit |
| [16.5.17 · Own document-scoped groups and links](85p-own-document-scoped-groups-and-links.md) | 3 | 498 | 5–10 min | 0.5–1 h | Within planning limit |
| [16.5.18 · Resolve current graph and group identities](85q-resolve-current-graph-and-group-identities.md) | 26 | 1383 | 14–28 min | 0.5–1 h | Within planning limit |
| [16.5.19 · Connect two original owners and save their links](85r-connect-two-original-owners-and-save-their-links.md) | 14 | 1537 | 16–31 min | 0.5–1 h | Within planning limit |
| [16.5.20 · Remap appended links and remove deleted owners](85s-remap-appended-links-and-remove-deleted-owners.md) | 12 | 1533 | 16–31 min | 0.5–1 h | Within planning limit |
| [16.5.21 · Plan world-preserving original groups](85t-plan-world-preserving-original-groups.md) | 25 | 2214 | 23–45 min | 0.5–1 h | Within planning limit |
| [16.5.22 · Adopt group branches with one reversible edit](85u-adopt-group-branches-with-one-reversible-edit.md) | 12 | 1766 | 18–36 min | 0.5–1 h | Within planning limit |
| [16.5.23 · Append namespaced graphs and preserve original row identities](85v-append-namespaced-graphs-and-preserve-original-row-identities.md) | 7 | 1103 | 12–23 min | 0.5–1 h | Within planning limit |
| [16.5.24 · Replace original families without changing world placement](85w-replace-original-families-without-changing-world-placement.md) | 8 | 1872 | 19–38 min | 0.5–1 h | Within planning limit |
| [16.5.25 · Save group namespaces and original planes](85x-save-group-namespaces-and-original-planes.md) | 11 | 1787 | 18–36 min | 0.5–1 h | Within planning limit |
| [16.5.26 · Keep named views and grid on the drawing plane](85y-keep-named-views-and-grid-on-the-drawing-plane.md) | 53 | 2714 | 28–55 min | 0.5–1 h | Within planning limit |
| [16.5.27 · Connect cached guide and placed grid lanes](85z-connect-cached-guide-and-placed-grid-lanes.md) | 15 | 2781 | 28–56 min | 0.5–1 h | Within planning limit |
| [16.5.28 · Expose graph links and the lasso overlay](85aa-expose-graph-links-and-the-lasso-overlay.md) | 9 | 2443 | 25–49 min | 0.5–1 h | Within planning limit |
| [16.5.29 · Own current pending command state](85ab-own-current-pending-command-state.md) | 12 | 877 | 9–18 min | 0.5–1 h | Within planning limit |
| [16.5.30 · Retain Text answers inside the existing point draft](85ac-retain-text-answers-inside-the-existing-point-draft.md) | 3 | 1122 | 12–23 min | 0.5–1 h | Within planning limit |
| [16.5.31 · Complete original Text and headed Arrow tools](85ad-complete-original-text-and-headed-arrow-tools.md) | 10 | 2632 | 27–53 min | 0.5–1 h | Within planning limit |
| [16.5.32 · Present cached extension guides and error endpoints](85ae-present-cached-extension-guides-and-error-endpoints.md) | 13 | 1550 | 16–31 min | 0.5–1 h | Within planning limit |
| [16.5.33 · Commit prepared trims once](85af-commit-prepared-trims-once.md) | 7 | 903 | 10–19 min | 0.5–1 h | Within planning limit |
| [16.5.34 · Advance Trim through targets cutters and parts](85ag-advance-trim-through-targets-cutters-and-parts.md) | 14 | 1534 | 16–31 min | 0.5–1 h | Within planning limit |
| [16.5.35 · Adopt original pieces and arrowhead changes atomically](85ah-adopt-original-pieces-and-arrowhead-changes-atomically.md) | 9 | 1133 | 12–23 min | 0.5–1 h | Within planning limit |
| [16.5.36 · Dispatch current original commands through checked owners](85ai-dispatch-current-original-commands-through-checked-owners.md) | 14 | 2255 | 23–46 min | 0.5–1 h | Within planning limit |
| [16.5.37 · Preserve previews selection and accepted tool answers](85aj-preserve-previews-selection-and-accepted-tool-answers.md) | 9 | 1786 | 18–36 min | 0.5–1 h | Within planning limit |
| [16.5.38 · Keep annotations during touch navigation and capture lasso input](85ak-keep-annotations-during-touch-navigation-and-capture-lasso-input.md) | 7 | 2753 | 28–56 min | 0.5–1 h | Within planning limit |
| [16.5.39 · Preserve pending originals during view changes](85al-preserve-pending-originals-during-view-changes.md) | 3 | 1866 | 19–38 min | 0.5–1 h | Within planning limit |
| [16.5.40 · Route named views and whole trim picks](85am-route-named-views-and-whole-trim-picks.md) | 8 | 2417 | 25–49 min | 0.5–1 h | Within planning limit |
| [16.5.41 · Capture source-backed command arguments](85an-capture-source-backed-command-arguments.md) | 18 | 1874 | 19–38 min | 0.5–1 h | Within planning limit |
| [16.5.42 · Restore typed requests and their selected owners](85ao-restore-typed-requests-and-their-selected-owners.md) | 6 | 1505 | 16–31 min | 0.5–1 h | Within planning limit |
| [16.5.43 · Witness identities and placement during source loading](85ap-witness-identities-and-placement-during-source-loading.md) | 8 | 828 | 9–17 min | 0.5–1 h | Within planning limit |
| [16.5.44 · Deduplicate original fetches and replay safely](85aq-deduplicate-original-fetches-and-replay-safely.md) | 7 | 958 | 10–20 min | 0.5–1 h | Within planning limit |
| [16.5.45 · Route source-backed edits through guarded replay](85ar-route-source-backed-edits-through-guarded-replay.md) | 13 | 1124 | 12–23 min | 0.5–1 h | Within planning limit |
| [16.5.46 · Register current names views and command aliases](85as-register-current-names-views-and-command-aliases.md) | 30 | 2103 | 22–43 min | 0.5–1 h | Within planning limit |
| [16.5.47 · Validate current command words and options](85at-validate-current-command-words-and-options.md) | 9 | 2079 | 21–42 min | 0.5–1 h | Within planning limit |
| [16.5.48 · Capture lasso pointer ownership in the browser](85au-capture-lasso-pointer-ownership-in-the-browser.md) | 7 | 1148 | 12–23 min | 0.5–1 h | Within planning limit |
| [16.5.49 · Route trim and lasso pointer input](85av-route-trim-and-lasso-pointer-input.md) | 7 | 2371 | 24–48 min | 0.5–1 h | Within planning limit |
| [16.5.50 · Refresh command panels source loading and cached previews](85aw-refresh-command-panels-source-loading-and-cached-previews.md) | 14 | 2619 | 27–53 min | 0.5–1 h | Within planning limit |
| [16.5.51 · Observe current original command state](85ax-observe-current-original-command-state.md) | 3 | 727 | 8–15 min | 0.5–1 h | Within planning limit |
| [16.5.52 · Exercise current original commands together](85ay-exercise-current-original-commands-together.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [17.1.1 · Bound the physical tile grid and reference storage](86-copy-finite-tile-allocation-rules.md) | 75 | 2258 | 23–46 min | 0.5–1 h | Within planning limit |
| [17.1.2 · Grow projection and reference pools from demand](86ma-grow-projection-and-reference-pools-from-demand.md) | 40 | 1156 | 12–24 min | 0.5–1 h | Within planning limit |
| [17.1.3 · Define one shared projected-triangle coverage contract](86a-copy-maintained-projection-and-scan-recipes.md) | 52 | 1886 | 19–38 min | 0.5–1 h | Within planning limit |
| [17.1.4 · Declare finite projection buffers and original vertex access](86b-copy-the-project-recipe.md) | 23 | 1202 | 13–25 min | 0.5–1 h | Within planning limit |
| [17.1.5 · Clip original triangles before dividing by depth](86ba-clip-original-triangles-before-dividing-by-depth.md) | 69 | 2377 | 24–48 min | 0.5–1 h | Within planning limit |
| [17.1.6 · Write bounded projected triangle records on the GPU](86bb-write-bounded-projected-triangle-records-on-the-gpu.md) | 72 | 2693 | 27–54 min | 0.5–1 h | Within planning limit |
| [17.1.7 · Assemble and validate the shared finite projection shader](86c-project-existing-original-mesh-buffers.md) | 23 | 1002 | 11–21 min | 0.5–1 h | Within planning limit |
| [17.1.8 · Declare atomic tile records and flat triangle inputs](86d-copy-the-triangle-tiles-recipe.md) | 40 | 2006 | 21–41 min | 0.5–1 h | Within planning limit |
| [17.1.9 · Count conservative triangle coverage per tile](86da-count-conservative-triangle-coverage-per-tile.md) | 50 | 2084 | 21–42 min | 0.5–1 h | Within planning limit |
| [17.1.10 · Fill bounded original triangle reference lists](86db-fill-bounded-original-triangle-reference-lists.md) | 29 | 1135 | 12–23 min | 0.5–1 h | Within planning limit |
| [17.1.11 · Assemble and validate the bounded tiles shader](86e-count-and-fill-original-triangle-references.md) | 6 | 251 | 3–6 min | 0.5–1 h | Within planning limit |
| [17.1.12 · Scan bounded tile counts in workgroup memory](86f-copy-the-scan-triangle-tiles-recipe.md) | 47 | 1559 | 16–32 min | 0.5–1 h | Within planning limit |
| [17.1.13 · Scan tile blocks and their block totals](86fa-scan-tile-blocks-and-their-block-totals.md) | 42 | 1146 | 12–23 min | 0.5–1 h | Within planning limit |
| [17.1.14 · Finalize bounded tile list offsets and demand](86fb-finalize-bounded-tile-list-offsets-and-demand.md) | 34 | 988 | 10–20 min | 0.5–1 h | Within planning limit |
| [17.1.15 · Assemble and validate the bounded scan shader](86fc-assemble-and-validate-the-bounded-scan-shader.md) | 6 | 252 | 3–6 min | 0.5–1 h | Within planning limit |
| [17.1.16 · Wake only after a tile report arrives](86g-wake-only-after-a-tile-report-arrives.md) | 3 | 164 | 2–4 min | 0.5–1 h | Within planning limit |
| [17.1.17 · Read actual pool demand asynchronously](86h-read-actual-pool-demand-asynchronously.md) | 85 | 2566 | 26–52 min | 0.5–1 h | Within planning limit |
| [17.1.18 · Describe explicit finite GPU layouts](86i-describe-explicit-finite-gpu-layouts.md) | 11 | 412 | 5–9 min | 0.5–1 h | Within planning limit |
| [17.1.19 · Build compute and count pipeline helpers](86j-build-compute-and-count-pipeline-helpers.md) | 4 | 1968 | 20–40 min | 0.5–1 h | Within planning limit |
| [17.1.20 · Connect projection count scan and fill pipelines](86k-connect-projection-count-scan-and-fill-pipelines.md) | 13 | 1734 | 18–35 min | 0.5–1 h | Within planning limit |
| [17.1.21 · Own bounded finite tile resources](86l-own-bounded-finite-tile-resources.md) | 20 | 1187 | 12–24 min | 0.5–1 h | Within planning limit |
| [17.1.22 · Allocate empty tile placeholders](86m-allocate-empty-tile-placeholders.md) | 6 | 852 | 9–18 min | 0.5–1 h | Within planning limit |
| [17.1.23 · Witness original buffers and world placement](86n-witness-original-buffers-and-world-placement.md) | 15 | 1381 | 14–28 min | 0.5–1 h | Within planning limit |
| [17.1.24 · Release obsolete tile lists and bindings](86o-release-obsolete-tile-lists-and-bindings.md) | 2 | 580 | 6–12 min | 0.5–1 h | Within planning limit |
| [17.1.25 · Grow lists only from actual demand](86p-grow-lists-only-from-actual-demand.md) | 14 | 1879 | 19–38 min | 0.5–1 h | Within planning limit |
| [17.1.26 · Rasterize bounded tile counts and references](86q-rasterize-bounded-tile-counts-and-references.md) | 1 | 680 | 7–14 min | 0.5–1 h | Within planning limit |
| [17.1.27 · Cache projection and finish current reports](86r-cache-projection-and-finish-current-reports.md) | 10 | 1742 | 18–35 min | 0.5–1 h | Within planning limit |
| [17.1.28 · Test visibility at the original center line](86s-test-visibility-at-the-original-center-line.md) | 23 | 2242 | 23–45 min | 0.5–1 h | Within planning limit |
| [17.1.29 · Reuse original stroke coverage](86t-reuse-original-stroke-coverage.md) | 9 | 760 | 8–16 min | 0.5–1 h | Within planning limit |
| [17.1.30 · Carry stroke axis and depth](86u-carry-stroke-axis-and-depth.md) | 11 | 764 | 8–16 min | 0.5–1 h | Within planning limit |
| [17.1.31 · Reuse original stroke integer coverage](86v-reuse-original-stroke-integer-coverage.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [17.1.32 · Keep stroke crop ownership aligned](86w-keep-stroke-crop-ownership-aligned.md) | 7 | 574 | 6–12 min | 0.5–1 h | Within planning limit |
| [17.1.33 · Reuse original chain coverage](86x-reuse-original-chain-coverage.md) | 5 | 374 | 4–8 min | 0.5–1 h | Within planning limit |
| [17.1.34 · Carry chain axis and depth](86y-carry-chain-axis-and-depth.md) | 11 | 863 | 9–18 min | 0.5–1 h | Within planning limit |
| [17.1.35 · Reuse original chain integer coverage](86z-reuse-original-chain-integer-coverage.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [17.1.36 · Keep chain crop ownership aligned](86aa-keep-chain-crop-ownership-aligned.md) | 7 | 637 | 7–13 min | 0.5–1 h | Within planning limit |
| [17.1.37 · Reuse original marker coverage](86ab-reuse-original-marker-coverage.md) | 5 | 376 | 4–8 min | 0.5–1 h | Within planning limit |
| [17.1.38 · Carry marker axis and depth](86ac-carry-marker-axis-and-depth.md) | 12 | 817 | 9–17 min | 0.5–1 h | Within planning limit |
| [17.1.39 · Reuse original marker integer coverage](86ad-reuse-original-marker-integer-coverage.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [17.1.40 · Keep marker crop ownership aligned](86ae-keep-marker-crop-ownership-aligned.md) | 7 | 572 | 6–12 min | 0.5–1 h | Within planning limit |
| [17.1.41 · Assemble one shared finite shader predicate](86af-assemble-one-shared-finite-shader-predicate.md) | 9 | 940 | 10–19 min | 0.5–1 h | Within planning limit |
| [17.1.42 · Own finite color and integer pipelines](86ag-own-finite-color-and-integer-pipelines.md) | 14 | 335 | 4–7 min | 0.5–1 h | Within planning limit |
| [17.1.43 · Build finite shader pipelines with original layouts](86ah-build-finite-shader-pipelines-with-original-layouts.md) | 7 | 1878 | 19–38 min | 0.5–1 h | Within planning limit |
| [17.1.44 · Bind finite coverage for original families](86ai-bind-finite-coverage-for-original-families.md) | 7 | 1805 | 19–37 min | 0.5–1 h | Within planning limit |
| [17.1.45 · Reuse bindings and draw original owners](86aj-reuse-bindings-and-draw-original-owners.md) | 3 | 1250 | 13–25 min | 0.5–1 h | Within planning limit |
| [17.1.46 · Expose original GPU models to projection](86ak-expose-original-gpu-models-to-projection.md) | 1 | 101 | 2–3 min | 0.5–1 h | Within planning limit |
| [17.1.47 · Connect finite visibility to all original ink lanes](86al-connect-finite-visibility-to-all-original-ink-lanes.md) | 12 | 1711 | 18–35 min | 0.5–1 h | Within planning limit |
| [17.1.48 · Separate integer depth writers and sampled ink](86am-separate-integer-depth-writers-and-sampled-ink.md) | 13 | 2778 | 28–56 min | 0.5–1 h | Within planning limit |
| [17.1.49 · Own tile state before color and first picks](86an-own-tile-state-before-color-and-first-picks.md) | 15 | 2151 | 22–44 min | 0.5–1 h | Within planning limit |
| [17.1.50 · Split opaque finite ink and cloud composition](86ao-split-opaque-finite-ink-and-cloud-composition.md) | 7 | 1605 | 17–33 min | 0.5–1 h | Within planning limit |
| [17.1.51 · Observe actual derived tile resources](86ap-observe-actual-derived-tile-resources.md) | 2 | 162 | 2–4 min | 0.5–1 h | Within planning limit |
| [17.1.52 · Exercise finite color and original picking](86aq-exercise-finite-color-and-original-picking.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [17.2.1 · Describe original clipping half-spaces](87-copy-current-original-plane-modes.md) | 55 | 1606 | 17–33 min | 0.5–1 h | Within planning limit |
| [17.2.2 · Describe clipping construction choices once](87sa-describe-clipping-construction-choices-once.md) | 60 | 2058 | 21–42 min | 0.5–1 h | Within planning limit |
| [17.2.3 · Construct and reverse original clipping planes](87sb-construct-and-reverse-original-clipping-planes.md) | 64 | 2166 | 22–44 min | 0.5–1 h | Within planning limit |
| [17.2.4 · Derive placed world half-spaces from original planes](87sc-derive-placed-world-half-spaces-from-original-planes.md) | 27 | 824 | 9–17 min | 0.5–1 h | Within planning limit |
| [17.2.5 · Gather six visible original world planes](87a-gather-six-visible-original-world-planes.md) | 23 | 1742 | 18–35 min | 0.5–1 h | Within planning limit |
| [17.2.6 · Pack the clipping uniform with explicit byte offsets](87b-copy-the-maintained-clipping-uniform-layout.md) | 27 | 1332 | 14–27 min | 0.5–1 h | Within planning limit |
| [17.2.7 · Condition the camera inverse for clipping](87c-copy-conditioned-screen-and-hatch-projections.md) | 18 | 511 | 6–11 min | 0.5–1 h | Within planning limit |
| [17.2.8 · Project a stable hatch coordinate onto a plane](87ca-project-a-stable-hatch-coordinate-onto-a-plane.md) | 27 | 1048 | 11–21 min | 0.5–1 h | Within planning limit |
| [17.2.9 · Fill one bounded clipping uniform from original planes](87cb-fill-one-bounded-clipping-uniform-from-original-planes.md) | 57 | 1879 | 19–38 min | 0.5–1 h | Within planning limit |
| [17.2.10 · Match the clipping uniform and sample conventions](87cc-match-the-clipping-uniform-and-sample-conventions.md) | 44 | 1980 | 20–40 min | 0.5–1 h | Within planning limit |
| [17.2.11 · Apply shared original half-space tests](87cd-apply-shared-original-half-space-tests.md) | 42 | 1267 | 13–26 min | 0.5–1 h | Within planning limit |
| [17.2.12 · Compute clipping coverage in framebuffer sample space](87ce-compute-clipping-coverage-in-framebuffer-sample-space.md) | 50 | 1416 | 15–29 min | 0.5–1 h | Within planning limit |
| [17.2.13 · Address section depth and original cap records](87cf-address-section-depth-and-original-cap-records.md) | 33 | 1183 | 12–24 min | 0.5–1 h | Within planning limit |
| [17.2.14 · Parse strict clipping modes and presentation switches](87d-parse-strict-clipping-modes-and-presentation-switches.md) | 20 | 1491 | 15–30 min | 0.5–1 h | Within planning limit |
| [17.2.15 · Preserve mode prefixes in generic point drafts](87e-preserve-mode-prefixes-in-generic-point-drafts.md) | 5 | 381 | 4–8 min | 0.5–1 h | Within planning limit |
| [17.2.16 · Build original clipping planes through the shared tool](87f-build-original-clipping-planes-through-the-shared-tool.md) | 20 | 1557 | 16–32 min | 0.5–1 h | Within planning limit |
| [17.2.17 · Own clipping presentation and one history transaction](87g-own-clipping-presentation-and-one-history-transaction.md) | 17 | 2527 | 26–51 min | 0.5–1 h | Within planning limit |
| [17.2.18 · Route Clipping Plane and preserve pending points](87h-route-clipping-plane-and-preserve-pending-points.md) | 7 | 810 | 9–17 min | 0.5–1 h | Within planning limit |
| [17.2.19 · Own one shared clipping GPU uniform](87i-own-one-shared-clipping-gpu-uniform.md) | 16 | 2282 | 23–46 min | 0.5–1 h | Within planning limit |
| [17.2.20 · Retain clipping state beside the renderer](87j-retain-clipping-state-beside-the-renderer.md) | 6 | 1086 | 11–22 min | 0.5–1 h | Within planning limit |
| [17.2.21 · Prepare shared clipping shader declarations](87k-prepare-shared-clipping-shader-declarations.md) | 2 | 403 | 5–9 min | 0.5–1 h | Within planning limit |
| [17.2.22 · Connect mesh and instance color half-spaces](87l-connect-mesh-and-instance-color-half-spaces.md) | 11 | 1400 | 14–28 min | 0.5–1 h | Within planning limit |
| [17.2.23 · Prepare explicit original mesh pick clipping](87m-prepare-explicit-original-mesh-pick-clipping.md) | 4 | 581 | 6–12 min | 0.5–1 h | Within planning limit |
| [17.2.24 · Bind shared clipping before the first integer request](87n-bind-shared-clipping-before-the-first-integer-request.md) | 14 | 2682 | 27–54 min | 0.5–1 h | Within planning limit |
| [17.2.25 · Preserve sparse source face IDs across the cut](87o-preserve-sparse-source-face-ids-across-the-cut.md) | 7 | 720 | 8–15 min | 0.5–1 h | Within planning limit |
| [17.2.26 · Invalidate selected masks on clipping changes](87p-invalidate-selected-masks-on-clipping-changes.md) | 10 | 1239 | 13–25 min | 0.5–1 h | Within planning limit |
| [17.2.27 · Share clipping at finite original center lines](87q-share-clipping-at-finite-original-center-lines.md) | 11 | 1788 | 18–36 min | 0.5–1 h | Within planning limit |
| [17.2.28 · Cache explicit original-plane ink flags](87r-cache-explicit-original-plane-ink-flags.md) | 9 | 1439 | 15–29 min | 0.5–1 h | Within planning limit |
| [17.2.29 · Bind original flags without changing instance strides](87s-bind-original-flags-without-changing-instance-strides.md) | 5 | 1742 | 18–35 min | 0.5–1 h | Within planning limit |
| [17.2.30 · Keep stroke color and pick exemptions aligned](87t-keep-stroke-color-and-pick-exemptions-aligned.md) | 16 | 1542 | 16–31 min | 0.5–1 h | Within planning limit |
| [17.2.31 · Keep chain color and pick exemptions aligned](87u-keep-chain-color-and-pick-exemptions-aligned.md) | 12 | 1606 | 17–33 min | 0.5–1 h | Within planning limit |
| [17.2.32 · Keep marker color and pick exemptions aligned](87v-keep-marker-color-and-pick-exemptions-aligned.md) | 12 | 1306 | 14–27 min | 0.5–1 h | Within planning limit |
| [17.2.33 · Resolve locked and selected original plane boundaries](87w-resolve-locked-and-selected-original-plane-boundaries.md) | 5 | 1506 | 16–31 min | 0.5–1 h | Within planning limit |
| [17.2.34 · Clip cached original point cloud discs](87x-clip-cached-original-point-cloud-discs.md) | 13 | 1343 | 14–27 min | 0.5–1 h | Within planning limit |
| [17.2.35 · Clip original sheet pens and integer entities](87y-clip-original-sheet-pens-and-integer-entities.md) | 13 | 1085 | 11–22 min | 0.5–1 h | Within planning limit |
| [17.2.36 · Clip text anchors with all six original planes](87z-clip-text-anchors-with-all-six-original-planes.md) | 4 | 593 | 6–12 min | 0.5–1 h | Within planning limit |
| [17.2.37 · Observe actual clipping resources and errors](87aa-observe-actual-clipping-resources-and-errors.md) | 2 | 461 | 5–10 min | 0.5–1 h | Within planning limit |
| [17.2.38 · Exercise original clipping color and picking](87ab-exercise-original-clipping-color-and-picking.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [17.3.1 · Classify original closedness and winding](88-copy-original-closedness-and-winding.md) | 73 | 2065 | 21–42 min | 0.5–1 h | Within planning limit |
| [17.3.2 · Cache verified original solids weakly](88a-cache-verified-original-solids-weakly.md) | 26 | 1685 | 17–34 min | 0.5–1 h | Within planning limit |
| [17.3.3 · Bound conservative plane crossings](88b-copy-conservative-plane-crossing-bounds.md) | 18 | 779 | 8–16 min | 0.5–1 h | Within planning limit |
| [17.3.4 · Cache local bounds and apply placement once](88c-cache-local-bounds-and-apply-placement-once.md) | 15 | 1167 | 12–24 min | 0.5–1 h | Within planning limit |
| [17.3.5 · Carry closedness and dense cap addresses](88d-carry-closedness-and-dense-cap-addresses.md) | 11 | 1256 | 13–26 min | 0.5–1 h | Within planning limit |
| [17.3.6 · Count original closed-solid crossings](88e-count-original-closed-solid-crossings.md) | 33 | 1971 | 20–40 min | 0.5–1 h | Within planning limit |
| [17.3.7 · Count exact original cap pick candidates](88f-count-exact-original-cap-pick-candidates.md) | 48 | 1949 | 20–39 min | 0.5–1 h | Within planning limit |
| [17.3.8 · Filter section bands over a pixel footprint](88g-copy-section-coverage-and-hatch-filtering.md) | 45 | 1828 | 19–37 min | 0.5–1 h | Within planning limit |
| [17.3.9 · Fade hatch detail and detect section boundaries](88ga-fade-hatch-detail-and-detect-section-boundaries.md) | 42 | 1672 | 17–34 min | 0.5–1 h | Within planning limit |
| [17.3.10 · Shade sections with shared original depth and sample ownership](88gb-shade-sections-with-shared-original-depth-and-sample-ownership.md) | 51 | 2049 | 21–41 min | 0.5–1 h | Within planning limit |
| [17.3.11 · Adapt color sections to forward depth](88h-adapt-color-sections-to-forward-depth.md) | 9 | 566 | 6–12 min | 0.5–1 h | Within planning limit |
| [17.3.12 · Return original integer section owners](88i-return-original-integer-section-owners.md) | 40 | 1731 | 18–35 min | 0.5–1 h | Within planning limit |
| [17.3.13 · Own lazy section images](88j-own-lazy-section-images.md) | 6 | 698 | 7–14 min | 0.5–1 h | Within planning limit |
| [17.3.14 · Build compatible section render pipelines](88k-build-compatible-section-render-pipelines.md) | 6 | 1373 | 14–28 min | 0.5–1 h | Within planning limit |
| [17.3.15 · Add signed counts and grey hatch color](88l-add-signed-counts-and-grey-hatch-color.md) | 8 | 1467 | 15–30 min | 0.5–1 h | Within planning limit |
| [17.3.16 · Build bounded integer section pipelines](88m-build-bounded-integer-section-pipelines.md) | 14 | 2075 | 21–42 min | 0.5–1 h | Within planning limit |
| [17.3.17 · Own bounded cap pick storage](88n-own-bounded-cap-pick-storage.md) | 11 | 1465 | 15–30 min | 0.5–1 h | Within planning limit |
| [17.3.18 · Prepare exact cap crop and owners](88o-prepare-exact-cap-crop-and-owners.md) | 10 | 2365 | 24–48 min | 0.5–1 h | Within planning limit |
| [17.3.19 · Draw cap count nearest owner and integer ID](88p-draw-cap-count-nearest-owner-and-integer-id.md) | 6 | 2314 | 24–47 min | 0.5–1 h | Within planning limit |
| [17.3.20 · Own section caches and placeholders](88q-own-section-caches-and-placeholders.md) | 16 | 1164 | 12–24 min | 0.5–1 h | Within planning limit |
| [17.3.21 · Gather crossing original owners](88r-gather-crossing-original-owners.md) | 4 | 941 | 10–19 min | 0.5–1 h | Within planning limit |
| [17.3.22 · Prepare active section images](88s-prepare-active-section-images.md) | 7 | 1095 | 11–22 min | 0.5–1 h | Within planning limit |
| [17.3.23 · Draw the closed union and section depth](88t-draw-the-closed-union-and-section-depth.md) | 8 | 2098 | 21–42 min | 0.5–1 h | Within planning limit |
| [17.3.24 · Route exact original cap picks and allocation](88u-route-exact-original-cap-picks-and-allocation.md) | 6 | 1006 | 11–21 min | 0.5–1 h | Within planning limit |
| [17.3.25 · Connect sections after opaque originals](88v-connect-sections-after-opaque-originals.md) | 8 | 1474 | 15–30 min | 0.5–1 h | Within planning limit |
| [17.3.26 · Expose section data to finite ink tables](88x-expose-section-data-to-finite-ink-tables.md) | 3 | 883 | 9–18 min | 0.5–1 h | Within planning limit |
| [17.3.27 · Cache finite section bindings](88y-cache-finite-section-bindings.md) | 5 | 2303 | 24–47 min | 0.5–1 h | Within planning limit |
| [17.3.28 · Retain ink exactly on sloping sections](88z-retain-ink-exactly-on-sloping-sections.md) | 11 | 444 | 5–9 min | 0.5–1 h | Within planning limit |
| [17.3.29 · Hide original ink behind visible caps](88aa-hide-original-ink-behind-visible-caps.md) | 6 | 649 | 7–13 min | 0.5–1 h | Within planning limit |
| [17.3.30 · Pick sections before any color frame](88ab-pick-sections-before-any-color-frame.md) | 9 | 2574 | 26–52 min | 0.5–1 h | Within planning limit |
| [17.3.31 · Observe actual section owners and errors](88ac-observe-actual-section-owners-and-errors.md) | 1 | 444 | 5–9 min | 0.5–1 h | Within planning limit |
| [17.3.32 · Exercise original sections bores and ink](88ad-exercise-original-sections-bores-and-ink.md) | 4 | 62 | 1–2 min | 0.5–1 h | Within planning limit |
| [18.1.1 · Reduce depth and radius at a bounded display resolution](89-copy-density-scaled-depth-reduction.md) | 51 | 1886 | 19–38 min | 0.5–1 h | Within planning limit |
| [18.1.2 · Prepare linear original depth and radius addresses](89a-prepare-linear-original-depth-and-radius-addresses.md) | 12 | 1119 | 12–23 min | 0.5–1 h | Within planning limit |
| [18.1.3 · Own six real depth and radius levels](89b-own-six-real-depth-and-radius-levels.md) | 8 | 982 | 10–20 min | 0.5–1 h | Within planning limit |
| [18.1.4 · Build prepare and nearest-depth pipelines](89c-build-prepare-and-nearest-depth-pipelines.md) | 15 | 2210 | 23–45 min | 0.5–1 h | Within planning limit |
| [18.1.5 · Prepare cached pyramid inputs and images](89d-prepare-cached-pyramid-inputs-and-images.md) | 17 | 2079 | 21–42 min | 0.5–1 h | Within planning limit |
| [18.1.6 · Encode six levels only when inputs change](89e-encode-six-levels-only-when-inputs-change.md) | 7 | 1883 | 19–38 min | 0.5–1 h | Within planning limit |
| [18.1.7 · Expose actual pyramid allocation and levels](89f-expose-actual-pyramid-allocation-and-levels.md) | 3 | 310 | 4–7 min | 0.5–1 h | Within planning limit |
| [18.1.8 · Route original depth into the pyramid owner](89g-route-original-depth-into-the-pyramid-owner.md) | 9 | 2034 | 21–41 min | 0.5–1 h | Within planning limit |
| [18.1.9 · Exercise actual depth levels and cache release](89h-exercise-actual-depth-levels-and-cache-release.md) | 4 | 99 | 1–2 min | 0.5–1 h | Within planning limit |
| [18.2.1 · Derive original contact radius](90-copy-maintained-original-contact-radius.md) | 9 | 225 | 3–5 min | 0.5–1 h | Within planning limit |
| [18.2.2 · Write current viewing rays](90a-copy-current-viewing-rays.md) | 42 | 1270 | 13–26 min | 0.5–1 h | Within planning limit |
| [18.2.3 · Adapt the earlier forward camera](90b-adapt-the-earlier-forward-camera.md) | 4 | 686 | 7–14 min | 0.5–1 h | Within planning limit |
| [18.2.4 · Gather weak original ambient properties](90c-gather-weak-original-ambient-properties.md) | 7 | 729 | 8–15 min | 0.5–1 h | Within planning limit |
| [18.2.5 · Declare the ambient pass resource contracts](90d-copy-current-horizon-and-bilateral-recipes.md) | 43 | 1586 | 16–32 min | 0.5–1 h | Within planning limit |
| [18.2.6 · Reconstruct original surfaces from depth](90da-reconstruct-original-surfaces-from-depth.md) | 36 | 1626 | 17–33 min | 0.5–1 h | Within planning limit |
| [18.2.7 · Encode normals and read original contact metadata](90db-encode-normals-and-read-original-contact-metadata.md) | 29 | 1959 | 20–40 min | 0.5–1 h | Within planning limit |
| [18.2.8 · Prepare a compact depth radius and normal target](90dc-prepare-a-compact-depth-radius-and-normal-target.md) | 39 | 1873 | 19–38 min | 0.5–1 h | Within planning limit |
| [18.2.9 · Measure ground contact along bounded horizons](90dd-measure-ground-contact-along-bounded-horizons.md) | 47 | 2386 | 24–48 min | 0.5–1 h | Within planning limit |
| [18.2.10 · Integrate cosine weighted surface visibility](90de-integrate-cosine-weighted-surface-visibility.md) | 59 | 2805 | 29–57 min | 0.5–1 h | Within planning limit |
| [18.2.11 · Filter contact without crossing unrelated surfaces](90df-filter-contact-without-crossing-unrelated-surfaces.md) | 40 | 1696 | 17–34 min | 0.5–1 h | Within planning limit |
| [18.2.12 · Reject invalid ambient history before blending](90dg-reject-invalid-ambient-history-before-blending.md) | 35 | 1756 | 18–36 min | 0.5–1 h | Within planning limit |
| [18.2.13 · Reconstruct full resolution contact from low resolution samples](90dh-reconstruct-full-resolution-contact-from-low-resolution-samples.md) | 30 | 1124 | 12–23 min | 0.5–1 h | Within planning limit |
| [18.2.14 · Collect bounded sample corrections with GPU atomics](90di-collect-bounded-sample-corrections-with-gpu-atomics.md) | 37 | 1508 | 16–31 min | 0.5–1 h | Within planning limit |
| [18.2.15 · Composite contact as black alpha over the original scene](90dj-composite-contact-as-black-alpha-over-the-original-scene.md) | 47 | 1954 | 20–40 min | 0.5–1 h | Within planning limit |
| [18.2.16 · Validate the assembled original ambient shader contracts](90e-bind-real-original-normals-and-radii.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [18.2.17 · Write original surface contact properties](90f-write-original-surface-contact-properties.md) | 14 | 1320 | 14–27 min | 0.5–1 h | Within planning limit |
| [18.2.18 · Build the depth-equal original property pass](90g-build-the-depth-equal-original-property-pass.md) | 11 | 1966 | 20–40 min | 0.5–1 h | Within planning limit |
| [18.2.19 · Cache original radius buffers and draw properties](90h-cache-original-radius-buffers-and-draw-properties.md) | 14 | 2257 | 23–46 min | 0.5–1 h | Within planning limit |
| [18.2.20 · Own actual ambient image allocation](90i-own-actual-ambient-image-allocation.md) | 6 | 774 | 8–16 min | 0.5–1 h | Within planning limit |
| [18.2.21 · Prepare reduced and full ambient targets](90j-prepare-reduced-and-full-ambient-targets.md) | 11 | 1583 | 16–32 min | 0.5–1 h | Within planning limit |
| [18.2.22 · Build explicit ambient fullscreen pipelines](90k-build-explicit-ambient-fullscreen-pipelines.md) | 11 | 1713 | 18–35 min | 0.5–1 h | Within planning limit |
| [18.2.23 · Match current ambient sampling layouts](90l-match-current-ambient-sampling-layouts.md) | 11 | 2547 | 26–51 min | 0.5–1 h | Within planning limit |
| [18.2.24 · Cache reduced levels and filtered samples](90m-cache-reduced-levels-and-filtered-samples.md) | 12 | 2438 | 25–49 min | 0.5–1 h | Within planning limit |
| [18.2.25 · Bind exact current ambient source images](90n-bind-exact-current-ambient-source-images.md) | 3 | 891 | 9–18 min | 0.5–1 h | Within planning limit |
| [18.2.26 · Record explicit ambient passes](90o-record-explicit-ambient-passes.md) | 5 | 731 | 8–15 min | 0.5–1 h | Within planning limit |
| [18.2.27 · Own cached ambient pipelines and release images](90p-own-cached-ambient-pipelines-and-release-images.md) | 12 | 1081 | 11–22 min | 0.5–1 h | Within planning limit |
| [18.2.28 · Prepare changed ambient framebuffer inputs](90q-prepare-changed-ambient-framebuffer-inputs.md) | 5 | 879 | 9–18 min | 0.5–1 h | Within planning limit |
| [18.2.29 · Render a complete ambient result in one frame](90r-render-a-complete-ambient-result-in-one-frame.md) | 18 | 2434 | 25–49 min | 0.5–1 h | Within planning limit |
| [18.2.30 · Count actual ambient resources and work](90s-count-actual-ambient-resources-and-work.md) | 1 | 227 | 3–5 min | 0.5–1 h | Within planning limit |
| [18.2.31 · Route original scene properties into ambient shading](90t-route-original-scene-properties-into-ambient-shading.md) | 9 | 2161 | 22–44 min | 0.5–1 h | Within planning limit |
| [18.2.32 · Enable Arctic as a presentation change](90u-enable-arctic-as-a-presentation-change.md) | 9 | 1610 | 17–33 min | 0.5–1 h | Within planning limit |
| [18.2.33 · Exercise actual contacts cache and source ownership](90v-exercise-actual-contacts-cache-and-source-ownership.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [18.3.1 · Include visible section coverage](91-include-visible-section-coverage.md) | 12 | 638 | 7–13 min | 0.5–1 h | Within planning limit |
| [18.3.2 · Cache visible section bindings](91a-cache-visible-section-bindings.md) | 30 | 2541 | 26–51 min | 0.5–1 h | Within planning limit |
| [18.3.3 · Combine outline alpha once](91b-combine-outline-alpha-once.md) | 4 | 507 | 6–11 min | 0.5–1 h | Within planning limit |
| [18.3.4 · Cache one black outline composite](91c-cache-one-black-outline-composite.md) | 15 | 2519 | 26–51 min | 0.5–1 h | Within planning limit |
| [18.3.5 · Separate source scope from outline selection](91d-separate-source-scope-from-outline-selection.md) | 7 | 466 | 5–10 min | 0.5–1 h | Within planning limit |
| [18.3.6 · Prepare masks without a second blend](91e-prepare-masks-without-a-second-blend.md) | 34 | 1670 | 17–34 min | 0.5–1 h | Within planning limit |
| [18.3.7 · Own independent global outline resources](91f-own-independent-global-outline-resources.md) | 13 | 2129 | 22–43 min | 0.5–1 h | Within planning limit |
| [18.3.8 · Draw all-solid and selected coverage consistently](91g-draw-all-solid-and-selected-coverage-consistently.md) | 11 | 1666 | 17–34 min | 0.5–1 h | Within planning limit |
| [18.3.9 · Connect Outline and Arctic presentation commands](91h-connect-outline-and-arctic-presentation-commands.md) | 10 | 2009 | 21–41 min | 0.5–1 h | Within planning limit |
| [18.3.10 · Prove actual outlines and original ownership](91i-prove-actual-outlines-and-original-ownership.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [18.4.1 · Link current Wood element builders](92-link-current-wood-element-builders.md) | 1 | 42 | 1–1 min | 0.5–1 h | Within planning limit |
| [18.4.2 · Write original beam column and plate builders](91zza-write-original-beam-column-and-plate-builders.md) | 88 | 2658 | 27–54 min | 0.5–1 h | Within planning limit |
| [18.4.3 · Write original support variable beam and block builders](92a-retain-current-positive-lengths-and-six-builders.md) | 59 | 1850 | 19–37 min | 0.5–1 h | Within planning limit |
| [18.4.4 · Own original Element geometry](92b-own-original-element-geometry.md) | 12 | 920 | 10–19 min | 0.5–1 h | Within planning limit |
| [18.4.5 · Read current Element command defaults](92c-read-current-element-command-defaults.md) | 25 | 2008 | 21–41 min | 0.5–1 h | Within planning limit |
| [18.4.6 · Build typed Elements from ordered world picks](92d-build-typed-elements-from-ordered-world-picks.md) | 14 | 1500 | 15–30 min | 0.5–1 h | Within planning limit |
| [18.4.7 · Capture original picks while the tool runs](92e-capture-original-picks-while-the-tool-runs.md) | 17 | 2066 | 21–42 min | 0.5–1 h | Within planning limit |
| [18.4.8 · Teach named and bare Element values](92f-teach-named-and-bare-element-values.md) | 40 | 1456 | 15–30 min | 0.5–1 h | Within planning limit |
| [18.4.9 · Make Element input batches atomic](92g-make-element-input-batches-atomic.md) | 1 | 238 | 3–5 min | 0.5–1 h | Within planning limit |
| [18.4.10 · Attach readonly Element attributes](92h-attach-readonly-element-attributes.md) | 8 | 1404 | 15–29 min | 0.5–1 h | Within planning limit |
| [18.4.11 · Adopt Elements and consume sources once](92i-adopt-elements-and-consume-sources-once.md) | 48 | 2362 | 24–48 min | 0.5–1 h | Within planning limit |
| [18.4.12 · Save and open original Element records](92j-save-and-open-original-element-records.md) | 6 | 948 | 10–19 min | 0.5–1 h | Within planning limit |
| [18.4.13 · Own independent Element tool state](92k-own-independent-element-tool-state.md) | 7 | 2242 | 23–45 min | 0.5–1 h | Within planning limit |
| [18.4.14 · Commit one Element creation and retry errors](92l-commit-one-element-creation-and-retry-errors.md) | 13 | 2063 | 21–42 min | 0.5–1 h | Within planning limit |
| [18.4.15 · Route Element commands and independent visibility](92m-route-current-element-commands-and-feature-visibility.md) | 27 | 2801 | 29–57 min | 0.5–1 h | Within planning limit |
| [18.4.16 · Pick Elements through the real command dock](92n-pick-elements-through-the-real-command-dock.md) | 7 | 2862 | 29–58 min | 0.5–1 h | Within planning limit |
| [18.4.17 · Prove Element recipes source history and archives](92o-prove-element-recipes-source-history-and-archives.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [18.5.1 · Classify original solids independently of clipping](93-classify-original-solids-independently-of-clipping.md) | 9 | 2733 | 28–55 min | 0.5–1 h | Within planning limit |
| [18.5.2 · Own opacity in the existing surface frame](93a-own-opacity-in-the-existing-surface-frame.md) | 7 | 1495 | 15–30 min | 0.5–1 h | Within planning limit |
| [18.5.3 · Blend closed faces and retain opaque open surfaces](93b-blend-closed-faces-and-retain-opaque-open-surfaces.md) | 7 | 615 | 7–13 min | 0.5–1 h | Within planning limit |
| [18.5.4 · Exclude x-ray faces from original picks and masks](93c-exclude-x-ray-faces-from-original-picks-and-masks.md) | 4 | 668 | 7–14 min | 0.5–1 h | Within planning limit |
| [18.5.5 · Carry opacity into finite original ink](93d-carry-opacity-into-finite-original-ink.md) | 15 | 1372 | 14–28 min | 0.5–1 h | Within planning limit |
| [18.5.6 · Fade hidden ink without showing clipped geometry](93e-fade-hidden-ink-without-showing-clipped-geometry.md) | 6 | 1059 | 11–22 min | 0.5–1 h | Within planning limit |
| [18.5.7 · Invalidate presentation and preserve cached geometry](93f-invalidate-presentation-and-preserve-cached-geometry.md) | 2 | 643 | 7–13 min | 0.5–1 h | Within planning limit |
| [18.5.8 · Connect strict Opacity presentation commands](93g-connect-strict-opacity-presentation-commands.md) | 9 | 1966 | 20–40 min | 0.5–1 h | Within planning limit |
| [18.5.9 · Prove actual glass x-ray and source ownership](93h-prove-actual-glass-x-ray-and-source-ownership.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [18.6.1 · Declare the rendering comparison contract](94-declare-the-rendering-comparison-contract.md) | 36 | 2034 | 21–41 min | 0.5–1 h | Within planning limit |
| [18.6.2 · Link the maintained native rendering reference](94a-link-the-maintained-native-rendering-reference.md) | 3 | 122 | 2–3 min | 0.5–1 h | Within planning limit |
| [18.6.3 · Read actual maintained frames without source mutation](94b-read-actual-maintained-frames-without-source-mutation.md) | 7 | 126 | 2–3 min | 0.5–1 h | Within planning limit |
| [18.6.4 · Prove the original comparison and inspect differences](94c-prove-the-original-comparison-and-inspect-differences.md) | 1 | 81 | 1–2 min | 0.5–1 h | Within planning limit |
| [18.7.1 · Record comparable large scene conditions](95-record-comparable-large-scene-conditions.md) | 23 | 890 | 9–18 min | 0.5–1 h | Within planning limit |
| [18.7.2 · Measure warmed frame times and the slow tail](95a-measure-warmed-frame-times-and-the-slow-tail.md) | 19 | 815 | 9–17 min | 0.5–1 h | Within planning limit |
| [18.7.3 · Reject incomparable before and after runs](95b-reject-incomparable-before-and-after-runs.md) | 30 | 1204 | 13–25 min | 0.5–1 h | Within planning limit |
| [18.7.4 · Witness immutable tree inputs across camera frames](95c-witness-immutable-tree-inputs-across-camera-frames.md) | 22 | 1006 | 11–21 min | 0.5–1 h | Within planning limit |
| [18.7.5 · Reuse layer rows while rotating and zooming](95d-reuse-layer-rows-while-rotating-and-zooming.md) | 8 | 1085 | 11–22 min | 0.5–1 h | Within planning limit |
| [18.7.6 · Borrow graph edges and index their labels](95e-borrow-graph-edges-and-index-their-labels.md) | 15 | 959 | 10–20 min | 0.5–1 h | Within planning limit |
| [18.7.7 · Prove cache invalidation and honest performance reports](95f-prove-cache-invalidation-and-honest-performance-reports.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [19.1.1 · Retain non drawing originals and validate raw planes](96-retain-non-drawing-originals-and-validate-raw-planes.md) | 31 | 1864 | 19–38 min | 0.5–1 h | Within planning limit |
| [19.1.2 · Merge unused definitions with displayed instances](96a-merge-unused-definitions-with-displayed-instances.md) | 14 | 830 | 9–17 min | 0.5–1 h | Within planning limit |
| [19.1.3 · Preserve component and interaction metadata in archives](96b-preserve-component-and-interaction-metadata-in-archives.md) | 8 | 1242 | 13–25 min | 0.5–1 h | Within planning limit |
| [19.1.4 · Keep current Element and frame definition families](96c-keep-current-element-and-frame-definition-families.md) | 7 | 1491 | 15–30 min | 0.5–1 h | Within planning limit |
| [19.1.5 · Observe the real current registry and parser](96d-observe-the-real-current-registry-and-parser.md) | 18 | 1249 | 13–25 min | 0.5–1 h | Within planning limit |
| [19.1.6 · Verify the complete declared current source closure](96e-verify-the-complete-declared-current-source-closure.md) | 28 | 1991 | 20–40 min | 0.5–1 h | Within planning limit |
| [19.1.7 · Report retained original metadata in the real viewer](96f-report-retained-original-metadata-in-the-real-viewer.md) | 1 | 145 | 2–3 min | 0.5–1 h | Within planning limit |
| [19.1.8 · Prove exact metadata and definition round trips](96g-prove-exact-metadata-and-definition-round-trips.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [19.1.9 · Prove current source grammar and explicit differences](96h-prove-current-source-grammar-and-explicit-differences.md) | 0 | 0 | 0–0 min | 0.5–1 h | Within planning limit |
| [19.2.1 · Separate own attributes from hosted features](96i-separate-own-and-hosted-element-features.md) | 38 | 1117 | 12–23 min | 0.5–1 h | Within planning limit |
| [19.2.2 · Group pieces by their original interaction](96j-group-features-by-original-interaction.md) | 32 | 1494 | 15–30 min | 0.5–1 h | Within planning limit |
| [19.2.3 · Keep nested Elements editable and projections unsaved](96k-separate-derived-rows-from-editable-elements.md) | 32 | 1616 | 17–33 min | 0.5–1 h | Within planning limit |
| [19.2.4 · Attach a bounded feature tree to its host](96l-attach-a-bounded-derived-feature-tree.md) | 48 | 2284 | 23–46 min | 0.5–1 h | Within planning limit |
| [19.2.5 · Project original outlines without changing their coordinates](96m-project-original-feature-outlines.md) | 51 | 2540 | 26–51 min | 0.5–1 h | Within planning limit |
| [19.2.6 · Connect imported Elements to their feature hierarchy](96n-connect-imported-interaction-hierarchies.md) | 10 | 560 | 6–12 min | 0.5–1 h | Within planning limit |
| [19.2.7 · Use the same projection path for created Elements](96o-reuse-projections-for-created-elements.md) | 24 | 1068 | 11–22 min | 0.5–1 h | Within planning limit |
| [19.2.8 · Build an original interaction specimen](96p-build-an-imported-interaction-specimen.md) | 45 | 2397 | 24–48 min | 0.5–1 h | Within planning limit |
| [19.2.9 · Prove imported feature ownership and round trips](96q-prove-imported-feature-ownership-and-roundtrips.md) | 3 | 44 | 1–1 min | 0.5–1 h | Within planning limit |
| [19.2.10 · Prove exact commands across the expanded vocabulary](96r-preserve-exact-command-completions.md) | 4 | 124 | 2–3 min | 0.5–1 h | Within planning limit |
| [19.3.1 · Resolve feature rows to original Element owners](96s-resolve-feature-rows-to-original-element-owners.md) | 42 | 1582 | 16–32 min | 0.5–1 h | Within planning limit |
| [19.3.2 · Select and highlight original feature owners](96t-select-and-highlight-original-feature-owners.md) | 9 | 798 | 8–16 min | 0.5–1 h | Within planning limit |
| [19.3.3 · Fit original box corners to the free canvas](96u-fit-original-box-corners-to-the-free-canvas.md) | 37 | 1553 | 16–32 min | 0.5–1 h | Within planning limit |
| [19.3.4 · Project definition and per-instance features](96v-project-definition-and-per-instance-features.md) | 25 | 1155 | 12–24 min | 0.5–1 h | Within planning limit |
| [19.3.5 · Detach copied original source identity](96w-detach-copied-original-source-identity.md) | 19 | 975 | 10–20 min | 0.5–1 h | Within planning limit |
| [19.3.6 · Regenerate display for copied feature owners](96x-regenerate-display-for-copied-feature-owners.md) | 33 | 1469 | 15–30 min | 0.5–1 h | Within planning limit |
| [19.3.7 · Address feature pixels to original CAD owners](96y-address-feature-pixels-to-original-cad-owners.md) | 15 | 583 | 6–12 min | 0.5–1 h | Within planning limit |
| [19.3.8 · Preserve source feature visibility and pixel styles](96z-preserve-source-feature-visibility-and-pixel-styles.md) | 29 | 1659 | 17–34 min | 0.5–1 h | Within planning limit |
| [19.4.1 · Address original controls with family keys](96za-address-original-controls-with-family-keys.md) | 8 | 799 | 8–16 min | 0.5–1 h | Within planning limit |
| [19.4.2 · Draw and pick original CAD control points](96zb-draw-and-pick-original-cad-control-points.md) | 17 | 1261 | 13–26 min | 0.5–1 h | Within planning limit |
| [19.4.3 · Share the current control-point style](96zc-share-the-current-control-point-style.md) | 12 | 921 | 10–19 min | 0.5–1 h | Within planning limit |
| [19.4.4 · Reuse world controls for their display nets](96zd-reuse-world-controls-for-their-display-nets.md) | 41 | 2381 | 24–48 min | 0.5–1 h | Within planning limit |
| [19.4.5 · Scope controls to the selected original](96ze-scope-controls-to-the-selected-original.md) | 28 | 1938 | 20–39 min | 0.5–1 h | Within planning limit |
| [19.4.6 · Edit original controls through the source preview](96zf-edit-original-controls-through-the-source-preview.md) | 12 | 992 | 10–20 min | 0.5–1 h | Within planning limit |
| [19.4.7 · Rebuild an edited Element without replacing its owner](96zg-rebuild-an-edited-element-without-replacing-its-owner.md) | 13 | 946 | 10–19 min | 0.5–1 h | Within planning limit |
| [20.1.1 · Bind the shared geometry as storage](96zh-bind-the-shared-geometry-as-storage.md) | 39 | 2339 | 24–47 min | 0.5–1 h | Within planning limit |
| [20.1.2 · Read source vertices through one shader helper](96zi-read-source-vertices-through-one-shader-helper.md) | 26 | 2061 | 21–42 min | 0.5–1 h | Within planning limit |
| [20.1.3 · Activate shared vertex pulling for color](96zj-activate-shared-vertex-pulling-for-color.md) | 16 | 700 | 7–14 min | 0.5–1 h | Within planning limit |
| [20.1.4 · Assign frame triangle ranges](96zk-assign-frame-triangle-ranges.md) | 30 | 1495 | 15–30 min | 0.5–1 h | Within planning limit |
| [20.1.5 · Own the integer primitive image](96zl-own-the-integer-primitive-image.md) | 26 | 1429 | 15–29 min | 0.5–1 h | Within planning limit |
| [20.1.6 · Write triangle addresses beside color](96zm-write-triangle-addresses-beside-color.md) | 18 | 905 | 10–19 min | 0.5–1 h | Within planning limit |
| [20.2.1 · Bind visible triangle addresses for ink](96zn-bind-visible-triangle-addresses-for-ink.md) | 13 | 1629 | 17–33 min | 0.5–1 h | Within planning limit |
| [20.2.2 · Describe ink depth and surface slack](96zo-describe-ink-depth-and-surface-slack.md) | 24 | 1096 | 11–22 min | 0.5–1 h | Within planning limit |
| [20.2.3 · Fit a neighboring surface along the stroke](96zp-fit-a-neighboring-surface-along-the-stroke.md) | 44 | 1709 | 18–35 min | 0.5–1 h | Within planning limit |
| [20.2.4 · Own a cropped primitive image for picks](96zq-own-a-cropped-primitive-image-for-picks.md) | 5 | 285 | 3–6 min | 0.5–1 h | Within planning limit |
| [20.2.5 · Share crop and clipping bindings for pulled picks](96zr-share-crop-and-clipping-bindings-for-pulled-picks.md) | 15 | 1008 | 11–21 min | 0.5–1 h | Within planning limit |
| [20.2.6 · Write first-pick primitives from shared geometry](96zs-write-first-pick-primitives-from-shared-geometry.md) | 29 | 2736 | 28–55 min | 0.5–1 h | Within planning limit |
| [20.2.7 · Fit surface depth beneath a point disc](96zt-fit-surface-depth-beneath-a-point-disc.md) | 25 | 1072 | 11–22 min | 0.5–1 h | Within planning limit |
| [20.2.8 · Test the disc center against a surface](96zu-test-the-disc-center-against-a-surface.md) | 38 | 1535 | 16–31 min | 0.5–1 h | Within planning limit |
| [20.2.9 · Carry the rasterized triangle gradient to ink](96zv-carry-the-rasterized-triangle-gradient-to-ink.md) | 28 | 1242 | 13–25 min | 0.5–1 h | Within planning limit |
| [20.2.10 · Connect surface-fitted point visibility](96zw-connect-surface-fitted-point-visibility.md) | 10 | 667 | 7–14 min | 0.5–1 h | Within planning limit |
| [20.2.11 · Test rasterized neighbors at the stroke source](96zx-test-the-rasterized-neighbors-at-the-stroke-source.md) | 24 | 1343 | 14–27 min | 0.5–1 h | Within planning limit |
| [20.2.12 · Connect fitted surfaces and exact stroke occlusion](96zy-connect-fitted-surfaces-and-exact-stroke-occlusion.md) | 28 | 1913 | 20–39 min | 0.5–1 h | Within planning limit |
| [20.2.13 · Separate hidden ink color from pick ownership](96zz-separate-hidden-ink-color-from-pick-ownership.md) | 16 | 1164 | 12–24 min | 0.5–1 h | Within planning limit |
| [20.2.14 · Connect section primitives to color and first picking](96zza-connect-section-primitive-images.md) | 29 | 2083 | 21–42 min | 0.5–1 h | Within planning limit |
| [20.2.15 · Read section visibility from one image](96zzb-read-section-visibility-from-one-image.md) | 14 | 673 | 7–14 min | 0.5–1 h | Within planning limit |
| [20.3.1 · Share section addresses with outlines and ambient shading](96zzc-share-section-addresses-with-outlines-and-ambient.md) | 45 | 1860 | 19–38 min | 0.5–1 h | Within planning limit |
| [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md) | 46 | 2580 | 26–52 min | 0.5–1 h | Within planning limit |
| [20.3.3 · Retain RGBA in prepared displays](96zze-retain-rgba-in-prepared-displays.md) | 27 | 1363 | 14–28 min | 0.5–1 h | Within planning limit |
| [20.4.1 · Start with current browser display defaults](96zzf-start-with-current-browser-display-defaults.md) | 58 | 2176 | 22–44 min | 0.5–1 h | Within planning limit |
| [20.5.1 · Grow buffers without losing rows](96zzg-grow-buffers-without-losing-rows.md) | 52 | 2513 | 26–51 min | 0.5–1 h | Within planning limit |
| [20.5.2 · Prepare shared compact mesh ranges](96zzh-prepare-shared-compact-mesh-ranges.md) | 36 | 1813 | 19–37 min | 0.5–1 h | Within planning limit |
| [20.5.3 · Upload compact ranges to shared GPU storage](96zzi-upload-compact-ranges-to-shared-gpu-storage.md) | 43 | 2285 | 23–46 min | 0.5–1 h | Within planning limit |
| [20.5.4 · Share the object record layout](96zzj-share-the-object-record-layout.md) | 26 | 848 | 9–17 min | 0.5–1 h | Within planning limit |
| [20.5.5 · Borrow the current arena buffers](96zzk-borrow-the-current-arena-buffers.md) | 38 | 2325 | 24–47 min | 0.8–1 h | Within planning limit |
| [20.5.6 · Read shared ranges through object records](96zzl-read-shared-ranges-through-object-records.md) | 22 | 888 | 9–18 min | 0.8–1 h | Within planning limit |
| [20.5.7 · Connect the renderer to one mesh arena](96zzm-connect-the-renderer-to-one-mesh-arena.md) | 20 | 1740 | 18–35 min | 0.8–1 h | Within planning limit |
| [20.5.8 · Describe eight-byte draw slots](96zzn-describe-eight-byte-draw-slots.md) | 27 | 1355 | 14–28 min | 0.8–1 h | Within planning limit |
| [20.5.9 · Read instance properties through draw slots](96zzo-read-instance-properties-through-draw-slots.md) | 26 | 1874 | 19–38 min | 0.8–1 h | Within planning limit |
| [20.6.1 · Classify contact fill shading](96zzp-classify-contact-fill-shading.md) | 11 | 598 | 6–12 min | 0.8–1 h | Within planning limit |
| [20.6.2 · Open files through one measured Session reader](96zzq-open-files-through-one-measured-session-reader.md) | 33 | 2575 | 26–52 min | 0.8–1 h | Within planning limit |
| [20.6.3 · Route current Close and Opacity commands](96zzr-route-current-close-and-opacity-commands.md) | 3 | 477 | 5–10 min | 0.8–1 h | Within planning limit |
| [20.6.4 · Reuse full source preparation for editor and reload](96zzs-reuse-full-source-preparation-for-editor-and-reload.md) | 13 | 1161 | 12–24 min | 0.8–1 h | Within planning limit |
| [20.6.5 · Share the current selection yellow](96zzt-share-the-current-selection-yellow.md) | 11 | 752 | 8–16 min | 0.8–1 h | Within planning limit |
| [20.7.1 · Separate primitive addresses from selection](96zzu-separate-primitive-addresses-from-selection.md) | 12 | 810 | 9–17 min | 0.8–1 h | Within planning limit |
| [20.7.2 · Bind the own-row slot for ordinary surfaces](96zzv-bind-the-own-row-slot-for-ordinary-surfaces.md) | 16 | 1143 | 12–23 min | 0.8–1 h | Within planning limit |
| [20.7.3 · Reload scenes through the existing publication owner](96zzw-reload-scenes-through-the-existing-publication-owner.md) | 40 | 2823 | 29–57 min | 0.8–1 h | Within planning limit |
| [20.7.4 · Update only changed slot runs](96zzx-update-only-changed-slot-runs.md) | 29 | 1452 | 15–30 min | 0.8–1 h | Within planning limit |
| [20.7.5 · Share one slot buffer across surface draws](96zzy-share-one-slot-buffer-across-surface-draws.md) | 21 | 1773 | 18–36 min | 0.8–1 h | Within planning limit |
| [20.7.6 · Prepare the integer primitive-slot texture](96zzz-prepare-the-integer-primitive-slot-texture.md) | 27 | 2200 | 22–44 min | 0.8–1 h | Within planning limit |
| [20.7.7 · Describe slot and triangle runs](96zzza-describe-slot-and-triangle-runs.md) | 80 | 1605 | 17–33 min | 0.8–1 h | Within planning limit |
| [20.7.8 · Read slot runs and lookup texels](96zzzb-read-slot-runs-and-lookup-texels.md) | 75 | 2279 | 23–46 min | 0.8–1 h | Within planning limit |
| [20.7.9 · Reuse and merge free slot ranges](96zzzc-reuse-and-merge-free-slot-ranges.md) | 59 | 1911 | 20–39 min | 0.8–1 h | Within planning limit |
| [20.7.10 · Follow arena growth with slot headroom](96zzzd-follow-arena-growth-with-slot-headroom.md) | 73 | 2102 | 22–43 min | 0.8–1 h | Within planning limit |
| [20.7.11 · Report only changed instance slots](96zzze-report-only-changed-instance-slots.md) | 90 | 2717 | 28–55 min | 0.8–1 h | Within planning limit |
| [20.7.12 · Project triangles at their physical addresses](96zzzf-project-triangles-at-their-physical-addresses.md) | 8 | 484 | 5–10 min | 0.8–1 h | Within planning limit |
| [20.8.1 · Clear the current Arctic background](96zzzg-clear-the-current-arctic-background.md) | 1 | 94 | 1–2 min | 1–1 h | Within planning limit |
| [20.8.2 · Share changed record runs](96zzzh-share-changed-record-runs.md) | 13 | 554 | 6–12 min | 0.8–1 h | Within planning limit |
| [20.8.3 · Prepare one shared object record table](96zzzi-prepare-one-shared-object-record-table.md) | 41 | 2172 | 22–44 min | 0.8–1 h | Within planning limit |
| [20.8.4 · Share instance object records across definitions](96zzzj-share-instance-object-records-across-definitions.md) | 22 | 1647 | 17–33 min | 0.8–1 h | Within planning limit |
| [20.8.5 · Reuse checked curve control coordinates](96zzzk-reuse-checked-curve-control-coordinates.md) | 7 | 554 | 6–12 min | 0.8–1 h | Within planning limit |
| [20.8.6 · Publish allocator texels through the integer table](96zzzl-publish-allocator-texels-through-the-integer-table.md) | 14 | 774 | 8–16 min | 0.8–1 h | Within planning limit |
| [20.8.7 · Resolve physical triangles with integer slots](96zzzm-resolve-physical-triangles-with-integer-slots.md) | 23 | 788 | 8–16 min | 0.8–1 h | Within planning limit |
| [20.8.8 · Batch shared geometry with original ownership](96zzzn-batch-shared-geometry-with-original-ownership.md) | 10 | 1097 | 11–22 min | 0.8–1 h | Within planning limit |
| [20.8.9 · Shrink record ranges without rewriting live prefixes](96zzzo-shrink-record-ranges-without-rewriting-live-prefixes.md) | 9 | 605 | 7–13 min | 0.8–1 h | Within planning limit |
| [20.8.10 · Share opaque ordinary surface records](96zzzp-share-opaque-ordinary-surface-records.md) | 23 | 2222 | 23–45 min | 0.8–1 h | Within planning limit |
| [20.8.11 · Blend ordinary surface records](96zzzq-blend-ordinary-surface-records.md) | 7 | 629 | 7–13 min | 0.8–1 h | Within planning limit |
| [20.8.12 · Share transparent instance records](96zzzr-share-transparent-instance-records.md) | 8 | 465 | 5–10 min | 0.8–1 h | Within planning limit |
| [20.9.1 · Allocate stable object record rows](96zzzs-allocate-stable-object-record-rows.md) | 7 | 774 | 8–16 min | 0.8–1 h | Within planning limit |
| [20.9.2 · Retain live surface record rows](96zzzt-retain-live-surface-record-rows.md) | 9 | 851 | 9–18 min | 0.8–1 h | Within planning limit |
| [20.9.3 · Prepare shared record pick consumers](96zzzu-prepare-shared-record-pick-consumers.md) | 15 | 1596 | 16–32 min | 0.8–1 h | Within planning limit |
| [20.9.4 · Pick originals through shared records](96zzzv-pick-originals-through-shared-records.md) | 23 | 1991 | 20–40 min | 0.8–1 h | Within planning limit |
| [20.9.5 · Read source faces through shared records](96zzzw-read-source-faces-through-shared-records.md) | 23 | 2169 | 22–44 min | 0.8–1 h | Within planning limit |
| [20.9.6 · Pack existing ambient radius in object records](96zzzx-pack-existing-ambient-radius-in-object-records.md) | 13 | 1494 | 15–30 min | 0.8–1 h | Within planning limit |
| [20.9.7 · Read ambient properties through shared records](96zzzy-read-ambient-properties-through-shared-records.md) | 18 | 1629 | 17–33 min | 0.8–1 h | Within planning limit |
| [20.10.1 · Share the object record shader schema](96zzzz-share-the-object-record-shader-schema.md) | 12 | 444 | 5–9 min | 0.8–1 h | Within planning limit |
| [20.10.2 · Prepare shared record triangle projection](96zzzza-prepare-shared-record-triangle-projection.md) | 10 | 577 | 6–12 min | 0.8–1 h | Within planning limit |
| [20.10.3 · Project triangles through shared records](96zzzzb-project-triangles-through-shared-records.md) | 19 | 2344 | 24–47 min | 0.8–1 h | Within planning limit |
| [20.10.4 · Prepare shared record cap counting](96zzzzc-prepare-shared-record-cap-counting.md) | 12 | 1274 | 13–26 min | 0.8–1 h | Within planning limit |
| [20.10.5 · Prepare cap plane and arena bindings](96zzzzd-prepare-cap-plane-and-arena-bindings.md) | 22 | 2044 | 21–41 min | 0.8–1 h | Within planning limit |
| [20.10.6 · Count cap color through shared records](96zzzze-count-cap-color-through-shared-records.md) | 21 | 2873 | 29–58 min | 0.8–1 h | Within planning limit |
| [20.10.7 · Prepare shared record cap picks](96zzzzf-prepare-shared-record-cap-picks.md) | 10 | 2230 | 23–45 min | 0.8–1 h | Within planning limit |
| [20.10.8 · Own shared record cap pick resources](96zzzzg-own-shared-record-cap-pick-resources.md) | 7 | 1116 | 12–23 min | 0.8–1 h | Within planning limit |
| [20.10.9 · Pick caps through shared records](96zzzzh-pick-caps-through-shared-records.md) | 8 | 1709 | 18–35 min | 0.8–1 h | Within planning limit |
| [20.11.1 · Adapt incremental slots to pulled draws](96zzzzi-adapt-incremental-slots-to-pulled-draws.md) | 4 | 496 | 5–10 min | 0.8–1 h | Within planning limit |
| [20.11.2 · Prepare exact arena slot definitions](96zzzzj-prepare-exact-arena-slot-definitions.md) | 22 | 1326 | 14–27 min | 0.8–1 h | Within planning limit |
| [20.11.3 · Retain live allocator member runs](96zzzzk-retain-live-allocator-member-runs.md) | 10 | 1140 | 12–23 min | 0.8–1 h | Within planning limit |
| [20.11.4 · Prepare one plan for draw slots and lookup](96zzzzl-prepare-one-plan-for-draw-slots-and-lookup.md) | 10 | 861 | 9–18 min | 0.8–1 h | Within planning limit |
| [20.11.5 · Prepare one owner for live address updates](96zzzzm-prepare-one-owner-for-live-address-updates.md) | 11 | 1168 | 12–24 min | 0.8–1 h | Within planning limit |
| [20.11.6 · Adopt retained GPU slots and physical bases](96zzzzn-adopt-retained-gpu-slots-and-physical-bases.md) | 21 | 2841 | 29–57 min | 0.8–1 h | Within planning limit |
| [20.11.7 · Separate projection rows and arena offsets](96zzzzo-separate-projection-rows-and-arena-offsets.md) | 6 | 384 | 4–8 min | 0.8–1 h | Within planning limit |
| [20.11.8 · Prepare lookup-selected triangle projection](96zzzzp-prepare-lookup-selected-triangle-projection.md) | 5 | 525 | 6–11 min | 0.8–1 h | Within planning limit |
| [20.11.9 · Prepare the lookup projection pipeline](96zzzzq-prepare-the-lookup-projection-pipeline.md) | 7 | 1092 | 11–22 min | 0.8–1 h | Within planning limit |
| [20.11.10 · Own current lookup projection bindings](96zzzzr-own-current-lookup-projection-bindings.md) | 20 | 2155 | 22–44 min | 0.8–1 h | Within planning limit |
| [20.11.11 · Project live triangles through the lookup](96zzzzs-project-live-triangles-through-the-lookup.md) | 14 | 2173 | 22–44 min | 0.8–1 h | Within planning limit |
| [20.12.1 · Plan reusable arena spans](96zzzzt-plan-reusable-arena-spans.md) | 18 | 871 | 9–18 min | 0.8–1 h | Within planning limit |
| [20.12.2 · Reuse retired arena record ranges](96zzzzu-reuse-retired-arena-record-ranges.md) | 22 | 2083 | 21–42 min | 0.8–1 h | Within planning limit |
| [20.12.3 · Upload reused arena spans in place](96zzzzv-upload-reused-arena-spans-in-place.md) | 21 | 1869 | 19–38 min | 0.8–1 h | Within planning limit |
| [20.12.4 · Reclaim ranges after the last GPU lease](96zzzzw-reclaim-ranges-after-the-last-gpu-lease.md) | 25 | 1893 | 19–38 min | 0.8–1 h | Within planning limit |
| [20.12.5 · Retire empty arena ranges after tail trimming](96zzzzx-retire-empty-arena-ranges-after-tail-trimming.md) | 1 | 40 | 1–1 min | 0.8–1 h | Within planning limit |
| [21.1.1 · Prepare unit-aware anchored camera space](96zzzzy-prepare-unit-aware-anchored-camera-space.md) | 32 | 1568 | 16–32 min | 0.8–1 h | Within planning limit |
| [21.1.2 · Prepare precise anchor-relative object records](96zzzzz-prepare-precise-anchor-relative-object-records.md) | 33 | 2044 | 21–41 min | 0.8–1 h | Within planning limit |
| [21.1.3 · Prepare anchor-relative original ink](96zzzzza-prepare-anchor-relative-original-ink.md) | 24 | 2124 | 22–43 min | 0.8–1 h | Within planning limit |
| [21.1.4 · Prepare shared-origin frame inputs](96zzzzzb-prepare-shared-origin-frame-inputs.md) | 18 | 1092 | 11–22 min | 0.8–1 h | Within planning limit |
| [21.1.5 · Prepare cloud and sheet frame placements](96zzzzzc-prepare-cloud-and-sheet-frame-placements.md) | 32 | 1903 | 20–39 min | 0.8–1 h | Within planning limit |
| [21.1.6 · Prepare local temporary overlays](96zzzzzd-prepare-local-temporary-overlays.md) | 32 | 2582 | 26–52 min | 0.8–1 h | Within planning limit |
| [21.1.7 · Preserve local precision before source placement](96zzzzze-preserve-local-precision-before-source-placement.md) | 30 | 2247 | 23–45 min | 0.8–1 h | Within planning limit |
| [21.1.8 · Prepare precise construction-grid origins](96zzzzzf-prepare-precise-construction-grid-origins.md) | 26 | 1298 | 13–26 min | 0.8–1 h | Within planning limit |
| [21.1.9 · Initialize object records in the local frame](96zzzzzg-initialize-object-records-in-the-local-frame.md) | 10 | 747 | 8–15 min | 0.8–1 h | Within planning limit |
| [21.1.10 · Prepare original highlights in the shared local frame](96zzzzzh-prepare-anchor-relative-original-highlights.md) | 19 | 2452 | 25–50 min | 0.8–1 h | Within planning limit |
| [21.1.11 · Verify the shared unit through original history](96zzzzzi-share-one-camera-and-measurement-unit.md) | 0 | 0 | 0–0 min | 0.1–0.25 h | Within planning limit |
| [21.2.1 · Project CPU queries around the precise target](96zzzzzj-project-cpu-queries-around-the-precise-target.md) | 11 | 586 | 6–12 min | 0.3–0.6 h | Within planning limit |
| [21.2.2 · Prepare precise text selection polygons](96zzzzzk-prepare-precise-text-selection-polygons.md) | 26 | 1558 | 16–32 min | 0.5–0.8 h | Within planning limit |
| [21.2.3 · Connect precise rectangle selection queries](96zzzzzl-connect-precise-rectangle-selection-queries.md) | 17 | 2894 | 29–58 min | 0.6–1 h | Within planning limit |
| [21.2.4 · Finish precise lasso and handle queries](96zzzzzm-finish-precise-lasso-and-handle-queries.md) | 14 | 1096 | 11–22 min | 0.5–0.9 h | Within planning limit |
| [21.2.5 · Stamp asynchronous picks with the scene origin](96zzzzzn-stamp-asynchronous-picks-with-the-origin.md) | 12 | 1228 | 13–25 min | 0.4–0.8 h | Within planning limit |
| [21.2.6 · Retry rebased picks with the latest cursor](96zzzzzo-retry-rebased-picks-with-the-latest-cursor.md) | 23 | 2103 | 22–43 min | 0.5–0.9 h | Within planning limit |
| [21.2.7 · Share derived inputs for origin refresh](96zzzzzp-share-derived-inputs-for-origin-refresh.md) | 23 | 1181 | 12–24 min | 0.5–0.9 h | Within planning limit |
| [21.3.1 · Prepare one shared render origin](96zzzzzq-prepare-one-shared-render-origin.md) | 20 | 1384 | 14–28 min | 0.5–0.9 h | Within planning limit |
| [21.3.2 · Prepare browser frame matrices and owned wakes](96zzzzzr-prepare-browser-frame-matrices-and-owned-wakes.md) | 20 | 2842 | 29–57 min | 0.6–1 h | Within planning limit |
| [21.3.3 · Adopt the shared origin in the browser](96zzzzzs-adopt-the-shared-origin-in-the-browser.md) | 42 | 2680 | 27–54 min | 0.6–1 h | Within planning limit |
| [21.4.1 · Share depth endpoints and comparisons](96zzzzzt-share-depth-endpoints-and-comparisons.md) | 24 | 2629 | 27–53 min | 0.6–1 h | Within planning limit |
| [21.4.2 · Share homogeneous near-plane ink clipping](96zzzzzu-share-homogeneous-near-plane-ink-clipping.md) | 37 | 1695 | 17–34 min | 0.5–1 h | Within planning limit |
| [21.4.3 · Use reversed depth across the frame](96zzzzzz-use-reversed-depth-across-the-frame.md) | 31 | 2922 | 30–59 min | 0.8–1 h | Within planning limit |
| [21.5.1 · Match physical camera defaults and limits](96zzzzzza-match-physical-camera-defaults-and-limits.md) | 12 | 556 | 6–12 min | 0.3–0.6 h | Within planning limit |
| [21.5.2 · Omit unused background pick pipelines](96zzzzzzb-omit-unused-background-pick-pipelines.md) | 12 | 1597 | 16–32 min | 0.6–1 h | Within planning limit |
| [21.5.3 · Draw the construction grid as physical lines](96zzzzzzc-draw-the-construction-grid-as-physical-lines.md) | 33 | 2271 | 23–46 min | 0.8–1.5 h | Within planning limit |
| [21.5.4 · Build viewing rays without matrix inversion](96zzzzzzd-build-viewing-rays-without-matrix-inversion.md) | 26 | 2044 | 21–41 min | 1–1.8 h | Within planning limit |
| [21.5.5 · Grow camera depth from cached world bounds](96zzzzzze-grow-camera-depth-from-cached-world-bounds.md) | 44 | 2756 | 28–56 min | 1–1.8 h | Within planning limit |
| [21.5.6 · Match wheel input and framed projection](96zzzzzzf-match-wheel-input-and-framed-projection.md) | 40 | 2343 | 24–47 min | 1–2 h | Within planning limit |
| [22.1.1 · Share flat and Arctic surface frames](96zzzzzzg-share-current-surface-lighting-and-camera-uniforms.md) | 32 | 2015 | 21–41 min | 1–2 h | Within planning limit |
| [22.1.2 · Connect headlight and backface controls](96zzzzzzh-connect-headlight-and-backface-controls.md) | 30 | 2264 | 23–46 min | 1–2 h | Within planning limit |
| [22.2.1 · Share one frame across color and ink](96zzzzzzi-share-one-frame-across-color-and-ink.md) | 45 | 2463 | 25–50 min | 1–2 h | Within planning limit |
| [22.2.2 · Fill original contact polygons](96zzzzzzj-fill-original-contact-polygons.md) | 53 | 2952 | 30–60 min | 1–2 h | Within planning limit |
| [22.2.3 · Broadcast ordinary ink metadata](96zzzzzzk-broadcast-ordinary-ink-metadata.md) | 22 | 1096 | 11–22 min | 1–2 h | Within planning limit |
| [22.2.4 · Layer original contact boundaries](96zzzzzzl-layer-original-contact-boundaries.md) | 50 | 2999 | 30–60 min | 1–2 h | Within planning limit |
| [22.2.5 · Query original faces under contact overlays](96zzzzzzm-query-original-faces-under-contact-overlays.md) | 13 | 723 | 8–15 min | 1–2 h | Within planning limit |
| [22.3.1 · Respect original boundary pens](96zzzzzzn-respect-original-boundary-pens.md) | 53 | 2899 | 29–58 min | 1–2 h | Within planning limit |
| [22.3.2 · Close original boundary joins](96zzzzzzo-close-original-boundary-joins.md) | 54 | 2999 | 30–60 min | 1–2 h | Within planning limit |
| [22.3.3 · Draw default mesh vertex dots](96zzzzzzp-draw-default-mesh-vertex-dots.md) | 33 | 2411 | 25–49 min | 1–2 h | Within planning limit |
| [22.3.4 · Taper crowded original mesh vertices](96zzzzzzq-taper-crowded-original-mesh-vertices.md) | 42 | 2444 | 25–49 min | 1–2 h | Within planning limit |
| [22.4.1 · Share geometric ambient inputs](96zzzzzzr-share-geometric-ambient-inputs.md) | 14 | 1379 | 14–28 min | 1–2 h | Within planning limit |
| [22.4.2 · Shade surfaces before drawing ink](96zzzzzzs-shade-surfaces-before-drawing-ink.md) | 2 | 456 | 5–10 min | 1–2 h | Within planning limit |
| [22.4.3 · Keep contact ambient inputs on originals](96zzzzzzt-keep-contact-ambient-inputs-on-originals.md) | 19 | 1643 | 17–33 min | 1–2 h | Within planning limit |
| [22.5.1 · Keep depth and address samples separate](96zzzzzzu-keep-depth-and-address-samples-separate.md) | 73 | 2792 | 28–56 min | 1–2 h | Within planning limit |
| [22.5.2 · Reuse coherent frame targets](96zzzzzzv-reuse-coherent-frame-targets.md) | 13 | 860 | 9–18 min | 1–2 h | Within planning limit |
| [22.5.3 · Keep resolved colours in the canvas format](96zzzzzzw-match-resolve-base-and-view-formats.md) | 37 | 1380 | 14–28 min | 1–2 h | Within planning limit |
| [22.5.4 · Reuse surface pipelines for each sample count](96zzzzzzx-reuse-count-specific-surface-pipelines.md) | 62 | 2803 | 29–57 min | 1–2 h | Within planning limit |
| [22.5.5 · Generate ink shaders for physical samples](96zzzzzzy-generate-ink-for-physical-samples.md) | 58 | 2805 | 29–57 min | 1–2 h | Within planning limit |
| [22.5.6 · Cache matching ink pipelines and bindings](96zzzzzzz-cache-ink-pipelines-with-independent-queries.md) | 43 | 2633 | 27–53 min | 1–2 h | Within planning limit |
| [22.5.7 · Retarget existing overlay lanes](96zzzzzzza-retarget-existing-overlay-lanes.md) | 43 | 3000 | 30–60 min | 1–2 h | Within planning limit |
| [22.5.8 · Read section crossings at each sample](96zzzzzzzb-read-section-crossings-at-each-sample.md) | 68 | 2819 | 29–57 min | 1–2 h | Within planning limit |
| [22.5.9 · Compose cloud colour, depth and queries](96zzzzzzzc-compose-cloud-depth-with-physical-addresses.md) | 62 | 2917 | 30–59 min | 1–2 h | Within planning limit |
| [22.5.10 · Cache cloud and sheet sample recipes](96zzzzzzzca-cache-cloud-and-sheet-samples.md) | 61 | 2197 | 22–44 min | 1–2 h | Within planning limit |
| [22.5.11 · Retarget cached text colour](96zzzzzzzd-retarget-cached-text.md) | 38 | 1798 | 18–36 min | 1–2 h | Within planning limit |
| [22.5.12 · Cache section masks and the final outline blend](96zzzzzzze-cache-outline-section-recipes.md) | 66 | 2990 | 30–60 min | 1–2 h | Within planning limit |
| [22.5.13 · Resolve float outline coverage](96zzzzzzzf-resolve-float-outline-coverage.md) | 66 | 2977 | 30–60 min | 1–2 h | Within planning limit |
| [22.5.14 · Retarget original ambient properties](96zzzzzzzg-retarget-original-ambient-properties.md) | 67 | 2817 | 29–57 min | 1–2 h | Within planning limit |
| [22.5.15 · Share ambient shader recipes](96zzzzzzzh-share-ambient-shader-recipes.md) | 64 | 2977 | 30–60 min | 1–2 h | Within planning limit |
| [22.5.16 · Correct ambient at physical samples](96zzzzzzzi-correct-ambient-at-physical-samples.md) | 53 | 2989 | 30–60 min | 1–2 h | Within planning limit |
| [22.5.17 · Cache physical depth-pyramid inputs](96zzzzzzzj-cache-physical-depth-pyramid-inputs.md) | 15 | 2973 | 30–60 min | 1–2 h | Within planning limit |
| [22.5.18 · Retarget the complete colour frame](96zzzzzzzk-retarget-the-complete-colour-frame.md) | 38 | 2644 | 27–53 min | 1–2 h | Within planning limit |
| [22.5.19 · Choose bounded frame quality](96zzzzzzzl-choose-bounded-frame-quality.md) | 31 | 1811 | 19–37 min | 1–2 h | Within planning limit |
| [22.6.1 · Retain visible instance ranges](96zzzzzzzm-retain-visible-instance-ranges.md) | 34 | 2760 | 28–56 min | 1–2 h | Within planning limit |
| [22.6.2 · Prepare original ink families](96zzzzzzzn-prepare-original-ink-families.md) | 23 | 1819 | 19–37 min | 1–2 h | Within planning limit |
| [22.6.3 · Show original ink families](96zzzzzzzo-show-original-ink-families.md) | 46 | 2922 | 30–59 min | 1–2 h | Within planning limit |
| [22.6.4 · Apply semantic view controls](96zzzzzzzp-apply-semantic-view-controls.md) | 39 | 2688 | 27–54 min | 1–2 h | Within planning limit |
| [22.6.5 · Draw original CAD wire curves](96zzzzzzzq-draw-original-cad-wire-curves.md) | 32 | 2976 | 30–60 min | 1–2 h | Within planning limit |
| [Reference comparison · Compare with the maintained browser runtime](97-finish-with-the-maintained-browser-runtime.md) | 30 | 2272 | 23–46 min | 0.5–1 h | Within planning limit |

## Current lesson audit completed

- [x] Count the exact source edits for all 1361 current checkpoints.
- [x] Refactor the long command-dock lesson into small, understandable responsibilities.
- [x] Split every current over-limit checkpoint into meaningful runnable lessons.
- [x] Put separate typing and study estimates at every current lesson’s top.
- [x] Verify each new endpoint in Rust and Chrome; capture its actual result.
- [x] Update the full roadmap, navigation and recovery instructions.
- [x] Recheck total lesson counts after splitting; the current 1361 slots are not a fixed final count.
- [ ] Apply the same direct style and typing limit to the 62 remaining feature lessons as they are written.

[Complete course checklist](roadmap.md) · [What the finished course must preserve](destination.md)

Check files explicitly labelled **Copy this check file** are excluded from typing time. They remain part of the complete checkpoint and its source comparison, build and native checks. Existing lessons retain their previous estimates.
