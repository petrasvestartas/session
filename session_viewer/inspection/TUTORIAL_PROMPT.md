# Session Viewer — tutorial prompt

How to write the tutorials that rebuild the viewer. Replaces the tutorial half of `Session Viewer.txt`. The per-tutorial file lists are in [TUTORIAL_ROADMAP.md §3](TUTORIAL_ROADMAP.md#3-proposed-roadmap-proposal-brief-9-sequence); the house rules are in `session/.claude/rules/viewer-docs.md` and `session_viewer/docs/maintaining.md` — read both first, they override anything here that conflicts.

## 1. What a tutorial is

A tutorial is an implementation manual for **one subject of the real viewer**, not a course page about WebGPU. Minimal explanation, complete code, one diagram.

- **Teach only code that exists and compiles at HEAD.** If a subject has no code (FastMCP, the native CLI, the core/UI split), the tutorial is *blocked*: record it as unfinished and build the feature first ([IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md) M4/M6). Never write a tutorial for planned code.
- **One cumulative program.** No toy apps, no parallel demo runtime. A later tutorial adds files and tagged registration lines (`// register:TAG`); it never rewrites a finished file.
- **Every production file appears in full exactly once**, in its final form, in the tutorial that owns it, through a source include (`--8<-- "session_viewer/<path>"` or a named section marker). Never paste code into Markdown. Hub files (`state.rs`, `app/runtime.rs`, `lib.rs`) show only the tagged lines the tutorial adds.
- **Teach the present program.** No history of drafts, fixes or commits inside a lesson.

## 2. Lesson format

Use this order; omit an empty section:

1. **Goal** — one sentence: the visible result and the subject.
2. **Diagram** — one figure drawn in the session viewer (§4).
3. **Prerequisites and files** — earlier tutorials needed; the files this tutorial owns, in build order.
4. **Build** — per file: `### Step k · path`, 2–3 sentences (what it owns, the one trap), then the include. The learner types the file by hand from the include.
5. **Integration** — the exact `// register:TAG` lines added to hub files.
6. **Run and observe** — the checkpoint command and what appears.
7. **Predict and try** — one experiment, prediction first.
8. **Question and answer** — always visible: *How to work it out*, then *The answer*. Never hidden, never faded; late tutorials explain as fully as early ones.
9. **Validation** — the checks that actually ran and any limitation.

Explain a Rust or rendering concept once, at first use, preferably as a short source comment. Do not repeat the course map or a "where you are" map per page.

## 3. Sequence (17 tutorials, adjusted to real dependencies)

| # | Tutorial | Owns (summary; full list in TUTORIAL_ROADMAP §3.3) | Status |
|---|---|---|---|
| T1 | Project and browser start | `Cargo.toml`, `index.html`, `Trunk.toml`, `lib.rs`, `app/launch.rs` | writable |
| T2 | wgpu device, surface, targets, present | `engine/gpu/{device,context,targets,present,pacing}.rs` | writable |
| T3 | Render engine and shaders; 3.1 egui painting; 3.2 2D mathematics (stage matrix, TeX layout) | `engine/pipelines/*`, shared WGSL headers (`projected_triangle`, `clip`, `ink_visibility` prelude), `app/stage` base | writable |
| T4 | Geometry and scene data (walk → rows → lanes) | `app/walk/*`, `engine/gpu/{arena,segments,vectors,glyphs,planes,objects}`, `app/scene*` | writable |
| T5 | Camera and navigation; **introduces `State` and `runtime.rs`** with T1–T5 tags only | `camera.rs`, `app/{input,touch,keys,gesture}`, `state.rs`, `app/runtime.rs` | writable |
| T6 | Object creation and transformations; **owns the command registry core** | `app/command/{mod,verbs/mod,tool}`, creation verbs, `app/{modeling,snap,cplane,edit}` (undo) | writable |
| T7 | Picking and selection | `engine/gpu/pick.rs`, `app/selection.rs`, `cloud_query`, `sheet_query` | writable |
| T8 | Gumball | `app/gizmo.rs`, `engine/gpu/widget*`, `state/drag.rs` | writable |
| T9 | Layers and scene management | `app/{layers,hierarchy}.rs`, `app/ui/{layers,graph}.rs`, `scene_display.rs` | writable |
| T10 | Display modes: outlines, Arctic (SSAO), attributes, text | `engine/gpu/{surface_outline,ssao,text,text_plane,text_plate}`, `verbs/arctic.rs` | writable |
| T11 | Point clouds, octrees, sheets, streaming | `walk/cloud*`, `engine/gpu/{cloud,lod,splat}`, `app/stream.rs`, `examples/add_lod.rs` (offline octree) | writable |
| T12 | Transparency and advanced rendering: tile visibility, clipping, caps | `engine/gpu/{triangle_tiles,clip}`, `clip/*` | writable |
| T13 | Command system: dock, completion, history | `command_dock/*`, remaining verbs | writable (native CLI part blocked on M6) |
| T14 | FastMCP | — | **blocked** until M6 |
| T15 | File formats: protobuf decode, `.session` archive, live reload | `app/{decode,loader,live,fetch,session_io*}` | writable |
| T16 | Performance: pacing tiers, LOD, diagnostics, the `selftest` harness | `engine/{performance,diagnostics}.rs`, `gpu/timing.rs`, `selftest/*` | writable (numbers from M1) |
| T17 | Complete application: stage in the real scene, final wiring | `app/stage` scene adapter, remaining hub tags | writable |

Rules for the sequence:

- Split any checkpoint that adds more than one coherent subject (the current `application` step adds 199 files at once).
- Every one of the 352 production files has exactly one owner tutorial, or an approved omission with a reason. Verify against the coverage matrix in TUTORIAL_ROADMAP §2.
- Subsystems the original prompt forgot must be owned: slide decks / stage (T3.2, T17), text (T10), streaming and out-of-core (T11), sheets (T11), clipping and caps (T12), snapping and construction planes (T6), undo transactions (T6), live reload (T15), SSAO (T10).
- The removed 3D matrix lane is not taught anywhere. The 2D rounded stage matrix stays in T3.2.

## 4. Diagrams

- One diagram per tutorial, **drawn in the session viewer** as a stage deck or scene (YAML under `examples/stage/<name>/`, see the deck guide `session/.claude/skills/session_viewer_math/SKILL.md`). Keep the YAML next to the captured image. **No Mermaid.**
- Existing figures in `docs/illustrations/stage/*.svg` and D2 sources in `docs/diagrams/*.d2` stay until their tutorial is rewritten; then redraw them as decks.
- Captures need a browser whose WebGPU actually renders. The development machine's browser renders the viewer black: capture elsewhere (`npm --prefix ../session_tests run course -- capture`), or record the figure as pending.

## 5. Verification (a tutorial is done only when all pass)

Run from `session/session_viewer`, through the build limits in [AGENT_BRIEF.md §4](AGENT_BRIEF.md#4-machine-and-build-rules):

| Check | Command |
|---|---|
| Checkpoint compiles and runs | `npm --prefix ../session_tests run course -- verify <ID>` |
| Every file explained in its owning section | `npm --prefix ../session_tests run course -- teaching` (no new errors) |
| Active course parity | `npm --prefix ../session_tests run course -- active-audit` |
| Includes and links resolve | in `session_tests`: `DOCS_STRICT_LINKS=1 DOCS_BASE=/docs/ npm run build -- --outDir ../session_viewer/target/docs/vue --emptyOutDir` |
| Native tests | `cargo xtest -j8` (no new failures beyond the two known arrow tests until M0 fixes them) |
| Browser behaviour | the Playwright test named in the tutorial, on a machine with working WebGPU; otherwise state it was not run |

Record what actually ran. A passing site build is not proof a tutorial is complete.

## 6. Code facts every tutorial must respect

- API examples match the pinned versions: wgpu 29.0.4, winit 0.30.13, egui 0.34.3, Rust 1.97.1. Do not copy older wgpu samples.
- `.cargo/config.toml` defaults every `cargo` command to wasm32; native tests use `cargo xtest`.
- Shaders are assembled at build time (`build.rs` `#include` expansion into `OUT_DIR`); the ink prelude (`PRELUDE`, `engine/pipelines/core.rs`) is shared by every lane.
- Registration happens through `lane_list!`, `lane::REGISTRY`, `pass::PASSES`, `features` hook tables and the `verbs!` macro, tagged `// register:TAG`.

## 7. Bibliography

Keep `docs/references.md` current: each entry gives the link, the subject and the viewer module it informs. Prefer the WebGPU and WGSL specifications, MDN WebGPU, wgpu docs for 29.x, WebGPU Fundamentals and the WebGPU samples; verify every API against the pinned version.
