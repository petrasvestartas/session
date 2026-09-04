//! The walk: one producer per kernel geometry type, each receiving ONLY the lane tables it
//! writes. `walk_geometry` dispatches; `Row` is what a producer reports for its object row.
//! Deleting a lane = deleting its producer file and its arm here.

use session_rust::Geometry;
use session_rust::element::ElementGeometry;
use crate::engine::gpu::Upload;
use crate::engine::gpu::arena::ArenaRows;
use crate::engine::gpu::glyphs::GlyphRows;
use crate::engine::gpu::segments::SegRows;
use crate::math::Aabb;
use brep::{walk_brep, walk_surface};
use curves::{walk_line, walk_nurbscurve, walk_polyline};
use frames::{walk_obb, walk_plane};
use hosts::Hosts;
use mesh::{walk_mesh, MeshCx, MeshOpts};
use mesh_ink::Ink;
use points::walk_point;

pub mod bounds;
pub mod brep;
pub mod curves;
pub mod encode;
pub mod frames;
pub mod hosts;
pub mod mesh;
pub mod mesh_ink;
pub mod mesh_topology;
pub mod points;

/// The lane tables a producer may write, borrowed from one `Upload` for one object.
pub struct Walk<'a> {
    pub arena: &'a mut ArenaRows,
    pub seg: &'a mut SegRows,
    pub glyph: &'a mut GlyphRows,
}

impl<'a> Walk<'a> {
    /// Every lane table of `t`.
    pub fn of(t: &'a mut Upload) -> Self {
        Self { arena: &mut t.arena, seg: &mut t.seg, glyph: &mut t.glyph }
    }

    /// The SOLID lane a tessellated surface reaches: the arena for its faces and the ink pair.
    fn solid(&mut self) -> (&mut ArenaRows, Ink<'_>) {
        (self.arena, Ink { seg: self.seg, glyph: self.glyph })
    }
}

/// Where one object's rows land: the arena rows already on the GPU (`walk_mesh` bases its
/// indices on it) and the object row.
pub struct WalkCx<'a> {
    pub vert_base: u32,
    pub row: u32,
    /// The file's plate faces, so a free outline lying on one inherits its normal and thickness.
    pub hosts: &'a Hosts,
}

/// What a producer reports for its object row: the local box, the point/vertex spacing and
/// the flags it earned.
pub struct Row {
    pub bounds: Aabb,
    pub spacing: f32,
    pub flags: u32,
    /// The row drew faces: the inside test (eye within the box) applies to it.
    pub faces: bool,
    /// The object's thickness in its own units, whatever its orientation (section 6 of
    /// ARCHITECTURE.md): the depth budget the shaders may spend on it.
    pub thickness: f32,
}

impl Row {
    /// Linework, points, frames: a box, no spacing, no flags, no faces; as thick as the box.
    pub fn thin(bounds: Aabb) -> Self {
        Self { bounds, spacing: 0.0, flags: 0, faces: false, thickness: bounds.thinnest() }
    }
}

/// An `Element` with no geometry gets no row at all; everything else does.
pub fn is_drawable(geom: &Geometry) -> bool {
    match geom {
        Geometry::Element(e) => !matches!(e.geometry(), ElementGeometry::None),
        _ => true,
    }
}

/// One object into the tables. Meshes, BReps and surfaces take the SOLID lane; free linework
/// and points the FLAT lane.
pub fn walk_geometry(w: &mut Walk, cx: &WalkCx, geom: &Geometry) -> Row {
    match geom {
        Geometry::Mesh(m) => {
            let (arena, mut ink) = w.solid();
            walk_mesh(arena, &mut ink, m, &MeshCx { cx, opts: &MeshOpts::OBJECT })
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
        Geometry::Polyline(pl) => walk_polyline(w.seg, pl, cx),
        Geometry::NurbsCurve(c) => walk_nurbscurve(w.seg, c, cx.row),
        Geometry::Plane(p) => walk_plane(w.seg, p, cx.row),
        Geometry::OBB(b) => walk_obb(w.seg, b, cx.row),
        Geometry::Point(p) => walk_point(w.glyph, p, cx.row),
        Geometry::PointCloud(_) => Row::thin(Aabb::empty()),
        Geometry::Element(e) => match e.geometry() {
            ElementGeometry::Mesh(m) => {
                let (arena, mut ink) = w.solid();
                walk_mesh(arena, &mut ink, m, &MeshCx { cx, opts: &MeshOpts::ELEMENT })
            }
            ElementGeometry::BRep(b) => {
                let (arena, mut ink) = w.solid();
                walk_brep(arena, &mut ink, b, cx)
            }
            ElementGeometry::None => Row::thin(Aabb::empty()),
        },
    }
}
