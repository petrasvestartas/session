//! The ink a mesh wears: one pipe per visible edge, one marker per vertex - the SOLID lane.
//! Reads the fused topology and the positions by slot; writes `SegRows.pipes` and
//! `GlyphRows.spheres`, nothing else.

use session_rust::Mesh;
use session_rust::mesh::ColorMode;
use crate::app::knobs;
use crate::engine::gpu::glyphs::GlyphRows;
use crate::engine::gpu::segments::SegRows;
use crate::engine::gpu::{CylinderSegment, GlyphPoint};
use super::encode::{encode_width, oct16, pack_facing, BLACK, FACING_UNKNOWN};
use super::mesh::{Lap, COPLANAR_DOT, WIREFRAME_BLACK_MIN};
use super::mesh_topology::{MeshTopo, SlotMap};

/// The two ink lanes a mesh reaches: pipes for its edges, spheres for its vertices.
pub struct Ink<'a> {
    pub seg: &'a mut SegRows,
    pub glyph: &'a mut GlyphRows,
}

/// What the ink pass needs from the face pass: the object row, the f32 positions by slot,
/// the key -> slot map and the profiling clock.
pub struct InkCx<'a> {
    pub row: u32,
    pub vpos: &'a [[f32; 3]],
    pub slots: &'a SlotMap,
    pub lap: &'a mut Lap,
}

/// Edge `i`'s pen width: one entry broadcasts to every edge, an absent one is the 1.0 default.
fn width_at(w: &[f64], i: usize) -> f64 {
    if w.len() == 1 { w[0] } else { w.get(i).copied().unwrap_or(1.0) }
}

/// Width 0 = hidden: a triangulated fill asks for no wireframe.
fn hidden(w: &[f64], i: usize) -> bool {
    width_at(w, i) == 0.0
}

/// The normal of the face in slot `side` of an edge's pair; None past a border.
fn normal_of(topo: &MeshTopo, faces: [u32; 2], side: usize) -> Option<[f64; 3]> {
    if faces[side] == u32::MAX { return None; }
    topo.normals[faces[side] as usize]
}

/// The two normals the facing test compares for edge `ei`. When the pair's winding disagrees,
/// meaning both faces walk the edge the same way, the second normal points into the solid, so
/// it is negated here: the test wants two outward normals, and the traversal direction is the
/// only local evidence of which of the two is the wrong way round.
fn edge_normals(topo: &MeshTopo, ei: usize) -> (Option<[f64; 3]>, Option<[f64; 3]>) {
    let f = topo.edge_faces[ei];
    let n0 = normal_of(topo, f, 0);
    let n1 = normal_of(topo, f, 1);
    if topo.opposed[ei] {
        return (n0, n1);
    }
    (n0, n1.map(|n| [-n[0], -n[1], -n[2]]))
}

/// Append edge `ei`'s faces to `fkeys`, deduped.
fn push_faces(edge_faces: &[[u32; 2]], ei: usize, fkeys: &mut Vec<usize>) {
    for &f in edge_faces[ei].iter() {
        if f == u32::MAX {
            continue;
        }
        let fk = f as usize;
        if !fkeys.contains(&fk) {
            fkeys.push(fk);
        }
    }
}

/// Word `k` of a marker's facing triple, by `pack_facing`'s rules.
fn facing_word(codes: &[u32], k: usize) -> u32 {
    match (codes.get(2 * k).copied(), codes.get(2 * k + 1).copied()) {
        (Some(a), b) => {
            let v = a | b.unwrap_or(a) << 16;
            if v == FACING_UNKNOWN { v ^ 1 } else { v }
        }
        _ => FACING_UNKNOWN,
    }
}

/// The pipe loop: one segment per visible, non-coplanar edge.
fn push_pipes(ink: &mut Ink, m: &Mesh, topo: &MeshTopo, cx: &InkCx) {
    let w = m.widths();
    let black_wire = topo.edges.len() >= WIREFRAME_BLACK_MIN;
    ink.seg.pipes.reserve(topo.edges.len());
    for (i, (a, b, col)) in topo.edges.iter().enumerate() {
        let (na, nb) = edge_normals(topo, i);
        let facing = pack_facing(na.as_ref(), nb.as_ref());
        if hidden(w, i) {
            continue;
        }
        // Interior tessellation: a diagonal across a flat region shares two coplanar faces.
        if let (Some(n0), Some(n1)) = (na, nb) {
            let dot = n0[0] * n1[0] + n0[1] * n1[1] + n0[2] * n1[2];
            if dot >= COPLANAR_DOT && !knobs::all_edges() {
                continue;
            }
        }
        ink.seg.pipes.push(CylinderSegment {
            p0: cx.vpos[cx.slots.slot(*a)],
            radius: encode_width(width_at(w, i)),
            p1: cx.vpos[cx.slots.slot(*b)],
            instance_id: cx.row,
            color: if black_wire { BLACK } else { *col },
            facing,
        });
    }
}

/// Per vertex: the widest visible incident edge (its width and index), and the incident
/// edge list as CSR (`vstart`, `vinc`). Hidden edges still count for adjacency.
struct Incidence {
    best: Vec<(f64, usize)>,
    vstart: Vec<u32>,
    vinc: Vec<u32>,
}

/// Build the incidence tables over the topology.
fn incidence(m: &Mesh, topo: &MeshTopo, cx: &InkCx) -> Incidence {
    let w = m.widths();
    let nv = cx.vpos.len();
    let mut best = vec![(f64::NEG_INFINITY, 0usize); nv];
    for (i, (a, b, _)) in topo.edges.iter().enumerate() {
        if hidden(w, i) {
            continue;
        }
        let wi = width_at(w, i);
        for vk in [*a, *b] {
            let e = &mut best[cx.slots.slot(vk)];
            if wi > e.0 {
                *e = (wi, i);
            }
        }
    }

    let mut vstart = vec![0u32; nv + 1];
    for (a, b, _) in topo.edges.iter() {
        vstart[cx.slots.slot(*a) + 1] += 1;
        vstart[cx.slots.slot(*b) + 1] += 1;
    }
    for i in 0..nv {
        vstart[i + 1] += vstart[i];
    }
    let mut vinc = vec![0u32; 2 * topo.edges.len()];
    let mut cur = vstart.clone();
    for (i, (a, b, _)) in topo.edges.iter().enumerate() {
        for vk in [*a, *b] {
            let s = cx.slots.slot(vk);
            vinc[cur[s] as usize] = i as u32;
            cur[s] += 1;
        }
    }
    Incidence { best, vstart, vinc }
}

/// What the marker loop reads: the ink context and the vertex incidence over the topology.
struct MarkerCx<'a, 'b> {
    cx: &'a InkCx<'b>,
    inc: &'a Incidence,
}

/// The marker loop: one glyph per vertex with a visible edge, carrying up to six incident
/// face normals (widest edge's pair first) so the disc hugs every face at a corner.
fn push_markers(ink: &mut Ink, m: &Mesh, topo: &MeshTopo, input: &MarkerCx) {
    let (cx, inc) = (input.cx, input.inc);
    let pc = m.get_pointcolors();
    let dots_colored = m.color_mode == ColorMode::POINTCOLORS && pc.len() == m.number_of_vertices();
    let nv = cx.vpos.len();
    let mut fkeys: Vec<usize> = Vec::new();
    let mut codes: Vec<u32> = Vec::new();
    ink.glyph.spheres.reserve(nv);
    for (i, &(vw, ei)) in inc.best.iter().enumerate().take(nv) {
        if vw == f64::NEG_INFINITY {
            continue;
        }
        fkeys.clear();
        push_faces(&topo.edge_faces, ei, &mut fkeys);
        for &j in &inc.vinc[inc.vstart[i] as usize..inc.vstart[i + 1] as usize] {
            push_faces(&topo.edge_faces, j as usize, &mut fkeys);
        }
        codes.clear();
        for fk in &fkeys {
            if let Some(n) = topo.normals[*fk] && let Some(code) = oct16(&n) && !codes.contains(&code) { codes.push(code); }
        }
        ink.glyph.spheres.push(GlyphPoint {
            center: cx.vpos[i],
            radius: encode_width(vw),
            color: if dots_colored { pc[i].to_f32() } else { [0.1, 0.1, 0.1, 1.0] },
            instance_id: cx.row,
            // A truncated normal list cannot prove every incident face points away.
            facing: if codes.len() > 6 { FACING_UNKNOWN } else { facing_word(&codes, 0) },
            facing_ext: if codes.len() > 6 { [FACING_UNKNOWN; 2] } else { [facing_word(&codes, 1), facing_word(&codes, 2)] },
        });
    }
}

/// Pipes, then markers unless VIEWER_NO_DOTS.
pub fn edges_and_dots(ink: &mut Ink, m: &Mesh, topo: &MeshTopo, cx: &mut InkCx) {
    let inc = incidence(m, topo, cx);
    cx.lap.mark("incidence");
    push_pipes(ink, m, topo, cx);
    cx.lap.mark("pipe loop");
    if knobs::no_dots() {
        return;
    }
    push_markers(ink, m, topo, &MarkerCx { cx, inc: &inc });
    cx.lap.mark("markers");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::walk::mesh::{MeshCx, MeshOpts, walk_mesh};
    use crate::app::walk::WalkCx;
    use crate::engine::gpu::arena::ArenaRows;

    /// A box wears twelve pipes and eight markers, every pipe with two known face normals.
    #[test]
    fn box_ink_rows() {
        let mesh = Mesh::create_box(10.0, 20.0, 30.0);
        let mut arena = ArenaRows::default();
        let mut segments = SegRows::default();
        let mut glyphs = GlyphRows::default();
        let mut ink = Ink { seg: &mut segments, glyph: &mut glyphs };
        let cx = WalkCx { vert_base: 50, cloud_px: 0.0, row: 7 };
        walk_mesh(&mut arena, &mut ink, &mesh, &MeshCx { cx: &cx, opts: &MeshOpts::OBJECT });
        assert_eq!(segments.pipes.len(), 12);
        assert_eq!(glyphs.spheres.len(), 8);
        for segment in &segments.pipes {
            assert_ne!(segment.facing, FACING_UNKNOWN);
            assert_eq!(segment.instance_id, 7);
        }
    }
}
