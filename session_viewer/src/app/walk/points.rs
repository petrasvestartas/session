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
    glyph.dots.push(GlyphPoint {
        center,
        radius: encode_width(p.width),
        color: p.pointcolor.to_f32(),
        instance_id: row,
        facing: FACING_UNKNOWN,
        facing_ext: [FACING_UNKNOWN; 2],
    });
    let mut bounds = Aabb::empty();
    bounds.grow(center);
    Row::thin(bounds)
}
