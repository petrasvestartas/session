use super::Row;
use super::encode::{FACING_UNKNOWN, Pen, encode_width, pack_rgba};
use crate::engine::gpu::CylinderSegment;
use crate::engine::gpu::segments::SegRows;
use session_rust::AABB;
use session_rust::{OBB, Plane, Point, Vector};

/// Half size of the square drawn for a plane, mm: a plane is infinite, so it is drawn as a 1 m square.
const PLANE_SIZE: f64 = 500.0;

/// The 12 box edges, corners bottom 0-3 then top 4-7.
const BOX_EDGES: [[usize; 2]; 12] = [
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
    [3, 7],
];

/// One corner of the plane square.
fn corner(o: &Point, x: &Vector, y: &Vector, s: [f64; 2]) -> [f32; 3] {
    let mut position = [0.0; 3];

    for (k, value) in position.iter_mut().enumerate() {
        *value = (o[k] + (x[k] * s[0] + y[k] * s[1]) * PLANE_SIZE) as f32;
    }

    position
}

/// Push the edges as segments; return the points' box.
fn push_loop(seg: &mut SegRows, pts: &[[f32; 3]], edges: &[[usize; 2]], pen: &Pen) -> AABB {
    let mut bounds = AABB::empty();

    for p in pts {
        bounds.union_with_point(p[0] as f64, p[1] as f64, p[2] as f64);
    }

    // `&[i, j]` unpacks each two-index edge as the loop takes it
    for &[i, j] in edges {
        seg.ribbons.push(CylinderSegment {
            p0: pts[i],
            radius: pen.radius,
            p1: pts[j],
            instance_id: pen.row,
            color: pen.color,
            facing: FACING_UNKNOWN, // no face orientation
        });
    }

    bounds
}

/// The four edges of the plane's square.
pub fn walk_plane(seg: &mut SegRows, pl: &Plane, row: u32) -> Row {
    let (o, x, y) = (pl.origin(), pl.x_axis(), pl.y_axis());
    let c = [
        corner(&o, &x, &y, [1.0, 1.0]),
        corner(&o, &x, &y, [-1.0, 1.0]),
        corner(&o, &x, &y, [-1.0, -1.0]),
        corner(&o, &x, &y, [1.0, -1.0]),
    ];
    let pen = Pen {
        row,
        radius: encode_width(pl.width),
        color: pack_rgba(pl.linecolor.to_f32()),
    };
    Row::thin(push_loop(seg, &c, &[[0, 1], [1, 2], [2, 3], [3, 0]], &pen))
}

/// A box as 12 black edges.
pub fn walk_obb(seg: &mut SegRows, b: &OBB, row: u32) -> Row {
    let c = b.corners_f32();
    let pen = Pen {
        row,
        radius: 0.0, // default pen
        color: pack_rgba([0.0, 0.0, 0.0, 1.0]),
    };
    Row::thin(push_loop(seg, &c, &BOX_EDGES, &pen))
}
