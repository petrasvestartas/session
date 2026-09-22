use crate::app::walk::brep::{walk_brep, walk_surface};
use crate::app::walk::mesh_ink::Ink;
use crate::app::walk::{Row, WalkCx};
use crate::engine::gpu::Upload;
use crate::engine::gpu::objects::ObjectRow;
// --8<-- [start:step-2a]
use session_rust::{BRep, Color, Geometry, NurbsSurface, Point, Xform};
// --8<-- [end:step-2a]
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

// --8<-- [start:step-2b]
/// A curved patch with a hole, mesh cached.
fn trimmed_surface() -> NurbsSurface {
    use session_rust::{NurbsSurfaceTrimmed, TrimLoops};
    let points = [
        Point::new(-200.0, -150.0, 0.0),
        Point::new(-200.0, 150.0, 0.0),
        Point::new(0.0, -150.0, 160.0),
        Point::new(0.0, 150.0, 160.0),
        Point::new(200.0, -150.0, 0.0),
        Point::new(200.0, 150.0, 0.0),
    ];
    let surface = NurbsSurface::create(false, false, 2, 1, 3, 2, &points).unwrap();
    let mut trimmed = NurbsSurfaceTrimmed::new();
    trimmed.m_surface = surface;
    let mut loops = TrimLoops::default();
    let mut outer = Vec::new();

    for edge in 0..4 {
        for sample in 0..24 {
            let t = sample as f64 / 24.0;
            let (u, v) = match edge {
                0 => (t, 0.0),
                1 => (1.0, t),
                2 => (1.0 - t, 1.0),
                _ => (0.0, 1.0 - t),
            };
            outer.push(Point::new(u, v, 0.0));
        }
    }

    let mut hole = Vec::new();

    for sample in 0..48 {
        let angle = sample as f64 / 48.0 * std::f64::consts::TAU;
        hole.push(Point::new(
            0.5 + 0.2 * angle.cos(),
            0.5 + 0.2 * angle.sin(),
            0.0,
        ));
    }

    loops.uv = vec![outer, hole];

    for ring in &loops.uv {
        let mut positions = Vec::with_capacity(ring.len());

        for uv in ring {
            positions.push(
                trimmed
                    .m_surface
                    .point_at(uv[0], uv[1])
                    .expect("fixture UV lies in the source domain"),
            );
        }

        loops.xyz.push(positions);
    }

    let mesh = trimmed.mesh_loops(&loops, 20.0, 0.005);
    assert!(
        !mesh.face.is_empty(),
        "the local constrained hole must triangulate"
    );
    let mut surface = trimmed.m_surface;
    surface.m_mesh = Some(mesh);
    surface.facecolors = vec![Color::grey()];
    surface.name = "cached curved trim with circular hole".into();
    surface
}

/// A trimmed patch and a torus.
pub fn build() -> CadFixture {
    let mut scene = CadFixture::new();
    scene.add(
        Geometry::NurbsSurface(Rc::new(trimmed_surface())),
        Xform::translation(-290.0, 0.0, 0.0),
    );
    let mut torus = BRep::create_torus(120.0, 35.0);
    torus.surfacecolor = Color::grey();
    scene.add(
        Geometry::BRep(Rc::new(torus)),
        Xform::translation(290.0, 0.0, 0.0),
        // --8<-- [end:step-2b]
    );
    scene
}
