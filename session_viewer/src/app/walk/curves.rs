//! Lines, polylines and NURBS curves into the FLAT ribbon lane: one segment per span,
//! `FACING_UNKNOWN` because free linework has no topological facing cull.

use super::Row;
use super::bounds::polyline_thickness;
use super::encode::{FACING_UNKNOWN, Pen, encode_width, pack_rgba};
use crate::engine::gpu::CylinderSegment;
use crate::engine::gpu::segments::SegRows;
use crate::math::Aabb;
use session_rust::{Line, NurbsCurve, Polyline};

/// Segments between consecutive points, growing `bounds` as they go.
pub(super) fn push_polyline(seg: &mut SegRows, pts: &[[f32; 3]], pen: &Pen, bounds: &mut Aabb) {
    let first = seg.ribbons.len() as u32;
    seg.ribbons.reserve(pts.len().saturating_sub(1));
    for w in pts.windows(2) {
        bounds.grow(w[0]);
        seg.ribbons.push(CylinderSegment {
            p0: w[0],
            radius: pen.radius,
            p1: w[1],
            instance_id: pen.row,
            color: pen.color,
            facing: FACING_UNKNOWN,
        });
    }
    seg.ribbon_chains.push(first..seg.ribbons.len() as u32);
    if let Some(last) = pts.last() {
        bounds.grow(*last);
    }
}

/// One ribbon segment; the ends are read by index (no kernel `Point` allocations).
pub fn walk_line(seg: &mut SegRows, l: &Line, row: u32) -> Row {
    let p0 = [l[0] as f32, l[1] as f32, l[2] as f32];
    let p1 = [l[3] as f32, l[4] as f32, l[5] as f32];
    let mut bounds = Aabb::empty();
    bounds.grow(p0);
    bounds.grow(p1);
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

/// One segment per span, straight from the flat coordinate array.
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
    let mut bounds = Aabb::empty();
    push_polyline(seg, &pts, &pen, &mut bounds);
    Row {
        thickness: polyline_thickness(&pts),
        ..Row::thin(bounds)
    }
}

/// Degrees of turning one chord may hide. A chord across `a` degrees of arc sags by
/// `r * (1 - cos(a/2))` of its own radius, so 5 degrees is a sag of 0.1% - under a pixel until
/// the curve is a thousand pixels across, at any zoom and at any size in world units.
const CHORD_DEGREES: f64 = 5.0;

/// Read one Euclidean control position using the curve's rational storage convention.
fn control_position(c: &NurbsCurve, i: usize) -> Option<[f64; 3]> {
    let p = c.cv(i)?;
    let w = if c.m_is_rat && p.len() > 3 && p[3] != 0.0 {
        p[3]
    } else {
        1.0
    };
    Some([p[0] / w, p[1] / w, p[2] / w])
}

/// Convert sampled world coordinates only at the viewer's f32 upload boundary.
pub(super) fn render_position(point: [f64; 3]) -> [f32; 3] {
    [point[0] as f32, point[1] as f32, point[2] as f32]
}

/// The control polygon turns by this much in total; a straight curve returns 0 and a full
/// circle 360, whatever its radius. Curvature, not world size, is what a chord has to follow.
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
        let u = [d[0] / len, d[1] / len, d[2] / len];
        if let Some(q) = prev {
            let dot = (q[0] * u[0] + q[1] * u[1] + q[2] * u[2]).clamp(-1.0, 1.0);
            total += dot.acos().to_degrees();
        }
        prev = Some(u);
    }
    total
}

/// The curve's f64 samples, one chord per `CHORD_DEGREES` of turning.
pub(super) fn sample_nurbscurve(c: &NurbsCurve) -> Vec<[f64; 3]> {
    if c.m_cv_count < 2 {
        return Vec::new();
    }
    let spans = c.span_count().max(1);
    let n = ((turning_degrees(c) / CHORD_DEGREES).ceil() as usize).clamp(spans, 512);

    let (t0, t1) = c.domain();
    let mut pts: Vec<[f64; 3]> = Vec::with_capacity(n + 1);
    for i in 0..=n {
        let point = c.point_at(t0 + (t1 - t0) * i as f64 / n as f64);
        pts.push([point[0], point[1], point[2]]);
    }
    pts
}

/// Sample the curve into a polyline whose segment count follows its size, then walk that.
pub fn walk_nurbscurve(seg: &mut SegRows, c: &NurbsCurve, row: u32) -> Row {
    let pts: Vec<_> = sample_nurbscurve(c)
        .into_iter()
        .map(render_position)
        .collect();
    let color = c
        .linecolors
        .first()
        .map(session_rust::Color::to_f32)
        .unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let pen = Pen {
        row,
        radius: encode_width(c.width),
        color: pack_rgba(color),
    };
    let mut bounds = Aabb::empty();
    push_polyline(seg, &pts, &pen, &mut bounds);
    Row::thin(bounds)
}
