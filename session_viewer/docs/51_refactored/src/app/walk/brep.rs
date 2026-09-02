//! A BRep into the tables: tessellate, tint with the surface colour, hand the mesh to
//! `walk_mesh` as a MODEL mesh - no sheet lanes, no FLAG_OPEN (a BRep tessellation is often
//! numerically non-watertight and its solids would lose the facing cull wholesale).

use session_rust::BRep;
use crate::engine::gpu::arena::ArenaRows;
use super::{Row, WalkCx};
use super::mesh::{walk_mesh, MeshOpts};
use super::mesh_ink::Ink;

/// Tessellate and walk.
pub fn walk_brep(arena: &mut ArenaRows, ink: &mut Ink, b: &BRep, cx: &WalkCx) -> Row {
    let mut bm = b.mesh();
    bm.set_objectcolor(b.surfacecolor.clone());
    walk_mesh(arena, ink, &bm, &MeshOpts::model(cx))
}
