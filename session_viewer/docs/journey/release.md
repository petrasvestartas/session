# Course release: command-line checkpoints

**All 39 current checkpoints build and pass their scripted checks in visible Chrome 153.0.8010.36.** They cover lessons 01–28 plus placement and source-ownership follow-ups, and the four early dock lessons, 03a–03d. Every interactive feature check types into the viewer’s actual egui command field. Canvas picking, dragging and wheel navigation retain their natural input paths. Keyboard feature shortcuts are absent.

This is the opening of the full viewer course. The [remaining lesson checklist](roadmap.md) and [destination contract](destination.md) still govern completion.

## What was checked

- Every displayed checkpoint reconstructs, builds for WebAssembly, and produces a Trunk browser bundle. Native state tests and GPU readbacks run where applicable.
- Every checkpoint fills the browser content area from its first frame. Chrome checks the canvas position and dimensions, rejects visible headings or teaching paragraphs, and checks that the GPU image matches the initial window size.
- The course’s dock model, layout and theme match the production source. An automated check rejects HTML feature buttons and button-based lesson actions.
- Chrome types commands through actual keyboard events. Enter clears the field and records exactly one command. Completion and Escape are checked separately.
- Lesson 04 submits Background twice and compares every scene pixel above the folded dock. The second submission restores the original drawing exactly.
- Later checks cover command actions, picking, resizing, orbit and release, wheel without page scrolling, immediate command-field focus without losing the first character, and Open with whole-import Undo/Redo.
- Fit is checked for repeatability, recovery after panning, document history, and geometry margins in wide and tall windows.
- Placement lessons keep local mesh coordinates separate from model matrices; bounds, ray picking and GPU drawing share world placement. Typed Move checks world-axis composition, immutable shared geometry, Undo/Redo, finite arguments and zero-offset history. Chrome checks rendered interior picking and visible command errors; exact raster-edge ownership is deferred to GPU picking.
- The source-ownership lessons retain original kernel meshes beside their derived display arrays. The final endpoint passes 51 native tests, including double-coordinate preservation, adapter failure, imported allocation identity and ownership across Move/Undo/Redo. Chrome checks import, placed drawing, history and interior picking; visibility and locking policies are still later work.
- Lesson 25 checks the perspective/orthographic pixel round trip and selects a visible imported beam in each projection. Its 39 Rust tests also check camera, fitting, picking and document invariants.

The scene comparison excludes the command strip. When a key opens completion over the drawing, the test dismisses that overlay before checking camera pixels. Full-page screenshots still show the real interface.

## Screenshot evidence

Each lesson includes its own Chrome capture from the reconstructed browser bundle. The capture file records source and bundle fingerprints, common and optional checkpoint checker hashes, Chrome version, timestamp, viewport conditions and scene hash in `screenshots/journey/browser.json`. Screenshots are included only while their source and checker fingerprints match.

The initial browser viewport is 900 × 760 CSS pixels at display density 1. The evidence records that viewport, the canvas bounds and the final screenshot viewport. Resize and Fit checks also change the window size. Linux WebGPU uses the flags documented in `open-chrome.sh`. This verifies those scripted cases on this machine; it does not establish complete browser or hardware coverage.

The full-window white canvas starts in lesson 01; GPU sizing starts in lesson 02. The first two screenshots intentionally look alike: lesson 02 replaces the CSS-only background with a verified GPU clear. Each is captured from its own bundle. Lesson 10 gives the depth image matching dimensions before the first draw. Lesson 20 adds resizing after startup and display-density handling. Earlier checkpoints should be reloaded after changing the window size.

The early dock stages have different purposes: 03a draws the styling, 03b adds the model, 03c draws production history/layout, and 03d connects keyboard input. Their captions state when input becomes usable. Native GPU images are separate evidence and are never labelled browser screenshots.

## Fixed inputs and remaining scope

The course pins wgpu 29.0.4 and egui/egui-wgpu 0.34.3. Locks are introduced for the initial project, egui drawing, dock inspection dependencies, the Rust geometry kernel, and protobuf import. The dependency command preserves the previous lock and installs binary fonts; all implementation code is displayed for typing.

Early commands such as Background, Pan and Example are teaching vocabulary. The component and its styling are the real viewer dock. Later command chapters build the complete production vocabulary, argument handling, clipboard, composition and touch integration.

The original production destination remains fingerprinted separately. Browser diagnostics, device-loss guards and the shared command dock are newer than that reference, so the full source-parity audit still reports differences. The course has not yet reached production feature parity. Old reference tutorials remain until their complete replacement is verified.

The production viewer separately passed a Chrome device-loss check: stop GPU work, attempt a diagnostic download, and recover the report on reload. The reported Firefox QueueId driver crash has not been reproduced or proven fixed. A web application cannot guarantee a dump when the browser process itself crashes.

## Reproduce

From `session_viewer`:

```sh
npm --prefix ../session_tests run course -- verify
npm --prefix ../session_tests run course -- verify --stored
npm --prefix ../session_tests run course -- serve 8781
```

Install browser-test dependencies once if needed:

```sh
npm install --prefix target/course-tools playwright@1.58.2 pngjs@7.0.0
```

In another terminal:

```sh
npm --prefix ../session_tests run course -- capture
npm --prefix ../session_tests run course -- generate
```

Capture checks the build fingerprints and runs each uploaded specimen’s handwritten Rust example before launching Chrome. These commands use `target` and leave `workspace/journey` untouched. The separate `structure` audit includes the unfinished production destination comparison; its remaining differences must be resolved before declaring the entire course complete.

[Return to the course](../journey.md)
