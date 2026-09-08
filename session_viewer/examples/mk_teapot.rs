// The Utah teapot as a BRep: Newell's 32 bicubic Bezier patches from the ten GLUT input
// patches - rim, body, lid and bottom turned through the four quadrants, handle and spout
// mirrored in y - with every shared boundary one edge, every exactly degenerate row a
// degenerated edge, and one shell per connected patch group. The hard case for the BRep walk:
// 32 grid faces whose seams are sampled by both sides, a pole, a tip, a 0.002 loop at the knob,
// and open shells. Scaled by SCALE so it sits with the mixed solids; the original unit is a
// few teapot-widths.
//
// cargo run --release --target x86_64-unknown-linux-gnu --example mk_teapot -- <out.pb>
use session_rust::brep::{BRep, BRepOrientation, BRepRef};
use session_rust::{Color, NurbsCurve, NurbsSurface, Point, Session};
use session_viewer::app::walk::brep::QUALITY;
use session_viewer::app::walk::brep_edges::edge_chains;

/// Teapot units to millimetres: the body is 4 units across, so 100 makes it 400 mm, the
/// size of the mixed scene's box.
const SCALE: f64 = 100.0;

const F: BRepOrientation = BRepOrientation::Forward;
const R: BRepOrientation = BRepOrientation::Reversed;

/// patchdata_teapot of the GLUT sources: the ten input patches as indices into POINTS.
const PATCHES: [[usize; 16]; 10] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [
        12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
    ],
    [
        24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
    ],
    [
        40, 41, 42, 40, 43, 44, 45, 46, 47, 47, 47, 47, 48, 49, 50, 51,
    ],
    [
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
    ],
    [
        64, 64, 64, 64, 65, 66, 67, 68, 69, 70, 71, 72, 39, 38, 37, 36,
    ],
    [
        73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88,
    ],
    [
        85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100,
    ],
    [
        101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    ],
    [
        113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128,
    ],
];

/// cpdata_teapot of the GLUT sources: Newell's 129 control points, in teapot units.
const POINTS: [[f64; 3]; 129] = [
    [1.40000, 0.00000, 2.40000],
    [1.40000, -0.78400, 2.40000],
    [0.78400, -1.40000, 2.40000],
    [0.00000, -1.40000, 2.40000],
    [1.33750, 0.00000, 2.53125],
    [1.33750, -0.74900, 2.53125],
    [0.74900, -1.33750, 2.53125],
    [0.00000, -1.33750, 2.53125],
    [1.43750, 0.00000, 2.53125],
    [1.43750, -0.80500, 2.53125],
    [0.80500, -1.43750, 2.53125],
    [0.00000, -1.43750, 2.53125],
    [1.50000, 0.00000, 2.40000],
    [1.50000, -0.84000, 2.40000],
    [0.84000, -1.50000, 2.40000],
    [0.00000, -1.50000, 2.40000],
    [1.75000, 0.00000, 1.87500],
    [1.75000, -0.98000, 1.87500],
    [0.98000, -1.75000, 1.87500],
    [0.00000, -1.75000, 1.87500],
    [2.00000, 0.00000, 1.35000],
    [2.00000, -1.12000, 1.35000],
    [1.12000, -2.00000, 1.35000],
    [0.00000, -2.00000, 1.35000],
    [2.00000, 0.00000, 0.90000],
    [2.00000, -1.12000, 0.90000],
    [1.12000, -2.00000, 0.90000],
    [0.00000, -2.00000, 0.90000],
    [2.00000, 0.00000, 0.45000],
    [2.00000, -1.12000, 0.45000],
    [1.12000, -2.00000, 0.45000],
    [0.00000, -2.00000, 0.45000],
    [1.50000, 0.00000, 0.22500],
    [1.50000, -0.84000, 0.22500],
    [0.84000, -1.50000, 0.22500],
    [0.00000, -1.50000, 0.22500],
    [1.50000, 0.00000, 0.15000],
    [1.50000, -0.84000, 0.15000],
    [0.84000, -1.50000, 0.15000],
    [0.00000, -1.50000, 0.15000],
    [0.00000, 0.00000, 3.15000],
    [0.00000, -0.00200, 3.15000],
    [0.00200, 0.00000, 3.15000],
    [0.80000, 0.00000, 3.15000],
    [0.80000, -0.45000, 3.15000],
    [0.45000, -0.80000, 3.15000],
    [0.00000, -0.80000, 3.15000],
    [0.00000, 0.00000, 2.85000],
    [0.20000, 0.00000, 2.70000],
    [0.20000, -0.11200, 2.70000],
    [0.11200, -0.20000, 2.70000],
    [0.00000, -0.20000, 2.70000],
    [0.40000, 0.00000, 2.55000],
    [0.40000, -0.22400, 2.55000],
    [0.22400, -0.40000, 2.55000],
    [0.00000, -0.40000, 2.55000],
    [1.30000, 0.00000, 2.55000],
    [1.30000, -0.72800, 2.55000],
    [0.72800, -1.30000, 2.55000],
    [0.00000, -1.30000, 2.55000],
    [1.30000, 0.00000, 2.40000],
    [1.30000, -0.72800, 2.40000],
    [0.72800, -1.30000, 2.40000],
    [0.00000, -1.30000, 2.40000],
    [0.00000, 0.00000, 0.00000],
    [0.00000, -1.42500, 0.00000],
    [0.79800, -1.42500, 0.00000],
    [1.42500, -0.79800, 0.00000],
    [1.42500, 0.00000, 0.00000],
    [0.00000, -1.50000, 0.07500],
    [0.84000, -1.50000, 0.07500],
    [1.50000, -0.84000, 0.07500],
    [1.50000, 0.00000, 0.07500],
    [-1.60000, 0.00000, 2.02500],
    [-1.60000, -0.30000, 2.02500],
    [-1.50000, -0.30000, 2.25000],
    [-1.50000, 0.00000, 2.25000],
    [-2.30000, 0.00000, 2.02500],
    [-2.30000, -0.30000, 2.02500],
    [-2.50000, -0.30000, 2.25000],
    [-2.50000, 0.00000, 2.25000],
    [-2.70000, 0.00000, 2.02500],
    [-2.70000, -0.30000, 2.02500],
    [-3.00000, -0.30000, 2.25000],
    [-3.00000, 0.00000, 2.25000],
    [-2.70000, 0.00000, 1.80000],
    [-2.70000, -0.30000, 1.80000],
    [-3.00000, -0.30000, 1.80000],
    [-3.00000, 0.00000, 1.80000],
    [-2.70000, 0.00000, 1.57500],
    [-2.70000, -0.30000, 1.57500],
    [-3.00000, -0.30000, 1.35000],
    [-3.00000, 0.00000, 1.35000],
    [-2.50000, 0.00000, 1.12500],
    [-2.50000, -0.30000, 1.12500],
    [-2.65000, -0.30000, 0.93750],
    [-2.65000, 0.00000, 0.93750],
    [-2.00000, 0.00000, 0.90000],
    [-2.00000, -0.30000, 0.90000],
    [-1.90000, -0.30000, 0.60000],
    [-1.90000, 0.00000, 0.60000],
    [1.70000, 0.00000, 1.42500],
    [1.70000, -0.66000, 1.42500],
    [1.70000, -0.66000, 0.60000],
    [1.70000, 0.00000, 0.60000],
    [2.60000, 0.00000, 1.42500],
    [2.60000, -0.66000, 1.42500],
    [3.10000, -0.66000, 0.82500],
    [3.10000, 0.00000, 0.82500],
    [2.30000, 0.00000, 2.10000],
    [2.30000, -0.25000, 2.10000],
    [2.40000, -0.25000, 2.02500],
    [2.40000, 0.00000, 2.02500],
    [2.70000, 0.00000, 2.40000],
    [2.70000, -0.25000, 2.40000],
    [3.30000, -0.25000, 2.40000],
    [3.30000, 0.00000, 2.40000],
    [2.80000, 0.00000, 2.47500],
    [2.80000, -0.25000, 2.47500],
    [3.52500, -0.25000, 2.49375],
    [3.52500, 0.00000, 2.49375],
    [2.90000, 0.00000, 2.47500],
    [2.90000, -0.15000, 2.47500],
    [3.45000, -0.15000, 2.51250],
    [3.45000, 0.00000, 2.51250],
    [2.80000, 0.00000, 2.40000],
    [2.80000, -0.15000, 2.40000],
    [3.20000, -0.15000, 2.40000],
    [3.20000, 0.00000, 2.40000],
];

/// A point of the data turned `q` quarter turns about z and scaled: (x, y) -> (-y, x) is
/// exact in floating point, so patches that share a boundary share its coordinates bit for
/// bit - up to the sign of zero, which `bits` folds away.
fn turned(p: [f64; 3], q: usize) -> Point {
    let (mut x, mut y) = (p[0], p[1]);
    for _ in 0..q {
        (x, y) = (-y, x);
    }
    Point::new(x * SCALE, y * SCALE, p[2] * SCALE)
}

/// The 32 patches as 4 x 4 control grids, u slowest. A mirrored copy (y negated) would turn
/// its normal inward, so it is transposed as well, which turns the normal back out.
fn patches() -> Vec<[Point; 16]> {
    let mut out = Vec::with_capacity(32);
    for (k, idx) in PATCHES.iter().enumerate() {
        let grid: Vec<[f64; 3]> = idx.iter().map(|&i| POINTS[i]).collect();
        if k < 6 {
            for q in 0..4 {
                out.push(std::array::from_fn(|n| turned(grid[n], q)));
            }
        } else {
            out.push(std::array::from_fn(|n| turned(grid[n], 0)));
            out.push(std::array::from_fn(|n| {
                let (iu, iv) = (n / 4, n % 4);
                let p = grid[iv * 4 + iu];
                Point::new(p[0] * SCALE, -p[1] * SCALE, p[2] * SCALE)
            }));
        }
    }
    out
}

/// One coordinate as bits, with -0.0 folded onto 0.0: the mirror negates y, and the seam
/// the two halves of the handle and the spout share sits at y = 0, where -0.0 is the same
/// point but not the same bits. The quarter turn (x, y) -> (-y, x) makes them too.
fn bits(x: f64) -> u64 {
    f64::to_bits(if x == 0.0 { 0.0 } else { x })
}

/// A point's coordinates as bits: the matching key, exact by construction.
fn key(p: &Point) -> [u64; 3] {
    [bits(p[0]), bits(p[1]), bits(p[2])]
}

/// One side of a patch: its four control points in the +parameter direction, which
/// `iso_curve` argument draws it, and the corner parameters its pcurve runs between.
struct Side {
    cvs: [Point; 4],
    iso: (usize, f64),
    from: (f64, f64),
    to: (f64, f64),
}

/// The four sides of a 4 x 4 grid in wire order: v = v0 along +u, u = u1 along +v, v = v1
/// along +u (the wire walks it backwards), u = u0 along +v (backwards too).
fn sides(g: &[Point; 16], dom: ((f64, f64), (f64, f64))) -> [Side; 4] {
    let ((u0, u1), (v0, v1)) = dom;
    let at = |iu: usize, iv: usize| g[iu * 4 + iv].clone();
    [
        Side {
            cvs: [at(0, 0), at(1, 0), at(2, 0), at(3, 0)],
            iso: (0, v0),
            from: (u0, v0),
            to: (u1, v0),
        },
        Side {
            cvs: [at(3, 0), at(3, 1), at(3, 2), at(3, 3)],
            iso: (1, u1),
            from: (u1, v0),
            to: (u1, v1),
        },
        Side {
            cvs: [at(0, 3), at(1, 3), at(2, 3), at(3, 3)],
            iso: (0, v1),
            from: (u0, v1),
            to: (u1, v1),
        },
        Side {
            cvs: [at(0, 0), at(0, 1), at(0, 2), at(0, 3)],
            iso: (1, u0),
            from: (u0, v0),
            to: (u0, v1),
        },
    ]
}

/// A straight pcurve between two corners of the domain.
fn uv_line(a: (f64, f64), b: (f64, f64)) -> NurbsCurve {
    NurbsCurve::create(
        false,
        1,
        &[Point::new(a.0, a.1, 0.0), Point::new(b.0, b.1, 0.0)],
    )
}

/// The builder's memory across patches: corner points to vertex indices and boundary rows
/// to the edge that already carries them.
struct Shared {
    vertices: Vec<([u64; 3], usize)>,
    edges: Vec<([[u64; 3]; 4], usize)>,
}

impl Shared {
    /// The vertex at `p`, made on first sight.
    fn vertex(&mut self, b: &mut BRep, p: &Point) -> usize {
        let k = key(p);
        if let Some((_, v)) = self.vertices.iter().find(|(kk, _)| *kk == k) {
            return *v;
        }
        let v = b.add_vertex(p, 0.0);
        self.vertices.push((k, v));
        v
    }

    /// The edge carrying these four control points, and whether it runs the same way: +1
    /// when the stored row reads forwards, -1 when it reads backwards, None when new.
    fn edge(&self, cvs: &[Point; 4]) -> Option<(usize, i32)> {
        let fwd: [[u64; 3]; 4] = std::array::from_fn(|i| key(&cvs[i]));
        let rev: [[u64; 3]; 4] = std::array::from_fn(|i| key(&cvs[3 - i]));
        for (k, e) in &self.edges {
            if *k == fwd {
                return Some((*e, 1));
            }
            if *k == rev {
                return Some((*e, -1));
            }
        }
        None
    }
}

/// One side into the BRep for surface `si`: the edge (shared, new, or degenerated), its
/// pcurve on this face in the edge's own direction, and the wire use. `walk` is +1 when the
/// wire walks this side along +parameter, -1 backwards.
fn add_side(b: &mut BRep, sh: &mut Shared, s: &Side, ctx: (usize, i32)) -> BRepRef {
    let (si, walk) = ctx;
    let degenerate = s.cvs.iter().all(|p| key(p) == key(&s.cvs[0]));
    if degenerate {
        let v = sh.vertex(b, &s.cvs[0]) as i32;
        let e = b.add_edge(-1, v, v);
        let c = b.add_curve_2d(&uv_line(s.from, s.to)) as i32;
        b.add_pcurve(e, si, c, -1);
        return BRepRef::new(e as i32, if walk > 0 { F } else { R });
    }
    let (e, dir) = match sh.edge(&s.cvs) {
        Some(found) => found,
        None => {
            let v0 = sh.vertex(b, &s.cvs[0]) as i32;
            let v1 = sh.vertex(b, &s.cvs[3]) as i32;
            let crv = b.m_surfaces[si]
                .iso_curve(s.iso.0, s.iso.1)
                .expect("iso curve");
            let c3 = b.add_curve_3d(&crv) as i32;
            let e = b.add_edge(c3, v0, v1);
            sh.edges.push((std::array::from_fn(|i| key(&s.cvs[i])), e));
            (e, 1)
        }
    };
    // SameParameter: the pcurve follows the edge's 3D direction, not the side's.
    let c = if dir > 0 {
        uv_line(s.from, s.to)
    } else {
        uv_line(s.to, s.from)
    };
    let ci = b.add_curve_2d(&c) as i32;
    b.add_pcurve(e, si, ci, -1);
    BRepRef::new(e as i32, if walk * dir > 0 { F } else { R })
}

/// Faces sharing a non-degenerated edge belong to one shell: union-find over the faces.
fn components(b: &BRep) -> Vec<Vec<usize>> {
    let nf = b.m_faces.len();
    let mut parent: Vec<usize> = (0..nf).collect();
    fn root(parent: &mut [usize], i: usize) -> usize {
        let mut i = i;
        while parent[i] != i {
            parent[i] = parent[parent[i]];
            i = parent[i];
        }
        i
    }
    for ei in 0..b.m_edges.len() {
        if b.m_edges[ei].degenerated {
            continue;
        }
        let uses = b.edge_faces(ei);
        for w in uses.windows(2) {
            let (a, c) = (
                root(&mut parent, w[0].index as usize),
                root(&mut parent, w[1].index as usize),
            );
            parent[a] = c;
        }
    }
    let mut groups: Vec<Vec<usize>> = Vec::new();
    let mut roots: Vec<usize> = Vec::new();
    for f in 0..nf {
        let r = root(&mut parent, f);
        match roots.iter().position(|&x| x == r) {
            Some(i) => groups[i].push(f),
            None => {
                roots.push(r);
                groups.push(vec![f]);
            }
        }
    }
    groups
}

/// Six times the signed volume the faces of one group enclose, summed over their triangles
/// about the origin: negative means the group's normals point inward and the shell is added
/// reversed. An open group still carries the sign of the side it mostly faces.
fn signed_volume(meshes: &[session_rust::Mesh], group: &[usize]) -> f64 {
    let mut six_v = 0.0;
    for &fi in group {
        let m = &meshes[fi];
        for verts in m.face.values() {
            if verts.len() < 3 {
                continue;
            }
            let p = |k: usize| m.vertex[&verts[k]].position();
            let (a, b, c) = (p(0), p(1), p(2));
            six_v += a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])
                + a[2] * (b[0] * c[1] - b[1] * c[0]);
        }
    }
    six_v
}

/// The teapot BRep: patches to faces with shared edges, then one shell per group, reversed
/// when its volume comes out negative, and a solid for every closed shell.
fn teapot() -> BRep {
    let mut b = BRep::new();
    b.name = "teapot".to_string();
    let mut sh = Shared {
        vertices: Vec::new(),
        edges: Vec::new(),
    };
    for g in patches() {
        let srf = NurbsSurface::create(false, false, 3, 3, 4, 4, &g).expect("bicubic patch");
        let dom = (
            srf.domain(0).expect("u domain"),
            srf.domain(1).expect("v domain"),
        );
        let si = b.add_surface(&srf);
        let walks = [1, 1, -1, -1];
        let mut uses = Vec::with_capacity(4);
        for (s, walk) in sides(&g, dom).iter().zip(walks) {
            uses.push(add_side(&mut b, &mut sh, s, (si, walk)));
        }
        let wi = b.add_wire(&uses);
        b.add_face(si as i32, &[BRepRef::new(wi as i32, F)], 0.0);
    }
    let meshes = b.face_meshes();
    for group in components(&b) {
        let o = if signed_volume(&meshes, &group) < 0.0 {
            R
        } else {
            F
        };
        let refs: Vec<BRepRef> = group.iter().map(|&f| BRepRef::new(f as i32, o)).collect();
        let s = b.add_shell(&refs);
        if b.is_closed(s) {
            b.add_solid(&[BRepRef::new(s as i32, F)]);
        }
    }
    b.surfacecolor = Color::new(0.93, 0.90, 0.82, 1.0);
    b
}

/// What the walk will find: how many edges have a chain, how many are degenerated, and how
/// many would fall back to the curve.
fn census(b: &BRep) -> (usize, usize, usize) {
    let fms = b.face_meshes_q(Some(QUALITY));
    let chains = edge_chains(b, &fms);
    let chained = chains.iter().filter(|c| c.is_some()).count();
    let degenerated = b.m_edges.iter().filter(|e| e.degenerated).count();
    (
        chained,
        degenerated,
        b.m_edges.len() - chained - degenerated,
    )
}

fn main() {
    let out = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "target/teapot.pb".into());
    let b = teapot();
    let (chained, degenerated, unchained) = census(&b);
    println!(
        "teapot: faces {} edges {} vertices {} shells {} chained {chained} degenerated {degenerated} unchained {unchained}",
        b.m_faces.len(),
        b.m_edges.len(),
        b.m_vertices.len(),
        b.m_shells.len()
    );
    let mut s = Session::new("teapot");
    s.add_brep(b, None);
    s.pb_dump(&out);
    println!("wrote {out}");
}
