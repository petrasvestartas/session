# What this viewer does not do yet

It reads, draws, picks and streams; it does not create, move, delete or edit. `session_viewer_archive` (~11,000 lines of `src/`) did all four, in a different architecture.

## Three rules every design below obeys

- **A widget never touches the session.** Events reach `src/app/input.rs`, which calls a named action on `State` (`escape_selection`, `enable_controls`, `hide_selected`, `fit_selected_or_all`, `set_cloud_size`, `request_selection`). New work is one more named action, never a reach into `self.scene.docs[..].session`.
- **Undo lives in the kernel.** `session_rust/src/history.rs`: transactions, tombstones, cursor; `Session::{begin, commit, undo, redo}`. A viewer stack is a second cursor that can disagree, and a save purges only one.
- **Pixels come out of lanes.** `Gpu` lists them by hand in `src/engine/gpu/mod.rs`: `backdrop, arena, segments, glyphs, controls, control_net, text, selection_outline, solid_outline, cloud, splat, pick`. New geometry reuses one or becomes one more.

## The four guides

- **Gumball** — grabbed and dragged: handles, drag, hit test, commit.
- **Command line** — typed: text surface, parser, verb table, prompt.
- **Tree** — a panel of rows kept in step with `Scene` identity.
- **This page** — snapping, construction plane, write-back address, refit math, commit path.

## The gap list

### 1 · No text entry anywhere on the page
- `index.html` has only `#viewer-status` (textContent, from `app/feedback.rs`) and `#viewer-error`; no input, no egui, `tabindex` only for key events.
- First: getpoint, typed coordinates, delete, naming and the prompt all need it. Depends on nothing.
- `<input id="viewer-command">` → existing `Msg` channel → `State::command(&str)`: under 150 lines, no new dependency, against the archive's egui box in `state_ui.rs` (843 lines). (command line)
- **Bites.** A DOM input steals canvas focus and `1`–`7`, `F`, `Q` die; the archive re-focuses its box every frame while a tool is active (`state_ui.rs:326-333`). Rule first: focused field owns keys, Escape returns focus.

### 2 · No screen ray
- `Camera` (`src/camera.rs`) cannot say which world ray goes through a pixel; `zoom_at` unprojects onto the target plane only, inline. Drawing tools, the plane and CPU snapping wait on it. Depends on nothing. (this page)

```rust
// sketch, src/camera.rs
pub fn ray(&self, cursor: (f64, f64), viewport: (f64, f64)) -> (Point, Vector);
```

- Under 40 lines plus a test: `zoom_at` has the frustum half-extents, `right` and `up`; perspective is eye → offset point, orthographic is offset point → forward.
- **Bites.** f64, not `[f32;3]`: an f32 ray a kilometre out has millimetre error before it hits anything — what the anchor exists to avoid (`gpu/objects.rs`, `the_anchor_is_what_keeps_a_small_move`).

### 3 · No construction plane, no typed coordinates
- Archive `cad_plane.rs` (42 lines): forward is the view matrix's third row, XY/YZ/XZ by largest component, perspective falls back to XY, plane through the world origin. `coord_parser.rs` (44 lines): `x,y,z` / `@dx,dy` / `@dist<ang`.
- Needs (2), and (1) for the typed half; ~86 lines again. The forward-axis rule works against this camera unchanged (`set_view(View::Front..Iso)`, `toggle_projection_framed`). (command line for the parser, this page for the plane)
- **Bites.** f64, not the archive's f32 (`coord_parser.rs:8-13`): a typed coordinate is the one input expected to be exact, and every kernel `Point` is f64.
- **Bites.** "Through the world origin" is a placeholder; a plane far from the origin is the anchor case — keep its origin f64.

### 4 · No object snapping
- Endpoints, midpoints, vertices, intersections, knots, nearest-point-on-edge in an aperture, ranked by kind then pixel distance, with a marker glyph.
- Archive `snap.rs` (401 lines) + ranking in `state_tool.rs:258-328`: one session walk per tool session, edges sampled 16 per edge, best within `SNAP_APERTURE_PX = 12.0`.
- **Read the call graph, not the plan.** `CAD_SKETCHER_PLAN.md` says intersection and knot snapping are done; `pub fn snap(...)` (`snap.rs:185-401`, ~216 lines) has no call site. The live cache holds Vertex, Endpoint, Midpoint; `SnapModes::default_on()` enables two modes that never fire.
- Needs (2), (3) for the fallback point, a lane for the marker. Emission ports nearly as written — sources are retained (`FileDoc.session: Rc<Session>`, `src/app/scene.rs:26-37`); the sweep does not. 450 lines, a test per kind. (this page)

```rust
// sketch — a candidate names its owner the way the rest of this viewer does
enum SnapKind { End, Vertex, Int, Mid, Knot, Center, Near, Plane }  // priority 0..6, lower wins
struct SnapHit { point: Point, kind: SnapKind, owner: (usize, Rc<str>) }
struct SnapCache {
    points: Vec<(Point, SnapKind, (usize, Rc<str>))>,
    edges: Vec<(u32, Vec<Point>)>,   // row + polyline, for Near and for a move ghost
    built_for: u64,                  // the geometry revision it was built from
}
```

- **CPU, and the split is the rule.** A candidate drawn as a lane (handle, edge, face) has an id: `PickMode::Controls`/`PickMode::Edge` answers it. One never drawn (midpoint, intersection, point along an edge) has none — `gpu/pick.rs` cannot return a pixel never rendered — so it is CPU work against the cache.
- **Bites.** A streamed cloud's session is an empty shell (`display_only`, `scene.rs:33-37`): a `session.lookup` sweep silently reports no snaps on a 40M-point cloud. A sheet is one row for tens of thousands of segments (`SheetBatch`, `scene.rs:80-93`): the sweep emits a candidate per segment. Ask what a row is — `Scene::sheet_slot`, `Scene::streamed_slot`, `app/cloud_query.rs`.
- **Bites.** `Session::world_xforms()`, one downward pass; per-object `world_xform` rescans the tree and is quadratic (`snap.rs:94-96`).
- **Bites.** Invalidate by revision, not a bool, as `geometry_revision` and `selection_revision` do.

### 5 · Nothing can be created
- Click or typed coordinate adds a point, Enter finishes, `c` closes, `u` drops the last, Esc cancels, rubber band follows. Needs (1)–(4) plus a transient segment-lane region.
- ~400 lines against 564 (`tool_state.rs` 77, `state_tool.rs` 487): the preview is a lane write, the commit four kernel calls rather than four plus GPU add plus undo push plus label rebuild. (command line for verbs, this page for the loop)

```rust
// sketch — the state, owned by State, driven only by named actions
enum Tool { Idle, Point, Line, Polyline, Curve { degree: usize }, Move }

struct Draft {
    tool: Tool,
    points: Vec<Point>,        // f64 kernel points, never f32
    cursor: Option<SnapHit>,   // resolved once per pointer move
    dirty: bool,               // preview rebuilt in render(), not per event
}

impl State {
    pub fn tool_begin(&mut self, tool: Tool);
    pub fn tool_hover(&mut self, x: f64, y: f64);
    pub fn tool_point(&mut self, p: Point);
    pub fn tool_text(&mut self, line: &str);
    pub fn tool_finish(&mut self);
    pub fn tool_cancel(&mut self);
}
```

```rust
// sketch — the only shape an edit may take in this viewer
let doc = &mut self.scene.docs[owner];
let session = Rc::make_mut(&mut doc.session);   // FIRST: other placements share this Rc
session.begin("Polyline");                      // no begin -> History::record is a silent no-op
session.add_polyline(polyline, None);
session.commit();                               // no commit -> nothing on the undo stack
self.scene.rebuild(&mut self.gpu);              // rows, ids, bounds, masks, hidden flags follow
self.touch();
```

- **Bites.** Without `begin`, `History::record` returns at once (`history.rs:249-255`) and `replace`/`set_xform`/`_add_object` snapshot only when `history.current.is_some()`: the edit works and is silently unundoable.
- **Bites.** `Rc::make_mut` first — a manifest listing one file twice hands both documents the same `Rc` (`scene.rs:29-35`).
- **Bites.** `State::render` never requests the next frame: a rubber band that does not move is a missing `touch()`.
- **Bites.** `Scene::rebuild`'s only call site is `src/selftest/lifecycle.rs:161`, asserting a rebuilt scene renders pixel-identical to the incrementally loaded one. That test is what the commit path stands on.
- **Bites.** `rebuild` cannot restore a streamed cloud or sheet — no kernel object to walk — so a commit in such a scene must not take that path.

### 6 · Nothing can be moved
- Base point, target point, translucent ghost while the originals stay put and snappable. Needs (5) plus a decision about which table the move writes.
- Archive (`commit_move`, `state_tool.rs:58-74`; `commit_object_transform`, `state_interaction.rs:40-143`): Mesh, Point, Line, Polyline, Plane, PointCloud, OBB **bake the matrix into coordinates**; BRep, NurbsSurface, NurbsSurfaceTrimmed are **matrix-only**, because re-tessellating a trimmed surface to move it is absurd. The question ports; the answer assumed one 4×4 per object per mouse-move.
- ~150 lines against ~250: bake is `Geometry::transform` through `Rc::make_mut` plus `Scene::rebuild`; pose is one `Session::set_xform`. (gumball for the drag, this page for the commit)
- **Bites, the important one.** `Instance` is 96 bytes with a zero translation column; the anchored translation is a separate 16-byte row (`instance.rs:6-22`, `objects.rs`). `rebase_anchor` rebuilds that whole buffer when the target drifts past `(view_dist * 0.25).clamp(1.0e3, 1.0e5)` world units, throttled to 200 ms, so a translation written only into anchored f32 is erased (`an_edit_written_past_the_base_does_not_survive_a_rebase`).
- **Rule.** A drag may write anchored values for feedback; a commit writes the f64 source — geometry or `set_xform` — and the rebuild refills both tables.
- **Bites.** Moving a sheet row moves ninety thousand segments. Refuse it with a status line or mean it.

### 7 · No delete
- Needs (1) or a key plus the commit path (5); ~40 lines. (command line)
- `Session::remove_object` already records a tombstone capturing the collection, the index in it, the local xform, the parent guid, the child index, the detached subtree node, the graph attribute and every incident edge. No viewer path calls it.
- **Bites.** The archive bakes a NurbsSurface's live GPU model into the stored clone before removing it, because the surface moved matrix-only (`state_cmd.rs:211-220`). That should not arise if `set_xform` is the pose, since the tombstone records the xform — confirm before relying on it.

### 8 · No undo or redo
- Ctrl+Z / Ctrl+Y over the kernel cursor plus a status line naming the transaction. Needs one edit to exist; ~60 lines of plumbing. (this page)
- **Replaced, not ported.** `undo_state.rs` (107) + `state_undo.rs` (196) are two capped `Vec<UndoAction>`s and two ~90-line match arms of hand-written table surgery; `Tombstone` captures what they reconstruct. `ARCHITECTURE.md` §7: that enum "won't scale to dozens of CAD tools".
- **The lesson kept.** Archive `Transform`/`EditGeom` carry absolute snapshots on both sides — redo once re-baked the delta, and after a multi-object drag the delta accumulated and objects jumped. `ReplaceOp`/`XformOp` are already that shape.
- **Bites.** Undo is per document; each `Session` owns its `History`. The selection's document is the defensible answer.
- **Bites.** Every save path clears the history (`session.rs:582, 591, 607, 875`); a reload is not a place undo survives.

### 9 · No multi-selection, no box select
- Depends on nothing, but everything after changes shape if it lands late. Archive `state_pick.rs` is 401 lines; here small and wide. (tree, which needs the same set)

```rust
// sketch, src/app/scene.rs
pub selected: Vec<u32>,   // was Option<u32>
```

- **Touches.** `State::select`, `hide_selected`, `fit_selected_or_all`, `enable_controls`, `apply_pick`, `update_label`, `Gpu::set_selected`, `selection_outline.set_selected`, the name annotations in `src/state/text.rs`.
- **Bites.** `Gpu::selection_revision` is in the coverage mask key (`MaskKey.selection`, `surface_outline.rs:30-40`): bump once per gesture, not per row.
- **Not the archive's box test.** `process_box_select` projects one centroid per object (`state_pick.rs:37-90`), so a long polyline crossing the box is missed. The picker reads ids in a window (`gpu/pick.rs`) and `Window::with_radius` goes to `MAX_RADIUS` 128: read the rectangle's ids, deduplicate rows, done.

### 10 · No sub-object editing that writes back
- Needs a drag (gumball) and the commit path (5).
- **Read half exists** (lesson 13): `ControlId::{Vertex, Curve{curve,point}, Surface{surface,u,v}, Point}`, `Controls::from_geometry` over Mesh, Line, Polyline, NurbsCurve, NurbsSurface, BRep, Element, PointCloud, Point (`src/app/selection.rs:8-13, 101-221`), the `controls` and `control_net` lanes, `PickMode::Controls { parent, cloud }`, `enable_controls`, `apply_control`.

```rust
// sketch — the write direction of the address app/selection.rs already reads
fn write_control(geometry: &mut Geometry, id: ControlId, to: Point) -> bool;
```

- 300 lines for `write_control`, its arms and tests, plus the drag — against `edit_state.rs` + `state_edit.rs`, 2,282 lines, 21% of the archive's `src/`.
- **Do not port `EditState`'s GPU half** (own `node_buf`/`edge_buf`, bind groups, doubling growth: `edit_state.rs:292-313, 426-460`): the `controls`/`control_net` lanes are that already, so porting adds a second overlay and a second address type meaning `ControlId`.
- **Bites.** `VertexData::set_position` bypasses the mutators that drop the per-mesh triangle BVH: call `Mesh::invalidate_triangle_bvh()` or picking keeps hitting the pre-edit shape (`state_edit.rs:1375-1379`).
- **Bites.** A BRep's 3D edges must be re-derived from the 2D trims after surfaces move (`recompute_brep_edges`, `state_edit.rs:1625`). That walk is `src/app/walk/brep_edges.rs`, run from `Scene::rebuild`: commit through the rebuild and it is free; an in-place update loses it.
- **Bites.** f64 through the drag, narrowed once at upload — `render_position` (`src/state.rs`) is that boundary for reads, `f32p` was the archive's for writes.

### 11 · No Greville edit points
- Handles *on* a curve or surface instead of in the control cage; dragging one refits the control points so the curve passes through the drag. Needs (10).
- Archive `edit_points.rs`, 157 lines of which ~65 are its two tests: Greville abscissae ξᵢ = mean of *p* = order−1 consecutive knots; Eᵢ = C(ξᵢ); square rational collocation matrix Rⱼᵢ = wᵢNᵢ(ξⱼ)/Σwₖ Nₖ(ξⱼ); column *k* of R⁻¹ solved once at drag start against a unit vector, so each move is ΔPᵢ = col[i]·Δ — weights preserved, a circle stays a circle.
- **The one file that ports unchanged**: it imports `session_rust::{nurbsknot, Matrix, NurbsCurve, Point}` and nothing else. Its second test asserts to 1e-7 that dragging edit point *k* by Δ moves it by exactly Δ. (this page)

```rust
// sketch, src/app/selection.rs — the only additions ControlId needs
enum ControlId {
    Vertex(usize), Curve { curve: usize, point: usize },
    Surface { surface: usize, u: usize, v: usize }, Point(u32),
    CurveEdit { curve: usize, index: usize },          // new
    SurfaceEdit { surface: usize, index: usize },      // new
}
```

- **Bites.** `Matrix::solve` returning `None` is the degenerate-curve guard: refuse the drag, do not unwrap.

### 12 · No live deform during a drag
- Needs (10) and an in-place range rewrite meshes lack; archive ~350 lines plus the lane work. (gumball)
- **The premise ports.** A NURBS point is linear in its control points: freeze the tessellation at drag start, precompute each tessellation vertex's influence weight per moved control point, and every move is a multiply-add — no `point_at`, no `normal_at`, no re-tessellation. The adaptive rebuild runs once, on release.
- **The lane gap.** `Buffer::write_at` (`buffers.rs:91`) plus `Scene::ribbon_range(row)` means strokes can be rewritten in place today. Faces cannot: `ArenaLane`'s `verts` is private (`arena.rs`), `Scene` keeps no per-row vertex range. Adding one is a real lane change — `Vec<Option<Range<u32>>>` beside `ribbon_ranges`, filled in `add_file`, plus `ArenaLane::write_verts`.
- **Bites.** An in-place vertex write does not bump `geometry_revision`, the key for the triangle tile index (`ProjectionKey { matrix, objects }`, `triangle_tiles.rs:150-153`) and the silhouette masks (`MaskKey.geometry`): visibility then culls against the pre-drag projection while the picture shows the deformed one. Bump per drag frame, paying a re-projection — the correct default — or prove the cheaper thing in a comment.
- **Bites.** `Instance::FLAG_SMOOTH` says a row is a tessellation; a deformed tessellation still is one.

### 13 · No edge you can drag
- Ctrl+Shift+click an edge and move it, the highlight following the real curve, not its chorded control polygon. Needs (10). (gumball)
- **Half exists**: `PickMode::Edge`, `Scene::edge_at(pick)`, `SegRows.pipe_ids`, `SegmentLane::set_edge` (`src/state.rs`, `segments.rs:343`); every subdivision keeps its source edge id. The archive's 140 lines of edge selection are replaced; only the drag and write remain.
- **Ports as a rule.** The drag highlight is the frozen base polyline mapped by the delta: a whole-edge transform is affine, so nothing is re-evaluated per move.

### 14 · No BRep hole → collar deform
- Pull a hole rim: the rim follows, the wall swells smoothly, the outer boundary holds, the trim is untouched. Archive ~300 lines — degree-elevate the face, inner radius from the rim, outer radius from the nearest outer-boundary control point, each control point moved by `g·(delta·P − P)` with `g` a smoothstep from 1 inside to 0 outside. Needs (10) and (12).
- **A shape, not code:** the viewer supplies a delta and two radii, the kernel does the geometry — which is why this does not belong in the viewer at all.
- **Bites.** The archive's write-back skips these faces: the flat overlay nodes are stale after a collar and writing them would undo it (`state_edit.rs:1387-1390`). Any deform moving control points the overlay does not know about needs the same exclusion.

### 15 · No selection filters
- Not a CPU test after the pick: `Picker::configure(mode, radius_css, scale)` decides which lanes draw ids, so the filter is a mask carried into that decision and a filtered-out lane never writes an id.

```rust
// sketch, src/engine/gpu/pick.rs
struct Filter(u32);   // one bit per lane: faces, strokes, markers, clouds, text, sheets
enum PickMode { Object { filter: Filter }, Edge, Component, Controls { parent: u32, cloud: bool } }
```

- Depends on nothing; under 100 lines (mask, per-lane draw guards in `gpu/pick.rs`, a setter). Cheapest item, and the one that makes the others usable. (tree)
- **Bites.** The archive filters by walking `session.lookup`, because its pick is a CPU raycast. Here that means picking twice, and it still picks wrong when the filtered-out object is in front, because the id pass already resolved depth against it.

### 16 · No tessellation wireframe toggle
- **Keep the decision:** 8 iso-curves per direction, adaptively polylined, not every triangle edge — ~25× fewer line vertices for the same answer. **Not the implementation:** it frees and reallocates every `"__tess__"`-prefixed arena slot per toggle and hand-allocates a tint instance to force black lines.
- `src/app/walk/mesh_ink.rs` decides seams at walk time, `VIEWER_SEAMS` (`src/app/knobs.rs`) is the launch-time switch; the toggle is a `View` field plus a walk variant, shaped like `Q`, `W`, `E`, `O`. Under 100 lines. (this page)

### 17 · No auto-naming, no prompt line
- `polyline_7`; `Polyline: next point (3) — Enter=finish, c=close, u=undo, Esc=cancel`.
- `app/feedback::status` writes one line into `#viewer-status` as `textContent`, never HTML. A prompt is a status line; the archive's log of the last 200 prompts is a panel, and belongs to the tree guide if wanted. Under 30 lines. (command line)

### 18 · No in-app geometry construction, and it stays that way
- **Not a gap.** Scenes come from manifests and `.pb` files (`src/app/manifest.rs`, `loader.rs`, `route.rs`, `live.rs`), validated before kernel constructors allocate from serialized counts (`src/app/validate.rs`). `ARCHITECTURE.md` §8 lists the archive's `demo.rs` (860 lines) as a structural defect — "app data, not engine", `State::new` hardcoding `demo::active_scene()`.
- **Worth having:** primitive *commands* (`box`, `sphere`) constructing through `Session::add_*` inside a transaction — item 5 with different verbs. (command line)

## What we take from the old viewer and what we do not

**Ports** — `edit_points.rs` (157 lines), kernel-only and tested; `coord_parser.rs` (44 lines) at f64; `SnapKind`, its priority ladder and `SnapModes` (priority class first, pixel distance breaks ties); absolute snapshots on both sides of a recorded edit, and the accumulating-delta bug behind them; the live-deform premise (freeze, precompute weights, multiply-add); f64 through, f32 at the boundary, now in the write direction too; iso-curves, not triangle edges.

**Adapts** — getpoint loop → a `Draft` driven by named actions with `touch()` at every mutation; snap emission ports but the `session.lookup` sweep does not (ask what a row is); snap ranking stays CPU only for candidates no lane draws; invalidation → a revision counter; construction plane → forward-axis rule kept, world-origin placeholder replaced; `NodeAddr` → `ControlId` plus two Greville arms and the write direction; the two write hazards (mesh BVH, BRep 3D edges) as rules; bake-vs-matrix → the question, re-answered against the two-table transform; multi-selection yes, the centroid box test → a rectangle id read; the transient preview → an owned region of the segment and glyph lanes; `log_prompt` → `feedback::status`.

**Replaced** — `undo_state.rs` + `state_undo.rs` (303 lines), because the kernel history is the cursor; `EditState`'s GPU half, because the `controls`/`control_net` lanes are it; `build_overlay_data`, because `Controls::from_geometry` covers more types and is tested; `pick.rs` CPU raycasting and `closest_object_under_ray`, because selection is GPU ids with a halo, a generation and a mode — the kernel's `ray_cast` has never been called here and should not start; `selected_centroid` and its per-type arms, because `InstanceTable::row_bounds(row)` gives the box and a widget origin is its centre; `text.rs` (256 lines, an 8×16 atlas as screen-anchored quads), replaced by text objects with scene identity; `demo.rs` (860 lines), replaced by manifests, loader, routes and live reload; `rebuild_tess_wireframe` as written; `selftest.rs` (164 lines), replaced by the headless harness; Escape falling through to `event_loop.exit()`, since Escape here is `escape_selection` with "cancel the active tool" in front of it.

- Also replaced: the `impl State`-at-crate-root arrangement (`state_tool.rs`, `state_edit.rs`, `state_undo.rs`, `state_interaction.rs`, `state_pick.rs`, `state_cmd.rs`, `state_ui.rs`, `state_render.rs`, `state_update.rs`). `commit_object_transform` alone reads and writes the session lookup, the xform table, four GPU pick-mesh caches, instance flags, colour overrides, the thickness uniform and the text labels. That coupling is why the gap list cannot be closed by copying files.

## The order to build it in

Each step compiles; each names what it must not break.

1. **Filters** (15) — picking: mask full is byte-identical, the headless harness proves it.
2. **Screen ray** (2) — nothing; a pure function with a test.
3. **Plane and coordinate parser** (3) — nothing; pure functions over the camera and a string.
4. **Text entry** (1) — the keyboard: `1`–`7`, `F`, `Q`, `W`, `E`, `O`, `P`, `D`, `B`, `H`, `S`, `T`, `[`, `]`, F10, Escape must still reach `app/input.rs` when the field is unfocused.
5. **Commit path**, one hardcoded verb (`point 0,0,0`) — history (`begin`/`commit`), source identity (`Rc::make_mut` first), clouds and sheets (a rebuild cannot restore them).
6. **Undo and redo** (8) — history: one cursor, chosen by the selection; clouds and sheets as in 5.
7. **Delete** (7) — source identity (`hidden`/`guid_to_row` keyed on `(document, guid)`), silhouettes (masks key on `geometry_revision`).
8. **Snapping** (4), marker in `glyphs` — clouds (ranged query, never the empty shell), sheets (one row is not one object), the tile index (a glyph is not a triangle).
9. **Getpoint and the four tools** (5), rubber band in a transient `segments` region — finite-triangle visibility (the preview is strokes), text identity (`register_text` on rebuild), one frame per demand (`touch()`).
10. **Multi-selection and rectangle pick** (9) — silhouettes (`selection_revision` once per gesture), sheets (a sheet row selects the sheet).
11. **Move** (6) — the transform split (commit writes the f64 source), the tile index and silhouettes, sheets (decide what moving ninety thousand segments means).
12. **Control write-back** (10) on Mesh, Polyline, NurbsCurve — picking (`invalidate_triangle_bvh()`), source identity (a rebuilt object keeps guid, row, hidden flag, name).
13. **NurbsSurface and BRep write-back** — the CAD contract: boundaries, trims and provenance are re-derived by `app/walk/brep_edges.rs` on rebuild, so commit through the rebuild.
14. **Greville edit points** (11) — nothing; the two `ControlId` arms are additive.
15. **Live deform** (12) plus the per-row vertex range — the tile index and silhouettes (bump `geometry_revision`), `FLAG_SMOOTH`.
16. **Edge drag** (13), **wireframe toggle** (16), **naming and prompts** (17) — the keyboard map, `feedback::status` staying `textContent`.
17. **BRep collar** (14), if wanted at all, as a kernel operation the viewer parameterizes.

## What to check on screen

- **1.** Strokes-only filter: a face click selects nothing, an edge click selects. Full filter: screenshot matches the old one, pixel for pixel.
- **2–3.** With `?inspect=1`, a click logs a world point that does not move when you orbit and click the same spot on the same object.
- **4.** Type in the field, press `F`: nothing fits. Escape, `F`: it fits.
- **5.** `point 0,0,0` marks the origin; reload and it is gone, because nothing was saved.
- **6.** Draw, undo, redo, undo — the status line names the transaction each time.
- **7.** Delete, undo: the object returns in the same tree position with the same name, not appended.
- **8.** Endpoint reads "End", mid-edge "Mid"; a streamed cloud gives no snap, no freeze, no error.
- **9.** Five-point polyline from three clicks and two typed coordinates; rubber band at full frame rate; `u` drops, `c` closes, Esc leaves nothing.
- **10.** A rectangle across a long polyline whose middle is outside the box: selected.
- **11.** Move an object a kilometre out by one millimetre, orbit until it re-anchors: the move survives.
- **12–13.** F10, drag, release: the object deforms, the outline follows, a click picks the deformed surface.
- **14.** Drag an edit point on a circle: it follows exactly, and the circle is still a circle.
- **15.** Drag a surface control point across another object's silhouette: the two stay correctly ordered throughout the drag, not only after release.
