use super::Row;
use super::encode::{FACING_UNKNOWN, encode_width};
use crate::engine::gpu::GlyphPoint;
use crate::engine::gpu::glyphs::GlyphRows;
use session_rust::AABB;
use session_rust::Point;

/// One SDF dot.
pub fn walk_point(glyph: &mut GlyphRows, p: &Point, row: u32) -> Row {
    let center = p.to_f32();
    // --8<-- [start:step-13]
    let radius = encode_width(p.width); // negative = pixels
    glyph.dots.push(GlyphPoint {
        center,
        radius: if radius == 0.0 { -3.0 } else { radius }, // no pen: 3 px dot
        // --8<-- [end:step-13]
        color: p.pointcolor.to_f32(),
        instance_id: row,
        facing: FACING_UNKNOWN, // no face orientation
        facing_ext: [FACING_UNKNOWN; 2], // no neighbour orientation
    });
    let mut bounds = AABB::empty();
    bounds.union_with_point(center[0] as f64, center[1] as f64, center[2] as f64);
    Row::thin(bounds)
}
