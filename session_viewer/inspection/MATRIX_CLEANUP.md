# Matrix visualization cleanup (Phase 11): removal plan

**Status: approved and executed 2026-11-19** (D1 remove, D2 kernel Matrix stays). Not done: recapturing Tutorial 7 in a browser (D3). The course records `completion-batch.json` (pinned to commit `87f844e`) and `production-course-map.json` are historical and were left unchanged; `completion-teaching.json` was updated.

Requirement (`inspection/audit_agent.txt`, Phase 5 and Phase 11): remove the 3D matrix visualization with black outlines and rectangular graphics from the implementation and the tutorials, and keep the 2D rounded-rectangle matrix visualization.

The identification evidence is in `SHADER_INVENTORY.md` §6. In short:

- **The 3D matrix (remove).** This is the kernel `Matrix` component lane: `walk/matrix.rs` → `gpu/matrices.rs` → `shaders/matrix.wgsl`. It draws square cells lying in the 3D scene, separated by black 1-px edges (`LINECOLOR = [0,0,0,1]`, `walk/matrix.rs:21`; `LINE_PX`, `matrix.wgsl:38`), with seven-segment white digits. The reference image `docs/screenshots/active/geometry-four-objects-1280.png` shows it: sha256 `2ee54686…`, which matches `?v=2ee546863e22` in the brief.
- **The 2D matrix (keep).** This is the stage deck matrix, `app/stage/matrix.rs` (`Stage::matrix`): rounded, gapped cells (`radius: 0.16 * min(cw, ch)`, `stage/matrix.rs:139`) painted by egui (`stage/paint.rs:445-466`, `stage/draw.rs:26-43`). See `docs/screenshots/active/math-matrix-light-1280.png` and `-dark`.

## 1. Source changes (3D lane)

All line numbers are from the working tree as audited on 2026-11-19.

| # | File | Change | Lines |
|---|---|---|---|
| 1 | `src/shaders/matrix.wgsl` | Delete the file | 1-284 |
| 2 | `src/engine/gpu/matrices.rs` | Delete the file, including its two unit tests `header_offsets_match_the_shader` and `blocks_start_after_their_values` | 1-264 |
| 3 | `src/app/walk/matrix.rs` | Delete the file (`TYPE_NAME`, `holds_matrix`, `is_matrix`, `walk`, `title`, `count`, `color`, `color_f32`) | 1-169 |
| 4 | `src/engine/gpu/mod.rs` | Remove `pub mod matrices; // register:matrices` | 18 |
| 5 | `src/engine/gpu/lane.rs` | Remove `super::matrices::REGISTERED, // register:matrices` from `REGISTRY`. `vectors` and `planes` stay | 88 |
| 6 | `src/engine/gpu/context.rs` | Remove `out.extend_from_slice(matrices::SHADERS); // register:matrices` (test-only `lane_shaders`) | 412 |
| 7 | `src/app/walk/mod.rs` | Remove `pub mod matrix; // register:matrices` | 31 |
| 8 | `src/app/scene.rs` | Remove `use crate::app::walk::matrix;` (17). Remove `use crate::app::walk::plane::unit;` (19, used only at 825-826). Remove `use crate::engine::gpu::matrices::MatrixRows;` (21). Remove `use crate::engine::text::{TextLabel, TextPlacement};` (24, used only at 120 and 816-823). Drop `Vector` from line 26 (used only at 813-814). Remove the `matrix_titles` field (120) and its initialiser (206), the `add_matrices` call (550), and the whole `impl Scene { fn add_matrices … }` block | 17, 19, 21, 24, 26, 120, 206, 550, 777-837 |
| 9 | `src/app/scene_text.rs` | Remove `add_matrix_titles` | 75-89 |
| 10 | `src/state.rs` | Remove `self.scene.add_matrix_titles(&mut self.gpu); // register:matrices` | 131 |
| 11 | `src/selftest/stroke_witness.rs` | Remove `scene.add_matrix_titles(&mut gpu);` | 499 |
| 12 | `src/app/loader.rs` | Drop the `&& !crate::app::walk::matrix::holds_matrix(&session)` clause | 469 |
| 13 | `src/app/live.rs` | Same clause | 371 |
| 14 | `src/app/decode.rs` | Comment only: drop "a kernel Matrix among them is drawn by the matrix lane". The component decode stays | 328 |
| 15 | `examples/course/geometry.rs` | Remove `Matrix` from the `session_rust` import (7) and the matrix block (67-77). The scene goes from four objects to three. Row index `3` was used only by the matrix | 7, 67-77 |

Keep, because other code uses these: `lane.rs` `REGISTRY` / `RowLane` / `Registered`; `engine/pipelines` `ink_module`, `ink_visibility.wgsl`, `Layouts::ink_rows`; `TextPlacement::WorldPlane` and `text_plane.wgsl` (world labels); `walk/encode.rs::pack_rgba`; `walk::Row::thin`; `walk/plane.rs::unit` (used inside `plane.rs`); `scene.rs::placement` (also used at `scene.rs:514` and `scene_instances.rs:159`); the generic component decode (`decode.rs:329-334`). Do not change the kernel `session_rust` / `session_py` / `session_cpp` `Matrix` class or `Session::add_matrix` from this viewer change. That is a separate cross-language decision (§5).

The course tooling needs no code change. `session_tests/scripts/course/completion.ts:78-86` copies `src/engine`, `src/shaders` and `src/app/walk` whole into checkpoint `geometry`, so deleted files simply drop out. Its `mathematics` list (`completion.ts:6-8`, which includes `'matrix'`) refers to `src/app/stage/matrix.rs` and **must stay**.

## 2. Docs, tutorials and screenshots

| Item | Path | Proposed action |
|---|---|---|
| Tutorial 7 page | `docs/journey/completion-geometry.md` | Edit lines 3 (summary sentence), 5-6 (screenshot captions), 14 ("four-object"), 148-150 (Step 3 heading and illustration alt text), 188-201 (matrix file table and the `matrix.wgsl` paragraph with the `kernel-matrix-upload` and `matrix.wgsl:7:38` includes), 238 ("`4`-object scene"), 243, 253 (run and observe) and 262 (Q&A). Also correct "`16` specialized WGSL lane shaders" (line 3) to the new count |
| Tutorial 7 screenshots | `docs/screenshots/active/geometry-four-objects-1280.png`, `geometry-reveal-1280.png`; entries in `docs/screenshots/active/evidence.json:58-72` | Recapture from the 3-object checkpoint under a new name (for example `geometry-three-objects-1280.png`). Update `evidence.json` (asset, sha256, bytes, caption). The published URL in the brief will go stale |
| Illustration | `docs/illustrations/stage/07c-glyphs-splats-matrix-widget.svg` (panel "matrix.wgsl · textureless 7-segment GPU cells"); referenced only by `completion-geometry.md:150` | Redraw without the matrix panel (for example `07c-glyphs-splats-widget.svg`) and update the link |
| Course records (generated) | `docs/journey/completion-batch.json` (1351, 1531, 1873), `docs/journey/completion-teaching.json` (2203, 2583, 3223-3229), `docs/journey/production-course-map.json` (feature "Matrix cells, procedural digits and independent world titles", around 2167-2215) | Regenerate with the course tooling. Do not hand-edit. Remove the production-course-map feature or mark it as an approved omission |
| Redraft planning docs | `docs/journey/redraft-10-sections/sections/section-10-shared-gpu-precision-and-lighting.md` (43, 61, 69, 105), `docs/journey/redraft-10-sections/10-section-course-outline.md` (650, 668), `docs/journey/redraft-10-sections/{README.md, old-to-new-coverage-map.*}` | Remove the "Matrix procedural digit/cell shader" items from the plan |
| Historical lesson and snapshots | `docs/journey/96zzzzzzzq-draw-original-cad-wire-curves.md` (46,727 lines; matrix-adapter steps from line 7245, includes up to 46383), 76 files `docs/journey/code/96zzzzzzzq-draw-original-cad-wire-curves-*matrix*`, plus `docs/journey/96zzzzf-…`, `96zzzzg-…`, `consolidation/96-…`, `consolidation/96zzzzzzzm-…` | Leave unchanged. Per `.claude/rules/viewer-docs.md` these are historical reconstruction inputs. Archive the matrix steps with stable links only if tooling no longer consumes them; check that before moving anything |
| Historical evidence | `docs/journey/release/matrix-browser-review.json`, `matrix-native-review.json`, the `math-boundary-validation-ledger-*`, `completion-geometry-*`, `completion-decoding-*` JSON, `docs/journey/coverage-audit-2026-11-12.json`, `current-viewer-*.json` | Do not rewrite (rule: never rewrite historical evidence). The review PNGs they reference (`docs/screenshots/journey/review/96zzzzzzzq-*matrix*`) are **not present** in the working tree |

Not related, despite the name: `docs/journey/09-matrices.md`, `docs/screenshots/journey/09-matrices*.png`, `70a-…-world-matrices`, `96zzzzzr-…-frame-matrices`, `96zzzzzzd-…-matrix-inversion` (camera and transform matrices), and `docs/lessons/*/tests/depth/_probe_matrix.py`, `_hidden_line_matrix.py` and `tests/depth/*matrix*.py` (test-case matrices).

## 3. What must not be removed (2D rounded-rectangle matrix)

| Area | Paths |
|---|---|
| Layout and drawing | `src/app/stage/matrix.rs` (265 lines), `stage/deck.rs:135, 170-181, 235, 337-410` (`Content::Matrix`, `MatrixSpec`, `matrix:` parsing), `stage/layout.rs:104-116`, `stage/object.rs:57-65, 171-190`, `stage/actions.rs:1-20, 185-201, 363`, `stage/run.rs:459`, `stage/net.rs:163` (net matrices from the solver), `stage/draw.rs:26-60` (`rounded`, `cells_of`), `stage/paint.rs:445-466`, `stage/tex.rs` (`Shape::Rect`) |
| Renderer | egui / egui-wgpu through `src/engine/gpu/ui.rs`. No WGSL |
| Tests | `src/app/stage/tests.rs:268-289, 394-460` (`yellow_matrix_highlights_use_black_numbers_and_accent_labels`, `matching_matrix_symbols_are_colored_together`, `matrix_cells_use_vivid_sign_fills`), `stage/presentation_tests.rs` |
| Decks (YAML key `matrix:` and matrices inside `row:`) | `docs/lessons/03a-math/example.yaml`, `examples/stage/equilibrium/01-equilibrium.yaml`, `examples/stage/force-density/01-method.yaml`, `examples/stage/force-density-derivation/{01-why,04-jacobian,05-matrix-form}.yaml`, `examples/stage/force-density-materialization/01-materialization.yaml`, `examples/stage/force-density-matrices/{01-givens,02-matrices,04-arch,05-several-nodes,06-net,07-pillow}.yaml` |
| Tutorials | `docs/journey/completion-mathematics.md` (Tutorial 3), `docs/journey/checkpoints/03a-mathematics.md`, `docs/lessons/03a-math/src/app/stage/matrix.rs`, `docs/journey/code/03a-mathematics-cumulative-12-matrix.rs`, `docs/journey/course.json` step `03a-mathematics` (path `src/app/stage/matrix.rs`, line 235) |
| Screenshots | `docs/screenshots/active/math-matrix-dark-1280.png`, `math-matrix-light-1280.png`, `stage-fdm-matrices-light-1280.png`, other `stage-*` captures |
| Course tooling | `session_tests/scripts/course/completion.ts:6-8` (`'matrix'` in the mathematics module list) |

Nothing matrix-specific is shared between the two paths. The `stage/` sources contain no reference to `MatrixLane`, `MatrixRows`, `walk::matrix`, `matrix.wgsl` or `session_rust::Matrix`. I checked this with a grep of `src/app/stage/*.rs`.

## 4. Risk

| Risk | Likelihood | Mitigation |
|---|---|---|
| A published session whose only content is a kernel `Matrix` component is now reported as "holds no geometry; skipped" (`loader.rs:466-472`, `live.rs:368-374`) instead of drawing | Certain for such files. No such file was found: a grep for `Matrix` over `session_data` `*.pb`, `*.json`, `*.toml` and `*.yaml` and over the viewer's `examples/` found none. The only hits were unrelated PDFs | Accepted by the requirement. A Matrix component inside a session with other geometry is decoded and silently not drawn |
| Kernel APIs keep producing `Matrix` components that the viewer no longer shows (`session_rust/src/session.rs:1695` `add_matrix`, and the Python and C++ equivalents) | Certain | Decision D2 below |
| Course tooling or the strict Vue build fails because of dangling includes (`--8<-- "session_viewer/src/app/walk/matrix.rs:kernel-matrix-upload"`, `matrix.wgsl:7:38`) or stale records | High if docs are not edited in the same change | Edit `completion-geometry.md` and regenerate the course JSON in the same commit. Run the strict `session_tests` build and the course structural checks |
| Hidden dependence on `REGISTERED == 3` in arrays or tests | Low. `REGISTERED` is `REGISTRY.len()` (`lane.rs:93`) | `cargo test` plus the naga validation test `instance/validation.rs:106` |
| Unused-import warnings or errors in `scene.rs` (`Vector`, `unit`, `TextLabel`, `TextPlacement`) | Certain if left in | Listed in step 8. Confirm with `cargo check` / clippy |
| Screenshot URL in the brief stops matching | Certain | Recapture and update `evidence.json` |

Overall risk is **low**. The lane is self-contained, it has registered extension seams (`// register:matrices` on every touch point), and no other feature reads its buffers.

## 5. Decisions requiring approval

- **D1.** Approve deleting the 3D lane: §1 items 1-15.
- **D2.** Kernel `Matrix` class and `Session::add_matrix` across `session_rust`, `session_py` and `session_cpp`. Should they stay as data only (recommended: the viewer change does not need them removed), or be removed in a separate three-kernel change?
- **D3.** Tutorial 7: recapture the screenshots under new names, keep the old PNGs for links, or archive them under `docs/archived/`?
- **D4.** Historical lesson `96zzzzzzzq` matrix-adapter steps and their 76 `docs/journey/code/` snapshots: leave them as historical inputs (recommended), or archive them?

## 6. Verification once approved

The build limits in `CLAUDE.md` apply: `buildslot`, `-j8` cargo, memory cap.

1. `cargo check --target wasm32-unknown-unknown` and native `cargo test` (the naga validation `shader_validation_and_layouts`, the lane and scene tests).
2. `grep -rn 'register:matrices\|walk::matrix\|MatrixRows\|matrix.wgsl' src examples` returns nothing.
3. Course: materialise checkpoint `geometry` (`npm --prefix ../session_tests run course -- checkpoint geometry …`), compile it, and run the strict Vue build.
4. Browser: Tutorial 7 shows the mesh cube, BRep cube and NURBS curve. Tutorial 3 and a `force-density-matrices` deck still show the rounded 2D matrices, light and dark.
