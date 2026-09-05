//! Planes and oriented boxes into the FLAT ribbon lane as outlines: a 1 m square for a
//! plane, the 12 edges for a box.

use session_rust::{Plane, Point, Vector, OBB};
use crate::engine::gpu::segments::SegRows;
use crate::engine::gpu::CylinderSegment;
use crate::math::Aabb;
use super::Row;
use super::encode::{encode_width, pack_rgba, Pen, FACING_UNKNOWN};

/// Half-extent of the square drawn for an infinite plane, world mm.
const PLANE_SIZE: f64 = 500.0;

/// The 12 edges of a box whose corners are ordered bottom 0-3, top 4-7.
const BOX_EDGES: [[usize; 2]; 12] = [[0, 1], [1, 2], [2, 3], [3, 0], [4, 5], [5, 6], [6, 7], [7, 4], [0, 4], [1, 5], [2, 6], [3, 7]];

/// The square's corner at signs `s` along the plane's x/y axes.
fn corner(o: &Point, x: &Vector, y: &Vector, s: [f64; 2]) -> [f32; 3] {
    [0usize, 1, 2].map(|k| (o[k] + (x[k] * s[0] + y[k] * s[1]) * PLANE_SIZE) as f32)
}

/// The `edges` over `pts` as segments with one pen; returns the points' box.
fn push_loop(seg: &mut SegRows, pts: &[[f32; 3]], edges: &[[usize; 2]], pen: &Pen) -> Aabb {
    let mut bounds = Aabb::empty();
    for p in pts {
        bounds.grow(*p);
    }
    for &[i, j] in edges {
        seg.ribbons.push(CylinderSegment { p0: pts[i], radius: pen.radius, p1: pts[j], instance_id: pen.row, color: pen.color, facing: FACING_UNKNOWN, support_start: 0, support_count: 0 });
    }
    bounds
}

/// The four edges of the plane's square.
pub fn walk_plane(seg: &mut SegRows, pl: &Plane, row: u32) -> Row {
    let (o, x, y) = (pl.origin(), pl.x_axis(), pl.y_axis());
    let c = [corner(&o, &x, &y, [1.0, 1.0]), corner(&o, &x, &y, [-1.0, 1.0]), corner(&o, &x, &y, [-1.0, -1.0]), corner(&o, &x, &y, [1.0, -1.0])];
    let pen = Pen { row, radius: encode_width(pl.width), color: pack_rgba(pl.linecolor.to_f32()) };
    Row::thin(push_loop(seg, &c, &[[0, 1], [1, 2], [2, 3], [3, 0]], &pen))
}

/// A box is its 12 edges; the OBB type carries no pen, so they draw black at the default width.
pub fn walk_obb(seg: &mut SegRows, b: &OBB, row: u32) -> Row {
    let c = b.corners_f32();
    let pen = Pen { row, radius: 0.0, color: pack_rgba([0.0, 0.0, 0.0, 1.0]) };
    Row::thin(push_loop(seg, &c, &BOX_EDGES, &pen))
}
