# Audit summary — Session Viewer

Audit of `session/session_viewer` at superproject HEAD `032cd8c0f`, 2026-11-19, per `audit_agent.txt`. No source file was changed. All reports are in this folder:

| Report | Phase | Content |
|---|---|---|
| [AGENT_BRIEF.md](AGENT_BRIEF.md) | — | **entry point for an agent**: reading order, rules, workflow, reporting |
| [PRODUCT_BRIEF.md](PRODUCT_BRIEF.md) | — | what the viewer must become (replaces the product half of `Session Viewer.txt`) |
| [TUTORIAL_PROMPT.md](TUTORIAL_PROMPT.md) | — | how tutorials are written and verified (replaces its tutorial half) |
| [REPOSITORY_INSPECTION.md](REPOSITORY_INSPECTION.md) | 1 | layout, versions, builds and tests actually run, measured frame numbers, CI, disk |
| [FEATURE_INVENTORY.md](FEATURE_INVENTORY.md) | 2 | every feature with status, source, tests, issues, tutorial; gaps vs brief §4 |
| [ARCHITECTURE.md](ARCHITECTURE.md) | 3 | module graph, lifecycle, ownership, cycles, coupling, target architecture |
| [DATA_STRUCTURES.md](DATA_STRUCTURES.md) | 4 | kernel → walk → GPU structures, sizes, ownership, duplication, hot-path allocations |
| [SHADER_INVENTORY.md](SHADER_INVENTORY.md) | 5 | 33 WGSL files, 73 render + 4 compute pipeline variants, per-frame order |
| [PERFORMANCE_AUDIT.md](PERFORMANCE_AUDIT.md) | 6 | measured vs hypothesis, missing instrumentation, prioritised fixes |
| [INTERFACE_INVENTORY.md](INTERFACE_INVENTORY.md) | 7 | 80 verbs, keys, URL knobs, channels; shared `Op` API proposal |
| [FILE_FORMATS.md](FILE_FORMATS.md) | 8 | formats read/written, memory per load, format evaluation |
| [TUTORIAL_ROADMAP.md](TUTORIAL_ROADMAP.md) | 9 | course state, coverage matrix of all 352 files, 17-tutorial plan |
| [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md) | 10 | milestones M0-M7 |
| [MATRIX_CLEANUP.md](MATRIX_CLEANUP.md) | 11 | removal plan for the 3D matrix lane (not executed) |

## 1. Current application state

- One crate, 319 `.rs` files / 106 k lines, 33 WGSL files; Rust 1.97.1, wgpu/naga 29.0.4, winit 0.30.13, egui 0.34.3. **WebGPU only, no WebGL2 fallback** (`engine/gpu/device.rs:31`).
- **The real working tree does not build**: `session_proto` sits at an older commit than the superproject records (`edc614e1e` missing locally), and `session_rust/src/proto/session_proto.rs` was regenerated from it at 11:20 today, before this audit started. A pristine copy of HEAD builds (`cargo check` wasm 10.7 s).
- **Native tests: 614 passed, 2 failed, 61 ignored** (`vectors::tests::arrow_is_exactly_as_long_as_its_line`, `vectors::walk_tests::curve_heads_land_on_their_ends`, both deterministic). CI is green because its runner has no GPU and these tests skip.
- CI runs only `cargo check` + `cargo xtest`; none of the 52 Playwright tests, 7 Python checks or 8 depth scripts run in CI; 8 Playwright tests import a missing `docs/extensions/browser.cjs`.
- Activity: 425 of 1,031 viewer commits in the last 30 days, mostly course docs, screenshots and `app/stage` (slide decks), not the engine.

## 2. Most important findings

1. **No UI-independent core.** `State`, `Scene` (59 fields) and `Gpu` are god objects; every one of the 67 verbs runs on `&mut State` (window + GPU). No verb is tested by running it.
2. **No CLI, no FastMCP, no programmatic API.** All automation channels are one-way (`?cmd=`, `?inspect=1`, postMessage reload).
3. **Arctic view exists** (`Arctic On/Off`, `verbs/arctic.rs`: SSAO + outlines + sky ambient) but only as a typed command: no key (the documented G key is unbound), no URL knob, not saved.
4. **The unwanted 3D matrix** (screenshot sha matches the brief) is the kernel Matrix lane: `walk/matrix.rs` → `gpu/matrices.rs` → `matrix.wgsl`, black edges from `LINECOLOR=[0,0,0,1]`. The 2D rounded matrix (`app/stage/matrix.rs`, egui-painted) shares no code with it. Removal is low-risk and fully planned.
5. **Spatial acceleration is thin:** the only octree is a point-cloud LOD built offline, off by default, ≥ 2 M points only, no frustum culling, no tests. No BVH, no object or back-face culling.
6. **Selection state is held in ≥ 9 places**; undo deep-clones whole objects on every geometry edit.
7. **Formats:** only session protobuf `.pb` (whole, gzip or byte-range streamed) and the `.session` archive. No OBJ/PLY/glTF/GLB/STL; JSON compiled out of the page build.
8. **Tutorials:** the active course is 19 pages that show 3.9% of source lines; it is missing from the sidebar (44 of 68 `mkdocs.yml` paths do not exist); the reconstruction scripts named in older notes were deleted on 2026-09-22.

## 3. Critical missing features

| Missing | Brief section |
|---|---|
| Core independent of UI; shared operation layer | §1, §3 |
| CLI and FastMCP | §1, §7 |
| Object/edge/face hover highlight | §4 Interaction |
| Wireframe mode (x-ray only; meshes > 200 k triangles have no edges) | §4 Rendering |
| Frustum / back-face / object culling; scene BVH | §4 Geometry, Rendering |
| Properties panel, material model, display-settings panel | §4 Scene, §3 UI |
| Import/export beyond `.pb` (no export at all; no local file import) | §6 |
| Camera and display state in saved files | §6 |
| Browser frame instrumentation (CPU split, p95, completion latency) | §5 |

## 4. Biggest performance problems

Measured (native Vulkan, RTX 4080 Laptop, 900×700, single or double runs; not browser):

| Finding | Number |
|---|---|
| GPU `ink` pass (strokes, edges, text) is ~90% of GPU frame time in every scene | 104 MB `view_local.yaml`: frame 18.5 ms, ink 17.4 ms, faces 0.05 ms |
| Heavy scene misses 60 Hz at rest and spikes while orbiting | 54 fps at rest; orbit p95 37.3 ms |
| MSAA is a minor share of the ink cost | ink −9% (`view_local`) / −22% (timber) at `VIEWER_MSAA=1` |
| Desktop default opacity 0.95 costs ink on timber-floor | 7.6 → 4.7 ms at opacity 1 (no effect on `view_local`) |
| Load is the BRep walk on the main thread | `treppenhaus` 48.6 MB: decode 319 ms, walk 3.34 s |

Strongest unconfirmed lead: the per-fragment ink-visibility tile walk (`ink_visibility.wgsl:386-418`) — PERFORMANCE_AUDIT H14. Browser-side hypotheses still unmeasured: every bare mouse move redraws the full scene (H1), egui lays out every layer row each frame (H2), one-frame-in-flight pacing quantises to vsync (H3), DOM/URL reads on the hot path (H7).

## 5. Architectural weaknesses

- Cycles: `engine → app` (gizmo constants), `modeling → command`, `layers ⇄ scene`, `stream ⇄ walk` (ARCHITECTURE.md §2).
- 18 `thread_local!` singletons including UI models; 253 `cfg(wasm32)` gates.
- `Scene` imports engine row types and takes `&mut Gpu`; `walk` uses 33+ `engine::gpu::*` paths.
- Three registration schemes plus `// register:` comment tags form an out-of-band module graph the compiler does not check.
- `app/stage` (10.9 k lines, a presentation engine) lives inside the viewer's `app` tree and dominates recent churn.
- `docs/lessons/` holds 44 full crate snapshots (1.96 M lines) tracked in git; `target/` is 201 GB (≈ 75 GB course scratch builds).

## 6. Recommended implementation order

1. **M0** make the real tree build (fetch `session_proto`), fix the two red arrow tests, make CI GPU tests fail loudly instead of skipping, repair stale key docs and the missing `browser.cjs`.
2. **M1** instrumentation: browser CPU split, p95, completion latency, redraw causes, ink sub-pass timers; reproducible baseline.
3. **M2** performance, measured items first: ink visibility walk, opacity default, main-thread walk yielding; then hover redraws, layer list virtualisation, pacing depth, hot-path clones.
4. **M3** remove the 3D matrix lane (after approval).
5. **M4** `CommandHost` → rows module → `State` split → `viewer-core` crate.
6. **M5** missing functionality, one approved item at a time.
7. **M6** `Op` API, CLI, FastMCP over the same core.
8. **M7** tutorial reconstruction on the final structure.

Details, files and acceptance criteria: IMPLEMENTATION_ROADMAP.md.

## 7. Decisions

Answered 2026-11-19:

| # | Decision | Answer | Done |
|---|---|---|---|
| D1 | Delete the 3D matrix lane | **Remove** | Yes: 3 files deleted, 13 edits, Tutorial 7 page, illustration `07c-glyphs-splats-widget.svg`, two screenshots and their `evidence.json` entries, the consolidation section that included the deleted source; `cargo check` (wasm + native all targets) clean, `cargo xtest` 612 passed / 2 failed (the two pre-existing arrow tests), strict docs build passes |
| D2 | Kernel `Matrix` / `add_matrix` in the three kernels | **Stays** (data only; the viewer no longer draws it) | nothing to change |
| D5 | Desktop default opacity 0.95 | **Keep** | IMPLEMENTATION_ROADMAP 2.2 dropped |
| D7 | Browser/Playwright tests in CI | **Not in CI** | M0 keeps CI as cargo check + xtest |
| D10 | Tutorial diagrams | **Drawn in the session viewer** (stage decks/scenes), no Mermaid | `.claude/rules/viewer-docs.md`, `docs/maintaining.md`, TUTORIAL_ROADMAP updated |
| D11 | `target/` cleanup | **Remove what is not needed** | `session_viewer/target` 211 → 37 GB (course/scratch caches, incremental dirs, old example binaries); `session_rust/target` incremental + scratch; stale `session/session_view/` (old build only) |
| — | Dead viewer code and old lessons | **Remove dead code, keep old lessons**, delete Codex work files | Yes: 4 dead mask pipelines + their shader entry points, `PipelineDesc::masks`, `ColorWrite::Max`, the 4 compiler warnings; `.codex/tutorial-integration-20261113/` and the cleanup log deleted (Codex agent config kept) |

Still open:

| # | Decision | Recommendation |
|---|---|---|
| D3 | Recapture Tutorial 7 (now three objects) | needs a machine whose browser WebGPU renders (`npm --prefix ../session_tests run course -- capture`); this machine renders black |
| D4 | Historical matrix lesson steps in `docs/journey/96zzzzzzzq-*` and `docs/journey/code/` | leave (historical inputs) |
| D6 | Arrow-tip tests: geometry or tolerances | find the regressing commit first |
| D8 | Start the M4 architecture split | only after M0-M2 |
| D9 | Shrink the protobuf schema (three-kernel change) | defer until M2 numbers |
| D12 | Element builder verbs removed 2026-11-05: restore, or delete course steps `92*` | delete stale steps |
