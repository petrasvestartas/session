use super::Row;
use super::encode::{FACING_UNKNOWN, Pen, encode_width, pack_rgba};
use crate::engine::gpu::CylinderSegment;
use crate::engine::gpu::segments::SegRows;
use session_rust::AABB;
use session_rust::{Line, NurbsCurve, Polyline};

/// One segment per pair of neighbours; grows `bounds`.
pub(super) fn push_polyline(seg: &mut SegRows, pts: &[[f32; 3]], pen: &Pen, bounds: &mut AABB) {
    let first = seg.ribbons.len() as u32;
    seg.ribbons.reserve(pts.len().saturating_sub(1));

    for w in pts.windows(2) {
        bounds.union_with_point(w[0][0] as f64, w[0][1] as f64, w[0][2] as f64);
        seg.ribbons.push(CylinderSegment {
            p0: w[0],
            radius: pen.radius,
            p1: w[1],
            instance_id: pen.row,
            color: pen.color,
            facing: FACING_UNKNOWN,
        });
    }

    seg.ribbon_chains.push(first..seg.ribbons.len() as u32); // one joined stroke

    if let Some(last) = pts.last() {
        bounds.union_with_point(last[0] as f64, last[1] as f64, last[2] as f64);
    }
}

/// A line as one segment.
pub fn walk_line(seg: &mut SegRows, l: &Line, row: u32) -> Row {
    let p0 = [l[0] as f32, l[1] as f32, l[2] as f32];
    let p1 = [l[3] as f32, l[4] as f32, l[5] as f32];
    let mut bounds = AABB::empty();
    bounds.union_with_point(p0[0] as f64, p0[1] as f64, p0[2] as f64);
    bounds.union_with_point(p1[0] as f64, p1[1] as f64, p1[2] as f64);
    seg.ribbons.push(CylinderSegment {
        p0,
        radius: encode_width(l.width),
        p1,
        instance_id: row,
        color: pack_rgba(l.linecolor.to_f32()),
        facing: FACING_UNKNOWN,
    });
    Row::thin(bounds)
}

/// A polyline as one segment per span.
pub fn walk_polyline(seg: &mut SegRows, pl: &Polyline, row: u32) -> Row {
    let mut pts: Vec<[f32; 3]> = Vec::with_capacity(pl.coords.len() / 3);

    for c in pl.coords.chunks_exact(3) {
        pts.push([c[0] as f32, c[1] as f32, c[2] as f32]);
    }

    let pen = Pen {
        row,
        radius: encode_width(pl.width),
        color: pack_rgba(pl.linecolor.to_f32()),
    };
    let mut bounds = AABB::empty();
    push_polyline(seg, &pts, &pen, &mut bounds);
    Row::thin(bounds)
}

/// Degrees of turning one chord may span.
const CHORD_DEGREES: f64 = 5.0;

/// Control point `i` with its weight divided out.
fn control_position(c: &NurbsCurve, i: usize) -> Option<[f64; 3]> {
    let p = c.cv(i)?;
    let w = if c.m_is_rat && p.len() > 3 && p[3] != 0.0 {
        p[3]
    } else {
        1.0
    };
    Some([p[0] / w, p[1] / w, p[2] / w])
}

/// f64 point to the f32 the GPU takes.
pub(super) fn render_position(point: [f64; 3]) -> [f32; 3] {
    [point[0] as f32, point[1] as f32, point[2] as f32]
}

/// Total turning of the control polygon in degrees.
fn turning_degrees(c: &NurbsCurve) -> f64 {
    let mut total = 0.0;
    let mut prev: Option<[f64; 3]> = None;

    for i in 1..c.m_cv_count {
        let (Some(a), Some(b)) = (control_position(c, i - 1), control_position(c, i)) else {
            continue;
        };
        let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();

        if len < 1e-12 {
            continue;
        }

        let u = [d[0] / len, d[1] / len, d[2] / len]; // unit direction

        if let Some(q) = prev {
            let dot = (q[0] * u[0] + q[1] * u[1] + q[2] * u[2]).clamp(-1.0, 1.0);
            total += dot.acos().to_degrees(); // angle between neighbours
        }

        prev = Some(u);
    }

    total
}

/// Sample the curve, one chord per `CHORD_DEGREES`.
pub(super) fn sample_nurbscurve(c: &NurbsCurve) -> Vec<[f64; 3]> {
    if c.m_cv_count < 2 {
        return Vec::new();
    }

    let spans = c.span_count().max(1);
    let n = ((turning_degrees(c) / CHORD_DEGREES).ceil() as usize).clamp(spans, 512); // chord count

    let (t0, t1) = c.domain();
    let mut pts: Vec<[f64; 3]> = Vec::with_capacity(n + 1);

    for i in 0..=n {
        let point = c.point_at(t0 + (t1 - t0) * i as f64 / n as f64);
        pts.push([point[0], point[1], point[2]]);
    }

    pts
}

/// A curve as a sampled polyline.
pub fn walk_nurbscurve(seg: &mut SegRows, c: &NurbsCurve, row: u32) -> Row {
    let pts: Vec<_> = sample_nurbscurve(c)
        .into_iter()
        .map(render_position)
        .collect();
    let color = c
        .linecolors
        .first()
        .map(session_rust::Color::to_f32)
        .unwrap_or([0.0, 0.0, 0.0, 1.0]); // black when unset
    let pen = Pen {
        row,
        radius: encode_width(c.width),
        color: pack_rgba(color),
    };
    let mut bounds = AABB::empty();
    push_polyline(seg, &pts, &pen, &mut bounds);
    Row::thin(bounds)
}
