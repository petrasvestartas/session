use super::encode::{BLACK, FACING_UNKNOWN, encode_width, oct16, pack_facing};
use super::mesh::{COPLANAR_DOT, CREASE_COS, Lap, WIREFRAME_BLACK_MIN};
use super::mesh_topology::{MeshTopo, SlotMap};
use crate::app::knobs;
use crate::engine::gpu::glyphs::GlyphRows;
use crate::engine::gpu::segments::SegRows;
use crate::engine::gpu::{CylinderSegment, GlyphPoint};
use session_rust::Mesh;
use session_rust::mesh::ColorMode;

/// Where a mesh's edges and vertex dots go.
pub struct Ink<'a> {
    pub seg: &'a mut SegRows,     // edge pipes
    pub glyph: &'a mut GlyphRows, // vertex spheres
}

/// What the ink pass needs from the face pass.
pub struct InkCx<'a> {
    pub row: u32,             // object row
    pub vpos: &'a [[f32; 3]], // vertex positions by slot
    pub slots: &'a SlotMap,   // vertex key to slot
    pub smooth: bool,         // only borders and creases are ink
    pub lap: &'a mut Lap,     // profiling timer
}

/// Pen width of edge `i`; one entry applies to all.
fn width_at(w: &[f64], i: usize) -> f64 {
    if w.len() == 1 {
        w[0]
    } else {
        w.get(i).copied().unwrap_or(1.0)
    }
}

/// Width 0 hides the edge.
fn hidden(w: &[f64], i: usize) -> bool {
    width_at(w, i) == 0.0
}

/// Normal of one of an edge's two faces.
fn normal_of(topo: &MeshTopo, faces: [u32; 2], side: usize) -> Option<[f64; 3]> {
    if faces[side] == u32::MAX {
        return None;
    }

    topo.normals[faces[side] as usize]
}

/// The two outward normals of edge `ei`.
fn edge_normals(topo: &MeshTopo, ei: usize) -> (Option<[f64; 3]>, Option<[f64; 3]>) {
    let f = topo.edge_faces[ei];
    let n0 = normal_of(topo, f, 0);
    let n1 = normal_of(topo, f, 1);

    if topo.opposed[ei] {
        return (n0, n1);
    }

    (n0, n1.map(reversed_normal)) // badly wound: flip the second
}

/// Dot product of two vectors.
fn dot3(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// True for a border or crease edge of a smooth mesh.
fn smooth_feature(topo: &MeshTopo, ei: usize, pair: (Option<[f64; 3]>, Option<[f64; 3]>)) -> bool {
    if topo.edge_faces[ei][1] == u32::MAX {
        return true; // border
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

/// Two normal codes packed into word `k`.
fn facing_word(codes: &[u32], k: usize) -> u32 {
    match (codes.get(2 * k).copied(), codes.get(2 * k + 1).copied()) {
        (Some(a), b) => {
            let v = a | b.unwrap_or(a) << 16;

            if v == FACING_UNKNOWN { v ^ 1 } else { v }
        }
        _ => FACING_UNKNOWN,
    }
}

/// One pipe per drawn edge.
fn push_pipes(ink: &mut Ink, m: &Mesh, topo: &MeshTopo, cx: &InkCx) {
    let w = m.widths();
    let black_wire = topo.edges.len() >= WIREFRAME_BLACK_MIN; // dense mesh: black edges
    ink.seg.pipes.reserve(topo.edges.len());

    for (i, (a, b, col)) in topo.edges.iter().enumerate() {
        let (na, nb) = edge_normals(topo, i);
        let facing = pack_facing(na.as_ref(), nb.as_ref());

        if hidden(w, i) {
            continue;
        }

        // skip a diagonal inside a flat region
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
            .push(if cx.smooth { u32::MAX } else { i as u32 }); // edge id for picking
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

/// Which edges touch each vertex.
struct Incidence {
    best: Vec<(f64, usize)>, // per vertex: widest edge (width, index)
    vstart: Vec<u32>,        // per vertex: start into `vinc`
    vinc: Vec<u32>,          // edge indices, grouped by vertex
}

/// Build the vertex to edge tables.
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

    // count edges per vertex, then prefix sum
    let mut vstart = vec![0u32; nv + 1];

    for (a, b, _) in topo.edges.iter() {
        vstart[cx.slots.slot(*a) + 1] += 1;
        vstart[cx.slots.slot(*b) + 1] += 1;
    }

    for i in 0..nv {
        vstart[i + 1] += vstart[i];
    }

    // fill each vertex's edge list
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

/// What the marker loop reads.
struct MarkerCx<'a, 'b> {
    cx: &'a InkCx<'b>,  // ink context
    inc: &'a Incidence, // vertex to edge tables
}

/// One dot per vertex with a visible edge.
fn push_markers(ink: &mut Ink, m: &Mesh, topo: &MeshTopo, input: &MarkerCx) {
    let (cx, inc) = (input.cx, input.inc);
    let pc = m.get_pointcolors();
    let dots_colored = m.color_mode == ColorMode::POINTCOLORS && pc.len() == m.number_of_vertices(); // per-vertex colours
    let nv = cx.vpos.len();
    let mut fkeys: Vec<usize> = Vec::new(); // faces around one vertex
    let mut codes: Vec<u32> = Vec::new(); // packed normals of those faces
    ink.glyph.spheres.reserve(nv);

    for (i, &(vw, ei)) in inc.best.iter().enumerate().take(nv) {
        if vw == f64::NEG_INFINITY {
            continue; // no visible edge
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
            // more than six faces: always draw
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

/// Edges, then vertex dots.
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

/// The opposite direction.
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
            attributes: false,
        };
        walk_mesh(&mut arena, &mut ink, mesh, &MeshCx { cx: &cx, opts });
        segments.pipes.len()
    }

    /// A slightly bulged 3x3 quad grid: 24 edges, 12 on the border.
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

    /// Two quads at a right angle: 7 edges, one a fold.
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

    /// A box has 12 edges and 8 dots.
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
            attributes: false,
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

    /// A smooth mesh draws only borders and creases.
    #[test]
    fn smooth_mesh_inks_borders_and_creases_only() {
        let grid = bulged_grid();
        assert_eq!(walk_pipes(&grid, &MeshOpts::OBJECT), 24);
        assert_eq!(walk_pipes(&grid, &MeshOpts::SURFACE), 12);
        assert_eq!(walk_pipes(&folded_pair(), &MeshOpts::SURFACE), 7);
    }
}
