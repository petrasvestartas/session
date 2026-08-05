//! Session geometry -> GPU ros.
//! 'CylinderSegment' / 'GlyphPoint' are private to 'gpu/mod.rs'
//! But Rust visibility is "this module and its descendents"
//! adapters.rs is a child if gpu.
//! So it sees them throguh a plan 'use super::...'
//! No 'pub' is needed on either struct.
//! 

use super::{CylinderSegment, GlyphPoint};
use session_rust::{Line, Point, Polyline};

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

pub fn point_to_glyph(p: &Point, instance_id: u32) -> GlyphPoint{
    GlyphPoint {
        center: p.to_f32(),
        radius: 0.0,
        color: p.pointcolor.to_f32(),
        instance_id,
        _pad: [0; 3],
    }
}

/// Kernel width (dimensionless, default 1.0) → the radius encoding's NEGATIVE lane (px
/// multiplier); 0.0 = plain global default. `walk_session` flips negatives into the POSITIVE
/// (world-mm) lane for planar 2D drawings — paper-space lineweights that scale with zoom.
pub(super) fn encode_width(w: f64) -> f32 {
    if w.is_finite() && w > 0.0 && (w - 1.0).abs() > 1e-9 { -(w as f32) } else { 0.0 }
}
