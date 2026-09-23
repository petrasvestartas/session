#[path = "scene_text.rs"]
mod text;
pub use text::SceneText;

use crate::app::knobs;
use crate::app::sheet_query::{EntityMeta, SheetTable};
use crate::app::stream::{CloudFields, CloudLod, SheetFields};
use crate::app::walk::bounds::{Baselines, file_extent, is_planar, mark_sheet};
use crate::app::walk::cloud::{StreamRows, StreamSlice, walk_stream_slice};
use crate::app::walk::mesh::Lap;
use crate::app::walk::sheet::{SheetRows, SheetSlice, walk_sheet_slice};
use crate::app::walk::{Walk, WalkCx, is_drawable, walk_geometry};
use crate::engine::gpu::{Gpu, Instance, ObjectRow, Pick, Upload};
use session_rust::{Geometry, Session, Xform};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

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

/// Rows already on the GPU, per table.
#[derive(Default)]
struct Bases {
    vert: u32,   // arena vertices
    ribbon: u32, // ribbon segments
    obj: u32,    // object rows
}

/// The open documents and their object rows.
pub struct Scene {
    pub docs: Vec<FileDoc>,                            // loaded files
    pub texts: Vec<SceneText>,                         // text objects
    pub tables: Upload,                                // rows walked but not yet uploaded
    pub streamed: Vec<StreamedCloud>,                  // streamed clouds
    pub sheets: Vec<SheetBatch>,                       // streamed sheets
    pub hidden: HashSet<(usize, Rc<str>)>,             // (document, guid) hidden
    pub selected: Option<u32>,                         // selected object row
    order: Vec<Rc<str>>,                               // guid of each row
    owners: Vec<usize>,                                // document of each row
    edge_sources: Vec<(u32, u32)>,                     // (object row, edge index) of each pipe
    ribbon_ranges: Vec<Option<std::ops::Range<u32>>>,  // ribbon rows of each object
    guid_to_row: HashMap<(usize, Rc<str>), u32>,       // (document, guid) to row
    bases: Bases,                                      // rows already on the GPU
    pub last_edited: Option<usize>,                    // document undo applies to
    pub(crate) created_doc: Option<usize>,
    pub(crate) row_revision: u64,                      // bumped when rows change
}

impl Default for Scene {
    /// Same as `new`.
    fn default() -> Self {
        Self::new()
    }
}

impl Scene {
    /// Empty: no documents, no rows.
    pub fn new() -> Self {
        Self {
            docs: Vec::new(),
            texts: Vec::new(),
            tables: Upload::default(),
            streamed: Vec::new(),
            sheets: Vec::new(),
            hidden: HashSet::new(),
            selected: None,
            order: Vec::new(),
            owners: Vec::new(),
            edge_sources: Vec::new(),
            ribbon_ranges: Vec::new(),
            guid_to_row: HashMap::new(),
            bases: Bases::default(),
            last_edited: None,
            created_doc: None,
            row_revision: 0,
        }
    }

    /// Drop every document and its GPU rows.
    pub fn clear(&mut self, gpu: &mut Gpu) {
        self.created_doc = None;
        self.last_edited = None;
        self.docs.clear();
        self.texts.clear();
        self.hidden.clear();
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
        self.edge_sources.clear();
        self.ribbon_ranges.clear();
        self.guid_to_row.clear();
        self.selected = None;
        self.bases = Bases::default();
    }

    /// Walk every document again and upload from scratch.
    pub fn rebuild(&mut self, gpu: &mut Gpu) {
        let docs = std::mem::take(&mut self.docs);
        let texts = std::mem::take(&mut self.texts);
        self.reset_rows();
        gpu.reset();

        for d in docs {
            if d.display_only {
                log::warn!(
                    "rebuild: '{}' is display_only, its geometry was released",
                    d.name
                );
            }

            self.add_file(d);
        }

        for text in texts {
            self.register_text(text.key, text.label, text.active);
        }

        self.upload_to(gpu);
        self.restore_text_visibility(gpu);
    }

    /// Upload the walked tables and clear them.
    pub fn upload_to(&mut self, gpu: &mut Gpu) {
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

        gpu.set_scene(&self.tables);
        self.bases.vert += self.tables.arena.verts.len() as u32;
        self.bases.obj += self.tables.obj.rows.len() as u32;
        self.bases.ribbon += self.tables.seg.ribbons.len() as u32;
        self.tables.drop_uploaded();
    }

    /// Add one object row for `guid` of document `owner`.
    pub(super) fn push_row(&mut self, owner: usize, guid: &str, place: Xform, flags: u32) -> u32 {
        self.row_revision = self.row_revision.wrapping_add(1);
        let row = self.bases.obj + self.tables.obj.rows.len() as u32;
        self.tables.obj.rows.push(ObjectRow::new(place, flags));
        let guid: Rc<str> = Rc::from(guid);
        self.guid_to_row.insert((owner, Rc::clone(&guid)), row);
        self.order.push(guid);
        self.owners.push(owner);
        self.ribbon_ranges.push(None);
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
        let from = Baselines::capture(&self.tables);
        let world = session.world_xforms();
        let mut lap = Lap::start("walk");
        let count = session.lookup.len();
        self.tables.obj.rows.reserve(count);
        self.order.reserve(count);
        self.guid_to_row.reserve(count);

        for guid in session.order() {
            let Some(geom) = session.lookup.get(&guid) else {
                continue;
            };

            if !is_drawable(geom) {
                continue;
            }

            let flags = if self
                .hidden
                .contains(&(self.docs.len(), Rc::from(guid.as_str())))
            {
                Instance::FLAG_HIDDEN
            } else {
                0
            };
            let object_place = placement(&world, &place, &guid);
            let row = self.push_row(self.docs.len(), &guid, object_place, flags);
            let ribbon_start = self.tables.seg.ribbons.len();
            let cx = WalkCx {
                vert_base: self.bases.vert,
                cloud_px: point_px,
                row,
            };
            let r = walk_geometry(&mut Walk::of(&mut self.tables), &cx, geom);
            let o = self.tables.obj.rows.last_mut().unwrap();
            o.flags |= r.flags;
            o.bounds = r.bounds;
            o.spacing = r.spacing;
            o.faces = r.faces;
            let ribbon_end = self.tables.seg.ribbons.len();

            if ribbon_start != ribbon_end {
                self.ribbon_ranges[row as usize] = Some(
                    self.bases.ribbon + ribbon_start as u32..self.bases.ribbon + ribbon_end as u32,
                );
            }
        }

        lap.mark("objects");

        let extent = file_extent(&self.tables, &from);
        self.tables.bounds.union_with(&extent);

        if is_planar(&self.tables, &from, &place) {
            mark_sheet(&mut self.tables, &from);
        }

        lap.mark("sweeps");

        if display_only || knobs::drop_sessions() {
            log::info!(
                "'{name}': retaining source geometry for controls; the legacy display_only/drop_sessions hint no longer releases it"
            );
        }

        self.docs.push(FileDoc {
            name,
            place,
            session,
            point_px,
            display_only: false,
        });
    }

    /// Add a streamed cloud from its first slice; returns its slot.
    pub fn add_streamed_cloud(&mut self, init: StreamedInit, gpu: &mut Gpu) -> usize {
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
        self.upload_to(gpu);

        let model = place.clone();
        self.docs.push(FileDoc {
            name: name.clone(),
            place,
            session: Rc::new(Session::new(&name)),
            point_px,
            display_only: true,
        });
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
        let slice = StreamSlice {
            rows,
            lod: &sc.lod,
            from: sc.done_to,
            to,
            row: sc.row,
            point_px: sc.point_px,
        };
        let bounds = walk_stream_slice(&mut self.tables.cloud, &slice);
        self.tables.bounds.union_with(&bounds.transformed(&place));
        self.streamed[idx].done_to = to;
        self.upload_to(gpu);
    }

    /// Add a streamed sheet from its first slice; returns its slot.
    pub fn add_sheet(&mut self, init: SheetInit, gpu: &mut Gpu) -> usize {
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
        self.upload_to(gpu);

        let model = place.clone();
        self.docs.push(FileDoc {
            name: name.clone(),
            place,
            session: Rc::new(Session::new(&name)),
            point_px: 0.0,
            display_only: true,
        });
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
        let slice = SheetSlice {
            rows,
            from: sheet.done_to,
            row: sheet.row,
        };
        let bounds = walk_sheet_slice(&mut self.tables.seg, &slice);
        self.tables.bounds.union_with(&bounds.transformed(&place));
        self.sheets[idx].done_to = to;
        self.upload_to(gpu);
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
        let guid = self.order.get(pick.row as usize)?.to_string();
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
            guid,
            row: pick.row,
            point,
            entity,
        })
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
        self.ribbon_ranges.get(row as usize).cloned().flatten()
    }

    /// The document and guid of a row.
    pub fn identity_of(&self, row: u32) -> Option<(usize, Rc<str>)> {
        Some((
            *self.owners.get(row as usize)?,
            Rc::clone(self.order.get(row as usize)?),
        ))
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

    /// Objects in row order.
    pub fn object_count(&self) -> usize {
        self.order.len()
    }
}

/// File placement times the object's own transform.
fn placement(world: &HashMap<String, Xform>, place: &Xform, guid: &str) -> Xform {
    match world.get(guid) {
        Some(local) => place * local,
        None => place.clone(),
    }
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
}
