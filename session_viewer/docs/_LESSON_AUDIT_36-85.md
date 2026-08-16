# Lesson audit 36-85 — against the post-35 tree (2026-08-12)

Audited every lesson after 35 against: the rewritten lesson-35 end state (`35_scene_struct/`
snapshot), the live kernel (Xform-on-Session refactor, `Rc` object storage, pub
`to_proto`/`from_proto`, P2/P3 of the datastructure plan — both DONE 2026-08-01), the deleted
`session_io` (importer now `session_rust/src/pdf.rs` behind `--features pdf`), and the current
`gpu/mod.rs` (empty-start `new()`, zero-copy `set_scene`, two-lane tables, `msaa_for` per scene).
Every claim below was verified against source, with lesson line numbers in the per-lesson notes
kept by the four audit passes; this file is the synthesis.

## The eight breakage classes (fix these once, most lessons follow)

**B1 — `scene.session` does not exist.** `Scene { docs: Vec<Doc{name, place, session}>, tables,
order, guid_to_row, hidden }`. Every `self.session.*` must resolve WHICH doc (or fold over
`docs`). Hits: 36, 38a, 38b, 39, 40, 42, 43, 44, 51, 54, 56, 57, 59, 60, 61, 62, 63, 64, 70,
71, 73, 74, 75, 79, 80, 81, 82, 83.

**B2 — geometry has no `.xform`; variants are `Rc<T>`.** `m.xform = …` fails twice (no member;
no `&mut` through `Rc`). Read placement: `session.world_xform(guid)` (bulk: `world_xforms()`);
write: `session.set_xform(guid, xf)`; bake: `Rc::make_mut(m).transform(&xf)`. Copies:
`duplicate()` already mints a fresh guid — `geom.clone()` clones the HANDLE, so "clone then
refresh_guid" no longer works at all. Hits: 36, 38a, 38b, 42, 43, 51, 54, 55, 56, 58, 59, 60,
61, 62, 63, 73, 74, 75, 79, 80.

**B3 — the `set_scene` wipe.** Every progressive `Msg::File` calls `gpu.set_scene(&scene.tables)`,
which rebuilds `objects_base`, `instances` (flags!) and all lane buffers wholesale. Any GPU-only
row state — selection bits (49), hidden bits (50), cull bits (41), gumball live models (54-56),
reserved gumball/ghost rows (56, 62) — is ERASED by the next arriving file unless it also lands
in `scene.tables.objects[row]` (or is re-applied after each `set_scene`). This constraint did
not exist when 36-58 were written.

**B4 — manifest `place` conjugation.** Rows are `place × world_xform`; the kernel's
`Session::ray_cast`/`world_xform` know NOTHING about `place`. Picking (46, 48), snapping (63),
work-plane (79), and any world↔local inverse must conjugate by the doc's `place`, or every hit
on a placed sheet is off by its manifest offset. Deltas committed as local `set_xform` must be
`place⁻¹ · delta · place` — cosmetic for translations, visibly wrong for rotations (59).

**B5 — the two-lane split.** 3D linework (iso curves 62, BRep edges 63, trimmed loops 64)
belongs in `tables.pipes` (SOLID lane, real cylinders that protrude), not `segments` (FLAT
ribbons at surface depth) — as written, the "tubes protrude → no z-fighting" claim is false for
exactly those lessons. 73's selection mask must mirror all four lane sub-draws, not one.

**B6 — no runtime-add path yet.** `add_file` is the only table writer. Draw tools (61, 62, 64)
need an "append one object to a doc + its rows" verb, which must also repeat the per-FILE planar
width flip and pick a TARGET doc (and doc-local coordinates via `place⁻¹`).

**B7 — loader owns lib.rs.** `Msg::Ready/Msg::File` + `ApplicationHandler<Msg>`;
`session_from_bytes` is DELETED (only the async chunked parse exists). Reload (42b), watch (44),
drag-drop import (83) should each become a `Msg` variant — the `Rc<RefCell<…>>` inbox machinery
in 40 is now redundant. Render-on-demand (70) must count `Msg::Ready/File` as poke sites or
progressively loaded sheets never appear.

**B8 — MSAA is dynamic.** `msaa_for` returns 1 (flat-only) or 4 (any solid), and the flip
rebuilds depth/msaa views + ALL pipelines mid-session. Any new pipeline (52 overlay, 65 ground,
67 GTAO, 69 outline, 72 text, 85 textures) must be built inside `Pipelines::new(device, samples,…)`
or handle the flip; unconditional `&msaa_view`/resolve targets are invalid on 1× scenes (the PDF
sheets!).

## Verdicts

| # | Lesson | Verdict | Dominant cause |
|---|---|---|---|
| 36 | scene-bvh | **REWRITE** | built in a `Scene::new(session)` that doesn't exist; `.xform` on Mesh/BRep/OBB; thin-geometry boxes must apply placement (correctness bug) |
| 37 | frustum-culling | TOUCH-UP | `clear()` is 2-arg; frustum must come from the ANCHORED matrix; cull bits wiped per append (B3) |
| 38a | gpu-arena | **REWRITE** | reshaped ArenaUpload drops the pipes/spheres lanes; fills a `Gpu::new` that takes nothing; `flatten_mesh` regresses 35's hidden-edge/width rules; layouts already hoisted |
| 38b | reconcile | **REWRITE** | content hash is blind to moves (placement left geometry!); `session_from_bytes` deleted; reload must be a `Msg`; `Doc` doesn't record its url; P3 already landed — cite `order()` |
| 39 | save | TOUCH-UP | `scene.session`; per-doc save policy needed; hash gap inherited from 38b; note placements ride `Session.xforms` (tag 7); P2 done — staleness caveat can go |
| 40 | watch | TOUCH-UP | `session_from_bytes` ×3; inbox → `Msg::Watched`; which doc does a url map to? |
| 41 | screen-to-ray | OK | one stale "mesh.xform" sentence in Next |
| 42 | raycast-meshes | TOUCH-UP | local frame is `place × world_xform`; borrow-order fix (clone xform before `get_mut`); PickHit needs doc identity |
| 43 | subobject-picking | TOUCH-UP | same placement fix; hoist `world_xforms()` out of the vertex loop |
| 44 | pick-thin | TOUCH-UP | **real bug**: `Session::ray_cast` ignores manifest `place` — conjugate ray per doc, loop docs, keep nearest |
| 45 | selection | TOUCH-UP | selection wiped per append (B3); `write_row` must live in gpu (flags private); one wgsl anchor typo |
| 46 | hidden-filter | TOUCH-UP | flags must land in `scene.tables.objects.2` (B3); `build()` prose |
| 47 | egui-hud | TOUCH-UP | missing `let camera` binding; second `thickness: 2.0` site; note empty-start Gpu::new |
| 48 | command-bus | OK | optional: per-doc `show <name>` now natural |
| 49 | command-options | OK | — |
| 50 | history-autocomplete | OK | — |
| 51 | delete-undo | **REWRITE** | `scene.session`; `add_mesh` wants owned value not Rc; snapshot must include `session.xform(guid)` or undo loses placement; rebuild path is tables+set_scene until 38 exists |
| 52 | gumball-geometry | TOUCH-UP | `gb_row` reserved in Gpu::new is wiped by set_scene (B3) — reserve per set_scene or own buffer; overlay pass must branch on `samples` (B8) |
| 53 | gumball-scale-hittest | OK | minor: `tol` source inconsistency |
| 54 | gumball-translate | **REWRITE** | `apply_delta` writes `.xform`/no-arg `transform()`; commit must be `set_xform` (which also fixes undo snapshots: store (guid, Xform) pairs); live drag wiped per append (B3); per-doc frames (B4) |
| 55 | gumball-rotate-scale | TOUCH-UP | inherits 58's commit prose; rotation about wrong origin if place not conjugated (B4) |
| 56 | gumball-numeric | **REWRITE** (Step 3) | `apply_transform_command` on `scene.session` + 58's dead `apply_delta`; Steps 1-2 fine |
| 57 | draw-tools | **REWRITE** | no runtime-add verb (B6); target doc undefined; `Geometry::Line(l)` needs Rc; verify text describes wrong lanes |
| 58 | draw-tools-2 | **REWRITE** | reserved ghost row in Gpu::new wiped (B3); `m.xform = …`; row 0 not free (gray fallback instance) |
| 59 | snapping | TOUCH-UP | `p.xform`/`transformed()`; placement via `place × world_xforms()`; GRID_STEP coupling with 65 |
| 60 | nurbscurve | **REWRITE** | 35 already draws curves (the lesson's premise!); its sampler would REGRESS 35's size-adaptive one; keep only CV handles + pick cache + tool |
| 61 | nurbssurface | **REWRITE** | entire SHAPE/PLACEMENT narrative is `.xform`-based; "two copies" claim now false (one Rc); thesis actually gets stronger: tess cache can't be invalidated by transforms BY CONSTRUCTION |
| 62 | isocurves | TOUCH-UP | iso lines in wrong lane (B5); keep vwidth pass when dropping edges; stamp `instance_id` at push time |
| 63 | brep | TOUCH-UP | lane (B5); cache the COLORED mesh (surfacecolor bake); `b.m_curves_3d` confirmed — drop the hedge |
| 64 | trimmed | TOUCH-UP | `all_objects` must be docs-based AND `order()`-canonical (lookup.values() is random); no `add_nurbssurfacetrimmed` — read-only first-class |
| 65 | ground-grid | TOUCH-UP | fade radius uses metres where mm needed (1000× error); pipeline must be built with `samples` inside Pipelines::new (B8) |
| 66 | render-on-demand | TOUCH-UP | `Msg::Ready/File` missing from poke table — progressive sheets would never appear (B7) |
| 67 | gtao | TOUCH-UP | dynamic sample count (B8): 1× scenes break the multisampled depth binding; radius mm→m; bbox grows per append |
| 68 | arctic-gi | OK | — |
| 69 | outline-aa | TOUCH-UP | mask draws only the cylinder lane — flat ribbons/dots get wrong/no ring (B5); pin mask pipeline samples across the flip |
| 70 | scene-tree | **REWRITE** | single-doc; **`node.guid()` vs `node.name`** (object guid IS `TreeNode.name`) — as written every row misses; `find_node_by_guid` is the wrong lookup (use `get_node_by_name`); multi-doc gives the panel its top level for free |
| 71 | tree-viewport | TOUCH-UP | reveal must find the owning doc; ten roots |
| 72 | text-labels | TOUCH-UP | label anchors are anchor-relative (world_pos rebased!) — labels jump on re-anchor; text pipeline needs `samples`; "only texture" claim vs 85 |
| 73 | control-points | **REWRITE** (Step 2) | `session.objects.…iter_mut()` + Rc (no DerefMut) → `Rc::make_mut`; CV drag must happen in local frame (inverse placement) |
| 74 | edit-points | **REWRITE** (same root) | ditto + `duplicate()` mints new guid (cache keying note); borrow-order fix |
| 75 | work-plane | **REWRITE** (Step 2) | `m.xform.transform_vector`; box placement via `set_xform`/`transform(&xf)`; `rebase_anchor` is 2-arg |
| 76 | advanced-perf | OK | baseline should name the manifest scene; INK_DEPTH_PREPASS is the one dormant lever |
| 77 | capstone | TOUCH-UP | `DEMO_SESSION_URL` gone (manifest); save policy per doc; phase map references single-session reconcile |
| 78 | section-planes | TOUCH-UP | `rebase_anchor` 2-arg; rename `origin`→`anchor` param; best-aged of the batch |
| 79 | import-export | **REWRITE** | `session_from_bytes` ×4 (async chunked now); `.obj` enters via manifest item or `Msg`; export must pick a doc; `Option<&Mesh>` vs Rc; importer topology: `session_rust/src/pdf.rs` + `file_obj.rs` + `io.rs` (session_io deleted) |
| 80 | copy-array | **REWRITE** (Step 1) | `geom.clone()` clones the Rc → copy IS the original; use `duplicate()` (already re-mints guid — "identity trap" section half-obsolete); placement copies = `set_xform` per new guid; target doc |
| 81 | layers | **REWRITE** | layers already EXIST per doc (one group per OCG layer × 10 sheets, names recur) — `active_layer: Option<String>` can't address that; open by LISTING real layers instead of creating them |
| 82 | measure | TOUCH-UP | `scene.session.lookup[g]` (also indexing panic — use `.get`); `what` can report `world_xform` cheaply; say which sheet the cursor is over |
| 83 | dev-toolbox | **REWRITE** (Step 1) | `Scene::new(session)` takes nothing; `.cargo/config.toml` pins wasm target — native selftest needs explicit `--target`; needs `crate-type += rlib` + `pub mod app` |
| 84 | web-polish | TOUCH-UP | sizes stale (36-132 MB per sheet, ~0.5 GB); "frozen tab" framing predates pipelined fetch + chunked parse; progress must be per manifest item and must not break the fetch window |
| 85 | textures | TOUCH-UP | paths `gpu.rs`→`gpu/mod.rs`; `Pipelines::new` missing `samples` arg; triplanar keyed on anchor-relative `world_pos` → pattern slides on re-anchor; state the mm unit |

## The five most dangerous traps (would ship silently broken)

1. **B3 set_scene wipe** — selection/hidden/cull/gumball state vanishing whenever a file streams
   in. Decide once: either all flags live in `scene.tables.objects[row].2` (engine re-derives),
   or `set_scene` gains a "re-apply viewer state" hook. Then 37/45/46/52/54/58 all follow.
2. **B4 place conjugation** — picking/snapping/deltas off by the manifest offset on every placed
   sheet; rotation/scale visibly wrong. One helper (`Scene::world_frame(guid) -> Xform` = place ×
   world_xform, plus its inverse) removes the class.
3. **74's `node.guid()` vs `node.name`** — the tree panel compiles and renders but every
   row-to-object association silently misses.
4. **42b's move-blind hash** — after the Xform refactor a moved object hashes identical, so
   reload/save-if-changed miss the most common edit. Hash must fold `session.xform(guid)`
   (cheapest: hash `to_proto()` bytes + the xform).
5. **B8 dynamic MSAA** — any post-35 pipeline built once at startup panics or mis-renders when
   the sample count flips on the first solid file (or on pure-2D scenes at 1×).

## What got EASIER (use these when rewriting)

- Gumball/transforms: `session.set_xform(guid, &delta * &session.xform(guid))` — one line, no
  per-variant match, no re-tessellation; undo snapshots become `(guid, Xform)` pairs.
- 65's whole thesis is now true by construction (shape cache can't see transforms).
- Diff/hash: pub `to_proto()` bytes are a uniform fingerprint; `Session::order()` (P3, done)
  gives deterministic buckets; P2 (done) makes `pb_dumps` on a mutated session safe.
- `FLAG_HIDDEN`, `guid_to_row`, `hidden` already exist (46/45 shrink).
- 81 can demo on REAL layers (the PDF importer already builds one group per OCG layer).
- Multi-doc gives 74's tree panel its natural top level (one row per `Doc`).

## Stale reference docs

- `_LESSON_AUDIT.md` (2026-07-20): header scope wrong (74 is a full lesson now); ~12 listed bugs
  already fixed in the current lesson texts; 65's suggested fix uses the deleted `.xform`; the
  three biggest CURRENT issues (Rc mutation, `Scene::session` removal, anchor-relative
  world_pos) are absent. Supersede with this file or fold in.
- `_KERNEL_GAPS.md`: #3 "mesh.xform is the placement, everywhere" is now false and misleading —
  restate as "placement moved off geometry onto Session"; #10 mentions Point.xform (gone);
  #13 should note `duplicate()` already re-mints guids and record the NEW gap: `Rc<T>` in
  `Geometry`/`Objects` makes every mutation path go through `Rc::make_mut` (subsumes #9).
  `.claude/FILE_IO_API_PLAN.md`'s "blocked on the xform refactor" note is stale — it landed.

## Suggested order of work

1. Decide B3 (viewer-state survival across `set_scene`) and B4 (place-conjugation helper) — they
   are design decisions, not typos, and 20+ lessons depend on the answer.
2. Rewrite the load-bearing four: 36 (BVH feeds 37/42/45), 38a/38b (or explicitly de-scope them
   to "tables rebuild + set_scene" until incremental updates are needed), 54 (its `apply_delta`
   is quoted by 55/56/57/61/63/80).
3. Touch-ups in numeric order; they are mostly one-to-three-line anchor fixes once B1-B8 are
   settled vocabulary.
4. Refresh `_KERNEL_GAPS.md` #3/#10/#13 and retire `_LESSON_AUDIT.md`.
