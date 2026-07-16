# 70 Scene tree — the document, as a panel

> **Big picture.** *Phase 12 — scene management UI (70–72).* Everything the viewer knows lives in
> maps and flags; users need it as a **list they can read and poke**: the Session's tree in a side
> panel, an eye icon per row driving 46's visibility, names instead of guids. Two disciplines carry
> the lesson: **virtualize** (build only the visible rows — a 42k-object drawing must scroll like a
> 10-row one) and **the tree renders state, it doesn't own it** — every toggle routes through the
> same `Scene` verbs the CLI uses, so the panel can never drift from the viewport.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the session tree flattens to visible rows only; each row shows an eye toggle and a name; toggles call the same scene verbs as the CLI" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="10" y="14" width="200" height="104" fill="none" stroke="#3a3a3a"/>
  <g fill="#d7dae0" font-size="10">
    <text x="20" y="32">👁 ▸ model</text>
    <text x="30" y="50">👁 ▾ walls</text>
    <text x="44" y="68" fill="#888">👁 wall_north</text>
    <text x="44" y="86" fill="#888">👁 wall_south  ◀ selected</text>
    <text x="30" y="104">👁 ▸ beams (1,204)</text>
  </g>
  <rect x="40" y="76" width="160" height="14" fill="none" stroke="#6fb3ff" stroke-width="1"/>
  <g transform="translate(270,20)">
    <text x="0" y="14" fill="#888">virtualized: only rows in the viewport exist this frame</text>
    <text x="0" y="34" fill="#888">eye → scene.hidden + flags (46's verbs — ONE authority)</text>
    <text x="0" y="54" fill="#888">click → scene.selected (45's verbs)</text>
    <text x="0" y="80" fill="#666" font-size="10">selection style: WHITE bg, BLACK text —</text>
    <text x="0" y="94" fill="#666" font-size="10">never white-on-dark (a real archive regression)</text>
  </g>
</svg>

## Files we touch

```
src/ui/tree.rs   # NEW — flatten visible tree rows; egui show_rows virtualization; eye + select
src/ui/mod.rs    # a left SidePanel hosting it; TreeUi state (expanded set, scroll target)
src/state.rs     # apply collected intents (toggle/select) after the closure — 47's rule
```

## Step 1 — flatten what's visible: `src/ui/tree.rs`

egui is immediate-mode: nested `CollapsingHeader`s would *build every row every frame* — the
anti-virtualization. Instead keep an `expanded: HashSet<String>` (node guids) and flatten the tree to
a `Vec<Row>` of only the rows expansion makes visible, then hand that to `ScrollArea::show_rows`,
which instantiates **only the on-screen slice**:

```rust
pub struct Row {
    pub guid: String,
    pub name: String,
    pub depth: usize,        // indent
    pub is_branch: bool,     // has children → draws the ▸/▾
    pub expanded: bool,
}

/// Walk session.tree depth-first, descending only into expanded branches. 42k objects with
/// everything collapsed = a handful of rows; fully expanded = a big Vec, but show_rows still
/// renders ~40 of them. Names: node name if set, else a short type tag + guid prefix.
pub fn flatten(scene: &Scene,
               expanded: &HashSet<String>) -> Vec<Row> { /* DFS over session.tree */ }
```

(`session.tree` is the kernel's `Tree`/`TreeNode` (Rc<RefCell<…>>) — the same structure
`remove_object` maintains (51), so deleted objects leave the panel automatically. Objects the tree
doesn't parent — 64's nurbs collections — get appended as top-level rows from `all_objects()`; the
every-map rule applies to UI too.)

## Step 2 — the rows, virtualized: `src/ui/tree.rs`

```rust
pub fn tree_panel(ui: &mut egui::Ui, rows: &[Row], scene_sel: &HashSet<String>,
                  scene_hidden: &HashSet<String>, out: &mut TreeIntent) {
    let row_h = 18.0;
    egui::ScrollArea::vertical().show_rows(ui, row_h, rows.len(), |ui, range| {
        for row in &rows[range] {                                       // ← ONLY the visible slice
            ui.horizontal(|ui| {
                ui.add_space(row.depth as f32 * 12.0);
                // right_to_left ordering puts the eye FIRST so long names truncate, not the control
                let visible = !scene_hidden.contains(&row.guid);
                if ui.selectable_label(false, if visible { "👁" } else { "—" }).clicked() {
                    out.toggled.push(row.guid.clone());
                }
                if row.is_branch {
                    let arrow = if row.expanded { "▾" } else { "▸" };
                    if ui.selectable_label(false, arrow).clicked() {
                        out.expand_toggled.push(row.guid.clone());
                    }
                }
                let selected = scene_sel.contains(&row.guid);
                if ui.selectable_label(selected, &row.name).clicked() {
                    out.clicked.push((row.guid.clone(), ui.input(|i| i.modifiers.shift)));
                }
            });
        }
    });
}
```

Two archive rules baked in: the **eye sits before the name** (a long name must truncate, never push
the control out of reach), and the selected row's style is **white background, black text** — 47's
`Visuals` already set `selection.bg_fill`/`override_text_color`; never restyle it white-on-dark (the
archive shipped that once; selected rows became unreadable).

## Step 3 — intents apply after: `src/state.rs`

`TreeIntent { toggled, expand_toggled, clicked }` collects inside the closure (47's rule); `State`
drains it after, through the **existing** verbs — no second authority:

```rust
        for g in intent.toggled {
            if !self.scene.hidden.remove(&g) { self.scene.hidden.insert(g); }
        }
        // 46
        if !intent.toggled_is_empty { self.scene.apply_visibility(&mut self.gpu); self.poke(); }

        for (g, shift) in intent.clicked {
            if shift { /* toggle in scene.selected */ }
            else { self.scene.selected.clear(); self.scene.selected.insert(g); }
        }
        // 45
        self.scene.apply_selection(&mut self.gpu);
        // 52
        self.refresh_gumball();
        for g in intent.expand_toggled { /* toggle in self.ui.tree_expanded */ }
```

Hiding a *branch* hides its subtree — resolve the node's descendant guids during the drain (a small
recursive collect on `session.tree`) and toggle them as a set; the eye shows mixed state (—) when
children disagree.

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Load the stress file and open the panel: **scrolling is smooth** — the frame builds ~40 rows
  whether the flattened list holds 50 or 42,000 (`show_rows`). Expand-all and scroll again: still
  smooth. That's the virtualization contract.
- Click an eye → that object vanishes in the viewport *and* stops picking (46's single authority —
  the panel called the same verb). Toggle a branch → the subtree blinks out as one.
- Click a name → viewport selection + gumball follow (45/52). Shift+click adds. The selected row
  reads black-on-white.
- Delete an object (51) → its row is gone next frame; undo → row returns. The panel was never told —
  it renders `session.tree`, which the Commands already maintain.

## Recap

```
Ch 69: Phase 11 closed — the look, fast.
Ch 70: THE TREE. Flatten session.tree depth-first through an `expanded` set → Vec<Row>; egui
       ScrollArea::show_rows instantiates ONLY the on-screen slice (immediate-mode virtualization —
       nested CollapsingHeaders would rebuild 42k rows/frame). Rows: eye BEFORE name (truncation
       order), ▸/▾ for branches, white-bg/black-text selection (the archive's unreadable-row
       regression, pre-fixed). Panel COLLECTS TreeIntent; State drains it through 46's visibility
       and 45's selection verbs — one authority, no drift; branch-eye resolves descendants as a set.
       Nurbs collections join as top-level rows (every-map, UI edition). Deletes/undo work — the
       panel renders the tree the Commands already maintain.
```

Edited: `ui/tree.rs` (NEW — `flatten`, `tree_panel`, `TreeIntent`), `ui/mod.rs` (SidePanel +
`tree_expanded`), `state.rs` (intent drain through existing verbs).

## Next

`71-tree-viewport.md` — the two views converge: picking in the viewport reveals and scrolls the tree
to the object (auto-reveal); selecting in the tree highlights in the viewport (already true) — plus
double-click-to-zoom. Small lesson, big daily-use payoff.
