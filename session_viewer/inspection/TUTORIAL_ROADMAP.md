# Session Viewer: Tutorial Roadmap (Phase 9)

Read-only audit of `session_viewer/docs` at commit `032cd8c0f` (2026-11-19). Inputs: `inspection/audit_agent.txt` §Phase 9, `inspection/Session Viewer.txt` §4 (matrix cleanup), §8–§9, `inspection/ARCHITECTURE.md` §1, `mkdocs.yml`, `docs/**`, `session/.claude/rules/viewer-docs.md`, `session_tests/scripts/course*`. Every number below comes from a command run on this tree (method in §6). Nothing was built.

Labels: **FACT** = read from files. **PROPOSAL** = this roadmap. **UNVERIFIED** = plausible, not checked by a build.

---

## 1. Current state

### 1.1 Tutorial series that exist

| Series | Location | Pages | Real vs redirect | In `mkdocs.yml` nav | Source form | Status (from its own pages) |
|---|---|---:|---|---|---|---|
| **Active course**: 9 cumulative subjects, chapter 7.1, chapters 9.1–9.9 | `docs/journey/completion-*.md`, `docs/journey/stage-scene.md`; index `docs/journey/active.md`; config `docs/journey/active-course.json` | 19 lesson pages (+ `active.md`, `completion-contract.md`) | all real, 60–267 lines | **No.** Reached only through links in `docs/README.md`, `docs/journey.md`, `docs/journey/active.md` | `--8<-- "session_viewer/src/…"` includes of the live tree (named sections or line ranges) | 8 of 9 accepted; `application` pending (1 of 37 GPU tests fails: `instanced_frames_match_the_baked_scene`, `src/app/scene_instances.rs:1318`) |
| **Historical journey** | `docs/journey/*.md` (other than the above), `journey/code/` (7,576 snippet files), `journey/course.json` | 1,386 top-level pages | 1,321 real; **65 one-hop redirect stubs** (3–7 lines, "Continue with …") | 7 entries ("Before you start" group: `journey.md`, recovery, corrections, destination, command-line, roadmap, release) | snippets replayed by `session_tests/scripts/course/model.ts`; 1,298 checkpoints / 110 subjects | historical; "not the active implementation queue" (`docs/journey.md`) |
| Historical consolidation | `docs/journey/consolidation/` | 109 | real (linked from `journey/contents.md`) | no | same snippets | historical |
| 10-section redraft | `docs/journey/redraft-10-sections/` | 124 | real | no; not linked from nav, README or contents | inline listings | external "Gemini Notebook redraft"; no checkpoint |
| **Archived numbered course** 00–37 | `docs/archived/NN-*.md` (43) + `docs/archived/reference-course.md`; typed chunks `docs/typing/code/` (978) + `docs/typing/manifest.json`; crates `docs/lessons/NN/` (44 snapshots, 5,501 `.rs`, 148 MB) | 44 | real | **Yes — the whole "Course" nav group (43) plus "Complete implementation"** | 978 insertion chunks (`at`/`lines` offsets) that build the lesson-37 crate (271 files) | archived; lesson 37 vs current `src`: 31 files identical, 240 differ, 81 current files absent |
| Archived journey pages | `docs/archived/journey-*.md` | 91 | real (targets of the 65 stubs) | no | inline listings | archived |
| Rust foundations | `docs/foundations.md` + `docs/foundations/01–08` + `foundations/code/` | 8 | real | `foundations.md` only | standalone Rust programs with assertions | separate notebook |
| Reference / design pages | `docs/{map,file-map,words,how-to-learn,capstone,ssao,debugging,performance-patterns,command-line-walkthrough,cad-design,maintaining,references}.md`, `docs/extend-*.md`, `ARCHITECTURE.md` | 17 | real | yes ("Reference guides", "Reference") | prose, design sketches | mixed; see §1.5 |

How the nav resolves 44 absent paths (FACT): `mkdocs.yml` is nav data only ("No MkDocs runtime is used"); `session_tests/plugins/course-archive.ts:archivedPage` maps a missing `docs/NN-*.md` to `docs/archived/NN-*.md`. The links work in the Vue site, but the sidebar "Course" group is the **archived** 44-lesson course, not the active nine-subject course.

### 1.2 How lessons are organised

| Mechanism | FACT |
|---|---|
| Active checkpoints | `npm --prefix ../session_tests run course -- checkpoint ID DIR` materialises one of 9 assemblies (`frame` 13 files → `painting` 15 → `mathematics` 42 → `dock` 46 → `camera` 48 → `rows` 69 → `geometry` 167 → `decoding` 171 → `application` 370). |
| Course runtime | `examples/course/{entry,registration,canvas,frame,painting,mathematics,dock,camera,rows,geometry,decoding}.rs` + `index.html`, compiled through `src/lib.rs:31-33` (`#[cfg(all(target_arch="wasm32", feature="tutorial"))] #[path="../examples/course/entry.rs"] mod course;`), dispatched by `?tutorial=ID`. |
| Additive registration | 198 distinct `// register:NAME` tags in `src/` (e.g. `state.rs` 69 tag lines, `engine/gpu/context.rs` 86, `app/command/verbs/mod.rs` 79, `app/scene.rs` 57). A checkpoint is the production file with later subjects' tagged lines removed. |
| Teaching witnesses | `docs/journey/completion-teaching.json`: 360 entries (352 `src` + 8 support files), each = owner subject + one paragraph + source SHA-256. 354 hashes match the current tree; 6 differ (`app/stage/{actions,draw,geometry,paint,run,tests}.rs`). |
| Planned regrouping | `docs/journey/completion-batch.json`: 17 subject groups, `status: planned`, pinned to commit `87f844e41`, 328 files (current: 352), `teaching: null` for all 328. |
| Patches / materialised git series | **Removed.** `docs/reconstruction/{series_git,fold,check_steps,course_pages,…}.py` and `patches/` were deleted in `7143d361c` (2026-09-22, "51 editable lesson crates, no generator"). Left on disk: `docs/reconstruction/__pycache__/replay.cpython-313.pyc` and `.ruff_cache/` (untracked residue). |

### 1.3 Complete source or partial listings

| Scope | FACT |
|---|---|
| Active pages show any line of | 93 of 352 `src` files |
| Active pages show the whole file | 10 files: `app/launch.rs`, `app/yielding.rs`, `command_dock/{theme,view}.rs`, `engine/gpu/ui.rs`, `shaders/{ambient_depth,background,physical,slot_table,vector}.wgsl` |
| Lines shown on active pages | 4,349 of 111,492 `src` lines (3.9 %) |
| Files with only a witness paragraph (no code) | 259 |
| WGSL shown whole | 5 of 33 (all others are line ranges, e.g. `matrix.wgsl:7:38` of 284) |
| `application` checkpoint | +199 files in one step (171 → 370) with paragraph witnesses; this is the "final source dump" the course contract forbids (`completion-contract.md`) |
| Archived course | complete text, but as 978 positional insertions into a 271-file crate that no longer matches `src` |

Per active page:

| Page | `src` files it owns (witness) | Their lines | Files with an include | Lines shown |
|---|---:|---:|---:|---:|
| `completion-frame.md` | 6 | 513 | 3 | 159 |
| `completion-painting.md` | 1 | 105 | 1 | 105 |
| `completion-mathematics.md` | 21 | 9,287 | 12 | 771 |
| `completion-dock.md` | 3 | 774 | 3 | 141 |
| `completion-camera.md` | 1 | 866 | 1 | 103 |
| `completion-rows.md` | 16 | 3,244 | 8 | 287 |
| `completion-geometry.md` | 74 | 22,979 | 19 | 805 |
| `completion-visibility.md` | 20 | 6,930 | 13 | 555 |
| `completion-decoding.md` | 3 | 1,716 | 3 | 58 |
| `completion-application.md` | 3 | 1,734 | 2 | 41 |
| `completion-documents.md` (9.1) | 25 | 13,271 | 5 | 101 |
| `completion-commands.md` (9.2) | 44 | 5,007 | 4 | 236 |
| `completion-input.md` (9.3) | 17 | 2,348 | 4 | 129 |
| `completion-modeling.md` (9.4) | 62 | 20,650 | 2 | 37 |
| `completion-loading.md` (9.5) | 7 | 3,172 | 4 | 120 |
| `completion-text.md` (9.6) | 10 | 3,829 | 2 | 109 |
| `completion-streaming.md` (9.7) | 6 | 2,781 | 3 | 88 |
| `stage-scene.md` (9.8) | 6 | 1,637 | 5 | 509 |
| `completion-debugging.md` (9.9) | 27 | 10,649 | 2 | 38 |

Format: every active page has the same seven sections (Goal, Prerequisites and ownership, Build, Run and observe, Predict and try, Question and answer, Validation). No `???` collapsible exists in any active, journey or archived page; answers are visible.

### 1.4 Build and verification checks that exist

| Check | Command (from `session_viewer`) | Proves | Does not prove |
|---|---|---|---|
| ~~`check_steps.py`~~ | removed `7143d361c` | — | — |
| Materialise | `npm --prefix ../session_tests run course -- checkpoint ID DIR` | checkpoint files | compilation |
| Build + pixels | `course -- verify ID` (`--web-only`, `--native-only`, `--stored`) | locked Cargo native/wasm build, Rust pixel checks | browser behaviour |
| Active audit | `course -- active-audit` | retained implementations, permitted registrations, final 352/352 source parity | teaching |
| Teaching | `course -- teaching` | 360 paragraph witnesses bound to source hashes | that code is shown |
| Historical | `course -- structure`, `ownership [--details]`, `parity [--details]`, `reference-parity`, `verify-reference --stored`, `subject-audit ID|--all`, `destination` | historical fingerprints, rewrites, endpoint parity | current course |
| Tooling tests | `npm --prefix ../session_tests run course:test` (`checkpoints.test.ts`) | tooling | — |
| Site | `DOCS_STRICT_LINKS=1 DOCS_BASE=/docs/ npm run build …` in `session_tests` | includes and links resolve | correctness |
| Browser | `node scripts/course/completion-browser.cjs URL DIR [--camera --rows --geometry --decoding]` (in `session_tests`); `tests/*.cjs` (52 Playwright scripts) | canvas size, resize, dock, subject effects | physical phones |
| Native | `cargo xtest` (alias `test --target x86_64-unknown-linux-gnu`, `.cargo/config.toml:11`); 677 `#[test]`, 61 `#[ignore]`d GPU tests | logic, WGSL validation, GPU parity (ignored ones only by hand) | — |

### 1.5 Stale or broken parts

| # | Finding (FACT) | Evidence |
|---|---|---|
| S1 | Active course absent from the sidebar; nav "Course" = archived lessons 00–37 | `mkdocs.yml:18-61` |
| S2 | 44 of 68 nav paths absent where listed; rescued only by the Vue archive resolver | `test -e` on every nav path; `session_tests/plugins/course-archive.ts` |
| S3 | `ARCHITECTURE.md:3` and nav link `docs/reference-course.md` (archived) | `ls docs/reference-course.md` fails |
| S4 | 65 redirect stubs; all one hop, 0 broken targets, but every target is a historical page (`03d-input.md` ×21, `22a-records.md` ×13, `12-identity.md` ×9, `19a-viewport.md` ×7, `17-solid.md` ×4, `10-depth.md` ×3, others ×8), not an active subject | script in §6 |
| S5 | Cited paths that do not exist: historical journey 826 of 1,143 distinct `src/…` paths (flat learner layout, e.g. `src/aabb.rs`, `src/ambient_gpu.rs`); consolidation 823 of 1,129; archived 48 of 378 (`src/browser.rs` ×117, `src/editor.rs` ×17, `src/document.rs` ×8); active pages 0 of 276 | `test -e` per cited path |
| S6 | Design notes cite files that were never built: `src/engine/gpu/gizmo.rs`, `src/shaders/gizmo.wgsl`, `src/math.rs` (`extend-gumball.md`); `src/app/filter.rs`, `src/app/panel.rs`, `src/engine/gpu/panel.rs` (`extend-panels.md`); `tests/command.cjs` (`extend-command-line.md`). Implemented as `app/gizmo.rs` + `engine/gpu/widget.rs` + `shaders/widget.wgsl`, `app/layers.rs` + `app/ui/layers.rs`, `tests/command-input.cjs` | grep |
| S7 | `file-map.md` maps the archived implementation (links `NN-*.md#code-NN-xxx`); its own "52 absent / 240 differ" (Nov 12) is now 81 absent / 240 differ vs `docs/lessons/37` | `filecmp` src vs `docs/lessons/37/src` |
| S8 | 6 teaching witnesses stale (stage files edited after binding) | SHA-256 vs `completion-teaching.json` |
| S9 | `completion-batch.json` pinned to `87f844e41`, 328 files, no teaching | json |
| S10 | `capstone.md` is a design exercise for a feature that is implemented (clipping, archived 18b) | `capstone.md:3` |
| S11 | `docs/illustrations/stage/*.svg` (29 active-course figures) have no D2 source; the rule requires D2 sources beside rendered SVG | `find docs -name '00-course-map*'` → SVG only |
| S12 | No CLI binary (`Cargo.toml` has no `[[bin]]`, no `src/bin`), no MCP/FastMCP code (`grep -ril mcp src` → no hits) | ARCHITECTURE §10.2 |
| S13 | Repo weight: `docs/` 546 MB, 26,321 tracked files; `docs/lessons` 7,979 tracked files | `git ls-files`, `du` |

---

## 2. Coverage matrix

Columns: **Active page** = owner of the teaching witness in `completion-teaching.json` (`completion-` prefix dropped; every file has one). **Source on page** = whole / part / none, measured from the `--8<--` includes of all 19 active pages. **Archived lesson** = first appearance in `docs/lessons/FILES.txt` (lesson-37 master; `—` = not in the archived course). **Proposed** = tutorial in §3 (`T3.2` = 2D mathematics chapter of T3; `REMOVE` = 3D matrix lane, §4).

### 2.1 Application subtrees and top-level files

| Subsystem | Files | Lines | Active page (witness count) | Source on page whole/part/none | Archived lesson of first appearance | Proposed tutorial(s) |
|---|---:|---:|---|---|---|---|
| `app/command/verbs/` | 86 | 14,996 | modeling 44, commands 37, documents 4, text 1 | 0/1/85 | 23, 23a, 23b, 23c, 23d, 24, 30, 31 … | T6, T7, T9, T13, T10, T12, T15 |
| `app/command/{mod,tests,tool}.rs` + `tool/` | 9 | 3,830 | modeling 5, commands 4 | 0/1/8 | 23, 23a, 23b, 23c, — | T6, T13 |
| `app/stage/` (2D core) | 20 | 9,260 | mathematics 20 | 0/11/9 | — | T3.2 |
| `app/stage/` (scene adapter) | 6 | 1,637 | stage-scene 6 | 0/5/1 | — | T17 |
| `app/walk/` (excl. cloud, sheet, matrix) | 19 | 7,835 | geometry 16, debugging 3 | 0/1/18 | 06, — | T4 |
| `app/walk/{cloud,cloud/*,sheet}.rs` | 4 | 359 | geometry 3, streaming 1 | 0/0/4 | 06, 19, — | T11 |
| `app/walk/matrix.rs` (3D matrix lane) | 1 | 169 | geometry 1 | 0/1/0 | — | REMOVE (3D matrix) |
| `app/scene_sync/` | 12 | 3,313 | documents 6, debugging 6 | 0/2/10 | 20, 21, 23, 26, 31 | T4, T6, T9 |
| `app/ui/` | 10 | 1,983 | input 4, documents 4, commands 1, text 1 | 0/1/9 | 22, 23, 30 | T9, T13 |
| `app/gesture/` | 5 | 122 | input 5 | 0/1/4 | 12, 21, — | T7, T8, T5 |
| `app/inspection.rs` + `inspection/` | 2 | 931 | debugging 2 | 0/0/2 | 12, 16 | T16 |
| `app/scene*.rs` (7 files) | 7 | 5,705 | documents 5, loading 1, text 1 | 0/2/5 | 12, 16, 17, 20, 21, — | T4, T9, T15, T10 |
| `app/{layers,hierarchy,edit,modeling}.rs` | 4 | 4,204 | documents 3, modeling 1 | 0/0/4 | 21, 23, 26 | T6, T9 |
| `app/` I/O: decode, validate, manifest, fetch, loader, route, range_gate, live, live_revision, stream, session_io, yielding | 12 | 6,642 | loading 5, decoding 3, streaming 3, documents 1 | 1/6/5 | 12, 14, 15, 23, — | T15 |
| `app/` shell: mod, launch, runtime, mode, input, touch, keys, agent, feedback, fonts | 10 | 1,936 | input 4, frame 2, debugging 1, text 1, mathematics 1, application 1 | 1/4/5 | 06, 12, 14, 23, — | T5, T1, T13, T16, T10, T3.2 |
| `app/` helpers: gizmo, snap, cplane, coords, deform, splitting, selection, interactions, knobs, clipping, mesh_preview, surface_preview, cloud_query, sheet_query | 14 | 5,796 | modeling 7, geometry 3, streaming 2, commands 1, input 1 | 0/5/9 | 06, 12, 15, 18b, 19, 21, 23, 31 … | T6, T11, T4, T12, T5, T8, T7 |
| `command_dock/` | 3 | 774 | dock 3 | 2/1/0 | — | T13 |
| `camera.rs` | 1 | 866 | camera 1 | 0/1/0 | 02 | T5 |
| `lib.rs`, `state.rs`, `text_quality.rs` | 3 | 1,524 | frame 1, application 1, text 1 | 0/1/2 | 00, 11, 12 | T1, T5, T10 |
| `selftest.rs` + `selftest/` | 5 | 2,769 | debugging 5 | 0/2/3 | 37, — | T16 |
| `engine/{mod,text,performance,diagnostics}.rs`, `engine/text/`, `engine/pipelines/` | 10 | 2,111 | rows 4, debugging 3, frame 1, geometry 1, text 1 | 0/1/9 | 01, 10, — | T3, T10, T2, T1, T16 |

### 2.2 `src/engine/gpu/` (per file; a subdirectory is folded into its parent row)

| File | Lines | Active page | Source on page | Archived lesson | Proposed |
|---|---:|---|---|---|---|
| `engine/gpu/ambient_warm.rs` | 118 | visibility | none | 32 | T10 |
| `engine/gpu/arena.rs` | 687 | geometry | none | 04a | T4 |
| `engine/gpu/backdrop.rs` | 224 | geometry | none | 01 | T3 — plus `backdrop/application_tests.rs` 165 (debugging, none, T3) |
| `engine/gpu/buffers.rs` | 373 | rows | none | 01 | T4 |
| `engine/gpu/clip.rs` | 929 | visibility | none | 18b | T12 — plus `clip/editing_tests.rs` 1146 (debugging, none, T12); `clip/pipelines.rs` 214 (visibility, none, T12); `clip/tests.rs` 425 (debugging, none, T12) |
| `engine/gpu/cloud.rs` | 403 | geometry | none | 04d | T11 |
| `engine/gpu/context.rs` | 664 | geometry | none | — | T2 |
| `engine/gpu/device.rs` | 347 | frame | part | 01 | T2 |
| `engine/gpu/faces.rs` | 405 | geometry | none | 04a | T4 |
| `engine/gpu/frame.rs` | 468 | geometry | none | 01 | T2 |
| `engine/gpu/glyphs.rs` | 362 | geometry | none | 04c | T4 |
| `engine/gpu/hull.rs` | 5 | rows | none | 01 | T4 — plus `hull/geometry.rs` 362 (rows, none, T4); `hull/upload.rs` 35 (geometry, none, T4) |
| `engine/gpu/instance.rs` | 6 | rows | none | 01 | T4 — plus `instance/layout.rs` 84 (rows, part, T4); `instance/validation.rs` 214 (geometry, none, T4) |
| `engine/gpu/instanced.rs` | 100 | geometry | none | 18a | T9 |
| `engine/gpu/lane.rs` | 153 | visibility | part | 01 | T3 |
| `engine/gpu/lod.rs` | 157 | geometry | none | 04d | T11 |
| `engine/gpu/matrices.rs` | 264 | geometry | none | — | REMOVE (3D matrix) |
| `engine/gpu/mod.rs` | 56 | frame | none | 01 | T1 |
| `engine/gpu/objects.rs` | 8 | rows | none | 01 | T4 — plus `objects/gpu_tests.rs` 52 (geometry, none, T4); `objects/ink.rs` 83 (geometry, none, T4); `objects/lane.rs` 17 (geometry, none, T4); `objects/table.rs` 1095 (rows, part, T4) |
| `engine/gpu/pacing.rs` | 34 | geometry | none | — | T2 |
| `engine/gpu/pass.rs` | 153 | geometry | none | 01 | T3 |
| `engine/gpu/patch.rs` | 176 | geometry | none | 03 | T4 |
| `engine/gpu/pick.rs` | 746 | geometry | part | 12 | T7 |
| `engine/gpu/planes.rs` | 288 | geometry | none | — | T4 |
| `engine/gpu/present.rs` | 273 | geometry | none | 01 | T2 |
| `engine/gpu/render.rs` | 486 | geometry | none | 01 | T3 |
| `engine/gpu/segments.rs` | 1,040 | geometry | part | 04b | T4 |
| `engine/gpu/slots.rs` | 958 | geometry | none | 04a | T4 |
| `engine/gpu/splat.rs` | 550 | geometry | none | 04d | T11 — plus `splat/cloud_witness.rs` 139 (debugging, none, T11); `splat/cloud_witness/lod.rs` 85 (debugging, none, T11) |
| `engine/gpu/ssao.rs` | 886 | visibility | none | 32 | T10 — plus `ssao/pipelines.rs` 298 (visibility, none, T10); `ssao/tests.rs` 641 (debugging, none, T10) |
| `engine/gpu/surface_outline.rs` | 916 | visibility | none | 17 | T10 — plus `surface_outline/tests.rs` 646 (debugging, none, T10) |
| `engine/gpu/targets.rs` | 363 | geometry | none | 01 | T2 |
| `engine/gpu/text.rs` | 1,079 | text | none | 11 | T10 |
| `engine/gpu/text_outline.rs` | 269 | text | none | 04a | T10 |
| `engine/gpu/text_plane.rs` | 791 | text | none | 11 | T10 |
| `engine/gpu/text_plate.rs` | 212 | geometry | none | 11 | T10 |
| `engine/gpu/timing.rs` | 127 | geometry | none | 32 | T16 |
| `engine/gpu/triangle_tiles.rs` | 1,187 | visibility | none | 04a | T12 |
| `engine/gpu/ui.rs` | 105 | painting | whole | 22 | T3 |
| `engine/gpu/upload.rs` | 108 | geometry | none | 01 | T4 |
| `engine/gpu/upload_padding.rs` | 74 | geometry | none | 20 | T4 |
| `engine/gpu/vectors.rs` | 977 | geometry | none | 04b | T4 |
| `engine/gpu/view.rs` | 190 | geometry | none | 01 | T2 |
| `engine/gpu/widget.rs` | 415 | geometry | none | 25 | T8 |
| `engine/gpu/widget_mesh.rs` | 139 | geometry | none | 25 | T8 |

### 2.3 `src/state/`

| File | Lines | Active page | Source on page | Archived lesson | Proposed |
|---|---:|---|---|---|---|
| `state/box_select.rs` | 106 | input | none | — | T7 |
| `state/clipping.rs` | 231 | commands | none | 18b | T12 |
| `state/cloud_query.rs` | 262 | input | none | 15 | T11 |
| `state/drag.rs` | 681 | modeling | none | 21 | T8 |
| `state/drawing.rs` | 885 | modeling | none | 23 | T6 |
| `state/edit.rs` | 1,176 | modeling | part | 21 | T6 |
| `state/features.rs` | 63 | application | none | 12 | T5 |
| `state/hydrate.rs` | 159 | loading | part | 21 | T15 |
| `state/number_box.rs` | 93 | documents | none | 21 | T9 |
| `state/panel.rs` | 662 | documents | none | 30 | T9 |
| `state/sheet_query.rs` | 86 | input | none | 19 | T11 |
| `state/splitting.rs` | 193 | modeling | none | 31 | T6 |
| `state/text.rs` | 203 | text | none | 17 | T10 |
| `state/tool.rs` | 486 | modeling | none | 23 | T6 |

### 2.4 WGSL shaders (`src/shaders/`)

| File | Lines | Active page | Source on page | Archived lesson | Proposed |
|---|---:|---|---|---|---|
| `shaders/ambient_composite.wgsl` | 47 | visibility | part | 32 | T10 |
| `shaders/ambient_depth.wgsl` | 35 | visibility | whole | 32 | T10 |
| `shaders/ambient_geometry.wgsl` | 49 | visibility | part | 32 | T10 |
| `shaders/arrow.wgsl` | 427 | geometry | part | — | T4 |
| `shaders/background.wgsl` | 26 | geometry | whole | 01 | T3 |
| `shaders/cap.wgsl` | 218 | visibility | part | 18b | T12 |
| `shaders/clip.wgsl` | 188 | rows | part | 01 | T3 |
| `shaders/face_coverage.wgsl` | 54 | visibility | part | 17 | T10 |
| `shaders/glyph.wgsl` | 179 | geometry | part | 04c | T4 |
| `shaders/grid.wgsl` | 78 | geometry | part | 02 | T3 |
| `shaders/ink_visibility.wgsl` | 428 | visibility | part | 04b | T3 |
| `shaders/matrix.wgsl` | 284 | geometry | part | — | REMOVE (3D matrix) |
| `shaders/normals.wgsl` | 59 | rows | part | 01 | T3 |
| `shaders/physical.wgsl` | 16 | rows | whole | 01 | T3 |
| `shaders/plane.wgsl` | 47 | geometry | part | — | T4 |
| `shaders/project_triangles.wgsl` | 239 | visibility | part | 04a | T12 |
| `shaders/projected_triangle.wgsl` | 52 | rows | part | 04a | T3 |
| `shaders/ribbon.wgsl` | 787 | geometry | part | 04b | T4 |
| `shaders/scan_triangle_tiles.wgsl` | 135 | visibility | part | 04a | T12 |
| `shaders/scene.wgsl` | 135 | rows | part | 01 | T3 |
| `shaders/slot_table.wgsl` | 45 | geometry | whole | 04a | T4 |
| `shaders/sphere.wgsl` | 209 | geometry | part | 04c | T4 |
| `shaders/splat.wgsl` | 259 | geometry | part | 04d | T11 |
| `shaders/splat_resolve.wgsl` | 151 | geometry | part | 04d | T11 |
| `shaders/ssao.wgsl` | 382 | visibility | part | 32 | T10 |
| `shaders/surface_outline.wgsl` | 129 | visibility | part | 17 | T10 |
| `shaders/text_outline.wgsl` | 70 | geometry | part | 04a | T10 |
| `shaders/text_plane.wgsl` | 81 | geometry | part | 11 | T10 |
| `shaders/text_plate.wgsl` | 67 | geometry | part | 11 | T10 |
| `shaders/triangle.wgsl` | 391 | visibility | part | 04a | T4 |
| `shaders/triangle_tiles.wgsl` | 122 | visibility | part | 04a | T12 |
| `shaders/vector.wgsl` | 22 | geometry | whole | 04b | T4 |
| `shaders/widget.wgsl` | 61 | geometry | part | 25 | T8 |

### 2.5 Support files

| File | Witness owner | Proposed |
|---|---|---|
| `Cargo.toml`, `Cargo.lock` | frame | T1 |
| `.cargo/config.toml` (wasm32 default target, `xtest` alias, 4 GiB `--max-memory`) | **none** (not in `completion-teaching.json`) | T1 |
| `Trunk.toml`, `index.html` | application | T1 (page shell), T17 (production `data-*` attributes) |
| `build.rs` (WGSL `#include` expansion into `OUT_DIR`) | rows | T3 |
| `assets/diagnostics.js` | application (9.9) | T16 |
| `assets/text-quality.html` | application (9.6) | T10 |
| `assets/view_local.yaml` | application | T15 |
| `examples/course/*.rs`, `examples/course/index.html`, `index.yaml` | course runtime | each tutorial adds its own `?tutorial=ID` entry |

### 2.6 Source with no lesson

| Level | Files | Lines | Which |
|---|---:|---:|---|
| No teaching witness | 0 | 0 | every `src` file has an owner paragraph |
| No code on any active page | 259 | 78,074 | the `none` entries in §2.1–2.4 |
| No code in the active **or** archived course | 51 | 11,592 | listed below |

The 51 (no code anywhere, not present in `docs/lessons/37/src`):

- `app/`: `interactions.rs`, `live_revision.rs`, `scene_display.rs`
- `app/command/tool/choices.rs`; `app/command/verbs/`: `bounding_box`, `construction_plane`, `grid_snap`, `interactions`, `length_bounding_box`, `report`, `select_all`, `view`
- `app/gesture/box_select.rs`; `state/box_select.rs`
- `app/stage/`: `actions`, `check`, `draw`, `eval`, `geometry`, `layout`, `mod`, `object`, `presentation_tests`, `tests`
- `app/walk/`: `application_tests`, `brep_shapes`, `display`, `display_features`, `brep/application_tests`, `cloud/resident`, `cloud/stream`, `frames/application_tests`
- `engine/diagnostics.rs`; `engine/gpu/`: `context`, `matrices`, `pacing`, `planes`, `backdrop/application_tests`, `hull/{geometry,upload}`, `instance/validation`, `objects/{gpu_tests,ink,lane}`, `splat/cloud_witness`, `splat/cloud_witness/lod`
- `engine/pipelines/{core,geometry}.rs`; `engine/text/application_tests.rs`
- `selftest/{profile,stroke_witness}.rs`

Features with no source at all (nothing to teach yet): native CLI, FastMCP / MCP bridge, typed JSON `Operation` API (ARCHITECTURE §10.2, §11).

---

## 3. Proposed roadmap (PROPOSAL, brief §9 sequence)

### 3.1 Rules every tutorial follows

| Rule | Source of the rule |
|---|---|
| One cumulative program; no toy apps; later tutorials only add files and tagged registration lines | brief §8; `viewer-docs.md` |
| Each file appears once, in final form, as a **whole-file include** `--8<-- "session_viewer/<path>"`; hub files show exactly their `// register:TAG` lines for this tutorial's tags | brief "Do not omit complete source code"; `viewer-docs.md` "source includes, never typed copies" |
| Per file: `### Step k · path` → 2–3 sentences (what it owns, the one trap) → the include. The learner types the file from the include | user preference (typed by hand, 2–3 lines each) |
| One real diagram per tutorial (named below), no per-step locator maps, no repeated course map | user preference; brief "Short architecture diagram" |
| Question and answer stays visible and shows *How to work it out* then *The answer*; later tutorials explain as fully as early ones | user preference (no hidden answers, no fading) |
| Sections: Goal · Diagram · Files · Build (steps) · Integration · Run and observe · Verification. Explanations at most one paragraph per concept, never repeated from an earlier tutorial | brief §8; `viewer-docs.md` lesson format |
| Command forms: native test `bash ../../wood/tools/buildslot.sh --run <agent> timeout 10m cargo xtest -j8 <filter>`; wasm `cargo check --lib -j8`; browser `REGEN_PROTO=0 NO_COLOR=true trunk serve --port 8770` then `node tests/<name>.cjs`; course `npm --prefix ../session_tests run course -- verify <ID>` | `CLAUDE.md` build limits, `tests/README.md`, `maintaining.md` |

Conflicts to resolve before writing (PROPOSAL needs a decision):

| Conflict | Options |
|---|---|
| Diagrams (decided 2026-11-19): **drawn in the session viewer** as stage decks or scenes, deck source kept beside the captured image; no Mermaid (`viewer-docs.md` updated). The 29 active figures are hand SVG/D2 and are redrawn as they are touched (S11) | the names below are deck file names; the type in brackets only describes the layout |
| Brief order puts the command system at T13, but creation verbs (T6) implement `Action` from `app/command/mod.rs`, and `app/modeling.rs:1` imports `command::verbs::geometry::Draw` (ARCHITECTURE §2 cycle) | **chosen here:** T6 owns the registry core (`command/mod.rs`, `verbs/mod.rs`, `tool.rs`); T13 owns the dock, completion, history, phone keyboard and the session verbs |
| `State` (`state.rs`, 69 tag lines) and `app/runtime.rs` (28) are the hubs every later subject registers into; current course introduces them only in the final 199-file step | **chosen here:** introduce them in T5 with only T1–T5 tags; each later tutorial adds its tagged lines. UNVERIFIED that every intermediate tag cut compiles: prove with `course -- subject-audit` / `ownership` per tutorial |
| `projected_triangle.wgsl`, `clip.wgsl`, `ink_visibility.wgsl` are prelude headers every lane includes (`pipelines/core.rs:305` `PRELUDE`; `completion-rows.md` "6 shared WGSL headers") | owned by T3; T12 explains clipping/visibility behaviour of the same text without retyping |
| No CLI and no FastMCP exist | T13 teaches the in-browser command system and marks the native CLI as unbuilt; T14 is blocked (needs IMPLEMENTATION_ROADMAP work, ARCHITECTURE §11 steps 2 and 8) |

### 3.2 Overview

| # | Tutorial | Files | Lines | Replaces / reuses (active → archived) |
|---|---|---:|---:|---|
| T1 | Project architecture and browser initialization | 5 + support | 166 | frame Steps 1–2 → 00 |
| T2 | WebGPU / wgpu initialization | 8 | 2,367 | frame Steps 3–4, geometry (context, targets, present) → 01, 05 |
| T3 | Rendering engine and shaders (+ 3.1 egui painting, 3.2 2D mathematics) | 19 + 21 | 3,202 + 9,287 | painting, mathematics, rows Steps 1–4 → 01, 22 |
| T4 | Geometry and scene data structures | 61 | 21,396 | rows, geometry Steps 1–3, documents (rows/sync) → 03, 04a–04c, 06–09, 17, 20 |
| T5 | Camera and navigation | 9 | 3,818 | camera, input (9.3) → 02, 12 |
| T6 | Object creation and transformations | 79 | 24,121 | commands (9.2), modeling (9.4) → 21, 23, 23a–23d, 24, 31 |
| T7 | Picking and selection | 14 | 2,431 | input (9.3), geometry `pick.rs` → 12, 13 |
| T8 | Gumball | 7 | 2,025 | geometry (`widget`), input, modeling → 25 |
| T9 | Layers and scene management | 22 | 8,715 | documents (9.1) → 18a, 20, 22, 26, 30 |
| T10 | Display modes and shaders (outline, Arctic/GTAO, attributes, text) | 29 | 8,587 | visibility (SSAO, outline), text (9.6), geometry Step 3 → 10, 11, 17, 32, 33, 35 |
| T11 | Point clouds, octrees and sheets | 15 | 3,332 | geometry Step 3 (splat), streaming (9.7) → 04d, 15, 19 |
| T12 | Transparency and advanced rendering (tile visibility, clipping, caps) | 13 | 5,443 | visibility, commands (clipping) → 05, 18, 18b, 36 |
| T13 | CLI and command system | 15 | 2,244 | dock, commands (9.2) → 23, 37 |
| T14 | FastMCP integration | 0 | 0 | none (no source) |
| T15 | File formats and serialization | 16 | 7,374 | decoding, loading (9.5), streaming (9.7), documents (archives) → 14, 15, 16 |
| T16 | Performance optimization | 10 | 4,630 | debugging (9.9) → 16, 37 |
| T17 | Complete application reconstruction (+ stage in the real scene) | 6 | 1,637 | application, stage-scene (9.8) |
| — | REMOVE: 3D matrix lane | 3 | 717 | §4 |
| | **Total** | **352** | **111,492** | |

### 3.3 Tutorials

#### T1 · Project architecture and browser initialization

| Field | Content |
|---|---|
| Goal | A crate that compiles to wasm32, a Trunk page with a full-viewport `#canvas`, and a winit loop started from `run_web`; `?tutorial=frame` loads with no console error. |
| Diagram | `t01-startup-sequence` (sequenceDiagram): browser → `index.html` → Trunk JS bridge → wasm start → `app::launch::spawn_app` → `EventLoopProxy<Message>` cloned before the async GPU handshake. Existing figure to retire into it: `illustrations/toolchain.svg`. |
| Files (build order) | `Cargo.toml`, `Cargo.lock` (`cargo generate-lockfile`), `.cargo/config.toml`, `Trunk.toml`, `index.html`, `src/lib.rs`, `src/app/mod.rs`, `src/engine/mod.rs`, `src/engine/gpu/mod.rs`, `src/app/launch.rs`, `examples/course/index.html`, `examples/course/canvas.rs`, `examples/course/registration.rs`, `examples/course/entry.rs` |
| Complete source | whole files; reuse `completion-frame.md` Steps 1–2 (`Cargo.toml:1:26`, `lib.rs:1:12` → widen to whole file, `launch.rs` whole, `index.html:course-page`, `canvas.rs:course-canvas`); `archived/00-environment.md`, `docs/lessons/00/` |
| Integration | `lib.rs` starts with `shader!`, `mod engine`, `pub mod app` and the `tutorial` gate; later tutorials add one line each (`camera`, `state` T5; `text_quality` T10; `command_dock` T13; `selftest` T16). `lib.rs` has no `register:` tags. `registration.rs` holds the `(id, start)` table every later tutorial appends to. |
| Verification | `cargo check --lib -j8` (wasm32 default); `trunk serve`; `#canvas` has no `data-course-error`; canvas width = `clientWidth × devicePixelRatio`. |

#### T2 · WebGPU / wgpu initialization

| Field | Content |
|---|---|
| Goal | Adapter, device (256 MiB storage cap, first-error capture), surface, MSAA/depth targets and a white clear that survives resize to 1280×800 and 390×844. |
| Diagram | `t02-device-surface-frame` (flowchart): `request_adapter` → `request_device(limits)` → `Surface::configure` → `Targets` (MSAA colour, reversed depth, pick IDs) → `Frame` acquire → clear pass → `queue.submit` → `present`; side branch: uncaptured error → `Arc<Mutex<Option<String>>>`. Reuse `illustrations/stage/01-surface-frame.svg`, `02-logical-vs-ndc.svg`. |
| Files | `src/engine/gpu/device.rs`, `src/engine/diagnostics.rs`, `src/engine/gpu/view.rs`, `src/engine/gpu/targets.rs`, `src/engine/gpu/present.rs`, `src/engine/gpu/frame.rs`, `src/engine/gpu/pacing.rs`, `src/engine/gpu/context.rs` (tags `frame`, `shell`, `device`, `present`, `targets`, `view` only), `examples/course/frame.rs` |
| Complete source | `completion-frame.md` Step 3 (`device.rs:20:154` → whole), Step 4 (`frame.rs:course-frame`); `targets.rs`/`present.rs`/`context.rs` are witness-only today (geometry page) → add whole includes; `archived/01-first-frame.md`, `05-visibility.md` (targets) |
| Integration | `engine/gpu/mod.rs` gains the module lines; `Gpu` (`context.rs:25`) is created once and owned by the app; `FramePacing` parks the loop when `frame()` returns `false`. |
| Verification | `cargo xtest first_gpu_error_survives_follow_on_submission_errors`, `phones_start_solid_and_desktops_keep_nearly_solid_faces`; `session_tests/scripts/course/first-frame-browser.cjs`; `tests/device_loss.cjs`; `course -- verify frame`. |

#### T3 · Rendering engine and shaders

| Field | Content |
|---|---|
| Goal | The frame encoder, pass list, lane registry and pipeline cache, with the WGSL prelude compiled through `build.rs`; draws background and grid. Chapters: **3.1** egui painter composited into the frame; **3.2** the complete 2D mathematics stage (TeX layout, OpenType MATH atlas, rounded-rectangle matrices, motion, deck YAML). |
| Diagram | `t03-frame-passes` (flowchart): `build.rs` `#include` expansion → `OUT_DIR` → `shader!` → `pipelines::core` lazy cache (`PRELUDE` = `scene` + `clip`) → `render.rs` pass order → lanes (`lane_list!`, `lane::REGISTRY`, `pass::PASSES`) → egui pass. 3.2: `t03b-stage-layout` (flowchart: deck YAML → `deck.rs` → `tex.rs` boxes → `layout.rs` 1600×900 frame → `paint.rs` egui glyphs; `matrix.rs` rounded cells). Reuse `illustrations/stage/03-opentype-math-atlas.svg`, `03-stage-frame-1600.svg`, `03-stage-palette-flat.svg`, `06-instance-112-headers.svg` (headers part). |
| Files | `build.rs`, `src/engine/pipelines/{mod,bindings,layouts,core,geometry}.rs`, `src/shaders/scene.wgsl`, `clip.wgsl`, `physical.wgsl`, `normals.wgsl`, `projected_triangle.wgsl`, `ink_visibility.wgsl`, `src/engine/gpu/lane.rs`, `pass.rs`, `render.rs`, `backdrop.rs`, `backdrop/application_tests.rs`, `src/shaders/background.wgsl`, `grid.wgsl`; 3.1: `src/engine/gpu/ui.rs`, `examples/course/painting.rs`; 3.2: `src/app/mode.rs`, `src/app/stage/{mod,font,tex,layout,flat,matrix,draw,paint,object,eval,actions,morph,tween,run,presentation,math_mode,deck,check}.rs`, `stage/{tests,presentation_tests}.rs`, `examples/course/mathematics.rs` |
| Complete source | `completion-rows.md` Steps 1–4 (headers, build script), `completion-painting.md` (`ui.rs` whole), `completion-mathematics.md` (12 files partial → whole); `pipelines/core.rs`, `geometry.rs`, `render.rs`, `pass.rs` are witness-only → add; `archived/01-first-frame.md`, `22-runtime-helpers.md` |
| Integration | `context.rs` tags `egui`, `backdrop`; `render.rs` tags `camera`, `egui`, `shell`; `app/mod.rs` `register:mode`; stage enters through `Mode` (`app/mode.rs`). |
| Verification | `cargo xtest lazy_objects_are_made_once_on_first_use`, `only_stroke_picks_read_the_tables`, `fraction_rule_sits_on_the_math_axis`, `pictures_keep_their_order_and_geometry_without_scene_rows`; `course -- verify painting`, `course -- verify mathematics`; `scripts/course/stage-core.test.ts`. |

#### T4 · Geometry and scene data structures

| Field | Content |
|---|---|
| Goal | Kernel meshes, BReps, curves (ribbons, arrows), points, planes become typed `Upload` rows in shared arenas, slots and 112-byte `Instance` rows; `Scene` holds `(doc, GUID)` rows and syncs them incrementally. Visible: mesh cube, BRep cube, arrowed NURBS curve (no Matrix, §4). |
| Diagram | `t04-source-to-rows` (flowchart): `session_rust` object → `walk::{mesh,brep,curves,plane,points}` → `Upload` → `arena` / `slots` / `segments` / `vectors` / `glyphs` / `planes` → `objects::table` (`Instance` 112 B) → lane draw; `Scene` → `scene_sync::{allocation,compaction,tombs,notes}`. Reuse `06-instance-112-headers.svg`, `07a-mesh-brep-topology.svg`, `07b-ribbon-arrow-lanes.svg`, `09a-identity-footprint-tombs.svg`; redraw `07c-glyphs-splats-matrix-widget.svg` without the matrix cell panel. |
| Files | 4.1 rows: `src/engine/gpu/{buffers,upload,upload_padding,instance,objects,hull,patch}.rs`, `instance/{layout,validation}.rs`, `objects/{table,lane,ink,gpu_tests}.rs`, `hull/{geometry,upload}.rs`. 4.2 storage: `arena.rs`, `slots.rs`, `src/shaders/slot_table.wgsl`. 4.3 lanes: `faces.rs` + `triangle.wgsl`; `segments.rs` + `ribbon.wgsl`, `arrow.wgsl`; `vectors.rs` + `vector.wgsl`; `glyphs.rs` + `glyph.wgsl`, `sphere.wgsl`; `planes.rs` + `plane.wgsl`. 4.4 walk: `src/app/walk/{mod,encode,bounds,mesh,mesh_topology,mesh_ink,brep,brep_orient,brep_shapes,brep_edges,curves,plane,points,frames,display,display_features}.rs` + `walk/{application_tests,brep/application_tests,frames/application_tests}.rs`. 4.5 scene: `src/app/{scene,scene_rows,scene_sync,knobs,interactions}.rs`, `src/app/scene_sync/{allocation,compaction,nodes,notes,tombs,testing,tests}.rs`; `examples/course/rows.rs`, `examples/course/geometry.rs` |
| Complete source | `completion-rows.md`, `completion-geometry.md` Steps 1–3, `completion-documents.md` (rows, tombs); 46 of these 61 files are witness-only → add whole includes; `archived/03-identity.md`, `04a-meshes.md`, `04b-strokes.md`, `04c-markers.md`, `06`–`09`, `17-source-presentation.md`, `20-history.md` |
| Integration | tags `meshes`, `strokes`, `markers`, `planes`, `arena`, `slots`, `segments`, `vectors`, `glyphs`, `faces`, `ink`, `patch`, `hull`, `upload`, `padding`, `document`, `walk`, `knobs`, `interactions` in `gpu/mod.rs`, `context.rs`, `render.rs`, `scene.rs`, `app/mod.rs`. |
| Verification | `cargo xtest packed_vertex_is_twenty_bytes`, `octahedral_normals_round_trip`, `edits_write_only_their_own_slots`, `instance_mirror`, `world_box_translates`, `cylinder_segment_mirror`, `teapot_boundary_ink_follows_authored_patch_curves`, `polyline_heads_aim_along_the_end_segments`, `duplicate_guids_keep_their_document_and_control_owners`; `tests/teapot.cjs`; `course -- verify rows`, `course -- verify geometry`. |

#### T5 · Camera and navigation

| Field | Content |
|---|---|
| Goal | Reverse-Z perspective/orthographic camera in metres over a millimetre scene, orbit/pan/zoom from mouse, wheel, touch and keys, bounds fit; `State` and `runtime.rs` appear here as the hubs later tutorials register into. |
| Diagram | `t05-camera-spaces` (flowchart): pointer/wheel/touch → `input.rs`/`touch.rs`/`keys.rs` → `gesture::mod` dispatch → `Camera` pose → view-projection (reverse-Z) → `view.rs` uniform; `coords.rs` ray unprojection; bounds fit. Reuse `05-camera-frustum-spaces.svg`, `09-state-event-loop.svg`. |
| Files | `src/camera.rs`, `src/app/coords.rs`, `src/state.rs` (T1–T5 tags), `src/state/features.rs` (empty hook tables), `src/app/runtime.rs` (`frame`, `shell`, `egui` tags), `src/app/input.rs`, `src/app/touch.rs`, `src/app/keys.rs`, `src/app/gesture/mod.rs`, `examples/course/camera.rs` (keys: arrows orbit, `P` projection, `F` fit, as in `examples/course/geometry.rs`; the `Fit`/`View` verbs arrive in T13) |
| Complete source | `completion-camera.md` (`camera.rs` 103 of 866 lines → whole), `completion-input.md` (9.3); `state.rs`, `runtime.rs` witness-only → tag-filtered whole files; `archived/02-camera.md`, `12-picking.md` (input) |
| Integration | `State` owns window, `Gpu`, `Camera`, `Scene`; `App` (`runtime.rs:61`) routes `Msg` (`runtime.rs:24`); tags `camera`, `input`, `touch`, `keys`, `gesture`, `coords`. |
| Verification | `cargo xtest pose_tracks_navigation_without_tracking_scene_extent`, `near_plane_cuts_a_millimetre_ahead_not_a_beam`, `distant_orthographic_depth_distinguishes_four_millimetres`, `a_quick_wobble_is_a_click`; `tests/wheel-navigation.cjs`, `tests/navigation-quality.cjs`; `course -- verify camera`. |

#### T6 · Object creation and transformations

| Field | Content |
|---|---|
| Goal | Create kernel objects from typed or clicked input on a construction plane with snaps, transform them with undoable transactions, and edit them (surfacing, trim, extend, split, explode, measure, annotate). |
| Diagram | `t06-command-to-transaction` (flowchart): `command::parse` → `Spec`/`Action` (`REGISTRY`) → `Tool` gathers points (`cplane`, `snap`) → preview (`mesh_preview`, `surface_preview`, `scene_sync::preview`) → `modeling` / `deform` → `edit` transaction (undo/redo) → `scene_sync` rows. Reuse `09b-command-spec-clipping.svg` (command part), `09d-cplane-snaps-recipes.svg`. |
| Files | 6.1 registry core: `src/app/command/mod.rs`, `command/verbs/mod.rs`, `command/verbs/geometry.rs`, `command/tool.rs`, `tool/{options,choices,gather}.rs`, `src/state/tool.rs`. 6.2 placement: `src/app/cplane.rs`, `snap.rs`, `verbs/{construction_plane,grid_snap,snap}.rs`. 6.3 transactions: `src/app/edit.rs`, `src/state/edit.rs`, `src/app/modeling.rs`. 6.4 primitives: `tool/shape.rs`, `src/state/drawing.rs`, `src/app/mesh_preview.rs`, `scene_sync/preview.rs`, verbs `point line polyline curve arrow box sphere cone cylinder capsule torus pyramid quad_sphere tetrahedron octahedron dodecahedron icosahedron block_with_hole bounding_box length_bounding_box nurbs_curve_{arc,circle,ellipse,parabola}`. 6.5 transforms: `src/app/deform.rs`, verbs `move rotate scale copy orient_3_points project_to_plane`. 6.6 surfacing: `tool/surfacing.rs`, `src/app/surface_preview.rs`, verbs `extrude loft nurbs_surface_{4_points,loft,network,revolve,sweep1,sweep2}`. 6.7 cutting: `tool/cut.rs`, `src/app/splitting.rs`, `src/state/splitting.rs`, verbs `trim` (+`trim/parts.rs`), `extend` (+`extend/reach.rs`), `split`, `explode`. 6.8 measure/annotate: verbs `area length measure measure_distance volume arrowhead`. Tests: `scene_sync/{commands_tests,editing_tests,split_tests}.rs`. |
| Complete source | `completion-modeling.md` (62 files, 37 lines shown → whole includes), `completion-commands.md`; `archived/21-editing.md`, `23-geometry-commands.md`, `23a-tools.md`, `23b-shapes.md`, `23c-surfacing.md`, `23d-annotate-measure.md`, `24-placed-controls.md`, `31-splitting.md`; `docs/lessons/23*/`, `31/` |
| Integration | `verbs!` macro rows in `verbs/mod.rs` (one tag per verb); `state.rs`/`features.rs` tags `tools`, `tool`, `drawing`, `edit`, `editing`, `snap`, `cplane`, `split`, `splitting`, `measure`, `annotate`; `runtime.rs` `commands`, `editing`. Until T13 the verbs run from `examples/course` buttons/keys and native tests. |
| Verification | `cargo xtest creation_undo_redo_and_invalid_input`, `create_many_adds_a_surface_and_a_brep_in_one_undo_step`, `group_transform_undo_restores_every_member_in_one_document`, `an_open_polyline_offers_ends_vertices_and_midpoints`, `view_mode_draws_on_the_ground_unless_square_on`, `split_face_keeps_solid_joined_and_invalid_cut_preserves_source`, `trim_parses_both_forms`; `tests/transforms.cjs`, `tests/trim-extend.cjs`, `tests/splitting.cjs`, `tests/editing-extensions.cjs`. |

#### T7 · Picking and selection

| Field | Content |
|---|---|
| Goal | GPU ID pass reads `(object, subobject)` under the cursor, generation-stamped; click, box and lasso select objects, faces, edges and control points. |
| Diagram | `t07-pick-roundtrip` (sequenceDiagram): pointer → `gesture` priority (control → gizmo → object → box) → `pick.rs` window read-back (async, generation stamp) → `selection.rs` → `State` selection → highlight rows. Reuse `09c-pick-window-gestures.svg`. |
| Files | `src/engine/gpu/pick.rs`, `src/app/selection.rs`, `src/app/gesture/{object,box_select}.rs`, `src/state/box_select.rs`, verbs `selecting select_all select_by_name select_lasso select_small object edge face controls` |
| Complete source | `completion-input.md` (9.3), `completion-geometry.md` (`pick.rs` 23 lines → whole); `archived/12-picking.md`, `13-controls.md` |
| Integration | tags `pick`, `selection`, `box_select`, `object`, `object_drag`, `selecting`, `select_*`, `controls`; `features::{BEFORE_PICKS, AFTER_PICKS, TAKE_PICK, CLICK_ROWS}`. |
| Verification | `cargo xtest window_clamps`, `ink_first_then_nearest`, `switching_parent_replaces_specialized_selection_and_escape_retains_parent`, `the_selection_keeps_pick_order`; `tests/selection.cjs`, `tests/selection-overlap.py`. |

#### T8 · Gumball

| Field | Content |
|---|---|
| Goal | A private depth-tested gumball tile with move/rotate/scale handles; dragging a handle previews and commits one transaction. |
| Diagram | `t08-gumball-drag` (stateDiagram): idle → hover handle (`gizmo` ray test, screen aperture) → drag (`state/drag.rs` preview) → release (`edit` transaction) / escape (cancel). |
| Files | `src/app/gizmo.rs`, `src/engine/gpu/widget_mesh.rs`, `src/engine/gpu/widget.rs`, `src/shaders/widget.wgsl`, `src/app/gesture/{gizmo,control}.rs`, `src/state/drag.rs` |
| Complete source | `archived/25-gumball.md` (typed chunks `typing/code/25-*`; `widget.rs` chunk 412 lines vs 415 now) → replace with whole includes; `completion-geometry.md` (`widget` witness), `completion-modeling.md`; `docs/extend-gumball.md` design notes (fix S6) |
| Integration | tags `gumball` (16), `gizmo`, `gizmo_drag`, `control`, `control_drag`, `drag`, `widget`, `widget_mesh`. |
| Verification | `cargo xtest arm_grab_accepts_a_ray_within_the_screen_aperture`, `the_handles_do_not_shadow_each_other`, `an_axis_seen_end_on_is_not_grabbable`; `tests/interaction.cjs`, `tests/large-object-dragging.cjs`. |

#### T9 · Layers and scene management

| Field | Content |
|---|---|
| Goal | Document/layer tree, element graph, visibility and lock lamps, number box, instancing, hide/show/delete, layer and group verbs, all in egui panels. |
| Diagram | `t09-layer-tree` (flowchart): `Scene` documents → `hierarchy.rs` tree/graph index → `layers.rs` buckets → `ui/layers.rs`, `ui/graph.rs` panels → lamp/lock edits → `scene_display.rs` flags → rows; `scene_instances.rs` definition → instances. |
| Files | `src/app/hierarchy.rs`, `src/app/layers.rs`, `src/app/scene_display.rs`, `src/app/scene_instances.rs`, `src/engine/gpu/instanced.rs`, `src/app/ui/{mod,theme,pointer,overlay,layers,graph,number_box}.rs`, `src/state/{panel,number_box}.rs`, verbs `layers add_group add_edge hide show delete interactions`, `scene_sync/panel_tests.rs` |
| Complete source | `completion-documents.md` (9.1; 25 files, 101 lines shown); `archived/18a-instancing.md`, `20-history.md`, `22-runtime-helpers.md`, `26-nested-panel.md`, `30-layer-tree.md`; `docs/extend-panels.md` (fix S6) |
| Integration | tags `layers`, `hierarchy`, `panel` (21), `number_box`, `graph`, `overlay`, `pointer`, `theme`, `ui`, `instancing` (19), `instanced`, `hide`, `show`, `delete`. |
| Verification | `cargo xtest the_panel_lists_documents_then_the_kinds_present`, `a_layer_names_the_rows_it_controls`, `a_layer_lamp_keeps_the_lamps_below`, `a_definition_is_walked_once_for_every_instance`; `instanced_frames_match_the_baked_scene` (GPU, `#[ignore]`, currently the one failing parity test); `tests/layer-workspace.cjs`, `tests/instancing.cjs`. |

#### T10 · Display modes and shaders

| Field | Content |
|---|---|
| Goal | Every display mode on one scene: shaded faces, silhouette/surface outline, Arctic (GTAO/SSAO composite), Outline, Attributes On/Off, scene text (world-plane, plate, outlined). |
| Diagram | `t10-display-passes` (flowchart): geometry pass → `face_coverage` → `surface_outline` → `ambient_depth`/`ambient_geometry` → `ssao` → `ambient_composite` → text lanes (`text_plane`, `text_plate`, `text_outline`) → present; mode switches from `arctic`/`outline`/`attributes` verbs and `view.rs` settings. Reuse `07e-caps-outlines-ssao.svg`, `09f-scene-text-placements.svg`. |
| Files | 10.1 outline: `src/engine/gpu/surface_outline.rs`, `surface_outline/tests.rs`, `src/shaders/face_coverage.wgsl`, `surface_outline.wgsl`. 10.2 ambient/Arctic: `src/engine/gpu/ssao.rs`, `ssao/{pipelines,tests}.rs`, `ambient_warm.rs`, `src/shaders/{ambient_depth,ambient_geometry,ssao,ambient_composite}.wgsl`, verbs `arctic outline attributes`. 10.3 text: `src/app/fonts.rs`, `src/engine/text.rs`, `engine/text/application_tests.rs`, `src/engine/gpu/{text,text_plane,text_plate,text_outline}.rs`, `src/shaders/{text_plane,text_plate,text_outline}.wgsl`, `src/app/scene_text.rs`, `src/state/text.rs`, verb `text`, `src/text_quality.rs`, `assets/text-quality.html` |
| Complete source | `completion-visibility.md` (SSAO/outline, 13 files partial), `completion-text.md` (9.6), `completion-commands.md` (Arctic/Outline verbs); `archived/10-text-layout.md`, `11-text-rendering.md`, `17-source-presentation.md`, `32-colors-lighting.md`, `33-contact-shadows.md`, `35-attributes.md`; `docs/ssao.md` |
| Integration | tags `gtao` (13), `ambient`, `ssao`, `surface_outline`, `outline`, `arctic`, `attributes`, `text` (9), `scene_text` (22), `text_outline`, `fonts`. |
| Verification | `cargo xtest rotation_reprojects_ground_shadows_without_erasing_them`, `resolution_follows_device_pixels_without_a_navigation_mode`, `curved_and_flat_outlines_keep_the_same_width_when_edges_are_shown`, `face_coverage_shader_validates`, `ligatures_accents_spaces_and_symbols_are_shaped`, `placement_and_color_do_not_reshape_but_font_reload_does`; `benchmark_arctic` (`#[ignore]`, exclusive slot); `tests/ambient-scenes.cjs`, `ambient-lighting.cjs`, `color-channels.cjs`, `world-text.cjs`, `text-quality.cjs`. |

#### T11 · Point clouds, octrees and sheets

| Field | Content |
|---|---|
| Goal | Point clouds with a precomputed octree (`LodNode`), EDL splats and resolve pass, resident vs streamed pages, CPU cloud picks; sheets (batched drawings with lazy metadata). |
| Diagram | `t11-cloud-lod` (flowchart): validated cloud arrays → `walk/cloud/resident` or `walk/cloud/stream` (HTTP Range pages) → `engine/gpu/cloud.rs` (`node_first`, `node_count`) → `lod.rs` selection → `splat.wgsl` → `splat_resolve.wgsl`; `cloud_query` CPU pick. Reuse `09g-out-of-core-streaming.svg`. |
| Files | `src/engine/gpu/cloud.rs`, `lod.rs`, `splat.rs`, `src/shaders/splat.wgsl`, `splat_resolve.wgsl`, `src/app/walk/cloud.rs`, `walk/cloud/{resident,stream}.rs`, `src/app/cloud_query.rs`, `src/state/cloud_query.rs`, `src/app/walk/sheet.rs`, `src/app/sheet_query.rs`, `src/state/sheet_query.rs`, `src/engine/gpu/splat/cloud_witness.rs`, `splat/cloud_witness/lod.rs` |
| Complete source | `completion-geometry.md` Step 3 (splat), `completion-streaming.md` (9.7); `archived/04d-clouds.md`, `15-publication.md`, `19-sheets.md` |
| Integration | tags `clouds` (26), `cloud`, `lod`, `splat`, `cloud_query` (14), `sheets` (17), `sheet_query`. Stream wiring (`stream.rs`) arrives in T15; T11 runs resident clouds only. |
| Verification | `cargo xtest lod_parallel_arrays_bounds_ranges_and_child_graph_are_validated`, `missing_indices_and_partial_cloud_attributes_are_errors`, `sheet_chunks_map_global_ribbon_rows_to_segments_and_ids`, `a_deleted_cloud_waits_in_its_tomb`; `tests/streamed-controls.cjs`, `tests/sheet-metadata.cjs`, `tests/phone-load.cjs`. No native test exists for `splat.rs`, `lod.rs`, `cloud.rs` (0 `#[test]`). |

#### T12 · Transparency and advanced rendering

| Field | Content |
|---|---|
| Goal | Translucent faces with Opacity, hidden-line visibility through screen-space triangle tiles, clipping planes with section caps. |
| Diagram | `t12-tile-visibility` (flowchart): `project_triangles` (compute) → `triangle_tiles` prefix → `scan_triangle_tiles` → `ink_visibility` per stroke sample; clip uniform (`clip.wgsl`) → discard → `cap.wgsl` section fill; `faces.rs` opacity → blended pass order. Reuse `07d-tile-prefix-visibility.svg`, `07e-caps-outlines-ssao.svg`, `09b-command-spec-clipping.svg` (clipping part). |
| Files | `src/engine/gpu/triangle_tiles.rs`, `src/shaders/{project_triangles,triangle_tiles,scan_triangle_tiles}.wgsl`, `src/engine/gpu/clip.rs`, `clip/{pipelines,tests,editing_tests}.rs`, `src/shaders/cap.wgsl`, `src/app/clipping.rs`, `src/state/clipping.rs`, verbs `clipping_plane opacity` |
| Complete source | `completion-visibility.md`, `completion-commands.md` (clipping, 24 lines of `clipping.rs`); translucency is the `faces.rs`/`triangle.wgsl` text already shown whole in T4 — T12 explains it, no retyping; `archived/05-visibility.md`, `18-finite-visibility.md`, `18b-clipping.md`, `36-translucent-faces.md`; `docs/capstone.md` (design) |
| Integration | tags `tiles` (25), `triangle_tiles`, `clip`, `clipping`, `clipping_plane`, `opacity`. |
| Verification | `cargo xtest tile_shaders_validate`, `projection_dispatch_stays_in_limits`, `projected_grows_by_capacity`, `clip_uniform_mirror`, `a_plane_crosses_the_boxes_it_passes_through`, `every_stage_stays_within_its_bindings`; `tests/clipping-mixed.cjs`, `tests/triangle-visibility.py`, `tests/visibility-idle.cjs`. No test targets translucency/Opacity (grep for `opac|transluc|transparen` in test names: none). |

#### T13 · CLI and command system

| Field | Content |
|---|---|
| Goal | The persistent command dock: typing, completion, history, option chips, aliases, focus ownership, phone keyboard (IME agent); session verbs (undo, redo, escape, close, report, fit, view). Native CLI: not built (state it, link IMPLEMENTATION_ROADMAP). |
| Diagram | `t13-command-line` (sequenceDiagram): key/IME → `agent.rs` → `ui/command_line.rs` → `command_dock::CommandLine` (completion, history) → `command::parse` → `Action::run(&mut State)` → status/feedback. Reuse `04-command-dock.svg`. |
| Files | `src/command_dock/{mod,theme,view}.rs`, `src/app/ui/command_line.rs`, `ui/command_line/tests.rs`, `src/app/ui/phone.rs`, `src/app/agent.rs`, `src/app/command/tests.rs`, verbs `undo redo escape close report fit view`, `examples/course/dock.rs` |
| Complete source | `completion-dock.md` (`theme.rs`, `view.rs` whole; `mod.rs` 30 of 663 lines → whole), `completion-commands.md`; `archived/23-geometry-commands.md`, `37-command-dock.md`; `docs/command-line-walkthrough.md`, `docs/extend-command-line.md` (fix S6) |
| Integration | tags `command`, `command_line`, `commands` (24), `phone` (15), `agent`, `undo`, `redo`, `escape`, `close`, `fit`, `view`; `Commands` trait keeps the dock free of `crate::` imports (ARCHITECTURE §1). |
| Verification | `cargo xtest every_verb_is_offered_in_alphabetical_order`, `every_name_and_option_is_title_case_words`, `the_verbs_and_their_short_forms`; `session_tests/scripts/course/dock-regression.rs`; `tests/command-dock.cjs`, `command-input.cjs`, `keyboard-shortcuts.cjs`; `course -- verify dock`. |

#### T14 · FastMCP integration

| Field | Content |
|---|---|
| Goal | BLOCKED: no MCP, FastMCP, `Operation` schema or `execute(json)` entry exists in `src/`, `index.html` or `docs/`. |
| Diagram | `t14-automation-bridge` (flowchart, target design from ARCHITECTURE §11): FastMCP server (Python) → stdio/WebSocket → `viewer-automation` `Operation` → `CommandHost` → same verbs as T6/T13. |
| Files | none today; prerequisite work: `CommandHost` trait replacing `&mut State` in `Action::run` (ARCHITECTURE §11 step 2), operation registry from `command::Spec` (step 8). |
| Complete source | none to reuse |
| Integration | after implementation: one new crate/module plus a wasm-bindgen `execute(operation_json)` in the web shell |
| Verification | to be defined with the implementation (headless host running every verb) |

#### T15 · File formats and serialization

| Field | Content |
|---|---|
| Goal | Manifests (YAML/JSON/TOML), bounded protobuf validation and decode (≤ 512 MiB, 25,000-object yields), save/open session archives (protobuf + JSON metadata), fetch, early hydration, HTTP Range streaming, ETag/EventSource live updates, source release. |
| Diagram | `t15-load-pipeline` (flowchart): URL `route` → `manifest` → `fetch` (`range_gate`) → `validate` → `decode` (`yielding`) → `loader` generation guard → `Scene` publish → `scene_release`; `session_io::{save,open}` round trip; `live`/`live_revision`. Reuse `08-protobuf-bounds-yield.svg`, `09e-early-fetch-hydration.svg`, `09g-out-of-core-streaming.svg`. |
| Files | `src/app/yielding.rs`, `validate.rs`, `decode.rs`, `manifest.rs`, `route.rs`, `fetch.rs`, `range_gate.rs`, `loader.rs`, `src/state/hydrate.rs`, `src/app/stream.rs`, `live.rs`, `live_revision.rs`, `scene_release.rs`, `session_io.rs`, verbs `open save`, `assets/view_local.yaml`, `examples/course/decoding.rs` |
| Complete source | `completion-decoding.md`, `completion-loading.md` (9.5), `completion-streaming.md` (9.7), `completion-documents.md` (archives); `archived/14-loading.md`, `15-publication.md`, `16-accounting.md` |
| Integration | tags `decode`, `validate`, `manifest`, `route`, `fetch`, `range_gate`, `loader`, `loading`, `hydrate`, `stream` (19), `live`, `release` (14), `session_io`, `open`, `save`. |
| Verification | `cargo xtest window_decode_matches_the_source`, `hostile_counts_do_not_reach_kernel_allocations`, `yaml_json_and_toml_have_identical_scene_semantics`, `a_created_text_survives_save_and_open`, `edited_documents_placements_hidden_state_and_history_survive_save`, `incomplete_or_foreign_files_are_rejected`, `metadata_window_checks_ranges_without_copying_or_following_large_skips`; `tests/loading.cjs`, `hydration.cjs`, `compressed-scene.cjs`, `live-revision.cjs`, `live-notification.cjs`; `course -- verify decoding`. |

#### T16 · Performance optimization

| Field | Content |
|---|---|
| Goal | Measure before changing: navigation tiers and wheel bursts, pass timers, `?perf`, `?inspect=1` memory/row snapshots, diagnostics overlay, headless self-test oracles and stroke witnesses; then apply the PERFORMANCE_AUDIT §6 changes inside the owning tutorial. |
| Diagram | `t16-measurement-loop` (flowchart): `State::render` → `performance.rs` tiers → `timing.rs` PassTimer (native) → `inspection.rs` publish → `diagnostics.js` / `feedback.rs`; `selftest` → `oracle` / `lifecycle` / `profile` / `stroke_witness` → PPM + JSON. Reuse `09i-four-diagnostic-oracles.svg`. |
| Files | `src/engine/performance.rs`, `src/engine/gpu/timing.rs`, `src/app/inspection.rs`, `inspection/source_memory.rs`, `src/app/feedback.rs`, `assets/diagnostics.js`, `src/selftest.rs`, `selftest/{oracle,lifecycle,profile,stroke_witness}.rs` |
| Complete source | `completion-debugging.md` (9.9; 27 files, 38 lines shown); `archived/16-accounting.md`, `37-command-dock.md` (self-test); `docs/performance-patterns.md`, `docs/phone-performance.md`, `inspection/PERFORMANCE_AUDIT.md` §5.1 baseline procedure |
| Integration | tags `inspection`, `feedback`, `timing`; `selftest` is `cfg(not(wasm32))` (`lib.rs`). |
| Verification | `cargo xtest wheel_bursts_wait_for_a_frame_then_expire_and_cancel`, `slow_drags_step_through_the_tiers`, `cheaper_frames_do_not_immediately_retry_expensive_navigation`, `close_up_edges_preserve_visibility_across_views_and_samples`; `tests/navigation-pacing.cjs`, `lifecycle.cjs`, `diagnostic_lifecycle.cjs`, `diagnostics.test.cjs`; baseline numbers recorded per PERFORMANCE_AUDIT §5.1 before and after each change. |

#### T17 · Complete application reconstruction

| Field | Content |
|---|---|
| Goal | Production shell and the stage in the real scene (`Scene3` row sync, camera label lens, cable-net decks); prove the learner tree equals production. |
| Diagram | `t17-assembled-viewer` (flowchart): T1–T16 modules as boxes on `State` / `Gpu` / `Scene` / `App`; `stage::{application,spatial,mode,geometry,net,view}` bridging `Mode` to scene rows. Reuse `09h-stage-scene-lens.svg`, `00-course-map.svg` (update counts). |
| Files | `src/app/stage/{geometry,net,view,spatial,mode,application}.rs`; `Trunk.toml`, `index.html` production attributes (`data-wasm-opt="0"`, `data-target-name`) |
| Complete source | `stage-scene.md` (9.8, 509 lines shown), `completion-application.md` |
| Integration | tags `application` (25), `mode`, `features`; no new hub. |
| Verification | `course -- active-audit` (352/352 parity, 0 missing/extra/rewrites), `course -- parity --details`, `course -- teaching`, `course -- verify application`; stage `presentation_tests.rs`; full `tests/*.cjs` sweep through `buildslot --run`; open item: `instanced_frames_match_the_baked_scene` (31 px differ by 3 levels vs ≤ 2). |

### 3.4 Feature → tutorial map

| Feature (brief §4) | Tutorial | Source |
|---|---|---|
| Points, lines, polylines, NURBS curves, arrows | T4 (rows), T6 (creation) | `walk/{points,curves}.rs`, `segments.rs`, `vectors.rs`, `ribbon.wgsl`, `arrow.wgsl`, verbs `point line polyline curve arrow arrowhead` |
| Meshes, faces, edges, BRep | T4 | `walk/{mesh*,brep*}.rs`, `faces.rs`, `triangle.wgsl` |
| Planes, construction plane, grid | T4, T6, T3 | `planes.rs`, `plane.wgsl`, `cplane.rs`, `grid.wgsl` |
| Point clouds, octrees, sheets | T11 | `cloud.rs`, `lod.rs`, `splat*`, `walk/cloud*`, `walk/sheet.rs` |
| Orbit, pan, zoom, fit, projection | T5 (verbs `fit view` T13) | `camera.rs`, `input.rs`, `touch.rs`, `keys.rs` |
| Picking, hover, selection (click, box, lasso, small, by name) | T7 | `pick.rs`, `selection.rs`, `gesture/*`, verbs `select_*` |
| Gumball translate / rotate / scale | T8 | `gizmo.rs`, `widget*.rs`, `widget.wgsl`, `state/drag.rs` |
| Transform commands, copy, orient, project | T6 | verbs `move rotate scale copy orient_3_points project_to_plane`, `deform.rs` |
| Primitives, surfacing, trim/extend/split/explode, measure | T6 | §3.3 T6 |
| Undo / redo, transactions | T6 (model), T13 (verbs) | `edit.rs`, `state/edit.rs`, verbs `undo redo` |
| Layers, hierarchy, visibility, lock, groups, instancing | T9 | `layers.rs`, `hierarchy.rs`, `scene_display.rs`, `scene_instances.rs`, `ui/*` |
| Shaded, outline, Arctic, Outline, Attributes | T10 | `surface_outline*`, `ssao*`, `ambient_*`, verbs `arctic outline attributes` |
| Transparency / Opacity | T12 (explains T4's `faces.rs`) | `faces.rs`, `triangle.wgsl`, verb `opacity` |
| Hidden-line visibility | T12 (headers T3) | `triangle_tiles.rs`, tile shaders, `ink_visibility.wgsl` |
| Clipping planes, section caps | T12 (header T3) | `clip.rs`, `clip.wgsl`, `cap.wgsl`, `clipping.rs` |
| Scene text, fonts, text quality page | T10 | `engine/text.rs`, `gpu/text*.rs`, `text_*.wgsl`, `scene_text.rs` |
| egui panels, theme, overlay, number box | T3.1, T9 | `gpu/ui.rs`, `app/ui/*` |
| Command line, completion, history, phone keyboard | T13 | `command_dock/*`, `ui/command_line*`, `ui/phone.rs`, `agent.rs` |
| CLI (native), FastMCP, programmatic API | T13 (gap noted), T14 (blocked) | none |
| Manifests, protobuf decode/validate, save/open, fetch, streaming, live | T15 | §3.3 T15 |
| Performance tiers, timers, inspection, diagnostics, self-test | T16 | §3.3 T16 |
| 2D mathematics stage (rounded-rectangle matrices) | T3.2 | `app/stage/*` core |
| Stage in the real scene | T17 | `stage/{application,spatial,mode,geometry,net,view}.rs` |
| 3D matrix lane (black-outlined cells) | not taught; removed | §4 |

---

## 4. Matrix visualisation cleanup (brief §4)

### 4.1 Which matrix is which (FACT)

| | **Remove** — 3D matrix lane (screenshot `docs/screenshots/active/geometry-four-objects-1280.png`) | **Keep** — 2D stage matrix (`docs/screenshots/active/math-matrix-light-1280.png`) |
|---|---|---|
| Look | cells on one quad in the 3D scene, black cell edges, procedural 7-segment digits (`matrix.wgsl:1-2`: "draws the cell edges and writes the value in white") | rounded cells with TeX glyph entries on a 2D slide (`stage/matrix.rs:8`: "A matrix as rounded cells with centered entries") |
| Source | `src/app/walk/matrix.rs` (169), `src/engine/gpu/matrices.rs` (264), `src/shaders/matrix.wgsl` (284) | `src/app/stage/matrix.rs`, `stage/draw.rs` (`rounded`), `stage/paint.rs` |
| Hooks | 7 `// register:matrices` lines: `app/scene.rs` ×2, `app/walk/mod.rs`, `engine/gpu/context.rs:412`, `engine/gpu/lane.rs:88`, `engine/gpu/mod.rs`, `state.rs`; untagged callers `app/scene.rs:788-811` (`matrix::walk`, `matrix::title`), `app/loader.rs:469`, `app/live.rs:371` (`holds_matrix`) | none to change |
| Tutorial | not taught (§3.2 `REMOVE`) | T3.2 |

Removing the lane from the viewer is Phase 11 work; the tutorial edits below are independent of it, except the two live includes (row 1, row 9), which break the strict site build the moment `walk/matrix.rs` is deleted.

### 4.2 References to the 3D matrix lane (`grep -ril matrix docs/` → 2,635 files; almost all are camera/view/object matrices and stay)

| # | File | Reference | Action |
|---|---|---|---|
| 1 | `docs/journey/completion-geometry.md` | `:3` intro lists "numerical `Matrix` objects" and "16 specialized WGSL lane shaders"; `:5` screenshot `geometry-four-objects-1280.png`; `:148` Step 3 heading "`7`-segment matrices"; `:150` figure `07c-glyphs-splats-matrix-widget.svg`; `:190-192` file table rows; `:194-201` prose + **live includes** `walk/matrix.rs:kernel-matrix-upload`, `matrix.wgsl:7:38`; `:243`, `:253` "2×3 Matrix"; `:262` question | delete matrix parts; recapture screenshot (mesh cube, BRep cube, arrowed curve); lane count 16 → 15 |
| 2 | `docs/screenshots/active/geometry-four-objects-1280.png` | the unwanted picture | replace with a matrix-free capture |
| 3 | `docs/screenshots/active/evidence.json` | `:61-65` asset + caption "2x3 Matrix" | update with the new capture |
| 4 | `docs/illustrations/stage/07c-glyphs-splats-matrix-widget.svg` | "7-segment matrix cells" panel | redraw without the matrix panel (rename) |
| 5 | `examples/course/geometry.rs` | `:8` imports `Matrix`; `:67-70` `Matrix::new(2, 3)` "Kernel Matrix" | remove the object |
| 6 | `docs/journey/completion-teaching.json` | witnesses for the 3 files | delete with the files |
| 7 | `docs/journey/completion-batch.json` | subject `geometry` title "Kernel geometry, BRep, curves and matrix display" | retitle |
| 8 | `docs/journey/completion-contract.md` | `:19` required coverage "Matrix display" | delete the item |
| 9 | `docs/journey/consolidation/96zzzzzzzm-retain-visible-instance-ranges.md` | `## Kernel Matrix display`, **live includes** `walk/matrix.rs:kernel-matrix-upload` (`:11`), `:kernel-matrix-title` (`:21`) | historical page: freeze the snippet into `journey/code/` or archive the page (rule: do not rewrite historical evidence) |
| 10 | `docs/file-map.md` | `:9` "Matrix display also require current subject owners" | delete clause |
| 11 | `docs/journey/stage-gap.md` | `:59` "Matrix lane", `:68` "matrix display" | delete clause |
| 12 | `docs/README.md:3`, `docs/journey/active.md:17`, `inspection/ARCHITECTURE.md:13` | "33 WGSL" shaders | 32 after removal |
| 13 | Historical journey: `96zzzzzzzq-draw-original-cad-wire-curves.md` (26 hits, `src/production_matrix.wgsl`), `96zzzzf-prepare-shared-record-cap-picks.md`, `96zzzzg-own-shared-record-cap-pick-resources.md` (inventory paths), `consolidation/96-retain-non-drawing-originals-and-validate-raw-planes.md`; snippets `journey/code/96zzzzzzzq-draw-original-cad-wire-curves-matrix-*` (7 files) | historical checkpoints | leave (frozen snippets, no live include); exclude from any new route |
| 14 | `docs/journey/redraft-10-sections/{README,10-section-course-outline,old-to-new-coverage-map}.md`, `sections/section-10-shared-gpu-precision-and-lighting.md`, `lessons/96*.md` | lists `matrix.wgsl` as a feature | leave or archive with the redraft |
| 15 | `docs/journey/release/{matrix-browser-review,matrix-native-review,plane-original-covered-matrix-native-review}.json` | review receipts (images not in `docs/`) | keep as historical evidence |
| 16 | `docs/archived/journey-stage-gap-planning.md:11` | "Matrix display already has renamed `production_matrix*`" | archived, leave |

### 4.3 2D matrix references to keep

| File | Asset |
|---|---|
| `docs/journey/completion-mathematics.md:6` | `screenshots/active/math-matrix-light-1280.png` (+ `math-matrix-dark-1280.png` in `evidence.json`) |
| `docs/journey/completion-mathematics.md:153` | `illustrations/stage/03-stage-palette-flat.svg` (co-highlighted cells) |
| `docs/journey/stage-scene.md:7,10,71,82` | `stage-fdm-givens-light-1280.png`, `stage-pillow-grid-1280.png`, `stage-fdm-matrices-light-1280.png`, `stage-fdm-pillow-390-light.png` |
| `docs/journey/03a-mathematics.md:224,226` (historical) | `03a-cumulative-mathematics-{landscape,portrait}-20261114.png` |
| `docs/archived/journey-03a-mathematics-standalone.md:229` | `screenshots/math-full/portrait-step-4.png` |

---

## 5. Open items for whoever writes the tutorials

| # | Item | Why |
|---|---|---|
| O1 | Put the nine (or seventeen) active subjects in `mkdocs.yml` and move the archived 00–37 into an "Archive" group | S1 |
| O2 | Replace partial/line-range includes with whole-file includes; add includes for the 259 witness-only files (78,074 lines) | brief "Do not omit complete source code"; §1.3 |
| O3 | Re-split the 199-file `application` step into T5–T17 using the existing `register:` tags; prove each cut with `course -- subject-audit ID` and `course -- verify ID` | contract forbids a final dump; UNVERIFIED that tag cuts compile today |
| O4 | Rebind the 6 stale witnesses; add `.cargo/config.toml` | S8, §2.5 |
| O5 | Delete the untracked `docs/reconstruction/` residue; `docs/maintaining.md` already states the Python generators are retired, but agent memory notes still describe `series_git.py`/`fold.py`/`check_steps.py` as current | §1.2 |
| O6 | Fix design notes citing never-built files (S6) or label them "proposal, superseded by …" | S6 |
| O7 | Redraw the 29 `illustrations/stage/*.svg` as session-viewer decks (decided: no Mermaid) | S11 |
| O8 | Add tests where a tutorial has none: translucency/Opacity (T12), `splat.rs`/`lod.rs`/`cloud.rs` (T11), `faces.rs`, `widget.rs`, `walk/mesh.rs`, `command_dock/mod.rs` (0 `#[test]` each) | verification column |
| O9 | T14 and the native half of T13 need implementation first (IMPLEMENTATION_ROADMAP) | S12 |

---

## 6. Method (commands run, all read-only)

| Measurement | Command |
|---|---|
| nav paths | `grep -oE 'docs/[^ ]+\.md' mkdocs.yml` + `test -e` per path |
| redirect stubs | Python over `docs/journey/*.md`: ≤ 11 lines and a "Continue with"/`archived/` link; follow non-archived link, `os.path.exists` |
| cited paths | regex `(src|examples|tests|assets)/….(rs|wgsl|cjs|js|html|yaml|toml)` per page group + `os.path.exists` |
| includes | regex `--8<-- "session_viewer/([^"]+)"` over the 19 active pages; ranges `a:b`, named sections resolved via `[start:NAME]`/`[end:NAME]` markers in source |
| witnesses | `docs/journey/completion-teaching.json` vs SHA-256 of LF-normalised files |
| lesson 37 parity | `filecmp.cmp` of `src/**` vs `docs/lessons/37/src/**` |
| tags | `grep -rho '// register:[a-z_0-9]*' src | sort | uniq -c` |
| tests | `grep -A3 '#\[test\]' <file> | grep -oE 'fn …'` per module; `ls tests` |
| removed tooling | `git log --oneline -- docs/reconstruction/series_git.py …` → `7143d361c` |
| matrix | `grep -ril matrix docs/`; `grep -rliE 'matrix\.wgsl|gpu/matrices|walk/matrix|7-segment|Matrix display|MatrixRows|four-objects' docs --include=*.md`; `grep -rn 'register:matrices' src` |
