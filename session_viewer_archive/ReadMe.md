# session_viewer_archive

The previous viewer, kept for reference. It is NOT built or taught any more: `session_viewer/`
is the tree of record, rebuilt lesson by lesson (`session_viewer/docs/`). What this archive
still knows how to do, and what is missing from the new viewer, is listed here so it is not
forgotten; the master copy is Phase 6 of `session_viewer/docs/_ROADMAP.md`.

## Next steps — what returns to session_viewer, in order

**Picking and selection** (archive: `pick.rs`, `state_pick.rs`, `engine/gpu` id pass)
- ⬜ scene BVH over object boxes; frustum culling per frame
- ⬜ cursor → ray in kernel space; ray against the kernel mesh BVH; object-level pick
  (`Scene.order` is the row → guid map)
- ⬜ picking thin geometry (segments, glyphs) by screen distance, solid-vs-thin priority
- ⬜ selection and hidden flags: one flag word shared by every lane (bits 2-5 are taken)
- ⬜ sub-object picking: face / edge / vertex ids (`Row` is the seam)
- ⬜ id-buffer picking: an R32Uint attachment and an async readback, for dense scenes

**Command line, history, undo** (archive: `state_cmd.rs`, `coord_parser.rs`, `undo_state.rs`,
`state_undo.rs`)
- ⬜ egui HUD: status line, command prompt, options panel
- ⬜ command bus: a `Command` trait at the first mutation; options and numeric input
- ⬜ history with autocomplete
- ⬜ delete + undo / redo as inverse commands

**Transform and draw** (archive: `gumball.rs`, `gumball_state.rs`, `snap.rs`, `state_tool.rs`,
`tool_state.rs`, `cad_plane.rs`, `CAD_SKETCHER_PLAN.md`)
- ⬜ gumball: geometry, scale hit-test, translate, rotate, scale, numeric entry, commit
- ⬜ draw tools: point, line, polyline, curve
- ⬜ snapping: end / mid / center / grid / perpendicular
- ⬜ work plane (`cad_plane.rs`): draw on any plane, not only the ground
- ⬜ copy / array
- ⬜ control-point and edit-point (Greville) editing (`edit_points.rs`, `edit_state.rs`,
  `state_edit.rs`)

**Panels and text** (archive: `tree_ui.rs`, `state_ui.rs`, `text.rs`, `text.wgsl`)
- ⬜ scene tree panel with visibility / selection, and the tree ↔ viewport link
- ⬜ layers
- ⬜ text labels in the viewport
- ⬜ measure: distance, angle, area

**Files** (old lessons 64-66, 106)
- ⬜ reconcile: reload a changed file into a live scene without rebuilding the rest
- ⬜ save: write the edited `Session` back to `.pb` (browser download / native file)
- ⬜ watch: reload when a file on disk changes (native)
- ⬜ import / export: other formats through the kernel's readers

**Post-processing and look** (archive: `ssao.wgsl`, `ssao_blur.wgsl`, `composite.wgsl`,
`mask.wgsl`, `ground.wgsl`, `state_render.rs`)
- ⬜ GTAO / SSAO, arctic global-illumination look, outline anti-aliasing, composite pass
  (extra targets and passes after the scene list)
- ⬜ section planes
- ⬜ textures on meshes
- ⬜ sheet impostors: a drawing as one textured quad at distance

**GPU tessellation** (old lessons 88-91; the CDT never ports, trim-by-fragment replaces it)
- ⬜ GPU curves, GPU surfaces, GPU trimming, GPU BRep

**Scale** (old lessons 103, 114-119)
- ⬜ compute-shader ink; segment batches; quantized meshes; meshlets; mesh LOD;
  hierarchical-Z occlusion

**Housekeeping** (old lessons 110-111)
- ⬜ dev toolbox (perf overlay, frame capture); web polish (loading states, errors, URL state)

Kernel-side work the audit named and the viewer cannot fix alone: the decode cost of the
protobuf wire (75-79% of a native load), packed mesh arrays, the octree build.

---

# The archived viewer itself

WebAssembly 3D viewer for session geometry — wgpu + winit + Trunk.

## Run

```bash
cd session_viewer
trunk serve
```

Open http://localhost:8769

## Kill

```bash
# Git Bash (MINGW):
taskkill //F //IM trunk.exe

# PowerShell / cmd:
taskkill /F /IM trunk.exe
```

## Switch demo

Edit `src/demo.rs`, comment/uncomment one line in `active_scene()`:

```rust
pub fn active_scene() -> (Session, Vec<CylinderSegment>) {
    make_floor_scene()
    // make_cdt_scene()
}
```

## Install (once)

```bash
cargo install trunk
rustup target add wasm32-unknown-unknown
```

## Architecture

```
lib.rs           — State struct, event loop (App/ApplicationHandler), GPU init
  tree_ui.rs     — Tree/group helpers: leaf collection, group-lock DFS, egui tree renderer
  camera.rs      — Camera transform, orbit/pan/zoom controller, named views (T/B/L/R)
  gpu_session.rs — Per-frame GPU data: mesh/line/point/cylinder/sphere/nurbs upload + picking
  gpu_adapters.rs— Geometry → GPU vertex conversion (Mesh, Polyline, NurbsSurface, BRep, ...)
  pipelines.rs   — wgpu render pipelines + bind group layouts (mesh, line, point, cylinder, ...)
  gumball.rs     — Translate/rotate handle widget: hit test, drag math, build geometry
  pick.rs        — Screen-to-ray, pick_by_ray (wraps session.ray_cast)
  demo.rs        — Active scene factory: active_scene() selects floor or CDT demo
  text.rs        — Point label rendering: font atlas, quad + glyph vertex builders
```

**Data flow:**
1. `demo::active_scene()` returns a `Session` (geometry + tree) and optional cylinder decorators
2. `GpuSession::rebuild_from` converts session objects to GPU buffers (instances, vertices, indices)
3. Each frame: `State::update()` uploads camera uniform; `State::render()` runs geometry → gumball → egui passes
4. Click: `process_pick()` ray-casts via `session.ray_cast` + GPU nurbs/brep/nc pickers → expands hit GUID to locked group → updates selected flags

**Key subsystems:**
- **Picking** — `ray_cast` returns all hits sorted by distance; solid (Mesh/OBB) beats thin (Polyline/Line) at same depth; hidden objects skip but don't block
- **Group lock** — `group_locked: HashSet<String>` stores group node names; `locked_group_for_guid` DFS finds innermost lock; click expands to all leaves
- **Gumball** — 3-axis translate + rotate arcs rendered as cylinders/cones/spheres; drag updates GPU model matrix; `commit_object_transform` bakes final xform back into session geometry
