# session_viewer — Architecture (end of lesson 50)

A browser-only (wasm32, WebGPU/wgpu 29 + winit 0.30) viewer over `session_rust::Session`: a thin
shell that turns kernel geometry into GPU rows and draws them, on demand. This is the map of the
tree as it IS after the ground refactor (lessons 44-50); `docs/_ROADMAP.md` is the map of what comes next.

## 0. The first hour — where anything is

The viewer is two halves with one line between them. Learn the line and the rest follows.

```text
  app/    knows what a Geometry IS      talks to session_rust, never to wgpu
  ────────────────────────────────────  Upload: rows in, no wgpu type either side
  engine/ knows what a ROW IS           talks to wgpu, never to session_rust
```

**The two halves are organised on DIFFERENT axes, and that is the single most important fact here.**

| | `app/walk/` | `engine/gpu/` |
|---|---|---|
| one file per | **geometry type** | **row format** |
| because | a producer starts from a kernel type | a shader reads a row format |
| files | `mesh.rs` `mesh_ink.rs` `mesh_topology.rs` `brep.rs` `surface.rs` `curves.rs` `points.rs` `frames.rs` `cloud.rs` | `objects.rs` `arena.rs` `segments.rs` `glyphs.rs` `cloud.rs` `stream.rs` `splat.rs` `backdrop.rs` |

The mapping is **many-to-many**: one `Mesh` produces triangles AND cylinder segments AND glyph
points; one `CylinderSegment` is produced by six types. What crosses is a **row**, and every row
carries an `instance_id` into the one object table (`objects.rs`) - the seam picking will use.

### Symptom -> file

| what you are looking at | open |
|---|---|
| a geometry type draws nothing / wrongly | `app/walk/<type>.rs` |
| the wrong pixels, right geometry | `engine/gpu/<family>.rs` - the family that owns that row |
| the wrong draw ORDER | `engine/gpu/render.rs` - `scene_list`, eleven lines |
| a pipeline / blend / depth setting | `engine/pipelines/mod.rs` - one `PipelineDesc` literal each |
| a buffer that grows, a bind group, the growth policy | `engine/gpu/buffers.rs` (`GrowBuf`: `max(need, cap*3/2)`) |
| per-object transform, tint, flags, the re-anchor, the inside test | `engine/gpu/objects.rs` + `instance.rs` |
| a per-frame uniform, the eye, the ortho height | `engine/gpu/frame.rs` |
| MSAA on or off, the sample count | `engine/gpu/targets.rs` - `samples_for` |
| adapter, surface format, device limits | `engine/gpu/device.rs` |
| a shader struct disagreeing with Rust | `cargo xtest` - five mirror tests name the file |
| memory after a Clear | `Gpu::release` (`gpu/mod.rs`) - every lane back to one row |
| WHERE a file sits in the world | `app/manifest.rs` (`scenes/*.toml`) |
| a URL becoming documents, a reload, a streamed scan | `app/loader.rs` (wasm) - produces `Msg`, touches no GPU |
| bytes becoming a `Session` | `app/decode.rs` (whole file) / `app/stream.rs` (Range slices) |
| a drag, a wheel, a key | `app/input.rs` - `Input` is the gesture state machine |
| why nothing redraws / why it never stops | `State::render` + `App::request_if_needed` (`state.rs`, `lib.rs`) |
| an env switch | `app/knobs.rs` (walk) or `engine/gpu/view.rs` (draw) - the table below |
| a number you want to trust | `src/selftest.rs` - the harness behind every `examples/*` |

### The five rules the code enforces

1. **A family may not build or renumber an object row.** One `(model, tint, flags)` per guid, owned
   by `InstanceTable`; everything else holds an index. `ObjectRows` is a per-upload delta.
2. **A module is defined by the ROW it owns** - its tables, its pipelines, its draws. Not by a
   shader: `ribbon.wgsl` is compiled once and drawn from two tables.
3. **The frame is an ordered LIST.** `scene_list` is eleven entries; no entry reaches past its own
   family. `grep -cE 'wgpu::Buffer|\.wgsl' engine/gpu/render.rs` is 0.
4. **A producer returns its object row; it never pushes one.** `Row` (`app/walk/mod.rs`).
5. **An option the caller must decide is a named field.** `MeshOpts`, `FrameInput`, `RecordCx` -
   no fn takes more than four parameters, no closure holds logic, every fn carries a `///`.

### The gates

```bash
cargo check --lib --target wasm32-unknown-unknown            # 0 warnings in this crate
cargo check --all-targets --target x86_64-unknown-linux-gnu  # 0 warnings in this crate
cargo xtest                                                  # 5 Rust<->WGSL mirror tests
./docs/_gate.sh                                              # 4 scenes x 4 configs, twice, vs docs/_GOLDENS.tsv
python3 docs/_replay_check.py --render docs/*.md             # the lessons render as written
```

### The knobs (native `VIEWER_*` env; wasm `?name=` query where named)

| knob | where | what |
|---|---|---|
| `?scene=` | loader | manifest path under `assets/`; default `scenes/bunny_drawings.toml` |
| `?perf=1` / `VIEWER_PERF` | performance.rs, `State::render` | once-a-second fps line AND continuous rendering (benchmark mode) |
| `?thickness=` / `VIEWER_THICKNESS` | view.rs | pen weight, px (default 2) |
| `?msaa=` / `VIEWER_MSAA` | view.rs, targets.rs | force 4 = 4x, anything else 1x; unset = policy |
| `VIEWER_LINE_STYLE=tubes`, `BENCH_NO_MARKERS` | view.rs | solid-lane style; no vertex markers |
| `VIEWER_CLOUD_SCALE`, `VIEWER_EDL`, `VIEWER_LOD` | view.rs | cloud size factor, EDL strength, LOD split px |
| `VIEWER_PROFILE`, `VIEWER_DROP_SESSIONS`, `VIEWER_NO_EDGES`, `VIEWER_NO_DOTS`, `VIEWER_ALL_EDGES` | knobs.rs | walk laps; force display_only; ink gates |
| `VIEWER_NO_DEPTH` | pipelines/build.rs | both solid-ink colour passes with depth Always |
| `VIEWER_W/H`, `VIEWER_ORBIT`, `VIEWER_ORTHO`, `VIEWER_VIEW`, `VIEWER_ZOOM` | selftest.rs | canvas and camera of a harness frame |
| `VIEWER_FRAMES=N` | selftest.rs | N still-camera frames, median ms (the splat static skip applies) |
| `VIEWER_INCREMENTAL=1`, `VIEWER_REBUILD=1` | selftest.rs | upload per file (the browser's path); re-walk from the kernel |
| `VIEWER_GPU_REPORT=1`, `VIEWER_CLEAR=1` | selftest.rs | wgpu allocator report per label; clear after the frame and report again |
| `BENCH_FRAMES` | bench_frame / bench_lines | frames per leg |

### Deliberately left (audit section B - known, measured, not fixed here)

- Kernel decode is 75-79% of native load (prost 50%); `Mesh` HashMap-of-HashMaps costs 61 B/vertex;
  `SpatialOctree` build is 80% of a cloud walk. Kernel work, three languages.
- Solid ink drawn twice (depth prepass + colour) = 31% of the flat bunny frame: the prepass keeps
  the AA rim free of flecks; alpha-to-coverage changes pixels.
- Sheet ribbons fetch the instance row five times per vertex; a `step_mode: Instance` table is a
  shader-side redesign. `blend: ALPHA` on the opaque triangle pipeline (~1 ms on sheets) stays.
- The splat colour pass re-projects every point; per-thread linear record search (`splat.wgsl`).
- The background draw duplicates the pass clear (0.1 ms); deleting it changes the `draws` golden.
- `GlyphPoint` carries 12 B the flat lane never reads; `stream.nrm` is 53 MiB of sentinel on lidar_14m.
- `drawings_rotated` (4 sheets of similar size) gains nothing from the 3/2 growth policy: no append
  fits the slack; `drawings` (10 sheets) gains 25%. The policy is one, measured on both.
- The harness measures an Intel iGPU under `PowerPreference::LowPower`; numbers under CPU load
  disagree by up to 2x, so every number in `CHANGES.md` carries its load average and both runs.

## 1. The tree (lines measured on this stage)

```text
src/lib.rs 165        wasm shell: App (ApplicationHandler), Msg, request_if_needed - the ONLY request_redraw sites
src/state.rs 101      State { window, gpu, camera, scene, needs_frame } - render() draws once, never requests
src/math.rs 159       Mat4, Aabb, eye/ortho solves       src/camera.rs 356   orbit/pan/zoom, named views, fit
src/selftest.rs 435   native harness: render_scene, bench_scene, frame_profile (+ rebase leg), object_bytes, gpu_report
src/app/  manifest 71 · knobs 46 · fetch 82 · decode 144 (LeanSession for display_only) · stream 152 (ColorRun)
          loader 247 (wasm: whole files + 8 MiB Range slices) · input 121 · scene 227 (Scene, Bases, Rc<str> guids)
src/app/walk/  mod 122 (Walk, WalkCx, Row) · bounds 157 (Baselines, sweeps) · encode 79 · mesh 189 · mesh_ink 200
          mesh_topology 128 · brep 16 · surface 17 · curves 86 · points 21 · frames 51 · cloud 114
src/engine/pipelines/  mod 146 (16 PipelineDesc literals) · layouts 116 (8 layouts; instance = rows + translations) · build 183
src/engine/gpu/  mod 274 (Gpu, 17 fields: set_scene, retarget, release) · device 96 · buffers 138 (GrowBuf, Template)
          upload 74 · view 86 · frame 188 · targets 106 (samples_for) · present 136 · instance 196 (+5 mirror tests)
          objects 288 (InstanceTable, BoundedRow, Rebase) · arena 137 · segments 222 · glyphs 176
          cloud 113 · stream 168 · splat 422 · backdrop 23 · render 141 (splat_prelude + scene_list)
src/engine/performance.rs 110   fps line, frame counter, perf_logging()
src/shaders/  triangle 136 · cylinder 186 · ribbon 522 · sphere 290 · glyph 174 · grid 96 · background 28 · splat 276 · splat_resolve 98
examples/  selftest · bench_frame · bench_lines · bench_load · probe_mem · probe_objects · check_determinism · mk_* · pb_bbox · potree_import
```

## 2. The frame (`engine/gpu/render.rs`) - the ORDER is the contract

```text
compute prelude   splat.is_current(mvp, cloud_size)? -> nothing. Else: records for both lanes into
                  the reused tables -> write, clear pixels, 2 lanes x 2 passes (narrow 2D grid)
render pass       1 background   2 grid   3 arena.draw_faces (counts 1 even when empty)
                  4 arena.draw_print   5 segments.draw_pipes (Tubes 1 · Flat prepass+colour 2)
                  6 splat resolve   7 glyphs.draw_spheres (prepass+colour 2) if show_mesh_edges && markers
                  8 [INK_DEPTH_PREPASS - const false]   9 ribbons if show_lines   10 arena.draw_text   11 dots if show_points
```
The `draws` count and the object count are what `docs/_GOLDENS.tsv` records. Bind groups: 0 = mvp,
1 = line/cloud uniform, 2 = instances (rows at 0, anchored translations at 1), 3 = the family's rows.

**Render on demand.** `State::render` clears `needs_frame`, draws one frame, and sets it again only
when `?perf=1`/`VIEWER_PERF` (continuous, for benchmarking) or a throttled re-anchor is still due.
`App` requests a redraw only when `needs_frame` is set: every `Msg`, every input that changed the
camera or a knob, `Resized`. A still scene draws nothing; `frames drawn: N` logs every 60th frame.

## 3. Memory - what lives where

- **CPU, per document:** the kernel `Session` unless `display_only` (every shipped sheet and cloud
  item is); then only the name and placement. `display_only` also skips `tree`/`graph`/`bvh` on the
  wire (`LeanSession`). The fetched bytes are dropped the moment prost is done.
- **CPU, per object:** `Instance` 96 B + `[f64; 3]` translation 24 B in `InstanceTable`, one
  `Rc<str>` guid shared by `Scene.order` and `guid_to_row` - 303 B measured (`probe_objects`), was 997.
  Bounded rows (meshes that drew ink) are a sparse list: 32 B each, 3 rows on a ten-sheet scene.
- **CPU, per upload:** `Upload` = one file's rows for every family, a DELTA, dropped by
  `drop_uploaded` after `set_scene`. The walk numbers rows from `Scene.bases` (vert, cloud, obj).
- **GPU:** every lane is a `GrowBuf` under one policy, `max(need, cap*3/2)`, prefix copied
  GPU-side; `StreamLane` reserves exact (the count is known before the first byte). Targets:
  4x only with solid geometry (faces, pipes, spheres) AND <= 4.2 Mpx, no `msaa` texture at 1x.
  `Gpu::release()` (from `Scene::clear`) puts every lane back to one row: 132 -> 26 MiB on drawings_rotated.
- **Re-anchor:** `Instance.model` holds rotation/scale with a zero translation column; the
  anchored translation is a 16 B/row buffer rewritten on re-anchor (11 MiB on 744k rows, was 68);
  an inside-flag flip writes its one 96 B row. Throttled to 5/s; a deferred one asks for a frame.
- **Streaming:** coords in 8 MiB slices (whole points), colours in 8 MiB slices with the split
  varint carried by `ColorRun`; one `Msg` per slice, nothing whole in wasm memory.

## 4. Precision boundary - f64 kernel, f32 GPU

The kernel is f64; the GPU is f32; the cast happens once at the upload edge (`mat_to_f32`, the row
structs) and once per frame for the camera (`Xform::to_f32`). Large scenes stay camera-relative:
rows are rebased about an anchor in f64 and only the small offset is cast. Never `as f32` inside
a computation that feeds more math.

## 5. What lesson 50 measured (both runs, load average in CHANGES.md)

| item | before | after |
|---|---|---|
| render on demand | a still `drawings` scene: 85-106 ms every frame, forever | 0 frames while still |
| per-object CPU bytes (drawings_rotated) | 997 B | 303 B |
| MSAA on a pure sheet (900x700) | 6.05 ms, 10.7 MiB msaa + 10.2 depth | 3.45 ms, 2.9 MiB depth |
| MSAA on bunny at 3840x2160 | 13.1 ms, 135 + 131 MiB | 5.7 ms, 36 MiB |
| forced re-anchor, 744k rows | 21.7 ms CPU + 6.4 ms GPU (68 MiB) | 10.7 + 1.6 ms (11.4 MiB) |
| display_only sheet decode | 194 ms | 125 ms |
| drawings_rotated resident (5 files) | 684 MB | 285-377 MB |
| 10-sheet incremental upload | 1883 ms | 1424 ms |
| Clear on drawings_rotated (GPU) | 132 MiB stayed | 26 MiB |
| streamed colours (lidar_14m) | 148 MB transient | 8 MiB slices |

## 6. Quick reference

- Build target pinned to `wasm32-unknown-unknown` in `.cargo/config.toml`; `cargo xtest` and the
  examples are native (`--target x86_64-unknown-linux-gnu`). `trunk serve --release` to run.
- Layer rule: `app/` may name `session_rust`; `engine/` names only `RenderVertex`, `Xform`, `Point`.
- File rule: one concern per file, `//!` header, `///` on every fn, no fn over four parameters.
- Every number in a lesson states the load average and both runs; a gate row is a number measured
  on SPECIFIC asset bytes (`# assets:` fingerprint in `docs/_GOLDENS.tsv`).
