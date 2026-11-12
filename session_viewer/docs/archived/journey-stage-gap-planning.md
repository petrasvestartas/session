> Archived planning notes. For current scope and status, use the [stage dependency audit](stage-gap.md).

# Stage tutorial dependency audit

Status: proposed ownership, not runnable checkpoints or accepted browser evidence.

## November 12 decision and fresh evidence

The learner must build wgpu from scratch and arrive at the maintained viewer, using the fewest coherent subject lessons and lean code. Keep the production tree as the destination; a separate demonstration or final runtime link does not complete it. The old 43 numbered pages and reference index are now in `docs/archived/`; their fixtures remain in place. All 43 reconstruction checks and the strict documentation build passed after that move.

The re-measured source has 323 files, including 20 stage modules and 9,428 stage lines. The journey has 1,361 steps, 7,413 snippet files and 717 edit paths. Only five edit paths coincide with production paths. This measures layout divergence, not absence of equivalent concepts. Matrix display already has renamed `production_matrix*` implementations in the late `96zzzzzzzq` checkpoint and must move to its subject owner.

Fresh checks fail independently of archival: `structure` lacks the stage inventory; `parity` compares the historical lesson 37 file list; `verify --stored` lacks current `01-canvas` evidence. The checkpoint suite passes 22 tests and fails two source expectations. These failures must be repaired without relabeling historical acceptance as current.

The restrictions and storage counts in the historical notes below describe earlier work. The current task includes every production subject, including the dock, matrices, planes, pacing, splats, box selection, scene display and live revision. None is excluded merely because it was outside that earlier task.

The active course is `docs/journey/course.json` with snippets in `docs/journey/code`; the 43 older crates remain historical reference inputs.

| Subject | Production source under src/app/stage | Owner | Proposed position |
| --- | --- | --- | --- |
| Slide frame and letterbox | paint.rs, run.rs constants | new | immediately after 03a-panel |
| Math font and metrics | font.rs | new | after frame |
| TeX layout | tex.rs | new | after metrics |
| Glyph atlas and painting | paint.rs, draw.rs | new | after TeX |
| Column and equals alignment | object.rs, layout.rs | new | after painting |
| Matrices and sparse patterns | matrix.rs | new | after column |
| Clock, easing, appear and vanish | tween.rs, actions.rs | new | after matrices |
| Steps, slides and replay backwards | run.rs | new | after clock |
| Matching transforms | morph.rs | new | after replay |
| Deck schema, blocks and contextual errors | deck.rs | new | end of early block |
| Position expressions and kernel geometry | eval.rs, geometry.rs, spatial.rs | new | after scene identity, camera, strokes, arrows, markers and loading |
| Placement, reveal and colour | spatial.rs | new | after geometry |
| Authored camera and fit | view.rs | new | after placement |
| Projected labels | geometry.rs, spatial.rs | new | after camera |
| Derive, move and fly | actions.rs | new | after labels |
| Net matrices and elimination | net.rs | new | after motion |
| Overlap checks and all-deck playback | check.rs, tests.rs | new | after net |
| Keys, touch and upright layout | mod.rs, paint.rs | new | after checks |
| R2 loading and early fetch | mod.rs, ../ui/mod.rs, ../../lib.rs, index.html | new | end of later block |
| Portable arrow corner | src/shaders/arrow.wgsl and vector/plane callers | 36bb-heads subject | update existing arrow subject |
| Device probe | historical engine/gpu/vectors.rs | docs/debugging.md | debugging reference |

## Dependency findings

The course has 1,361 steps and no stage source owners. `36bb-heads` writes `src/chain.wgsl`, not `src/shaders/arrow.wgsl`; neither production caller is owned by the course. Its explanatory production note now records the field interface and phone fault, but its runnable implementation is still divergent.

The generator replays the `steps` array in order (`scripts/course/model.ts`); IDs need not be renumbered. Book section boundaries and consolidated checkpoint membership must be updated when inserting steps. The current inventory is `current-viewer-2026-11-05.json`; structure checks exact source names and hashes, so adding coverage alone does not update the baseline.

`paint.rs` imports `run::Stage`, `draw::rounded` and drawing types. `run.rs` imports geometry, camera, deck and object types. `deck.rs` imports the net solver. These are real compile dependencies, not merely teaching order. Copying whole modules after 03a would introduce the later subjects early. Splitting their teaching across two blocks would reopen earlier-owned files.

Stage integration also adds an event payload, loading handler, input routing and render dispatch in `src/lib.rs`, a state field, and `stage_frame` in `src/app/ui/mod.rs`. The latter has no current journey owner. There is no existing single registration line that installs all of this.

## Smallest proposed architecture change

Separate the design-space frame and glyph/cell drawing interface from `Stage`, so the early math checkpoint can paint a column using only egui, fonts and math primitives. Keep scene rows and the camera in a later adapter.

Introduce one application-mode registration point whose interface covers load completion, input consumption and frame rendering. The ordinary application installs the dispatch once; the stage supplies its adapter later. Avoid adding stage-specific branches to already-taught entry-point and UI files. The exact interface needs a source design review; this audit does not assert that a registry alone resolves the coupled deck/action types.

The user requested tutorial-only changes, so no viewer refactor has been applied. Early checkpoints must remain planned until that dependency split is authorized and implemented, or the requested order/write-once constraint changes.

## Verification and archive constraints

`course -- parity` currently compares production directly with `docs/lessons/37/src` in `scripts/course/reference.ts`. Refreshing an inventory cannot make it pass. Replacing this with a comparison against the reconstructed journey destination needs a separately reviewed tooling change; repairing lesson 37 is explicitly outside this task.

The site plugin still publishes the old numbered pages and builds a separate reference navigation chain. The old crates, SERIES.txt and typing manifest also remain inputs to reference checks. Moving or removing them without migrating these consumers would break checks and links. Archival must preserve those fixtures while removing the old learner route; a label alone is not a completed archive or deployment.

The initial storage check failed at 29,152,559,922 bytes total and 20,143,493,856 generated, against 20 GiB and 8 GiB limits. No heavy verification or site build was started. Existing uncommitted stage matrix/UI edits and tutorial quality-progress evidence belong to other work and are preserved.

## Worked-example and checkpoint contract

Use equilibrium for the first complete scene; use the other four shipped decks for matrices, elimination and materialization. Each eventual subject needs one visible goal, a prediction experiment, a question/answer, exact source edits and a runnable checkpoint. Do not publish these proposed owners as completed lessons.

Teach once that geometry uses real scene rows and the real camera; math glyph IDs require the MATH-table layout and egui glyph painter. Movement must express the idea: projection drops its first component and draws subsequent components in order. Explain kernel Vector/Color/Line lerp, Polyline resampling/morph and Xform interpolation where scene motion consumes them, with links to the existing three-language implementations and tests rather than copied kernel listings.

Command dock, GPU planes/matrices/pacing, splats, box selection, scene display and live revision remain outside the requested additions. No production behavior or historical lesson-37 source has been changed.
