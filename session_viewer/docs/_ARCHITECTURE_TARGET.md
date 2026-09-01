# session_viewer — Target Architecture (revision 3, 2026-08-30)

> Revision 2 (the compartment axis, the two maps, the contracts, the 45-51 cut) is below;
> revision 3 — the maintenance kit that makes those rules enforceable — is at the end of this file.

Status: **revision of the 2026-08-29 final spec.** It replaces §2 (target tree), §3 (contracts) and §8 (the 45-51 cut) wholesale, adds the two maps the user asked for by name (§2 geometry chains, §3 shader chains), and adds an explicit borrow-rules section and a lesson-writing template. §1 (the two rules + the three classes), §4 (the frame), §5 (the object-table invariant), §6 (Q1-Q16), §7 (the seam ledger), §9-§12 stand except where §10 below lists a delta.

**Every number in this document was re-measured on the tree at `/home/petras/code/code_rust/session/session_viewer` on 2026-08-29.** Where it disagrees with revision 1, revision 1 is wrong and §10 says so.

| fact | measured | command |
|---|---|---|
| `.wgsl` files | **10** (not 11) | `ls src/shaders/*.wgsl \| wc -l` |
| `Gpu` fields | **99** today | `sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs \| grep -cE '^\s+(pub )?[a-z_0-9]+\s*:'` |
| `gpu/mod.rs` | 2,162 | `wc -l` |
| `app/scene.rs` | 1,287 | `wc -l` |
| `push_mesh` | 830-1143 = **314 lines, 8 params**, returns `(Option<Bounds>, bool)` | `sed -n '830,839p' src/app/scene.rs` |
| `encode_frame` | 1529-1838 = **310 lines** | `grep -n 'fn encode_frame\|fn reset_arena' src/engine/gpu/mod.rs` |
| `set_scene` | 937-1133 = 197 lines | same |
| `ArenaUpload` | **18 columns** today, 19 at end-of-44 (`cloud_nodes`) | `sed -n '/^pub struct ArenaUpload/,/^}/p'` |
| `upload_to` drops | **13** columns by hand; 3 kept by comment (`scene.rs:191-207`) | `sed -n '187,208p' src/app/scene.rs` |
| `(buffer, count, cap)` shape | **12** `_cap` fields, and **`arena_vids` has no cap of its own** — it rides `arena_vert_cap` | `grep -n '_cap' src/engine/gpu/mod.rs` |
| `Pipelines::new` | **10 params**, 15 render pipelines, `edges` has **0** draw sites | `cat src/engine/pipelines/mod.rs` |
| `ribbon` vs `ribbon_solid` builders | labels + whitespace + comments **and one live `VIEWER_NO_DEPTH → CompareFunction::Always` branch present only in `ribbon_solid` (build.rs:621)** | `diff <(sed -n '/fn build_ribbon_pipeline/,/^}/p' …) <(…solid…)` |
| `Element::geometry()` | `-> &ElementGeometry` (`session_rust/src/element.rs:276`) | `grep -n 'pub fn geometry'` |
| upload-column rename cost | **~173 qualified accesses + ~36 declaration lines across 4 files** (`app/scene.rs`, `engine/gpu/mod.rs`, `src/selftest.rs`, `examples/check_lean.rs`) | per-column grep, table in §6 |

---

## 1. What changed and why — the compartmentalization decision

The user's complaint is that one flat 18-column `ArenaUpload`, one 99-field `Gpu` and one 314-line 8-param `push_mesh` smash every geometry type and every GPU input together; the decision is that **the engine compartment is the row format together with every shader that reads it (the "family"), and the app compartment is the geometry type**, joined by a typed per-family upload group that neither side can reach across. Geometry type is not a GPU concept — five kernel types tessellate into the same triangle arena and five write the same 40-byte `CylinderSegment` — so a per-type engine split would have produced five copies of one buffer, one layout and one pipeline set, which is today's disease rebuilt one floor down. Conversely ten shaders resolve to only **five** row formats, so a strict one-module-per-shader split would leave `CylinderSegment` homeless between `ribbon.wgsl` and `cylinder.wgsl`, `GlyphPoint` homeless between `sphere.wgsl` and `glyph.wgsl`, and `CloudUniform` homeless between the two halves of one splatting algorithm — which is why those three pairs are one module each and the per-shader unit is a `PipelineDesc` literal, not a file. The axis that wins is therefore the one revision 1 already had, sharpened by one enforcement mechanism it lacked: **a producer receives only the row groups it writes**, so `walk_line(s: &mut SegRows, …)` cannot reach a cloud column and the reader can read which shaders a type can touch off its signature — the compartment becomes a type rather than a comment. From the geometry-type proposal we graft the `CHAIN` const and the `chain_table` test, so §2's end-to-end map is generated from the code and fails the build instead of rotting the way `ARCHITECTURE.md` §2 rotted between lessons 27 and 44; we reject its `app/rows/` layer, because putting `pack_rgba` and `CylinderSegment::new` app-side is the exact layer inversion settled Q12 names and it breaks at 67/73/74 when overlays build rows inside `engine/`. From the lane proposal we graft the `Element(Mesh)` `FLAG_OPEN` **mask kept forever** (no golden scene contains an Element, so restoring the bit inside a pixel-gated block ships an unmeasurable change — settled Q13(2)), the merge of `grid.wgsl` + `background.wgsl` into one 55-line `backdrop.rs` so `scene_list` is uniformly one line per family with no exception, and the ruling that the shared WGSL prelude — the one duplication no module boundary removes — must **not** land inside a moves block, because it edits nine shader files under a pixel gate; it moves to lesson 104 beside the error scope that must subtract its line offset, and lesson 47 instead ships a free `#[cfg(test)]` mirror test that reads the five `.wgsl` copies of `struct Instance` and fails `cargo test` when the Rust field list drifts. The `ArenaUpload`-stays-flat decision is **reversed**: it is regrouped per family, in four instalments of 31-97 whole-word sites paid inside the lesson that creates the consuming family, never as one 173-site table. Finally the block's two non-axis passengers — the `persistence.rs` 3-way split and the `impl State { render, resize }` move — are **deferred to lesson 59**, because they are size problems unrelated to compartmentalization and their presence is what made revision 1's lesson 51 three lessons stapled together.

---

## 2. THE MAP: every geometry type, end to end

One row per kernel type. `→` crosses a file boundary. Read left to right and you have the whole life of a `Geometry` variant.

**This table is generated, not written.** Each `app/walk/<type>.rs` carries `pub const CHAIN: Chain` (§5.6); `cargo test chain_table` prints this table and asserts it against the sinks each producer's signature actually names. A producer that starts writing a second row format without widening its `CHAIN` fails the build.

| # | `Geometry::` | walk file (lines of its own) | row format(s) produced | `Upload` group.field | family module | pipeline(s) | shader(s) | what is type-specific |
|---|---|---|---|---|---|---|---|---|
| 1 | **Mesh** | `walk/mesh.rs` 200 + `mesh_ink.rs` 255 + `mesh_topology.rs` 125 | `RenderVertex` ×verts, `u32` vid ×verts, `u32` idx ×3·tris, `CylinderSegment` ×edges, `GlyphPoint` ×verts | `arena.{verts,vids,idx｜idx_print｜idx_text}` · `seg.pipes` · `glyph.spheres` · `obj.*` | `arena.rs` + `segments.rs` + `glyphs.rs` | `triangle` ｜ `triangle_sheet` · `cylinder`｜`ribbon_solid`(+`ribbon_solid_depth`) · `sphere`(+`sphere_depth`) | `triangle.wgsl` · `cylinder.wgsl`/`ribbon.wgsl` · `sphere.wgsl` | **the only type with real per-type logic.** `is_print_fill` (`widths()==[0.0]`) routes the index run and sets `FLAG_PRINT` (`scene.rs:262-267, 278-281`); `FLAG_OPEN` from the fused topology's `closed` (294-296) — set for no other type; `MESH_RAW_MIN=200_000` dense gate (890-892); `WIREFRAME_BLACK_MIN=10_000`; `COPLANAR_DOT=0.9999`; per-edge `width_at`/`hidden`; the dense-vs-sparse vertex slot table (938-950); `VIEWER_NO_EDGES`/`NO_DOTS`/`ALL_EDGES` |
| 2 | **BRep** | `walk/brep.rs` 90 — **3 lines of its own** | identical to #1, `idx` run only, `closed` discarded | `arena.{verts,vids,idx}` · `seg.pipes` · `glyph.spheres` · `obj.*` | same three | as #1 minus the sheet pipelines | as #1 | `b.mesh()`, `set_objectcolor(b.surfacecolor)`, `MeshOpts { lane: Solid, closed: false }` |
| 3 | **NurbsSurface** | `walk/surface.rs` 90 — **2 lines of its own** | as #2 | as #2 | same three | as #2 | as #2 | `s.mesh()`, and colour from `facecolors.first()` **not** `surfacecolor` — the single real divergence from BRep, one line |
| 4 | **Element(Mesh)** | **no file** — `walk_geometry` unwraps `e.geometry()` and calls `walk_mesh`, then **masks `FLAG_OPEN` off** | as #1 | as #1 | same three | as #1 | as #1 | **nothing.** `scene.rs:350-370` is today a hand copy of 257-298 with the `FLAG_OPEN` block deleted; the mask makes one body serve both and is **kept forever** (Q13(2)) |
| 5 | **Element(BRep)** | **no file** — unwraps and calls `walk_brep` | as #2 | as #2 | same three | as #2 | as #2 | **nothing.** `scene.rs:371-385` is a byte-level copy of 299-313 |
| 6 | **Element(None)** | the `continue` guard in `add_file` (`scene.rs:245-249`), before the object row | none | none | — | — | — | the unreachable arm at `scene.rs:386` is **deleted**; the guard must stay ahead of `objects.push` because `ri` is taken there |
| 7 | **Line** | `walk/curves.rs::walk_line` — **10 lines** | 1 `CylinderSegment` (40 B) | `seg.ribbons` · `obj.*` | `segments.rs` | `ribbon` (+`ribbon_depth`, dormant) | `ribbon.wgsl` | `encode_width(l.width)`, `pack_rgba(l.linecolor)`, `facing = FACING_UNKNOWN` |
| 8 | **Polyline** | `walk/curves.rs::walk_polyline` — **12 lines** | n−1 `CylinderSegment` | `seg.ribbons` · `obj.*` | `segments.rs` | `ribbon` | `ribbon.wgsl` | one pen hoisted out of a `windows(2)` map |
| 9 | **NurbsCurve** | `walk/curves.rs::walk_nurbscurve` — **45 lines** | 4…64 `CylinderSegment` | `seg.ribbons` · `obj.*` | `segments.rs` | `ribbon` | `ribbon.wgsl` | rational-CV de-weighting for the control-net box (551-560); empty-curve early return; `n = clamp(ceil(sqrt(diag/0.2)), 4, 64)`; colour from `linecolors.first()`. After line 578 it **is** `walk_polyline` and calls it |
| 10 | **Plane** | `walk/frames.rs::walk_plane` — **13 lines** | exactly 4 `CylinderSegment` | `seg.ribbons` · `obj.*` | `segments.rs` | `ribbon` | `ribbon.wgsl` | `PLANE_SIZE` = 0.5 m half-extent — a display convention, not geometry |
| 11 | **OBB** | `walk/frames.rs::walk_obb` — **19 lines** | exactly 12 `CylinderSegment` | `seg.ribbons` · `obj.*` | `segments.rs` | `ribbon` | `ribbon.wgsl` | the const 12-edge table over `corners_f32()`, and the only **hard-coded pen** (radius 0.0 screen-constant, black) because OBB carries no colour or width |
| 12 | **Point** | `walk/points.rs::walk_point` — **10 lines** | 1 `GlyphPoint` (48 B) | `glyph.dots` · `obj.*` | `glyphs.rs` | `glyph` (+`glyph_depth`, dormant) | `glyph.wgsl` | `facing = FACING_UNKNOWN`, `facing_ext = [UNKNOWN; 2]` — a free point decorates no surface |
| 13 | **PointCloud** | `walk/cloud.rs` 115 | three columnar `f32`/`u32`/`u32` arrays (20 B/pt) + 1 `CloudDraw` + `LodNode`s | `cloud.{pos,col,nrm,draws,nodes}` · `obj.*` | `cloud.rs` (holds) → `splat.rs` (draws) | `splat_depth`, `splat_color` (compute) → `splat_resolve` | `splat.wgsl` ×2 entries → `splat_resolve.wgsl` | **the only type that is its own compartment end to end.** SoA instead of a row struct; a cumulative cross-file `cloud_base`; median-neighbour spacing at ≤1024 strides; the per-file `point_size` **pixel** override; a 2-pass atomic compute splatter with a 256-record cap and a static-frame skip; a per-pixel depth/colour buffer pair resolved by one fullscreen triangle |
| 14 | **NurbsSurfaceTrimmed** | **none today** | — | — | — | — | — | it is **not** a `Geometry` variant (`session_rust/src/session.rs:19-31` lists 11); it exists only inside `BRep::mesh()` and reaches the GPU as row #2's arena triangles. `walk/trimmed.rs` + `IdxLane::Trimmed` are reserved for 56/84 and **cost zero lines in 45-51** |

**Arithmetic.** 11 `Geometry` variants; the `Element` arm has 3 sub-arms, so **13 live leaves + 1 dead**. **Five** producers earn a file of their own (Mesh, NurbsCurve, PointCloud, and the two adapters BRep/NurbsSurface which earn one because 55/85 grow them); the rest group **by output row, not by kernel name** — which is why `curves.rs` holds three types and `frames.rs` holds two.

**Two facts every reader must carry out of this table.** (a) Every type produces **exactly one** object row `(model, tint, flags)`, pushed once by `walk_geometry` from the returned `Row` — never by a producer body; today eight arms hand-push `t.object_bounds.push(None); t.object_spacing.push(0.0);` (`scene.rs:314-317, 347-348, 386`) and two arms hand-write `t.objects.last_mut().unwrap().2 |= FLAG_PRINT` (278-281, 366-368). (b) `spacing` currently carries **two different units** into one `Instance.spacing` f32 — world-unit vertex spacing for meshes (`scene.rs:681-688`) and **screen pixels** for clouds (`scene.rs:326-328`). It is not split (the shaders read one field and splitting it is a behaviour change under a pixel gate); the *write site* is split into `Row::world_spacing(v)` and `Row::point_size_px(v)`, so the unit is named where it is chosen. The full `enum Spacing { World(f32), Pixels(f32) }` is deferred to the first lesson that needs both on one row.

---

## 3. THE MAP: every shader, end to end

Ten `.wgsl` files; `edges.wgsl` is deleted at 45 (0 draw sites — `grep -o 'pipelines\.\w*' src/` yields 15 hits, none of them `edges`), leaving **nine shaders owned by five family modules, 14 render + 2 compute pipelines**.

| `.wgsl` | ln | owning module | bind groups, slot by slot | vertex buffers | pipelines | fed by (§2 rows) |
|---|---|---|---|---|---|---|
| `triangle.wgsl` | 135 | **`gpu/arena.rs`** | g0/b0 uniform `mat4x4 mvp` ← `frame.mvp_group` · g1/b0 uniform `f32 time` ← `frame.time_group` (**declared, bound, never read** — a live 4-byte uniform kept alive only by this declaration) · g2/b0 storage `array<Instance>` ← `objects.group` | slot 0 `RenderVertex::layout()` stride 40 (`session_rust/src/render_mesh.rs:256`) ← `arena.vbo`; slot 1 `INSTANCE_ID_ATTRIBS` stride 4, `StepMode::Vertex`, **one row id per vertex** ← `arena.vids` | `triangle` (depth write on), `triangle_sheet` (depth write off) — same builder, one `bool` | 1, 2, 3, 4, 5 |
| `ribbon.wgsl` | 520 | **`gpu/segments.rs`** | g0/b0 mvp · g1/b0 `LineUniform` (VERTEX\|FRAGMENT — fs recovers ndc from `vp_w`/`vp_h`) · g2/b0 `array<Instance>` · g3/b0 `array<CylinderSegment>` ← **either** `seg.ribbons_group` **or** `seg.pipes_group`, chosen at the draw site, one layout | **none** (`buffers: &[]`); 4 verts from `vertex_index` over `TriangleStrip`, row from `instance_index` | `ribbon`, `ribbon_solid`, `ribbon_depth`, `ribbon_solid_depth` — **4 of the 14** | free lane 7, 8, 9, 10, 11 · solid lane 1, 2, 3, 4, 5 (edges, when `LineStyle::Flat`) |
| `cylinder.wgsl` | 184 | **`gpu/segments.rs`** (same file) | identical slots to `ribbon.wgsl`; g3 is **always** `seg.pipes_group` | slot 0 `cyl_template_layout()` stride 12 ← `seg.tube.vbo` (`unit_cylinder(CYL_SIDES)`), indexed | `cylinder` | 1, 2, 3, 4, 5 (edges, when `LineStyle::Tubes`) |
| `sphere.wgsl` | 288 | **`gpu/glyphs.rs`** | g0/b0 mvp · g1/b0 `LineUniform` · g2/b0 `array<Instance>` · g3/b0 `array<GlyphPoint>` ← `glyphs.markers_group` | slot 0 `cyl_template_layout()` **reused** ← `glyphs.quad.vbo`, which is `unit_quad()` — four corners, not a sphere; both names are historical | `sphere`, `sphere_depth` (the only ink-depth pipeline with a vertex buffer) | 1, 2, 3, 4, 5 (vertices) |
| `glyph.wgsl` | 172 | **`gpu/glyphs.rs`** (same file) | same four slots; g3 ← `glyphs.dots_group`. **Live layout alias:** the colour pipeline is built as `build_glyph_pipeline(.., segment_layout)` (`pipelines/mod.rs:65`) while the parameter is named `glyph_layout` — it works only because the two descriptors are byte-identical; `glyph_depth` gets the right one | **none**; 3 verts per dot, row = `vertex_index / 3u` — the only lane indexing rows by vertex index | `glyph`, `glyph_depth` | 12 |
| `splat.wgsl` | 167 | **`gpu/splat.rs`** | g0/b0 mvp · g0/b1 `CloudUniform` · g0/b2 `array<vec4<f32>> instances_unused` ← the instance buffer, **bound and never read** (a third, structurally wrong view of `Instance`) · g0/b3 `array<u32> table` ← `splat.recs` · g1/b0 `array<f32> positions` · g1/b1 `array<u32> colors` · g1/b2 `array<atomic<u32>> sdepth` · g1/b3 `array<u32> scolor` · g1/b4 `array<u32> normals` | **none** — compute. `@workgroup_size(64)`, 2D dispatch (gx ≤ 4096) because a 1D dispatch caps at 65535 groups | `splat_depth`, `splat_color` (**the only compute pipelines**; built inline in `Gpu::build` today, moved into `Pipelines` at 45) | 13 |
| `splat_resolve.wgsl` | 98 | **`gpu/splat.rs`** (same file) | g0/b0 uniform `CloudUniform` ← `frame.cloud_group`, whose bind group is created with `layout: &line_layout` (`gpu/mod.rs:696`) and whose pipeline declares group 0 as `line_layout` too — **the 48-byte LineUniform layout is doing triple duty** · g1/b0 `array<u32> sdepth` · g1/b1 `array<u32> scolor` | **none**; fullscreen triangle | `splat_resolve` (writes `@builtin(frag_depth)`, so splats and solids occlude exactly) | 13 |
| `grid.wgsl` | 96 | **`gpu/backdrop.rs`** | g0/b0 mvp · g1/b0 `LineUniform` — declares all 48 bytes, **reads exactly one field** (`anchor`), because the grid is authored in absolute world mm | **none**; 50 verts from `vertex_index`. `pass.draw(0..50, 0..1)` (`gpu/mod.rs:1664`) mirrors the shader's `FLOOR+6` **by hand** — the const moves next to it as `GRID_VERTS` | `grid` | nothing — scene furniture |
| `background.wgsl` | 28 | **`gpu/backdrop.rs`** (same file) | **none** — `bind_group_layouts: &[]`, the only pipeline with an empty layout | **none**; 3 verts | `background` | nothing |
| ~~`edges.wgsl`~~ | 24 | — | g0/b0 mvp only — no instance group, so it predates instancing and cannot honour the camera-relative anchor | slot 0 `RenderVertex::layout()`, declaring only locations 0 and 2 (a legal *partial* mirror) | **deleted at 45**, with `build_edges_pipeline`, `Pipelines.edges` and `storage_buffer` (0 callers) | — |
| — | — | **`gpu/instance.rs`** | — | — | — | owns `Instance` (96 B) + the flag table, **hand-mirrored in 5 shaders** (triangle, cylinder, ribbon, sphere, glyph) and wrongly-on-purpose in a 6th (`splat.wgsl:28`) |
| — | — | **`gpu/frame.rs`** | — | — | — | owns `LineUniform` (48 B, static-asserted) mirrored in **5** shaders, and `CloudUniform` (16 B, `_pad` load-bearing as EDL strength) in 2 |

**Why one module per shader loses, in three facts.** `ribbon.wgsl` and `cylinder.wgsl` read one identical 40-byte `CylinderSegment` through one `segment_layout`, and the choice between them is a runtime `LineStyle` branch at a single draw site — split them and the row struct is homeless or duplicated. `sphere.wgsl` and `glyph.wgsl` read one identical 48-byte `GlyphPoint`; `glyph.wgsl` declares `facing`/`facing_ext` **only so the stride matches**, a dependency that today exists purely in a comment. `splat.wgsl` and `splat_resolve.wgsl` are two halves of one algorithm over `CloudUniform` and two pixel buffers. Conversely `ribbon.wgsl` alone feeds four pipelines over two buffers fed by different geometry populations, and `triangle.wgsl` feeds two pipelines over three index runs that exist only to encode draw order — so **the per-shader unit is a `PipelineDesc` literal, and the per-row-format unit is the module.**

**The mirror hazard, stated once.** Adding a field to `Instance` without editing five `.wgsl` files changes the storage-array **stride**: row N is read from byte 96N in Rust and from something else in WGSL, so every object past row 0 gets another object's matrix, colour and flags — **no compile error, no validation error, a wrong picture.** Declaring `eye` as a `vec3` in any one of the five `LineUniform` copies pushes `anchor` from offset 32 to 48 in that shader only, and that lane's ink drifts on every re-anchor while the other four stay correct. Lesson 47 ships `instance_mirror` and `line_uniform_mirror` — `#[cfg(test)]` functions that read the `.wgsl` files and compare the field-name lists to a const in `instance.rs`/`frame.rs`. They change no shader text, so they are free under the pixel gate. The **shared prelude** that would make this a compiler error is deferred to **104** (§10 delta 8).

---

## 4. The revised module tree

`B45`-`B51` = placed by the restructure. `~` = end-state budget; ~300 is the soft cap and a leaf at 300 is a split signal for its next grower. Only rows this revision changes are listed with a mark; everything else in revision 1 §2 stands.

### `src/engine/gpu/` — **five families**, and the floor beneath them

| path | owns | ~ln | lesson | mark |
|---|---|---|---|---|
| `mod.rs` | `Gpu` (**18 fields**), `build`, `set_scene`, `reset_arena`, `resize`, `msaa_now`, the re-export list | 300 | B49 final | = |
| `buffers.rs` | `GpuCtx { device, queue }`, `GrowBuf { buf, count, cap, usage }`, `append_rows`, `append_index_run`, `zeroed_buffer`, `mk_rows_group` | 175 | B46 | = |
| `upload.rs` | **`Upload`** + the 5 row groups + `Span` + `drop_uploaded` | 130 | B46 (moved flat) → B47/48/49 (grouped) | **R** — was `tables.rs`; row structs are evicted to their families |
| `frame.rs` | `FrameUniforms`, `LineUniform`, `CloudUniform`, `FrameInput`, `write_camera`, `write_cloud`, `line_thickness_px`, `Binds<'a>`, `line_uniform_mirror` test | 265 | B46 | = |
| `targets.rs` | `Targets { depth: Texture, depth_view, msaa, msaa_view, samples }`, `begin_pass` (4 params) | 135 | B46 | = |
| `present.rs` | `Frame { surface, pub view, pub encoder }`, `begin_present -> Result<Option<Frame>>`, `end_present`, `clear`, `render_offscreen` | 185 | **B46** | **M** — was 49 |
| `view.rs` | `View` — the runtime knobs; `View::from_env()` | 70 | **B46** | **M** — was 49 |
| `instance.rs` | `Instance` (96 B), the 9-slot flag-bit table, `REANCHOR_*`, `instance_mirror` test | 130 | B47 | = |
| `objects.rs` | `InstanceTable`, `ObjectBase`, `ObjectRows`, rebase/anchor/`update_inside_flags`, `bounds_world()` | 300 | B47 | = |
| **`arena.rs`** | **family: `triangle.wgsl`** — `Arena`, `ArenaRows`, `IdxLane { Solid, Print, Text }`, `run`/`run_mut`, `INSTANCE_ID_ATTRIBS`, `Pipes { triangle, sheet }`, `descs`, `draw_faces`/`draw_print`/`draw_text` | 235 | B47 | = |
| **`segments.rs`** | **family: `ribbon.wgsl` + `cylinder.wgsl`** — `CylinderSegment` (40 B) + `::new`, `FACING_UNKNOWN`, `SegRows { pipes, ribbons }`, `LineStyle`, `Template`, `unit_cylinder`, `CYL_SIDES`, `SegmentLane`, `Pipes ×5`, `descs`, 3 draws | 300 | B48 | = |
| **`glyphs.rs`** | **family: `sphere.wgsl` + `glyph.wgsl`** — `GlyphPoint` (48 B) + `::new`, `GlyphRows { spheres, dots }`, `unit_quad`, `Template`, `GlyphLane`, `Pipes ×4`, `descs`, 4 draws | 215 | B48 | = |
| `cloud.rs` | `PointBufs`, `CloudRows`, `CloudDraw`, `LodNode`, `group1_entries` — the splat family's **feed**, not a shader | 170 | B49 | = |
| **`splat.rs`** | **family: `splat.wgsl` + `splat_resolve.wgsl`** — **`#[repr(C)] SplatRecord`** (36 words; **there is no Rust type today**), `REC_WORDS`, `MAX_RECORDS`, `SplatSlot`, `SplatShared`, `PixelBufs`, `splat_records`, `encode_splat`, `draw_resolve`, `descs` | 300 | B49 | = |
| `stream.rs` | streamed cloud rows (typed at 43, pure file move) | 125 | B49 | = |
| **`backdrop.rs`** | **family: `grid.wgsl` + `background.wgsl`** — no rows, no `Gpu` field; `GRID_VERTS`, `descs`, `draw(pass, b, show_grid) -> u32` | 55 | B49 | **N** |
| `render.rs` | `INK_DEPTH_PREPASS`, `encode_frame` (3 fenced regions), **`scene_list`** | 100 | B49 | = |

### `src/engine/pipelines/`

| path | owns | ~ln | lesson |
|---|---|---|---|
| `layouts.rs` | `Layouts` — the single owner of all 9 bind-group layouts; `Layouts::new(device)`; `compute_entry` | 270 | B45 |
| `build.rs` | `Target`, `PipelineDesc`, presets `opaque`/`ink`/`sheet`/`depth_only`, `build`, `build_compute` | 175 | B45 (**rewritten**, 845 →) |
| `mod.rs` | `Pipelines { arena, seg, glyphs, splat, backdrop }` — **5 fields** of per-family `Pipes`; `Pipelines::new(ctx, t, &l)` **frozen at 3 params** | 205 | B45 |

### `src/app/`

| path | owns | ~ln | lesson |
|---|---|---|---|
| `manifest.rs` | `Manifest`, `Doc`, `auto_grid`, `placement` (`scene.rs:1-84`) | 95 | B50 |
| `knobs.rs` | `env_flag` + the 5 `OnceLock`s | 30 | B50 |
| `scene.rs` | `Scene`, `clear`, `rebuild`, `upload_to`, `add_file` (**~45 lines**: loop head + one `walk_geometry` call + the file sweeps) | 230 | B50/51 |
| `walk/mod.rs` | `Walk<'a>` + its **sink accessors**, `WalkCx`, `Caches`, `Row`, `Chain`/`RowKind`, `walk_geometry` (13 arms + the `FLAG_OPEN` mask), the `chain_table` test | 130 | B50 |
| `walk/encode.rs` | `encode_width`, `oct16`, `pack_facing`, `BLACK` — **document** conventions (row-format words stay engine-side, Q12) | 85 | B50 |
| `walk/bounds.rs` | `file_extent`, `sheet_thickness`, `mark_sheet` — the two file sweeps | 150 | B50 |
| `walk/curves.rs` | Line · Polyline · NurbsCurve → `SegRows` | 165 | B50 |
| `walk/frames.rs` | Plane · OBB → `SegRows` | 60 | B50 |
| `walk/points.rs` | Point → `GlyphRows` | 30 | B50 |
| `walk/cloud.rs` | PointCloud → `CloudRows` | 115 | B50 |
| `walk/mesh.rs` | Mesh → arena rows + the 5 gates + `MeshOpts` | 200 | B51 |
| `walk/mesh_ink.rs` | mesh edges → `SegRows.pipes`, mesh verts → `GlyphRows.spheres` | 255 | B51 |
| `walk/mesh_topology.rs` | the fused edges/edge_faces/normals/closed walk | 125 | B51 |
| `walk/brep.rs` | BRep adapter | 90 (155 by 85) | B51 |
| `walk/surface.rs` | NurbsSurface adapter | 90 (165 by 85) | B51 |
| `walk/trimmed.rs` | reserved — **costs nothing until 56** | 115 | 56 |
| `src/math.rs` | `Mat4`, `mat_mul`, `mat_to_f32`, `eye_from_view_proj`, `ortho_half_height`, `xform_point`, `grow_bounds`, `Bounds`, `Aabb64` | 200 | B45 |
| `app/persistence.rs` | unchanged in the block — **declared over cap (~453 lines after 43); 3-way split moves to 59** | 453 | **59** |
| `app/render.rs` | `impl State { render, resize }` + `ViewState` | 140 | **59** |

**End of block:** `Gpu` **18 fields** — `surface, ctx, config, layouts, pipelines, frame, targets, view, objects, arena, seg, glyphs, cloud, splat, stream, performance, scene_min, scene_max`. Largest engine leaves `mod.rs`/`objects.rs`/`segments.rs`/`splat.rs` at 300; largest app leaf `walk/mesh_ink.rs` 255; no function over 5 params; `backdrop.rs` is the only file with no `Gpu` field.

---

## 5. The contracts

### 5.1 A producer makes rows — the signature names the sinks

```rust
// app/walk/mod.rs
pub struct Bases { pub vert: u32, pub cloud: u32, pub node: u32, pub cloud_px: f32 }

pub struct Walk<'a> { up: &'a mut Upload, pub bases: Bases }     // the WRITE end
impl<'a> Walk<'a> {
    pub fn seg  (&mut self) -> &mut SegRows;                     // 0 params
    pub fn glyph(&mut self) -> &mut GlyphRows;                   // 0
    pub fn cloud(&mut self) -> &mut CloudRows;                   // 0
    pub fn mesh_sinks(&mut self) -> MeshSinks<'_>;               // 0 — three disjoint &mut, ONE statement
}
pub struct MeshSinks<'a> { pub arena: &'a mut ArenaRows, pub seg: &'a mut SegRows,
                           pub glyph: &'a mut GlyphRows, pub vert_base: u32 }

pub struct WalkCx<'a> { pub caches: &'a mut Caches, pub guid: &'a str }   // the READ end
pub struct Row { pub bounds: Option<Bounds>, pub spacing: f32, pub flags: u32 }
impl Row {
    pub const NONE: Row = Row { bounds: None, spacing: 0.0, flags: 0 };
    pub fn world_spacing(v: f32) -> Row { Row { spacing: v, ..Row::NONE } }
    pub fn point_size_px(v: f32) -> Row { Row { spacing: v, ..Row::NONE } }
}

// the producers — NARROW SINKS, never the whole Upload
pub(crate) fn walk_line      (s: &mut SegRows,   l: &Line,        ri: u32)                  -> Row; // 3
pub(crate) fn walk_polyline  (s: &mut SegRows,   p: &Polyline,    ri: u32)                  -> Row; // 3
pub(crate) fn walk_nurbscurve(s: &mut SegRows,   c: &NurbsCurve,  ri: u32)                  -> Row; // 3 (4 at 52: + cx)
pub(crate) fn walk_plane     (s: &mut SegRows,   p: &Plane,       ri: u32)                  -> Row; // 3
pub(crate) fn walk_obb       (s: &mut SegRows,   b: &OBB,         ri: u32)                  -> Row; // 3
pub(crate) fn walk_point     (g: &mut GlyphRows, p: &Point,       ri: u32)                  -> Row; // 3
pub(crate) fn walk_cloud     (c: &mut CloudRows, pc: &PointCloud, ri: u32, b: &Bases)       -> Row; // 4
pub(crate) fn walk_mesh      (s: &mut MeshSinks<'_>, m: &Mesh,    ri: u32, o: MeshOpts)     -> Row; // 4
pub(crate) fn walk_brep      (s: &mut MeshSinks<'_>, b: &BRep,    ri: u32)                  -> Row; // 3
pub(crate) fn walk_surface   (s: &mut MeshSinks<'_>, n: &NurbsSurface, ri: u32)             -> Row; // 3

pub struct MeshOpts { pub lane: IdxLane, pub closed: bool }      // the ONLY divergence between #1-#5
pub(crate) fn walk_geometry(w: &mut Walk<'_>, cx: &mut WalkCx<'_>, g: &Geometry, ri: u32) -> Row; // 4
```

- **W1 Narrowest sink.** A producer receives only the row groups it writes. `walk_line` **cannot** touch a cloud column, because it never received one. This is the compartment, enforced by the type system rather than by a comment, and it is the one place this revision diverges from revision 1's uniform `walk_<type>(w, cx, g, ri)`.
- **W2 One `Row` out, never a column push.** No producer writes `obj.rows`/`obj.bounds`/`obj.spacing`; `walk_geometry` does it once from the returned `Row`. That deletes eight copies of the two-push pair and makes `t.objects.last_mut().unwrap().2 |= FLAG_PRINT` unrepresentable.
- **W3 Functional update at every construction site:** `Row { bounds, spacing, ..Row::NONE }`. A later `Row` field costs one struct line, not fourteen literal rewrites.
- **W4 `walk_geometry` lives in `walk/mod.rs`**, never in `scene.rs` (settled Q14). `add_file` = the loop head + the `Element(None)` guard + one call + the file sweeps.
- **W5 Produce vs append.** Each producer file separates `sample_curve(...) -> Vec<Point>` / `mesh_ink(...) -> (Vec<CylinderSegment>, Vec<GlyphPoint>)` from the push, so 94 (edit) and 112 (LOD) can get one object's rows without a `Walk`.
- **W6 Zero `wgpu::` under `app/walk/`.** Asserted by a shape test (§5.6).
- **W7 Adapters, not copies.** `walk_brep`/`walk_surface` are 3 and 2 lines of their own and **re-enter** `walk_mesh` with a `MeshOpts`. A future divergence must be a new **field on `MeshOpts`**, visible in one struct — never a new statement in one adapter. This is precisely how `FLAG_OPEN` was lost at `scene.rs:356`.

### 5.2 A family consumes rows — no trait, one shape

```rust
// gpu/<family>.rs — every family file, in this order
const SRC: &str = include_str!("../../shaders/<name>.wgsl");        // one or two
#[repr(C)] #[derive(Clone, Copy, Pod, Zeroable)] pub struct <Row> { … }
const _: () = assert!(size_of::<<Row>>() == N);
#[derive(Default)] pub struct <Rows> { pub a: Vec<Row>, pub b: Vec<Row> }   // the CPU sink
pub fn layout(dev: &wgpu::Device) -> wgpu::BindGroupLayout;                          // 1
pub struct Pipes { pub p1: wgpu::RenderPipeline, … }
impl Pipes { pub fn new(ctx: &GpuCtx, t: Target, l: &Layouts) -> Self }              // 3
pub struct <Lane> { rows: GrowBuf, group: wgpu::BindGroup /* pub(super) */ }
impl <Lane> {
    pub(crate) fn new   (ctx: &GpuCtx, l: &Layouts) -> Self;                         // 2
    pub(crate) fn append(&mut self, ctx: &GpuCtx, rows: &<Rows>) -> bool;            // 2, true = REPLACED
    pub(crate) fn rebind(&mut self, ctx: &GpuCtx, l: &Layouts);                      // 2
    pub(crate) fn draw_<what>(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds<'_>) -> u32; // 2
    pub(crate) fn reset (&mut self);                                                 // 0
}
```

- **F1 One file per family**, `pub(super)` fields. The file **is** the compartment.
- **F2 `append` takes the family's own `Rows`, never `&Upload`.** `SegmentLane` is physically unable to name `cloud.pos`.
- **F3 A draw is `(pass, b)` and `&self`.** Groups 0-2 arrive in `Binds`; group 3 is family-owned. This is a **borrow-checker consequence**, not a style choice (§7 B2).
- **F4 A family never holds `Device`/`Queue`** — `&GpuCtx` is passed, which is what keeps every signature ≤3 params.
- **F5 A family never reads another family.** Cross-family frame steps stay `impl Gpu` in the file owning the shared resource (`splat.rs` owns `encode_splat`; `render.rs` owns `encode_frame`).
- **F6 Field names inside a family are today's minus the prefix** (`arena_vbo` → `vbo`, `pipe_count` → `pipes.count`), so a moved body is byte-identical modulo a ≤12-row `Replace-all` **inside one file**. This is the definition of the **Move** verb.
- **F7 No trait.** The set of families is closed and authored in-tree; draw methods differ in name and arity; every later lesson needs a *specific* family's fields. `Vec<Box<dyn Family>>` + downcast is strictly worse than five named fields.
- **F8 Shader knowledge lives in the family**, exposed as `descs(l: &Layouts) -> [PipelineDesc<'_>; N]`. `Pipelines` keeps named fields (needed for the MSAA rebuild) but stops being where shaders are known.

### 5.3 The frame is an ordered list

```rust
// gpu/render.rs
fn scene_list(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds<'_>) -> u32 {
    let mut d = 0;
    d += backdrop::draw(pass, b, self.view.show_grid);        // background.wgsl, then grid.wgsl
    d += self.arena.draw_faces(pass, b);                      // triangle.wgsl, depth write ON
    if self.view.show_edges { d += self.seg.draw_pipes(pass, b, self.view.line_style); }
    d += self.splat.draw_resolve(pass, b);                    // splat_resolve.wgsl
    d += self.glyphs.draw_markers(pass, b);                   // sphere.wgsl
    d += self.seg.draw_ribbons_depth(pass, b);                // INK_DEPTH_PREPASS
    d += self.glyphs.draw_dots_depth(pass, b);
    d += self.arena.draw_print(pass, b);                      // triangle_sheet, document order
    d += self.seg.draw_ribbons(pass, b);                      // ribbon.wgsl
    d += self.glyphs.draw_dots(pass, b);                      // glyph.wgsl
    d += self.arena.draw_text(pass, b);                       // lettering LAST of all
    d
}
```

Twelve lines that **are** the frame's documentation. The order carries real semantics (fills before ink before text); a render graph would hide it behind node registration and buys nothing at one pass, one thread, sixteen pipelines and a fixed order. Reordering is a line move.

### 5.4 Recipe — add a NEW geometry type

1. `src/app/walk/<type>.rs`: `pub const CHAIN: Chain = Chain { emits: &[RowKind::Ribbon], shaders: &["ribbon.wgsl"] };` and `pub(crate) fn walk_<type>(<narrowest sink>, g: &T, ri: u32) -> Row` — no `wgpu::`.
2. one `pub mod <type>;` in `walk/mod.rs`.
3. one arm in `walk_geometry`: `Geometry::X(x) => walk_x(w.seg(), x, ri),`.
4. inside the body, push into **one existing group** and return `Row { bounds, ..Row::NONE }`.

**Total: 1 new file + 2 lines. Zero engine edits, zero shader edits, zero `Upload` edits.** `chain_table` picks up the new row automatically. If the type needs a row format that does not exist, it is not a geometry-type change — it is §5.5, and the lesson must say which of the two the reader is doing before they type anything.

### 5.5 Recipe — add a NEW shader / family

1. `src/shaders/<name>.wgsl`.
2. `src/engine/gpu/<family>.rs` — the §5.2 shape: `SRC`, the `#[repr(C)]` row + size assert, `Rows`, `layout()`, `Pipes::new`, `Lane { new, append, rebind, draw_*, reset }`, `descs`.
3. `gpu/mod.rs`: one `pub mod`, one `Gpu` field, one `::new` in `build`, one line each in `set_scene`/`reset_arena`/`resize`.
4. `pipelines/layouts.rs`: one `Layouts` field + one block — **only if it needs a new layout**.
5. `pipelines/mod.rs`: one field + one `<family>::Pipes::new(ctx, t, l)` line.
6. `gpu/upload.rs`: one group field.
7. `gpu/render.rs`: one line in `scene_list`, positioned where the order demands.

**Total: 2 new files + 6-7 list lines, zero body edits.** A **second view of an existing row** (as `cylinder` is of `ribbon`) skips 4 and 6 entirely: it is one `PipelineDesc` in the family's `descs()` and one `draw_*` method in the family that already owns the row. That is 2 lines, and it is the payoff §8's lesson 48 makes the reader type.

### 5.6 Recipe — add a NEW uniform, knob, flag bit, or layout

| addition | recipe |
|---|---|
| **uniform** (per-frame, read by ≥2 families) | 1 field on `FrameUniforms` + 1 line in `write_camera` (data arrives via `FrameInput`) + 1 entry in the `mvp` layout's entry list. **A single family's uniform lives in that family's file instead.** |
| **knob** (user-facing, no GPU bytes) | 1 field on `View` + 1 `Default` line + 1 read at its use site. **Never** a `pub` field on `Gpu`; a knob is not a uniform — `show_grid` gates a draw, `line_style` picks a pipeline. |
| **flag bit** | 1 const in `instance.rs` from the budgeted 9-slot table + 1 const in each `.wgsl` that reads it + 1 line in the mirror test. `Instance` past 96 B is **banned**. |
| **layout** | 1 field on `Layouts` + 1 block in `Layouts::new`. `Pipelines::new(ctx, t, &l)` is frozen at 3 params so no lesson threads a layout through 14 desc literals. |
| **pipeline** | 1 `PipelineDesc` literal inside the owning family's `descs()`. **Never** a new builder fn. |

### 5.7 The shape test that keeps all of it

`src/app/walk/mod.rs`, `#[cfg(test)]`, ~25 lines, runs in milliseconds, lands in **lesson 50**:

```rust
#[test] fn compartments_hold() {
    for p in files("src/app/walk") {                       // W6, W1
        let s = read(&p);
        assert!(!s.contains("wgpu::"),        "{p:?} must stay CPU-only");
        assert!(!s.contains("CylinderSegment {"), "{p:?} must use CylinderSegment::new");
        assert!(s.contains("pub const CHAIN"), "{p:?} must declare its chain");
        assert!(s.lines().count() < 350,      "{p:?} is over the file budget");
    }
    for p in files("src/engine/gpu") {                      // the corrected litmus
        let s = read(&p);
        for bad in ["Scene", "Doc", "Geometry", "Session", "egui"] {
            assert!(!s.contains(bad), "{p:?} names {bad}");
        }
    }
}
```

`ARCHITECTURE.md` §2/§3 drifted from lesson 27 to 44 because it was prose. This is the only mechanism that keeps the compartments after the tutorial ends.

---

## 6. The data tables: what happens to `ArenaUpload`

**Decision: regrouped per family, in four instalments.** This reverses revision 1's Q11 note ("`ArenaUpload` stays flat, names unchanged") — that flat 18-column struct **is** what the user is objecting to, and the "drop 13, keep 3" rule it carries lives in a comment at `scene.rs:205-207` rather than in a type.

**Per-type upload buffers are rejected.** A column is a *draw call*: `seg.ribbons` is one `draw(0..4, 0..count)` over one buffer. Split per geometry type and a scene with 40 polylines becomes 40 buffers and 40 draws. Per-type buffers are also unsortable, and the sheet lanes exist precisely to control draw order **across** objects.

```rust
// engine/gpu/upload.rs — the ONE type that names both sides
#[derive(Default)]
pub struct Upload {
    pub obj:   ObjectRows,   // objects.rs   — NEVER dropped after upload
    pub arena: ArenaRows,    // arena.rs
    pub seg:   SegRows,      // segments.rs
    pub glyph: GlyphRows,    // glyphs.rs
    pub cloud: CloudRows,    // cloud.rs + splat.rs
    pub span:  Span,         // walk/bounds.rs
}
#[derive(Default)] pub struct ObjectRows { pub rows: Vec<ObjectBase>,
    pub bounds: Vec<Option<Bounds>>, pub spacing: Vec<f32> }
#[derive(Default)] pub struct ArenaRows  { pub verts: Vec<RenderVertex>, pub vids: Vec<u32>,
    pub idx: Vec<u32>, pub idx_print: Vec<u32>, pub idx_text: Vec<u32> }
#[derive(Default)] pub struct SegRows    { pub pipes: Vec<CylinderSegment>,
    pub ribbons: Vec<CylinderSegment> }
#[derive(Default)] pub struct GlyphRows  { pub spheres: Vec<GlyphPoint>, pub dots: Vec<GlyphPoint> }
#[derive(Default)] pub struct CloudRows  { pub pos: Vec<f32>, pub col: Vec<u32>, pub nrm: Vec<u32>,
    pub draws: Vec<CloudDraw>, pub nodes: Vec<LodNode> }
#[derive(Clone, Copy)] pub struct Span { pub min: [f32; 3], pub max: [f32; 3] }

impl Upload {
    /// The GPU is the only holder of these three groups after set_scene. `obj` is NOT dropped:
    /// the walk indexes it by global row and the instance table is rebased from it.
    pub fn drop_uploaded(&mut self) { self.arena.clear(); self.seg.clear();
                                      self.glyph.clear(); self.cloud.clear(); }
}
```

`upload_to`'s thirteen hand-written `drop_rows` calls and the three-line comment become **one method call**, and `obj` is *structurally* the group that is not cleared.

**The rename table — measured, and paid in four instalments.** Total **~173 qualified accesses + ~36 declaration lines across 4 files**: `app/scene.rs`, `engine/gpu/mod.rs`, `src/selftest.rs`, `examples/check_lean.rs`. Every site is compiler-checked; a count that differs means the wrong region was cut.

| instalment | lesson | old → new | sites | files |
|---|---|---|---|---|
| type rename | **46** | `ArenaUpload` → `Upload`, moved to `gpu/upload.rs` **flat** | 11 | 2 |
| `obj` | **47** | `objects` → `obj.rows` (24) · `object_bounds` → `obj.bounds` (15) · `object_spacing` → `obj.spacing` (14) | **53** | 4 |
| `arena` | **47** | `verts` → `arena.verts` (17) · `vids` → `arena.vids` (9) · `idx` → `arena.idx` (10) · `idx_print` → `arena.idx_print` (4) · `idx_text` → `arena.idx_text` (4) | **44** | 3 |
| `seg` + `glyph` | **48** | `pipes` → `seg.pipes` (16) · `segments` → **`seg.ribbons`** (10, leaf renamed — ledger seam 9's own name) · `spheres` → `glyph.spheres` (14) · `glyphs` → **`glyph.dots`** (5, leaf renamed) | **45** | 3 |
| `cloud` + `span` | **49** | `cloud_pos/col/nrm/draws/nodes` → `cloud.pos/col/nrm/draws/nodes` (17+) · `min`/`max` → `span.min`/`span.max` (14) | **~31** | 3 |

**What the grouping buys, concretely.** `SegmentLane::append(ctx, &SegRows)` cannot see `cloud`; `mesh_ink(&mut s.seg, m, ri)` cannot see `arena`; the drop invariant becomes a method; and a new family adds a field to a 6-field struct **that no producer sees**, instead of a 20th column every producer sees. Be honest with the reader about what it does *not* buy: disjoint field borrows already work on a flat struct, so grouping relieves no borrow — it **restricts a capability**, which is a design property, not a compiler property.

**One structural question the grouping surfaces and the block must answer.** `arena_vids` has **no cap of its own** — it rides `arena_vert_cap`, which is the structural cause of the "the on-disk fence forgets vids → slot-1 desync" hazard. Under `GrowBuf` this becomes explicit: `ArenaRows` gets **one** `GrowBuf` per buffer plus `debug_assert_eq!(vids.count, verts.count)` inside `Arena::append`. Decided at 47, three tokens, FREE-SHAPE.

---

## 7. Borrow-checker and ownership rules

The single most valuable Rust content in the block. Each rule is placed in the lesson **before** the step that hits it, in the 46-nurbssurface voice: the failing form, the error code, the compiling form, then the generalisation.

**B1 — two families in one `set_scene` body (E0499/E0502). Taught in 46, used in 47/48/49.** `self.seg.append(&self.ctx, &up.seg)` written as a `&mut self` method on `Gpu` fails, because *a method borrows all of `self`*. The compiling form borrows **fields**, which are disjoint places:
```rust
let Gpu { ctx, layouts, arena, seg, glyphs, cloud, splat, .. } = self;   // ONE destructure
if arena.append(ctx, &up.arena) { arena.rebind(ctx, layouts); }
if seg  .append(ctx, &up.seg)   { seg  .rebind(ctx, layouts); }
```
This is the single most likely place a hand-typing reader gets stuck; it gets its own paragraph.

**B2 — a draw method must be `&self`. Taught in 47, restated in 49.** `RenderPass<'e>` borrows the encoder for the pass's whole lifetime, so no `&mut self` method can run while it lives. **That is why F3 says `draw_*(&self, pass, b) -> u32`** — a borrow consequence, not a style rule. Say it here and F3 stops looking arbitrary.

**B3 — `Binds<'a>` survives the pass because it is all-shared.** `Binds { p: &Pipelines, mvp, time, line, cloud, instances }` — six shared reborrows into `frame`/`pipelines`/`objects`, held across a `RenderPass<'e>` that borrows only the encoder. Any future method taking `&mut Gpu` **and** a family reference will not compose; those go through a `Gpu`-level forwarder in the file owning the shared resource, and today that list is exactly `rebase_anchor` and `set_live_models`.

**B4 — three sinks at once, from one `&mut Upload` (E0499). Taught in 51.** `walk_mesh` needs `arena`, `seg` and `glyph` simultaneously. Calling `w.arena()` then `w.seg()` does **not** compile — two live `&mut` reborrows of `w`. The compiling form destructures the fields in one statement:
```rust
impl<'a> Walk<'a> {
    pub fn mesh_sinks(&mut self) -> MeshSinks<'_> {
        let Upload { arena, seg, glyph, .. } = &mut *self.up;      // three disjoint places
        MeshSinks { arena, seg, glyph, vert_base: self.bases.vert }
    }
}
```
The compartment gain is real: `mesh_sinks` names **exactly** the three families a mesh may touch, so a mesh can never write to `cloud`.

**B5 — `Walk` vs `WalkCx`, and the cache trap (E0502). Taught in 50, and it is the block's best teaching moment.** `Walk` borrows the field `self.tables`; `WalkCx` borrows the field `self.caches`; two disjoint `&mut` locals over disjoint `Scene` fields compile. The trap the seam ledger exists to prevent — and which the survey hit **independently at three separate lessons** — is a producer that takes both `w` and `cx` while its geometry argument is reborrowed *through* `cx`:
```rust
let m = cx.caches.tess.get(&k).unwrap();      // borrows *cx immutably
walk_mesh(&mut w.mesh_sinks(), m, ri, opts);  // ✓ w and cx are separate locals
// walk_mesh(&mut w, cx, m, ri, opts);        // ✗ E0502: cx borrowed mutably and immutably
```
**This is why producers take sinks and not `Walk`**, and why `cx` is passed only to the two producers (52, 53) that actually read a cache. Revision 1's uniform `(w, cx, g, ri)` re-opens this at lesson 53.

**B6 — nowhere in this design is a `RefCell`, an `Arc<Mutex>`, an `RwLock`, a channel, a `Send + Sync` bound, or a clone of any row table.** Single-threaded wasm, one scene, one pass: each would be pure cost and each `.lock().unwrap()` a footgun in a document a human types by hand.

**B7 — visibility is the compartmentalization.** Family-internal fields `pub(super)`; methods the app needs `pub(crate)`; `Gpu`'s family **fields** `pub(crate)` so `gpu.objects.write_row_flags(&gpu.ctx, row, f)` compiles as a disjoint-field borrow. Default to private. The current `Gpu` exposes ~60 `pub` fields, which means every one of them is API and no family can hold an invariant.

---

## 8. The revised 45-51 cut

**Seven lessons, each one idea, each one sitting.** Gate after **every step**: `cargo check --target wasm32-unknown-unknown --lib` — a step that cannot be made to compile is two half-steps that must be **merged**, never a declared window. Gate after every lesson, run **twice** (house rule): both targets `--all-targets`; the end-of-44 golden set; `VIEWER_LINE_STYLE=tubes`; `VIEWER_REBUILD=1`; `VIEWER_INCREMENTAL=1`; draw pairs **lion 4/1, cloud_mix 11/210892 (Tubes 10)**; `check_determinism` + `check_lean` wherever `scene.rs` moves. **Every lesson ends with a payoff the reader types, runs, sees and reverts.**

| # | title — **the idea** | what moves / what is new | compile checkpoints inside it | gate + payoff |
|---|---|---|---|---|
| **45** | **A pipeline is data, not a function.** Eleven builders at 7-11 params exist because a pipeline was modelled as code; they differ in 5 fields out of ~25. Rejected alternative named: a builder file per family. Also: **delete before you move** — code nothing draws must not be faithfully relocated. | **NEW** `src/math.rs` (`Mat4`, `mat_mul`, `mat_to_f32`, `eye_from_view_proj`, `ortho_half_height`, `xform_point`, `grow_bounds`, `Bounds`, `Aabb64`) + re-exports so `scene.rs`/examples are unedited. **NEW** `pipelines/layouts.rs` (the 9 layout blocks out of `Gpu::build`, + `compute_entry`). `pipelines/build.rs` **REWRITTEN** 845 → ~175 (`Target`, `PipelineDesc`, 4 presets, `build`, `build_compute`). `pipelines/mod.rs`: `Pipelines::new(ctx, t, &l)` 10 → **3 params**, 5 per-family `Pipes` fields, the 2 compute pipelines folded in. **DELETE** `edges.wgsl`, `build_edges_pipeline`, `Pipelines.edges`, `storage_buffer`. | after `math.rs` · after `layouts.rs` · then **one per desc group, each with its own golden**: ink descs (tubes golden) → opaque → sheet → depth-only → the 2 compute, deleting the matching old builder at each group | `Gpu` **113 → 103**; `build.rs` ~175; 14 render + 2 compute. **⚠ `build_ribbon_solid_pipeline` carries a live `VIEWER_NO_DEPTH → CompareFunction::Always` branch (build.rs:621) that `build_ribbon_pipeline` does not** — the descs must preserve it, and the lesson says so. **Payoff:** add a wireframe pipeline as one `PipelineDesc::ink(..)` literal, see it, revert (it was ~70 lines of copy-pasted builder). |
| **46** | **The floor is not a lane.** A buffer, its count and its cap are **one value** — that shape recurs **12 times** in `Gpu` (~33 fields) and is where 99 → 18 actually comes from. Everything that belongs to no family belongs beneath them all. Introduces `&GpuCtx` and **the destructure line** (§7 B1) before anything needs it. | **NEW** `buffers.rs` (`GpuCtx`, `GrowBuf`, `append_rows` 7 → 4 params, `append_index_run`, `zeroed_buffer`, `mk_rows_group`); `Gpu { device, queue }` → `ctx` (**Replace-all 41 + 27 hits**). **NEW** `upload.rs` — `ArenaUpload` → `Upload`, moved **flat**, names unchanged (11 sites). **NEW** `frame.rs`. **NEW** `targets.rs`. **NEW** `present.rs` (moved from 49). **NEW** `view.rs` (moved from 49). | after `buffers.rs` · after the `ctx` Replace-all · after `upload.rs` · after `frame.rs` · after `targets.rs` · after `present.rs` · after `view.rs` — **7 checkpoints, one per file** | `Gpu` **103 → 86**; goldens + determinism + lean + **a browser smoke check** (the goldens run `render_offscreen` and never touch `State::render`). **Payoff:** add a uniform field + one line in `write_camera`, tint the frame, revert. |
| **47** | **One row per object — and the first family that points at it.** The object row is the seam the whole design hangs on: exactly one `(model, tint, flags)` per guid, every row struct's `instance_id` pointing at it. Then `arena.rs` as the **worked example of the family contract**, done *with* the reader. Introduces the **Move** verb — no lesson in the curriculum has ever used it — with its own "how to type a Move" section. | **NEW** `instance.rs` (`Instance`, the 9-slot flag table, `REANCHOR_*`, **`instance_mirror` + `line_uniform_mirror` tests**). **NEW** `objects.rs` (`InstanceTable`, `ObjectBase`, `bounds_world()`). **NEW** `arena.rs` (`Arena`, `IdxLane`, `run`/`run_mut`, `INSTANCE_ID_ATTRIBS`, `Pipes`, `descs`, `draw_faces`/`draw_print`/`draw_text`, `append -> bool`, the `vids.count == verts.count` assert). `Upload.obj` group (**53 sites**) and `Upload.arena` group (**44 sites**). | after `instance.rs` · after the two mirror tests (**they fail loudly if a `.wgsl` copy already drifted** — run them before anything else) · after `InstanceTable::{new,append}` · after rebase/anchor/`update_inside_flags` · after the `obj` rename · after `arena.rs` struct + `new` · after `append` · after the three `IdxLane` draws · after the `arena` rename — **9 checkpoints** | `Gpu` **86 → 63**; goldens + REBUILD + INCREMENTAL + draw pairs. **Payoff:** add `FLAG_DEBUG` from the budgeted table, light one row from `triangle.wgsl`, revert. |
| **48** | **One row, two shaders — the module follows the DATA.** The central argument of the whole axis, and it earns a lesson: `ribbon.wgsl` and `cylinder.wgsl` read one identical `CylinderSegment` through one layout with the choice at one draw site, so they are **one module and five pipelines**. `sphere`/`glyph` is the same shape — so the second half is written **by** the reader against the stated contract, with the snapshot crate as the answer key. Names the two layout aliases the type system is not tracking. | **NEW** `segments.rs` (`CylinderSegment` + `::new`, `FACING_UNKNOWN`, `LineStyle`, `Template`, `unit_cylinder`, `CYL_SIDES`, `SegRows`, `SegmentLane`, `Pipes ×5`, `descs`, 3 draws). **NEW** `glyphs.rs` — reader-written (`GlyphPoint` + `::new`, `unit_quad`, `GlyphRows`, `GlyphLane`, `Pipes ×4`, `descs`, 4 draws). `Upload.seg` + `Upload.glyph` groups (**45 sites**). | after the `segments.rs` skeleton (an empty `impl` compiles) · after the row + template · after `Pipes`/`descs` · after each of the 3 draws · after the `seg` rename · then the reader's `glyphs.rs`, same five checkpoints | `Gpu` **63 → 43**; goldens Flat **and** Tubes + REBUILD; the litmus re-checked. **Payoff:** the reader's own `glyphs.rs` renders the byte-identical golden — and then they add a second *view* of an existing row (one `PipelineDesc` + one `draw_*`, 2 lines) and revert. |
| **49** | **The frame is a list you can read.** Point clouds are the only type that is its own compartment end to end — and the splat record, which has **no Rust type today** (36 words packed by four `extend_from_slice` calls, read back by literal index, its size written in three places), gets one. Then `encode_frame`'s **310 lines** become three fenced regions plus a 12-line `scene_list`, and `Gpu`'s constructor literal becomes 12 `::new` lines. Argued: a list, not a graph. | **NEW** `cloud.rs`, **NEW** `splat.rs` (incl. `#[repr(C)] SplatRecord`, `REC_WORDS`, `MAX_RECORDS`), **NEW** `stream.rs` (pure move), **NEW** `backdrop.rs` (`grid` + `background`, `GRID_VERTS` beside the shader's `FLOOR+6`), **NEW** `render.rs`. `gpu/mod.rs` final + the explicit re-export list so `lib.rs`/`selftest.rs`/the three examples compile unedited. `Upload.cloud` + `Upload.span` (**~31 sites**). | after `cloud.rs` · after `splat.rs` · after `SplatRecord` replaces the four `extend_from_slice` calls (**its own golden — a wrong word index puts the cloud in the wrong place at the wrong size with no error anywhere**) · after `stream.rs` · after `backdrop.rs` · after `render.rs` · after the `cloud` rename · after the `Gpu` literal → 12 `::new` lines — **8 checkpoints** | `Gpu` = **18 fields**; `gpu/mod.rs` ~300; `scene_list` 12 lines; ALL configs × 2 scenes × 2 runs + draw pairs + determinism + browser smoke. **Payoff:** move `draw_text` above `draw_ribbons`, watch lettering go under the ink, move it back. **⚠ highest-risk lesson: see the contingency below.** |
| **50** | **A producer's signature names the shaders it can reach.** The app axis, argued with the measured numbers — Line 10 lines, Point 10, Polyline 12, Plane 13, OBB 19 — so grouping is **by output row**, not by kernel name. Narrow sinks are the law: `walk_line(s: &mut SegRows, …)` cannot reach a cloud column. `walk_geometry` lives in `walk/mod.rs`, so 56/64/73 wrap a call instead of re-indenting 13 arms. | `scene.rs:1-84` → `manifest.rs`; `env_flag` + the 5 `OnceLock`s → `knobs.rs`; `encode_width`/`oct16`/`pack_facing`/`BLACK` → `walk/encode.rs`; **NEW** `walk/mod.rs` (`Walk` + sink accessors, `WalkCx`, `Caches`, `Row`, `Chain`, `walk_geometry`, `chain_table` + `compartments_hold` tests); converters 519-598 → `curves.rs`/`points.rs`; 1173-1207 → `frames.rs`; `push_cloud`/`cloud_spacing` → `cloud.rs`; the two file sweeps → `bounds.rs`. `Element(None)`'s unreachable arm deleted. | after `manifest.rs`+`knobs.rs` · after `encode.rs` · after `curves.rs` · after `points.rs`+`frames.rs` · after `cloud.rs` · after `bounds.rs` · after the sink accessors · after `walk_geometry` replaces the match — **8 checkpoints** | `scene.rs` ~620; every producer ≤4 params; determinism + lean + goldens **including `drawings`** (the six converter bodies are *retyped* onto the engine row ctors, not moved — its own checkpoint). **Payoff:** add a `Geometry::Circle` producer in one file + two lines, zero engine edits; `chain_table` grows a row; revert. |
| **51** | **Five types, one body: adapters, not copies.** `push_mesh` is **314 lines and 8 params** because it does four unrelated jobs; four of its params are `&mut Vec<..>` threaded by position, and its second return value is discarded by **three of five callers** — which is exactly how `Element(Mesh)` lost `FLAG_OPEN` at `scene.rs:356`. Split by job, then BRep and NurbsSurface become 3- and 2-line adapters that **re-enter** the mesh producer with a `MeshOpts`. | `mesh_topology` (766-828) → `walk/mesh_topology.rs`; `push_mesh` **892-1143** → `walk/mesh_ink.rs`; `push_mesh` **830-891** + the 5 gates → `walk/mesh.rs`; `walk/brep.rs`; `walk/surface.rs`; the five arms → one-liners; `push_mesh` gone. `MeshOpts` replaces the 8 params and the tuple return. **The `Element(Mesh)` `FLAG_OPEN` mask is kept** (Q13(2)). | after `mesh_topology.rs` · after `mesh_ink.rs` · after `mesh.rs` · after `brep.rs` · after `surface.rs` · after the five arms — **6 checkpoints** | `scene.rs` ~230; `add_file` ~45; largest app leaf 255; **no fn over 5 params; `compartments_hold` green**; goldens + determinism + lean, twice. **Payoff:** the reader fills in §2's 15th row for their own `Circle` and watches `chain_table` print the complete map. |

**Predicted field ladder: 113 → 103 → 86 → 63 → 43 → 18.** It is arithmetic, not a measurement (revision 1 §12 item 2): count `Gpu`'s fields on the built end-of-44 tree and correct every gate. Today's measured anchor is **99**.

**Two named contingencies, both decided BEFORE 45 is typed** (either splits the block and therefore renumbers 63 files):
1. **49 is the highest risk** — 6 new files plus the `gpu/mod.rs` collapse. Split point if its measured draft exceeds one sitting: the point families (`cloud`/`splat`/`stream`) | the frame (`backdrop`/`render`/`mod`).
2. **47 is second** — 3 new files plus 97 rename sites. Split point: `instance`+`objects` | `arena`.

**Two artefacts must exist before 45 is drafted.** (a) The end-of-44 goldens, recorded **twice**; nothing in the block can carry an Expected-state block in the house form until they exist, and lesson 43's standard — "measured, not estimated" — is the curriculum's one credibility rule. (b) `docs/_moved_check.sh`: sorted, trimmed, non-blank line multiset of the pre-lesson file against the union of the post-lesson files, whose expected diff is exactly the re-rooted paths plus the counted new lines. The compiler proves a Move type-checks and the golden proves the pixels agree; **neither proves a Move was byte-identical**, and a dropped line inside a `#[cfg]` arm passes both.

---

## 9. How a refactor lesson is written

A feature lesson pays the reader in pixels; a refactor lesson has nothing to show but a picture that did not change, so it must pay them in a **model** and a repeatable **method**, and it must never leave them unable to tell a typo from the plan. Section-by-section template — **one compartment idea per lesson; if it has two, it is two lessons.**

**0. Title + epigraph (4-6 lines).** The title names the **compartment**, not the files: *"48 — One row, two shaders: the module follows the data"*. The epigraph states three clauses: what this makes possible in one line later (cite the lesson number), the promise that the picture does not change, and the snapshot crate path for `diff -u`. Then the block banner, once per lesson: *"Lessons 45-51 move code. Every body you cut is pasted byte-identical except for `self.x` → `self.y` path re-roots inside ONE file; if you find yourself improving a line while moving it, stop — the deferral list at the end says which lesson owns that change."*

**1. Why this seam — before the first edit.** Three parts, in order.
 *1a. The evidence, reproducible with one command, never asserted.* `grep -cE '^\s+(pub )?[a-z_0-9]+\s*:' <(sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs)` → the reader must re-run it on their own tree and get the doc's number.
 *1b. The one law this seam enforces*, named from §5 (F1…F8, W1…W7, B1…B7). One law per lesson, stated as what it **forbids** — that is what makes it testable at the end.
 *1c. The rejected alternative*, in 43's voice: *"The obvious cut is X. Do not make it."* — 45: a builder per family. 46: `RowTable<T>` now. 47: `Instance` staying inside `objects.rs`. 48: one `InkLane` with a style flag. 49: a render graph. 50: `Walk.caches` as a field. 51: `brep.rs` copying the mesh body. Two to four sentences, ending in the future lesson number that would have re-opened the wrong choice.

**2. Where the code lives after this lesson.** (a) An **ownership table**, four columns: `symbol | today's home | new home | who may touch it` — the fourth column is what makes it a compartment and not a filing system. (b) An inline SVG or fenced ASCII of the compartment, arrows labelled with what crosses the boundary (`&GpuCtx` down, `bool grew` up, `&SegRows` in), captioned with the lesson's **exit litmus verbatim** so the reader can grep it at the end. (c) **For every lesson touching a geometry type or a shader (47, 48, 49, 50, 51): the relevant rows of §2's end-to-end chain table**, filled in as far as this lesson takes them. That table is R1 answered on the page, and 51 re-draws it complete as the block's exit artefact.

**3. Files we touch.** `file | NEW / MOVED-FROM / DELETED / RE-ROOTED | step | one-clause reason`, with a **line budget for every NEW file** so a bad paste is visible by size alone.

**4. The destination skeleton — created FIRST.** `**Create** src/engine/gpu/<family>.rs` with the header doc comment (naming the one or two `.wgsl` files it owns), the `use` lines, the struct with `pub(super)` fields, and an **empty** `impl`. Then one `pub mod` line. Gate: `cargo check` — an empty impl compiles. Said to the reader: *the file's shape is the idea; the bodies are furniture.* It also makes every subsequent Move use `**at the end**`, which removes the invisible ordering dependency chained anchor-Moves create.

**5. The steps — ordering law, and a gate on every one.**
 (i) **Leaves before roots:** consts → `#[repr(C)]` rows → free fns → `impl` methods → the `Gpu` fields → the struct literal in `build` → the call sites. Nothing is ever left naming a symbol that has not moved.
 (ii) **One compartment per step**, and a step is the complete round trip: extend the destination → Move the bodies `**at the end**` → `Replace-all` the paths **inside the new file** → move the fields → fix the residual call sites → delete the dead forwarders.
 (iii) **The god-struct's field list changes last within a step, never between steps.**
 (iv) **Never split a Move from its `Replace-all`** — they are one edit; the compiler is only a gate when they land together.
 Per-step text, fixed shape: one sentence of *why this body belongs in this file* (§1b's law applied to this body — no step is a bare Move) · the Move written as `**Move** src/engine/gpu/mod.rs <first line, byte-exact> **through** <last line> **to** src/engine/gpu/<family>.rs **at the end**`, with the region's **line count** and its last line quoted again in prose so the reader can confirm the selection before cutting · the `Replace-all` **with its asserted hit count** and the sentence *"if the count differs, you cut the wrong region"* · the destination file's **table of contents so far** (3-6 fenced lines with line counts — this is what makes a Move *visible*; without it the reader has copied text they never read) · **the gate**, as a literal command with expected output, `cargo check` on every step and the golden on every second or third.
 Say once, in 47, and reference thereafter: **how to read a refactor's error wall** — `cargo check 2>&1 | grep -c '^error'` is your progress bar; fix only the **first** error and re-run, because 200 E0609s are usually one missing `Replace-all`.

**6. Where the borrow checker bites — placed BEFORE the step that hits it.** A blockquote with the failing form, the error code, the compiling form, the rule, then *"it recurs every time …"*. The five from §7 have scheduled homes: B1 → 46, B2 → 47, B3 → 49, B4 → 51, B5 → 50. During a refactor a red compiler reads as *"the refactor was wrong"*; pre-empting the exact error with its rule converts a dead end into the lesson's most valuable Rust content.

**7. Proving nothing changed — three ladders, with commands, and what each one CANNOT catch.** (1) The compiler: both targets, `--all-targets` native — proves it type-checks. (2) `docs/_moved_check.sh` — the sorted trimmed-line multiset, the reader's only proof a Move was byte-identical; the expected diff is printed in the doc. (3) The pixel goldens, run **twice**, both runs quoted, plus `check_determinism`/`check_lean` where `scene.rs` moves and a **browser smoke check** wherever `State::render`/`present` is touched — the goldens go through `render_offscreen` and never exercise those.

**8. What you can now do in one line — the payoff, mandatory, never skipped.** The reader types the additive change, runs it, sees it, reverts it (§8's last column). This is what converts seven identical goldens into seven visible wins, and it is the honest test of whether the seam was worth typing.

**9. What is deliberately not here.** Bullets from the seam ledger's DEFER rows, each with the lesson number that owns it (`RowTable<T>` → 57, `upload_rows` → 62, `Frustum` → 62, `persistence` split → 59, the WGSL prelude → 104, the `Spacing` enum → first lesson needing both units). Plus the standing rule: **a body you are moving is not a body you are fixing.** For 50 and 51 this section must additionally carry *"why the upload table is grouped per family and not per geometry type"*, with the numbers from §6 — otherwise the reader reads the split as the refactor dodging their request.

**10. Expected state · Recap · Edited · Reference · Next.** Exact commands, exact numbers, both runs, and the field count in human terms (*"`Gpu` 86 → 63 fields; `gpu/mod.rs` 2,162 → 1,180 lines — you can now read it end to end"*). Recap as a fenced continuous-prose block, one paragraph per lesson in the chain, ending with this lesson's law restated. `Edited:` one line naming every file with a parenthesised summary. Reference: the snapshot crate path, the literal `diff -u` command, the per-step commit table — **including any dead-end commits kept on purpose**, with a note saying what happened. Next: one paragraph naming the next compartment and the one line of §1's evidence that motivates it.

---

## 10. Deltas against revision 1 of `_ARCHITECTURE_TARGET.md`

1. **`ArenaUpload` is regrouped per family** (§6), reversing the "stays flat, names unchanged" note in Q11 and §8-delta-5. Paid in four instalments of 31-97 sites inside the lesson that creates the consuming family, never as one table. Renamed `Upload` and moved to `gpu/upload.rs`; `tables.rs` no longer exists as a name, and the row structs are **evicted to their families** (`CylinderSegment` → `segments.rs`, `GlyphPoint` → `glyphs.rs`, `Instance` → `instance.rs`, `CloudUniform` → `frame.rs`+`splat.rs`).
2. **Producer signatures are narrow-sink, not uniform 4-param** (§5.1, W1). Revision 1's `walk_<type>(w, cx, g, ri)` hands every producer the whole table and re-opens the cache E0502 at lesson 53 (§7 B5). `walk_geometry` keeps its 4 params via `Walk`'s sink accessors.
3. **`grid.wgsl` + `background.wgsl` merge into `gpu/backdrop.rs`** (55 lines, no `Gpu` field), removing the only two inline `set_pipeline` blocks from `scene_list` so it is uniformly one line per family.
4. **`present.rs` and `view.rs` move 49 → 46**; **`app/render.rs` and the `persistence.rs` 3-way split move out of the block to 59.** Rationale: they are the app spine and a size problem, not compartmentalization, and their presence is what made revision 1's 51 three lessons in one. This retires §8-delta-2 and partially overturns Q9 (the State-spine pattern now lands at 59, judge 1's original position). **`persistence.rs` is declared over cap for the duration of the block.**
5. **The WGSL prelude is scheduled at 104**, with `PRELUDE_LINES` subtracted by that lesson's error scope. Lesson 47 instead ships the free `instance_mirror` + `line_uniform_mirror` tests, which change no shader text and therefore cost nothing under the pixel gate. This partially overturns S3c's blanket REJECT: the prelude is right on the merits — it is the only mechanism that turns an `Instance` field addition into a compiler error rather than a wrong picture — but it edits nine shaders and does not belong in a moves block.
6. **`CHAIN` + `chain_table` + `compartments_hold`** (§5.6-5.7) are new: §2's map is generated and asserted, not written.
7. **`Row::world_spacing` / `Row::point_size_px`** (§5.1) name the two units that share `Instance.spacing` today. Zero behaviour change; the `enum Spacing` full fix is deferred.
8. **`MeshOpts { lane, closed }`** replaces `push_mesh`'s 8 positional params and its two-value return; `walk_brep`/`walk_surface` re-enter `walk_mesh` (§5.1 W7).
9. **Corrected numbers** carried into every Expected-state block: `.wgsl` = **10**, not 11 · `Gpu` = **99** today, not 98 · `push_mesh` = **314** lines, not 313 · `encode_frame` = **310** lines, not 275 · `ArenaUpload` = **18** columns · the `(buffer, count, cap)` shape recurs **12** times, not 8 · **`arena_vids` has no cap of its own** (§6) · the `Upload` regroup is **~173 sites across 4 files including `examples/check_lean.rs`**, not 3.
10. **`build_ribbon_solid_pipeline` is not a byte-identical twin of `build_ribbon_pipeline`**: it carries a live `VIEWER_NO_DEPTH → CompareFunction::Always` branch at `build.rs:621` that the free-lane builder lacks. The desc rewrite must preserve it, and lesson 45 says so.
11. **Field ladder** restated as `113 → 103 → 86 → 63 → 43 → 18`, with the final 18 enumerated (§4) and the standing instruction to re-measure on end-of-44.
12. **Every lesson gains: a payoff step, a per-step compile gate, a scheduled borrow-checker blockquote, and a chain-table section** (§9) — the four things the pedagogy audit found missing from revision 1 §8.

---

## 11. What did NOT change, and why

- **The lesson count is seven, 45-51, and the +7 renumber of 63 files stands** (Q16). Two contingencies are named with their split points and decided before 45 is typed; neither is taken speculatively.
- **The three classes (FREE-SHAPE / PRE-SEAM / DEFER), the four pre-seam admission conditions, and the ≤20-per-lesson, ≤130-per-block ceiling** (§1, Q15) are unchanged and govern every new line in §8.
- **Rule 2's definition of a Move** — byte-identical modulo whole-word path re-roots inside ONE file, a ≤12-row `Replace-all` the compiler checks — is unchanged (F6/L5). The block's one sanctioned rewrite is still `pipelines/build.rs`.
- **No trait, no render graph, no ECS, no `Box<dyn Family>`, no pipeline-specialization cache, no material system, no macro-generated families** (Q8, F7). Sixteen pipelines, one pass, one thread, a fixed draw order: a straight-line `scene_list` naming twelve draws in order is both the fastest implementation and the best documentation of the frame.
- **`Pipelines` keeps named fields and `Pipelines::new` is frozen at 3 params** (Q13(1), B8), because the MSAA flip rebuilds every pipeline and lesson 114's id pass re-runs `scene_list` against a second `Pipelines` set. The one change is that shader knowledge moves into each family's `descs()`, which is what makes 114 cheaper rather than merely possible.
- **`RowTable<T>` stays deferred to 57** (Q5): its CPU mirror is exactly the second copy of the scene that lessons 37/38 deleted (263 MB freed on lidar), and `append_rows` already returns the `grew` bool that is the only part 45-51 needs.
- **`upload_rows` → 62, `Frustum` → 62/76, `new_sized` → 107, the tessellation producers → 82, `Row.bounds` fill → 61** (the last being the only deliberate pixel-relevant change after 51, and the reason `Instance.extent` must be gated in the same lesson).
- **The object-table invariant, the flag-bit budget, the 96-byte `Instance` ban, and overlay-owned instance rows** (§5, Q3) are unchanged — they are what makes `walk_geometry` able to return a small `Row` instead of pushing three parallel columns eight times.
- **Q12's ownership split survives intact**: `pack_rgba`, `FACING_UNKNOWN`, `CylinderSegment::new`, `GlyphPoint::new` are **engine** truth (they move into `segments.rs`/`glyphs.rs`, not into a new `app/rows/`); `encode_width`, `oct16`, `pack_facing` are **app** truth in `walk/encode.rs`. An `app/rows/` layer would make `engine/gumball.rs` (74) and `app/tools/*` (80/81) import upward.
- **Q13(2) survives: the `Element(Mesh)` `FLAG_OPEN` mask is kept forever.** No golden scene contains an Element, so restoring the bit inside a pixel-gated block ships an unmeasured change; the removal is documented as a separate, separately-gated non-refactor edit.
- **The corrected engine/app litmus** — *`engine/` names no `Scene`, `Doc`, `Geometry`, `Session`, `egui`, CLI or demo symbol; kernel math types are the one allowed vocabulary* — is unchanged and is now **executed** by `compartments_hold` rather than asserted in prose.

---

# Carried forward from revision 1

The sections below stand as written in the 2026-08-29 spec: the two rules and the three
classes of change (§1), the frame (§4), the object-table invariant (§5), decisions Q1-Q16
(§6), the seam ledger (§7), the landing map for every lesson 52-114 (§9), the B1-B8
cross-cutting rules (§10), the renumber map (§11) and the open risks (§12). Revision 1's
§2 (target tree), §3 (contracts) and §8 (the 45-51 cut) are DELETED — revision 2 above
replaces them. Where a carried section disagrees with revision 2 on a measured number,
revision 2 is right (its header table lists every re-measurement).

## 1. Purpose and the two rules

The viewer is `engine/gpu/mod.rs` = 2,120 lines with a **98-field** `Gpu` (`build` 352-935 = 584, `set_scene` 944-1129 = 186, `encode_frame` 1534-1808 = 275), `app/scene.rs` = 1,289 with `add_file` = 305 and `push_mesh` 831-1143 = 313 lines / 8 params, and `pipelines/build.rs` = 845 lines of 11 copy-pasted builders at 7-11 params. `ARCHITECTURE.md` §3's rules (one concern per file, ~300-line soft cap, real `mod`s, ≤5 params) were locked at lesson 01 and drifted from lesson 27.

This document defines the tree the whole curriculum ends in — not just the restructure — so that **every lesson after 51 lands by adding a compartment and naming it once in a list.**

### Rule 1 — lessons 52-114 (on-disk 45-107) are additive

A later lesson lands as: **new file(s) + one `pub mod` line + one struct field + one `::new` line + one list line + one match arm + one desc literal.** It never re-opens a body that 45-51 placed. Four exceptions are declared and budgeted (§12); everything else is a defect in the design, not in the lesson.

### Rule 2 — lessons 45-51 are moves only, under a pixel gate

A moved body is **byte-identical modulo whole-word path re-roots inside ONE file** (`self.arena_vbo` → `self.vbo`, `self.device` → `self.ctx.device`) — a ≤12-row Replace-all the compiler checks, never a cross-file rename table (the dead on-disk 43 died of a ~280-row one). One sanctioned rewrite: `pipelines/build.rs`, 845 → ~150 lines around `PipelineDesc`, gated field-by-field by the tubes golden.

New code inside the block is classified in **three classes** (grafted from the minimal-seam proposal; all three judges):

| class | definition | charged | optional? |
|---|---|---|---|
| **FREE-SHAPE** | the shape a body is cut into *while it is already being moved* — a list instead of two dispatch pairs, an entry array instead of six positional buffers, an enum instead of two fields, a named struct instead of a tuple, a `LoadOp` parameter instead of a hardcoded literal, a bool that already exists, an explicit `drop(pass)`, R5's prefix trim | **0 lines** | **never** — getting it wrong is precisely what forces a later move |
| **PRE-SEAM** | genuinely NEW code typed inside 45-51 | counted | only under the four conditions below |
| **DEFER** | a field, a method on an existing impl, a match arm, an enum variant, a `use`, a layout entry, a list line, a desc literal, or a whole new file plus one `pub mod` line | 0 | always deferred to the lesson that needs it |

**Pre-seam admission (all four must hold):** (1) a **named** later lesson would otherwise MOVE, re-cut, re-indent or COPY a body 45-51 placed, or make `engine/` name an app symbol; (2) behaviour-neutral, compiles, and provable by the end-of-44 golden set run twice; (3) an open shape (list / enum / field / bool / parameter / accessor) — **never a trait, generic or wrapper type**; (4) ≤20 new lines in its lesson, ≤130 across the block.

**Measured spend:** 45 = 14 · 46 = 22 · 47 = 18 · 48 = 26 · 49 = 20 · 50 = 22 · 51 = 8 → **130 lines** against a ~6,700-line tree (1.9%). Reclassifying `encode_splat`'s batch array, the `View` field moves, `stream.rs`, `ObjectBase`/`CloudDraw`, `IdxLane`, `Arena::append -> bool` and the depth `Texture` as FREE-SHAPE is what takes lesson 49's discretionary new code from ~45 lines to 20 — the difference between 49 fitting one sitting and forcing an eighth lesson that would renumber 63 files.

### Baseline (hard correction)

The block is cut on the **end-of-44 tree**, not today's. Today's `Gpu` is **98 fields** (`mod.rs:223-334`; end-of-39 = 97, `edl_strength` already typed in the working diff). End-of-44 adds `last_ortho_h` (40), 11 `stream_*` fields (43), `lod_split_px` + `last_eye` + `cloud_nodes` (44) → **113 fields**. The plan's `97 → 86 → 73 → 50 → 28 → 19` ladder is end-of-39 arithmetic and is wrong at every checkpoint; the correct ladder is **113 → 102 → 89 → 66 → 44 → 18** (§8), and every source line range in every lesson must be **re-measured on that tree** — today's `gpu/mod.rs` tail already runs 8-12 lines ahead of the numbers both proposals quote (`zeroed_buffer` 2087 not 2075, `unit_cylinder` 2033 not 2025, `eye_from_view_proj` 1415 not 1404, `update_inside_flags` 1504 — note the name, not `update_inside`).

### Engine/app litmus (restated — the old form is already false)

`grep -n session_rust src/engine/` → `gpu/mod.rs:16` and `pipelines/build.rs:1`. `ARCHITECTURE.md` §2's "zero references to `session_rust`" is unachievable and `FrameInput { view_proj: &Xform, anchor: Option<&Point> }` extends it. The rule becomes:

> **`engine/` names no `Scene`, `Doc`, `Geometry`, `Session`, `egui`, CLI or demo symbol. Kernel *math* types — `Xform`, `Point`, `Vector`, `Plane`, `RenderVertex`, `Mat4` — are the one allowed vocabulary.** Row *format* (`pack_rgba`, `FACING_UNKNOWN`, `CylinderSegment::new`, `GlyphPoint::new`) is engine truth; *document* conventions (`encode_width`, `oct16`, `pack_facing`) are app truth.

Lesson 51's exit gate asserts that form, not the unachievable one.

---

## 4. The frame

### `engine/gpu/render.rs` — three ordered regions in ONE body

```rust
pub const INK_DEPTH_PREPASS: bool = false;

impl Gpu {
    pub(crate) fn encode_frame(&mut self, encoder: &mut wgpu::CommandEncoder,
                               view: &wgpu::TextureView, color: wgpu::Color) -> (u32, u32) {
        // ── A. compute prelude ───────────────────────────────────────────────
        self.encode_splat(encoder);            // 109 seg_cull · 111 meshlet_cull · 113 hiz test
        // ── B. scene pass ────────────────────────────────────────────────────
        let b = self.frame.binds(&self.pipelines, &self.objects.bind_group, &self.material);
        let mut pass = self.targets.begin_pass(encoder, view, wgpu::LoadOp::Clear(color),
                                               Some(wgpu::LoadOp::Clear(0.0)));
        let draws = self.scene_list(&mut pass, &b);   // ← the twelve draw lines, factored (see below)
        drop(pass);                                    // explicit — region C needs a real seam
        // ── C. post ──────────────────────────────────────────────────────────
        //   74 overlay pass · 88 encode_ao · 90 encode_outline · 88 encode_composite
        //   93 text (in B) · 113 hiz pyramid · 114 encode_id
        (draws, self.objects.rows.len() as u32)
    }

    fn scene_list(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds<'_>) -> u32 {
        let mut d = 0u32;
        pass.set_pipeline(&b.p.background); pass.draw(0..3, 0..1); d += 1;
        if self.view.show_grid { pass.set_pipeline(&b.p.grid); pass.set_bind_group(0, b.mvp, &[]);
                                 pass.set_bind_group(1, b.line, &[]); pass.draw(0..50, 0..1); d += 1; }
        d += self.arena.draw_faces(pass, b);                       // 84 trim · 106 group 3 · 111 indirect
        if self.view.show_edges { d += self.seg.draw_pipes(pass, b, self.view.line_style); }
        d += self.splat.draw_resolve(pass, b);
        d += self.glyphs.draw_markers(pass, b);
        d += self.seg.draw_ribbons_depth(pass, b);                 // INK_DEPTH_PREPASS
        d += self.glyphs.draw_dots_depth(pass, b);
        d += self.seg.draw_ribbons(pass, b);                       // 80 preview · 81 marker follow
        d += self.arena.draw_text(pass, b);
        d += self.glyphs.draw_dots(pass, b);
        d                                                           // 86 ground · 93 text · 107 impostor
    }
}
```

`scene_list` is a **FREE-SHAPE** cut taken while the body is already moving (~3 lines). It is what lets lesson 114's id pass re-run the identical list against `Pipelines::new_id(device, Target { samples: 1, format: R32Uint }, &l)` with zero lane edits — removing the plan's last "never re-open a body" violation. *Judges split here: judge 1 grafts this at 49; judges 2 and 3 leave the factoring to 114 as a declared exception. I followed judge 1, because under the three-class rule the cut costs nothing while the body is in motion and it converts a late-lesson body split into a call.* Draw counts move inside their methods with today's values; the asserted pairs stay **lion 4/1, cloud_mix 11/210892 (Tubes 10)**.

### `engine/gpu/present.rs` — the split

```rust
pub struct Frame { surface: Option<wgpu::SurfaceTexture>, pub view: wgpu::TextureView,
                   pub encoder: wgpu::CommandEncoder }

impl Gpu {
    /// None = nothing to present (headless, or the surface was lost and reconfigured).
    pub fn begin_present(&mut self, color: wgpu::Color, view_proj: &Xform)
        -> anyhow::Result<Option<Frame>> {
        self.begin_frame(view_proj);
        let Some(surface) = &self.surface else { return Ok(None) };          // headless
        let output = match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t) | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            _ => { surface.configure(&self.ctx.device, &self.config); return Ok(None) }  // lost
        };
        let view = output.texture.create_view(&Default::default());
        let mut encoder = self.ctx.device.create_command_encoder(&/* "clear encoder" */);
        let (draws, objects) = self.encode_frame(&mut encoder, &view, color);
        self.performance.frame(draws, objects);
        Ok(Some(Frame { surface: Some(output), view, encoder }))
    }
    pub fn end_present(&mut self, f: Frame) {
        self.ctx.queue.submit([f.encoder.finish()]);
        if let Some(o) = f.surface { o.present(); }
    }
    /// Unchanged 2-arg signature — state.rs, 87's early return and 62/55's anchors depend on it.
    pub fn clear(&mut self, color: wgpu::Color, view_proj: &Xform) -> anyhow::Result<()> {
        if let Some(f) = self.begin_present(color, view_proj)? { self.end_present(f); }
        Ok(())
    }
}
```

`Option<Frame>` is **not** cosmetic: today's `clear` (`mod.rs:1294-1312`) has exactly two early returns — `self.surface == None` and the lost-surface reconfigure — and `Frame.view` needs a real `TextureView`. Neither proposal saw this; judge 1 found it and judges 2/3's Q7 verdicts are compatible.

`Frame`'s public `view`/`encoder` are how `ui/` encodes the egui pass without `engine/` ever naming `egui` (69), and how 74's gumball pass, 107's impostor raster and 114's id pass + copy append to the **same** encoder in a fixed order. **Ordering law:** the gumball overlay pass precedes the egui pass (egui must be last, or the MSAA resolve blits over the UI); enforced by the order of the two calls in `app/render.rs`.

**Gate note (must be stated in lesson 49 and in `ARCHITECTURE.md` §3):** `examples/selftest.rs` builds `Gpu::new_headless` and calls `render_offscreen`, which runs `encode_frame` directly — it **never constructs `State`**. So the pixel gate covers `encode_frame` only; `clear`, `begin_present`/`end_present` and the lesson-51 `impl State` move are **compiler-gated**. Lesson 49 therefore ends with a browser smoke check, and `clear` is written as the literal composition above so its behaviour cannot drift.

---

## 5. The object table invariant

`engine/gpu/objects.rs` — **`InstanceTable` is ONE invariant over parallel columns, and the only place a row is mutated.**

| column | type | filled by |
|---|---|---|
| `rows` | `Vec<Instance>` (96 B) | `append` |
| `table` | `GrowBuf` (the GPU mirror) | `append` / `upload_rows` |
| `bind_group` | `wgpu::BindGroup` | `new`, rebuilt on replace |
| `base` | `Vec<ObjectBase { model: Mat4, color: [f32;4], flags: u32 }>` | `append` — the TRUE world transform |
| `base_f32` | `Vec<[f32;16]>` | `append` — cached `mat_to_f32`, rebase re-patches 3 slots |
| `bounds_world` | `Vec<Option<Aabb64>>` | `append` — **THE one world-AABB cache** |
| `bounded_rows` | `Vec<u32>` | `append` — rows with `Some(bounds)`; the FLAG_INSIDE walk |
| `inside` | `Vec<bool>` | `update_inside_flags` |
| `culled` | `Vec<bool>` | 62 `update_culled` |
| `last_origin`, `last_rebase_ms` | rebase state (200 ms throttle) | `rebase` |

**Methods (all `pub(crate)`, all ≤5 params, `ctx` first):** `new` · `append` · `rebase -> bool` · `anchor` · `update_inside_flags` · `reset` · `bounds_world() -> &[Option<Aabb64>]` — then, added by their own lessons: `set_row` (58), `update_culled` + private `upload_rows(ctx, &[u32])` (62), `write_row_flags` / `set_flag_rows` (67), `write_models(ctx, &[(u32, Mat4)])` incl. `base_f32` (76), `mask_range`/`restore` (107), `boxes` upload (110).

**Append-only.** `set_scene` does `let base = self.objects_base.len(); debug_assert!(up.objects.len() >= base, "the object table only ever grows"); extend_from_slice(&up.objects[base..])`. A pushed reserved row makes `base > up.objects.len()`: the assert fires in debug and the slice **panics**; in release a real object is skipped. Hence **overlays own their own 1-row instance buffer** (Q3) — never `reserve_row`.

**Flag-bit budget** (`instance.rs`, ONE documented table):

| bit | name | lesson |
|---|---|---|
| 0 | `FLAG_SELECTED` | 67 |
| 1 | `FLAG_HIDDEN` | exists |
| 2 | `FLAG_INSIDE` | exists |
| 3 | `FLAG_PRINT` | exists |
| 4 | `FLAG_OPEN` | exists |
| 5 | `FLAG_SHEET` | exists |
| 6 | *free* | — |
| 7 | `FLAG_CULLED` | 62 |
| 8, 9 | **reserved: LOD level** | 112 |

Widening `Instance` past 96 B is banned: `struct Instance` is hand-mirrored in five `.wgsl` files, so a stride change is a six-file edit. 110's per-object box rides a **sibling `boxes` GrowBuf at instance-layout binding 1**, which 110/111/113 then share.

**`Instance.extent` is derived from the same column** (`set_scene`: `extent: bounds.get(base+i).and_then(|b| *b).map_or(0.0, |(lo,hi)| diagonal)`) and is read as the ink lift cap by `ribbon.wgsl:198` (`select(1e30, LIFT_MAX_EXTENT * extent, extent > 0.0)`), `ribbon.wgsl:440`, `sphere.wgsl`, `cylinder.wgsl`, `glyph.wgsl`. **Consequence (found by judge 1, verified here):** lesson 61's fill of `Row.bounds` for curves/points/frames/clouds flips those rows from `extent == 0.0` (no cap) to a real cap — a *second* pixel-relevant change that narrowing `bounded_rows` does not touch. Lesson 61 must therefore either carry a separate `Row.pick_bounds` that never reaches `ArenaUpload.object_bounds`, **or** gate `extent` with the same predicate that narrows `bounded_rows` — and re-measure `drawings` + `bunny_drawings` in that lesson, twice.

---

## 6. Decisions Q1-Q16

| # | question | decision | why | rejected |
|---|---|---|---|---|
| **Q1** | Caches: walker param vs `Walk.caches`; key | `Walk` is the WRITE end; a second `&mut` local `cx: &mut WalkCx { caches, guid }` is the READ end; every walker 4 params. `cx` is rebuilt per object inside the loop (`guid` changes). Keys `curves: HashMap<String, Vec<Point>>`, `tess: HashMap<(String,u8), Tess { mesh, ink }>` declared at first use (53). | `push(w: &mut Walk, m: &Mesh, ..)` with `m` reborrowed through `w.caches` is **E0502**; the survey hit it independently at 46, 48, 49. Two disjoint `&mut` locals over disjoint `Scene` fields compile. Uniform arity means no dispatch arm changes when a lesson adds a cache. | judge2's `Walk.caches` field (compiles for 52, then forces `push` to be re-signed at 53 — moving code 51 placed); min's optional extra params (arity 3-or-5, zero headroom); `HashMap<String, Mesh>` key (renamed by 112) |
| **Q2** | home of the user knobs | ONE `View` in `engine/gpu/view.rs`; one `pub view: View` field on `Gpu`; `View::from_env()` absorbs the `VIEWER_*` reads (`mod.rs:919,925,927`). **FREE-SHAPE at 49** — `line_style`, `cloud_size`, `edl_strength`, `lod_split_px` are already `pub` fields being moved. | Nine knobs arrive after 51; as flat `pub` fields each costs two edits in `gpu/mod.rs` and takes `Gpu` 18 → 28. `View` makes each one field + one `Default` line in a 70-line leaf, and `FrameInput.view` hands the set to the frame writer by reference. 44's `lod_split_px`-below-`edl_strength` anchor holds because 44 is typed pre-block. | `FrameUniforms` + setters (a knob is not a uniform: `show_grid` gates a draw, `line_style` picks a pipeline); nine flat `pub Gpu` fields (god-struct regrowth); seams_from_survey item 12's "pick Gpu" |
| **Q3** | overlay instance rows | `OverlayLane` owns its OWN 1-row `Instance` buffer + bind group at group 2; `place(ctx, world, anchor)` rebases it. `InstanceTable` is never asked to reserve. Decided **now**, mechanism lands at 74. | Verified: a reserved row makes `&up.objects[base..]` an out-of-range slice (`debug_assert` fires first). Worse, a reserved row is an object to 61's BVH, 62's cull, 67's marquee, 68's hidden set and 114's id readback. | `InstanceTable::reserve_row` with a trailing-row-aware `append` — a permanent conditional in the one body six later lessons extend, plus a re-reservation re-entrancy bug |
| **Q4** | per-row world-AABB cache | ONE cache: `InstanceTable.bounds_world`, read via `pub(crate) fn bounds_world(&self) -> &[Option<Aabb64>]`. `Scene.world_boxes` rejected. The `Row.bounds` fill for every type lands at **61**, paired *in the same lesson* with the `bounded_rows` narrowing **and** the `extent` gate (§5), goldens re-measured there. | On-disk 54 builds a second row-indexed cache by re-walking every geometry and re-tessellating every BRep; the two copies diverge under rebase and reconcile. `bounds_world` is derived at `InstanceTable::append` from `object_bounds × model` and rebased already. | both caches; Scene-side only; treating the fill as behaviour-neutral (it is not — twice) |
| **Q5** | `RowTable<T>` now or at 57 | Defer to **57**. 45-51 ships `GrowBuf` + bool-returning `append_rows` + `pub(super)` GrowBufs. | RowTable's CPU mirror is exactly the second copy of the scene that 37/38 deleted (263 MB freed on lidar; `upload_to`'s `drop_rows` exists for that reason). 56 lessons before its first consumer it is a measured regression for a map nothing reads. The bool exists already. | RowTable at 46 |
| **Q6** | `Targets` shape at 46 | `Targets { depth: wgpu::Texture (RENDER_ATTACHMENT\|TEXTURE_BINDING), depth_view, msaa, msaa_view, samples }`; `begin_pass(&self, encoder, target: &TextureView, load: LoadOp<Color>, depth_load: Option<LoadOp<f32>>)` — 4 params. Keeping the Texture is FREE-SHAPE (the body is being moved); the two load params are 2 lines. | Verified: `create_depth_view` (`mod.rs:1859`) is RENDER_ATTACHMENT-only and returns only a view; `begin_pass` hardwires both `Clear`s. 74 needs colour Load + depth Clear (no new method), 113 needs Load on both, 88/107 aim colour elsewhere, and 88/113 **sample** the depth. Without the Texture, 88/113 change a field type — which min's own rule forbids as a deferral. | `begin_pass(encoder, view, color)`; a full `PassDesc` struct (post passes are one-off fullscreen descriptors belonging to their own effect file) |
| **Q7** | `render.rs` / `present.rs` shape | Three comment-fenced regions in one `encode_frame` + `scene_list` + explicit `drop(pass)`; `Frame { surface, pub view, pub encoder }` + `begin_present -> Result<Option<Frame>>` + `end_present`, `clear` kept as the literal 2-arg composition. | Inside the frame additions are ordered draws (a list line is the cheapest correct edit); outside it they are whole passes owned by other layers that need the live encoder and resolved view. `Frame`'s pub fields keep `egui` out of `engine/`. `Option` is forced by `clear`'s two early returns. | `clear(color, vp, ui: Option<UiFrame>)` (engine naming a ui type); a `FrameCtx` trait/closure (lifetime-heavy, unorderable, and two lessons need a fixed order); empty `encode_prelude()`/`encode_post()` stubs (dead code a human types for nothing) |
| **Q8** | lane trait vs contract | Documented contract, **no trait** (L1-L8 above; L7 verbatim). | No generic consumer exists or is planned; a trait would force differing arities into `Option`/no-ops, add dynamic dispatch to the hot path, and be re-opened at 82, 90, 109. | `trait Lane { append; draw; reset; }` |
| **Q9** | State split — shape and lesson | `State` declared once in `state.rs` holding only sub-struct fields; each sub-struct declared in the file owning its concern; every `impl State` block in an `app/*.rs` file. Pattern established by moving `impl State { render, resize }` → `app/render.rs` (with `ViewState`) **in lesson 51**; first sub-struct (`Sync`) at 59. No separate numbered State lesson. | `state.rs` is 48 lines and takes ~30 fields / ~350 lines across 15 lessons — the god-file one layer up. Moving `render`/`resize` at 51 makes the pattern exist before any growth, so every later field lands in a named struct in that lesson's own file. A dedicated lesson would shift the user-fixed +7 numbering for 63 files. **This overturns the plan's "state.rs unchanged" row — declared as a delta in §8, and compiler-gated only.** | *Judges split:* judge 1 defers the move to 59 and keeps the plan's promise literal; judges 2 and 3 keep it at 51 (judge 3: "my preference is the former… worth the row edit"). **I followed judges 2 and 3**, with judge 1's condition attached: it is listed in 51's *what moves* column, not smuggled in as a pre-seam |
| **Q10** | SplatSlot generalisation | All three at 49: (a) `SplatSlot::new(ctx, layout, shared, group1: &[wgpu::BindGroupEntry<'_>])` + `PointBufs::group1_entries(&self, splat) -> [BindGroupEntry; 5]`; (b) `PixelBufs { depth, color }`; (c) `encode_splat` as records-then-batch-array, depth-for-ALL before colour-for-ANY. `splat_records(&self, draws, nodes)` verbatim (44 anchors on it). (a) is a 12-line PRE-SEAM; (b) 4 lines; **(c) is FREE-SHAPE** — at end-of-44 lesson 43 already added the second slot. | Verified: `mk_splat_group1(device, layout, pos, col, sdepth, scolor)` is six positional args, four of them `&wgpu::Buffer` — which is exactly why on-disk 40 (nrm 5th) and on-disk 42 (nrm 7th) both compile while silently binding the depth buffer as normals. The entry-list form kills that class permanently and stops 108's segment slot (no `PointBufs`) from rewriting splat.rs's constructors. | deferring to 108/114 — both late, high-risk; the survey's worst named residual |
| **Q11** | Arena index runs | At 47: `pub enum IdxLane { Solid, Print, Text }` in `gpu/tables.rs` (engine-safe, re-exported to the walk) + `Arena::run(&self, lane) -> &GrowBuf` / `run_mut` + `append -> bool` + `vbo` usage carrying `STORAGE`. All FREE-SHAPE on bodies being moved. | `draw_faces` is one unconditional whole-scene `draw_indexed(0..index_count)`; without a run, 84's untrimmed rectangle draws underneath the trimmed one and 111's big meshes draw twice — both would re-cut `draw_faces` **and** `append`. STORAGE + the bool are three tokens that stop 82/83 shipping a dangling bind group after the first arena growth, a bug those lessons cannot see from their own file. The enum in `tables.rs` (not `walk/mod.rs`) means ONE arm serves both sides. | `ArenaUpload.idx: [Vec<u32>; N]` (violates "ArenaUpload stays flat"); deferring to 84/111 |
| **Q12** | row ctors; `pack_rgba` / `FACING_UNKNOWN` / `encode_width` | Split by ownership of the ROW FORMAT. **Engine:** `pack_rgba`, `FACING_UNKNOWN` → `gpu/tables.rs`; `CylinderSegment::new(p0,p1,radius,color,row)` (5) → `segments.rs`; `GlyphPoint::new(center,radius,color,row)` (4) → `glyphs.rs`, at 48. **App:** `encode_width`, `oct16`, `pack_facing` stay in `walk/encode.rs` (50). | `engine/gumball.rs` (74) and `app/tools/*` (80/81) build rows; with `pack_rgba` app-side the engine imports **upward**. The alternative — each overlay re-spelling the words — is the drift the survey found. **Rationale corrected:** `CylinderSegment` (40 B) packs `color: u32` + `facing: u32`; `GlyphPoint` (48 B) keeps `color: [f32;4]` unpacked plus `facing`/`facing_ext` — so `GlyphPoint::new`'s job is defaulting adjacency, not packing, and the drift in 67/73/74 is `_pad: [0;3]` where `facing`/`facing_ext` now live. | all five app-side (layer inversion at 74); a separate `gpu/pack.rs` (two symbols do not earn a file) |
| **Q13** | judge2's three open questions | (1) **`PipelineDesc` rewrite at 45** + `label` + optional per-desc `Target` + generic `build_compute(device, label, wgsl, entry, layouts)` with `build_splat_compute` as a 2-line wrapper. **Count: 14 render descs**, not 15 — `edges` is deleted. (2) **Keep the `Element(Mesh)` `FLAG_OPEN` mask forever** inside `walk_geometry`; document the removal recipe as a separate non-refactor change. (3) **`stream.rs` keeps its own file.** | (1) `Pipelines::new` is 10 params today; seven later lessons each add exactly one compute pipeline and eight add render descs. Moving the ten builders leaves ~700 lines of repeated descriptors and an 11-param signature 104 would wrap ten times. `label` gives 104 its error scope for one edit; per-desc `Target` lets 88/90/114 pin RGBA16Float / R8Unorm-at-4× / R32Uint without threading a second Target. (2) No golden contains an Element, so removing the mask inside a pixel-gated refactor ships an unmeasured bit change. (3) The stream lane's lifecycle genuinely differs (exact-growth `reserve`, no walk `append`, deliberately skipped by `reset_arena`); a bool-parameterised `CloudLane` is worse than two files. | moving the builders into `pipelines/{solid,flat,depth,screen,compute}.rs`; accepting the FLAG_OPEN delta; `CloudLane` ×2 |
| **Q14** | where `walk_geometry` lives | `app/walk/mod.rs`, `pub(crate) fn walk_geometry(w, cx, geom, ri) -> Row` (4 params), holding the 12 arms + the Element mask. `Walk`'s fields `pub(crate)` so any app file can build one. | 56 wraps the dispatch one level (`match obj { Geom(g) => …, Trimmed(ts) => … }`) — two lines with the fn, a 12-arm re-indent without it. 58's `apply_object`, 79/80's runtime add and 112's LOD all re-run ONE object on a scratch `Walk` through the same fn, so reconcile has no second dispatch. Homing it outside `scene.rs` stops `scene.rs` becoming a second per-type dispatch site. | `scene.rs` (judge2's sketch) |
| **Q15** | the pre-seam rule | The **three classes** + the four admission conditions + ≤20/lesson, ≤130/block (§1). Applied lists in §7. | The binary rule forces every shaping decision into the "new code" column and produces a 205-line budget containing at least four items that are moves at the block's own baseline; that inflation is what made lesson 49 carry ~45 discretionary lines on top of the present split, `View` and the final `gpu/mod.rs`. min's ≤15 per lesson is the right ceiling but its 39-line total is unreachable without under-pricing `Targets` and losing `View`, `Template` and the engine row ctors. | "pre-seam everything" (23 seams, ~450 lines, six structures dead for 20+ lessons); "pre-seam nothing" (11 body rewrites, three in the frame path) |
| **Q16** | boundaries; does the count stay 7 | **Seven**, no file crossing a boundary, the plan's *what moves* column kept except the three §8 deltas, the +7 renumber unchanged. Two named contingencies, both decided **before 45 is typed**: (a) measure lesson 49's doc length the day the end-of-44 tree exists — if it exceeds one sitting, split between the point lanes (`cloud`/`splat`/`stream`) and the frame (`render`/`present`/`mod`); (b) judge 3's rebalance (`View` 49→45, persistence split 51→46) is the fallback if 49 or 51 still overflows. | Seven parts is already ≤4 files each. Keeping the count fixed keeps every forward reference in the authored 40-44 chain and the 68 surveyed lessons valid under one arithmetic rule; an eighth lesson renumbers 63 files for one sitting's typing. | splitting 49 or 51 now (judge 3's rebalance is 1 of 3 — recorded as the contingency, not the plan) |

---

## 7. Seam ledger

Class: **F** = free-shape (0 lines, mandatory) · **P** = pre-seam (counted) · **D** = deferred. All pre-seams are behaviour-neutral unless marked.

| id | where | shape | needed by (on-disk) | lands in | class | neutral | lines |
|---|---|---|---|---|---|---|---|
| S3a | `pipelines/build.rs` | `Target { samples, format }`; `PipelineDesc { label, shader, fs_entry, topology, vertex_buffers, bind_groups, blend, write_mask, depth_write, depth_compare, target: Option<Target> }` + presets `opaque`/`ink`/`depth_only`; `build_pipeline(device, t, &desc)` = the ONE `create_render_pipeline` | 79, 81, 83, 86, 97, 99, 100, 107 | **45** | F (the sanctioned rewrite) | yes | 0 |
| S3b | `pipelines/build.rs` | `build_compute(device, label, wgsl, entry, &[&BindGroupLayout])`; `build_splat_compute` a 2-line wrapper | 75-78, 101, 102, 104, 106 | **45** | P | yes | −9 (net) |
| S4 | `pipelines/layouts.rs` | `Layouts` = single owner of all 9 layouts, `new(device)` from editable entry lists, `compute_entry(binding, ty)`; `Pipelines::new(device, t, &l)` frozen at 3 params | 75-77, 79, 81, 83, 86, 92, 99, 101, 102, 104, 106, 107 | **45** | F | yes | 0 |
| A13 | `src/math.rs` | `pub type Bounds = ([f32;3],[f32;3]); pub type Aabb64 = ([f64;3],[f64;3]);` | 54, 55, 60, 100, 103, 105 | **45** | P | yes | 2 |
| S14 | build.rs, mod.rs, shaders | delete `edges` pipeline + `build_edges_pipeline` + `edges.wgsl` (0 draw sites) and `storage_buffer` (0 callers); **keep** `ribbon_depth`/`glyph_depth` (INK_DEPTH_PREPASS) and the `time` uniform | 67, 73, 74 | 45 / 46 | F | yes | −109 |
| A3 | `gpu/frame.rs` | `FrameInput<'a> { config, view_proj, anchor, view }`; `write_camera(&mut self, ctx, f: &FrameInput<'_>)` | 55, 62, 79, 89, 92, 99, 102, 104, 106 | **46** | P | yes | 10 |
| S2a | `gpu/targets.rs` | depth kept as `wgpu::Texture` with `TEXTURE_BINDING` (F) + `begin_pass(encoder, target, load, depth_load)` (P) | 67, 81, 106, 107 | **46** | F + P | yes | 2 |
| S5 | `gpu/frame.rs` | `FrameUniforms { …, mvp_f32, last_ortho_h, last_eye }` + `Binds<'a> { p, mvp, time, line, cloud, instances }` | 40, 44, 55, 62, 79, 89, 92, 99, 102, 104, 106 | **46** | F | yes | 0 |
| — | `gpu/tables.rs` | `ObjectBase { model: Mat4, color, flags }`, `CloudDraw { first, count, instance, spacing, node_first, node_count }`, `LodNode` replace the two tuples (12 hunks) | 43, 44, 51, 54, 60, 61, 66, 96 | **46** | F | yes | 0 |
| S8a | `gpu/tables.rs` + `gpu/arena.rs` | `pub enum IdxLane { Solid, Print, Text }` + `Arena::run(lane)` / `run_mut` | 77, 87, 104, 105 | **47** | F | yes | 0 |
| S8b | `gpu/arena.rs` | `Arena::append -> bool`; `vbo` usage gains `STORAGE` at both creation sites | 75, 76, 77 | **47** | F | yes | 0 |
| A4 | `gpu/objects.rs` | `pub(crate) fn bounds_world(&self) -> &[Option<Aabb64>]` — the accessor only | 54, 55, 60, 61, 100, 105 | **47** | P | yes | 3 |
| A8 | `gpu/*` | visibility policy: lane fields `pub(super)`, methods `pub(crate)`, `Gpu` lane fields `pub(crate)` (kills ~11 forwarders) | 51, 54, 60, 61, 66, 69, 87, 92, 94, 100 | **46-49** | F | yes | 0 |
| **S6** | `gpu/objects.rs` | `upload_rows(ctx, &[u32])` — the flip-tracked partial upload | 55, 60, 61, 69, 94, 100, 105, 107 | **DEFER → 62** | D | — | 0 |
| S10a | `gpu/segments.rs`, `glyphs.rs` | `pub(super) struct Template { vbo, ibo, index_count }` + `draw_template(pass, b, pipeline, template, group3)` | 67, 73, 74 | **48** | P | yes | 14 |
| A6 | `gpu/tables.rs`, `segments.rs`, `glyphs.rs` | `pack_rgba`, `FACING_UNKNOWN` engine-side; `CylinderSegment::new` (5), `GlyphPoint::new` (4) | 67, 68, 73, 74, 101 | **48** | P | yes ¹ | 12 |
| S1a | `gpu/render.rs` | three comment-fenced regions + explicit `drop(pass)` + `fn scene_list(&self, pass, b) -> u32` | 67, 73, 74, 77, 79, 81, 83, 86, 100, 102, 104, 106, **107** | **49** | F | yes | 0 |
| S1b | `gpu/present.rs` | `Frame { surface, pub view, pub encoder }`; `begin_present -> Result<Option<Frame>>`; `end_present`; `clear` as the 2-arg composition | 62, 67, 73, 80, 100, 107 | **49** | P | yes ² | 16 |
| S7a | `gpu/splat.rs`, `cloud.rs` | `SplatSlot::new(ctx, layout, shared, group1: &[BindGroupEntry])` + `PointBufs::group1_entries(&self, splat)` | 43, 101, 107 | **49** | P | yes | 12 |
| S7b | `gpu/splat.rs` | `encode_splat` = records-then-batch-array, depth-all before colour-all | 43, 44, 101 | **49** | F (exists at end-of-44) | yes | 0 |
| S7c | `gpu/splat.rs` | `PixelBufs { depth, color }` | 101, 107 | **49** | P | yes | 4 |
| S12 | `gpu/view.rs` | `View` + `pub view: View` on `Gpu`; `View::from_env()` | 62, 79, 82, 83, 89, 92, 105 | **49** | F (fields) + P (`from_env`) | yes ³ | 4 |
| A1 | `app/walk/mod.rs` | `Caches` (empty) + `WalkCx<'a> { caches, guid }` + `Scene.caches` (never cleared by rebuild/clear) + uniform 4-param walkers | 45-49, 66, 87, 105 | **50** | P | yes | 14 |
| W3 | `app/walk/mod.rs` | `Row::NONE` + the `..Row::NONE` spread convention at every construction site | 54, 103, 105 | **50** | F | yes | 0 |
| A10 | `app/scene.rs` | `Doc { rows: Range<u32>, planar: bool }`, filled from `Marks.obj0` and the existing `let planar` local (scene.rs:478) | 100, 101, 102 | **50** | P | yes | 4 |
| A6-use | `app/walk/{curves,points,frames}.rs` | the six converter bodies call the engine row ctors | 67, 73, 74, 80, 81 | **50** | P (⚠ retyped) | pixel-gated ¹ | 4 |
| S19 | `app/scene.rs`, `walk/mod.rs` | `Scene.node_base` / `Walk.node_base` (GPU-absolute octree bases) | 44 | typed at 44; moved 50 | F | yes | 0 |
| A2 | `app/walk/mod.rs` | `walk_geometry(w, cx, geom, ri) -> Row` — 12 arms + the Element mask | 49, 51, 72, 73, 87, 105 | **51** | P | yes | 6 |
| A11 | `app/persistence/{mod,reader,lean}.rs` | 3-way split (fetch / chunked reader + `file_hash` / dump + download) | 52, 53, 93, 98 | **51** | F (pure move) | yes | 0 |
| A12 | `state.rs` + `app/render.rs` | the State spine documented; `impl State { render, resize }` + `ViewState` moved | 52, 53, 56, 60, 62, 63, 66, 67, 69, 71-74, 80, 87 | **51** | F (move) | compiler-gated ² | 2 |
| S18 | `app/*.rs` | the `impl Scene` file-split convention (documented, files created by their own lessons) | 51, 54, 57-61, 84, 87, 88, 93-96 | **51** | F (doc) | yes | 0 |
| S22 | `ARCHITECTURE.md` §2/§3 | rewritten to this tree + the lane/walk contracts + the State spine + the three-class rule + the corrected litmus | all | **51** | F (doc) | yes | 0 |
| S9 | `gpu/buffers.rs` | `RowTable<T>` (GrowBuf + bind group + mirror + guid→Range) | 50, 51, 87 | **DEFER → 57** | D | — | 0 |
| S13 | `gpu/{curve_tess,surface_tess,trim,tess}.rs` | producer files with `encode(cp)`; `impl Gpu { tessellate }` = ONE encoder | 75-78 | **DEFER → 82** (a *writing rule*, not code) | D | — | 0 |
| S20 | `engine/performance.rs` | `drawn`, `total`, `last_draws` | 55, 62 | **DEFER → 62** | D | — | 0 |
| S21 | `src/math.rs` | `Frustum::{from_view_proj, rebased_to_world, aabb_visible}`; `xform_from_mat` | 55, 60, 69, 102, 104, 106 | **DEFER → 62 / 76** | D | — | 0 |
| S2b | `gpu/targets.rs` | `Targets::new_sized(ctx, size, format, samples)` | 100, 107 | **DEFER → 107** | D | — | 0 |
| S2c | `targets.rs` / `post.rs` / `outline.rs` | `scene_color` + the half-res effect targets, each owned by its effect file | 81, 83 | **DEFER → 88 / 90** | D | — | 0 |
| S1c | `gpu/present.rs` | `render_offscreen` → `render_to_texture(size, view_proj)` + native readback | 100 | **DEFER → 107** | D | — | 0 |
| S23 | `gpu/overlay.rs` | overlay-owned 1-row instance buffer | 67, 73, 74 | mechanism **74**, decision **now** | D | — | 0 |
| A9 | `walk/mod.rs::Caches` | `tess: HashMap<(String,u8), Tess { mesh, ink }>` declared with its final key/value | 46, 47, 105 | **DEFER → 53** | D | — | 0 |
| A4-fill | `walk/*.rs` + `objects.rs` | `Row.bounds` `Some` for every type + the `bounded_rows` narrowing + the `extent` gate | 54 | **61** | D (⚠ **not** neutral) | **no** | — |
| S17 | `walk/mesh.rs::push` | 5th param `edges: Edges` | 47 | **REJECT** → 54 adds it | — | — | 0 |
| S3c | `pipelines/build.rs` | `shader()` prelude string; MRT colour targets | 82, 103 | **REJECT** → sibling fns later | — | — | 0 |
| — | `app/spatial.rs` | `Scene.world_boxes` | 54 | **REJECT** (Q4) | — | — | 0 |

¹ A6-use retypes ~18 lines of six converter bodies onto the engine ctors — a **mechanical rewrite, not a byte-identical move**, covered by the `drawings` golden. It must be presented like `build.rs`: its own compile + golden checkpoint inside lesson 50. ² Compiler-gated only (the goldens run `render_offscreen`, never `State`) — lesson 49/51 ends with a browser smoke check. ³ `View::from_env()` moves the `VIEWER_*` reads (an expression change) — its own checkpoint inside 49.

**Per-lesson pre-seam spend:** 45 = 14 · 46 = 22 · 47 = 18 · 48 = 26 · 49 = 20 · 50 = 22 · 51 = 8 → **130**.

---

## 9. Landing map — every lesson

**Pre-block chain (typed on today's FLAT paths; 45-51 then absorbs their code as Moves).**

| new № | on-disk | title | lands in (flat tree) | one-line edits | absorbed by | residual risk |
|---|---|---|---|---|---|---|
| 40 | 40 §1-2 | Potree look — EDL + attenuated splats | `gpu/mod.rs`, `splat.wgsl`, `splat_resolve.wgsl` | `pub edl_strength` beside `cloud_size`; `write_cloud` `_pad = edl`; record k/r_min words | 46 (`last_ortho_h`→frame.rs), 49 (`edl`→View, records→splat.rs) | doc-only if ever replayed post-51 |
| 41 | 40 §3-4 | Cloud normals — oct16 lane, lambert, importers | `point_nrm_buffer` + `splat_group1` binding 4 + 2 examples | `compute_entry(4, …)`; `mk_splat_group1` binding 4 | 45 (`Layouts`), 49 (`group1_entries`) | the nrm-position hazard vs 43 is live until 49 — **put a review note in 43's text** |
| 42 | 41 | Cloud scenes — datasets, bbox packing, stress test | `selftest.rs`, `lib.rs`, `examples/pb_bbox.rs` | `DEMO_SCENE_URL` | none (0 gpu fences) | needs only `render_offscreen`'s signature and `cloud_size`'s name to survive 49 — both do |
| 43 | 42 | Streaming cloud — HTTP Range in, GPU rows out | `stream_*` fields; `persistence.rs` +161 | 4 `Msg` arms; `Scene.clouds` + `begin_cloud` | 49 (→`stream.rs`), 51 (persistence split) | "own lane" must be re-argued from `reset_arena` skipping stream; its second record slot is what makes S7b free at 49 |
| — | 43 | Lane structs | **DELETED** (written against end-of-42; 19/21 anchors dead) | — | superseded by 45-51 | keep only its lane NAMES, `ObjectBase`/`CloudDraw` destructuring, the Expected-state block form |
| 44 | 44 | Cloud octree — Potree LOD on the splat lane | `LodNode` + `CloudDraw` fields + node ordering in the walk | `Scene.node_base` + `Walk.node_base`; `lod_split_px` below `edl_strength`; `splat_records(draws, nodes)` | 46 (`LodNode`), 49 (`splat_records`, `View`), 50 (`node_base`→`Walk`) | must be **re-anchored to main's append-only cloud lane** (GPU-absolute `node_first`/`LodNode.first`) BEFORE typing — new content, not a re-root |

**Post-block (52-114). "One-line edits" = the additive list lines; "seams" are §7 ids.**

| new № | on-disk | title | new file(s) | one-line edits | seams | residual risk |
|---|---|---|---|---|---|---|
| 52 | 45 | NurbsCurve — sample once, cache per guid | — | `Caches.curves` field + init; body in `walk/curves.rs` | A1, A9 | the arm Find must quote `walk_nurbscurve`'s body, not `add_file`'s one-liner |
| 53 | 46 | NurbsSurface — tessellate once | — | `Caches.tess: HashMap<(String,u8), Tess>` + `struct Tess` | A1, **A9** | none: `cx` and `w` are disjoint `&mut` locals, which is exactly what removes the predicted E0502 |
| 54 | 47 | Iso-curves | — | `push` gains 5th param `edges: Edges` + 2-line gate; 3 callers pass `Draw`; surface passes `Suppress`; `Tess.ink` filled | A6, S17-rejected | `push` hits the **5-param ceiling** here — any later input rides `Walk` or `IdxLane`. Re-quote the `MESH_RAW_MIN` gate from main (`return (None, false);`) |
| 55 | 48 | BRep — many faces, one object | — | none (51 already merged the two BRep arms) | A1, A6, `sample_curve` | the "two sites, one pattern" prose collapses to nothing — cut it |
| 56 | 49 | Trimmed surfaces + the every-map rule | `app/walk/trimmed.rs` | loop head → `all_objects`; the call → a 2-arm `match obj`; `pub mod trimmed;` | **A2**, A1 | `all_objects`/`ObjRef` live in `scene.rs`; if they pass ~30 lines they move to `app/query.rs` |
| 57 | 50 | Reconcile I — a per-object GPU arena | `gpu/arena_alloc.rs`; `RowTable<T>` in `buffers.rs` | `SegmentLane`/`GlyphLane` fields → `RowTable` (12-site Replace-all in 2 files); `ArenaUpload` gains per-object guid/range columns | S8a, S8b, **S9**, A2 | **⚠ EXCEPTION 1**: rewrites `Arena::append`'s body (code 47 placed). Accepted — addressability is 57's product and the edit is inside one lane file under the goldens. Must be **re-authored from scratch** against `SegmentLane.pipes/ribbons` + the flat `ArenaUpload`; its single-buffer premise is wrong on main in 25 fences. `GpuArena::allocate` takes `&GpuCtx` to stay ≤5 |
| 58 | 51 | Reconcile II — diff by guid | `app/reconcile.rs` | `InstanceTable::set_row`; lane `add`/`remove`; `Arena` allocate/free forwarders; 3-line `Gpu` verbs | S6-deferred, S9, **A2**, A8 | `CloudLane` has no per-guid remove — state the "clouds re-stream wholesale" rule. Move `pick_after_reconcile` to 61 |
| 59 | 52 | Save — write the file back | `app/sync.rs` (`Sync`) | `state.rs` `sync: Sync` (**first sub-struct**); `Scene.dirty`; 2 key arms; Cargo features | A12, A11, S18 | 87 invalidates the frame clock — keep the debounce in `Sync` so 87 edits one file |
| 60 | 53 | Watch — external edits flow back | — | `Msg::Watched(String, Session)` + one `user_event` arm; `file_hash` | A12, A11, 58's reconcile | `State::new(window, Scene::new())` anchor depends on 43's empty-boot loader |
| 61 | 54 | Scene BVH — one broad-phase | `app/spatial.rs`, `app/query.rs` | `Scene.index`; one post-match line in `add_file`; **one predicate narrowing `bounded_rows`**; **one `extent` gate**; 6 small `Row.bounds` fills | **A4**, A13, S18, A10, W3 | **⚠ EXCEPTION 2**: the only deliberate pixel-relevant change after 51. Fill + `bounded_rows` predicate + `extent` gate must land in ONE lesson and `drawings`/`bunny_drawings` re-measured twice. Splitting them changes the ink silently |
| 62 | 55 | Frustum culling | — | `math.rs` `Frustum`; `objects.rs` `update_culled` + `culled` + **`upload_rows`** (S6 lands here); one `begin_frame` line; `FLAG_CULLED`; 5 wgsl collapse lines; `Performance.drawn/total` | S6, A3, A4, S20, S21 | step 4b needs the frustum inside `splat_records` **without** changing its signature — stash it on `Splat` in this lesson, never on `CloudDraw` |
| 63 | 56 | Screen → ray | `engine/pick.rs`, `app/input.rs` | `pub mod pick;`; `state.rs input: Input`; lib.rs stash + Left arm | A12 | none |
| 64 | 57 | Ray-cast meshes | `app/pick.rs` | one `app/mod.rs` line | S18, A4, Q1's tess cache | the 5-arm candidate match is a second per-type dispatch — acceptable as ONE fn; if it grows, export `pick_<type>` per walk file |
| 65 | 58 | Sub-object picking | — | `PickCtx` + `resolve_subobject` in `app/pick.rs`; `project_to_screen` in `engine/pick.rs`; one call in `input.rs` | S18, PickCtx | without `PickCtx` this fn is 6 params — the lesson must adopt it |
| 66 | 59 | Pick thin geometry | — | `camera.rs world_per_pixel`; `pick_ray` → `pick_mesh` rename inside one file | S18, B4 via `query.rs::world_frame` | derive `tol` from `frame.rs::line_thickness_px` / `math.rs::ortho_half_height`, never re-typed `1/tan(30°)` |
| 67 | 60 | Selection — highlight + marquee | `app/select.rs` | `camera.rs marquee_frustum(vp, origin, rect)`; `write_row_flags` + `set_flag_rows` over `upload_rows`; `FLAG_SELECTED`; 5 wgsl tint lines | S6, A8, A4 | two of five shader anchors are stale (cylinder/ribbon use `unpack4x8unorm`) — doc re-anchor. Write `set_flag_rows` in its **general** form so 68 costs zero engine lines |
| 68 | 61 | Hidden objects | — | 3 pick/marquee guards; 2 key arms | S6 | none — 67's general form makes step 1 disappear |
| 69 | 62 | egui overlay — HUD + settings | `ui/mod.rs`, `ui/settings.rs` | `app/render.rs`: `begin_present` → egui pass → `end_present`; two `if view.show_*` wraps; 3 `View` fields; `state.rs ui: Ui`; `Performance.last_draws` | **S1b**, S12, A3, A8 | egui must be the LAST pass; 74's gumball pass precedes it — enforce by call order in `app/render.rs` |
| 70 | 63 | Command bus + Get-loop | `app/getloop.rs`, `app/commands.rs`, `ui/cli.rs` | 2 `app/mod.rs` lines; `state.rs cli: Cli`; one click reroute | A12, S12 | none — `dispatch`'s match is the growth list |
| 71 | 64 | Command options | — | one `"probe"` arm | 70's trait | none |
| 72 | 65 | History & autocomplete | — | `Cli.cli_history`; 2 `UiState` fields | A12 | `cli_panel` has 7 params — `ui/` is outside the engine rule, but prefer a `CliView` struct |
| 73 | 66 | Delete + undo/redo | `app/history/{mod,remove}.rs` | `pub mod history;`; 3 dispatch arms + VERBS rows; `state.rs history`; 3 key arms; `restore_geometry` → `app/reconcile.rs` | S6, A1 (cache eviction), S18 | `history.clear()` needs a hook at `Scene::rebuild` and 58's `commit` — one line each, added here |
| 74 | 67 | Gumball I — the widget appears | `engine/gumball.rs`, `gpu/overlay.rs`, `app/tools/gumball.rs` | `Gpu.overlay` + one `::new`; one post-region line (`begin_pass(encoder, view, Load, Some(Clear(0.0)))` + `overlay.draw`); `state.rs gb` | **S23**, **S10a**, **A6**, S2a, S14 | every gpu fence must be re-authored onto `overlay.rs`/`targets.rs`/`render.rs`; the gumball.rs/state/scene halves survive |
| 75 | 68 | Gumball II — constant size + hit test | — | zero `Gpu` fields (`FixedRows.scratch` exists from 74) | S10a | the tint must run on the unpacked `GumballGeom` colour **before** `CylinderSegment::new` packs it |
| 76 | 69 | Gumball III — drag to translate | `app/history/transform.rs` | `InstanceTable::write_models(ctx, &[(u32, Mat4)])`; `Gpu::set_live_models` forwarder (sequences `splat.state = None`); `math.rs xform_from_mat`; `apply_world_delta` in `query.rs` | S6, A8, S21 | the on-disk text omits `base_f32` (rotate/scale snap back at the next 200 ms rebase) — `write_models` makes that impossible to miss; keep the note |
| 77 | 70 | Gumball IV — rotate and scale | — | `DragCtx` a0/d0 | 76's live path | none |
| 78 | 71 | Gumball V — type a number | — | 4 `UiState` fields; one Esc guard | A12 | none |
| 79 | 72 | Draw tools I | `app/tools/{point,line}.rs`, `app/history/add.rs` | 2 dispatch arms + VERBS rows; 2 `tools/mod.rs` lines | **A2**, S18 | **B6**: the runtime-add path must pick a TARGET doc and convert to doc-local coords via `place⁻¹` (`query.rs::world_frame`) |
| 80 | 73 | Draw tools II + ghost preview | `app/tools/{polyline,rect,box,nurbscurve}.rs` | `overlay.preview: FixedRows<CylinderSegment>`; one in-pass line after `draw_ribbons`; `ActiveCommand::on_move` default | S23, S10a, A6 | `BoxTool`'s `m.xform` is a kernel-API bug (**B2**) — placement goes through `session.set_xform` |
| 81 | 74 | Snapping | `app/snap.rs` | `overlay.marker: FixedRows<GlyphPoint>` (cap 1); one line after preview; one `snap` verb | S23, S10a, A6, A4 | two API mismatches vs 61 (`doc_of_row` returns `usize`; `objects_in` needs `&self`) must be fixed when re-authored |
| 82 | 75 | GPU curves — a compute producer | `gpu/curve_tess.rs`, `gpu/tess.rs`, `curve_tess.wgsl` | `Layouts.curve_tess`; one `build_compute` line; `ArenaUpload.curve_uploads` + one `drop_rows`; body in `walk/curves.rs`; 4 `Gpu` list lines | S3b, S4, S8b, **S13** | **⚠ EXCEPTION 3**: reserved zeroed rows are swept by `walk/bounds.rs::file_extent`/`sheet_thickness` — a curves-only file measures thickness 0 and is marked `FLAG_SHEET` (fills stop writing depth, ink loses its lift). **This lesson MUST add the reserved-range skip.** Also: author `encode(cp)`, never a self-submitting dispatch |
| 83 | 76 | GPU surfaces | `gpu/surface_tess.rs`, `surface_tess.wgsl` | `Layouts` + `Pipelines` one line each; `ArenaUpload.surface_uploads`; `walk/surface.rs` body; one `tess.rs` line; 4 `Gpu` lines | S3b, S8b, S8a, S13 | the on-disk fence forgets `vids` — a slot-1 desync mis-instancing every later mesh; the reserve helper must push verts AND vids AND idx |
| 84 | 77 | GPU trimming | `gpu/trim.rs`, `trim_classify.wgsl` + `fs_trimmed` | `IdxLane::Trimmed` (one arm) + `Arena.trimmed` run; one `render.rs` line after `draw_faces`; 2 `Layouts` + 2 descs; `ArenaUpload.trim_uploads`; `walk/trimmed.rs` body | **S8a**, S3a, S3b, S5 | highest of the group: the render side is implicit in the stub and must be authored against `trim.rs` + the Trimmed run |
| 85 | 78 | GPU BRep | — | **ZERO engine lines** | S13, A1, the 3 reserve helpers | only if 82 shipped a self-submitting dispatch — S13's writing rule exists to prevent that |
| 86 | 79 | Analytic ground + infinite grid | `gpu/ground.rs`, `ground.wgsl` | `Layouts.ground`; one desc; one `render.rs` line between background and grid; `FrameInput.view_dist` + `rebase_anchor` stash; one `Gpu` field + `::new` | A3, S3a, S4, L4 | none — a lane file, not `FrameUniforms` growth, is what keeps `frame.rs` under 300 and gives 96 a one-field edit |
| 87 | 80 | Render-on-demand | — | `ViewState.dirty` + the early return; ~12 poke sites; one HUD counter | **S1b**, A12 | invalidates 59/60's frame clock and watch poll — both in `app/sync.rs`, a one-file follow-up |
| 88 | 81 | GTAO | `gpu/post.rs`, `gtao.wgsl`, `blur5.wgsl`, `composite.wgsl` | `Targets.scene_color` + one resize line; two `render.rs` post lines; 4 `Layouts` + 4 descs (`post` preset); `PostUniform` + `write_ao` from `set_scene` | **S2a**, S3a, S1a | `encode_ao` and `encode_composite` must be SEPARATE calls so 90 slots between them without touching `post.rs` |
| 89 | 82 | Arctic + cheap GI | — | `View.arctic`; one checkbox; one key arm; one u32 in `PostUniform` | S12 | none; MRT albedo stays deferred (S3c) |
| 90 | 83 | Selection outline + AA | `gpu/outline.rs`, `outline_mask.wgsl`, `outline_sep.wgsl` | `Arena`/`SegmentLane`/`GlyphLane` each gain `draw_mask` (3 additive methods); one gated `render.rs` line between blur and composite; `View.outline_needed`; +6 descs pinned to `Target { samples: 4, format: R8Unorm }` | S10a, S3a, S12, S1a | the on-disk 60-line fence carries a today-bug (cyl mask binds `segment_bind_group` while iterating `pipe_count`) — `draw_mask` makes it unrepresentable |
| 91 | 84 | Scene tree | `ui/tree.rs` | `Scene.generation`; `query.rs::object_name`; 4 `UiState` fields | S18, A12 | none |
| 92 | 85 | Tree ↔ viewport | — | 2 `UiState` fields; one `TreeIntent` field; one drain line | S18 | fix 85's `HashSet<String>` vs 84's `HashSet<u32>` |
| 93 | 86 | Text labels | `gpu/text.rs`, `engine/text.rs`, `app/labels.rs`, `text.wgsl` | `Layouts.atlas`; one desc (`ink`); one `::new`; one `render.rs` line (last); one resize line | **L4** (six list lines exactly), S4, S3a, A4 | none — the canonical proof that a new render lane is additive |
| 94 | 87 | Control-point editing | `app/edit.rs` | `Arena::write_range` + `SegmentLane::write_range`; 2 three-line `Gpu` forwarders; `walk/{curves,surface}.rs` expose their producer half | S9, A1, S18, **W5** | depends on 57 giving `SegmentLane` a per-object range map; if 57 slips, fall back to the documented v1 (glyph preview live, `set_scene` rebuild on release) |
| 95 | 88 | Edit points (Greville) | `app/edit_points.rs` | `Scene.greville_cache` | 94's partial write, S18 | none |
| 96 | 89 | Work plane | `app/tools/cplane.rs` | `View.work_plane`; one line in `ground.rs::write`; `cursor_world_point` swaps ray∩z=0 for ray∩plane (one line — every tool becomes plane-aware free) | S12, A3, 86's lane | **B2**: use the row's placed frame, never `m.xform` |
| 97 | 90 | Advanced perf (prose) | — | — | S2a, S8a, S8b, S3b, S1b | its levers are the deep versions of 112/113; those five seams are what keeps them additive |
| 98 | 91 | Capstone (prose) | — | — | — | its phase table carries pre-renumber numbers — shift by +7 |
| 99 | 92 | Section planes | — | `Layouts.mvp` binding 1 + `splat_group0` binding 4; `SectionUniform` + `write_sections` in `frame.rs`; `View.sections`; 6 shader discard blocks incl. **ribbon + glyph**; one early return in splat.wgsl's two kernels | S4, S5, A3, S12, S7a | the cloud cut cannot be an fs discard (the cloud lane is compute) — re-author around the compute early return. Omit ribbon/glyph and flat ink draws through the cut |
| 100 | 93 | Import / export | — | `.obj`/`.step` arms in `persistence/reader.rs`; export in `lean.rs`; `State.proxy`; 3 verbs; 3 web-sys features; `first_selected_mesh` in `query.rs` | A11, S18 | `Msg::File` arity (5-tuple on main) must be re-anchored |
| 101 | 94 | Copy, duplicate, array | `app/tools/copy.rs` | `AddGeometry::of_snapshots`; one `Gpu::set_live_model` call; 2 dispatch arms; `Input.alt` | S6, A12, S18 | `ModifiersChanged` is matched in lib.rs, not `State` |
| 102 | 95 | Layers | `app/layers.rs` | `Scene.active_layer`; 4 dispatch arms | S18 | none — the purest "new file + arms" lesson in the tail |
| 103 | 96 | Measure + status bar | `ui/status.rs` | 4 verbs; 3 `UiState` fields | 70's ProbeCmd | one prose `.0` → `.model` re-anchor |
| 104 | 97 | Developer toolbox | `gpu/errors.rs`, `examples/invariants.rs`, `viewer.yml` | ONE `push/pop_error_scope` pair around `build_pipeline` (14 pipelines, one edit, named by `desc.label`); one line in the bring-up for `on_uncaptured_error` | **S3a**, the verbatim bring-up block | `pop_error_scope` returns a future — `engine/` must use `pollster`/`cfg`, never `spawn_local`, or the native `--all-targets` gate breaks |
| 105 | 98 | Web polish — load progress | — | `fetch_finish_with_progress` in `persistence/mod.rs`; one call site in `resumed()`; Cargo.toml + index.html | A11, 104's `push_gpu_error` | must **extend** `fetch_finish`, not re-issue the GET, or the window-of-2 loader is lost |
| 106 | 99 | Textures | `gpu/material.rs` | `Layouts.material`; `&l.material` in the triangle + triangle_sheet descs (2 lines); `Binds.material` (one field + one arg); one bind line inside `Arena::draw_faces`; `tex_anchor` at mvp binding 2; one `Gpu` field + `::new` | S4, S3a, S5, A3 | four of the lesson's five Rust anchors no longer exist after 45-51 — the replacement is strictly smaller but the text is a full re-author |
| 107 | 100 | Sheet impostors | `gpu/impostor.rs`, `impostor.wgsl` | `render_offscreen` → `render_to_texture` + readback (S1c); `Targets::new_sized`; `InstanceTable::mask_range`/`restore`; one `render.rs` line; `Doc.rows`/`planar` READ (already filled by A10); one invalidation line in reconcile | **A10**, S1b, S2b, S6 | **⚠ EXCEPTION 4**: edits `render_offscreen`, a body 49 placed. Accepted because the goldens run through that exact path — the edit is measured, not argued |
| 108 | 101 | Compute ink | `splat_seg.wgsl` | `PixelBufs.coverage` (one field); one record line + one batch tuple in `encode_splat`; `Layouts.splat_seg_group1` + a coverage entry; +2 compute descs; `SegmentLane.slot` | **S7a**, S7b, S7c, S3b, A10 | highest of the scale group; the per-sheet skip of the raster ribbons needs 109's batches or a flag bit |
| 109 | 102 | Segment batches | `gpu/seg_cull.rs`, `seg_cull.wgsl` | `ArenaUpload.seg_batches`; `walk/bounds.rs::mark_sheet` emits them (planar is known only there); one prelude line; frustum uniform + `Layouts` entry; one indirect branch in `draw_ribbons` | S1a, S3b, A3, A10 | ribbons draw non-indexed — the stub's `draw_indexed_indirect` must become `draw_indirect` |
| 110 | 103 | Quantized meshes | — | the quantize fold in `walk/mesh.rs::push`; a sibling `boxes` GrowBuf at instance binding 1 (**never** a wider `Instance` row — that is a 6-file stride edit across 5 wgsl mirrors); `tables.rs` vert type; `arena.rs` stride; `VIEWER_VQ` | S8a, A13, `oct16` | `walk/bounds.rs` sweeps read `verts[i].position` as world f32 — they must run BEFORE the fold or through a dequant helper; the stub does not mention it |
| 111 | 104 | Meshlets | `app/meshlets.rs`, `meshlet_cull.wgsl` | `IdxLane::Big` (one arm) + `Arena.big` run; one indirect draw line inside `draw_faces`; `ArenaUpload.meshlets`; one prelude line; one `Layouts` + one desc | **S8a**, S3b, A3, S9 | depends on 57's `ArenaSlot` ranges for "one indirect draw per big mesh" |
| 112 | 105 | Mesh LOD | `app/simplify.rs` | `Caches` key is ALREADY `(String, u8)` (A9 — zero renames); `arena_alloc` free+alloc for the swap; 2 reserved flag bits (8/9); `screen_height_px` over `bounds_world`; a quality index in `walk/{surface,brep}.rs` | A9, A2, A4, S6, W3 | none structural — A9 and W3 are what make this lesson cheap |
| 113 | 106 | HiZ occlusion | `gpu/hiz.rs`, `hiz.wgsl` | one post line in `render.rs`; one binding + one WGSL block in each of 109/111's cull shaders; `Targets` already keeps the depth Texture with `TEXTURE_BINDING`; `begin_pass` already takes `LoadOp` | **S2a**, S1a, S3b | MSAA depth cannot be `textureLoad`-ed as `texture_2d` — the lesson must add the resolve pass the stub omits |
| 114 | 107 | Id-buffer picking | `gpu/pick.rs`, 5 `fs_id` fragment entries | one `PipelineDesc::id` preset + `Pipelines::new_id(device, Target { samples: 1, format: R32Uint }, &l)`; `encode_id` appends to the open `Frame` and calls **`scene_list`**; copy + map in `present.rs` | **S1b**, S3a, L2, **S1a** | **the exception app/judge2/judge3 carried is removed** by factoring `scene_list` at 49 (§4). The cloud lane's id (`Splat.id`) is the one genuinely new mechanism |

---

## 10. Cross-cutting rules carried forward (B1-B8)

| class | rule in the target tree |
|---|---|
| **B1** `scene.session` does not exist | `Scene { docs, tables, caches, order, guid_to_row, hidden, … }`. Every `self.session.*` resolves WHICH doc or folds over `docs`. `Doc.session` is the only session. `Scene`'s bookkeeping fields are `pub(crate)` so the `impl Scene` files (S18) reach them. |
| **B2** geometry has no `.xform`; variants are `Rc<T>` | Read placement `session.world_xform(guid)` (bulk `world_xforms()`), write `session.set_xform`, bake `Rc::make_mut(m).transform(&xf)`. `duplicate()` mints a fresh guid; `clone()` clones the handle. `ObjectBase.model: Mat4` is the composed world matrix — never an `Xform` field on geometry. Hits 80 (`BoxTool`), 96 (work plane), 101 (copy). |
| **B3** the `set_scene` wipe | `set_scene` is **append-only** (`base = objects_base.len()`, `extend_from_slice(&up.objects[base..])`); a full rebuild happens only through `Scene::rebuild` (= `reset_arena` + full `set_scene`). Any GPU-only row state — selection (67), hidden (68), cull (62), live models (76), LOD bits (112) — must ALSO land in `scene.tables.objects[row].flags` or be re-applied after each rebuild. **Overlay/preview/marker rows are NOT objects** (Q3), so they are immune. |
| **B4** manifest `place` conjugation | `app/query.rs::world_frame` is the ONE helper: rows are `place × world_xform`; the kernel's `ray_cast`/`world_xform` know nothing about `place`. Deltas committed as a local `set_xform` are `place⁻¹ · delta · place`. Consumed by 64/66 (pick), 81 (snap), 96 (work plane), 76/101 (transform, copy). `placed_frame(row)` returns an **owned** `Xform { m: base[row].model }`. |
| **B5** the two-lane split | 3D linework (iso curves 54, BRep edges 55, trimmed loops 56) goes to `tables.pipes` (SOLID → real cylinders that protrude), never `segments` (FLAT ribbons at surface depth). 90's mask mirrors **all** lane sub-draws via the three `draw_mask` methods, not one. |
| **B6** the runtime-add path | `walk_geometry` on a **scratch `Walk`** over a one-object `ArenaUpload` (A2 + W2) is the single append-one-object verb, reached through 58's `apply_object`. It must repeat the per-FILE planar width flip, pick a TARGET doc, and convert to doc-local coordinates via `place⁻¹`. Used by 73 (restore), 79/80 (draw tools), 101 (copy), 112 (LOD). |
| **B7** the loader owns lib.rs | `Msg::{Ready, File, Clear}` + `ApplicationHandler<Msg>`; the async chunked parse is the only reader. Reload (60), watch (60), drag-drop import (100) are each **one `Msg` variant + one `user_event` arm**. 87's render-on-demand must count `Msg::Ready`/`Msg::File` as poke sites or progressively loaded sheets never appear. |
| **B8** MSAA is dynamic | `msaa_now()` returns 1 (flat-only) or 4 (any solid) and the flip rebuilds `Targets` + **all** pipelines mid-session. **Every new pipeline is a `PipelineDesc` literal inside `Pipelines::new(device, t, &l)`**, so the flip carries it for free. Unconditional `&msaa_view` / resolve targets are invalid on 1× scenes (the PDF sheets) — 74's overlay pass must go through `Targets::begin_pass`, never a hand-built descriptor. Pipelines that must NOT follow the flip pin their own `Target` in the desc (90's R8Unorm-at-4×, 88's RGBA16Float, 114's R32Uint-at-1×). |

---

## 11. Renumber map and what it touches

`old 40` splits into **40** (EDL + attenuation) and **41** (normals + importers). `old 41 → 42`, `old 42 → 43`, **`old 43` is DELETED**, `old 44` stays **44**. The restructure is **45-51**. Everything from `old 45` onward shifts by **+7**:

| on-disk | new | on-disk | new | on-disk | new | on-disk | new |
|---|---|---|---|---|---|---|---|
| 45-49 | 52-56 | 60-64 | 67-71 | 75-79 | 82-86 | 90-94 | 97-101 |
| 50-54 | 57-61 | 65-69 | 72-76 | 80-84 | 87-91 | 95-99 | 102-106 |
| 55-59 | 62-66 | 70-74 | 77-81 | 85-89 | 92-96 | 100-107 | 107-114 |

Files the renumber must touch:

- `session_viewer/docs/NN-*.md` — 63 filenames renamed (`45-nurbscurve.md` → `56-nurbscurve.md` … `107-id-buffer-picking.md` → `118-id-buffer-picking.md`); the 40-44 chain renumber is **DONE (2026-08-29)**: `43-lane-structs.md` deleted, `41-potree-look.md` split into `41-potree-look.md` + `42-cloud-normals.md`, `41-cloud-scenes.md` → `43-cloud-scenes.md`, `42-streaming-cloud.md` → `44-streaming-cloud.md`, `45-cloud-octree.md` unchanged. What REMAINS is the +7 shift of the 63 files from `45-nurbscurve.md` → `56-nurbscurve.md` through `107-id-buffer-picking.md` → `118-id-buffer-picking.md`.
- `session_viewer/docs/NN_*/` snapshot directories, and `.gitignore`'s `docs/*/assets/` rule.
- `session_viewer/docs/_ROADMAP.md` — the lesson list and every forward reference.
- `session_viewer/docs/_RESTRUCTURE_PLAN.md` — the 7-row part table (§8 deltas), the "97 → 18 fields" line, the "15 render + 2 compute" line, the `state.rs`/`persistence.rs` "unchanged" rows, the end-of-44 gate table.
- `session_viewer/docs/_LESSON_AUDIT_36-85.md` — the B1-B8 hit lists cite old numbers.
- `session_viewer/docs/_replay_check.py` — the "Region verbs (lessons 40a+)" header, plus the new whitespace-stripped **body-permutation assertion** for a Move lesson (prerequisite before 45 is written).
- `session_viewer/ARCHITECTURE.md` — §2 tree and §3 replaced by this document; §5's chapter table renumbered.
- **In-lesson cross-references**, which the survey found in at least: 91→98 (phase table, "Phase 6 (45-45)", "tree reveal (83)", "cplane (87)", "81's levers"), 66→73 ("48's free-list"/"45 reclaims" → 57), 90→97, and every "see lesson NN" in 52-114.
- `.claude/` memory topic files that quote lesson numbers (`project_viewer_restructure_40.md`, `project_cloud_splat_lane.md`, `project_cloud_octree_lod.md`, `project_viewer_lane_structs_refactor.md`).

---

## 12. Open risks and what still needs measuring

**Must measure before 45 is typed**

1. **Every line range, on the end-of-44 tree.** Today's `gpu/mod.rs` already runs 8-12 lines ahead of both proposals (`zeroed_buffer` 2087, `storage_buffer` 2106, `unit_cylinder` 2033, `unit_quad` 2065, `line_thickness_px` 2083, `Instance` 1891, `CylinderSegment` 1945, `GlyphPoint` 1998, `LineUniform` 1969, `CloudUniform` 2018, `eye_from_view_proj` 1415, `ortho_half_height` 1457, `msaa_now` 1853, `mk_rows_group` 1256, `update_inside_flags` 1504 — **the fn is `update_inside_flags`, not `update_inside`**). `scene.rs` ranges verified exact today: `mesh_topology` 767-829, `push_mesh` 831-1143 (natural split at the `MESH_RAW_MIN` early-out, **891**), converters 520-588, `point_to_glyph` 590, `xform_point`/`grow_bounds` 1154-1173, `PLANE_SIZE`+frames 1175-1215, `push_cloud`/`cloud_spacing` 1225-1289, `let planar` **478**.
2. **The end-of-44 field ladder.** `113 → 102 → 89 → 66 → 44 → 18` is arithmetic, not a measurement. Count `Gpu`'s fields on the built end-of-44 tree and correct every `ends_with` gate — the field count is the one cheap thing a typist can verify after each part.
3. **The end-of-44 golden numbers** (`lion`, `drawings`, `bunny_drawings`, `bunny`, tubes, REBUILD, INCREMENTAL, `cloud_mix`, `lidar14`), each run twice. The plan records them "once that tree exists"; nothing in 45-51 can start without them.
4. **Lesson 49's doc length.** All three judges name it as the one part at real risk of exceeding a sitting. If it does after the free-shape re-pricing, the split point is between the point lanes (`cloud`/`splat`/`stream`) and the frame (`render`/`present`/`mod`) — and that decision **renumbers 63 lessons**, so it is taken before 45 is typed, never during 49. Judge 3's alternative (move `View` 49→45 and the persistence split 51→46) is the cheaper fallback and crosses no boundary.

**Declared exceptions to the zero-move rule** (each inside the lesson whose own product it is, each golden-gated)

| # | lesson | what | why accepted |
|---|---|---|---|
| 1 | 57 (50) | rewrites `Arena::append`'s body | addressability is 57's product; the edit is confined to one lane file under the goldens. **Largest post-block body change in the curriculum** — 57 must be re-authored from scratch (its single-buffer premise is wrong on main in 25 fences) |
| 2 | 61 (54) | fills `Row.bounds` for every type — **the only deliberate pixel-relevant change after 51** | the fill, the `bounded_rows` narrowing and the `extent` gate must land together and the goldens be re-measured in that lesson. Split across two lessons, the ink on `bunny`/`drawings` changes silently |
| 3 | 82 (75) | adds the reserved-range skip to `walk/bounds.rs` | deferred by the budget rule; if 82 is typed without it, a curves-only file measures thickness 0, is marked `FLAG_SHEET`, and the bug is invisible until such a file is loaded |
| 4 | 107 (100) | splits `render_offscreen` in `present.rs` | the goldens run through that exact path, so the edit is measured rather than argued |

**Residual risks**

- **The 130 pre-seam lines are the only code in the block not covered by the byte-identical-move check.** They are covered by compilation and the goldens, but the three that change an *expression* — `begin_pass`'s load parameters, `View::from_env`'s env reads, and the six converter bodies onto the engine ctors — are where a silent regression would hide. Each is its own compile+golden checkpoint.
- **The pixel gate does not reach `State`.** `examples/selftest.rs` builds `Gpu::new_headless` + `render_offscreen` and never constructs `State`, so S1b (16 lines) and A12's move are compiler-gated only. Lessons 49 and 51 must each end with a browser smoke check, and `clear` must stay the literal composition of `begin_present`/`end_present`.
- **`Gpu`'s lane fields being `pub(crate)`** relies on `gpu.objects.f(&gpu.ctx)` being a disjoint-field borrow. It is — but any future method taking `&mut Gpu` **and** a lane reference will not compose; only `rebase_anchor` and `set_live_models` use the `Gpu`-level forwarder today, and that list should stay short.
- **Parameter ceilings are now binding.** `walk/mesh.rs::push` reaches 5 at lesson 54; `Pipelines::new` is frozen at 3; `write_camera` is frozen at 2 + self via `FrameInput`; `SplatSlot::new` is at 4. Any later input rides `Walk`, `IdxLane`, `FrameInput`, `Binds` or an entry list.
- **The nrm-binding hazard is live between lesson 41 and lesson 49.** On-disk 40 puts `nrm` 5th in `mk_splat_group1` and on-disk 42 passes it 7th; all params are `&wgpu::Buffer`, so it compiles and silently binds the depth buffer as normals. S7a kills the class permanently at 49; until then it needs a **review note in lesson 43's text**.
- **Every "one-line edit" in §9 assumes the lesson is re-anchored first.** The survey found stale Find blocks in most lessons independent of this design (`push_mesh`'s tuple return, `msaa_for` vs `msaa_now`, a 3-arg `clear` that never existed, `Msg::File` arity, `point.wgsl`, `unpack4x8unorm` in cylinder/ribbon, `HashSet<String>` vs `HashSet<u32>`). The +7 renumber plus the re-anchor pass is AI-side work of roughly the same size as the block itself, and it is a prerequisite, not a follow-up.
- **`persistence.rs` is ~453 lines when 45 starts** (292 today + 161 from lesson 43). If A11 slips out of 51, the app ships a file 50% over the cap that lessons 100 and 105 both edit.

---

# Revision 3 — the maintenance kit (2026-08-30)

Five maintainer lenses (cold newcomer, 2am debugger, rule enforcer, two-years-later evolver,
curriculum maintainer) reviewed revision 2 against the code. The axis, the maps, the recipes and
the borrow rules survived untouched. What they found missing is **the machinery that makes this
document's own rules true** — the section below is the addition, and its artefact A is a
BLOCKER: seven lessons of "the picture did not change" rest on a gate that has no runner today.

## 1. Verdict

The spec's axis, its maps, its recipes and its borrow rules are right, and nothing below touches them. What it skipped is **the machinery that makes its own rules true**: it names four `#[cfg(test)]` functions as "the only mechanism that keeps the compartments after the tutorial ends" (`docs/_ARCHITECTURE_TARGET.md:304`) while `session_viewer` has zero `#[test]` today, `.cargo/config.toml:4` pins every bare `cargo` command to `wasm32-unknown-unknown` (where `std::fs::read` does not link and no test binary runs), and the only viewer CI step is `trunk build --release` (`.github/workflows/viewer-pages.yml:37`) — so as written, `cargo test chain_table` cannot execute and never will. Second, the pixel gate cited ~20 times has no artefact: no baseline file, no compare script, `docs/_moved_check.sh` does not exist, and **four of the six named gate scenes reference gitignored `.pb` files** (`drawings` 5, `cloud_mix` 3, `bunny_drawings` 1, `lidar14` its only file), so a second contributor — or the same person on a new machine — can reproduce two of them. Third, the mirror test in §3 is specified as a field-**name** comparison and is structurally impossible on correct code: Rust `LineUniform` has 8 names against WGSL's 9 (`eye: [f32;3]` vs `eye_x/eye_y/eye_z`), Rust `Instance` has 6 against WGSL's 5 (`_pad`), so lesson 47's "run them before anything else" fails on a clean tree and the reader will weaken the test out of existence — while `CylinderSegment`, `GlyphPoint` and `CloudUniform` (2 WGSL copies each) go unguarded and `Instance`/`CylinderSegment` have no `size_of` assert at all. Fourth, §5.3's twelve-line `scene_list` does not match `encode_frame`: the sheet-fill draw is at position 4 in code (`gpu/mod.rs:1695`, right after faces) and position 8 in the spec, and text (`:1814`) precedes dots (`:1828`) in code but follows them in the spec — and §4 prints a third, different copy that omits `draw_print` entirely. Fifth, `ARCHITECTURE.md` is the file README sends every reader to and it is comprehensively false (`GpuSession` 0 hits in `src/`, `bash/build_viewer.sh` and `session_tests/viewer_sections/` do not exist), yet seam S22 budgets its rewrite at **0 lines** and lesson 51's row never names it. Sixth, the vocabulary is unmanaged: `lane` occurs 173 times in `src/` in at least four incompatible senses, and the spec adds `IdxLane::Solid` for the **faces** run while today's code says "the SOLID lane" for mesh **edges** (`pipelines/mod.rs:31`, `lib.rs:300`) — a head-on collision that is free to remove while the name is still on paper. **One blocker**: the test-runner + gate-artefact hole, because seven lessons of "the picture did not change" rest on it and the block cannot honestly start without it; everything else is high-value but survivable.

---

## 2. The kit

| artefact | what it is | what it prevents | lesson | lines |
|---|---|---|---|---|
| **A** `.cargo/config.toml` alias + `.github/workflows/viewer-check.yml` | `xtest` alias pinning the native target; a CI job running `cargo xtest` + both `cargo check --all-targets`, gating the Pages deploy | The four rule-keeping tests never running. Today `cargo test` builds a wasm binary that cannot execute and `compartments_hold`'s `read()` does not link. | **45** (prereq) | 30 |
| **B** `docs/_gate.sh` + `docs/_GOLDENS.tsv` | one command that runs the 4 clean-clone scenes × 4 configs × 2 passes, records ink + draws + objects + PPM sha256, diffs against a tracked TSV; `--record` re-baselines | A gate whose reference values live in prose on one laptop, and a baseline measured on an uncommitted tree (`show_mesh_edges` was added after the spec's 99-field measurement). | **45** (prereq) | 55 |
| **C** `docs/_replay_check.py --moves` | the promised `_moved_check.sh`, folded into the parser that already reads `**Move**`/`**Replace-all**` ops | A dropped line inside a `#[cfg(target_arch)]` arm — invisible to the compiler on the default wasm target *and* to the goldens. | **45** (prereq) | 40 |
| **D** offset-table mirrors in `gpu/buffers.rs` | one `#[cfg(test)] fn mirror(src, name, &[(field, offset)])` + 5 call sites + 2 missing `const _: () = assert!` | The name-list test failing on correct code and being deleted; and stride drift in the 3 mirrored structs the spec leaves unguarded. | **47** | 35 |
| **E** `ARCHITECTURE.md` §0 "First hour" | the run/gate/test commands, a 10-row symptom→file table, the 24-knob table | A cold reader whose only orientation doc points at `GpuSession`, `adapters.rs` and `bash/build_viewer.sh`, none of which exist. **This is S22's real budget.** | **51** | 90 |
| **F** `ARCHITECTURE.md` §0.1 glossary | 18 terms, one line each, owner file named; the `lane` ruling | `lane` meaning four things and `IdxLane::Solid` colliding with "the SOLID lane". | **45** seeded, **51** complete | 30 |
| **G** §5.2 / §5.5 amendments | `//!` header law · naming law · recipe step 8 (`walk/bounds.rs`) · `GrowBuf.label` | 25 unlabelled files; six suffixes for one role; a new family silently excluded from `file_extent`; 16 named buffers going anonymous in every wgpu error. | **46**–**50** | 35 |
| **H** `compartments_hold` hardening | strip comments before the litmus; assert `Gpu`'s field-name set, `//!` presence, the knob table, `Bases` coverage | A test that fails on day one (5 banned substrings are live in comments Rule 2 requires to move byte-identically) and covers 9 of ~35 files. | **50** | 30 |
| **I** free corrections | one canonical `scene_list`; 51→59 propagated in 6 places; `IdxLane` → `IdxRun { Faces, Print, Text }` | A doc that teaches the wrong frame order; a lesson author guessing which of two answers is current. | in place | 12 |
| | | | **total** | **357** |

---

## 3. Per-artefact detail

### A — make the tests runnable (prerequisite, before 45 is drafted)

`session_viewer/.cargo/config.toml`, three lines appended:

```toml
# Tests and examples are NATIVE: wasm32 has no test runner and no std::fs.
[alias]
xtest = "test --target x86_64-unknown-linux-gnu"
```

Every `cargo test` in the spec and in lessons 47-51 becomes **`cargo xtest`**. Every test file is headed `#![cfg(not(target_arch = "wasm32"))]` and roots its paths at `env!("CARGO_MANIFEST_DIR")`, never a relative `src/...`.

`.github/workflows/viewer-check.yml` — same `paths:` filter as `viewer-pages.yml` **plus `session_rust/**`** (the daily kernel-audit bot bumps the gitlink, which does not match `session_viewer/**`, so today a kernel API change cannot trigger the only job that compiles the viewer):

```yaml
name: viewer-check
on: { push: { branches: [main], paths: ['session_viewer/**','session_rust/**','.github/workflows/viewer-check.yml'] }, pull_request: {}, workflow_dispatch: {} }
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: git submodule update --init --depth 1 session_rust
      - run: rustup target add wasm32-unknown-unknown
      - uses: Swatinem/rust-cache@v2
        with: { workspaces: session_viewer, shared-key: viewer }
      - run: cd session_viewer && cargo check --target wasm32-unknown-unknown --lib
      - run: cd session_viewer && cargo check --target x86_64-unknown-linux-gnu --all-targets
      - run: cd session_viewer && cargo xtest
```

Add `needs: check` to `viewer-pages.yml`'s `build` job so a broken compartment cannot deploy. Add one line to `CLAUDE.md`'s Git section: *a submodule pointer bump now triggers `viewer-check` — watch it too.*

### B — `docs/_gate.sh` + `docs/_GOLDENS.tsv` (prerequisite)

The mandatory gate is the **four scenes that resolve entirely to tracked `.pb`**: `lion`, `bunny`, `bunny_cloud`, `drawings_rotated`. `drawings`, `bunny_drawings`, `cloud_mix` and `lidar14` stay in the TSV as an **advisory local-only** block, skipped loudly by name when their assets are absent — the spec must stop citing them as the gate.

```bash
#!/usr/bin/env bash   # docs/_gate.sh  [--record]
set -euo pipefail; cd "$(dirname "$0")/.."
T=x86_64-unknown-linux-gnu; OUT=${TMPDIR:-/tmp}/gate.ppm; NEW=$(mktemp)
for s in lion bunny bunny_cloud drawings_rotated; do
  for cfg in "" "VIEWER_LINE_STYLE=tubes" "VIEWER_REBUILD=1" "VIEWER_INCREMENTAL=1"; do
    for pass in 1 2; do                       # house rule: measure TWICE
      r=$(env $cfg cargo run -q --example selftest --target $T --release -- \
            "$OUT" "assets/scenes/$s.toml" 2>/dev/null)
      ink=$(sed -n 's/.*non-background pixels: \([0-9]*\).*/\1/p' <<<"$r")
      dro=$(sed -n 's/.*: \([0-9]*\) draws, \([0-9]*\) objects.*/\1 \2/p' <<<"$r")
      printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$s" "${cfg:-default}" "$pass" "$ink" "$dro" \
             "$(sha256sum "$OUT" | cut -c1-16)" >> "$NEW"
    done; done; done
if [ "${1:-}" = --record ]; then mv "$NEW" docs/_GOLDENS.tsv
else diff -u docs/_GOLDENS.tsv "$NEW" && echo "gate OK"; fi
```

Three things ride this file and cost one line each: the **PPM sha256** (a scalar ink count passes when geometry moves and the count does not — the sha does not, at zero repo weight); `git tag end-of-44 <sha>`, named in the TSV header, because the spec's baseline is an uncommitted working tree (`show_mesh_edges: false` at `gpu/mod.rs:329,919` was added *after* the 99-field measurement and gates two draws at `:1708,:1758`, so the quoted `lion 4/1` pair is already stale); and `!session_viewer/Cargo.lock` after `.gitignore:44`, because `wgpu = "29.0"` is a caret range and an unpinned resolver makes a golden diff indistinguishable from a regression. Also fix `examples/bench_lines.rs:17` — `p.ends_with(".json")` only, while every scene is `.toml`, so the one frame-time harness has been dead since the format migrated.

### C — `docs/_replay_check.py --moves`

`_moved_check.sh` is promised twice (`:423`, `:454`) and does not exist; `_replay_check.py` already parses every op into `(verb, target, find_blocks, arg, doc_line)` and already copies a snapshot tree, so this is a mode, not a file:

```python
def moves(snap, work, doc):            # after apply(work, doc)
    def bag(p):                        # sorted multiset of stripped, non-blank lines
        return sorted(l.strip() for l in p.read_text().splitlines() if l.strip())
    fails = []
    for src, dsts in move_map(doc).items():        # from the **Move** ops already parsed
        before = collections.Counter(bag(snap / src))
        after  = collections.Counter(itertools.chain(*(bag(work / d) for d in {src, *dsts})))
        lost, gained = before - after, after - before
        if lost: fails.append((src, "LOST", list(lost)[:8]))
        expected = declared_replace_all(doc) + declared_new_lines(doc)
        if (gained - collections.Counter(expected)): fails.append((src, "UNDECLARED", ...))
    return fails
```

Second mode, `--stale <tree>`, at ~15 more lines: for every op in every `docs/NN-*.md`, assert the target path exists and each Find literal occurs **exactly once**; print `lesson · doc_line · verb · target · verdict`. That is the enumeration the +7 re-anchor pass needs (§12 calls it a prerequisite and schedules nothing) — 38 of the 63 downstream lessons name `gpu/mod.rs`, `app/scene.rs` or `pipelines/build.rs` in a verb line, and all three dissolve.

### D — offset-table mirrors, in `gpu/buffers.rs`, called from each family file

The §3 test as written cannot pass. Compare **offsets and total size**, which is what the hazard actually is:

```rust
#[cfg(test)]
pub fn mirror(src: &str, name: &str, want: &[(&str, usize)], size: usize) {
    let body = src.split(&format!("struct {name}")).nth(1).unwrap().split('}').next().unwrap();
    let mut off = 0usize; let mut got = Vec::new();
    for f in body.lines().filter_map(|l| l.split_once(':')) {
        let (n, ty) = (f.0.trim().trim_start_matches(','), f.1.trim().trim_end_matches(','));
        let (sz, al) = match ty.split(&[' ', '/'][..]).next().unwrap() {
            "f32" | "u32" | "i32" => (4, 4), "vec2<f32>" => (8, 8),
            "vec3<f32>" => (12, 16), "vec4<f32>" => (16, 16), "mat4x4<f32>" => (64, 16),
            t => panic!("{name}: unknown wgsl type {t}"),
        };
        off = (off + al - 1) / al * al; got.push((n.to_string(), off)); off += sz;
    }
    assert_eq!(got.iter().map(|(n,o)|(n.as_str(),*o)).collect::<Vec<_>>(), want, "{name} drifted");
    assert_eq!((off + 15) / 16 * 16, size, "{name} stride");
}
```

Five call sites, one per mirrored struct, each inside the family that owns it — `instance.rs` (`Instance`, 5 shaders, 96 B), `frame.rs` (`LineUniform`, 5 shaders, 48 B; `CloudUniform`, 2, 16 B), `segments.rs` (`CylinderSegment`, 2, 40 B), `glyphs.rs` (`GlyphPoint`, 2, 48 B) — plus the two static asserts that do not exist today: `const _: () = assert!(size_of::<Instance>() == 96);` and `== 40` for `CylinderSegment` (only `LineUniform` at `gpu/mod.rs:2022` and `GlyphPoint` at `:2042` have one). §5.2's family shape gains the mirror line so a future family arrives with it.

### E — `ARCHITECTURE.md` §0 "First hour" (S22's real budget: 90 lines, not 0)

Widen S22 from "§2/§3" to the whole file — §6's `session_tests/viewer_sections/` rule and §7's `bash/build_viewer.sh` both point at paths that do not exist, and rule 2's "engine contains zero references to `session_rust`" is false at `pipelines/build.rs:1` and `gpu/mod.rs:15` (the spec corrects it at `:539`, inside a planning document that will not ship). Add lesson 51's row: *rewrite ARCHITECTURE.md — §0 new, §2/§3 replaced, §6 deleted, §7 replaced.* Three blocks:

**§0.a Commands** — `trunk serve --release` (port 8770); `cargo check` (wasm, the default); `cargo check --target x86_64-unknown-linux-gnu --all-targets`; `cargo xtest`; `./docs/_gate.sh`; `cargo run --example check_determinism|check_lean --target …`; and the **browser smoke check**, which the spec mandates four times (`:410,413,454,630`) and never defines — twelve numbered observations: the four console lines in order (`adapter:` `gpu/mod.rs:391`, `viewer init OK` `:831`, `scene: N objects` `:1107`, and **no** `wgpu on_uncaptured_error` `:413`), orbit/pan/zoom, view keys 1-7, `E` (mesh edges — the lane that is **off** by default), `L`, `[`/`]`, a window resize, `?perf=1`. It is the sole gate on `clear`/`begin_present`/`end_present`/`impl State`, which `render_offscreen` never touches.

**§0.b Symptom → compartment** — ten rows, `symptom | files upstream → downstream | the knob that isolates it`. This is the payoff of the split and the one thing the spec never converts into a page: *black screen* → `present.rs` (adapter line, `on_uncaptured_error`); *a whole lane missing* → the `set_scene` row-count line → the `walk/` producer → `View`'s gate → `append`'s `grew` bool vs `rebind`; *every object past row 0 wrong* → `instance_mirror` → `instance.rs` + 5 `.wgsl`; *one lane's ink drifting, four correct* → `line_uniform_mirror` → `frame.rs`; *cloud wrong place/size* → `SplatRecord` word indices → `splat.rs`; *sheets z-fighting* → `IdxRun` + `msaa_now` → `arena.rs`/`targets.rs`; *lines too thick at some zooms* → `walk/encode.rs::encode_width` → the planar sheet override (`scene.rs:487-497`) → `line_thickness_px` (`gpu/mod.rs:2114`, reads env **and** `?thickness=`) → `ribbon.wgsl`/`cylinder.wgsl`; *mesh shading wrong* → `session_rust/src/render_mesh.rs::vertex_normal` → `to_render()` → `triangle.wgsl:109`; *nothing after a rebuild* → `reset_arena` skips `stream`; *fit view frames nothing* → `walk/bounds.rs::file_extent`. Also: add `to_render()` to §2's Mesh row, which today names `b.mesh()` for BRep and `s.mesh()` for NurbsSurface but leaves the Mesh row's left edge unattached to the kernel.

**§0.c Knobs** — the 24 `VIEWER_*`/`BENCH_*`/`DETAIL`/`SELF_CHECK`/`PB_BYTES` names, read across six files, documented in **zero** places today (`grep -c VIEWER_ ARCHITECTURE.md README.md` → 0, 0). One table: `name | read at | native / browser | what it isolates`. State once, at the top, the fact that lives only in two comments (`performance.rs:38-42`, `gpu/mod.rs:2113`): env vars are unreachable on wasm, so only `?scene=`, `?perf=1` and `?thickness=` work where the viewer actually runs. `View::from_env()` (Q2) absorbs three of the twenty-four; the rest keep their homes and their table row.

### F / G / H / I — the small amendments

**F glossary** — §4 below, dropped in as `ARCHITECTURE.md` §0.1.

**G, four edits to §5.2/§5.5.** (1) *Header law*: §5.2's shape starts at `const SRC` with no `//!`, while §9 step 4 asks for one — so it is written for the first family and forgotten by the third. Put four lines into the shape (what this file owns · who may call it and under which rule (F1-F8 / W1-W7 / B7) · the `.wgsl` files or `Geometry::` variants it touches · the §2/§3 row it implements), and note in lesson 50 that the `scene.rs:1-84` Move takes `scene.rs`'s own `//!` block with it, leaving `scene.rs` headerless. (2) *Naming law*, six lines: module `<family>.rs` · row `<Row>` · CPU sink `<Family>Rows` · GPU holder `<Family>Lane` · `Gpu` field and `Upload` group spelled **identically** · pipelines `Pipes`. Today §4 has six suffixes for one role (`Arena`, `SegmentLane`, `GlyphLane`, `PointBufs`, `SplatSlot`, `InstanceTable`) and `Gpu.glyphs`/`Upload.glyph`, `Gpu.objects`/`Upload.obj` disagree on number. Renames ride the F6 Replace-all that already happens; `InstanceTable` and the `PointBufs`+`SplatSlot` pair stay, each with one sentence saying why. (3) *§5.5 step 8*: `walk/bounds.rs` — one `.chain(rows.iter().skip(base))` in `file_extent` **and** in `sheet_thickness`, plus one `Bases` field. `add_file` captures seven per-population bases (`scene.rs:217-225`) and the sweeps hard-code one skip chain each (`:407,414,420,464,470,484,487`); §5.4's "**Total: 1 new file + 2 lines. Zero engine edits**" is true only for a type writing into an existing group, and the spec's own declared exception 3 names the resulting bug (thickness 0 → whole file `FLAG_SHEET`). (4) *`GrowBuf.label`*: `append_rows` threads `label: &str` into `zeroed_buffer` on every growth (`gpu/mod.rs:109`, `:2146`) and 16 distinct labels are in use; §4's `GrowBuf { buf, count, cap, usage }` drops all of them, so a binding-size error at the exact moment a scene crosses a cap names an unlabelled buffer. One field, FREE-SHAPE.

**H `compartments_hold`.** As sketched it fails on day one: `s.contains("Scene"|"Doc"|"Geometry"|"Session"|"egui")` over `src/engine/gpu` hits live comments that Rule 2 requires to move byte-identically — `gpu/mod.rs:34,43,49,942,1019,1088` and `:1482` ("later an egui slider"), which lands in `frame.rs` at lesson 46 and fails the test the moment the file exists. Strip `//` comments first, match on word boundaries, and add four clauses that cost one line each: `Gpu`'s field-name set equals a `const GPU_FIELDS: &[&str]` of the 18 (a named-list edit is reviewable; `show_mesh_edges` proves the ratchet is already slipping and §5.6 forbids exactly that); every file under `engine/gpu/` and `app/walk/` starts with `//!`; every `VIEWER_`/`BENCH_` literal in `src/` appears in ARCHITECTURE.md §0.c; every `Bases` field is named in `bounds.rs`. Add a `const OVER_CAP: &[(&str, u32, &str)]` allow-list so the two files the spec's §4 tree omits entirely — `src/camera.rs` (356 lines) and `src/lib.rs` (394, B7 says the loader owns it) — are visible debt rows with an owning lesson rather than silence. And while the §6 regroup is happening, `#[derive(PartialEq)]` on the five row groups turns `check_determinism`'s hand-written `same!` list (9 of 18 columns; **`objects`, `object_bounds`, `object_spacing`, `idx_print`, `idx_text`, `cloud_draws` are never compared**) into one `a != b` — the flags column that lesson 51 is *about* is the one it does not check.

**I, free.** One canonical `scene_list`, transcribed from `encode_frame` and line-cited — background `:1665` · grid `:1670` · faces `:1677` · **print `:1695`** · pipes `:1715` · splat_resolve `:1739` · markers `:1768` · ribbons_depth `:1781` · dots_depth `:1790` · ribbons `:1800` · **text `:1814`** · **dots `:1828`** — with one clause per line; delete §4's second copy and cross-reference. Propagate the 59 answer for `persistence.rs` and `impl State { render, resize }` into ledger rows A11/A12 (`:732,:733`), Q9 (`:687`), and `:904`, which today all say 51 against §1/§4/§10's 59. Rename `IdxLane { Solid, Print, Text }` → `IdxRun { Faces, Print, Text }` at lesson 47, where the enum is created.

---

## 4. The vocabulary — `ARCHITECTURE.md` §0.1

| term | one line | owner |
|---|---|---|
| **row** | one fixed-size `#[repr(C)]` record the GPU reads by index; the unit every producer emits | the family file |
| **row format** | one of the five: `RenderVertex`, `CylinderSegment` (40 B), `GlyphPoint` (48 B), `Instance` (96 B), the cloud SoA triple | §2 |
| **family** | a row format together with every shader that reads it — five of them, one file each; **the engine compartment** | `gpu/<family>.rs` |
| **producer** | an `app/walk/*.rs` function turning one `Geometry::` variant into rows; **the app compartment** | `walk/mod.rs` |
| **sink** | the `&mut …Rows` a producer receives; it can write nothing else (W1) | `upload.rs` |
| **`Rows` / `Lane` / `Pipes`** | CPU sink · GPU buffers+bind group · the family's pipelines (naming law, §5.2) | §5.2 |
| **arena** | the one shared vertex+index buffer every tessellating type writes into | `arena.rs` |
| **ink** | screen-width linework and markers — ribbons, tubes, dots — as opposed to fills | `segments.rs`, `glyphs.rs` |
| **pen** | the `(radius, color)` pair encoded per segment; `radius == 0.0` means screen-constant px | `walk/encode.rs` |
| **pipes vs ribbons** | mesh/BRep **edges** as protruding 3D cylinders vs curve/line rows as camera-facing flat quads — the split B5 depends on | `segments.rs` |
| **spheres vs dots** | mesh/BRep **vertices** as markers vs free `Point` rows as SDF dots — same file, different sink | `glyphs.rs` |
| **splat** | the 2-pass atomic compute point rasterizer and its resolve | `splat.rs` |
| **walk** | the one traversal from `Doc` to rows; `walk_geometry` is its only dispatch | `walk/mod.rs` |
| **upload** | the CPU-side `Upload` of five row groups handed to `set_scene` once and dropped except `obj` | `upload.rs` |
| **anchor / rebase** | the camera-relative origin instance rows are expressed about; re-anchoring rewrites every `model` | `objects.rs` |
| **print / text run** | index runs off the same arena verts encoding **draw order** for PDF fills and lettering, not geometry | `arena.rs` |
| **base** | a per-file starting index into a population, so a file sweep touches only its own rows | `walk/bounds.rs` |
| **knob** | a runtime toggle that gates a draw or picks a pipeline; never a uniform, never a `pub` field on `Gpu` | `view.rs`, `knobs.rs` |

**Terms used inconsistently today — flagged, then fixed by this glossary.** `lane` occurs **173 times in `src/`** in four senses: a drawing population (`pipelines/mod.rs:31` "the SOLID flat lane", `pipelines/build.rs:472,548,552` "FLAT lane"/"SOLID lane", `gpu/mod.rs:158-171` "Solid lane"/"Flat lane"/"Raw lane"/"Sheet lanes"), a shader half (`cylinder.wgsl:91` "the tube lane's half", `sphere.wgsl:198,244` "the ribbon lane"), a data type (`splat.wgsl:1` "the cloud lane"), and — added by the spec — an index run (`IdxLane`) and a buffer holder (`SegmentLane`, `GlyphLane`). **`IdxLane::Solid` names the *faces* run while "the SOLID lane" in `lib.rs:300` and `pipelines/mod.rs:31` means mesh *edges*** — renamed `IdxRun::Faces` by amendment I. `family`, `producer` and `sink` appear **0 times** in `src/` today; the header law (G1) is what puts them there. `spacing` carries two units into one f32 — world for meshes (`scene.rs:681-688`), screen pixels for clouds (`:326-328`) — which §2 names and `Row::world_spacing` / `Row::point_size_px` label at the write site.

---

## 5. What was correctly left out

| proposed | why not |
|---|---|
| `examples/ppm_diff.rs` + committed reference images | 60 lines and repo weight for a solo viewer; the PPM **sha256** in `_gate.sh` catches "pixels moved, count unchanged" for one line, and localization is what the browser is for. |
| A snapshot-crate build checker (`bash/check_snapshots.sh`, `SNAPSHOTS.md`) | The 45 `docs/NN_*/` crates are frozen chapter artefacts by design; making 45 stale crates a green-CI obligation is a permanent tax to protect a `diff -u` reference. |
| Offline WGSL validation via a `naga` dev-dependency | `_gate.sh` already turns a broken shader into ink=0 on four scenes; a new dependency to re-detect what one command detects is not worth its line. |
| `VIEWER_DUMP` row dump + `VIEWER_LANE` single-lane isolation | Genuinely useful, genuinely not needed to *land the block* — and the symptom table plus the existing 24 knobs cover the same ground. Revisit when a bug demands it. |
| Per-family `FrameStats` and `Performance` breakdown | S20 already defers the perf counters to lesson 62; splitting one accumulator early adds a struct to the highest-risk lesson for no gate. |
| A perf/`.wasm`-size regression gate | Seven lessons that move code byte-identically cannot regress frame time meaningfully; the one-line `bench_lines.rs` `.toml` fix rides lesson 45 and that is enough. |
| `docs/_TEMPLATE.md` rewrite, `@lesson` tags, a file→lesson index | Curriculum tooling, not viewer maintenance, and the +7 renumber must land first or every hand-written link is wrong. |
| Manifest `deny_unknown_fields` + `Result` error path | A behaviour change under a pixel gate, in a format one person authors; §5.6 has no manifest row because a manifest field is one `#[serde(default)]` line. |
| A `params()` ≤5 ratchet with a 14-entry allow-list | The cap is real but the allow-list is bureaucracy at this size; `GPU_FIELDS` in `compartments_hold` guards the number the spec actually headlines. |

---

## 6. Where each addition lands

| lesson | what it gains | added lines |
|---|---|---|
| **prereq** (before 45 is drafted; §8's "two artefacts" becomes **four**) | **A** the `xtest` alias + `viewer-check.yml`; **B** `_gate.sh` + `_GOLDENS.tsv` + `git tag end-of-44` + `!Cargo.lock`; **C** `_replay_check.py --moves`; the corrected gate scene list (4 mandatory / 4 advisory) | 125 |
| **45** — *a pipeline is data* | **F** glossary seeded (the block's vocabulary is introduced here); **I** the `scene_list` correction and the 51→59 propagation land as doc edits; the `bench_lines.rs` `.toml` one-liner rides "delete before you move" | 16 |
| **46** — *the floor is not a lane* | **G4** `GrowBuf { …, label: &'static str }` (FREE-SHAPE, rides a struct being created); **G1** the `//!` header law enters §5.2's shape and every file 46 creates carries one | 12 |
| **47** — *one row per object* | **D** `buffers::mirror` + 5 call sites + the 2 missing `size_of` asserts, replacing the impossible name-list test; **I** `IdxLane` → `IdxRun { Faces, Print, Text }`; `#[derive(PartialEq)]` on `ObjectRows`/`ArenaRows` so `check_determinism` covers the flags column | 40 |
| **48** — *one row, two shaders* | **G2** the naming law applied as part of the `seg`/`glyph` Replace-all (`Gpu.glyphs`↔`Upload.glyph`, `Gpu.objects`↔`Upload.obj` aligned); `mirror` calls for `CylinderSegment` and `GlyphPoint` | 8 |
| **49** — *the frame is a list* | the canonical `scene_list` transcribed from `encode_frame` with per-line reasons; `mirror` for `CloudUniform`; the `set_scene` log line extended to name every group | 10 |
| **50** — *narrow sinks* | **H** `compartments_hold` hardened (comment-stripped litmus, `GPU_FIELDS`, `//!`, knob-table, `Bases`, `OVER_CAP` with `camera.rs`/`lib.rs`); **G3** recipe step 8 (`walk/bounds.rs`) written into §5.5 as `bounds.rs` is created | 38 |
| **51** — *adapters, not copies* | **E** the `ARCHITECTURE.md` rewrite as a **named, budgeted deliverable in the what-moves column** — §0 commands + symptom table + knob table, §0.1 the glossary complete, §2/§3 replaced, §6 deleted, §7 rewritten; the five `//!` section pointers in `src/` (`state.rs:2,39`, `lib.rs:3`, `engine/mod.rs:1`, `gpu/mod.rs:1`) re-pointed | 108 |
| | | **357** |

**Deferred, deliberately.** `_replay_check.py --stale` runs against the post-51 tree as the **first step of the +7 re-anchor pass**, not inside the block. The `Spacing` enum stays at the first lesson needing both units; the WGSL prelude stays at 104 (`mirror` is what covers the gap until then, now that it can actually run); `persistence.rs`'s 3-way split and `impl State { render, resize }` stay at **59** in all six places. `src/camera.rs`'s re-root to `engine/camera.rs` is **not** scheduled — it is recorded as an `OVER_CAP` debt row with lesson 62 (`Frustum`) as its owner, because moving it buys nothing the block needs and costs a golden.
---

# Revision 4 — measured on the real end-of-44 tree (2026-08-31)

Revisions 1-3 were written against an end-of-39 working tree and predicted end-of-44 by arithmetic.
The tree now exists: lesson 43 was re-authored to deliver all 31 of its ops (it delivered 9), lesson
44 was re-authored onto main's flat cloud lane (33 ops), and the chain replays green from the
end-of-42 snapshot and compiles on both targets. **Every number below was measured on that tree,
twice, both passes identical. Where this section disagrees with revisions 1-3, this section is
right** — and each disagreement is a gate the seven lessons must be re-based on.

Baseline tree: replay `docs/44-streaming-cloud.md` then `docs/45-cloud-octree.md` onto the
end-of-42 source. Goldens: `docs/_GOLDENS.tsv`, 64 rows, recorded by `docs/_gate.sh --record`.

## The corrections

| # | revisions 1-3 say | measured at end-of-44 | what it changes |
|---|---|---|---|
| 1 | `Gpu` = **113** | **116** | the whole field ladder |
| 2 | ladder `113→103→86→63→43→18` (§8) *and* `113→102→89→66→44→18` (§1) — the document contradicts itself | start = **116** | every lesson's exit gate — see below |
| 3 | today's `Gpu` = 99 / 98 | end-of-42 = **102** | the base of #1 |
| 4 | `encode_frame` = **310** lines | **271** (`gpu/mod.rs:1845-2115`) | lesson 49's headline shrinks: 43 already pulled `splat_records` (114 lines) out |
| 5 | `set_scene` = 197 lines | **194** (`1022-1215`) | minor |
| 6 | `encode_splat` exists, seam S7b is free-shape "because it exists at end-of-44" | **there is no such function** — the compute encode is inline in `encode_frame:1860-1900` | S7b rests on a phantom. The *shape* (records → depth-for-all → colour-for-all, two lanes) is real; **extracting it is new code in lesson 49 and must be budgeted** |
| 7 | `COPLANAR_DOT = 0.9999` | **`1.0 - 1e-9`** (`scene.rs:774`) | five orders of magnitude; §2's Mesh row is wrong |
| 8 | `upload_to` drops **13** columns | **14** (44 adds `cloud_nodes`) | the count asserted in 46 and 49 |
| 9 | **one** live `VIEWER_NO_DEPTH` branch (`build.rs:621`) | **two** — `build_sphere_pipeline:461` **and** `build_ribbon_solid_pipeline:621` | lesson 45's descs must preserve **both** |
| 10 | `show_mesh_edges: false`, added after the 99-field measurement | **`true`** | the stated reason for calling `lion 4/1` stale does not exist |
| 11 | "the quoted `lion 4/1` pair is already stale" | **not stale**: 77543 ink / 4 draws / 1 object, all four configs, both passes | keep the assertion |
| 12 | `cloud_mix 11/210892 (Tubes 10)` | **confirmed exactly** (7469 ink / 11 / 210892; tubes 7455 / 10) | keep |
| 13 | the pixel gate is a PPM checksum | **two independent nondeterminisms** — see below | `drawings_rotated` is the ONLY mandatory scene a checksum still gates |

**Confirmed correct and unchanged:** 10 `.wgsl` files · `build.rs` 845 · `ArenaUpload` **19** columns
· **12** `_cap` fields · `arena_vids` has no cap and rides `arena_vert_cap` · `Pipelines::new` 10
params · 15 render pipelines · `edges` has **0** draw sites (yet is still declared, built and
compiled) · `push_mesh` **314** lines / 8 params / `-> (Option<Bounds>, bool)` · **9** layout blocks
in `Gpu::build` · `state.rs` 48 · `persistence.rs` 453 · `MESH_RAW_MIN` 200_000 ·
`WIREFRAME_BLACK_MIN` 10_000.

## The ladder, re-based

`116` start with revision 2's per-lesson deltas gives `116 → 106 → 89 → 66 → 46 → 21`, which
overshoots §4's **enumerated** 18-name end state by three. The three are exactly
`show_points`, `show_lines`, `show_mesh_edges` — the Q/W/E lane toggles typed after revision 2 was
measured. They are **knobs, not uniforms** (§5.6), so `View` absorbs them at lesson 46 along with
`line_style`, `cloud_size`, `edl_strength` and `lod_split_px`, and the end state stays **18**.

**Corrected ladder: `116 → 106 → 88 → 65 → 45 → 18`.** It is still arithmetic. Each lesson's
Expected-state block quotes the count measured on its own output tree, and the count is the one
cheap thing a reader can verify after every part.

## The pixel gate is weaker than revisions 1-3 assume

Two independent sources of frame-to-frame nondeterminism, both found by the house double-run rule,
neither previously known:

1. **The splat lane.** A 2-pass **atomic** compute rasterizer: which point wins a contested pixel is
   a race. `lion` produced **three distinct PPM checksums over eight runs** while ink/draws/objects
   stayed 77543/4/1 exactly. Affects `lion`, `bunny_cloud`, `cloud_mix`, `lidar14`, `bunny_drawings`.
2. **The mesh lane.** `bunny` carries **no cloud** and still gave **four distinct checksums over 24
   runs** under `VIEWER_REBUILD=1` (three over twelve at default). The magnitude is **three bytes of
   2,880,016** — one pixel at (625, 220), grey 171 against 170 — below the ink threshold, so the
   scalar counts never move. Root cause not yet chased; it is not the output path (a private
   `TMPDIR` produced a fourth checksum) and not the splat lane.

So a lesson gate is `ink + draws + objects` on three of the four mandatory scenes, and only
`drawings_rotated` still carries a checksum. Revision 3's claim that the sha256 catches "pixels
moved, count unchanged" holds for exactly one scene. Restoring it elsewhere needs a deterministic
tie-break in `splat.wgsl` (depth-then-index `atomicMin`) and a diagnosis of the mesh case — both
behaviour changes in `src/`, and therefore **not** part of a moves-only block.

---

# Revision 5 — the ladder as BUILT (2026-08-31, after 45-48 shipped)

Revision 4 predicted `116 -> 106 -> 88 -> 65 -> 45 -> 18` from the deltas. Four lessons are now
built and gated, and the measured numbers are **revision 3's original arithmetic**, not
revision 4's:

| | 44 | 45 | 46 | 47 | 48 | 49 |
|---|---|---|---|---|---|---|
| predicted (rev 4) | 116 | 106 | 88 | 65 | 45 | 18 |
| **measured** | 116 | **106** | **86** | **63** | **43** | 18 (target) |

Revision 4's two wrong slots came from the same miscount: `mvp_f32`, `last_ortho_h` and `last_eye`
went into `FrameUniforms` at 46 (they are computed with the buffers they sit beside), and
`last_rebase_ms` went into `InstanceTable` at 47 (it throttles nothing else). The 49 target is
unchanged and now arithmetic rather than hope: `Gpu` holds 43, of which 25 are the point lanes and
the splat machinery, and 43 - 25 = 18 exactly matches §4's enumerated end-state list.

## Corrections to §8's table, from building it

- **45.** `Pipelines::new` takes `device`, not `ctx` — `GpuCtx` does not exist until 46. The
  3-parameter signature must land in the SAME step as `PipelineDesc`, because a desc literal
  references `&l.mvp`. **Two** `VIEWER_NO_DEPTH` branches exist, not one (`sphere` and
  `ribbon_solid`); both are preserved and the grep count of 2 is itself a gate.
- **46.** `present.rs` and `view.rs` moved here from 49 as planned; the `Gpu` head order
  (`surface, ctx, config, layouts, pipelines, frame, targets, view`) is set here, not at 49.
- **47.** `INSTANCE_ID_ATTRIBS` and `instance_id_layout` move from `pipelines/build.rs` into
  `arena.rs` — `vids` is that family's second vertex buffer and nothing else has one. The old
  `reset_arena` never cleared `base_f32`, so a rebuild read the previous scene's rotation and
  scale; `InstanceTable::clear` fixes it, and it is the block's one declared behaviour change.
- **48.** `Template` (a positions-only instanced mesh) is shared by both ink families and lives in
  `segments.rs`; `glyphs.rs` imports it. `GrowBuf::append` arrives here, but `append_rows`'s
  six-parameter form survives until 49 because the raw and streamed point lanes are not `GrowBuf`s
  yet. Every family draw RETURNS its draw count — `&mut Gpu` plus a `&Binds` borrowed from it is
  E0502, and a shared counter cannot be threaded through.

## Two rules for writing the remaining lessons, learned the hard way

1. **A moved body is extracted from the tree, never retyped.** `unit_cylinder` was retyped from
   memory into `segments.rs` and came out with different triangle winding — invisible to the
   compiler, invisible to the pixel gate until a tube is seen end-on. Every moved region is now
   pulled programmatically and `difflib`-checked against its original before the doc is written.
2. **Write the ops before the prose.** Build an ops-only scratch document, iterate
   replay -> `diff -r` until the tree is byte-identical, and only then wrap prose around the
   proven blocks. Doing both at once means every prose edit risks an anchor.

---

# Revision 6 — the block as BUILT (2026-08-31, all seven lessons shipped)

| | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 |
|---|---|---|---|---|---|---|---|---|
| `Gpu` fields | 116 | 106 | 86 | 63 | 43 | **18** | 18 | 18 |
| `engine/gpu/mod.rs` | 2,447 | 2,139 | 1,691 | 1,336 | 1,055 | **524** | 524 | 524 |
| `app/scene.rs` | 1,382 | 1,365 | 1,341 | 1,341 | 1,335 | 1,333 | 739 | **284** |
| `engine/pipelines/mod.rs` | 80 | 148 | 148 | 130 | 67 | **52** | 52 | 52 |

`Gpu`'s end state is §4's enumerated 18 exactly. `gpu/mod.rs` is 524 rather than the ~300 targeted:
what remains is `build` (~250 lines of resource construction), `set_scene`, `resize`, `reset_arena`,
`msaa_now` and the re-export list. Splitting `build` is a **57** candidate, not a 49 one.

## What the spec got wrong, corrected by building it

1. **The ladder.** Revision 4's `88 / 65` were wrong in both slots; revision 3's arithmetic was
   right. `mvp_f32`/`last_ortho_h`/`last_eye` went to `FrameUniforms` at 46, `last_rebase_ms` to
   `InstanceTable` at 47.
2. **`encode_splat` does not exist** and never did; the compute encode is inline in `encode_frame`.
3. **49's suggested payoff does not work.** "Move `draw_text` above `draw_ribbons`, watch lettering
   go under the ink" changes **0 pixels** — `drawings_rotated` carries no lettering. Replaced with
   the vertex-marker reorder: 25,353 pixels.
4. **`walk_geometry` cannot take narrow sinks yet.** It takes `&mut Upload` and reaches disjoint
   FIELDS, which is the only reason the mesh arm compiles; a `t.arena_mut()` accessor borrows all
   of `t`. Narrow sinks need the mesh split first, so 50 ships `Row` instead.

## Three gates the spec never named, all earned by a real defect

- **`--render`** (fence parity, duplicate `Create` bodies, repeated large blocks). Lesson 47
  shipped ~900 lines rendering inside-out from one mistyped ` ```text `; replay, `--audit` and
  `--moves` were all green.
- **The warning count.** Three new "never used" warnings exposed an edit script that asserted its
  way out before writing, so a whole batch of fixes silently never landed.
- **The chain replay.** The only check that catches an anchor in lesson N+1 invalidated by a prose
  edit to lesson N.

## The two behaviour changes, both declared in their lessons

1. **47** — `InstanceTable::reset` clears `base_f32`; the old `reset_arena` forgot it, so after a
   rebuild `rebuild` read the previous scene's rotation and scale.
2. **51** — `MeshOpts::allow_open` gives `Element(Mesh)` back `FLAG_OPEN`. No mandatory scene holds
   an Element, so the pixel gate is silent either way.

Plus one PRE-EXISTING bug found and fixed at its origin: lesson 43's streamed lane bound its
pixel buffers where its normals belonged (`docs/44-streaming-cloud.md`, both call sites). No gate
scene streams, so nothing had ever exercised it.
