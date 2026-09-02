//! The document side of the scene: `Scene` owns WHAT is loaded - every kernel `Session` with
//! its placement, the merged `Upload` tables and the row bookkeeping. `add_file` walks one new
//! session into the shared tables; rows are appended, never rebuilt. The producers live in
//! `walk/`; this file never names a `Geometry` variant.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use session_rust::{Session, Xform};
use crate::engine::gpu::{Gpu, Instance, Upload};
use crate::math::{mat_mul, Aabb, Mat4};
use crate::app::knobs;
use crate::app::walk::{is_drawable, walk_geometry, Walk, WalkCx};
use crate::app::walk::bounds::{file_extent, mark_sheet, sheet_thickness, Baselines};
use crate::app::walk::mesh::Lap;

/// One loaded file: the kernel `Session` kept alive (picking, undo and save read it) plus the
/// placement the manifest gave it.
pub struct Doc {
    pub name: String,
    pub place: Xform,
    pub session: Session,
    pub point_px: f32, // per-file cloud point size, px; 0 = the pb's own
    /// This doc's `session` was RELEASED after the walk (manifest `display_only`): an empty
    /// shell that still names the document and holds its placement. `rebuild` cannot bring it back.
    pub display_only: bool,
}

/// One parsed file on its way into the scene: what the loader hands `add_file`.
pub struct FileDoc {
    pub name: String,
    pub session: Session,
    pub place: Xform,
    pub point_px: f32,
    pub display_only: bool,
}

/// A cloud about to stream in: the count is known from the file's packed-double length prefix
/// before a single point has been fetched.
pub struct CloudBegin {
    pub name: String,
    pub place: Xform,
    pub count: u32,
    pub px: f32,
}

/// A cloud whose points never became kernel objects: the loader streamed them from the file
/// straight into GPU memory. This is the ENTIRE CPU-side footprint of a 13.8M-point scan.
pub struct CloudSlot {
    pub name: String,
    pub place: Xform,
    pub count: u32,
    pub px: f32,
    pub instance: u32,
}

/// The open document set + the merged GPU tables. Rows are appended, never rebuilt, so
/// progressive loading costs each file only its own walk. Viewer-only bookkeeping (row order,
/// guid -> row, hidden) lives here, never in the kernel type three languages share. Both
/// directions of the guid map share ONE `Rc<str>` per object - picking and selection will
/// need both, and two Strings per guid were 24% of the per-object cost.
pub struct Scene {
    pub docs: Vec<Doc>,
    pub clouds: Vec<CloudSlot>,
    pub tables: Upload,
    order: Vec<Rc<str>>, // renderable guids, global row order across docs
    pub guid_to_row: HashMap<Rc<str>, u32>,
    pub hidden: HashSet<String>,
    bases: Bases,
}

/// Rows already uploaded, per table that keeps global numbering: the next walk counts from here.
#[derive(Default)]
struct Bases {
    vert: u32,  // arena rows - walk_mesh bases its indices on this
    cloud: u32, // cloud points - a draw record's `first` counts from here
    obj: u32,   // object rows - every `instance_id` counts from here
}

impl Scene {
    /// Empty: no documents, inverted bounds, bases at 0.
    pub fn new() -> Self {
        Self {
            docs: Vec::new(),
            clouds: Vec::new(),
            tables: Upload::default(),
            order: Vec::new(),
            guid_to_row: HashMap::new(),
            hidden: HashSet::new(),
            bases: Bases::default(),
        }
    }

    /// Drop every document and its GPU rows, keeping the scene usable: the counterpart to
    /// `rebuild`, same reset minus the re-walk, so a scene can be REPLACED without tearing down
    /// `State` (camera, surface and pipelines survive a reload).
    pub fn clear(&mut self, gpu: &mut Gpu) {
        self.docs.clear();
        self.tables = Upload::default();
        self.order.clear();
        self.guid_to_row.clear();
        self.hidden.clear();
        self.bases = Bases::default();
        gpu.release();
    }

    /// Widen the shared walk box by a streamed cloud's world AABB. Without this the box lives
    /// only in `Gpu` and the next `set_scene` from a real walk would replace it.
    pub fn grow_bounds(&mut self, world: &Aabb) {
        self.tables.bounds.union(world);
    }

    /// Reserve the document row for a cloud that is about to stream in. Returns the instance
    /// row the streamed points will draw against; `order` stays aligned with the rows.
    pub fn begin_cloud(&mut self, c: CloudBegin) -> u32 {
        let CloudBegin { name, place, count, px } = c;
        let row = self.bases.obj + self.tables.obj.rows.len() as u32;
        self.tables.obj.rows.push((place.m, [1.0; 4], 0));
        self.tables.obj.bounds.push(None);
        self.tables.obj.spacing.push(px); // the manifest px rides the spacing row, like the walk's clouds
        let guid: Rc<str> = Rc::from(format!("cloud:{name}"));
        self.guid_to_row.insert(Rc::clone(&guid), row);
        self.order.push(guid);
        self.clouds.push(CloudSlot { name, place, count, px, instance: row });
        row
    }

    /// Re-flatten EVERY document from its kernel `Session` and re-upload from scratch. Once
    /// `upload_to` drops the tables there is no CPU copy left to patch, so a geometry edit has
    /// nothing to rewrite; a full re-walk belongs behind an edit commit, not behind a drag.
    pub fn rebuild(&mut self, gpu: &mut Gpu) {
        let docs = std::mem::take(&mut self.docs);
        let clouds = std::mem::take(&mut self.clouds);
        self.tables = Upload::default();
        self.order.clear();
        self.guid_to_row.clear();
        self.bases = Bases::default();
        gpu.reset_arena();

        for d in docs {
            if d.display_only {
                // Nothing to re-walk - the kernel document was released after the first walk.
                // Saying so beats silently dropping the sheet out of the frame.
                log::warn!("rebuild: '{}' is display_only, its geometry was released", d.name);
            }
            self.add_file(FileDoc { name: d.name, session: d.session, place: d.place, point_px: d.point_px, display_only: d.display_only });
        }
        // Clouds keep their GPU rows; only the instance they draw against is re-issued and the
        // Gpu's stream draw list patched to match. Index i here is index i there.
        for (i, c) in clouds.into_iter().enumerate() {
            let row = self.begin_cloud(CloudBegin { name: c.name, place: c.place, count: c.count, px: c.px });
            gpu.stream.retarget(i, row);
        }
        self.upload_to(gpu);
    }

    /// Upload the walked tables, then FORGET the rows (`Upload::drop_uploaded`): the GPU is
    /// their only holder. Only the running bases stay, so the next file's indices still land in
    /// the right place.
    pub fn upload_to(&mut self, gpu: &mut Gpu) {
        gpu.set_scene(&self.tables);
        self.bases.vert += self.tables.arena.verts.len() as u32;
        self.bases.cloud += (self.tables.cloud.pos.len() / 3) as u32;
        self.bases.obj += self.tables.obj.rows.len() as u32;
        self.tables.drop_uploaded();
    }

    /// Walk one session into the shared tables: one object row per guid in the kernel's
    /// canonical `order()` (the row a guid gets here is the row it keeps - picking relies on
    /// it), then the per-file sweeps: extent, planar test, sheet marking.
    pub fn add_file(&mut self, doc: FileDoc) {
        let FileDoc { name, session, place, point_px, display_only } = doc;
        let from = Baselines::capture(&self.tables, self.bases.cloud, self.bases.obj);
        let (vb, cb, ob) = (self.bases.vert, self.bases.cloud, self.bases.obj); // read before `t` borrows self.tables
        let world = session.world_xforms();
        let place_m = place.m;
        let mut lap = Lap::start("walk");
        let t = &mut self.tables;
        let count = session.lookup.len();
        t.obj.rows.reserve(count);
        t.obj.bounds.reserve(count);
        t.obj.spacing.reserve(count);
        self.order.reserve(count);
        self.guid_to_row.reserve(count);
        for guid in session.order() {
            let Some(geom) = session.lookup.get(&guid) else { continue };
            if !is_drawable(geom) { continue }
            let ri = ob + t.obj.rows.len() as u32;
            let flags = if self.hidden.contains(&guid) { Instance::FLAG_HIDDEN } else { 0 };
            t.obj.rows.push((placement(&world, &place_m, &guid), [1.0; 4], flags));
            let cx = WalkCx { vert_base: vb, cloud_base: cb, cloud_px: point_px, row: ri };
            let r = walk_geometry(&mut Walk::of(t), &cx, geom);
            t.obj.rows.last_mut().unwrap().2 |= r.flags;
            t.obj.bounds.push(r.bounds);
            t.obj.spacing.push(r.spacing);
            let guid: Rc<str> = Rc::from(guid);
            self.guid_to_row.insert(Rc::clone(&guid), ri);
            self.order.push(guid);
        }
        lap.mark("objects");

        let extent = file_extent(t, &from);
        lap.mark("bounds");
        t.bounds.union(&extent);

        let thickness = sheet_thickness(t, &from, &place, &extent);
        let planar = thickness.is_finite() && thickness.abs() < 1e-3;

        if planar { mark_sheet(t, &from) }

        // The walk is done and the tables are about to be uploaded, so a display-only document
        // has nothing left to answer: release it here, the exact point after which nothing
        // reads it. VIEWER_DROP_SESSIONS=1 forces it on for every file.
        let display_only = display_only || knobs::drop_sessions();
        let session = if display_only { Session::new(&name) } else { session };
        self.docs.push(Doc { name, place, session, point_px, display_only });
    }
}

/// An object's placement: the manifest `place` times the session's own world xform for that
/// guid. The 99% path (a flat sheet, a mesh file) has NO local transforms, so every row's
/// placement IS the file placement - `place_m` is composed once, not 90k times with kernel `Xform`s.
fn placement(world: &HashMap<String, Xform>, place_m: &Mat4, guid: &str) -> Mat4 {
    match world.get(guid) {
        Some(local) => mat_mul(place_m, &local.m),
        None => *place_m,
    }
}
