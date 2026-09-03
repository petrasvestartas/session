//! The walk: one producer per kernel geometry type, each receiving ONLY the row families it
//! writes (which shaders a type can reach is readable off its signature). `walk_geometry`
//! dispatches; `Row` is what a producer hands back for its object row - producers never push one.

use session_rust::Geometry;
use session_rust::element::ElementGeometry;
use crate::engine::gpu::Upload;
use crate::engine::gpu::arena::ArenaRows;
use crate::engine::gpu::cloud::CloudRows;
use crate::engine::gpu::glyphs::GlyphRows;
use crate::engine::gpu::objects::ObjectRows;
use crate::engine::gpu::segments::SegRows;
use brep::walk_brep;
use cloud::walk_cloud;
use curves::{walk_line, walk_nurbscurve, walk_polyline};
use frames::{walk_obb, walk_plane};
use mesh::{walk_mesh, MeshOpts};
use mesh_ink::Ink;
use points::walk_point;
use surface::walk_surface;

pub mod bounds;
pub mod brep;
pub mod cloud;
pub mod curves;
pub mod encode;
pub mod frames;
pub mod mesh;
pub mod mesh_ink;
pub mod mesh_topology;
pub mod points;
pub mod surface;

pub use encode::FACING_UNKNOWN;

/// The sinks a producer may write: the five groups of one `Upload`, borrowed for one object.
pub struct Walk<'a> {
    pub obj: &'a mut ObjectRows,
    pub arena: &'a mut ArenaRows,
    pub seg: &'a mut SegRows,
    pub glyph: &'a mut GlyphRows,
    pub cloud: &'a mut CloudRows,
}

impl<'a> Walk<'a> {
    /// Every group of `t`.
    pub fn of(t: &'a mut Upload) -> Self {
        Self { obj: &mut t.obj, arena: &mut t.arena, seg: &mut t.seg, glyph: &mut t.glyph, cloud: &mut t.cloud }
    }

    /// The SOLID lane a tessellated surface reaches: the arena for its faces, the ink pair
    /// (pipes, spheres) for its edges and dots.
    pub fn solid(&mut self) -> (&mut ArenaRows, Ink<'_>) {
        (self.arena, Ink { seg: self.seg, glyph: self.glyph })
    }
}

/// Where one object's rows land: the arena rows already on the GPU (`walk_mesh` bases its
/// indices on it), the cloud points already uploaded (a draw's `first` counts from it), the
/// file's point-size override in px (0 = the pb's own) and the object row being walked.
pub struct WalkCx {
    pub vert_base: u32,
    pub cloud_base: u32,
    pub cloud_px: f32,
    pub row: u32,
}

/// What a producer reports for its object row: the mesh-local box (meshes that drew ink only),
/// the point/vertex spacing and the flags it earned. The caller pushes the columns.
pub struct Row {
    pub bounds: Option<([f32; 3], [f32; 3])>,
    pub spacing: f32,
    pub flags: u32,
}

impl Row {
    /// Linework, points, frames: no box, no spacing, no flags.
    pub fn none() -> Self {
        Self { bounds: None, spacing: 0.0, flags: 0 }
    }

    /// A tessellated surface's row.
    pub fn solid(bounds: Option<([f32; 3], [f32; 3])>, spacing: f32, flags: u32) -> Self {
        Self { bounds, spacing, flags }
    }

    /// A cloud's row: the per-file point size rides the spacing column.
    pub fn point_size_px(px: f32) -> Self {
        Self { bounds: None, spacing: px, flags: 0 }
    }
}

/// An `Element` with no geometry gets no row at all; everything else does.
pub fn is_drawable(geom: &Geometry) -> bool {
    match geom {
        Geometry::Element(e) => !matches!(e.geometry(), ElementGeometry::None),
        _ => true,
    }
}

/// One object into the tables. 3D geometry takes the SOLID lane (edges are cylinders,
/// vertices spheres); free linework and points the FLAT lane; every cloud the splat lane.
/// FLAG_OPEN for `Mesh` objects only - an Element's mesh never raised it.
pub fn walk_geometry(w: &mut Walk, cx: &WalkCx, geom: &Geometry) -> Row {
    match geom {
        Geometry::Mesh(m) => { let (arena, mut ink) = w.solid(); walk_mesh(arena, &mut ink, m, &MeshOpts::sheet(cx, true)) }
        Geometry::BRep(b) => { let (arena, mut ink) = w.solid(); walk_brep(arena, &mut ink, b, cx) }
        Geometry::Line(l) => walk_line(w.seg, l, cx.row),
        Geometry::Polyline(pl) => walk_polyline(w.seg, pl, cx.row),
        Geometry::NurbsCurve(c) => walk_nurbscurve(w.seg, c, cx.row),
        Geometry::Point(p) => walk_point(w.glyph, p, cx.row),
        Geometry::PointCloud(pc) => walk_cloud(w.cloud, pc, cx),
        Geometry::NurbsSurface(s) => { let (arena, mut ink) = w.solid(); walk_surface(arena, &mut ink, s, cx) }
        Geometry::Plane(p) => walk_plane(w.seg, p, cx.row),
        Geometry::OBB(b) => walk_obb(w.seg, b, cx.row),
        Geometry::Element(e) => match e.geometry() {
            ElementGeometry::Mesh(m) => { let (arena, mut ink) = w.solid(); walk_mesh(arena, &mut ink, m, &MeshOpts::sheet(cx, false)) }
            ElementGeometry::BRep(b) => { let (arena, mut ink) = w.solid(); walk_brep(arena, &mut ink, b, cx) }
            ElementGeometry::None => Row::none(),
        },
    }
}
