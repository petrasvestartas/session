//! Lines, polylines and NURBS curves into the FLAT ribbon lane: one segment per span,
//! `FACING_UNKNOWN` because free linework has no adjacent faces. Every producer reports the
//! object's local box, which caps the ink lift so a line behind a plate stays behind it.

use session_rust::{Line, NurbsCurve, Polyline};
use crate::engine::gpu::segments::SegRows;
use crate::engine::gpu::CylinderSegment;
use crate::math::Aabb;
use super::Row;
use super::bounds::polyline_thickness;
use super::encode::{encode_width, pack_rgba, Pen, FACING_UNKNOWN};

/// Segments between consecutive points, growing `bounds` as they go.
fn push_polyline(seg: &mut SegRows, pts: &[[f32; 3]], pen: &Pen, bounds: &mut Aabb) {
    seg.ribbons.reserve(pts.len().saturating_sub(1));
    for w in pts.windows(2) {
        bounds.grow(w[0]);
        seg.ribbons.push(CylinderSegment { p0: w[0], radius: pen.radius, p1: w[1], instance_id: pen.row, color: pen.color, facing: FACING_UNKNOWN });
    }
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
    seg.ribbons.push(CylinderSegment { p0, radius: encode_width(l.width), p1, instance_id: row, color: pack_rgba(l.linecolor.to_f32()), facing: FACING_UNKNOWN });
    Row::thin(bounds)
}

/// One segment per span, straight from the flat coordinate array.
pub fn walk_polyline(seg: &mut SegRows, pl: &Polyline, row: u32) -> Row {
    let mut pts: Vec<[f32; 3]> = Vec::with_capacity(pl.coords.len() / 3);
    for c in pl.coords.chunks_exact(3) {
        pts.push([c[0] as f32, c[1] as f32, c[2] as f32]);
    }
    let pen = Pen { row, radius: encode_width(pl.width), color: pack_rgba(pl.linecolor.to_f32()) };
    let mut bounds = Aabb::empty();
    push_polyline(seg, &pts, &pen, &mut bounds);
    Row { thickness: polyline_thickness(&pts), ..Row::thin(bounds) }
}

/// The box of the control points (a NURBS curve never leaves its control net).
fn control_box(c: &NurbsCurve) -> Option<([f64; 3], [f64; 3])> {
    let (mut lo, mut hi) = ([f64::MAX; 3], [f64::MIN; 3]);
    for i in 0..c.m_cv_count {
        let Some(cv) = c.cv(i) else { continue };
        let w = if c.m_is_rat && cv.len() > 3 && cv[3] != 0.0 { cv[3] } else { 1.0 };
        for k in 0..3 {
            lo[k] = lo[k].min(cv[k] / w);
            hi[k] = hi[k].max(cv[k] / w);
        }
    }
    if lo[0] > hi[0] { None } else { Some((lo, hi)) }
}

/// Sample the curve into a polyline whose segment count follows its size, then walk that.
pub fn walk_nurbscurve(seg: &mut SegRows, c: &NurbsCurve, row: u32) -> Row {
    let Some((lo, hi)) = control_box(c) else { return Row::thin(Aabb::empty()) };
    let size = ((hi[0] - lo[0]).powi(2) + (hi[1] - lo[1]).powi(2) + (hi[2] - lo[2]).powi(2)).sqrt();
    let n = ((size / 0.2).sqrt().ceil() as usize).clamp(4, 64);

    let (t0, t1) = c.domain();
    let mut pts: Vec<[f32; 3]> = Vec::with_capacity(n + 1);
    for i in 0..=n {
        pts.push(c.point_at(t0 + (t1 - t0) * i as f64 / n as f64).to_f32());
    }
    let color = c.linecolors.first().map(|c| c.to_f32()).unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let pen = Pen { row, radius: encode_width(c.width), color: pack_rgba(color) };
    let mut bounds = Aabb::empty();
    push_polyline(seg, &pts, &pen, &mut bounds);
    Row::thin(bounds)
}
