//! A NURBS surface into the tables: tessellate, tint with its first face colour, hand the mesh
//! to `walk_mesh` as a MODEL mesh - no sheet lanes, no FLAG_OPEN.

use session_rust::NurbsSurface;
use crate::engine::gpu::arena::ArenaRows;
use super::{Row, WalkCx};
use super::mesh::{walk_mesh, MeshOpts};
use super::mesh_ink::Ink;

/// Tessellate and walk.
pub fn walk_surface(arena: &mut ArenaRows, ink: &mut Ink, s: &NurbsSurface, cx: &WalkCx) -> Row {
    let mut sm = s.mesh();
    if let Some(c) = s.facecolors.first() {
        sm.set_objectcolor(c.clone());
    }
    walk_mesh(arena, ink, &sm, &MeshOpts::model(cx))
}
