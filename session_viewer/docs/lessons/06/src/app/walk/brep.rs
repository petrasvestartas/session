use super::mesh::{MeshCx, MeshOpts, mesh_spacing, walk_mesh};
use super::mesh_ink::Ink;
use super::{Row, WalkCx};
use crate::engine::gpu::Instance;
use crate::engine::gpu::arena::ArenaRows;
use session_rust::AABB;
use session_rust::remesh_nurbssurface_grid::RemeshNurbsSurfaceGrid;
use session_rust::{BRep, NurbsSurface, RenderMesh};

/// Mesh quality: 5° between samples, chord sag 0.001 of the size.
pub const QUALITY: (f64, f64) = (5.0, 0.001);

/// Every face uploaded so far.
struct Solid {
    pos: Vec<[f32; 3]>, // vertex positions
    tris: Vec<u32>,     // triangle indices into `pos`
    bounds: AABB,       // box of all vertices
}

/// Append one face mesh to the arena.
fn push_face(arena: &mut ArenaRows, rm: &RenderMesh, cx: &WalkCx, solid: &mut Solid) {
    let base = cx.vert_base + arena.verts.len() as u32; // first GPU vertex index
    let local = solid.pos.len() as u32; // first index in `solid`
    arena.verts.reserve(rm.vertices.len());
    arena.vids.reserve(rm.vertices.len());

    for v in &rm.vertices {
        solid.bounds.union_with_point(
            v.position[0] as f64,
            v.position[1] as f64,
            v.position[2] as f64,
        );
        solid.pos.push(v.position);
        arena.verts.push(*v);
        arena.vids.push(cx.row);
    }

    arena.idx.reserve(rm.indices.len());

    for &i in &rm.indices {
        arena.idx.push(base + i);
        solid.tris.push(local + i);
    }
}

/// Upload each face mesh on its own.
pub fn walk_brep(arena: &mut ArenaRows, _ink: &mut Ink, brep: &BRep, cx: &WalkCx) -> Row {
    let mut solid = Solid {
        pos: Vec::new(),
        tris: Vec::new(),
        bounds: AABB::empty(),
    };
    let mut vertices = 0;

    for mut face in brep.face_meshes_q(Some(QUALITY)) {
        face.set_objectcolor(brep.surfacecolor.clone());
        vertices += face.vertex.len();
        push_face(arena, &face.to_render(), cx, &mut solid);
    }

    Row {
        bounds: solid.bounds,
        spacing: mesh_spacing(&solid.bounds, vertices),
        flags: Instance::FLAG_SMOOTH
            | if brep.is_solid() {
                0
            } else {
                Instance::FLAG_OPEN
            },
        faces: true,
    }
}

/// The whole UV domain; trims come later.
pub fn walk_surface(
    arena: &mut ArenaRows,
    ink: &mut Ink,
    surface: &NurbsSurface,
    cx: &WalkCx,
) -> Row {
    let mut mesh = RemeshNurbsSurfaceGrid::from_u_v_q(surface.clone(), 0, 0, QUALITY.0, QUALITY.1);

    if let Some(color) = surface.facecolors.first() {
        mesh.set_objectcolor(color.clone());
    }

    let options = MeshOpts {
        sheet_lanes: false,
        allow_open: true,
        smooth: true,
    };
    walk_mesh(arena, ink, &mesh, &MeshCx { cx, opts: &options })
}
