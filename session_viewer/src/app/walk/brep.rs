//! A BRep or a NURBS surface into the tables: tessellate, tint, hand the mesh to `walk_mesh`
//! as a MODEL mesh - no sheet lanes, no `FLAG_OPEN` (a tessellation is often numerically
//! non-watertight and would lose the facing cull wholesale).

use session_rust::{BRep, Color, Mesh, NurbsSurface};
use session_rust::remesh_nurbssurface_grid::RemeshNurbsSurfaceGrid;
use crate::engine::gpu::arena::ArenaRows;
use super::{Row, WalkCx};
use super::mesh::{walk_mesh, MeshCx, MeshOpts};
use super::mesh_ink::Ink;
use super::curves::{push_polyline, sample_nurbscurve};
use super::encode::{encode_width, pack_rgba, Pen};
use crate::math::Aabb;

/// How finely the VIEWER wants a surface tessellated: the normal may turn 5 degrees between
/// samples and a chord may sag a thousandth of the object. The kernel's own default is 20
/// degrees and 0.005, which turns a cylinder into an 18-sided prism with a visibly polygonal
/// silhouette - tessellation quality is a display decision, so the display makes it.
/// Measured on 25 extruded circles: 1650 -> 4050 faces, 1.6 -> 1.8 ms a frame.
pub const QUALITY: (f64, f64) = (5.0, 0.001);

/// Tessellate a BRep with its surface colour and walk it.
pub fn walk_brep(arena: &mut ArenaRows, ink: &mut Ink, b: &BRep, cx: &WalkCx) -> Row {
    let mut polygons = Vec::new();
    for fm in b.face_meshes_q(Some(QUALITY)) {
        // faces() is sorted; face.values() is HashMap order, which Rust does not keep
        // stable. from_polylines welds by first-seen point, so an unstable order renumbers
        // the welded mesh on every walk - and the ink, whose edges are decided from that
        // numbering, flickers as a result.
        for key in fm.faces() {
            let verts = &fm.face[&key];
            polygons.push(verts.iter().map(|vi| fm.vertex[vi].position()).collect());
        }
    }
    let mut bm = Mesh::from_polylines(polygons, Some(1e-6));
    // The weld inherits whatever orientation the face meshes carried, and this mesh never
    // passes through `Mesh::from_proto`, which is where a file's winding is settled. A BRep
    // whose face flags disagree would otherwise reach the ink lanes with normals pointing both
    // ways - the lines, not the fill, are what shows it.
    bm.orient_faces();
    bm.set_objectcolor(b.surfacecolor.clone());
    // The tessellation is a FILL. Its triangle edges are an artifact of meshing: width 0
    // hides them (mesh_ink::hidden); the solid's real edges come off the BRep below.
    bm.set_linecolors(vec![b.surfacecolor.clone()], vec![0.0]);

    let mut row = walk_mesh(arena, ink, &bm, &MeshCx { cx, opts: &MeshOpts::MODEL });
    walk_brep_edges(ink, b, cx.row, &mut row.bounds);
    row
}

/// The solid's own edges, sampled off its 3D curves. Iterating `m_edges` rather than face
/// wires draws an edge shared by two faces ONCE, and every edge is a real feature of the
/// solid - so what is drawn does not depend on the tessellation or on the view direction.
fn walk_brep_edges(ink: &mut Ink, b: &BRep, row: u32, bounds: &mut Aabb) {
    let pen = Pen {
        row,
        radius: encode_width(b.width),
        color: pack_rgba(Color::black().to_f32()),
    };
    for edge in &b.m_edges {
        if edge.degenerated || edge.curve_3d_index < 0 {
            continue;
        }
        let points: Vec<[f32; 3]> = sample_nurbscurve(&b.m_curves_3d[edge.curve_3d_index as usize])
            .into_iter()
            .map(|p| p.map(|v| v as f32))
            .collect();
        if points.len() < 2 {
            continue;
        }
        push_polyline(ink.seg, &points, &pen, bounds);
    }
}

/// Tessellate a surface with its first face colour and walk it. A planar surface has no
/// curvature to follow, so the kernel's own corner quad is already exact.
pub fn walk_surface(arena: &mut ArenaRows, ink: &mut Ink, s: &NurbsSurface, cx: &WalkCx) -> Row {
    let mut sm = if s.is_planar(1e-6) {
        s.mesh()
    } else {
        RemeshNurbsSurfaceGrid::from_u_v_q(s.clone(), 0, 0, QUALITY.0, QUALITY.1)
    };
    if let Some(c) = s.facecolors.first() {
        sm.set_objectcolor(c.clone());
    }
    walk_mesh(arena, ink, &sm, &MeshCx { cx, opts: &MeshOpts::MODEL })
}
