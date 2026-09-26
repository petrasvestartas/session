use crate::State;
use crate::app::command::{Action, Spec};
use crate::app::layers::{index, place, placement};
use crate::app::scene::Scene;
use session_rust::{Session, TreeNode, Xform};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

pub const SPEC: Spec = Spec {
    names: &["Add Group"],
    aliases: &[],
    hint: "Add Group [name] · put the selection under a new group node; a click on one member then selects them all",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Group the selection under a typed name, or the next free `Group N`.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let name = rest.join(" ");

    if name.contains('/') {
        return Err("A group name must be without /".into());
    }

    Ok(Box::new(AddGroup(name)))
}

#[derive(Debug)]
struct AddGroup(String); // the typed name, empty for the next free one

impl Action for AddGroup {
    /// Group the selected objects in one undo step and unfold the panel to the new group.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let rows = state.selected_rows();

        if let Some(reason) = state.locked_reason(&rows) {
            return Err(reason);
        }

        let result = state.scene.add_group(&rows, &self.0);
        state.commit_rows();
        let (doc, name, count) = result?;
        state.reveal_layer(doc, &name); // the members show moved into the new branch
        state.refresh_layers();
        state.touch();
        let plural = if count == 1 { "" } else { "s" };
        Ok(format!(
            "Grouped {count} object{plural} as {name}. A click on one selects all; Undo removes the group"
        ))
    }

    fn needs_selection(&self) -> bool {
        true
    }
}

impl Scene {
    /// Group the objects of `rows` under a new node of their document; returns (document, name, count).
    pub(crate) fn add_group(
        &mut self,
        rows: &[u32],
        name: &str,
    ) -> Result<(usize, String, usize), String> {
        let mut objects = rows
            .iter()
            .map(|&row| self.identity_of(row).ok_or("An object no longer exists"))
            .collect::<Result<Vec<_>, _>>()?;
        let mut seen = HashSet::new();
        objects.retain(|object| seen.insert(object.clone())); // row order, each once
        let doc = objects.first().ok_or("Select objects first")?.0;

        if objects
            .iter()
            .any(|(owner, _)| self.docs.get(*owner).is_none())
        {
            return Err("Only document objects can be grouped".into());
        }

        if objects.iter().any(|(owner, _)| *owner != doc) {
            return Err(
                "Add Group keeps objects in their document; select objects of one document".into(),
            );
        }

        let guids: Vec<Rc<str>> = objects.into_iter().map(|(_, guid)| guid).collect();
        let key = self.step_key("add group")?;
        let (id, name) =
            self.layer_step(doc, &key, |session| group(session, &guids, name.trim()))?;
        self.groups.insert((doc, Rc::from(id)));
        Ok((doc, name, guids.len()))
    }
}

/// Hang `guids` under a new node in their deepest shared layer, in place; returns (guid, name).
fn group(session: &mut Session, guids: &[Rc<str>], name: &str) -> Result<(String, String), String> {
    let nodes = index(session);
    let taken = |name: &str| nodes.contains_key(name) || session.lookup.contains_key(name);
    let name = if name.is_empty() {
        (1..)
            .map(|i| format!("Group {i}"))
            .find(|name| !taken(name))
            .unwrap_or_default()
    } else if taken(name) {
        return Err(format!("{name} is already used in this document"));
    } else {
        name.to_string()
    };
    let root = session.tree.root().ok_or("This document has no tree")?;

    if session.lookup.contains_key(&root.borrow().name) {
        return Err("This document has no layer to hold a group".into());
    }

    let chosen: HashSet<&str> = guids.iter().map(|guid| guid.as_ref()).collect();
    let mut members: Vec<(&str, Option<Node>)> = Vec::new();
    let mut shared: Option<Vec<Node>> = None; // layers above every member, root first

    for guid in guids {
        let node = nodes.get(guid.as_ref()).cloned();
        // parent first; an object outside the tree counts as under the root
        let above = node
            .as_ref()
            .map_or_else(|| vec![Rc::clone(&root)], |node| node.borrow().ancestors());

        // a member inside another chosen member moves with it
        if above
            .iter()
            .any(|ancestor| chosen.contains(ancestor.borrow().name.as_str()))
        {
            continue;
        }

        // a group is never inside an object
        let layers: Vec<Node> = above
            .into_iter()
            .rev()
            .take_while(|ancestor| !session.lookup.contains_key(&ancestor.borrow().name))
            .collect();
        shared = Some(match shared {
            None => layers,
            Some(mut common) => {
                let same = common
                    .iter()
                    .zip(&layers)
                    .take_while(|(a, b)| Rc::ptr_eq(a, b))
                    .count();
                common.truncate(same);
                common
            }
        });
        members.push((guid, node));
    }

    let parent = shared
        .and_then(|layers| layers.last().cloned())
        .unwrap_or_else(|| Rc::clone(&root));
    let back = placement(session, &parent)
        .inverse()
        .ok_or("The layer is degenerate")?;
    let worlds: Vec<Xform> = members
        .iter()
        .map(|(guid, node)| match node {
            Some(node) => placement(session, node),
            None => session.xform(guid),
        })
        .collect();

    let group = TreeNode::new(&name);
    let id = group.borrow().guid().to_string();
    session.add(&group, Some(&parent));

    for ((guid, node), world) in members.iter().zip(&worlds) {
        match node {
            Some(node) => session.add(node, Some(&group)),
            None => session.add(&TreeNode::new(guid), Some(&group)),
        }

        place(session, guid, &back, world);
    }

    Ok((id, name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::scene::FileDoc;
    use session_rust::Point;

    /// One document `site`: layer `walls` holding points a and b, layer `roof` holding point c.
    fn site() -> Scene {
        let mut session = Session::new("site");
        let walls = session.add_group("walls");
        let roof = session.add_group("roof");
        session.add_point(Point::new(0.0, 0.0, 0.0), Some(&walls));
        session.add_point(Point::new(1.0, 0.0, 0.0), Some(&walls));
        session.add_point(Point::new(2.0, 0.0, 0.0), Some(&roof));
        session.set_xform("roof", Xform::translation(0.0, 0.0, 10.0));
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "site".into(),
            session: Rc::new(session),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        scene
    }

    /// The names of a node's children.
    fn children(scene: &Scene, name: &str) -> Vec<String> {
        let node = scene.docs[0].session.tree.get_node_by_name(name).unwrap();
        node.borrow()
            .children()
            .iter()
            .map(|child| child.borrow().name.clone())
            .collect()
    }

    /// The guid of row `row`.
    fn guid(scene: &Scene, row: u32) -> String {
        scene.identity_of(row).unwrap().1.to_string()
    }

    /// Grouping moves the selection under one new node; undo puts the tree back, redo brings the same node.
    #[test]
    fn a_group_takes_the_selection_under_one_new_node_and_undo_puts_the_tree_back() {
        let mut scene = site();
        let (a, b) = (guid(&scene, 0), guid(&scene, 1));
        assert_eq!(
            scene.add_group(&[0, 1], "").unwrap(),
            (0, "Group 1".into(), 2)
        );
        assert_eq!(children(&scene, "walls"), vec!["Group 1"]);
        assert_eq!(children(&scene, "Group 1"), vec![a.clone(), b.clone()]);
        assert_eq!(scene.groups.len(), 1);
        assert!(scene.undo());
        assert_eq!(children(&scene, "walls"), vec![a, b]);
        assert!(
            scene.docs[0]
                .session
                .tree
                .get_node_by_name("Group 1")
                .is_none()
        );
        assert!(scene.redo());
        let node = scene.docs[0]
            .session
            .tree
            .get_node_by_name("Group 1")
            .unwrap();
        let id = node.borrow().guid().to_string();
        assert!(scene.groups.contains(&(0, Rc::from(id))));
    }

    /// A click on any member reaches the outermost group; an ungrouped object stays alone.
    #[test]
    fn a_click_on_any_member_selects_the_outermost_group() {
        let mut scene = site();
        assert_eq!(scene.group_rows(0), vec![0]);
        scene.add_group(&[0, 1], "").unwrap();
        assert_eq!(scene.group_rows(0), vec![0, 1]);
        assert_eq!(scene.group_rows(1), vec![0, 1]);
        assert_eq!(scene.group_rows(2), vec![2]);
        assert_eq!(scene.add_group(&[0], "").unwrap().1, "Group 2");
        assert_eq!(children(&scene, "Group 2"), vec![guid(&scene, 0)]);
        assert_eq!(scene.group_rows(0), vec![0, 1], "the outer group wins");
        assert!(scene.undo());
        assert!(scene.undo());
        assert_eq!(scene.group_rows(0), vec![0]);
    }

    /// Members of two layers go under their shared parent and keep their placement through undo.
    #[test]
    fn members_from_two_layers_group_under_their_shared_parent_and_stay_in_place() {
        let mut scene = site();
        let before: Vec<Xform> = (0..3).map(|row| scene.placement_of(row).unwrap()).collect();
        scene.add_group(&[0, 2], "").unwrap();
        assert_eq!(children(&scene, "site"), vec!["walls", "roof", "Group 1"]);
        let same = |scene: &Scene| {
            (0..3).all(|row| {
                let now = scene.placement_of(row).unwrap();
                now.m
                    .iter()
                    .zip(before[row as usize].m)
                    .all(|(a, b)| (a - b).abs() < 1e-9)
            })
        };
        assert!(same(&scene));
        assert!(scene.undo());
        assert!(same(&scene));
        assert_eq!(children(&scene, "roof"), vec![guid(&scene, 2)]);
    }

    /// Objects without a tree node get one under the group, gone again after undo.
    #[test]
    fn objects_outside_the_tree_get_a_node_under_the_group() {
        let mut scene = site();
        let (a, b) = (guid(&scene, 0), guid(&scene, 1));
        let session = Rc::make_mut(&mut scene.docs[0].session);

        for guid in [&a, &b] {
            let node = session.tree.get_node_by_name(guid).unwrap();
            session.tree.remove(&node);
        }

        scene.forget_nodes(0);
        scene.add_group(&[0, 1], "").unwrap();
        assert_eq!(children(&scene, "site"), vec!["walls", "roof", "Group 1"]);
        assert_eq!(children(&scene, "Group 1"), vec![a.clone(), b]);
        assert!(scene.undo());
        assert!(scene.docs[0].session.tree.get_node_by_name(&a).is_none());
    }

    /// Default names count up; a taken name is refused without an undo step.
    #[test]
    fn names_count_up_and_a_taken_name_is_refused() {
        let mut scene = site();
        assert_eq!(scene.add_group(&[0], "").unwrap().1, "Group 1");
        assert_eq!(scene.add_group(&[1], "").unwrap().1, "Group 2");
        let steps = scene.docs[0].session.history.undo_stack.len();
        assert!(scene.add_group(&[2], "walls").is_err());
        assert_eq!(scene.docs[0].session.history.undo_stack.len(), steps);
        assert_eq!(
            scene.add_group(&[2], " North wall ").unwrap().1,
            "North wall"
        );
    }

    /// Rows of two documents or a text are refused, recording nothing.
    #[test]
    fn two_documents_or_a_text_row_are_refused() {
        let mut scene = site();
        let mut other = Session::new("other");
        other.add_point(Point::new(5.0, 0.0, 0.0), None);
        scene.add_file(FileDoc {
            name: "other".into(),
            session: Rc::new(other),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        assert!(scene.add_group(&[0, 3], "").is_err());
        let text = scene.add_text(crate::app::edit::tests::text(1.0));
        assert!(scene.add_group(&[0, text], "").is_err());
        assert!(scene.docs[0].session.history.undo_stack.is_empty());
        assert!(scene.groups.is_empty());
    }

    /// A group unfolded in the panel is unfolded again when redo brings it back.
    #[test]
    fn an_open_group_reopens_after_undo_and_redo() {
        let mut scene = site();
        scene.add_group(&[0, 1], "").unwrap();
        let mut hierarchy = crate::app::hierarchy::Hierarchy::default();
        hierarchy.rebuild(&scene);
        hierarchy
            .open
            .insert(hierarchy.index_of(0, "Group 1").unwrap());
        assert!(scene.undo());
        hierarchy.rebuild(&scene);
        assert!(hierarchy.index_of(0, "Group 1").is_none());
        assert!(scene.redo());
        hierarchy.rebuild(&scene);
        assert!(
            hierarchy
                .open
                .contains(&hierarchy.index_of(0, "Group 1").unwrap())
        );
    }

    /// Groups come back with a saved session.
    #[test]
    fn groups_survive_save_and_open() {
        let mut scene = site();
        scene.add_group(&[0, 1], "").unwrap();
        let bytes = crate::app::session_io::save(&scene).unwrap();
        let restored = crate::app::session_io::open(&bytes).unwrap();
        assert_eq!(restored.group_rows(0), vec![0, 1]);
    }
}
