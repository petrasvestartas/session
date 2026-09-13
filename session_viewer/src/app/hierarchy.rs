use crate::app::scene::Scene;
use session_rust::Session;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Range;
use std::rc::Rc;

const MAX_NODES: usize = 200_000;
const MAX_ROWS: usize = 1_000_000;
pub const PAGE_SIZE: usize = 128;
type Lookup = HashMap<usize, HashMap<Rc<str>, u32>>;

pub struct Node {
    pub label: String,
    pub depth: usize,
    pub end: usize,
    pub rows: Range<usize>,
}

#[derive(Default)]
pub struct Hierarchy {
    pub nodes: Vec<Node>,
    pub rows: Vec<u32>,
    pub open: HashSet<usize>,
    pub page: usize,
    pub selected: Vec<u32>,
    pub truncated: bool,
    revision: Option<u64>,
}

impl Hierarchy {
    pub fn refresh(&mut self, scene: &Scene) {
        if self.revision != Some(scene.row_revision) {
            self.rebuild(scene);
        }
    }

    /// Index tree subtrees and graph attributes without copying geometry.
    pub fn rebuild(&mut self, scene: &Scene) {
        self.revision = Some(scene.row_revision);
        self.nodes.clear();
        self.rows.clear();
        self.truncated = scene.object_count() > MAX_NODES;
        if self.truncated {
            return;
        }
        let mut lookup = Lookup::new();
        for row in 0..scene.object_count() as u32 {
            if let Some(identity) = scene.identity_of(row) {
                lookup
                    .entry(identity.0)
                    .or_default()
                    .insert(identity.1, row);
            }
        }
        for (doc, file) in scene.docs.iter().enumerate() {
            let start = self.nodes.len();
            let rows = self.rows.len();
            if !self.tree(scene, doc, &lookup)
                || !self.graph(&file.session, doc, &file.name, &lookup)
            {
                self.nodes.truncate(start);
                self.rows.truncate(rows);
                self.truncated = true;
                break;
            }
        }
        self.open.retain(|index| *index < self.nodes.len());
    }

    fn tree(&mut self, scene: &Scene, doc: usize, lookup: &Lookup) -> bool {
        let file = &scene.docs[doc];
        let start = self.nodes.len();
        if !self.push(&file.name, 0) {
            return false;
        }
        let mut seen = HashSet::new();
        let mut stack = Vec::new();
        if let Some(root) = file.session.tree.root() {
            stack.push((root, 1, None));
        }
        for _ in 0..MAX_NODES * 2 {
            let Some((node, depth, exit)) = stack.pop() else {
                break;
            };
            if let Some(index) = exit {
                self.finish(index);
                continue;
            }
            if !seen.insert(Rc::as_ptr(&node)) {
                continue;
            }
            let borrowed = node.borrow();
            let row = row_of(lookup, doc, &borrowed.name);
            let label = row.map(|r| scene.object_name(r)).unwrap_or(&borrowed.name);
            let index = self.nodes.len();
            if !self.push(label, depth) {
                return false;
            }
            if let Some(row) = row {
                self.rows.push(row);
            }
            stack.push((Rc::clone(&node), depth, Some(index)));
            let children = borrowed.children();
            if stack.len() + children.len() > MAX_NODES {
                return false;
            }
            for child in children.into_iter().rev() {
                stack.push((child, depth + 1, None));
            }
        }
        if !stack.is_empty() {
            return false;
        }
        if self.rows.len() == self.nodes[start].rows.start {
            for row in 0..scene.object_count() as u32 {
                if scene
                    .identity_of(row)
                    .is_some_and(|(owner, _)| owner == doc)
                {
                    self.rows.push(row);
                }
            }
        }
        self.finish(start);
        self.rows.len() <= MAX_ROWS
    }

    fn graph(&mut self, session: &Session, doc: usize, name: &str, lookup: &Lookup) -> bool {
        let vertices = session.graph.number_of_vertices();
        let edges: usize = session.graph.edges.values().map(|edges| edges.len()).sum();
        let remaining = MAX_ROWS.saturating_sub(self.rows.len());
        if vertices > MAX_NODES || vertices > remaining || edges > remaining - vertices {
            return false;
        }
        let mut groups: BTreeMap<String, Vec<u32>> = BTreeMap::new();
        for vertex in session.graph.get_vertices() {
            if let Some(row) = row_of(lookup, doc, &vertex.name) {
                groups
                    .entry(format!("vertex: {}", vertex.attribute))
                    .or_default()
                    .push(row);
            }
        }
        for (from, edges) in &session.graph.edges {
            for (to, edge) in edges {
                if from > to {
                    continue;
                }
                let rows = groups
                    .entry(format!("edge: {}", edge.attribute))
                    .or_default();
                for guid in [from, to] {
                    if let Some(row) = row_of(lookup, doc, guid) {
                        rows.push(row);
                    }
                    if from == to {
                        break;
                    }
                }
            }
        }
        for (label, mut rows) in groups {
            rows.sort_unstable();
            rows.dedup();
            let index = self.nodes.len();
            if !self.push(&format!("{name} / {label}"), 0) {
                return false;
            }
            self.rows.extend(rows);
            self.finish(index);
        }
        true
    }

    fn push(&mut self, label: &str, depth: usize) -> bool {
        if self.nodes.len() >= MAX_NODES {
            return false;
        }
        let start = self.rows.len();
        let label = label.chars().take(160).collect();
        self.nodes.push(Node {
            label,
            depth,
            end: 0,
            rows: start..start,
        });
        true
    }

    fn finish(&mut self, index: usize) {
        self.nodes[index].end = self.nodes.len();
        self.nodes[index].rows.end = self.rows.len();
    }

    pub fn visible(&self) -> Vec<usize> {
        let mut result = Vec::new();
        let mut index = 0;
        for _ in 0..self.nodes.len() {
            let Some(node) = self.nodes.get(index) else {
                break;
            };
            result.push(index);
            index = if self.open.contains(&index) {
                index + 1
            } else {
                node.end
            };
        }
        result
    }

    pub fn targets(&self, index: usize) -> Vec<u32> {
        let Some(node) = self.nodes.get(index) else {
            return Vec::new();
        };
        let mut rows = self.rows[node.rows.clone()].to_vec();
        rows.sort_unstable();
        rows.dedup();
        rows
    }
}

fn row_of(lookup: &Lookup, doc: usize, guid: &str) -> Option<u32> {
    lookup.get(&doc)?.get(guid).copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::scene::FileDoc;
    use session_rust::Point;
    use session_rust::Session;
    use session_rust::Xform;

    #[test]
    fn graph_refuses_vertex_overflow_before_allocating_groups() {
        let mut session = Session::new("budget");
        session.add_point(Point::new(0.0, 0.0, 0.0), None);
        session.add_point(Point::new(1.0, 0.0, 0.0), None);
        let mut index = Hierarchy::default();
        index.rows.resize(MAX_ROWS - 1, 0);
        assert!(!index.graph(&session, 0, "budget", &Lookup::new()));
        assert_eq!(index.rows.len(), MAX_ROWS - 1);
        assert!(index.nodes.is_empty());
    }

    #[test]
    fn nested_groups_and_graph_endpoints_are_document_scoped() {
        let mut session = Session::new("test");
        let parent = session.add_group("parent");
        let child = session_rust::TreeNode::new("child");
        session.add(&child, Some(&parent));
        let a = session.add_point(Point::new(0.0, 0.0, 0.0), Some(&child));
        let b = session.add_point(Point::new(1.0, 0.0, 0.0), Some(&parent));
        session.add_edge(&a.borrow().name, &b.borrow().name, "joint");
        let shared = Rc::new(session);
        let mut scene = Scene::new();
        for name in ["first", "second"] {
            scene.add_file(FileDoc {
                name: name.into(),
                session: Rc::clone(&shared),
                place: Xform::identity(),
                point_px: 0.0,
                display_only: false,
            });
        }
        let mut index = Hierarchy::default();
        index.rebuild(&scene);
        let parent = index
            .nodes
            .iter()
            .position(|node| node.label == "parent")
            .unwrap();
        let child = index
            .nodes
            .iter()
            .position(|node| node.label == "child")
            .unwrap();
        assert_eq!(index.targets(parent), vec![0, 1]);
        assert_eq!(index.targets(child), vec![0]);
        let edge = index
            .nodes
            .iter()
            .position(|node| node.label == "first / edge: joint")
            .unwrap();
        assert_eq!(index.targets(edge), vec![0, 1]);
        assert!(!index.visible().contains(&child));
        let count = index.rows.len();
        index.rebuild(&scene);
        assert_eq!(index.rows.len(), count);
        scene = Scene::new();
        index.rebuild(&scene);
        assert!(index.rows.is_empty());
        assert!(index.nodes.is_empty());
    }
}
