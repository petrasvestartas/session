# Adding panels: tree, types and graph

## One visibility set, three filters

- `Scene.hidden: HashSet<(usize, Rc<str>)>` (`src/app/scene.rs`) is the only record of what is not drawn; written today only by `State::hide_selected` and `show_all` (`src/state.rs`).

```rust
// Sketch, not code to type. src/state.rs
struct Filters {
 tree: Vec<(usize, Vec<u16>)>, // hidden subtrees: document index, child-index path
 kinds: Vec<Kind>, // hidden type buckets
 vertex: Vec<String>, // hidden graph vertex attributes
 edge: Vec<String>, // hidden graph edge attributes
 manual: HashSet<(usize, Rc<str>)>, // what H hid, one object at a time
}
```

- Axes, not a hidden set per panel; `Scene.hidden` is their union — else clearing the tree filter reveals rows the type filter still hides.
- `H` is a fourth axis, so `show_all` is one loop.
- Axes store the FILTER, never rows: `Scene::rebuild` renumbers rows, paths and attribute strings survive.
- So un-hiding is per axis, not per row. "Hide all but this" is `isolate`, the complement into `manual`.

## What each filter keys on

### By element type

```rust
// Sketch. src/app/filter.rs
enum Kind {
 Point, Line, Plane, OBB, Polyline, PointCloud, Mesh,
 NurbsCurve, NurbsSurface, BRep, Element, // the kernel's own order
 StreamedCloud, Sheet, Text, // row kinds only this viewer has
}
struct TypeRanges { base: u32, ends: [u32; 11] }
```

- The eleven are `Session::order`'s sequence (`session_rust/src/session.rs`); `add_file` pushes rows in it, so each (document, kind) is a CONTIGUOUS run — eleven `u32`, 44 bytes per document, no per-row storage.
- Two skips shorten a bucket, never reorder one: a guid missing from `session.lookup`, and `is_drawable` (`src/app/walk/mod.rs`) rejecting an `Element` with `None` geometry. Elements are last, so nothing after them moves.
- Hiding a type is a RANGE write, not a scan.

### By tree node

```rust
// Sketch. src/app/filter.rs
struct TreeIndex {
 nodes: Vec<Node>, // depth-first: a parent always precedes its children
 row_node: HashMap<u32, u32>, // object row -> index of the leaf node that names it
}
struct Node { parent: Option<u32>, child_index: u16, depth: u16, label: Rc<str>, rows: Vec<u32> }
```

- Leaf test: `Scene::guid_to_row` contains `(document, node.name)` — `TreeNode.name` IS the geometry guid (`session_rust/src/tree.rs`) and `push_row` fills `guid_to_row`, so it cannot disagree with the rows.
- `Node.rows` is the group's whole subtree, resolved once: rows are what `set_hidden` takes, guids what survives a rebuild.
- `row_node` + `Node.parent` opens ancestors in O(depth); `src/app/validate.rs` caps hierarchies at 64 levels.
- Key the filter and the open set on `(document, path)` — child indices from the root, `(0, [2, 1])`. Reason below.
- Walk once, in `add_file`: `TreeNode::children` clones its `Rc` vector on every call.

### By graph element

```rust
// Sketch. src/app/filter.rs
struct GraphIndex {
 vertex: HashMap<Rc<str>, Vec<u32>>, // vertex attribute -> rows
 edge: HashMap<Rc<str>, Vec<u32>>, // edge attribute -> the rows of BOTH endpoints
}
enum Filter { Tree(usize, Vec<u16>), Kind(Kind), VertexAttribute(String), EdgeAttribute(String) }
```

- `Graph::add_node(key, attribute)` takes the geometry guid as key (`session_rust/src/graph.rs:290`): a `Vertex`'s name is a guid, `attribute` the label to group on. `src/app/decode.rs` matches, so wasm and native agree. `src/app/decode.rs` matches, so wasm and native agree.
- Edges have no rows, so an edge filter hides both endpoints — and the row says "joint — 8 objects", not "4 edges".
- Read once per load: `get_vertices` clones every `Vertex`, `get_edges` de-duplicates into a new `Vec<(String, String)>` — ~114k allocations on the sheet fixture.
- `node_attribute` and `edge_attribute` need `&mut self`, unreachable behind `Rc<Session>`; the index is the only read path.
- Free later, all `&self`: `neighbors`, `bfs`, `connected_components`, `shortest_path`, `cycle_basis`.

## One visibility decision, and the path it takes to the GPU

```rust
// Sketch. src/state.rs
pub fn set_filter(&mut self, f: Filter, on: bool) // write one axis, then apply
pub fn isolate(&mut self, f: &Filter) // complement into `manual`, then apply
pub fn hide_rows(&mut self, rows: &[u32], on: bool) // what hide_selected becomes
fn apply_filters(&mut self) // union -> diff -> runs -> GPU
```

- `apply_filters`: union the indexes plus `manual`, diff against `Scene.hidden`, coalesce the FLIPPED rows into runs, write, replace.
- Coalesce changed rows, not targeted rows — `set_flag` early-returns on a matching bit, so a naive run write makes a redundant toggle cost megabytes.
- `Scene.hidden` stays `(document, guid)`: `add_file` re-applies that form while pushing rows, and guids collide between documents (`Scene::identity_of`).

```rust
// Sketch. src/engine/gpu/objects.rs and src/engine/gpu/mod.rs
impl InstanceTable { pub fn set_flags_run(&mut self, ctx: &GpuCtx, rows: Range<u32>, bit: u32, on: bool) }
impl Gpu { pub fn set_hidden_rows(&mut self, runs: &[Range<u32>], on: bool) }
```

- `set_flags_run`: flip the CPU mirror slice, ONE `GrowBuf::write_at` (it already takes a slice and multiplies by the stride).
- `set_hidden_rows`: that per run, then `splat.invalidate` ONCE per batch — the rebuild it forces walks every cloud.
- Both beside the single-row `set_flag`/`set_hidden`; `InstanceTable` owns the rows, so the stride stays there.

## What a hide costs

- One row: 96 bytes (`Instance` is the 96 B storage stride, asserted in `src/engine/gpu/instance.rs`), a `geometry_revision` bump, one cloud-record invalidation.
- One bucket on `assets/pb/view_local_sheet_querschnitt.pb`: 56,889 line rows, one run, 96 × 56,889 ≈ 5.5 MB in a single `queue.write_buffer` against 56,889 writes — why `set_flags_run` exists.
- Untouched: the model matrix, the 16-byte anchored translation row (`src/engine/gpu/objects.rs`). No re-anchor, no rebuild, no camera work.
- Free elsewhere: every vertex stage parks hidden rows outside the clip volume (`shaders/triangle.wgsl`, `ribbon.wgsl`, `glyph.wgsl`, `sphere.wgsl`) and the id pass reuses those stages, so drawing and picking stop together. A "do not pick the hidden" list would be a second truth.
- One CPU consumer: `src/engine/gpu/splat.rs` drops hidden rows while rebuilding cloud records — what `invalidate` is for.
- A REPARENT is a full `Scene::rebuild`, not a hide: `session.world_xforms` composes placements at walk time in `add_file`. Say so in the UI.

## Where the indexes are built

- All three in `Scene::add_file` (`src/app/scene.rs`): eleven counters in the existing `push_row` loop, one tree DFS, one `get_vertices` pass.
- Not `src/app/decode.rs` — wasm-only, and the native harness (`examples/check_determinism.rs`, `src/selftest/lifecycle.rs`) hand-builds a `FileDoc`, so no native test would see the index.
- Not per frame — `order`, `owners` and `guid_to_row` are private to `scene.rs`, and `add_file` already knows each guid's row.
- Clear in `reset_rows`, so `Scene::clear` and `rebuild` need no new field names.
- `add_streamed_cloud`/`add_sheet`: one `Kind` entry each, and empty — not absent — tree and graph entries, because those vectors are indexed alongside `Scene.docs`.

## The panel's rows

- Five row kinds, none an object row: document header, tree group, tree leaf, type bucket, attribute bucket.
- `src/app/panel.rs` (new, beside `scene.rs`) owns the flat list, open set, scroll and hit test; it names no wgpu type, so it unit-tests with no device — `src/app/mod.rs`'s rule for that directory.
- Flat list rebuilt on open-set, search or scene change, never per frame: an open group costs its whole visible subtree, paid on the click that opened it.
- Window: `first = floor(scroll / ROW_H)`, `count = ceil(panel_height / ROW_H) + 1` — the `+ 1` is the partly visible bottom row, submitted so the clip cuts it, or rows pop at the boundary.
- `ROW_H = 18.0` CSS px: `validate_label` (`src/engine/text.rs`) needs `line_height >= font_size`; 13 px text leaves 5 px of leading and squares the 18 × 18 icon cells.
- `INDENT_W = 14.0`, `ARROW_W = 12.0`: group `depth * INDENT_W`, leaf `+ ARROW_W`, so leaf text starts under parent text — the group's triangle costs `ARROW_W` and the leaf has none to spend.
- `PANEL_W = 280.0` CSS px: the clip rectangle cuts long names there instead of spilling them across the scene.
- The panel OVERLAYS the canvas (`State::logical_size` reads the canvas element); shrinking the viewport would retarget every pass for a UI strip. Cost: that 280 px strip is unpickable while open.
- Bucket rows make 56,889 objects readable: one per non-empty kind per document, with a count. Querschnitt's graph panel collapses to ONE row — `line_my_line`, 56,889 objects — which is why the type panel exists beside it.

## Drawing the panel

- Row TEXT via the text lane: one `TextLabel` per visible row, `TextPlacement::Screen { left, top }` in CSS px, `clip: Some([l, t, r, b])`, `object: None`.
- `object: None` because a plate writes `object.row + 1` into the ID target (`text_plate.rs`) and would answer scene picks with an unrelated row, and `text_rectangle` (`text.rs`) would give it a rounded pill, not a row band.
- One `gpu.text.set_labels(...)` for panel and scene together: it REPLACES the document, so a second call erases the nameplate and every source text. Extend `State::update_label` (`src/state/text.rs`).
- Ids unique per submission: objects use `row + 1`, the nameplate `0`, panel rows `PANEL_LABEL_ID = 1 << 31` plus the flat-list index — object rows never reach 2^31.
- The id is also the shaping-reuse key (`same_layout`: text, font size, line height), so scrolling moves `Screen { left, top }` and reshapes NOTHING — pinned by `placement_and_color_do_not_reshape_but_font_reload_does` (`src/engine/text.rs`). Key on row identity, never a screen slot.
- `include_text_bounds` skips `Screen`: panel rows never grow the scene bounds, and `F` still frames the model.
- 256 KiB of text per submission (`MAX_TEXT_BYTES`) — another reason to submit only the window.
- Row CHROME is its own lane: `src/engine/gpu/panel.rs`, a field on `Gpu` beside `text`, drawn in `Renderer::scene_list` just before `self.text.draw(pass)` so bands sit under their glyphs.
 - One `GrowBuf` of CSS-px quads converted to NDC on the CPU, one pipeline, alpha blend, `depth_write_enabled: false`, `depth_compare: GreaterEqual` at 1.0 — copy `src/engine/gpu/text_plate.rs`, already that shape for that reason.
 - No id pipeline, not in `Renderer::id_pass`: a lane that draws no ids cannot be mis-picked.
 - Not `Plates` widened: it is `pub(super)`, its vertex carries an object row, and `text_rectangle` only emits nameplate and object-bearing rectangles.

## Hit testing: on the CPU, deliberately

- The GPU could do it with no new `PickMode`: `TextLane::draw_ids` runs outside the `match mode` in `Renderer::id_pass`, so a screen label with an `object` is pickable today.
- Don't: the id path answers occlusion, and costs a pass, a `copy_window` readback, a generation check and a frame of lag (`Picker::poll`, applied atop `State::render`) — hover cannot survive that. A row is an axis-aligned CSS-px box computed one call earlier: four comparisons, no lag, no stale-generation drop.
- The CPU test must add precedence: run it in `Input::left` (`src/app/input.rs`) BEFORE `state.request_selection(...)` and return on a hit.
- Units bite: `Input.last_cursor` is surface px (`CursorMoved` multiplies by `surface_per_physical`), `Screen` is CSS px. Divide by `device_pixel_ratio` (`src/engine/gpu/view.rs`) and lay out against `State::logical_size`.
- Hover on `CursorMoved` when not dragging; return `true` only when the hovered row CHANGED, else every mouse motion redraws the scene.
- `l` toggles the panel in `Input::key` — taken are c, f, q, w, e, o, d, h, s, t, b, p, 1–7, `[`, `]`, Space, Escape, F10.

## Expand, collapse, scroll, search

- Open set `HashSet<(usize, Vec<u16>)>`: document index plus child-index path. A path survives because both loaders preserve child order.
- Never `TreeNode::guid`: minted lazily from a per-process `OnceLock` and restored by neither loader (`build_tree` in `src/app/decode.rs` and the kernel's `proto_to_treenode` both call `TreeNode::new(&proto.name)`). Keyed on it, groups collapse and tree filters orphan on every reload.
- Default closed.
- Search filters the flat list, no auto-expand: that is 56,889 entries rebuilt per keystroke, and an unreadable tree. Matches flat, capped, counted, group as trailing context.
- Match `Scene::object_name(row)`, which falls back to the TYPE — "Line", "Mesh". Never the raw guid: 56,889 UUIDs are unreadable.

## Panel selection and viewport selection, in step

- `State::select(Option<u32>)` is the ONE place the highlight moves; a leaf click calls it, and the panel touches neither `Scene` nor `Gpu`.
- `Scene.selected` is one `Option<u32>`, so a GROUP row cannot select its subtree — multi-selection also changes `fit_selected_or_all`, the nameplate and `State::enable_controls`.
- A group row's targets: its triangle, its filter dot, and its label, which expands — the generous hit area.
- Reveal-on-pick inside `State::select`: `row_node[row]` up `Node.parent` (O(depth)), open ancestors, scroll into view. No cross-app flag, no picking change.
- Scroll only when the row is outside the window, else it fights the user's own scrolling.
- The filter dot is DERIVED from `Filters.tree`; a stored flag drifts the moment `H`, `show_all` or another axis touches those rows.

## Reload, rebuild, and what survives

- `Scene::rebuild` keeps `docs` and `hidden` and re-runs `add_file`, so the indexes return with the rows — provided `reset_rows` cleared them, or they double.
- `Scene::clear` drops documents and `hidden`; `State::clear` must clear `Filters` too, or a new scene starts half-hidden.
- Open state and scroll are path-keyed: they survive a rebuild of the same documents and die with a `clear`.
- Streamed clouds and sheets cannot return after a rebuild: there is no kernel object to re-walk, and `reset_rows` empties `Scene.streamed` and `Scene.sheets` before the first `add_file`, so the documents survive in `docs` as empty shells that walk to no rows. Refuse a reparent or delete while either is non-empty, say why on the row, and drop their rows if a rebuild happens anyway.
- Nothing prunes panel state: rebuild the flat list after every `add_file`, `rebuild` and `clear`, and drop entries whose document index is gone.

## The awkward rows

- STREAMED CLOUD: one row, guid `stream:{url}`, points not all resident (`done_to`, `total`). A single leaf under its own document; hiding is one flag write however much has arrived.
- STREAMED SHEET: one row, guid `sheet:{url}`, an EMPTY `Session::new(&name)` — no tree, graph or lookup. Hence the panel's ROOT is the document list, with session trees nested under it.
- LOCAL SHEET: `assets/view_local.yaml` loads `pb/view_local_sheet_querschnitt.pb` through `add_file` — 56,889 rows, each with a tree node and a graph vertex. Every cost claim here must survive that file.
- TEXT OBJECTS: rows with owner `usize::MAX`, keys like `document-title/0`. `Scene::visible_texts` and `restore_text_visibility` both recompute from `hidden`, so a filter calling `Gpu::set_hidden` alone is silently un-hidden by the next text replacement — write `Scene.hidden`.
- Text rows belong to no document: give them their own section.

## Edits, and the one undo

- A hide records NOTHING: `Op` (`session_rust/src/history.rs`) is `Add | Remove | Replace | Xform`, and hiding is view state dropped by `Scene::clear`. Recording it would make Ctrl+Z un-hide.
- No viewer-side undo stack: transactions, tombstones and the cursor are the kernel's.
- Transactions only for document changes (`History::begin`, `record`, `commit`): reparent and delete, not hide, expand, scroll or filter.
- No reparent op: `Tombstone` carries `parent_guid`, `index`, the detached `node` subtree, the graph `attribute` and incident `edges`, so a reparent round-trips as Remove + Add in ONE transaction. Otherwise add a kernel op — say which.
- `&mut Session`: `FileDoc.session` is `Rc<Session>`; the only precedent is `Rc::make_mut(&mut scene.docs[0].session)` in `scene.rs`'s tests — in place with a sole handle, a full clone otherwise. Check before editing 56,889 objects.

## The order that compiles

1. `src/app/filter.rs`, registered in `src/app/mod.rs`: `Kind`, `TypeRanges`, `TreeIndex`, `GraphIndex`, `Filter` — pure data, no `Scene`, no `Gpu`. Tests: a path key round-trips, a range is half-open, an empty index answers with no rows.
2. `Scene` grows `types`, `trees`, `graphs` — filled in `add_file`, cleared in `reset_rows`, synthetic entries from `add_streamed_cloud`/`add_sheet`. Check: existing `scene.rs` tests unchanged.
3. `Scene::filter_rows(&Filter) -> Vec<u32>`, tested in `scene.rs`: two documents sharing a guid (the existing collision test), a point/line/mesh document proving contiguity, an `Element` with no geometry shortening only its own block.
4. `InstanceTable::set_flags_run` and `Gpu::set_hidden_rows` — no callers, headless GPU tests still pass.
5. `State::hide_rows`; `hide_selected` its one-row case, `show_all` its clear-all. `hidden_rows` iterates a `HashSet`, so sort before coalescing.
6. `set_filter`/`isolate`/`apply_filters`, `Filters` on `State`, cleared in `State::clear`. Native example before any UI: load querschnitt, hide `Kind::Line`, assert the hidden count and ONE run.
7. `src/app/panel.rs` — rows, open set, scroll, window, `row_at(css_x, css_y)`; pure CSS-px geometry, no device.
8. `src/engine/gpu/panel.rs` — the chrome lane before `self.text.draw(pass)`: empty quad list, then one fixed rectangle, then the row model.
9. Panel labels inside `State::update_label`, visible window only; `?inspect=1` already reports `text_labels`.
10. `src/app/input.rs` — `l`, the hit test ahead of `request_selection`, hover on `CursorMoved`. First end-to-end step.
11. Reveal-on-pick inside `State::select`.
12. `?inspect=1` gains `filters`, `panel_rows`, `panel_rect` and a `hidden` count (`src/app/inspection.rs`), plus one `tests/*.cjs` beside `interaction.cjs`: a type bucket hides the right number of rows, a row click selects without firing a scene pick.

## What we take from the old viewer and what we do not

- **PORTS — the leaf rule**, a kernel property, against ONE guid set (`Scene::guid_to_row`) instead of that viewer's three disagreeing ones.
- **PORTS — the subtree leaf DFS, precomputed**; per frame it is O(subtree) per group row. The result here is `Vec<u32>` of ROWS, the write address.
- **PORTS — the indent arithmetic**: `depth * INDENT_W` for a group, `+ ARROW_W` for a leaf.
- **PORTS — derived group state**: the dot is computed, never stored, so it cannot drift from the viewport.
- **PORTS — claim the icon cells first, let the name shrink**; the rule ports, the widget calls do not.
- **ADAPTS — reveal-on-pick**: a flag the picking code reached across the app to set becomes a step inside `State::select`.
- **ADAPTS — panel click to selection**: a row click calls `State::select`, and a group row does not select at all.
- **ADAPTS — visibility**: a UI-written guid set becomes `Scene.hidden`, keyed `(document, guid)` because guids collide between documents, re-applied by `add_file`, written through one action.
- **ADAPTS — search**: the substring predicate ports, the auto-expand does not — one keystroke would rebuild 56,889 entries.
- **REPLACED — egui and every widget call**: none in `Cargo.toml`, no DOM panel in `index.html`. Rows are text-lane labels plus one quad lane.
- **REPLACED — the hit test**: `Response`-based becomes a four-comparison CPU box test.
- **REPLACED — the per-frame snapshot**: a guid→label map, a cloned leaf cache and a re-listed edge set are six figures of allocation per frame here. Indexes are built once, in `add_file`.
- **REPLACED — the god state and its change vectors**: they escaped a borrow conflict inside a UI closure; there is no closure — `src/app/input.rs` calls a named action on `&mut State`.
- **REPLACED — the viewer-side undo stack**: the history is `session_rust/src/history.rs`.
- **REPLACED — nothing, for the colour columns**: per-object override does not exist, `Instance.color` is written by the walk. Two pickers per row means an override system, a re-apply path after every rebuild, and a double write under group and leaf keys. Layer colour's honest source is `TreeNode.color` — a kernel change, three encode/decode gaps first.
- **REPLACED — nothing, for the separate web app**: its one transferable idea, labelling a graph node by its `attribute`, arrives via `Vertex.attribute`.
- **REJECTED — the group selection lock**: it makes one pick a multi-object selection this viewer cannot represent.
- **REJECTED — the per-leaf transform lock**: its only consumer there was a transform gizmo, which this viewer does not have.
- **REJECTED — behaviour keyed on a literal group name**: auto-hiding groups called "FloorModel" gives a different file different behaviour with nothing on screen saying so.
- **REJECTED — `TreeNode::guid` as any key**: minted per process, restored by neither loader.

## What to check on screen

- Step 3: nothing visible; `cargo test` passes, new `filter_rows` tests and every pre-existing `scene.rs` test.
- Step 5: `H` still hides the selection, `S` still shows everything, on a local file and a streamed sheet. Nothing on screen changed — that is the check.
- Step 6: hiding `Kind::Line` on querschnitt leaves the page blank of linework and reports one run; `show_all` restores it, frame time unchanged.
- Step 8: one fixed rectangle over the scene at the panel's position, in front of geometry, unmoved by orbiting.
- Step 9: rows read at the top-left, clipped at the right edge, scrolling without flicker; `?inspect=1` shows `text_labels` rising by the window count, not the row count.
- Step 10: `l` toggles the panel; hover highlights only that row; a leaf click selects that object and shows its nameplate; clicks inside the panel never change the 3D selection; clicks outside still pick.
- Step 11: picking an object opens its groups and scrolls its row into view exactly once, landing on the first matching row.
- Step 12: a type bucket on querschnitt removes exactly that bucket's objects; the graph attribute row removes the same set; both on, then one off, leaves the other's rows hidden.
