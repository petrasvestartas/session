# session_viewer

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
