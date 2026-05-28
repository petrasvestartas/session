use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;
use session_rust::{Session, TreeNode};

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
    let guid = node.borrow().guid().to_string();
    if !cache.contains_key(&guid) {
        cache.insert(guid, collect_leaf_guids(node, vmap));
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
    let new_locked = if group_locked.contains(&name) { Some(node.clone()) } else { current_locked };
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
        group_locked.insert(name);
    } else {
        for child in &children {
            auto_lock_leaf_groups(child, geom_guids, group_locked);
        }
    }
}

pub fn render_tree_node(
    ui: &mut egui::Ui,
    node: &Rc<RefCell<TreeNode>>,
    vmap: &HashMap<String, String>,
    selected: &HashSet<String>,
    hidden: &HashSet<String>,
    locked: &HashSet<String>,
    new_sel: &mut Option<(Vec<String>, bool)>,
    vis_chg: &mut Vec<(String, bool)>,
    lock_chg: &mut Vec<(String, bool)>,
    leaf_cache: &HashMap<String, Vec<String>>,
) {
    let name = node.borrow().name.clone();
    if vmap.contains_key(&name) {
        let label = vmap.get(&name).cloned().unwrap_or_else(|| name.clone());
        let is_sel = selected.contains(&name);
        let mut vis = !hidden.contains(&name);
        ui.horizontal(|ui| {
            let resp = ui.selectable_label(is_sel, &label);
            if resp.clicked() {
                let shift = ui.ctx().input(|i| i.modifiers.shift);
                *new_sel = Some((vec![name.clone()], shift));
            }
            if ui.toggle_value(&mut vis, "V").on_hover_text("Visibility").changed() {
                vis_chg.push((name.clone(), !vis));
            }
        });
    } else {
        let children = node.borrow().children();
        let node_guid = node.borrow().guid().to_string();
        let leaf_guids = leaf_cache.get(&node_guid)
            .cloned()
            .unwrap_or_else(|| collect_leaf_guids(node, vmap));
        let group_vis = leaf_guids.iter().all(|g| !hidden.contains(g));
        let id = ui.make_persistent_id(&node_guid);
        egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false)
            .show_header(ui, |ui| {
                let is_group_sel = !leaf_guids.is_empty() && leaf_guids.iter().all(|g| selected.contains(g));
                let resp = ui.selectable_label(is_group_sel, &*name);
                if resp.clicked() {
                    let shift = ui.ctx().input(|i| i.modifiers.shift);
                    *new_sel = Some((leaf_guids.clone(), shift));
                }
                let mut gv = group_vis;
                if ui.toggle_value(&mut gv, "V").on_hover_text("Visibility").changed() {
                    for g in &leaf_guids {
                        vis_chg.push((g.clone(), !gv));
                    }
                }
                let mut lk = locked.contains(&name);
                if ui.toggle_value(&mut lk, "G").on_hover_text("Group lock: select together").changed() {
                    lock_chg.push((name.clone(), lk));
                }
            })
            .body(|ui| {
                for child in &children {
                    render_tree_node(ui, child, vmap, selected, hidden, locked, new_sel, vis_chg, lock_chg, leaf_cache);
                }
            });
    }
}
