# Stage tutorial ownership and integration

The [early mathematics subject](03a-mathematics.md) teaches the complete 2D stage core. Its standalone checkpoint contains 17 production modules, verified by `scripts/course/stage-core.test.ts`. The complete checkpoint passes 24 native tests, a WebAssembly build and browser checks at desktop and upright phone dimensions using software WebGPU. The active cumulative course does not yet include it. Its 17 copied modules comprise the 14 mathematics-core modules plus `geometry`, `view` and `net` support. `Presentation` and the generic mathematics mode are checked by the separate production-mode probe; they are not yet wired into that standalone browser harness.

The [real-scene integration subject](stage-scene.md) explains the current row, camera, solver and input boundaries. Its cumulative checkpoint remains pending.

## Two subjects, one implementation

| Subject | Source under `src/app/stage/` | What the learner implements |
| --- | --- | --- |
| A slide of mathematics, after first egui painting | font, tex, paint, layout, matrix, object, tween, actions, draw, run, morph, deck, eval, check, presentation, presentation_tests, math_mode | Frame mapping, glyph metrics, TeX, atlas, ordered painting, column alignment, matrices, time, actions, replay, matching, deck errors and presentation ownership/painting tests. |
| A slide in the real scene, after camera, rows, strokes, arrows, markers and loading | geometry, view, net, spatial, mode, mod | Kernel geometry, scene-row updates, camera fitting and label projection, spatial motion, net solve, input and loading. |

The first checkpoint supplies `geometry.rs`, `view.rs` and `net.rs` as support files because the deck and runtime refer to their types. Their implementation is retained for the later subject. Supplying a file is not evidence that its subject has been taught.

The core does not require viewer GPU scene allocation. Its geometry morph helper does call the live Rust kernel; the checkpoint therefore uses the kernel as a path dependency. `spatial.rs` and `StageApp` cross into scene rows, camera state and application events. These are the later integration boundary.

Keep the full painter and runtime in their owning subject. Do not introduce a small formula-only substitute that a subsequent lesson must replace. Named source sections explain mechanisms; complete source files provide the runnable implementation. The [source comparison](../lessons/03a-math/source-parity.json) records the copied modules, but runtime acceptance still requires builds and browser review.

## Application registration still required

Current stage integration touches `src/lib.rs`, state, loader dispatch and `src/app/ui/mod.rs`. Installing it by teaching successive edits to those completed subjects would violate the ownership rule.

The production application now has one loaded-mode message and one active-mode slot. `Loaded<Host>` consumes the fetched payload and constructs `Mode<Host>`; the mode owns its key handler, pointer handler, frame callback and clear colour. [The contract](../../src/app/mode.rs) is generic over the host, so the mathematics implementation can compile without camera or scene types. The later [scene mode](../../src/app/stage/mode.rs) chooses the full viewer `State`. The local native, native test-configuration and WebAssembly checks pass. The optimized build also passes; cumulative acceptance remains pending.

```rust
--8<-- "session_viewer/src/app/mode.rs:application-mode"
```

The application owns the slot and dispatch once. Loading installs a boxed mode; input borrows it, and a frame temporarily takes it out of the host so its callback can borrow the host exclusively. Restoring the box before rendering keeps its current background available to the GPU clear. The UI owns input collection, platform output and texture uploads; the mode owns its content. This avoids adding a new concrete stage variant and new stage-specific frame code to completed application files for every mode.

The early painting checkpoint currently owns a concrete `Panel::draw`; the dock replaces that file and the browser loop. Inserting mathematics only in that draw method would make it disappear when the dock checkpoint is replayed. Moving the standalone browser harness into the course would instead replace the finished first-frame implementation. Neither is cumulative integration.

The application owner should establish mode dispatch before first painting, with registration entries supplying the loaded payload constructor, input handler and frame callback. Within the frame callback, preserve production's order: advance stage time, apply the camera and derive its label projection, collect the mathematics paint output and spatial descriptions, synchronize scene rows, prepare egui resources, then submit the scene and UI rendering. `StageApp::paint` produces the spatial list, so synchronizing rows before that call would use stale geometry. The early mathematics adapter needs only the core and painter; the later scene adapter adds the `State`-dependent synchronization. Do not split or rewrite the 17 finished core modules to accomplish this.

`Presentation` provides the painting boundary and shared navigation. Its mathematics mode ticks and paints without using the host. `StageApp` retains pointer state, scene allocation and browser state publication; its frame callback performs camera projection and row synchronization around the same painter. The new native test exercises navigation, theme changes and painting with a numeric host, without constructing a scene. The native probe passes all 18 tests, including the host-independent mode test and complete production-deck playback. Two initial compiler/test attempts were stopped by the storage guard; after approved cache recovery, the same compiled executable passed under the resource guard with unchanged source and binary hashes. Full application native, native test-configuration and WebAssembly compilation also pass. The optimized build passes. Two equilibrium browser comparisons match the baseline; the separate history assertions still fail. A factory or module registration is wiring; changing an earlier callback's implementation remains an ownership failure.

The recorded 17-test probe and full application native, native test-configuration, wasm and optimized Trunk checks apply to the published Presentation extraction before this mode dispatch change. The standalone mathematics sources remain unchanged. The mode interface now has its own 18-test native probe plus passing full application native, test-configuration and WebAssembly checks. Its optimized build passes; cumulative registration remains pending. The [Windows mode receipt](release/mode-native-windows-20261114.json) records the compiled inputs, executable, probe construction, actual commands, resource limits and interrupted attempts.

For the mode boundary, the [equilibrium browser receipt](release/mode-browser-windows-20261114.json) records 46 exact paired state-and-pixel frames, and the [force-density receipt](release/mode-force-density-windows-20261114.json) records 120 more across chapters 1 through 3. Eight of 46 chapter/viewport comparisons are complete; 38 remain. All eight complete commands still fail the two history assertions described below. Chrome uses the Intel gen-12lp adapter; upright is an emulated viewport. The optimized build and these comparisons do not establish cumulative acceptance.

The separate Presentation extraction receipts cover 166 paired state-and-pixel frames across the same eight chapter/viewport cases. They bind an earlier source tree and do not extend the current Mode coverage.

Every complete case command still fails two separate history-frame assertions. Automatic label placement remembers prior directions, so replay and subsequent pointer stepping need not match an earlier animated frame; the unchanged baseline reproduces this failure. These successful local-versus-production comparisons do not establish full stage or cumulative acceptance. The [equilibrium receipt](release/presentation-production-20261113.json), [force-density chapter 1 receipt](release/stage-force-density-windows-20261114.json) and [chapters 2 and 3 receipt](release/stage-force-density-2-3-windows-20261114.json) retain the actual failed command status and each exact comparison.

## Existing subjects that must absorb current behavior

| Existing owner | Required content |
| --- | --- |
| Object identity and rows | 112-byte instance rows, effect values, placement writes and non-pickable decoration. |
| Strokes and connected paths | 72-byte stroke rows, cumulative dash phase, endpoint trims, curve heads and authored widths. |
| Arrows | Shared vector/plane shader, 64-byte vector rows and field-by-field `arrow_corner` arguments. |
| BRep display | Curve quality, parameter recovery, contact display and silhouettes. |
| Documents and hierarchy | Graph attributes, feature/own lines, scene publication and redraw state. |
| Rendering and self-test | Matrix lane, backdrop, splat selection, pacing, current oracle and stroke witnesses. |
| Build and loading | Font/raster dependencies, release configuration, deck copying and fetch alongside wasm. |

The journey's current arrow checkpoint still teaches a different shader. Its production notes do not establish implementation parity. The [debugging guide](../debugging.md#probe-a-phone-only-shader-failure) explains the device-probe method and the field-argument constraint.

## Acceptance

Each subject needs one visible goal, prediction experiment, question, current source and runnable checkpoint. The early subject must pass all parser/layout tests and the lesson deck; the later subject must play every step of the five shipped decks and be reviewed at desktop and upright phone sizes. Geometry must use the real scene and camera; mathematics uses MATH-table layout and egui glyph quads.

The full task also covers the command dock, planes, matrix display, splats, box selection, scene display and live revisions. None is excluded by the stage split. The [initial per-file audit](coverage-audit-2026-11-12.json) remains a dated baseline, not current acceptance. The active course must eventually prove source and behavior parity with the maintained viewer; historical lesson 37 is not that destination.

On the integrated handoff `b2b922817`, the reconstructed final journey has 712 files under `src/`, compared with 323 production files. Exact path comparison finds 318 missing, 707 extra and three changed shared files. Renamed implementations can teach related concepts, but this endpoint does not yet produce the required production tree. Run `course -- parity --details` to inspect the actual cumulative comparison; `reference-parity` separately preserves the historical lesson 37 check.

The current local mode implementation has 328 production source files. A fresh cumulative comparison reports 323 missing, 707 extra and three changed files; the unchanged structural checker reports 25 files absent from the historical source inventory. These are outstanding failures, not approved exclusions. The 17-module standalone parity and recorded-evidence checks still pass.
