use super::upload::Upload;
use session_rust::{AABB, Xform};
use std::rc::Rc;

/// Points that can be extreme in some direction: every hull corner of a set, a few inner points at most.
pub type Hull = Rc<[[f32; 3]]>;

/// Least signed volume, in the unit-scaled frame, that puts a point above a face.
const ABOVE: f64 = 1e-12;

/// Most extreme points kept, 96 KB; a rounder set keeps its own box (a dragon keeps 5,583 of 437,645).
const MOST: usize = 8192;

/// Six times the signed volume of a-b-c-d: positive when d lies on the side the face a-b-c faces.
fn volume(a: [f64; 3], b: [f64; 3], c: [f64; 3], d: [f64; 3]) -> f64 {
    let (u, v, w) = (sub(b, a), sub(c, a), sub(d, a));
    dot(cross(u, v), w)
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// The point of `candidates` farthest by `measure`, with its value; `candidates` is never empty.
fn farthest(
    candidates: impl Iterator<Item = usize>,
    measure: impl Fn(usize) -> f64,
) -> (usize, f64) {
    let mut best = (usize::MAX, f64::NEG_INFINITY);

    for i in candidates {
        let value = measure(i);

        if value > best.1 {
            best = (i, value);
        }
    }

    best
}

/// Points inside rows of plain words: `stride` words a row, a point at each of `offsets`.
#[derive(Clone, Copy)]
pub struct Points<'a> {
    words: &'a [f32],     // the rows
    stride: usize,        // words a row
    offsets: &'a [usize], // first word of each point in a row
}

impl<'a> Points<'a> {
    /// The points of `rows`, one at each word offset in `offsets`.
    pub fn of<T: bytemuck::Pod>(rows: &'a [T], offsets: &'a [usize]) -> Self {
        Self {
            words: bytemuck::cast_slice(rows),
            stride: std::mem::size_of::<T>() / 4,
            offsets,
        }
    }

    fn len(&self) -> usize {
        self.words.len() / self.stride * self.offsets.len()
    }

    fn at(&self, i: usize) -> [f32; 3] {
        let n = self.offsets.len();
        let w = i / n * self.stride + self.offsets[i % n];
        [self.words[w], self.words[w + 1], self.words[w + 2]]
    }
}

/// The points a linear function can peak at: the corners of their convex hull, and perhaps a few
/// points inside it. A box placed by any transform is exact over these alone. None when more than
/// `MOST` would be kept.
pub fn extreme_points(points: Points) -> Option<Vec<[f32; 3]>> {
    let count = points.len();
    let at = |i: usize| points.at(i);

    if count <= 8 {
        return Some((0..count).map(at).collect());
    }

    // unit-scaled about the box center, so one tolerance fits every size
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];

    for i in 0..count {
        let p = at(i);

        for k in 0..3 {
            lo[k] = lo[k].min(f64::from(p[k]));
            hi[k] = hi[k].max(f64::from(p[k]));
        }
    }

    let center = [
        0.5 * (lo[0] + hi[0]),
        0.5 * (lo[1] + hi[1]),
        0.5 * (lo[2] + hi[2]),
    ];
    let scale = (0..3).map(|k| hi[k] - lo[k]).fold(0.0, f64::max);

    if scale == 0.0 || !scale.is_finite() {
        return Some(vec![at(0)]);
    }

    let q = |i: usize| {
        let p = at(i);
        [0, 1, 2].map(|k| (f64::from(p[k]) - center[k]) / scale)
    };
    let volume_at = |f: [usize; 3], i: usize| volume(q(f[0]), q(f[1]), q(f[2]), q(i));
    // a face: its corners, their unit-scaled positions and the points waiting above it
    let face = |f: [usize; 3]| (f, f.map(q), Vec::<u32>::new());
    let keep = |corners: &[usize]| {
        (corners.len() <= MOST).then(|| corners.iter().map(|&i| at(i)).collect())
    };

    // the two points farthest apart among the axis extremes span the set
    let all = || 0..count;
    let mut ends = Vec::with_capacity(6);

    for k in 0..3 {
        ends.push(farthest(all(), |i| -q(i)[k]).0);
        ends.push(farthest(all(), |i| q(i)[k]).0);
    }

    let mut span = (ends[0], ends[1], -1.0);

    for &i in &ends {
        for &j in &ends {
            let d = sub(q(i), q(j));

            if dot(d, d) > span.2 {
                span = (i, j, dot(d, d));
            }
        }
    }

    let (a, b) = (span.0, span.1);
    let axis = sub(q(b), q(a));
    let (c, off_line) = farthest(all(), |i| {
        let n = cross(axis, sub(q(i), q(a)));
        dot(n, n)
    });

    // all on one line: its two ends
    if off_line <= ABOVE * ABOVE {
        return keep(&[a, b]);
    }

    let (d, height) = farthest(all(), |i| volume_at([a, b, c], i).abs());

    // all in one plane: every point, as flat definitions are small
    if height <= ABOVE {
        return keep(&all().collect::<Vec<usize>>());
    }

    // a tetrahedron with every face turned outward
    let (b, c) = if volume_at([a, b, c], d) > 0.0 {
        (c, b)
    } else {
        (b, c)
    };
    let mut kept = vec![a, b, c, d];
    let mut faces: Vec<_> = [[a, b, c], [a, d, b], [b, d, c], [c, d, a]]
        .into_iter()
        .map(face)
        .collect();
    let above = |p: &[[f64; 3]; 3], i: usize| volume(p[0], p[1], p[2], q(i)) > ABOVE;

    // each point waits above the first face it clears; one inside every face is no corner
    for i in all() {
        if let Some((_, _, waiting)) = faces.iter_mut().find(|(_, p, _)| above(p, i)) {
            waiting.push(i as u32);
        }
    }

    // a face with points above raises a tent to the highest; points under the tent are dropped
    while let Some(([a, b, c], p, waiting)) = faces.pop() {
        if waiting.is_empty() {
            continue;
        }

        let height = |i: usize| volume(p[0], p[1], p[2], q(i));
        let (apex, _) = farthest(waiting.iter().map(|&i| i as usize), height);
        kept.push(apex);

        if kept.len() > MOST {
            return None;
        }

        let mut tent: Vec<_> = [[a, b, apex], [b, c, apex], [c, a, apex]]
            .into_iter()
            .map(face)
            .collect();

        for i in waiting {
            if i as usize == apex {
                continue;
            }

            if let Some((_, _, next)) = tent.iter_mut().find(|(_, p, _)| above(p, i as usize)) {
                next.push(i);
            }
        }

        faces.extend(tent);
    }

    keep(&kept)
}

/// The world box of `points` placed by `place`, in f64.
pub fn placed_box(points: &[[f32; 3]], place: &Xform) -> AABB {
    let m = &place.m;
    let mut out = AABB::empty();

    for p in points {
        let [x, y, z] = p.map(f64::from);
        out.union_with_point(
            m[0] * x + m[4] * y + m[8] * z + m[12],
            m[1] * x + m[5] * y + m[9] * z + m[13],
            m[2] * x + m[6] * y + m[10] * z + m[14],
        );
    }

    out
}

/// True when `hull`'s own box is `bounds`, to a millionth of its size.
fn spans(hull: &[[f32; 3]], bounds: &AABB) -> bool {
    let own = placed_box(hull, &Xform::identity());
    let tolerance = 1e-6 * bounds.diagonal().max(1e-9);
    own.is_valid()
        && [
            (own.cx, bounds.cx),
            (own.cy, bounds.cy),
            (own.cz, bounds.cz),
            (own.hx, bounds.hx),
            (own.hy, bounds.hy),
            (own.hz, bounds.hz),
        ]
        .iter()
        .all(|(a, b)| (a - b).abs() <= tolerance)
}

/// The extreme points of every vertex, segment end and marker one walk wrote, with `more`; None
/// when the walk wrote rows without points here, or its points do not span `bounds`.
pub fn hull_of(up: &Upload, more: &[[f32; 3]], bounds: &AABB) -> Option<Hull> {
    let seg = &up.seg;
    let lanes = super::patch::Counts::of(up)
        .lanes
        .iter()
        .any(|rows| *rows > 0);
    let unread = lanes || !up.cloud.pos.is_empty() || !seg.sheet_rows.is_empty();

    if unread || !bounds.is_valid() {
        return None;
    }

    // each source on its own first: a mesh's vertices are never gathered
    let sources = [
        Points::of(&up.arena.verts, &[0]),
        Points::of(&seg.pipes, &[0, 4]),
        Points::of(&seg.ribbons, &[0, 4]),
        Points::of(&up.glyph.spheres, &[0]),
        Points::of(&up.glyph.dots, &[0]),
    ];
    let mut points = more.to_vec();

    for source in sources {
        points.extend(extreme_points(source)?);
    }

    let hull = extreme_points(Points::of(&points, &[0]))?;
    spans(&hull, bounds).then(|| Hull::from(hull))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A beam with its axis poking out of both ends: placed over its extreme points the box is
    /// exact, where its own box turned is looser.
    #[test]
    fn a_turned_beam_is_boxed_by_its_extreme_points() {
        let mut points = Vec::new();

        for x in [-1.5f32, 1.5] {
            for y in [-0.5f32, 0.5] {
                for z in [-0.5f32, 0.5] {
                    points.push([x, y, z]);
                }
            }
        }

        // the axis ends are extreme, the center and an edge middle never
        points.extend([
            [-2.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.5, 0.5],
        ]);
        let hull = extreme_points(Points::of(&points, &[0])).unwrap();
        assert_eq!(hull.len(), 10, "the corners and the axis ends");

        let turn = &Xform::rotation_z(37.0, true) * &Xform::rotation_x(20.0, true);
        let exact = placed_box(&points, &turn);
        let from_hull = placed_box(&hull, &turn);
        let loose = placed_box(&points, &Xform::identity()).transformed(&turn);
        assert_eq!(exact.min_point(), from_hull.min_point());
        assert_eq!(exact.max_point(), from_hull.max_point());
        assert!(loose.hx > exact.hx + 0.1, "the turned box is looser");
    }

    /// A dense cloud keeps a few of its points, and every direction peaks on one of them.
    #[test]
    fn every_direction_peaks_on_a_kept_point() {
        let mut points = Vec::new();
        let mut seed = 7u32;
        let mut next = || {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            (seed >> 8) as f32 / (1 << 24) as f32 * 2.0 - 1.0
        };

        for _ in 0..20_000 {
            points.push([next() * 3.0, next(), next() * 0.2 + 100.0]);
        }

        let hull = extreme_points(Points::of(&points, &[0])).unwrap();
        assert!(hull.len() < points.len() / 10, "{} kept", hull.len());

        for k in 0..200 {
            let angle = k as f64 * 0.37;
            let turn =
                &Xform::rotation_z(angle * 57.0, true) * &Xform::rotation_y(angle * 23.0, true);
            let exact = placed_box(&points, &turn);
            let from_hull = placed_box(&hull, &turn);
            assert_eq!(exact.min_point(), from_hull.min_point(), "turn {k}");
            assert_eq!(exact.max_point(), from_hull.max_point(), "turn {k}");
        }
    }

    /// Points all on a sphere are all extreme: past `MOST` the set keeps its own box.
    #[test]
    fn a_round_set_gives_up() {
        let round: Vec<[f32; 3]> = (0..MOST * 2)
            .map(|i| {
                let (a, b) = (
                    i as f32 * 0.618_034 * std::f32::consts::TAU,
                    i as f32 / (MOST * 2) as f32,
                );
                let z = 2.0 * b - 1.0;
                let r = (1.0 - z * z).sqrt();
                [r * a.cos(), r * a.sin(), z]
            })
            .collect();
        assert!(extreme_points(Points::of(&round, &[0])).is_none());
    }

    /// Flat sets keep every point, straight ones their two ends.
    #[test]
    fn flat_and_straight_sets() {
        let flat: Vec<[f32; 3]> = (0..40)
            .map(|i| {
                let t = i as f32 * 0.3;
                [t.cos() * 2.0, t.sin(), 5.0]
            })
            .chain([[0.0, 0.0, 5.0], [0.1, 0.2, 5.0]])
            .collect();
        let hull = extreme_points(Points::of(&flat, &[0])).unwrap();
        assert_eq!(hull.len(), flat.len());
        let turn = Xform::rotation_x(30.0, true);
        assert_eq!(
            placed_box(&flat, &turn).max_point(),
            placed_box(&hull, &turn).max_point()
        );

        let line: Vec<[f32; 3]> = (0..20).map(|i| [i as f32, 2.0 * i as f32, 0.0]).collect();
        let ends = extreme_points(Points::of(&line, &[0])).unwrap();
        assert_eq!(ends.len(), 2);
        assert!(ends.contains(&[0.0, 0.0, 0.0]) && ends.contains(&[19.0, 38.0, 0.0]));
    }
}
