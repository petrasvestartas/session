# Viewer curriculum — roadmap

The path from "clears the screen" to a full CAD viewer, in small chunks. Each lesson
is **one concept** and edits as few files as possible. Numbering continues the existing
files (`NN-title.md`, in-page heading is `NN-1`). After each lesson, snapshot the crate
into `viewer_sections/NN_title/` (as with `03_window/`, `04_pipeline/`).

Rule of thumb: if a lesson would touch more than ~2 files or introduce more than one new
idea, split it. Every lesson ends with something visible on screen.

Legend: ✅ done · ▶ next · ⬜ planned. Archive feature in (parens).

---

## Phase 0 — Setup & skeleton  ✅
- ✅ 01 Run — dev loop: npm + trunk
- ✅ 02 Dependencies — Cargo.toml, what each crate is for
- ✅ 03 Window — winit + wgpu surface, clear to grey
- ✅ 04 Pipeline — one hard-coded triangle (shader + pipeline)
- ✅ 05 Resize — canvas size vs drawing buffer vs DPR; no stretch

## Phase 1 — A camera (clip space → a real 3D world)
- ⬜ 06 Vertex buffer — move the triangle's corners out of the shader into a GPU buffer (`bytemuck`)
- ⬜ 07 Uniforms & bind groups — pass one value (a color/time) from Rust into the shader
- ⬜ 08 MVP matrix — a `view_proj` uniform; draw the triangle in world space (glam)
- ⬜ 09 Perspective vs ortho — projection matrix; toggle the two
- ⬜ 10 Orbit camera — RMB-drag to orbit, scroll to zoom (camera controller)
- ⬜ 11 Pan + named views — Shift-drag pan; T/B/L/R/front keys; mm scale (reference_viewer_camera)
- ⬜ 12 Depth buffer — depth texture + depth test so near hides far

## Phase 2 — Real geometry on screen
- ⬜ 13 Index buffer — draw a quad/cube from indices (DrawIndexed)
- ⬜ 14 The grid — procedural LineList ground grid, depth-aware (reference_grid_shader)
- ⬜ 15 Mesh shading — normals + a simple light model (hemisphere/key/fill) (reference_mesh_shading)
- ⬜ 16 Flat vs smooth — per-face vs interpolated normals; the FLAG_SMOOTH idea
- ⬜ 17 A real Mesh — build vertices/normals from a `session_rust::Mesh` (kernel → GPU)

## Phase 3 — Many objects, one scene
- ⬜ 18 Instancing — one mesh, many transforms via an instance buffer
- ⬜ 19 Lines as cylinders — render Line/Polyline as instanced cylinders (reference_instanced_picking)
- ⬜ 20 Points as spheres — PointCloud → instanced spheres
- ⬜ 21 Load a Session — read a `.pb`/`.json` and add every object (serde/prost)
- ⬜ 22 Scene struct — a `Scene` holding meshes/lines/points; per-object color & visibility

## Phase 4 — Picking & selection
- ⬜ 23 Screen → ray — unproject a mouse click into a world ray
- ⬜ 24 Ray-cast meshes — BVH/triangle hit test; nearest hit (reference_viewer_picking_system)
- ⬜ 25 Pick thin geometry — radius test for lines/points; solid-vs-thin priority
- ⬜ 26 Selection highlight — FLAG_SELECTED tint + click/Shift-click/box select
- ⬜ 27 Hidden-object filter — skip invisible objects in picking

## Phase 5 — Transform & edit
- ⬜ 28 Gumball geometry — draw the 3-axis gizmo (cylinders + cones + arcs) (reference_gumball_widget)
- ⬜ 29 Gumball scale — keep constant screen size; hit-test handles
- ⬜ 30 Drag to translate — axis drag math; move the selection
- ⬜ 31 Rotate + scale handles — arcs and boxes; commit transform
- ⬜ 32 Numeric entry — click a handle → type an exact value popup
- ⬜ 33 Undo / redo — command stack; restore geometry snapshots (reference_viewer_tree_undo)

## Phase 6 — Curved geometry
- ⬜ 34 NurbsCurve — evaluate + draw as a polyline
- ⬜ 35 NurbsSurface — tessellate to a mesh (deflection/angle) (reference_viewer_nurbs_brep_pipeline)
- ⬜ 36 Iso-curve boundaries — surface edges as lines
- ⬜ 37 BRep — faces + edges; transform as matrix-only (project_viewer_edge_brep_fixes)
- ⬜ 38 Trimmed surface — first-class trimmed NurbsSurface (reference_viewer_trimmed_firstclass)

## Phase 7 — Rendering quality (the "arctic" look)
- ⬜ 39 MSAA — multisampled color + resolve
- ⬜ 40 Background gradient — fullscreen horizon gradient
- ⬜ 41 Ground plane — analytic white ground with fade
- ⬜ 42 SSAO — depth-based ambient occlusion pass (project_arctic_ssao_viewer)
- ⬜ 43 Arctic mode — white/object-color ambient + AO toggle (B)
- ⬜ 44 Selection outline — screen-space coverage-mask outline + FXAA

## Phase 8 — UI & scene management (egui)
- ⬜ 45 egui overlay — wire egui-wgpu; a perf HUD (fps, draw calls)
- ⬜ 46 Settings panel — checkboxes (arctic, grid, gumball, outline…)
- ⬜ 47 Scene tree — collapsible object tree; visibility toggles (reference_viewer_tree_undo)
- ⬜ 48 Tree ↔ viewport — select in tree highlights object and vice-versa
- ⬜ 49 Text labels — billboarded glyph text in the scene (text.rs)

## Phase 9 — Sub-object editing & polish
- ⬜ 50 Control-point edit — F10 mode: move mesh verts / curve CVs (reference_viewer_subobject_edit)
- ⬜ 51 Edit points (Greville) — reshape curve/surface via refit (project_edit_points_greville)
- ⬜ 52 Snapping — grid/endpoint/plane snap markers (snap.rs)
- ⬜ 53 CAD plane / work plane — construction plane (cad_plane.rs)
- ⬜ 54 Frustum culling + perf — cull off-screen instances; batched mesh draw

## Capstone
- ⬜ 55 Load the floor model — the compas_tf demo as a first-class scene; full feature run-through

---

### Notes
- Snapshots: `NN_title/` mirrors only what's needed to build that chapter (like `03_window/`).
- Each archive feature above has detailed notes in Claude memory (`reference_*` / `project_*`)
  — pull the real implementation from `session_viewer_archive/` when writing each lesson.
- Split any lesson that grows: e.g. "10 Orbit camera" can become 10a controller + 10b input wiring.
