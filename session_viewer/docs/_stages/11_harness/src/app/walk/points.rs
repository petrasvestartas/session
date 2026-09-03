//! A free point into the FLAT glyph lane: one SDF dot, `FACING_UNKNOWN` because it
//! decorates no surface.

use session_rust::Point;
use crate::engine::gpu::glyphs::GlyphRows;
use crate::engine::gpu::GlyphPoint;
use crate::math::Aabb;
use super::Row;
use super::encode::{encode_width, FACING_UNKNOWN};

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
