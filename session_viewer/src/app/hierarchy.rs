use crate::app::scene::Scene;
use session_rust::Geometry;
#[cfg(test)]
use session_rust::Session;
#[cfg(test)]
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Range;
use std::rc::Rc;

const MAX_NODES: usize = 200_000; // most tree nodes shown

const MAX_ROWS: usize = 1_000_000; // most object rows indexed

const MAX_EDGES: usize = 100_000; // most graph edges listed

pub const PAGE_SIZE: usize = 128; // nodes per panel page

type Lookup = HashMap<usize, HashMap<Rc<str>, u32>>; // document -> guid -> row

/// One line of the tree panel.
pub struct Node {
    pub label: String,      // text shown
    pub depth: usize,       // indent level
    pub end: usize,         // index one past its last descendant
    pub rows: Range<usize>, // its object rows, as a slice of `Hierarchy::rows`
    pub doc: usize,         // its document
    pub name: String,       // its tree node name, empty when outside the tree
    pub layer: bool,        // a group or document, not an object
    pub pick: Option<Pick>, // the element features the line stands for, when it lists features
}

/// The features of one element a panel line stands for: an interaction, one of its pieces, or one of the element's own.
#[derive(Clone, Debug, PartialEq)]
pub struct Pick {
    pub row: u32,            // the element's row
    pub features: Vec<usize>, // indices into its features
    pub interactions: bool,  // drawn while Element Interactions is on, else while Element Attributes is
}

/// The tree panel's flattened nodes.
#[derive(Default)]
pub struct Hierarchy {
    pub nodes: Vec<Node>,           // every node, parents before children
    pub rows: Vec<u32>,             // object rows the nodes point into
    pub open: HashSet<usize>,       // expanded nodes
    pub page: usize,                // current panel page
    pub truncated: bool,            // scene too large to show
    pub edges: Vec<[u32; 2]>,       // graph edges between object rows, from and to
    pub active: Vec<usize>,         // clicked layer nodes
    revision: Option<u64>,          // scene revision this was built from
    away: HashSet<(usize, String)>, // open nodes an undo took away, open again when they return
}

impl Hierarchy {
    /// Rebuild when the scene changed.
    pub fn refresh(&mut self, scene: &Scene) {
        if self.revision != Some(scene.row_revision) {
            self.rebuild(scene);
        }
    }

    /// Rebuild the nodes from every document's tree; past the node limit only layers get a line.
    pub fn rebuild(&mut self, scene: &Scene) {
        self.build(scene, scene.object_count() <= MAX_NODES);
    }

    /// Rebuild the nodes, a line per object too when `objects`.
    fn build(&mut self, scene: &Scene, objects: bool) {
        self.revision = Some(scene.row_revision);
        // open and clicked nodes by name, so a rebuilt tree keeps them
        let mut open = self.names(self.open.iter());
        open.extend(self.away.drain());
        let active = self.names(self.active.iter());
        self.active.clear();
        self.nodes.clear();
        self.rows.clear();
        self.edges.clear();
        self.open.clear();
        self.truncated = false;

        // guid to row, per document
        let mut lookup = Lookup::new();

        for row in 0..scene.row_count() as u32 {
            if let Some(identity) = scene.identity_of(row) {
                lookup
                    .entry(identity.0)
                    .or_default()
                    .insert(identity.1, row);
            }
        }

        for doc in 0..scene.docs.len() {
            let start = self.nodes.len();
            let rows = self.rows.len();

            if !self.tree(scene, doc, &lookup, objects) {
                self.nodes.truncate(start);
                self.rows.truncate(rows);
                self.truncated = true;
                break;
            }
        }

        let mut back = HashSet::new(); // open nodes found again

        for (index, node) in self.nodes.iter().enumerate() {
            let name = (node.doc, node.name.clone());

            if open.contains(&name) {
                self.open.insert(index);
                back.insert(name.clone());
            }

            if node.layer && active.contains(&name) {
                self.active.push(index);
            }
        }

        open.retain(|name| !back.contains(name));
        self.away = open;
        self.connect(scene, &lookup);
    }

    /// The (document, name) of some nodes, which outlive a rebuild.
    fn names<'a>(&self, indices: impl Iterator<Item = &'a usize>) -> HashSet<(usize, String)> {
        indices
            .filter_map(|index| self.nodes.get(*index))
            .map(|node| (node.doc, node.name.clone()))
            .collect()
    }

    /// Collect every graph edge whose two ends are object rows, in the order they were made.
    fn connect(&mut self, scene: &Scene, lookup: &Lookup) {
        let mut edges = Vec::new(); // (document, edge index, from row, to row)

        for (doc, file) in scene.docs.iter().enumerate() {
            for (from, neighbours) in &file.session.graph.edges {
                for (to, edge) in neighbours {
                    // each edge is kept both ways; take it once, in its own direction
                    let forward = if edge.v0.is_empty() {
                        from <= to
                    } else {
                        edge.v0 == *from && edge.v1 == *to
                    };

                    if forward
                        && edges.len() < MAX_EDGES
                        && let Some(a) = row_of(lookup, doc, from)
                        && let Some(b) = row_of(lookup, doc, to)
                    {
                        edges.push((doc, edge.index, a, b));
                    }
                }
            }
        }

        edges.sort_unstable();
        self.edges = edges.into_iter().map(|(_, _, a, b)| [a, b]).collect();
    }

    /// The node of a layer, by document and name.
    pub fn index_of(&self, doc: usize, name: &str) -> Option<usize> {
        self.nodes
            .iter()
            .position(|node| node.layer && node.doc == doc && node.name == name)
    }

    /// Unfold the parents of node `index` and turn to its page.
    pub fn reveal(&mut self, index: usize) {
        for (parent, node) in self.nodes.iter().enumerate().take(index) {
            if node.end > index {
                self.open.insert(parent);
            }
        }

        if let Some(at) = self.visible().iter().position(|node| *node == index) {
            self.page = at / PAGE_SIZE;
        }
    }

    /// Add one document and its tree, a line per object when `objects`; false when a limit is hit.
    fn tree(&mut self, scene: &Scene, doc: usize, lookup: &Lookup, objects: bool) -> bool {
        let file = &scene.docs[doc];
        let start = self.nodes.len();
        // the document line stands for the root group, whatever its name
        let root = file
            .session
            .tree
            .root()
            .filter(|root| !file.session.lookup.contains_key(&root.borrow().name));
        let name = root
            .as_ref()
            .map(|root| root.borrow().name.clone())
            .unwrap_or_default();

        if !self.push(&file.name, 0, doc, name, root.is_some()) {
            return false;
        }

        let mut seen = HashSet::new(); // nodes visited
        let mut seen_rows = HashSet::new(); // rows placed
        let mut stack = Vec::new(); // (node, depth, index to close, inside an object, the element row it hangs under)

        for child in root.iter().flat_map(|root| root.borrow().children()).rev() {
            stack.push((child, 1, None, false, None));
        }

        // without object lines one layer may hold every row
        let most = if objects { MAX_NODES } else { MAX_ROWS };

        // depth first; a node is pushed again to close it after its children
        for _ in 0..most * 2 {
            let Some((node, depth, exit, inside, host)) = stack.pop() else {
                break;
            };

            if let Some(index) = exit {
                if !self.feature_lines(scene, index, host, lookup) {
                    return false;
                }

                self.finish(index);
                continue;
            }

            if !seen.insert(Rc::as_ptr(&node)) {
                continue;
            }

            let borrowed = node.borrow();
            let row = row_of(lookup, doc, &borrowed.name);
            let index = self.nodes.len();
            // an object without a row is still no layer, nor a group inside an object
            let object = row.is_some()
                || file.session.lookup.contains_key(&borrowed.name)
                || file.session.component_lookup.contains_key(&borrowed.name);
            let layer = !object && !inside;

            // an object without a line of its own still counts under its layer
            if objects || layer {
                let label = row.map(|r| scene.object_name(r)).unwrap_or(&borrowed.name);

                if !self.push(label, depth, doc, borrowed.name.clone(), layer) {
                    return false;
                }

                stack.push((Rc::clone(&node), depth, Some(index), inside, host));
            }

            if let Some(row) = row
                && seen_rows.insert(row)
            {
                self.rows.push(row);
            }

            let children = borrowed.children();

            if stack.len() + children.len() > most {
                return false;
            }

            // the children of an element know it, so its `attributes` group lists its own features
            let element = row.filter(|&r| matches!(scene.geometry(r), Some(Geometry::Element(_))));

            for child in children.into_iter().rev() {
                stack.push((child, depth + 1, None, inside || object, element));
            }
        }

        if !stack.is_empty() {
            return false;
        }

        // objects outside the tree go under the document
        for row in 0..scene.row_count() as u32 {
            if scene
                .identity_of(row)
                .is_some_and(|(owner, _)| owner == doc)
                && seen_rows.insert(row)
            {
                let index = self.nodes.len();

                if objects && !self.push(scene.object_name(row), 1, doc, String::new(), false) {
                    return false;
                }

                self.rows.push(row);

                if objects {
                    self.finish(index);
                }
            }
        }

        self.finish(start);
        self.rows.len() <= MAX_ROWS
    }

    /// Add the graph's vertices and edges as groups.
    #[cfg(test)]
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

            if !self.push(&format!("{name} / {label}"), 0, doc, String::new(), false) {
                return false;
            }

            self.rows.extend(rows);
            self.finish(index);
        }

        true
    }

    /// Add one open node; false at the limit.
    fn push(&mut self, label: &str, depth: usize, doc: usize, name: String, layer: bool) -> bool {
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
            doc,
            name,
            layer,
            pick: None,
        });
        true
    }

    /// Closing node `index`: an element's `attributes` group lists the element's own features, and an element ends with its `interactions` group, a line per interaction it hosts holding a line per feature that interaction drew.
    fn feature_lines(&mut self, scene: &Scene, index: usize, host: Option<u32>, lookup: &Lookup) -> bool {
        let (doc, depth, name) = {
            let node = &self.nodes[index];
            (node.doc, node.depth, node.name.clone())
        };

        // the element's `attributes` group: its own features after its objects
        if name == "attributes"
            && let Some(row) = host
            && let Some(Geometry::Element(element)) = scene.geometry(row)
        {
            return self.own_lines(element, row, depth + 1, doc);
        }

        let Some(row) = row_of(lookup, doc, &name) else {
            return true;
        };
        let Some(Geometry::Element(element)) = scene.geometry(row) else {
            return true;
        };
        let session = &scene.docs[doc].session;

        // no `attributes` group of its own: one made for its own features
        let grouped = session
            .tree
            .get_node_by_name(&name)
            .is_some_and(|node| node.borrow().children().iter().any(|c| c.borrow().name == "attributes"));

        if !grouped && !crate::app::interactions::own(element).is_empty() {
            let group = self.nodes.len();

            if !self.push("attributes", depth + 1, doc, format!("{name}/attributes"), false)
                || !self.own_lines(element, row, depth + 2, doc)
            {
                return false;
            }

            self.nodes[group].pick = Some(Pick {
                row,
                features: crate::app::interactions::own(element),
                interactions: false,
            });
            self.finish(group);
        }

        let hosted = crate::app::interactions::hosted(session, &name, element);

        if hosted.is_empty() {
            return true;
        }

        let group = self.nodes.len();

        if !self.push("interactions", depth + 1, doc, format!("{name}/interactions"), false) {
            return false;
        }

        self.nodes[group].pick = Some(Pick {
            row,
            features: hosted.iter().flat_map(|h| h.features.iter().copied()).collect(),
            interactions: true,
        });

        for (at, interaction) in hosted.iter().enumerate() {
            let line = self.nodes.len();

            if !self.push(&interaction.label, depth + 2, doc, format!("{name}/interactions/{at}"), false) {
                return false;
            }

            self.nodes[line].pick = Some(Pick {
                row,
                features: interaction.features.clone(),
                interactions: true,
            });

            // its attributes: the pieces it drew
            for &i in &interaction.features {
                let piece = self.nodes.len();
                let label = crate::app::interactions::feature_label(&element.features()[i]);

                if !self.push(&label, depth + 3, doc, format!("{name}/interactions/{at}/{i}"), false) {
                    return false;
                }

                self.nodes[piece].pick = Some(Pick { row, features: vec![i], interactions: true });
                self.finish(piece);
            }

            self.finish(line);
        }

        self.finish(group);
        true
    }

    /// A line per feature the element carries of its own, an axis or a section, at `depth`.
    fn own_lines(&mut self, element: &session_rust::Element, row: u32, depth: usize, doc: usize) -> bool {
        for i in crate::app::interactions::own(element) {
            let line = self.nodes.len();
            let label = crate::app::interactions::feature_label(&element.features()[i]);

            if !self.push(&label, depth, doc, format!("{}/own/{i}", element.guid()), false) {
                return false;
            }

            self.nodes[line].pick = Some(Pick { row, features: vec![i], interactions: false });
            self.finish(line);
        }

        true
    }

    /// Close node `index` at the current end.
    fn finish(&mut self, index: usize) {
        self.nodes[index].end = self.nodes.len();
        self.nodes[index].rows.end = self.rows.len();
    }

    /// The nodes shown, skipping closed subtrees.
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

    /// The object rows under node `index`.
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

/// The row of one guid in one document.
fn row_of(lookup: &Lookup, doc: usize, guid: &str) -> Option<u32> {
    lookup.get(&doc)?.get(guid).copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::scene::FileDoc;
    use session_rust::Point;
    #[cfg(test)]
    use session_rust::Session;
    use session_rust::Xform;

    /// A full row table refuses the graph.
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

    /// Groups and edges name rows of their own document.
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
        let tree_count = index.rows.len();
        let mut lookup = Lookup::new();

        for row in 0..scene.row_count() as u32 {
            let (doc, id) = scene.identity_of(row).unwrap();
            lookup.entry(doc).or_default().insert(id, row);
        }

        assert!(index.graph(&shared, 0, "first", &lookup));
        let edge = index
            .nodes
            .iter()
            .position(|node| node.label == "first / edge: joint")
            .unwrap();
        assert_eq!(index.targets(edge), vec![0, 1]);
        assert!(!index.visible().contains(&child));
        index.rebuild(&scene);
        assert_eq!(index.rows.len(), tree_count);
        scene = Scene::new();
        index.rebuild(&scene);
        assert!(index.rows.is_empty());
        assert!(index.nodes.is_empty());
    }

    /// Groups are layers, objects are not even without a row, and edges keep their direction.
    #[test]
    fn layers_are_groups_and_edges_keep_their_direction() {
        let mut session = Session::new("site");
        let walls = session.add_group("walls");
        let attributes = session.add_group("attributes");
        let a = session.add_point(Point::new(0.0, 0.0, 0.0), Some(&walls));
        let b = session.add_point(Point::new(1.0, 0.0, 0.0), Some(&walls));
        session.add_point(Point::new(2.0, 0.0, 0.0), Some(&attributes));
        session.add(&session_rust::TreeNode::new("features"), Some(&a));
        session.add_edge(&b.borrow().name, &a.borrow().name, "joint");
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "site".into(),
            session: Rc::new(session),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        let mut index = Hierarchy::default();
        index.rebuild(&scene);
        let layers: Vec<&str> = index
            .nodes
            .iter()
            .filter(|node| node.layer)
            .map(|node| node.name.as_str())
            .collect();
        assert_eq!(layers, vec!["site", "walls", "attributes"]);
        let a = scene.row_of(0, &a.borrow().name).unwrap();
        let b = scene.row_of(0, &b.borrow().name).unwrap();
        assert_eq!(index.edges, vec![[b, a]], "from b to a, as it was made");
        let walls = index.index_of(0, "walls").unwrap();
        assert!(!index.visible().contains(&(walls + 1)));
        index.reveal(walls + 1);
        assert!(index.visible().contains(&(walls + 1)));
        scene.docs[0].name = "renamed file".into();
        index.rebuild(&scene);
        assert_eq!(index.nodes[0].label, "renamed file");
        assert_eq!(
            index.nodes[0].name, "site",
            "the document line is the root layer"
        );
        assert_eq!(index.nodes[1].name, "walls");
    }

    /// An element lists its own features under `attributes` and, under `interactions`, a line per interaction it hosts, led by the full type name, holding the pieces it drew.
    #[test]
    fn an_element_lists_its_attributes_and_interactions() {
        use session_rust::element::{Element, ElementFeature};
        let mut session = Session::new("site");
        let walls = session.add_group("walls");
        let line = session_rust::Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)]);
        let contact = session_rust::InteractionUnknown::new("InteractionContactFace", &[], "seam_wedge_0");
        let mut touch = ElementFeature::new("contact", 0, vec![line.clone()], "side_side");
        touch.set_guid(session_rust::Interaction::guid(&contact).to_string());
        let mut beam = Element::new("beam");
        beam.set_geometry(session_rust::Mesh::create_box(1.0, 1.0, 1.0));
        beam.add_feature(ElementFeature::new("axis", -1, vec![line], "axis"));
        beam.add_feature(touch);
        let mut bare = Element::new("bare");
        bare.set_geometry(session_rust::Mesh::create_box(1.0, 1.0, 1.0));
        let beam = session.add_element(beam, Some(&walls)).borrow().name.clone();
        let bare = session.add_element(bare, Some(&walls)).borrow().name.clone();
        session.graph.add_edge(&bare, &beam, "");
        let edge = session.graph.edges[&bare][&beam].guid().to_string();
        session.interactions.insert(edge, vec![Box::new(contact)]);
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "site".into(),
            session: Rc::new(session),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        let mut index = Hierarchy::default();
        index.rebuild(&scene);
        let row = scene.row_of(0, &beam).unwrap();
        let lines: Vec<(&str, usize, Vec<usize>, bool)> = index
            .nodes
            .iter()
            .filter_map(|node| node.pick.as_ref().map(|pick| (node.label.as_str(), node.depth, pick.features.clone(), pick.interactions)))
            .collect();
        assert_eq!(
            lines,
            vec![
                ("attributes", 3, vec![0], false),
                ("axis", 4, vec![0], false),
                ("interactions", 3, vec![1], true),
                ("InteractionContactFace seam_wedge_0", 4, vec![1], true),
                ("contact side_side · face 0", 5, vec![1], true),
            ],
            "the bare element lists nothing"
        );
        assert!(index.nodes.iter().filter_map(|node| node.pick.as_ref()).all(|pick| pick.row == row));
        let group = index.nodes.iter().position(|node| node.label == "interactions").unwrap();
        assert!(index.targets(group).is_empty(), "feature lines hold no rows");
    }

    /// Past the node limit only layers get lines, and they still hold their objects.
    #[test]
    fn a_scene_past_the_node_limit_lists_its_layers() {
        let mut session = Session::new("site");
        let walls = session.add_group("walls");
        let a = session.add_point(Point::new(0.0, 0.0, 0.0), Some(&walls));
        session.add_point(Point::new(1.0, 0.0, 0.0), Some(&walls));
        session.add(&session_rust::TreeNode::new("features"), Some(&a));
        session.add_point(Point::new(2.0, 0.0, 0.0), None);
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "site".into(),
            session: Rc::new(session),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        let mut index = Hierarchy::default();
        index.build(&scene, false);
        let labels: Vec<&str> = index.nodes.iter().map(|node| node.label.as_str()).collect();
        assert_eq!(labels, vec!["site", "walls"]);
        assert!(!index.truncated);
        assert_eq!(index.targets(0).len(), 3, "the document holds every object");
        assert_eq!(index.targets(1).len(), 2, "walls holds its two points");
        index.build(&scene, true);
        assert_eq!(index.nodes.len(), 6, "with a line per object and group");
    }
}
