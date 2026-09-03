//! A BRep or a NURBS surface into the tables: tessellate, tint, hand the mesh to `walk_mesh`
//! as a MODEL mesh - no sheet lanes, no `FLAG_OPEN` (a tessellation is often numerically
//! non-watertight and would lose the facing cull wholesale).

use session_rust::{BRep, NurbsSurface};
use crate::engine::gpu::arena::ArenaRows;
use super::{Row, WalkCx};
use super::mesh::{walk_mesh, MeshCx, MeshOpts};
use super::mesh_ink::Ink;

/// Tessellate a BRep with its surface colour and walk it.
pub fn walk_brep(arena: &mut ArenaRows, ink: &mut Ink, b: &BRep, cx: &WalkCx) -> Row {
    let mut bm = b.mesh();
    bm.set_objectcolor(b.surfacecolor.clone());
    walk_mesh(arena, ink, &bm, &MeshCx { cx, opts: &MeshOpts::MODEL })
}

/// Tessellate a surface with its first face colour and walk it.
pub fn walk_surface(arena: &mut ArenaRows, ink: &mut Ink, s: &NurbsSurface, cx: &WalkCx) -> Row {
    let mut sm = s.mesh();
    if let Some(c) = s.facecolors.first() {
        sm.set_objectcolor(c.clone());
    }
    walk_mesh(arena, ink, &sm, &MeshCx { cx, opts: &MeshOpts::MODEL })
}
