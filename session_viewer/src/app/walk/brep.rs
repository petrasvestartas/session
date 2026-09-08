//! A BRep or a NURBS surface into the tables. A BRep's faces are uploaded one by one with the
//! kernel's own vertices and analytic normals - no weld across faces, which is what let the
//! shader fall back to flat derivative normals - and its edges are pipes read off the face
//! tessellations (`brep_edges`). No sheet lanes; `FLAG_OPEN` only when the BRep is not a
//! solid, from its own topology rather than from a welded mesh.

use session_rust::{BRep, Color, NurbsSurface, RenderMesh};
use session_rust::remesh_nurbssurface_grid::RemeshNurbsSurfaceGrid;
use crate::app::knobs;
use crate::engine::gpu::arena::ArenaRows;
use crate::engine::gpu::Instance;
use super::{Row, WalkCx};
use super::bounds::mesh_thickness;
use super::brep_edges::{edge_chains, push_edge_pipes, EdgeChain, EdgePen};
use super::brep_orient::face_signs;
use super::mesh::{mesh_spacing, walk_mesh, MeshCx, MeshOpts};
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

/// The positions and the file-local triangle indices of every face uploaded so far: what the
/// thickness measure reads once all faces are in.
struct Solid {
    pos: Vec<[f32; 3]>,
    tris: Vec<u32>,
    bounds: Aabb,
}

/// One face mesh into the arena under `cx.row`: its own vertices, its normals as the kernel
/// evaluated them, its triangles based on the file's vertex base.
fn push_face(arena: &mut ArenaRows, rm: &RenderMesh, cx: &WalkCx, solid: &mut Solid) {
    let base = cx.vert_base + arena.verts.len() as u32;
    let local = solid.pos.len() as u32;
    arena.verts.reserve(rm.vertices.len());
    arena.vids.reserve(rm.vertices.len());
    for v in &rm.vertices {
        solid.bounds.grow(v.position);
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

/// Tessellate a BRep face by face, upload each with its surface colour and normals, then ink
/// its edges. The row is one object; `FLAG_SMOOTH` tells the marker lane the vertices are
/// samples; `FLAG_OPEN` is the BRep's own `is_solid`, since an open shell shows its inside.
pub fn walk_brep(arena: &mut ArenaRows, ink: &mut Ink, b: &BRep, cx: &WalkCx) -> Row {
    let mut fms = b.face_meshes_q(Some(QUALITY));
    let mut solid = Solid { pos: Vec::new(), tris: Vec::new(), bounds: Aabb::empty() };
    let mut verts = 0;
    for fm in fms.iter_mut() {
        fm.set_objectcolor(b.surfacecolor.clone());
        verts += fm.vertex.len();
        push_face(arena, &fm.to_render(), cx, &mut solid);
    }
    let mut flags = Instance::FLAG_SMOOTH;
    if !b.is_solid() {
        flags |= Instance::FLAG_OPEN;
    }
    let thickness = mesh_thickness(&solid.pos, &solid.tris);
    let mut row = Row { bounds: solid.bounds, spacing: mesh_spacing(&solid.bounds, verts), flags, faces: true, thickness };
    if !knobs::no_edges() {
        let pen = Pen { row: cx.row, radius: encode_width(b.width), color: pack_rgba(Color::black().to_f32()) };
        let chains = edge_chains(b, &fms);
        let signs = face_signs(b, &fms, &chains);
        let ep = EdgePen { fms: &fms, signs: &signs, pen };
        walk_brep_edges(ink, b, &chains, (&ep, &mut row.bounds));
    }
    row
}

/// The solid's own edges, one chain per BRep edge off the tessellation (pipes, culled by the
/// two adjacent faces); an edge no grid face owns is sampled off its 3D curve as a ribbon,
/// today's path, until the kernel supplies every edge's polygon.
fn walk_brep_edges(ink: &mut Ink, b: &BRep, chains: &[Option<EdgeChain>], out: (&EdgePen, &mut Aabb)) {
    let (ep, bounds) = out;
    for (ei, chain) in chains.iter().enumerate() {
        match chain {
            Some(c) => {
                push_edge_pipes(ink.seg, c, ep, bounds);
            }
            None => push_curve_ribbon(ink, b, ei, (&ep.pen, bounds)),
        }
    }
}

/// The fallback for an edge with no grid face: the 3D curve sampled by turning angle, drawn as
/// a ribbon with no facing (nothing exact is known about its neighbours).
fn push_curve_ribbon(ink: &mut Ink, b: &BRep, ei: usize, out: (&Pen, &mut Aabb)) {
    let edge = &b.m_edges[ei];
    if edge.degenerated || edge.curve_3d_index < 0 {
        return;
    }
    let points: Vec<[f32; 3]> = sample_nurbscurve(&b.m_curves_3d[edge.curve_3d_index as usize])
        .into_iter()
        .map(|p| p.map(|v| v as f32))
        .collect();
    if points.len() < 2 {
        return;
    }
    push_polyline(ink.seg, &points, out.0, out.1);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::glyphs::GlyphRows;
    use crate::engine::gpu::segments::SegRows;
    use crate::engine::gpu::Instance;
    use crate::app::walk::brep_edges::edge_chains;

    /// The walked tables of one BRep at row 5: arena rows, pipes, ribbons, markers, flags.
    fn walked(b: &BRep) -> (ArenaRows, SegRows, GlyphRows, Row) {
        let mut arena = ArenaRows::default();
        let mut seg = SegRows::default();
        let mut glyph = GlyphRows::default();
        let cx = WalkCx { vert_base: 100, cloud_px: 0.0, row: 5 };
        let row = {
            let mut ink = Ink { seg: &mut seg, glyph: &mut glyph };
            walk_brep(&mut arena, &mut ink, b, &cx)
        };
        (arena, seg, glyph, row)
    }

    /// A cylinder uploads every face mesh's own vertices (no weld: the side's 146 plus the two
    /// caps' 72 each, read from the kernel, not assumed), each with a unit normal, its
    /// indices based on the file's vertex base, one pipe per chain segment and no ribbons,
    /// no markers, FLAG_SMOOTH and not FLAG_OPEN.
    #[test]
    fn cylinder_walks_unwelded_with_normals() {
        let b = BRep::create_cylinder(150.0, 400.0);
        let fms = b.face_meshes_q(Some(QUALITY));
        let verts: usize = fms.iter().map(|m| m.vertex.len()).sum();
        let tris: usize = fms.iter().map(|m| m.to_render().indices.len()).sum();
        let segments: usize = edge_chains(&b, &fms).iter().flatten().map(|c| c.keys.len() - 1).sum();
        let (arena, seg, glyph, row) = walked(&b);
        assert_eq!(arena.verts.len(), verts);
        assert_eq!(arena.vids.len(), verts);
        assert_eq!(arena.idx.len(), tris);
        assert!(arena.idx.iter().all(|&i| i >= 100 && i < 100 + verts as u32));
        for v in &arena.verts {
            let n = v.normal;
            let l = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            assert!((l - 1.0).abs() < 1e-3, "normal {n:?}");
        }
        assert_eq!(seg.pipes.len(), segments);
        assert!(seg.ribbons.is_empty());
        assert!(glyph.spheres.is_empty());
        assert_ne!(row.flags & Instance::FLAG_SMOOTH, 0);
        assert_eq!(row.flags & Instance::FLAG_OPEN, 0);
        assert!(row.faces);
        assert!(row.bounds.diagonal() > 400.0);
    }

    /// A single face pulled out of a solid is not solid: it is walked FLAG_OPEN so the facing
    /// cull, whose premise is a closed surface, is skipped.
    #[test]
    fn open_brep_is_flagged_open() {
        let mut b = BRep::create_box(400.0, 300.0, 250.0);
        b.m_solids.clear();
        let (_, _, _, row) = walked(&b);
        assert_ne!(row.flags & Instance::FLAG_OPEN, 0);
    }
}
