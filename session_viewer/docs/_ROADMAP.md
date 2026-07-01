# Viewer curriculum — roadmap

The path from "clears the screen" to a full CAD viewer, in small daily chunks. **One lesson
= one numbered day = one concept**, editing as few files as possible: file `NN-title.md`,
heading `# NN Title`, and a standalone crate snapshot `NN_title/`. Lessons aim at the
**fully-polished, archive-grade** version of each feature (not throwaway stubs).

Rule of thumb: if a lesson would touch more than ~2–3 files or introduce more than one new
idea, split it into the next numbered day. Every lesson ends with something visible on screen.

Legend: ✅ done · ▶ next · ⬜ planned. Archive feature in (parens).

## Architecture decisions (locked — see "Research sources" at the end)
- **Browser-only (wasm)** — the target is the browser canvas, forever.
- **WebGPU-only — no WebGL2 fallback.** Storage buffers + compute + GPU-driven rendering are
  available; show a **"WebGPU required" overlay** when `navigator.gpu`/`request_adapter()` is
  absent (it fails silently otherwise). Retrofit: `Cargo.toml` drop `webgl`; `gpu.rs`
  `backends: BROWSER_WEBGPU`, WebGPU limits, availability gate + `on_uncaptured_error`.
- **Commands-only interface — no end-user coding/REPL.** (Python-via-Pyodide is the only
  browser-viable live language; deferred/out of scope.) Command line (Rhino Get-loop, Phase 12)
  is *the* interface.
- **Distribute, don't smash — one module per subsystem.** Mirror the archive's engine→app→ui
  layering: `engine/gpu.rs` stays pure device/surface/pipelines; the **camera** lives in
  `camera.rs` (+ a `CameraController`), the scene in `scene.rs`, picking in `pick.rs`, commands
  in `command.rs`, etc. A feature may start small inside an existing file, but is **refactored
  into its own module** once it grows (e.g. lesson 13 extracts the camera out of `gpu.rs`).
  `State` wires the layers; lower layers never reach up.

---

## Phase 0 — Setup & skeleton  ✅
- ✅ 01 Run — dev loop: npm + trunk
- ✅ 02 Dependencies — Cargo.toml, what each crate is for
- ✅ 03 Window — winit + wgpu surface, clear to grey
- ✅ 04 Pipeline — one hard-coded triangle (shader + pipeline)
- ✅ 05 Resize — canvas size vs drawing buffer vs DPR; no stretch

## Phase 1 — A camera (clip space → the archive-grade CAD camera)
- ✅ 06 Vertex buffer — move the triangle's corners out of the shader into a GPU buffer (`bytemuck`)
- ✅ 07 Uniforms & bind groups — per-frame `time` uniform in its own bind group; pulse the triangle
- ✅ 08 MVP matrix — a `view_proj` uniform from `session_rust::Xform` (no external maths crate)
- ✅ 09 Perspective vs ortho — projection matrix; toggle the two (Space)
- ✅ 10 Orbit camera — RMB-drag to orbit, scroll to zoom (spherical yaw/pitch/distance)
- ✅ 11 Pan — Ctrl+right-drag moves the stored target along the camera's right/up axes (eye follows)
- ✅ 12 Depth buffer — depth texture + depth test (Depth32Float, clear 1.0, `Less`) so near hides
  far. NOTE: viewport **reverse-Z** (minDepth>maxDepth) is **rejected by WebGPU** (`setViewport`
  requires minDepth≤maxDepth) — true reverse-Z needs the projection matrix; deferred to lesson 16.
- ✅ 13 Camera module (architecture refactor) — extract camera state + orbit/pan/zoom + view_proj
  out of `gpu.rs` into its own `camera.rs` (`Camera`) + a `CameraController` for input; `State`
  owns the camera; `gpu.rs` returns to pure device/surface/pipelines. (the "distribute, don't
  smash" refactor)
- ✅ 14 Named views & reset — Top/Bottom/Left/Right/Front/Back/Iso as yaw/pitch presets (snap,
  switch to ortho); C resets to the home view
- ✅ 15 Fit / zoom-extents — F frames the scene (or selection) bounding box: target ← box centre,
  distance ← r / sin(half-FOV) from the bounding sphere (orientation-free); pure (min,max)→camera
- ✅ 16 Projection polish — seamless persp↔ortho (ortho half-height = `distance·tan(30°)`, derived
  from `distance` so it zooms too), **adaptive near = distance·0.001 / far = distance·100** (constant
  ratio → steady depth precision), and a **`Unit` enum (mm / m)** whose `to_meters()` scale is baked
  into `view_proj` (viewer renders in m); unit fixed in code (`new()` default / `set_unit`, no key),
  `fit` honours it, demo triangles + `SCENE_*` move to mm (reference_viewer_camera)
- ✅ 17 Quaternion turntable (archive parity) — replace spherical yaw/pitch with a **quaternion**
  orientation (Z-up turntable, no pitch clamp); `update_position()` reads right/up/fwd **straight off
  the quaternion** (no pole, no `last_right` band — see [[reference_camera_pole_handling]]); orbit =
  yaw·world_up ∘ pitch·local-right; named views are single quaternions, upright for free. Verified
  native (7 views + pole crossing + horizon level). Only `camera.rs` changes. (archive-grade camera)

## Phase 2 — Real geometry on screen
- ▶ 18 Index buffer — draw a cube from indices (DrawIndexed): 8 shared verts + 36 u16 indices replace
  the 6 flat triangle verts; `index_buffer`/`num_indices` on `Gpu`, `set_index_buffer`+`draw_indexed`
  in `clear()`; pipeline/shader unchanged (indexing precedes the vertex shader). Only `engine/gpu.rs`.
- ✅ 19 Link the kernel — draw your first real `Mesh` via `mesh.gpu_mesh(&device)` → cached
  `GpuMesh { vbo, ibo, index_count }` (flattened once by `to_render()`, f64→f32). Pipeline
  `buffers: &[RenderVertex::layout()]` (pos@0/normal@1/color@2, stride 40); shader reads color@2 vec4;
  `draw_indexed` with **Uint32**; demo = `Mesh::create_box` + `set_objectcolor`. No `cast_slice`/
  offsets/`as f32` in the viewer. `session_rust` dep already present (camera). (kernel→GPU bridge)
  GPU bridge later consolidated into `render_mesh.rs` (`gpu_mesh`/`invalidate_gpu` moved there).
- ✅ 20 The grid — second pipeline: `grid.wgsl` + `build_grid_pipeline` (LineList, `depth_write` off),
  drawn first so the solid box paints over it. **Vertexless** — no vertex buffer, no `RenderVertex`
  (`buffers: &[]`); the shader builds all 50 endpoints from `@builtin(vertex_index)` (44 grey floor +
  6 for a +X/+Y/+Z coordinate-frame: colored positive half overpaints the grey centre line). Grid in
  mm (mvp applies mm→m), 1 m cells over ±5 m. Binds group 0 only (no time). Rust picks the vertex
  count (`draw(0..50)`), the shader owns the geometry. (reference_grid_shader)
- ▶ 21 Mesh shading — normals + a simple light model (hemisphere/key/fill) (reference_mesh_shading).
  One file (`triangle.wgsl`): flat per-face normal `cross(dpdy, dpdx)` (outward under WebGPU Y-down),
  flip on `!front_facing`, Z-up hemisphere ambient + fixed world-space key/fill. Camera-relative
  lights + `CameraUniform` deferred.
- ⬜ 22 Flat vs smooth — per-face vs interpolated normals; the FLAG_SMOOTH idea
- ⬜ 23 A real Mesh — per-vertex color, smooth normals (`nx/ny/nz`), cached `triangulation` for
  n-gons are already baked into the cached `GpuMesh` by `to_render()`. Focus on the mesh shader
  (`@location(1)` normal, `@location(2)` color) + invalidation (mutate → `gpu_mesh()` rebuilds)

## Phase 3 — Materials & textures (beyond per-vertex colour)
Bind-group convention: **0 = camera**, **1 = globals/time**, **2 = material**, **3 = per-object**.
- ⬜ 24 UV coordinates — extend `RenderVertex` with `uv: [f32;2]` (kernel reads mesh `u`/`v` attrs
  in `to_render`); update `RenderVertex::layout()` (stride 40→48, uv at `@location(3)`).
- ⬜ 25 Load a texture — image bytes → `wgpu::Texture` + view + `Sampler`, uploaded once + cached.
- ⬜ 26 Textured shading — sample albedo in `fs_main` via the group-2 material; combine with the
  lesson-21 light model; toggle vertex-colour vs texture.
- ⬜ 27 Material struct — `Material { albedo, base_color, metallic, roughness, … }` + its own
  bind group (group 2); **batch by material** + texture **atlas/array** (pairs with lesson 31).
- ⬜ 28 PBR maps (optional) — normal / metallic-roughness / emissive; small PBR or half-Lambert.

## Phase 4 — Many objects, one scene  (built for scale from the start)
> **PREREQUISITE before lesson 31 — do the "WebGPU-only retrofit" first (architecture note above).**
> Storage buffers (and any compute) are **forbidden** by the current device setup: `gpu.rs` uses
> `required_limits: downlevel_webgl2_defaults()` (which sets `max_storage_buffers_per_shader_stage = 0`)
> and `backends: BROWSER_WEBGPU | GL` with Cargo `features = ["webgl"]`. Lesson 31 (storage-buffer
> instance table), 37/76 (GPU compute cull), and 45 (async GPU id-buffer) will fail device/pipeline
> creation until you: drop `features = ["webgl"]`, set `backends: BROWSER_WEBGPU`, switch limits to
> `wgpu::Limits::default()`, and add the `navigator.gpu` availability overlay. Schedule it as its own
> lesson (~30) right before instancing/batching.
- ⬜ 29 Perf counter (console-first) — log fps + frame time + **draw-call count** + drawn/total;
  build **before** lesson 31 to *see* batching collapse the draw count. Graduates to the HUD (64).
- ⬜ 30 Instancing — one mesh, many transforms via an instance buffer
- ⬜ 31 Batching & the GPU arena — one growable vertex/index **GpuArena** + a single **instance
  table** in a **storage buffer** indexed by `@builtin(instance_index)` (WebGPU-only). Collapses N
  per-object draws into a few — the real cure for CPU/driver draw-call cost (worse on the single
  wasm thread). Scaling successor to `gpu_mesh()`-per-Mesh (lesson 19). **#1 large-scene win.**
- ⬜ 32 Lines as cylinders — Line/Polyline as instanced cylinders (reference_instanced_picking)
- ⬜ 33 Points as spheres — PointCloud → instanced spheres
- ⬜ 34 Load a Session — read a `.pb`/`.json` and add every object (serde/prost)
- ⬜ 35 Scene struct — a `Scene` (scene.rs) holding meshes/lines/points; per-object color &
  visibility; **static vs dynamic split** (bake static geometry into the arena once)

## Phase 5 — Acceleration & culling  (BEFORE picking/scenes grow)
- ⬜ 36 Scene AABB BVH — top-level structure over per-object world AABBs; shared broad-phase for
  BOTH culling and picking. Rebuild incrementally as objects move/add/remove.
- ⬜ 37 Frustum culling — cull off-screen objects/instances via the scene BVH before drawing;
  feed the perf counter (drawn vs total). CPU cull now; GPU compute cull is an option later.

## Phase 6 — Document & file sync (the `.pb` file is the source, like a real CAD app)
- ⬜ 38 Document ↔ scene reconcile — on (re)load, **diff by `guid`**: added → build+upload;
  removed → free buffers; changed (content-hash differs) → re-upload that object; unchanged →
  skip. Never rebuild the whole scene. Keep a `guid → (object, hash, gpu handle)` map.
- ⬜ 39 Save (viewer → file) — **dirty flag** → **debounced** autosave that writes **only when
  the content-hash changed**, via an **atomic write** (temp + rename). New objects get a `guid`.
- ⬜ 40 Watch (file → viewer) — detect external edits → run the lesson-38 reconcile. Browser
  can't watch the FS: File System Access handle + poll `lastModified`, or a watcher→WebSocket
  bridge. **Self-write guard:** ignore watch events whose hash matches your own save.

## Phase 7 — Picking & selection
- ⬜ 41 Screen → ray — unproject a mouse click into a world ray
- ⬜ 42 Ray-cast meshes — **scene-BVH broad-phase** (36) → per-mesh BVH/triangle hit; nearest
  hit. WebGPU has **no sync readback** → **CPU ray+BVH is the interactive path** (reference_viewer_picking_system)
- ⬜ 43 Sub-object picking — point/edge/face via **per-primitive BVH** (three-mesh-bvh-style)
- ⬜ 44 Pick thin geometry — radius test for lines/points; solid-vs-thin priority
- ⬜ 45 Selection highlight & marquee — FLAG_SELECTED tint + click/Shift-click; box/marquee may
  use an **async GPU id-buffer** pass (~5–15 ms readback, hidden behind async UX)
- ⬜ 46 Hidden-object filter — skip invisible objects in picking

## Phase 8 — Transform & edit
- ⬜ 47 Gumball geometry — draw the 3-axis gizmo (cylinders + cones + arcs) (reference_gumball_widget)
- ⬜ 48 Gumball scale — keep constant screen size; hit-test handles
- ⬜ 49 Drag to translate — axis drag math; move the selection (matrix-only — no re-tessellation)
- ⬜ 50 Rotate + scale handles — arcs and boxes; commit transform
- ⬜ 51 Numeric entry — click a handle → type an exact value popup
- ⬜ 52 Undo / redo — command stack; restore geometry snapshots (reference_viewer_tree_undo)

## Phase 9 — Curved geometry
- ⬜ 53 NurbsCurve — evaluate + draw as a polyline
- ⬜ 54 NurbsSurface — tessellate to a mesh (deflection/angle) (reference_viewer_nurbs_brep_pipeline)
- ⬜ 55 Iso-curve boundaries — surface edges as lines
- ⬜ 56 BRep — faces + edges; transform as matrix-only (project_viewer_edge_brep_fixes)
- ⬜ 57 Trimmed surface — first-class trimmed NurbsSurface (reference_viewer_trimmed_firstclass)

## Phase 10 — Rendering quality (the "arctic" look)
- ⬜ 58 MSAA — multisampled color + resolve
- ⬜ 59 Background gradient — fullscreen horizon gradient
- ⬜ 60 Ground plane — analytic white ground with fade
- ⬜ 61 SSAO — depth-based ambient occlusion pass (project_arctic_ssao_viewer)
- ⬜ 62 Arctic mode — white/object-color ambient + AO toggle (B)
- ⬜ 63 Selection outline — screen-space coverage-mask outline + FXAA

## Phase 11 — UI & scene management (egui)
- ⬜ 64 egui overlay + perf HUD — wire egui-wgpu; fps, draw calls, drawn/total (graduates lesson 29)
- ⬜ 65 Settings panel — checkboxes (arctic, grid, gumball, outline…)
- ⬜ 66 Scene tree — collapsible object tree; visibility toggles; **virtualized rows** (build only
  visible nodes — scales to thousands) (reference_viewer_tree_undo)
- ⬜ 67 Tree ↔ viewport — select in tree highlights object and vice-versa
- ⬜ 68 Text labels — billboarded glyph text in the scene (text.rs)

## Phase 12 — Command line (the interface)
- ⬜ 69 Command bus + Get-loop — a command registry + a Rhino-style **Get-loop** state machine
  that accepts interactive geometry input **and** typed options at the same prompt (point-or-
  option). Every command runs through one bus → kernel → `Session` → reconcile (hides internals).
- ⬜ 70 Command options & modal multi-step — register options (toggle/number/list); chain
  multi-step prompts (Line: from → to) with Esc cancel / Enter accept / undo-step.
- ⬜ 71 History & autocomplete — command history, prefix autocomplete, alias table; dock the
  input box (egui, Phase 11) at the screen edge.

## Phase 13 — Sub-object editing & polish
- ⬜ 72 Control-point edit — F10 mode: move mesh verts / curve CVs; **partial GPU updates**
  (`queue.write_buffer` only the changed vertex range) instead of a full re-flatten
  (reference_viewer_subobject_edit)
- ⬜ 73 Edit points (Greville) — reshape curve/surface via refit (project_edit_points_greville)
- ⬜ 74 Snapping — grid/endpoint/plane snap markers (snap.rs)
- ⬜ 75 CAD plane / work plane — construction plane (cad_plane.rs)
- ⬜ 76 Advanced perf — LOD/decimation, occlusion culling, GPU-driven / indirect draw (culling +
  batching already landed in 31/37)

## Capstone
- ⬜ 77 Load the floor model — the compas_tf demo as a first-class scene; full feature run-through

---

### Precision: f64 kernel, f32 render (the Bevy split)
- The `session_rust` kernel is **f64** everywhere — `Point`/`Vector`/`Line`/`Mesh` — for modeling
  precision (NURBS, intersections, tolerances) and byte-identical C++/Python parity. Unchanged.
- The GPU wants **f32**. The kernel exposes one render bridge: `Mesh::to_render() -> RenderMesh`
  with `RenderVertex { position:[f32;3], normal:[f32;3], color:[f32;4] }` (`#[repr(C)] + Pod`).
- The f64→f32 cast happens **once**, cached: `Mesh::gpu_mesh(&device)` flattens + uploads and
  caches a `GpuMesh` (`#[serde(skip)]`, dropped on edit via `invalidate_gpu()`). Viewer binds it
  — zero conversion, zero per-frame re-upload. `wgpu` is a kernel dep (wasm:
  `fragile-send-sync-non-atomic-wasm` so `Mesh: Send`). Lessons 19/23 are the first users.
- **Camera-relative rendering (large scenes):** f32 world positions jitter far from origin even
  with the f64 kernel. Keep mesh vertices **local** + per-object transform (matrix-only); subtract
  the camera/scene origin in the **view** matrix **in f64**, casting only the small offset. Design
  into the camera section (13–17).
- Rule: f64 to *compute* (geometry, picking, transforms); f32 only the *render snapshot*.

### Performance principles (an "extremely fast" viewer, by construction)
1. **Few draws, not many.** Batch into the GpuArena + instance table (31); never one draw per
   object. Draw-call count is the CPU/driver bottleneck — worse on the single wasm thread.
2. **Upload once, reuse.** Cached f32 snapshot (`gpu_mesh`); only re-upload what changed (partial
   `write_buffer`, 72). Transforms are matrix-only.
3. **Touch only what's visible.** Frustum-cull via the scene BVH (36/37); same BVH = picking's
   broad-phase (42).
4. **One acceleration structure, many uses.** Scene AABB BVH for culling + picking + box select.
5. **f64 to compute, f32 to draw, near the origin** (camera-relative).
6. **Static vs dynamic.** Bake static geometry once; per-frame work only on movers.
7. **Virtualize the UI.** Build only visible tree rows / labels (66).
8. **Dedup GPU state & exploit WebGPU.** Skip redundant `set_pipeline`/`set_bind_group`; use
   storage buffers + a compute cull pass when CPU work gets tight.
Measure, don't guess: a console fps/draw-call counter lands early (29) → egui HUD (64).

### Document & file sync — how real apps do it
- **Single source of truth = the in-memory document** (`Session`), not the file; the GPU scene is
  a *view* of it. **Update = delta (38), not reload** — diff by `guid`, like virtual-DOM / asset
  hot-reload / Vite HMR / CRDT-Git. **Write only when changed (39).** **No feedback loop (40)**
  (self-write hash guard). Browser can't watch the FS → File System Access poll or WebSocket.

### Notes
- Snapshots: `NN_title/` mirrors only what's needed to build that chapter; commits its `Cargo.lock`.
- Each archive feature has notes in Claude memory (`reference_*` / `project_*`) — pull the real
  implementation from `session_viewer_archive/` when writing each lesson.
- Per-lesson routine (memory `feedback-user-writes-code`): author lesson → verify in a throwaway
  `_chNN_verify/` crate → generate audio → **update these ✅/▶/⬜ markers** → never edit
  `session_viewer/src` (the user writes it).
- If a lesson grows, split into the next numbered day and renumber — one concept per number, no
  letter suffixes.

### Research sources
Pass 1 (verified): wgpu Limits (docs.rs) · gfx-rs/wgpu#2053, #5545 · bevyengine/bevy#17869, #5117
· bevy rendering_summary · godot#58516 · Unity HDRP Camera-Relative · gpuweb#497/#1972/#4432/#2217
· v8 4gb-wasm · three-mesh-bvh · Rhino "Add Command Line Options" · AutoCAD command line.
Pass 2 (commands-only justification, primary-sourced): Pyodide PyO3 · webgpu_pyodide_experiments ·
protobuf.dev Rust · wgpu-py#407 · compiler-research clang-repl/xeus-cpp-lite · CadQuery CQGI.
Camera comparison: `session_viewer_archive/src/camera.rs` (quaternion turntable, last_right pole
handling, named views, ortho_scale, adaptive near, mm scale, fit_to_box) + memory
`reference_viewer_camera`. Full reasoning: `~/.claude/plans/but-why-our-mesh-linked-puddle.md`.
