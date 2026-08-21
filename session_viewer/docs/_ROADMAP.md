# Viewer curriculum — roadmap

The path from "clears the screen" to a full CAD viewer, in small daily chunks. **One lesson
= one numbered day = one concept**, editing as few files as possible: file `NN-title.md`,
heading `# NN Title`, and a standalone crate snapshot `NN_title/`. Lessons aim at the
**fully-polished, archive-grade** version of each feature (not throwaway stubs).

Rule of thumb: if a lesson would touch more than ~2–3 files or introduce more than one new
idea, split it into the next numbered day. Every lesson ends with something visible on screen.

Legend: ✅ done · ▶ next · ⬜ planned. ALL 84 LESSONS WRITTEN (Phase 14 added + completed 2026-07-16). Archive feature in (parens).

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
- ✅ 18 Index buffer — draw a cube from indices (DrawIndexed): 8 shared verts + 36 u16 indices replace
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
- ✅ 21 Mesh shading — normals + a simple light model (hemisphere/key/fill) (reference_mesh_shading).
  One file (`triangle.wgsl`): flat per-face normal `cross(dpdy, dpdx)` (outward under WebGPU Y-down),
  flip on `!front_facing`, Z-up hemisphere ambient + fixed world-space key/fill. Camera-relative
  lights + `CameraUniform` deferred.
- ✅ 22 Flat vs smooth — per-vertex normal @location(1); data chooses smooth vs flat
  - files: `shaders/triangle.wgsl`, `engine/gpu.rs`
  - steps: VsIn normal @1 → VsOut @2 → fs `select(flat_n, normalize(in.normal), dot(n,n)>0.5)`;
    `meshes: Vec<Mesh>` + 2 dodecahedra, bake one via `vertex_normals()` + `set_normal` (transform
    FIRST, then bake); draw loop; invalidation contract (`invalidate_gpu` for attribute pokes)
  - verify: left dodeca faceted, right smooth, box still crisp; orbit → highlight rolls vs snaps
- ✅ 23 Mesh edges — the missing half of the "CAD look": dark edges over the shaded solid
  - **v1 on purpose**: 1px hardware LineList — storage buffers are still forbidden before 27.
    Lesson 31 migrates edges into the screen-constant-thickness cylinder path; the LineList
    pipeline is KEPT afterwards for wireframe/overlay/OBB use (the archive does exactly this —
    its `line.wgsl` serves crosshairs/OBB/wireframe only)
  - files: `shaders/edges.wgsl` (grid-style passthrough + depth nudge), `pipelines/build.rs`
    (`build_edges_pipeline`: LineList, depth-tested). NOTE: WebGPU `DepthBiasState` does NOT apply
    to line topologies — bias in the SHADER instead: `o.pos.z -= 1e-4 * o.pos.w` pulls edges a hair
    toward the camera (31's tubes won't need any of this, they protrude), `engine/gpu.rs`
  - steps: kernel `mesh.edges()` → for each (a,b) two `RenderVertex` at `vertex_point(a/b)`,
    dark color → upload once per mesh, cache beside the GpuMesh; draw AFTER solids
  - verify: box shows 12 dark edges, each dodeca 30; no shimmer/z-fight while orbiting
  - edge EXTRACTION written here is permanent (31/62/63 reuse it); only the pipeline is v1

> **Replanned 2026-07-02 (22–77 renumbered; 01–21 untouched).** Five fixes, aimed at a
> professional / efficient / good-looking CAD app with the command line as THE interface:
> **(1) Command line moved up** (was 69–71 → now 48–50): commands-only is the locked interface —
> the bus must exist before gumball/tools so every mutation is born as a command, never retrofitted.
> **(2) Undo at the first mutation** (was 52 → now 51, delete): ARCHITECTURE.md pattern (a).
> **(3) Draw tools added + snapping pulled up** (57–59): a CAD app creates geometry; `Tool` trait =
> pattern (b); snap belongs with drawing, not post-polish (was 74).
> **(4) Cheap visual wins early** (MSAA/background → 24–25, reverse-Z → 26, mesh edges → 23): the
> viewer reads "professional CAD" by lesson 26.
> **(5) Textures/PBR demoted** (old 24–28 → an optional unnumbered appendix): the CAD default look
> (shaded + edges + arctic) never uses them.

## Phase 3 — Look like a CAD app (three cheap, high-visibility lessons)
- ✅ 24 MSAA — crisp edges and lines everywhere (pulled from old 58)
  - files: `engine/gpu.rs` (4× color + 4× depth textures, rebuilt in `resize`), `pipelines/build.rs`
  - steps: `create_texture` with `sample_count: 4` (surface format + Depth32Float) → pass color
    attachment = msaa view with `resolve_target: Some(&surface_view)` → depth attachment = msaa
    depth → set `multisample.count = 4` on EVERY pipeline (triangle, grid, edges)
  - gotcha: surface itself stays single-sample; forgetting one pipeline = validation error
  - verify: grid lines and box edges smooth at glancing angles; resize still correct
- ✅ 25 Background gradient — the scene stops floating in flat grey (pulled from old 59)
  - files: `shaders/background.wgsl`, `pipelines/build.rs`, `engine/gpu.rs` (draw FIRST)
  - steps: fullscreen triangle from `@builtin(vertex_index)` (3 verts, no buffer — lesson-20/25
    trick: positions (-1,-1) (3,-1) (-1,3)); fs mixes two colors by screen y; depth state present
    (pass has a depth attachment) but `depth_write: false` + `compare: Always`, so all geometry
    paints over it
  - verify: horizon gradient behind everything; clear color never visible again
- ✅ 26 Reverse-Z — locked decision (reference_webgpu_cad_caveats); completes lesson 16's story
  - files: `engine/camera.rs`, `pipelines/build.rs` (all pipelines), `engine/gpu.rs` (clear value)
  - steps: reversed depth mapping in the 0..1 perspective — FIRST verify whether the kernel
    `Xform::perspective(fovy, aspect, far, near)` arg-swap yields it (test: project two points,
    nearer must get LARGER depth); else a tiny viewer-side matrix tweak in `math.rs`; then
    `CompareFunction::Less → Greater` everywhere (grid/edges/background `Always` stays), depth
    clear 1.0 → 0.0; ortho: swap near/far too
  - verify: no far-field z-fighting zoomed way out; near still occludes far; grid/edges unchanged

## Phase 4 — Many objects, one scene  (built for scale from the start)
Bind-group convention going forward: **0 = camera**, **1 = globals/time**, **2 = material**,
**3 = per-object**.
- ✅ 27 WebGPU-only retrofit — was a blocking footnote, now its own lesson
  - why: storage buffers/compute are FORBIDDEN by the current setup — `required_limits:
    downlevel_webgl2_defaults()` sets `max_storage_buffers_per_shader_stage = 0`;
    `backends: BROWSER_WEBGPU | GL` with Cargo `features = ["webgl"]`
  - files: `Cargo.toml` (drop `webgl` feature), `engine/gpu.rs`, `lib.rs`/`index.html` (overlay)
  - steps: `backends: BROWSER_WEBGPU` → `wgpu::Limits::default()` → `navigator.gpu` absent /
    `request_adapter` fail → show a "WebGPU required" overlay instead of a silent hang → register
    `device.on_uncaptured_error` logging
  - verify: adapter limits log shows storage buffers > 0; app runs in Chrome; overlay appears in a
    browser without WebGPU. Unblocks 29/30 (storage table), 45 (id-buffer), 76 (GPU cull)
- ✅ 28 Perf counter (console-first) — see batching collapse the draw count before it happens
  - files: new `engine/perf.rs` (frame timer, counters), hooks in `engine/gpu.rs`
  - steps: rolling-average frame ms + fps (`web_sys::Performance::now`); `draws += 1` next to every
    draw call; drawn/total objects; `log::info!` once per second (graduates to the HUD, 47)
  - verify: numbers move when meshes are added; ~3 draws today (grid, meshes, edges)
- ▶ 29 Instancing — one mesh, many transforms
  - files: `shaders/triangle.wgsl` (instance row), `engine/gpu.rs` (instance storage buffer)
  - steps: `Instance { model: [[f32;4];4], color: [f32;4], flags: u32 }` rows in a STORAGE buffer
    (needs 27) read by `@builtin(instance_index)` in vs (the locked decision — skip step_mode
    Instance, go straight to the storage table); demo 10×10 field of dodecahedra;
    `draw_indexed(.., 0..100)`
  - verify: 100 objects, ONE draw call on the 28-counter; orbit stays smooth
- ✅ 30 Batching & the GPU arena — **#1 large-scene win**; scaling successor to gpu_mesh-per-Mesh
  - files: `engine/gpu/arena.rs` (`GpuArena`: growable vbo/ibo, bump alloc, free list),
    `engine/gpu.rs`
  - steps: all meshes flatten into ONE vertex+index buffer (record base_vertex/first_index per
    object) + one instance-table row each → draw contiguous ranges in a few `draw_indexed` calls;
    grow-on-demand (copy to bigger buffer); static geometry baked once
  - verify: draw count collapses to ~1–3 for N meshes; add/remove a mesh without full re-upload
- ✅ 31 Lines as cylinders — THE linework lesson: ALL thick lines in the app, ported from the
  archive's proven design (reference_linework_rendering)
  - files: `shaders/cylinder.wgsl`, `engine/gpu/templates.rs` (unit cylinder registered once),
    adapters (`pts_to_segments`), `engine/gpu.rs` (flat segment storage buffer)
  - data: `CylinderSegment { p0:[f32;3], radius:f32, p1:[f32;3], instance_id:u32, color:[f32;4] }`
    (32 B) rows in ONE scene-global storage buffer; unit-cylinder template `SIDES` const = perf
    knob (12 → 50 verts/144 idx = 48 tris/segment; 6–8 sides fine at 1–2 px width)
  - draw: ONE `draw_indexed(0..N_CYL_INDICES, 0, 0..segments.len())` for the WHOLE scene — draw
    calls O(1), the vs aligns template +Z to (p1−p0) per instance (rotation in shader, no
    per-segment matrix)
  - thickness: SCREEN-CONSTANT pixels — camera uniform `line_thickness`; shader
    `r = thickness * depth_factor(∝ clip.w, tan_half_fov) / screen.y` (+ ortho branch);
    `radius == 0` → this default, `> 0` → world-mm override (gumball, highlights). Changing
    thickness = one uniform write, free
  - feeds: Line, Polyline, sampled NurbsCurve/arc (`pts_to_segments`, drop zero-length), and the
    23 mesh edges MIGRATE here (all/naked/crease-angle extraction; crease emits black); tubes
    protrude → 23's depth bias no longer needed
  - CONSOLIDATE 23's leftovers (deferred from lesson 23 on purpose):
    - kill the eager `edge_buffers: Vec<(Buffer,u32)>` built in `Gpu::new()` — replace with a
      **cached `Mesh::gpu_edges(&device)`** mirroring `gpu_mesh()` (lazy, rebuilt on the same
      `invalidate_gpu()`); draw loop becomes symmetric with the triangle loop, no stale parallel Vec
    - per-edge colors: 23 samples `edge_color()` (one color/mesh) because `edges()` is sorted order
      and `linecolors` is `add_face` insertion order — NOT index-aligned. Fix here with
      `edges_with_colors() -> Vec<(usize,usize,Color)>` that walks edges in linecolor order (robust
      for built meshes; note the `remove_face` desync caveat). Only then can crease/naked/selection
      edges carry distinct colors
  - bookkeeping (archive bugs, don't repeat): idempotent add = remove-first or ghost duplicates;
    `guid → Range<usize>` sub-ranges with drain-and-shift on remove; in-place
    `update_object_segments` for single-object edits
  - verify: polylines + box edges as thick tubes, ONE cylinder draw call on the perf HUD;
    thickness changes without re-upload
- ✅ 32 Points (split: 32a sphere glyphs, 32b billboard clouds) — spheres where it matters, billboards at scale
  - sphere glyphs (unit sphere 74 v/432 idx = 144 tris) for line/curve ENDPOINTS + edit handles
    only; mesh-vertex glyphs behind `FLAG_GLYPHS_HIDDEN`, hidden by default — 144 tris per vertex
    is the real iGPU cost
  - **PointCloud → screen-space SDF billboard circles** (6-vert quad, 2 tris/point, fs draws an
    anti-aliased circle — archive `point.wgsl`), NOT spheres: looks identical at point sizes,
    ~70× cheaper
  - verify: 100k-point cloud stays interactive; endpoints show as spheres; glyph toggle works;
    one draw call each (spheres, billboards)
- ✅ 33 Camera-relative rendering — locked decision, now explicit (reference_f64_f32_boundary)
  - why: f32 world positions jitter far from origin even with the f64 kernel
  - files: `engine/camera.rs`, instance-row upload in `engine/gpu.rs`
  - steps: origin = camera target (f64); view = look_at(eye−origin, target−origin, up) in kernel
    `Xform` math; every instance row's translation −= origin at upload (f64 subtract, THEN cast);
    vertices stay local + per-object matrix
  - verify: demo scene at x = 1e7 mm — orbit jitters without, rock-solid with
- ✅ 34 Load a Session (split: 34a fetch the file, 34b walk it into the tables)
  - files: `app/persistence.rs` (load half), demo hook
  - steps: fetch bytes (or `<input type=file>`) → `Session` from `.pb`/`.json` (serde/prost) →
    iterate objects → meshes/lines/points into the arena + instance table via the 31/32 adapters
  - verify: load a fixture from `session_tests/` — objects appear with correct colors/placement
  - **STRESS GATE** (user requirement): load `session_data/30700_querschnitt_gg.pb` — the real PDF
    technical drawing (`30700 Querschnitt G-G.pdf`) converted by `session_data/pdf_to_session.py`
    (lines/polylines/béziers→curves). Record perf-HUD fps + segment count; whole drawing visible
    after `F`; interactive orbit/pan at that density
- ✅ 34c Floating anchor — rebuild_instances only past REANCHOR_DIST (100m); translation-column
  rebase (no f64 matmul); view_proj_anchored; 42k objects 300ms/frame → GPU-bound
- ✅ 34d Proper CAD reader — line.proto +width/+linecolor (ONLY class missing them; gen_proto ×3
  languages, polyline is the pattern); converter reads PDF stroke color/width, page arg, --nojson;
  querschnitt = dark-red, five pen weights; Point default black ×3; floor_model linecolors patched
- ✅ 34e Many files, one wall — SceneTables + walk_session (stream: fetch→parse→walk→DROP);
  Gpu::new(&[SceneTables]); grid cells cycle files (STRESS_GRID² floor, bounds include offsets —
  fixes F); wasm max-memory 4GB (OOM "unreachable"); dev.package opt-level 3 (debug parse 5×);
  503,516 objects / 598k segments / 4 draws from 9 real drawings
- ✅ 34f Flat linework at scale — pay per pixel: CYL_SIDES 6 (free 2×), capsule ribbons (2 tris,
  ROUND caps via SDF), glyph dots (1 tri), LINEWORK_SOLID switch (cylinders/spheres kept),
  LineUniform +vp_w; paper-space lineweights for planar sheets (world lane + 1px floor) vs
  screen-constant 3D; measured 149→28ms headless / ~100→18ms real GPU on the 503k wall
- ✅ 34g Camera UX — cursor-centered zoom (target pulled by 1−k), NO zoom clamps (old 0.2–100mm
  clamp culled fitted scenes), MMB pan; anchor keeps it all rebuild-free
- 34h Colors & widths — honor what the user set (one resolution rule, CPU-side)
  - why: triangle.wgsl discards to_render's baked vertex color (pointcolors dead); FACECOLORS
    meshes render white (kernel pipe primitive ships that way — live bug); no width field ever read
  - files: `session_rust/src/render_mesh.rs` (FACECOLORS branch — Rust-only bridge, no py/cpp
    port), `gpu/adapters.rs`, `gpu/mod.rs`, all four shaders
  - steps: row color = user color (color_mode gates FACECOLORS/POINTCOLORS — auto-seeded vecs mean
    nothing); Instance.color = WHITE TINT multiplied everywhere (45 selection's channel); width =
    multiplier in the radius sign lane (0 default / neg px-mult / pos world); BRep surfacecolor
    baked via set_objectcolor on the built mesh; dots honor POINTCOLORS
  - verify: fixtures pixel-identical (defaults encode to today's bits); colors_widths.pb positive
    fixture — facecolors box, pointcolors gradient + colored dots, width-5 polyline, fat point
  - consumers: 50 selection (tint), 47 thickness slider (scales all multiplier widths), PointCloud
    lesson (point_size via Instance._pad[0]), 63 BRep per-face colors (3-language kernel change)
- ✅ 35 Scene struct — the document returns, loading stops hurting (ARCHITECTURE §2). REWRITTEN
  2026-08-12 for the post-Xform tree, ONE continuous lesson (doc + snapshot `35_scene_struct/`
  verified: wasm check clean, engine litmus empty)
  - files: `app/scene.rs` (`Scene { docs: Vec<Doc{name,place,session}>, tables, guid→row, hidden }`
    + `add_file`), `engine/gpu/mod.rs` (`ArenaUpload`, zero-copy `set_scene`, `zeroed_buffer`, pub
    row structs, stored layouts, empty-start `new()`), `app/persistence.rs` (chunked parse),
    `state.rs` (loop out), `lib.rs` (`Msg::Ready/Msg::File` loader), `Cargo.toml` (+prost)
  - steps: the WHOLE 34 walk moves into `Scene::add_file` (appends into shared tables; placement =
    manifest place × `world_xforms()`); `Gpu::new()` starts EMPTY, `set_scene` is the ONE upload
    path — zero-copy `write_buffer` lane splice (WebGPU zero-init); parse sliced 25k objects per
    `setTimeout(0)` macrotask on the kernel's pub `from_proto` split (`Rc` keeps Session off
    workers — slicing IS the fix); first sheet ~2-3 s + camera auto-fit, rest stream in, UI live
  - verify: drag DURING load (no freezes); 3 colors_widths boxes separated; paper pen weights;
    `grep Session|Mesh|BRep src/engine/` empty
    is empty (litmus test)

- ✅ 35 (part 2) The mesh edge lane — a pen that survives the surface it draws on. ADDED 2026-08-18.
  Mesh edges were 3D tubes: 12 tris an edge + 36 per vertex sphere = 90x the geometry decorated.
  Flat camera-facing quads are 2 tris, and the whole lesson is why that is hard: a flat quad is a
  PLANE through the edge, so at a convex edge it cuts into the wedge the two faces form
  - files: `engine/gpu/mod.rs` (`LineStyle`, `CylinderSegment` 48→40 B, `Instance.extent/.spacing`,
    `eye_from_view_proj`), `app/scene.rs` (`pack_rgba`/`oct16`/`pack_facing`/`mesh_spacing`,
    adjacency in `push_mesh`), `shaders/ribbon.wgsl` + `sphere.wgsl` + `cylinder.wgsl`,
    `selftest.rs` + `examples/selftest.rs` (VIEWER_ORBIT/VIEWER_ZOOM, manifests, footprint+RSS)
  - the law: a band's depth is its CENTRELINE's, one value across the width, but the test is per
    fragment - at distance d off centre the face has risen by d*tan(theta). So the offset needed
    scales with PEN WIDTH and is unbounded; and any offset that size makes two faces meeting at an
    edge fight over a band the same width. Constant lift, relative face push, hardware slope bias,
    dpdx/dpdy slope bias, per-edge secant lift ALL sit on that trade. **A constant that needs
    tuning proves the model is wrong**
  - the fix: the ink HUGS the adjacent face. Planes built in CLIP space as `join3` of three
    transformed points (no matrix inverse; w<0 still algebraically on the plane), solved per
    fragment, `max()` against the centreline so silhouettes keep full width, back-facing planes
    skipped, epsilon DERIVED (abs + plane's ndc-z slope per px for the MSAA sample spread +
    fraction of local rise for oct16's 1.4 deg)
  - four bugs that MIMIC the depth bug: width 2x (NDC spans 2 units over vp_h px, twice over);
    half-width as a varying (trapezoid -> projective, not affine -> triangular bite on the quad
    diagonal); no near-plane clip (`abs(w)` MIRRORS a vertex behind the eye); oct16 folding both
    poles onto one code (`signum(0.0)` is 0.0) which collided with an all-zeros sentinel
  - NOT depth: at distance a dense mesh goes see-through because 104k edges + 36k markers at
    screen-constant width is more ink than pixels -> WIRE_MIN_PX / MARKER_MIN_PX density LOD; and
    the lift itself was unbounded (fraction of EYE DEPTH: exceeds a 1000 mm box at 242 m band /
    91 m marker) -> `lift_capped` at a tenth of the object AABB diagonal
  - verify: depth-on vs `VIEWER_NO_DEPTH=1` must agree on visible edges — 1804 → 12 px of 675k at
    zoom 19; marker rim 394 = 394 vs the oracle; flat vs tube 4 px = 4 px on a box edge; drawing
    sheet unchanged at 52244 ink px

- ✅ 36 The raw cloud lane — one vertex, one pixel. ADDED 2026-08-16. 35 sent clouds through
  the GLYPH lane: 3 verts and a blended ~38 px dot per point, which for a 13.8M-point scan is
  41.4M vertices and ~520M blended fragments a frame (~100 ms/frame, stalled the desktop)
  - files: `app/scene.rs` (`CLOUD_RAW_MIN`, `push_cloud` off the flat `coords()`/`colors()`
    slices), `engine/gpu/mod.rs` (`points` table, draw moved up next to the solids),
    `engine/pipelines/build.rs` (PointList + `blend: None` + depth-write), `shaders/point.wgsl`
  - steps: opaque restores early-Z, which blending had disabled; lane chosen by point COUNT so
    nothing degrades mid-orbit; first lane to actually honour `Instance::FLAG_HIDDEN`
  - verify: dense clump of single pixels, flat ink in front still visible, small clouds
    unchanged from 32b
- ▶ 37 Cloud memory — the five copies, and the tab that died. Loading 3 scans (10.6M pts) got
  the tab OOM-killed: renderer 2490 MB + gpu-process 1034 MB = 3.5 GB for a 323 MB GPU payload
  - files: `app/persistence.rs` (bytes by value + `drop` after decode), `lib.rs`, `app/scene.rs`
    (`reserve_exact`, `Scene::upload_to`), `engine/gpu/mod.rs` (`point_capacity`, append via
    `write_buffer`, `storage_buffer` off `create_buffer_init`), `state.rs`,
    `session_rust/src/pointcloud.rs` (one word: `.iter()` → `.into_iter()`)
  - steps: the FIFTH copy is wgpu's `temporary_mapping` (`webgpu.rs:1431`) - `mapped_at_creation`
    mirrors each uploaded table into the wasm heap, which is why the browser measured 1589 MB
    where a native replay of the four CPU copies accounts for 839 MB
  - verify: peak 839 → 518 MB native; ZERO `/dev/shm/.com.google.Chrome.*` mappings (was 651 MB
    in three stale buffers); `upload …ms` stops growing per file; ~110 fps at 10.6M unchanged
- ⬜ 38 Sixteen bytes a point — split positions (`array<f32>`, 12 B) and colours (`array<u32>`
  RGBA8, 4 B); the per-point `instance_id` leaves via ONE draw per cloud (`first_vertex` makes
  `vertex_index` absolute, `first_instance` lands on `instance_index`). GPU 421 → 221 MB at 14M
  - verify: picture identical, `naga` clean, bytes/point 32 → 16
- ⬜ 39 Streaming cloud — HTTP Range + a wire walk to `Session.3 → Objects.8 → PointCloud.3/.4`.
  The packed-double length prefix gives the exact point count before any payload, so both GPU
  buffers are sized once, exactly. No kernel `PointCloud` for the raw lane; `CloudSlot` (a name,
  a count, an instance row) is the whole CPU footprint
  - steps: Range not ReadableStream (split doubles/varints/headers are push-only risks); 8 MB
    slices rounded to whole points; 206 or refuse; GPU up first; `next_tick` between slices;
    empty `submit` to recycle Dawn staging
  - verify: peak becomes CONSTANT; `pb/lidar_14m.pb` (13.8M pts) becomes loadable; and
    `scenes/mixed.json` closes the series - sheets/model (prost path) INTERLEAVED with scans
    (Range path), which is what proves the MSAA flip, the draw order and the F-fit all hold
    with both kinds of data resident at once

## Phase 5 — Acceleration & culling  (BEFORE picking/scenes grow)
- ✅ 40 Scene AABB BVH — ONE broad-phase for culling, picking, and box-select
  - files: `engine/bvh.rs` (or reuse the kernel's spatial AABB tree if its API fits — check
    `session_rust` spatial_bvh/aabbtree first, don't rewrite what exists)
  - steps: node = AABB + children/leaf(object id); build median-split over per-object WORLD AABBs;
    refit on transform, insert/remove on add/delete (incremental, not full rebuild)
  - verify: `#[cfg(test)]` query box → same id set as brute force over all objects
- ✅ 41 Frustum culling — draw only what's on screen
  - files: `engine/cull.rs`, hook in the instance-table upload
  - steps: extract 6 planes from view_proj (f64, kernel math) → walk the BVH, AABB-vs-planes →
    survivors keep their instance rows (or set a culled flag the vs collapses, archive-style bit 7);
    feed drawn/total to the perf counter
  - verify: zoom in → drawn count drops; slow-orbit → nothing pops at the screen edge (test the
    "AABB intersects but center outside" case). CPU cull now; GPU compute cull is 76

## Phase 6 — Document & file sync (the `.pb` file is the source, like a real CAD app)
- ✅ 43 Reconcile (split: 38a per-object GpuArena, 38b diff by guid) — never rebuild the whole scene
  - files: `app/reconcile.rs`; extend `app/scene.rs`'s `guid → (hash, row/handle)` map
  - steps: on (re)load diff by `guid`: added → build+upload; removed → free arena range + row;
    changed (content-hash differs) → re-flatten that object only; unchanged → skip
  - verify: reload a file with 1 of N objects edited → log shows 1 changed / N−1 skipped
- ✅ 44 Save (viewer → file) — write only when something actually changed
  - files: `app/persistence.rs` (save half), dirty hooks where mutations happen
  - steps: mutation → dirty flag → debounce (~1 s) → recompute content-hash, skip if unchanged →
    `pb_dumps` → Blob download (or File System Access write); new objects get a `guid`
  - verify: one save fires after an edit burst; zero writes when nothing changed
- ✅ 45 Watch (file → viewer) — external edits flow in
  - steps: browser can't watch the FS — File System Access handle + poll `lastModified` (or a
    watcher→WebSocket bridge); on change run the 38-reconcile; **self-write guard**: ignore events
    whose hash matches your own last save
  - verify: edit the file externally → viewer updates just that object; own saves don't loop

## Phase 7 — Picking & selection
- ✅ 46 Screen → ray — unproject the mouse into a world ray
  - files: `engine/pick.rs` (`screen_to_world_ray`), `app/pick.rs` dispatch
  - steps: cursor px → NDC → inverse view_proj (f64 kernel math) → near/far points → ray;
    **use ndc_z = 0.5 for p_far in perspective** — ndc_z = 1 divides by zero at huge far/near
    ratios (project_picking_bug_fix, a real archive bug)
  - verify: click the grid → marker spawned at ray∩z=0 lands under the cursor from every angle
- ✅ 47 Ray-cast meshes — nearest hit wins
  - steps: scene-BVH broad-phase (40) → per-mesh triangle test via the kernel's cached triangle
    BVH (`ensure_triangle_bvh`) in the mesh's LOCAL frame (inverse-transform the ray, f64) →
    smallest t. WebGPU has NO sync readback → CPU ray+BVH IS the interactive path
    (reference_viewer_picking_system)
  - verify: click each object → correct guid; an occluded object never wins; `#[cfg(test)]` on a
    known ray/triangle pair
- ✅ 48 Sub-object picking — vertex / edge / face
  - steps: from the hit triangle, resolve nearest vertex (screen-px radius), nearest edge
    (point-segment distance), else the face; return `SubHit { guid, kind, key }`
  - verify: hover highlights the intended vertex/edge at several zoom levels
- ✅ 49 Pick thin geometry — lines & points are 1D/0D, rays never hit them exactly
  - steps: ray↔segment / ray↔point distance with a `pick_radius` floor in screen px
    (reference_instanced_picking); **solid-vs-thin priority**: mesh wins at equal depth
    (reference_viewer_picking_system)
  - verify: a line lying ON a mesh face → mesh wins; line alone in space → pickable within radius
  - **STRESS GATE**: click-pick + marquee on the PDF drawing (34) — the intended entity wins on
    dense linework, no freeze (BVH broad-phase holds)
- ✅ 50 Selection highlight & marquee — see what you selected
  - steps: FLAG_SELECTED bit in the instance row → vs/fs tint; click = replace, Shift+click =
    toggle; drag rectangle → 4-plane sub-frustum → BVH query (async GPU id-buffer readback is the
    later upgrade, ~5–15 ms, hidden behind async UX)
  - verify: click/shift-click behave like Rhino; marquee selects exactly the visible set
- ✅ 51 Hidden-object filter — visibility is real state
  - steps: per-object visible flag (35) respected by draw (row collapsed/culled), picking (skip),
    and marquee; `hide`/`show` groundwork for the CLI verbs (53)
  - verify: hidden object neither draws nor picks; show restores both

## Phase 8 — The interface: egui shell + THE command line (moved up)
Commands-only is the locked interface (reference_webgpu_cad_caveats). The bus lands **before**
gumball and tools, so every later mutation is born as a command — pattern (a)/(b) compose from the
start instead of being retrofitted at lesson 74 like the old plan.
- ✅ 52 egui overlay + perf HUD + first settings
  - files: `ui/mod.rs` (`build_ui`), `ui/settings.rs`, egui-wgpu/egui-winit wiring in
    `app/render.rs` + `lib.rs` (feed winit events to egui FIRST; it reports "consumed")
  - steps: egui render pass AFTER the 3D pass, same encoder; HUD window = fps / frame ms /
    draws / drawn-vs-total (graduates 28); checkboxes: grid, edges, projection; line-thickness
    slider (writes the 31 camera uniform — free, the archive's `apply_thickness` is a no-op)
  - rule: UI collects intent inside the closure into a small struct, applied to `State` AFTER it
    (can't borrow `self` mutably inside)
  - verify: unchecking grid hides it next frame; typing in egui doesn't orbit the camera
- ✅ 53 Command bus + Get-loop — THE interface arrives
  - files: `app/commands.rs` (registry: verb → factory), `app/getloop.rs` (state machine),
    `ui/cli.rs` (input line docked at the screen edge)
  - steps: `enum GetState { Idle, WaitingPoint(prompt), WaitingOption(..) }`; a running command
    asks for input, the loop feeds it a picked point OR a typed value — Rhino's point-or-option at
    one prompt; every mutation goes bus → kernel → `Session` → reconcile (43); first verbs:
    `hide`/`show`/`zoom`
  - verify: type `hide` → selection hides; unknown verb → friendly message in the CLI log
- ✅ 54 Command options & modal multi-step
  - steps: option kinds toggle/number/list registered per command, rendered in the prompt line
    (`Line (From, Snap=On):`); chained prompts (from → to) with Esc = cancel, Enter = accept
    default, one-step-back
  - verify: a two-step dummy command completes, cancels, and steps back correctly
- ✅ 55 History & autocomplete — ↑/↓ recall, Tab prefix-complete, alias table (`l` → `line`)
  - verify: ↑ recalls last command; `li<Tab>` completes; aliases dispatch
- ✅ 56 Delete + undo/redo — the FIRST scene mutation, so ARCHITECTURE pattern (a) lands HERE
  - files: `app/history/{mod,remove}.rs`, `ui/toolbar.rs`
  - steps: `trait Command { apply(scene,gpu) / revert / label }` + `History { done, undone }`
    stacks — NEVER an UndoAction enum (archive's documented dead-end); `RemoveObjects` holds
    absolute snapshots of the removed objects; `delete` CLI verb + toolbar button + Del key;
    Ctrl+Z / Ctrl+Y (reference_viewer_tree_undo)
  - verify: delete → undo restores identical guids/rows; redo repeats; HUD object count tracks

## Phase 9 — Transform & draw (every tool is a command)
- ✅ 57 Gumball geometry — the 3-axis gizmo appears (reference_gumball_widget has the full recipe)
  - files: `engine/gumball/mod.rs` (geometry + handle ids), `app/gumball_state.rs`
  - steps: 3 axis cylinders + cone tips + 3 rotate arcs + 3 scale boxes, built from kernel meshes
    into instance rows (31's templates); one stable id per handle; drawn last, depth-tested but
    depth-cleared (or compare Always) so it floats over geometry; appears at selection centroid
  - verify: select → gizmo at centroid, xyz = red/green/blue; deselect → gone
- ✅ 58 Gumball scale & hit-test — constant screen size + pickable handles
  - steps: scale = distance-based formula so the gizmo is ~90 px at every zoom
    (reference_gumball_widget has the exact tuning constants); ray→handle test picks the nearest
    handle BEFORE scene picking; hovered handle brightens
  - verify: gizmo same pixel size zoomed in/out; each handle hit-tests exactly
- ✅ 59 Drag to translate — first real transform
  - files: `engine/gumball/drag.rs` (math), `app/interaction/transform.rs`,
    `app/history/transform.rs`
  - steps: on press snapshot; drag = closest-point-on-axis delta (f64 ray math); LIVE = matrix-only
    on the instance row (no re-tessellation); release = `TransformObjects` Command with absolute
    before/after snapshots → undoable for free (56)
  - verify: motion locked to the axis; Ctrl+Z restores exactly; geometry untouched until commit
  - **STRESS GATE**: marquee a large region of the PDF drawing (34), gumball-move it — matrix-only
    update keeps the fps, undo restores exactly at that object count
- ✅ 60 Rotate + scale handles — arcs → angle about axis; boxes → (uniform) scale; same commit path
  - verify: rotation snaps visually to the arc plane; undo/redo across mixed drags works
- ✅ 61 Numeric entry — click a handle, type `500`, exact move (reuses the Get-loop input, 48;
  archive gotchas: lmb_down gate, deferred drag, Escape guard — reference_gumball_widget)
  - verify: typed value applies once, Esc cancels cleanly mid-entry
- ✅ 62 Draw tools I — ARCHITECTURE pattern (b) lands: creating geometry
  - files: `app/tools/{mod,point,line}.rs`, `app/history/add.rs`
  - steps: `trait Tool { on_click / on_move / preview / finish → Box<dyn Command> }` + a ToolHost
    slot on State — NEVER a DrawTool enum; `PointTool` (1 click), `LineTool` (2 clicks), driven by
    the Get-loop prompts (53); finish yields `AddGeometry` → undoable for free
  - verify: `line` command: click-click → line exists in the Session, undo removes it
- ✅ 63 Draw tools II — `PolylineTool` (N clicks, Enter finishes), `RectangleTool`/`BoxTool` (on
  the z=0 plane until 79's work plane); ghost preview via a transient instance row on `on_move`
  - verify: preview follows the cursor, never enters the Session until finish
- ✅ 64 Snapping — drawing becomes precise (was lesson 79 — belongs WITH drawing)
  - files: `app/snap.rs`; extend `engine/pick.rs` with nearest-candidate queries
  - steps: candidates = vertex / endpoint / grid intersection within a screen-px radius; the
    Get-loop's point acquisition consults snap FIRST; on-screen marker glyph at the active snap
  - verify: line endpoints land exactly on box corners / grid crossings; toggle via CLI option

## Phase 10 — Curved geometry
- ✅ 65 NurbsCurve — evaluate + draw
  - steps: kernel `NurbsCurve` sampled by parameter (adaptive count from span count) → polyline →
    the 31 cylinder path; `NurbsCurveTool` (N control clicks + Enter) via the Tool trait (62)
  - verify: curve renders smooth at every zoom; tool-drawn curve undoes as one Command
- ✅ 66 NurbsSurface — tessellate to a mesh (reference_viewer_nurbs_brep_pipeline)
  - steps: kernel tessellation (grid/adaptive remesh, ~24×24 archive default) → mesh WITH baked
    vertex normals → smooth shading arrives free via 22's data-driven select; cache the
    tessellation; transform stays matrix-only (re-tessellating on gumball commit was the archive's
    perf bug — project_viewer_perf_plan)
  - verify: sphere/torus surfaces read smooth; dragging a surface never re-tessellates (perf HUD)
- ✅ 67 Iso-curve boundaries — surface edges + iso lines through the 23/31 line path
  - verify: boundaries hug the surface with no z-fight (uses 23's depth bias)
- ✅ 68 BRep — faces + edges as one object; transform matrix-only (project_viewer_edge_brep_fixes)
  - verify: pick selects the whole BRep; edges + faces move together under the gumball
- ✅ 69 Trimmed surface — first-class `NurbsSurfaceTrimmed` (reference_viewer_trimmed_firstclass:
  include it in every object map — tree, picking, visibility — the archive forgot repeatedly)
  - verify: trimmed circle/cut renders the hole; picking respects the trim

## Phase 10b — GPU tessellation (compute producers for existing tables)
Study + rationale: `_GPU_TESSELLATION_PLAN.md`. The pattern, four times: a compute shader
becomes a PRODUCER for a table the viewer already draws (segment table, vertex arena) — no new
lanes downstream, CPU proxies stay for picking, f32 is the display contract, no frag_depth.
- ⬜ 69a GPU curves — de Boor in compute, one invocation per segment, rows into the shared
  segment table (verified vs kernel `point_at`: f32 rounding only)
  - verify: CPU-vs-GPU polyline diff = float noise; tight bend smooth at spans×64; L toggle works
- ⬜ 69b GPU surfaces — tensor product per (u,v) grid vertex into the arena; `mesh_q` criteria
  become the up-front density law; FD normals with the exact-derivative upgrade path named
  - verify: same-resolution CPU/GPU vertex diff = float noise; no shading delta; no add_file stall
- ⬜ 69c GPU trimming — the CDT stays CPU (sequential by construction); full-rect grid +
  per-fragment winding parity = concave + holes with zero cases; compute classifies cells
  in/out/boundary so only perimeter cells pay `discard`
  - verify: hole + concave test set matches CPU silhouettes sub-pixel; trimmed orbit costs
    the untrimmed frame
- ⬜ 69d GPU BRep — assembly only: face = 69b+69c, edge = 69a, vertex = 32a; shared-edge
  matching demoted to an EXPORT requirement (both faces clip the same curve; cracks sub-pixel
  under the drawn edge pen); `BRep::mesh()` stays the watertight truth
  - verify: boolean-result BRep matches 68's build; seam audit under edge pen; load stall drops

## Phase 11 — Rendering quality: the "arctic" GI look, engineered FAST
> **Why the redesign (2026-07-03, archive measured):** the archive's post stack costs **~200+
> texture fetches/pixel/FRAME** — SSAO 32+16 taps (mode 0 default) + 2×13-tap bilateral blur +
> ~92-tap composite (9 FXAA + an 81-tap outline box search) — ALL at full resolution, re-run
> EVERY frame (`request_redraw()` unconditional), plus 1–2 extra full-scene mask rasterizations.
> Even with arctic OFF the default outline+fxaa kept the whole chain running. That's why it
> crawled on an iGPU. The rebuild deletes the waste WITHOUT touching quality — **USER RULE:
> quality must NOT decrease while rotating/interacting.** No motion-adaptive degradation; the
> savings come from architecture: skip unchanged frames, AO at half-res, fewer-but-smarter taps.
- ✅ 70 Analytic ground + infinite grid — white ground with distance fade
  (project_arctic_ssao_viewer's analytic-plane technique: per-pixel ray∩plane in the fragment
  shader, exact `frag_depth`, horizon alpha fade; NEVER a giant world-space quad — it flickers);
  optionally upgrade the lesson-20 grid to the fragment-shader infinite grid (`fract`/`fwidth`)
  - verify: ground reaches the horizon, fades smoothly, never z-fights the grid
- ✅ 71 Render-on-demand — the single biggest perf win, and it never touches the image
  - steps: a `dirty` flag set by input/camera/scene/UI changes; dirty → draw the (always
    full-quality) frame; clean → SKIP rendering entirely (CAD apps do this; games don't);
    perf HUD (28) gains a "frames drawn/s" counter
  - the frame is IDENTICAL whether drawn during a drag or after — this lesson changes WHEN we
    draw, never WHAT we draw; the outline mask re-raster (74) stops burning battery on static
    scenes for free
  - verify: static scene = 0 frames/s on the HUD; orbit = instant response, image identical
    frame-for-frame; GPU% visibly drops when idle
- ✅ 72 GTAO — half-res, CONSTANT quality (replaces the archive's 48-tap always-on SSAO)
  - steps: one fixed-quality GTAO, same result every frame — moving or still: AO at HALF
    resolution, R16Float (AO is low-frequency — 4× fewer shaded pixels) + depth-aware upsample
    in composite; 3 slices × 6 steps (~42 taps at ¼ pixels ≈ 10/px — 5× under the archive's
    ~54/px for equal-or-better quality: GTAO per tap ≫ naive kernel per tap); STATIC IGN noise
    (per-pixel, blur-cancelable — NOT per-frame jitter, which shimmers during rotation); ONE
    5-tap depth-aware blur at half-res
  - NO temporal/motion-adaptive path — user rule: rotating must not change quality. (Idle
    super-refinement rejected for the same reason: if idle looks better, starting a rotation
    reads as a quality DROP.)
  - port from the archive (proven, don't re-derive): view-pos reconstruction via ANALYTIC
    inv_proj (`Xform::inverse` WAS affine-only — found & fixed kernel-wide in lesson 46), IGN noise,
    the tangent-plane gate (`dot(D,N) > len·0.07 + bias` — MANDATORY or grazing planes stripe),
    radius = %-of-bbox-diag clamped, R16Float against banding, MSAA depth `textureLoad` sample 0
  - output a BENT NORMAL beside AO (free byproduct of GTAO horizon search) → 68 consumes it
  - budget: whole AO stack ≈ 12 reads/px every drawn frame (vs archive ~112 for SSAO+blur), and
    66 makes idle cost ZERO
  - verify: orbit smooth on an iGPU with NO visible quality change vs standstill — screenshot a
    frame mid-orbit and at rest, diff them; no grazing-angle stripes on the ground (the gate)
- ✅ 73 Arctic + cheap global illumination — a better DEFAULT look for the same money
  - steps: arctic ambient (0.72..1.0 hemisphere) × AO upgraded to: sky visibility from the BENT
    normal (directional occlusion — creases darken toward the open sky, reads as real GI) +
    Jimenez multi-bounce approximation (one polynomial of AO and albedo — bounce light for free);
    AO micro-shadowing on the key light in the DEFAULT (non-arctic) mode too; IGN dither before
    the 8-bit swapchain; B toggle + settings checkbox (52)
  - verify: default look visibly improves (contact micro-shadow); arctic reads ≥ archive; zero
    extra texture fetches vs 67
- ✅ 74 Selection outline + AA polish — the archive look without the per-frame tax
  - steps: coverage-mask outline (4× MSAA fractional coverage, sharp 1px ramp — archive
    technique; 24's MSAA pays off here) BUT mask passes render only when dirty (71), and the
    composite outline search becomes separable (two 1×N passes) instead of the 81-tap box;
    FXAA becomes OPTIONAL and OFF by default — 4× MSAA already covers geometry edges
  - verify: outline crisp at 3 px; static scene draws nothing; selection change redraws once

## Phase 12 — Scene management UI
- ✅ 75 Scene tree — the Session's tree in a panel (reference_viewer_tree_undo)
  - steps: egui collapsible rows over `session.tree`; **virtualized** (build only visible rows —
    scales to thousands); eye icon toggles the 46 visibility flag; row order right_to_left
    (vis first) per the archive lesson
  - verify: 1k objects scroll smoothly; eye toggles match viewport
- ✅ 76 Tree ↔ viewport — select in tree ⇄ highlight in viewport; auto-reveal-on-pick (expand +
  scroll to the picked object — archive's tree_open + scroll_to_me)
  - verify: pick in viewport scrolls the tree; tree click sets FLAG_SELECTED
- ✅ 77 Text labels — billboarded glyph text (archive `text.rs`: glyph atlas + TextVertex quads)
  - verify: labels face the camera, readable at the four named views

## Phase 13 — Sub-object editing & polish
- ✅ 78 Control-point edit — F10 mode (reference_viewer_subobject_edit)
  - steps: sub-pick (48) grabs verts/CVs; gumball moves them; **partial GPU update**
    (`queue.write_buffer` only the changed vertex range) instead of full re-flatten; kernel
    gotchas: `set_cv_4d` not `set_cv` (weights), mesh edits need `invalidate_triangle_bvh`
  - verify: dragging one vertex updates just that range (perf HUD upload counter)
- ✅ 79 Edit points (Greville) — reshape curve/surface via R⁻¹ refit, weights kept
  (project_edit_points_greville); F10+modifier switches raw-CV vs edit-point mode
  - verify: curve passes through the dragged edit point; validate against the kernel refit test
- ✅ 80 CAD plane / work plane — construction plane (cad_plane.rs): set by 3 points / to-object
  (CLI command, 48); draw tools (62/63) and grid snap (64) target the active plane
  - verify: rectangle drawn on a tilted work plane lands in that plane
- ✅ 81 Advanced perf — LOD/decimation, occlusion culling, GPU compute cull + indirect draw
  (culling + batching already landed in 30/37; 27 unlocked compute)
  - verify: perf HUD before/after on the capstone scene
- ✅ 90 (optional appendix) Materials & textures — `docs/90-textures.md`. Written against the real
  post-instancing pipeline: material = **`@group(3)`**, optional per-vertex UV = **`@location(4)`**
  (group 2 / loc 3 are taken by instances / inst_id since lesson 29 — the old note below was stale).
  Generated RGBA8 checker → `write_texture` (`TexelCopyTextureInfo`/`TexelCopyBufferLayout`), group-3
  {texture, sampler} bind group threaded through `Pipelines::new`/`build_triangle_pipeline`, triplanar
  world-pos sample in `fs_main` (no UV attribute needed); variant adds `uv:[f32;2]` to `RenderVertex`
  (stride 40→48, `@location(4)`). Verified: naga-validated WGSL + wgpu-29 API cargo-check.
  The CAD default look (shaded + edges + arctic GI) does not use textures.

## Phase 14 — CAD completeness (post-capstone review, 2026-07-16; ranked by importance)
Gaps found by reviewing the finished 77-lesson plan against "what does a real CAD viewer that
people trust actually have". None block the capstone; 78–80 are the ones users notice first.
- ✅ 83 Section / clipping planes — THE missing CAD feature (AEC scenes demand cuts)
  - steps: N world clip planes in a uniform; every geometry fs gains `if (dot(p, plane) < 0) {
    discard; }` behind a plane-count uniform (0 = free); `section` command sets a plane by 3 points
    or from the work plane (80), gumball-draggable along its normal; optional: darker "cut" tint on
    back faces as a cheap cap illusion (true caps = kernel plane-splits, later)
  - picking must respect the cut: 47's local-frame cast filters hits behind active planes
  - verify: section a floor model wall — the inside reads; drag the plane through the building live
- ✅ 84 Import / export OBJ + STEP — the kernel codecs are ALREADY THERE (file_obj, file_step,
  round-tripped by minitests); the viewer only speaks .pb/.json today
  - steps: extend 34a's extension dispatch (`.obj`/`.step` → the kernel decoders) + `<input
    type=file>` open (34a's noted alternative) + `export obj|step` verbs via 44's download path
  - verify: drag a real .step in → tessellated BReps appear; export → reopens in FreeCAD/Rhino
- ✅ 85 Copy / duplicate / array — daily-use editing (trivially cheap on this architecture)
  - steps: `copy` verb = clone selection, fresh guids, offset via Get-loop point pair →
    AddGeometry (62) so undo is free; Alt+gumball-drag = drag a copy (59's skeleton, clone at
    begin_drag); `array N dx dy dz` as the loop form
  - verify: Alt-drag a beam → original stays, copy follows; Ctrl+Z removes the copy only
- ✅ 86 Layers via tree groups — CAD organization users expect (Session::add_group already exists)
  - steps: group nodes get eye/color chips in the tree (75); branch visibility = 51's set ops;
    `layer <name>` verb creates + assigns selection; new objects land on the active layer
  - verify: hide "beams" layer → all beams gone from draw AND pick; saved file round-trips groups
- ✅ 87 Measure + status bar — `probe` (54) grows up: `distance`, `angle` (3 points), `radius`
  (3 points on an arc), object info (`what`: type/verts/area from the kernel); a one-line status
  bar showing live cursor world coords + active snap + selection count
  - verify: measured beam length matches the kernel value to the digit
- ✅ 88 Developer toolbox — the workflow lesson (headless selftest + debugging)
  - steps: `cargo run --example selftest` — headless kernel+scene checks without a browser (the
    archive's proven pattern); wgpu error scopes surfaced to the CLI log instead of silent console;
    a "black screen checklist" appendix (naga validate → error scope → perf HUD counts → 28's
    draw-count sanity); CI: `trunk build` + selftest on push
  - verify: an intentionally broken shader reports IN the viewer's CLI log, not just F12
- ✅ 89 Web polish — load-time & size (the 17.5 MB stress file over a real network)
  - steps: streaming fetch with a progress bar in the CLI log line (Content-Length → %),
    `wasm-opt -O2` + release profile (`opt-level = 'z'`, `lto = true`) — measure the wasm size
    before/after in the lesson; optional: gzip/brotli note for static hosts
  - verify: cold-load the stress file on throttled 3G devtools — progress visible, no frozen tab

## Capstone
- ✅ 82 Load the floor model — the compas_tf demo as a first-class scene
  - steps: load the `.pb` (34/43), fit (15), run the full loop: pick → tree reveal → gumball →
    numeric entry → draw with snap → save → undo history — fixing whatever breaks IS the lesson
  - second acceptance scene: the PDF drawing (`30700_querschnitt_gg.pb`, 34) — the curve-heavy
    counterpart to the mesh-heavy floor model; both must pass the same loop
  - verify: every phase's "verify" line passes on BOTH real models in one session

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
3. **Touch only what's visible.** Frustum-cull via the scene BVH (40/41); same BVH = picking's
   broad-phase (47).
4. **One acceleration structure, many uses.** Scene AABB BVH for culling + picking + box select.
5. **f64 to compute, f32 to draw, near the origin** (camera-relative).
6. **Static vs dynamic.** Bake static geometry once; per-frame work only on movers.
7. **Virtualize the UI.** Build only visible tree rows / labels (71).
8. **Dedup GPU state & exploit WebGPU.** Skip redundant `set_pipeline`/`set_bind_group`; use
   storage buffers + a compute cull pass when CPU work gets tight.
Measure, don't guess: a console fps/draw-call counter lands early (29) → egui HUD (69).

### Document & file sync — how real apps do it
- **Single source of truth = the in-memory document** (`Session`), not the file; the GPU scene is
  a *view* of it. **Update = delta (43), not reload** — diff by `guid`, like virtual-DOM / asset
  hot-reload / Vite HMR / CRDT-Git. **Write only when changed (44).** **No feedback loop (45)**
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
