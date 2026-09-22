// --8<-- [start:step-13]
use crate::app::walk::brep::{walk_brep, walk_surface};
use crate::app::walk::mesh_ink::Ink;
use crate::app::walk::{Row, WalkCx};
use crate::engine::gpu::Upload;
use crate::engine::gpu::objects::ObjectRow;
use session_rust::{Color, Geometry, NurbsSurface, Point, Xform};
use std::rc::Rc;

/// The f64 source objects, kept beside their rows.
pub struct CadFixture {
    pub upload: Upload, // the lane tables for the GPU
    pub sources: Vec<Geometry>, // the f64 source objects
    pub identities: Vec<SourceIdentity>, // row and GUID per object
    pub pipe_source_edges: Vec<u32>, // edges drawn as pipes
}

/// Row = GPU address; GUID = the source object.
#[derive(serde::Serialize)]
pub struct SourceIdentity {
    pub object_row: u32, // the GPU row
    pub guid: String, // the source id
    pub kind: &'static str, // mesh, brep or surface
}

impl CadFixture {
    /// Empty until the chosen test scene adds its objects.
    fn new() -> Self {
        Self {
            upload: Upload::default(),
            sources: Vec::new(),
            identities: Vec::new(),
            pipe_source_edges: Vec::new(),
        }
    }

    /// Prepare one source object and keep it.
    fn add(&mut self, geometry: Geometry, place: Xform) {
        let row = self.upload.obj.rows.len() as u32;
        let first_pipe = self.upload.seg.pipes.len();
        let context = WalkCx {
            vert_base: 0,
            cloud_px: 0.0,
            row,
        };
        let mut ink = Ink {
            seg: &mut self.upload.seg,
            glyph: &mut self.upload.glyph,
        };
        let (prepared, guid, kind) = match &geometry {
            Geometry::BRep(source) => (
                walk_brep(&mut self.upload.arena, &mut ink, source, &context),
                source.guid().to_string(),
                "BRep",
            ),
            Geometry::NurbsSurface(source) => (
                walk_surface(&mut self.upload.arena, &mut ink, source, &context),
                source.guid().to_string(),
                "NurbsSurface",
            ),
            _ => unreachable!("the CAD fixture only constructs BRep and surface source objects"),
        };
        self.pipe_source_edges
            .extend_from_slice(&self.upload.seg.pipe_ids[first_pipe..]);
        self.push_row(prepared, place);
        self.identities.push(SourceIdentity {
            object_row: row,
            guid,
            kind,
        });
        self.sources.push(geometry);
    }

    /// Local bounds and placement stay separate.
    fn push_row(&mut self, prepared: Row, place: Xform) {
        let mut object = ObjectRow::new(place.clone(), prepared.flags);
        object.bounds = prepared.bounds;
        object.spacing = prepared.spacing;
        object.faces = prepared.faces;
        self.upload
            .bounds
            .union_with(&prepared.bounds.transformed(&place));
        self.upload.obj.rows.push(object);
    }
}

/// A flat surface with its four UV borders.
fn planar_surface() -> NurbsSurface {
    let points = [
        Point::new(-200.0, -150.0, 0.0),
        Point::new(-200.0, 150.0, 0.0),
        Point::new(200.0, -150.0, 0.0),
        Point::new(200.0, 150.0, 0.0),
    ];
    let mut surface = NurbsSurface::create(false, false, 1, 1, 2, 2, &points).unwrap();
    surface.facecolors = vec![Color::grey()];
    surface.name = "local source face".into();
    surface
}

/// Build the first source-face checkpoint entirely from local geometry.
pub fn build() -> CadFixture {
    let mut scene = CadFixture::new();
    scene.add(
        Geometry::NurbsSurface(Rc::new(planar_surface())),
        Xform::identity(),
    );
    scene
    // --8<-- [end:step-13]
}
