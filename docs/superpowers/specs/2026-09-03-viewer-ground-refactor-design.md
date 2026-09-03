# session_viewer ground refactor #2 — design (2026-09-03)

Scope: `session_viewer/` (Rust, wasm32 + native harness), `bash/view_*.sh`, and a fresh
`session_viewer/docs/` curriculum. The kernel (`session_rust` and its two ports) is touched only
where a NEW pure-math helper is needed; none is planned (the kernel already has `AABB`,
`Line: Index`, `PointCloud::lod_*`). The viewer's `math.rs` is the f32 GPU edge (the f32 box
the rows carry, the f32 matrix helpers, the projection readers) and stays in the viewer on
purpose: it is GPU-shaped, not kernel geometry, and porting it to C++/Python would burden the
kernel with helpers only a renderer reads (decided 2026-09-03 after review).

## 1. Goals

- Every render lane is one module that can be deleted by removing its file, its `Upload` field,
  its walk producer and its line in the frame list. No lane reaches into another.
- Every function takes at most four parameters; grouped inputs are named structs; every `fn` has a
  `///`; no closure carries logic; files stay under ~300 lines; blank lines separate ideas.
- Same pixels for the gate scenes EXCEPT the declared changes in §6.
- Performance and memory fixes from the audit and from this reading, measured before/after.
- The floor-model penetration bug fixed (§6.1).
- R2 publish → page swap measured and cut (§7).
- Seams for picking, selection, CLI/HUD, tree, gumball, post-processing named in code and in
  `ARCHITECTURE.md`; object picking implemented as the first of them (§5).

## 2. Layering

```
lib.rs        wasm shell: App (winit), Msg dispatch          talks to State only
state.rs      State { window, gpu, camera, scene, needs_frame }
app/          knows what a Geometry IS (session_rust), never wgpu
  route.rs    the URL → where the scene comes from (local / named / live) + query knobs
  manifest.rs Item / Manifest / auto_grid
  fetch.rs    fetch, conditional fetch (ETag), Range fetch, sleep/next_tick     [wasm]
  decode.rs   bytes → Session, chunked with yields                              [wasm]
  stream.rs   protobuf header walk: CloudFields, CloudLod, colour slicing (pure, tested native)
  loader.rs   boot(): live | named | local; whole files; streamed clouds; budget [wasm]
  live.rs     LiveSource: R2 poll with If-None-Match + ntfy wake                 [wasm]
  input.rs    mouse + keys → bool "redraw"        touch.rs finger gestures
  scene.rs    Scene: docs, Upload tables, order/guid_to_row/hidden, streamed slots, bases
  walk/       one producer per geometry type → rows (mod, bounds, encode, mesh, mesh_ink,
              mesh_topology, brep, surface, curves, points, frames, cloud)
  knobs.rs    native env switches read once
engine/       knows what a ROW IS (wgpu), never session_rust beyond RenderVertex/Xform/Point/AABB
  pipelines/  mod.rs: PipelineDesc, DepthMode, Target, build(), module(); layouts.rs: Layouts
  gpu/mod.rs  Gpu = floor + lanes; set_scene, resize, reset, release, retarget
  gpu/device.rs buffers.rs (GpuCtx, GrowBuf, Template) frame.rs (uniforms) targets.rs
  gpu/present.rs (swapchain clear, offscreen, bench, perf overlay) view.rs (knobs) upload.rs
  gpu/instance.rs objects.rs (InstanceTable: rows, f64 translations, rebase, inside test)
  gpu/render.rs the frame list (ONE ordered list of lane draws)
  gpu/pick.rs   id pass on demand: R32Uint object + sub-object targets, async readback
  gpu/lanes: backdrop.rs arena.rs segments.rs glyphs.rs cloud.rs splat.rs
```

A lane file owns: its row struct(s) + size assert, its `GrowBuf`s and bind group, its shader
module and pipelines (rebuilt by `retarget(Target)`), `append`, `draw_*` (returns draw count),
`draw_ids` (the id pass), `reset`, `release`. The shared bind-group scheme for every draw:
0 = mvp, 1 = line/pen uniform, 2 = instances (rows + anchored translations), 3 = the lane's rows.

## 3. Data flow

```
manifest ──fetch──▶ bytes ──decode──▶ Session ──walk──▶ Upload (deltas) ──set_scene──▶ lanes
                        └─stream (Range)─────────▶ CloudRows chunks ─────────────────▶ cloud lane
camera ──▶ FrameInput ──▶ FrameUniforms (mvp, line, cloud) ──▶ render.rs frame list ──▶ surface
```

- `Upload` = `{ obj: ObjectRows, arena: ArenaRows, seg: SegRows, glyph: GlyphRows, cloud:
  CloudRows, bounds: AABB }` — every table is THIS file's delta; `Scene::upload_to` drops them.
- `Scene.bases` = rows already uploaded (vert, cloud, obj); the walk numbers rows from them.
- `InstanceTable` is the only owner of object rows: `Instance` (96 B, zero translation column) +
  `[f64; 3]` true translation + sparse `BoundedRow`; a re-anchor rewrites the 16 B/row
  translation buffer only. `Rc<str>` guids shared by `order` and `guid_to_row`.
- Streamed clouds: `CloudRows` chunks appended to the cloud lane; `Scene.streamed[idx]` keeps the
  slot (instance row, node base, lod table, done_to, total).

## 4. The frame (render.rs), in order

1 background · 2 grid · 3 arena faces · 4 arena print fills · 5 pipes (tubes 1 / flat 2)
· 6 splat resolve · 7 sphere markers (2) · 8 [ink depth prepass, const off] · 9 ribbons
· 10 lettering · 11 dots. Before the pass: the point pass (splat) into its own 1x targets, only
when `(mvp, cloud_size, lod)` changed. Render on demand: `State::render` never requests the next
frame; `?perf=1` / `?spin=1` keep it continuous.

## 5. Picking (implemented) and the other seams (named)

- `pick.rs`: on `request_pick(x, y)` render an ID PASS at 1x into `object: R32Uint` +
  `sub: R32Uint` + depth, every lane drawing with its `fs_id` entry (object = instance row + 1;
  sub = point row / 0). One 8-byte copy, async map, `poll_pick()` next frame → `Pick { row,
  sub }` → `Scene::resolve(pick)` → `Picked { doc, guid, instance, point: Option<..> }`.
  Left click picks; the picked row gets `FLAG_SELECTED` (bit 0) and the shaders tint it.
  Replaces the per-frame id target of the point pass (bandwidth every moving frame → only on click).
- Seams (documented in ARCHITECTURE.md, not built): HUD/CLI (`ui/` module consuming
  `WindowEvent` before `Input`), tree/layers (`Scene.order` + `hidden` + `FLAG_HIDDEN`), gumball
  (a lane drawn after 11 with depth Always, drag through `Scene.docs[i].place`), post-processing
  (extra targets in `targets.rs`, passes after the list), control points (`Doc.session` is kept
  for non-display_only files; `Scene::rebuild` re-walks after an edit commit).

## 6. Declared pixel changes (goldens re-recorded with the reason)

6.1 **Penetration fix - the thickness rule.** Every object row carries `thickness`, measured
by the walk orientation-free (a mesh: the smallest spread along its own dominant face normals;
a polyline: the spread across its plane), through the placement scale, floored at 0.1 % of
its diagonal - a local-AABB axis was 2.4x too thick (median) on the floor model's rotated plates.
`triangle.wgsl` pushes faces back by `min(0.4 % of eye depth, 0.25 x thickness)`; the
ribbon/sphere/glyph lifts (1.5 / 0.5 / 2.5 / 2.0 pen half-widths, the same number in both
projections) are capped at `0.25 x thickness`. A 4 m x 40 mm plate never recedes more than
10 mm, so its back outline (40 mm behind) cannot surface at any distance; a 1 m box seen from
2 m keeps the uncapped values. A diagonal-based cap (the first attempt) was useless for long
thin plates. Linework rows carry local bounds; a planar outline has thickness ~0, no lift, and
relies on its plate's exact push.
6.2 **MSAA policy**: 4x only with solid geometry (faces / pipes / spheres) and ≤ 4.2 Mpx;
`?msaa=4|1` override. Pure sheets render at 1x (they antialias in the shader).
6.3 **Translation split**: rows carry a zero translation column; the anchored translation is
added in the shader. 1-ulp differences possible.
6.4 **Point pass** loses its id target (colour only); picking reads the id pass.
6.5 `LineUniform` gains nothing; the `time` uniform and `edges.wgsl` are deleted.

## 7. R2 turnaround

Now: `aws s3 cp` (~1 s Python start) → curl HEAD verify → ntfy POST; page: EventSource →
flag → 500 ms tick → conditional GET manifest (304) → conditional GET each file (full body on
change, DISCARDED) → `load_all` GETs every file AGAIN → decode → swap.
Target: (a) publisher uses `curl --aws-sigv4` when available (no Python start), HEAD verify
kept; (b) page tick 100 ms; (c) the bytes a conditional GET returned are the bytes loaded (one
download); (d) unchanged files are not re-fetched by `load_all` — only changed ones are decoded,
the rest are re-used from the previous documents. Measured before/after with a stopwatch in the
browser log (publish start → `Msg::Fit`).

## 8. Performance / memory list (measured before/after in `docs/_PERF.md`)

1. render on demand; 2. object CPU mirrors 1 KB → ~150 B; 3. MSAA policy; 4. re-anchor rewrites
16 B/row; 5. drop raw bytes right after prost decode; 6. one `GrowBuf` policy `max(need, cap*3/2)`;
7. splat records built only when the key changed; 8. dead `time` uniform / `instances_unused` /
`edges` pipeline / egui deps removed (wasm size measured); 9. `Line` read through `Index`, no
`Point` allocations; polylines read `coords` directly; 10. shaders compiled once per source;
11. `Scene::clear` releases GPU buffers and CPU mirrors; 12. cloud normals table only for clouds
that carry normals (per-record `nrm_first`, sentinel table gone: −4 B/point on scans);
13. `View` knobs read once; 14. live lane single download.

## 9. Docs (fresh)

- `docs/` archived to `docs_archive/` (git mv; untracked build junk deleted).
- New `docs/`: `README.md` (how to read, legend ✍ type / 📋 paste), `00-architecture.md`
  (SVG diagrams: layers, data flow, frame list, one lane anatomy, picking), lessons 01..NN each
  = one compilable stage, ops in the checker's grammar, `_replay_check.py` + `_gate.sh` kept,
  `_stages/` end-of-lesson trees for the checker, `_PERF.md` measurements.
- Stage trees are derived from the final tree by removing features; every stage compiles on
  wasm and native and renders in the browser.
