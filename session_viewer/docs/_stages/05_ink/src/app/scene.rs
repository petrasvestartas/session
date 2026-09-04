//! The document side: `Scene` owns WHAT is loaded - every kernel `Session` with its placement,
//! the `Upload` tables and the row bookkeeping. `add_file` walks one
//! session into the tables; rows are appended, never rebuilt. This file never names a
//! `Geometry` variant - the producers live in `walk/`.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use session_rust::{Session, Xform};
use crate::app::knobs;
use crate::app::walk::bounds::{file_extent, Baselines};
use crate::app::walk::hosts::Hosts;
use crate::app::walk::mesh::Lap;
use crate::app::walk::{is_drawable, walk_geometry, Walk, WalkCx};
use crate::engine::gpu::{Gpu, Instance, ObjectRow, Upload};
use crate::math::{mat_mul, Mat4};

/// One loaded file: the kernel `Session` (kept for picking, editing and saving) plus the
/// placement the manifest gave it.
pub struct Doc {
    pub name: String,
    pub place: Xform,
    /// Shared with whoever decoded it, never copied.
    pub session: Rc<Session>,
    /// The session was RELEASED after the walk (manifest `display_only`): an empty shell.
    pub display_only: bool,
}

/// One parsed file on its way into the scene.
pub struct FileDoc {
    pub name: String,
    pub session: Rc<Session>,
    pub place: Xform,
    pub display_only: bool,
}

/// Rows already uploaded, per table with global numbering.
#[derive(Default)]
struct Bases {
    vert: u32,
    obj: u32,
}

/// The open document set, the pending upload and the row bookkeeping.
pub struct Scene {
    pub docs: Vec<Doc>,
    pub tables: Upload,
    pub hidden: HashSet<String>,
    order: Vec<Rc<str>>,
    guid_to_row: HashMap<Rc<str>, u32>,
    bases: Bases,
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

impl Scene {
    /// Empty: no documents, no rows.
    pub fn new() -> Self {
        Self {
            docs: Vec::new(),
            tables: Upload::default(),
            hidden: HashSet::new(),
            order: Vec::new(),
            guid_to_row: HashMap::new(),
            bases: Bases::default(),
        }
    }

    /// Drop every document and its GPU rows, keeping the scene usable: a scene can be
    /// REPLACED without tearing down `State` (camera, surface and pipelines survive).
    pub fn clear(&mut self, gpu: &mut Gpu) {
        self.docs.clear();
        self.tables = Upload::default();
        self.order.clear();
        self.guid_to_row.clear();
        self.hidden.clear();
        self.bases = Bases::default();
        gpu.release();
    }

    /// Re-flatten EVERY document from its kernel `Session` and re-upload from scratch - the
    /// path an edit commit takes.
    pub fn rebuild(&mut self, gpu: &mut Gpu) {
        let docs = std::mem::take(&mut self.docs);
        self.tables = Upload::default();
        self.order.clear();
        self.guid_to_row.clear();
        self.bases = Bases::default();
        gpu.reset();
        for d in docs {
            if d.display_only {
                log::warn!("rebuild: '{}' is display_only, its geometry was released", d.name);
            }
            self.add_file(FileDoc { name: d.name, session: d.session, place: d.place, display_only: d.display_only });
        }
        self.upload_to(gpu);
    }

    /// Upload the walked tables, then FORGET the rows: the GPU is their only holder.
    pub fn upload_to(&mut self, gpu: &mut Gpu) {
        gpu.set_scene(&self.tables);
        self.bases.vert += self.tables.arena.verts.len() as u32;
        self.bases.obj += self.tables.obj.rows.len() as u32;
        self.tables.drop_uploaded();
    }

    /// The next object row and its guid bookkeeping.
    fn push_row(&mut self, guid: &str, place: Mat4, flags: u32) -> u32 {
        let row = self.bases.obj + self.tables.obj.rows.len() as u32;
        self.tables.obj.rows.push(ObjectRow::new(place, flags));
        let guid: Rc<str> = Rc::from(guid);
        self.guid_to_row.insert(Rc::clone(&guid), row);
        self.order.push(guid);
        row
    }

    /// Walk one session into the tables: one object row per guid in the kernel's canonical
    /// order (the row a guid gets is the row it keeps), then the per-file sweeps.
    pub fn add_file(&mut self, doc: FileDoc) {
        let FileDoc { name, session, place, display_only } = doc;
        let from = Baselines::capture(&self.tables);
        let world = session.world_xforms();
        let mut lap = Lap::start("walk");
        let hosts = Hosts::from_session(&session);
        lap.mark("hosts");
        let count = session.lookup.len();
        self.tables.obj.rows.reserve(count);
        self.order.reserve(count);
        self.guid_to_row.reserve(count);

        for guid in session.order() {
            let Some(geom) = session.lookup.get(&guid) else { continue };
            if !is_drawable(geom) {
                continue;
            }
            let flags = if self.hidden.contains(&guid) { Instance::FLAG_HIDDEN } else { 0 };
            let row = self.push_row(&guid, placement(&world, &place.m, &guid), flags);
            let cx = WalkCx { vert_base: self.bases.vert, row, hosts: &hosts };
            let r = walk_geometry(&mut Walk::of(&mut self.tables), &cx, geom);
            let o = self.tables.obj.rows.last_mut().unwrap();
            o.flags |= r.flags;
            o.bounds = r.bounds;
            o.spacing = r.spacing;
            o.faces = r.faces;
            o.thickness = r.thickness;
        }
        lap.mark("objects");

        let extent = file_extent(&self.tables, &from);
        self.tables.bounds.union(&extent);
        lap.mark("sweeps");

        let display_only = display_only || knobs::drop_sessions();
        let session = if display_only { Rc::new(Session::new(&name)) } else { session };
        self.docs.push(Doc { name, place, session, display_only });
    }

    /// Objects in row order.
    pub fn object_count(&self) -> usize {
        self.order.len()
    }
}

/// An object's placement: the manifest `place` times the session's own world xform for that
/// guid. Composed once per row on raw matrices - no kernel `Xform` allocations.
fn placement(world: &HashMap<String, Xform>, place: &Mat4, guid: &str) -> Mat4 {
    match world.get(guid) {
        Some(local) => mat_mul(place, &local.m),
        None => *place,
    }
}
