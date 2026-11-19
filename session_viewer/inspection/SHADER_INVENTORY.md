# Session Viewer: Shader Inventory (Phase 5)

Scope: every WGSL source in `src/shaders/` (33 files, 5,472 lines), every WGSL string assembled in Rust, every render and compute pipeline, the per-frame pass order, and the Phase 11 matrix finding. Everything here was read from the source at the paths given. No shader was compiled, no build was run, and nothing was timed. Performance notes are hypotheses unless they say otherwise. Measured numbers belong in `PERFORMANCE_AUDIT.md`.

These documents already cover the material below, so this file does not repeat it: `../ARCHITECTURE.md` (bind-group scheme, frame diagram, depth and ink rules), `ARCHITECTURE.md` §6 (lane and pass registries), and the tutorial pages `docs/journey/completion-rows.md` (Tutorial 6), `completion-geometry.md` (7) and `completion-visibility.md` (7.1). Between them those three pages cover all 33 shaders.

---

## 1. How shader text reaches the GPU

| Mechanism | Where | What it does |
|---|---|---|
| `build.rs` (65 lines) | build time | Copies `src/shaders/*.wgsl` to `OUT_DIR/shaders`. It expands `#include "x.wgsl"` (each file once per shader) and strips `//` comments, indentation and blank lines. It rejects `/* */` comments. |
| `shader!("x.wgsl")` macro | `src/lib.rs:1-6` | Calls `include_str!` on the minified copy. It has 56 use sites. |
| `#include` users | — | `plane.wgsl` and `vector.wgsl` include `arrow.wgsl`. `ink_visibility.wgsl`, `triangle_tiles.wgsl`, `scan_triangle_tiles.wgsl` and `project_triangles.wgsl` include `projected_triangle.wgsl`; `project_triangles.wgsl` also includes `slot_table.wgsl` and `clip.wgsl`. `splat.wgsl` includes `clip.wgsl`. `ssao.wgsl` includes `ambient_geometry.wgsl` and `slot_table.wgsl`. |
| String suffixes | `engine/pipelines/core.rs:304-389, 609-619` | `module()` appends `normals.wgsl` and `physical.wgsl` to every module built through it (`shared`). `scene_module()` appends `PRELUDE` = `scene.wgsl` + `clip.wgsl`. `ink_module()` adds `ink_visibility.wgsl` before the prelude. |
| Rust `format!` prefixes | `engine/gpu/clip/pipelines.rs:4-30` (`cap_source`), `engine/gpu/surface_outline.rs:750-761` (`face_coverage_source`) | Inject binding declarations whose texture type depends on the sample count (`texture_2d` or `texture_multisampled_2d`), plus `SAMPLES` and accessor functions. |
| Rust string replace | `engine/gpu/ssao.rs:713-726` (`shader_source`) | Rewrites `const MSAA`, `texture_depth_2d` and the `physical:` declaration for 4x. |
| Pipeline-overridable constants | `core.rs:513-523` | `SCENE_MSAA` (which depth texture the ink stage reads) and `CLIPPING` (whether clip tests compile away). Ink pipelines are keyed on both. |
| Embedded WGSL literal | `engine/gpu/device.rs:297` | A test-only, deliberately invalid compute shader (`invalid_gpu_shader_is_fatal`, `#[ignore]`). It is not part of the runtime. |
| Third-party shaders | `engine/gpu/ui.rs:14` (`egui_wgpu::Renderer`, egui 0.34.3), `engine/gpu/text.rs:574` (`glyphon::TextRenderer`, glyphon 0.11.0, two instances: anchored and overlay) | Their WGSL and pipelines live inside the crates. |

Compilation is lazy. `pipelines::build` (`core.rs:425`) returns a `Lazy<RenderPipeline>`, cached by its full description. The pipeline compiles the first time a pass calls `set_pipeline`. Shader modules are cached by (label, length, hash) (`core.rs:369-391`). Only `prewarm_initial` (`context.rs:236-252`) compiles anything early: the face pipeline (opaque or blended) and `ribbon.unselected` / `ribbon`. `Cache::compile_all` is used only by tests (`context.rs:659`, `pipelines/geometry.rs:71`).

---

## 2. Per-shader inventory

Conventions used below:
- **G0..G3** are bind groups. The shared groups are G0 `mvp` uniform, G1 `LineUniform` + `ClipUniform` uniforms (`scene.wgsl:1-3`), and G2 `instances` + `translations` storage (`scene.wgsl:16-17`). Ink shaders also get G2 bindings 2-7 from `ink_visibility.wgsl`: scene depth at 1x and 4x, the primitive id at 1x and 4x, `projected` and `triangle_tiles`.
- **Target** is `scene` for the surface format at the scene sample count (1 or 4), and `ID` for `Rg32Uint` at 1x (`core.rs:141`). `+tri` means the pipeline also writes the `Rg16Uint` triangle-id attachment (`physical()`, `core.rs:490-500`).
- Depth names come from `DepthMode` (`core.rs:146-153`). Depth is reversed: `Opaque` is write + `Greater`, `ReadOnly` is test-only `Greater`, `Always` means no test and no write but the pass still has a depth attachment, and `Detached` means the pass has no depth attachment.

### 2.1 Shared code (no entry points of its own)

| File | Lines | Included by | Content |
|---|---|---|---|
| `scene.wgsl` | 135 | `PRELUDE` (every `scene_module` / `ink_module`) | `Instance` (112 B), `LineUniform` (80 B), flag bits, G0-G2 declarations, `place()` |
| `clip.wgsl` | 188 | `PRELUDE`, `project_triangles`, `splat` | `ClipUniform` (448 B). `override CLIPPING`. Loops over `clipping.count` planes (6 loops) |
| `ink_visibility.wgsl` | 428 | `ink_module` (ribbon, vector, plane, matrix, glyph, sphere) | Physical-depth comparison against G2 b2-b5. Finite-triangle fallback walks the tile list (`for i < head.x`, `:396`). Includes `projected_triangle.wgsl` |
| `projected_triangle.wgsl` | 52 | 4 shaders | 96 B screen-space triangle record, tile-span rule |
| `normals.wgsl` | 59 | every `module()` | Octahedral and 2x8 / 2x16 normal unpacking, normal-matrix helpers |
| `physical.wgsl` | 16 | every `module()` | `PhysicalColor` / `PhysicalId` outputs, triangle id as two u16 halves |
| `slot_table.wgsl` | 45 | `project_triangles`, `ssao` (used by `ambient_geometry`) | Binary search (`while lo < hi`) over a 1024-wide `texture_2d<u32>`, mapping an instance triangle id to (arena triangle, row) |
| `ambient_geometry.wgsl` | 49 | `ssao` | G0 b3-b7: vertices, owners, indices, objects, slot table. Triangle fetch for SSAO normals |
| `arrow.wgsl` | 427 | `vector`, `plane` | `arrow_corner` (18 corners: shaft plus heads, `HEAD_ASPECT = 0.30`), `fs_main` (per-sample) and `fs_id` shared by the vector and plane lanes |

### 2.2 Face pass and pick (physical surfaces)

**`triangle.wgsl`** (391 lines). The main mesh shader. It is built twice under one cache key (`arena.rs:174`, `clip.rs:35` with label `triangle.shader`), so the module is shared.
- Entries: V `vs_main` (VsIn loc0 pos f32x3, loc1 oct normal u32, loc2 color unorm8x4, loc3 row u32, loc4 slot), `vs_triangle` and `vs_face` (vertex pulling by `vertex_index`, slot vertex buffer only), `vs_count`, `vs_count_placed`. F `fs_main`, `fs_clipped`, `fs_face_highlight`, `fs_id`, `fs_selection_mask`, `fs_masks`, `fs_count`, `fs_pick_count`, `fs_pick_owner`.
- Bindings: G0-G2 (prelude). G3 b0-b4: `face_vertices`, `face_objects`, `face_indices`, `source_faces`, uniform `selected_face`. G3 b5: `pick_caps` atomic storage (section picks).
- Pipelines: `faces.rs:297-357` (9 descriptions): `physical` (scene+tri, Blended, Opaque depth), `opaque` (scene+tri), `clipped` / `clipped_opaque` (built only once a plane cuts), `object_ids` and `pick` (ID+tri), `highlight` (scene, ReadOnlyEqual), `mask` and `masks` (R8Unorm at the scene sample count, ReadOnlyEqual). `arena.rs:513, 523`: `triangle.selection_mask`, `triangle.masks` (R8Unorm). `clip/pipelines.rs:63-131, 165`: section count (R16Float, Add, Detached), count placed, pick count / owner (R8Unorm, ColorWrite::Nothing, side effects through atomics).
- Draws: `faces.rs:215-232`, one non-indexed draw of 3 × triangles, plus one per instanced definition. Arena runs `arena.rs:357-388` (indexed). Clip `arena.rs:391-455`.
- Features: solid faces, transparency (desktop opacity 0.95), back-face red, contact fills, face highlight and selection, section counting and picking, face and component pick.
- Perf: 10 `discard`, `fwidth` / `dpdx`. Blending is on whenever opacity < 1, so overdraw is paid on every layer. The non-opaque path is the desktop default (`../ARCHITECTURE.md`, PERFORMANCE_AUDIT H9).
- Dead variants: `fs_masks` (through `arena.draw_masks`, `faces.draw_masks`) has no runtime caller (§4.3).

**`background.wgsl`** (26 lines): V `vs_main` (fullscreen triangle), F `fs_main` → `PhysicalColor`. Prelude groups G0-G1. Pipeline `backdrop.rs:174` (scene+tri, depth `Always`). Draw `backdrop.rs:118`: 3 vertices, skipped in dark view. It writes triangle id 0 so the background never occludes ink. Paper white, 0.94 with SSAO on.

**`grid.wgsl`** (78 lines): V `vs_main` (procedural lines, 50 vertices, `LineList`), F `fs_main`. G2 b0 `grid_frame` uniform (anchor-rebased f64 frame). Pipeline `backdrop.rs:212` (scene+tri, ReadOnly). Draw `backdrop.rs:192`. Floor construction grid and RGB axes.

**`splat.wgsl`** (259 lines; includes `clip`). V `vs_point` / `vs_point_aa`, F `fs_point`, `fs_point_aa`, `fs_point_id`. G0 b0 `CloudUniform` (48 B), b1 `ClipUniform`. G1 b0-b3 storage: `table`, `positions`, `colors`, `normals`. Target `Rgba8Unorm` at the scene sample count, with its own `Depth32Float` (`splat.rs:21, 91`). The ID variant writes ID+tri. Pipelines `splat.rs:521` (`build_point`, called for points and ids at `:173-193` and again on retarget `:233`). Draw `splat.rs:326`: 6 vertices × points, skipped when (camera, LOD, revision) is unchanged. Perf: binary search per vertex over records (`while hi - lo > 1`, `:47`), 3 `discard`.

**`splat_resolve.wgsl`** (151 lines): V `vs_main` (fullscreen), F `fs_main` and `fs_main_aa` (per-sample). G0 b0 `CloudUniform`, G1 b0-b1 `sdepth` / `scolor` (and `_aa` at b2-b3). Pipeline `splat.rs:531` (scene+tri, Opaque depth, writes `frag_depth`). Draw `splat.rs:346`, inside the face pass. Eye-dome lighting: 4-tap loops, 6 `textureLoad`, 2 `discard`. `frag_depth` turns off early-Z for the whole fullscreen pass.

**`cap.wgsl`** (218 lines). It has a Rust-generated prefix (`cap_source`) declaring G3 b0 `counts` (R16Float), b1 `primitives`, `SAMPLES`, `count_at`, `primitive_at`, plus b5 `pick_caps` atomic storage. V `vs_cap` (fullscreen, the instance selects the plane). F `fs_cap`, `fs_cap_masks`, `fs_cap_selection`, `fs_cap_id`. Pipelines `clip/pipelines.rs:74` (ID+tri, `cap_source(1)`), `:179` cap (scene+tri), `:195` masks, `:200` selection (R8Unorm, Always). Draws `clip.rs:440-500, 609`. Features: section caps of closed solids. Perf: per-sample loops (`for k < SAMPLES`), an 8-iteration loop (`:81`), `frag_depth` (2 uses), `discard` ×4. With planes active, every plane but the last opens its own face pass (`clip.rs:703-739`). `fs_cap_masks` is not reached at runtime (§4.3).

### 2.3 Visibility and pre-passes (compute and buffer-writing raster)

**`project_triangles.wgsl`** (239 lines). C `cs_main`, `@workgroup_size(64)`. G0 mvp, G1 b0 `ProjectLine` (a copy of the first five `LineUniform` fields) and b1 `ClipUniform`, G2 b0-b1 `ProjectInstance` (an `Instance` copy) and translations. G3 b0-b2 vertices, objects and indices, b3 `projected` (read_write), b4 `live_count`, b5 `slot_table`. Pipeline `triangle_tiles.rs:632` through `compute_pipeline` (`:701-721`, the only `create_compute_pipeline` call, `:713`). Dispatch `triangle_tiles.rs:493`, run only when the (camera, geometry) key changes. Perf: polygon clipping loops per triangle, scaling with total triangle count.

**`triangle_tiles.wgsl`** (122 lines). V `vs_main` (a quad over the triangle's tile box, `TriangleStrip`, 4 vertices × triangles instances). F `fs_count`, `fs_fill`. These write storage through atomics and write no colour (`ColorWrite::Nothing`, R8Unorm dummy target). G0 mvp, G1 `TileLine`, G3 b0 `projected`, b1 `tile_records`. Pipelines `triangle_tiles.rs:657, 658`. Draws `triangle_tiles.rs:523` (pass `triangle.tiles`, twice per rebuild). Perf: 9 atomic operations, overdraw equal to triangle-box tile coverage.

**`scan_triangle_tiles.wgsl`** (135 lines). C `scan_tiles`, `scan_blocks`, `finish_offsets`, all `@workgroup_size(256)`. G0 `ScanLine`, G1 `records` read_write. Pipelines `triangle_tiles.rs:667-669`. Dispatched as three separate compute passes (`triangle_tiles.rs:431-442`). The prefix sum over tile counts gives each tile its list offset.

### 2.4 Ink pass (strokes, markers, lanes)

These are all `ink_module` shaders: prelude plus `ink_visibility.wgsl`, G2 = `Layouts::ink_instance`, depth `Always` (the shader tests physical depth itself), `scene_samples` set, colour pipelines `Blended`. Each ID variant targets ID at `scene_samples(1)`.

**`ribbon.wgsl`** (787 lines, the largest). V `vs_main`, `vs_unselected`, `vs_selected`, `vs_front`, `vs_front_unselected`, `vs_front_selected`. F `fs_main` (per-sample), `fs_mask`, `fs_masks`, `fs_masks_selected` (per-sample), `fs_id`, `fs_edge_id`. G3 (`segment_rows`) b0 `segments` (72 B `StrokeSegment`), b1 `source_edges`, b2 uniform `edge_selection`. Vertex slot 0 holds the instance slots. Pipelines `segments.rs:751-818` (9 descriptions): `unselected`, `selected`, `ribbon`, `id_ribbon`, `id_edge`, and four mask variants. Draws `segments.rs:647-675`: 6 vertices × segments per table (pipes, ribbons, sheet), plus per-definition instance draws. It is used by two lanes, `segments` and `control_net` (`context.rs:41-42`). Features: mesh and BRep edges, polylines and NURBS, dashes, arrowhead trims, sheets, selection, edge pick. Perf: 7 `discard`, `HEAD_STEPS` loop (`:96`), per-sample shading at 4x, and every fragment may walk a tile list (`ink_visibility.wgsl:396`). A vertex-stage cull exists (`segment_covered`, off-screen box rejection). The four mask pipelines and the `vs_front_*selected` / `fs_mask*` entries have no runtime caller (§4.3).

**`vector.wgsl`** (22 lines; includes `arrow`). V `vs_main` (18 corners). F `fs_main` / `fs_id` come from `arrow.wgsl`. G3 b0 `vectors` (64 B `VectorRow`). Pipelines `vectors.rs:211, 216`. Draw `vectors.rs:114`: one instanced draw. A registered `RowLane` (`lane.rs:86`). Features: vectors, curve arrowheads, cut arrows.

**`plane.wgsl`** (47 lines; includes `arrow`). V `vs_plane`. F from `arrow.wgsl`. G3 b0 `planes` (48 B `PlaneRow`). Pipelines `planes.rs:209, 214`. Draw `planes.rs:112`: `22*6 + 3*18 = 186` vertices × planes, one draw. A registered `RowLane` (`lane.rs:87`). Features: plane objects (10×10 grid plus XYZ arrows), clipping-plane rectangles.

**`matrix.wgsl`** (284 lines). **This is the Phase 11 removal target (§6).** V `vs_matrix` (one 6-vertex quad per matrix, from `QUAD` and `instance_index` = block start). F `fs_main` (per-sample), `fs_id`. G3 b0 `words` (u32 word table: 16-word header + 3 words per value). Pipelines `matrices.rs:210` (`matrix`, scene, Blended, Always) and `:215` (`matrix.id`). Draw `matrices.rs:116-129`: **one `draw` call per matrix** (`for start in starts { draw(0..6, start..start+1) }`). A registered `RowLane` (`lane.rs:88`). Perf: per fragment it formats the number (`label()`: loops and a 16-entry private array) and evaluates up to 7 segment distances × characters (`glyph_distance`). It also uses `fwidth` and 2 `discard`.

**`glyph.wgsl`** (179 lines). V `vs_main`, `vs_source` (3-vertex triangle around a disc). F `fs_main` (per-sample), `fs_id`, `fs_source_id`. G3 b0 `glyphs` (48 B `GlyphPoint`). Pipelines `glyphs.rs:274` dot, `:287` id_dot, `:292` source_dot (ID, OpaqueEqual). Draw `glyphs.rs:212-227`: 3 vertices × dots, one draw. It is used by the `glyphs` and `controls` lanes. Features: point dots, control points, source-point pick.

**`sphere.wgsl`** (209 lines). V `vs_main`, `vs_front` (loc0 template position f32x3, a quad template). F `fs_main` (per-sample), `fs_id`. G3 b0 `glyphs`, the same struct and binding as `glyph.wgsl`. Pipelines `glyphs.rs:269` sphere, `:279` id_sphere. Draw `glyphs.rs:196-208`: indexed and instanced. Features: shaded vertex markers (F10, mesh vertices) with radius taper. Perf: a 3×2 normal loop (`:59-68`), 3 `discard`.

**`text_outline.wgsl`** (70 lines). V `vs_main` (loc0 pos, loc2 colour, loc3 row: the arena vertex plus a row buffer). F `fs_main`, `fs_id`, `fs_physical_id`. Prelude G0-G2. Pipelines `text_outline.rs:107` (scene, Blended, ReadOnly), `:115` id (ReadOnlyEqual), `:122` physical_id (ID+tri). Draws `text_outline.rs:73-90`, called twice in the ink pass: once for print fills before the strokes and once for lettering after the markers (`render.rs:116, 133`). Features: sheet fills and triangulated CAD lettering.

### 2.5 Post-face passes (outline, ambient occlusion)

**`face_coverage.wgsl`** (54 lines). It has a Rust prefix declaring G0 b0 `physical` (triangle id, 1x or MSAA) and `SAMPLES`. V `vs_main` (fullscreen), F `fs_fraction` and `fs_samples`. Pipeline `surface_outline.rs:782` (R8Unorm 1x, Detached). `fs_samples` has no pipeline. Feature: solid coverage mask for the outline. Perf: per-pixel loop over the samples.

**`surface_outline.wgsl`** (129 lines). V `vs_main` (fullscreen). F `fs_main`, `fs_alpha`, `fs_pool`, `fs_dilate`. G0 b0 `mask`, b1 radius uniform, b2 `coarse`. G1 the selection's copies. G2 b0 `taps` uniform. Pipelines `surface_outline.rs:706` (`black selection outline`, scene, Blended, Always), `:728` alpha (R8Unorm), `:746` pool and dilate (R8Unorm). Draws `surface_outline.rs:545, 577, 619, 633`. The final composite is drawn inside the ink pass (`Outline::over_ink`). Masks are rebuilt only when `MaskKey` (mvp, geometry, selection, faces, size, samples) changes (`surface_outline.rs:847-905`). Perf: the tap loop (`taps.count.x`), `block × block` pooling loops, 8 `textureLoad`. The same source is compiled as **3 modules** because the labels differ (`selection outline`, `selection outline alpha`, `selection outline pool`, `:700, 715, 733`).

**`ssao.wgsl`** (382 lines; includes `ambient_geometry`, `slot_table`). V `vs_main` (fullscreen). F `fs_prepare`, `fs_ground`, `fs_main` (horizons), `fs_filter_x`, `fs_filter_y`, `fs_history`, `fs_upsample`. G0 b0 `depth`, b1 `Ambient` uniform, b2 `physical`, b3-b7 geometry. G1 b0-b2 `linear`, `radii`, `normals`. G2 b0 `occlusion`, b1 filtering sampler. G3 b0-b3 read_write `corrections`, `edge_flags` (atomic), `edge_tiles`, `draw_edges`, plus b4-b5 history textures. Targets: R32Float + R16Uint + Rg8Unorm pyramid, R8Unorm, and R32Float history (`ssao.rs:217-296`). Pipelines `ssao/pipelines.rs:140` prepare and `:141` reduce. These two are **uncached raw `create_render_pipeline`** calls (closure `:108-134`, `:113`) that compile eagerly whenever `SsaoPipelines` is built. Then `:147` occupancy, `:162` raw, `:163` ground, `:189, 194` filters, `:203` history, `:231` upsample (all `build`). Draws `ssao.rs:405-680`: when the view changed, about 13 fullscreen passes (6-level depth pyramid, occupancy, ground, 4 shading or filter passes, history), then the composite. Perf: 11 loops, including a 24/stride × 6-step horizon search (`:142-147`), 14 `textureLoad`, 2 `textureSample`, 4 atomics. Off by default (`view.ssao`).

**`ambient_depth.wgsl`** (35 lines). V `vs_main`, F `fs_reduce` (4-tap) and `fs_occupancy` (25-tap). G0 b0 `linear`, b1 `radii`. Pipelines `ssao/pipelines.rs:141` (raw) and `:147`.

**`ambient_composite.wgsl`** (47 lines). V `vs_main`, `vs_edges`. F `fs_main`, `fs_edges`. G0 b0 `Ambient`, b1-b2 `corrections` and `tiles` storage. G1 b0 `occlusion`. Pipelines `ssao/pipelines.rs:267, 272` (scene, Blended, Detached). Draw `ssao.rs:657, 675`. The edge-correction draw runs at 4x only.

### 2.6 Text, gumball

**`text_plate.wgsl`** (67 lines). V `vs_main`, `vs_id` (6 attributes, stride 40: pos f32x3, local, half size, radius, row, selected). F `fs_main`, `fs_id`. G0 b0 `pick` uniform (ID pass only). Pipeline `text_plate.rs:158-216` uses **raw `create_render_pipeline` (`:173`) inside a `Lazy`**. It creates its own shader module per pipeline (`:164`), bypasses the shader cache, and uses an auto layout for the colour variant (`layout: None`). It is called at `:27, 28, 35`. Depth is read-only `GreaterEqual`. Draw `text_plate.rs:106-137`, twice: depth-tested plates, then overlay plates. Features: rounded label backing (black, or yellow when selected).

**`text_plane.wgsl`** (81 lines). V `vs_main`, `vs_id` (7 attributes, stride 68). F `fs_main`, `fs_id`. G0 b0 `coverage_texture`, b1 linear sampler. G1 b0 `pick` (ID only). Pipeline `text_plane.rs:598-631` uses **raw `create_render_pipeline` (`:616`) inside a `Lazy`**, with its own module per pipeline. It is called at `:72, 73, 90`. Draw `text_plane.rs:267-280`: **one bind-group switch and one 6-vertex draw per label**, because every label has its own coverage texture. Features: world-plane text and the plate-less matrix titles. Perf: `textureSample` in uniform control flow, `dpdx` and `fwidth`, 3 `discard`.

**`widget.wgsl`** (61 lines). V `vs_main` (loc0 pos f32x3, loc1 colour u32, loc2 handle u32), `vs_composite`. F `fs_main`, `fs_composite`. G0 b0 `Widget` uniform (96 B), G1 b0 `picture` and b1 sampler. Pipelines `widget.rs:239` (mesh into a private Rgba8Unorm 4x tile, `MESH_TARGET` `:252`) and `:359` (composite, surface format 1x, Detached, Blended). Both pipelines compile the same source under 2 labels (`widget`, `widget composite`), so 2 modules. Draw `widget.rs:166-219`: 2 render passes and 2 draws, only while a gumball is shown.

### 2.7 Counts

| | Count |
|---|---|
| WGSL files | 33: 25 declare entry points (`arrow.wgsl`'s are reached only through `#include`), 8 have none |
| Entry points | 105 (63 fragment, 38 vertex, 4 compute; regex count of `@fragment` / `@vertex` / `@compute`) |
| Render-pipeline descriptions in source (§3) | 73, plus egui and 2 glyphon renderers |
| Compute pipelines | 4 (`cs_main`, `scan_tiles`, `scan_blocks`, `finish_offsets`) |
| Raw `create_render_pipeline` sites | 4: `core.rs:536` (the one behind `build`), `text_plate.rs:173`, `text_plane.rs:616`, `ssao/pipelines.rs:113` |
| Raw `create_compute_pipeline` sites | 1: `triangle_tiles.rs:713` |

---

## 3. Pipeline table

Every pipeline description in the source, in file order. "Built when" means the pipeline object is created lazily on first `set_pipeline` unless stated otherwise. Ink variants are rebuilt on every MSAA retarget and on every change of `cache.clipping`.

| Site | Label | Shader / VS / FS | Target | Depth | Blend | Notes |
|---|---|---|---|---|---|---|
| `faces.rs:297` | source face IDs | triangle / vs_face / fs_id | ID+tri | Opaque | — | Component pick |
| `faces.rs:302` | selected face | vs_face / fs_face_highlight | scene | ReadOnlyEqual | — | Ink pass, first draw |
| `faces.rs:309` | selected face mask | vs_face / fs_selection_mask | R8 ×samples | ReadOnlyEqual | — | Outline selection mask |
| `faces.rs:319` | selected face masks | vs_face / fs_masks | 2×R8 | ReadOnlyEqual | Max | **No runtime caller** |
| `faces.rs:332` | physical triangle | vs_triangle / fs_main | scene+tri | Opaque | Blended | Opacity < 1 (desktop default) |
| `faces.rs:340` | physical triangle opaque | vs_triangle / fs_main | scene+tri | Opaque | — | Opacity = 1; prewarmed |
| `faces.rs:347` | object triangle IDs | vs_triangle / fs_id | ID+tri | Opaque | — | Object pick |
| `faces.rs:356` / `:357` | clipped triangle (±opaque) | vs_triangle / fs_clipped | scene+tri | Opaque | Blended / — | Only while a plane cuts |
| `arena.rs:513` | triangle.selection_mask | vs_main / fs_selection_mask | R8 | ReadOnlyEqual | — | |
| `arena.rs:523` | triangle.masks | vs_main / fs_masks | 2×R8 | ReadOnlyEqual | Max | **No runtime caller** |
| `segments.rs:751` | ribbon.unselected | ribbon / vs_unselected / fs_main | scene | Always | Blended | Prewarmed |
| `segments.rs:759` | ribbon.selected | vs_selected / fs_main | scene | Always | Blended | |
| `segments.rs:767` | ribbon | vs_main / fs_main | scene | Always | Blended | Control net; prewarmed |
| `segments.rs:772` | ribbon.id | vs_front / fs_id | ID | Always | — | |
| `segments.rs:780` | edge.id | vs_front / fs_edge_id | ID | Always | — | |
| `segments.rs:788-812` | ribbon.mask, .mask.selected, .masks, .masks.selected | vs_front_* / fs_mask, fs_masks, fs_masks_selected | R8 | Always | Max | **No runtime caller (4)** |
| `glyphs.rs:269` | sphere | sphere / vs_main / fs_main | scene | Always | Blended | |
| `glyphs.rs:274` | glyph | glyph / vs_main / fs_main | scene | Always | Blended | Also used by the controls lane |
| `glyphs.rs:279` | sphere.id | vs_front / fs_id | ID | Always | — | |
| `glyphs.rs:287` | glyph.id | vs_main / fs_id | ID | Always | — | |
| `glyphs.rs:292` | glyph.source | vs_source / fs_source_id | ID | OpaqueEqual | — | Source-point query |
| `vectors.rs:211` / `:216` | vector, vector.id | vector / vs_main / arrow fs | scene / ID | Always | Blended / — | |
| `planes.rs:209` / `:214` | plane, plane.id | plane / vs_plane / arrow fs | scene / ID | Always | Blended / — | |
| `matrices.rs:210` / `:215` | matrix, matrix.id | matrix / vs_matrix / fs_main, fs_id | scene / ID | Always | Blended / — | **Phase 11 target** |
| `text_outline.rs:107` | text-outline | text_outline / vs_main / fs_main | scene | ReadOnly | Blended | |
| `text_outline.rs:115` | text-outline.id | fs_id | ID | ReadOnlyEqual | — | |
| `text_outline.rs:122` | text-outline.physical_id | fs_physical_id | ID+tri | ReadOnlyEqual | — | |
| `backdrop.rs:174` | background | background | scene+tri | Always | — | |
| `backdrop.rs:212` | grid | grid (LineList) | scene+tri | ReadOnly | — | |
| `widget.rs:239` | (unlabelled) | widget / vs_main / fs_main | Rgba8 ×4 | Opaque | — | Private tile |
| `widget.rs:359` | widget composite | vs_composite / fs_composite | surface ×1 | Detached | Blended | |
| `splat.rs:521` (×2) | splat.points, splat.points.id | splat / vs_point[_aa] / fs_point[_aa], fs_point_id | Rgba8 ×samples / ID+tri | Opaque | — | |
| `splat.rs:531` | splat.resolve | splat_resolve / fs_main[_aa] | scene+tri | Opaque | — | |
| `surface_outline.rs:706` | black selection outline | surface_outline / fs_main | scene | Always | Blended | Drawn in the ink pass |
| `surface_outline.rs:728` | selection outline alpha | fs_alpha | R8 ×1 | Detached | — | |
| `surface_outline.rs:746` (×2) | selection outline pool | fs_pool, fs_dilate | R8 ×1 | Detached | — | |
| `surface_outline.rs:782` | face coverage | face_coverage / fs_fraction | R8 ×1 | Detached | — | |
| `clip/pipelines.rs:63` / `:68` | section pick count / owner | triangle / vs_count / fs_pick_* | R8 ×1 | Detached | Nothing | Atomics |
| `clip/pipelines.rs:74` | section cap ids | cap(1) / vs_cap / fs_cap_id | ID+tri | Opaque | — | |
| `clip/pipelines.rs:109` / `:114` | … placed | vs_count_placed | R8 ×1 | Detached | Nothing | |
| `clip/pipelines.rs:131` | section count placed | vs_count_placed / fs_count | R16F ×samples | Detached | Add | |
| `clip/pipelines.rs:165` | section count | vs_count / fs_count | R16F ×samples | Detached | Add | |
| `clip/pipelines.rs:179` | section caps | cap(samples) / fs_cap | scene+tri | Opaque | — | |
| `clip/pipelines.rs:195` | section cap masks | fs_cap_masks | 2×R8 | Always | Max | **No runtime caller** (`in_masks(.., false)` only) |
| `clip/pipelines.rs:200` | section cap selection | fs_cap_selection | R8 | Always | — | |
| `ssao/pipelines.rs:140` | fs_prepare | ssao | R32F+R16U+Rg8 | none | — | **Raw, eager, uncached** |
| `ssao/pipelines.rs:141` | fs_reduce | ambient_depth | R32F+R16U | none | — | **Raw, eager, uncached** |
| `ssao/pipelines.rs:147` | ambient ground occupancy | ambient_depth / fs_occupancy | R8 | Detached | — | |
| `ssao/pipelines.rs:162, 163` | ambient horizons, ground contacts | ssao / fs_main, fs_ground | R8 | Detached | — | |
| `ssao/pipelines.rs:189, 194` | horizontal / temporal filter | fs_filter_x / fs_filter_y | R8 | Detached | — | |
| `ssao/pipelines.rs:203` | ambient history depth | fs_history | R32F | Detached | — | |
| `ssao/pipelines.rs:231` | ambient upsample | fs_upsample | R8 | Detached | — | Writes storage |
| `ssao/pipelines.rs:267, 272` | ambient composite, edge correction | ambient_composite | scene | Detached | Blended | |
| `triangle_tiles.rs:657, 658` | fs_count, fs_fill | triangle_tiles (TriangleStrip) | R8 ×1 | Detached | Nothing | Atomics |
| `text_plate.rs:173` (×2) | text plates | text_plate / vs_main, vs_id | scene / ID | RO GreaterEqual | Alpha / — | **Raw, own module** |
| `text_plane.rs:616` (×2) | world text | text_plane / vs_main, vs_id | scene / ID | RO GreaterEqual | Alpha / — | **Raw, own module** |
| `triangle_tiles.rs:632` | cs_main | project_triangles | compute | | | |
| `triangle_tiles.rs:667-669` | scan_tiles, scan_blocks, finish_offsets | scan_triangle_tiles | compute | | | |
| `ui.rs:14` | egui | egui-wgpu internal | surface | | | Last in the frame |
| `text.rs:574` (×2) | glyphon anchored / overlay | glyphon internal | scene | RO (compare given) | | |

---

## 4. Duplicate and near-duplicate shader code

I compared the sources line by line, counting identical code lines of 28 characters or more after comments were stripped.

| Pair | Shared lines | What is copied | Suggested owner |
|---|---|---|---|
| `arrow.wgsl` ↔ `ribbon.wgsl` | 42 | `HEAD_PENS`, `HEAD_ASPECT`, `FILTER_REACH`, `corner_of`, `half_width_px`, `head_px`, `tuck`, `box_cdf`, `band_area`, `coverage`, `ink_axis`, `axis_cut`, the `fs_main` / `fs_id` bodies | One `pen.wgsl` `#include`d by both. The arrowhead geometry must agree with the curve trims (`completion-geometry.md` Step 2), so a copy that drifts is a correctness risk as well as clutter. |
| `cap.wgsl` ↔ `ribbon.wgsl` | 11 | `box_cdf`, `band_area` (antialiasing integrals) | The same include |
| `glyph.wgsl` ↔ `sphere.wgsl` | 23 | `GlyphPoint` struct, G3 b0 binding, `VsOut`, `dead_*`, `ramp`, `coverage`, `fs_main` / `fs_id` structure | `glyph_point.wgsl` include |
| `text_plate.wgsl` ↔ `text_plane.wgsl` | 5 + pattern | `VsOut` layout (object, selected), pick-window `vs_id`, rounded-box SDF | Small include; also use the shared `build` cache (§5) |
| `project_triangles` / `triangle_tiles` / `scan_triangle_tiles` | 3 structs | `ProjectLine` / `TileLine` / `ScanLine` are each "the first five fields of `LineUniform`". `ProjectInstance` re-declares `Instance` (comment at `project_triangles.wgsl`: "field order and size must match the Rust row"). | A `line_head.wgsl` include. The `instance/validation.rs` layout test checks `Instance` / `LineUniform` redeclarations only for lane shaders, not for these copies. |
| Fullscreen triangle VS | ~9 copies | `background`, `cap`, `face_coverage`, `surface_outline`, `ambient_depth`, `ambient_composite`, `ssao`, `splat_resolve` (`vs_main`), `widget` (`vs_composite`) | One `fullscreen.wgsl` |
| Off-screen dead vertex | 5 copies | `vec4(3.0, 3.0, 0.5, 1.0)` in arrow, ribbon, glyph, sphere, matrix | Prelude |
| `CloudUniform` | 2 copies | `splat.wgsl:13` and `splat_resolve.wgsl:13` | Include |

### 4.1 Five specialisation mechanisms

`#include`, string-suffix preludes, `format!` prefixes, `str::replace` and override constants (§1) all solve the same problem. Two of them, the `format!` prefixes and `str::replace`, are invisible to `naga` validation of the files on disk. The `ssao.wgsl` replacement silently does nothing if the text it targets changes.

### 4.2 Shader modules compiled more than once for identical source

The cache key includes the label (`core.rs:111`). So `surface_outline.wgsl` compiles as 3 modules and `widget.wgsl` as 2. `text_plate.wgsl` and `text_plane.wgsl` bypass the cache entirely and compile one module per pipeline, so 2 each per retarget. `module()` also appends `normals.wgsl` + `physical.wgsl` to shaders that do not use them (widget, surface_outline, ssao, face_coverage). That costs some compile time but has no runtime effect.

### 4.3 Entry points and pipelines with no runtime caller

**Removed 2026-11-19** (with `PipelineDesc::masks`, `ColorWrite::Max` and the `both` flag of `Pass::in_masks`); 26 GPU tests for outlines, clipping and pipelines pass. The table records what was removed.

These are lazily built, so their runtime cost is nil. They are dead code.

| Item | Evidence |
|---|---|
| `segments.rs` `draw_solid_mask`, `draw_masks`, `draw_pipes`, `draw_selection_mask` and pipelines `mask_unselected`, `mask_selected`, `masks_unselected`, `masks_selected` (`segments.rs:313-316, 599-623, 788-818`) | No call site in `src/` (grep of `segments.draw_*` and `control_net.draw_*`). The outline uses `arena.draw_selection_mask` (`surface_outline.rs:883`). |
| `ribbon.wgsl` `vs_front_unselected`, `vs_front_selected`, `fs_mask`, `fs_masks`, `fs_masks_selected` | Referenced only by the pipelines above |
| `arena.draw_masks` (`arena.rs:332`), `faces.draw_masks` (`faces.rs:206`), pipelines `triangle.masks`, `selected face masks`, and `triangle.wgsl::fs_masks` | No caller |
| `clip.draw_cap_masks` → `fs_cap_masks` | `Pass::in_masks` is only called with `both = false` (`surface_outline.rs:886`) |
| `face_coverage.wgsl::fs_samples` | No pipeline names it |

### 4.4 Documentation drift

`../ARCHITECTURE.md` ("A frame", "Rust ↔ WGSL") calls the face-pass metadata attachment `Rgba16Float` (gradient in xy, primitive id in zw). The code allocates `Rg16Uint` triangle-id halves (`targets.rs:63, 69`; `core.rs:490-500`; `physical.wgsl`).

---

## 5. Per-frame pass and pipeline order

The order below is from `Gpu::encode_frame` (`render.rs:7-44`) and the pass hooks (`pass.rs:101-106`: `instanced`, `clip`, `ssao`, `surface_outline`). It assumes a typical desktop solid scene: blended faces, outlines on, SSAO off, no clipping plane, one selected object, no pick pending.

| # | Pass (encoder) | Pipelines set, in order | Runs when |
|---|---|---|---|
| 1 | `triangle.project` (compute) | `cs_main` | Strokes are drawn and the (mvp, geometry) key changed |
| 2 | `triangle.tiles` (render) | `fs_count` | Same, with lists needed and not a rough drag tier |
| 3-5 | `triangle.scan` ×3 (compute) | `scan_tiles`, `scan_blocks`, `finish_offsets` | Same |
| 6 | `triangle.tiles` | `fs_fill` | Same |
| 7 | splat prelude | `splat.points` | Clouds present and (camera, LOD, revision) changed |
| 8 | clip counts / extra face passes | `section count`(+placed), `background`, `grid`, `section caps` | Only with cap planes, one extra face pass per plane but the last |
| 9 | **physical face pass** | `background` → `grid` → (`section caps`) → `physical triangle` or `opaque` or `clipped*` → `splat.resolve` | Always |
| 10 | ambient (~13 passes + composite) | §2.5 | `view.ssao` |
| 11 | face coverage, selection mask, pool and dilate (×2), alpha | `face coverage`; `triangle.selection_mask`, `selected face mask`, (`section cap selection`); `pool`, `dilate`; `selection outline alpha` | Masks only when `MaskKey` changed. The alpha pass runs when the mask or its radius changed (`encode_alpha`, `surface_outline.rs:581-627`) |
| 12 | **visible ink pass** | `selected face` → `text-outline` (print) → `ribbon.unselected` ×≤3 (pipes, ribbons, sheet) → `ribbon.selected` (pipes) → `black selection outline` → `ribbon.selected` ×≤2 → `vector` → `plane` → `matrix` → `sphere` → `text-outline` (lettering) → `glyph` → `ribbon` (control net) → `glyph` (controls) → `world text` (per label) → `text plates` → glyphon anchored → `text plates` (overlay) → glyphon overlay | Always |
| 13 | id pass (3 sub-passes) | tiles again (1-6), cap ids, face or object ids, `splat.points.id`; then `edge.id` or `ribbon.id`, `sphere.id`, `text-outline.id`, `glyph.id`, `vector.id`, `plane.id`, `matrix.id`, text ids | Only when a click is pending (`render.rs:170-341`) |
| 14 | `widget overlay`, `widget composite` | widget mesh, composite | Gumball visible |
| 15 | egui | egui-wgpu | UI present |

### Switching observations

These come from reading the code. None has been measured.

- **The ink pass binds the same pipeline repeatedly.** `SegmentLane::draw_table` (`segments.rs:647-675`) calls `set_pipeline` and `Binds::set` (three groups) plus group 3 and the slot buffer for every table. So `ribbon.unselected` is bound up to 3 times in a row and `ribbon.selected` twice. The same pattern appears in `arena.draw_run`, `faces.draw`, `glyphs`, `vectors`, `planes` and `matrices`. wgpu's WebGPU backend forwards each call to the browser, so the cost is per call even when the state does not change (see PERFORMANCE_AUDIT H7).
- **The order is fixed and interleaved for visual layering.** `text-outline` and `glyph` appear twice each, separated by other pipelines. This is intentional (silhouette above selected meshes, under selected curves; `../ARCHITECTURE.md`), so batching would have to keep the same layering.
- **Matrix lane:** `MatrixLane::draw` issues one `draw` per matrix (`matrices.rs:125-127`). Every other registered lane uses one instanced draw.
- **World text:** one `set_bind_group` + `draw` per label (`text_plane.rs:276-279`), because each label owns a texture. An atlas would make it one draw.
- **Per-sample shading at MSAA 4x.** Every ink colour shader declares `@builtin(sample_index)`: `ribbon` `fs_main` / `fs_mask*`, `arrow` `fs_main`, `glyph`, `sphere`, `matrix` and `splat_resolve` `fs_main_aa`. That forces sample-rate shading, about 4x the ink fragment invocations at 4x MSAA. The sample index is used to read the matching depth sample. This hypothesis is not in PERFORMANCE_AUDIT.md and is a candidate for its §4.
- **Ink fragments may walk a variable-length tile list** of storage reads (`ink_visibility.wgsl:386-418`) when the fitted-plane test is inconclusive. The cost scales with triangle density per tile.
- **The three scan dispatches** are separate compute passes (`triangle_tiles.rs:431-442`). WebGPU synchronises storage between dispatches inside one pass, so they could share one pass. This is unverified.
- **Pipeline compile hitches.** Everything except the prewarmed face and ribbon pipelines compiles on first use inside a frame (`core.rs:425-460`). The first frame with vectors, planes, matrices, markers, text, picks, outlines or SSAO pays for those compiles. The SSAO `prepare` and `reduce` pipelines compile eagerly whenever `SsaoPipelines` is built (`ssao/pipelines.rs:108-141`). `after_present` pre-warms SSAO only when it is enabled (`ssao.rs:880-885`). Toggling a clipping plane flips `cache.clipping` and recompiles every ink pipeline under new constants (`core.rs:444-445`).

---

## 6. Matrix visualization cleanup (Phase 11)

The removal plan with line ranges is in `inspection/MATRIX_CLEANUP.md`. This section records the identification.

**What the reference image shows.** `docs/screenshots/active/geometry-four-objects-1280.png` (sha256 `2ee54686…`, which matches the `?v=2ee546863e22` in the brief; `docs/screenshots/active/evidence.json:61-66`) is the Tutorial 7 checkpoint at 1280×800. It shows two light-grey cubes with thin black edges, a black double-arrow NURBS curve, and, lying flat in the 3D scene and seen in isometric perspective, a 2×3 grid of square cells. The cells are navy-to-pink with one grey zero cell, separated by thin black lines, with white seven-segment numerals (−2, 0, 2 / 1, −1, 3). That grid is the "3D matrix visualization with black outlines and rectangular graphics". `geometry-reveal-1280.png` shows the same matrix.

**What produces it.** The kernel `Matrix` component lane, end to end:

| Layer | Path | Role |
|---|---|---|
| Kernel data | `session_rust::Matrix::to_component` (`../session_rust/src/matrix.rs:1220`), `Session::add_matrix` (`../session_rust/src/session.rs:1695`) | A `Component` with `type_name == "Matrix"` and fields `rows`, `cols`, `cell`, `data`, `mincolor`, `maxcolor`, `zerocolor`, `textcolor`, `linecolor` |
| Decode | `src/app/decode.rs:328-334` | Generic component decode (shared; only the comment names the matrix) |
| Walk | `src/app/walk/matrix.rs` (169 lines) | `LINECOLOR = black` (`:21`) is the black outline. It packs the 16-word header and 3 words per value |
| Scene | `src/app/scene.rs:777-837` (`add_matrices`), `:550`, `:120`, `:206`; `src/app/scene_text.rs:75-89`; `src/state.rs:131` | Object rows and plate-less `WorldPlane` titles |
| Lane | `src/engine/gpu/matrices.rs` (264 lines) | `MatrixLane`, a registered `RowLane` (`lane.rs:88`) |
| Shader | `src/shaders/matrix.wgsl` (284 lines) | Square cells (`QUAD`), black edges (`LINE_PX = 1`, `matrix_color`), seven-segment SDF digits |
| Course | `examples/course/geometry.rs:67-77` | The 2×3 "Kernel Matrix" in the checkpoint scene |

**What must be kept (the 2D rounded-rectangle matrix).** `docs/screenshots/active/math-matrix-light-1280.png` and `-dark` show the stage deck matrix: rounded, gapped cells with no outline and TeX entries, in the right-hand column. It is produced by `src/app/stage/matrix.rs` (`Stage::matrix`, `Shape::Rect { radius: 0.16·cell }`), `src/app/stage/deck.rs:170-181, 337-410` (`MatrixSpec`, YAML key `matrix:`), `src/app/stage/paint.rs:445-466` (`draw::rounded`, `egui::Shape::convex_polygon`) and `src/app/stage/draw.rs:26-43`. It is drawn by egui through `engine/gpu/ui.rs` and has **no WGSL of its own**. Nothing in `src/app/stage/` references `MatrixLane`, `walk::matrix`, `matrix.wgsl` or the kernel `Matrix`, so the two paths share no matrix-specific code.

**Shared infrastructure the 3D lane uses that must stay:** `lane.rs` `REGISTRY` / `RowLane` (vectors, planes), `ink_module` and `ink_visibility.wgsl`, `Layouts::ink_rows`, `TextPlacement::WorldPlane` and `text_plane.wgsl`, `walk/encode.rs::pack_rgba`, `Row::thin`, generic component decode, and `scene.rs::placement` (also used at `:514` and in `scene_instances.rs:159`).
