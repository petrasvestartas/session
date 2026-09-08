//! The ink a mesh wears: one pipe per visible edge, one marker per vertex - the SOLID lane.
//! Reads the fused topology and the positions by slot; writes `SegRows.pipes` and
//! `GlyphRows.spheres`, nothing else.

use super::encode::{BLACK, FACING_UNKNOWN, encode_width, oct16, pack_facing};
use super::mesh::{COPLANAR_DOT, CREASE_COS, Lap, WIREFRAME_BLACK_MIN};
use super::mesh_topology::{MeshTopo, SlotMap};
use crate::app::knobs;
use crate::engine::gpu::glyphs::GlyphRows;
use crate::engine::gpu::segments::SegRows;
use crate::engine::gpu::{CylinderSegment, GlyphPoint};
use session_rust::Mesh;
use session_rust::mesh::ColorMode;

/// The two ink lanes a mesh reaches: pipes for its edges, spheres for its vertices.
pub struct Ink<'a> {
    pub seg: &'a mut SegRows,
    pub glyph: &'a mut GlyphRows,
}

/// What the ink pass needs from the face pass: the object row, the f32 positions by slot,
/// the key -> slot map, whether the mesh is a smooth tessellation, and the profiling clock.
pub struct InkCx<'a> {
    pub row: u32,
    pub vpos: &'a [[f32; 3]],
    pub slots: &'a SlotMap,
    /// The mesh samples a smooth surface, so only its borders and creases are ink.
    pub smooth: bool,
    pub lap: &'a mut Lap,
}

/// Edge `i`'s pen width: one entry broadcasts to every edge, an absent one is the 1.0 default.
fn width_at(w: &[f64], i: usize) -> f64 {
    if w.len() == 1 {
        w[0]
    } else {
        w.get(i).copied().unwrap_or(1.0)
    }
}

/// Width 0 = hidden: a triangulated fill asks for no wireframe.
fn hidden(w: &[f64], i: usize) -> bool {
    width_at(w, i) == 0.0
}

/// The normal of the face in slot `side` of an edge's pair; None past a border.
fn normal_of(topo: &MeshTopo, faces: [u32; 2], side: usize) -> Option<[f64; 3]> {
    if faces[side] == u32::MAX {
        return None;
    }
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
    (n0, n1.map(reversed_normal))
}

/// The cosine between two unit face normals.
fn dot3(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// On a smooth tessellation only two edges are geometry: the BORDER, where the surface ends,
/// and the CREASE, where it genuinely folds. Everything between is the sampling grid, and
/// drawing it draws the mesher's choices instead of the shape. A pair with an unknown normal
/// is kept: a degenerate face proves nothing either way.
fn smooth_feature(topo: &MeshTopo, ei: usize, pair: (Option<[f64; 3]>, Option<[f64; 3]>)) -> bool {
    if topo.edge_faces[ei][1] == u32::MAX {
        return true;
    }
    match pair {
        (Some(n0), Some(n1)) => dot3(&n0, &n1) < CREASE_COS,
        _ => true,
    }
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

/// The pipe loop: one segment per visible, non-coplanar edge - and on a smooth mesh, only
/// where that edge is a border or a crease.
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
        if let (Some(n0), Some(n1)) = (na, nb)
            && dot3(&n0, &n1) >= COPLANAR_DOT
            && !knobs::all_edges()
        {
            continue;
        }
        if cx.smooth && !smooth_feature(topo, i, (na, nb)) {
            continue;
        }
        ink.seg
            .pipe_ids
            .push(if cx.smooth { u32::MAX } else { i as u32 });
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
            if let Some(n) = topo.normals[*fk]
                && let Some(code) = oct16(&n)
                && !codes.contains(&code)
            {
                codes.push(code);
            }
        }
        ink.glyph.spheres.push(GlyphPoint {
            center: cx.vpos[i],
            radius: encode_width(vw),
            color: if dots_colored {
                pc[i].to_f32()
            } else {
                [0.1, 0.1, 0.1, 1.0]
            },
            instance_id: cx.row,
            // A truncated normal list cannot prove every incident face points away.
            facing: if codes.len() > 6 {
                FACING_UNKNOWN
            } else {
                facing_word(&codes, 0)
            },
            facing_ext: if codes.len() > 6 {
                [FACING_UNKNOWN; 2]
            } else {
                [facing_word(&codes, 1), facing_word(&codes, 2)]
            },
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

/// Reverse the second incident face normal to match the corrected winding.
fn reversed_normal(n: [f64; 3]) -> [f64; 3] {
    [-n[0], -n[1], -n[2]]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::walk::WalkCx;
    use crate::app::walk::mesh::{MeshCx, MeshOpts, walk_mesh};
    use crate::engine::gpu::arena::ArenaRows;
    use session_rust::Point;

    /// How many pipes one mesh pushes when walked under `opts`.
    fn walk_pipes(mesh: &Mesh, opts: &MeshOpts) -> usize {
        let mut arena = ArenaRows::default();
        let mut segments = SegRows::default();
        let mut glyphs = GlyphRows::default();
        let mut ink = Ink {
            seg: &mut segments,
            glyph: &mut glyphs,
        };
        let cx = WalkCx {
            vert_base: 0,
            cloud_px: 0.0,
            row: 0,
        };
        walk_mesh(&mut arena, &mut ink, mesh, &MeshCx { cx: &cx, opts });
        segments.pipes.len()
    }

    /// A 3x3 grid of quads on the bulge z = 2e-4 * (x^2 + y^2), 100 mm apart: 24 edges, of
    /// which 12 are border. Adjacent facets turn a couple of degrees - far too little for the
    /// packed 16-bit normals to tell apart, which is what put these seams on screen.
    fn bulged_grid() -> Mesh {
        let mut points = Vec::with_capacity(16);
        for i in 0..4 {
            for j in 0..4 {
                let (x, y) = (i as f64 * 100.0, j as f64 * 100.0);
                points.push(Point::new(x, y, 2e-4 * (x * x + y * y)));
            }
        }
        let mut faces = Vec::with_capacity(9);
        for i in 0..3 {
            for j in 0..3 {
                let k = i * 4 + j;
                faces.push(vec![k, k + 4, k + 5, k + 1]);
            }
        }
        Mesh::from_vertices_and_faces(points, faces)
    }

    /// Two quads meeting at a right angle over the shared edge (1, 2): 7 edges, 6 of them
    /// border and the seventh a fold no threshold can call sampling.
    fn folded_pair() -> Mesh {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(100.0, 0.0, 0.0),
            Point::new(100.0, 100.0, 0.0),
            Point::new(0.0, 100.0, 0.0),
            Point::new(100.0, 0.0, 100.0),
            Point::new(100.0, 100.0, 100.0),
        ];
        Mesh::from_vertices_and_faces(points, vec![vec![0, 1, 2, 3], vec![1, 4, 5, 2]])
    }

    /// A box wears twelve pipes and eight markers, every pipe with two known face normals.
    #[test]
    fn box_ink_rows() {
        let mesh = Mesh::create_box(10.0, 20.0, 30.0);
        let mut arena = ArenaRows::default();
        let mut segments = SegRows::default();
        let mut glyphs = GlyphRows::default();
        let mut ink = Ink {
            seg: &mut segments,
            glyph: &mut glyphs,
        };
        let cx = WalkCx {
            vert_base: 50,
            cloud_px: 0.0,
            row: 7,
        };
        walk_mesh(
            &mut arena,
            &mut ink,
            &mesh,
            &MeshCx {
                cx: &cx,
                opts: &MeshOpts::OBJECT,
            },
        );
        assert_eq!(segments.pipes.len(), 12);
        assert_eq!(glyphs.spheres.len(), 8);
        for segment in &segments.pipes {
            assert_ne!(segment.facing, FACING_UNKNOWN);
            assert_eq!(segment.instance_id, 7);
        }
    }

    /// A smooth tessellation inks its border and its creases, nothing else: the bulged grid
    /// keeps all 24 edges as an authored OBJECT and only its 12 border edges as a MODEL, and
    /// the folded pair keeps its 6 border edges plus the fold.
    #[test]
    fn smooth_mesh_inks_borders_and_creases_only() {
        let grid = bulged_grid();
        assert_eq!(walk_pipes(&grid, &MeshOpts::OBJECT), 24);
        assert_eq!(walk_pipes(&grid, &MeshOpts::MODEL), 12);
        assert_eq!(walk_pipes(&folded_pair(), &MeshOpts::MODEL), 7);
    }
}
