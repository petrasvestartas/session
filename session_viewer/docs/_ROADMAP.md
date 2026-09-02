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

## Phase 6 — what the archive already knows how to do (after 51)

`session_viewer_archive/` implemented these once, badly; each returns as its own lesson on the
refactored tree, in this order, each landing on the seam ARCHITECTURE.md names for it:

1. picking: cursor→ray in kernel space, ray against the kernel BVH, object level (`Scene.order`
   is the row → guid map)
2. selection and visibility flags, one flag word shared by every lane (bits 2-5 are taken)
3. the frame as an appendable pass list: an overlay pass (gumball, edit points) after the scene
   list, an egui pass last
4. the command line and the history (Command trait at the first mutation)
5. draw tools (point, line, polyline, curve) with snapping
6. gumball transform and commit
7. sub-object picking and edit points (`Row` is the seam for a face/edge id)
8. tree and graph panels
9. text labels
10. post-processing (SSAO, outline, composite) as extra targets and passes
11. section planes; id-buffer picking (an R32Uint attachment and an async readback)

Kernel-side work the audit named and the viewer cannot fix alone: the decode cost of the
protobuf wire (75-79% of a native load), packed mesh arrays, the octree build.
