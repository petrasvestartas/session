//! The ink a mesh wears: one pipe per visible edge, one marker per vertex - the SOLID lane.
//! Reads the fused topology and the positions by slot; writes `SegRows.pipes` and
//! `GlyphRows.spheres`, nothing else.

use session_rust::Mesh;
use session_rust::mesh::ColorMode;
use crate::app::knobs;
use crate::engine::gpu::glyphs::GlyphRows;
use crate::engine::gpu::segments::{InkSupport, SegRows};
use crate::engine::gpu::{CylinderSegment, GlyphPoint};
use super::encode::{encode_width, oct16, pack_facing, BLACK, FACING_UNKNOWN};
use super::mesh::{Lap, COPLANAR_DOT, WIREFRAME_BLACK_MIN};
use super::mesh_topology::{MeshTopo, SlotMap};
use super::mesh_faces::FaceSupport;

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
    pub tokens: &'a [Vec<FaceSupport>],
}

/// Edge `i`'s pen width: one entry broadcasts to every edge, an absent one is the 1.0 default.
fn width_at(w: &[f64], i: usize) -> f64 {
    if w.len() == 1 { w[0] } else { w.get(i).copied().unwrap_or(1.0) }
}

/// Width 0 = hidden: a triangulated fill asks for no wireframe.
fn hidden(w: &[f64], i: usize) -> bool {
    width_at(w, i) == 0.0
}

/// The normal in face slot `k` of an edge's pair, borrowed.
fn normal_of(tokens: &[Vec<FaceSupport>], faces: [u32; 2], edge: [usize; 2], side: usize) -> Option<&[f64; 3]> {
    if faces[side] == u32::MAX { return None; }
    tokens[faces[side] as usize].iter().find(|part| part.contains(&edge)).map(|part| &part.normal)
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

/// The physical vertices and segment region requesting support.
struct SupportRequest<'a> {
    region: u32,
    vertices: &'a [usize],
    start: usize,
}

/// Append unique physical face tokens, respecting warped-triangle incidence.
fn push_supports(out: &mut Vec<InkSupport>, faces: &[usize], tokens: &[Vec<FaceSupport>], request: &SupportRequest) {
    for &face in faces {
        for part in &tokens[face] {
            if part.contains(request.vertices) && !out[request.start..].iter().any(|entry| entry.face == part.token && (entry.region == 0 || entry.region == request.region)) {
                out.push(InkSupport { face: part.token, region: request.region });
            }
        }
    }
}

/// Incidence shared by edge and marker support extraction.
struct SupportCx<'a, 'b> {
    cx: &'a InkCx<'b>,
    inc: &'a Incidence,
}

/// The pipe loop: one segment per visible, non-coplanar edge.
fn push_pipes(ink: &mut Ink, m: &Mesh, topo: &MeshTopo, input: &SupportCx) {
    let cx = input.cx;
    let w = m.widths();
    let black_wire = topo.edges.len() >= WIREFRAME_BLACK_MIN;
    ink.seg.pipes.reserve(topo.edges.len());
    for (i, (a, b, col)) in topo.edges.iter().enumerate() {
        let f = topo.edge_faces[i];
        let facing = pack_facing(normal_of(cx.tokens, f, [*a, *b], 0), normal_of(cx.tokens, f, [*a, *b], 1));
        if hidden(w, i) {
            continue;
        }
        // Interior tessellation: a diagonal across a flat region shares two coplanar faces.
        if let (Some(n0), Some(n1)) = (normal_of(cx.tokens, f, [*a, *b], 0), normal_of(cx.tokens, f, [*a, *b], 1)) {
            let dot = n0[0] * n1[0] + n0[1] * n1[1] + n0[2] * n1[2];
            if dot >= COPLANAR_DOT && !knobs::all_edges() {
                continue;
            }
        }
        let support_start = ink.seg.supports.len() as u32;
        let mut adjacent = Vec::new();
        push_faces(&topo.edge_faces, i, &mut adjacent);
        push_supports(&mut ink.seg.supports, &adjacent, cx.tokens, &SupportRequest { region: 0, vertices: &[*a, *b], start: support_start as usize });
        for (end, key) in [a, b].iter().enumerate() {
            let slot = cx.slots.slot(**key);
            let mut faces = Vec::new();
            for &edge in &input.inc.vinc[input.inc.vstart[slot] as usize..input.inc.vstart[slot + 1] as usize] {
                push_faces(&topo.edge_faces, edge as usize, &mut faces);
            }
            push_supports(&mut ink.seg.supports, &faces, cx.tokens, &SupportRequest { region: end as u32 + 1, vertices: &[**key], start: support_start as usize });
        }
        let support_count = ink.seg.supports.len() as u32 - support_start;
        ink.seg.pipes.push(CylinderSegment {
            p0: cx.vpos[cx.slots.slot(*a)],
            radius: encode_width(width_at(w, i)),
            p1: cx.vpos[cx.slots.slot(*b)],
            instance_id: cx.row,
            color: if black_wire { BLACK } else { *col },
            facing,
            support_start,
            support_count,
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

/// The marker loop: one glyph per vertex with a visible edge, carrying up to six incident
/// face normals (widest edge's pair first) so the disc hugs every face at a corner.
fn push_markers(ink: &mut Ink, m: &Mesh, topo: &MeshTopo, input: &SupportCx) {
    let cx = input.cx;
    let inc = input.inc;
    let pc = m.get_pointcolors();
    let dots_colored = m.color_mode == ColorMode::POINTCOLORS && pc.len() == m.number_of_vertices();
    let nv = cx.vpos.len();
    let mut fkeys: Vec<usize> = Vec::new();
    let mut codes: Vec<u32> = Vec::new();
    let vertex_keys = m.vertices();
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
            for part in &cx.tokens[*fk] {
                if part.contains(&[vertex_keys[i]]) && let Some(code) = oct16(&part.normal) && !codes.contains(&code) { codes.push(code); }
            }
        }
        let support_start = ink.glyph.supports.len() as u32;
        push_supports(&mut ink.glyph.supports, &fkeys, cx.tokens, &SupportRequest { region: 0, vertices: &[vertex_keys[i]], start: support_start as usize });
        let support_count = ink.glyph.supports.len() as u32 - support_start;
        ink.glyph.spheres.push(GlyphPoint {
            center: cx.vpos[i],
            radius: encode_width(vw),
            color: if dots_colored { pc[i].to_f32() } else { [0.1, 0.1, 0.1, 1.0] },
            instance_id: cx.row,
            // A truncated normal list cannot prove every incident face points away.
            facing: if codes.len() > 6 { FACING_UNKNOWN } else { facing_word(&codes, 0) },
            facing_ext: if codes.len() > 6 { [FACING_UNKNOWN; 2] } else { [facing_word(&codes, 1), facing_word(&codes, 2)] },
            support_start,
            support_count,
            _pad: [0; 2],
        });
    }
}

/// Pipes, then markers unless VIEWER_NO_DOTS.
pub fn edges_and_dots(ink: &mut Ink, m: &Mesh, topo: &MeshTopo, cx: &mut InkCx) {
    let inc = incidence(m, topo, cx);
    cx.lap.mark("incidence");
    push_pipes(ink, m, topo, &SupportCx { cx, inc: &inc });
    cx.lap.mark("pipe loop");
    if knobs::no_dots() {
        return;
    }
    push_markers(ink, m, topo, &SupportCx { cx, inc: &inc });
    cx.lap.mark("markers");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::walk::mesh::{MeshCx, MeshOpts, walk_mesh};
    use crate::app::walk::WalkCx;
    use crate::engine::gpu::arena::ArenaRows;

    /// Endpoint-only faces never become unconditional support for an entire box edge.
    #[test]
    fn cube_supports_keep_endpoint_regions() {
        let mesh = Mesh::create_box(10.0, 20.0, 30.0);
        let mut arena = ArenaRows::default();
        let mut segments = SegRows::default();
        let mut glyphs = GlyphRows::default();
        let mut ink = Ink { seg: &mut segments, glyph: &mut glyphs };
        let cx = WalkCx { vert_base: 50, face_base: 100, cloud_px: 0.0, row: 7 };
        walk_mesh(&mut arena, &mut ink, &mesh, &MeshCx { cx: &cx, opts: &MeshOpts::OBJECT });
        assert_eq!(segments.pipes.len(), 12);
        for segment in &segments.pipes {
            let start = segment.support_start as usize;
            let supports = &segments.supports[start..start + segment.support_count as usize];
            assert_eq!(supports.iter().filter(|entry| entry.region == 0).count(), 2);
            assert_eq!(supports.iter().filter(|entry| entry.region == 1).count(), 1);
            assert_eq!(supports.iter().filter(|entry| entry.region == 2).count(), 1);
            for entry in supports { assert!(arena.face_ids.contains(&entry.face)); }
        }
        for marker in &glyphs.spheres { assert_eq!(marker.support_count, 3); }
    }
}
