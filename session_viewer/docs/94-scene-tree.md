# 94 Scene tree — the documents, as a panel

> **Big picture.** *Phase 12 — scene management UI (75–77).* Everything the viewer knows lives in
> maps and flags; users need it as a **list they can read and poke**: every document's tree in a
> side panel, an eye icon per row driving 59's visibility, names instead of guids. Two disciplines
> carry the lesson: **virtualize** (build only the visible rows — a 744k-object scene must scroll
> like a 10-row one) and **the tree renders state, it doesn't own it** — every toggle routes
> through the same `Scene` verbs the CLI uses, so the panel can never drift from the viewport.
> And one identity rule that decides whether any of it works — see the warning below.

<svg viewBox="0 0 680 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the docs each contribute a top-level row; each session tree flattens to visible rows only; each row shows an eye toggle and a name; toggles call the same scene verbs as the CLI" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="10" y="14" width="220" height="124" fill="none" stroke="#3a3a3a"/>
  <g fill="#d7dae0" font-size="10">
    <text x="20" y="32">👁 ▾ Querschnitt G-G   (doc 0)</text>
    <text x="34" y="50">👁 ▾ Bemassung</text>
    <text x="48" y="68" fill="#888">👁 line_4021…</text>
    <text x="48" y="86" fill="#888">👁 line_4022…  ◀ selected</text>
    <text x="34" y="104">👁 ▸ Schraffur (12,381)</text>
    <text x="20" y="122">👁 ▸ Längsschnitt C-C  (doc 1)</text>
  </g>
  <rect x="44" y="76" width="180" height="14" fill="none" stroke="#6fb3ff" stroke-width="1"/>
  <g transform="translate(280,20)">
    <text x="0" y="14" fill="#888">top level = one row per Doc (name from the manifest)</text>
    <text x="0" y="34" fill="#888">virtualized: only rows in the viewport exist this frame</text>
    <text x="0" y="54" fill="#888">eye → scene.hidden + flags (59's verbs — ONE authority)</text>
    <text x="0" y="74" fill="#888">click → scene.selected (58's verbs)</text>
    <text x="0" y="100" fill="#666" font-size="10">selection style: WHITE bg, BLACK text —</text>
    <text x="0" y="114" fill="#666" font-size="10">never white-on-dark (a real archive regression)</text>
  </g>
</svg>

## ⚠ The identity rule — read this before typing anything

The kernel's `TreeNode` has TWO identifiers, and confusing them makes the panel compile, render,
and silently match nothing:

- **`node.name`** is the identity the *document* uses: *"for geometry nodes, this is the
  geometry's GUID"* (tree.rs) — it is what `session.world_xform`/`world_xforms` key off, what
  `guid_to_row` contains, and what `scene.hidden` stores. Group nodes carry their
  human name here (the PDF importer makes one group per CAD layer). (`scene.selected` is
  **row-keyed** since 50 — `HashSet<u32>` — so the panel resolves guid→row through
  `guid_to_row` once, at flatten time: `Row.scene_row` below.)
- **`node.guid()`** is the node's OWN lazily-minted uuid — fine as UI-local expansion state, wrong
  for everything else.

Corollary: the object-keyed tree lookup is **`tree.get_node_by_name(guid)`**;
`find_node_by_guid` matches node uuids and will return `None` for every object guid you pass it.

## Files we touch

```
src/ui/tree.rs   # NEW — flatten visible tree rows; egui show_rows virtualization; eye + select
src/ui/mod.rs    # a left SidePanel hosting it; TreeUi state (expanded set, scroll target, flatten cache)
src/state.rs     # apply collected intents (toggle/select) after the closure — 60's rule
src/app/scene.rs # object_name / untreed_guids — doc-aware adapters
```

## Step 1 — flatten what's visible: `src/ui/tree.rs`

egui is immediate-mode: nested `CollapsingHeader`s would *build every row every frame* — the
anti-virtualization. Instead keep an `expanded: HashSet<String>` (expansion KEYS — node uuids plus
the synthetic `doc:N` keys) and flatten to a `Vec<Row>` of only the rows expansion makes visible,
then hand that to `ScrollArea::show_rows`, which instantiates **only the on-screen slice**:

```rust
pub struct Row {
    pub key: String,                   // expansion identity: node.guid() uuid, or "doc:N"
    pub guid: Option<String>,          // Some(OBJECT guid) for geometry rows — 76 scrolls/zooms by it
    pub scene_row: Option<u32>,        // guid_to_row[guid] at flatten time — selection is ROW-keyed (58)
    pub node_name: String,             // node.name — object guid or group name (the doc identity)
    pub doc: usize,
    pub name: String,                  // display
    pub depth: usize,
    pub is_branch: bool,               // has children → draws the ▸/▾
    pub expanded: bool,
}

/// One top-level row per Doc, then that doc's session.tree depth-first, descending only into
/// expanded branches. Ten sheets fully collapsed = ten rows; fully expanded = a big Vec, but
/// show_rows still renders ~40 of them.
pub fn flatten(scene: &Scene, expanded: &HashSet<String>) -> Vec<Row> {
    fn walk(node: &Rc<RefCell<TreeNode>>, depth: usize, d: usize, scene: &Scene,
            expanded: &HashSet<String>, out: &mut Vec<Row>) {
        let n = node.borrow();
        let key = n.guid().to_string();          // UI-only expansion identity
        let node_name = n.name.clone();          // the DOCUMENT identity (guid for objects)
        let children = n.children();
        let is_branch = !children.is_empty();
        let is_expanded = expanded.contains(&key);
        let guid = scene.docs[d].session.lookup.contains_key(&node_name)
            .then(|| node_name.clone());
        let scene_row = scene.guid_to_row.get(&node_name).copied();   // Some for geometry rows
        out.push(Row { key, guid, scene_row, node_name: node_name.clone(), doc: d,
                       name: row_name(scene, d, &node_name), depth,
                       is_branch, expanded: is_expanded });
        if is_branch && is_expanded {
            for c in &children { walk(c, depth + 1, d, scene, expanded, out); }
        }
    }
    let mut rows = Vec::new();
    for (d, doc) in scene.docs.iter().enumerate() {
        let key = format!("doc:{d}");
        let open = expanded.contains(&key);
        rows.push(Row { key, guid: None, scene_row: None, node_name: String::new(), doc: d,
                        name: format!("{}  ({} objects)", doc.name,
                                      doc.session.lookup.len()),
                        depth: 0, is_branch: true, expanded: open });
        if !open { continue }
        // the kernel root is synthetic — start from its children
        if let Some(root) = doc.session.tree.root() {
            for c in &root.borrow().children() { walk(c, 1, d, scene, expanded, &mut rows); }
        }
        // objects the tree doesn't parent (47's nurbs collections) → rows under the doc
        for guid in scene.untreed_guids(d) {
            rows.push(Row { scene_row: scene.guid_to_row.get(&guid).copied(),
                            key: guid.clone(), guid: Some(guid.clone()),
                            node_name: guid.clone(), doc: d,
                            name: row_name(scene, d, &guid), depth: 1,
                            is_branch: false, expanded: false });
        }
    }
    rows
}

/// Display name: the object's own name if it set one, the group name for groups, else short guid.
fn row_name(scene: &Scene, d: usize, node_name: &str) -> String {
    scene.object_name(d, node_name)
        .unwrap_or_else(|| if node_name.chars().count() > 12 {
            // CHARS, not bytes — &name[..8] panics on a multi-byte boundary (guids are ASCII,
            // but a group's human name passes through here; German layer names carry umlauts)
            format!("{}…", node_name.chars().take(8).collect::<String>())
        } else {
            node_name.to_string()   // a group's human name passes through
        })
}
```

`object_name` / `untreed_guids` are two thin adapters on `impl Scene` (`app/scene.rs`) — doc-aware,
and using the RIGHT tree lookup:

```rust
    /// The object's own name, if the kernel type carries one it set. Doc-scoped.
    pub fn object_name(&self, d: usize, guid: &str) -> Option<String> {
        let name = match self.docs[d].session.lookup.get(guid)? {
            Geometry::Mesh(m) => m.name.clone(),
            Geometry::BRep(b) => b.name.clone(),
            Geometry::Line(l) => l.name.clone(),
            Geometry::Polyline(p) => p.name.clone(),
            Geometry::Point(p) => p.name.clone(),
            Geometry::NurbsCurve(c) => c.name.clone(),
            Geometry::NurbsSurface(s) => s.name.clone(),
            _ => return None,
        };
        if name.is_empty() { None } else { Some(name) }
    }

    /// Renderable guids of doc `d` with no tree node — collection-only citizens (47) become
    /// rows under the doc. NOTE get_node_by_name: object guids live in node.NAME.
    pub fn untreed_guids(&self, d: usize) -> Vec<String> {
        let session = &self.docs[d].session;
        session.order().into_iter()
            .filter(|g| self.guid_to_row.contains_key(g))
            .filter(|g| session.tree.get_node_by_name(g).is_none())
            .collect()
    }
```

(`session.tree` is the kernel's `Tree`/`TreeNode` (`Rc<RefCell<…>>`) — the same structure
`remove_object` maintains (64), so deleted objects leave the panel automatically.)

## Step 2 — the rows, virtualized: `src/ui/tree.rs`

`TreeIntent` is the collect-then-apply buffer (60's rule) the panel fills and `State` drains — add
it to `src/ui/tree.rs` above `tree_panel`. Eye/expand intents carry document identities
(`node_name` / expansion key); name clicks carry the **scene row** — 58's `selected` set is
row-keyed, and `flatten` already resolved it:

```rust
#[derive(Default)]
pub struct TreeIntent {
    pub toggled: Vec<(usize, String)>,           // eye clicks → (doc, node_name)
    pub expand_toggled: Vec<String>,             // ▸/▾ clicks → expansion KEY
    pub clicked: Vec<(usize, u32, bool)>,        // name clicks → (doc, scene ROW, shift)
}
```

```rust
pub fn tree_panel(ui: &mut egui::Ui, rows: &[Row], scene_sel: &HashSet<u32>,
                  scene_hidden: &HashSet<String>, out: &mut TreeIntent) {
    let row_h = 18.0;
    egui::ScrollArea::vertical().show_rows(ui, row_h, rows.len(), |ui, range| {
        for row in &rows[range] {                                       // ← ONLY the visible slice
            ui.horizontal(|ui| {
                ui.add_space(row.depth as f32 * 12.0);
                // eye FIRST so long names truncate, not the control
                let visible = !scene_hidden.contains(&row.node_name);
                if ui.selectable_label(false, if visible { "👁" } else { "—" }).clicked() {
                    out.toggled.push((row.doc, row.node_name.clone()));
                }
                if row.is_branch {
                    let arrow = if row.expanded { "▾" } else { "▸" };
                    if ui.selectable_label(false, arrow).clicked() {
                        out.expand_toggled.push(row.key.clone());
                    }
                }
                let selected = row.scene_row
                    .map_or(false, |r| scene_sel.contains(&r));
                if ui.selectable_label(selected, &row.name).clicked() {
                    if let Some(r) = row.scene_row {
                        out.clicked.push((row.doc, r, ui.input(|i| i.modifiers.shift)));
                    } else {
                        out.expand_toggled.push(row.key.clone());   // clicking a group toggles it
                    }
                }
            });
        }
    });
}
```

Two archive rules baked in: the **eye sits before the name** (a long name must truncate, never push
the control out of reach), and the selected row's style is **white background, black text** — 60's
`Visuals` already set `selection.bg_fill`/`override_text_color`; never restyle it white-on-dark (the
archive shipped that once; selected rows became unreadable).

## ⚠ Cache the flatten — `show_rows` virtualizes widgets, not your `Vec`

`flatten` walks every expanded node and allocates several `String`s per row — O(visible tree) with
allocator churn. Once per *change* that's nothing; once per *frame* a fully-expanded 744k-object
sheet stutters. `show_rows` only saves you the egui *widgets* for off-screen rows — the `Vec<Row>`
you hand it is built in full either way. So treat the flattened rows as derived state and cache
them, keyed on two generation counters:

```rust
// UiState additions (init: empty Vec, (u64::MAX, u64::MAX) so the first frame rebuilds):
pub flattened_rows: Vec<Row>,
pub rows_key: (u64, u64),     // (scene generation, expansion generation) the cache was built from
pub expansion_gen: u64,       // bumped wherever tree_expanded mutates (Step 3's drain, 83's reveal)
```

```rust
// where the panel is drawn (ui/mod.rs), IN PLACE OF a per-frame flatten(...):
let key = (self.scene.generation, self.ui.expansion_gen);
if self.ui.rows_key != key {
    self.ui.flattened_rows = flatten(&self.scene, &self.ui.tree_expanded);
    self.ui.rows_key = key;
}
tree_panel(ui, &self.ui.flattened_rows, &self.scene.selected, &self.scene.hidden, &mut intent);
```

`scene.generation` is a `u64` that every scene mutation bumps — add it to 50/51/64's verbs and to
drains that touch the scene sets directly (Step 3's is one). Same discipline as 78's
render-on-demand: rebuild on change, never on vsync.

## Step 3 — intents apply after: `src/state.rs`

`TreeIntent { toggled, expand_toggled, clicked }` collects inside the closure (60's rule); `State`
drains it after, through the **existing** verbs — no second authority. Add this drain at the bottom
of the per-frame UI method, right **after** the egui closure that ran `tree_panel` returns (the
`intent` it filled is now owned here):

```rust
        let any_toggle = !intent.toggled.is_empty();   // read BEFORE the loop moves the Vec
        for (d, name) in intent.toggled {
            // a BRANCH (group) toggles its whole descendant set — object guids live in node.NAME
            if let Some(node) = self.scene.docs[d].session.tree.get_node_by_name(&name) {
                let kids = node.borrow().descendants();
                if !kids.is_empty() {
                    let hide = !self.scene.hidden.contains(&name);   // branch state leads
                    for k in kids {
                        let kg = k.borrow().name.clone();            // NAME, not guid()
                        if hide { self.scene.hidden.insert(kg); } else { self.scene.hidden.remove(&kg); }
                    }
                    if hide { self.scene.hidden.insert(name); } else { self.scene.hidden.remove(&name); }
                    continue;
                }
            }
            if !self.scene.hidden.remove(&name) { self.scene.hidden.insert(name); }
        }
        // 51
        if any_toggle { self.scene.apply_visibility(&mut self.gpu); self.poke(); }

        for (_, row, shift) in intent.clicked {
            if shift {
                if !self.scene.selected.remove(&row) { self.scene.selected.insert(row); }
            } else {
                self.scene.selected.clear();
                self.scene.selected.insert(row);
            }
        }
        // 50
        self.scene.apply_selection(&mut self.gpu);
        // 57
        self.refresh_gumball();
        let any_expand = !intent.expand_toggled.is_empty();
        for k in intent.expand_toggled {
            if !self.ui.tree_expanded.remove(&k) { self.ui.tree_expanded.insert(k); }
        }
        if any_expand { self.ui.expansion_gen += 1; }   // invalidate the flatten cache (⚠ above)
```

(`tree_expanded: HashSet<String>` is the `UiState` field the panel's `flatten` call reads — add it
with the SidePanel wiring, init empty. The eye can show mixed state (—) when children disagree —
cosmetic, left as polish.)

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://127.0.0.1:8770
```

- The panel opens with **one row per sheet** (the manifest names), each expandable into its CAD
  layers — the PDF importer built one group per OCG layer, so real drawings arrive pre-organized.
- **Scrolling is smooth** with everything expanded — `show_rows` instantiates ~40 row *widgets*
  whether the flattened list holds 50 or 744,000 rows, and on a pure scroll frame the cached
  flatten doesn't rebuild at all. That's the virtualization contract.
- Click an eye on a LAYER → the whole layer blinks out in the viewport *and* stops picking (59's
  single authority — the panel called the same verb, and the branch resolved its descendants by
  node **name**). Click an object's eye → just that object.
- Click a name → viewport selection + gumball follow (58/65). Shift+click adds. The selected row
  reads black-on-white.
- Delete an object (64) → its row is gone next frame; undo → row returns. The panel was never told —
  it renders each `session.tree`, which the Commands already maintain.

## Recap

```
Ch 74: Phase 11 closed — the look, fast.
Ch 75: THE TREE. Top level = one row per Doc (manifest name + object count); each doc's
       session.tree flattens depth-first through an `expanded` set → Vec<Row>; egui
       ScrollArea::show_rows instantiates ONLY the on-screen slice (immediate-mode virtualization —
       nested CollapsingHeaders would rebuild 744k rows/frame). THE IDENTITY RULE: an object's guid
       lives in node.NAME (node.guid() is the node's own uuid; the object-keyed lookup is
       get_node_by_name) — hidden/world_xform key off the name, selection off the ROW (58, resolved
       to `Row.scene_row` at flatten). Rows: eye
       BEFORE name, ▸/▾ for branches, white-bg/black-text selection. Panel COLLECTS TreeIntent
       ((doc, name) tuples); State drains it through 59's visibility and 58's selection verbs — one
       authority, no drift; branch-eye resolves descendants by NAME. Untreed collections join under
       their doc. The flatten is CACHED, keyed on (scene generation, expansion generation) —
       show_rows virtualizes widgets, not your Vec. Deletes/undo work — the panel renders the
       trees the Commands already maintain.
```

Edited: `ui/tree.rs` (NEW — `Row`, `flatten`, `tree_panel`, `TreeIntent`), `ui/mod.rs` (SidePanel +
`tree_expanded`, flatten cache), `app/scene.rs` (`object_name(d, guid)`, `untreed_guids(d)`),
`state.rs` (intent drain through existing verbs).

## Next

`95-tree-viewport.md` — the two views converge: picking in the viewport reveals and scrolls the tree
to the object (auto-reveal — find the owning doc, then `get_node_by_name` + `ancestors()`);
selecting in the tree highlights in the viewport (already true) — plus double-click-to-zoom. Small
lesson, big daily-use payoff.
