//! Planes and oriented boxes into the FLAT ribbon lane as their outlines: a 1 m square for a
//! plane, the 12 edges for a box. Writes `SegRows.ribbons` only.

use session_rust::{Plane, Point, Vector, OBB};
use crate::engine::gpu::segments::SegRows;
use crate::engine::gpu::CylinderSegment;
use super::Row;
use super::encode::{encode_width, pack_rgba, FACING_UNKNOWN};

/// A plane is infinite - draw a fixed square around its origin, spanned by its x/y axes.
/// Half-extent in world mm (a 1 m square).
const PLANE_SIZE: f64 = 500.0;

/// The square's corner at signs `s` = (sx, sy) along the plane's x/y axes.
fn corner(o: &Point, x: &Vector, y: &Vector, s: [f64; 2]) -> [f32; 3] {
    [0usize, 1, 2].map(|k| (o[k] + (x[k] * s[0] + y[k] * s[1]) * PLANE_SIZE) as f32)
}

/// The four edges of the plane's square.
pub fn walk_plane(seg: &mut SegRows, pl: &Plane, row: u32) -> Row {
    let (o, x, y) = (pl.origin(), pl.x_axis(), pl.y_axis());
    let c = [corner(&o, &x, &y, [1.0, 1.0]), corner(&o, &x, &y, [-1.0, 1.0]), corner(&o, &x, &y, [-1.0, -1.0]), corner(&o, &x, &y, [1.0, -1.0])];
    let color = pack_rgba(pl.linecolor.to_f32());
    let radius = encode_width(pl.width);
    seg.ribbons.extend((0..4).map(|i| CylinderSegment { p0:c[i], radius, p1: c[(i+1) % 4], instance_id: row, color, facing: FACING_UNKNOWN }));
    Row::none()
}

/// A box is its 12 edges: bottom loop, top loop, four verticals - `corners_f32()` orders the
/// bottom face 0-3 and the top 4-7 with i / i+4 vertically aligned. The OBB type carries no
/// pen, so the edges draw black at screen-constant width (radius 0.0 = global default).
pub fn walk_obb(seg: &mut SegRows, b: &OBB, row: u32) -> Row {
    const EDGES: [[usize; 2]; 12] = [
        [0, 1],
        [1, 2],
        [2, 3],
        [3, 0],
        [4, 5],
        [5, 6],
        [6, 7],
        [7, 4],
        [0, 4],
        [1, 5],
        [2, 6],
        [3, 7]
    ];

    let c = b.corners_f32();
    seg.ribbons.extend(EDGES.iter().map(|&[i, j]| CylinderSegment { p0: c[i], radius: 0.0, p1: c[j], instance_id: row, color: pack_rgba([0.0, 0.0, 0.0, 1.0]), facing: FACING_UNKNOWN }));
    Row::none()
}
