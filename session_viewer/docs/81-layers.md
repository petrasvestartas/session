# 81 Layers — naming what the importer already built

> **Big picture.** *Phase 14.* Ask any CAD user how they keep a model sane: layers — named buckets
> with one-click visibility. The viewer already *has* them: the PDF importer builds **one tree
> group per OCG layer, per doc** (pdf.rs — `add_group` per named CAD layer), 70's panel renders
> them with eye icons, and branch visibility resolves to 46's set operations. What's missing is
> *addressing them*: an architect's "Bemassung" layer exists on **every one of the ten sheets**,
> and "hide the dimensions" must mean all of them at once. So this lesson's real content is one
> reframe — **a layer is a NAME, not a node** — plus the `layer` verbs and an active layer for new
> objects.

## A layer is a NAME (read this before typing)

70's identity rule decides everything here. In the kernel tree, `node.name` is the document
identity: *"for geometry nodes, this is the geometry's GUID"* (tree.rs) — and **group nodes carry
their human name there**. So:

- the layer "Bemassung" is a *different node in every doc* that has it — a layer operation folds
  over `scene.docs` and looks the name up per doc: `doc.session.tree.get_node_by_name(name)`
  (NOT `find_node_by_guid`, which matches node uuids and returns `None` for names and object
  guids alike). One group per name per doc — the importer dedups (`or_insert_with`).
- a layer's members are its descendants' **names** — object guids live in `node.name`, which is
  exactly what `scene.hidden` and `scene.selected` store. No translation layer needed.
- `active_layer` is therefore an `Option<String>` holding a **name**, resolved to a node only at
  the moment of use, in whichever doc the new object targets.

## Files we touch

```
src/app/scene.rs      # active_layer: Option<String>; layer_names/layer_members/layer_node/
                      # assign_to_layer — all NAME-keyed, doc-folding
src/app/commands.rs   # `layer list` / `layer <name>` / `layer off|on <name>` / `layer active <name>`
```

## Step 1 — list what's already there: `src/app/scene.rs`

The best first demo is not creating a toy layer — it's *seeing the real ones* the import built.
Top-level tree children whose name is **not** an object guid in that doc are groups; collect them
across all docs (add `BTreeMap` to scene.rs's `std::collections` import — it sorts the listing
for free):

```rust
    pub active_layer: Option<String>,        // ← ADD to `struct Scene` — a layer NAME, not a
                                             //   guid; AND init `active_layer: None` in
                                             //   Scene::new's struct literal (35), else E0063

    /// Every distinct layer (group) name across ALL docs, with total member count —
    /// `layer list`. A top-level child that isn't an object guid in its doc is a group.
    pub fn layer_names(&self) -> Vec<(String, usize)> {
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for doc in &self.docs {
            let Some(root) = doc.session.tree.root() else { continue };
            for c in root.borrow().children() {
                let n = c.borrow();
                if !doc.session.lookup.contains_key(&n.name) {
                    *counts.entry(n.name.clone()).or_default() += n.descendants().len();
                }
            }
        }
        counts.into_iter().collect()
    }

    /// Union of the named layer's descendant OBJECT guids across EVERY doc that has it —
    /// fed to 46's hidden set. Object guids live in node.NAME (70's identity rule).
    pub fn layer_members(&self, name: &str) -> Vec<String> {
        let mut out = Vec::new();
        for doc in &self.docs {
            if let Some(node) = doc.session.tree.get_node_by_name(name) {
                out.extend(node.borrow().descendants().iter()
                    .map(|n| n.borrow().name.clone()));
            }
        }
        out
    }
```

## Step 2 — creating and assigning: `src/app/scene.rs`

*Finding* a layer folds over all docs; *creating* one cannot — a new group needs a **target doc**.
The find-or-create is doc-scoped (the caller says which — the active doc for new layers, the
object's own doc for assignment):

```rust
    /// Find-or-create the named layer in ONE doc. (The kernel ships Session::find_group, but
    /// it *panics* when the name is missing — probe with get_node_by_name so we can fall
    /// through to creation.)
    pub fn layer_node(&mut self, d: usize, name: &str) -> Rc<RefCell<TreeNode>> {
        if let Some(node) = self.docs[d].session.tree.get_node_by_name(name) {
            return node;
        }
        self.docs[d].session.add_group(name)             // kernel: tree group node
    }

    /// Re-parent the selection under the named layer — each object in its OWN doc (a
    /// selection can span sheets, and a layer exists per doc). Pure document organization:
    /// see the cheap-move note below.
    pub fn assign_to_layer(&mut self, name: &str) {
        for g in self.selected.clone() {
            let Some(&row) = self.guid_to_row.get(&g) else { continue };
            let d = self.doc_of_row(row);
            let layer = self.layer_node(d, name);
            let tree = &mut self.docs[d].session.tree;
            if let Some(node) = tree.get_node_by_name(&g) {   // object guid lives in node.NAME
                tree.remove(&node);                           // detach (its subtree rides along)
                tree.add(&node, Some(&layer));                // Some(_) ALWAYS — add(node, None)
            }                                                 // REPLACES the doc's root!
        }
    }
```

(`Tree::remove` finds the node's parent and unlinks it there — the node keeps its own children, so
detach/re-attach is a genuine move. And the key property is now *provable*, not asserted: rows
were minted from `session.order()` in `add_file` (35), and `order()` walks the **object vectors**
in a fixed type sequence — it never consults the tree. So **a tree move changes no guid and no
row**: draw, pick, selection, and the arena are all untouched. Layers cost nothing.)

New objects land on the active layer with a one-line change where 57's `add_object` parents its
tree node: pass `state.scene.active_layer` — resolved at that moment via
`layer_node(d, name)` with `d` = the target doc (57's `active_doc`) — as the `parent` argument
that `add_mesh`/`add_line`/… always accepted. It was `None` since 57 — the parameter was waiting.

## Step 3 — the verbs: `src/app/commands.rs`

```rust
        "layer" => match (parts.next(), parts.next()) {
            (Some("list"), _) => {
                let layers = state.scene.layer_names();
                if layers.is_empty() { return Dispatch::Instant("no layers".into()); }
                Dispatch::Instant(layers.iter()
                    .map(|(n, c)| format!("{n} ({c})"))
                    .collect::<Vec<_>>().join("\n"))
            }
            (Some("active"), Some(name)) => {
                state.scene.active_layer = Some(name.to_string());   // a NAME — resolved per doc
                Dispatch::Instant(format!("active layer: {name}"))
            }
            (Some(dir @ ("off" | "on")), Some(name)) => {
                // fold over the docs, union the descendants, hand them to 46's set ops
                let hide = dir == "off";                          // bind the token, then read it
                for g in state.scene.layer_members(name) {
                    if hide { state.scene.hidden.insert(g); } else { state.scene.hidden.remove(&g); }
                }
                state.scene.apply_visibility(&mut state.gpu);
                state.poke();
                Dispatch::Instant(format!("layer {name}: {}", if hide { "off" } else { "on" }))
            }
            (Some("active"), None) => Dispatch::Instant("layer active <name>".into()),
            (Some(name), None) => {
                let n = state.scene.selected.len();
                state.scene.assign_to_layer(name);
                state.poke();
                Dispatch::Instant(format!("{n} object(s) → layer {name}"))
            }
            _ => Dispatch::Instant(
                "layer list | layer <name> | layer active <name> | layer off|on <name>".into()),
        }
```

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://127.0.0.1:8770
```

- `layer list` → the *imported* layers, by name, with member counts — Bemassung, Schraffur, … —
  exactly the groups 70's panel shows per sheet, now summed across docs. Nothing was created;
  the import did this.
- `layer off Bemassung` → dimensions vanish from **every sheet at once** — draw AND pick (the
  cross-doc fold feeding 46's one authority). `on` → back. 70's per-sheet branch eyes still work
  independently — same `hidden` set, two entrances. Undo history untouched — visibility isn't a
  document mutation (46's rule).
- Select all beams (marquee), `layer beams` → each owning doc grows a **beams** group holding its
  own; nothing changes on screen (the provable cheap-move property: `order()` never saw the tree).
- `layer active beams`, draw a `line` → its tree row appears **under beams** in the active doc,
  and `layer off beams` takes it with the rest.
- Save (39) → reload → the layer structure round-trips **per doc**: each doc's `.pb` carries its
  own `session.tree`, groups included, as it always did. Nothing new was serialized because
  nothing new exists.

## Recap

```
Ch 80: duplication.
Ch 81: LAYERS = tree groups the PDF importer ALREADY builds (one per OCG layer, per doc) — the
       lesson names them. A layer is a NAME, not a node: the same name is a different group in
       every doc, so every op folds over scene.docs + get_node_by_name (70's identity rule — group
       names AND object guids both live in node.name; find_node_by_guid matches nothing you want,
       and find_group PANICS on a miss). layer list = top-level non-object children, summed;
       off/on = union of descendants' names → scene.hidden → 46; assign = a TREE MOVE in each
       object's own doc (detach keeps the subtree; add(node, None) would replace the ROOT) — and
       provably free: rows came from session.order(), which walks object vectors, never the tree.
       active_layer: Option<String> = a name, resolved per target doc (57's active_doc) into the
       parent argument every add_* had since day one. Round-trips per doc in the .pb for free.
```

Edited: `app/scene.rs` (`active_layer`, `layer_names`, `layer_members`, `layer_node(d, name)`,
`assign_to_layer`), `app/commands.rs` (`layer` verbs).

## Next

`82-measure.md` — `probe` grows up: distance, angle, radius, object info — and a status bar that
always tells you where the cursor is.
