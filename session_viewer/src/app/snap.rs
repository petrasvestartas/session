use crate::app::selection::Controls;
use session_rust::{AABB, Geometry, NurbsCurve, NurbsSurface, Point, Vector, Xform};

/// Polyline segments standing in for a curve.
const CURVE_SAMPLES: usize = 64;

/// Screen cells a row may cover before bins try it on every query instead.
const LARGE_CELLS: f64 = 8.0;

/// Kinds of snap point, best first.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SnapKind {
    End,    // end of an open line
    Vertex, // interior or loop vertex
    Mid,    // middle of a segment
    Center, // centre of a shape
    Perp,   // foot of the perpendicular from the last point
    Near,   // nearest point on a segment
}

pub const END: u8 = 1; // ends and vertices
pub const NEAR: u8 = 2; // anywhere on a curve or edge
pub const MID: u8 = 4; // segment midpoints
pub const CENTER: u8 = 8; // centres of closed shapes
pub const PERP: u8 = 16; // perpendicular from the last point
pub const DEFAULT: u8 = END | NEAR | MID | CENTER; // modes on at start

/// The toolbar buttons, by label and mode bit.
pub const MODES: [(&str, u8); 5] = [
    ("End", END),
    ("Near", NEAR),
    ("Mid", MID),
    ("Center", CENTER),
    ("Perp", PERP),
];

impl SnapKind {
    /// The mode bit that turns this kind on.
    pub fn mode(self) -> u8 {
        match self {
            SnapKind::End | SnapKind::Vertex => END,
            SnapKind::Mid => MID,
            SnapKind::Center => CENTER,
            SnapKind::Perp => PERP,
            SnapKind::Near => NEAR,
        }
    }
}

/// The bit of a mode named `word`, any case.
pub fn mode(word: &str) -> Option<u8> {
    MODES
        .iter()
        .find(|(label, _)| label.eq_ignore_ascii_case(word))
        .map(|(_, bit)| *bit)
}

/// One snap candidate.
#[derive(Clone, Debug)]
pub struct Snap {
    pub point: Point,   // where it is
    pub kind: SnapKind, // what it is
    pub owner: u32,     // object row it belongs to
}

/// Add the ends, vertices and midpoints of a polyline.
pub fn from_polyline(points: &[Point], closed: bool, owner: u32, out: &mut Vec<Snap>) {
    if points.is_empty() {
        return;
    }

    let last = points.len() - 1;

    for (i, p) in points.iter().enumerate() {
        let interior = i != 0 && i != last;
        let kind = if closed || interior {
            SnapKind::Vertex
        } else {
            SnapKind::End
        };
        out.push(Snap {
            point: p.clone(),
            kind,
            owner,
        });
    }

    let spans = if closed { points.len() } else { last }; // segment count

    for i in 0..spans {
        let a = &points[i];
        let b = &points[(i + 1) % points.len()];
        out.push(Snap {
            point: Point::new(
                (a[0] + b[0]) * 0.5,
                (a[1] + b[1]) * 0.5,
                (a[2] + b[2]) * 0.5,
            ),
            kind: SnapKind::Mid,
            owner,
        });
    }
}

/// The centre of a closed loop, when the points close.
pub fn from_loop(points: &[Point], owner: u32, out: &mut Vec<Snap>) {
    let [first, .., last] = points else {
        return;
    };

    if points.len() < 4 || first.distance(last, None) > 1e-9 {
        return;
    }

    let corners = &points[..points.len() - 1]; // the repeated end counted once
    let count = corners.len() as f64;
    let sum = |i: usize| corners.iter().map(|p| p[i]).sum::<f64>() / count;
    out.push(Snap {
        point: Point::new(sum(0), sum(1), sum(2)),
        kind: SnapKind::Center,
        owner,
    });
}

/// The point of segment `a`-`b` nearest to `to`.
pub fn nearest_on_segment(a: &Point, b: &Point, to: &Point, owner: u32) -> Snap {
    let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let ap = [to[0] - a[0], to[1] - a[1], to[2] - a[2]];
    let len2 = ab[0] * ab[0] + ab[1] * ab[1] + ab[2] * ab[2];
    let t = if len2 > 0.0 {
        ((ap[0] * ab[0] + ap[1] * ab[1] + ap[2] * ab[2]) / len2).clamp(0.0, 1.0)
    } else {
        0.0
    };
    Snap {
        point: Point::new(a[0] + ab[0] * t, a[1] + ab[1] * t, a[2] + ab[2] * t),
        kind: SnapKind::Near,
        owner,
    }
}

/// The point of segment `a`-`b` nearest to the ray from `origin` along `direction`.
pub fn nearest_to_ray(a: &Point, b: &Point, origin: &Point, direction: &Vector) -> Point {
    let u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let w = [a[0] - origin[0], a[1] - origin[1], a[2] - origin[2]];
    let d = [direction[0], direction[1], direction[2]];
    let dot = |p: [f64; 3], q: [f64; 3]| p[0] * q[0] + p[1] * q[1] + p[2] * q[2];
    let (uu, ud, dd) = (dot(u, u), dot(u, d), dot(d, d));
    let denominator = uu * dd - ud * ud;
    // parallel: any point is as near, take the start
    let t = if denominator > 1e-12 * uu * dd {
        ((ud * dot(d, w) - dd * dot(u, w)) / denominator).clamp(0.0, 1.0)
    } else {
        0.0
    };
    Point::new(a[0] + u[0] * t, a[1] + u[1] * t, a[2] + u[2] * t)
}

/// The foot of the perpendicular from `from` onto segment `a`-`b`, if it falls inside.
pub fn perpendicular(a: &Point, b: &Point, from: &Point) -> Option<Point> {
    let u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let length = u[0] * u[0] + u[1] * u[1] + u[2] * u[2];

    if length <= 1e-24 {
        return None;
    }

    let t = ((from[0] - a[0]) * u[0] + (from[1] - a[1]) * u[1] + (from[2] - a[2]) * u[2]) / length;
    (t > 1e-9 && t < 1.0 - 1e-9)
        .then(|| Point::new(a[0] + u[0] * t, a[1] + u[1] * t, a[2] + u[2] * t))
}

/// Near and Perp snaps on the wires under the cursor ray.
pub fn along_wires(
    wires: &[(Vec<Point>, u32)],
    ray: &(Point, Vector),
    from: Option<&Point>,
    modes: u8,
    out: &mut Vec<Snap>,
) {
    for (points, owner) in wires {
        for pair in points.windows(2) {
            if modes & NEAR != 0 {
                out.push(Snap {
                    point: nearest_to_ray(&pair[0], &pair[1], &ray.0, &ray.1),
                    kind: SnapKind::Near,
                    owner: *owner,
                });
            }

            if modes & PERP != 0
                && let Some(foot) = from.and_then(|p| perpendicular(&pair[0], &pair[1], p))
            {
                out.push(Snap {
                    point: foot,
                    kind: SnapKind::Perp,
                    owner: *owner,
                });
            }
        }
    }
}

/// Snap points and wires of one placed object; `room` caps the edges a mesh or BRep adds as wires.
pub fn of_geometry(
    geometry: &Geometry,
    place: &Xform,
    owner: u32,
    room: usize,
    out: &mut Vec<Snap>,
    wires: &mut Vec<(Vec<Point>, u32)>,
) {
    match geometry {
        Geometry::Point(p) => out.push(Snap {
            point: p.transformed(place),
            kind: SnapKind::End,
            owner,
        }),
        Geometry::Line(line) => {
            let points = vec![
                line.start().transformed(place),
                line.end().transformed(place),
            ];
            from_polyline(&points, false, owner, out);
            wires.push((points, owner));
        }
        Geometry::Polyline(line) => {
            let points: Vec<_> = line
                .get_points()
                .iter()
                .map(|p| p.transformed(place))
                .collect();
            from_polyline(&points, false, owner, out);
            from_loop(&points, owner, out);
            wires.push((points, owner));
        }
        Geometry::NurbsCurve(curve) => {
            let (a, b) = curve.domain();
            for t in [a, b] {
                out.push(Snap {
                    point: curve.point_at(t).transformed(place),
                    kind: SnapKind::End,
                    owner,
                });
            }
            out.push(Snap {
                point: curve.point_at((a + b) * 0.5).transformed(place),
                kind: SnapKind::Mid,
                owner,
            });
            // a fine polyline stands in for the curve
            let (samples, _) = curve.divide_by_count(CURVE_SAMPLES, true);
            let points: Vec<_> = samples.iter().map(|p| p.transformed(place)).collect();
            from_loop(&points, owner, out);
            wires.push((points, owner));
        }
        Geometry::Mesh(_) | Geometry::BRep(_) | Geometry::NurbsSurface(_) => {
            let controls = Controls::from_geometry(geometry);
            let points: Vec<_> = controls
                .points
                .iter()
                .map(|p| Point::new(p.position[0], p.position[1], p.position[2]).transformed(place))
                .collect();
            // edges are wires too, while there are not too many
            if controls.links.len() <= room {
                for [a, b] in &controls.links {
                    wires.push((vec![points[*a].clone(), points[*b].clone()], owner));
                }
            }
            out.extend(points.into_iter().map(|point| Snap {
                point,
                kind: SnapKind::Vertex,
                owner,
            }));
        }
        _ => {}
    }
}

/// Control points a mesh, BRep or surface offers, counted without collecting them.
pub fn control_count(geometry: &Geometry) -> usize {
    match geometry {
        Geometry::Mesh(mesh) => mesh.vertex.len(),
        Geometry::BRep(brep) => {
            brep.m_vertices.len()
                + brep
                    .m_curves_3d
                    .iter()
                    .map(NurbsCurve::cv_count)
                    .sum::<usize>()
                + brep
                    .m_surfaces
                    .iter()
                    .map(NurbsSurface::cv_count_total)
                    .sum::<usize>()
        }
        Geometry::NurbsSurface(surface) => surface.cv_count_total(),
        _ => 0,
    }
}

/// The best candidate of an enabled mode within `aperture` pixels of the cursor.
pub fn best<F>(
    candidates: &[Snap],
    modes: u8,
    cursor: (f64, f64),
    aperture: f64,
    project: F,
) -> Option<Snap>
where
    F: Fn(&Point) -> Option<(f64, f64)>,
{
    best_in(candidates, modes, cursor, aperture, project)
}

/// The best candidate from any source of snaps, several lists joined without copying.
pub fn best_in<'a, I, F>(
    candidates: I,
    modes: u8,
    cursor: (f64, f64),
    aperture: f64,
    project: F,
) -> Option<Snap>
where
    I: IntoIterator<Item = &'a Snap>,
    F: Fn(&Point) -> Option<(f64, f64)>,
{
    let mut winner: Option<(SnapKind, f64, &Snap)> = None;

    for c in candidates {
        if modes & c.kind.mode() == 0 {
            continue;
        }

        let Some((x, y)) = project(&c.point) else {
            continue;
        };
        let (dx, dy) = (x - cursor.0, y - cursor.1);
        let d = (dx * dx + dy * dy).sqrt();

        if d > aperture {
            continue;
        }

        // better kind first, then nearer
        let better = match winner {
            None => true,
            Some((kind, best_d, _)) => c.kind < kind || (c.kind == kind && d < best_d),
        };

        if better {
            winner = Some((c.kind, d, c));
        }
    }

    winner.map(|(_, _, c)| c.clone())
}

/// One view's world to pixel mapping, made once and used for many points.
#[derive(Clone, PartialEq)]
pub struct Screen {
    matrix: [f64; 16],    // view projection relative to `origin`, column major
    origin: [f64; 3],     // the point the matrix is anchored at
    pub size: (f64, f64), // viewport, pixels
}

impl Screen {
    /// The view `matrix`, anchored at `origin`, onto a `size` viewport.
    pub fn new(matrix: [f64; 16], origin: [f64; 3], size: (f64, f64)) -> Self {
        Self {
            matrix,
            origin,
            size,
        }
    }

    /// Clip coordinates of a world point.
    fn clip(&self, p: [f64; 3]) -> [f64; 4] {
        let v = [
            p[0] - self.origin[0],
            p[1] - self.origin[1],
            p[2] - self.origin[2],
        ];
        let m = &self.matrix;
        std::array::from_fn(|r| m[r] * v[0] + m[r + 4] * v[1] + m[r + 8] * v[2] + m[r + 12])
    }

    /// Clip coordinates to pixels.
    fn pixel(&self, clip: [f64; 4]) -> (f64, f64) {
        (
            (clip[0] / clip[3] * 0.5 + 0.5) * self.size.0,
            (0.5 - clip[1] / clip[3] * 0.5) * self.size.1,
        )
    }

    /// A world point in pixels, None behind the eye.
    pub fn point(&self, p: &Point) -> Option<(f64, f64)> {
        let clip = self.clip([p[0], p[1], p[2]]);
        (clip[3] > 0.0).then(|| self.pixel(clip))
    }

    /// A box as a circle in pixels, centre and radius; an endless radius when it reaches the eye.
    pub fn circle(&self, b: &AABB) -> Option<(f64, f64, f64)> {
        let m = &self.matrix;
        let clip = self.clip([b.cx, b.cy, b.cz]);
        let radius = (b.hx * b.hx + b.hy * b.hy + b.hz * b.hz).sqrt();
        // the nearest depth the box can have: clip w of its closest point
        let near = clip[3] - (m[3] * m[3] + m[7] * m[7] + m[11] * m[11]).sqrt() * radius;

        if !near.is_finite() {
            return None;
        }

        if near <= 1e-9 {
            return Some((self.size.0 * 0.5, self.size.1 * 0.5, f64::INFINITY));
        }

        let (x, y) = self.pixel(clip);
        let vertical = (m[1] * m[1] + m[5] * m[5] + m[9] * m[9]).sqrt(); // clip units per world unit
        Some((x, y, radius * vertical * 0.5 * self.size.1 / near))
    }

    /// The pixel box of a chain of points as a circle; an endless radius when one is behind the eye.
    pub fn footprint(&self, points: &[Point]) -> Option<(f64, f64, f64)> {
        let (mut low, mut high) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);

        for p in points {
            let Some((x, y)) = self.point(p) else {
                return Some((self.size.0 * 0.5, self.size.1 * 0.5, f64::INFINITY));
            };
            low = [low[0].min(x), low[1].min(y)];
            high = [high[0].max(x), high[1].max(y)];
        }

        (low[0] <= high[0]).then(|| {
            let (width, height) = (high[0] - low[0], high[1] - low[1]);
            (
                low[0] + width * 0.5,
                low[1] + height * 0.5,
                width.hypot(height) * 0.5,
            )
        })
    }
}

/// Ids sorted into screen cells by where their circles land: rows filled a slice at a time, or any id put in.
pub struct Bins {
    cell: f64,            // cell size, pixels
    columns: usize,       // cells across
    lines: usize,         // cells down
    cells: Vec<Vec<u32>>, // the rows of each cell
    large: Vec<u32>,      // rows too large to bin, offered on every query
    next: u32,            // the first row not binned yet
}

impl Bins {
    /// Empty cells over a `size` pixel screen.
    pub fn new(size: (f64, f64), cell: f64) -> Self {
        let columns = (size.0 / cell).ceil().clamp(1.0, 4096.0) as usize;
        let lines = (size.1 / cell).ceil().clamp(1.0, 4096.0) as usize;
        Self {
            cell,
            columns,
            lines,
            cells: vec![Vec::new(); columns * lines],
            large: Vec::new(),
            next: 0,
        }
    }

    /// Bin up to `budget` more rows below `end`, where the last call stopped; `circle` puts a row on screen.
    pub fn fill<F>(&mut self, end: u32, budget: u32, mut circle: F)
    where
        F: FnMut(u32) -> Option<(f64, f64, f64)>,
    {
        let stop = self.next.saturating_add(budget).min(end);

        for row in self.next..stop {
            if let Some((x, y, radius)) = circle(row) {
                self.insert(row, (x, y), radius);
            }
        }

        self.next = self.next.max(stop);
    }

    /// Put `id` in every cell its circle touches; one too large for that is offered on every query.
    pub fn insert(&mut self, id: u32, at: (f64, f64), radius: f64) {
        if radius > self.cell * LARGE_CELLS {
            self.large.push(id);
            return;
        }

        let Some([left, top, right, bottom]) = self.span(at, radius) else {
            return;
        };

        for line in top..=bottom {
            for column in left..=right {
                self.cells[line * self.columns + column].push(id);
            }
        }
    }

    /// Rows whose circles may come within `reach` of `at`, sorted, each once.
    pub fn near(&self, at: (f64, f64), reach: f64, out: &mut Vec<u32>) {
        out.clear();
        out.extend_from_slice(&self.large);

        if let Some([left, top, right, bottom]) = self.span(at, reach) {
            for line in top..=bottom {
                for column in left..=right {
                    out.extend_from_slice(&self.cells[line * self.columns + column]);
                }
            }
        }

        out.sort_unstable();
        out.dedup();
    }

    /// The cells a circle touches: left, top, right, bottom; None when it misses the screen.
    fn span(&self, at: (f64, f64), radius: f64) -> Option<[usize; 4]> {
        let (left, right) = ((at.0 - radius) / self.cell, (at.0 + radius) / self.cell);
        let (top, bottom) = ((at.1 - radius) / self.cell, (at.1 + radius) / self.cell);

        // off screen, or not a number
        if !(right >= 0.0 && bottom >= 0.0 && left < self.columns as f64 && top < self.lines as f64)
        {
            return None;
        }

        let clamp = |value: f64, count: usize| (value.max(0.0) as usize).min(count - 1);
        Some([
            clamp(left, self.columns),
            clamp(top, self.lines),
            clamp(right, self.columns),
            clamp(bottom, self.lines),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A point on z = 0.
    fn p(x: f64, y: f64) -> Point {
        Point::new(x, y, 0.0)
    }

    /// x and y are screen pixels already.
    fn flat(point: &Point) -> Option<(f64, f64)> {
        Some((point[0], point[1]))
    }

    /// An open polyline: two ends, inner vertices, one midpoint per span.
    #[test]
    fn an_open_polyline_offers_ends_vertices_and_midpoints() {
        let mut out = Vec::new();
        from_polyline(
            &[p(0.0, 0.0), p(10.0, 0.0), p(10.0, 10.0)],
            false,
            7,
            &mut out,
        );
        let count = |k: SnapKind| out.iter().filter(|s| s.kind == k).count();
        assert_eq!(count(SnapKind::End), 2);
        assert_eq!(count(SnapKind::Vertex), 1);
        assert_eq!(count(SnapKind::Mid), 2);
        assert!(out.iter().all(|s| s.owner == 7));
    }

    /// A closed loop has no ends.
    #[test]
    fn a_closed_loop_has_no_ends() {
        let mut out = Vec::new();
        from_polyline(
            &[p(0.0, 0.0), p(10.0, 0.0), p(10.0, 10.0)],
            true,
            0,
            &mut out,
        );
        assert_eq!(out.iter().filter(|s| s.kind == SnapKind::End).count(), 0);
        assert_eq!(out.iter().filter(|s| s.kind == SnapKind::Vertex).count(), 3);
        assert_eq!(out.iter().filter(|s| s.kind == SnapKind::Mid).count(), 3);
    }

    /// A farther end beats a nearer near-point.
    #[test]
    fn kind_wins_before_distance() {
        let candidates = vec![
            Snap {
                point: p(2.0, 0.0),
                kind: SnapKind::Near,
                owner: 0,
            },
            Snap {
                point: p(6.0, 0.0),
                kind: SnapKind::End,
                owner: 0,
            },
        ];
        let best = best(&candidates, DEFAULT, (0.0, 0.0), 12.0, flat).expect("a snap");
        assert_eq!(best.kind, SnapKind::End);
    }

    /// Within one kind, the nearest wins.
    #[test]
    fn distance_decides_within_a_kind() {
        let candidates = vec![
            Snap {
                point: p(9.0, 0.0),
                kind: SnapKind::End,
                owner: 1,
            },
            Snap {
                point: p(3.0, 0.0),
                kind: SnapKind::End,
                owner: 2,
            },
        ];
        assert_eq!(
            best(&candidates, DEFAULT, (0.0, 0.0), 12.0, flat)
                .unwrap()
                .owner,
            2
        );
    }

    /// Too far or not visible: no snap.
    #[test]
    fn out_of_reach_and_out_of_sight_do_not_snap() {
        let candidates = vec![Snap {
            point: p(40.0, 0.0),
            kind: SnapKind::End,
            owner: 0,
        }];
        assert!(
            best(&candidates, DEFAULT, (0.0, 0.0), 12.0, flat).is_none(),
            "too far"
        );
        let near = vec![Snap {
            point: p(1.0, 0.0),
            kind: SnapKind::End,
            owner: 0,
        }];
        assert!(
            best(&near, DEFAULT, (0.0, 0.0), 12.0, |_| None).is_none(),
            "not visible"
        );
    }

    /// The nearest point stays on the segment.
    #[test]
    fn nearest_on_a_segment_stays_on_the_segment() {
        let (a, b) = (p(0.0, 0.0), p(10.0, 0.0));
        let middle = nearest_on_segment(&a, &b, &p(4.0, 5.0), 0);
        assert!((middle.point[0] - 4.0).abs() < 1e-12 && middle.point[1].abs() < 1e-12);
        let past = nearest_on_segment(&a, &b, &p(50.0, 5.0), 0);
        assert!((past.point[0] - 10.0).abs() < 1e-12);
        let before = nearest_on_segment(&a, &b, &p(-50.0, 5.0), 0);
        assert!(before.point[0].abs() < 1e-12);
    }

    /// A mode switched off never snaps.
    #[test]
    fn a_mode_switched_off_is_skipped() {
        let candidates = vec![Snap {
            point: p(1.0, 0.0),
            kind: SnapKind::Vertex,
            owner: 0,
        }];
        assert!(best(&candidates, END, (0.0, 0.0), 12.0, flat).is_some());
        assert!(best(&candidates, NEAR | MID, (0.0, 0.0), 12.0, flat).is_none());
        assert_eq!(mode("near"), Some(NEAR));
        assert_eq!(mode("perp"), Some(PERP));
        assert_eq!(mode("sideways"), None);
    }

    /// The ray over a segment finds the point under it.
    #[test]
    fn nearest_to_a_ray_lands_under_the_cursor() {
        let (a, b) = (p(0.0, 0.0), p(10.0, 0.0));
        let down = Vector::new(0.0, 0.0, -1.0);
        let hit = nearest_to_ray(&a, &b, &Point::new(3.0, 2.0, 5.0), &down);
        assert!((hit[0] - 3.0).abs() < 1e-12 && hit[1].abs() < 1e-12);
        let past = nearest_to_ray(&a, &b, &Point::new(30.0, 2.0, 5.0), &down);
        assert!((past[0] - 10.0).abs() < 1e-12);
        let along = nearest_to_ray(
            &a,
            &b,
            &Point::new(-5.0, 0.0, 0.0),
            &Vector::new(1.0, 0.0, 0.0),
        );
        assert!(along[0].abs() < 1e-12, "a parallel ray takes the start");
    }

    /// The perpendicular foot exists only inside the segment.
    #[test]
    fn a_perpendicular_foot_stays_inside() {
        let (a, b) = (p(0.0, 0.0), p(10.0, 0.0));
        let foot = perpendicular(&a, &b, &p(4.0, 7.0)).expect("inside");
        assert!((foot[0] - 4.0).abs() < 1e-12 && foot[1].abs() < 1e-12);
        assert!(perpendicular(&a, &b, &p(12.0, 7.0)).is_none());
        assert!(perpendicular(&a, &a, &p(1.0, 1.0)).is_none());
    }

    /// A closed loop offers its centre; an open chain does not.
    #[test]
    fn a_closed_loop_offers_its_center() {
        let square = [
            p(0.0, 0.0),
            p(4.0, 0.0),
            p(4.0, 4.0),
            p(0.0, 4.0),
            p(0.0, 0.0),
        ];
        let mut out = Vec::new();
        from_loop(&square, 3, &mut out);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].kind, SnapKind::Center);
        assert!((out[0].point[0] - 2.0).abs() < 1e-12 && (out[0].point[1] - 2.0).abs() < 1e-12);
        from_loop(&square[..4], 3, &mut out);
        assert_eq!(out.len(), 1, "open chains have no centre");
    }

    /// Kind wins across lists, then distance; an empty list or a switched-off mode offers nothing.
    #[test]
    fn best_in_ranks_across_lists() {
        let snap = |x, kind, owner| Snap {
            point: p(x, 0.0),
            kind,
            owner,
        };
        let first = vec![snap(1.0, SnapKind::Near, 1), snap(8.0, SnapKind::Mid, 2)];
        let second = vec![snap(6.0, SnapKind::End, 3), snap(3.0, SnapKind::Mid, 4)];
        let best = |lists: &[&[Snap]], modes| {
            let joined = lists.iter().flat_map(|list| list.iter());
            best_in(joined, modes, (0.0, 0.0), 12.0, flat).map(|s| s.owner)
        };
        assert_eq!(
            best(&[&first[..], &second[..]], DEFAULT),
            Some(3),
            "an End beats a nearer Near"
        );
        assert_eq!(
            best(&[&first[..], &second[..]], MID),
            Some(4),
            "Mid only: the nearest Mid of both"
        );
        assert_eq!(best(&[&first[..], &[]], END), None, "no End anywhere");
        assert_eq!(best(&[], DEFAULT), None, "nothing to rank");
    }

    /// An identity view makes clip space the world: pixels follow from x and y.
    #[test]
    fn a_screen_puts_points_and_boxes_in_pixels() {
        let mut matrix = [0.0; 16];

        for i in [0, 5, 10, 15] {
            matrix[i] = 1.0;
        }

        let screen = Screen::new(matrix, [0.0; 3], (200.0, 100.0));
        assert_eq!(screen.point(&p(0.0, 0.0)), Some((100.0, 50.0)));
        assert_eq!(screen.point(&p(1.0, 1.0)), Some((200.0, 0.0)));
        let small = AABB {
            cx: 0.0,
            cy: 0.0,
            cz: 0.0,
            hx: 0.1,
            hy: 0.0,
            hz: 0.0,
        };
        let (x, y, radius) = screen.circle(&small).expect("on screen");
        assert_eq!((x, y), (100.0, 50.0));
        assert!((radius - 5.0).abs() < 1e-12, "a tenth of the half height");
        let (x, y, radius) = screen
            .footprint(&[p(0.0, 0.0), p(1.0, 1.0)])
            .expect("a wire");
        assert_eq!((x, y), (150.0, 25.0));
        assert!((radius - 100.0_f64.hypot(50.0) * 0.5).abs() < 1e-12);
        assert_eq!(screen.footprint(&[]), None);

        // w is the distance in front of the eye, down -z
        matrix[15] = 0.0;
        matrix[11] = -1.0;
        let eye = Screen::new(matrix, [0.0; 3], (200.0, 100.0));
        let around = AABB {
            cz: -0.5,
            hx: 1.0,
            ..small
        };
        assert_eq!(
            eye.circle(&around).map(|c| c.2),
            Some(f64::INFINITY),
            "a box around the eye is everywhere"
        );
        assert_eq!(
            eye.point(&Point::new(0.0, 0.0, 1.0)),
            None,
            "behind the eye"
        );
        let behind = eye.footprint(&[Point::new(0.0, 0.0, -1.0), Point::new(0.0, 0.0, 1.0)]);
        assert_eq!(
            behind.map(|c| c.2),
            Some(f64::INFINITY),
            "a wire reaching behind the eye"
        );
    }

    /// Rows land in the cells their circles cover; a huge one is always offered, one off screen never.
    #[test]
    fn bins_offer_the_rows_near_a_point() {
        let circles = [
            (15.0, 15.0, 2.0),
            (150.0, 80.0, 3.0),
            (100.0, 50.0, 500.0),
            (-50.0, 20.0, 5.0),
        ];
        let mut bins = Bins::new((200.0, 100.0), 10.0);
        let mut out = Vec::new();
        bins.fill(4, 2, |row| Some(circles[row as usize]));
        bins.near((150.0, 80.0), 4.0, &mut out);
        assert_eq!(out, [1]);
        // the next slice goes on where the first stopped
        bins.fill(4, 2, |row| Some(circles[row as usize]));
        bins.near((15.0, 15.0), 4.0, &mut out);
        assert_eq!(out, [0, 2]);
        bins.near((190.0, 10.0), 4.0, &mut out);
        assert_eq!(out, [2]);
        bins.near((f64::NAN, 10.0), 4.0, &mut out);
        assert_eq!(out, [2]);
        // any id can go in by hand, a point included
        bins.insert(9, (190.0, 10.0), 0.0);
        bins.near((188.0, 12.0), 4.0, &mut out);
        assert_eq!(out, [2, 9]);
    }

    /// A placed line offers its moved ends and middle, and itself as a wire.
    #[test]
    fn a_placed_line_offers_ends_middle_and_a_wire() {
        let line = Geometry::Line(session_rust::Line::new(0.0, 0.0, 0.0, 10.0, 0.0, 0.0).into());
        let (mut out, mut wires) = (Vec::new(), Vec::new());
        of_geometry(
            &line,
            &Xform::translation(0.0, 5.0, 0.0),
            9,
            0,
            &mut out,
            &mut wires,
        );
        let found: Vec<_> = out
            .iter()
            .map(|s| (s.kind, s.point[0], s.point[1], s.owner))
            .collect();
        assert_eq!(
            found,
            [
                (SnapKind::End, 0.0, 5.0, 9),
                (SnapKind::End, 10.0, 5.0, 9),
                (SnapKind::Mid, 5.0, 5.0, 9)
            ]
        );
        assert_eq!(wires.len(), 1);
        assert_eq!(control_count(&line), 0, "no mesh or surface controls");
    }
}
