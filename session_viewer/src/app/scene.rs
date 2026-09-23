#[path = "scene_text.rs"]
mod text;
pub use text::SceneText;

#[path = "scene_rows.rs"]
pub(crate) mod rows;

#[path = "scene_sync.rs"]
pub(crate) mod sync;

use crate::app::knobs;
use crate::app::sheet_query::{EntityMeta, SheetTable};
use crate::app::stream::{CloudFields, CloudLod, SheetFields};
use crate::app::walk::bounds::{Baselines, file_extent, mark_sheet, planar_band};
use crate::app::walk::cloud::{StreamRows, StreamSlice, walk_stream_slice};
use crate::app::walk::mesh::Lap;
use crate::app::walk::sheet::{SheetRows, SheetSlice, walk_sheet_slice};
use crate::app::walk::{Walk, WalkCx, is_drawable, walk_geometry};
use crate::engine::gpu::patch::{Counts, LaneId, Span};
use crate::engine::gpu::{Gpu, Instance, ObjectRow, Pick, Upload};
use rows::{Cap, DocState, FREE, Footprint, Ids, Note, SINK, Spans, Staged};
use session_rust::{Geometry, Session, TreeNode, Xform};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};

/// Why a streamed object refuses an edit.
pub const READ_ONLY: &str =
    "Streamed objects are display only: select, hide and query them, but they cannot be edited";

/// One loaded file and its placement.
pub struct FileDoc {
    pub name: String,         // display name
    pub place: Xform,         // world placement
    pub session: Rc<Session>, // the kernel document, shared when a file is loaded twice
    pub point_px: f32,        // point size override, 0 = the file's own
    pub display_only: bool,   // a streamed shell with no kernel objects
}

/// A streamed cloud's first slice.
pub struct StreamedInit {
    pub name: String,        // display name
    pub url: String,         // the cloud file
    pub place: Xform,        // world placement
    pub rows: StreamRows,    // the first points
    pub lod: CloudLod,       // the whole node table
    pub fields: CloudFields, // array positions in the file
    pub resident: u32,       // points in this slice
    pub point_px: f32,       // point size override
    pub col_at: u64,         // byte position of the next colour
}

/// A streamed cloud's slot in the scene.
pub struct StreamedCloud {
    pub name: String,        // display name
    pub url: String,         // the cloud file
    pub row: u32,            // its object row
    pub lod: CloudLod,       // the whole node table
    pub fields: CloudFields, // array positions in the file
    pub place: Xform,        // world placement
    pub done_to: u32,        // points loaded so far
    pub total: u32,          // points in the file
    pub point_px: f32,       // point size override
}

/// A streamed sheet's first slice.
pub struct SheetInit {
    pub name: String,             // display name
    pub url: String,              // the sheet file
    pub meta_url: Option<String>, // its entity side table
    pub place: Xform,             // world placement
    pub rows: SheetRows,          // the first segments
    pub fields: SheetFields,      // array positions in the file
    pub resident: u32,            // segments in this slice
}

/// A sheet's slot in the scene.
pub struct SheetBatch {
    pub name: String,                        // display name
    pub url: String,                         // the sheet file
    pub meta_url: Option<String>,            // its entity side table
    pub row: u32,                            // its object row
    pub fields: SheetFields,                 // array positions in the file
    pub place: Xform,                        // world placement
    pub done_to: u32,                        // segments loaded so far
    pub total: u32,                          // segments in the file
    pub resolved: Option<(u32, EntityMeta)>, // entity the last pick found
    pub table: Option<SheetTable>,           // side table head, read once
}

/// What a pick landed on.
#[derive(Clone, Debug)]
pub struct Picked {
    pub doc: String,                // document name
    pub guid: String,               // object guid
    pub row: u32,                   // object row
    pub point: Option<PickedPoint>, // the point, for a cloud
    pub entity: Option<u32>,        // the entity id, for a sheet
}

/// A picked cloud point.
#[derive(Clone, Debug)]
pub struct PickedPoint {
    pub local: u32,         // index in the cloud
    pub id: u32,            // the point's stable id
    pub position: [f64; 3], // world position
}

/// The open documents and their object rows; a row id stays with its object for the object's life.
pub struct Scene {
    pub docs: Vec<FileDoc>,                              // loaded files
    pub texts: Vec<SceneText>,                           // text objects
    pub tables: Upload,                                  // rows walked but not yet uploaded
    pub streamed: Vec<StreamedCloud>,                    // streamed clouds
    pub sheets: Vec<SheetBatch>,                         // streamed sheets
    pub hidden: HashSet<(usize, Rc<str>)>,               // (document, guid) hidden
    pub locked: HashSet<(usize, Rc<str>)>,               // (document, guid) not selectable
    pub colors: HashMap<(usize, Rc<str>), [u8; 3]>,      // face colour overrides
    pub edge_colors: HashMap<(usize, Rc<str>), [u8; 3]>, // edge colour overrides
    pub selected: Option<u32>,                           // selected object row
    pub attributes: bool,                                // element features drawn
    order: Vec<Rc<str>>,                                 // guid of each row, empty when free
    owners: Vec<usize>,   // document of each row, or TEXT, FREE, SINK
    feet: Vec<Footprint>, // lane rows of each row
    nodes: Vec<Weak<RefCell<TreeNode>>>, // tree node each row was placed from; a cache
    spans: Spans,         // footprints spanning several lanes
    caps: HashMap<u32, Cap>, // rows owning more lane rows than they fill
    graves: HashMap<(usize, Rc<str>), Footprint>, // an object's last allocation, kept for its return
    ids: Ids,                                     // row ids nothing holds
    sink: Option<u32>,                            // the hidden row dead lane rows point at
    empty: Rc<str>,                               // the guid of free and sink rows
    doc_state: Vec<DocState>,                     // per document: sheet band, node cache source
    pending: Vec<(usize, Note)>,                  // what edits did, not yet synced
    hints: Vec<(usize, Weak<RefCell<TreeNode>>)>, // tree nodes edit sites already hold
    staged: Staged,                               // GPU work of the last sync
    dead: Counts,                                 // lane rows holding nothing live
    dead_points: u32,                             // cloud points of dropped clouds
    compactions: u32,                             // editable lanes walked again
    loaded: bool,                                 // rows of a new document wait in the tables
    preview: Option<(u32, sync::Previews)>,       // drag previews of one row
    pub(crate) bounds_stale: bool,                // the scene box may be larger than what is left
    edge_sources: Vec<(u32, u32)>,                // (object row, edge index) of each pipe
    guid_to_row: HashMap<(usize, Rc<str>), u32>,  // (document, guid) to row
    object_rows: u32,                             // object rows already on the GPU
    uploaded: Counts,                             // editable lane rows already on the GPU
    pub(crate) undo_steps: Vec<Vec<(usize, String)>>, // each edit: (document, undo label) per document it changed
    pub(crate) redo_steps: Vec<Vec<(usize, String)>>, // undone edits, the newest last
    pub(crate) created_doc: Option<usize>,            // the `Created` document
    pub(crate) row_revision: u64,                     // bumped when rows come or go
    pub(crate) current_layer: Option<(usize, String)>, // (document, tree node) new objects go to
    pub(crate) layer_trees: HashMap<(usize, String), crate::app::layers::LayerStep>, // kept tree per (document, layer step)
    pub(crate) layer_steps: u64, // layer steps made, for unique labels
    #[cfg(test)]
    pub(crate) ledger: HashMap<u32, ObjectRow>, // object rows as the GPU would hold them
    #[cfg(test)]
    pub(crate) searches: usize, // tree walks the syncs needed
}

impl Default for Scene {
    /// Same as `new`.
    fn default() -> Self {
        Self::new()
    }
}

impl Scene {
    /// True when the row is not locked.
    pub fn selectable(&self, row: u32) -> bool {
        self.identity_of(row)
            .is_some_and(|id| !self.locked.contains(&id))
    }

    /// Empty: no documents, no rows.
    pub fn new() -> Self {
        Self {
            docs: Vec::new(),
            texts: Vec::new(),
            tables: Upload::default(),
            streamed: Vec::new(),
            sheets: Vec::new(),
            hidden: HashSet::new(),
            locked: HashSet::new(),
            colors: HashMap::new(),
            edge_colors: HashMap::new(),
            selected: None,
            attributes: true,
            order: Vec::new(),
            owners: Vec::new(),
            feet: Vec::new(),
            nodes: Vec::new(),
            spans: Spans::default(),
            caps: HashMap::new(),
            graves: HashMap::new(),
            ids: Ids::default(),
            sink: None,
            empty: Rc::from(""),
            doc_state: Vec::new(),
            pending: Vec::new(),
            hints: Vec::new(),
            staged: Staged::default(),
            dead: Counts::default(),
            dead_points: 0,
            compactions: 0,
            loaded: false,
            preview: None,
            bounds_stale: false,
            edge_sources: Vec::new(),
            guid_to_row: HashMap::new(),
            object_rows: 0,
            uploaded: Counts::default(),
            undo_steps: Vec::new(),
            redo_steps: Vec::new(),
            created_doc: None,
            row_revision: 0,
            current_layer: None,
            layer_trees: HashMap::new(),
            layer_steps: 0,
            #[cfg(test)]
            ledger: HashMap::new(),
            #[cfg(test)]
            searches: 0,
        }
    }

    /// Drop every document and its GPU rows.
    pub fn clear(&mut self, gpu: &mut Gpu) {
        self.created_doc = None;
        self.undo_steps.clear();
        self.redo_steps.clear();
        self.current_layer = None;
        self.layer_trees.clear();
        self.docs.clear();
        self.doc_state.clear();
        self.texts.clear();
        self.hidden.clear();
        self.locked.clear();
        self.colors.clear();
        self.edge_colors.clear();
        self.reset_rows();
        gpu.release();
    }

    /// Forget every row.
    fn reset_rows(&mut self) {
        self.row_revision = self.row_revision.wrapping_add(1);
        self.tables = Upload::default();
        self.streamed.clear();
        self.sheets.clear();
        self.order.clear();
        self.owners.clear();
        self.feet.clear();
        self.nodes.clear();
        self.spans.clear();
        self.caps.clear();
        self.graves.clear();
        self.ids.clear();
        self.sink = None;
        self.pending.clear();
        self.hints.clear();
        self.staged = Staged::default();
        self.dead = Counts::default();
        self.dead_points = 0;
        self.loaded = false;
        self.preview = None;
        self.bounds_stale = false;
        self.edge_sources.clear();
        self.guid_to_row.clear();
        self.selected = None;
        self.object_rows = 0;
        self.uploaded = Counts::default();
        #[cfg(test)]
        self.ledger.clear();
    }

    /// Add a document and its viewer state.
    pub(crate) fn push_doc(&mut self, doc: FileDoc, state: DocState) {
        self.docs.push(doc);
        self.doc_state.push(state);
    }

    /// Write what the last sync staged, then append the walked tables and clear them.
    pub fn upload_to(&mut self, gpu: &mut Gpu) {
        let staged = std::mem::take(&mut self.staged);
        let sink = self.sink.unwrap_or(u32::MAX);

        if !staged.retire.is_empty() {
            gpu.objects.retire_many(&gpu.ctx, &staged.retire);

            for &row in &staged.retire {
                gpu.set_selected(row, false);
            }
        }

        for &row in &staged.clouds {
            self.dead_points += gpu.cloud.kill_instance(row);
            gpu.splat.invalidate();
        }

        for (lane, first, count) in runs(staged.kills) {
            gpu.kill_rows(lane, first, count, sink);

            if lane == LaneId::Pipes {
                self.forget_edges(first, count);
            }
        }

        for (lane, first, count, vertex) in staged.tails {
            gpu.degenerate_rows(lane, first, count, vertex);
        }

        for (at, up) in &staged.patches {
            gpu.write_rows(*at, up);
            self.write_edges(at.pipes, up);
        }

        if !self.tables_empty() {
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

            // clouds coming back would not fit beside the dead points: pack the live ones first
            let incoming = self.tables.cloud.point_count();

            if incoming > 0 && self.dead_points > 0 && !gpu.cloud.fits(&gpu.ctx, incoming) {
                self.compact_clouds(gpu);
            }

            gpu.set_scene(&self.tables);

            // a loaded document re-centres the origin at the camera, as it always did; an edit keeps it
            if std::mem::take(&mut self.loaded) {
                gpu.objects.forget_anchor();
            }

            self.object_rows += self.tables.obj.rows.len() as u32;
            self.uploaded = self.uploaded.plus(Counts::of(&self.tables));
            self.tables.drop_uploaded();
        }

        for (row, object) in &staged.rows {
            gpu.objects.set_row(&gpu.ctx, *row, object);
            gpu.grew_bounds(*row);
        }

        for (row, object) in &staged.geometry {
            gpu.objects.update_geometry(&gpu.ctx, *row, object);
            gpu.grew_bounds(*row);
        }

        for (row, place) in &staged.places {
            if gpu.objects.placement_matches(*row, place) {
                continue;
            }

            gpu.objects.set_placement(&gpu.ctx, *row, place);
            gpu.grew_bounds(*row);
            self.bounds_stale = true;
        }

        gpu.set_dead(self.dead, self.dead_points);
        gpu.refresh_samples();
    }

    /// True when nothing waits to be appended.
    fn tables_empty(&self) -> bool {
        let t = &self.tables;
        t.obj.rows.is_empty()
            && Counts::of(t).is_empty()
            && t.cloud.pos.is_empty()
            && t.cloud.draws.is_empty()
            && t.seg.sheet_rows.is_empty()
            && t.seg.sheets.is_empty()
    }

    /// Pipes no longer carry a source edge.
    fn forget_edges(&mut self, first: u32, count: u32) {
        let end = (first + count) as usize;

        for edge in self.edge_sources.iter_mut().take(end).skip(first as usize) {
            *edge = (u32::MAX, u32::MAX);
        }
    }

    /// Pipes rewritten from `first` take the source edges of `up`.
    fn write_edges(&mut self, first: u32, up: &Upload) {
        for (index, pipe) in up.seg.pipes.iter().enumerate() {
            let edge = up.seg.pipe_ids.get(index).copied().unwrap_or(u32::MAX);

            if let Some(slot) = self.edge_sources.get_mut(first as usize + index) {
                *slot = (pipe.instance_id, edge);
            }
        }
    }

    /// An object row with the colors its identity was given.
    fn object_row(&self, owner: usize, guid: &Rc<str>, place: Xform, flags: u32) -> ObjectRow {
        let mut object = ObjectRow::new(place, flags);
        let key = (owner, Rc::clone(guid));

        if let Some(color) = self.colors.get(&key) {
            object.color = [
                color[0] as f32 / 255.,
                color[1] as f32 / 255.,
                color[2] as f32 / 255.,
                1.,
            ];
            object.flags |= Instance::FLAG_COLOR;
        }

        if let Some(color) = self.edge_colors.get(&key) {
            object.edge_color = u32::from_le_bytes([color[0], color[1], color[2], 255]);
            object.flags |= Instance::FLAG_EDGE_COLOR;
        }

        object
    }

    /// Add one object row for `guid` of document `owner`, after every other row.
    pub(super) fn push_row(&mut self, owner: usize, guid: &str, place: Xform, flags: u32) -> u32 {
        self.row_revision = self.row_revision.wrapping_add(1);
        self.loaded = true;
        let row = self.object_rows + self.tables.obj.rows.len() as u32;
        let guid: Rc<str> = Rc::from(guid);
        let object = self.object_row(owner, &guid, place, flags);
        self.tables.obj.rows.push(object);
        self.guid_to_row.insert((owner, Rc::clone(&guid)), row);
        self.order.push(guid);
        self.owners.push(owner);
        self.feet.push(Footprint::None);
        self.nodes.push(Weak::new());
        row
    }

    /// Add one document: one row per object, then the file sweeps.
    pub fn add_file(&mut self, doc: FileDoc) {
        self.row_revision = self.row_revision.wrapping_add(1);
        let FileDoc {
            name,
            session,
            place,
            point_px,
            display_only,
        } = doc;
        let index = self.docs.len();
        let from = Baselines::capture(&self.tables);
        let world = session.world_xforms();
        let mut lap = Lap::start("walk");
        let count = session.lookup.len();
        self.tables.obj.rows.reserve(count);
        self.order.reserve(count);
        self.guid_to_row.reserve(count);

        let baked = baked_attributes(&session);
        let order = session.order();
        let mut placed: HashMap<&str, u32> = HashMap::with_capacity(count); // guid to row, for the node cache

        for guid in &order {
            let Some(geom) = session.lookup.get(guid) else {
                continue;
            };

            if !is_drawable(geom) || baked.contains(guid.as_str()) {
                continue;
            }

            let flags = if self.hidden.contains(&(index, Rc::from(guid.as_str()))) {
                Instance::FLAG_HIDDEN
            } else {
                0
            };
            let object_place = placement(&world, &place, guid);
            let row = self.push_row(index, guid, object_place, flags);
            placed.insert(guid, row);
            let cx = WalkCx {
                vert_base: self.uploaded.verts,
                cloud_px: point_px,
                row,
                attributes: self.attributes,
            };
            let start = self.uploaded.plus(Counts::of(&self.tables));
            let r = walk_geometry(&mut Walk::of(&mut self.tables), &cx, geom);
            let end = self.uploaded.plus(Counts::of(&self.tables));
            self.feet[row as usize] = if matches!(geom, Geometry::PointCloud(_)) {
                Footprint::Cloud
            } else {
                self.spans.foot(Span {
                    start,
                    count: end.minus(start),
                })
            };
            let o = self.tables.obj.rows.last_mut().unwrap();
            o.flags |= r.flags;
            o.bounds = r.bounds;
            o.spacing = r.spacing;
            o.faces = r.faces;

            if r.faces {
                o.flags |= Instance::FLAG_HAS_FACES;
            }
        }

        lap.mark("objects");

        // each row remembers the tree node it was placed from
        for node in session.tree.nodes() {
            if let Some(&row) = placed.get(node.borrow().name.as_str()) {
                self.nodes[row as usize] = Rc::downgrade(&node);
            }
        }

        drop(placed);
        let extent = file_extent(&self.tables, &from);
        self.tables.bounds.union_with(&extent);

        // a flat file is a drawing sheet, unless it was drawn here
        let sheet = if self.created_doc != Some(index) {
            planar_band(&self.tables, &from, &place)
        } else {
            None
        };

        if sheet.is_some() {
            mark_sheet(&mut self.tables, &from);
        }

        lap.mark("sweeps");

        if display_only || knobs::drop_sessions() {
            log::info!(
                "'{name}': retaining source geometry for controls; the legacy display_only/drop_sessions hint no longer releases it"
            );
        }

        let nodes_from = sync::tree_key(&session);
        self.push_doc(
            FileDoc {
                name,
                place,
                session,
                point_px,
                display_only: false,
            },
            DocState { sheet, nodes_from },
        );
    }

    /// Add a streamed cloud from its first slice; returns its slot.
    pub fn add_streamed_cloud(&mut self, init: StreamedInit, gpu: &mut Gpu) -> usize {
        let slot = self.stream_cloud(init);
        self.upload_to(gpu);
        slot
    }

    /// The rows of a streamed cloud's first slice and its read-only shell document.
    pub(crate) fn stream_cloud(&mut self, init: StreamedInit) -> usize {
        let StreamedInit {
            name,
            url,
            place,
            rows,
            lod,
            fields,
            resident,
            point_px,
            col_at: _,
        } = init;
        let total = fields.count;
        let row = self.push_row(self.docs.len(), &format!("stream:{url}"), place.clone(), 0);
        let slice = StreamSlice {
            rows,
            lod: &lod,
            from: 0,
            to: resident,
            row,
            point_px,
        };
        let bounds = walk_stream_slice(&mut self.tables.cloud, &slice);
        let o = self.tables.obj.rows.last_mut().unwrap();
        o.bounds = bounds;
        o.spacing = point_px;
        self.tables.bounds.union_with(&bounds.transformed(&place));
        let model = place.clone();
        self.push_doc(
            FileDoc {
                name: name.clone(),
                place,
                session: Rc::new(Session::new(&name)),
                point_px,
                display_only: true,
            },
            DocState::default(),
        );
        self.streamed.push(StreamedCloud {
            name,
            url,
            row,
            lod,
            fields,
            place: model,
            done_to: resident,
            total,
            point_px,
        });
        self.streamed.len() - 1
    }

    /// Add the next slice of streamed cloud `idx`.
    pub fn extend_streamed_cloud(&mut self, idx: usize, rows: StreamRows, to: u32, gpu: &mut Gpu) {
        let Some(sc) = self.streamed.get(idx) else {
            return;
        };

        if to <= sc.done_to {
            return;
        }

        let place = match self.document(sc.row) {
            Some(document) => document.place.clone(),
            None => Xform::identity(),
        };
        let row = sc.row;
        let slice = StreamSlice {
            rows,
            lod: &sc.lod,
            from: sc.done_to,
            to,
            row,
            point_px: sc.point_px,
        };
        let bounds = walk_stream_slice(&mut self.tables.cloud, &slice);
        self.tables.bounds.union_with(&bounds.transformed(&place));
        self.streamed[idx].done_to = to;
        self.upload_to(gpu);
        gpu.objects
            .grow_local_bounds(&gpu.ctx, row, &bounds, &place);
    }

    /// Add a streamed sheet from its first slice; returns its slot.
    pub fn add_sheet(&mut self, init: SheetInit, gpu: &mut Gpu) -> usize {
        let slot = self.stream_sheet(init);
        self.upload_to(gpu);
        slot
    }

    /// The rows of a streamed sheet's first slice and its read-only shell document.
    pub(crate) fn stream_sheet(&mut self, init: SheetInit) -> usize {
        let SheetInit {
            name,
            url,
            meta_url,
            place,
            rows,
            fields,
            resident,
        } = init;
        let total = fields.count;
        let row = self.push_row(
            self.docs.len(),
            &format!("sheet:{url}"),
            place.clone(),
            Instance::FLAG_SHEET,
        );
        let slice = SheetSlice { rows, from: 0, row };
        let bounds = walk_sheet_slice(&mut self.tables.seg, &slice);
        let o = self.tables.obj.rows.last_mut().unwrap();
        o.bounds = bounds;
        self.tables.bounds.union_with(&bounds.transformed(&place));
        let model = place.clone();
        self.push_doc(
            FileDoc {
                name: name.clone(),
                place,
                session: Rc::new(Session::new(&name)),
                point_px: 0.0,
                display_only: true,
            },
            DocState::default(),
        );
        self.sheets.push(SheetBatch {
            name,
            url,
            meta_url,
            row,
            fields,
            place: model,
            done_to: resident,
            total,
            resolved: None,
            table: None,
        });
        self.sheets.len() - 1
    }

    /// Add the next slice of sheet `idx`.
    pub fn extend_sheet(&mut self, idx: usize, rows: SheetRows, to: u32, gpu: &mut Gpu) {
        let Some(sheet) = self.sheets.get(idx) else {
            return;
        };

        if to <= sheet.done_to {
            return;
        }

        let place = sheet.place.clone();
        let row = sheet.row;
        let slice = SheetSlice {
            rows,
            from: sheet.done_to,
            row,
        };
        let bounds = walk_sheet_slice(&mut self.tables.seg, &slice);
        self.tables.bounds.union_with(&bounds.transformed(&place));
        self.sheets[idx].done_to = to;
        self.upload_to(gpu);
        gpu.objects
            .grow_local_bounds(&gpu.ctx, row, &bounds, &place);
    }

    /// The sheet slot on object row `row`, if that row is a sheet.
    pub fn sheet_slot(&self, row: u32) -> Option<usize> {
        for (slot, sheet) in self.sheets.iter().enumerate() {
            if sheet.row == row {
                return Some(slot);
            }
        }

        None
    }

    /// The sheet on object row `row`.
    pub fn sheet_at(&self, row: u32) -> Option<&SheetBatch> {
        self.sheets.get(self.sheet_slot(row)?)
    }

    /// What a GPU pick landed on.
    pub fn resolve(&self, pick: Pick, gpu: &Gpu) -> Option<Picked> {
        let (_, guid) = self.identity_of(pick.row)?;
        let mut point = None;

        if let Some((parent, local)) = gpu.cloud.row_of(pick.sub)
            && parent == pick.row
        {
            point = self.point_at(pick.row, local);
        }

        let mut entity = None;
        // bit 31 set: the sub id is a ribbon row
        let ribbon = pick.sub & 0x7fff_ffff;

        if pick.sub & 0x8000_0000 != 0
            && self.sheet_at(pick.row).is_some()
            && let Some((parent, _)) = gpu.segments.row_of(ribbon)
            && parent == pick.row
        {
            entity = gpu.segments.source_id(ribbon).filter(|id| *id != u32::MAX);
        }

        let doc = match self.document(pick.row) {
            Some(document) => document.name.clone(),
            None => String::new(),
        };
        Some(Picked {
            doc,
            guid: guid.to_string(),
            row: pick.row,
            point,
            entity,
        })
    }

    /// True for the row of a streamed sheet or cloud, which is never edited.
    pub fn display_only(&self, row: u32) -> bool {
        self.document(row).is_some_and(|file| file.display_only)
    }

    /// The document a row belongs to.
    pub fn document(&self, row: u32) -> Option<&FileDoc> {
        self.docs.get(*self.owners.get(row as usize)?)
    }

    /// The kernel geometry of a row.
    pub fn geometry(&self, row: u32) -> Option<&Geometry> {
        self.document(row)?
            .session
            .lookup
            .get(self.order.get(row as usize)?.as_ref())
    }

    /// The row's name, or its type when unnamed.
    pub fn object_name(&self, row: u32) -> &str {
        if let Some(text) = self.text_at(row) {
            return &text.label.text;
        }

        if let Some(sheet) = self.sheet_at(row) {
            return match &sheet.resolved {
                Some((_, meta)) if !meta.name.trim().is_empty() => &meta.name,
                Some((_, meta)) if !meta.kind.trim().is_empty() => &meta.kind,
                _ => &sheet.name,
            };
        }

        let (name, kind) = match self.geometry(row) {
            Some(Geometry::OBB(value)) => (value.name.as_str(), "Box"),
            Some(Geometry::BRep(value)) => (value.name.as_str(), "BRep"),
            Some(Geometry::Element(value)) => (value.name.as_str(), "Element"),
            Some(Geometry::Line(value)) => (value.name.as_str(), "Line"),
            Some(Geometry::Mesh(value)) => (value.name.as_str(), "Mesh"),
            Some(Geometry::NurbsCurve(value)) => (value.name.as_str(), "NURBS curve"),
            Some(Geometry::NurbsSurface(value)) => (value.name.as_str(), "NURBS surface"),
            Some(Geometry::Plane(value)) => (value.name.as_str(), "Plane"),
            Some(Geometry::Point(value)) => (value.name.as_str(), "Point"),
            Some(Geometry::PointCloud(value)) => (value.name.as_str(), "Point cloud"),
            Some(Geometry::Polyline(value)) => (value.name.as_str(), "Polyline"),
            None => ("", "Object"),
        };

        if name.trim().is_empty() { kind } else { name }
    }

    /// The edge index a pipe pick landed on.
    pub fn edge_at(&self, pick: Pick) -> Option<u32> {
        if pick.sub & 0x8000_0000 == 0 {
            return None;
        }

        let &(parent, edge) = self.edge_sources.get((pick.sub & 0x7fff_ffff) as usize)?;
        (parent == pick.row && edge != u32::MAX).then_some(edge)
    }

    /// Point `local` of the cloud on `row`; None when streamed.
    pub fn point_at(&self, row: u32, local: u32) -> Option<PickedPoint> {
        let Some(Geometry::PointCloud(pc)) = self.geometry(row) else {
            return None;
        };
        let c = pc.coords();
        let i = local as usize * 3;

        if i + 2 >= c.len() {
            return None;
        }

        Some(PickedPoint {
            local,
            id: pc.point_id(local as usize),
            position: [c[i], c[i + 1], c[i + 2]],
        })
    }

    /// The ribbon rows of one object.
    pub fn ribbon_range(&self, row: u32) -> Option<std::ops::Range<u32>> {
        let span = self.spans.span(*self.feet.get(row as usize)?);
        let start = span.start.ribbons;
        (span.count.ribbons > 0).then_some(start..start + span.count.ribbons)
    }

    /// The solid face indices of one object.
    pub fn face_range(&self, row: u32) -> Option<std::ops::Range<u32>> {
        let span = self.spans.span(*self.feet.get(row as usize)?);
        let start = span.start.faces;
        (span.count.faces > 0).then_some(start..start + span.count.faces)
    }

    /// The row of one object, by document and guid.
    pub fn row_of(&self, doc: usize, guid: &str) -> Option<u32> {
        self.guid_to_row.get(&(doc, Rc::from(guid))).copied()
    }

    /// The document and guid of a row; None for a free id and the sink.
    pub fn identity_of(&self, row: u32) -> Option<(usize, Rc<str>)> {
        let owner = *self.owners.get(row as usize)?;

        if owner == FREE || owner == SINK {
            return None;
        }

        Some((owner, Rc::clone(self.order.get(row as usize)?)))
    }

    /// The rows currently hidden.
    pub fn hidden_rows(&self) -> Vec<u32> {
        let mut rows = Vec::new();

        for identity in &self.hidden {
            if let Some(&row) = self.guid_to_row.get(identity) {
                rows.push(row);
            }
        }

        rows
    }

    /// Row ids handed out so far, live or not; loops over rows run to here.
    pub fn row_count(&self) -> usize {
        self.order.len()
    }

    /// Rows holding an object, a text or a streamed shell.
    pub fn object_count(&self) -> usize {
        self.order.len() - self.ids.len() - usize::from(self.sink.is_some())
    }
}

/// File placement times the object's own transform.
fn placement(world: &HashMap<String, Xform>, place: &Xform, guid: &str) -> Xform {
    match world.get(guid) {
        Some(local) => place * local,
        None => place.clone(),
    }
}

/// Guids under an `attributes` group; they get no row. The root is named after the session, never a group.
fn baked_attributes(session: &Session) -> HashSet<String> {
    let mut out = HashSet::new();
    let mut stack: Vec<_> = session
        .tree
        .root()
        .into_iter()
        .flat_map(|root| root.borrow().children())
        .map(|n| (n, false))
        .collect();

    while let Some((node, inside)) = stack.pop() {
        let node = node.borrow();
        let inside = inside || node.name == "attributes";

        if inside && session.lookup.contains_key(&node.name) {
            out.insert(node.name.clone());
        }

        stack.extend(node.children().into_iter().map(|c| (c, inside)));
    }

    out
}

/// Kills sorted and joined into contiguous runs per lane.
fn runs(mut kills: Vec<(LaneId, u32, u32)>) -> Vec<(LaneId, u32, u32)> {
    kills.retain(|kill| kill.2 > 0);
    kills.sort_unstable();
    let mut out: Vec<(LaneId, u32, u32)> = Vec::with_capacity(kills.len());

    for (lane, first, count) in kills {
        match out.last_mut() {
            Some(last) if last.0 == lane && last.1 + last.2 >= first => {
                last.2 = last.2.max(first + count - last.1);
            }
            _ => out.push((lane, first, count)),
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::selection::Controls;
    use session_rust::{BRep, Point};

    /// A document at the origin.
    fn file(name: &str, session: Rc<Session>, display_only: bool) -> FileDoc {
        FileDoc {
            name: name.into(),
            session,
            place: Xform::identity(),
            point_px: 0.0,
            display_only,
        }
    }

    /// The same guid in two documents stays two objects.
    #[test]
    fn duplicate_guids_keep_their_document_and_control_owners() {
        let first = Point::new(10.0, 0.0, 0.0);
        let guid = first.guid().to_string();
        let mut second = first.clone();
        second[0] = 20.0;
        assert_eq!(second.guid(), guid);
        let mut left = Session::new("left");
        left.add_point(first, None);
        let mut right = Session::new("right");
        right.add_point(second, None);
        let mut scene = Scene::new();
        scene.add_file(file("left", Rc::new(left), false));
        scene.add_file(file("right", Rc::new(right), false));
        assert_eq!(scene.object_count(), 2);
        assert_eq!(scene.document(0).unwrap().name, "left");
        assert_eq!(scene.document(1).unwrap().name, "right");
        let a = Controls::from_geometry(scene.geometry(0).unwrap());
        let b = Controls::from_geometry(scene.geometry(1).unwrap());
        assert_eq!(a.points[0].position, [10.0, 0.0, 0.0]);
        assert_eq!(b.points[0].position, [20.0, 0.0, 0.0]);
        scene.hidden.insert(scene.identity_of(1).unwrap());
        assert_eq!(scene.hidden_rows(), vec![1]);
        assert!(scene.document(2).is_none());
    }

    /// Two placements share one session until one is edited.
    #[test]
    fn an_edit_must_split_a_session_two_placements_share() {
        let mut source = Session::new("twice");
        source.add_point(Point::new(1.0, 0.0, 0.0), None);
        let shared = Rc::new(source);
        let mut scene = Scene::new();
        scene.add_file(file("first", Rc::clone(&shared), false));
        scene.add_file(file("second", Rc::clone(&shared), false));
        assert!(Rc::ptr_eq(&scene.docs[0].session, &scene.docs[1].session));

        Rc::make_mut(&mut scene.docs[0].session).add_point(Point::new(2.0, 0.0, 0.0), None);

        assert!(!Rc::ptr_eq(&scene.docs[0].session, &scene.docs[1].session));
        assert_eq!(scene.docs[0].session.lookup.len(), 2);
        assert_eq!(scene.docs[1].session.lookup.len(), 1);
    }

    /// The old `display_only` flag keeps the controls.
    #[test]
    fn legacy_display_only_hint_retains_source_controls() {
        let mut source = Session::new("retained CAD source");
        source.add_brep(BRep::create_box(2.0, 3.0, 4.0), None);
        let source = Rc::new(source);
        let mut scene = Scene::new();
        scene.add_file(file("display hint", Rc::clone(&source), true));
        assert!(Rc::ptr_eq(&source, &scene.docs[0].session));
        assert!(!scene.docs[0].display_only);
        assert_eq!(scene.object_count(), 1);
        let controls = Controls::from_geometry(scene.geometry(0).unwrap());
        assert!(controls.points.len() >= 8);
        assert!(!scene.tables.arena.idx.is_empty());
    }

    /// Element features draw in the element's row and move with it.
    #[test]
    fn attributes_share_the_element_row_and_its_placement() {
        use session_rust::element::ElementFeature;
        use session_rust::{Element, Mesh, Polyline};

        let mut element = Element::new("beam");
        element.set_geometry(Mesh::create_box(10.0, 10.0, 10.0));
        let axis = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(100.0, 0.0, 0.0)]);
        element.add_feature(ElementFeature::new("axis", -1, vec![axis], "axis"));
        let mut source = Session::new("attributes");
        source.add_element(element, None);
        let mut scene = Scene::new();
        scene.attributes = false;
        scene.add_file(file("beam", Rc::new(source), false));
        let plain = scene.tables.seg.ribbons.len();
        assert_eq!(scene.object_count(), 1);

        scene.attributes = true;
        scene.rewalk_cpu();
        assert_eq!(scene.object_count(), 1);
        let range = scene.ribbon_range(0).unwrap();
        assert_eq!(range.len(), plain + 1);

        let moved = scene.transform_rows(&[0], &Xform::translation(5.0, 0.0, 0.0), "Move");
        assert_eq!(moved.map(|m| m.len()), Some(1));
        assert_eq!(
            scene
                .placement_of(0)
                .unwrap()
                .transform_point(&Point::new(100.0, 0.0, 0.0))[0],
            105.0
        );
    }

    /// Baked attribute copies never get a row.
    #[test]
    fn baked_attributes_never_get_a_row() {
        use session_rust::{Element, Mesh, Polyline};

        let mut element = Element::new("beam");
        element.set_geometry(Mesh::create_box(10.0, 10.0, 10.0));
        let mut source = Session::new("attributes");
        source.add_element(element, None);
        let group = source.add_group("attributes");
        let axis = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(100.0, 0.0, 0.0)]);
        source.add_polyline(axis, Some(&group));
        assert_eq!(source.lookup.len(), 2);
        let mut scene = Scene::new();
        scene.add_file(file("beam", Rc::new(source), false));
        assert_eq!(scene.object_count(), 1);

        scene.attributes = true;
        scene.rewalk_cpu();
        assert_eq!(scene.object_count(), 1);
    }

    /// Kills join into one run per contiguous stretch of a lane.
    #[test]
    fn kills_merge_into_runs() {
        let kills = vec![
            (LaneId::Ribbons, 10, 2),
            (LaneId::Verts, 0, 4),
            (LaneId::Ribbons, 12, 3),
            (LaneId::Ribbons, 20, 1),
            (LaneId::Verts, 4, 0),
        ];
        assert_eq!(
            runs(kills),
            vec![
                (LaneId::Verts, 0, 4),
                (LaneId::Ribbons, 10, 5),
                (LaneId::Ribbons, 20, 1)
            ]
        );
    }
}
