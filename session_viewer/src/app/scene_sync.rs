use super::Scene;
use super::rows::{Cap, FREE, Footprint, GEOMETRY, Note, PLACE, PRESENCE, SINK, SUBTREE};
use crate::app::mesh_preview::MeshPreview;
use crate::app::surface_preview::SurfacePreview;
use crate::app::walk::bounds::{in_band, mark_pens_from};
use crate::app::walk::{Walk, WalkCx, is_drawable, walk_geometry};
use crate::engine::gpu::faces::FaceSource;
use crate::engine::gpu::glyphs::GlyphPoint;
use crate::engine::gpu::patch::{Counts, LaneId, Span};
use crate::engine::gpu::segments::CylinderSegment;
use crate::engine::gpu::{Gpu, Instance, ObjectRow, Upload};
use session_rust::history::{Op, Transaction};
use session_rust::{Geometry, History, Session, TreeNode, Xform};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};

type Node = Rc<RefCell<TreeNode>>;

/// Dead editable bytes that start a compaction, at least.
const COMPACT_MIN: u64 = 16 * 1024 * 1024;

/// Dead cloud points that start a cloud compaction, at least.
const CLOUD_COMPACT_MIN: u32 = 2_000_000;

/// Commit the open transaction; its ops name the objects it touched. Every viewer edit commits here.
pub(crate) fn commit(session: &mut Session) -> Vec<Note> {
    let notes = match &session.history.current {
        Some(transaction) => applied(transaction, true),
        None => Vec::new(),
    };
    session.commit();
    notes
}

/// Notes of the transaction an undo (`back`) or redo just stepped; `nodes` false after a tree swap.
pub(crate) fn stepped(history: &History, back: bool, nodes: bool) -> Vec<Note> {
    let notes = if back {
        history.redo_stack.last().map(|t| reverted(t, nodes))
    } else {
        history.undo_stack.last().map(|t| applied(t, nodes))
    };
    notes.unwrap_or_default()
}

/// Notes of a transaction done or redone.
fn applied(transaction: &Transaction, nodes: bool) -> Vec<Note> {
    let mut notes = Vec::with_capacity(transaction.ops.len());

    for op in &transaction.ops {
        let note = match op {
            Op::Add(t) => Note {
                guid: t.guid.as_str().into(),
                what: PRESENCE | GEOMETRY,
                node: None,
                parent: t.parent_guid.clone().map(|parent| (parent, t.index)),
            },
            // its node left the tree with every node below it
            Op::Remove(t) => Note {
                guid: t.guid.as_str().into(),
                what: PRESENCE | SUBTREE,
                node: t.node.as_ref().filter(|_| nodes).map(Rc::downgrade),
                parent: None,
            },
            Op::Replace(r) => Note::new(&r.guid, GEOMETRY),
            // the marker pair a tree-only layer step leaves
            Op::Xform(x) if x.guid == transaction.label => continue,
            Op::Xform(x) => Note::new(&x.guid, PLACE | SUBTREE),
            // definitions own no row until instances are drawn
            Op::Definition(_) => continue,
        };
        notes.push(note);
    }

    notes
}

/// Notes of a transaction undone, in the order its ops were reverted.
fn reverted(transaction: &Transaction, nodes: bool) -> Vec<Note> {
    let mut notes = Vec::with_capacity(transaction.ops.len());

    for op in transaction.ops.iter().rev() {
        let note = match op {
            Op::Add(t) => Note::new(&t.guid, PRESENCE),
            // its node came back with every node below it
            Op::Remove(t) => Note {
                guid: t.guid.as_str().into(),
                what: PRESENCE | GEOMETRY | SUBTREE,
                node: t.node.as_ref().filter(|_| nodes).map(Rc::downgrade),
                parent: None,
            },
            Op::Replace(r) => Note::new(&r.guid, GEOMETRY),
            Op::Xform(x) if x.guid == transaction.label => continue,
            Op::Xform(x) => Note::new(&x.guid, PLACE | SUBTREE),
            // definitions own no row until instances are drawn
            Op::Definition(_) => continue,
        };
        notes.push(note);
    }

    notes
}

/// One identity a sync looks at.
struct Work {
    doc: usize,                            // its document
    guid: Rc<str>,                         // its object or group name
    what: u8,                              // the note bits, merged
    weak: Option<Weak<RefCell<TreeNode>>>, // the node a note named
    parent: Option<(String, usize)>,       // where an added object was put
    node: Option<Node>,                    // its tree node, once resolved
    in_tree: bool,                         // that node hangs from the document's root
}

/// Add one identity, or merge it into the entry already there.
fn merge(work: &mut Vec<Work>, at: &mut HashMap<(usize, Rc<str>), usize>, item: Work) {
    match at.get(&(item.doc, Rc::clone(&item.guid))) {
        Some(&index) => {
            let entry = &mut work[index];
            entry.what |= item.what;

            if entry.weak.is_none() {
                entry.weak = item.weak;
            }

            if entry.parent.is_none() {
                entry.parent = item.parent;
            }

            if entry.node.is_none() && item.node.is_some() {
                entry.node = item.node;
                entry.in_tree = item.in_tree;
            }
        }
        None => {
            at.insert((item.doc, Rc::clone(&item.guid)), work.len());
            work.push(item);
        }
    }
}

/// The node when it is still `guid`'s, and whether it hangs from the document's root.
fn check(session: &Session, node: Node, guid: &str) -> Option<(Node, bool)> {
    if node.borrow().name != guid {
        return None;
    }

    let mut top = Rc::clone(&node);

    loop {
        let parent = top.borrow().parent();

        match parent {
            Some(parent) => top = parent,
            None => break,
        }
    }

    let in_tree = session
        .tree
        .root()
        .is_some_and(|root| Rc::ptr_eq(&root, &top));
    Some((node, in_tree))
}

/// The tree a node cache is filled from, by its root's address; a session moved by `Rc::make_mut` keeps it.
pub(crate) fn tree_key(session: &Session) -> usize {
    session
        .tree
        .root()
        .map_or(0, |root| Rc::as_ptr(&root) as usize)
}

/// True when an ancestor below the root is an `attributes` group: the object is drawn by its element.
fn baked(node: &Node) -> bool {
    let mut current = node.borrow().parent();

    while let Some(ancestor) = current {
        let parent = ancestor.borrow().parent();

        if parent.is_some() && ancestor.borrow().name == "attributes" {
            return true;
        }

        current = parent;
    }

    false
}

/// A dragged row's previews, captured from a walk of that row alone.
pub(crate) struct Previews {
    pub mesh: Option<MeshPreview>,       // vertex keys of a mesh
    pub surface: Option<SurfacePreview>, // surface parameters of a BRep or surface
    span: Span,                          // where the captured rows sit
}

impl Scene {
    /// True while an edit's notes wait for a sync.
    pub(crate) fn has_pending(&self) -> bool {
        !self.pending.is_empty()
    }

    /// Queue what an edit did to document `doc`.
    pub(crate) fn noted(&mut self, doc: usize, notes: Vec<Note>) {
        self.pending
            .extend(notes.into_iter().map(|note| (doc, note)));
    }

    /// Remember a tree node the edit site already holds.
    pub(crate) fn hint(&mut self, doc: usize, node: &Node) {
        self.hints.push((doc, Rc::downgrade(node)));
    }

    /// Note every identity of every editable document, as if all of it changed.
    #[cfg(test)]
    pub(crate) fn touch_all(&mut self) {
        for doc in 0..self.docs.len() {
            if self.docs[doc].display_only {
                continue;
            }

            let session = Rc::clone(&self.docs[doc].session);

            for guid in session.order() {
                self.pending
                    .push((doc, Note::new(&guid, PRESENCE | GEOMETRY | PLACE)));
            }
        }

        // rows whose object is gone
        for row in 0..self.row_count() {
            let owner = self.owners[row];

            if self.docs.get(owner).is_some_and(|file| {
                !file.display_only && !file.session.lookup.contains_key(self.order[row].as_ref())
            }) {
                self.pending
                    .push((owner, Note::new(&self.order[row], PRESENCE)));
            }
        }
    }

    /// Fix only the rows named by `commit`, `stepped` and `LayerStep::touched`; the GPU work waits in `staged`.
    pub(crate) fn sync(&mut self) {
        let pending = std::mem::take(&mut self.pending);
        let hints = std::mem::take(&mut self.hints);
        let mut work = Vec::new();
        let mut at = HashMap::new();

        for (doc, note) in pending {
            if self.docs.get(doc).is_none_or(|file| file.display_only) {
                continue;
            }

            let item = Work {
                doc,
                guid: note.guid,
                what: note.what,
                weak: note.node,
                parent: note.parent,
                node: None,
                in_tree: false,
            };
            merge(&mut work, &mut at, item);
        }

        // a tree that changed under the cache is walked once here, not once per row by a later lookup
        for doc in 0..self.docs.len() {
            self.fresh_nodes(doc);
        }

        self.find_nodes(&mut work, hints);
        self.expand(&mut work, &mut at);
        let mut changed = false;

        for item in &work {
            changed |= self.reconcile(item);
        }

        self.ids.settle();

        if changed {
            self.row_revision = self.row_revision.wrapping_add(1);
        }
    }

    /// Find each identity's tree node: a hint, the note's node, the row's cache, then one search per document.
    fn find_nodes(&mut self, work: &mut [Work], hints: Vec<(usize, Weak<RefCell<TreeNode>>)>) {
        let mut hinted: HashMap<(usize, String), Node> = HashMap::new();

        for (doc, weak) in hints {
            if let Some(node) = weak.upgrade() {
                let name = node.borrow().name.clone();
                hinted.insert((doc, name), node);
            }
        }

        let mut misses: HashMap<usize, Vec<usize>> = HashMap::new(); // document to work items

        for (index, item) in work.iter_mut().enumerate() {
            let session = Rc::clone(&self.docs[item.doc].session);
            let hint = hinted.remove(&(item.doc, item.guid.to_string()));
            let noted = item.weak.take().and_then(|weak| weak.upgrade());
            let found = hint
                .and_then(|node| check(&session, node, &item.guid))
                .or_else(|| noted.and_then(|node| check(&session, node, &item.guid)))
                .or_else(|| self.cached(item.doc, &item.guid))
                .or_else(|| self.beside_parent(item, &session));

            // an object gone from its document only loses its row: no tree walk for it
            let gone = item.what & SUBTREE == 0 && !session.lookup.contains_key(item.guid.as_ref());

            match found {
                Some((node, in_tree)) => {
                    item.node = Some(node);
                    item.in_tree = in_tree;
                }
                None if gone => {}
                None => misses.entry(item.doc).or_default().push(index),
            }
        }

        for (doc, indices) in misses {
            let names: HashSet<&str> = indices.iter().map(|&i| work[i].guid.as_ref()).collect();
            let found = self.search(doc, &names);

            for index in indices {
                if let Some(node) = found.get(work[index].guid.as_ref()) {
                    work[index].node = Some(Rc::clone(node));
                    work[index].in_tree = true;
                }
            }
        }
    }

    /// The cached node of an identity's row, while the cache is for this session.
    fn cached(&self, doc: usize, guid: &Rc<str>) -> Option<(Node, bool)> {
        self.cache_hit(*self.guid_to_row.get(&(doc, Rc::clone(guid)))?)
    }

    /// A row's cached tree node, when it is still valid.
    fn cache_hit(&self, row: u32) -> Option<(Node, bool)> {
        let (doc, guid) = self.identity_of(row)?;
        let file = self.docs.get(doc)?;

        if self.doc_state.get(doc)?.nodes_from != tree_key(&file.session) {
            return None;
        }

        let node = self.nodes.get(row as usize)?.upgrade()?;
        check(&file.session, node, &guid)
    }

    /// An added object's node among its parent's children, when the parent is found cheaply.
    fn beside_parent(&self, item: &Work, session: &Session) -> Option<(Node, bool)> {
        let (name, index) = item.parent.as_ref()?;
        let root = session.tree.root()?;
        let parent = if root.borrow().name == *name {
            root
        } else {
            let row = self.row_of(item.doc, name)?;
            let (parent, _) = self.cache_hit(row)?;
            parent
        };
        let children = parent.borrow().children();
        let guess = children
            .get(*index)
            .filter(|c| c.borrow().name == *item.guid);
        let node = match guess {
            Some(node) => Rc::clone(node),
            None => children
                .into_iter()
                .find(|c| c.borrow().name == *item.guid)?,
        };
        check(session, node, &item.guid)
    }

    /// One walk of a document's tree for `names`; it refills the row node cache when that is stale.
    fn search(&mut self, doc: usize, names: &HashSet<&str>) -> HashMap<String, Node> {
        #[cfg(test)]
        {
            self.searches += 1;
        }

        let session = Rc::clone(&self.docs[doc].session);
        let refill = self.doc_state[doc].nodes_from != tree_key(&session);
        let mut found = HashMap::new();

        if refill {
            for row in 0..self.row_count() {
                if self.owners[row] == doc {
                    self.nodes[row] = Weak::new();
                }
            }

            self.doc_state[doc].nodes_from = tree_key(&session);
        }

        let Some(root) = session.tree.root() else {
            return found;
        };
        let mut stack = vec![root];

        while let Some(node) = stack.pop() {
            let borrowed = node.borrow();

            if refill && let Some(row) = self.row_of(doc, &borrowed.name) {
                self.nodes[row as usize] = Rc::downgrade(&node);
            }

            if names.contains(borrowed.name.as_str()) {
                found.insert(borrowed.name.clone(), Rc::clone(&node));

                if !refill && found.len() == names.len() {
                    break;
                }
            }

            stack.extend(borrowed.children().into_iter().rev());
        }

        found
    }

    /// The tree of `doc` was replaced: its cached nodes refill on the next use.
    pub(crate) fn forget_nodes(&mut self, doc: usize) {
        if let Some(state) = self.doc_state.get_mut(doc) {
            state.nodes_from = 0;
        }
    }

    /// Fill the node cache of every row of `doc` from one walk of its tree.
    pub(crate) fn refill_nodes(&mut self, doc: usize) {
        self.doc_state[doc].nodes_from = 0;
        self.search(doc, &HashSet::new());
    }

    /// Refill the node cache of `doc` when its tree changed: a stale cache makes every lookup walk the tree.
    pub(crate) fn fresh_nodes(&mut self, doc: usize) {
        let Some(file) = self.docs.get(doc) else {
            return;
        };

        if !file.display_only && self.doc_state[doc].nodes_from != tree_key(&file.session) {
            self.refill_nodes(doc);
        }
    }

    /// Add every object below each SUBTREE identity, to be placed and judged again.
    fn expand(&mut self, work: &mut Vec<Work>, at: &mut HashMap<(usize, Rc<str>), usize>) {
        for index in 0..work.len() {
            if work[index].what & SUBTREE == 0 {
                continue;
            }

            let Some(node) = work[index].node.clone() else {
                continue;
            };
            let doc = work[index].doc;
            let in_tree = work[index].in_tree;
            let session = Rc::clone(&self.docs[doc].session);
            let mut stack = node.borrow().children();

            while let Some(child) = stack.pop() {
                let name = child.borrow().name.clone();

                if session.lookup.contains_key(&name) {
                    let item = Work {
                        doc,
                        guid: name.as_str().into(),
                        what: PLACE | PRESENCE,
                        weak: None,
                        parent: None,
                        node: Some(Rc::clone(&child)),
                        in_tree,
                    };
                    merge(work, at, item);
                }

                stack.extend(child.borrow().children());
            }
        }
    }

    /// Kill, create, redraw or move the row of one identity; true when a row came or went.
    fn reconcile(&mut self, item: &Work) -> bool {
        let session = Rc::clone(&self.docs[item.doc].session);
        let geometry = session.lookup.get(item.guid.as_ref());
        let under = item.in_tree && item.node.as_ref().is_some_and(baked);
        let wanted = geometry.is_some_and(is_drawable) && !under;
        let row = self
            .guid_to_row
            .get(&(item.doc, Rc::clone(&item.guid)))
            .copied();

        match (row, geometry) {
            (Some(row), _) if !wanted => {
                self.kill(row);
                true
            }
            (None, Some(geometry)) if wanted => {
                self.create(item, geometry);
                true
            }
            (Some(row), Some(geometry)) => {
                let place =
                    self.world_place(item.doc, item.node.as_ref(), item.in_tree, &item.guid);

                if let Some(node) = &item.node {
                    self.nodes[row as usize] = Rc::downgrade(node);
                }

                if item.what & GEOMETRY != 0 {
                    self.redraw_row(row, item.doc, &item.guid, geometry, place, false);
                } else {
                    self.staged.places.push((row, place));
                }

                false
            }
            _ => false,
        }
    }

    /// A document's placement times every transform from its root down to the node, as `add_file` computes it.
    pub(crate) fn world_place(
        &self,
        doc: usize,
        node: Option<&Node>,
        in_tree: bool,
        guid: &str,
    ) -> Xform {
        let file = &self.docs[doc];
        let xforms = &file.session.xforms;

        if xforms.is_empty() {
            return file.place.clone();
        }

        if in_tree && let Some(node) = node {
            let mut path = vec![Rc::clone(node)];

            loop {
                let parent = path.last().and_then(|last| last.borrow().parent());

                match parent {
                    Some(parent) => path.push(parent),
                    None => break,
                }
            }

            let mut acc = Xform::identity();

            for step in path.iter().rev() {
                if let Some(local) = xforms.get(&step.borrow().name) {
                    acc = &acc * local;
                }
            }

            return &file.place * &acc;
        }

        match xforms.get(guid) {
            Some(local) => &file.place * local,
            None => file.place.clone(),
        }
    }

    /// The tree node of a row and whether it hangs from its document's root.
    pub(crate) fn node_of(&self, row: u32) -> Option<(Node, bool)> {
        if let Some(found) = self.cache_hit(row) {
            return Some(found);
        }

        let (doc, guid) = self.identity_of(row)?;
        self.docs
            .get(doc)?
            .session
            .tree
            .get_node_by_name(&guid)
            .map(|node| (node, true))
    }

    /// One object walked on its own at vertex 0: its rows and its object row without colors.
    fn walk_one(
        &self,
        doc: usize,
        row: u32,
        geometry: &Geometry,
        place: &Xform,
    ) -> (Upload, ObjectRow) {
        let file = &self.docs[doc];
        let mut up = Upload::default();
        let cx = WalkCx {
            vert_base: 0,
            cloud_px: file.point_px,
            row,
            attributes: self.attributes,
        };
        let walked = walk_geometry(&mut Walk::of(&mut up), &cx, geometry);
        let mut object = ObjectRow::new(place.clone(), walked.flags);
        object.bounds = walked.bounds;
        object.spacing = walked.spacing;
        object.faces = walked.faces;

        if walked.faces {
            object.flags |= Instance::FLAG_HAS_FACES;
        }

        // an object drawn flat into a drawing sheet takes the sheet's pens
        if let Some(band) = self.doc_state[doc].sheet
            && in_band(band, &walked.bounds, place, &file.place)
        {
            object.flags |= Instance::FLAG_SHEET;
            mark_pens_from(&mut up, 0, 0);
        }

        (up, object)
    }

    /// Give an identity a row: a freed id, else one after every row.
    fn create(&mut self, item: &Work, geometry: &Geometry) {
        let doc = item.doc;
        let place = self.world_place(doc, item.node.as_ref(), item.in_tree, &item.guid);
        let row = match self.ids.take() {
            Some(row) => {
                let i = row as usize;
                self.order[i] = Rc::clone(&item.guid);
                self.owners[i] = doc;
                row
            }
            None => {
                let row = self.object_rows + self.tables.obj.rows.len() as u32;
                self.tables
                    .obj
                    .rows
                    .push(ObjectRow::new(Xform::identity(), 0));
                self.order.push(Rc::clone(&item.guid));
                self.owners.push(doc);
                self.feet.push(Footprint::None);
                self.nodes.push(Weak::new());
                row
            }
        };
        let i = row as usize;
        self.nodes[i] = item.node.as_ref().map(Rc::downgrade).unwrap_or_default();
        self.guid_to_row.insert((doc, Rc::clone(&item.guid)), row);
        let (up, walked) = self.walk_one(doc, row, geometry, &place);
        let hidden = if self.hidden.contains(&(doc, Rc::clone(&item.guid))) {
            Instance::FLAG_HIDDEN
        } else {
            0
        };
        let mut object = self.object_row(doc, &item.guid, place, hidden | walked.flags);
        object.bounds = walked.bounds;
        object.spacing = walked.spacing;
        object.faces = walked.faces;
        let cloud = matches!(geometry, Geometry::PointCloud(_));
        self.feet[i] = self.place_rows(doc, &item.guid, up, cloud);

        if row < self.object_rows {
            self.staged.rows.push((row, object));
        } else {
            let world = object.bounds.transformed(&object.place);

            if world.is_valid() {
                self.tables.bounds.union_with(&world);
            }

            self.tables.obj.rows[(row - self.object_rows) as usize] = object;
        }
    }

    /// Put walked rows back in the identity's grave when they fill it exactly, else after every row.
    fn place_rows(&mut self, doc: usize, guid: &Rc<str>, mut up: Upload, cloud: bool) -> Footprint {
        let counts = Counts::of(&up);
        let key = (doc, Rc::clone(guid));

        if !cloud
            && !counts.is_empty()
            && let Some(&grave) = self.graves.get(&key)
        {
            let span = self.spans.span(grave);

            if span.count == counts {
                self.graves.remove(&key);
                up.shift_vertices(span.start.verts);
                self.staged.patches.push((span.start, up));
                self.dead = self.dead.minus(counts);
                return grave;
            }
        }

        self.append(up, cloud)
    }

    /// Rows after every row on the GPU and in the tables.
    fn append(&mut self, up: Upload, cloud: bool) -> Footprint {
        let start = self.uploaded.plus(Counts::of(&self.tables));
        let count = Counts::of(&up);
        self.tables.merge(up, start.verts);

        if cloud {
            return Footprint::Cloud;
        }

        self.spans.foot(Span { start, count })
    }

    /// Draw `row` from `geometry` again: at its grave, in its own rows, or after every row.
    fn redraw_row(
        &mut self,
        row: u32,
        doc: usize,
        guid: &Rc<str>,
        geometry: &Geometry,
        place: Xform,
        preview: bool,
    ) {
        let (mut up, walked) = self.walk_one(doc, row, geometry, &place);
        let new = Counts::of(&up);
        let i = row as usize;
        let foot = self.feet[i];
        let cur = self.spans.span(foot);
        let cap = self.caps.remove(&row);
        let alloc = cap.map_or(cur.count, |cap| cap.alloc);
        let key = (doc, Rc::clone(guid));
        // a committed allocation exactly its content waits as the identity's grave; a preview one never
        let exact = cap.is_none() && !cur.count.is_empty();
        let cloud = foot == Footprint::Cloud || matches!(geometry, Geometry::PointCloud(_));
        let grave = self
            .graves
            .get(&key)
            .copied()
            .filter(|grave| !cloud && !new.is_empty() && self.spans.span(*grave).count == new);

        self.feet[i] = if cloud {
            if foot == Footprint::Cloud {
                self.staged.clouds.push(row);
            } else {
                self.retire(cur, exact, &key, foot);
            }

            self.append(up, matches!(geometry, Geometry::PointCloud(_)))
        } else if let Some(grave) = grave {
            let at = self.spans.span(grave).start;
            self.graves.remove(&key);
            self.retire(cur, exact, &key, foot);
            self.dead = self.dead.minus(new);
            up.shift_vertices(at.verts);
            self.staged.patches.push((at, up));
            grave
        } else if new.fits(&alloc) {
            // in place; rows the new content leaves die
            self.tails(cur.start, new, cur.count);
            self.dead = self.dead.plus(cur.count).minus(new);
            up.shift_vertices(cur.start.verts);
            self.staged.patches.push((cur.start, up));
            self.spans.release(foot);
            let span = Span {
                start: cur.start,
                count: new,
            };

            // a preview allocation stays one until a commit lands in it
            let preview = preview && cap.is_some_and(|cap| cap.preview);

            if new == alloc && !preview {
                self.spans.foot(span)
            } else {
                self.caps.insert(row, Cap { alloc, preview });
                self.spans.keep(span)
            }
        } else {
            self.retire(cur, exact, &key, foot);

            if preview {
                self.append_with_headroom(row, up)
            } else {
                self.append(up, false)
            }
        };

        self.staged.geometry.push((row, walked));
        self.bounds_stale = true;
    }

    /// Hand an allocation's content to the sink; it waits as the identity's grave when `grave`.
    fn retire(&mut self, cur: Span, grave: bool, key: &(usize, Rc<str>), foot: Footprint) {
        self.kill_lanes(cur.start, cur.count);
        self.dead = self.dead.plus(cur.count);

        if grave {
            if let Some(old) = self.graves.insert(key.clone(), foot) {
                self.spans.release(old);
            }
        } else {
            self.spans.release(foot);
        }
    }

    /// Stage the kill of `count` rows per lane from `start`.
    fn kill_lanes(&mut self, start: Counts, count: Counts) {
        if count.is_empty() {
            return;
        }

        self.sink_row();

        for (lane, rows) in count.each() {
            if rows > 0 {
                self.staged.kills.push((lane, start.get(lane), rows));
            }
        }
    }

    /// Stage the death of the rows past `new` up to `cur`, lane by lane, after an in-place redraw.
    fn tails(&mut self, start: Counts, new: Counts, cur: Counts) {
        for (lane, rows) in cur.each() {
            let keep = new.get(lane);

            if rows <= keep {
                continue;
            }

            let first = start.get(lane) + keep;

            match lane {
                LaneId::Faces | LaneId::Print | LaneId::Text => {
                    self.staged
                        .tails
                        .push((lane, first, rows - keep, start.verts))
                }
                _ => {
                    self.sink_row();
                    self.staged.kills.push((lane, first, rows - keep));
                }
            }
        }
    }

    /// Append a preview's rows with half again as many dead rows behind, so later frames fit.
    fn append_with_headroom(&mut self, row: u32, up: Upload) -> Footprint {
        let sink = self.sink_row();
        let start = self.uploaded.plus(Counts::of(&self.tables));
        let new = Counts::of(&up);
        self.tables.merge(up, start.verts);
        let mut alloc = new;
        let pad = |rows: u32| rows.div_ceil(2);
        let t = &mut self.tables;
        let vertex = start.verts;

        for _ in 0..pad(new.verts) {
            t.arena.verts.push(bytemuck::Zeroable::zeroed());
            t.arena.vids.push(sink);
        }

        let dead_index = |rows: u32| pad(rows).div_ceil(3) * 3;

        for _ in 0..dead_index(new.faces) {
            t.arena.idx.push(vertex);
        }

        for _ in 0..dead_index(new.print) {
            t.arena.idx_print.push(vertex);
        }

        for _ in 0..dead_index(new.text) {
            t.arena.idx_text.push(vertex);
        }

        for _ in 0..pad(new.sources) {
            t.arena.face_sources.push(FaceSource {
                parent: u32::MAX,
                face: 0,
            });
        }

        let segment = CylinderSegment {
            p0: [0.0; 3],
            radius: 0.0,
            p1: [0.0; 3],
            instance_id: sink,
            color: 0,
            facing: u32::MAX,
        };
        t.seg.pipe_ids.resize(t.seg.pipes.len(), u32::MAX);
        t.seg.ribbon_ids.resize(t.seg.ribbons.len(), u32::MAX);

        for _ in 0..pad(new.pipes) {
            t.seg.pipes.push(segment);
            t.seg.pipe_ids.push(u32::MAX);
        }

        for _ in 0..pad(new.ribbons) {
            t.seg.ribbons.push(segment);
            t.seg.ribbon_ids.push(u32::MAX);
        }

        let glyph = GlyphPoint {
            instance_id: sink,
            ..bytemuck::Zeroable::zeroed()
        };

        for _ in 0..pad(new.spheres) {
            t.glyph.spheres.push(glyph);
        }

        for _ in 0..pad(new.dots) {
            t.glyph.dots.push(glyph);
        }

        alloc.verts += pad(new.verts);
        alloc.faces += dead_index(new.faces);
        alloc.print += dead_index(new.print);
        alloc.text += dead_index(new.text);
        alloc.sources += pad(new.sources);
        alloc.pipes += pad(new.pipes);
        alloc.ribbons += pad(new.ribbons);
        alloc.spheres += pad(new.spheres);
        alloc.dots += pad(new.dots);
        self.dead = self.dead.plus(alloc).minus(new);
        let span = Span { start, count: new };

        if alloc == new {
            return self.spans.foot(span);
        }

        self.caps.insert(
            row,
            Cap {
                alloc,
                preview: true,
            },
        );
        self.spans.keep(span)
    }

    /// Drop an identity's row: its lane rows go to the sink, its id waits for the end of the sync.
    fn kill(&mut self, row: u32) {
        let i = row as usize;
        let doc = self.owners[i];
        let guid = std::mem::replace(&mut self.order[i], Rc::clone(&self.empty));
        let foot = self.feet[i];
        let key = (doc, guid);

        if foot == Footprint::Cloud {
            self.staged.clouds.push(row);
        } else {
            let cap = self.caps.remove(&row);
            let span = self.spans.span(foot);
            let grave = cap.is_none() && !span.count.is_empty();
            self.retire(span, grave, &key, foot);
        }

        self.staged.retire.push(row);
        self.guid_to_row.remove(&key);
        self.owners[i] = FREE;
        self.feet[i] = Footprint::None;
        self.nodes[i] = Weak::new();
        self.ids.give(row);
        self.bounds_stale = true;

        if self.preview.as_ref().is_some_and(|(held, _)| *held == row) {
            self.preview = None;
        }
    }

    /// The hidden row dead lane rows point at, made at the first kill.
    fn sink_row(&mut self) -> u32 {
        if let Some(sink) = self.sink {
            return sink;
        }

        let row = self.object_rows + self.tables.obj.rows.len() as u32;
        self.tables.obj.rows.push(ObjectRow::new(
            Xform::identity(),
            Instance::FLAG_HIDDEN | Instance::FLAG_DEAD,
        ));
        self.order.push(Rc::clone(&self.empty));
        self.owners.push(SINK);
        self.feet.push(Footprint::None);
        self.nodes.push(Weak::new());
        self.sink = Some(row);
        row
    }

    /// Draw `row` from `geometry` without touching its document; `preview` during a drag.
    pub(crate) fn redraw(&mut self, row: u32, geometry: &Geometry, preview: bool) {
        let Some((doc, guid)) = self.identity_of(row) else {
            return;
        };

        if self.docs.get(doc).is_none_or(|file| file.display_only) {
            return;
        }

        let Some(place) = self.placement_of(row) else {
            return;
        };

        self.redraw_row(row, doc, &guid, geometry, place, preview);
    }

    /// True when the dead editable rows outweigh the live ones and 16 MiB.
    pub(crate) fn compaction_due(&self) -> bool {
        let dead = self.dead.bytes();
        let live = self.uploaded.bytes().saturating_sub(dead);
        dead > 0 && dead >= COMPACT_MIN.max(live)
    }

    /// True when the dropped cloud points outweigh the live ones and two million.
    pub(crate) fn cloud_compaction_due(&self, gpu: &Gpu) -> bool {
        let live = gpu.cloud.point_count.saturating_sub(self.dead_points);
        self.dead_points > 0 && self.dead_points >= CLOUD_COMPACT_MIN.max(live)
    }

    /// Copy the live cloud points together into buffers of exact size.
    pub(crate) fn compact_clouds(&mut self, gpu: &mut Gpu) {
        gpu.cloud.compact(&gpu.ctx);
        gpu.splat
            .rebind(&gpu.ctx, &gpu.layouts, gpu.cloud.buffers());
        gpu.splat.invalidate();
        gpu.splat.set_point(None);
        self.dead_points = 0;
        self.compactions += 1;
        gpu.set_dead(self.dead, 0);
    }

    /// Walk every editable document again into fresh lanes, in load order; ids and everything else stay.
    /// False, having asked for them, while a released document would lose its rows.
    pub fn rewalk_editable(&mut self, gpu: &mut Gpu) -> bool {
        if self.want_all() {
            return false;
        }

        self.sync();
        self.upload_to(gpu);
        gpu.release_editable();
        self.forget_editable();

        for doc in 0..self.docs.len() {
            self.rewalk_doc(doc);
            self.upload_to(gpu);
        }

        self.compactions += 1;
        true
    }

    /// Forget where editable rows sit; the lanes are about to be walked again.
    fn forget_editable(&mut self) {
        self.uploaded = Counts::default();
        self.dead = Counts::default();
        self.edge_sources.clear();
        self.spans.clear();
        self.caps.clear();
        self.graves.clear();
        self.preview = None;

        for foot in &mut self.feet {
            if *foot != Footprint::Cloud {
                *foot = Footprint::None;
            }
        }
    }

    /// Walk the rows of one document again, in its object order.
    fn rewalk_doc(&mut self, doc: usize) {
        if self.docs[doc].display_only {
            return;
        }

        let session = Rc::clone(&self.docs[doc].session);

        if self.doc_state[doc].nodes_from != tree_key(&session) {
            self.refill_nodes(doc);
        }

        let point_px = self.docs[doc].point_px;
        let sheet = self.doc_state[doc].sheet;

        for guid in session.order() {
            let Some(row) = self.row_of(doc, &guid) else {
                continue;
            };
            let Some(geometry) = session.lookup.get(&guid) else {
                continue;
            };

            if matches!(geometry, Geometry::PointCloud(_)) {
                continue;
            }

            let (node, in_tree) = match self.node_of(row) {
                Some((node, in_tree)) => (Some(node), in_tree),
                None => (None, false),
            };
            let place = self.world_place(doc, node.as_ref(), in_tree, &guid);
            let before = Counts::of(&self.tables);
            let start = self.uploaded.plus(before);
            let cx = WalkCx {
                vert_base: self.uploaded.verts,
                cloud_px: point_px,
                row,
                attributes: self.attributes,
            };
            let walked = walk_geometry(&mut Walk::of(&mut self.tables), &cx, geometry);
            let end = self.uploaded.plus(Counts::of(&self.tables));
            let mut object = ObjectRow::new(place, walked.flags);
            object.bounds = walked.bounds;
            object.spacing = walked.spacing;
            object.faces = walked.faces;

            if walked.faces {
                object.flags |= Instance::FLAG_HAS_FACES;
            }

            if let Some(band) = sheet
                && in_band(band, &walked.bounds, &object.place, &self.docs[doc].place)
            {
                object.flags |= Instance::FLAG_SHEET;
                mark_pens_from(
                    &mut self.tables,
                    before.pipes as usize,
                    before.ribbons as usize,
                );
            }

            self.feet[row as usize] = self.spans.foot(Span {
                start,
                count: end.minus(start),
            });
            self.staged.geometry.push((row, object));
        }
    }

    /// Capture the drag previews of `row` from a walk of it alone; nothing when its rows differ.
    pub(crate) fn capture_preview(&mut self, row: u32) {
        let span = self
            .spans
            .span(*self.feet.get(row as usize).unwrap_or(&Footprint::None));

        if self
            .preview
            .as_ref()
            .is_some_and(|(held, previews)| *held == row && previews.span == span)
        {
            return;
        }

        self.preview = None;
        let Some((doc, guid)) = self.identity_of(row) else {
            return;
        };

        if self.docs.get(doc).is_none_or(|file| file.display_only) {
            return;
        }

        let session = Rc::clone(&self.docs[doc].session);
        let Some(geometry) = session.lookup.get(guid.as_ref()) else {
            return;
        };
        let Some(place) = self.placement_of(row) else {
            return;
        };
        let (up, _) = self.walk_one(doc, row, geometry, &place);

        if Counts::of(&up) != span.count {
            return;
        }

        let previews = Previews {
            mesh: MeshPreview::capture(&up, span, Counts::default(), geometry),
            surface: SurfacePreview::capture(&up, span, Counts::default(), geometry),
            span,
        };
        self.preview = Some((row, previews));
    }

    /// The mesh preview of a dragged row.
    pub(crate) fn mesh_preview(&self, row: u32) -> Option<&MeshPreview> {
        let (held, previews) = self.preview.as_ref()?;
        (*held == row).then_some(previews.mesh.as_ref()).flatten()
    }

    /// Drop the drag previews.
    pub(crate) fn drop_preview(&mut self) {
        self.preview = None;
    }

    /// Re-evaluate a dragged surface into its own rows; false when the rows moved or the surface cannot.
    pub(crate) fn patch_surface(&self, row: u32, geometry: &Geometry, gpu: &mut Gpu) -> bool {
        let Some((held, previews)) = self.preview.as_ref() else {
            return false;
        };
        let span = self.spans.span(self.feet[row as usize]);

        if *held != row || previews.span != span {
            return false;
        }

        let Some(surface) = &previews.surface else {
            return false;
        };
        let Some((vertices, pipes, bounds)) = surface.evaluate(geometry) else {
            return false;
        };
        let Some(place) = self.placement_of(row) else {
            return false;
        };

        gpu.arena
            .patch_vertices(&gpu.ctx, span.start.verts, &vertices);
        gpu.segments.patch_pipes(&gpu.ctx, span.start.pipes, &pipes);
        gpu.objects
            .set_geometry_bounds(&gpu.ctx, row, bounds, 0.0, &place);
        gpu.grew_bounds(row);
        true
    }

    /// Memory held by the drag previews.
    pub fn preview_cache_bytes(&self) -> usize {
        self.preview.as_ref().map_or(0, |(_, previews)| {
            previews
                .mesh
                .as_ref()
                .map_or(0, MeshPreview::allocated_bytes)
                + previews
                    .surface
                    .as_ref()
                    .map_or(0, SurfacePreview::allocated_bytes)
        })
    }

    /// CPU bytes of the per-row tables and the maps keyed by identity.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn row_table_bytes(&self) -> usize {
        let identity = std::mem::size_of::<((usize, Rc<str>), u32)>() + 8; // entry and hash slack
        self.order.capacity() * std::mem::size_of::<Rc<str>>()
            + self.owners.capacity() * std::mem::size_of::<usize>()
            + self.feet.capacity() * std::mem::size_of::<Footprint>()
            + self.nodes.capacity() * std::mem::size_of::<Weak<RefCell<TreeNode>>>()
            + self.edge_sources.capacity() * 8
            + self.spans.bytes()
            + self.caps.capacity() * std::mem::size_of::<(u32, Cap)>()
            + (self.guid_to_row.capacity() + self.graves.capacity()) * identity
    }

    /// Row-level counters for the inspection: (dead lane rows, free ids, dead bytes, graves, compactions).
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn row_counters(&self) -> (u64, usize, u64, usize, u32) {
        let dead: u64 = self.dead.each().map(|(_, rows)| u64::from(rows)).sum();
        (
            dead,
            self.ids.len(),
            self.dead.bytes(),
            self.graves.len(),
            self.compactions,
        )
    }
}

#[cfg(test)]
impl Scene {
    /// What `upload_to` does, without a GPU: the ledger takes the object rows, the tables count as uploaded.
    pub(crate) fn settle(&mut self) {
        const KEEP: u32 = Instance::FLAG_SELECTED
            | Instance::FLAG_HIDDEN
            | Instance::FLAG_INSIDE
            | Instance::FLAG_COLOR
            | Instance::FLAG_EDGE_COLOR
            | Instance::FLAG_DEAD;
        let staged = std::mem::take(&mut self.staged);

        for row in &staged.retire {
            self.ledger.remove(row);
        }

        for (lane, first, count) in super::runs(staged.kills) {
            if lane == LaneId::Pipes {
                self.forget_edges(first, count);
            }
        }

        for (at, up) in &staged.patches {
            self.write_edges(at.pipes, up);
        }

        for (index, pipe) in self.tables.seg.pipes.iter().enumerate() {
            let edge = self
                .tables
                .seg
                .pipe_ids
                .get(index)
                .copied()
                .unwrap_or(u32::MAX);
            self.edge_sources.push((pipe.instance_id, edge));
        }

        for (index, object) in self.tables.obj.rows.iter().enumerate() {
            self.ledger
                .insert(self.object_rows + index as u32, object.clone());
        }

        self.object_rows += self.tables.obj.rows.len() as u32;
        self.uploaded = self.uploaded.plus(Counts::of(&self.tables));
        self.tables.drop_uploaded();
        self.loaded = false;

        for (row, object) in staged.rows {
            self.ledger.insert(row, object);
        }

        for (row, object) in staged.geometry {
            if let Some(held) = self.ledger.get_mut(&row) {
                held.flags = (held.flags & KEEP) | (object.flags & !KEEP);
                held.place = object.place;
                held.bounds = object.bounds;
                held.spacing = object.spacing;
                held.faces = object.faces;
            }
        }

        for (row, place) in staged.places {
            if let Some(held) = self.ledger.get_mut(&row) {
                held.place = place;
            }
        }
    }

    /// `rewalk_editable` without a GPU.
    pub(crate) fn rewalk_cpu(&mut self) {
        self.sync();
        self.settle();
        self.forget_editable();

        for doc in 0..self.docs.len() {
            self.rewalk_doc(doc);
            self.settle();
        }

        self.compactions += 1;
    }

    /// Every live object as a fresh scene of the same documents holds it, and every pipe names its row.
    pub(crate) fn verify(&self) {
        let mut fresh = Scene::new();
        fresh.attributes = self.attributes;
        fresh.created_doc = self.created_doc;
        fresh.hidden = self.hidden.clone();
        fresh.colors = self.colors.clone();
        fresh.edge_colors = self.edge_colors.clone();

        for file in &self.docs {
            fresh.add_file(super::FileDoc {
                name: file.name.clone(),
                place: file.place.clone(),
                session: Rc::clone(&file.session),
                point_px: file.point_px,
                display_only: false,
            });
        }

        let editable = |doc: usize| self.docs.get(doc).is_some_and(|file| !file.display_only);
        let mut mine: Vec<_> = self
            .guid_to_row
            .iter()
            .filter(|((doc, _), _)| editable(*doc))
            .collect();
        let mut theirs: Vec<_> = fresh
            .guid_to_row
            .iter()
            .filter(|((doc, _), _)| editable(*doc))
            .collect();
        mine.sort();
        theirs.sort();
        let ids = |list: &[(&(usize, Rc<str>), &u32)]| {
            list.iter().map(|(id, _)| (*id).clone()).collect::<Vec<_>>()
        };
        assert_eq!(ids(&mine), ids(&theirs), "live identities");
        let bits = |x: &Xform| x.m.map(f64::to_bits);
        let boxed = |b: &session_rust::AABB| [b.cx, b.cy, b.cz, b.hx, b.hy, b.hz].map(f64::to_bits);

        for (&(id, &row), &(_, &other)) in mine.iter().zip(&theirs) {
            let foot = self.feet[row as usize];
            let fresh_foot = fresh.feet[other as usize];
            assert_eq!(
                foot == Footprint::Cloud,
                fresh_foot == Footprint::Cloud,
                "{id:?} cloud"
            );
            let span = self.spans.span(foot);
            assert_eq!(
                span.count,
                fresh.spans.span(fresh_foot).count,
                "{id:?} rows per lane"
            );
            let held = self
                .ledger
                .get(&row)
                .unwrap_or_else(|| panic!("{id:?} row {row} is uploaded"));
            let want = &fresh.tables.obj.rows[other as usize];
            assert_eq!(bits(&held.place), bits(&want.place), "{id:?} placement");
            assert_eq!(boxed(&held.bounds), boxed(&want.bounds), "{id:?} box");
            assert_eq!(
                held.spacing.to_bits(),
                want.spacing.to_bits(),
                "{id:?} spacing"
            );
            assert_eq!(held.faces, want.faces, "{id:?} faces");
            let judged =
                self.doc_state[id.0].sheet.is_some() == fresh.doc_state[id.0].sheet.is_some();
            let mask = if judged {
                u32::MAX
            } else {
                !Instance::FLAG_SHEET
            };
            assert_eq!(held.flags & mask, want.flags & mask, "{id:?} flags");

            for pipe in span.start.pipes..span.start.pipes + span.count.pipes {
                assert_eq!(
                    self.edge_sources[pipe as usize].0, row,
                    "{id:?} pipe {pipe}"
                );
            }
        }

        for &(owner, _) in &self.edge_sources {
            assert!(
                owner == u32::MAX || self.identity_of(owner).is_some(),
                "a pipe names dead row {owner}"
            );
        }

        assert!(
            self.dead.fits(&self.uploaded),
            "dead rows are uploaded rows"
        );
        assert_eq!(self.pending.len(), 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::hierarchy::Hierarchy;
    use crate::app::modeling::Modeling;
    use crate::app::scene::{FileDoc, SheetInit, StreamedInit};
    use crate::app::stream::{CloudFields, CloudLod, SheetFields};
    use crate::app::walk::cloud::StreamRows;
    use crate::app::walk::sheet::SheetRows;
    use session_rust::element::ElementFeature;
    use session_rust::{Element, Line, Mesh, NurbsCurve, Point, Polyline};

    /// A document placed at `place`.
    fn file(name: &str, session: Session, place: Xform) -> FileDoc {
        FileDoc {
            name: name.into(),
            session: Rc::new(session),
            place,
            point_px: 0.0,
            display_only: false,
        }
    }

    /// A point.
    fn p(x: f64, y: f64, z: f64) -> Point {
        Point::new(x, y, z)
    }

    /// A curve through eight points of an arc: trimming it changes its sample count.
    fn arc() -> NurbsCurve {
        let points: Vec<Point> = (0..8)
            .map(|i| {
                let angle = i as f64 * 0.4;
                p(10.0 * angle.cos(), 10.0 * angle.sin(), 0.0)
            })
            .collect();
        NurbsCurve::create(false, 3, &points)
    }

    /// Nested placed groups, a child under a placed parent, each curve kind and an element with baked features.
    fn site() -> Session {
        let mut session = Session::new("site");
        let walls = session.add_group("walls");
        session.set_xform("walls", Xform::translation(0.0, 0.0, 3.0));
        let inner = TreeNode::new("inner");
        session.add(&inner, Some(&walls));
        session.set_xform("inner", Xform::rotation_z(30.0, true));
        let parent = session.add_point(p(0.0, 0.0, 0.0), Some(&walls));
        let parent_guid = parent.borrow().name.clone();
        session.set_xform(&parent_guid, Xform::translation(1.0, 0.0, 0.0));
        let child = session.add_line(Line::new(0.0, 0.0, 0.0, 1.0, 1.0, 0.0), Some(&parent));
        let child_guid = child.borrow().name.clone();
        session.set_xform(&child_guid, Xform::translation(0.0, 2.0, 0.0));
        session.add_polyline(
            Polyline::new(vec![p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(1.0, 1.0, 0.0)]),
            Some(&inner),
        );
        session.add_nurbscurve(arc(), Some(&inner));
        session.add_line(Line::new(-5.0, 0.0, 0.0, 5.0, 0.0, 0.0), None);
        let mut element = Element::new("beam");
        element.set_geometry(Mesh::create_box(10.0, 10.0, 10.0));
        let axis = Polyline::new(vec![p(0.0, 0.0, 0.0), p(100.0, 0.0, 0.0)]);
        element.add_feature(ElementFeature::new("axis", -1, vec![axis], "axis"));
        let beam = session.add_element(element, None);
        let attributes = TreeNode::new("attributes");
        session.add(&attributes, Some(&beam));
        session.add_polyline(
            Polyline::new(vec![p(0.0, 0.0, 0.0), p(0.0, 0.0, 20.0)]),
            Some(&attributes),
        );
        session.add_mesh(Mesh::create_box(2.0, 2.0, 2.0), Some(&inner));
        session.add_group("roof");
        session
    }

    /// A second document with a layer of its own.
    fn other() -> Session {
        let mut session = Session::new("other");
        let inbox = session.add_group("inbox");
        session.add_point(p(3.0, 3.0, 3.0), Some(&inbox));
        session
    }

    /// A streamed sheet and a streamed cloud, read-only shells.
    fn shells(scene: &mut Scene) {
        scene.stream_sheet(SheetInit {
            name: "plan".into(),
            url: "plan.pb".into(),
            meta_url: None,
            place: Xform::identity(),
            rows: SheetRows {
                positions: vec![
                    0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 10.0, 0.0,
                ],
                colors: Vec::new(),
                widths: Vec::new(),
                ids: vec![7, 8],
            },
            fields: SheetFields {
                count: 2,
                ..Default::default()
            },
            resident: 2,
        });
        scene.stream_cloud(StreamedInit {
            name: "scan".into(),
            url: "scan.pb".into(),
            place: Xform::translation(0.0, 0.0, 5.0),
            rows: StreamRows {
                positions: vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
                colors: vec![u32::MAX; 2],
                normals: Vec::new(),
            },
            lod: CloudLod::default(),
            fields: CloudFields {
                end: 0,
                coords_at: 0,
                coords_len: 0,
                colors_at: 0,
                colors_len: 0,
                normals_at: 0,
                normals_len: 0,
                count: 2,
                ids_at: 0,
                ids_len: 0,
                revision: None,
            },
            resident: 2,
            point_px: 3.0,
            col_at: 0,
            ceiling: 2,
        });
    }

    /// Two placed documents, one shared by two placements, and the streamed shells.
    fn scene() -> Scene {
        let mut scene = Scene::new();
        scene.add_file(file("site", site(), Xform::translation(100.0, 0.0, 0.0)));
        shells(&mut scene);
        scene.add_file(file("other", other(), Xform::identity()));
        let shared = Rc::new(site());

        for x in [500.0, 900.0] {
            scene.add_file(FileDoc {
                name: format!("shared {x}"),
                session: Rc::clone(&shared),
                place: Xform::translation(x, 0.0, 0.0),
                point_px: 0.0,
                display_only: false,
            });
        }

        scene.settle();
        scene.verify();
        scene
    }

    /// Sync, flush without a GPU, compare with a fresh scene.
    fn check(scene: &mut Scene) {
        scene.sync();
        scene.settle();
        scene.verify();
    }

    /// Live rows of editable documents, by identity.
    fn live(scene: &Scene) -> Vec<(u32, (usize, Rc<str>))> {
        (0..scene.row_count() as u32)
            .filter_map(|row| {
                let id = scene.identity_of(row)?;
                scene
                    .docs
                    .get(id.0)
                    .is_some_and(|file| !file.display_only)
                    .then_some((row, id))
            })
            .collect()
    }

    /// The first live row whose geometry passes `test`.
    fn find(scene: &Scene, test: impl Fn(&Geometry) -> bool) -> Option<u32> {
        live(scene)
            .into_iter()
            .map(|(row, _)| row)
            .find(|&row| scene.geometry(row).is_some_and(&test))
    }

    /// Notes come from the committed transaction; an empty one and the layer marker give none.
    #[test]
    fn notes_come_from_the_committed_transaction() {
        let mut session = Session::new("notes");
        session.begin("create");
        let node = session.add_point(p(0.0, 0.0, 0.0), None);
        let guid = node.borrow().name.clone();
        let notes = commit(&mut session);
        assert_eq!(notes.len(), 1);
        assert_eq!(&*notes[0].guid, guid.as_str());
        assert_eq!(notes[0].what, PRESENCE | GEOMETRY);
        assert_eq!(notes[0].parent, Some(("notes".to_string(), 0)));

        session.begin("move");
        session.set_xform(&guid, Xform::translation(1.0, 0.0, 0.0));
        assert_eq!(commit(&mut session)[0].what, PLACE | SUBTREE);

        session.begin("replace");
        session.replace(&guid, Geometry::Point(Rc::new(p(2.0, 0.0, 0.0))));
        assert_eq!(commit(&mut session)[0].what, GEOMETRY);

        session.begin("delete");
        session.remove_object(&guid);
        let notes = commit(&mut session);
        assert_eq!(notes[0].what, PRESENCE | SUBTREE);
        assert!(
            notes[0]
                .node
                .as_ref()
                .is_some_and(|n| n.upgrade().is_some())
        );

        session.begin("empty");
        assert!(commit(&mut session).is_empty());
        session.begin("marker #1");
        session.set_xform("marker #1", Xform::identity());
        session.remove_xform("marker #1");
        assert!(
            commit(&mut session).is_empty(),
            "the layer marker pair notes nothing"
        );
        assert_eq!(session.history.undo_stack.len(), 5);

        assert!(session.undo()); // the marker
        assert!(session.undo()); // the delete
        let back = stepped(&session.history, true, true);
        assert_eq!(back[0].what, PRESENCE | GEOMETRY | SUBTREE);
        assert!(session.redo());
        assert_eq!(
            stepped(&session.history, false, true)[0].what,
            PRESENCE | SUBTREE
        );
        assert!(stepped(&session.history, false, false)[0].node.is_none());
    }

    /// A new point takes one row after the others; every other object keeps its id.
    #[test]
    fn create_adds_one_row_and_keeps_every_other_id() {
        let mut scene = scene();
        let before: Vec<_> = (0..scene.row_count() as u32)
            .map(|row| scene.identity_of(row))
            .collect();
        let revision = scene.row_revision;
        let (doc, guid) = scene
            .model(&Modeling::Point([1.0, 2.0, 3.0]))
            .unwrap()
            .unwrap();
        scene.sync();
        assert!(scene.staged.patches.is_empty() && scene.staged.kills.is_empty());
        assert_eq!(scene.tables.obj.rows.len(), 1, "one object row, appended");
        assert_eq!(Counts::of(&scene.tables).dots, 1, "one dot");
        scene.settle();
        scene.verify();
        assert_ne!(scene.row_revision, revision);
        assert_eq!(scene.row_of(doc, &guid), Some(before.len() as u32));

        for (row, id) in before.iter().enumerate() {
            assert_eq!(
                &scene.identity_of(row as u32),
                id,
                "row {row} kept its object"
            );
        }
    }

    /// A deleted line and mesh keep their rows as graves; undo gives the same ids and rows back.
    #[test]
    fn delete_then_undo_restores_id_and_footprint() {
        let mut scene = scene();
        let line = find(&scene, |g| matches!(g, Geometry::Line(_))).unwrap();
        let mesh = find(&scene, |g| matches!(g, Geometry::Mesh(_))).unwrap();
        let feet = (scene.feet[line as usize], scene.feet[mesh as usize]);
        let spans = (scene.spans.span(feet.0), scene.spans.span(feet.1));
        let ids = (
            scene.identity_of(line).unwrap(),
            scene.identity_of(mesh).unwrap(),
        );
        assert!(scene.delete_row(line));
        check(&mut scene);
        assert!(scene.delete_row(mesh));
        check(&mut scene);
        assert_eq!(scene.graves.len(), 2);
        assert!(scene.identity_of(line).is_none() && scene.identity_of(mesh).is_none());

        assert!(scene.undo());
        scene.sync();
        assert_eq!(scene.staged.patches.len(), 1, "written back at its grave");
        assert!(scene.staged.kills.is_empty());
        scene.settle();
        scene.verify();
        assert_eq!(scene.identity_of(mesh), Some(ids.1.clone()), "same id");
        assert_eq!(
            scene.spans.span(scene.feet[mesh as usize]),
            spans.1,
            "same rows"
        );
        assert!(scene.undo());
        check(&mut scene);
        assert_eq!(scene.identity_of(line), Some(ids.0.clone()));
        assert_eq!(scene.spans.span(scene.feet[line as usize]), spans.0);
        assert!(scene.graves.is_empty());

        assert!(scene.redo());
        check(&mut scene);
        assert!(scene.identity_of(line).is_none());
        assert_eq!(scene.graves.len(), 1);
    }

    /// A trim that keeps the counts writes in place; one that changes them moves and leaves a grave.
    #[test]
    fn replace_in_place_or_via_grave() {
        let mut scene = scene();
        let line = find(&scene, |g| matches!(g, Geometry::Line(_))).unwrap();
        let revision = scene.row_revision;
        scene.selected = Some(line);
        scene.model(&Modeling::Trim(0.2, 0.8)).unwrap();
        scene.sync();
        assert_eq!(scene.staged.patches.len(), 1);
        assert!(scene.staged.kills.is_empty() && scene.tables_empty());
        scene.settle();
        scene.verify();
        assert_eq!(scene.row_revision, revision, "a redraw keeps the rows");

        let curve = find(&scene, |g| matches!(g, Geometry::NurbsCurve(_))).unwrap();
        let before = scene.spans.span(scene.feet[curve as usize]);
        scene.selected = Some(curve);
        scene.model(&Modeling::Extend(-0.5, 1.5)).unwrap();
        scene.sync();
        assert_eq!(scene.staged.kills.len(), 1, "the old rows die");
        assert!(
            Counts::of(&scene.tables).ribbons > before.count.ribbons,
            "the new rows are appended"
        );
        scene.settle();
        scene.verify();
        let after = scene.spans.span(scene.feet[curve as usize]);
        assert_ne!(after.count, before.count);
        assert_eq!(scene.graves.len(), 1);

        assert!(scene.undo());
        check(&mut scene);
        assert_eq!(
            scene.spans.span(scene.feet[curve as usize]),
            before,
            "back at the start"
        );
        assert!(scene.redo());
        check(&mut scene);
        assert_eq!(scene.spans.span(scene.feet[curve as usize]), after);
    }

    /// A tiny deterministic random source.
    struct Dice(u64);

    impl Dice {
        /// A number below `n`.
        fn roll(&mut self, n: usize) -> usize {
            self.0 = self
                .0
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            ((self.0 >> 33) as usize) % n.max(1)
        }
    }

    /// One random edit of the kinds the viewer makes.
    fn edit(scene: &mut Scene, dice: &mut Dice) {
        let rows: Vec<u32> = live(scene).into_iter().map(|(row, _)| row).collect();
        let pick = |dice: &mut Dice| rows[dice.roll(rows.len())];
        let layer = |scene: &Scene, doc: usize| {
            let mut names: Vec<String> = scene.docs[doc]
                .session
                .tree
                .nodes()
                .iter()
                .skip(1)
                .map(|node| node.borrow().name.clone())
                .filter(|name| !scene.docs[doc].session.lookup.contains_key(name))
                .collect();
            names.sort();
            names
        };
        let x = dice.roll(100) as f64;

        match dice.roll(20) {
            0 => drop(scene.model(&Modeling::Point([x, 1.0, 2.0]))),
            1 => drop(scene.model(&Modeling::Line([x, 0.0, 0.0], [x, 5.0, 1.0]))),
            2 => drop(scene.model(&Modeling::Polyline(vec![
                [0.0, 0.0, 0.0],
                [x, 1.0, 0.0],
                [x, x, 0.0],
            ]))),
            3 => drop(scene.model(&Modeling::Curve(vec![
                [0.0, 0.0, 0.0],
                [x, 3.0, 0.0],
                [x, x, 2.0],
                [0.0, x, 1.0],
            ]))),
            4 | 5 if !rows.is_empty() => {
                scene.delete_row(pick(dice));
            }
            6 if !rows.is_empty() => {
                scene.selected = Some(pick(dice));
                let _ = scene.model(&Modeling::Trim(0.1, 0.6));
            }
            7 if !rows.is_empty() => {
                scene.selected = Some(pick(dice));
                let _ = scene.model(&Modeling::Extend(-0.5, 1.2));
            }
            8 if !rows.is_empty() => {
                scene.selected = Some(pick(dice));
                let _ = scene.model(&Modeling::Explode);
            }
            9 if !rows.is_empty() => {
                let picked = [pick(dice), pick(dice)];
                scene.transform_rows(&picked, &Xform::translation(x, 1.0, -2.0), "move");
            }
            10 | 11 => {
                scene.undo();
            }
            12 => {
                scene.redo();
            }
            13 => {
                let doc = dice.roll(scene.docs.len());
                if let Some(at) = layer(scene, doc).first().cloned() {
                    let _ = scene.new_layer(doc, &at, dice.roll(2) == 0);
                }
            }
            14 => {
                let doc = dice.roll(scene.docs.len());
                let names = layer(scene, doc);
                if !names.is_empty() {
                    let name = names[dice.roll(names.len())].clone();
                    let to = if dice.roll(4) == 0 {
                        "attributes".to_string()
                    } else {
                        format!("{name} {x}")
                    };
                    let _ = scene.rename_layer(doc, &name, &to);
                }
            }
            15 => {
                let doc = dice.roll(scene.docs.len());
                let names = layer(scene, doc);
                if !names.is_empty() {
                    let name = names[dice.roll(names.len())].clone();
                    let _ = scene.delete_layer(doc, &name);
                }
            }
            16 => {
                let doc = dice.roll(scene.docs.len());
                let names = layer(scene, doc);
                if !names.is_empty() {
                    let name = names[dice.roll(names.len())].clone();
                    let _ = scene.duplicate_layer(doc, &name);
                }
            }
            17 | 18 if !rows.is_empty() => {
                let doc = dice.roll(scene.docs.len());
                let names = layer(scene, doc);
                if !names.is_empty() {
                    let name = names[dice.roll(names.len())].clone();
                    let picked = [pick(dice)];
                    let _ = if dice.roll(3) == 0 {
                        scene.copy_object_layer(&picked, doc, &name)
                    } else {
                        scene.change_object_layer(&picked, doc, &name)
                    };
                }
            }
            19 if rows.len() > 1 => {
                let target = pick(dice);
                let cutter = pick(dice);
                let _ = scene.split_rows(target, None, &[cutter]);
            }
            _ => {}
        }
    }

    /// Hundreds of random edits, undos and layer moves: after each, the rows match a fresh walk.
    #[test]
    fn incremental_matches_fresh_oracle() {
        for seed in [1, 7, 42] {
            let mut scene = scene();
            let mut dice = Dice(seed);

            for step in 0..150 {
                edit(&mut scene, &mut dice);
                scene.sync();
                scene.settle();
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| scene.verify()))
                    .unwrap_or_else(|_| panic!("seed {seed} step {step}"));
            }
        }
    }

    /// Hints and the node cache only save walks: without them every row ends up the same.
    #[test]
    fn hints_and_node_cache_are_only_a_cache() {
        let run = |cached: bool| {
            let mut scene = scene();
            let mut dice = Dice(5);

            for _ in 0..120 {
                edit(&mut scene, &mut dice);

                if !cached {
                    scene.hints.clear();

                    for state in &mut scene.doc_state {
                        state.nodes_from = 0;
                    }
                }

                check(&mut scene);
            }

            // guids are new in every run: compare documents, rows and lanes
            let rows: Vec<_> = (0..scene.row_count() as u32)
                .map(|row| {
                    (
                        scene.identity_of(row).map(|id| id.0),
                        scene.spans.span(scene.feet[row as usize]),
                    )
                })
                .collect();
            (rows, scene.searches)
        };
        let (cached, searched) = run(true);
        let (plain, all) = run(false);
        assert_eq!(cached, plain);
        assert!(searched < all, "the cache saves walks: {searched} of {all}");

        // undo of a move finds every node in the cache
        let mut scene = scene();
        let row = find(&scene, |g| matches!(g, Geometry::Point(_))).unwrap();
        scene.transform_rows(&[row], &Xform::translation(1.0, 0.0, 0.0), "move");
        check(&mut scene);
        let searches = scene.searches;
        assert!(scene.undo());
        check(&mut scene);
        assert_eq!(scene.searches, searches, "no tree walk");

        // undo of a create drops a node the tree no longer has: no walk looks for it
        scene.model(&Modeling::Point([1.0, 2.0, 3.0])).unwrap();
        check(&mut scene);
        let searches = scene.searches;
        assert!(scene.undo());
        check(&mut scene);
        assert_eq!(
            scene.searches, searches,
            "no tree walk for a removed object"
        );

        // a weak handle on a session, as the inspection keeps, moves it at every edit: the cache stays
        let row = find(&scene, |g| matches!(g, Geometry::Point(_))).unwrap();
        let (doc, _) = scene.identity_of(row).unwrap();
        let held = Rc::downgrade(&scene.docs[doc].session);
        scene.transform_rows(&[row], &Xform::translation(0.0, 1.0, 0.0), "move");
        check(&mut scene);
        assert!(held.upgrade().is_none(), "the edit moved the session");
        assert_eq!(
            scene.searches, searches,
            "no tree walk after the session moved"
        );

        // a tree swapped under the cache is walked once by the next sync, not per row by later lookups
        scene.forget_nodes(doc);
        check(&mut scene);
        assert!(scene.cache_hit(row).is_some_and(|(_, in_tree)| in_tree));

        // a session another holder shares is copied by the edit; the copy is cached before the sync
        let shared = Rc::clone(&scene.docs[doc].session);
        scene.transform_rows(&[row], &Xform::translation(0.0, 0.0, 1.0), "move");
        assert!(scene.cache_hit(row).is_some_and(|(_, in_tree)| in_tree));
        drop(shared);
        check(&mut scene);
    }

    /// Noting everything, the fallback for an edit path that notes nothing, changes no row.
    #[test]
    fn touch_all_keeps_a_synced_scene() {
        let mut scene = scene();
        let mut dice = Dice(13);

        for _ in 0..40 {
            edit(&mut scene, &mut dice);
            check(&mut scene);
        }

        let before: Vec<_> = (0..scene.row_count() as u32)
            .map(|row| scene.identity_of(row))
            .collect();
        let revision = scene.row_revision;
        scene.touch_all();
        check(&mut scene);
        let after: Vec<_> = (0..scene.row_count() as u32)
            .map(|row| scene.identity_of(row))
            .collect();
        assert_eq!(before, after);
        assert_eq!(scene.row_revision, revision, "no row came or went");
    }

    /// Hide or color one live object as the panel does: its identity and its row.
    fn mark(scene: &mut Scene, dice: &mut Dice) {
        let rows: Vec<u32> = live(scene).into_iter().map(|(row, _)| row).collect();

        if rows.is_empty() {
            return;
        }

        let row = rows[dice.roll(rows.len())];
        let id = scene.identity_of(row).unwrap();
        let held = scene.ledger.get_mut(&row).unwrap();

        if dice.roll(2) == 0 {
            scene.hidden.insert(id);
            held.flags |= Instance::FLAG_HIDDEN;
        } else {
            scene.colors.insert(id, [200, 40, 40]);
            held.flags |= Instance::FLAG_COLOR;
        }
    }

    /// Hidden and colored objects keep both through every kind of edit, undo and compaction.
    #[test]
    fn hidden_and_colored_objects_keep_their_flags() {
        for seed in [2, 9] {
            let mut scene = scene();
            let mut dice = Dice(seed);

            for step in 0..150 {
                if dice.roll(3) == 0 {
                    mark(&mut scene, &mut dice);
                }

                edit(&mut scene, &mut dice);
                scene.sync();
                scene.settle();

                if step % 50 == 49 {
                    scene.rewalk_cpu();
                }

                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| scene.verify()))
                    .unwrap_or_else(|_| panic!("seed {seed} step {step}"));
            }
        }
    }

    /// Deleting a parent draws its children at their own transform and frees baked features.
    #[test]
    fn removing_a_parent_draws_children_like_a_rewalk() {
        let mut scene = scene();
        let element = find(&scene, |g| matches!(g, Geometry::Element(_))).unwrap();
        let parent = find(&scene, |g| matches!(g, Geometry::Point(_))).unwrap();
        let count = scene.object_count();
        assert!(scene.delete_row(element));
        check(&mut scene);
        assert_eq!(
            scene.object_count(),
            count,
            "the baked polyline takes the element's place"
        );
        assert!(scene.delete_row(parent));
        check(&mut scene);
        assert!(scene.undo());
        check(&mut scene);
        assert!(scene.undo());
        check(&mut scene);
        assert_eq!(scene.object_count(), count);
    }

    /// Moving a parent moves its unselected children too.
    #[test]
    fn moving_a_parent_moves_its_children() {
        let mut scene = scene();
        let parent = find(&scene, |g| matches!(g, Geometry::Point(_))).unwrap();
        let (doc, guid) = scene.identity_of(parent).unwrap();
        let child = scene.docs[doc]
            .session
            .tree
            .get_node_by_name(&guid)
            .unwrap()
            .borrow()
            .children()[0]
            .borrow()
            .name
            .clone();
        let child = scene.row_of(doc, &child).unwrap();
        scene.transform_rows(&[parent], &Xform::translation(0.0, 7.0, 0.0), "move");
        scene.sync();
        let moved: Vec<u32> = scene.staged.places.iter().map(|(row, _)| *row).collect();
        assert!(
            moved.contains(&parent) && moved.contains(&child),
            "{moved:?}"
        );
        scene.settle();
        scene.verify();
    }

    /// Layer undo and redo swap whole trees: the node cache refills once per step, rows stay right.
    #[test]
    fn layer_step_tree_swaps_refill_the_cache() {
        let mut scene = scene();
        let line = find(&scene, |g| matches!(g, Geometry::Line(_))).unwrap();
        let (doc, _) = scene.identity_of(line).unwrap();
        scene.change_object_layer(&[line], doc, "roof").unwrap();
        check(&mut scene);

        for back in [true, false, true] {
            let searches = scene.searches;
            assert!(if back { scene.undo() } else { scene.redo() });
            check(&mut scene);
            assert_eq!(scene.searches, searches + 1, "one refill");
        }

        scene.delete_layer(doc, "walls").unwrap();
        check(&mut scene);
        assert!(scene.undo());
        check(&mut scene);
        assert!(scene.redo());
        check(&mut scene);
        assert!(scene.undo());
        check(&mut scene);

        // a layer renamed to `attributes` bakes what it holds; undo frees it
        let other = scene
            .docs
            .iter()
            .position(|file| file.name == "other")
            .unwrap();
        let count = scene.object_count();
        scene.rename_layer(other, "inbox", "attributes").unwrap();
        check(&mut scene);
        assert_eq!(scene.object_count(), count - 1);
        assert!(scene.undo());
        check(&mut scene);
        assert_eq!(scene.object_count(), count);
        assert!(scene.redo());
        check(&mut scene);
        assert_eq!(scene.object_count(), count - 1);
    }

    /// With a streamed sheet and cloud loaded every edit works; the shells stay read-only.
    #[test]
    fn streamed_scene_is_editable() {
        let mut scene = scene();
        let sheet = scene.sheets[0].row;
        let cloud = scene.streamed[0].row;
        let shell = |scene: &Scene| {
            (
                scene.identity_of(sheet),
                scene.identity_of(cloud),
                scene.feet[sheet as usize],
                scene.feet[cloud as usize],
                scene.sheet_slot(sheet),
            )
        };
        let before = shell(&scene);
        let (point_doc, point) = scene
            .model(&Modeling::Point([1.0, 1.0, 1.0]))
            .unwrap()
            .unwrap();
        check(&mut scene);
        let point = scene.row_of(point_doc, &point).unwrap();
        scene
            .model(&Modeling::Line([0.0, 0.0, 0.0], [4.0, 4.0, 0.0]))
            .unwrap();
        check(&mut scene);
        assert!(scene.delete_row(point));
        check(&mut scene);
        assert!(scene.undo());
        check(&mut scene);
        assert!(scene.redo());
        check(&mut scene);
        let line = find(&scene, |g| matches!(g, Geometry::Line(_))).unwrap();
        scene.selected = Some(line);
        scene.model(&Modeling::Trim(0.1, 0.9)).unwrap();
        check(&mut scene);
        let polyline = find(&scene, |g| matches!(g, Geometry::Polyline(_))).unwrap();
        scene.selected = Some(polyline);
        scene.model(&Modeling::Explode).unwrap();
        check(&mut scene);
        let (doc, _) = scene.identity_of(line).unwrap();
        let made = scene.new_layer(doc, "roof", false).unwrap();
        check(&mut scene);
        scene.rename_layer(doc, &made, "made").unwrap();
        check(&mut scene);
        scene.change_object_layer(&[line], doc, "made").unwrap();
        check(&mut scene);

        for _ in 0..3 {
            assert!(scene.undo());
            check(&mut scene);
        }

        assert_eq!(shell(&scene), before, "shell rows untouched");
        assert!(!scene.delete_row(sheet), "a shell cannot be deleted");
        assert!(
            scene
                .transform_rows(&[cloud], &Xform::translation(1.0, 0.0, 0.0), "move")
                .is_none()
        );
        assert!(
            scene
                .commit_geometry(sheet, Geometry::Point(Rc::new(p(0.0, 0.0, 0.0))), "edit")
                .is_err()
        );

        // a picked sheet entity or cloud point says why it stays put
        let entity = crate::app::deform::Target::Edge(0);
        let shift = Xform::translation(1.0, 0.0, 0.0);
        let point = crate::app::selection::ControlId::Point(0);
        let read_only = Err(crate::app::scene::READ_ONLY.to_string());
        assert_eq!(
            scene.edit_subobject(sheet, entity, &shift, "move"),
            read_only
        );
        assert_eq!(
            scene.set_source_control(cloud, point, &p(1.0, 0.0, 0.0)),
            read_only
        );
        scene.hidden.insert(scene.identity_of(sheet).unwrap());
        scene.locked.insert(scene.identity_of(cloud).unwrap());
        assert_eq!(scene.hidden_rows(), vec![sheet]);
        assert!(!scene.selectable(cloud));
        check(&mut scene);
    }

    /// A flat drawing's new flat line takes the sheet flag and pens; a 3D point and `Created` do not.
    #[test]
    fn sheet_rule() {
        let mut flat = Session::new("plan");
        flat.add_line(Line::new(0.0, 0.0, 0.0, 10.0, 0.0, 0.0), None);
        flat.add_line(Line::new(0.0, 5.0, 0.0, 10.0, 5.0, 0.0), None);
        let mut scene = Scene::new();
        scene.add_file(file("plan", flat, Xform::identity()));
        scene.settle();
        assert!(scene.doc_state[0].sheet.is_some());
        scene.current_layer = Some((0, "plan".into()));
        let (_, line) = scene
            .model(&Modeling::Line([0.0, 9.0, 0.0], [10.0, 9.0, 0.0]))
            .unwrap()
            .unwrap();
        let (_, point) = scene
            .model(&Modeling::Point([0.0, 0.0, 50.0]))
            .unwrap()
            .unwrap();
        scene.sync();
        let pens: Vec<f32> = scene.tables.seg.ribbons.iter().map(|s| s.radius).collect();
        assert_eq!(pens, vec![0.5], "the flat line takes the sheet pen");
        scene.settle();
        scene.verify();
        let flags = |scene: &Scene, guid: &str| {
            scene.ledger[&scene.row_of(0, guid).unwrap()].flags & Instance::FLAG_SHEET
        };
        assert_ne!(flags(&scene, &line), 0);
        assert_eq!(flags(&scene, &point), 0);
        scene.rewalk_cpu();
        assert_ne!(flags(&scene, &line), 0, "compaction keeps it");

        scene.current_layer = None;
        let (doc, created) = scene
            .model(&Modeling::Line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]))
            .unwrap()
            .unwrap();
        check(&mut scene);
        assert_eq!(
            scene.ledger[&scene.row_of(doc, &created).unwrap()].flags & Instance::FLAG_SHEET,
            0,
            "Created is never a sheet"
        );
    }

    /// No per-row preview table: only a dragged row holds preview memory.
    #[test]
    fn per_row_tables_are_small() {
        let mut scene = scene();
        assert_eq!(scene.preview_cache_bytes(), 0);
        let mesh = find(&scene, |g| matches!(g, Geometry::Mesh(_))).unwrap();
        scene.capture_preview(mesh);
        assert!(scene.mesh_preview(mesh).is_some());
        let held = scene.preview_cache_bytes();
        assert!(
            held > 0 && held < 64 * 1024,
            "{held} bytes for one small mesh"
        );
        scene.drop_preview();
        assert_eq!(scene.preview_cache_bytes(), 0);
        assert!(
            std::mem::size_of::<Footprint>() + std::mem::size_of::<Weak<RefCell<TreeNode>>>() <= 24
        );
    }

    /// After edits a picked row still names its object, and its lane rows name the row.
    #[test]
    fn row_identity_survives_edits() {
        let mut scene = scene();
        let mut dice = Dice(11);

        for _ in 0..60 {
            edit(&mut scene, &mut dice);
            check(&mut scene);
        }

        for (row, (doc, guid)) in live(&scene) {
            let geometry = scene.geometry(row).expect("a live row has geometry");
            assert_eq!(geometry.guid(), guid.as_ref());
            assert!(scene.docs[doc].session.lookup.contains_key(guid.as_ref()));

            if let Some(range) = scene.ribbon_range(row) {
                assert!(range.end <= scene.uploaded.ribbons);
            }
        }
    }

    /// The layers panel sees created objects and loses deleted ones.
    #[test]
    fn hierarchy_follows_edits() {
        let mut scene = scene();
        let mut panel = Hierarchy::default();
        let (doc, guid) = scene
            .model(&Modeling::Point([1.0, 1.0, 1.0]))
            .unwrap()
            .unwrap();
        check(&mut scene);
        panel.refresh(&scene);
        let row = scene.row_of(doc, &guid).unwrap();
        assert!(panel.rows.contains(&row));
        assert!(scene.delete_row(row));
        check(&mut scene);
        panel.refresh(&scene);
        assert!(!panel.rows.contains(&row));
        assert!(scene.undo());
        check(&mut scene);
        panel.refresh(&scene);
        assert!(panel.rows.contains(&row), "the same row comes back");
        let walls = panel.index_of(0, "walls").unwrap();
        let targets = panel.targets(walls);
        let expected: Vec<u32> = {
            let session = &scene.docs[0].session;
            let node = session.tree.get_node_by_name("walls").unwrap();
            let mut rows: Vec<u32> = node
                .borrow()
                .descendants()
                .iter()
                .filter_map(|n| scene.row_of(0, &n.borrow().name))
                .collect();
            rows.sort_unstable();
            rows
        };
        assert_eq!(targets, expected);
    }

    /// Drawing, deleting and undoing cost the same in a scene of a thousand objects and of a hundred thousand.
    #[test]
    fn edit_cost_does_not_scale_with_the_scene() {
        let time = |objects: usize| {
            let mut session = Session::new("big");

            for i in 0..objects {
                session.add_line(Line::new(i as f64, 0.0, 0.0, i as f64, 1.0, 0.0), None);
            }

            let mut scene = Scene::new();
            scene.add_file(file("big", session, Xform::identity()));
            scene.settle();
            let started = std::time::Instant::now();

            for i in 0..40 {
                let (doc, guid) = scene
                    .model(&Modeling::Point([i as f64, 5.0, 0.0]))
                    .unwrap()
                    .unwrap();
                scene.sync();
                scene.settle();
                let row = scene.row_of(doc, &guid).unwrap();
                assert!(scene.delete_row(row));
                scene.sync();
                scene.settle();
                assert!(scene.undo());
                scene.sync();
                scene.settle();
            }

            started.elapsed().as_secs_f64()
        };
        let small = time(1_000);
        let large = time(100_000);
        assert!(
            large < small * 4.0 + 0.05,
            "120 edits: {small:.4} s with 1k objects, {large:.4} s with 100k"
        );
    }

    /// Dead rows past the threshold call a compaction, which leaves the lanes as a fresh walk.
    #[test]
    fn compaction_reclaims_dead_rows_and_keeps_ids() {
        let mut scene = scene();
        let mut dice = Dice(3);

        for _ in 0..80 {
            edit(&mut scene, &mut dice);
            check(&mut scene);
        }

        let ids: Vec<_> = (0..scene.row_count() as u32)
            .map(|row| scene.identity_of(row))
            .collect();
        scene.rewalk_cpu();
        scene.verify();
        assert_eq!(scene.dead, Counts::default());
        assert!(scene.graves.is_empty() && scene.caps.is_empty());
        let after: Vec<_> = (0..scene.row_count() as u32)
            .map(|row| scene.identity_of(row))
            .collect();
        assert_eq!(ids, after, "ids stay");

        // a fresh walk's lanes: every live row packed in document order
        let packed: u64 = live(&scene)
            .iter()
            .map(|(row, _)| scene.spans.span(scene.feet[*row as usize]).count.bytes())
            .sum();
        assert_eq!(packed, scene.uploaded.bytes());
        assert!(!scene.compaction_due());
    }

    /// A drag that grows a curve mostly fits its headroom; cancelling returns to the grave.
    #[test]
    fn preview_growth_is_bounded_and_cancel_restores() {
        let mut scene = scene();
        let curve = find(&scene, |g| matches!(g, Geometry::NurbsCurve(_))).unwrap();
        let (doc, guid) = scene.identity_of(curve).unwrap();
        let source = scene.docs[doc].session.lookup[guid.as_ref()].clone();
        let start = scene.spans.span(scene.feet[curve as usize]);
        let Geometry::NurbsCurve(original) = &source else {
            panic!()
        };
        let mut moves = 0;
        let mut largest = 0u64;

        for frame in 0..200 {
            let mut grown = (**original).clone();
            let (lo, hi) = grown.domain();
            let reach = 1.0 + (frame % 50 + 1) as f64 * 0.04;
            assert!(grown.extend(lo, lo + (hi - lo) * reach));
            scene.redraw(curve, &Geometry::NurbsCurve(Rc::new(grown)), true);
            moves += usize::from(!scene.tables_empty());
            scene.settle();
            let alloc = scene
                .caps
                .get(&curve)
                .map_or(scene.spans.span(scene.feet[curve as usize]).count, |cap| {
                    cap.alloc
                });
            largest = largest.max(alloc.bytes());
        }

        assert!(moves <= 16, "{moves} of 200 frames needed new rows");
        assert!(
            scene.dead.bytes() <= (moves as u64 + 1) * largest,
            "{} dead bytes",
            scene.dead.bytes()
        );
        scene.redraw(curve, &source, false);
        scene.settle();
        assert_eq!(
            scene.spans.span(scene.feet[curve as usize]),
            start,
            "back at its grave"
        );
        scene.verify();
    }

    /// A few frames of a curve or polyline growing, then the release or the cancel.
    fn drag(scene: &mut Scene, dice: &mut Dice) {
        let rows: Vec<u32> = live(scene)
            .into_iter()
            .map(|(row, _)| row)
            .filter(|&row| {
                matches!(
                    scene.geometry(row),
                    Some(Geometry::NurbsCurve(_) | Geometry::Polyline(_))
                )
            })
            .collect();

        if rows.is_empty() {
            return;
        }

        let row = rows[dice.roll(rows.len())];
        let source = scene.geometry(row).unwrap().clone();
        let mut shape = source.clone();

        for _ in 0..1 + dice.roll(6) {
            shape = match &source {
                Geometry::NurbsCurve(curve) => {
                    let mut grown = (**curve).clone();
                    let (lo, hi) = grown.domain();
                    let _ = grown.extend(lo, hi + (hi - lo) * (0.1 + dice.roll(20) as f64 * 0.1));
                    Geometry::NurbsCurve(Rc::new(grown))
                }
                Geometry::Polyline(line) => {
                    let mut points = line.get_points();

                    for i in 0..dice.roll(4) {
                        points.push(p(i as f64, 9.0, 1.0));
                    }

                    Geometry::Polyline(Rc::new(Polyline::new(points)))
                }
                _ => unreachable!(),
            };
            scene.redraw(row, &shape, true);
            scene.settle();
        }

        if dice.roll(2) == 0 {
            scene.redraw(row, &source, false);
        } else {
            scene.commit_geometry(row, shape, "drag").unwrap();
            scene.sync();
        }

        scene.settle();
    }

    /// Drags released or cancelled between edits, undos and compactions leave what a fresh scene draws.
    #[test]
    fn drags_between_edits_match_a_fresh_scene() {
        for seed in [4, 17, 23] {
            let mut scene = scene();
            let mut dice = Dice(seed);

            for step in 0..120 {
                if dice.roll(2) == 0 {
                    drag(&mut scene, &mut dice);
                } else {
                    edit(&mut scene, &mut dice);
                    scene.sync();
                    scene.settle();
                }

                if step % 40 == 39 {
                    scene.rewalk_cpu();
                }

                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| scene.verify()))
                    .unwrap_or_else(|_| panic!("seed {seed} step {step}"));
            }
        }
    }

    /// Frames drawn by a headless GPU; every test here needs a native adapter.
    #[cfg(not(target_arch = "wasm32"))]
    mod gpu {
        use super::*;
        use crate::camera::Camera;
        use crate::engine::gpu::FrameInput;
        use session_rust::{BRep, Color, PointCloud, Vector};

        /// Colors and ids of one frame.
        struct Shot {
            color: Vec<u8>,     // RGBA pixels
            ids: Vec<[u32; 2]>, // object and sub id per pixel
        }

        /// Draw one frame and its id frame.
        fn shot(gpu: &mut Gpu, camera: &Camera) -> Shot {
            let anchor = gpu
                .rebase_anchor(&camera.origin(), camera.distance_world(), 0.0)
                .anchor;
            let input = FrameInput {
                view_proj: camera.view_proj_anchored(4.0 / 3.0, &anchor),
                clear: wgpu::Color::WHITE,
                now_ms: 0.0,
            };
            Shot {
                color: gpu.render_offscreen(&input),
                ids: gpu.render_ids_offscreen(&input),
            }
        }

        /// The ids of a frame as identities; a drawn id must be a live row.
        fn named(scene: &Scene, ids: &[[u32; 2]]) -> Vec<Option<(usize, Rc<str>)>> {
            ids.iter()
                .map(|&[object, _]| {
                    (object != 0).then(|| {
                        scene
                            .identity_of(object - 1)
                            .unwrap_or_else(|| panic!("row {} is drawn but dead", object - 1))
                    })
                })
                .collect()
        }

        /// A mesh, a BRep, a polyline, a point, a curve, a cloud and two overlapping elements.
        fn solids() -> Session {
            let mut session = Session::new("solids");
            let at = |session: &mut Session, node: Rc<RefCell<TreeNode>>, x: f64, y: f64| {
                let guid = node.borrow().name.clone();
                session.set_xform(&guid, Xform::translation(x, y, 0.0));
            };
            let mesh = session
                .add_mesh(Mesh::create_box(10.0, 10.0, 10.0), None)
                .unwrap();
            at(&mut session, mesh, 0.0, 0.0);
            let brep = session
                .add_brep(BRep::create_box(10.0, 10.0, 10.0), None)
                .unwrap();
            at(&mut session, brep, 20.0, 0.0);
            session.add_polyline(
                Polyline::new(vec![
                    p(35.0, -5.0, 0.0),
                    p(45.0, 5.0, 0.0),
                    p(50.0, -5.0, 0.0),
                ]),
                None,
            );
            let mut point = p(60.0, 0.0, 0.0);
            point.width = 12.0;
            session.add_point(point, None);
            let curve = session.add_nurbscurve(arc(), None).unwrap();
            at(&mut session, curve, 0.0, 20.0);
            let points: Vec<Point> = (0..400)
                .map(|i| p((i % 20) as f64, -20.0 - (i / 20) as f64, 0.0))
                .collect();
            let normals = vec![Vector::new(0.0, 0.0, 1.0); points.len()];
            let colors = vec![Color::red(); points.len()];
            session.add_pointcloud(PointCloud::new(points, normals, colors), None);

            for (x, y) in [(40.0, 30.0), (45.0, 33.0)] {
                let mut element = Element::new("glass");
                element.set_geometry(Mesh::create_box(10.0, 10.0, 10.0));
                let node = session.add_element(element, None);
                at(&mut session, node, x, y);
            }

            session
        }

        /// A headless GPU with the scene loaded and a camera fitted to it.
        fn loaded(session: &Rc<Session>) -> (Gpu, Scene, Camera) {
            let mut gpu =
                pollster::block_on(Gpu::new_headless(400, 300)).expect("a native adapter");
            gpu.view.show_grid = false;
            gpu.view.opacity = 0.7;
            let mut scene = Scene::new();
            scene.add_file(FileDoc {
                name: "solids".into(),
                session: Rc::clone(session),
                place: Xform::identity(),
                point_px: 0.0,
                display_only: false,
            });
            scene.upload_to(&mut gpu);
            let mut camera = Camera::new();
            camera.fit(&gpu.bounds, 4.0 / 3.0);
            (gpu, scene, camera)
        }

        /// Sync and flush.
        fn commit(scene: &mut Scene, gpu: &mut Gpu) {
            scene.sync();
            scene.upload_to(gpu);
        }

        /// A deleted object leaves no pixel and no id: the frame equals a fresh load without it.
        #[test]
        #[ignore = "requires a native GPU adapter"]
        fn kill_leaves_no_pixels_and_no_ids() {
            let (mut gpu, mut scene, camera) = loaded(&Rc::new(solids()));
            let kinds: [fn(&Geometry) -> bool; 5] = [
                |g| matches!(g, Geometry::Mesh(_)),
                |g| matches!(g, Geometry::BRep(_)),
                |g| matches!(g, Geometry::Polyline(_)),
                |g| matches!(g, Geometry::Point(_)),
                |g| matches!(g, Geometry::PointCloud(_)),
            ];

            for kind in kinds {
                let row = find(&scene, kind).unwrap();
                assert!(scene.delete_row(row));
                commit(&mut scene, &mut gpu);
                let (mut fresh_gpu, fresh, _) = loaded(&scene.docs[0].session);

                for samples in [1, 4] {
                    for gpu in [&mut gpu, &mut fresh_gpu] {
                        gpu.view.msaa_forced = Some(samples);
                        gpu.resize(400, 300);
                    }

                    let after = shot(&mut gpu, &camera);
                    let want = shot(&mut fresh_gpu, &camera);
                    let differ = after
                        .color
                        .chunks_exact(4)
                        .zip(want.color.chunks_exact(4))
                        .filter(|(a, b)| a != b)
                        .count();
                    assert_eq!(differ, 0, "{samples}x: pixels differ from a fresh load");
                    assert_eq!(named(&scene, &after.ids), named(&fresh, &want.ids));
                }
            }
        }

        /// Delete, create, a count-changing trim and a cancelled drag, undone: the same pixels and ids.
        #[test]
        #[ignore = "requires a native GPU adapter"]
        fn edit_then_undo_is_pixel_and_id_identical() {
            let (mut gpu, mut scene, camera) = loaded(&Rc::new(solids()));

            for samples in [1, 4] {
                gpu.view.msaa_forced = Some(samples);
                gpu.resize(400, 300);
                let before = shot(&mut gpu, &camera);
                let same = |gpu: &mut Gpu, label: &str| {
                    let now = shot(gpu, &camera);
                    assert!(now.color == before.color, "{samples}x {label}: pixels");
                    assert!(now.ids == before.ids, "{samples}x {label}: ids");
                };

                let element = find(&scene, |g| matches!(g, Geometry::Element(_))).unwrap();
                assert!(scene.delete_row(element));
                commit(&mut scene, &mut gpu);
                assert!(scene.undo());
                commit(&mut scene, &mut gpu);
                same(&mut gpu, "delete, undo");

                scene.model(&Modeling::Point([5.0, 5.0, 5.0])).unwrap();
                commit(&mut scene, &mut gpu);
                assert!(scene.undo());
                commit(&mut scene, &mut gpu);
                same(&mut gpu, "create, undo");

                let curve = find(&scene, |g| matches!(g, Geometry::NurbsCurve(_))).unwrap();
                scene.selected = Some(curve);
                scene.model(&Modeling::Extend(-0.5, 1.5)).unwrap();
                commit(&mut scene, &mut gpu);
                assert!(scene.undo());
                commit(&mut scene, &mut gpu);
                same(&mut gpu, "trim, undo");

                let source = scene.geometry(curve).unwrap().clone();
                let Geometry::NurbsCurve(original) = &source else {
                    panic!()
                };
                let mut grown = (**original).clone();
                let (lo, hi) = grown.domain();
                assert!(grown.extend(lo, hi + (hi - lo)));
                scene.redraw(curve, &Geometry::NurbsCurve(Rc::new(grown)), true);
                scene.upload_to(&mut gpu);
                scene.redraw(curve, &source, false);
                scene.upload_to(&mut gpu);
                same(&mut gpu, "drag, cancel");
            }
        }

        /// A compaction draws what a fresh load draws, ids mapped by identity, and shrinks the lanes.
        #[test]
        #[ignore = "requires a native GPU adapter"]
        fn compaction_is_exact_and_shrinks() {
            let (mut gpu, mut scene, camera) = loaded(&Rc::new(solids()));
            let mut dice = Dice(9);

            for _ in 0..60 {
                edit(&mut scene, &mut dice);
                commit(&mut scene, &mut gpu);
            }

            assert!(scene.dead.bytes() > 0);
            scene.rewalk_editable(&mut gpu);
            let (mut fresh_gpu, fresh, _) = loaded(&scene.docs[0].session);
            let mut extra = Vec::new();

            for file in &scene.docs[1..] {
                extra.push(FileDoc {
                    name: file.name.clone(),
                    session: Rc::clone(&file.session),
                    place: file.place.clone(),
                    point_px: file.point_px,
                    display_only: false,
                });
            }

            let mut fresh = fresh;
            fresh.created_doc = scene.created_doc;

            for doc in extra {
                fresh.add_file(doc);
                fresh.upload_to(&mut fresh_gpu);
            }

            for samples in [1, 4] {
                for gpu in [&mut gpu, &mut fresh_gpu] {
                    gpu.view.msaa_forced = Some(samples);
                    gpu.resize(400, 300);
                }

                let after = shot(&mut gpu, &camera);
                let want = shot(&mut fresh_gpu, &camera);
                assert!(after.color == want.color, "{samples}x: pixels");
                assert_eq!(named(&scene, &after.ids), named(&fresh, &want.ids));
                let subs = |shot: &Shot| shot.ids.iter().map(|id| id[1]).collect::<Vec<_>>();
                assert_eq!(
                    subs(&after),
                    subs(&want),
                    "{samples}x: lane rows as a fresh load"
                );
            }

            let lanes = |gpu: &Gpu| {
                gpu.arena.allocated_bytes()
                    + gpu.segments.allocated_bytes()
                    + gpu.glyphs.allocated_bytes()
            };
            assert_eq!(
                lanes(&gpu),
                lanes(&fresh_gpu),
                "lanes as small as a fresh load"
            );
        }

        /// Time the phases of one edit in a scene the size of `view_lines`: kernel, sync, flush.
        #[test]
        #[ignore = "requires a native GPU adapter"]
        fn bench_edit_phases() {
            let mut gpu =
                pollster::block_on(Gpu::new_headless(400, 300)).expect("a native adapter");
            let mut scene = Scene::new();

            for doc in 0..9 {
                let mut session = Session::new(&format!("sheet {doc}"));

                for i in 0..80_000 {
                    let x = (i % 400) as f64;
                    let y = (i / 400) as f64 + doc as f64 * 300.0;
                    session.add_line(Line::new(x, y, 0.0, x + 0.5, y + 0.5, 0.0), None);
                }

                scene.add_file(file(&format!("sheet {doc}"), session, Xform::identity()));
                scene.upload_to(&mut gpu);
            }

            let ms = |start: std::time::Instant| start.elapsed().as_secs_f64() * 1e3;
            let mut phases = Vec::new();

            for round in 0..5 {
                let t = std::time::Instant::now();
                let (doc, guid) = scene
                    .model(&Modeling::Point([round as f64, 0.0, 0.0]))
                    .unwrap()
                    .unwrap();
                let kernel = ms(t);
                let t = std::time::Instant::now();
                scene.sync();
                let sync = ms(t);
                let t = std::time::Instant::now();
                scene.upload_to(&mut gpu);
                let flush = ms(t);
                let row = scene.row_of(doc, &guid).unwrap();
                let t = std::time::Instant::now();
                scene.delete_row(row);
                scene.sync();
                scene.upload_to(&mut gpu);
                let delete = ms(t);
                let t = std::time::Instant::now();
                scene.undo();
                scene.sync();
                scene.upload_to(&mut gpu);
                let undo = ms(t);
                phases.push(format!(
                    "create {kernel:.2}+{sync:.2}+{flush:.2} ms, delete {delete:.2} ms, undo {undo:.2} ms"
                ));
            }

            eprintln!("{} objects:\n{}", scene.object_count(), phases.join("\n"));
        }

        /// Streamed sheet segments and cloud points survive edits and both compactions.
        #[test]
        #[ignore = "requires a native GPU adapter"]
        fn streamed_rows_survive_edits_and_compaction() {
            let (mut gpu, mut scene, camera) = loaded(&Rc::new(solids()));
            shells(&mut scene);
            scene.upload_to(&mut gpu);
            let sheet = scene.sheets[0].row;
            let cloud = scene.streamed[0].row;
            let points = |gpu: &Gpu| {
                let found = gpu
                    .cloud
                    .clouds
                    .iter()
                    .find(|c| c.instance == cloud)
                    .unwrap();
                (found.resident, found.chunks.len())
            };
            let before = points(&gpu);
            let entity = |gpu: &Gpu| {
                (0..gpu.segments.ribbon_count())
                    .filter_map(|row| {
                        gpu.segments
                            .row_of(row)
                            .map(|hit| (hit, gpu.segments.source_id(row)))
                    })
                    .collect::<Vec<_>>()
            };
            let entities = entity(&gpu);
            assert_eq!(entities.len(), 2);
            let mut dice = Dice(21);

            for _ in 0..40 {
                edit(&mut scene, &mut dice);
                commit(&mut scene, &mut gpu);
            }

            // a document cloud dies and comes back until its points outweigh the rest
            for _ in 0..3 {
                if let Some(row) = find(&scene, |g| matches!(g, Geometry::PointCloud(_))) {
                    assert!(scene.delete_row(row));
                    commit(&mut scene, &mut gpu);
                    assert!(scene.undo());
                    commit(&mut scene, &mut gpu);
                }
            }

            assert!(scene.dead_points > 0);
            scene.compact_clouds(&mut gpu);
            scene.rewalk_editable(&mut gpu);
            assert_eq!(points(&gpu), before, "the streamed cloud keeps its points");
            assert_eq!(gpu.cloud.point_count, gpu.cloud.resident());
            assert_eq!(entity(&gpu), entities, "sheet segments keep their entities");
            assert_eq!(
                scene.identity_of(sheet).map(|id| id.1.to_string()),
                Some("sheet:plan.pb".into())
            );
            let now = shot(&mut gpu, &camera);
            assert!(named(&scene, &now.ids).iter().flatten().count() > 0);
        }
    }
}
