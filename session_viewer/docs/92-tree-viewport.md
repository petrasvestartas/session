# 92 Tree ↔ viewport — one selection, two views

> **Big picture.** *Phase 12.* The tree and the viewport show the same document; the moment they can
> disagree about *what's selected*, users stop trusting both. Half the sync already exists — tree
> clicks route through 58's verbs (82), so tree → viewport is free. This lesson closes the loop the
> other way: **pick in the viewport and the tree reveals the row** — expands its ancestors, scrolls
> it into view, highlights it. The archive called it auto-reveal (`tree_open` + `scroll_to_me`), and
> it's the feature that makes a 42k-row tree navigable at all.

<svg viewBox="0 0 680 120" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="clicking an object in the viewport expands its ancestor chain in the tree and scrolls the row into view; both views show one selection" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="20" y="20" width="150" height="80" fill="none" stroke="#3a3a3a"/>
  <rect x="55" y="45" width="46" height="30" fill="none" stroke="#e0b040" stroke-width="2"/>
  <text x="95" y="112" fill="#888" text-anchor="middle">click in viewport</text>
  <line x1="180" y1="60" x2="240" y2="60" stroke="#6fb3ff" stroke-width="1.4" marker-end="url(#ah71)"/>
  <rect x="250" y="14" width="190" height="92" fill="none" stroke="#3a3a3a"/>
  <g font-size="10">
    <text x="260" y="32" fill="#d7dae0">👁 ▾ model            ← expanded</text>
    <text x="272" y="50" fill="#d7dae0">👁 ▾ walls           ← expanded</text>
    <text x="286" y="68" fill="#000">wall_south</text>
    <rect x="282" y="58" width="150" height="14" fill="#ffffff" opacity="0.9"/>
    <text x="286" y="69" fill="#000">👁 wall_south  ◀ scrolled to</text>
    <text x="272" y="90" fill="#888">👁 ▸ beams</text>
  </g>
  <text x="530" y="45" fill="#666" font-size="10">expand ancestors</text>
  <text x="530" y="61" fill="#666" font-size="10">scroll_to_row</text>
  <text x="530" y="77" fill="#666" font-size="10">highlight (58's set)</text>
  <defs><marker id="ah71" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/ui/tree.rs   # reveal support: scroll_to: Option<String> honored inside show_rows
src/state.rs     # pick success → reveal(guid); double-click in the tree → zoom to object
```

## Step 1 — expand the ancestors: `src/state.rs`

On a successful viewport pick (55/57's `pick_ray`, right after the selection update in 58's click
handler), walk the picked node's parent chain and add every ancestor to `tree_expanded`:

```rust
    /// Make `guid`'s row exist and be on screen next frame.
    fn reveal_in_tree(&mut self, guid: &str) {
        // Multi-doc (35): find the Doc that owns this object first, then search ITS tree.
        // The object guid is the tree node's NAME, not its uuid → get_node_by_name.
        if let Some((d, doc)) = self.scene.docs.iter().enumerate()
                .find(|(_, doc)| doc.session.lookup.contains_key(guid)) {
            // the panel has N roots, one per Doc — open the owning doc's top-level row too
            self.ui.tree_expanded.insert(format!("doc:{d}"));
            if let Some(node) = doc.session.tree.get_node_by_name(guid) {
                // ancestors(): the kernel's TreeNode accessor — immediate parent up to root
                for a in node.borrow().ancestors() {
                    self.ui.tree_expanded.insert(a.borrow().guid().to_string());
                }
            }
            self.ui.expansion_gen += 1;   // invalidate 82's flatten cache
        }
        self.ui.tree_scroll_to = Some(guid.to_string());   // consumed by the panel next frame
        self.poke();                                       // 71 — the tree must redraw
    }
```

(`TreeNode::ancestors()` returns `Vec<Rc<RefCell<TreeNode>>>` from immediate parent up to root and
already `upgrade()`s the internal `Weak` parent link for you — never touch the private `.parent` field
directly. The lookup trap: for geometry nodes the object's GUID is stored as the node's **name**
(`tree.rs`), so the object-keyed lookup is `Tree::get_node_by_name` — `find_node_by_guid` matches the
nodes' *own* uuids and returns `None` for an object guid. And since 35 there is one `Session` per
`Doc`, so the owning doc must be found first — and its own top-level row key (`doc:N`) inserted
into `tree_expanded` (the snippet does both): the panel has N roots, one per `Doc`, and an expanded
ancestor chain inside a collapsed doc root is still invisible. If a picked object has no tree node
— a top-level nurbs row from 69 — `ancestors()` is empty and only the scroll fires.)

## Step 2 — scroll to the row: `src/ui/tree.rs`

The scroll request is *input* to the panel (State → panel), so it can't ride on 82's `TreeIntent`
(that flows panel → State). Give `tree_panel` one more parameter — extend 82's signature with
`scroll_to: &mut Option<String>` — and at the call site (in `ui/mod.rs`) pass `&mut self.ui.tree_scroll_to`.
**That is the bridge**: Step 1's `ui.tree_scroll_to` field and this `scroll_to` param are the same
one-shot slot. (If 82's wiring didn't add it: `tree_scroll_to: Option<String>` is a field on the
tree UI state, init `None`.)

```rust
pub fn tree_panel(ui: &mut egui::Ui, rows: &[Row], scene_sel: &HashSet<String>,
                  scene_hidden: &HashSet<String>,
                  scroll_to: &mut Option<String>,          // ← NEW: &mut self.ui.tree_scroll_to
                  out: &mut TreeIntent) {
```

`show_rows` gives us the geometry for free: rows have uniform height, so the target row's y is
`index × row_h`. Honor the one-shot request before drawing (find the existing `ScrollArea::vertical`
from 82's Step 2 and replace it with this):

```rust
    // inside tree_panel, before show_rows:
    let scroll_target = scroll_to.take()
        .and_then(|g| rows.iter().position(|r| r.guid.as_deref() == Some(g.as_str())));
    let mut area = egui::ScrollArea::vertical();
    if let Some(ix) = scroll_target {
        // ~3 rows of context above
        area = area.vertical_scroll_offset((ix as f32 * row_h - 60.0).max(0.0));
    }
    area.show_rows(ui, row_h, rows.len(), |ui, range| { … });
```

Order matters within the frame: `flatten` runs *after* Step 1's expansions landed in
`tree_expanded` (and bumped `expansion_gen`, so the cache rebuilds), so the target row exists in
`rows` by the time we look for its index. (Both happen
in the same `State` before `build_ui` — the data flow 52 set up already guarantees it.)

## Step 3 — double-click zooms: `src/ui/tree.rs` + `src/state.rs`

The reverse convenience: finding an object in the tree, then hunting it in a big scene, is the same
problem mirrored. Zoom *is* a genuine intent (panel → State), so it rides on 82's `TreeIntent` — add
`pub zoom_to: Option<String>` to that struct (the `#[derive(Default)]` covers it). Double-click a
row → frame it:

```rust
    // in the row: if row.guid.is_some() && ui.selectable_label(selected, &row.name).double_clicked()
    //     { out.zoom_to = row.guid.clone(); }

    // in the drain: zoom = the object's world box through 15's fit —
    if let Some(g) = intent.zoom_to {
        let (lo, hi) = self.scene.world_aabb(&g);                      // 53's helper
        let aspect = self.gpu.config.width as f64 / self.gpu.config.height as f64;
        self.camera.fit([lo[0] as f32, lo[1] as f32, lo[2] as f32],
                        [hi[0] as f32, hi[1] as f32, hi[2] as f32], aspect);
        self.poke();
    }
```

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Collapse the whole tree. **Click an object deep in the viewport** → the tree unfolds just that
  ancestor chain and the row appears highlighted, ~3 rows below the top edge. Click another → the
  tree follows. This is the archive's auto-reveal, and once you have it you can't go back.
- **Shift+click** several objects in the viewport → all their rows highlight (one selection set, two
  renderers of it); the reveal scrolls to the *last* picked.
- **Double-click** a row → the camera frames that object (15's fit on its box). Esc/click-empty
  deselects — both views clear together, because there is only one `scene.selected`.
- The stress test: reveal in the fully-collapsed 42k tree → instant (one chain expanded, one flatten,
  one scroll — never a full expansion).

## Recap

```
Ch 75: the tree renders state.
Ch 76: THE LOOP CLOSED. Viewport pick → reveal_in_tree: find the owning Doc, get_node_by_name
       (the object guid IS the node's name — find_node_by_guid matches node uuids), walk the
       TreeNode parent chain into tree_expanded (+ the doc's root row), set a one-shot
       tree_scroll_to, poke. The panel resolves it AFTER flatten (same-frame ordering via 60's data
       flow): row index × row_h → vertical_scroll_offset, ~3 rows of context. Uniform row height is
       what makes scroll-to-index trivial — a reason to keep rows uniform beyond aesthetics.
       Double-click a row → world_aabb → 15's camera.fit (the mirror convenience). One selection
       set, two views, no drift — the property everything in Phase 12 hangs on.
```

Edited: `ui/tree.rs` (`scroll_to` handling, double-click intent), `state.rs` (`reveal_in_tree`,
zoom-to drain).

## Next

`93-text-labels.md` — names in the 3-D view itself: billboarded text from a glyph atlas, readable at
every angle, one draw call for all labels — the archive's `text.rs` recipe on 29/30's instancing
bones.
