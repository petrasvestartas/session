#[path = "scene_sync/compaction.rs"]
mod compaction;

#[path = "scene_sync/tombs.rs"]
mod tombs;

#[path = "scene_sync/allocation.rs"]
mod allocation;

#[path = "scene_sync/nodes.rs"]
mod nodes;

#[path = "scene_sync/notes.rs"]
mod notes;
pub(crate) use notes::{commit, stepped};
#[path = "scene_sync/preview.rs"] // register:editing
mod preview; // register:editing
pub(crate) use preview::Previews; // register:editing

use super::Scene;
use super::rows::{
    Cap, FREE, Footprint, GEOMETRY, Note, PLACE, PRESENCE, SINK, SUBTREE, TOMB, Tomb,
};
use crate::app::walk::bounds::{Baselines, in_band, mark_pens_from};
use crate::app::walk::{Walk, WalkCx, is_drawable, walk_geometry};
use crate::engine::gpu::patch::{Counts, LaneId, Span};
use crate::engine::gpu::{Gpu, Instance, ObjectRow, Upload};
use session_rust::history;
use session_rust::session::PURGE_WORK;
use session_rust::{Collection, Geometry, Session, TreeNode, Xform};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};

type Node = Rc<RefCell<TreeNode>>;

/// Dead editable bytes that start a compaction, at least.
const COMPACT_MIN: u64 = 16 * 1024 * 1024;

/// Dead cloud points that start a cloud compaction, at least.
const CLOUD_COMPACT_MIN: u32 = 2_000_000;

/// Lane bytes deleted objects may keep on the GPU for an undo; past it the oldest are released.
pub(crate) const TOMB_CAP: u64 = 256 * 1024 * 1024;

/// GPU bytes of one cloud point: position, colour and packed normal.
const CLOUD_POINT_BYTES: u64 = 20;

/// The address of the object a geometry wraps.
fn address(geometry: &Geometry) -> usize {
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
    if record.definition {
        return None;
    }

    let slot = record.slot.get();
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
    (slot < list.number_of_slots()).then(|| Rc::as_ptr(list.get_item(slot)) as usize)
}

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

    /// Kill, create, redraw or move the row of one identity; true when a row came or went.
    fn reconcile(&mut self, item: &Work) -> bool {
        let mut instance: Option<bool> = None;
        instance = instance.or_else(|| self.reconcile_instance(item)); // register:instancing

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

        match (row, geometry) {
            (Some(row), _) if !wanted => {
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
            mark_pens_from(&mut up, &Baselines::default());
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

    /// Drop an identity's row: its lane rows go to the sink, its id waits for the end of the sync.
    pub(super) fn kill(&mut self, row: u32) {
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

        self.forget_preview(row); // register:editing
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

#[cfg(test)]
#[path = "scene_sync/testing.rs"]
mod testing;

#[cfg(test)]
#[path = "scene_sync/tests.rs"]
mod tests;

#[cfg(test)]
#[path = "scene_sync/editing_tests.rs"] // register:editing
mod editing_tests; // register:editing

#[cfg(test)]
#[path = "scene_sync/commands_tests.rs"] // register:commands
mod commands_tests; // register:commands

#[cfg(test)]
#[path = "scene_sync/panel_tests.rs"] // register:panel
mod panel_tests; // register:panel
