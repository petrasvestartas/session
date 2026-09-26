// --8<-- [start:sync-budgets]
// Sync = apply the notes of one edit to the GPU rows they name: editing 1 object of 100 000 touches 1 row.
use super::Scene;
use super::rows::{
    Cap, FREE, Footprint, GEOMETRY, Note, PLACE, PRESENCE, SINK, SUBTREE, TOMB, Tomb,
};
use crate::app::walk::bounds::{Baselines, in_band, mark_pens_from};
use crate::app::walk::{Walk, WalkCx, is_drawable, walk_geometry};
use crate::engine::gpu::faces::FaceSource;
use crate::engine::gpu::glyphs::GlyphPoint;
use crate::engine::gpu::patch::{Counts, LaneId, Span};
use crate::engine::gpu::segments::CylinderSegment;
use crate::engine::gpu::{Gpu, Instance, ObjectRow, Upload};
use session_rust::history::{self, Op, Transaction};
use session_rust::session::PURGE_WORK;
use session_rust::{Collection, Geometry, History, Session, TreeNode, Xform};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};

// `type` names a long type once; a tree node is shared by the tree and its caches, and changed through the RefCell.
type Node = Rc<RefCell<TreeNode>>;

/// Dead editable bytes that start a compaction, at least.
const COMPACT_MIN: u64 = 16 * 1024 * 1024;

/// Dead cloud points that start a cloud compaction, at least.
const CLOUD_COMPACT_MIN: u32 = 2_000_000;

/// Lane bytes deleted objects may keep on the GPU for an undo; past it the oldest are released.
pub(crate) const TOMB_CAP: u64 = 256 * 1024 * 1024;

/// GPU bytes of one cloud point: position, colour and packed normal.
const CLOUD_POINT_BYTES: u64 = 20;
// --8<-- [end:sync-budgets]

// --8<-- [start:sync-notes]
// Transaction = the ops of one user action: moving 3 objects is 1 transaction of 3 Xform ops, undone by 1 undo.
/// Commit the open transaction; its ops name the objects it touched. Every viewer edit commits here.
pub(crate) fn commit(session: &mut Session) -> Vec<Note> {
    let notes = match &session.history.current {
        Some(transaction) => applied(transaction),
        None => Vec::new(),
    };
    session.commit();
    notes
}

/// Notes of the transaction an undo (`back`) or redo just stepped.
pub(crate) fn stepped(history: &History, back: bool) -> Vec<Note> {
    // an undo moved the transaction onto the redo stack, a redo moved it back onto the undo stack
    let notes = if back {
        history.redo_stack.last().map(reverted)
    } else {
        history.undo_stack.last().map(applied)
    };
    notes.unwrap_or_default()
}

/// The note of a tree op: its node is placed and judged again with everything below; a colour change redraws.
fn tree_note(t: &session_rust::history::TreeOp) -> Note {
    let colored = t.color_before != t.color_after;
    Note {
        guid: t.node.borrow().name.as_str().into(),
        what: PLACE | PRESENCE | SUBTREE | if colored { GEOMETRY } else { 0 },
        node: Some(Rc::downgrade(&t.node)),
        parent: None,
        tomb: None,
    }
}

/// Notes of a transaction done or redone.
fn applied(transaction: &Transaction) -> Vec<Note> {
    let mut notes = Vec::with_capacity(transaction.ops.len());

    for op in &transaction.ops {
        let note = match op {
            // its own node, the same one after a redo: no search for it
            Op::Add(t) => Note {
                guid: t.guid.as_str().into(),
                what: PRESENCE | GEOMETRY,
                node: t.node.as_ref().map(Rc::downgrade),
                parent: t.parent_guid.clone().map(|parent| (parent, t.index)),
                tomb: Some(Rc::downgrade(&t.tomb)),
            },
            // its node stays in the tree, dead, with every node below it
            Op::Remove(t) => Note {
                guid: t.guid.as_str().into(),
                what: PRESENCE | SUBTREE,
                node: t.node.as_ref().map(Rc::downgrade),
                parent: None,
                tomb: Some(Rc::downgrade(&t.tomb)),
            },
            Op::Replace(r) => Note::new(&r.guid, GEOMETRY),
            // `if` on an arm is a match guard; `continue` skips the marker pair an Add Edge step leaves
            Op::Xform(x) if x.guid == transaction.label => continue,
            Op::Xform(x) => Note::new(&x.guid, PLACE | SUBTREE),
            Op::Tree(t) => tree_note(t),
        };
        notes.push(note);
    }

    notes
}

/// Notes of a transaction undone, in the order its ops were reverted.
fn reverted(transaction: &Transaction) -> Vec<Note> {
    let mut notes = Vec::with_capacity(transaction.ops.len());

    for op in transaction.ops.iter().rev() {
        let note = match op {
            Op::Add(t) => Note {
                guid: t.guid.as_str().into(),
                what: PRESENCE,
                node: t.node.as_ref().map(Rc::downgrade),
                parent: None,
                tomb: Some(Rc::downgrade(&t.tomb)),
            },
            // its node came back live with every node below it
            Op::Remove(t) => Note {
                guid: t.guid.as_str().into(),
                what: PRESENCE | GEOMETRY | SUBTREE,
                node: t.node.as_ref().map(Rc::downgrade),
                parent: None,
                tomb: Some(Rc::downgrade(&t.tomb)),
            },
            Op::Replace(r) => Note::new(&r.guid, GEOMETRY),
            Op::Xform(x) if x.guid == transaction.label => continue,
            Op::Xform(x) => Note::new(&x.guid, PLACE | SUBTREE),
            Op::Tree(t) => tree_note(t),
        };
        notes.push(note);
    }

    notes
}
// --8<-- [end:sync-notes]

// --8<-- [start:sync-addresses]
/// The address of the object a geometry wraps.
fn address(geometry: &Geometry) -> usize {
    // the heap address inside the Rc is the object's identity: equal addresses mean the very object, not a copy
    match geometry {
        Geometry::OBB(g) => Rc::as_ptr(g) as usize,
        Geometry::BRep(g) => Rc::as_ptr(g) as usize,
        Geometry::Element(g) => Rc::as_ptr(g) as usize,
        Geometry::Line(g) => Rc::as_ptr(g) as usize,
        Geometry::Mesh(g) => Rc::as_ptr(g) as usize,
        Geometry::NurbsCurve(g) => Rc::as_ptr(g) as usize,
        Geometry::NurbsSurface(g) => Rc::as_ptr(g) as usize,
        Geometry::Plane(g) => Rc::as_ptr(g) as usize,
        Geometry::Point(g) => Rc::as_ptr(g) as usize,
        Geometry::PointCloud(g) => Rc::as_ptr(g) as usize,
        Geometry::Polyline(g) => Rc::as_ptr(g) as usize,
    }
}

/// The address of the object in a kernel tomb's slot, dead or alive; None for a definition or a node.
fn slot_address(session: &Session, record: &history::Tomb) -> Option<usize> {
    // a kernel tomb records the collection slot a removed object still sits in, dead, until a purge frees it
    if record.definition {
        return None;
    }

    let slot = record.slot.get(); // a Cell: a purge can renumber the slot through a shared reference
    let objects = &session.objects;

    match record.collection.as_str() {
        "points" => held(&objects.points, slot),
        "lines" => held(&objects.lines, slot),
        "planes" => held(&objects.planes, slot),
        "bboxes" => held(&objects.bboxes, slot),
        "polylines" => held(&objects.polylines, slot),
        "pointclouds" => held(&objects.pointclouds, slot),
        "meshes" => held(&objects.meshes, slot),
        "nurbscurves" => held(&objects.nurbscurves, slot),
        "nurbssurfaces" => held(&objects.nurbssurfaces, slot),
        "breps" => held(&objects.breps, slot),
        "elements" => held(&objects.elements, slot),
        _ => None,
    }
}

/// Points of the cloud in a kernel tomb's slot; None for any other kind.
fn cloud_points(session: &Session, record: &history::Tomb) -> Option<u64> {
    let slot = record.slot.get();
    let clouds = &session.objects.pointclouds;

    if record.definition || record.collection != "pointclouds" || slot >= clouds.number_of_slots() {
        return None;
    }

    Some(clouds.get_item(slot).len() as u64)
}

/// The address of the object in `slot` of `list`.
fn held<T>(list: &Collection<Rc<T>>, slot: usize) -> Option<usize> {
    // a Collection keeps dead slots in place, so slot numbers stay valid for an undo; `then` turns true into Some
    (slot < list.number_of_slots()).then(|| Rc::as_ptr(list.get_item(slot)) as usize)
}
// --8<-- [end:sync-addresses]

// --8<-- [start:sync-work]
// Identity = (document index, guid): one file placed twice gives each of its objects two identities.
/// One identity a sync looks at.
pub(super) struct Work {
    pub(super) doc: usize,                            // its document
    pub(super) guid: Rc<str>,                         // its object or group name
    pub(super) what: u8,                              // the note bits, merged
    pub(super) weak: Option<Weak<RefCell<TreeNode>>>, // the node a note named
    pub(super) parent: Option<(String, usize)>,       // where an added object was put
    pub(super) tomb: Option<Weak<history::Tomb>>,     // the kernel tomb of its newest add or remove
    pub(super) node: Option<Node>,                    // its tree node, once resolved
    pub(super) in_tree: bool,                         // that node hangs from the document's root
}

/// Add one identity, or merge it into the entry already there.
fn merge(work: &mut Vec<Work>, at: &mut HashMap<(usize, Rc<str>), usize>, item: Work) {
    // two notes on one object become one entry with both bits: a move then a recolour is PLACE | GEOMETRY
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

            if item.tomb.is_some() {
                entry.tomb = item.tomb;
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

    // climb to the top: a removed subtree keeps its nodes, so its top is not the document's root
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

// `pub(crate) use` re-exports scene.rs's tree_key, so callers reach it as sync::tree_key too
pub(crate) use super::tree_key;

/// True when an ancestor below the root is an `attributes` group: the object is drawn by its element.
pub(super) fn baked(node: &Node) -> bool {
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
// --8<-- [end:sync-work]

// --8<-- [start:sync-queue]
// A second `impl Scene` in another file: this file is a child module of scene.rs, so it also sees Scene's private fields.
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

            for instance in &session.objects.instances {
                self.pending
                    .push((doc, Note::new(instance.guid(), PRESENCE | GEOMETRY | PLACE)));
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

    /// Fix only the rows named by `commit` and `stepped`; the GPU work waits in `staged`.
    pub(crate) fn sync(&mut self) {
        // `mem::take` moves the queue out and leaves an empty one, so the loop owns the notes while `self` stays free
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
                tomb: note.tomb,
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

        self.settle_tombs();
        self.ids.settle();

        if changed {
            self.row_revision = self.row_revision.wrapping_add(1);
        }
    }
    // --8<-- [end:sync-queue]

    // --8<-- [start:sync-find-nodes]
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
            // each `or_else` runs only when the finder before it found nothing, so the cheap ones go first
            let found = hint
                .and_then(|node| check(&session, node, &item.guid))
                .or_else(|| noted.and_then(|node| check(&session, node, &item.guid)))
                .or_else(|| self.cached(item.doc, &item.guid))
                .or_else(|| self.beside_parent(item, &session));

            // an object gone from its document only loses its row: no tree walk for it
            let gone = item.what & SUBTREE == 0
                && !session.lookup.contains_key(item.guid.as_ref())
                && !session.instance_lookup.contains_key(item.guid.as_ref());

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

        // the cache names the tree it was filled from; a replaced tree fails this one compare
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
        // a Vec used as a stack walks the tree depth first without recursion; reversed children pop in order
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
    #[cfg(test)]
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
        // an index loop, not an iterator: `merge` pushes onto `work` inside it, which a live borrow would forbid
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

                if session.lookup.contains_key(&name) || session.instance_lookup.contains_key(&name)
                {
                    let item = Work {
                        doc,
                        guid: name.as_str().into(),
                        what: PLACE | PRESENCE,
                        weak: None,
                        parent: None,
                        tomb: None,
                        node: Some(Rc::clone(&child)),
                        in_tree,
                    };
                    merge(work, at, item);
                }

                stack.extend(child.borrow().children());
            }
        }
    }
    // --8<-- [end:sync-find-nodes]

    // --8<-- [start:sync-reconcile]
    /// One row's world placement: file placement times every transform down its tree path.
    pub fn placement_of(&self, row: u32) -> Option<Xform> {
        let (doc, guid) = self.identity_of(row)?;
        self.docs.get(doc)?;
        let (node, in_tree) = match self.node_of(row) {
            Some((node, in_tree)) => (Some(node), in_tree),
            None => (None, false),
        };
        Some(self.world_place(doc, node.as_ref(), in_tree, &guid))
    }

    /// Kill, create, redraw or move the row of one identity; true when a row came or went.
    fn reconcile(&mut self, item: &Work) -> bool {
        // from lesson 21 an instanced object answers here first; None means an ordinary object
        let mut instance: Option<bool> = None;

        if let Some(changed) = instance {
            return changed;
        }

        let session = Rc::clone(&self.docs[item.doc].session);
        let geometry = session.lookup.get(item.guid.as_ref());
        let under = item.in_tree && item.node.as_ref().is_some_and(baked);
        let wanted = geometry.is_some_and(is_drawable) && !under;
        let row = self
            .guid_to_row
            .get(&(item.doc, Rc::clone(&item.guid)))
            .copied();

        // one match over a tuple names all four cases: a row not wanted, wanted without a row, both, neither
        match (row, geometry) {
            (Some(row), _) if !wanted => {
                // a delete an undo can reach is buried, anything else is killed
                match self.record_of(row, item) {
                    Some(record) => self.bury(row, record, item),
                    None => self.kill(row),
                }

                true
            }
            (None, Some(geometry)) if wanted => {
                if !self.revive(item, geometry) {
                    self.create(item, geometry);
                }

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

            // multiply from the root down: file x group x object, so moving a group moves everything below it
            let mut acc = Xform::identity();

            for step in path.iter().rev() {
                if let Some(local) = xforms.get(&step.borrow().name) {
                    acc = &acc * local; // `&a * &b` multiplies borrowed matrices: neither is moved or copied
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
        let session = &self.docs.get(doc)?.session;
        check(session, session.get_node(&guid)?, &guid)
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
            vert_base: 0, // walked alone from vertex 0; `shift_vertices` moves the rows to their real place later
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
            mark_pens_from(&mut up, &Baselines::default());
        }

        (up, object)
    }
    // --8<-- [end:sync-reconcile]

    // --8<-- [start:sync-create]
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

        // rows below object_rows are already on the GPU and change through `staged`; newer ones still wait in the tables
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
    pub(super) fn append(&mut self, up: Upload, cloud: bool) -> Footprint {
        let start = self.uploaded.plus(Counts::of(&self.tables));
        let count = Counts::of(&up);
        self.tables.merge(up, start.verts);

        if cloud {
            return Footprint::Cloud;
        }

        self.spans.foot(Span { start, count })
    }
    // --8<-- [end:sync-create]

    // --8<-- [start:sync-redraw]
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
    pub(super) fn retire(
        &mut self,
        cur: Span,
        grave: bool,
        key: &(usize, Rc<str>),
        foot: Footprint,
    ) {
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
        // headroom = spare dead rows behind a dragged object, so the next frames can grow it by half without moving it
        let pad = |rows: u32| rows.div_ceil(2); // a closure, a small unnamed function; div_ceil rounds up: 5 gives 3
        let t = &mut self.tables; // a short name for one mutable borrow; it ends before `self.dead` is used below
        let vertex = start.verts;

        for _ in 0..pad(new.verts) {
            t.arena.verts.push(bytemuck::Zeroable::zeroed()); // an all-zero vertex owned by the sink draws nothing
            t.arena.vids.push(sink);
        }

        let dead_index = |rows: u32| pad(rows).div_ceil(3) * 3; // index lanes grow in whole triangles

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
    // --8<-- [end:sync-redraw]

    // --8<-- [start:sync-bury]
    /// Drop an identity's row: its lane rows go to the sink, its id waits for the end of the sync.
    pub(super) fn kill(&mut self, row: u32) {
        let i = row as usize;
        let doc = self.owners[i];
        // `mem::replace` puts the empty name in and hands the old one back, so the guid is moved, not cloned
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

    }

    /// The kernel tomb of a dead object whose uploaded rows can wait for an undo: exact rows or a whole cloud.
    fn record_of(&self, row: u32, item: &Work) -> Option<Weak<history::Tomb>> {
        let i = row as usize;
        let foot = self.feet[i];
        let session = &self.docs[item.doc].session;

        if row >= self.object_rows
            || foot == Footprint::None
            || self.caps.contains_key(&row)
            || session.lookup.contains_key(item.guid.as_ref())
        {
            return None;
        }

        // the op's own tomb, else the one pinning its dead node
        let record = match item.tomb.as_ref().and_then(Weak::upgrade) {
            Some(record) => record,
            None => {
                let node = item.node.clone().or_else(|| self.nodes[i].upgrade())?;
                let node = node.borrow();

                if !node.is_dead() || node.name != *item.guid {
                    return None;
                }

                node.get_tomb()?
            }
        };

        slot_address(session, &record)?;
        Some(Rc::downgrade(&record))
    }

    /// Hide a deleted object's row; its lane rows and id wait for an undo.
    fn bury(&mut self, row: u32, record: Weak<history::Tomb>, item: &Work) {
        let i = row as usize;
        let place = self.doc_state[item.doc]
            .sheet
            .map(|_| self.world_place(item.doc, item.node.as_ref(), item.in_tree, &item.guid));
        let guid = std::mem::replace(&mut self.order[i], Rc::clone(&self.empty));
        let key = (self.owners[i], guid);

        // a twin buried under the same identity before
        if let Some(old) = self.tombs.remove(&key) {
            self.free_tomb(&key, old, false);
        }

        let foot = self.feet[i];
        let points = match foot {
            Footprint::Cloud => record
                .upgrade()
                .and_then(|record| cloud_points(&self.docs[item.doc].session, &record))
                .unwrap_or(0),
            _ => 0,
        };
        self.tombed = self.tombed.plus(self.spans.span(foot).count);
        self.tomb_points += points;
        self.burials += 1;
        self.staged.bury.push(row);
        self.guid_to_row.remove(&key);
        self.owners[i] = TOMB;
        self.nodes[i] = Weak::new();
        self.bounds_stale = true;
        let born = self.burials;
        self.tombs.insert(
            key,
            Tomb {
                row,
                foot,
                record,
                born,
                place,
                attributes: self.attributes,
                points,
            },
        );

    }
    // --8<-- [end:sync-bury]

    // --8<-- [start:sync-revive]
    /// Show a buried identity again when its tomb holds this very object walked the same way; false when it walks anew.
    fn revive(&mut self, item: &Work, geometry: &Geometry) -> bool {
        let key = (item.doc, Rc::clone(&item.guid));

        if !self.tombs.contains_key(&key) {
            return false;
        }

        let place = self.world_place(item.doc, item.node.as_ref(), item.in_tree, &item.guid);
        let session = &self.docs[item.doc].session;
        // placements compared as bits: the same placement is the very same 16 numbers, no tolerance
        let bits = |x: &Xform| x.m.map(f64::to_bits);
        let same = self.tombs.get(&key).is_some_and(|tomb| {
            tomb.attributes == self.attributes
                && tomb
                    .place
                    .as_ref()
                    .is_none_or(|held| bits(held) == bits(&place))
                && tomb
                    .record
                    .upgrade()
                    .and_then(|record| slot_address(session, &record))
                    == Some(address(geometry))
        });

        let Some(tomb) = self.tombs.remove(&key) else {
            return false;
        };

        // walked anew: its old rows wait as the identity's grave
        if !same {
            self.free_tomb(&key, tomb, true);
            return false;
        }

        let i = tomb.row as usize;
        self.tombed = self.tombed.minus(self.spans.span(tomb.foot).count);
        self.tomb_points -= tomb.points;
        self.order[i] = Rc::clone(&item.guid);
        self.owners[i] = item.doc;
        self.nodes[i] = item.node.as_ref().map(Rc::downgrade).unwrap_or_default();
        let hidden = if self.hidden.contains(&key) {
            Instance::FLAG_HIDDEN
        } else {
            0
        };
        let object = self.object_row(item.doc, &item.guid, place, hidden);
        self.guid_to_row.insert(key, tomb.row);
        self.staged.unbury.push((tomb.row, object));
        true
    }

    /// Hand a tomb's lane rows to the sink and its id back; its allocation waits as the identity's grave when `grave`.
    fn free_tomb(&mut self, key: &(usize, Rc<str>), tomb: Tomb, grave: bool) {
        let i = tomb.row as usize;
        let span = self.spans.span(tomb.foot);
        self.tombed = self.tombed.minus(span.count);
        self.tomb_points -= tomb.points;

        // a buried cloud's points die with it
        if tomb.foot == Footprint::Cloud {
            self.staged.clouds.push(tomb.row);
        } else {
            self.retire(span, grave, key, tomb.foot);
        }

        self.staged.retire.push(tomb.row);
        self.owners[i] = FREE;
        self.feet[i] = Footprint::None;
        self.ids.give(tomb.row);
    }

    /// GPU bytes the tombs hold: their lane rows and cloud points.
    fn tomb_bytes(&self) -> u64 {
        self.tombed.bytes() + self.tomb_points * CLOUD_POINT_BYTES
    }

    /// Release the tombs no undo reaches any more, then the oldest while they hold more than the cap.
    fn settle_tombs(&mut self) {
        if self.tombs.is_empty() {
            return;
        }

        let gone: Vec<_> = self
            .tombs
            .iter()
            .filter(|(_, tomb)| tomb.record.strong_count() == 0) // history dropped the record: no undo reaches it
            .map(|(key, _)| key.clone())
            .collect();

        for key in gone {
            if let Some(tomb) = self.tombs.remove(&key) {
                self.free_tomb(&key, tomb, false);
            }
        }

        if self.tomb_bytes() <= self.tomb_cap {
            return;
        }

        // oldest first, sorted once: a large delete past the cap stays O(n log n)
        let mut oldest: Vec<_> = self
            .tombs
            .iter()
            .map(|(key, tomb)| (tomb.born, key.clone()))
            .collect();
        oldest.sort_unstable_by_key(|(born, _)| *born);

        for (_, key) in oldest {
            if self.tomb_bytes() <= self.tomb_cap {
                break;
            }

            if let Some(tomb) = self.tombs.remove(&key) {
                self.free_tomb(&key, tomb, true);
            }
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
    // --8<-- [end:sync-revive]

    // --8<-- [start:sync-compaction]
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

    // Compaction = walk every editable object again into fresh lanes without gaps; row ids stay, so selection survives.
    /// True when the dead editable rows pass 16 MiB and, with the tombs, outweigh the live ones.
    pub(crate) fn compaction_due(&self) -> bool {
        let dead = self.dead.bytes();
        let tombs = self.tombed.bytes();
        let live = self.uploaded.bytes().saturating_sub(dead + tombs);
        dead >= COMPACT_MIN && dead + tombs >= live
    }

    // Purge = the kernel freeing what no undo reaches, 16 384 slots per idle frame (about 2 ms), so no frame stalls.
    /// Spend one idle kernel purge step on every document that owes one; true while a cycle is unfinished.
    pub(crate) fn purge_step(&mut self) -> bool {
        let mut running = false;

        for file in &mut self.docs {
            let owed = file.session.purge_due() || file.session.is_purging();

            // a shared session waits: purging it would copy it and drop its history
            if !owed || Rc::strong_count(&file.session) > 1 {
                continue;
            }

            // `Rc::make_mut` gives mutable access to the inside; with one owner, checked above, it copies nothing
            running |= Rc::make_mut(&mut file.session).purge_step(PURGE_WORK);
        }

        running
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
        // from lesson 21 a released document first asks for its kernel data back
        let mut waiting = false;

        if waiting {
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

        // the fresh lanes hold no tomb: an undo walks the object again
        for (_, tomb) in std::mem::take(&mut self.tombs) {
            if tomb.foot == Footprint::Cloud {
                self.staged.clouds.push(tomb.row);
            }

            self.staged.retire.push(tomb.row);
            self.owners[tomb.row as usize] = FREE;
            self.feet[tomb.row as usize] = Footprint::None;
            self.ids.give(tomb.row);
        }

        self.tombed = Counts::default();
        self.tomb_points = 0;

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
            let from = Baselines::capture(&self.tables);
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
                mark_pens_from(&mut self.tables, &from);
            }

            self.feet[row as usize] = self.spans.foot(Span {
                start,
                count: end.minus(start),
            });
            self.staged.geometry.push((row, object));
        }

    }
    // --8<-- [end:sync-compaction]

    // --8<-- [start:sync-counters]
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

    /// Tombs for the inspection: (deleted objects kept on the GPU, their lane bytes).
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn tomb_counters(&self) -> (usize, u64) {
        (self.tombs.len(), self.tomb_bytes())
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
// --8<-- [end:sync-counters]

// --8<-- [start:sync-test-scene]
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
        const OWN: u32 = Instance::FLAG_SELECTED
            | Instance::FLAG_HIDDEN
            | Instance::FLAG_COLOR
            | Instance::FLAG_EDGE_COLOR
            | Instance::FLAG_DEAD;
        let staged = std::mem::take(&mut self.staged);

        for row in &staged.retire {
            self.ledger.remove(row);
        }

        for row in &staged.bury {
            if let Some(held) = self.ledger.get_mut(row) {
                held.flags = (held.flags & !Instance::FLAG_SELECTED)
                    | Instance::FLAG_HIDDEN
                    | Instance::FLAG_DEAD;
            }
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
                held.hull = object.hull.clone();
                held.spacing = object.spacing;
                held.faces = object.faces;
            }
        }

        for (row, object) in staged.unbury {
            if let Some(held) = self.ledger.get_mut(&row) {
                held.flags = (held.flags & !OWN) | (object.flags & OWN);
                held.color = object.color;
                held.edge_color = object.edge_color;
                held.place = object.place;
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
                held.hull.as_deref(),
                want.hull.as_deref(),
                "{id:?} extreme points"
            );
            assert_eq!(
                held.spacing.to_bits(),
                want.spacing.to_bits(),
                "{id:?} spacing"
            );
            assert_eq!(held.faces, want.faces, "{id:?} faces");
            // a sheet band is judged once, at load: a fresh walk of edited content may judge another
            let judged = self.doc_state[id.0].sheet == fresh.doc_state[id.0].sheet;
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
                owner == u32::MAX
                    || self.identity_of(owner).is_some()
                    || self.owners.get(owner as usize) == Some(&TOMB),
                "a pipe names dead row {owner}"
            );
        }

        let mut tombed = Counts::default();

        for tomb in self.tombs.values() {
            assert_eq!(
                self.owners[tomb.row as usize], TOMB,
                "row {} is a tomb",
                tomb.row
            );
            tombed = tombed.plus(self.spans.span(tomb.foot).count);
        }

        assert!(tombed == self.tombed, "tomb rows add up");
        assert!(
            self.dead.plus(self.tombed).fits(&self.uploaded),
            "dead and buried rows are uploaded rows"
        );
        assert_eq!(self.pending.len(), 0);
    }
}
// --8<-- [end:sync-test-scene]

// --8<-- [start:sync-test-helpers]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::scene::{FileDoc, SheetInit, StreamedInit};
    use crate::app::stream::{CloudFields, CloudLod, SheetFields};
    use crate::app::walk::cloud::StreamRows;
    use crate::app::walk::sheet::SheetRows;
    use session_rust::element::ElementFeature;
    use session_rust::{Element, Line, Mesh, NurbsCurve, Point, Polyline};

    /// A document placed at `place`.
    pub(super) fn file(name: &str, session: Session, place: Xform) -> FileDoc {
        FileDoc {
            name: name.into(),
            session: Rc::new(session),
            place,
            point_px: 0.0,
            display_only: false,
        }
    }

    /// A point.
    pub(super) fn p(x: f64, y: f64, z: f64) -> Point {
        Point::new(x, y, z)
    }

    /// A curve through eight points of an arc: trimming it changes its sample count.
    pub(super) fn arc() -> NurbsCurve {
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
    pub(super) fn other() -> Session {
        let mut session = Session::new("other");
        let inbox = session.add_group("inbox");
        session.add_point(p(3.0, 3.0, 3.0), Some(&inbox));
        session
    }

    /// A streamed sheet and a streamed cloud, read-only shells.
    pub(super) fn shells(scene: &mut Scene) {
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
    pub(super) fn scene() -> Scene {
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
    pub(super) fn check(scene: &mut Scene) {
        scene.sync();
        scene.settle();
        scene.verify();
    }

    /// Live rows of editable documents, by identity.
    pub(super) fn live(scene: &Scene) -> Vec<(u32, (usize, Rc<str>))> {
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
    pub(super) fn find(scene: &Scene, test: impl Fn(&Geometry) -> bool) -> Option<u32> {
        live(scene)
            .into_iter()
            .map(|(row, _)| row)
            .find(|&row| scene.geometry(row).is_some_and(&test))
    }

    /// A tiny deterministic random source.
    pub(super) struct Dice(pub(super) u64);

    impl Dice {
        /// A number below `n`.
        pub(super) fn roll(&mut self, n: usize) -> usize {
            self.0 = self
                .0
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            ((self.0 >> 33) as usize) % n.max(1)
        }
    }

    /// Hide or color one live object as the panel does: its identity and its row.
    pub(super) fn mark(scene: &mut Scene, dice: &mut Dice) {
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
}
// --8<-- [end:sync-test-helpers]
