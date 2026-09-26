// --8<-- [start:step-6a]
use super::brep_edges::{EdgeChain, EdgePen, edge_chains, push_edge_pipes};
use super::brep_orient::face_signs;
use super::curves::{push_polyline, sample_nurbscurve};
use super::encode::{Pen, encode_width, pack_rgba};
use super::mesh::{MeshCx, MeshOpts, mesh_spacing, walk_mesh};
use super::mesh_ink::Ink;
use super::{Row, WalkCx};
use crate::app::knobs;
use crate::engine::gpu::Instance;
use crate::engine::gpu::arena::ArenaRows;
use session_rust::AABB;
use session_rust::remesh_nurbssurface_grid::RemeshNurbsSurfaceGrid;
use session_rust::{BRep, Color, NurbsSurface, RenderMesh};
// --8<-- [end:step-6a]

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

// --8<-- [start:step-6b]
/// A BRep: every face meshed and uploaded, then its edges.
pub fn walk_brep(arena: &mut ArenaRows, ink: &mut Ink, b: &BRep, cx: &WalkCx) -> Row {
    let mut fms = b.face_meshes_q(Some(QUALITY)); // one mesh per face
    let chains = edge_chains(b, &fms); // edge polylines on the meshes
    let signs = face_signs(b, &fms, &chains); // +1 or -1 per face
    let mut solid = Solid {
        pos: Vec::new(),
        tris: Vec::new(),
        bounds: AABB::empty(),
    };
    let mut verts = 0; // vertex total for spacing

    for (fi, fm) in fms.iter_mut().enumerate() {
        fm.set_objectcolor(b.surfacecolor.clone());
        verts += fm.vertex.len();
        let mut rm = fm.to_render();

        // an inside-out face: flip normals and winding
        if signs[fi] < 0.0 {
            for vertex in &mut rm.vertices {
                for component in &mut vertex.normal {
                    *component = -*component;
                }
            }

            for triangle in rm.indices.chunks_exact_mut(3) {
                triangle.swap(1, 2);
            }
        }

        push_face(arena, &rm, cx, &mut solid);
    }

    let mut flags = Instance::FLAG_SMOOTH; // vertices are samples

    if !b.is_solid() {
        flags |= Instance::FLAG_OPEN;
    }

    if b.face_count() == 1 {
        flags |= Instance::FLAG_SINGLE;
    }

    let mut row = Row {
        bounds: solid.bounds,
        spacing: mesh_spacing(&solid.bounds, verts),
        flags,
        faces: true,
    };

    if !knobs::no_edges() {
        let pen = Pen {
            row: cx.row,
            radius: encode_width(b.width),
            color: pack_rgba(Color::black().to_f32()),
        };
        let ep = EdgePen {
            fms: &fms,
            signs: &signs,
            pen,
        };
        walk_brep_edges(ink, b, &chains, (&ep, &mut row.bounds));
    }

    row
}

/// Every BRep edge as pipes, or as a ribbon when no mesh owns it.
fn walk_brep_edges(
    ink: &mut Ink,
    b: &BRep,
    chains: &[Option<EdgeChain>],
    out: (&EdgePen, &mut AABB),
) {
    let (ep, bounds) = out;

    for (ei, chain) in chains.iter().enumerate() {
        match chain {
            Some(c) => {
                push_edge_pipes(ink.seg, c, ep, bounds);
            }
            None => {
                if !b.m_edges[ei].degenerated && !b.edge_faces(ei).is_empty() {
                    log::warn!(
                        "BREP {:?} edge {ei}: missing tessellation boundary mapping; analytic display fallback is not a coherent CAD boundary",
                        b.name
                    );
                }

                push_curve_ribbon(ink, b, ei, (&ep.pen, bounds));
            }
        }
    }
}

/// An edge sampled from its 3D curve as a ribbon.
fn push_curve_ribbon(ink: &mut Ink, b: &BRep, ei: usize, out: (&Pen, &mut AABB)) {
    let edge = &b.m_edges[ei];

    if edge.degenerated || edge.curve_3d_index < 0 {
        return;
    }

    let points: Vec<[f32; 3]> = sample_nurbscurve(&b.m_curves_3d[edge.curve_3d_index as usize])
        .into_iter()
        .map(super::curves::render_position)
        .collect();

    if points.len() < 2 {
        return;
    }

    push_polyline(ink.seg, &points, out.0, out.1);
    // --8<-- [end:step-6b]
}

/// The whole UV domain; trims come later.
pub fn walk_surface(
    arena: &mut ArenaRows,
    ink: &mut Ink,
    surface: &NurbsSurface,
    cx: &WalkCx,
) -> Row {
    let mut mesh = RemeshNurbsSurfaceGrid::from_u_v_q(surface, 0, 0, QUALITY.0, QUALITY.1);

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
