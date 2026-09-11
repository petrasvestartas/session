# What this viewer does not do yet

It reads, draws, picks, streams, and since lesson 21 moves, deletes and edits. It does not CREATE. `session_viewer_archive` (~11,000 lines of `src/`) created too, in a different architecture.

## Three rules every design below obeys

- **A widget never touches the session.** Events reach `src/app/input.rs`, which calls a named action on `State` (`escape_selection`, `enable_controls`, `hide_selected`, `fit_selected_or_all`, `set_cloud_size`, `request_selection`). New work is one more named action, never a reach into `self.scene.docs[..].session`.
- **Undo lives in the kernel.** `session_rust/src/history.rs`: transactions, tombstones, cursor; `Session::{begin, commit, undo, redo}`. A viewer stack is a second cursor that can disagree, and a save purges only one.
- **Pixels come out of lanes.** `Gpu` lists them by hand in `src/engine/gpu/mod.rs`: `backdrop, arena, segments, glyphs, controls, control_net, text, selection_outline, solid_outline, cloud, splat, pick`. New geometry reuses one or becomes one more.

## The four guides

- **Gumball**, **Command line** and **Tree** were the designs; [lesson 21](21-editing.md) is the
  code they became. They are kept as the reasoning behind it, not as instructions to follow.
- **This page** is what is still missing.

## Built, in lesson 21

Nine of the gaps this page listed are closed. The code is the description now; a second one
here would drift from it.

| was | now |
| --- | --- |
| no screen ray | `Camera::ray`, f64 |
| no construction plane, no typed coordinates | `app/cplane.rs`, `app/coords.rs` |
| no object snapping | `app/snap.rs`, ranked in screen space |
| nothing can be moved | the gumball, and `move` on the command line |
| no delete | `Delete`, and `delete` |
| no undo or redo | the kernel's history, per document |
| no sub-object editing that writes back | a control drag, through `Session::replace` |
| no text entry anywhere on the page | the command line, opened with `:` |
| no panel of rows beside the scene | the layers panel, opened with `L` |

### 5 · Nothing can be created
- Click or typed coordinate adds a point, Enter finishes, `c` closes, `u` drops the last, Esc cancels, rubber band follows. Needs (1)–(4) plus a transient segment-lane region.
- ~400 lines against 564 (`tool_state.rs` 77, `state_tool.rs` 487): the preview is a lane write, the commit four kernel calls rather than four plus GPU add plus undo push plus label rebuild. (command line for verbs, this page for the loop)

```rust
// sketch — the state, owned by State, driven only by named actions
enum Tool { Idle, Point, Line, Polyline, Curve { degree: usize }, Move }

struct Draft {
 tool: Tool,
 points: Vec<Point>, // f64 kernel points, never f32
 cursor: Option<SnapHit>, // resolved once per pointer move
 dirty: bool, // preview rebuilt in render, not per event
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
let session = Rc::make_mut(&mut doc.session); // FIRST: other placements share this Rc
session.begin("Polyline"); // no begin -> History::record is a silent no-op
session.add_polyline(polyline, None);
session.commit; // no commit -> nothing on the undo stack
self.scene.rebuild(&mut self.gpu); // rows, ids, bounds, masks, hidden flags follow
self.touch;
```

- **Bites.** Without `begin`, `History::record` returns at once (`history.rs:249-255`) and `replace`/`set_xform`/`_add_object` snapshot only when `history.current.is_some`: the edit works and is silently unundoable.
- **Bites.** `Rc::make_mut` first — a manifest listing one file twice hands both documents the same `Rc` (`scene.rs`).
- **Bites.** `State::render` never requests the next frame: a rubber band that does not move is a missing `touch`.
- **Bites.** `Scene::rebuild`'s only call site is `src/selftest/lifecycle.rs`, asserting a rebuilt scene renders pixel-identical to the incrementally loaded one. The commit path stands on that test.
- **Bites.** `rebuild` cannot restore a streamed cloud or sheet — no kernel object to walk — so a commit in such a scene must not take that path.

### 9 · No multi-selection, no box select
- Depends on nothing, but everything after changes shape if it lands late. Archive `state_pick.rs` is 401 lines; here small and wide. (tree, which needs the same set)

```rust
// sketch, src/app/scene.rs
pub selected: Vec<u32>, // was Option<u32>
```

- **Touches.** `State::select`, `hide_selected`, `fit_selected_or_all`, `enable_controls`, `apply_pick`, `update_label`, `Gpu::set_selected`, `selection_outline.set_selected`, the name annotations in `src/state/text.rs`.
- **Bites.** `Gpu::selection_revision` is in the coverage mask key (`MaskKey.selection`, `surface_outline.rs:30-40`): bump once per gesture, not per row.
- **Not the archive's box test.** `process_box_select` projects one centroid per object (`state_pick.rs:37-90`), so a long polyline crossing the box is missed. The picker reads ids in a window (`gpu/pick.rs`) and `Window::with_radius` goes to `MAX_RADIUS` 128: read the rectangle's ids, deduplicate rows, done.

### 11 · No Greville edit points
- Handles *on* a curve or surface instead of in the control cage; dragging one refits the control points so the curve passes through the drag. Needs (10).
- Archive `edit_points.rs`, 157 lines of which ~65 are its two tests: Greville abscissae ξᵢ = mean of *p* = order−1 consecutive knots; Eᵢ = C(ξᵢ); square rational collocation matrix Rⱼᵢ = wᵢNᵢ(ξⱼ)/Σwₖ Nₖ(ξⱼ); column *k* of R⁻¹ solved once at drag start against a unit vector, so each move is ΔPᵢ = col[i]·Δ — weights preserved, a circle stays a circle.
- **The one file that ports unchanged**: it imports `session_rust::{nurbsknot, Matrix, NurbsCurve, Point}` and nothing else. Its second test asserts to 1e-7 that dragging edit point *k* by Δ moves it by exactly Δ. (this page)

```rust
// sketch, src/app/selection.rs — the only additions ControlId needs
enum ControlId {
 Vertex(usize), Curve { curve: usize, point: usize },
 Surface { surface: usize, u: usize, v: usize }, Point(u32),
 CurveEdit { curve: usize, index: usize }, // new
 SurfaceEdit { surface: usize, index: usize }, // new
}
```

- **Bites.** `Matrix::solve` returning `None` is the degenerate-curve guard: refuse the drag, do not unwrap.

### 12 · No live deform during a drag
- Needs (10) and an in-place range rewrite meshes lack; archive ~350 lines plus the lane work. (gumball)
- **The premise ports.** A NURBS point is linear in its control points: freeze the tessellation at drag start, precompute each tessellation vertex's influence weight per moved control point, and every move is a multiply-add — no `point_at`, no `normal_at`, no re-tessellation. The adaptive rebuild runs once, on release.
- **The lane gap.** `Buffer::write_at` (`buffers.rs`) plus `Scene::ribbon_range(row)` means strokes can be rewritten in place today. Faces cannot: `ArenaLane`'s `verts` is private (`arena.rs`), `Scene` keeps no per-row vertex range. Adding one is a real lane change — `Vec<Option<Range<u32>>>` beside `ribbon_ranges`, filled in `add_file`, plus `ArenaLane::write_verts`.
- **Bites.** An in-place vertex write does not bump `geometry_revision`, the key for the triangle tile index (`ProjectionKey { matrix, objects }`, `triangle_tiles.rs:178-181`) and the silhouette masks (`MaskKey.geometry`): visibility then culls against the pre-drag projection while the picture shows the deformed one. Bump per drag frame, paying a re-projection — the correct default — or prove the cheaper thing in a comment.
- **Bites.** `Instance::FLAG_SMOOTH` says a row is a tessellation; a deformed tessellation still is one.

### 13 · No edge you can drag
- Ctrl+Shift+click an edge and move it, the highlight following the real curve, not its chorded control polygon. Needs (10). (gumball)
- **Half exists**: `PickMode::Edge`, `Scene::edge_at(pick)`, `SegRows.pipe_ids`, `SegmentLane::set_edge` (`src/state.rs`, `segments.rs`); every subdivision keeps its source edge id. The archive's 140 lines of edge selection are replaced; only the drag and write remain.
- **Ports as a rule.** The drag highlight is the frozen base polyline mapped by the delta: a whole-edge transform is affine, so nothing is re-evaluated per move.

### 14 · No BRep hole → collar deform
- Pull a hole rim: the rim follows, the wall swells smoothly, the outer boundary holds, the trim is untouched. Archive ~300 lines — degree-elevate the face, inner radius from the rim, outer radius from the nearest outer-boundary control point, each control point moved by `g·(delta·P − P)` with `g` a smoothstep from 1 inside to 0 outside. Needs (10) and (12).
- **A shape, not code:** the viewer supplies a delta and two radii, the kernel does the geometry — which is why this does not belong in the viewer at all.
- **Bites.** The archive's write-back skips these faces: the flat overlay nodes are stale after a collar and writing them would undo it (`state_edit.rs:1387-1390`). Any deform moving control points the overlay does not know about needs the same exclusion.

### 15 · No selection filters
- Not a CPU test after the pick: `Picker::configure(mode, radius_css, scale)` decides which lanes draw ids, so the filter is a mask carried into that decision and a filtered-out lane never writes an id.

```rust
// sketch, src/engine/gpu/pick.rs
struct Filter(u32); // one bit per lane: faces, strokes, markers, clouds, text, sheets
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
- **Not a gap.** Scenes come from manifests and `.pb` files (`src/app/manifest.rs`, `loader.rs`, `route.rs`, `live.rs`), validated before kernel constructors allocate from serialized counts (`src/app/validate.rs`). `ARCHITECTURE.md` §8 lists the archive's `demo.rs` (860 lines) as a structural defect — "app data, not engine", `State::new` hardcoding `demo::active_scene`.
- **Worth having:** primitive *commands* (`box`, `sphere`) constructing through `Session::add_*` inside a transaction — item 5 with different verbs. (command line)

## What we take from the old viewer and what we do not

**Ports** — `edit_points.rs` (157 lines), kernel-only and tested; `coord_parser.rs` (44 lines) at f64; `SnapKind`, its priority ladder and `SnapModes` (priority class first, pixel distance breaks ties); absolute snapshots on both sides of a recorded edit, and the accumulating-delta bug behind them; the live-deform premise (freeze, precompute weights, multiply-add); f64 through, f32 at the boundary, now in the write direction too; iso-curves, not triangle edges.

**Adapts** — getpoint loop → a `Draft` driven by named actions with `touch` at every mutation; snap emission ports but the `session.lookup` sweep does not (ask what a row is); snap ranking stays CPU only for candidates no lane draws; invalidation → a revision counter; construction plane → forward-axis rule kept, world-origin placeholder replaced; `NodeAddr` → `ControlId` plus two Greville arms and the write direction; the two write hazards (mesh BVH, BRep 3D edges) as rules; bake-vs-matrix → the question, re-answered against the two-table transform; multi-selection yes, the centroid box test → a rectangle id read; the transient preview → an owned region of the segment and glyph lanes; `log_prompt` → `feedback::status`.

**Replaced** — `undo_state.rs` + `state_undo.rs` (303 lines), because the kernel history is the cursor; `EditState`'s GPU half, because the `controls`/`control_net` lanes are it; `build_overlay_data`, because `Controls::from_geometry` covers more types and is tested; `pick.rs` CPU raycasting and `closest_object_under_ray`, because selection is GPU ids with a halo, a generation and a mode — the kernel's `ray_cast` has never been called here and should not start; `selected_centroid` and its per-type arms, because `InstanceTable::row_bounds(row)` gives the box and a widget origin is its centre; `text.rs` (256 lines, an 8×16 atlas as screen-anchored quads), replaced by text objects with scene identity; `demo.rs` (860 lines), replaced by manifests, loader, routes and live reload; `rebuild_tess_wireframe` as written; `selftest.rs` (164 lines), replaced by the headless harness; Escape falling through to `event_loop.exit`, since Escape here is `escape_selection` with "cancel the active tool" in front of it.

- Also replaced: the `impl State`-at-crate-root arrangement (`state_tool.rs`, `state_edit.rs`, `state_undo.rs`, `state_interaction.rs`, `state_pick.rs`, `state_cmd.rs`, `state_ui.rs`, `state_render.rs`, `state_update.rs`). `commit_object_transform` alone reads and writes the session lookup, the xform table, four GPU pick-mesh caches, instance flags, colour overrides, the thickness uniform and the text labels. That coupling is why the gap list cannot be closed by copying files.

## The order to build the rest in

Each step compiles; each names what it must not break. Steps 1 to 8 of the original order are
lesson 21; what follows is what is left.

1. **Filters** (15) — picking: mask full is byte-identical, the headless harness proves it.
2. **Multi-selection and rectangle pick** (9) — silhouettes (`selection_revision` once per gesture), sheets (a sheet row selects the sheet).
3. **Getpoint and the four tools** (5), rubber band in a transient `segments` region — finite-triangle visibility (the preview is strokes), text identity (`register_text` on rebuild), one frame per demand (`touch`).
4. **Control write-back on Mesh and NurbsSurface** — lesson 21 does Polyline and NurbsCurve; a mesh vertex and a surface CV need `invalidate_triangle_bvh` and the CAD contract re-derived by `app/walk/brep_edges.rs` on rebuild.
5. **Greville edit points** (11) — nothing; the two `ControlId` arms are additive.
6. **Live deform** (12) plus the per-row vertex range — the tile index and silhouettes (bump `geometry_revision`), `FLAG_SMOOTH`.
7. **Edge drag** (13), **wireframe toggle** (16), **naming and prompts** (17) — the keyboard map, `feedback::status` staying `textContent`.
8. **BRep collar** (14), if wanted at all, as a kernel operation the viewer parameterizes.

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
