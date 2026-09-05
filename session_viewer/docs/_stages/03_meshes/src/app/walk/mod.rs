//! The walk: one producer per kernel geometry type, each receiving ONLY the lane tables it
//! writes. `walk_geometry` dispatches; `Row` is what a producer reports for its object row.
//! Deleting a lane = deleting its producer file and its arm here.

use session_rust::Geometry;
use session_rust::element::ElementGeometry;
use crate::engine::gpu::Upload;
use crate::engine::gpu::arena::ArenaRows;
use crate::math::Aabb;
use mesh::{walk_mesh, MeshCx, MeshOpts};

pub mod bounds;
pub mod mesh;

/// The lane tables a producer may write, borrowed from one `Upload` for one object.
pub struct Walk<'a> {
    pub arena: &'a mut ArenaRows,
}

impl<'a> Walk<'a> {
    /// Every lane table of `t`.
    pub fn of(t: &'a mut Upload) -> Self {
        Self { arena: &mut t.arena }
    }
}

/// Where one object's rows land: the arena rows already on the GPU (`walk_mesh` bases its
/// indices on it) and the object row.
pub struct WalkCx {
    pub vert_base: u32,
    pub row: u32,
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

/// One object into the tables. Meshes take the SOLID lane.
pub fn walk_geometry(w: &mut Walk, cx: &WalkCx, geom: &Geometry) -> Row {
    match geom {
        Geometry::Mesh(m) => walk_mesh(w.arena, m, &MeshCx { cx, opts: &MeshOpts::OBJECT }),
        Geometry::BRep(_) => Row::thin(Aabb::empty()),
        Geometry::NurbsSurface(_) => Row::thin(Aabb::empty()),
        Geometry::Line(_) => Row::thin(Aabb::empty()),
        Geometry::Polyline(_) => Row::thin(Aabb::empty()),
        Geometry::NurbsCurve(_) => Row::thin(Aabb::empty()),
        Geometry::Plane(_) => Row::thin(Aabb::empty()),
        Geometry::OBB(_) => Row::thin(Aabb::empty()),
        Geometry::Point(_) => Row::thin(Aabb::empty()),
        Geometry::PointCloud(_) => Row::thin(Aabb::empty()),
        Geometry::Element(_) => Row::thin(Aabb::empty()),
    }
}
