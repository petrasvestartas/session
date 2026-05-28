use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;
use session_rust::{Session, TreeNode};

pub const ICON_W:   f32 = 18.0;
pub const ROW_H:    f32 = 18.0;
const INDENT_W: f32 = 14.0;
const ARROW_W:  f32 = 12.0;

/// Collect all leaf GUIDs under a named top-level group in the session tree.
pub fn collect_group_leaf_guids(session: &Session, group_name: &str) -> Vec<String> {
    let Some(root) = session.tree.root() else { return vec![]; };
    for child in root.borrow().children() {
        if child.borrow().name == group_name {
            return collect_tree_leaf_guids_from_lookup(&child, session);
        }
    }
    vec![]
}

pub fn collect_tree_leaf_guids_from_lookup(node: &Rc<RefCell<TreeNode>>, session: &Session) -> Vec<String> {
    let name = node.borrow().name.clone();
    if session.lookup.contains_key(&name) {
        return vec![name];
    }
    let children = node.borrow().children();
    let mut out = vec![];
    for c in &children {
        out.extend(collect_tree_leaf_guids_from_lookup(c, session));
    }
    out
}

/// DFS leaf collection keyed by vmap presence (used by egui tree for V-button and selection state).
pub fn collect_leaf_guids(node: &Rc<RefCell<TreeNode>>, vmap: &HashMap<String, String>) -> Vec<String> {
    let borrowed = node.borrow();
    if vmap.contains_key(&borrowed.name) {
        return vec![borrowed.name.clone()];
    }
    let children = borrowed.children();
    drop(borrowed);
    let mut result = Vec::new();
    for child in &children {
        result.extend(collect_leaf_guids(child, vmap));
    }
    result
}

/// Pre-populate leaf_guid_cache for all group nodes under `node` (call before egui run).
pub fn populate_leaf_cache(
    node: &Rc<RefCell<TreeNode>>,
    vmap: &HashMap<String, String>,
    cache: &mut HashMap<String, Vec<String>>,
) {
    let name = node.borrow().name.clone();
    if vmap.contains_key(&name) { return; }
    let uid = node.borrow().guid().to_string();
    if !cache.contains_key(&uid) {
        cache.insert(uid, collect_leaf_guids(node, vmap));
    }
    let children = node.borrow().children();
    for child in &children {
        populate_leaf_cache(child, vmap, cache);
    }
}

/// Collect all leaf GUIDs under a tree node (checks geom_guid_set for O(1) leaf test).
pub fn collect_group_leaves(node: &Rc<RefCell<TreeNode>>, geom_guids: &HashSet<String>) -> Vec<String> {
    let name = node.borrow().name.clone();
    if geom_guids.contains(&name) {
        return vec![name];
    }
    let children = node.borrow().children();
    let mut result = Vec::new();
    for child in &children {
        result.extend(collect_group_leaves(child, geom_guids));
    }
    result
}

/// DFS: find innermost locked group ancestor containing `target` guid.
pub fn find_locked_ancestor_impl(
    node: &Rc<RefCell<TreeNode>>,
    target: &str,
    group_locked: &HashSet<String>,
    geom_guids: &HashSet<String>,
    current_locked: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Option<Rc<RefCell<TreeNode>>>> {
    let name = node.borrow().name.clone();
    if geom_guids.contains(&name) {
        return if name == target { Some(current_locked) } else { None };
    }
    let uid = node.borrow().guid().to_string();
    let new_locked = if group_locked.contains(&uid) && current_locked.is_none() {
        Some(node.clone())
    } else {
        current_locked
    };
    let children = node.borrow().children();
    for child in &children {
        if let Some(r) = find_locked_ancestor_impl(child, target, group_locked, geom_guids, new_locked.clone()) {
            return Some(r);
        }
    }
    None
}

pub fn locked_group_for_guid(
    root: &Rc<RefCell<TreeNode>>,
    target: &str,
    group_locked: &HashSet<String>,
    geom_guids: &HashSet<String>,
) -> Option<Rc<RefCell<TreeNode>>> {
    find_locked_ancestor_impl(root, target, group_locked, geom_guids, None).flatten()
}

/// O(1) geometry leaf check via pre-computed set.
pub fn is_geom_leaf(name: &str, geom_guids: &HashSet<String>) -> bool {
    geom_guids.contains(name)
}

/// True if every child is either a geometry leaf or a group whose children are all leaves (max 2 levels).
pub fn is_shallow_element(node: &Rc<RefCell<TreeNode>>, geom_guids: &HashSet<String>) -> bool {
    let children = node.borrow().children();
    if children.is_empty() { return false; }
    children.iter().all(|child| {
        let cn = child.borrow().name.clone();
        if is_geom_leaf(&cn, geom_guids) { return true; }
        let grandchildren = child.borrow().children();
        !grandchildren.is_empty() && grandchildren.iter().all(|gc| {
            is_geom_leaf(&gc.borrow().name.clone(), geom_guids)
        })
    })
}

/// Auto-lock groups at the element level: first group in each branch whose subtree is all geometry.
pub fn auto_lock_leaf_groups(
    node: &Rc<RefCell<TreeNode>>,
    geom_guids: &HashSet<String>,
    group_locked: &mut HashSet<String>,
) {
    let name = node.borrow().name.clone();
    if is_geom_leaf(&name, geom_guids) { return; }
    let children = node.borrow().children();
    let leaf_count: usize = children.iter().map(|c| {
        let cn = c.borrow().name.clone();
        if is_geom_leaf(&cn, geom_guids) { 1 }
        else { c.borrow().children().iter().filter(|gc| is_geom_leaf(&gc.borrow().name.clone(), geom_guids)).count() }
    }).sum();
    if leaf_count >= 2 && is_shallow_element(node, geom_guids) {
        group_locked.insert(node.borrow().guid().to_string());
    } else {
        for child in &children {
            auto_lock_leaf_groups(child, geom_guids, group_locked);
        }
    }
}

fn icon_btn(ui: &mut egui::Ui, symbol: &str, tooltip: &str) -> bool {
    ui.add_sized(
        [ICON_W, ROW_H],
        egui::Button::new(egui::RichText::new(symbol).monospace()).frame(false),
    )
    .on_hover_text(tooltip)
    .clicked()
}

fn vis_btn(ui: &mut egui::Ui, visible: bool, tooltip: &str) -> bool {
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(ICON_W, ROW_H), egui::Sense::click());
    let p = ui.painter();
    let c = rect.center();
    if visible {
        p.circle_filled(c, 4.5, egui::Color32::BLACK);
    } else {
        p.circle_stroke(c, 4.5, egui::Stroke::new(1.5, egui::Color32::BLACK));
    }
    resp.on_hover_text(tooltip).clicked()
}

/// Returns Some(Some(new_color)) on change, Some(None) on clear (right-click), None if unchanged.
fn color_btn(
    ui: &mut egui::Ui,
    color: Option<[f32; 4]>,
    id: egui::Id,
    tooltip: &str,
) -> Option<Option<[f32; 4]>> {
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(ICON_W, ROW_H), egui::Sense::click());
    let p = ui.painter();
    let c = rect.center();
    if let Some([r, g, b, _]) = color {
        let col = egui::Color32::from_rgb((r*255.0) as u8, (g*255.0) as u8, (b*255.0) as u8);
        p.circle_filled(c, 4.5, col);
        p.circle_stroke(c, 4.5, egui::Stroke::new(0.8, egui::Color32::from_gray(80)));
    } else {
        let gray = egui::Color32::from_gray(180);
        p.circle_stroke(c, 4.5, egui::Stroke::new(1.0, gray));
        let o = 2.5_f32;
        let sw = egui::Stroke::new(1.0, gray);
        p.line_segment([egui::pos2(c.x-o, c.y-o), egui::pos2(c.x+o, c.y+o)], sw);
        p.line_segment([egui::pos2(c.x+o, c.y-o), egui::pos2(c.x-o, c.y+o)], sw);
    }
    let resp = resp.on_hover_text(tooltip);
    if resp.secondary_clicked() { return Some(None); }
    if resp.clicked() {
        #[allow(deprecated)]
        ui.memory_mut(|m| m.toggle_popup(id));
        let init = color.unwrap_or([0.8, 0.8, 0.8, 1.0]);
        let init_c = egui::Color32::from_rgb((init[0]*255.0) as u8, (init[1]*255.0) as u8, (init[2]*255.0) as u8);
        ui.ctx().data_mut(|d| { if d.get_temp::<egui::Color32>(id).is_none() { d.insert_temp(id, init_c); } });
    }
    let mut result = None;
    #[allow(deprecated)]
    egui::popup_below_widget(ui, id, &resp, egui::PopupCloseBehavior::CloseOnClickOutside, |ui| {
        let init = color.unwrap_or([0.8, 0.8, 0.8, 1.0]);
        let init_c = egui::Color32::from_rgb((init[0]*255.0) as u8, (init[1]*255.0) as u8, (init[2]*255.0) as u8);
        let mut col = ui.ctx().data_mut(|d| d.get_temp::<egui::Color32>(id).unwrap_or(init_c));
        if egui::color_picker::color_picker_color32(ui, &mut col, egui::color_picker::Alpha::Opaque) {
            let (r, g, b) = (col.r() as f32/255.0, col.g() as f32/255.0, col.b() as f32/255.0);
            result = Some(Some([r, g, b, 1.0]));
        }
        ui.ctx().data_mut(|d| d.insert_temp(id, col));
        ui.separator();
        if ui.button("Reset to default").clicked() {
            result = Some(None);
            #[allow(deprecated)]
            ui.memory_mut(|m| m.close_popup(id));
        }
    });
    result
}

fn row_line(ui: &mut egui::Ui, rect: egui::Rect) {
    let color = ui.visuals().widgets.noninteractive.bg_stroke.color;
    ui.painter().hline(rect.x_range(), rect.bottom(), egui::Stroke::new(0.5, color));
}

pub fn render_tree_node(
    ui: &mut egui::Ui,
    node: &Rc<RefCell<TreeNode>>,
    vmap: &HashMap<String, String>,
    selected: &HashSet<String>,
    hidden: &HashSet<String>,
    group_locked: &HashSet<String>,
    transform_locked: &HashSet<String>,
    face_colors: &HashMap<String, [f32; 4]>,
    pt_colors: &HashMap<String, [f32; 4]>,
    new_sel: &mut Option<(Vec<String>, bool)>,
    vis_chg: &mut Vec<(String, bool)>,
    lock_chg: &mut Vec<(String, bool)>,
    transform_lock_chg: &mut Vec<(String, bool)>,
    face_color_chg: &mut Vec<(String, Option<[f32; 4]>)>,
    pt_color_chg: &mut Vec<(String, Option<[f32; 4]>)>,
    leaf_cache: &HashMap<String, Vec<String>>,
    search: &str,
    depth: usize,
) -> bool {
    let name = node.borrow().name.clone();

    if vmap.contains_key(&name) {
        // ── Leaf node ──────────────────────────────────────────────
        let label = vmap.get(&name).cloned().unwrap_or_else(|| name.clone());
        if !search.is_empty() && !label.to_lowercase().contains(search) { return false; }
        let is_sel = selected.contains(&name);
        let vis = !hidden.contains(&name);
        let is_transform_locked = transform_locked.contains(&name);

        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if vis_btn(ui, is_transform_locked, "Lock transform") {
                    transform_lock_chg.push((name.clone(), !is_transform_locked));
                }
                ui.separator();
                if vis_btn(ui, vis, "Toggle visibility") {
                    vis_chg.push((name.clone(), vis));
                }
                ui.separator();
                if let Some(r) = color_btn(ui, pt_colors.get(&name).copied(), egui::Id::new(("pt_color", &name)), "Point/line color\nRight-click to clear") {
                    pt_color_chg.push((name.clone(), r));
                }
                ui.separator();
                if let Some(r) = color_btn(ui, face_colors.get(&name).copied(), egui::Id::new(("fc_color", &name)), "Face color\nRight-click to clear") {
                    face_color_chg.push((name.clone(), r));
                }
                ui.separator();
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.add_space(depth as f32 * INDENT_W + ARROW_W);
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
                    ui.visuals_mut().selection.bg_fill = egui::Color32::from_gray(180);
                    let text = egui::RichText::new(&label).color(egui::Color32::BLACK);
                    let resp = ui.selectable_label(is_sel, text);
                    if resp.clicked() {
                        let shift = ui.ctx().input(|i| i.modifiers.shift);
                        *new_sel = Some((vec![name.clone()], shift));
                    }
                });
            });
        });
        row_line(ui, ui.min_rect());
        true
    } else {
        // ── Group node ─────────────────────────────────────────────
        let children = node.borrow().children();
        let node_guid = node.borrow().guid().to_string();
        let leaf_guids = leaf_cache.get(&node_guid)
            .cloned()
            .unwrap_or_else(|| collect_leaf_guids(node, vmap));
        let group_vis    = leaf_guids.iter().all(|g| !hidden.contains(g));
        let is_group_sel = !leaf_guids.is_empty() && leaf_guids.iter().all(|g| selected.contains(g));
        let is_locked    = group_locked.contains(&node_guid);

        // Search: show group if name matches or any child matches.
        let name_matches = search.is_empty() || name.to_lowercase().contains(search);

        let open_id = egui::Id::new(("tree_open", &node_guid));
        let is_open = ui.ctx().data_mut(|d| d.get_persisted::<bool>(open_id).unwrap_or(false));
        // When searching, auto-expand groups to show matches.
        let effective_open = is_open || !search.is_empty();

        // Render children first (into a dummy check) to know if any match search.
        let any_child_matches = if !search.is_empty() {
            leaf_guids.iter().any(|g| {
                vmap.get(g).map(|l| l.to_lowercase().contains(search)).unwrap_or(false)
            })
        } else {
            true
        };

        if !name_matches && !any_child_matches { return false; }

        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if vis_btn(ui, is_locked, "Group lock: select together") {
                    lock_chg.push((node_guid.clone(), !is_locked));
                }
                ui.separator();
                if vis_btn(ui, group_vis, "Toggle visibility") {
                    for g in &leaf_guids { vis_chg.push((g.clone(), group_vis)); }
                }
                ui.separator();
                if let Some(r) = color_btn(ui, pt_colors.get(&node_guid).copied(), egui::Id::new(("pt_color", &node_guid)), "Point/line color\nRight-click to clear") {
                    pt_color_chg.push((node_guid.clone(), r));
                }
                ui.separator();
                if let Some(r) = color_btn(ui, face_colors.get(&node_guid).copied(), egui::Id::new(("fc_color", &node_guid)), "Face color\nRight-click to clear") {
                    face_color_chg.push((node_guid.clone(), r));
                }
                ui.separator();
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.add_space(depth as f32 * INDENT_W);
                    let arrow = if effective_open { "▾" } else { "▸" };
                    if icon_btn(ui, arrow, if is_open { "Collapse" } else { "Expand" }) {
                        ui.ctx().data_mut(|d| {
                            let v = d.get_persisted_mut_or_default::<bool>(open_id);
                            *v = !*v;
                        });
                    }
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
                    ui.visuals_mut().selection.bg_fill = egui::Color32::from_gray(180);
                    let text = egui::RichText::new(&*name).color(egui::Color32::BLACK);
                    let resp = ui.selectable_label(is_group_sel, text);
                    if resp.clicked() {
                        let shift = ui.ctx().input(|i| i.modifiers.shift);
                        *new_sel = Some((leaf_guids.clone(), shift));
                    }
                });
            });
        });
        row_line(ui, ui.min_rect());

        if effective_open {
            for child in &children {
                render_tree_node(ui, child, vmap, selected, hidden, group_locked, transform_locked, face_colors, pt_colors, new_sel, vis_chg, lock_chg, transform_lock_chg, face_color_chg, pt_color_chg, leaf_cache, search, depth + 1);
            }
        }
        true
    }
}
