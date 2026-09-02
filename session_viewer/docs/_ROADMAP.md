# Viewer curriculum — roadmap

One lesson = one numbered file `NN-title.md` in this folder, one idea each, followed by typing
the code into `session_viewer/src`. Every lesson from 34c on is replayable: `docs/_replay_check.py`
applies its edits to the previous tree and the result must match the next snapshot byte for byte.

Legend: ✅ typed · ▶ next to type · ⬜ written, not yet typed.

## Phase 0-3 — a window, a camera, real geometry, the CAD look (01-33) ✅

01 run · 02 dependencies · 03 window · 04 pipeline · 05 resize · 06 vertex buffer · 07 uniforms ·
08 mvp · 09 projection · 10 orbit · 11 pan · 12 depth · 13 camera module · 14 named views · 15 fit ·
16 projection polish · 17 quaternion camera · 18 index buffer · 19 link the kernel · 20 grid ·
21 shading · 22 flat vs smooth · 23 edges · 24 MSAA · 25 background · 26 reverse-Z · 27 WebGPU only ·
28 performance · 29 instancing · 30 batching · 31 edges as cylinders · 32a point spheres ·
32b point clouds · 33 camera-relative rendering.

## Phase 4 — files, scenes, clouds (34-45)

- ✅ 34a-34h load a file, walk a session, many files, flat linework, camera UX, colours and widths
- ✅ 35 the scene struct
- ✅ 36-43 the cloud lane: tables, memory, append-only, big scenes, compute splatting, the Potree
  look, normals, scenes
- ▶ 44 streaming a cloud by HTTP range · ⬜ 45 the octree LOD (`docs/45_cloud_octree/` is the end state)

## Phase 5 — the ground refactor (46-51) ⬜

Same viewer, same pixels, split into ~45 files with a struct for every group of inputs, a
docstring on every function, almost no closures, and a measured performance pass. `Gpu` goes
from 116 fields to 17; `gpu/mod.rs` from 2447 lines to 259; `scene.rs` from 1382 to 212;
`lib.rs` from 522 to 150. ARCHITECTURE.md describes the result.

| lesson | idea | creates | `Gpu` |
|---|---|---|---|
| 46 pipelines are data | one `build`, fourteen `PipelineDesc` literals, `Layouts`, `math.rs` | `pipelines/{layouts,build}.rs`, `math.rs` | 116 → 102 |
| 47 the GPU floor | `GpuCtx`, `GrowBuf`, `Upload`, `View`, `FrameUniforms`, `Targets`, present | `gpu/{device,buffers,upload,view,frame,targets,present}.rs` | 102 → 64 |
| 48 row families | one file per row format, draws return their count, 4 mirror tests | `gpu/{instance,objects,arena,segments,glyphs}.rs` | 64 → 39 |
| 49 point lanes and the frame list | `SplatRecord`, two lanes, `scene_list` | `gpu/{cloud,stream,splat,backdrop,render}.rs` | 39 → 17 |
| 50 the walk and the shell | one producer per geometry type, narrow sinks, `Row`; loader/input/fetch/decode/stream | `app/walk/*`, `app/{manifest,knobs,loader,input,fetch,decode,stream}.rs` | 17 |
| 51 performance and memory | render on demand, one owner per object row, MSAA policy, translation split, sliced colours, growth policy, measured before/after | — | 17 |

## Phase 6 — what returns from the archive (after 51) ⬜

`session_viewer_archive/` implemented all of this once, in a way the refactor rejected; the old
lessons 63-120 taught it on the old tree and were deleted with commit b001bddf. Each bullet
returns as its own lesson on the refactored tree, on the seam ARCHITECTURE.md names for it.
The same list sits in `session_viewer_archive/ReadMe.md` so it is visible from both sides.

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
