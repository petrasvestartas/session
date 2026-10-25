# 19.3 · Original owners across selection, copying and display

**Steps:** 8 · **Source:** [app/session_io.rs](../../../../src/app/session_io.rs) · [app/command/mod.rs](../../../../src/app/command/mod.rs) · [app/hierarchy.rs](../../../../src/app/hierarchy.rs) · [app/scene.rs](../../../../src/app/scene.rs)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Retained Plane Triad: O + X_axis (R) + Y_axis (G) + Z_axis (B)</text>
  <polygon points="68,136 132,76 268,76 204,136" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <circle cx="168" cy="106" r="4.5" fill="#111111"/>
  <line x1="168" y1="106" x2="236" y2="106" stroke="#dc2626" stroke-width="2.5" marker-end="url(#arrR)"/>
  <line x1="168" y1="106" x2="202" y2="74" stroke="#16a34a" stroke-width="2.5" marker-end="url(#arrG)"/>
  <line x1="168" y1="106" x2="168" y2="48" stroke="#2563eb" stroke-width="2.5" marker-end="url(#arrB)"/>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Family Bitmask: Mesh | BRep | Curve | Point | Plane | Cloud</text>
  <rect x="430" y="56" width="78" height="36" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="469" y="78" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Mesh ✓</text>
  <rect x="520" y="56" width="78" height="36" rx="4" fill="#bbf7d0" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="559" y="78" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">BRep ✓</text>
  <rect x="610" y="56" width="78" height="36" rx="4" fill="#fef08a" stroke="#d97706" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="649" y="78" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Plane ✓</text>
  <rect x="475" y="104" width="168" height="34" rx="4" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="559" y="125" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">w_pen = w_base * scale</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Filtered Geometry Families + Explicit Plane Frame</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="838,136 888,68 938,136" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <polygon points="948,132 978,94 1036,94 1006,132" fill="#fef9c3" stroke="#111111" stroke-width="1.5"/>
  <line x1="992" y1="113" x2="1028" y2="113" stroke="#dc2626" stroke-width="2"/>
  <line x1="992" y1="113" x2="992" y2="74" stroke="#2563eb" stroke-width="2"/>
</svg>
</div>

## <a id="checkpoint-96s-resolve-feature-rows-to-original-element-owners"></a>1. Resolve feature rows to Element owners

### 1. `src/feature_owner.rs`

Create the file and type:

```rust
use crate::{
    hierarchy::Document,
    scene::{ObjectId, Scene},
};

fn ancestor(scene: &Scene, doc: &Document, index: usize) -> Option<ObjectId> {
    let mut parent = doc.nodes[index].parent;
    while let Some(index) = parent {
        let node = &doc.nodes[index];
        if let Some(id) = node.object {
            if scene.objects().iter().any(|row| {
                row.id == id
                    && matches!(
                        row.original.as_ref(),
                        Some(session_rust::Geometry::Element(_))
                    )
            }) {
                return scene.pick_id(id.value());
            }
        }
        parent = node.parent;
    }
    None
}

pub fn row_owner(scene: &Scene, key: &str) -> Option<ObjectId> {
    let (document, guid) = key.split_once('/')?;
    let document = document.parse::<u32>().ok()?;
    let doc = scene
        .forest
        .documents
        .iter()
        .find(|doc| doc.key == document)?;
    let index = doc
        .nodes
        .iter()
        .position(|node| node.source.guid == guid)?;
    if !doc.nodes[index].readonly {
        return None;
    }
    ancestor(scene, doc, index)
}

pub fn highlighted(scene: &Scene, selected: &[ObjectId]) -> Vec<ObjectId> {
    let mut ids = selected.to_vec();
    for doc in &scene.forest.documents {
        for (index, node) in doc.nodes.iter().enumerate() {
            if !node.derived {
                continue;
            }
            let Some(id) = node.object.filter(|id| scene.visible(*id)) else {
                continue;
            };
            if ancestor(scene, doc, index)
                .is_some_and(|owner| selected.contains(&owner))
            {
                ids.push(id);
            }
        }
    }
    ids.sort_unstable_by_key(|id| id.value());
    ids.dedup();
    ids
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
mod command_vocabulary;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod command_vocabulary;
pub mod feature_owner;
#[cfg(test)]
```

## <a id="checkpoint-96t-select-and-highlight-original-feature-owners"></a>2. Select and highlight feature owners

### 1. `src/tree_panel.rs`

**Locate this existing block:**

```rust
                                .all(|id| scene.view.locked.contains(id)),
                        selectable: row
                            .ids
                            .iter()
                            .any(|id| scene.pick_id(id.value()).is_some()),
                        current: crate::layer_edit::current_key(scene).as_ref()
                            == Some(&row.key),
                        has_faces: row.ids.iter().any(|id| {
                            scene
                                .objects()
                                .iter()
                                .any(|mesh| mesh.id == *id)
                        }),
                        colours: [
                            row.ids
                                .iter()
                                .find_map(|id| scene.view.faces.get(id).copied()),
                            row.ids
                                .iter()
                                .find_map(|id| scene.view.edges.get(id).copied()),
                        ],
                        group: crate::layer_edit::locate(scene, &row.key).is_ok(),
                        root: row.depth == 0,
                        holds_current: crate::layer_edit::holds_current(
                            scene, &row.key,
                        ),
                    },
                )
            })
            .collect();
    }
    pub fn rename_result(&mut self, error: Option<&str>) {
        if let Some(error) = error {
            self.rename_error = error.into();
            self.rename_focus = true;
        } else {
            self.rename = None;
            self.rename_error.clear();
        }
    }
    pub fn all(&mut self) {
        self.open = true;
        self.expanded.extend(
            self.rows
                .iter()
                .enumerate()
                .filter(|(index, r)| r.end > *index + 1)
                .map(|(_, r)| r.key.clone()),
        );
        self.page = 0;
    }
    pub fn visible(&self) -> Vec<usize> {
        let mut out = Vec::new();
        let mut index = 0;
        while index < self.rows.len() {
            out.push(index);
            let row = &self.rows[index];
            index = if self.expanded.contains(&row.key) {
                index + 1
            } else {
                row.end.max(index + 1)
            };
        }
        out
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
pub fn draw(
    root: &mut egui::Ui,
    model: &mut TreePanel,
    controls: &mut Option<Vec<Control>>,
) {
    model.rect = None;
    model.menu_rects.clear();
    if !model.open {
        return;
    }
    let visible = model.visible();
    let pages = visible.len().div_ceil(128).max(1);
    model.page = model.page.min(pages - 1);
    let response=egui::Area::new("session-tree".into()).anchor(egui::Align2::RIGHT_TOP,egui::vec2(-8.,8.)).order(egui::Order::Foreground).show(root.ctx(),|ui| {
        egui::Frame::group(ui.style()).fill(egui::Color32::from_rgb(248,248,248)).show(ui,|ui| {
            ui.set_width((root.available_width()/3.).clamp(240.,413.33334));ui.set_height((root.max_rect().height()-32.).max(60.));ui.horizontal(|ui| {ui.strong("Layers");let close=ui.button("×");record(controls,"tree/close","Close Layers",&close);if close.clicked() {model.open=false;}});
            dialogs(ui,model,controls);
            ui.horizontal(|ui|{let button=ui.selectable_label(model.graph_open,format!("Graph ({})",model.graph.len()));record(controls,"tree/graph","Graph",&button);if button.clicked(){model.graph_open=!model.graph_open;}});
            if model.graph_open{egui::ScrollArea::vertical().id_salt("graph-table").max_height(150.).show(ui,|ui|{egui::Grid::new("graph-edges").striped(true).show(ui,|ui|{for title in ["Document","From","To","Attribute"]{ui.strong(title);}ui.end_row();for(doc,from,to,attribute)in model.graph.iter().take(128){for word in [doc,from,to,attribute]{ui.label(word);}ui.end_row();}});if model.graph.len()>128{ui.label(format!("Showing first 128 of {} edges",model.graph.len()));}});}
            if pages>1 {ui.horizontal(|ui| {if ui.button("Previous").clicked() {model.page=model.page.saturating_sub(1);}ui.label(format!("{} / {pages}",model.page+1));if ui.button("Next").clicked() {model.page=(model.page+1).min(pages-1);}});}
            let height=(root.max_rect().height()-100.).max(60.);
            egui::ScrollArea::vertical().max_height(height).show(ui,|ui| {
                for &index in visible.iter().skip(model.page*128).take(128) {
                    let row=model.rows[index].clone();let state=model.states.get(&row.key).copied().unwrap_or_default();ui.push_id(&row.key,|ui|ui.horizontal(|ui| {
                        ui.add_space(row.depth.min(12) as f32*12.);
                        if row.end>index+1 {let arrow=ui.small_button(if model.expanded.contains(&row.key) {"▾"} else {"▸"});record(controls,&format!("tree/open/{}",row.key),"Expand or collapse",&arrow);if arrow.clicked() {if !model.expanded.insert(row.key.clone()) {model.expanded.remove(&row.key);}}} else {ui.add_space(18.);}
                        let selected=row.ids.iter().any(|id|model.selected.contains(id));
                        let label=if row.label.is_empty() {row.ids.first().map_or("Group".into(),|id|format!("Object {}",id.value()))} else {row.label.clone()};
                        let label_width=(ui.available_width()-90.).max(24.);
                        let button=ui.add_enabled_ui(state.group || !row.readonly && state.selectable,|ui|ui.add_sized([label_width,20.],egui::Button::new(format!("{label} ({})",row.ids.len())).selected(selected).truncate())).inner.on_hover_text(&label).on_disabled_hover_text("Source attribute: read only; edit its parent");
                        record(controls,&format!("tree/select/{}",row.key),&label,&button);
                        if button.clicked() && state.selectable {model.action=Some((row.ids.clone(),ui.input(|input|input.modifiers.shift)));}
                        if state.group {if let Some(menu)=egui::Popup::context_menu(&button).show(|ui| {
                            use crate::layer_edit::Edit;
                            for (name,edit,enabled) in [("Set current",Some(Edit::Current),true),("New layer",Some(Edit::New {sublayer:false}),true),("New sublayer",Some(Edit::New {sublayer:true}),true),("Duplicate layer",Some(Edit::Duplicate),!state.root),("Change object layer",Some(Edit::Move(model.selected.clone())),!model.selected.is_empty()),("Copy object layer",Some(Edit::Copy(model.selected.clone())),!model.selected.is_empty()),("Rename",None,!state.root),("Delete",Some(Edit::Delete),!state.root&&!state.holds_current)] {
                                let button=ui.add_enabled(enabled,egui::Button::new(name));record(controls,&format!("layer/menu/{}/{name}",row.key),name,&button);
                                if button.clicked() {
                                    if name=="Rename" {model.rename=Some((row.key.clone(),row.label.clone()));model.rename_error.clear();model.rename_focus=true;}
                                    else if name=="Delete"&&!row.ids.is_empty() {model.confirm=Some((row.key.clone(),row.ids.len()));}
                                    else {model.edit=Some((row.key.clone(),edit.unwrap()));}
                                    ui.close();
                                }
                            }
                        }) {model.menu_rects.push(menu.response.rect);}}
                        properties(ui,&row.key,state,&mut model.property,&mut model.menu_rects,controls);
                    }));
                }
            });
        });
    });
    model.rect = model.open.then_some(response.response.rect);
}

#[derive(Clone, Copy, Default)]
struct RowState {
    hidden: bool,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                                .all(|id| scene.view.locked.contains(id)),
                        selectable: crate::feature_owner::row_owner(scene, &row.key)
                            .is_some()
                            || row
                                .ids
                                .iter()
                                .any(|id| scene.pick_id(id.value()).is_some()),
                        owner: crate::feature_owner::row_owner(scene, &row.key),
                        current: crate::layer_edit::current_key(scene).as_ref()
                            == Some(&row.key),
                        has_faces: row.ids.iter().any(|id| {
                            scene
                                .objects()
                                .iter()
                                .any(|mesh| mesh.id == *id)
                        }),
                        colours: [
                            row.ids
                                .iter()
                                .find_map(|id| scene.view.faces.get(id).copied()),
                            row.ids
                                .iter()
                                .find_map(|id| scene.view.edges.get(id).copied()),
                        ],
                        group: crate::layer_edit::locate(scene, &row.key).is_ok(),
                        root: row.depth == 0,
                        holds_current: crate::layer_edit::holds_current(
                            scene, &row.key,
                        ),
                    },
                )
            })
            .collect();
    }
    pub fn rename_result(&mut self, error: Option<&str>) {
        if let Some(error) = error {
            self.rename_error = error.into();
            self.rename_focus = true;
        } else {
            self.rename = None;
            self.rename_error.clear();
        }
    }
    pub fn all(&mut self) {
        self.open = true;
        self.expanded.extend(
            self.rows
                .iter()
                .enumerate()
                .filter(|(index, r)| r.end > *index + 1)
                .map(|(_, r)| r.key.clone()),
        );
        self.page = 0;
    }
    pub fn visible(&self) -> Vec<usize> {
        let mut out = Vec::new();
        let mut index = 0;
        while index < self.rows.len() {
            out.push(index);
            let row = &self.rows[index];
            index = if self.expanded.contains(&row.key) {
                index + 1
            } else {
                row.end.max(index + 1)
            };
        }
        out
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
pub fn draw(
    root: &mut egui::Ui,
    model: &mut TreePanel,
    controls: &mut Option<Vec<Control>>,
) {
    model.rect = None;
    model.menu_rects.clear();
    if !model.open {
        return;
    }
    let visible = model.visible();
    let pages = visible.len().div_ceil(128).max(1);
    model.page = model.page.min(pages - 1);
    let response=egui::Area::new("session-tree".into()).anchor(egui::Align2::RIGHT_TOP,egui::vec2(-8.,8.)).order(egui::Order::Foreground).show(root.ctx(),|ui| {
        egui::Frame::group(ui.style()).fill(egui::Color32::from_rgb(248,248,248)).show(ui,|ui| {
            ui.set_width((root.available_width()/3.).clamp(240.,413.33334));ui.set_height((root.max_rect().height()-32.).max(60.));ui.horizontal(|ui| {ui.strong("Layers");let close=ui.button("×");record(controls,"tree/close","Close Layers",&close);if close.clicked() {model.open=false;}});
            dialogs(ui,model,controls);
            ui.horizontal(|ui|{let button=ui.selectable_label(model.graph_open,format!("Graph ({})",model.graph.len()));record(controls,"tree/graph","Graph",&button);if button.clicked(){model.graph_open=!model.graph_open;}});
            if model.graph_open{egui::ScrollArea::vertical().id_salt("graph-table").max_height(150.).show(ui,|ui|{egui::Grid::new("graph-edges").striped(true).show(ui,|ui|{for title in ["Document","From","To","Attribute"]{ui.strong(title);}ui.end_row();for(doc,from,to,attribute)in model.graph.iter().take(128){for word in [doc,from,to,attribute]{ui.label(word);}ui.end_row();}});if model.graph.len()>128{ui.label(format!("Showing first 128 of {} edges",model.graph.len()));}});}
            if pages>1 {ui.horizontal(|ui| {if ui.button("Previous").clicked() {model.page=model.page.saturating_sub(1);}ui.label(format!("{} / {pages}",model.page+1));if ui.button("Next").clicked() {model.page=(model.page+1).min(pages-1);}});}
            let height=(root.max_rect().height()-100.).max(60.);
            egui::ScrollArea::vertical().max_height(height).show(ui,|ui| {
                for &index in visible.iter().skip(model.page*128).take(128) {
                    let row=model.rows[index].clone();let state=model.states.get(&row.key).copied().unwrap_or_default();ui.push_id(&row.key,|ui|ui.horizontal(|ui| {
                        ui.add_space(row.depth.min(12) as f32*12.);
                        if row.end>index+1 {let arrow=ui.small_button(if model.expanded.contains(&row.key) {"▾"} else {"▸"});record(controls,&format!("tree/open/{}",row.key),"Expand or collapse",&arrow);if arrow.clicked() {if !model.expanded.insert(row.key.clone()) {model.expanded.remove(&row.key);}}} else {ui.add_space(18.);}
                        let selected=state.owner.map_or_else(||row.ids.iter().any(|id|model.selected.contains(id)),|id|model.selected.contains(&id));
                        let label=if row.label.is_empty() {row.ids.first().map_or("Group".into(),|id|format!("Object {}",id.value()))} else {row.label.clone()};
                        let label_width=(ui.available_width()-90.).max(24.);
                        let button=ui.add_enabled_ui(state.group || state.owner.is_some() || !row.readonly && state.selectable,|ui|ui.add_sized([label_width,20.],egui::Button::new(format!("{label} ({})",row.ids.len())).selected(selected).truncate())).inner.on_hover_text(&label).on_disabled_hover_text("Source attribute: read only; edit its parent");
                        record(controls,&format!("tree/select/{}",row.key),&label,&button);
                        if button.clicked() && state.selectable {model.action=Some((state.owner.map_or_else(||row.ids.clone(),|id|vec![id]),ui.input(|input|input.modifiers.shift)));}
                        if state.group {if let Some(menu)=egui::Popup::context_menu(&button).show(|ui| {
                            use crate::layer_edit::Edit;
                            for (name,edit,enabled) in [("Set current",Some(Edit::Current),true),("New layer",Some(Edit::New {sublayer:false}),true),("New sublayer",Some(Edit::New {sublayer:true}),true),("Duplicate layer",Some(Edit::Duplicate),!state.root),("Change object layer",Some(Edit::Move(model.selected.clone())),!model.selected.is_empty()),("Copy object layer",Some(Edit::Copy(model.selected.clone())),!model.selected.is_empty()),("Rename",None,!state.root),("Delete",Some(Edit::Delete),!state.root&&!state.holds_current)] {
                                let button=ui.add_enabled(enabled,egui::Button::new(name));record(controls,&format!("layer/menu/{}/{name}",row.key),name,&button);
                                if button.clicked() {
                                    if name=="Rename" {model.rename=Some((row.key.clone(),row.label.clone()));model.rename_error.clear();model.rename_focus=true;}
                                    else if name=="Delete"&&!row.ids.is_empty() {model.confirm=Some((row.key.clone(),row.ids.len()));}
                                    else {model.edit=Some((row.key.clone(),edit.unwrap()));}
                                    ui.close();
                                }
                            }
                        }) {model.menu_rects.push(menu.response.rect);}}
                        properties(ui,&row.key,state,&mut model.property,&mut model.menu_rects,controls);
                    }));
                }
            });
        });
    });
    model.rect = model.open.then_some(response.response.rect);
}

#[derive(Clone, Copy, Default)]
struct RowState {
    owner: Option<ObjectId>,
    hidden: bool,
```

### 2. `src/editor.rs`

**Locate this existing block:**

```rust
            .map_or_else(
                || self.selected_objects(),
                |g| {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            .map_or_else(
                || {
                    crate::feature_owner::highlighted(
                        &self.scene,
                        &self.selected_objects(),
                    )
                },
                |g| {
```

### 3. `src/feature_owner_tests.rs`

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    scene::ObjectId,
};
use std::rc::Rc;

fn imported() -> Editor {
    let mut editor = Editor::default();
    editor.scene = crate::scene::Scene::empty();
    crate::scene_document::import(
        &mut editor.scene,
        &crate::interaction_example::bytes(),
        "fixture",
        &session_rust::Xform::identity(),
        0.,
    )
    .unwrap();
    editor
}
fn owner(editor: &Editor, name: &str) -> ObjectId {
    editor
        .scene
        .objects()
        .iter()
        .find(|row| row.metadata.name == name)
        .unwrap()
        .id
}
fn piece_key(editor: &Editor, parent: ObjectId) -> String {
    crate::hierarchy::rows(&editor.scene)
        .into_iter()
        .find(|row| {
            row.label.starts_with("piece ")
                && crate::feature_owner::row_owner(&editor.scene, &row.key)
                    == Some(parent)
        })
        .unwrap()
        .key
}

#[test]
fn feature_rows_target_the_nearest_original_and_respect_owner_visibility_and_locks() {
    let mut e = imported();
    e.apply(Action::ElementAttributes(Some(true)))
        .unwrap();
    let host = owner(&e, "host");
    let child = owner(&e, "child");
    let host_key = piece_key(&e, host);
    let child_key = piece_key(&e, child);
    assert_eq!(
        crate::feature_owner::row_owner(&e.scene, &host_key),
        Some(host)
    );
    assert_eq!(
        crate::feature_owner::row_owner(&e.scene, &child_key),
        Some(child)
    );
    for row in crate::hierarchy::rows(&e.scene)
        .into_iter()
        .filter(|row| row.readonly)
    {
        assert!(crate::feature_owner::row_owner(&e.scene, &row.key).is_some());
    }
    Rc::make_mut(&mut e.scene.view).hide(&[host], true);
    assert_eq!(
        crate::feature_owner::row_owner(&e.scene, &host_key),
        None
    );
    assert_eq!(
        crate::feature_owner::row_owner(&e.scene, &child_key),
        Some(child)
    );
    Rc::make_mut(&mut e.scene.view).hide(&[host], false);
    Rc::make_mut(&mut e.scene.view).lock(&[host], true);
    assert_eq!(
        crate::feature_owner::row_owner(&e.scene, &host_key),
        None
    );
    assert_eq!(
        crate::feature_owner::row_owner(&e.scene, "missing/key"),
        None
    );
}

#[test]
fn display_highlighting_stops_at_independent_children_and_never_widens_editing_selection()
 {
    let mut e = imported();
    e.apply(Action::ElementAttributes(Some(true)))
        .unwrap();
    let host = owner(&e, "host");
    let child = owner(&e, "child");
    e.apply(Action::TreePick {
        ids: vec![host],
        additive: false,
    })
    .unwrap();
    assert_eq!(e.selected_objects(), [host]);
    let highlight = e.highlighted_objects();
    assert_eq!(highlight.len(), 6);
    assert!(!highlight.contains(&child));
    let child_piece = crate::hierarchy::rows(&e.scene)
        .into_iter()
        .find(|row| {
            row.label.starts_with("piece ")
                && crate::feature_owner::row_owner(&e.scene, &row.key) == Some(child)
        })
        .unwrap()
        .ids[0];
    assert!(!highlight.contains(&child_piece));
    e.apply(Action::ElementInteractions(Some(false)))
        .unwrap();
    assert_eq!(e.highlighted_objects().len(), 2);
    assert_eq!(e.selected_objects(), [host]);
    assert!(e.gizmo_origin().is_some());
}

#[test]
fn feature_owner_delete_and_undo_restore_original_and_generated_paths_once() {
    let mut e = imported();
    let host = owner(&e, "host");
    let child = owner(&e, "child");
    let source = match e
        .scene
        .objects()
        .iter()
        .find(|row| row.id == host)
        .unwrap()
        .original
        .as_ref()
        .unwrap()
    {
        session_rust::Geometry::Element(source) => source.clone(),
        _ => panic!("Element expected"),
    };
    let key = piece_key(&e, host);
    let target = crate::feature_owner::row_owner(&e.scene, &key).unwrap();
    e.apply(Action::TreePick {
        ids: vec![target],
        additive: false,
    })
    .unwrap();
    assert_eq!(e.selected_objects(), [host]);
    e.apply(Action::Delete).unwrap();
    assert!(!e.scene.contains(host));
    assert!(!e.scene.contains(child));
    assert_eq!(e.scene.paths().len(), 0);
    e.apply(Action::Undo).unwrap();
    assert!(e.scene.contains(host));
    assert!(e.scene.contains(child));
    assert_eq!(e.scene.paths().len(), 6);
    let restored = match e
        .scene
        .objects()
        .iter()
        .find(|row| row.id == host)
        .unwrap()
        .original
        .as_ref()
        .unwrap()
    {
        session_rust::Geometry::Element(source) => source,
        _ => panic!("Element expected"),
    };
    assert!(Rc::ptr_eq(restored, &source));
    assert_eq!(
        crate::feature_owner::row_owner(&e.scene, &key),
        Some(host)
    );
}
```

### 4. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
#[path = "command_dock/tool_buttons_tests.rs"]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod feature_owner_tests;
#[cfg(test)]
#[path = "command_dock/tool_buttons_tests.rs"]
```

### 5. `src/editor.rs`

**Locate this existing block:**

```rust
                self.select_objects(eligible, additive);
                return self.apply(Action::FitSelected);
            }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                self.select_objects(eligible, additive);
                self.apply(Action::FitSelected)?;
                return Ok(Change::Preview);
            }
```

## <a id="checkpoint-96u-fit-original-box-corners-to-the-free-canvas"></a>3. Fit box corners to the free canvas

### 1. `src/camera.rs`

**Locate this existing block:**

```rust
    pub fn fit(&mut self, bounds: &crate::bounds::Bounds) {
        let half_y = (HALF_FOV.tan() * (self.frame[3] - self.frame[1])).atan();
        let half_x =
            (self.aspect * HALF_FOV.tan() * (self.frame[2] - self.frame[0])).atan();
        let radius = bounds.radius();
        self.radius = if radius > 0.0 {
            radius
        } else {
            1.0
        };
        self.target = bounds.centre();
        let opening = match self.projection {
            Projection::Perspective => half_x.min(half_y).sin(),
            Projection::Orthographic => half_y.tan().min(half_x.tan()),
        };
        self.distance = self.radius * 1.1 / opening;
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn fit(&mut self, bounds: &crate::bounds::Bounds) {
        if bounds
            .min
            .iter()
            .chain(&bounds.max)
            .any(|v| !v.is_finite())
            || (0..3).any(|i| bounds.min[i] > bounds.max[i])
        {
            return;
        }
        let centre = bounds.centre();
        let radius = bounds.radius();
        if !radius.is_finite() || centre.iter().any(|v| !v.is_finite()) {
            return;
        }
        if radius == 0.0 {
            self.target = centre;
            return;
        }
        let tx = self.aspect * HALF_FOV.tan() * (self.frame[2] - self.frame[0]);
        let ty = HALF_FOV.tan() * (self.frame[3] - self.frame[1]);
        if !tx.is_finite() || tx <= 0.0 || !ty.is_finite() || ty <= 0.0 {
            return;
        }
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());
        let up = self
            .orientation
            .rotate_vector(Vector::z_axis());
        let forward = self
            .orientation
            .rotate_vector(Vector::y_axis());
        let mut distance: f64 = 0.0;
        for corner in 0..8 {
            let p: [f64; 3] = std::array::from_fn(|i| {
                let coordinate = if corner & (1 << i) == 0 {
                    bounds.min[i]
                } else {
                    bounds.max[i]
                };
                coordinate - centre[i]
            });
            let dot = |axis: &Vector| (0..3).map(|i| p[i] * axis[i]).sum::<f64>();
            let z = dot(&forward);
            distance = distance.max(dot(&right).abs() / tx + z);
            distance = distance.max(dot(&up).abs() / ty + z);
        }
        self.target = centre;
        self.radius = radius;
        self.distance = (distance * 1.05).max(1.0e-6);
    }
```

### 2. `src/projected_fit_tests.rs`

Create the file and type:

```rust
use crate::{
    bounds::Bounds,
    camera::{Camera, Projection, View},
};
use session_rust::Point;

#[test]
fn fit_frames_all_box_corners_inside_the_panel_free_rectangle() {
    for projection in [
        Projection::Perspective,
        Projection::Orthographic,
    ] {
        for view in [
            View::Front,
            View::Top,
            View::Iso,
            View::IsoBack,
        ] {
            let mut camera = Camera::default();
            camera.set_view(view);
            camera.projection = projection;
            camera.aspect = 0.75;
            camera.frame([0.25, 0.1, 0.95, 0.85]);
            let bounds = Bounds {
                min: [1_000_000.125, -30.0, 10.0],
                max: [1_000_600.125, 10.0, 15.0],
            };
            camera.fit(&bounds);
            let matrix = camera.view_projection();
            for corner in 0..8 {
                let p: [f64; 3] = std::array::from_fn(|i| {
                    if corner & (1 << i) == 0 {
                        bounds.min[i]
                    } else {
                        bounds.max[i]
                    }
                });
                let screen = matrix.transform_point(&Point::new(p[0], p[1], p[2]));
                assert!(screen[0] >= 2.0 * camera.frame[0] - 1.0 - 1e-8);
                assert!(screen[0] <= 2.0 * camera.frame[2] - 1.0 + 1e-8);
                assert!(screen[1] >= 1.0 - 2.0 * camera.frame[3] - 1e-8);
                assert!(screen[1] <= 1.0 - 2.0 * camera.frame[1] + 1e-8);
                assert!((0.0..=1.0).contains(&screen[2]));
            }
            assert_eq!(bounds.min[0], 1_000_000.125);
        }
    }
}

#[test]
fn thin_boxes_fill_the_frame_without_a_sphere_margin() {
    let mut camera = Camera::default();
    camera.set_view(View::Front);
    let bounds = Bounds {
        min: [-100.0, -0.1, -0.1],
        max: [100.0, 0.1, 0.1],
    };
    camera.fit(&bounds);
    let a = camera
        .view_projection()
        .transform_point(&Point::new(-100.0, 0.0, 0.0));
    let b = camera
        .view_projection()
        .transform_point(&Point::new(100.0, 0.0, 0.0));
    assert!(b[0] - a[0] > 1.8);
    let before = (camera.target, camera.radius, camera.distance);
    camera.fit(&Bounds {
        min: [f64::NAN, 0.0, 0.0],
        max: [1.0; 3],
    });
    assert_eq!(
        (camera.target, camera.radius, camera.distance),
        before
    );
    camera.fit(&Bounds::point([7.0; 3]));
    assert_eq!(camera.target, [7.0; 3]);
    assert_eq!(
        (camera.radius, camera.distance),
        (before.1, before.2)
    );
}
```

### 3. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
#[path = "command_dock/tool_buttons_tests.rs"]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod projected_fit_tests;
#[cfg(test)]
#[path = "command_dock/tool_buttons_tests.rs"]
```

## <a id="checkpoint-96v-project-definition-and-per-instance-features"></a>4. Project definition and per-instance features

### 1. `src/instance_features.rs`

Create the file and type:

```rust
use crate::{
    instance_plan::Owner,
    scene::{ObjectId, Scene},
};
use session_rust::{Element, Geometry};

pub fn append(
    scene: &mut Scene,
    id: ObjectId,
    owner: &Owner,
) -> Result<(), &'static str> {
    let mut display = Element::new(&owner.instance.name);
    display.set_guid(owner.instance.guid().to_owned());
    if let Geometry::Element(definition) = &owner.definition {
        display
            .features
            .extend(definition.features.iter().cloned());
    }
    display
        .features
        .extend(owner.instance.features.iter().cloned());
    crate::feature_display::append(scene, id, &owner.document, &display)
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod range_shape;

pub mod instance_plan;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod range_shape;

mod instance_features;
pub mod instance_plan;
```

### 3. `src/scene_document.rs`

**Locate this existing block:**

```rust
        .filter_map(|(guid, id, _)| {
            matches!(
                source.document.lookup.get(guid),
                Some(session_rust::Geometry::Element(_))
            )
            .then_some(*id)
        })
        .collect::<std::collections::HashSet<_>>();
    candidate
        .forest
        .classify_elements(key, &elements);
    for (guid, id, _) in &owners {
        if let Some(session_rust::Geometry::Element(element)) =
            source.document.lookup.get(guid)
        {
            crate::feature_display::append(
                &mut candidate,
                *id,
                &source.document,
                element,
            )
            .map_err(str::to_owned)?;
        }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .filter_map(|(guid, id, _)| {
            (matches!(
                source.document.lookup.get(guid),
                Some(session_rust::Geometry::Element(_))
            ) || candidate.instances.contains_key(id))
            .then_some(*id)
        })
        .collect::<std::collections::HashSet<_>>();
    candidate
        .forest
        .classify_elements(key, &elements);
    for (guid, id, _) in &owners {
        if let Some(session_rust::Geometry::Element(element)) =
            source.document.lookup.get(guid)
        {
            crate::feature_display::append(
                &mut candidate,
                *id,
                &source.document,
                element,
            )
            .map_err(str::to_owned)?;
        }
    }
    for (_, id, _) in &owners {
        if let Some(owner) = candidate.instances.get(id).cloned() {
            crate::instance_features::append(&mut candidate, *id, &owner)
                .map_err(str::to_owned)?;
        }
```

### 4. `src/feature_owner.rs`

**Locate this existing block:**

```rust
        if let Some(id) = node.object {
            if scene.objects().iter().any(|row| {
                row.id == id
                    && matches!(
                        row.original.as_ref(),
                        Some(session_rust::Geometry::Element(_))
                    )
            }) {
                return scene.pick_id(id.value());
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        if let Some(id) = node.object {
            if scene.instances.contains_key(&id)
                || scene.objects().iter().any(|row| {
                    row.id == id
                        && matches!(
                            row.original.as_ref(),
                            Some(session_rust::Geometry::Element(_))
                        )
                })
            {
                return scene.pick_id(id.value());
```

### 5. `src/instance_feature_tests.rs`

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    scene::Scene,
};
use prost::Message;
use session_rust::{Element, Point, Polyline, Xform, element::ElementFeature, proto};

fn message() -> proto::Session {
    let mut source = crate::instance_specimen::message();
    let mesh = session_rust::Mesh::from_proto(
        source.definitions.as_ref().unwrap().meshes[0].clone(),
    );
    let mut definition = Element::new("prototype");
    definition.set_guid("prototype".into());
    definition.set_geometry(mesh);
    let outline = || {
        Polyline::new(vec![
            Point::new(-0.2, 0., 0.),
            Point::new(0.2, 0., 0.),
        ])
    };
    let mut attribute =
        ElementFeature::new("axis", -1, vec![outline()], "prototype axis");
    attribute.set_guid("prototype-axis".into());
    definition.features.push(attribute);
    source.definitions = Some(proto::Objects {
        elements: vec![definition.to_proto()],
        ..Default::default()
    });
    for (i, instance) in source
        .objects
        .as_mut()
        .unwrap()
        .instances
        .iter_mut()
        .enumerate()
    {
        instance.definition_guid = "prototype".into();
        let mut feature = ElementFeature::new(
            "cut",
            -1,
            vec![outline()],
            &format!("cut {i}"),
        );
        feature.set_guid(format!("instance-feature-{i}"));
        instance.features.push(feature.to_proto());
    }
    source
}

fn same_definition(actual: &proto::Element, expected: &proto::Element) {
    assert_eq!(
        proto::Mesh::decode(actual.geometry_data.as_slice()).unwrap(),
        proto::Mesh::decode(expected.geometry_data.as_slice()).unwrap()
    );
    let mut a = actual.clone();
    let mut b = expected.clone();
    a.geometry_data.clear();
    b.geometry_data.clear();
    assert_eq!(a, b);
}

#[test]
fn instance_projection_keeps_shared_sources_and_independent_feature_owners() {
    let source = message();
    let mut e = Editor::default();
    e.scene = Scene::empty();
    crate::scene_document::import(
        &mut e.scene,
        &source.encode_to_vec(),
        "instances",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    assert_eq!(e.scene.instances.len(), 2);
    assert_eq!(e.scene.paths().len(), 4);
    let ids = e
        .scene
        .objects()
        .iter()
        .map(|row| row.id)
        .collect::<Vec<_>>();
    let (session_rust::Geometry::Element(a), session_rust::Geometry::Element(b)) = (
        &e.scene.instances[&ids[0]].definition,
        &e.scene.instances[&ids[1]].definition,
    ) else {
        panic!("Element definitions")
    };
    assert!(std::rc::Rc::ptr_eq(a, b));
    assert_eq!(
        crate::feature_owner::highlighted(&e.scene, &[ids[0]]).len(),
        2
    );
    e.apply(Action::ElementAttributes(Some(true)))
        .unwrap();
    assert_eq!(
        crate::feature_owner::highlighted(&e.scene, &[ids[0]]).len(),
        3
    );
    for path in e.scene.paths() {
        let doc = &e.scene.forest.documents[0];
        let index = doc
            .nodes
            .iter()
            .position(|node| node.object == Some(path.id))
            .unwrap();
        let key = format!(
            "{}/{}",
            doc.key, doc.nodes[index].source.guid
        );
        let owner = crate::feature_owner::row_owner(&e.scene, &key);
        let host = if path.model.m[12] == e.scene.placement(ids[0]).unwrap().m[12] {
            ids[0]
        } else {
            ids[1]
        };
        assert_eq!(
            path.model.m,
            e.scene.placement(host).unwrap().m
        );
        assert_eq!(
            owner,
            (host == ids[0]).then_some(host),
            "locked instance does not become editable"
        );
    }
    let original = e.scene.instances[&ids[0]]
        .document
        .to_proto();
    same_definition(
        &original.definitions.unwrap().elements[0],
        &source.definitions.as_ref().unwrap().elements[0],
    );
    assert_eq!(
        original.objects.unwrap().instances[0].features,
        source.objects.as_ref().unwrap().instances[0].features
    );
}

#[test]
fn saved_instances_regenerate_features_without_archiving_display_paths() {
    let source = message();
    let mut e = Editor::default();
    e.scene = Scene::empty();
    crate::scene_document::import(
        &mut e.scene,
        &source.encode_to_vec(),
        "instances",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    let first = e.scene.objects()[0].id;
    e.apply(Action::TreePick {
        ids: vec![first],
        additive: false,
    })
    .unwrap();
    e.apply(Action::Delete).unwrap();
    assert_eq!(e.scene.paths().len(), 2);
    e.apply(Action::Undo).unwrap();
    assert_eq!(e.scene.paths().len(), 4);
    for _ in 0..3 {
        let saved = crate::scene_archive::save(&e.scene).unwrap();
        let bundle = crate::archive::Bundle::decode(&saved).unwrap();
        let document = proto::Session::decode(bundle.documents[0].as_slice()).unwrap();
        assert!(
            document
                .objects
                .as_ref()
                .unwrap()
                .polylines
                .is_empty()
        );
        assert_eq!(
            document
                .objects
                .as_ref()
                .unwrap()
                .instances
                .len(),
            2
        );
        same_definition(
            &document
                .definitions
                .as_ref()
                .unwrap()
                .elements[0],
            &source.definitions.as_ref().unwrap().elements[0],
        );
        crate::scene_document::open(&mut e.scene, &saved).unwrap();
        assert_eq!(e.scene.paths().len(), 4);
    }
}
```

### 6. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod projected_fit_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod instance_feature_tests;
#[cfg(test)]
mod projected_fit_tests;
```

## <a id="checkpoint-96w-detach-copied-original-source-identity"></a>5. Detach copied source identity

### 1. `src/scene.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        if let Some(owner) = self.instances.get(&id).cloned() {
            self.instances.insert(copy, owner);
        }
        self.next_id = next;
```

Replace it with:

```rust
        if let Some(owner) = self.instances.get(&id).cloned() {
            self.instances.insert(copy, owner);
        }
        self.detach_copy_identity(copy);
        self.next_id = next;
```

**Locate this block:**

```rust
    }

    pub(crate) fn copy_row(
        &mut self,
        id: ObjectId,
```

Replace it with:

```rust
    }

    fn detach_copy_identity(&mut self, id: ObjectId) {
        let instance = self.instances.contains_key(&id);
        if let Some(row) = self
            .objects
            .iter_mut()
            .find(|row| row.id == id)
        {
            let element = !instance
                && matches!(
                    row.original,
                    Some(session_rust::Geometry::Element(_))
                );
            if element {
                if let Some(session_rust::Geometry::Element(source)) =
                    row.original.as_mut()
                {
                    Rc::make_mut(source).set_guid(row.guid.clone());
                }
            }
            if instance || element {
                Rc::make_mut(&mut row.metadata).source_guid = row.guid.clone();
            }
        }
        let guid = self.row_guid(id).map(str::to_owned);
        if let (Some(guid), Some(owner)) = (guid, self.instances.get_mut(&id)) {
            Rc::make_mut(&mut owner.instance).set_guid(guid);
        }
    }

    pub(crate) fn copy_row(
        &mut self,
        id: ObjectId,
```

## <a id="checkpoint-96x-regenerate-display-for-copied-feature-owners"></a>6. Regenerate display for copied feature owners

### 1. `src/layer_edit.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    for (owner, tree) in trees.into_iter().enumerate() {
        replace(scene, owner, tree);
    }
    Ok(copies)
```

Replace it with:

```rust
    for (owner, tree) in trees.into_iter().enumerate() {
        replace(scene, owner, tree);
    }
    project_copies(scene, &copies)?;
    Ok(copies)
```

**Locate this block:**

```rust
            };
            let parent = node.parent.ok_or("Object has no parent")?;
            let mut tree = branch(doc, index);
            tree.children.retain(|child| {
                child.node.object.is_none() && child.node.source.name == "attributes"
            });
            sources.push((
                owner,
                doc.nodes[parent].source.guid.clone(),
```

Replace it with:

```rust
            };
            let parent = node.parent.ok_or("Object has no parent")?;
            let mut tree = branch(doc, index);
            copy_attributes(scene, &mut tree);
            sources.push((
                owner,
                doc.nodes[parent].source.guid.clone(),
```

**Locate this block:**

```rust
}

pub(crate) fn copy_objects(
    scene: &mut Scene,
    ids: &[crate::scene::ObjectId],
```

Replace it with:

```rust
}

fn copy_attributes(scene: &Scene, tree: &mut Branch) {
    tree.children.retain(|group| {
        group.node.object.is_none() && group.node.source.name == "attributes"
    });
    for group in &mut tree.children {
        group.children.retain(|child| {
            child.node.object.is_some_and(|id| {
                !child.node.derived
                    && !scene.instances.contains_key(&id)
                    && !scene.sheets().iter().any(|row| row.id == id)
                    && !scene.texts().iter().any(|row| row.id == id)
            })
        });
        for child in &mut group.children {
            child.children.clear();
        }
    }
}

fn project_copies(
    scene: &mut Scene,
    ids: &[crate::scene::ObjectId],
) -> Result<(), &'static str> {
    let detached = session_rust::Session::new("copies");
    let owners = scene.forest.descendants(ids);
    for id in owners {
        if let Some(owner) = scene.instances.get(&id).cloned() {
            crate::instance_features::append(scene, id, &owner)?;
        } else {
            let original = scene
                .objects()
                .iter()
                .find(|row| row.id == id)
                .and_then(|row| row.original.clone());
            if let Some(session_rust::Geometry::Element(element)) = original {
                crate::feature_display::append(scene, id, &detached, &element)?;
            }
        }
    }
    Ok(())
}

pub(crate) fn copy_objects(
    scene: &mut Scene,
    ids: &[crate::scene::ObjectId],
```

### 2. `src/copy_feature_tests.rs`

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    scene::Scene,
};
use prost::Message;
use session_rust::{Geometry, Xform, proto};
use std::rc::Rc;

fn same_source(actual: &proto::Element, expected: &proto::Element) {
    assert_eq!(
        proto::Mesh::decode(actual.geometry_data.as_slice()).unwrap(),
        proto::Mesh::decode(expected.geometry_data.as_slice()).unwrap()
    );
    let mut a = actual.clone();
    let mut b = expected.clone();
    a.geometry_data.clear();
    b.geometry_data.clear();
    assert_eq!(a, b);
}

#[test]
fn copied_elements_get_fresh_original_identity_and_regenerate_only_their_own_display() {
    let mut e = Editor::default();
    e.scene = Scene::empty();
    crate::scene_document::import(
        &mut e.scene,
        &crate::interaction_example::bytes(),
        "source",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    let host = e
        .scene
        .objects()
        .iter()
        .find(|row| row.metadata.name == "host")
        .unwrap()
        .id;
    let original = match e
        .scene
        .objects()
        .iter()
        .find(|row| row.id == host)
        .unwrap()
        .original
        .as_ref()
        .unwrap()
    {
        Geometry::Element(source) => source.clone(),
        _ => panic!("Element"),
    };
    let before = original.to_proto();
    let child = e
        .scene
        .objects()
        .iter()
        .find(|row| row.metadata.name == "child")
        .unwrap()
        .id;
    let connector = e
        .scene
        .objects()
        .iter()
        .find(|row| row.metadata.name == "connector")
        .unwrap()
        .id;
    crate::layer_edit::attach_attributes(
        &mut e.scene,
        child,
        vec![connector],
        "attributes",
    )
    .unwrap();
    let ids = crate::layer_edit::copy_objects(
        &mut e.scene,
        &[host],
        &Xform::translation(4., 0., 0.),
    )
    .unwrap();
    assert_eq!(ids.len(), 1);
    let copied = ids[0];
    let row = e
        .scene
        .objects()
        .iter()
        .find(|row| row.id == copied)
        .unwrap();
    let Geometry::Element(copy) = row.original.as_ref().unwrap() else {
        panic!("Copied Element")
    };
    assert!(!Rc::ptr_eq(&original, copy));
    assert_eq!(copy.guid(), row.guid);
    assert_ne!(copy.guid(), original.guid());
    assert_eq!(
        copy.features
            .iter()
            .map(|f| f.to_proto())
            .collect::<Vec<_>>(),
        before.features
    );
    same_source(&original.to_proto(), &before);
    assert_eq!(e.scene.objects().len(), 5);
    assert_eq!(e.scene.paths().len(), 12);
    assert_eq!(
        e.scene
            .objects()
            .iter()
            .filter(|row| row.metadata.name == "connector")
            .count(),
        1,
        "Copy keeps direct source attributes, not their nested tree descendants"
    );
    let tree = &e.scene.forest.documents[0];
    let index = tree
        .nodes
        .iter()
        .position(|node| node.object == Some(copied))
        .unwrap();
    let children = &tree.nodes[index + 1..tree.nodes[index].end];
    assert_eq!(
        children
            .iter()
            .filter(|node| node.derived && node.object.is_some())
            .count(),
        6
    );
    assert_eq!(
        children
            .iter()
            .filter(|node| !node.derived && node.object.is_some())
            .count(),
        1
    );
    assert!(
        children
            .iter()
            .all(|node| node.label != "InteractionContactFace seam")
    );
    for node in children
        .iter()
        .filter(|node| node.derived && node.object.is_some())
    {
        let path = e
            .scene
            .paths()
            .iter()
            .find(|row| Some(row.id) == node.object)
            .unwrap();
        let owner = crate::feature_owner::row_owner(
            &e.scene,
            &format!("{}/{}", tree.key, node.source.guid),
        )
        .unwrap();
        assert!(
            owner == copied
                || e.scene
                    .objects()
                    .iter()
                    .any(|row| row.id == owner && row.metadata.name == "child")
        );
        assert_eq!(
            path.model.m,
            e.scene.placement(owner).unwrap().m
        );
    }
    let placements = e
        .scene
        .objects()
        .iter()
        .map(|row| (row.guid.clone(), row.model.m))
        .collect::<std::collections::HashMap<_, _>>();
    for _ in 0..3 {
        let saved = crate::scene_archive::save(&e.scene).unwrap();
        let archive = crate::archive::Bundle::decode(&saved).unwrap();
        let session = proto::Session::decode(archive.documents[0].as_slice()).unwrap();
        assert_eq!(
            session
                .objects
                .as_ref()
                .unwrap()
                .elements
                .len(),
            5
        );
        assert!(
            session
                .objects
                .as_ref()
                .unwrap()
                .polylines
                .is_empty()
        );
        crate::scene_document::open(&mut e.scene, &saved).unwrap();
        assert_eq!(e.scene.objects().len(), 5);
        assert_eq!(e.scene.paths().len(), 12);
        for row in e.scene.objects() {
            assert_eq!(row.model.m, placements[&row.guid]);
        }
    }
}

#[test]
fn copied_instances_keep_one_definition_and_separate_reference_identity() {
    let mut e = Editor::default();
    e.scene = Scene::empty();
    crate::scene_document::import(
        &mut e.scene,
        include_bytes!("../assets/pb/course-instance-features.pb"),
        "instances",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    let first = e.scene.objects()[0].id;
    let original = e.scene.instances[&first].clone();
    let copied = crate::layer_edit::copy_objects(
        &mut e.scene,
        &[first],
        &Xform::translation(2., 0., 0.),
    )
    .unwrap()[0];
    let owner = &e.scene.instances[&copied];
    assert!(!Rc::ptr_eq(
        &original.instance,
        &owner.instance
    ));
    assert_eq!(
        owner.instance.guid(),
        e.scene.row_guid(copied).unwrap()
    );
    assert_ne!(
        original.instance.guid(),
        owner.instance.guid()
    );
    let (Geometry::Element(a), Geometry::Element(b)) =
        (&original.definition, &owner.definition)
    else {
        panic!("Element definitions")
    };
    assert!(Rc::ptr_eq(a, b));
    assert_eq!(e.scene.paths().len(), 6);
    e.apply(Action::ElementAttributes(Some(true)))
        .unwrap();
    assert_eq!(
        crate::feature_owner::highlighted(&e.scene, &[copied]).len(),
        3
    );
    let saved = crate::scene_archive::save(&e.scene).unwrap();
    let archive = crate::archive::Bundle::decode(&saved).unwrap();
    let session = proto::Session::decode(archive.documents[0].as_slice()).unwrap();
    assert_eq!(
        session.definitions.unwrap().elements.len(),
        1
    );
    assert_eq!(session.objects.unwrap().instances.len(), 3);
    crate::scene_document::open(&mut e.scene, &saved).unwrap();
    assert_eq!(e.scene.paths().len(), 6);
}
```

### 3. `src/lib.rs`

**Locate this existing block:**

```rust
mod command_vocabulary;
pub mod feature_owner;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod command_vocabulary;
#[cfg(test)]
mod copy_feature_tests;
pub mod feature_owner;
```

## <a id="checkpoint-96y-address-feature-pixels-to-original-cad-owners"></a>7. Address feature pixels to CAD owners

### 1. `src/feature_owner.rs`

**Locate this existing block:**

```rust
}

pub fn row_owner(scene: &Scene, key: &str) -> Option<ObjectId> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub fn display_owner(scene: &Scene, id: ObjectId) -> Option<ObjectId> {
    if !scene.forest.derived(id) {
        return scene.pick_id(id.value());
    }
    for doc in &scene.forest.documents {
        if let Some(index) = doc
            .nodes
            .iter()
            .position(|node| node.object == Some(id))
        {
            return ancestor(scene, doc, index);
        }
    }
    None
}

pub fn row_owner(scene: &Scene, key: &str) -> Option<ObjectId> {
```

### 2. `src/pick_sources.rs`

**Locate this existing block:**

```rust
            [
                scene
                    .pick_id(owner.value())
                    .map_or(0, ObjectId::value),
                tag,
            ]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            [
                crate::feature_owner::display_owner(scene, owner)
                    .map_or(0, ObjectId::value),
                if scene.forest.derived(owner) {
                    0
                } else {
                    tag
                },
            ]
```

### 3. `src/feature_pick_tests.rs`

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    pick_sources::{Mode, Part, Sources, Target},
    scene::Scene,
};
use session_rust::Xform;
use std::rc::Rc;

#[test]
fn derived_path_pick_addresses_target_only_original_owners() {
    let mut e = Editor::default();
    e.scene = Scene::empty();
    crate::scene_document::import(
        &mut e.scene,
        &crate::interaction_example::bytes(),
        "source",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    e.apply(Action::ElementAttributes(Some(true)))
        .unwrap();
    let table = Sources::new(&e.scene).unwrap();
    let mut offset = 0;
    for path in e
        .scene
        .paths()
        .iter()
        .filter(|row| e.scene.visible(row.id))
    {
        let owner = crate::feature_owner::display_owner(&e.scene, path.id).unwrap();
        assert_ne!(owner, path.id);
        for pair in path
            .points()
            .windows(2)
            .filter(|pair| pair[0].to_f32() != pair[1].to_f32())
        {
            let _ = pair;
            assert_eq!(table.path_ids[offset], [owner.value(), 0]);
            assert_eq!(
                table.resolve(
                    &e.scene,
                    table.path_ids[offset],
                    Mode::Object
                ),
                Some(Target {
                    owner,
                    part: Part::Object
                })
            );
            assert_eq!(
                table.resolve(&e.scene, table.path_ids[offset], Mode::Edge),
                None
            );
            offset += 1;
        }
    }
    assert_eq!(offset, 6);
    let child = e
        .scene
        .objects()
        .iter()
        .find(|row| row.metadata.name == "child")
        .unwrap()
        .id;
    let child_path = e
        .scene
        .paths()
        .iter()
        .find(|row| {
            crate::feature_owner::display_owner(&e.scene, row.id) == Some(child)
        })
        .unwrap()
        .id;
    Rc::make_mut(&mut e.scene.view).lock(&[child], true);
    assert_eq!(
        crate::feature_owner::display_owner(&e.scene, child_path),
        None
    );
    assert!(
        Sources::new(&e.scene)
            .unwrap()
            .path_ids
            .iter()
            .any(|ids| *ids == [0, 0])
    );
    Rc::make_mut(&mut e.scene.view).lock(&[child], false);
    e.select_objects([child], false);
    e.apply(Action::Hide).unwrap();
    assert!(!e.scene.visible(child_path));
}

#[test]
fn tree_original_targets_preserve_complete_feature_presentation_membership() {
    use crate::layer_properties::{Property, apply};
    let mut e = Editor::default();
    e.scene = Scene::empty();
    crate::scene_document::import(
        &mut e.scene,
        &crate::interaction_example::bytes(),
        "source",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    e.apply(Action::ElementAttributes(Some(true)))
        .unwrap();
    let child = e
        .scene
        .objects()
        .iter()
        .find(|row| row.metadata.name == "child")
        .unwrap()
        .id;
    let source = e
        .scene
        .objects()
        .iter()
        .map(|row| row.geometry().unwrap().to_proto())
        .collect::<Vec<_>>();
    let rows = crate::hierarchy::rows(&e.scene);
    let parent = rows
        .iter()
        .find(|row| row.label == "child")
        .unwrap();
    assert_eq!(parent.ids, [child]);
    assert!(parent.members.len() > parent.ids.len());
    assert!(
        rows.iter()
            .filter(|row| !row.readonly)
            .flat_map(|row| &row.ids)
            .all(|id| !e.scene.forest.derived(*id))
    );
    let pieces = rows
        .iter()
        .filter(|row| {
            row.label.starts_with("piece ")
                && crate::feature_owner::row_owner(&e.scene, &row.key) == Some(child)
        })
        .collect::<Vec<_>>();
    assert_eq!(pieces.len(), 1);
    let piece = pieces[0];
    assert!(piece.readonly);
    assert_eq!(piece.ids, piece.members);
    assert_eq!(piece.ids.len(), 1);
    assert!(e.scene.forest.derived(piece.ids[0]));
    assert!(parent.members.contains(&piece.ids[0]));
    e.apply(Action::TreePick {
        ids: parent.ids.clone(),
        additive: false,
    })
    .unwrap();
    assert_eq!(e.selected_objects(), [child]);
    assert!(
        e.highlighted_objects()
            .contains(&piece.ids[0])
    );
    apply(
        &mut e.scene,
        &parent.key,
        Property::Locked(true),
    )
    .unwrap();
    assert!(
        parent
            .members
            .iter()
            .all(|id| e.scene.view.locked.contains(id))
    );
    assert_eq!(
        crate::feature_owner::display_owner(&e.scene, piece.ids[0]),
        None
    );
    apply(
        &mut e.scene,
        &parent.key,
        Property::Locked(false),
    )
    .unwrap();
    assert!(
        parent
            .members
            .iter()
            .all(|id| !e.scene.view.locked.contains(id))
    );
    assert_eq!(
        crate::feature_owner::display_owner(&e.scene, piece.ids[0]),
        Some(child)
    );
    for edge in [false, true] {
        apply(
            &mut e.scene,
            &parent.key,
            Property::Colour {
                edge,
                rgb: Some([12, 34, 56]),
            },
        )
        .unwrap();
        let colours = if edge {
            &e.scene.view.edges
        } else {
            &e.scene.view.faces
        };
        assert!(
            parent
                .members
                .iter()
                .all(|id| colours.get(id) == Some(&[12, 34, 56]))
        );
    }
    apply(
        &mut e.scene,
        &parent.key,
        Property::Hidden(true),
    )
    .unwrap();
    assert!(
        parent
            .members
            .iter()
            .all(|id| !e.scene.visible(*id))
    );
    assert_eq!(
        crate::feature_owner::row_owner(&e.scene, &piece.key),
        None
    );
    apply(
        &mut e.scene,
        &parent.key,
        Property::Hidden(false),
    )
    .unwrap();
    assert!(
        parent
            .members
            .iter()
            .all(|id| e.scene.visible(*id))
    );
    assert_eq!(
        crate::feature_owner::row_owner(&e.scene, &piece.key),
        Some(child)
    );
    assert_eq!(e.selected_objects(), [child]);
    assert_eq!(
        e.scene
            .objects()
            .iter()
            .map(|row| row.geometry().unwrap().to_proto())
            .collect::<Vec<_>>(),
        source
    );
}
```

### 4. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod instance_feature_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod feature_pick_tests;
#[cfg(test)]
mod instance_feature_tests;
```

### 5. `src/pick_gpu_tests.rs`

**Locate this existing block:**

```rust
#[test]
fn actual_integer_pick_preserves_high_ids_depth_and_locked_occlusion() {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[test]
fn actual_projected_feature_pixel_selects_original_element_not_display_path() {
    let mut scene = Scene::empty();
    let mut element = session_rust::Element::new("feature pick");
    element.set_geometry(session_rust::Mesh::create_box(0.2, 0.2, 0.2));
    let outline = session_rust::Polyline::new(vec![
        session_rust::Point::new(-0.8, 0.4, 0.2),
        session_rust::Point::new(0.8, 0.4, 0.2),
    ]);
    element.add_feature(session_rust::element::ElementFeature::new(
        "cut",
        0,
        vec![outline],
        "pick line",
    ));
    let original = Rc::new(element);
    let owner = crate::element_original::insert(&mut scene, original.clone()).unwrap();
    crate::feature_display::append(
        &mut scene,
        owner,
        &session_rust::Session::new("test"),
        &original,
    )
    .unwrap();
    assert_eq!(scene.paths().len(), 1);
    let path = scene.paths()[0].id;
    let mut renderer = renderer(&scene);
    renderer
        .request_pick([0., 0.4], identity())
        .unwrap();
    let pixel = finish(&mut renderer).unwrap();
    assert_eq!(pixel, [owner.value(), 0]);
    assert_ne!(pixel[0], path.value());
    assert_eq!(
        renderer.resolve_pick(&scene, pixel),
        Some(crate::pick_sources::Target {
            owner,
            part: crate::pick_sources::Part::Object
        })
    );
    Rc::make_mut(&mut scene.view).lock(&[owner], true);
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([0., 0.4], identity())
        .unwrap();
    assert_eq!(finish(&mut renderer), None);
}

#[test]
fn actual_integer_pick_preserves_high_ids_depth_and_locked_occlusion() {
```

### 6. `src/hierarchy.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            depth: 0,
            end: index + 1,
            ids: vec![id],
            readonly: false,
```

Replace it with:

```rust
            depth: 0,
            end: index + 1,
            ids: vec![id],
            members: vec![id],
            readonly: false,
```

**Locate this block:**

```rust
            depth: 0,
            end: index + 1,
            ids: vec![row.id],
            readonly: false,
        });
    }
```

Replace it with:

```rust
            depth: 0,
            end: index + 1,
            ids: vec![row.id],
            members: vec![row.id],
            readonly: false,
        });
    }
```

**Locate this block:**

```rust
                depth: node.depth,
                end,
                ids,
                readonly: node.readonly,
            });
        }
```

Replace it with:

```rust
                depth: node.depth,
                end,
                ids,
                members,
                readonly: node.readonly,
            });
        }
```

**Locate this block:**

```rust
        let start = rows.len();
        for &index in &visible {
            let node = &doc.nodes[index];
            let ids = doc.nodes[index..node.end]
                .iter()
                .filter_map(|n| n.object)
                .filter(|id| scene.contains(*id))
                .collect::<Vec<_>>();
```

Replace it with:

```rust
        let start = rows.len();
        for &index in &visible {
            let node = &doc.nodes[index];
            let members = doc.nodes[index..node.end]
                .iter()
                .filter_map(|n| n.object)
                .filter(|id| scene.contains(*id))
                .collect::<Vec<_>>();
            let ids = doc.nodes[index..node.end]
                .iter()
                .filter(|n| node.readonly || !n.derived)
                .filter_map(|n| n.object)
                .filter(|id| scene.contains(*id))
                .collect::<Vec<_>>();
```

**Locate this block:**

```rust
    pub ids: Vec<ObjectId>,
    pub readonly: bool,
}
pub fn rows(scene: &Scene) -> Vec<Row> {
```

Replace it with:

```rust
    pub ids: Vec<ObjectId>,
    pub members: Vec<ObjectId>,
    pub readonly: bool,
}
pub fn rows(scene: &Scene) -> Vec<Row> {
```

### 7. `src/tree_panel.rs`

**Locate this existing block:**

```rust
                        hidden: scene.view.lamps.contains(&row.key)
                            || !row.ids.is_empty()
                                && row
                                    .ids
                                    .iter()
                                    .all(|id| scene.view.hidden.contains(id)),
                        locked: !row.ids.is_empty()
                            && row
                                .ids
                                .iter()
                                .all(|id| scene.view.locked.contains(id)),
                        selectable: crate::feature_owner::row_owner(scene, &row.key)
                            .is_some()
                            || row
                                .ids
                                .iter()
                                .any(|id| scene.pick_id(id.value()).is_some()),
                        owner: crate::feature_owner::row_owner(scene, &row.key),
                        current: crate::layer_edit::current_key(scene).as_ref()
                            == Some(&row.key),
                        has_faces: row.ids.iter().any(|id| {
                            scene
                                .objects()
                                .iter()
                                .any(|mesh| mesh.id == *id)
                        }),
                        colours: [
                            row.ids
                                .iter()
                                .find_map(|id| scene.view.faces.get(id).copied()),
                            row.ids
                                .iter()
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                        hidden: scene.view.lamps.contains(&row.key)
                            || !row.members.is_empty()
                                && row
                                    .members
                                    .iter()
                                    .all(|id| scene.view.hidden.contains(id)),
                        locked: !row.members.is_empty()
                            && row
                                .members
                                .iter()
                                .all(|id| scene.view.locked.contains(id)),
                        selectable: crate::feature_owner::row_owner(scene, &row.key)
                            .is_some()
                            || row
                                .ids
                                .iter()
                                .any(|id| scene.pick_id(id.value()).is_some()),
                        owner: crate::feature_owner::row_owner(scene, &row.key),
                        current: crate::layer_edit::current_key(scene).as_ref()
                            == Some(&row.key),
                        has_faces: row.members.iter().any(|id| {
                            scene
                                .objects()
                                .iter()
                                .any(|mesh| mesh.id == *id)
                        }),
                        colours: [
                            row.members
                                .iter()
                                .find_map(|id| scene.view.faces.get(id).copied()),
                            row.members
                                .iter()
```

### 8. `src/layer_properties.rs`

**Locate this existing block:**

```rust
            } else {
                view.hide(&row.ids, on);
            }
            view.refresh(&scene.forest);
        }
        Property::Locked(on) => view.lock(&row.ids, on),
        Property::Colour { edge, rgb } => {
            let colours = if edge {
                &mut view.edges
            } else {
                &mut view.faces
            };
            for id in &row.ids {
                if let Some(rgb) = rgb {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            } else {
                view.hide(&row.members, on);
            }
            view.refresh(&scene.forest);
        }
        Property::Locked(on) => view.lock(&row.members, on),
        Property::Colour { edge, rgb } => {
            let colours = if edge {
                &mut view.edges
            } else {
                &mut view.faces
            };
            for id in &row.members {
                if let Some(rgb) = rgb {
```

## <a id="checkpoint-96z-preserve-source-feature-visibility-and-pixel-styles"></a>8. Preserve source feature visibility and pixel styles

### 1. `src/feature_display.rs`

**Locate this existing block:**

```rust
                let feature = &element.features[index];
                let mut outlines = Vec::new();
                for (part, outline) in feature.outlines.iter().enumerate() {
                    let mut source = outline.clone();
                    source.set_guid(uuid::Uuid::new_v4().to_string());
                    source.name = format!("piece {}", part + 1);
                    source.linecolor = session_rust::Color::black();
                    let piece = scene.insert_path(
                        crate::chain::PreparedChain::polyline(Rc::new(source))?,
                    )?;
                    scene.place_following(piece, model.clone())?;
                    if !crate::feature_plan::interaction(feature) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                let feature = &element.features[index];
                if !feature.visible {
                    continue;
                }
                let mut outlines = Vec::new();
                for (part, outline) in feature.outlines.iter().enumerate() {
                    let point = outline.get_point(0).filter(|p| {
                        outline.coords.chunks_exact(3).all(|q| {
                            [q[0] as f32, q[1] as f32, q[2] as f32] == p.to_f32()
                        })
                    });
                    let piece = if let Some(mut point) = point {
                        point.set_guid(uuid::Uuid::new_v4().to_string());
                        point.name = format!("piece {}", part + 1);
                        point.pointcolor = outline.linecolor.clone();
                        point.width = 12.0;
                        point.is_visible = true;
                        point.is_locked = false;
                        let piece = scene.insert_point(
                            crate::marker::PreparedPoint::new(Rc::new(point))?,
                        )?;
                        scene.place_point(piece, model.clone())?;
                        piece
                    } else {
                        let mut source = outline.clone();
                        source.set_guid(uuid::Uuid::new_v4().to_string());
                        source.name = format!("piece {}", part + 1);
                        source.width = 2.0;
                        source.is_visible = true;
                        source.is_locked = false;
                        source.arrowhead = session_rust::Arrowhead::NONE;
                        let piece = scene.insert_path(
                            crate::chain::PreparedChain::polyline(Rc::new(source))?,
                        )?;
                        scene.place_following(piece, model.clone())?;
                        piece
                    };
                    if !crate::feature_plan::interaction(feature) {
```

### 2. `src/feature_style_tests.rs`

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    scene::Scene,
};
use prost::Message;
use session_rust::element::ElementFeature;
use session_rust::{Element, Geometry, Mesh, Point, Polyline, Session, Xform};

#[test]
fn original_feature_visibility_colour_and_point_styles_survive_projection() {
    let mut element = Element::new("styled");
    element.set_geometry(Mesh::create_box(2., 2., 2.));
    let mut line = Polyline::new(vec![
        Point::new(3., 0., 0.),
        Point::new(5., 0., 0.),
    ]);
    line.width = 7.;
    line.is_visible = false;
    line.is_locked = true;
    line.linecolor = session_rust::Color::new(0.2, 0.6, 0.8, 1.);
    line.arrowhead = session_rust::Arrowhead::END;
    element.add_feature(ElementFeature::new(
        "axis",
        -1,
        vec![line.clone()],
        "axis",
    ));
    let mut hidden = ElementFeature::new("cut", -1, vec![line.clone()], "hidden");
    hidden.visible = false;
    element.add_feature(hidden);
    let mut dot = line.clone();
    dot.coords = vec![6., 1., 0., 6., 1., 0.];
    element.add_feature(ElementFeature::new(
        "drill",
        -1,
        vec![dot],
        "dot",
    ));
    element.add_feature(ElementFeature::new(
        "cut",
        -1,
        vec![line.clone()],
        "visible cut",
    ));
    let mut session = Session::new("styles");
    let node = session.add_element(element, None);
    let guid = node.borrow().name.clone();
    session.xforms.insert(
        guid.clone(),
        Xform::translation(10., 20., 30.),
    );
    let bytes = session.to_proto().encode_to_vec();
    let mut editor = Editor::default();
    editor.scene = Scene::empty();
    crate::scene_document::import(
        &mut editor.scene,
        &bytes,
        "styles",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    assert_eq!(editor.scene.paths().len(), 2);
    assert_eq!(editor.scene.points().len(), 1);
    let path = &editor.scene.paths()[0];
    assert_eq!(path.prepared.width, 2.);
    assert_eq!(path.prepared.colour, [0.2, 0.6, 0.8, 1.]);
    assert_eq!(path.prepared.heads, 0);
    assert!(!editor.scene.visible(path.id));
    assert!(
        editor
            .scene
            .visible(editor.scene.paths()[1].id),
        "Feature visibility and the original owner govern displays, not standalone outline flags"
    );
    let point = &editor.scene.points()[0];
    assert_eq!(point.marker().diameter, 12.);
    assert_eq!(point.marker().center, [16., 21., 30.]);
    assert_eq!(point.marker().colour, [0.2, 0.6, 0.8, 1.]);
    let owner = editor.scene.objects()[0].id;
    assert_eq!(
        crate::feature_owner::display_owner(&editor.scene, point.id),
        Some(owner)
    );
    editor
        .apply(Action::ElementAttributes(Some(true)))
        .unwrap();
    assert!(
        editor
            .scene
            .visible(editor.scene.paths()[0].id)
    );
    assert_eq!(
        editor.scene.paths().len(),
        2,
        "a global toggle cannot resurrect an invisible source feature"
    );
    let retained = editor.scene.objects()[0]
        .original
        .as_ref()
        .unwrap();
    let Geometry::Element(original) = retained else {
        panic!("original Element");
    };
    assert!(!original.features[1].visible);
    assert_eq!(original.features[0].outlines[0].width, 7.);
    assert!(!original.features[0].outlines[0].is_visible);
    assert!(original.features[0].outlines[0].is_locked);
    assert_eq!(
        original.features[0].outlines[0].arrowhead,
        session_rust::Arrowhead::END
    );
    assert_eq!(
        original.features[0].outlines[0]
            .linecolor
            .to_f32(),
        [0.2, 0.6, 0.8, 1.]
    );
}
```

### 3. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod instance_feature_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod feature_style_tests;
#[cfg(test)]
mod instance_feature_tests;
```

### 4. `src/pick_gpu_tests.rs`

**Locate this existing block:**

```rust
#[test]
fn actual_integer_pick_preserves_high_ids_depth_and_locked_occlusion() {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[test]
fn actual_feature_point_disc_uses_original_owner_and_css_diameter() {
    let mut scene = Scene::empty();
    let mut element = session_rust::Element::new("feature dot");
    element.set_geometry(session_rust::Mesh::create_box(0.2, 0.2, 0.2));
    let mut dot = session_rust::Polyline::new(vec![
        session_rust::Point::new(0.5, 0.4, 0.2),
        session_rust::Point::new(0.5, 0.4, 0.2),
    ]);
    dot.linecolor = session_rust::Color::new(0.9, 0.1, 0.1, 1.);
    element.add_feature(session_rust::element::ElementFeature::new(
        "drill",
        -1,
        vec![dot],
        "dot",
    ));
    let original = Rc::new(element);
    let owner = crate::element_original::insert(&mut scene, original.clone()).unwrap();
    crate::feature_display::append(
        &mut scene,
        owner,
        &session_rust::Session::new("dot"),
        &original,
    )
    .unwrap();
    assert_eq!(scene.points().len(), 1);
    let mut renderer = renderer(&scene);
    for density in [1., 2.] {
        renderer.resize(Viewport {
            width: 320,
            height: 240,
        });
        renderer.set_density(density);
        let image = colour_pixels(&mut renderer);
        let pixels: Vec<_> = image
            .chunks_exact(4)
            .enumerate()
            .filter_map(|(index, pixel)| {
                let (x, y) = (index % 320, index / 320);
                (x > 210
                    && (40..105).contains(&y)
                    && u16::from(pixel[0]) > 2 * u16::from(pixel[1])
                    && u16::from(pixel[0]) > 2 * u16::from(pixel[2]))
                .then_some((x, y))
            })
            .collect();
        assert!(!pixels.is_empty());
        for axis in [0, 1] {
            let values: Vec<_> = pixels
                .iter()
                .map(|&(x, y)| {
                    if axis == 0 {
                        x
                    } else {
                        y
                    }
                })
                .collect();
            let span = values.iter().max().unwrap() - values.iter().min().unwrap() + 1;
            assert!(
                (span as f32 - 12. * density).abs() <= 1.,
                "DPR {density}, axis {axis}, physical span {span}"
            );
        }
        // Picking searches the already-taught six-CSS-pixel neighbourhood around the disc.
        for (offset, expected) in [
            (0., Some([owner.value(), 0])),
            (5., Some([owner.value(), 0])),
            (13., None),
        ] {
            renderer
                .request_pick(
                    [0.5 + 2. * offset * density / 320., 0.4],
                    identity(),
                )
                .unwrap();
            assert_eq!(
                finish(&mut renderer),
                expected,
                "DPR {density}, CSS offset {offset}"
            );
        }
    }
}

#[test]
fn actual_integer_pick_preserves_high_ids_depth_and_locked_occlusion() {
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 19.3
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 19.3
npm --prefix ../session_tests run course -- save 19.3
```

Continue with [19.4 · Family-aware original controls and source previews](../../consolidation/96za-address-original-controls-with-family-keys.md).
