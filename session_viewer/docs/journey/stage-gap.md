# Stage tutorial ownership and integration

The [early mathematics subject](03a-mathematics.md) teaches the complete 2D stage core. Its standalone checkpoint contains 17 production modules, verified by `scripts/course/stage-core.test.ts`. The complete checkpoint passes 24 native tests, a WebAssembly build and browser checks at desktop and upright phone dimensions using software WebGPU. The active cumulative course does not yet include it.

## Two subjects, one implementation

| Subject | Source under `src/app/stage/` | What the learner implements |
| --- | --- | --- |
| A slide of mathematics, after first egui painting | font, tex, paint, layout, matrix, object, tween, actions, draw, run, morph, deck, eval, check | Frame mapping, glyph metrics, TeX, atlas, ordered painting, column alignment, matrices, time, actions, replay, matching and deck errors. |
| A slide in the real scene, after camera, rows, strokes, arrows, markers and loading | geometry, view, net, spatial, mod | Kernel geometry, scene-row updates, camera fitting and label projection, spatial motion, net solve, input and loading. |

The first checkpoint supplies `geometry.rs`, `view.rs` and `net.rs` as support files because the deck and runtime refer to their types. Their implementation is retained for the later subject. Supplying a file is not evidence that its subject has been taught.

The core does not require viewer GPU scene allocation. Its geometry morph helper does call the live Rust kernel; the checkpoint therefore uses the kernel as a path dependency. `spatial.rs` and `StageApp` cross into scene rows, camera state and application events. These are the later integration boundary.

Keep the full painter and runtime in their owning subject. Do not introduce a small formula-only substitute that a subsequent lesson must replace. Named source sections explain mechanisms; complete source files provide the runnable implementation. The [source comparison](../lessons/03a-math/source-parity.json) records the copied modules, but runtime acceptance still requires builds and browser review.

## Application registration still required

Current stage integration touches `src/lib.rs`, state, loader dispatch and `src/app/ui/mod.rs`. Installing it by teaching successive edits to those completed subjects would violate the ownership rule.

The smallest proposed interface is an application-mode registration with three operations: accept a loaded payload, consume input, and render a frame. The application subject owns dispatch and shared state; the later stage subject supplies its mode adapter. GPU lanes already have their own registry, so this does not require a second GPU renderer. No production refactor has been applied. The exact mode interface and its behavior-preserving tests remain to be implemented before the cumulative checkpoint can claim integration.

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
