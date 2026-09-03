# session_viewer - Architecture

A browser-only (wasm32, WebGPU/wgpu 29 + winit 0.30) viewer over `session_rust::Session`: a thin
shell that turns kernel geometry into GPU rows and draws them, on demand. The native harness
(`src/selftest.rs`, `examples/`) runs the same tree headless through Vulkan for numbers and
pixels. This file is the map of the tree as it is; `docs/` teaches how it was built.

## 0. The first hour - where anything is

The viewer is two halves with one line between them. Learn the line and the rest follows.

```text
  app/     knows what a Geometry IS     talks to session_rust, never to wgpu
  ─────────────────────────────────────  Upload: rows in, no wgpu type either side
  engine/  knows what a ROW IS          talks to wgpu, never to session_rust
```

The two halves are organised on DIFFERENT axes, and that is the most important fact here.

| | `app/walk/` | `engine/gpu/` |
|---|---|---|
| one file per | **geometry type** | **row format (a lane)** |
| because | a producer starts from a kernel type | a shader reads a row format |
| files | `mesh.rs` `mesh_ink.rs` `mesh_topology.rs` `brep.rs` `curves.rs` `points.rs` `frames.rs` `cloud.rs` `bounds.rs` `encode.rs` | `arena.rs` `segments.rs` `glyphs.rs` `cloud.rs`+`splat.rs`+`lod.rs` `backdrop.rs` |

The mapping is many-to-many: one `Mesh` produces triangles AND segments AND glyph points; one
`CylinderSegment` is produced by six types. What crosses is a **row**, and every row carries an
`instance_id` into the one object table (`objects.rs`) - the seam picking and selection use.

### Symptom -> file

| what you are looking at | open |
|---|---|
| a geometry type draws nothing / wrongly | `app/walk/<type>.rs` |
| the wrong pixels, right geometry | `engine/gpu/<lane>.rs` + its `.wgsl` |
| the wrong draw ORDER, or a lane not pickable | `engine/gpu/render.rs` - `scene_list` and `id_pass`, same order, same toggles |
| a pipeline / blend / depth setting | `engine/pipelines/mod.rs` - one `PipelineDesc` literal each |
| a buffer that grows, a bind group, the growth policy | `engine/gpu/buffers.rs` (`GrowBuf`: `max(need, cap*3/2)`) |
| per-object transform, tint, flags, thickness, the re-anchor, the inside test | `engine/gpu/objects.rs` + `instance.rs` |
| a per-frame uniform, the eye, the ortho height | `engine/gpu/frame.rs` |
| MSAA on or off, the sample count | `engine/gpu/targets.rs` - `samples_for` |
| adapter, surface format, device limits | `engine/gpu/device.rs` |
| a click that hits the wrong thing | `engine/gpu/pick.rs` (the id pass + readback), `app/scene.rs::resolve` |
| ink showing through a face, or cut by one | the thickness rule (section 6) - `triangle.wgsl` `push_frac`, `ribbon.wgsl` `lift_capped` |
| a cloud too sparse / too dense / wrong in ortho | `engine/gpu/lod.rs` (the walk), `splat.rs` (records), `splat.wgsl` |
| a shader struct disagreeing with Rust | `cargo xtest` - the mirror tests name the file |
| memory after a Clear | `Gpu::release` (`gpu/mod.rs`) - every lane back to one row |
| WHERE a file sits in the world | `app/manifest.rs` (`scenes/*.toml`) |
| a URL becoming documents, a reload, a streamed scan | `app/loader.rs` (wasm) - produces `Msg`, touches no GPU |
| bytes becoming a `Session` | `app/decode.rs` (whole file) / `app/stream.rs` (Range reads) |
| the deployed page not following a publish | `app/live.rs` (conditional reads, the relay flag) + `bash/lib/view.sh` |
| a drag, a wheel, a key, a finger | `app/input.rs` (`Input`), `app/touch.rs` |
| why nothing redraws / why it never stops | `State::render` + `App::request_if_needed` (`state.rs`, `lib.rs`) |
| an env switch or `?query` | `app/knobs.rs` (walk) or `engine/gpu/view.rs` (draw) - section 9 |
| a number you want to trust | `src/selftest.rs` - the harness behind every `examples/*` |

### The rules the code enforces

1. **A lane may not build or renumber an object row.** One `Instance` per guid, owned by
   `InstanceTable`; everything else holds an index. `ObjectRows` is a per-upload delta.
2. **A lane is defined by the ROW it owns** - its tables, its groups, its pipelines, its draws,
   its shaders. `render.rs` names lanes and toggles, never a buffer or a shader.
3. **The frame is an ordered list**, and the id pass is the same list under the same toggles.
4. **A producer returns its object row; it never pushes one.** `Row` (`app/walk/mod.rs`).
5. **Rows are appended, never rebuilt.** Every table on the GPU is append-only; a reload is
   `release` + append from row 0.
6. **At most four parameters.** Grouped inputs are a named struct (`MeshCx`, `StreamSlice`,
   `RecordCx`, `TextureSpec`, `Pen`, ...).
7. **No closures** unless they are the fastest way (`sort_by`, `find`, `retain`).
8. **The kernel stays f64; f32 exists only at the GPU edge.** `math.rs` holds that edge: the
   f32 `Aabb`, the f32 matrix helpers, the eye/ortho readers. They are GPU-shaped by design and
   that is why they are not kernel code (section 10).

## 1. Tree

```text
src/
  lib.rs            the shell: Msg, App (winit handler), the canvas size, run_web
  state.rs          State: window, gpu, camera, scene; append / add_streamed / clear / fit / pick / render
  camera.rs         orbit camera, named views, fit, projection toggle (f64)
  math.rs           the GPU edge: Mat4 helpers, f32 Aabb, eye_from_view_proj, ortho_half_height, FOVY_DEG
  selftest.rs       (native) the harness: SceneFile, camera knobs, load, render, pick, profile
  app/
    mod.rs          the app-side modules
    manifest.rs     Item, Manifest::parse (TOML/JSON), place, name_of
    route.rs        DATA_BASE, LOCAL_SCENE, scene_route (path / query / live), is_local_url, AUTO_GRID
    knobs.rs        walk-time env flags, read once
    fetch.rs        (wasm) get with GetOpts {no_store, revalidate, if_none_match, range}, sleep, next_tick
    decode.rs       bytes -> Session, yielding to the browser between objects
    stream.rs       the protobuf header walk: CloudFields, CloudLod, packed arrays, (wasm) range readers
    loader.rs       (wasm) boot, load_route, stream_prefix / spawn_stream_rest, the point budget, GENERATION
    live.rs         (wasm) LiveSource: conditional reads, Rc<Session> set, ntfy relay flag
    input.rs        mouse + keys + the left-click pick; touch.rs the finger gestures
    scene.rs        Scene: docs, Upload tables, object rows, streamed slots, resolve a pick
    walk/           producers, one file per geometry type (mod.rs dispatches on Geometry)
  engine/
    mod.rs
    performance.rs  now_ms, heap_mb, the perf line
    pipelines/      Target, DepthMode, ColorWrite, PipelineDesc builder, vertex layouts; layouts.rs the bind group layouts
    gpu/
      mod.rs        Gpu: the lanes, build / set_scene / retarget / resize / reset / release / set_selected
      buffers.rs    GpuCtx, GrowBuf, Template, uniform_buffer, bind_group
      device.rs     adapter / device / surface
      targets.rs    Targets (depth + MSAA colour), samples_for, TextureSpec, texture, texture_view
      frame.rs      FrameInput, FrameCx, Binds, LineUniform (48 B), CloudUniform, FrameUniforms
      view.rs       View knobs, LineStyle, knob(env, query)
      instance.rs   Instance (96 B) + flags; the Instance/LineUniform mirror tests
      objects.rs    ObjectRow, InstanceTable (rows, f64 translations, re-anchor, inside test, thickness)
      upload.rs     Upload: one file's rows for every lane, dropped after upload
      backdrop.rs   background + grid
      arena.rs      mesh faces, sheet fills, lettering (one vertex table, three index runs)
      segments.rs   pipes (solid lane) + ribbons (flat lane) over the 40 B CylinderSegment
      glyphs.rs     spheres (solid lane) + dots (flat lane) over the 48 B GlyphPoint
      cloud.rs      the point tables, the node table, Cloud {chunks}
      lod.rs        LodWalk: which octree ranges to draw, clipped to what is resident
      splat.rs      the point pass (own 1x targets), records, the resolve, the id pass
      pick.rs       Picker: the id target, the copy, the async readback
      render.rs     encode_frame: point pass -> scene_list -> id pass
      present.rs    present / render_offscreen / bench_frames
  shaders/          one .wgsl per lane draw: triangle, cylinder, ribbon, sphere, glyph, grid, background, splat, splat_resolve
```

## 2. Data flow

```text
  scene .toml ──► Manifest ──► loader (wasm) / selftest (native)
                                 │  whole file: fetch ─► decode ─► Msg::File(FileDoc)
                                 │  cloud with octree: stream_prefix ─► Msg::StreamedCloud, then CloudChunk...
                                 ▼
  State::append ──► Scene::add_file ──► walk_geometry per guid ──► Upload (rows) ──► Gpu::set_scene ──► lanes append
                                                                  └── ObjectRow per guid ──► InstanceTable
```

- A `FileDoc` carries an `Rc<Session>`: the scene keeps it for picking; the live source keeps
  the same `Rc` as its current set, so a swap re-walks unchanged files and never re-decodes.
- `Upload` is a delta: every table in it is THIS file's rows; `Scene.bases` numbers them
  globally. After `set_scene` the rows are dropped - the GPU is their only holder.
- `display_only` (manifest) or `VIEWER_DROP_SESSIONS` releases the kernel object after the
  walk: no picking into it, no rebuild, a fraction of the heap.

## 3. Frame order

`render.rs::scene_list`, in order, each line one lane and one toggle:

1. background · 2. grid · 3. faces · 4. sheet fills · 5. mesh edges (`E`) · 6. clouds (resolve) ·
7. vertex markers (`E`, markers) · 8. lines (`W`) · 9. lettering · 10. point dots (`Q`).

Everything that writes depth comes first; the blended flat ink after; lettering last so a page
paints its text over its hatching. The point lane draws BEFORE the scene pass into its own 1x
depth + colour (`Splat::prelude`), skipped while the camera, the knobs and the tables are what
they were; the resolve (6) composites it with `frag_depth` under the scene's depth test.

MSAA is 4x only when SOLID geometry (faces, pipes, spheres) is on the GPU and the canvas is at
most 4.2 Mpx; `?msaa=` forces. Ribbons, dots and splats antialias themselves.

## 4. Picking

- A left click that did not drag calls `State::request_pick(x, y)`; the next frame runs the id
  pass: the scene list again, opaque, at 1x, into `Rg32Uint` = (object row + 1, sub id + 1),
  under the SAME toggles - what a lane hides it cannot pick.
- `Picker` copies one texel, maps it asynchronously, and `poll` hands the `Pick` back a frame or
  two later; `Scene::resolve` names the document, the guid, and for a cloud the point (row-local
  index and the kernel's stable id).
- `FLAG_SELECTED` on the row tints every lane's fragments; `Escape` clears.

## 5. Object rows and the f64 anchor

- `Instance` (96 B): rotation/scale (translation zeroed), tint, flags, **thickness**, spacing.
  The translation lives in a separate 16 B/row table, rewritten in f64 relative to the camera's
  anchor whenever the target drifts a quarter of the view distance (`rebase_anchor`, throttled).
- Flags: `SELECTED` `HIDDEN` `INSIDE` `PRINT` `OPEN` `SHEET`. `INSIDE` is refreshed per frame
  for rows that drew faces only (`ObjectRow.faces`), so a pure-linework sheet costs nothing.

## 6. The thickness rule (why ink never shows through a plate)

- Faces recede along their view ray by `min(0.4 % of eye depth, 0.25 x thickness)`
  (`triangle.wgsl::push_frac`), so the wireframe drawn on them is never cut by them.
- Ink lifts toward the eye by pen half-widths - mesh wires 1.5, free linework 0.5, markers
  2.5, dots 2.0 - capped at `0.25 x thickness` (`ribbon.wgsl::lift_capped`, sphere, glyph).
- `thickness` = the object's thinnest local axis through the placement scale, floored at
  0.1 % of its diagonal (`objects.rs::thickness`). A 40 mm plate 4 m long therefore never
  recedes more than 10 mm and its back outline (at 40 mm) can never surface; a 1 m box seen
  from 2 m keeps the uncapped values, so close-ups look as before.
- A planar outline has a thinnest axis of 0: it gets no lift and relies on its plate's push,
  which is exact (the same vertices) and scale-free.
- Verified by rendering the floor model from above with the caps disabled versus enabled: the
  lines that disappear are the back outlines; close-ups differ by anti-aliasing speckle only.

## 7. Point clouds

- `cloud.rs`: three append-only tables (positions, colours, oct16 normals), the node table, and
  one `Cloud` per cloud with a **chunk list** - a whole file is one chunk; a streamed file is a
  prefix and then slices, interleaved with other files' rows, so a cloud maps its own point
  index to lane rows through its chunks. A draw with `from == 0` opens a cloud; a later `from`
  extends the cloud on the same object row.
- `lod.rs`: the walk from node 0 over the cloud's WHOLE node table (uploaded with the first
  slice), descending while a node's spacing projects wider than `lod_px`, every node clipped to
  the points resident so far. Clouds under 2 M points draw whole.
- `splat.rs`: one `SplatRecord` (160 B) per range per chunk; the shader finds its record by a
  binary search on `cum`. Targets are made on the first frame that has points, dropped on
  resize and on release.
- Units: spacing and radii are computed in metres; `ortho_h` (world mm) is converted where used.

## 8. Streaming and the live source

- A `.pb` is probed with an 8 KB Range read (`stream.rs::walk_to_coords`): a file that is one
  cloud with an octree streams; anything else is fetched and decoded whole.
- The node table is read by hopping field headers after the colours, fetching only the seven
  LOD arrays - never the normals or the point ids (380 MB on a 14 M cloud).
- The first slice is the octree's coarse prefix (2 M points, at least 250 k); the rest arrives
  in 2 M slices under a page budget of 6 M resident points (`?points=`). A `Clear` bumps a
  generation counter and resets the budget; a slice from an older scene stops itself.
- The live page re-reads `view_live.toml` and each file with `If-None-Match`; an idle poll is
  `304`s. The ntfy relay only raises a flag (`tick_ms` in memory); the conditional reads decide.
- Publishing (`bash/view_live.sh`, `view_put.sh`): curl SigV4 PUT (credentials through a
  0600 config file, never argv), verifies overlapped, then the relay is poked.

## 9. Knobs

| where | env (native) | query (wasm) | meaning |
|---|---|---|---|
| view.rs | `VIEWER_THICKNESS` | `?thickness=` | pen weight, px |
| view.rs | `VIEWER_LINE_STYLE=tubes` | `?style=tubes` | solid-lane style at start (`L` flips) |
| view.rs | `VIEWER_CLOUD_SCALE` | `?cloud=` | point size scale (`[` `]`) |
| view.rs | `VIEWER_EDL` | `?edl=` | eye-dome lighting strength, 0 off |
| view.rs | `VIEWER_LOD` | `?lod=` | octree cutoff in px, 0 = draw whole |
| view.rs | `VIEWER_MSAA` | `?msaa=` | force 4 or 1 |
| view.rs | `VIEWER_PERF` | `?perf=1` | continuous frames + the perf line |
| view.rs | `VIEWER_SPIN` | `?spin=1` | orbit every frame (a benchmark) |
| view.rs | `BENCH_NO_MARKERS` | `?nomarkers` | no vertex markers |
| knobs.rs | `VIEWER_PROFILE` | - | walk laps to stderr |
| knobs.rs | `VIEWER_DROP_SESSIONS` | - | release every kernel object after the walk |
| knobs.rs | `VIEWER_NO_EDGES` / `VIEWER_NO_DOTS` / `VIEWER_ALL_EDGES` | - | wireframe content |
| loader.rs | - | `?points=` | resident point budget |
| live.rs | - | `?live=off|url` `?poll=s` `?notify=off|url` | the live source |
| selftest.rs | `VIEWER_W/H` `VIEWER_ORBIT` `VIEWER_ZOOM` `VIEWER_VIEW` `VIEWER_ORTHO` `VIEWER_FRAMES` `VIEWER_PICK` `VIEWER_INCREMENTAL` `VIEWER_REBUILD` | - | the harness camera and modes |

Keys: `1`-`7` named views, `Space` projection, `C` reset, `F` fit, `Q` `W` `E` lanes, `L` style,
`[` `]` point size, `Esc` deselect. Mouse: right orbit, middle pan, wheel zoom, left pick.
Touch: one finger orbit, two pan/zoom.

## 10. Adding or deleting a lane

A lane is `engine/gpu/<lane>.rs` plus its shaders. It touches the tree in exactly these places:

1. its file (rows struct + size assert + `SHADERS` + lane struct + mirror test);
2. one field in `Upload` (`upload.rs`) and one in `Gpu` (`gpu/mod.rs`);
3. one line each in `Gpu::build`, `set_scene`, `retarget`, `reset`, `release`, `lane_shaders`;
4. one line in `scene_list` and one in `id_pass` (`render.rs`);
5. the producer(s) in `app/walk/` that fill its rows.

Delete those and the rest compiles. `math.rs` is deliberately NOT kernel code: everything in
it is f32 or reads a projection matrix; kernel math stays f64 in `session_rust` and is ported
to C++ and Python, which these GPU-edge helpers would only burden.

## 11. Measuring

- `cargo xtest`: the mirror tests and the stream parser tests.
- `cargo run --release --example selftest -- out.ppm scene.toml` renders headless and prints
  the non-background pixel count; `examples/bench_frame.rs` times frames; `bench_load.rs` the
  walk; `check_determinism.rs` the row bytes; `stream_decode_check.rs` the header walk.
- In the browser: `?perf=1` puts `f<n> gap <ms> enc <ms> heap <MB>` on the page. A hidden tab
  renders nothing (rAF is paused), so measure in a visible tab.
