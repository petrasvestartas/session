# session_viewer — Architecture & Roadmap (from-zero rebuild)

A browser-only (wasm32, WebGPU/wgpu + winit + egui) 3D viewer, rebuilt from scratch one documented
chapter at a time, growing into a proper **web CAD** environment. It is a thin shell over
`session_rust::Session` — the project's native protobuf file format and 9-type geometry model, which
is already done (full protobuf/JSON persistence, tree + graph, O(1) `lookup`).

This document is the contract for the rebuild. Two rules govern everything:

1. **No feature ever stacks into one file.** Every new concern gets its own module/file. When in
   doubt, make a new file. This is the #1 rule.
2. **The engine is reusable; the app is not.** `engine/` is scene-agnostic and contains **zero**
   references to demo scenes, group names, CLI verbs, `egui`, or `session_rust`. The archive's single
   biggest coupling sin was `State::new` hardcoding `demo::active_scene()` and group names like
   `"FloorModel"`; we do not repeat it.

The previous build (`../session_viewer_archive/`) reached ~13.7K LOC with camera, gumball, picking,
snapping, F10 edit-points, tree UI, undo/redo, draw tools, CLI, and egui all working. We treat its
`ARCHITECTURE.md` as the gold-standard *target shape* — but we rebuild clean, **not** port.

---

## 0. Working cadence (user-gated)

Chapters advance **only when the user says "I understand"** and asks to proceed. Today the viewer is
just a **window that clears the screen** (Chapter 1). We learn the current chapter together; no later
chapter's code is written ahead of that signal. The roadmap (§5) is the map, not a queue to run.

---

## 1. The mental model: a layered stack

`State` (in `state.rs`) is split into sub-structs, each owning one concern. Higher layers may
read/drive lower layers; lower layers never reach up. To isolate a broken layer, comment out its
render step — the others still run.

```
┌──────────────────────────────────────────────────────────┐
│  ui     (Ui)         egui overlay + CLI + history panel   │  ui/
├──────────────────────────────────────────────────────────┤
│  hist   (History)    Box<dyn Command> done/undone stacks  │  app/history/
├──────────────────────────────────────────────────────────┤
│  tools  (ToolHost)   active Box<dyn Tool> + preview       │  app/tools/
├──────────────────────────────────────────────────────────┤
│  gumball(GumballState) transform gizmo + its GPU buffers  │  app/gumball_state.rs
├──────────────────────────────────────────────────────────┤
│  scene  (Scene)      session + gpu_session + camera + pick│  app/scene.rs
├──────────────────────────────────────────────────────────┤
│  gpu    (Gpu + Pipelines)    device/queue/surface/pipes   │  engine/gpu, engine/pipelines
└──────────────────────────────────────────────────────────┘
```

`State` is defined **once** in `state.rs`; its method `impl`s are split across `app/*.rs` files as
real `impl crate::State { … }` blocks in real `mod`s — **never `include!`** (see §3). The layers
appear over chapters — Chapter 1 owns only `gpu`.

---

## 2. Target module tree

Every leaf file under ~300 lines, real modules, a hard **engine / app / ui** split. This is the
destination; chapters (§5) add these files incrementally. A file listed here does **not** mean it
exists yet — the chapter that introduces it is named in the chapter table.

```
src/
├── lib.rs                  // App (ApplicationHandler), run_web, event loop ONLY
├── state.rs                // struct State + State::new wiring (NO include!, NO demo data)
├── math.rs                 // mat4 helpers, ray/plane intersect — pure, no wgpu
│
├── engine/                 // REUSABLE, scene-agnostic. ZERO demo/CLI/group/session/egui refs.
│   ├── mod.rs              // pub mod gpu; pub mod pipelines; pub mod camera; pub mod pick; …
│   ├── gpu/
│   │   ├── mod.rs          // Gpu (device/queue/surface/config) + GpuSession structs live HERE
│   │   │                   //   so child files can impl them & see private fields (privacy trick, §3)
│   │   ├── arena.rs        // GpuArena<V>: growable bump-allocated vertex/index buffer
│   │   ├── session.rs      // GpuSession orchestration: flush_geometry / draw passes
│   │   ├── geometry.rs     // add_geometry / add_mesh / add_brep / remove (impl GpuSession)
│   │   ├── templates.rs    // instance groups, register_template_*  (GPU instancing)
│   │   ├── transform.rs    // update_transform / set_flag / set_color  (impl GpuSession)
│   │   └── adapters.rs     // *_to_vertices / *_to_segments — pure CPU→GPU converters
│   ├── pipelines/
│   │   ├── mod.rs          // Pipelines struct + new()
│   │   ├── camera_uniform.rs // CameraUniform + its bind group
│   │   ├── layouts.rs      // bind-group-layout builders
│   │   └── build.rs        // build_pipeline + per-pipeline ctors
│   ├── camera.rs           // Camera + Controller + projection + named views
│   ├── pick.rs             // screen_to_world_ray, ray↔mesh/BVH, snap candidates
│   ├── text.rs             // glyph/text rendering
│   └── gumball/
│       ├── mod.rs          // Gumball geometry (cylinders/cones/spheres) + hit test
│       └── drag.rs         // DragState + delta-matrix math
│
├── app/                    // the CAD application BUILT ON the engine
│   ├── mod.rs              // pub mod scene; pub mod update; pub mod render; …
│   ├── scene.rs            // struct Scene { session, gpu_session, camera, selected_guids, … }
│   ├── update.rs           // impl State::update() — camera→uniform upload, pick dispatch
│   ├── render.rs           // impl State::render() — ordered GPU passes + egui pass
│   ├── pick.rs             // impl State — process_pick(), selected_centroid()
│   ├── interaction/
│   │   ├── mod.rs          // handle_key / handle_mouse_* / handle_scroll
│   │   ├── transform.rs    // commit_object_transform() (bake matrix into geometry)
│   │   ├── box_select.rs   // drag-rectangle selection
│   │   └── fit.rs          // fit_view(), named-view shortcuts
│   ├── gumball_state.rs    // GumballState: gizmo instance + drag snapshots (app layer)
│   ├── demo.rs             // hardcoded demo scene = APP data, never in engine
│   ├── commands.rs         // CLI parse + dispatch (execute_command)
│   ├── persistence.rs      // save (pb_dumps→Blob→download) / load (file→pb_loads→rebuild)
│   ├── tools/
│   │   ├── mod.rs          // trait Tool + ToolHost (active-tool slot + dispatch)
│   │   ├── point.rs        // PointTool (1 click)
│   │   ├── line.rs         // LineTool (2 clicks)
│   │   ├── polyline.rs     // PolylineTool (N clicks + finish)
│   │   └── nurbscurve.rs   // NurbsCurveTool
│   ├── snap.rs             // snap-mode state + nearest vertex/grid/edge resolution
│   ├── edit_points.rs      // F10 control-point editing (NURBS CV drag)
│   └── history/
│       ├── mod.rs          // trait Command + History { done, undone } + do/undo/redo
│       ├── add.rs          // AddGeometry command
│       ├── remove.rs       // RemoveObjects command
│       └── transform.rs    // TransformObjects command (absolute snapshots)
│
├── ui/                     // egui PRESENTATION only — collects intent, applies after closure
│   ├── mod.rs              // build_ui() top-level layout
│   ├── toolbar.rs          // undo/redo, save/load, tool buttons
│   ├── cli.rs              // command text field + autocomplete + log
│   ├── tree.rs             // scene-tree panel (session.tree)
│   ├── graph.rs            // relationship-graph panel (session.graph)
│   ├── settings.rs         // shading/backface/projection toggles
│   └── history_panel.rs    // history list, click-to-jump
│
└── shaders/*.wgsl          // one .wgsl per pipeline (mesh/line/point/grid/gumball/text/…)
```

> **Grow a file into a folder, not the reverse (Chapter 1 reality).** A module in Rust can be either
> `name.rs` **or** `name/mod.rs` — identical to importers. So `gpu/` above starts life as a single
> file `engine/gpu.rs` (just the `Gpu` struct) and *becomes* the folder when `GpuSession` arrives in
> Chapter 7. We never create empty folders ahead of need; each leaf splits only when it actually grows.

### The engine/app boundary (enforced, not aspirational)

- `engine/` may depend on `wgpu`, `winit`, math (`math.rs`), and its own submodules. It may **not**
  name `session_rust`, `demo`, group strings, CLI verbs, or `egui` panels. `GpuSession::add_geometry`
  takes engine-level vertex/segment data produced by `adapters.rs`; the **translation** from a
  `session_rust::Geometry` into those structs happens in `app/scene.rs`, not in the engine.
- `app/` depends on `engine` + `session_rust`. It owns the demo scene, group/auto-hide conventions,
  CLI, persistence, tools, and history.
- `ui/` depends on `egui` and reads `app` state. It produces intent (selections, button presses),
  applied to `State` **after** the egui closure (you can't borrow `self` mutably inside it).
- `state.rs` is the only place all three meet. `State::new` wires layers but constructs **no** demo
  geometry inline — it calls `app::demo::active_scene()`, so swapping the scene never touches
  `state.rs` or the engine.

**Litmus test:** you could lift `engine/` into its own crate and embed a *different* app on top
without editing one engine file. If a demo change forces an engine edit, the boundary broke — fix the
boundary, not the symptom.

---

## 3. Modularity guidelines — "no feature stacks into one file"

Apply these every time a file grows or a feature is added.

### Hard rules
1. **Soft cap ~300 lines per leaf file.** Crossing it is a *split signal*, not a crime — but split by
   **concern**, not by line count. Cohesive tables (e.g. 7 pipeline ctors) may run longer if
   splitting would scatter one idea.
2. **One concern per file.** A file's name is a noun or verb-group; if you can't name it in one
   phrase, it's two files. New feature → new file, always. Never append a second subsystem "for now."
3. **Real `mod`s, never `include!`.** Every split file is a real module with its own `use` header.
   `include!` shares the parent's scope and hides coupling — banned. (The archive spent a whole
   migration step undoing `include!("state_*.rs")`; we never introduce it.)
4. **Method `impl`s split across files via `impl crate::State { … }`.** `State` is defined once in
   `state.rs`; `app/update.rs`, `app/render.rs`, `app/pick.rs` each contain a real `mod` with
   `impl crate::State { … }`. Cross-module helper methods are `pub(crate)`.

### The privacy trick (where a struct goes so children see its private fields)
A private field is visible in its defining module **and descendants**. Therefore:
- Put `GpuSession` (and its fields) in `engine/gpu/mod.rs`. Then `geometry.rs`, `transform.rs`,
  `session.rs` (children of `gpu`) can `impl GpuSession` and touch private fields.
- If `GpuSession` lived in a *sibling* (`gpu/types.rs`), the other `gpu/*` files could NOT — that's
  the trap. The defining file is the parent `mod.rs`; impls are the children.
- Same for `State`: defined in `state.rs`; impls live in `app/*` (descendants of the crate root) and
  reach `pub(crate)` fields. The layer that owns a field is the only one that mutates it.

### The "file got too big / new feature" checklist
- [ ] Does this file now hold **two** concerns? → split by concern into a child module.
- [ ] Is this a **new** feature? → new file under the correct layer (engine/app/ui), never appended.
- [ ] Did I use `include!`? → replace with a real `mod` + `impl crate::Type` + `use` header.
- [ ] New code touches private fields? → is the struct in the **parent `mod.rs`** of the impl files?
- [ ] Anything in `engine/` now names a demo/scene/group/CLI/session/egui symbol? → move it to `app/`.
- [ ] Cross-module method used by a sibling? → mark `pub(crate)`, not `pub`.
- [ ] egui code mutating `self` inside the closure? → collect intent in a local, apply after.
- [ ] WASM-boundary call (surface/adapter/device/`get_current_texture`/canvas/file/`pb_loads`)?
      → returns `Result`, no `unwrap`/`expect`.
- [ ] New chapter? → ship the matching `session_tests/viewer_sections/NN-*.md` (§6).

### Standing rule: robust WASM-boundary error handling
No `unwrap`/`expect` on anything that can fail at the browser boundary. Such calls return
`Result`/`Option` and bubble up via `anyhow::Result`, logged with `log::error!` and (where
user-facing) surfaced in the egui status line. A browser without WebGPU/WebGL2 must show a fallback,
not a silent panic. (Chapter 1's `lib.rs::resumed()` still `.unwrap()`s the canvas lookup as a
stopgap — Chapter 4's error pass replaces it.)

---

## 4. The three bake-in-early patterns (scheduled, never retrofitted)

Introduced the moment their first instance exists, so the codebase never grows a throwaway enum that
later needs ripping out.

- **(c) engine/app separation — from Chapter 7** (first real `session_rust` data): the `GpuSession`
  mirror lives in `engine/gpu/`, but the `Session`→GPU translation and the demo scene live in `app/`.
  From here on, every change is checked against the engine-purity litmus test (§2).
- **(a) Command-pattern history — at the FIRST mutation (Chapter 9):**
  `trait Command { fn apply(&mut self, scene, gpu); fn revert(&mut self, scene, gpu); fn label(&self) -> &str; }`
  and `History { done: Vec<Box<dyn Command>>, undone: Vec<Box<dyn Command>> }`. **Never** an
  `UndoAction` enum with per-operation variants (the archive's documented dead-end). Absolute
  snapshots are the simplest correct revert primitive; each Command owns whatever it needs.
- **(b) `Tool` trait — with the FIRST draw tool (Chapter 10):**
  `trait Tool { fn on_click(..)->ToolStatus; fn on_move(..); fn preview(..)->Vec<PreviewGeom>; fn finish(self: Box<Self>)->Box<dyn Command>; }`
  and a `ToolHost` slot on `State`. **Never** a `DrawTool` enum. `PointTool` is the first impl; each
  finished tool yields a `Box<dyn Command>` → undoable for free (composes with pattern a).

---

## 5. Chapter-by-chapter roadmap (learning order)

Each chapter: new files, their layer, the rule it demonstrates, and its docs page. Chapters 1–3 of
docs already exist (`01-run`, `02-dependencies`, `03-window`); Chapter 1 of *code* is built.
**Advancement is user-gated (§0).**

| Ch | Goal (new capability) | New files (layer) | Architecture rule demonstrated | Docs page |
|----|-----------------------|-------------------|--------------------------------|-----------|
| 1 | window that clears the screen ✅ | `lib.rs`, `state.rs`, `engine/gpu.rs` | lowest layer owns the 5 wgpu objects | `03-window.md` |
| 2 | first pipeline: a triangle from a shader | `engine/pipelines/{mod,build,layouts}.rs`; `shaders/triangle.wgsl` | a pipeline is a file; shaders foldered; ordered pass | `04-pipeline.md` |
| 3 | camera + uniforms (orbit a mesh) | `engine/camera.rs`, `pipelines/camera_uniform.rs`; `math.rs` | uniform bind group is its own file; camera is engine | `05-camera.md` |
| 4 | egui overlay + WASM error pass | `ui/{mod,settings}.rs`; error sweep | egui pass after 3D; no unwrap at the boundary | `06-ui.md` |
| 5 | orbit / pan / zoom + named views | `engine/camera.rs` (Controller); `app/interaction/{mod,fit}.rs` | engine = math, app = event→camera wiring | `07-input.md` |
| 6 | picking + selection | `engine/pick.rs`; `app/pick.rs` | ray math engine, selection app; `#[cfg(test)]` on ray math | `08-picking.md` |
| 7 | geometry + `GpuSession` mirror | `engine/gpu/{session,geometry,arena,adapters,transform}.rs`; `app/{scene,demo}.rs` | **(c) engine/app split**; privacy trick | `09-geometry.md` |
| 8 | grid (ground reference) | `shaders/grid.wgsl`; grid ctor in `pipelines/build.rs` | new pipeline = shader + ctor + one draw, nothing else | `10-grid.md` |
| 9 | delete selection + undo/redo | `app/history/{mod,remove}.rs`; `ui/toolbar.rs` | **(a) Command pattern at first mutation** | `11-history.md` |
| 10 | draw tools (point → line → polyline) | `app/tools/{mod,point,line,polyline}.rs`; `app/history/add.rs` | **(b) Tool trait at first draw tool** | `12-tools.md` |
| 11 | gumball transform + commit + undo | `engine/gumball/{mod,drag}.rs`; `app/{gumball_state,interaction/transform,history/transform}.rs` | gizmo geometry engine; commit + snapshot Command app | `13-gumball.md` |
| 12 | snapping (vertex / grid / edge) | `app/snap.rs`; extend `engine/pick.rs` | snap state app, nearest-candidate math engine | `14-snapping.md` |
| 13 | tree + graph UI + CLI | `ui/{tree,graph,cli}.rs`; `app/commands.rs` | UI collects intent, applies after closure | `15-cli-tree.md` |
| 14 | persistence: save/load | `app/persistence.rs`; `ui/toolbar.rs` | boundary I/O returns `Result`; `pb_dumps`/`pb_loads`; round-trip test | `16-persistence.md` |
| 15 | history panel + F10 edit-points | `ui/history_panel.rs`; `app/edit_points.rs` | everything already flows through `Command` | `17-history-panel.md` |

Beyond ch.15 (same rules): box/extrude tools, instancing at scale (`engine/gpu/templates.rs`), text
labels (`engine/text.rs`), more snap modes, BRep live-matrix handling — each its own file under the
right layer.

---

## 6. Where each chapter is documented

Every chapter ships a matching `session_tests/viewer_sections/NN-title.md`, rendered Shiki-highlighted
in the "Viewer" docs tab (one md per chapter; `NN` sets sidebar order; first `# heading` = sidebar
label; files starting with `_` are ignored — `_TEMPLATE.md` is the source). Each page follows the
template: **Goal · How it works · Code · My notes · Compare to the archive · Run · Verify.** A chapter
is **not done** until its `NN-*.md` exists.

---

## 7. Quick reference

- **Build target:** pinned to `wasm32-unknown-unknown` in `.cargo/config.toml` → **no `cfg` gates**.
- **Compile-check fast:** `cargo check` (already wasm by default).
- **Run:** `../bash/build_viewer.sh` → docs "Live" item, or `trunk serve` → `localhost:8770`.
- **Layer rule:** higher drives lower, lower never reaches up.
- **File rule:** one concern, ~300-line soft cap, real `mod`s, no `include!`.
- **Boundary rule:** `engine/` names no app symbol; WASM-edge calls return `Result`.
