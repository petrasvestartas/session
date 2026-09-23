use crate::engine::gpu::Upload;
use crate::engine::gpu::arena::ArenaRows;
use crate::engine::gpu::cloud::CloudRows;
use crate::engine::gpu::glyphs::GlyphRows;
use crate::engine::gpu::segments::SegRows;
use brep::{walk_brep, walk_surface};
use cloud::walk_cloud;
use curves::{walk_line, walk_nurbscurve, walk_polyline};
use frames::{walk_obb, walk_plane};
use mesh::{MeshCx, MeshOpts, walk_mesh};
use mesh_ink::Ink;
use points::walk_point;
use session_rust::AABB;
use session_rust::Geometry;
use session_rust::element::ElementGeometry;

pub mod bounds;
pub mod brep;
pub mod brep_edges;
pub mod brep_orient;
pub mod cloud;
pub mod curves;
pub mod encode;
pub mod frames;
pub mod mesh;
pub mod mesh_ink;
pub mod mesh_topology;
pub mod points;

/// The four row tables one object writes into.
pub struct Walk<'a> {
    pub arena: &'a mut ArenaRows, // triangles of faces
    pub seg: &'a mut SegRows, // line segments
    pub glyph: &'a mut GlyphRows, // dots and labels
    pub cloud: &'a mut CloudRows, // point cloud points
}

impl<'a> Walk<'a> {
    /// Borrow every table of one upload.
    pub fn of(t: &'a mut Upload) -> Self {
        Self {
            arena: &mut t.arena,
            seg: &mut t.seg,
            glyph: &mut t.glyph,
            cloud: &mut t.cloud,
        }
    }

    /// Tables a solid needs: faces plus its edge ink.
    fn solid(&mut self) -> (&mut ArenaRows, Ink<'_>) {
        (
            self.arena,
            Ink {
                seg: self.seg,
                glyph: self.glyph,
            },
        )
    }
}

/// Where one object's rows go.
pub struct WalkCx {
    pub vert_base: u32, // arena vertices already on the GPU
    pub cloud_px: f32, // point size override in px, 0 = file's own
    pub row: u32, // this object's row index
}

/// What a producer reports for one object row.
pub struct Row {
    pub bounds: AABB, // local bounding box
    pub spacing: f32, // point or vertex spacing
    pub flags: u32, // row flag bits
    pub faces: bool, // row drew faces
}

impl Row {
    /// A row with only a box: lines, points, frames.
    pub fn thin(bounds: AABB) -> Self {
        Self {
            bounds,
            spacing: 0.0,
            flags: 0,
            faces: false,
        }
    }
}

/// An element without geometry gets no row.
pub fn is_drawable(geom: &Geometry) -> bool {
    match geom {
        Geometry::Element(e) => !matches!(e.geometry(), ElementGeometry::None),
        _ => true,
    }
}

/// Write one object into the tables and report its row.
pub fn walk_geometry(w: &mut Walk, cx: &WalkCx, geom: &Geometry) -> Row {
    match geom {
        Geometry::Mesh(m) => {
            let (arena, mut ink) = w.solid();
            walk_mesh(
                arena,
                &mut ink,
                m,
                &MeshCx {
                    cx,
                    opts: &MeshOpts::OBJECT,
                },
            )
        }
        Geometry::BRep(b) => {
            let (arena, mut ink) = w.solid();
            walk_brep(arena, &mut ink, b, cx)
        }
        Geometry::NurbsSurface(s) => {
            let (arena, mut ink) = w.solid();
            walk_surface(arena, &mut ink, s, cx)
        }
        Geometry::Line(l) => walk_line(w.seg, l, cx.row),
        Geometry::Polyline(pl) => walk_polyline(w.seg, pl, cx.row),
        Geometry::NurbsCurve(c) => walk_nurbscurve(w.seg, c, cx.row),
        Geometry::Plane(p) => walk_plane(w.seg, p, cx.row),
        Geometry::OBB(b) => walk_obb(w.seg, b, cx.row),
        Geometry::Point(p) => walk_point(w.glyph, p, cx.row),
        Geometry::PointCloud(pc) => walk_cloud(w.cloud, pc, cx),
        Geometry::Element(e) => match e.geometry() {
            ElementGeometry::Mesh(m) => {
                let (arena, mut ink) = w.solid();
                walk_mesh(
                    arena,
                    &mut ink,
                    m,
                    &MeshCx {
                        cx,
                        opts: &MeshOpts::ELEMENT,
                    },
                )
            }
            ElementGeometry::BRep(b) => {
                let (arena, mut ink) = w.solid();
                walk_brep(arena, &mut ink, b, cx)
            }
            ElementGeometry::None => Row::thin(AABB::empty()),
        },
    }
}
