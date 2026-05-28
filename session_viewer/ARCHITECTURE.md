# session_viewer — Architecture & Roadmap

A WebGPU (wgpu) + winit + egui 3D viewer compiled to WASM. Its long-term goal is to grow
from a **viewer** into a **CAD interface**: draw model geometry interactively, then save it to a
protobuf **Session** — the project's native file format (`session_rust::Session`).

This document is the map: how it works today, how to extend it safely, and the staged plan to
turn it into a CAD app. Read it top to bottom once; after that, jump to "How to extend".

---

## 1. The mental model: a 5-layer stack

`State` (in `lib.rs`) is split into five sub-structs, each owning one concern. Higher layers may
read/drive lower layers; lower layers never reach up. To isolate a broken layer, comment out its
render step — the others still run.

```
┌─────────────────────────────────────────────────────────┐
│  ShellState   (shell)  egui UI + CLI + command log       │  shell_state.rs
├─────────────────────────────────────────────────────────┤
│  UndoState    (hist)   undo/redo action stacks           │  undo_state.rs
├─────────────────────────────────────────────────────────┤
│  GumballState (gb)     transform gizmo + its GPU buffers  │  gumball_state.rs
├─────────────────────────────────────────────────────────┤
│  SceneState   (scene)  session geometry + camera + pick  │  scene_state.rs
├─────────────────────────────────────────────────────────┤
│  GpuCtx       (gpu)    device/queue/surface/pipelines    │  gpu_ctx.rs
└─────────────────────────────────────────────────────────┘
```

`State`'s methods are physically split across `state_*.rs` files that are `include!`d into `lib.rs`
(see §8 — this is tech debt, not the target). Each file is one verb-group:

| file | what it adds to `impl State` |
|------|------------------------------|
| `state_update.rs`      | `update()` — per-frame camera/uniform upload, pick dispatch |
| `state_render.rs`      | `render()` — the GPU passes + egui pass |
| `state_pick.rs`        | `process_pick()`, `selected_centroid()` |
| `state_cmd.rs`         | `execute_command()` (CLI), `apply_thickness()` |
| `state_ui.rs`          | `build_ui()` — the whole egui panel |
| `state_interaction.rs` | mouse/key handlers, `commit_object_transform()`, `fit_view()` |
| `state_undo.rs`        | `undo()`, `redo()`, `apply_undo()`, `apply_redo()` |

---

## 2. The frame lifecycle

winit drives an event loop (`App` in `lib.rs`). Rendering is **continuous**: `render()` calls
`self.window.request_redraw()` on its first line, so every frame schedules the next. Any state
mutation is therefore visible on the following frame without manual repaint.

```
WindowEvent
  ├─ KeyboardInput → (Ctrl+Z/Ctrl+U intercepted FIRST, before egui) → undo()/redo()
  ├─ egui_state.on_window_event(...)  → if consumed, stop 3D handling
  └─ else → handle_key / handle_mouse_* / handle_scroll → mutate SceneState/GumballState
RedrawRequested
  ├─ update()   // camera matrix → camera_buf; run pending pick; size the gumball
  └─ render()   // request_redraw; geometry pass (MSAA) → gumball pass → resolve → egui pass
```

Key detail: **Ctrl+Z/Ctrl+U are intercepted at the top of `window_event` before egui sees them**,
so focus in the CLI text field doesn't swallow the shortcut. Ctrl state is tracked from
`ControlLeft`/`ControlRight` key events (`scene.ctrl_down`) because `WindowEvent::ModifiersChanged`
is unreliable on the web backend — see §7.

---

## 3. How geometry reaches the screen

The single most important data path. Source of truth is `scene.session` (a `session_rust::Session`);
the GPU mirror is `scene.gpu_session` (`GpuSession` in `gpu_session.rs`).

```
session.lookup: HashMap<guid, Geometry>          (CPU truth)
        │  gpu_session.rebuild_from(&session)      ← full rebuild
        │  gpu_session.add_geometry(guid, geom)    ← incremental add
        ▼
GpuSession CPU-side buffers (one per primitive kind):
   tri/line/point vertices · segments_cpu (cylinders) · glyphs_cpu (spheres)
   cloud points · cones_cpu · instances_cpu (per-object model matrix + color + flags)
        │  *_dirty flags set on mutation
        │  flush_geometry()  uploads only dirty buffers to the GPU each frame
        ▼
render() geometry pass issues one draw per pipeline:
   grid → mesh → line → point → cylinder → sphere → cloud → cone → text → glyph
```

Supporting pieces:

- **`gpu_adapters.rs`** — pure CPU converters: `mesh_to_vertices`, `mesh_edges_to_segments`,
  `pts_to_segments`, unit cylinder/sphere templates, etc. (CPU geometry → GPU vertex/segment structs).
- **`gpu_arena.rs`** — a growable GPU vertex/index buffer (`GpuArena<V>`) so meshes share one buffer
  with bump allocation instead of one buffer per object.
- **`gpu_instance_groups.rs`** — GPU instancing: many copies of one template mesh in a single draw
  call (used by the instancing demo; the path that scales to thousands of repeated parts).
- **`InstanceData`** (in `gpu_session.rs`) — per-object `model` matrix, color, and `flags`
  (`FLAG_SELECTED`, `FLAG_HIDDEN`, `FLAG_GLYPHS_HIDDEN`, `FLAG_EDGES_HIDDEN`). Selection, hiding and
  highlighting are all just flag bits read by the shaders — no geometry rebuild.
- **`PickTable`** — bidirectional `guid ↔ instance_id` map; every drawable registers one instance id.

NURBS surfaces and BReps don't live in `lookup`; they live in `session.objects.nurbssurfaces` /
are tessellated on add. Their CPU pick meshes are cached in `gpu_session.nurbs_pick_meshes` /
`brep_pick_meshes` for ray testing.

---

## 4. Selection & picking

`pick.rs` casts a world-space ray from the cursor (`screen_to_world_ray`) and tests it against the
session BVH and the cached pick meshes. Priority: a solid (mesh/BRep) wins over a thin
(line/polyline) at the same depth. The hit's guid is toggled in `scene.selected_guids` and its
`FLAG_SELECTED` bit is set, which the shaders render as a highlight. The **Tree** and **Graph** UI
panels select the same way (they push into `new_sel`, applied after the egui closure in `build_ui`).

Tree/Graph selection highlight is grey (`Color32::from_gray(180)`) with black text — set per-widget
via `ui.visuals_mut().selection.bg_fill` (the global default is black, which is invisible behind
black text, so each selectable list must opt into grey).

---

## 5. The gumball (transform gizmo)

`gumball.rs` builds a 3-axis translate/rotate gizmo as cylinders (shafts/arcs), cones (arrowheads)
and spheres (handles), drawn in a dedicated overlay pass (always-on-top). Drag flow:

```
mouse down on handle → Gumball.drag = Some(DragState); snapshot pre-drag geometry into
                       gb.drag_geom_snapshots / drag_nurbs_snapshots; record drag_origins (models)
mouse move           → update_drag() computes delta matrix → update_transform() on each instance
mouse up             → commit_object_transform() bakes the matrix into CPU geometry,
                       then push UndoAction::Transform onto the history (see §7)
```

`commit_object_transform` **bakes** the matrix into coordinates for most types (`Mesh::transform`,
`Polyline::transform`, `NurbsSurface::transform`, …) and resets the GPU instance to identity. BReps
are the exception: they keep a live model matrix (no re-tessellation).

---

## 6. The session data model (`session_rust`)

The viewer is a thin shell over `session_rust::Session`. Persistence is **already solved at the data
layer** — the viewer just doesn't call it yet (see §10).

| API | location | role |
|-----|----------|------|
| `Session::pb_dumps() -> Vec<u8>` | `session.rs:272` | serialize whole session to protobuf bytes |
| `Session::pb_loads(&[u8]) -> Result<Session>` | `session.rs:371` | deserialize |
| `Session::file_json_dumps/loads` | `session.rs:250` | JSON variant (debug/inspect) |
| `add_point/line/plane/polyline/pointcloud/mesh/brep/element` | `session.rs:873-1006` | add geometry + tree node + lookup entry, consistently |
| `add_group`, `add_edge`, `add_hierarchy` | `session.rs:1083-1178` | tree groups + relational graph |
| `lookup: HashMap<String, Geometry>` | `session.rs:62` | O(1) geometry-by-guid (CPU truth) |

The model is dual-structured: a **tree** (`session.tree`, hierarchy → the Tree panel) and a
**graph** (`session.graph`, relationships → the Graph panel). Both serialize into the `.pb`.

---

## 7. History / undo — design & current state

### Current model
`UndoState` holds two `Vec<UndoAction>` stacks (cap 64). `UndoAction` is a coarse-grained enum:

- `AddLookup { guid, geom }` / `AddNurbs { ns }` — CLI adds. Undo removes, redo re-adds.
- `RemoveObjects { objects }` — CLI `del`. Undo re-adds all, redo removes all.
- `Transform { objects, snapshots, snapshots_after }` — gumball drag. **Absolute snapshots on both
  sides**: `snapshots` = pre-drag geometry (undo target), `snapshots_after` = post-drag geometry
  (redo target). BReps carry no snapshot and use the before/after model matrices instead.

### Two bugs that made history feel broken (both fixed)
1. **Dead trigger.** Ctrl+Z checked `key_mods.control_key()`, but `key_mods` was only populated by
   `WindowEvent::ModifiersChanged`, which the winit **web backend delivers unreliably** — so the
   check read false and `undo()` never ran. *Fix:* track Ctrl directly from `ControlLeft`/
   `ControlRight` key events (`scene.ctrl_down`). Plus **on-screen ↶/↷ buttons** (top of the right
   panel) that bypass the keyboard entirely.
2. **Asymmetric redo.** Undo restored absolute snapshots, but redo re-baked the *delta* matrix via
   `commit_object_transform` (which bakes into CPU coords). After multi-object drags or repeated
   cycles the delta accumulated and objects jumped/doubled. *Fix:* capture `snapshots_after` at
   commit and restore it symmetrically — both directions are now absolute-state restores.

### Where history should go (the CAD target)
The enum approach works but every new editable operation needs a new variant + matching
undo/redo arms — it won't scale to dozens of CAD tools. Target: a **Command pattern**.

```rust
trait Command {
    fn apply(&mut self, scene: &mut SceneState, gpu: &GpuCtx);   // do / redo
    fn revert(&mut self, scene: &mut SceneState, gpu: &GpuCtx);  // undo
    fn label(&self) -> &str;                                     // for a history list UI
}
```

- Every mutation (add, delete, transform, recolor, draw-new-geometry) becomes a `Box<dyn Command>`.
- `History { done: Vec<Box<dyn Command>>, undone: Vec<Box<dyn Command>> }` replaces `UndoState`.
  `do(cmd)` = `cmd.apply()` + push to `done` + clear `undone`. `undo` = pop `done`, `revert`, push
  `undone`. `redo` = pop `undone`, `apply`, push `done`. The ↶/↷ buttons and Ctrl+Z/U call these.
- The history list can be shown as a UI panel (the user's "go through history") — click any entry to
  jump to that point.
- Snapshots stay the simplest correct revert primitive for baked geometry; matrix-based revert stays
  for BReps. The Command just owns whichever it needs.

This generalizes today's three actions into one extensible mechanism and is the foundation for
interactive drawing (§9), where each completed draw is just another `Command`.

---

## 8. Module map & what's too big

Current `src/` (lines as of this writing). The project dislikes big files; everything over ~300
lines is a split candidate.

| file | lines | role | verdict |
|------|------:|------|---------|
| `engine/gpu/*` | 8 files | GPU mirror, split: types/session/geometry/edit/instancing/draw/pick | ✅ done (was `gpu_session.rs` 1360) |
| `engine/pipelines/*` | 4 files | pipelines, split: mod/camera_uniform/layouts/build | ✅ done (was `pipelines.rs` 661) |
| `lib.rs` | 550 | App, event loop, `State`, `State::new`, `include!`s | **split** |
| `gpu_adapters.rs` | 493 | CPU→GPU geometry converters | borderline |
| `gumball.rs` | 475 | gizmo geometry + drag math + hit test | split (geometry / drag) |
| `state_ui.rs` | 460 | the whole egui panel (CLI + tree + graph + settings) | split per panel |
| `camera.rs` | 459 | camera, controller, projection, named views | borderline |
| `tree_ui.rs` | 378 | scene-tree panel rendering | ok |
| `state_interaction.rs` | 345 | mouse/key handlers + commit_transform | ok |
| `demo.rs` | 344 | **hardcoded demo scene (app data, not engine)** | move to app/ |
| `gpu_arena.rs` | 303 | growable GPU buffer | ok |
| `pick.rs` | 302 | ray-cast picking | ok |
| (others < 300) | | scene/gumball/shell/undo state, update, render, cmd, text, ctx | ok |
| `shaders/*.wgsl` | 721 | 10 WGSL shaders (already foldered ✓) | done |

### Structural issues beyond size
- ~~**`include!` is not modularity.**~~ ✅ Fixed (step 4): the seven `state_*.rs` are now real `mod`s
  with their own imports and `pub(crate)` cross-module methods — proper module boundaries, no shared
  scope. (They still sit at the crate root; grouping them under `app/`/`ui/` is step 5.)
- **No engine/app separation.** `State::new` hardcodes `demo::active_scene()`, the group names
  `"FloorModel"`/`"FloorPolylines"`, and app-specific auto-hide rules. The reusable viewer engine
  (GPU, pipelines, camera, picking, gumball, render loop) can't be embedded with a different scene
  without editing `lib.rs`.
- **Fragile WASM-boundary error handling.** `State::new` uses `.unwrap()`/`.expect()` for surface,
  adapter and canvas. On a browser without WebGPU/WebGL2 this panics with no user-facing fallback.
- **Almost no tests.** Only `pick.rs` has `#[cfg(test)]`. Undo logic, matrix math, arena range-shift
  on `remove()`, and command parsing are untested — exactly the error-prone parts.
- **Ambiguous build setup.** Two Trunk configs (`Trunk.toml`, `Trunk_floor.toml`), two HTML entries
  (`index.html`, `floor.html`), and a stray Vite/TS toolchain (`vite.config.ts`, `tsconfig.json`,
  `package.json`) coexist undocumented. `assets/` holds only 4 SVG icons. Canonical path should be
  documented (or the unused one removed).

---

## 9. Target architecture

Goal: every leaf file < ~300 lines, **real modules** (not `include!`), and a clean **engine / app /
ui** split so the engine is reusable and the app owns scene content and tools.

```
src/
├── lib.rs                 // App, run_web, event loop only
├── state.rs              // struct State + State::new wiring (no include!)
├── math.rs              // mat4_mul_cm + matrix helpers (out of lib.rs)
│
├── engine/               // reusable, scene-agnostic — zero demo/CLI/group-name references
│   ├── gpu/
│   │   ├── mod.rs        // GpuSession struct + fields (so children see private fields)
│   │   ├── ctx.rs        // GpuCtx, depth/msaa textures
│   │   ├── arena.rs      // GpuArena
│   │   ├── session.rs    // rebuild/flush/draw orchestration
│   │   ├── geometry.rs   // add_geometry / add_mesh / add_brep / remove
│   │   ├── templates.rs  // instance groups + register_template_*
│   │   ├── transform.rs  // update_transform / set_flag / set_color
│   │   └── adapters.rs   // *_to_vertices / *_to_segments
│   ├── pipelines/
│   │   ├── mod.rs        // Pipelines struct + new()
│   │   ├── camera_uniform.rs
│   │   ├── layouts.rs    // bind-group-layout builders
│   │   └── build.rs      // build_pipeline + per-pipeline ctors
│   ├── camera.rs · pick.rs · text.rs
│   └── gumball/{mod.rs, drag.rs}
│
├── app/                  // the CAD application built ON the engine
│   ├── scene_state.rs · gumball_state.rs · shell_state.rs
│   ├── demo.rs           // hardcoded scene = app data, not engine
│   ├── commands.rs       // CLI execute_command
│   ├── persistence.rs    // NEW: save (pb_dumps→Blob) + load (file input→pb_loads)
│   ├── interaction/{mod.rs, transform.rs, box_select.rs, fit.rs}
│   ├── pick.rs · update.rs · render.rs
│   ├── tools/            // NEW: interactive drawing (§ roadmap)
│   │   ├── mod.rs        // Tool trait + active-tool dispatch
│   │   └── point.rs · line.rs · …
│   └── history/{mod.rs, command.rs}   // Command-pattern history (§7)
│
├── ui/{mod.rs, tree.rs, panels…}      // egui presentation only
└── shaders/*.wgsl                      // done ✓
```

**Privacy note (why the struct goes in `gpu/mod.rs`):** in Rust a private field is visible in its
defining module *and descendants*. Put `GpuSession` in `gpu/mod.rs` and the `impl GpuSession` blocks
in child files (`geometry.rs`, `draw`, …) can access private fields. If the struct lived in a sibling
(`gpu/session.rs`) the other files couldn't — that's the trap to avoid. Same pattern for `State`:
define it in `state.rs`, put method `impl`s in submodules of the crate root.

**Migration order (each step compiles + is independently reviewable):**
1. ✅ shaders → `src/shaders/` (done).
2. ✅ Split `gpu_session.rs` (1360) → `engine/gpu/{types,session,geometry,edit,instancing,draw,pick}.rs`.
   Every `GpuSession` field is `pub`, so the struct lives in `types.rs` and the `impl` blocks sit in
   sibling files; cross-module helper calls (`write_instance*`, `grow_instance_buffer`) became
   `pub(crate)`. Pure code movement — `cargo check` proves no behavior change.
3. ✅ Split `pipelines.rs` (661) → `engine/pipelines/{mod,camera_uniform,layouts,build}.rs`.
   `include_str!` paths became `../../shaders/…` (relative to the new file location). The crate root
   keeps the old names via `use engine::gpu as gpu_session;` and `use engine::pipelines;` so no other
   call site changed.
4. ✅ Converted the 7 `include!("state_*.rs")` into real `mod`s with `impl crate::State`, per-module
   `use` headers, and `pub(crate)` on cross-module methods. `include!` is fully eliminated. (They
   live at the crate root for now; the `app/` grouping below is step 5.)
5. Pull `engine/` out so it has no app references; move `demo.rs` + CLI + group conventions to `app/`,
   the `state_*.rs` modules under `app/`, the `ui` (build_ui/tree) under `ui/`. Also move
   `gpu_adapters.rs`/`gpu_arena.rs`/`gpu_instance_groups.rs` under `engine/gpu/`.
6. (Optional) extract `engine/` into its own crate once the boundary is clean.

Remaining files > ~300 lines (next split candidates): `lib.rs` 550, `gpu_adapters.rs` 493,
`gumball.rs` 475, `state_ui.rs` 460, `camera.rs` 459, `engine/pipelines/build.rs` 434 (7 cohesive
pipeline ctors), `tree_ui.rs` 378, `state_interaction.rs` 345, `demo.rs` 344, `engine/gpu/types.rs`
342 (cohesive layout structs), `gpu_arena.rs` 303, `pick.rs` 302.

---

## 10. How to extend (recipes)

### Add a new geometry primitive (CPU → screen)
1. In `session_rust`, ensure the type is in `Geometry` and serializes (it likely already is).
2. In `gpu_adapters.rs`: write `my_type_to_vertices/segments`.
3. In `gpu_session.rs::add_geometry`: add a match arm that fills the right CPU buffer + sets the
   dirty flag + registers an instance id in `PickTable`.
4. If it needs a new buffer kind, add the `*_cpu` vec + `*_dirty` flag + upload in `flush_geometry` +
   a `draw_*` method, and call that draw in `state_render.rs`.
5. Picking: add its pick mesh to the appropriate cache (`*_pick_meshes`) so rays can hit it.

### Add a new shader + pipeline
1. Drop `myshader.wgsl` in `src/shaders/` with `vs_main`/`fs_main`.
2. In `pipelines.rs`: add a `RenderPipeline` field, build it in `Pipelines::new` via the right helper
   (`build_pipeline` / `build_instanced_pipeline` / …), `source: include_str!("shaders/myshader.wgsl")`.
3. In `state_render.rs`: `set_pipeline` + bind groups + draw in the geometry pass.

### Add a new CLI command
`state_cmd.rs::execute_command` — add a match arm, build geometry, call `session.add_*`, mirror into
`gpu_session`, and `hist.push(UndoAction::…)`. Add it to the `CMDS` autocomplete list in `state_ui.rs`.

### Add a new keyboard tool / mouse interaction
`state_interaction.rs::handle_key` (or `handle_mouse_*`). Mutate `scene`/`gb`; the continuous redraw
shows it next frame. For undoable edits, push an `UndoAction` (or a `Command` once §7 lands).

### Add a new UI panel section
`state_ui.rs::build_ui` — add an `egui::CollapsingHeader`. Collect user intent into a local var inside
the egui closure, then apply it to `self` **after** the closure (the existing `new_sel`/`vis_chg`
pattern) — you can't borrow `self` mutably inside the closure.

### Add a new undoable action
Today: add an `UndoAction` variant + arms in `apply_undo`/`apply_redo`. Prefer absolute snapshots for
baked geometry (see `Transform`). After §7: implement `Command` instead — no enum churn.

---

## 11. CAD roadmap

The data layer is ready; the gaps are I/O and interactive creation.

### Phase 1 — Save / Load sessions (small, unblocks everything)
- `app/persistence.rs`:
  - **Save:** `let bytes = scene.session.pb_dumps();` → JS `Blob` → trigger a download
    (`web_sys`: create a `Blob` from a `Uint8Array`, `URL::create_object_url`, click a temp `<a>`).
  - **Load:** an `<input type="file">` (or drag-drop) → read `ArrayBuffer` → `Session::pb_loads(&bytes)`
    → `scene.session = …; gpu_session.rebuild_from(&session, device, queue);` and re-run the
    tree/visibility setup currently in `State::new`.
- Wire to two toolbar buttons next to ↶/↷, and to `save`/`load` CLI commands.
- Add a round-trip test in `session_rust`: build mixed geometry → `pb_dumps` → `pb_loads` → assert
  `lookup` keys equal. (Catches serialization regressions; satisfies the "verify before claiming"
  rule.)

### Phase 2 — Interactive drawing (the actual CAD-ness)
- A `Tool` trait + an active-tool slot on `State`:
  ```rust
  trait Tool {
      fn on_click(&mut self, world: Point, scene: &mut SceneState) -> ToolStatus; // accumulate pts
      fn on_move(&mut self, world: Point);                                        // rubber-band
      fn preview(&self) -> Vec<PreviewGeom>;                                      // ghost geometry
      fn finish(self: Box<Self>) -> Box<dyn Command>;                            // commit as a Command
  }
  ```
- Start with `PointTool` (one click → `add_point`), then `LineTool` (two clicks), `PolylineTool`,
  `BoxTool` (base rectangle + drag height). Each completed tool yields a `Command` (§7) → undoable
  for free.
- **Snapping** lives in `pick.rs`: extend the ray hit to also return nearest vertex / grid point /
  edge, with a tolerance, so clicks land precisely. This is what separates a CAD tool from a toy.

### Phase 3 — Unify on the Command history (§7)
Once tools exist, migrate `UndoState` → `History<Box<dyn Command>>`. Add a **History panel** (the
user's "go through history") listing labels with click-to-jump. Drawing, editing, deleting,
recoloring all flow through one mechanism.

---

## 12. Quick reference

- **Run:** `trunk serve` → `localhost:8769` (kill `trunk.exe` before restarting). Trunk auto-rebuilds
  on file save; **reload the browser tab** and **click the canvas** (focus) for keyboard shortcuts.
- **Compile-check fast:** `cargo check --target wasm32-unknown-unknown` (≈1–2s incremental).
- **Shortcuts:** RMB orbit · Shift+RMB pan · scroll zoom · LMB select · LMB-drag box-select ·
  Shift+LMB add · F fit · C reset cam · P/O persp/ortho · T/B/L/R views · Q shading · E back-face ·
  **Ctrl+Z undo · Ctrl+U redo** (or the ↶/↷ buttons).
