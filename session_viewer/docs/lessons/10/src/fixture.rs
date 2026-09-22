use crate::app::walk::brep::{walk_brep, walk_surface};
use crate::app::walk::mesh_ink::Ink;
use crate::app::walk::{Row, WalkCx};
use crate::engine::gpu::Upload;
use crate::engine::gpu::objects::ObjectRow;
use session_rust::{BRep, Color, Geometry, NurbsSurface, Point, Xform};
use std::rc::Rc;

/// Keep the f64 source objects beside their GPU rows.
pub struct CadFixture {
    pub upload: Upload, // rows ready for the GPU
    pub sources: Vec<Geometry>, // the f64 geometry
    pub identities: Vec<SourceIdentity>, // one per object
    pub pipe_source_edges: Vec<u32>, // rows drawn as pipes
}

/// A row is a GPU slot; the GUID names the object.
#[derive(serde::Serialize)]
/// One object: its GUID, kind and GPU row.
pub struct SourceIdentity {
    pub object_row: u32, // the GPU row
    pub guid: String,
    pub kind: &'static str, // mesh, polyline, point…
}

impl CadFixture {
    /// Start empty; the chosen case adds objects.
    fn new() -> Self {
        Self {
            upload: Upload::default(), // empty
            sources: Vec::new(),
            identities: Vec::new(), // no objects yet
            pipe_source_edges: Vec::new(), // no pipes yet
        }
    }

    /// Prepare one object, keeping its id and geometry.
    fn add(&mut self, geometry: Geometry, place: Xform) {
        let row = self.upload.obj.rows.len() as u32;
        let first_pipe = self.upload.seg.pipes.len();
        let context = WalkCx {
            vert_base: 0,
            cloud_px: 0.0,
            row,
        };
        let mut ink = Ink {
            seg: &mut self.upload.seg, // line rows
            glyph: &mut self.upload.glyph, // marker rows
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
            object_row: row, // the GPU row
            guid,
            kind,
        });
        self.sources.push(geometry);
    }

    /// Local bounds, placed by the instance for fitting.
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

/// An instance mirrored and scaled unevenly.
fn affine_placement() -> Xform {
    let scale = Xform::from_matrix([
        -1.4, 0.0, 0.0, 0.0, 0.0, 0.65, 0.0, 0.0, 0.0, 0.0, 1.15, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]);
    Xform::rotation_z(23.0, true) * Xform::rotation_x(17.0, true) * scale
}

/// A folded surface with a crease.
fn crease_surface() -> NurbsSurface {
    let points = [
        Point::new(-200.0, -100.0, 0.0),
        Point::new(-200.0, 100.0, 0.0),
        Point::new(0.0, -100.0, 0.0),
        Point::new(0.0, 100.0, 0.0),
        Point::new(200.0, -100.0, 160.0),
        Point::new(200.0, 100.0, 160.0),
    ];
    let mut surface = NurbsSurface::create(false, false, 1, 1, 3, 2, &points).unwrap();
    surface.facecolors = vec![Color::grey()];
    surface.name = "CAD C0 crease".into();
    surface
}

/// Same topology as the kernel boundary tests.
fn solid(kind: &str) -> BRep {
    let mut brep = match kind {
        "cylinder" => BRep::create_cylinder(120.0, 240.0),
        "sphere" => BRep::create_sphere(160.0),
        "torus" => BRep::create_torus(120.0, 35.0),
        "hole" => BRep::create_block_with_hole(400.0, 300.0, 120.0, 70.0),
        _ => panic!("unknown CAD fixture"),
    };
    brep.name = format!("CAD {kind}");
    brep.surfacecolor = Color::grey();
    brep
}

/// A curved patch with a hole, mesh precomputed.
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

/// One diagnostic scene chosen by URL.
fn query(name: &str) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        let search = web_sys::window()?.location().search().ok()?;
        let prefix = format!("{name}=");

        for part in search.trim_start_matches('?').split('&') {
            if let Some(value) = part.strip_prefix(&prefix) {
                return Some(value.to_string());
            }
        }

        None
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = name;
        None
    }
}

/// `?cad=` picks one local CAD fixture.
pub fn build() -> CadFixture {
    let mut scene = CadFixture::new();
    let kind = query("cad").unwrap_or("sphere".into());
    let geometry = match kind.as_str() {
        "crease" => Geometry::NurbsSurface(Rc::new(crease_surface())),
        "trimmed" => Geometry::NurbsSurface(Rc::new(trimmed_surface())),
        "torus" => Geometry::BRep(Rc::new(solid("torus"))),
        "cylinder" | "sphere" | "hole" => Geometry::BRep(Rc::new(solid(&kind))),
        _ => Geometry::BRep(Rc::new(solid("sphere"))),
    };
    let place = if query("affine").as_deref() == Some("1") {
        affine_placement()
    } else {
        Xform::identity()
    };
    scene.add(geometry, place);
    scene
}
