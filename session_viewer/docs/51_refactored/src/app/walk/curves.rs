//! Lines, polylines and NURBS curves into the FLAT ribbon lane: one `CylinderSegment` per
//! span, `FACING_UNKNOWN` because free-standing linework has no adjacent faces and is always
//! drawn. Reads a kernel curve; writes `SegRows.ribbons` only.

use session_rust::{Line, NurbsCurve, Polyline};
use crate::engine::gpu::segments::SegRows;
use crate::engine::gpu::CylinderSegment;
use super::Row;
use super::encode::{encode_width, pack_rgba, FACING_UNKNOWN};

/// One ribbon segment. The ends are read by index: `start()`/`end()` build a kernel `Point`
/// each (two Strings apiece), 947k allocations on one sheet for six floats.
pub fn walk_line(seg: &mut SegRows, l: &Line, row: u32) -> Row {
    seg.ribbons.push(CylinderSegment {
        p0: [l[0] as f32, l[1] as f32, l[2] as f32],
        radius: encode_width(l.width),
        p1: [l[3] as f32, l[4] as f32, l[5] as f32],
        instance_id: row,
        color: pack_rgba(l.linecolor.to_f32()),
        facing: FACING_UNKNOWN,
    });
    Row::none()
}

/// One segment per span of the polyline.
pub fn walk_polyline(seg: &mut SegRows, pl: &Polyline, row: u32) -> Row {
    let pts = pl.get_points();
    let color = pack_rgba(pl.linecolor.to_f32());
    seg.ribbons.extend(pts.windows(2).map(|w| CylinderSegment {
        p0: w[0].to_f32(),
        radius: encode_width(pl.width),
        p1: w[1].to_f32(),
        instance_id: row,
        color,
        facing: FACING_UNKNOWN,
    }));
    Row::none()
}

/// Sample the curve into a polyline whose segment count follows its SIZE, then walk that.
pub fn walk_nurbscurve(seg: &mut SegRows, c: &NurbsCurve, row: u32) -> Row {
    // Bounding box of the CONTROL POINTS - cheap, and it bounds the curve (a NURBS curve
    // never leaves its control net), so it stands in for "how big is this curve".
    let (mut lo, mut hi) = ([f64::MAX; 3], [f64::MIN; 3]);
    for i in 0..c.m_cv_count {
        if let Some(cv) = c.cv(i) {
            // Rational curves store WEIGHTED CVs [x*w, y*w, z*w, w] - divide by w to get
            // the real point; non-rational (or w=0 guard) uses the coords as-is.
            let w = if c.m_is_rat && cv.len() > 3 && cv[3] != 0.0 {
                cv[3]
            } else {
                1.0
            };
            for k in 0..3 {
                lo[k] = lo[k].min(cv[k] / w);
                hi[k] = hi[k].max(cv[k] / w);
            }
        }
    }
    // No CV ever grew the box (empty/invalid curve) -> lo is still MAX: nothing to draw.
    if lo[0] > hi[0] {
        return Row::none();
    }
    // Sample count follows curve SIZE (box diagonal): a 2mm glyph outline gets 4 segments,
    // a metre-long arc ~50 - sqrt scaling, clamped so nothing under- or over-tessellates.
    let size = ((hi[0]-lo[0]).powi(2) + (hi[1]-lo[1]).powi(2) + (hi[2]-lo[2]).powi(2)).sqrt();
    let n = ((size / 0.2).sqrt().ceil() as usize).clamp(4, 64);

    // Evaluate the curve at n+1 evenly spaced parameters across its domain [t0, t1] ...
    let (t0, t1) = c.domain();
    let color = pack_rgba(c.linecolors.first().map(|c| c.to_f32()).unwrap_or([0.0, 0.0, 0.0, 1.0]));
    let radius = encode_width(c.width);
    let pts: Vec<[f32; 3]> = (0..=n)
        .map(|i| c.point_at(t0 + (t1 - t0) * i as f64 / n as f64).to_f32())
        .collect();
    // ... then it IS a polyline: consecutive pairs -> segments, same as walk_polyline.
    seg.ribbons.extend(pts.windows(2).map(|w| CylinderSegment {
        p0: w[0],
        radius,
        p1: w[1],
        instance_id: row,
        color,
        facing: FACING_UNKNOWN,
    }));
    Row::none()
}
