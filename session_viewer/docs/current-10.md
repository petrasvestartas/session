# 10 · Split curves and faces while keeping the shell joined

[Previous](current-9.md) · [Sequence](extend-integrated-tutorial.md) · [Next](command-line-walkthrough.md)

Continue in the same checkpoint workspace. Complete the edits below before compiling.

![Ownership and data flow](illustrations/16-01.svg)

First add the exact split service to the learning workspace’s sibling session_rust kernel, including the older checkpoint’s knot-multiplicity, independent tree-clone and split-boundary meshing corrections. Then connect source transactions, cutter selection, keyboard and touch confirmation. Keep visible thin curves selectable at pixel boundaries. Curve creates a non-rational NURBS curve from control points (degree up to three). Line and Polyline retain their types after splitting. Existing holes are preserved, all resulting face regions stay in the same BRep and incident boundary edges are updated. No booleans, projection, caps or volume splitting are performed.

### `../session_rust/src/simple_split.rs`

**NEW FILE · TYPE THIS**

```rust
//! Intersection splits that retain source curves and shared BRep topology.
use crate::brep::{BRep, BRepOrientation, BRepRef};
use crate::closest::Closest;
use crate::nurbscurve::NurbsCurve;
use crate::nurbssurface::NurbsSurface;
use crate::point::Point;
use crate::tolerance::Tolerance;
use std::collections::BTreeMap;

const EPS: f64 = Tolerance::ZERO_TOLERANCE;
const WORK_LIMIT: usize = 200000;
const FORWARD: BRepOrientation = BRepOrientation::Forward;
const REVERSED: BRepOrientation = BRepOrientation::Reversed;

fn require(condition: bool, message: &str) -> Result<(), String> {
    if condition {
        Ok(())
    } else {
        Err(message.into())
    }
}
fn check_tolerance(tolerance: f64) -> Result<(), String> {
    require(
        tolerance.is_finite() && tolerance > 0.0,
        "Split tolerance must be finite and positive",
    )
}
fn check_curve(curve: &NurbsCurve) -> Result<(), String> {
    require(curve.is_valid(), "Split requires valid curves")?;
    for i in 0..curve.cv_count() {
        let p = curve.get_cv(i).ok_or("Split requires valid controls")?;
        require(
            (0..3).all(|d| p[d].is_finite())
                && curve.weight(i).is_finite()
                && curve.weight(i) > 0.0,
            "Split requires finite controls and positive rational weights",
        )?;
    }
    Ok(())
}
fn check_surface(surface: &NurbsSurface) -> Result<(), String> {
    require(surface.is_valid(), "Split requires a valid NURBS surface")?;
    for i in 0..surface.m_cv_count[0] {
        for j in 0..surface.m_cv_count[1] {
            let p = surface.get_cv(i, j).ok_or("Invalid surface control")?;
            let w = surface.weight(i, j);
            require(
                (0..3).all(|d| p[d].is_finite()) && w.is_finite() && w > 0.,
                "Split requires finite surface controls and positive rational weights",
            )?;
        }
    }
    Ok(())
}
fn interval(curve: &NurbsCurve, a: f64, b: f64) -> Result<NurbsCurve, String> {
    let mut result = curve.duplicate();
    let (lo, hi) = curve.domain();
    let (a, b) = (a.clamp(lo, hi), b.clamp(lo, hi));
    require(b > a, "Split produced an empty curve interval")?;
    if a > lo || b < hi {
        require(result.trim(a, b), "Kernel refused a split interval")?;
    }
    Ok(result)
}
fn distance(a: &Point, b: &Point) -> f64 {
    a.distance(b, None)
}
fn dot(a: &Point, b: &Point) -> f64 {
    (0..3).map(|d| a[d] * b[d]).sum()
}
fn subtract(a: &Point, b: &Point) -> Point {
    Point::new(a[0] - b[0], a[1] - b[1], a[2] - b[2])
}
fn closest(curve: &NurbsCurve, point: &Point) -> Result<(f64, f64), String> {
    let (mut t, _) = Closest::curve_point(curve, point, 0.0, 0.0);
    let (lo, hi) = curve.domain();
    if curve.degree() == 1 {
        let spans = curve.get_span_vector();
        let mut best = f64::INFINITY;
        for span in spans.windows(2) {
            let a = curve.point_at(span[0]);
            let b = curve.point_at(span[1]);
            let v = subtract(&b, &a);
            let length2 = dot(&v, &v);
            if length2 <= EPS * EPS {
                continue;
            }
            let fraction = (dot(&subtract(point, &a), &v) / length2).clamp(0.0, 1.0);
            let segment = interval(curve, span[0], span[1])?;
            let w0 = segment.weight(0);
            let w1 = segment.weight(segment.cv_count() - 1);
            let normalized = fraction * w0 / (w1 * (1.0 - fraction) + fraction * w0);
            let candidate = span[0] + normalized * (span[1] - span[0]);
            let gap = distance(&curve.point_at(candidate), point);
            if gap < best {
                best = gap;
                t = candidate;
            }
        }
        return Ok((t, distance(&curve.point_at(t), point)));
    }
    for _ in 0..24 {
        let value = curve.evaluate(t, 1);
        let d = Point::new(value[1][0], value[1][1], value[1][2]);
        let r = Point::new(
            value[0][0] - point[0],
            value[0][1] - point[1],
            value[0][2] - point[2],
        );
        let dd = dot(&d, &d);
        if dd <= EPS * EPS {
            break;
        }
        let next = (t - dot(&d, &r) / dd).clamp(lo, hi);
        if (next - t).abs() <= EPS * (hi - lo) {
            t = next;
            break;
        }
        t = next;
    }
    Ok((t, distance(&curve.point_at(t), point)))
}
fn unique_parameters(mut values: Vec<f64>, lo: f64, hi: f64) -> Vec<f64> {
    values.sort_by(f64::total_cmp);
    let mut result = Vec::<f64>::new();
    for value in values {
        let value = value.clamp(lo, hi);
        if result
            .last()
            .is_none_or(|last| value - last > (hi - lo) * EPS * 16.0)
        {
            result.push(value);
        }
    }
    result
}
struct Box3 {
    lo: [f64; 3],
    hi: [f64; 3],
}
impl Box3 {
    fn new(curve: &NurbsCurve) -> Result<Self, String> {
        let p = curve.get_cv(0).ok_or("Missing curve control")?;
        let mut result = Self {
            lo: [p[0], p[1], p[2]],
            hi: [p[0], p[1], p[2]],
        };
        for i in 1..curve.cv_count() {
            let p = curve.get_cv(i).ok_or("Missing curve control")?;
            for d in 0..3 {
                result.lo[d] = result.lo[d].min(p[d]);
                result.hi[d] = result.hi[d].max(p[d]);
            }
        }
        Ok(result)
    }
    fn diagonal(&self) -> f64 {
        (self.hi[0] - self.lo[0])
            .hypot(self.hi[1] - self.lo[1])
            .hypot(self.hi[2] - self.lo[2])
    }
    fn overlaps(&self, other: &Self, tolerance: f64) -> bool {
        (0..3)
            .all(|d| self.hi[d] + tolerance >= other.lo[d] && other.hi[d] + tolerance >= self.lo[d])
    }
}
fn flat(curve: &NurbsCurve, tolerance: f64) -> Result<bool, String> {
    let a = curve.point_at_start();
    let b = curve.point_at_end();
    let v = subtract(&b, &a);
    let length2 = dot(&v, &v);
    if length2 <= tolerance * tolerance {
        return Ok(Box3::new(curve)?.diagonal() <= tolerance);
    }
    for i in 0..curve.cv_count() {
        let p = curve.get_cv(i).ok_or("Missing curve control")?;
        let t = dot(&subtract(&p, &a), &v) / length2;
        if !(-EPS..=1.0 + EPS).contains(&t)
            || distance(
                &p,
                &Point::new(a[0] + v[0] * t, a[1] + v[1] * t, a[2] + v[2] * t),
            ) > tolerance
        {
            return Ok(false);
        }
    }
    Ok(true)
}
fn refine(a: &NurbsCurve, b: &NurbsCurve, mut ta: f64, mut tb: f64) -> (f64, f64) {
    let (a0, a1) = a.domain();
    let (b0, b1) = b.domain();
    for _ in 0..40 {
        let da = a.evaluate(ta, 1);
        let db = b.evaluate(tb, 1);
        let r = Point::new(
            da[0][0] - db[0][0],
            da[0][1] - db[0][1],
            da[0][2] - db[0][2],
        );
        let u = Point::new(da[1][0], da[1][1], da[1][2]);
        let v = Point::new(db[1][0], db[1][1], db[1][2]);
        let aa = dot(&u, &u);
        let ab = dot(&u, &v);
        let bb = dot(&v, &v);
        let determinant = aa * bb - ab * ab;
        if determinant <= EPS * EPS * aa * bb {
            break;
        }
        let ar = dot(&u, &r);
        let br = dot(&v, &r);
        let na = (ta + (-bb * ar + ab * br) / determinant).clamp(a0, a1);
        let nb = (tb + (-ab * ar + aa * br) / determinant).clamp(b0, b1);
        if (na - ta).abs() < EPS * (a1 - a0) && (nb - tb).abs() < EPS * (b1 - b0) {
            ta = na;
            tb = nb;
            break;
        }
        ta = na;
        tb = nb;
    }
    (ta, tb)
}
fn intersections(
    a: &NurbsCurve,
    b: &NurbsCurve,
    tolerance: f64,
    budget: &mut usize,
) -> Result<Vec<(f64, f64)>, String> {
    let av = a.get_span_vector();
    let bv = b.get_span_vector();
    require(
        av.len() > 1 && bv.len() > 1,
        "Split requires nonempty curve spans",
    )?;
    require(
        av.len() - 1 <= *budget / (bv.len() - 1),
        "Curve intersection exceeds the bounded split workload",
    )?;
    let mut work = Vec::new();
    for aa in av.windows(2) {
        for bb in bv.windows(2) {
            work.push((interval(a, aa[0], aa[1])?, interval(b, bb[0], bb[1])?, 0));
        }
    }
    let mut hits = Vec::<(f64, f64)>::new();
    while let Some((ca, cb, depth)) = work.pop() {
        require(
            *budget > 0,
            "Curve intersection exceeds the bounded split workload",
        )?;
        *budget -= 1;
        let ba = Box3::new(&ca)?;
        let bb = Box3::new(&cb)?;
        if !ba.overlaps(&bb, tolerance) {
            continue;
        }
        if (flat(&ca, tolerance * 0.1)? && flat(&cb, tolerance * 0.1)?) || depth >= 48 {
            let ap = ca.point_at_start();
            let aq = ca.point_at_end();
            let bp = cb.point_at_start();
            let bq = cb.point_at_end();
            let u = subtract(&aq, &ap);
            let v = subtract(&bq, &bp);
            let aa = dot(&u, &u);
            let ab = dot(&u, &v);
            let vv = dot(&v, &v);
            if aa > tolerance * tolerance
                && vv > tolerance * tolerance
                && aa * vv - ab * ab < EPS * EPS * aa * vv
            {
                let t0 = dot(&subtract(&bp, &ap), &u) / aa;
                let t1 = dot(&subtract(&bq, &ap), &u) / aa;
                let gap = distance(
                    &bp,
                    &Point::new(ap[0] + u[0] * t0, ap[1] + u[1] * t0, ap[2] + u[2] * t0),
                );
                require(
                    !(gap <= tolerance
                        && 1.0_f64.min(t0.max(t1)) - 0.0_f64.max(t0.min(t1))
                            > tolerance / aa.sqrt()),
                    "Overlapping curves do not define isolated split points",
                )?;
            }
            let (ta, tb, d) = Closest::curve_curve(&ca, &cb);
            if d > tolerance * 2.0 {
                continue;
            }
            let (ta, tb) = refine(&ca, &cb, ta, tb);
            if distance(&a.point_at(ta), &b.point_at(tb)) > tolerance {
                continue;
            }
            if !hits.iter().any(|hit| {
                distance(&a.point_at(hit.0), &a.point_at(ta)) <= tolerance * 2.0
                    && distance(&a.point_at((hit.0 + ta) * 0.5), &a.point_at(ta)) <= tolerance * 2.0
                    && distance(&b.point_at(hit.1), &b.point_at(tb)) <= tolerance * 2.0
                    && distance(&b.point_at((hit.1 + tb) * 0.5), &b.point_at(tb)) <= tolerance * 2.0
            }) {
                hits.push((ta, tb));
            }
        } else if ba.diagonal() >= bb.diagonal() {
            let (lo, hi) = ca.domain();
            let mid = (lo + hi) * 0.5;
            work.push((interval(&ca, lo, mid)?, cb.clone(), depth + 1));
            work.push((interval(&ca, mid, hi)?, cb, depth + 1));
        } else {
            let (lo, hi) = cb.domain();
            let mid = (lo + hi) * 0.5;
            work.push((ca.clone(), interval(&cb, lo, mid)?, depth + 1));
            work.push((ca, interval(&cb, mid, hi)?, depth + 1));
        }
    }
    hits.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
    Ok(hits)
}

/// Split at isolated 3D intersections, retaining every original interval. No projection.
/// Invalid inputs and overlapping cutters return an error; a valid no-op returns one copy.
pub fn split_curve_by_curves(
    curve: &NurbsCurve,
    cutters: &[NurbsCurve],
    tolerance: f64,
) -> Result<Vec<NurbsCurve>, String> {
    check_tolerance(tolerance)?;
    check_curve(curve)?;
    require(!cutters.is_empty(), "Select at least one cutter")?;
    let (lo, hi) = curve.domain();
    let mut cuts = vec![lo, hi];
    let mut cut_at_seam = false;
    let mut budget = WORK_LIMIT;
    for cutter in cutters {
        check_curve(cutter)?;
        for (a, _) in intersections(curve, cutter, tolerance, &mut budget)? {
            if (a - lo).abs() <= (hi - lo) * EPS * 16.0 || (a - hi).abs() <= (hi - lo) * EPS * 16.0
            {
                cut_at_seam = true;
            }
            cuts.push(a);
        }
    }
    let cuts = unique_parameters(cuts, lo, hi);
    if cuts.len() == 2 {
        return Ok(vec![curve.clone()]);
    }
    let mut result = cuts
        .windows(2)
        .map(|span| interval(curve, span[0], span[1]))
        .collect::<Result<Vec<_>, _>>()?;
    if curve.is_closed() && result.len() > 1 && !cut_at_seam {
        let joined = NurbsCurve::join(
            &[result.last().unwrap().clone(), result[0].clone()],
            Some(tolerance),
        );
        require(
            joined.len() == 1,
            "Cannot join the uncut seam of a closed curve",
        )?;
        result[0] = joined[0].clone();
        result.pop();
    }
    Ok(result)
}

fn surface_point(surface: &NurbsSurface, u: f64, v: f64) -> Result<Point, String> {
    surface
        .point_at(u, v)
        .ok_or_else(|| "Cannot evaluate the source surface".into())
}
// The released kernel uses Option<f64>; the current kernel accepts f64.
#[allow(clippy::useless_conversion)]
fn pullback(
    surface: &NurbsSurface,
    curve: &NurbsCurve,
    tolerance: f64,
) -> Result<Vec<NurbsCurve>, String> {
    if surface.m_cv_count == [2, 2] && surface.m_order == [2, 2] && !surface.m_is_rat {
        let p = surface.get_cv(0, 0).ok_or("Missing surface control")?;
        let u = subtract(&surface.get_cv(1, 0).ok_or("Missing surface control")?, &p);
        let v = subtract(&surface.get_cv(0, 1).ok_or("Missing surface control")?, &p);
        let last = surface.get_cv(1, 1).ok_or("Missing surface control")?;
        let uu = dot(&u, &u);
        let uv = dot(&u, &v);
        let vv = dot(&v, &v);
        let determinant = uu * vv - uv * uv;
        if determinant > EPS * EPS * uu * vv
            && distance(
                &last,
                &Point::new(p[0] + u[0] + v[0], p[1] + u[1] + v[1], p[2] + u[2] + v[2]),
            ) <= tolerance
        {
            let mut result = curve.duplicate();
            let (u0, u1) = surface.domain(0).ok_or("Invalid surface domain")?;
            let (v0, v1) = surface.domain(1).ok_or("Invalid surface domain")?;
            for i in 0..curve.cv_count() {
                let q = curve.get_cv(i).ok_or("Missing curve control")?;
                let d = subtract(&q, &p);
                let du = dot(&d, &u);
                let dv = dot(&d, &v);
                let a = (du * vv - dv * uv) / determinant;
                let b = (dv * uu - du * uv) / determinant;
                if distance(
                    &q,
                    &Point::new(
                        p[0] + a * u[0] + b * v[0],
                        p[1] + a * u[1] + b * v[1],
                        p[2] + a * u[2] + b * v[2],
                    ),
                ) > tolerance
                {
                    return Ok(vec![]);
                }
                let w = curve.weight(i);
                result.set_cv_4d(
                    i,
                    (u0 + a * (u1 - u0)) * w,
                    (v0 + b * (v1 - v0)) * w,
                    0.0,
                    w,
                );
            }
            return Ok(vec![result]);
        }
    }
    Ok(Closest::surface_curve(
        surface,
        curve,
        0.0,
        0.0,
        tolerance.into(),
    ))
}
fn polygon(curve: &NurbsCurve, tolerance: f64) -> Result<Vec<Point>, String> {
    let spans = curve.get_span_vector();
    let mut work = vec![];
    for span in spans.windows(2).rev() {
        work.push((interval(curve, span[0], span[1])?, 0));
    }
    let mut result = vec![];
    let mut visited = 0;
    while let Some((part, depth)) = work.pop() {
        visited += 1;
        require(
            visited <= WORK_LIMIT,
            "Trim sampling exceeds the bounded workload",
        )?;
        if flat(&part, tolerance * 0.25)? {
            result.push(part.point_at_start());
            continue;
        }
        require(depth < 40, "Trim sampling exceeds parameter precision")?;
        let (lo, hi) = part.domain();
        let mid = (lo + hi) * 0.5;
        work.push((interval(&part, mid, hi)?, depth + 1));
        work.push((interval(&part, lo, mid)?, depth + 1));
    }
    Ok(result)
}
fn inside(p: &Point, polygon: &[Point]) -> bool {
    let mut result = false;
    for i in 0..polygon.len() {
        let a = &polygon[i];
        let b = &polygon[(i + polygon.len() - 1) % polygon.len()];
        if (a[1] > p[1]) != (b[1] > p[1])
            && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0]
        {
            result = !result;
        }
    }
    result
}
fn inside_loops(p: &Point, loops: &[Vec<Point>]) -> bool {
    !loops.is_empty() && inside(p, &loops[0]) && !loops[1..].iter().any(|hole| inside(p, hole))
}
struct Source {
    edge: i32,
    world: NurbsCurve,
    uv: NurbsCurve,
}
#[derive(Clone, Copy)]
struct Run {
    source: usize,
    a: f64,
    b: f64,
}
type Loop = Vec<Run>;
type Region = Vec<Loop>;
fn graph_node(
    vertices: &mut Vec<Point>,
    outgoing: &mut Vec<Vec<usize>>,
    p: Point,
    tolerance: f64,
) -> usize {
    if let Some(i) = vertices
        .iter()
        .position(|q| distance(&p, q) <= tolerance * 4.0)
    {
        return i;
    }
    vertices.push(p);
    outgoing.push(vec![]);
    vertices.len() - 1
}
fn arrange(
    sources: &[Source],
    original_loops: &[Vec<Point>],
    tolerance: f64,
) -> Result<Vec<Region>, String> {
    struct Span {
        source: usize,
        a: f64,
        b: f64,
        cuts: Vec<f64>,
        curve: NurbsCurve,
    }
    let mut spans = vec![];
    for (si, source) in sources.iter().enumerate() {
        let mut knots = source.uv.get_span_vector();
        // A closed Bezier span needs distinct graph nodes on its interior.
        if knots.len() == 2 && source.uv.is_closed() {
            let (lo, hi) = (knots[0], knots[1]);
            knots = (0..=4).map(|i| lo + (hi - lo) * i as f64 / 4.).collect();
        }
        for pair in knots.windows(2) {
            spans.push(Span {
                source: si,
                a: pair[0],
                b: pair[1],
                cuts: pair.to_vec(),
                curve: interval(&source.uv, pair[0], pair[1])?,
            });
        }
    }
    require(
        !spans.is_empty() && spans.len() <= WORK_LIMIT / spans.len(),
        "Face split exceeds the bounded workload",
    )?;
    let mut budget = WORK_LIMIT;
    for i in 0..spans.len() {
        for j in i + 1..spans.len() {
            for (a, b) in intersections(&spans[i].curve, &spans[j].curve, tolerance, &mut budget)? {
                spans[i].cuts.push(a);
                spans[j].cuts.push(b);
            }
        }
    }
    struct Directed {
        b: usize,
        run: Run,
    }
    let mut vertices = vec![];
    let mut edges = Vec::<Directed>::new();
    let mut outgoing = vec![];
    for span in spans {
        let cuts = span
            .cuts
            .into_iter()
            .map(|t| {
                let p = span.curve.point_at(t);
                if distance(&p, &span.curve.point_at(span.a)) <= tolerance {
                    span.a
                } else if distance(&p, &span.curve.point_at(span.b)) <= tolerance {
                    span.b
                } else {
                    t
                }
            })
            .collect();
        for pair in unique_parameters(cuts, span.a, span.b).windows(2) {
            let (lo, hi) = (pair[0], pair[1]);
            let source = &sources[span.source];
            if source.edge < 0
                && !inside_loops(&source.uv.point_at((lo + hi) * 0.5), original_loops)
            {
                continue;
            }
            let a = graph_node(
                &mut vertices,
                &mut outgoing,
                source.uv.point_at(lo),
                tolerance,
            );
            let b = graph_node(
                &mut vertices,
                &mut outgoing,
                source.uv.point_at(hi),
                tolerance,
            );
            if a == b {
                continue;
            }
            let index = edges.len();
            edges.push(Directed {
                b,
                run: Run {
                    source: span.source,
                    a: lo,
                    b: hi,
                },
            });
            edges.push(Directed {
                b: a,
                run: Run {
                    source: span.source,
                    a: hi,
                    b: lo,
                },
            });
            outgoing[a].push(index);
            outgoing[b].push(index + 1);
        }
    }
    let angle = |edge: usize| {
        let run = edges[edge].run;
        let d = sources[run.source].uv.evaluate(run.a, 1)[1].clone();
        let sign = if run.b > run.a { 1.0 } else { -1.0 };
        (sign * d[1]).atan2(sign * d[0])
    };
    for choices in &mut outgoing {
        choices.sort_by(|&a, &b| angle(a).total_cmp(&angle(b)));
    }
    struct Cycle {
        area: f64,
        paths: Loop,
        points: Vec<Point>,
    }
    let mut cycles = vec![];
    let mut used = vec![false; edges.len()];
    for initial in 0..edges.len() {
        if used[initial] {
            continue;
        }
        let mut paths = vec![];
        let mut points = vec![];
        let mut edge = initial;
        while !used[edge] {
            used[edge] = true;
            let item = &edges[edge];
            let run = item.run;
            paths.push(run);
            let mut part = interval(&sources[run.source].uv, run.a.min(run.b), run.a.max(run.b))?;
            if run.b < run.a {
                part.reverse();
            }
            points.extend(polygon(&part, tolerance)?);
            let options = &outgoing[item.b];
            let slot = options
                .iter()
                .position(|&e| e == edge ^ 1)
                .ok_or("Invalid trim graph adjacency")?;
            edge = options[(slot + options.len() - 1) % options.len()];
        }
        require(edge == initial, "Invalid trim graph cycle")?;
        let area = (0..points.len())
            .map(|i| {
                let a = &points[i];
                let b = &points[(i + 1) % points.len()];
                (a[0] * b[1] - b[0] * a[1]) * 0.5
            })
            .sum::<f64>();
        if area.abs() <= tolerance * tolerance {
            continue;
        }
        let run = paths[0];
        let curve = &sources[run.source].uv;
        let t = (run.a + run.b) * 0.5;
        let p = curve.point_at(t);
        let d = curve.evaluate(t, 1)[1].clone();
        let sign = if run.b > run.a { 1.0 } else { -1.0 };
        let length = d[0].hypot(d[1]);
        require(length > EPS, "Cannot orient a degenerate trim fragment")?;
        let left = Point::new(
            p[0] - sign * d[1] / length * tolerance * 8.0,
            p[1] + sign * d[0] / length * tolerance * 8.0,
            0.0,
        );
        if !inside_loops(&left, original_loops) {
            continue;
        }
        cycles.push(Cycle {
            area,
            paths,
            points,
        });
    }
    let positive: Vec<_> = cycles
        .iter()
        .enumerate()
        .filter_map(|(i, c)| (c.area > 0.0).then_some(i))
        .collect();
    let mut result: Vec<Region> = positive
        .iter()
        .map(|&i| vec![cycles[i].paths.clone()])
        .collect();
    for cycle in &cycles {
        if cycle.area >= 0.0 {
            continue;
        }
        let parent = positive
            .iter()
            .enumerate()
            .filter(|(_, index)| {
                let outer = &cycles[**index];
                outer.area > cycle.area.abs() + tolerance * tolerance
                    && inside(&cycle.points[0], &outer.points)
            })
            .min_by(|(_, a), (_, b)| cycles[**a].area.total_cmp(&cycles[**b].area))
            .map(|(i, _)| i)
            .ok_or("Unowned interior trim loop")?;
        result[parent].push(cycle.paths.clone());
    }
    Ok(result)
}
fn vertex(result: &mut BRep, p: &Point, tolerance: f64) -> usize {
    result
        .m_vertices
        .iter()
        .position(|v| distance(&v.point, p) <= tolerance)
        .unwrap_or_else(|| result.add_vertex(p, tolerance))
}
fn lifted_parameter(
    surface: &NurbsSurface,
    uv: &NurbsCurve,
    p: &Point,
    expected: f64,
    tolerance: f64,
) -> Result<f64, String> {
    let (lo, hi) = uv.domain();
    let gap = |t: f64| -> Result<f64, String> {
        let q = uv.point_at(t);
        Ok(distance(&surface_point(surface, q[0], q[1])?, p))
    };
    if gap(expected)? <= tolerance {
        return Ok(expected);
    }
    let mut best = expected;
    let mut d = gap(best)?;
    let mut index = 0usize;
    for i in 0..=128 {
        let t = lo + (hi - lo) * i as f64 / 128.0;
        let value = gap(t)?;
        if value < d {
            d = value;
            best = t;
            index = i;
        }
    }
    let mut a = lo + (hi - lo) * index.saturating_sub(1) as f64 / 128.0;
    let mut b = lo + (hi - lo) * (index + 1).min(128) as f64 / 128.0;
    for _ in 0..60 {
        let x = a + (b - a) / 3.0;
        let y = b - (b - a) / 3.0;
        if gap(x)? < gap(y)? {
            b = y;
        } else {
            a = x;
        }
    }
    let mid = (a + b) * 0.5;
    if gap(mid)? < d {
        best = mid;
    }
    require(
        gap(best)? <= tolerance * 4.0,
        "Cannot keep an adjacent trim on its original shared edge",
    )?;
    Ok(best)
}
fn validate(result: &BRep, original: &BRep, tolerance: f64) -> Result<(), String> {
    require(result.is_valid(), "Split produced invalid BRep references")?;
    for s in 0..original.m_shells.len() {
        if original.is_closed(s) {
            require(result.is_closed(s), "Split would open a joined shell")?;
        }
    }
    for face in &result.m_faces {
        for wire in &face.wires {
            let edges = result.wire_edges(wire);
            for i in 0..edges.len() {
                let a = &result.m_edges[edges[i].index as usize];
                let next = edges[(i + 1) % edges.len()];
                let b = &result.m_edges[next.index as usize];
                let tail = if edges[i].orientation == REVERSED {
                    a.start_vertex
                } else {
                    a.end_vertex
                };
                let head = if next.orientation == REVERSED {
                    b.end_vertex
                } else {
                    b.start_vertex
                };
                require(tail == head, "Split produced an open face boundary")?;
            }
        }
    }
    for edge in &result.m_edges {
        if edge.degenerated {
            continue;
        }
        let world = &result.m_curves_3d[edge.curve_3d_index as usize];
        require(
            distance(
                &world.point_at_start(),
                &result.m_vertices[edge.start_vertex as usize].point,
            ) <= tolerance * 4.0
                && distance(
                    &world.point_at_end(),
                    &result.m_vertices[edge.end_vertex as usize].point,
                ) <= tolerance * 4.0,
            "Split edge does not meet its vertices",
        )?;
        for pc in &edge.pcurves {
            for ci in [pc.curve_2d_index, pc.curve_2d_index_2] {
                if ci < 0 {
                    continue;
                }
                let uv = &result.m_curves_2d[ci as usize];
                let (lo, hi) = uv.domain();
                for k in 0..=32 {
                    let q = uv.point_at(lo + (hi - lo) * k as f64 / 32.0);
                    let p =
                        surface_point(&result.m_surfaces[pc.surface_index as usize], q[0], q[1])?;
                    require(
                        closest(world, &p)?.1 <= tolerance.max(edge.tolerance) * 8.0,
                        "Split edge and surface trim do not coincide",
                    )?;
                }
            }
        }
    }
    Ok(())
}

struct Piece {
    source: usize,
    lo: f64,
    hi: f64,
    edge: i32,
}
struct Builder<'a> {
    sources: &'a [Source],
    surface: &'a NurbsSurface,
    original: &'a BRep,
    surface_index: usize,
    tolerance: f64,
    result: BRep,
    pieces: Vec<Piece>,
    replacements: BTreeMap<i32, Vec<(f64, i32)>>,
}
impl Builder<'_> {
    fn edge(&mut self, run: &Run) -> Result<BRepRef, String> {
        let source = &self.sources[run.source];
        let uv = &source.uv;
        let qa = uv.point_at(run.a);
        let qb = uv.point_at(run.b);
        let pa = surface_point(self.surface, qa[0], qa[1])?;
        let pb = surface_point(self.surface, qb[0], qb[1])?;
        let parameter = |t: f64, p: &Point| -> Result<(f64, f64), String> {
            let (lo, hi) = source.world.domain();
            let (a, b) = uv.domain();
            let expected = lo + (t - a) / (b - a) * (hi - lo);
            let gap = distance(&source.world.point_at(expected), p);
            if gap <= self.tolerance {
                Ok((expected, gap))
            } else {
                closest(&source.world, p)
            }
        };
        let (mut wa, da) = parameter(run.a, &pa)?;
        let (mut wb, db) = parameter(run.b, &pb)?;
        require(
            da <= self.tolerance * 4.0 && db <= self.tolerance * 4.0,
            "Cutter is not on the selected surface",
        )?;
        let (w0, w1) = source.world.domain();
        let (c0, c1) = uv.domain();
        if source.world.is_closed() {
            if (wa - w0).abs() < (w1 - w0) * EPS && run.a > (c0 + c1) * 0.5 {
                wa = w1;
            }
            if (wb - w0).abs() < (w1 - w0) * EPS && run.b > (c0 + c1) * 0.5 {
                wb = w1;
            }
        }
        let lo = wa.min(wb);
        let hi = wa.max(wb);
        require(
            hi - lo > (w1 - w0) * EPS,
            "Split would create a collapsed edge",
        )?;
        let orientation = if wa < wb { FORWARD } else { REVERSED };
        for piece in &self.pieces {
            let same = if source.edge >= 0 {
                self.sources[piece.source].edge == source.edge
            } else {
                piece.source == run.source
            };
            if same
                && distance(&source.world.point_at(lo), &source.world.point_at(piece.lo))
                    <= self.tolerance * 4.0
                && distance(&source.world.point_at(hi), &source.world.point_at(piece.hi))
                    <= self.tolerance * 4.0
            {
                return Ok(BRepRef::new(piece.edge, orientation));
            }
        }
        let world = interval(&source.world, lo, hi)?;
        let a = vertex(
            &mut self.result,
            &world.point_at_start(),
            self.tolerance * 4.0,
        );
        let b = vertex(
            &mut self.result,
            &world.point_at_end(),
            self.tolerance * 4.0,
        );
        let ci = self.result.add_curve_3d(&world);
        let ei = self.result.add_edge(ci as i32, a as i32, b as i32);
        self.result.m_edges[ei].tolerance = self.tolerance;
        if source.edge >= 0 {
            let old = &self.original.m_edges[source.edge as usize];
            for pc in &old.pcurves {
                let mut ids = [-1, -1];
                for (at, ci) in [pc.curve_2d_index, pc.curve_2d_index_2]
                    .iter()
                    .copied()
                    .enumerate()
                {
                    if ci < 0 {
                        continue;
                    }
                    let c = &self.original.m_curves_2d[ci as usize];
                    let (c0, c1) = c.domain();
                    let surface = &self.original.m_surfaces[pc.surface_index as usize];
                    let ca = lifted_parameter(
                        surface,
                        c,
                        &world.point_at_start(),
                        c0 + (lo - w0) / (w1 - w0) * (c1 - c0),
                        self.tolerance,
                    )?;
                    let cb = lifted_parameter(
                        surface,
                        c,
                        &world.point_at_end(),
                        c0 + (hi - w0) / (w1 - w0) * (c1 - c0),
                        self.tolerance,
                    )?;
                    require(cb > ca, "A split crosses an unsupported periodic trim seam")?;
                    ids[at] = self.result.add_curve_2d(&interval(c, ca, cb)?) as i32;
                }
                self.result
                    .add_pcurve(ei, pc.surface_index as usize, ids[0], ids[1]);
            }
            self.replacements
                .entry(source.edge)
                .or_default()
                .push((lo, ei as i32));
        } else {
            let mut pc = interval(uv, run.a.min(run.b), run.a.max(run.b))?;
            if (wb - wa) * (run.b - run.a) < 0.0 {
                pc.reverse();
            }
            let ci = self.result.add_curve_2d(&pc);
            self.result
                .add_pcurve(ei, self.surface_index, ci as i32, -1);
        }
        self.pieces.push(Piece {
            source: run.source,
            lo,
            hi,
            edge: ei as i32,
        });
        Ok(BRepRef::new(ei as i32, orientation))
    }
}

/// Partition one face inside its owning BRep, keeping every region and shared shell edge.
/// The input is unchanged on success or failure. Unsupported/invalid trim topology returns an error.
pub fn split_brep_face_by_curves(
    brep: &BRep,
    face_index: usize,
    cutters: &[NurbsCurve],
    tolerance: f64,
) -> Result<BRep, String> {
    check_tolerance(tolerance)?;
    require(brep.is_valid(), "Split requires a valid BRep")?;
    require(
        face_index < brep.face_count(),
        "Select one BRep face to split",
    )?;
    require(!cutters.is_empty(), "Select at least one cutter")?;
    let face = &brep.m_faces[face_index];
    let surface = &brep.m_surfaces[face.surface_index as usize];
    check_surface(surface)?;
    let (u0, u1) = surface.domain(0).ok_or("Invalid surface domain")?;
    let (v0, v1) = surface.domain(1).ok_or("Invalid surface domain")?;
    let origin = surface_point(surface, u0, v0)?;
    let scale = (distance(&origin, &surface_point(surface, u1, v0)?) / (u1 - u0))
        .max(distance(&origin, &surface_point(surface, u0, v1)?) / (v1 - v0));
    require(scale > EPS, "Cannot split a degenerate surface domain")?;
    let uv_tolerance = tolerance / scale;
    let mut sources = vec![];
    let mut original_loops = vec![];
    for wire in &face.wires {
        let mut points = vec![];
        for er in brep.wire_edges(wire) {
            let edge = &brep.m_edges[er.index as usize];
            require(!edge.degenerated, "Pole-edge splitting is not supported")?;
            let ci = brep.pcurve_index(er.index as usize, face_index, er.orientation);
            require(ci >= 0, "Face has no source UV boundary")?;
            let mut uv = brep.m_curves_2d[ci as usize].clone();
            check_curve(&uv)?;
            check_curve(&brep.m_curves_3d[edge.curve_3d_index as usize])?;
            sources.push(Source {
                edge: er.index,
                world: brep.m_curves_3d[edge.curve_3d_index as usize].clone(),
                uv: uv.clone(),
            });
            if er.orientation == REVERSED {
                uv.reverse();
            }
            points.extend(polygon(&uv, uv_tolerance)?);
        }
        require(points.len() >= 3, "Face has an invalid boundary")?;
        original_loops.push(points);
    }
    for cutter in cutters {
        check_curve(cutter)?;
        for uv in pullback(surface, cutter, tolerance)? {
            sources.push(Source {
                edge: -1,
                world: cutter.clone(),
                uv,
            });
        }
    }
    let regions = arrange(&sources, &original_loops, uv_tolerance)?;
    if regions.len() < 2 {
        return Ok(brep.clone());
    }
    let mut builder = Builder {
        sources: &sources,
        surface,
        original: brep,
        surface_index: face.surface_index as usize,
        tolerance,
        result: brep.clone(),
        pieces: vec![],
        replacements: BTreeMap::new(),
    };
    let mut new_wires = vec![];
    for region in regions {
        let mut wires = vec![];
        for paths in region {
            let refs = paths
                .iter()
                .map(|run| builder.edge(run))
                .collect::<Result<Vec<_>, _>>()?;
            let wi = builder.result.add_wire(&refs);
            wires.push(BRepRef::new(wi as i32, FORWARD));
        }
        new_wires.push(wires);
    }
    let Builder {
        mut result,
        mut replacements,
        ..
    } = builder;
    for items in replacements.values_mut() {
        items.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        items.dedup();
    }
    for (wi, wire) in brep.m_wires.iter().enumerate() {
        let mut refs = vec![];
        for er in &wire.edges {
            let Some(items) = replacements.get(&er.index) else {
                refs.push(*er);
                continue;
            };
            let mut items = items.clone();
            if er.orientation == REVERSED {
                items.reverse();
            }
            refs.extend(
                items
                    .into_iter()
                    .map(|(_, edge)| BRepRef::new(edge, er.orientation)),
            );
        }
        result.m_wires[wi].edges = refs;
    }
    result.m_faces[face_index].wires = new_wires[0].clone();
    let mut added = vec![];
    for wires in new_wires.into_iter().skip(1) {
        let mut next = face.clone();
        next.wires = wires;
        added.push(result.face_count());
        result.m_faces.push(next);
    }
    for shell in &mut result.m_shells {
        let mut refs = vec![];
        for fr in &shell.faces {
            refs.push(*fr);
            if fr.index == face_index as i32 {
                refs.extend(
                    added
                        .iter()
                        .map(|&i| BRepRef::new(i as i32, fr.orientation)),
                );
            }
        }
        shell.faces = refs;
    }
    validate(&result, brep, tolerance)?;
    Ok(result)
}

/// Wrap a standalone surface's natural boundary in a BRep, then retain all split regions.
/// Closed/pole natural boundaries require a BRep with explicit seam topology.
pub fn split_surface_by_curves(
    surface: &NurbsSurface,
    cutters: &[NurbsCurve],
    tolerance: f64,
) -> Result<BRep, String> {
    check_tolerance(tolerance)?;
    check_surface(surface)?;
    let mut result = BRep::new();
    let si = result.add_surface(surface);
    let (u0, u1) = surface.domain(0).ok_or("Invalid surface domain")?;
    let (v0, v1) = surface.domain(1).ok_or("Invalid surface domain")?;
    let uv = [
        Point::new(u0, v0, 0.0),
        Point::new(u1, v0, 0.0),
        Point::new(u1, v1, 0.0),
        Point::new(u0, v1, 0.0),
    ];
    let mut edges = vec![];
    for i in 0..4 {
        let mut curve = surface
            .iso_curve(i % 2, [v0, u1, v1, u0][i])
            .ok_or("Cannot extract a natural boundary")?;
        if i >= 2 {
            curve.reverse();
        }
        let a = vertex(&mut result, &curve.point_at_start(), tolerance);
        let b = vertex(&mut result, &curve.point_at_end(), tolerance);
        require(
            a != b,
            "Closed or pole boundaries need a BRep with explicit seam topology",
        )?;
        let ci = result.add_curve_3d(&curve);
        let edge = result.add_edge(ci as i32, a as i32, b as i32);
        result.m_edges[edge].tolerance = tolerance;
        let pc = NurbsCurve::create(false, 1, &[uv[i].clone(), uv[(i + 1) % 4].clone()]);
        let ci = result.add_curve_2d(&pc);
        result.add_pcurve(edge, si, ci as i32, -1);
        edges.push(BRepRef::new(edge as i32, FORWARD));
    }
    let wi = result.add_wire(&edges);
    result.add_face(si as i32, &[BRepRef::new(wi as i32, FORWARD)], 0.0);
    split_brep_face_by_curves(&result, 0, cutters, tolerance)
}

/// Split a line at isolated 3D intersections, retaining line types and display attributes.
pub fn split_line_by_curves(
    line: &crate::Line,
    cutters: &[NurbsCurve],
    tolerance: f64,
) -> Result<Vec<crate::Line>, String> {
    let curve = NurbsCurve::create(false, 1, &[line.point_at(0.), line.point_at(1.)]);
    split_curve_by_curves(&curve, cutters, tolerance)?
        .into_iter()
        .map(|piece| {
            let mut next = crate::Line::from_points(&piece.point_at_start(), &piece.point_at_end());
            next.name = line.name.clone();
            next.width = line.width;
            next.dash = line.dash.clone();
            next.linecolor = line.linecolor.clone();
            Ok(next)
        })
        .collect()
}
/// Split a polyline, retaining each original corner, piece order and display attributes.
pub fn split_polyline_by_curves(
    polyline: &crate::Polyline,
    cutters: &[NurbsCurve],
    tolerance: f64,
) -> Result<Vec<crate::Polyline>, String> {
    let curve = NurbsCurve::create(false, 1, &polyline.get_points());
    split_curve_by_curves(&curve, cutters, tolerance)?
        .into_iter()
        .map(|piece| {
            let mut next = crate::Polyline::new(
                piece
                    .get_span_vector()
                    .into_iter()
                    .map(|t| piece.point_at(t))
                    .collect(),
            );
            next.name = polyline.name.clone();
            next.width = polyline.width;
            next.dash = polyline.dash.clone();
            next.linecolor = polyline.linecolor.clone();
            Ok(next)
        })
        .collect()
}
```

### `../session_rust/src/lib.rs`

**TYPE THIS**

**CURRENT**

```rust
pub use xform::Xform;
```

**ADD BELOW**

```rust

/// Exact curve and trimmed face splits.
pub mod simple_split;
```

### `../session_rust/src/nurbscurve.rs`

**TYPE THIS**

**CURRENT**

```rust
        if trim_start {
            if !self.insert_nurbsknot(t0, p) {
                return false;
            }
        }
        if trim_end {
            if !self.insert_nurbsknot(t1, p) {
                return false;
```

**REPLACE WITH**

```rust
        if trim_start {
            // This checkpoint inserts additional knots; request only the missing multiplicity.
            let existing = self.m_nurbsknot.iter().filter(|&&k| (k - t0).abs() < Tolerance::ZERO_TOLERANCE).count();
            if existing < p && !self.insert_nurbsknot(t0, p - existing) {
                return false;
            }
        }
        if trim_end {
            // This checkpoint inserts additional knots; request only the missing multiplicity.
            let existing = self.m_nurbsknot.iter().filter(|&&k| (k - t1).abs() < Tolerance::ZERO_TOLERANCE).count();
            if existing < p && !self.insert_nurbsknot(t1, p - existing) {
                return false;
```

### `../session_rust/src/tree.rs`

**TYPE THIS**

**CURRENT**

```rust
/// A hierarchical data structure with parent-child relationships.
#[derive(Debug, Clone)]
pub struct Tree {
```

**REPLACE WITH**

```rust
/// A hierarchical data structure with parent-child relationships.
#[derive(Debug)]
pub struct Tree {
```

**TYPE THIS**

**CURRENT**

```rust
    root_node: Option<Rc<RefCell<TreeNode>>>,
```

**ADD BELOW**

```rust
}

/// Session copy-on-write and undo snapshots must own independent node hierarchies.
impl Clone for Tree {
    fn clone(&self) -> Self {
        Self {
            guid: self.guid.clone(),
            name: self.name.clone(),
            root_node: self.root_node.as_ref().map(|root| TreeNode::from_serde(root.borrow().to_serde())),
        }
    }
```

### `../session_rust/src/brep.rs`

**TYPE THIS**

**CURRENT**

```rust
                (polygon_signed_area(&outer).abs() - domain_area).abs() < 1e-3 * domain_area;
```

**ADD BELOW**

```rust
            // Topological edge ends must be domain corners; internal polyline controls are not new vertices.
            let mesh_brep = self;
            for er in mesh_brep.wire_edges(&face.wires[0]) {
                let ci = mesh_brep.pcurve_index(er.index as usize, fi, er.orientation);
                if ci < 0 { continue; }
                let curve = &mesh_brep.m_curves_2d[ci as usize];
                for k in [0, curve.cv_count().saturating_sub(1)] {
                    let p = curve.get_cv(k).unwrap_or_default();
                    let corner_u = (p[0] - u0).abs().min((p[0] - u1).abs()) <= (u1 - u0) * 1e-9;
                    let corner_v = (p[1] - v0).abs().min((p[1] - v1).abs()) <= (v1 - v0) * 1e-9;
                    if !corner_u || !corner_v { face_direct[fi] = false; }
                }
            }
```

### `src/app/command.rs`

**TYPE THIS**

**CURRENT**

```rust
    Scale(f64),
```

**ADD BELOW**

```rust
    Split,
```

**TYPE THIS**

**CURRENT**

```rust
        "line" => "Line start end · Example: Line 0,0,0 100,0,0",
```

**ADD BELOW**

```rust
        "curve" => "Curve control points… · Example: Curve 0,0,0 50,100,0 100,0,0",
```

**TYPE THIS**

**CURRENT**

```rust
        "explode" => "Select a polyline · Explode creates its individual line segments",
```

**ADD BELOW**

```rust
        "split" => {
            "Select a curve or face · Split · choose cutter curves · Enter confirms · Esc cancels"
        }
```

**TYPE THIS**

**CURRENT**

```rust
        "rotate" | "rot" => Some(2),
        "save" | "open" | "delete" | "del" | "undo" | "redo" | "hide" | "show" | "fit"
        | "escape" | "esc" => Some(0),
        _ => None,
    };
    if expected.is_some_and(|count| rest.len() != count) {
        return Err(format!("wrong number of arguments for `{verb}`"));
    }
    match verb.as_str() {
        "point" | "line" | "polyline" | "trim" | "extend" | "explode" => {
            model(&verb, &rest).map(Command::Model)
```

**REPLACE WITH**

```rust
        "rotate" | "rot" => Some(2),
        "split" | "save" | "open" | "delete" | "del" | "undo" | "redo" | "hide" | "show"
        | "fit" | "escape" | "esc" => Some(0),
        _ => None,
    };
    if expected.is_some_and(|count| rest.len() != count) {
        return Err(format!("wrong number of arguments for `{verb}`"));
    }
    match verb.as_str() {
        "point" | "line" | "polyline" | "curve" | "trim" | "extend" | "explode" => {
            model(&verb, &rest).map(Command::Model)
```

**TYPE THIS**

**CURRENT**

```rust
        "save" => Ok(Command::Save),
```

**ADD ABOVE**

```rust
        "split" => Ok(Command::Split),
```

**TYPE THIS**

**CURRENT**

```rust
        }
        "point" | "line" | "polyline" => {
            let mut points = Vec::new();
```

**REPLACE WITH**

```rust
        }
        "point" | "line" | "polyline" | "curve" => {
            let mut points = Vec::new();
```

**TYPE THIS**

**CURRENT**

```rust
                ("polyline", 2..) => Ok(Modeling::Polyline(points)),
                _ => Err("point needs one coordinate; line two; polyline at least two".into()),
            }
```

**REPLACE WITH**

```rust
                ("polyline", 2..) => Ok(Modeling::Polyline(points)),
                ("curve", 2..) => Ok(Modeling::Curve(points)),
                _ => {
                    Err("point needs one coordinate; line two; polyline/curve at least two".into())
                }
            }
```

### `src/app/input.rs`

**TYPE THIS**

**CURRENT**

```rust
            Key::Named(NamedKey::Escape) => state.escape_selection(),
```

**ADD BELOW**

```rust
            Key::Named(NamedKey::Enter) => state.confirm_split(),
```

### `src/app/inspection.rs`

**TYPE THIS**

**CURRENT**

```rust
    snapshot["color_count"] = serde_json::json!(state.scene.colors.len());
```

**ADD BELOW**

```rust
    snapshot["split"] = serde_json::json!(state.split_status());
    snapshot["source_faces"] =
        serde_json::json!(parent.and_then(|row| match state.scene.geometry(row)? {
            session_rust::Geometry::BRep(brep) => Some(brep.face_count()),
            session_rust::Geometry::Element(element) => match element.geometry() {
                session_rust::element::ElementGeometry::BRep(brep) => Some(brep.face_count()),
                _ => None,
            },
            _ => None,
        }));
```

### `src/app/mod.rs`

**TYPE THIS**

**CURRENT**

```rust
pub mod surface_preview;
```

**ADD BELOW**

```rust

pub mod splitting;
```

### `src/app/modeling.rs`

**TYPE THIS**

**CURRENT**

```rust
    Polyline(Vec<[f64; 3]>),
```

**ADD BELOW**

```rust
    Curve(Vec<[f64; 3]>),
```

**TYPE THIS**

**CURRENT**

```rust
                self.create_geometry(Geometry::Polyline(Rc::new(Polyline::new(points))))
```

**ADD BELOW**

```rust
            }
            Modeling::Curve(points) => {
                if !(2..=MAX_POINTS).contains(&points.len()) {
                    return Err(format!("curve needs 2–{MAX_POINTS} control points"));
                }
                let points = points
                    .iter()
                    .map(|p| point(*p))
                    .collect::<Result<Vec<_>, _>>()?;
                let curve =
                    session_rust::NurbsCurve::create(false, (points.len() - 1).min(3), &points);
                self.create_geometry(Geometry::NurbsCurve(Rc::new(curve)))
```

**TYPE THIS**

**CURRENT**

```rust
                debug_assert!(added.is_some());
```

**ADD BELOW**

```rust
            }
            Geometry::NurbsCurve(curve) => {
                session.add_nurbscurve((*curve).clone(), None);
```

### `src/app/scene.rs`

**TYPE THIS**

**CURRENT**

```rust
        self.tables.bounds.union(&extent);
        if is_planar(&self.tables, &from, &place.m) {
            mark_sheet(&mut self.tables, &from);
```

**REPLACE WITH**

```rust
        self.tables.bounds.union(&extent);
        // Typed modeling geometry stays in the 3D workspace even when all its points are coplanar.
        if self.created_doc != Some(self.docs.len()) && is_planar(&self.tables, &from, &place.m) {
            mark_sheet(&mut self.tables, &from);
```

### `src/app/session_io.rs`

**TYPE THIS**

**CURRENT**

```rust
struct Metadata {
```

**ADD BELOW**

```rust
    #[serde(default)]
    created_doc: Option<usize>,
```

**TYPE THIS**

**CURRENT**

```rust
    let metadata = Metadata {
```

**ADD BELOW**

```rust
        created_doc: scene.created_doc,
```

**TYPE THIS**

**CURRENT**

```rust
    }
    let mut scene = Scene::new();
    for (meta, bytes) in metadata.documents.into_iter().zip(archive.documents) {
```

**REPLACE WITH**

```rust
    }
    if metadata
        .created_doc
        .is_some_and(|index| index >= metadata.documents.len())
    {
        return Err("Created document index is outside the inventory".into());
    }
    let mut scene = Scene::new();
    scene.created_doc = metadata.created_doc;
    for (meta, bytes) in metadata.documents.into_iter().zip(archive.documents) {
```

**TYPE THIS**

**CURRENT**

```rust
    use session_rust::{Geometry, Mesh, Point};
```

**ADD BELOW**

```rust
    #[test]
    fn created_curves_keep_visible_screen_pens_after_open() {
        let mut scene = Scene::new();
        scene
            .model(&crate::app::modeling::Modeling::Line(
                [-3000., -5000., 200.],
                [-3000., -1000., 200.],
            ))
            .unwrap();
        let restored = open(&save(&scene).unwrap()).unwrap();
        assert_eq!(restored.created_doc, Some(0));
        assert_eq!(restored.tables.seg.ribbons.len(), 1);
        assert_eq!(restored.tables.seg.ribbons[0].radius, 0.);
        assert_eq!(
            restored.tables.obj.rows[0].flags & crate::engine::gpu::Instance::FLAG_SHEET,
            0
        );
    }
```

### `src/app/splitting.rs`

**NEW FILE · TYPE THIS**

```rust
//! Source curve/face splits; the kernel computes every region before the session is changed.
use super::scene::Scene;
use session_rust::simple_split;
use session_rust::{BRep, Geometry, NurbsCurve, Xform};
use std::rc::Rc;

pub fn is_cutter(geometry: &Geometry) -> bool {
    matches!(
        geometry,
        Geometry::Line(_) | Geometry::Polyline(_) | Geometry::NurbsCurve(_)
    )
}

pub fn face_index(geometry: &Geometry, selected: Option<usize>) -> Result<Option<usize>, String> {
    match geometry {
        Geometry::Line(_)
        | Geometry::Polyline(_)
        | Geometry::NurbsCurve(_)
        | Geometry::NurbsSurface(_) => Ok(None),
        Geometry::BRep(brep) => brep_face(brep, selected).map(Some),
        Geometry::Element(element) => match element.geometry() {
            session_rust::element::ElementGeometry::BRep(brep) => {
                brep_face(brep, selected).map(Some)
            }
            _ => Err("Split accepts curves and NURBS/BRep faces".into()),
        },
        _ => Err("Split accepts lines, polylines, NURBS curves and surface faces".into()),
    }
}
fn brep_face(brep: &BRep, selected: Option<usize>) -> Result<usize, String> {
    selected
        .or((brep.face_count() == 1).then_some(0))
        .filter(|face| *face < brep.face_count())
        .ok_or_else(|| {
            "Ctrl+Shift-select one BRep face before Split; the solid stays joined".into()
        })
}
fn curve(geometry: &Geometry) -> Result<NurbsCurve, String> {
    match geometry {
        Geometry::Line(line) => Ok(NurbsCurve::create(
            false,
            1,
            &[line.point_at(0.), line.point_at(1.)],
        )),
        Geometry::Polyline(polyline) => Ok(NurbsCurve::create(false, 1, &polyline.get_points())),
        Geometry::NurbsCurve(curve) => Ok((**curve).clone()),
        _ => Err("Choose a line, polyline or NURBS curve as cutter".into()),
    }
}

impl Scene {
    /// Preserve the original object identity, placement and tree node; add curve pieces as siblings.
    pub fn split_rows(
        &mut self,
        target: u32,
        face: Option<usize>,
        cutters: &[u32],
    ) -> Result<usize, String> {
        if !self.streamed.is_empty() || !self.sheets.is_empty() {
            return Err("Splitting requires complete retained source documents".into());
        }
        if cutters.is_empty() || cutters.len() > 64 {
            return Err("Choose 1–64 cutter curves".into());
        }
        if !self.selectable(target) {
            return Err("Unlock the target before splitting".into());
        }
        let (doc, guid) = self.identity_of(target).ok_or("Target no longer exists")?;
        let file = self.docs.get(doc).ok_or("Target has no source document")?;
        if file.display_only {
            return Err("Target is display only".into());
        }
        let back = Xform::from_matrix(self.placement_of(target).ok_or("Target has no placement")?)
            .inverse()
            .ok_or("Target placement is singular")?;
        let mut tools = Vec::new();
        for &row in cutters {
            if row == target || !self.selectable(row) {
                return Err("Choose an unlocked cutter distinct from the target".into());
            }
            let mut cutter = curve(self.geometry(row).ok_or("Cutter no longer exists")?)?;
            let place =
                Xform::from_matrix(self.placement_of(row).ok_or("Cutter has no placement")?);
            if place.inverse().is_none() {
                return Err("Cannot transform the cutter into target coordinates".into());
            }
            cutter.transform(&(&back * &place));
            tools.push(cutter);
        }
        let source = self
            .geometry(target)
            .ok_or("Source geometry is unavailable")?;
        let face = face_index(source, face)?;
        let tolerance = 1e-6;
        let (mut pieces, regions) = match source {
            Geometry::Line(line) => {
                let pieces: Vec<_> = simple_split::split_line_by_curves(line, &tools, tolerance)?
                    .into_iter()
                    .map(|p| Geometry::Line(Rc::new(p)))
                    .collect();
                let count = pieces.len();
                (pieces, count)
            }
            Geometry::Polyline(line) => {
                let pieces: Vec<_> =
                    simple_split::split_polyline_by_curves(line, &tools, tolerance)?
                        .into_iter()
                        .map(|p| Geometry::Polyline(Rc::new(p)))
                        .collect();
                let count = pieces.len();
                (pieces, count)
            }
            Geometry::NurbsCurve(curve) => {
                let pieces: Vec<_> = simple_split::split_curve_by_curves(curve, &tools, tolerance)?
                    .into_iter()
                    .map(|p| Geometry::NurbsCurve(Rc::new(p)))
                    .collect();
                let count = pieces.len();
                (pieces, count)
            }
            Geometry::NurbsSurface(surface) => {
                let mut brep = simple_split::split_surface_by_curves(surface, &tools, tolerance)?;
                brep.name = surface.name.clone();
                let count = brep.face_count();
                (vec![Geometry::BRep(Rc::new(brep))], count)
            }
            Geometry::BRep(brep) => {
                let next = simple_split::split_brep_face_by_curves(
                    brep,
                    face.ok_or("Select a face")?,
                    &tools,
                    tolerance,
                )?;
                let count = next.face_count() - brep.face_count() + 1;
                (vec![Geometry::BRep(Rc::new(next))], count)
            }
            Geometry::Element(element) => {
                let session_rust::element::ElementGeometry::BRep(brep) = element.geometry() else {
                    return Err("Element has no BRep".into());
                };
                let next = simple_split::split_brep_face_by_curves(
                    brep,
                    face.ok_or("Select a face")?,
                    &tools,
                    tolerance,
                )?;
                let count = next.face_count() - brep.face_count() + 1;
                let mut result = (**element).clone();
                result.set_brep_geometry(next);
                (vec![Geometry::Element(Rc::new(result))], count)
            }
            _ => return Err("Unsupported split target".into()),
        };
        if regions < 2 {
            return Ok(1);
        }
        let parent_name = file
            .session
            .tree
            .get_node_by_name(&guid)
            .and_then(|node| node.borrow().parent())
            .map(|node| node.borrow().name.clone());
        let place = file.session.xform(&guid);
        let color = self.colors.get(&(doc, Rc::clone(&guid))).copied();
        if pieces.len() > 1 {
            let name = source.name();
            for (index, piece) in pieces.iter_mut().enumerate() {
                let name = format!("{name} (part {})", index + 1);
                match piece {
                    Geometry::Line(p) => Rc::make_mut(p).name = name,
                    Geometry::Polyline(p) => Rc::make_mut(p).name = name,
                    Geometry::NurbsCurve(p) => Rc::make_mut(p).name = name,
                    _ => unreachable!("only curves create sibling objects"),
                }
            }
        }
        let first = pieces.remove(0);
        let session = Rc::make_mut(&mut self.docs[doc].session);
        // Resolve the parent after copy-on-write, inside the edited document's tree.
        let parent = parent_name.and_then(|name| session.tree.get_node_by_name(&name));
        session.begin("split");
        // All fallible geometry work has completed; these validated pieces have at least two controls.
        let replaced = session.replace(&guid, first);
        debug_assert!(replaced);
        for piece in pieces {
            let node = match piece {
                Geometry::Line(piece) => session.add_line((*piece).clone(), parent.as_ref()),
                Geometry::Polyline(piece) => session
                    .add_polyline((*piece).clone(), parent.as_ref())
                    .expect("valid split polyline"),
                Geometry::NurbsCurve(piece) => session
                    .add_nurbscurve((*piece).clone(), parent.as_ref())
                    .expect("valid split curve"),
                _ => unreachable!("only curves create sibling objects"),
            };
            let id = node.borrow().name.clone();
            session.set_xform(&id, place.clone());
            if let Some(color) = color {
                self.colors.insert((doc, Rc::from(id)), color);
            }
        }
        session.commit();
        self.last_edited = Some(doc);
        Ok(regions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::scene::FileDoc;
    use session_rust::{Line, Point, Session};
    fn add(scene: &mut Scene, session: Rc<Session>, name: &str, place: Xform) {
        scene.add_file(FileDoc {
            name: name.into(),
            session,
            place,
            point_px: 0.,
            display_only: false,
        });
    }
    #[test]
    fn split_preserves_tree_placement_and_other_shared_documents_and_undo() {
        let mut session = Session::new("shared");
        let group = session.add_group("parts");
        session.set_xform(&group.borrow().name, Xform::translation(10., 0., 0.));
        let target = session.add_line(
            Line::from_points(&Point::new(-2., 0., 0.), &Point::new(2., 0., 0.)),
            Some(&group),
        );
        let id = target.borrow().name.clone();
        let shared = Rc::new(session);
        let mut scene = Scene::new();
        add(
            &mut scene,
            Rc::clone(&shared),
            "first",
            Xform::translation(100., 0., 0.),
        );
        add(
            &mut scene,
            Rc::clone(&shared),
            "second",
            Xform::translation(200., 0., 0.),
        );
        let mut cutters = Session::new("cutters");
        cutters.add_line(
            Line::from_points(&Point::new(110., -2., 0.), &Point::new(110., 2., 0.)),
            None,
        );
        add(&mut scene, Rc::new(cutters), "cutters", Xform::identity());
        scene
            .colors
            .insert(scene.identity_of(0).unwrap(), [60, 170, 100]);
        assert_eq!(scene.split_rows(0, None, &[2]).unwrap(), 2);
        let first = &scene.docs[0].session;
        assert_eq!(first.objects.lines.len(), 2);
        assert_eq!(scene.docs[1].session.objects.lines.len(), 1);
        assert_eq!(shared.objects.lines.len(), 1);
        let parent = first.tree.get_node_by_name("parts").unwrap();
        assert_eq!(parent.borrow().children().len(), 2);
        assert_eq!(
            shared
                .tree
                .get_node_by_name("parts")
                .unwrap()
                .borrow()
                .children()
                .len(),
            1
        );
        assert!(first.lookup.contains_key(&id));
        let Geometry::Line(line) = &first.lookup[&id] else {
            panic!()
        };
        assert!(line.point_at(1.).distance(&Point::new(0., 0., 0.), None) < 1e-6);
        assert_eq!(scene.colors.len(), 2);
        assert!(scene.undo());
        assert_eq!(scene.docs[0].session.objects.lines.len(), 1);
        assert!(scene.redo());
        assert_eq!(scene.docs[0].session.objects.lines.len(), 2);
        let bytes = crate::app::session_io::save(&scene).unwrap();
        let restored = crate::app::session_io::open(&bytes).unwrap();
        assert_eq!(restored.docs[0].session.objects.lines.len(), 2);
    }
    #[test]
    fn split_face_keeps_solid_joined_and_invalid_cut_preserves_source() {
        let brep = BRep::create_box(10., 10., 10.);
        let original_area = brep.face_meshes_q(Some((20., 0.005)))[0].area();
        let s = &brep.m_surfaces[0];
        let a = s.get_cv(0, 0).unwrap();
        let u = s.get_cv(1, 0).unwrap();
        let v = s.get_cv(0, 1).unwrap();
        let p = |x: f64, y: f64| {
            Point::new(
                a[0] + x * (u[0] - a[0]) + y * (v[0] - a[0]),
                a[1] + x * (u[1] - a[1]) + y * (v[1] - a[1]),
                a[2] + x * (u[2] - a[2]) + y * (v[2] - a[2]),
            )
        };
        let mut session = Session::new("box");
        let node = session.add_brep(brep, None).unwrap();
        let id = node.borrow().name.clone();
        session.add_line(Line::from_points(&p(0.5, -1.), &p(0.5, 2.)), None);
        let mut scene = Scene::new();
        add(&mut scene, Rc::new(session), "box", Xform::identity());
        let row = (0..scene.object_count() as u32)
            .find(|&row| matches!(scene.geometry(row), Some(Geometry::BRep(_))))
            .unwrap();
        let cutter = (0..scene.object_count() as u32)
            .find(|&row| matches!(scene.geometry(row), Some(Geometry::Line(_))))
            .unwrap();
        assert!(scene.split_rows(row, None, &[cutter]).is_err());
        assert_eq!(scene.split_rows(row, Some(0), &[cutter]).unwrap(), 2);
        let Geometry::BRep(result) = &scene.docs[0].session.lookup[&id] else {
            panic!()
        };
        assert_eq!(result.face_count(), 7);
        assert!(result.is_solid());
        let meshes = result.face_meshes_q(Some((20., 0.005)));
        assert!(
            (meshes[0].area() - original_area / 2.).abs() < 1e-6,
            "first region area: {} of {original_area}",
            meshes[0].area()
        );
        assert!(
            (meshes[6].area() - original_area / 2.).abs() < 1e-6,
            "second region area: {} of {original_area}",
            meshes[6].area()
        );
        assert!(scene.undo());
        let Geometry::BRep(original) = &scene.docs[0].session.lookup[&id] else {
            panic!()
        };
        assert_eq!(original.face_count(), 6);
        assert!(original.is_solid());
    }
}
```

### `src/app/ui.rs`

**TYPE THIS**

**CURRENT**

```rust
                    model.command_open = false;
                    response.surrender_focus();
                    crate::app::feedback::focus_canvas();
```

**REPLACE WITH**

```rust
                    model.command_open = false;
                    model.focus_command = false;
                    response.surrender_focus();
                    close.surrender_focus();
                    crate::app::feedback::focus_canvas();
```

**TYPE THIS**

**CURRENT**

```rust
    ("Undo", "Undo the last edit", "undo"),
```

**ADD ABOVE**

```rust
    (
        "Split",
        "Split selected curve or face with cutter curves",
        "split",
    ),
```

**TYPE THIS**

**CURRENT**

```rust
                        if command.contains(' ') {
```

**ADD ABOVE**

```rust
                        response.surrender_focus();
```

### `src/shaders/ribbon.wgsl`

**TYPE THIS**

**CURRENT**

```wgsl
@fragment
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    if (coverage(in) < 0.5 || !ink_visible(in.pos.xy, ink_axis(in), 0u)) {
        discard;
```

**REPLACE WITH**

```wgsl
@fragment
// Keep visible hairlines pickable even when their coverage is shared across adjacent pixels.
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    if (coverage(in) <= 0.0 || !ink_visible(in.pos.xy, ink_axis(in), 0u)) {
        discard;
```

**TYPE THIS**

**CURRENT**

```wgsl
fn fs_edge_id(in: VsOut) -> @location(0) vec2<u32> {
    if (in.source_edge == 0xffffffffu || coverage(in) < 0.5 || !ink_visible(in.pos.xy, ink_axis(in), 0u)) {
        discard;
```

**REPLACE WITH**

```wgsl
fn fs_edge_id(in: VsOut) -> @location(0) vec2<u32> {
    if (in.source_edge == 0xffffffffu || coverage(in) <= 0.0 || !ink_visible(in.pos.xy, ink_axis(in), 0u)) {
        discard;
```

### `src/state.rs`

**TYPE THIS**

**CURRENT**

```rust
mod sheet_query;
```

**ADD BELOW**

```rust
mod splitting;
```

**TYPE THIS**

**CURRENT**

```rust
    hierarchy: crate::app::hierarchy::Hierarchy,
```

**ADD BELOW**

```rust
    pending_split: Option<splitting::Pending>,
```

**TYPE THIS**

**CURRENT**

```rust
            hierarchy: Default::default(),
```

**ADD BELOW**

```rust
            pending_split: None,
```

**TYPE THIS**

**CURRENT**

```rust
    pub fn clear(&mut self) {
```

**ADD BELOW**

```rust
        self.cancel_split();
```

**TYPE THIS**

**CURRENT**

```rust
    pub fn select(&mut self, row: Option<u32>) {
```

**ADD BELOW**

```rust
        self.cancel_split();
```

**TYPE THIS**

**CURRENT**

```rust
    fn apply_pick(&mut self, pick: Option<Pick>) {
```

**ADD BELOW**

```rust
        if self.pending_split.is_some() {
            if let Some(pick) = pick {
                self.pick_split_cutter(pick.row);
            }
            return;
        }
```

**TYPE THIS**

**CURRENT**

```rust
    pub fn request_selection(&mut self, x: u32, y: u32, edge: bool, face: bool) {
        let face = face || self.selection_tool == crate::app::selection::SelectionTool::Face;
        let edge = edge || self.selection_tool == crate::app::selection::SelectionTool::Edge;
        self.cancel_cloud_query();
        self.gpu.pick.cancel();
        #[cfg(target_arch = "wasm32")]
        if !edge && self.start_cloud_query(x, y) {
            return;
        }
        let mode = if face {
            PickMode::Component
```

**REPLACE WITH**

```rust
    pub fn request_selection(&mut self, x: u32, y: u32, edge: bool, face: bool) {
        let splitting = self.pending_split.is_some();
        let face = !splitting
            && (face || self.selection_tool == crate::app::selection::SelectionTool::Face);
        let edge = !splitting
            && (edge || self.selection_tool == crate::app::selection::SelectionTool::Edge);
        self.cancel_cloud_query();
        self.gpu.pick.cancel();
        #[cfg(target_arch = "wasm32")]
        if !splitting && !edge && self.start_cloud_query(x, y) {
            return;
        }
        let mode = if splitting {
            PickMode::Object
        } else if face {
            PickMode::Component
```

### `src/state/edit.rs`

**TYPE THIS**

**CURRENT**

```rust
    /// patched. The selection is dropped because the row it named may not exist any more.
    fn after_history(&mut self) {
        self.hierarchy.open.clear();
```

**REPLACE WITH**

```rust
    /// patched. The selection is dropped because the row it named may not exist any more.
    pub(super) fn after_history(&mut self) {
        self.hierarchy.open.clear();
```

**TYPE THIS**

**CURRENT**

```rust
        let command = crate::app::command::parse(line)?;
```

**ADD BELOW**

```rust
        if command != Command::Split {
            self.cancel_split();
        }
```

**TYPE THIS**

**CURRENT**

```rust
        match command {
```

**ADD BELOW**

```rust
            Command::Split => self.split_command(),
```

**TYPE THIS**

**CURRENT**

```rust
                    command,
                    Modeling::Point(_) | Modeling::Line(..) | Modeling::Polyline(_)
                );
```

**REPLACE WITH**

```rust
                    command,
                    Modeling::Point(_)
                        | Modeling::Line(..)
                        | Modeling::Polyline(_)
                        | Modeling::Curve(_)
                );
```

**TYPE THIS**

**CURRENT**

```rust
                        Modeling::Line(..) => "line",
```

**ADD BELOW**

```rust
                        Modeling::Curve(_) => "NURBS curve",
```

### `src/state/panel.rs`

**TYPE THIS**

**CURRENT**

```rust
                    let rows = self.hierarchy.targets(index);
                    self.select(None);
```

**REPLACE WITH**

```rust
                    let rows = self.hierarchy.targets(index);
                    if self.pending_split.is_some() {
                        for row in rows {
                            self.pick_split_cutter(row);
                        }
                        return;
                    }
                    self.select(None);
```

### `src/state/splitting.rs`

**NEW FILE · TYPE THIS**

```rust
use super::State;
use crate::app::{feedback, splitting};

pub(super) struct Pending {
    pub target: u32,
    pub face: Option<usize>,
    pub cutters: Vec<u32>,
}
impl State {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn split_status(&self) -> Option<(u32, Option<usize>, &[u32])> {
        self.pending_split
            .as_ref()
            .map(|p| (p.target, p.face, p.cutters.as_slice()))
    }
    pub(super) fn split_command(&mut self) -> Result<String, String> {
        if self.pending_split.is_some() {
            return self.finish_split();
        }
        let row = self
            .scene
            .selected
            .ok_or("Select a curve or Ctrl+Shift-select a face, then run Split")?;
        let selected = match self.selection {
            crate::app::selection::SelectionMode::Face { face, .. } => Some(face),
            _ => None,
        };
        let face = splitting::face_index(
            self.scene.geometry(row).ok_or("Source unavailable")?,
            selected,
        )?;
        self.pending_split = Some(Pending {
            target: row,
            face,
            cutters: vec![],
        });
        self.place_gizmo(None);
        feedback::command_line(false);
        feedback::focus_canvas();
        Ok("Split: select cutter lines, polylines or curves, then press Enter or tap Split again. Esc cancels. Faces require on-surface cutters.".into())
    }
    pub(super) fn cancel_split(&mut self) {
        if let Some(pending) = self.pending_split.take() {
            for row in pending.cutters {
                self.gpu.set_selected(row, false);
            }
        }
    }
    pub(super) fn pick_split_cutter(&mut self, row: u32) {
        let Some(pending) = self.pending_split.as_mut() else {
            return;
        };
        if row == pending.target
            || !self.scene.selectable(row)
            || !self.scene.geometry(row).is_some_and(splitting::is_cutter)
        {
            self.status(
                "Choose an unlocked line, polyline or NURBS curve distinct from the target",
            );
            return;
        }
        if let Some(at) = pending.cutters.iter().position(|item| *item == row) {
            pending.cutters.remove(at);
            self.gpu.set_selected(row, false);
        } else if pending.cutters.len() < 64 {
            pending.cutters.push(row);
            self.gpu.set_selected(row, true);
        }
        let count = pending.cutters.len();
        self.status(&format!(
            "Split: {count} cutter curves selected. Enter or Split confirms; Esc cancels."
        ));
        self.touch();
    }
    pub fn confirm_split(&mut self) {
        if self.pending_split.is_some() {
            let message = self.finish_split().unwrap_or_else(|error| error);
            self.status(&message);
            self.touch();
        }
    }
    fn finish_split(&mut self) -> Result<String, String> {
        let pending = self.pending_split.take().ok_or("Start Split first")?;
        if pending.cutters.is_empty() {
            self.pending_split = Some(pending);
            return Err("Select at least one cutter curve, then press Enter".into());
        }
        for &row in &pending.cutters {
            self.gpu.set_selected(row, false);
        }
        let identity = self.scene.identity_of(pending.target);
        let result = self
            .scene
            .split_rows(pending.target, pending.face, &pending.cutters);
        match result {
            Ok(regions) if regions > 1 => {
                self.after_history();
                let row = identity.and_then(|id| {
                    (0..self.scene.object_count() as u32)
                        .find(|&row| self.scene.identity_of(row).as_ref() == Some(&id))
                });
                self.select(row);
                Ok(format!(
                    "Split into {regions} regions. The BRep stays joined; Undo restores the original. Cutters are retained."
                ))
            }
            Ok(_) => {
                self.place_gizmo(self.scene.selected);
                Ok("No division: cutters must cross the curve or lie on the selected face.".into())
            }
            Err(error) => {
                self.place_gizmo(self.scene.selected);
                Err(error)
            }
        }
    }
}
```

### Check step 10

**Verified:** the complete step compiles for WebAssembly.

```bash
cargo check -j4 --lib
```

## Check

```bash
cargo xtest -j4 --lib
trunk serve --port 8780
```

Open <http://localhost:8780/?data=off&inspect=1>. Stop the server with **Ctrl+C**.

### Reproduce the screenshots

The screenshots use the small [nested fixture](extensions/nested.pb) and [manifest](extensions/nested.yaml), not private project files. Save both into your workspace:

```bash
cp "$COURSE_REPO/docs/extensions/nested.pb" assets/extension-nested.pb
cp "$COURSE_REPO/docs/extensions/nested.yaml" assets/extension-nested.yaml
```

Open <http://localhost:8780/?scene=extension-nested.yaml&data=off&inspect=1>.

## What changed

The docked workspace supports source mesh, curve, surface and compatible BRep subobject edits. Unsupported BRep trim reconstructions and incomplete streamed-source exports fail without discarding the retained scene.

## Try

Use the command walkthrough, tree selection and gumball controls. Every feature is present.

## Questions and answers

**What goes to the GPU?** Modeling rebuilds existing geometry lanes; panels change object flags; controls upload a small preview. The solid gumball owns a fixed mesh, an unlit shader and a bounded antialiasing tile.

**Why clear row selection after rebuilding?** Row numbers are upload addresses, not permanent identities. A rebuild can assign the same number to a different object.

**Where is the exact patch?** [step 1](extensions/integrated-1.patch), [step 2](extensions/integrated-2.patch), [step 3](extensions/integrated-3.patch), [step 4](extensions/integrated-4.patch), [step 5](extensions/integrated-5.patch), [step 6](extensions/integrated-6.patch), [step 7](extensions/integrated-7.patch), [step 8](extensions/integrated-8.patch), [step 9](extensions/integrated-9.patch), [step 10](extensions/integrated-10.patch). The patch and these visible instructions are generated from the same changes.

## Answers and next action

**How are trims handled?** A face has an outer trim and optional hole trims in surface parameter space. Cutter curves must lie on that surface within tolerance. The kernel partitions this trimmed region, preserves every region and hole, and updates shared boundary edges in neighboring faces. It rejects ambiguous overlaps and unsupported seam cases before replacing the source.

**Does the solid stay joined?** Yes. Select a face with Ctrl+Shift or the Face toolbar, run Split, then choose cutter curves and press Enter or tap Split again. A multi-face BRep needs an explicit face. The operation returns the owning BRep with its shell and solid relationships retained. It subdivides faces, without producing separate volumes.

**What is saved?** The original source is replaced in one undoable transaction. Curve pieces inherit the original placement, parent and color; cutters remain. Save/Open serializes the complete edited session.

**Run now**, in the same learning workspace:

```bash
cargo check -j4 --lib
trunk serve --port 8780
```

Expected compiler result: `Finished` with no errors. Open <http://localhost:8780/?data=off&inspect=1>. For the pictured shell, download [split.pb](extensions/split.pb) into assets/extension-split.pb and [split.yaml](extensions/split.yaml) into assets/extension-split.yaml. Open http://localhost:8780/?scene=extension-split.yaml&data=off&inspect=1, create Line 0,-150,20 0,150,20, select the top face and split it with that line. For the curve-only example, create Line -100,40,0 100,40,0 and Line 0,-60,0 0,140,0. Clear the selection and Fit, select the first line, press Split, pick the second line and press Enter or Split again. You should see two target pieces and the retained cutter. Undo restores the original; Redo and Save/Open retain the pieces. On the example shell, split its top face with a line lying on that face and confirm that both regions remain joined.

Stop the server with **Ctrl+C** before editing the next checkpoint. Then follow [Use the command line](command-line-walkthrough.md) to exercise the finished interface.

[Previous](current-9.md) · [Sequence](extend-integrated-tutorial.md) · [Next](command-line-walkthrough.md)

## Expected viewer result

The shell has been divided into two face regions by an on-surface line. The cutter remains and the selected half is highlighted. The shell stays joined, and the editable session stores the updated source geometry. Use Undo to restore the original or Save to keep this result. See the [phone layout](screenshots/extensions-split-phone.png).

[![Full viewer result for current 10](screenshots/extensions-split-face.png)](screenshots/extensions-split-face.png)
