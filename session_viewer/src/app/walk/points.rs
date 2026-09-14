//! A free point into the FLAT glyph lane: one SDF dot with no topology-facing cull.

use super::Row;
use super::encode::{FACING_UNKNOWN, encode_width};
use crate::engine::gpu::GlyphPoint;
use crate::engine::gpu::glyphs::GlyphRows;
use crate::math::Aabb;
use session_rust::Point;

/// One SDF dot.
pub fn walk_point(glyph: &mut GlyphRows, p: &Point, row: u32) -> Row {
    let center = p.to_f32();
    let radius = encode_width(p.width);
    glyph.dots.push(GlyphPoint {
        center,
        // A standalone point needs a visible marker, independent of the line pen.
        radius: if radius == 0.0 { -3.0 } else { radius },
        color: p.pointcolor.to_f32(),
        instance_id: row,
        facing: FACING_UNKNOWN,
        facing_ext: [FACING_UNKNOWN; 2],
    });
    let mut bounds = Aabb::empty();
    bounds.grow(center);
    Row::thin(bounds)
}
