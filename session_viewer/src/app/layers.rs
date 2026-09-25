use crate::app::scene::{FileDoc, Scene, Shape, sync};
use session_rust::{Edge, Geometry, History, Session, TreeNode, Xform};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

/// What one panel row controls.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layer {
    Document(usize), // one loaded file, by index
    Kind(Kind),      // every object of one kind
}

/// The kinds the panel groups objects by.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Solids,   // BReps, boxes, elements
    Surfaces, // NURBS surfaces, planes
    Meshes,
    Curves, // lines, polylines, NURBS curves
    Points,
    Clouds, // point clouds
}

impl Kind {
    /// The row label.
    pub fn label(self) -> &'static str {
        match self {
            Kind::Solids => "solids",
            Kind::Surfaces => "surfaces",
            Kind::Meshes => "meshes",
            Kind::Curves => "curves",
            Kind::Points => "points",
            Kind::Clouds => "clouds",
        }
    }

    /// The kind of one geometry type.
    fn of(shape: Shape) -> Self {
        match shape {
            Shape::BRep | Shape::Box | Shape::Element => Kind::Solids,
            Shape::Surface | Shape::Plane => Kind::Surfaces,
            Shape::Mesh => Kind::Meshes,
            Shape::Line | Shape::Polyline | Shape::Curve => Kind::Curves,
            Shape::Point => Kind::Points,
            Shape::Cloud => Kind::Clouds,
        }
    }
}

impl Layer {
    /// The row's text key, e.g. `doc:0` or `kind:curves`.
    pub fn key(self) -> String {
        match self {
            Layer::Document(index) => format!("doc:{index}"),
            Layer::Kind(kind) => format!("kind:{}", kind.label()),
        }
    }

    /// The layer from a row key.
    pub fn from_key(key: &str) -> Option<Self> {
        if let Some(index) = key.strip_prefix("doc:") {
            return index.parse().ok().map(Layer::Document);
        }

        let label = key.strip_prefix("kind:")?;

        for kind in [
            Kind::Solids,
            Kind::Surfaces,
            Kind::Meshes,
            Kind::Curves,
            Kind::Points,
            Kind::Clouds,
        ] {
            if kind.label() == label {
                return Some(Layer::Kind(kind));
            }
        }

        None
    }
}

/// One line of the panel.
pub struct Row {
    pub layer: Layer,  // what it controls
    pub label: String, // text shown
    pub count: usize,  // objects it controls
    pub hidden: bool,  // all of them hidden
}

/// The panel rows: documents, then the kinds present.
pub fn rows(scene: &Scene) -> Vec<Row> {
    let mut documents = vec![(0, 0); scene.docs.len()]; // (count, hidden) per document
    let mut kinds = [(0, 0); 6]; // (count, hidden) per kind

    for row in 0..scene.row_count() as u32 {
        let Some(identity) = scene.identity_of(row) else {
            continue;
        };
        let hidden = usize::from(scene.hidden.contains(&identity));

        if let Some(count) = documents.get_mut(identity.0) {
            count.0 += 1;
            count.1 += hidden;
        }

        if let Some(shape) = scene.shape(row) {
            let count = &mut kinds[Kind::of(shape) as usize];
            count.0 += 1;
            count.1 += hidden;
        }
    }

    let mut out = Vec::new();

    for (index, &(count, hidden)) in documents.iter().enumerate() {
        if count > 0 {
            out.push(Row {
                layer: Layer::Document(index), // one document as a layer
                label: scene.docs[index].name.clone(),
                count,
                hidden: hidden == count,
            });
        }
    }

    for kind in [
        Kind::Solids,
        Kind::Surfaces,
        Kind::Meshes,
        Kind::Curves,
        Kind::Points,
        Kind::Clouds,
    ] {
        let (count, hidden) = kinds[kind as usize];

        if count > 0 {
            out.push(Row {
                layer: Layer::Kind(kind),
                label: kind.label().into(),
                count,
                hidden: hidden == count,
            });
        }
    }

    out
}

/// The object rows of one layer.
pub fn of_layer(scene: &Scene, layer: Layer) -> Vec<u32> {
    let mut rows = Vec::new();

    for row in 0..scene.row_count() as u32 {
        let matches = match layer {
            Layer::Document(index) => scene.identity_of(row).map(|(doc, _)| doc) == Some(index),
            Layer::Kind(kind) => scene.shape(row).map(Kind::of) == Some(kind),
        };

        if matches {
            rows.push(row);
        }
    }

    rows
}

impl Scene {
    /// The tree node holding the object `guid` of `doc`, when it hangs from the tree.
    pub(crate) fn parent_of(&self, doc: usize, guid: &str) -> Option<Node> {
        let (node, in_tree) = self.node_of(self.row_of(doc, guid)?)?;
        in_tree.then(|| node.borrow().parent()).flatten()
    }

    /// The layer new objects go to: the chosen one while it exists, else the `Created` root.
    pub fn current_layer(&self) -> Option<(usize, String)> {
        if let Some((doc, name)) = &self.current_layer
            && self.layer_node(*doc, name).is_ok()
        {
            return Some((*doc, name.clone()));
        }

        let doc = self.created_doc?;
        let root = self.docs.get(doc)?.session.tree.root()?;
        let name = root.borrow().name.clone();
        Some((doc, name))
    }

    /// The one group node named `name` in a document; objects and groups inside objects are no layers.
    fn layer_node(&self, doc: usize, name: &str) -> Result<Node, String> {
        let file = self.docs.get(doc).ok_or("The layer's document is gone")?;
        let session = &file.session;

        if session.lookup.contains_key(name) {
            return Err(format!("{name} is an object, not a layer"));
        }

        let mut found = session.tree.get_nodes_by_name(name).into_iter();
        let Some(node) = found.next() else {
            return Err(format!("Layer {name} is gone"));
        };

        // layers go by name, so a name several groups share must not pick one of them
        if found.next().is_some() {
            return Err(format!("Several layers are named {name}"));
        }

        let inside = node
            .borrow()
            .ancestors()
            .iter()
            .any(|ancestor| session.lookup.contains_key(&ancestor.borrow().name));

        if inside {
            return Err(format!("{name} is inside an object, not a layer"));
        }

        Ok(node)
    }

    /// Make a layer the one new objects go to.
    pub fn set_current_layer(&mut self, doc: usize, name: &str) -> Result<(), String> {
        self.layer_node(doc, name)?;
        self.editable(doc)?; // a released document comes back first

        if self.docs[doc].display_only {
            return Err("This document is display only".into());
        }

        self.current_layer = Some((doc, name.to_string()));
        Ok(())
    }

    /// The rows a viewport click on `row` selects: every object of its outermost group, else the row.
    pub fn group_rows(&self, row: u32) -> Vec<u32> {
        if self.groups.is_empty() {
            return vec![row];
        }

        let Some((doc, _)) = self.identity_of(row) else {
            return vec![row];
        };
        let Some((node, true)) = self.node_of(row) else {
            return vec![row];
        };
        // ancestors run parent first, so the last group found is the outermost
        let outer = node.borrow().ancestors().into_iter().rev().find(|ancestor| {
            let ancestor = ancestor.borrow();
            ancestor.has_guid() && self.groups.contains(&(doc, Rc::from(ancestor.guid())))
        });
        let Some(outer) = outer else {
            return vec![row];
        };
        let mut rows: Vec<u32> = outer
            .borrow()
            .descendants()
            .iter()
            .filter_map(|member| self.row_of(doc, &member.borrow().name))
            .collect();
        rows.sort_unstable();
        rows.dedup();
        rows
    }

    /// True when the layer is current or holds the current one.
    pub fn holds_current(&self, doc: usize, name: &str) -> bool {
        let Some((current_doc, current)) = self.current_layer() else {
            return false;
        };

        if current_doc != doc {
            return false;
        }

        current == name
            || self.layer_node(doc, &current).is_ok_and(|node| {
                node.borrow()
                    .ancestors()
                    .iter()
                    .any(|ancestor| ancestor.borrow().name == name)
            })
    }

    /// Add a layer beside `at`, or under it as a sublayer; returns its name.
    pub fn new_layer(&mut self, doc: usize, at: &str, sublayer: bool) -> Result<String, String> {
        self.layer_node(doc, at)?;
        let key = self.step_key("new layer")?;

        self.layer_step(doc, &key, |session| {
            let node = group(session, at)?;
            // beside the root means under it; the parent's own name may be shared
            let parent = match node.borrow().parent() {
                Some(parent) if !sublayer => parent,
                _ => Rc::clone(&node),
            };
            let name = (1..)
                .map(|i| format!("Layer {i:02}"))
                .find(|name| !taken(session, name))
                .unwrap_or_default();
            session.add(&TreeNode::new(&name), Some(&parent));
            Ok(name)
        })
    }

    /// Rename a layer; names stay unique in their document.
    pub fn rename_layer(&mut self, doc: usize, name: &str, to: &str) -> Result<(), String> {
        let to = to.trim();
        let node = self.layer_node(doc, name)?;

        if node.borrow().parent().is_none() {
            return Err("The root layer cannot be renamed".into());
        }

        if to == name {
            return Ok(());
        }

        if to.is_empty() || to.contains('/') {
            return Err("A layer name must be non-empty and without /".into());
        }

        if taken(&self.docs[doc].session, to) {
            return Err(format!("{to} is already used in this document"));
        }

        let key = self.step_key("rename layer")?;
        self.layer_step(doc, &key, |session| {
            let node = group(session, name)?;

            if !session.rename_node(&node, to) {
                return Err("This layer cannot be renamed".into());
            }

            // a group transform follows its new name
            if let Some(xform) = session.xforms.get(name).cloned() {
                session.remove_xform(name);
                session.set_xform(to, xform);
            }

            Ok(())
        })?;

        if self.current_layer == Some((doc, name.to_string())) {
            self.current_layer = Some((doc, to.to_string()));
        }

        Ok(())
    }

    /// Delete a layer, its sublayers and their objects; refused for the current layer.
    pub fn delete_layer(&mut self, doc: usize, name: &str) -> Result<usize, String> {
        let node = self.layer_node(doc, name)?;

        if node.borrow().parent().is_none() {
            return Err("The root layer cannot be deleted".into());
        }

        if self.holds_current(doc, name) {
            return Err("The current layer cannot be deleted".into());
        }

        let key = self.step_key("delete layer")?;
        self.layer_step(doc, &key, |session| {
            let node = group(session, name)?;
            Ok(delete(session, &node))
        })
    }

    /// Copy a layer beside itself with its sublayers and objects; returns the copy's name.
    pub fn duplicate_layer(&mut self, doc: usize, name: &str) -> Result<String, String> {
        let node = self.layer_node(doc, name)?;

        if node.borrow().parent().is_none() {
            return Err("The root layer cannot be duplicated".into());
        }

        let key = self.step_key("duplicate layer")?;
        let (copy, guids) = self.layer_step(doc, &key, |session| {
            let node = group(session, name)?;
            let parent = node.borrow().parent().ok_or("Not a layer")?;
            let parts = parts(session, &node);
            let (copy, guids) = build(session, &parts, &parent, true)?;
            let name = copy.borrow().name.clone();
            Ok((name, guids))
        })?;

        for (from, to) in guids {
            self.inherit(&(doc, from.into()), &(doc, to.into()), true);
        }

        Ok(copy)
    }

    /// Move objects onto a layer, keeping them in place; returns their (document, guid) there.
    pub fn change_object_layer(
        &mut self,
        rows: &[u32],
        doc: usize,
        name: &str,
    ) -> Result<Vec<(usize, Rc<str>)>, String> {
        self.editable_rows(rows, doc)?;
        let layer = self.layer_node(doc, name)?;
        let mut objects = rows
            .iter()
            .map(|&row| self.identity_of(row).ok_or("Object is gone"))
            .collect::<Result<Vec<_>, _>>()?;
        objects.sort();
        objects.dedup();

        if objects.is_empty() {
            return Err("Select objects first".into());
        }

        let chosen: HashSet<(usize, &str)> = objects
            .iter()
            .map(|(owner, guid)| (*owner, guid.as_ref()))
            .collect();
        let mut local = Vec::new(); // guids of this document
        let mut moves: Vec<(usize, Vec<Rc<str>>)> = Vec::new(); // guids per other document
        let mut nodes = HashMap::new(); // the tree of the document being looked at
        let mut indexed = None; // which document that is

        for (owner, guid) in &objects {
            let file = self.docs.get(*owner).ok_or("Object is gone")?;

            if file.display_only {
                return Err("This document is display only".into());
            }

            if indexed != Some(*owner) {
                nodes = index(&file.session);
                indexed = Some(*owner);
            }

            let node = nodes.get(guid.as_ref()).cloned();

            // an object goes along with a chosen parent object
            if node.as_ref().is_some_and(|node| {
                node.borrow()
                    .ancestors()
                    .iter()
                    .any(|ancestor| chosen.contains(&(*owner, ancestor.borrow().name.as_str())))
            }) {
                continue;
            }

            if *owner != doc {
                match moves.last_mut() {
                    Some((last, guids)) if last == owner => guids.push(Rc::clone(guid)),
                    _ => moves.push((*owner, vec![Rc::clone(guid)])),
                }

                continue;
            }

            // a layer is never inside an object, so it cannot end up below itself
            let there = node.is_some_and(|node| {
                node.borrow()
                    .parent()
                    .is_some_and(|parent| Rc::ptr_eq(&parent, &layer))
            });

            if !there {
                local.push(Rc::clone(guid));
            }
        }

        if local.is_empty() && moves.is_empty() {
            return Err(format!("The objects are already on {name}"));
        }

        let key = self.step_key("change object layer")?;
        let place_at = self.docs[doc].place.clone();
        let mut sources = Vec::new(); // documents already edited
        let mut taken = Vec::new(); // (document, subtree, world placement) from other documents

        for (owner, guids) in moves {
            let from = self.docs[owner].place.clone();
            let parts = self.layer_step(owner, &key, |session| {
                let nodes = index(session);
                Ok(guids
                    .iter()
                    .map(|guid| {
                        let node = nodes.get(guid.as_ref());
                        let world =
                            node.map_or_else(|| session.xform(guid), |n| placement(session, n));
                        (take(session, guid, node), &from * &world)
                    })
                    .collect::<Vec<_>>())
            })?;
            sources.push(owner);
            taken.extend(
                parts
                    .into_iter()
                    .map(|(parts, world)| (owner, parts, world)),
            );
        }

        let result = self.layer_step(doc, &key, |session| {
            let layer = group(session, name)?;
            let inside =
                frame(session, &Xform::identity(), name).ok_or("The layer is degenerate")?;
            let outside = frame(session, &place_at, name).ok_or("The layer is degenerate")?;
            let nodes = index(session);
            let mut moved = Vec::new();
            let mut guids = Vec::new(); // (document, old guid, new guid) of every object brought in

            for guid in &local {
                // an object outside the tree gets a node
                let world = match nodes.get(guid.as_ref()) {
                    Some(node) => {
                        let world = placement(session, node);
                        session.add(node, Some(&layer));
                        world
                    }
                    None => {
                        session.add(&TreeNode::new(guid), Some(&layer));
                        session.xform(guid)
                    }
                };
                place(session, guid, &inside, &world);
                moved.push(guid.to_string());
            }

            for (owner, parts, world) in &taken {
                let (top, renamed) = build(session, parts, &layer, false)?;
                let guid = top.borrow().name.clone();
                place(session, &guid, &outside, world);
                moved.push(guid);
                guids.extend(renamed.into_iter().map(|(from, to)| (*owner, from, to)));
            }

            Ok((moved, guids))
        });

        // the other documents give their objects back when this one fails
        let (moved, guids) = match result {
            Ok(done) => done,
            Err(error) => {
                for owner in sources {
                    self.unstep(owner, &key);
                }

                return Err(error);
            }
        };

        for (owner, from, to) in guids {
            self.inherit(&(owner, from.into()), &(doc, to.into()), false);
        }

        Ok(moved.into_iter().map(|guid| (doc, guid.into())).collect())
    }

    /// Copy objects onto a layer of any document, keeping them in place; returns the copies.
    pub fn copy_object_layer(
        &mut self,
        rows: &[u32],
        doc: usize,
        name: &str,
    ) -> Result<Vec<(usize, Rc<str>)>, String> {
        self.editable_rows(rows, doc)?;
        self.layer_node(doc, name)?;
        let place_at = self.docs[doc].place.clone();
        // (identity, geometry, world placement) of each source
        let sources: Vec<((usize, Rc<str>), Geometry, Xform)> = rows
            .iter()
            .filter_map(|&row| {
                Some((
                    self.identity_of(row)?,
                    self.geometry(row)?.clone(),
                    self.placement_of(row)?,
                ))
            })
            .collect();

        if sources.is_empty() {
            return Err("Select objects first".into());
        }

        let key = self.step_key("copy object layer")?;
        let copies = self.layer_step(doc, &key, |session| {
            let layer = group(session, name)?;
            let back = frame(session, &place_at, name).ok_or("The layer is degenerate")?;
            let mut copies = Vec::new();

            for (_, geometry, world) in &sources {
                let node =
                    add(session, geometry, &layer, false).ok_or("Cannot copy this object")?;
                let guid = node.borrow().name.clone();
                place(session, &guid, &back, world);
                copies.push(guid);
            }

            Ok(copies)
        })?;
        let copies: Vec<(usize, Rc<str>)> =
            copies.into_iter().map(|guid| (doc, guid.into())).collect();

        for ((source, _, _), copy) in sources.iter().zip(&copies) {
            self.inherit(source, copy, false);
        }

        Ok(copies)
    }

    /// Give a copy the colors of its original, and its lock and visibility when `all`.
    pub(crate) fn inherit(&mut self, from: &(usize, Rc<str>), to: &(usize, Rc<str>), all: bool) {
        if let Some(color) = self.colors.get(from).copied() {
            self.colors.insert(to.clone(), color);
        }

        if let Some(color) = self.edge_colors.get(from).copied() {
            self.edge_colors.insert(to.clone(), color);
        }

        if all && self.locked.contains(from) {
            self.locked.insert(to.clone());
        }

        if all && self.hidden.contains(from) {
            self.hidden.insert(to.clone());
        }
    }

    /// A new undo label for one layer edit.
    pub(crate) fn step_key(&mut self, label: &str) -> Result<String, String> {
        self.layer_steps += 1;
        Ok(format!("{label} #{}", self.layer_steps))
    }

    /// Run one layer edit of document `doc` as the undo step `key`; a failed edit is aborted and leaves nothing.
    pub(crate) fn layer_step<T>(
        &mut self,
        doc: usize,
        key: &str,
        edit: impl FnOnce(&mut Session) -> Result<T, String>,
    ) -> Result<T, String> {
        self.editable(doc)?;
        let file = self
            .docs
            .get_mut(doc)
            .ok_or("The layer's document is gone")?;

        if file.display_only {
            return Err("This document is display only".into());
        }

        let session = Rc::make_mut(&mut file.session);
        session.begin(key);
        let result = edit(session);

        if result.is_err() {
            // nothing half done stays, and the redo steps survive
            session.abort();
            return result;
        }

        let notes = sync::commit(session);
        let stepped = newest(&session.history, true) == Some(key);
        self.noted(doc, notes);

        if stepped {
            self.edited(&[doc]);
        }

        self.row_revision = self.row_revision.wrapping_add(1);
        result
    }

    /// Take back a step just made in document `doc`, leaving nothing to redo.
    fn unstep(&mut self, doc: usize, key: &str) {
        let top = self.docs[doc].session.history.undo_stack.last();

        if top.is_some_and(|step| step.label == key) && self.step_document(doc, true).is_some() {
            let session = Rc::make_mut(&mut self.docs[doc].session);
            session.history.redo_stack.pop();

            // nor anything for undo to reach
            if let Some(step) = self.undo_steps.last_mut() {
                step.retain(|(held, label)| *held != doc || label != key);

                if step.is_empty() {
                    self.undo_steps.pop();
                }
            }
        }
    }

    /// Undo or redo the newest step of document `doc`; returns its label.
    pub(crate) fn step_document(&mut self, doc: usize, back: bool) -> Option<String> {
        let session = Rc::make_mut(&mut self.docs.get_mut(doc)?.session);
        let label = newest(&session.history, back)?.to_string();
        let stepped = if back { session.undo() } else { session.redo() };

        if !stepped {
            return None;
        }

        // the kernel records no graph edit: the added edge goes or comes back here
        if let Some(step) = self.edge_steps.get(&(doc, label.clone())) {
            put_edge(session, step, back);
        }

        let notes = sync::stepped(&session.history, back);

        // a renamed current layer follows
        if let Some((from, to)) = renamed(&session.history, back) {
            let (old, new) = if back { (to, from) } else { (from, to) };

            if self.current_layer.as_ref() == Some(&(doc, old)) {
                self.current_layer = Some((doc, new));
            }
        }

        self.noted(doc, notes);
        self.row_revision = self.row_revision.wrapping_add(1);
        Some(label)
    }
}

/// True for the label of a layer step, `<verb> #<n>` from `step_key`.
pub(crate) fn is_layer_key(label: &str) -> bool {
    label
        .rsplit_once(" #")
        .is_some_and(|(_, n)| n.parse::<u64>().is_ok())
}

/// The (from, to) names of a layer the step undo (`back`) or redo just took renamed.
fn renamed(history: &History, back: bool) -> Option<(String, String)> {
    let stack = if back {
        &history.redo_stack
    } else {
        &history.undo_stack
    };

    stack.last()?.ops.iter().find_map(|op| match op {
        session_rust::history::Op::Tree(t) if t.name_before != t.name_after => {
            Some((t.name_before.clone(), t.name_after.clone()))
        }
        _ => None,
    })
}

/// One node of a subtree being copied or moved.
struct Part {
    parent: Option<usize>,              // index of its parent part
    name: String,                       // group label or object guid
    geometry: Option<Geometry>,         // the object, None for a group
    xform: Option<Xform>,               // its local transform
    color: Option<session_rust::Color>, // its node color
}

/// The graph edge one Add Edge step made, and the vertices it made for it.
#[derive(Debug)]
pub(crate) struct EdgeStep {
    pub edge: Edge,            // the edge, its guid kept for redo
    pub vertices: Vec<String>, // vertices the step added, in order
}

/// Take away the edge a step added and its new vertices (`back`), or put them back.
fn put_edge(session: &mut Session, step: &EdgeStep, back: bool) {
    let graph = &mut session.graph;
    let (from, to) = (step.edge.v0.as_str(), step.edge.v1.as_str());

    if !back {
        for vertex in &step.vertices {
            graph.add_node(vertex, "");
        }

        let mut edge = step.edge.clone();
        edge.index = graph.edge_count;
        graph
            .edges
            .entry(to.to_string())
            .or_default()
            .insert(from.to_string(), edge.clone());
        graph
            .edges
            .entry(from.to_string())
            .or_default()
            .insert(to.to_string(), edge);
        graph.edge_count += 1;
        return;
    }

    let index = graph
        .edges
        .get(from)
        .and_then(|edges| edges.get(to))
        .map(|edge| edge.index);

    // the newest edge leaves without renumbering the others
    if index == Some(graph.edge_count - 1) {
        for (a, b) in [(from, to), (to, from)] {
            if let Some(edges) = graph.edges.get_mut(a) {
                edges.remove(b);
            }
        }

        graph.edge_count -= 1;
    } else {
        graph.remove_edge((from, to));
    }

    for vertex in &step.vertices {
        graph.remove_node(vertex);
    }
}

/// True when document `doc` can still undo or redo the step `label`.
pub(crate) fn in_history(docs: &[FileDoc], doc: usize, label: &str) -> bool {
    docs.get(doc).is_some_and(|file| {
        let history = &file.session.history;
        history
            .undo_stack
            .iter()
            .chain(&history.redo_stack)
            .any(|step| step.label == label)
    })
}

/// The label of the step undo (`back`) or redo takes next.
pub(crate) fn newest(history: &History, back: bool) -> Option<&str> {
    let stack = if back {
        &history.undo_stack
    } else {
        &history.redo_stack
    };
    stack.last().map(|step| step.label.as_str())
}

/// The group node `name` of a session.
fn group(session: &Session, name: &str) -> Result<Node, String> {
    if session.lookup.contains_key(name) {
        return Err(format!("{name} is an object, not a layer"));
    }

    session
        .tree
        .get_node_by_name(name)
        .ok_or_else(|| format!("Layer {name} is gone"))
}

/// True when a tree node or object already has this name.
fn taken(session: &Session, name: &str) -> bool {
    session.lookup.contains_key(name) || session.tree.get_node_by_name(name).is_some()
}

/// `base`, or `base 02`, `base 03` ... when it is taken.
fn unique(session: &Session, base: &str) -> String {
    if !taken(session, base) {
        return base.to_string();
    }

    (2..)
        .map(|i| format!("{base} {i:02}"))
        .find(|name| !taken(session, name))
        .unwrap_or_default()
}

/// The inverse of layer `name`'s world frame, in a document placed at `place`.
pub(crate) fn frame(session: &Session, place: &Xform, name: &str) -> Option<Xform> {
    (place * &session.world_xform(name)).inverse()
}

/// Give `guid` the local transform that puts it at `world` under a layer whose inverse frame is `back`.
pub(crate) fn place(session: &mut Session, guid: &str, back: &Xform, world: &Xform) {
    let local = back * world;
    let current = session.xform(guid);

    if local
        .m
        .iter()
        .zip(current.m)
        .any(|(a, b)| (a - b).abs() > 1e-12)
    {
        session.set_xform(guid, local);
    }
}

/// Every node of a session by name; object guids are unique.
pub(crate) fn index(session: &Session) -> HashMap<String, Node> {
    let mut nodes = HashMap::new();

    for node in session.tree.nodes() {
        let name = node.borrow().name.clone();
        nodes.insert(name, node);
    }

    nodes
}

/// A node's placement in its document: its own transform and every ancestor's.
pub(crate) fn placement(session: &Session, node: &Node) -> Xform {
    let node = node.borrow();
    let mut world = session.xform(&node.name);

    for ancestor in node.ancestors() {
        if let Some(xform) = session.xforms.get(&ancestor.borrow().name) {
            world = xform * &world;
        }
    }

    world
}

/// A subtree, parents before children.
fn parts(session: &Session, node: &Node) -> Vec<Part> {
    let mut parts = Vec::new();
    let mut stack = vec![(Rc::clone(node), None)];

    while let Some((node, parent)) = stack.pop() {
        let index = parts.len();
        let node = node.borrow();
        parts.push(Part {
            parent,
            name: node.name.clone(),
            geometry: session.lookup.get(&node.name).cloned(),
            xform: session.xforms.get(&node.name).cloned(),
            color: node.color.clone(),
        });

        for child in node.children().into_iter().rev() {
            stack.push((child, Some(index)));
        }
    }

    parts
}

/// Take an object and everything under its node out of a session.
fn take(session: &mut Session, guid: &str, node: Option<&Node>) -> Vec<Part> {
    let Some(node) = node else {
        // an object outside the tree goes alone
        let part = Part {
            parent: None,
            name: guid.to_string(),
            geometry: session.lookup.get(guid).cloned(),
            xform: session.xforms.get(guid).cloned(),
            color: None,
        };
        session.remove_object(guid);
        return vec![part];
    };
    let parts = parts(session, node);
    delete(session, node);
    parts
}

/// Delete a node, everything under it and their objects; returns the objects deleted.
fn delete(session: &mut Session, node: &Node) -> usize {
    let mut nodes = vec![Rc::clone(node)];
    nodes.extend(node.borrow().descendants());
    let mut count = 0;

    // deepest first, so a removed object never carries a child away
    for member in nodes.iter().rev() {
        let name = member.borrow().name.clone();

        if session.lookup.contains_key(&name) {
            count += usize::from(session.remove_object(&name));
        } else {
            session.remove_xform(&name);
        }
    }

    session.remove_group(node);
    count
}

/// Rebuild parts under `parent`, new guids when `fresh`; returns the top node and (old, new) guids.
fn build(
    session: &mut Session,
    parts: &[Part],
    parent: &Node,
    fresh: bool,
) -> Result<(Node, Vec<(String, String)>), String> {
    let mut nodes: Vec<Node> = Vec::with_capacity(parts.len());
    let mut inside: Vec<bool> = Vec::with_capacity(parts.len()); // under an object
    let mut guids = Vec::new();

    for part in parts {
        let under = part
            .parent
            .and_then(|index| nodes.get(index))
            .unwrap_or(parent)
            .clone();
        let within = part
            .parent
            .is_some_and(|index| inside[index] || parts[index].geometry.is_some());
        let node = match &part.geometry {
            Some(geometry) => {
                let keep = !fresh && !session.lookup.contains_key(&part.name);
                let node = add(session, geometry, &under, keep).ok_or("Cannot copy this object")?;
                guids.push((part.name.clone(), node.borrow().name.clone()));
                node
            }
            None => {
                // a group inside an object is no layer and keeps its name; `attributes` stay baked
                let name = if within || part.name == "attributes" {
                    part.name.clone()
                } else if fresh {
                    unique(session, &format!("{} copy", part.name))
                } else {
                    unique(session, &part.name)
                };
                let node = TreeNode::new(&name);
                node.borrow_mut().color = part.color.clone();
                session.add(&node, Some(&under));
                node
            }
        };

        if part.geometry.is_some() {
            node.borrow_mut().color = part.color.clone();
        }

        if let Some(xform) = &part.xform {
            let name = node.borrow().name.clone();
            session.set_xform(&name, xform.clone());
        }

        inside.push(within);
        nodes.push(node);
    }

    let top = nodes.first().cloned().ok_or("Nothing to copy")?;
    Ok((top, guids))
}

/// `node` as a node of `session`: a session copied for the edit has nodes of its own, found by their path.
pub(crate) fn owned(session: &Session, node: Option<Node>) -> Option<Node> {
    let node = node?;
    let mut path = Vec::new(); // child indices from the root down
    let mut top = Rc::clone(&node);

    loop {
        let parent = top.borrow().parent();
        let Some(parent) = parent else {
            break;
        };
        let index = parent
            .borrow()
            .children()
            .iter()
            .position(|child| Rc::ptr_eq(child, &top))?;
        path.push(index);
        top = parent;
    }

    let mut found = session.tree.root()?;

    if Rc::ptr_eq(&found, &top) {
        return Some(node);
    }

    for &index in path.iter().rev() {
        let child = found.borrow().children().get(index).cloned()?;
        found = child;
    }

    Some(found)
}

/// Add a geometry under `parent`, keeping its guid or with a fresh one.
pub(crate) fn add(
    session: &mut Session,
    geometry: &Geometry,
    parent: &Node,
    keep: bool,
) -> Option<Node> {
    let mut geometry = geometry.clone();

    if !keep {
        geometry.set_guid(TreeNode::new("").borrow().guid()); // a fresh uuid
    }

    let under = Some(parent);

    match geometry {
        Geometry::OBB(value) => {
            // a box lands under the root first, then moves
            let node = session.add_obb(Rc::unwrap_or_clone(value));
            session.add(&node, under);
            Some(node)
        }
        Geometry::BRep(value) => session.add_brep(Rc::unwrap_or_clone(value), under),
        Geometry::Element(value) => Some(session.add_element(Rc::unwrap_or_clone(value), under)),
        Geometry::Line(value) => Some(session.add_line(Rc::unwrap_or_clone(value), under)),
        Geometry::Mesh(value) => session.add_mesh(Rc::unwrap_or_clone(value), under),
        Geometry::NurbsCurve(value) => session.add_nurbscurve(Rc::unwrap_or_clone(value), under),
        Geometry::NurbsSurface(value) => {
            session.add_nurbssurface(Rc::unwrap_or_clone(value), under)
        }
        Geometry::Plane(value) => Some(session.add_plane(Rc::unwrap_or_clone(value), under)),
        Geometry::Point(value) => Some(session.add_point(Rc::unwrap_or_clone(value), under)),
        Geometry::PointCloud(value) => session.add_pointcloud(Rc::unwrap_or_clone(value), under),
        Geometry::Polyline(value) => session.add_polyline(Rc::unwrap_or_clone(value), under),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::scene::FileDoc;
    use session_rust::{Point, Polyline, Session, Xform};
    use std::rc::Rc;

    /// Two files: a point and a polyline, then a point.
    fn scene_with_two_files() -> Scene {
        let mut left = Session::new("left");
        left.add_point(Point::new(0.0, 0.0, 0.0), None);
        left.add_polyline(
            Polyline::new(vec![
                Point::new(0.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
                Point::new(1.0, 1.0, 0.0),
            ]),
            None,
        );
        let mut right = Session::new("right");
        right.add_point(Point::new(5.0, 0.0, 0.0), None);
        let mut scene = Scene::new();

        for (name, session) in [("left", left), ("right", right)] {
            scene.add_file(FileDoc {
                name: name.into(),
                session: Rc::new(session),
                place: Xform::identity(),
                point_px: 0.0,
                display_only: false,
            });
        }

        scene
    }

    /// Documents first, then only the kinds present.
    #[test]
    fn the_panel_lists_documents_then_the_kinds_present() {
        let scene = scene_with_two_files();
        let rows = rows(&scene);
        let labels: Vec<&str> = rows.iter().map(|r| r.label.as_str()).collect();
        assert_eq!(labels, vec!["left", "right", "curves", "points"]);
        assert_eq!(rows[0].count, 2, "left holds a point and a polyline");
        assert_eq!(rows[3].count, 2, "one point in each file");
    }

    /// Row counts match the rows a layer controls.
    #[test]
    fn bucket_counts_match_membership_with_mixed_visibility() {
        let mut scene = scene_with_two_files();
        scene.hidden.insert(scene.identity_of(0).unwrap());
        scene.hidden.insert(scene.identity_of(2).unwrap());

        for row in rows(&scene) {
            let members = of_layer(&scene, row.layer);
            assert_eq!(row.count, members.len());
            let hidden = members
                .iter()
                .all(|row| scene.hidden.contains(&scene.identity_of(*row).unwrap()));
            assert_eq!(row.hidden, hidden);
        }
    }

    /// A kind spans documents; a document is only its own.
    #[test]
    fn a_layer_names_the_rows_it_controls() {
        let scene = scene_with_two_files();
        assert_eq!(of_layer(&scene, Layer::Document(1)), vec![2]);
        assert_eq!(of_layer(&scene, Layer::Kind(Kind::Points)), vec![0, 2]);
        assert!(of_layer(&scene, Layer::Kind(Kind::Clouds)).is_empty());
    }

    /// A key parses back to its layer.
    #[test]
    fn a_row_key_survives_the_round_trip() {
        for layer in [
            Layer::Document(0),
            Layer::Document(17),
            Layer::Kind(Kind::Clouds),
        ] {
            assert_eq!(Layer::from_key(&layer.key()), Some(layer));
        }

        assert_eq!(Layer::from_key("kind:sandwiches"), None);
        assert_eq!(Layer::from_key("nonsense"), None);
    }

    /// A layer is hidden only when all its objects are.
    #[test]
    fn a_layer_is_hidden_when_all_of_it_is() {
        let mut scene = scene_with_two_files();
        assert!(!rows(&scene)[1].hidden);
        let identity = scene.identity_of(2).expect("row 2 exists");
        scene.hidden.insert(identity);
        let rows = rows(&scene);
        assert!(rows[1].hidden, "the whole of `right` is hidden");
        assert!(!rows[3].hidden, "only one of the two points is");
    }

    /// One document `site` with layer `walls` holding two joined points, and an empty layer `roof`.
    fn site() -> Scene {
        let mut session = Session::new("site");
        let walls = session.add_group("walls");
        session.add_group("roof");
        let a = session.add_point(Point::new(0.0, 0.0, 0.0), Some(&walls));
        let b = session.add_point(Point::new(1.0, 0.0, 0.0), Some(&walls));
        session.add_edge(&a.borrow().name, &b.borrow().name, "joint");
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

    /// The names of a layer's children.
    fn children(scene: &Scene, doc: usize, name: &str) -> Vec<String> {
        let node = scene.docs[doc].session.tree.get_node_by_name(name).unwrap();
        node.borrow()
            .children()
            .iter()
            .map(|child| child.borrow().name.clone())
            .collect()
    }

    /// Every node of a document as (name, parent), for comparing trees.
    fn shape(scene: &Scene, doc: usize) -> Vec<(String, String)> {
        scene.docs[doc]
            .session
            .tree
            .nodes()
            .iter()
            .map(|node| {
                let node = node.borrow();
                let parent = node.parent().map(|parent| parent.borrow().name.clone());
                (node.name.clone(), parent.unwrap_or_default())
            })
            .collect()
    }

    /// True when two placements agree.
    fn same(a: &Xform, b: &Xform) -> bool {
        a.m.iter().zip(b.m).all(|(a, b)| (a - b).abs() < 1e-9)
    }

    /// A new layer and sublayer undo and redo as tree steps.
    #[test]
    fn new_layers_undo_and_redo() {
        let mut scene = site();
        assert_eq!(scene.new_layer(0, "roof", false).unwrap(), "Layer 01");
        assert_eq!(scene.new_layer(0, "roof", true).unwrap(), "Layer 02");
        assert_eq!(
            children(&scene, 0, "site"),
            vec!["walls", "roof", "Layer 01"]
        );
        assert_eq!(children(&scene, 0, "roof"), vec!["Layer 02"]);
        assert!(scene.undo());
        assert!(children(&scene, 0, "roof").is_empty());
        assert!(scene.undo());
        assert_eq!(children(&scene, 0, "site"), vec!["walls", "roof"]);
        assert!(scene.redo());
        assert!(scene.redo());
        assert_eq!(children(&scene, 0, "roof"), vec!["Layer 02"]);
        assert!(
            scene.docs[0].session.xforms.is_empty(),
            "the step marker leaves nothing"
        );
        assert!(scene.new_layer(0, "nowhere", false).is_err());
    }

    /// A rename keeps names unique; the current layer and a group transform follow it through undo.
    #[test]
    fn rename_keeps_names_unique_and_the_current_layer_follows() {
        let mut scene = site();
        Rc::make_mut(&mut scene.docs[0].session)
            .set_xform("roof", Xform::translation(0.0, 0.0, 3.0));
        scene.set_current_layer(0, "roof").unwrap();

        for to in ["walls", " ", "a/b"] {
            assert!(scene.rename_layer(0, "roof", to).is_err(), "{to}");
        }

        assert!(scene.rename_layer(0, "site", "other").is_err());
        scene.rename_layer(0, "roof", " attic ").unwrap();
        assert_eq!(scene.current_layer(), Some((0, "attic".to_string())));
        assert!(scene.docs[0].session.xforms.contains_key("attic"));
        assert!(scene.undo());
        assert_eq!(children(&scene, 0, "site"), vec!["walls", "roof"]);
        assert_eq!(scene.current_layer(), Some((0, "roof".to_string())));
        assert!(scene.docs[0].session.xforms.contains_key("roof"));
        assert!(scene.redo());
        assert_eq!(scene.current_layer(), Some((0, "attic".to_string())));
    }

    /// The current layer and the root stay; another layer takes its objects, and undo brings them back.
    #[test]
    fn delete_refuses_the_current_layer_and_undo_restores_objects() {
        let mut scene = site();
        let before = shape(&scene, 0);
        let guid = scene.identity_of(0).unwrap().1;
        scene.set_current_layer(0, "walls").unwrap();
        assert!(scene.delete_layer(0, "walls").is_err());
        assert!(scene.delete_layer(0, "site").is_err());
        assert!(scene.holds_current(0, "site"));
        scene.set_current_layer(0, "roof").unwrap();
        assert_eq!(scene.delete_layer(0, "walls").unwrap(), 2);
        assert_eq!(children(&scene, 0, "site"), vec!["roof"]);
        assert!(scene.docs[0].session.lookup.is_empty());
        assert!(scene.undo());
        assert_eq!(shape(&scene, 0), before);
        assert_eq!(scene.docs[0].session.lookup.len(), 2);
        assert_eq!(
            scene.docs[0].session.get_neighbours(&guid).len(),
            1,
            "the joint is back"
        );
        assert!(scene.redo());
        assert_eq!(children(&scene, 0, "site"), vec!["roof"]);
        assert!(scene.docs[0].session.lookup.is_empty());
    }

    /// A duplicate holds copies with new guids and the colors of the originals.
    #[test]
    fn duplicate_copies_objects_with_new_guids() {
        let mut scene = site();
        scene
            .colors
            .insert(scene.identity_of(0).unwrap(), [255, 0, 0]);
        assert_eq!(scene.duplicate_layer(0, "walls").unwrap(), "walls copy");
        let copies = children(&scene, 0, "walls copy");
        assert_eq!(copies.len(), 2);
        assert!(
            copies
                .iter()
                .all(|guid| !children(&scene, 0, "walls").contains(guid))
        );
        assert_eq!(scene.docs[0].session.lookup.len(), 4);
        assert_eq!(scene.colors.len(), 2, "the copy keeps its color");
        let after = shape(&scene, 0);
        assert!(scene.undo());
        assert_eq!(scene.docs[0].session.lookup.len(), 2);
        assert_eq!(children(&scene, 0, "site"), vec!["walls", "roof"]);
        assert!(scene.redo());
        assert_eq!(shape(&scene, 0), after);
        assert_eq!(scene.docs[0].session.lookup.len(), 4);
        assert_eq!(scene.duplicate_layer(0, "walls").unwrap(), "walls copy 02");
        assert!(scene.duplicate_layer(0, "site").is_err());
    }

    /// A group under an object keeps its name when copied or moved, so `attributes` get no rows.
    #[test]
    fn a_group_under_an_object_keeps_its_name_through_duplicate_and_move() {
        let mut session = Session::new("site");
        let walls = session.add_group("walls");
        let beam = session.add_point(Point::new(0.0, 0.0, 0.0), Some(&walls));
        let attributes = TreeNode::new("attributes");
        session.add(&attributes, Some(&beam));
        session.add_point(Point::new(0.0, 1.0, 0.0), Some(&attributes));
        let beam = beam.borrow().name.clone();
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "site".into(),
            session: Rc::new(session),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        scene.duplicate_layer(0, "walls").unwrap();
        let after = shape(&scene, 0);
        let copy = children(&scene, 0, "walls copy")[0].clone();
        assert_eq!(children(&scene, 0, &copy), vec!["attributes"]);
        assert!(scene.undo());
        assert!(scene.redo());
        assert_eq!(shape(&scene, 0), after);
        assert_eq!(scene.docs[0].session.lookup.len(), 4);

        // into a document that has an `attributes` group of its own
        let mut other = Session::new("other");
        other.add_group("inbox");
        other.add_group("attributes");
        scene.add_file(FileDoc {
            name: "other".into(),
            session: Rc::new(other),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        let row = scene.row_of(0, &beam).unwrap();
        scene.change_object_layer(&[row], 1, "inbox").unwrap();
        assert_eq!(children(&scene, 1, &beam), vec!["attributes"]);
        let mut fresh = Scene::new();

        for file in &scene.docs {
            fresh.add_file(FileDoc {
                name: file.name.clone(),
                session: Rc::clone(&file.session),
                place: Xform::identity(),
                point_px: 0.0,
                display_only: false,
            });
        }

        assert_eq!(fresh.object_count(), 2, "beams only, no attributes");
    }

    /// A name several layers share picks none of them; a duplicate keeps `attributes` baked.
    #[test]
    fn a_shared_layer_name_is_refused_and_attributes_stay_baked() {
        let mut session = Session::new("site");

        // one group per element holding it and its `attributes`, as element files do
        for element in ["plate_0", "beam_1"] {
            let group = session.add_group(element);
            session.add_point(Point::new(0.0, 0.0, 0.0), Some(&group));
            let attributes = TreeNode::new("attributes");
            session.add(&attributes, Some(&group));
            session.add_point(Point::new(0.0, 1.0, 0.0), Some(&attributes));
        }

        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "site".into(),
            session: Rc::new(session),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        assert!(scene.delete_layer(0, "attributes").is_err());
        assert!(scene.set_current_layer(0, "attributes").is_err());
        assert!(scene.rename_layer(0, "attributes", "features").is_err());
        assert_eq!(scene.docs[0].session.lookup.len(), 4);
        scene.duplicate_layer(0, "plate_0").unwrap();
        assert_eq!(children(&scene, 0, "plate_0 copy")[1], "attributes");
        let mut fresh = Scene::new();
        fresh.add_file(FileDoc {
            name: "site".into(),
            session: Rc::clone(&scene.docs[0].session),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        assert_eq!(fresh.object_count(), 3, "elements only, no attributes");
    }

    /// Change Object Layer moves the node and keeps the world placement.
    #[test]
    fn change_object_layer_moves_and_keeps_placement() {
        let mut scene = site();
        let session = Rc::make_mut(&mut scene.docs[0].session);
        session
            .xforms
            .insert("roof".into(), Xform::translation(0.0, 0.0, 10.0));
        let before = scene.placement_of(0).unwrap();
        let moved = scene.change_object_layer(&[0], 0, "roof").unwrap();
        assert_eq!(moved, vec![scene.identity_of(0).unwrap()]);
        assert_eq!(children(&scene, 0, "roof").len(), 1);
        assert!(same(&scene.placement_of(0).unwrap(), &before));
        assert!(
            scene.change_object_layer(&[0], 0, "roof").is_err(),
            "already there"
        );
        assert!(scene.undo());
        assert!(children(&scene, 0, "roof").is_empty());
        assert!(same(&scene.placement_of(0).unwrap(), &before));
        assert!(scene.redo());
        assert_eq!(children(&scene, 0, "roof").len(), 1);
        assert!(same(&scene.placement_of(0).unwrap(), &before));
        assert!(scene.change_object_layer(&[], 0, "roof").is_err());
    }

    /// Change Object Layer carries an object into another document; one undo puts it back.
    #[test]
    fn change_object_layer_across_documents_is_one_step() {
        let mut scene = site();
        let mut other = Session::new("other");
        other.add_group("inbox");
        scene.add_file(FileDoc {
            name: "other".into(),
            session: Rc::new(other),
            place: Xform::translation(5.0, 0.0, 0.0),
            point_px: 0.0,
            display_only: false,
        });
        scene
            .colors
            .insert(scene.identity_of(1).unwrap(), [0, 0, 255]);
        let guid = scene.identity_of(1).unwrap().1;
        let moved = scene.change_object_layer(&[1], 1, "inbox").unwrap();
        assert_eq!(
            moved,
            vec![(1, Rc::clone(&guid))],
            "the object keeps its guid"
        );
        assert!(!scene.docs[0].session.lookup.contains_key(guid.as_ref()));
        assert_eq!(children(&scene, 1, "inbox"), vec![guid.to_string()]);
        let local = scene.docs[1].session.xform(&guid);
        assert_eq!(
            [local.m[12], local.m[13], local.m[14]],
            [-5.0, 0.0, 0.0],
            "same world spot"
        );
        assert_eq!(scene.colors.get(&(1, Rc::clone(&guid))), Some(&[0, 0, 255]));
        assert!(scene.undo());
        assert!(children(&scene, 1, "inbox").is_empty());
        assert_eq!(children(&scene, 0, "walls").len(), 2);
        assert_eq!(
            scene.docs[0].session.get_neighbours(&guid).len(),
            1,
            "its joint is back"
        );
        assert!(scene.redo());
        assert_eq!(children(&scene, 1, "inbox"), vec![guid.to_string()]);
        assert_eq!(children(&scene, 0, "walls").len(), 1);
    }

    /// `site` with a second document `other` holding an empty layer `inbox`.
    fn site_and_other() -> Scene {
        let mut scene = site();
        let mut other = Session::new("other");
        other.add_group("inbox");
        scene.add_file(FileDoc {
            name: "other".into(),
            session: Rc::new(other),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        scene
    }

    /// A move across documents undoes in both of them or in neither.
    #[test]
    fn a_move_across_documents_undoes_in_both_or_neither() {
        let mut scene = site_and_other();
        let guid = scene.identity_of(1).unwrap().1;
        let point = scene.identity_of(0).unwrap().1;
        scene.change_object_layer(&[1], 1, "inbox").unwrap();

        // a later edit in each document, the one in `site` last
        for (doc, moved) in [(1, &guid), (0, &point)] {
            let session = Rc::make_mut(&mut scene.docs[doc].session);
            session.begin("move");
            session.set_xform(moved, Xform::translation(1.0, 0.0, 0.0));
            session.commit();
            scene.edited(&[doc]);
        }

        let owners = |scene: &Scene| {
            [0, 1].map(|doc| scene.docs[doc].session.lookup.contains_key(guid.as_ref()))
        };
        assert!(scene.undo(), "the later edit in site");
        assert!(scene.undo(), "then the later edit in other");
        assert_eq!(owners(&scene), [false, true]);
        assert!(scene.undo(), "then the move, in both documents");
        assert_eq!(owners(&scene), [true, false]);
        assert!(scene.redo());
        assert_eq!(owners(&scene), [false, true]);
    }

    /// A move that one document can no longer step stays in the other too.
    #[test]
    fn a_move_one_document_forgot_is_not_undone_in_the_other() {
        let mut scene = site_and_other();
        let guid = scene.identity_of(1).unwrap().1;
        scene.change_object_layer(&[1], 1, "inbox").unwrap();
        let session = Rc::make_mut(&mut scene.docs[1].session);

        // `other` drops the move from its full history, then a layer edit there forgets old trees
        for step in 0..=session_rust::history::CAPACITY {
            session.begin("move");
            session.set_xform(&guid, Xform::translation(step as f64, 0.0, 0.0));
            session.commit();
        }

        scene.new_layer(1, "inbox", false).unwrap();
        assert!(scene.undo(), "the new layer");
        assert!(!scene.undo(), "the move other forgot");
        assert!(!scene.docs[0].session.lookup.contains_key(guid.as_ref()));
        assert!(scene.docs[1].session.lookup.contains_key(guid.as_ref()));
    }

    /// Copy Object Layer reaches another document and keeps the world placement.
    #[test]
    fn copy_object_layer_crosses_documents() {
        let mut scene = site();
        let mut other = Session::new("other");
        other.add_group("inbox");
        scene.add_file(FileDoc {
            name: "other".into(),
            session: Rc::new(other),
            place: Xform::translation(5.0, 0.0, 0.0),
            point_px: 0.0,
            display_only: false,
        });
        let copies = scene.copy_object_layer(&[1], 1, "inbox").unwrap();
        let guid = children(&scene, 1, "inbox")[0].clone();
        assert_eq!(copies, vec![(1, Rc::from(guid.as_str()))]);
        assert_ne!(guid, scene.identity_of(1).unwrap().1.to_string());
        let local = scene.docs[1].session.xform(&guid);
        assert_eq!(
            [local.m[12], local.m[13], local.m[14]],
            [-5.0, 0.0, 0.0],
            "same world spot"
        );
        assert_eq!(scene.docs[0].session.lookup.len(), 2, "the original stays");
        assert!(scene.undo());
        assert!(children(&scene, 1, "inbox").is_empty());
        assert!(scene.copy_object_layer(&[], 1, "inbox").is_err());
    }

    /// New objects go to the current layer, keeping the typed world coordinates.
    #[test]
    fn drawing_lands_on_the_current_layer() {
        let mut scene = site();
        Rc::make_mut(&mut scene.docs[0].session)
            .xforms
            .insert("roof".into(), Xform::translation(0.0, 0.0, 10.0));
        scene.set_current_layer(0, "roof").unwrap();
        let (doc, guid) = scene
            .model(&crate::app::modeling::Modeling::Point([1.0, 2.0, 3.0]))
            .unwrap()
            .unwrap();
        assert_eq!(doc, 0);
        assert_eq!(children(&scene, 0, "roof"), vec![guid.clone()]);
        let world = scene.docs[0].session.world_xform(&guid);
        assert_eq!([world.m[12], world.m[13], world.m[14]], [0.0, 0.0, 0.0]);
        assert!(scene.created_doc.is_none());
        assert!(
            scene.set_current_layer(0, &guid).is_err(),
            "an object is not a layer"
        );
        let session = Rc::make_mut(&mut scene.docs[0].session);
        let point = session.tree.get_node_by_name(&guid).unwrap();
        session.add(&TreeNode::new("features"), Some(&point));
        assert!(
            scene.set_current_layer(0, "features").is_err(),
            "nor a group inside an object"
        );
    }

    /// A failed edit leaves the tree, the objects and the redo steps as they were.
    #[test]
    fn a_failed_layer_edit_leaves_no_trace() {
        let mut scene = site();
        scene.new_layer(0, "roof", false).unwrap();
        assert!(scene.undo());
        let before = shape(&scene, 0);
        let result: Result<(), String> = scene.layer_step(0, "broken", |session| {
            session.add_point(Point::new(9.0, 9.0, 9.0), None);
            session.add_group("half");
            Err("broken".into())
        });
        assert!(result.is_err());
        assert_eq!(shape(&scene, 0), before);
        assert_eq!(scene.docs[0].session.lookup.len(), 2);
        assert!(scene.redo(), "the undone step can still be redone");
        assert_eq!(
            children(&scene, 0, "site"),
            vec!["walls", "roof", "Layer 01"]
        );
    }

    /// Past the history budget the oldest edits can no longer be undone; the rest still can.
    #[test]
    fn old_layer_edits_are_forgotten_past_the_budget() {
        let mut scene = site();
        Rc::make_mut(&mut scene.docs[0].session).history.budget = 1;

        for _ in 0..3 {
            scene.new_layer(0, "roof", false).unwrap();
        }

        let depth = scene.docs[0].session.history.undo_stack.len();
        assert_eq!(depth, 1);
        assert!(scene.undo());
        assert!(!scene.undo());
        assert_eq!(
            children(&scene, 0, "site"),
            vec!["walls", "roof", "Layer 01", "Layer 02"]
        );
    }
}
