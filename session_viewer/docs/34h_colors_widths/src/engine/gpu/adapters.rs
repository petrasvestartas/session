//! Session geometry -> GPU ros.
//! 'CylinderSegment' / 'GlyphPoint' are private to 'gpu/mod.rs'
//! But Rust visibility is "this module and its descendents"
//! adapters.rs is a child if gpu.
//! So it sees them throguh a plan 'use super::...'
//! No 'pub' is needed on either struct.
//! 

use super::{CylinderSegment, GlyphPoint};
use session_rust::{Line, NurbsCurve, Point, Polyline};

pub fn line_to_segment(l: &Line, instance_id: u32) -> CylinderSegment{
    CylinderSegment {
        p0: l.start().to_f32(),
        radius: encode_width(l.width),
        p1: l.end().to_f32(),
        instance_id,
        color: l.linecolor.to_f32(),
    }
}

pub fn polyline_to_segments(pl: &Polyline, instance_id: u32) -> Vec<CylinderSegment>{
    let pts = pl.get_points();
    let color = pl.linecolor.to_f32();
    pts.windows(2).map(|w| CylinderSegment{
        p0: w[0].to_f32(),
        radius: encode_width(pl.width),
        p1: w[1].to_f32(),
        instance_id,
        color,
    }).collect()
}

/// A curve becomes a polyline of ribbon segments. Sample count follows the curve's SIZE, not a
/// fixed number: a PDF sheet is mostly 1-2 mm glyph outlines (4 segments is already smoother than
/// a pixel) next to metre-long arcs (which need ~50), and a flat count would either shatter the
/// budget or visibly facet the big ones.
pub fn nurbscurve_to_segments(c: &NurbsCurve, instance_id: u32) -> Vec<CylinderSegment>{
    let (mut lo, mut hi) = ([f64::MAX; 3], [f64::MIN; 3]);
    for i in 0..c.m_cv_count {
        if let Some(cv) = c.cv(i) {
            let w = if c.m_is_rat && cv.len() > 3 && cv[3] != 0.0 { cv[3] } else { 1.0 };
            for k in 0..3 { lo[k] = lo[k].min(cv[k] / w); hi[k] = hi[k].max(cv[k] / w); }
        }
    }
    if lo[0] > hi[0] { return Vec::new(); }
    let size = ((hi[0]-lo[0]).powi(2) + (hi[1]-lo[1]).powi(2) + (hi[2]-lo[2]).powi(2)).sqrt();
    let n = ((size / 0.2).sqrt().ceil() as usize).clamp(4, 64);

    let (t0, t1) = c.domain();
    let color = c.linecolors.first().map(|c| c.to_f32()).unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let radius = encode_width(c.width);
    let pts: Vec<[f32; 3]> = (0..=n)
        .map(|i| c.point_at(t0 + (t1 - t0) * i as f64 / n as f64).to_f32())
        .collect();
    pts.windows(2).map(|w| CylinderSegment{
        p0: w[0],
        radius,
        p1: w[1],
        instance_id,
        color,
    }).collect()
}

pub fn point_to_glyph(p: &Point, instance_id: u32) -> GlyphPoint{
    GlyphPoint {
        center: p.to_f32(),
        radius: encode_width(p.width),
        color: p.pointcolor.to_f32(),
        instance_id,
        _pad: [0; 3],
    }
}

/// Kernel width - the radius encoding's negative lane (px multiplier); 0.0 = global default.
/// Radius 0.0 and -1.0 render identically (mult = select(1.0, -r, r<0)), so every w > 0 encodes
/// as-is - a special case for 1.0 would silently lose a real 1.0 pen (PDF widths are mm now).
/// walk_session flips negatives into the positive world-mm lane for planar 2d drawings:
/// paper-space lineweights that scale with zoom.
pub(super) fn encode_width(w: f64) -> f32{
    if w.is_finite() && w > 0.0 {
        -(w as f32)
    } else {
        0.0
    }
}