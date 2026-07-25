# 81 Layers — organization on rails that already exist

> **Big picture.** *Phase 14.* Ask any CAD user how they keep a model sane: layers — named buckets
> with one-click visibility. The viewer almost has them already: the kernel's tree has **group
> nodes** (`Session::add_group`), the tree panel (70) renders branches with eye icons, and branch
> visibility resolves to 46's set operations. This lesson is mostly *naming the composition* and
> adding the two missing conveniences: a `layer` verb and an **active layer** for new objects.

## Files we touch

```
src/app/scene.rs      # active_layer: Option<String>; assign_to_layer(); layer-aware creation
src/app/commands.rs   # `layer <name>` / `layer off|on <name>` / `layer active <name>`
src/ui/tree.rs        # group rows get a bolder style + object count (cosmetic)
```

## Step 1 — layers ARE group nodes: `src/app/scene.rs`

No new data structure. A layer is a top-level group node in `session.tree`; membership is tree
parentage; visibility is the branch-eye behaviour 70 already built (toggle a branch → 46 hides the
descendant set). What's genuinely new is three small verbs:

```rust
    pub active_layer: Option<String>,        // ← ADD to `struct Scene` — AND init `active_layer: None`
                                             //   in Scene::new's struct literal (35), else E0063

    /// Find-or-create the named layer (a top-level tree group). Kernel API, one call.
    pub fn layer_node(&mut self, name: &str) -> Rc<RefCell<TreeNode>> {
        if let Some(node) = self.find_group_by_name(name) {
            return node;
        }
        self.session.add_group(name)                     // kernel: tree group node
    }

    /// Re-parent the selection under the layer (tree move — guids and rows are untouched,
    /// so the GPU side needs NOTHING: layers are pure document organization).
    pub fn assign_to_layer(&mut self, name: &str) {
        let layer = self.layer_node(name);
        for g in self.selected.clone() {
            if let Some(node) = self.session.tree.find_node_by_guid(&g) {
                self.session.tree.remove(&node);
                self.session.tree.add(&node, Some(&layer));
            }
        }
    }

    /// Top-level layer group by name, or None. (The kernel ships `Session::find_group`, but it
    /// *panics* when the name is missing — we want an Option so `layer_node` can fall through.)
    fn find_group_by_name(&self, name: &str) -> Option<Rc<RefCell<TreeNode>>> {
        let root = self.session.tree.root()?;
        let node = root.borrow().children().into_iter().find(|c| c.borrow().name == name);
        node
    }

    /// The active layer as a `parent` argument for `add_*` (None until `layer active` ran).
    pub fn active_layer_node(&self) -> Option<Rc<RefCell<TreeNode>>> {
        let guid = self.active_layer.clone()?;
        self.session.tree.find_node_by_guid(&guid)
    }

    /// Every object guid under the named layer (its descendants) — fed to 46's hidden set.
    pub fn layer_members(&self, name: &str) -> Vec<String> {
        match self.find_group_by_name(name) {
            Some(layer) => layer.borrow().descendants().iter()
                .map(|n| n.borrow().guid().to_string()).collect(),
            None => Vec::new(),
        }
    }
```

(`find_group_by_name` walks the root's children comparing names — a 5-line helper. Check your
kernel `Tree`'s exact move semantics: if `remove` drops children, detach/re-attach via the node's
parent link instead. The key property: **a tree move changes no guid and no row**, so draw, pick,
selection, and the arena are all untouched — layers cost nothing.)

New objects land on the active layer with a one-line change in 57's tools: `AddGeometry`'s
insert passes `state.scene.active_layer_node().as_ref()` as the `parent` argument that `add_mesh`/
`add_line`/… always accepted (`.as_ref()` because `active_layer_node` returns an owned `Option`
while `add_*` want `Option<&Rc<…>>`). It was `None` since 57 — the parameter was waiting.

## Step 2 — the verbs: `src/app/commands.rs`

```rust
        "layer" => match (parts.next(), parts.next()) {
            (Some("active"), Some(name)) => {
                let node = state.scene.layer_node(name);
                state.scene.active_layer = Some(node.borrow().guid().to_string());
                Dispatch::Instant(format!("active layer: {name}"))
            }
            (Some(dir @ ("off" | "on")), Some(name)) => {
                // resolve the group's descendant guids and hand them to 46's set ops
                let hide = dir == "off";                          // bind the token, then read it
                let members = state.scene.layer_members(name);
                for g in members {
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
            _ => Dispatch::Instant("layer <name> | layer active <name> | layer off|on <name>".into()),
        }
```

## Step 3 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Select all beams (marquee), `layer beams` → the tree grows a **beams** branch holding them;
  nothing changes on screen (pure reorganization — the cheap-move property, confirmed).
- `layer off beams` (or click the branch eye, 70) → every beam vanishes from draw AND pick; `on` →
  back. Undo history is untouched — visibility isn't a document mutation (46's rule).
- `layer active beams`, draw a `line` → its tree row appears **under beams**, and `layer off beams`
  takes it with the rest.
- Save (39) → reload → the layer structure round-trips: it was `session.tree` all along, which the
  `.pb` always carried. Nothing new was serialized because nothing new exists.

## Recap

```
Ch 80: duplication.
Ch 81: LAYERS = tree groups, named. layer_node = find-or-create via Session::add_group;
       assign = a TREE MOVE (no guid, no row, no GPU work — pure document organization); visibility
       = the branch-eye path 70+46 already run; active layer = the parent argument every add_* had
       since day one, finally non-None. Round-trips in the .pb for free because the tree always did.
       One lesson, ~zero new machinery — the reward for the tree, the flags, and the verbs being
       three clean layers themselves.
```

Edited: `app/scene.rs` (`active_layer`, `layer_node`, `assign_to_layer`, `layer_members`),
`app/commands.rs` (`layer`), `ui/tree.rs` (group-row styling).

## Next

`82-measure.md` — `probe` grows up: distance, angle, radius, object info — and a status bar that
always tells you where the cursor is.
