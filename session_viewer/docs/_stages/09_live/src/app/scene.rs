//! The document side: `Scene` owns WHAT is loaded - every kernel `Session` with its placement,
//! the `Upload` tables, the row bookkeeping and the streamed-cloud slots. `add_file` walks one
//! session into the tables; rows are appended, never rebuilt. This file never names a
//! `Geometry` variant - the producers live in `walk/`.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use session_rust::{Session, Xform};
use crate::app::knobs;
use crate::app::stream::{CloudFields, CloudLod};
use crate::app::walk::bounds::{file_extent, is_planar, mark_sheet, Baselines};
use crate::app::walk::cloud::{walk_stream_slice, StreamRows, StreamSlice};
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
    /// Shared with whoever decoded it (the live source keeps its current set), never copied.
    pub session: Rc<Session>,
    pub point_px: f32,
    /// The session was RELEASED after the walk (manifest `display_only`): an empty shell.
    pub display_only: bool,
}

/// One parsed file on its way into the scene.
pub struct FileDoc {
    pub name: String,
    pub session: Rc<Session>,
    pub place: Xform,
    pub point_px: f32,
    pub display_only: bool,
}

/// A streamed cloud's first slice and what later slices need: its file's node table, how
/// many points are resident, and the total.
pub struct StreamedInit {
    pub name: String,
    pub url: String,
    pub place: Xform,
    pub rows: StreamRows,
    pub lod: CloudLod,
    /// Where the packed arrays sit in the file, so the next slices need no second probe.
    pub fields: CloudFields,
    pub resident: u32,
    pub point_px: f32,
    /// Where the colour run continues for the next slice.
    pub col_at: u64,
}

/// A cloud still arriving off the wire.
pub struct StreamedCloud {
    pub name: String,
    pub url: String,
    pub row: u32,
    pub lod: CloudLod,
    pub done_to: u32,
    pub total: u32,
    pub point_px: f32,
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
    pub streamed: Vec<StreamedCloud>,
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
            streamed: Vec::new(),
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
        self.streamed.clear();
        self.order.clear();
        self.guid_to_row.clear();
        self.hidden.clear();
        self.bases = Bases::default();
        gpu.release();
    }

    /// Re-flatten EVERY document from its kernel `Session` and re-upload from scratch - the
    /// path an edit commit takes. Streamed clouds cannot come back (no kernel object).
    pub fn rebuild(&mut self, gpu: &mut Gpu) {
        let docs = std::mem::take(&mut self.docs);
        self.tables = Upload::default();
        self.streamed.clear();
        self.order.clear();
        self.guid_to_row.clear();
        self.bases = Bases::default();
        gpu.reset();
        for d in docs {
            if d.display_only {
                log::warn!("rebuild: '{}' is display_only, its geometry was released", d.name);
            }
            self.add_file(FileDoc { name: d.name, session: d.session, place: d.place, point_px: d.point_px, display_only: d.display_only });
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
        let FileDoc { name, session, place, point_px, display_only } = doc;
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
            let cx = WalkCx { vert_base: self.bases.vert, cloud_px: point_px, row, hosts: &hosts };
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
        if is_planar(&self.tables, &from, &place.m) {
            mark_sheet(&mut self.tables, &from);
        }
        lap.mark("sweeps");

        let display_only = display_only || knobs::drop_sessions();
        let session = if display_only { Rc::new(Session::new(&name)) } else { session };
        self.docs.push(Doc { name, place, session, point_px, display_only });
    }

    /// Add a streamed cloud from its first slice and upload it at once, so the slot knows the
    /// absolute row its point 0 landed on. Returns the slot index later slices address.
    pub fn add_streamed_cloud(&mut self, init: StreamedInit, gpu: &mut Gpu) -> usize {
        let StreamedInit { name, url, place, rows, lod, fields, resident, point_px, col_at: _ } = init;
        let total = fields.count;
        let row = self.push_row(&format!("stream:{url}"), place.m, 0);
        let slice = StreamSlice { rows, lod: &lod, from: 0, to: resident, row, point_px };
        let bounds = walk_stream_slice(&mut self.tables.cloud, &slice);
        let o = self.tables.obj.rows.last_mut().unwrap();
        o.bounds = bounds;
        o.spacing = point_px;
        o.thickness = bounds.thinnest();
        self.tables.bounds.union(&bounds.placed(&place.m));
        self.upload_to(gpu);

        self.docs.push(Doc { name: name.clone(), place, session: Rc::new(Session::new(&name)), point_px, display_only: true });
        self.streamed.push(StreamedCloud { name, url, row, lod, done_to: resident, total, point_px });
        self.streamed.len() - 1
    }

    /// Append the next slice `[done_to, to)` of streamed cloud `idx` and upload it.
    pub fn extend_streamed_cloud(&mut self, idx: usize, rows: StreamRows, to: u32, gpu: &mut Gpu) {
        let Some(sc) = self.streamed.get(idx) else { return };
        if to <= sc.done_to {
            return;
        }
        let place = self.docs.iter().find(|d| d.name == sc.name).map(|d| d.place.m).unwrap_or(Xform::identity().m);
        let slice = StreamSlice { rows, lod: &sc.lod, from: sc.done_to, to, row: sc.row, point_px: sc.point_px };
        let bounds = walk_stream_slice(&mut self.tables.cloud, &slice);
        self.tables.bounds.union(&bounds.placed(&place));
        self.streamed[idx].done_to = to;
        self.upload_to(gpu);
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
