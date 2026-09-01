# The restructuring — lessons 45–51, 2026-08-28 (USER DECISION: after the pointcloud chain, plain numbers)

> **Renumbered 2026-09-01.** Lesson 38 'Append, don't rebuild' went in ahead of 'Big scenes',
> so this block is now **46–52** and the lessons after it **53–115**. The numbers in the tables
> below are the ones the block was planned under; the files on disk carry the shifted ones.

> **SUPERSEDED IN PART, 2026-08-29 (revision 2) — read `_ARCHITECTURE_TARGET.md` first.** That document is the
> full-curriculum spec (the tree every lesson 52–114 lands in, the lane/walker contracts, decisions
> Q1–Q16, the seam ledger, the landing map for all 68 surveyed lessons). It keeps this file's settled
> decisions and its 7-part shape, and overrides it on seven points, listed in its §8:
> the field ladder is `113 → 102 → 89 → 66 → 44 → 18` on the **end-of-44** tree (this file's
> `97 → … → 19` is end-of-39 arithmetic); lesson 51 also splits `persistence.rs` and moves
> `impl State { render, resize }` (so the "state.rs unchanged" row below is no longer literal —
> `lib.rs`, `camera.rs`, `selftest.rs` still are); lesson 50 contains one retyped rewrite (the six
> converters onto engine row constructors); the pipeline count is **14 render + 2 compute** once
> `edges` is deleted; `gpu/instance.rs` and `gpu/view.rs` join the tree; and every source line range
> below must be re-measured on the end-of-44 tree before 45 is typed.
>
> **Revision 2 (same day)** answers the user's compartmentalization brief and changes three more
> things: the engine compartment is the **row format plus every shader that reads it** (10 shaders
> resolve to 5 families, so `ribbon`+`cylinder`, `sphere`+`glyph` and `splat`+`splat_resolve` are one
> module each and a *shader* is a `PipelineDesc` literal, not a file); **`ArenaUpload` is regrouped
> per family** — the "stays flat" decision below is REVERSED, paid in four instalments of 31-97 sites
> inside the lesson that creates the consuming family; and a producer receives **only the row groups
> it writes** (`walk_line(s: &mut SegRows, ..)` cannot reach a cloud column), so the compartment is a
> type rather than a comment. The `persistence.rs` split and the `impl State` move leave the block
> for lesson 59.


Why this exists: after lesson 39 the viewer is `src/engine/gpu/mod.rs` = 2108 lines with a 97-field
`Gpu`, `Gpu::build` = 585 lines, `set_scene` = 188, `encode_frame` = 275; `app/scene.rs` = 1254 lines
with `add_file` = 300 lines (one match over every geometry type) and `push_mesh` = 300 lines / 8
params; `pipelines/build.rs` = 845 lines of 11 copy-pasted builders taking 7–11 params each.
Nothing says which fields belong together, every new feature widens a signature, and a lesson that
touches "the cloud" edits ten places in one file. The rules in ARCHITECTURE.md §2/§3 (one concern per
file, ~300 lines, one module per subsystem) were locked at lesson 01 and drifted since 27.

This plan puts the code back into compartments — **one file per render lane** on the GPU side,
**one file per geometry type** on the walk side — with **zero behaviour change**, verified by
pixel-identical goldens after every part. Baseline = the end-of-44 tree (potree + normals + scenes + streaming + octree typed).

## Decisions (settled)

| question | decision | why |
|---|---|---|
| where in the curriculum | **after the pointcloud chain — lesson 45 onward, plain numbers** (user decision 2026-08-28). Chain before it: 40 Potree look (EDL + attenuation), 41 Cloud normals (split out of the old 40: oct16 lane, lambert, lion/bunny importers), 42 Cloud scenes (ex-41), 43 Streaming cloud (ex-42), 44 Cloud octree (re-anchored to main's flat paths — the old `43-lane-structs.md` it assumed is DELETED (done 2026-08-29): written against `end-of-42`, not an ancestor of main). Restructuring = 45…51; everything that was 45–99 shifts up by the number of restructuring lessons | the user wants every pointcloud lesson finished first, then nurbs/breps typed into the clean structure; no lettered lessons |
| pure moves vs rewrite | **moves only** — a moved body is byte-identical except for whole-word path re-roots (`self.arena_vbo` → `self.vbo`, `self.device` → `self.ctx.device`). ONE exception: `pipelines/build.rs` is rewritten around a `PipelineDesc` (845 → ≈135 lines); its correctness is gated by the tubes variant, which is the only golden that exercises the cylinder pipeline | a human can cut-and-paste a body; they cannot re-derive one. The goldens prove the moves; the descriptor table in the 40a log proves the one rewrite field by field |
| `Pipelines` struct | **kept** (`Pipelines::new(device, target, &layouts)` — 3 params, was 10); the 15 render + 2 compute pipelines stay in one place | it is the ARCHITECTURE §5 contract since lesson 04; lane-owned pipelines would put the MSAA flip in six files |
| device/queue | one `GpuCtx { device, queue }` owned by `Gpu`, passed as `&GpuCtx` | wgpu's `Device`/`Queue` are `Arc` handles; lanes never hold them, and no method grows a `(device, queue)` pair |
| the splat lane | `Splat` (per-pixel buffers, resolve) + one `SplatSlot` (records + 2 bind groups) **per point lane**, inside `CloudLane`; `Gpu::splat_records(&self, draws: &[CloudDraw]) -> ([u32; 4], Vec<u8>, u32)` keeps that exact signature | lesson 44 (streaming) adds `stream.rs` as a second lane owning its own slot; lesson 45 (octree) anchors on `splat_records` |
| `Element(Mesh)` and `FLAG_OPEN` | the existing behaviour (the Element arm discards `closed`) is preserved by a one-line mask in the dispatch | a refactor changes no bit; whether that drift is intentional is a separate question for lesson 48 |
| `ArenaUpload` | stays flat (18 fields, names unchanged); lanes pick their columns | a regroup would rename every `t.<field>` in scene.rs, three examples and lessons 45/54 for a naming gain the lane files already make explicit |

## The target tree

```
src/
├── lib.rs · state.rs · camera.rs · selftest.rs        unchanged (one path in selftest.rs)
├── math.rs                    Mat4, mat_mul, mat_to_f32, eye_from_view_proj, ortho_half_height,
│                              xform_point, grow_bounds — pure, no wgpu
├── engine/
│   ├── gpu/
│   │   ├── mod.rs        ~280  Gpu (18 fields), build = bring-up + one ::new per lane,
│   │   │                       set_scene / reset_arena / rebase_anchor / begin_frame / resize as ~30-line lists
│   │   ├── buffers.rs         GpuCtx, GrowBuf { buffer, count, cap }, append_rows, append_index_run, zeroed_buffer, mk_rows_group
│   │   ├── tables.rs          ArenaUpload (CPU delta tables), ObjectBase { model, color, flags }, CloudDraw { first, count, instance, spacing }
│   │   ├── frame.rs           FrameUniforms (mvp/time/line/cloud uniforms), write_camera, write_cloud, Binds<'a>
│   │   ├── targets.rs         Targets { depth_view, msaa_view, samples }, begin_pass
│   │   ├── objects.rs         Instance + FLAG_*, InstanceTable (rows, base, base_f32, bounds_world, inside, rebase, update_inside)
│   │   ├── arena.rs           Arena (vbo/vids/ibo + print/text index runs): append, draw_faces, draw_text
│   │   ├── segments.rs        LineStyle, CylinderSegment, SegmentLane (pipes = mesh edges, ribbons = free linework): draw_pipes, draw_ribbons
│   │   ├── glyphs.rs          GlyphPoint, GlyphLane (spheres = mesh vertices, dots = free points): draw_markers, draw_dots
│   │   ├── cloud.rs           PointBufs { pos, col, nrm }, CloudLane { pts, count, draws, slot }
│   │   ├── splat.rs           Splat, SplatSlot, SplatShared, mk_splat_group0/1/resolve, impl Gpu { splat_records, encode_splat }
│   │   ├── render.rs          impl Gpu { encode_frame }  = THE FRAME, as an ordered list of lane draw calls
│   │   ├── present.rs         impl Gpu { clear, render_offscreen, bench_frames }
│   │   └── stream.rs          (lesson 43) StreamLane — a second point lane with its own SplatSlot
│   └── pipelines/
│       ├── layouts.rs         Layouts { mvp, time, instance, line, segment, glyph, splat_group0, splat_group1, splat_resolve }
│       ├── build.rs      ~135  Target, PipelineDesc, presets opaque/ink/depth_only, build_pipeline, build_splat_compute
│       └── mod.rs             Pipelines (15 render + 2 compute) — new(device, t, &l), one desc literal per pipeline
├── app/
│   ├── manifest.rs            Item, Manifest::parse, auto_grid
│   ├── knobs.rs               env_flag + the VIEWER_* OnceLocks
│   ├── persistence.rs         unchanged
│   ├── scene.rs         ~195  Doc, Scene, add_file = the loop + ONE dispatch over geometry types + the file sweeps
│   └── walk/                  ONE FILE PER GEOMETRY TYPE, all `fn walk_<type>(w: &mut Walk, g: &T, ri: u32) -> Row`
│       ├── mod.rs             Walk { t: &mut ArenaUpload, vert_base, cloud_base, cloud_px }, Marks, Row { bounds, spacing, flags }, IdxLane
│       ├── encode.rs          encode_width, pack_rgba, oct16, pack_facing
│       ├── bounds.rs          file_extent, sheet_thickness, mark_sheet (the per-file sweeps)
│       ├── mesh.rs            walk_mesh + push (flatten, index lane, AABB, gates)
│       ├── mesh_ink.rs        mesh_ink — edges → pipes, vertices → spheres (the 230-line pen)
│       ├── mesh_topology.rs   MeshTopo, mesh_topology
│       ├── brep.rs · surface.rs · curves.rs · points.rs · frames.rs · cloud.rs
└── shaders/                   unchanged (edges.wgsl deleted — it had no draw site)
```

`Gpu` goes 97 → 18 fields. No leaf file over ~300 lines (largest: `walk/mesh_ink.rs` ≈255). No function
over 5 parameters (`push_mesh` 8 → `push(w, m, ri, lane)`; `build_ink_depth_pipeline` 11 → a desc;
`Pipelines::new` 10 → 3; `mk_splat_group1` 6 → 4).

How a BRep reaches the screen after the split, one chain of files: `scene.rs` (dispatch) →
`walk/brep.rs` → `walk/mesh.rs` → `walk/mesh_ink.rs` → `gpu/tables.rs` (the delta) →
`gpu/mod.rs::set_scene` → `gpu/arena.rs` + `gpu/segments.rs` + `gpu/glyphs.rs` (append + draw) →
`gpu/render.rs` (the frame) → `triangle.wgsl` / `ribbon.wgsl` / `sphere.wgsl`.

## The lessons (numbers final once the count is; 7 parts = 45–51)

Each part ends compiling (native `--all-targets` + wasm) and pixel-identical on the goldens below.
The lesson verbs are the usual three plus **Move** (cut a named item from file A through its closing
brace, paste into file B after a named anchor, then the listed whole-word re-roots inside B with their
hit counts) — a Move never asks you to retype a body.

| part | what moves | ends with |
|---|---|---|
| **45** Layouts, pipeline descriptors, math | `math.rs`; `pipelines/layouts.rs` (the 9 layouts out of `Gpu::build`); `build.rs` rewritten around `PipelineDesc`; `Pipelines::new(device, t, &l)`; `edges` pipeline + `edges.wgsl` deleted | `Gpu` 97 → 86 fields; `build.rs` 845 → ≈135 |
| **46** Ground | `buffers.rs` (GpuCtx, GrowBuf, the appenders); `tables.rs` (ArenaUpload, `ObjectBase`/`CloudDraw` replace the two tuples); `frame.rs`; `targets.rs` | `Gpu` 86 → 73 |
| **47** Objects and arena | `objects.rs` (the instance table = one invariant, rebase, inside flags); `arena.rs`; `Binds` in `encode_frame` | `Gpu` 73 → 50 |
| **48** The ink lanes | `segments.rs` (pipes + ribbons); `glyphs.rs` (spheres + dots) | `Gpu` 50 → 28 |
| **49** The point lanes and the frame as a list | `cloud.rs`; `splat.rs`; `render.rs` (`encode_frame` = the list); `present.rs`; `gpu/mod.rs` final | `Gpu` 18 fields; `gpu/mod.rs` ≈280 |
| **50** The walk I | `manifest.rs`, `knobs.rs`, `walk/{mod, encode, bounds, curves, points, frames, cloud}.rs`; `add_file` becomes the dispatch (mesh arms still call `push_mesh`) | `scene.rs` ≈620 |
| **51** The walk II | `walk/{mesh, mesh_ink, mesh_topology, brep, surface}.rs`; `push_mesh` gone | `scene.rs` ≈195; `add_file` ≈50 lines |

After the block: **52–56** curves / surfaces / isocurves / brep / trimmed (ex-45–49) — one `walk/<type>.rs` file + one match arm each; **57** gpu-arena (ex-50) etc.

## The gate

Measured on the baseline tree and re-run after every lesson, twice. The end-of-39 numbers below are
the reference for the chain 40–44 (each of those lessons states its own expected numbers); the
restructuring lessons 45–51 gate on the end-of-44 numbers, recorded here once that tree exists.

`VIEWER_W=1200 VIEWER_H=800 VIEWER_ZOOM=6 VIEWER_ORBIT="25,-10" cargo run --example selftest --release --target x86_64-unknown-linux-gnu -- out.ppm assets/scenes/<scene>.toml`

| scene (end-of-39) | non-background px | draws / objects |
|---|---|---|
| lion | 189148 (19.7%) | 4 / 1 |
| drawings | 114318 (11.9%) | 10 / 744040 |
| bunny_drawings | 109687 (11.4%) | 11 / 148559 |
| bunny | 129671 (13.5%) | 9 / 6 |
| bunny_drawings + `VIEWER_LINE_STYLE=tubes` | 109551 | 10 / 148559 |
| bunny_drawings + `VIEWER_REBUILD=1` | 117203 | 10 / 7 |
| bunny_drawings + `VIEWER_INCREMENTAL=1` | 109687 | 11 / 148559 |

Plus `check_determinism` on colors_widths / mesh_bunny_grey / lion (DETERMINISTIC) and `check_lean`
on colors_widths (IDENTICAL). `cloud_mix` (11446 px, 11 / 210892; tubes 11419 / 10) takes 34 min
natively and is a final gate only. Any number that moves means a move changed an expression.

## Status

- 2026-09-01: the block's own target for `gpu/mod.rs` (~280) was missed — it ended at 524, all of
  it in `Gpu::build`, which was 232 lines. Closed inside lesson 50 rather than as a new lesson, so
  nothing renumbers: `device.rs` takes the 74 lines of driver negotiation (the only code in the
  viewer decided by the machine rather than by the scene), and `FrameUniforms::new` takes the 80
  lines that built the four uniform blocks. `build` 232 → 81, `gpu/mod.rs` 524 → 374 with no
  function over 81 lines, and it is no longer the largest file in the tree. Pixel gate OK
  (4 scenes × 4 configs × 2 passes), warning count unchanged, lessons 51-52 still replay clean.
  Still over ~300 and ranked: `lib.rs` 523, `persistence.rs` 453, `splat.rs` 419, `camera.rs` 356,
  `segments.rs` 338, `objects.rs` 337, `frame.rs` 329, `arena.rs` 303.

- 2026-08-28: analysis done (3 censuses, lesson-43 audit, anchor-churn costing, 3 designs, 2 judges).
  USER DECISION: restructure after 44, as 45+, no letters. In progress: (1) the chain 40–44 made
  replayable on main — 40 split into 40 + 41, 42/43 renumbered, old 43 deleted, 44 re-anchored;
  (2) the end-of-44 tree built in a scratch copy by replaying those docs; (3) the restructuring built
  and verified on it, one commit per lesson; (4) lessons 45–51 generated from the verified diffs and
  replay-checked; (5) 45–99 renumbered.
