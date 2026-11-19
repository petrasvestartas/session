# Session Viewer — product brief

What the viewer must become. Replaces the product half of `Session Viewer.txt`; every statement below is grounded in the 2026-11-19 audit (this folder). The tutorial half is [TUTORIAL_PROMPT.md](TUTORIAL_PROMPT.md). Work order and acceptance criteria are in [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md).

Live: https://petrasvestartas.github.io/session/ · Docs: https://petrasvestartas.github.io/session/docs/

## 1. Fixed requirements

| Requirement | Meaning |
|---|---|
| Browser application | The product ships only as wasm + WebGPU in the browser. Native builds exist **only** for tests, the headless `selftest` harness and a future CLI; they are not a product. |
| Rust + wgpu | Pinned versions: Rust 1.97.1, wgpu / naga 29.0.4, winit 0.30.13, egui 0.34.3, wasm-bindgen 0.2.128 ([REPOSITORY_INSPECTION.md](REPOSITORY_INSPECTION.md)). No WebGL2 fallback today; adding one is not a goal. |
| One geometry engine | `session_rust` (sibling repo) is the only geometry code. The viewer's `Scene` is the only scene model. Automation calls the same operations; never a second engine. |
| No silent removal | A feature is removed only with an explicit, recorded decision ([AUDIT_SUMMARY.md §7](AUDIT_SUMMARY.md#7-decisions)). |
| Performance first, correctness kept | Measure before and after every optimisation; never trade visual correctness for speed without a decision. |

## 2. Current state (facts, not goals)

| Area | State | Evidence |
|---|---|---|
| Build | Real tree fails (stale `session_proto` checkout); clean HEAD builds | REPOSITORY_INSPECTION "What was run" |
| Tests | 612 native pass, 2 fail (arrow tips), 61 GPU-ignored; 52 Playwright tests run by hand only | same |
| Architecture | `State` / `Scene` / `Gpu` god objects; every command needs window + GPU; 4 module cycles | [ARCHITECTURE.md §10](ARCHITECTURE.md#10-findings-with-evidence) |
| Automation | No CLI, no FastMCP, no programmatic API | [INTERFACE_INVENTORY.md §0](INTERFACE_INVENTORY.md) |
| Rendering | Unlit flat default, opacity 0.95, x-ray instead of wireframe, Arctic view (SSAO + outlines) by command only; no culling of any kind | [FEATURE_INVENTORY.md §4](FEATURE_INVENTORY.md) |
| Spatial structures | Point-cloud octree only, built offline, LOD off by default | [DATA_STRUCTURES.md §9.4](DATA_STRUCTURES.md) |
| Files | Session protobuf `.pb` and `.session` archive only | [FILE_FORMATS.md](FILE_FORMATS.md) |
| Performance (native, RTX 4080) | Ink pass ≈ 90% of GPU time; 104 MB scene 18.5 ms at rest, orbit p95 37 ms; BRep walk 3.3 s on load | [PERFORMANCE_AUDIT.md §3.4](PERFORMANCE_AUDIT.md) |

## 3. Target architecture

From [ARCHITECTURE.md §11](ARCHITECTURE.md#11-target-architecture-proposal--not-implemented), unchanged:

| Component | Owns | Must not depend on |
|---|---|---|
| `viewer-core` | scene, geometry adapters, layers, selection (one authoritative copy), camera, snapping, modeling, commands, undo, serialization | wgpu, winit, egui, web-sys |
| `viewer-rows` | Pod GPU row types, the walk → upload contract | wgpu |
| `viewer-gpu` | device, pipelines, WGSL, lanes, picking, text | winit, egui, app |
| `viewer-ui` | egui panels, command dock (read a view model, emit operations) | — |
| `viewer-stage` | slide decks, TeX layout | `State` |
| `viewer-automation` | `Op` schema, native CLI, FastMCP bridge | wgpu, winit |
| `viewer-web` | wasm entry, winit adapter, fetch/live, `index.html` glue | — |

One operation layer: `execute(&mut dyn Host, Op) -> Result<Reply, OpError>` reached identically by the egui command line, keys/tools, a native CLI and FastMCP ([INTERFACE_INVENTORY.md §4](INTERFACE_INVENTORY.md)).

## 4. Required capabilities

Status today → target. "Built" means traced in code, not a button.

| Capability | Today | Target |
|---|---|---|
| Points, lines, polylines, curves, meshes, BReps, surfaces, text, sheets | built | keep |
| Point clouds + octree LOD | partial | LOD on by default for large clouds, frustum rejection, tests |
| Spatial culling (frustum, back-face for closed solids) | missing | object frustum culling from bounds; optional back-face cull |
| Orbit / pan / zoom / fit / views / projection | built | keep; orbit p95 < 20 ms on the reference scene |
| Picking, selection (object / edge / face / control point) | built | one authoritative selection state |
| Hover pre-highlight | partial | object, edge and face hover |
| Gumball (move / rotate / scale) | partial (world axes) | pivot, planar handles, local frame |
| Layers, hierarchy, visibility, lock, colour | built | undo for visibility / lock / colour |
| Properties and display-settings panels | missing | read-only properties first |
| Shaded, x-ray, edges/silhouettes, Arctic (SSAO), clipping/caps, text | built | wireframe mode; Arctic by key / URL / saved |
| Transparency | partial (global, unsorted) | per-object alpha; sorting only with a use case |
| Import / export | `.pb`, `.session` | local `.pb` import; OBJ export through the kernel writer; no new format without a use case |
| CLI + FastMCP | missing | shared `Op` API (M6) |
| Instrumentation | partial | browser CPU split, p50/p95, completion latency, redraw causes (M1) |

## 5. Decisions already made (do not reopen)

| Decision | Answer |
|---|---|
| 3D kernel-matrix lane (square cells, black outlines) | **Removed** 2026-11-19. Kernel `Matrix` class stays as data. 2D rounded stage matrix stays. |
| Desktop default opacity | **Keep 0.95** |
| Browser / Playwright tests in CI | **No**; CI is `cargo check` + `cargo xtest` |
| Tutorial diagrams | **Drawn in the session viewer** (stage decks / scenes); no Mermaid |
| Old lessons and archived courses | **Keep** |

## 6. Performance targets

Reference: `assets/view_local.yaml` and `tests/fixtures/timber-floor.pb` through the native `selftest` harness (PERFORMANCE_AUDIT §5.1 A), plus browser numbers once M1 exists.

| Metric | Baseline (2026-11-19) | Target |
|---|---|---|
| `view_local.yaml` GPU frame at rest | 18.5 ms (ink 17.4) | < 16.7 ms |
| `view_local.yaml` orbit p95 | 37.3 ms | < 20 ms |
| Hover with no button pressed | full redraw per move (hypothesis H1) | 0 scene redraws |
| Largest BRep walk blocking input | 3.3 s | no main-thread task > 50 ms |

Every row is re-measured with the same command before a change is called an improvement.
