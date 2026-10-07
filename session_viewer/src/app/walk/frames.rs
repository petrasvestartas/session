use super::Row;
use super::curves::push_heads;
use super::encode::{FACING_UNKNOWN, Pen, encode_width, pack_rgba};
use crate::engine::gpu::CylinderSegment;
use crate::engine::gpu::lane::LaneRows;
use crate::engine::gpu::segments::SegRows;
use session_rust::AABB;
use session_rust::{Arrowhead, OBB, Plane, Point, Vector};
use std::sync::atomic::{AtomicU64, Ordering};

/// Half side of the square drawn for every plane and length of its normal arrow, mm, as f64 bits; `View Plane Size` sets it.
static PLANE_SIZE: AtomicU64 = AtomicU64::new(100.0_f64.to_bits());

/// Half side of the square drawn for every plane, mm.
pub fn plane_size() -> f64 {
    f64::from_bits(PLANE_SIZE.load(Ordering::Relaxed))
}

/// Set the half side of every plane's square and its normal's length, mm; planes walked from now on use it.
pub fn set_plane_size(size: f64) {
    PLANE_SIZE.store(size.to_bits(), Ordering::Relaxed);
}

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
        *value = (o[k] + (x[k] * s[0] + y[k] * s[1]) * plane_size()) as f32;
    }

    position
}

/// Push the edges as segments; return the points' box.
fn push_loop(seg: &mut SegRows, pts: &[[f32; 3]], edges: &[[usize; 2]], pen: &Pen) -> AABB {
    let mut bounds = AABB::empty();

    for p in pts {
        bounds.union_with_point(p[0] as f64, p[1] as f64, p[2] as f64);
    }

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

/// The axis colours of the ground grid: x pink, y yellow-green, z blue.
const AXIS_COLORS: [[f32; 4]; 3] = [
    [0.910, 0.278, 0.545, 1.0],
    [0.604, 0.804, 0.196, 1.0],
    [0.129, 0.588, 0.918, 1.0],
];

/// The plane's square and a small gumball on it: x, y and z arrows from the origin to the square's edges, coloured like the grid's axes.
pub fn walk_plane(seg: &mut SegRows, lanes: &mut LaneRows, pl: &Plane, row: u32) -> Row {
    let (o, x, y) = (pl.origin(), pl.x_axis(), pl.y_axis());
    let z = pl.z_axis();
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
    let mut bounds = push_loop(seg, &c, &[[0, 1], [1, 2], [2, 3], [3, 0]], &pen);
    let origin = [o[0] as f32, o[1] as f32, o[2] as f32];
    let mut flags = 0;

    // each axis its own headed segment, so each gets its arrowhead
    for (axis, color) in [x, y, z].iter().zip(AXIS_COLORS) {
        let tip = [0, 1, 2].map(|k| (o[k] + axis[k] * plane_size()) as f32);
        let arrow = Pen {
            color: pack_rgba(color),
            ..pen
        };
        bounds.union_with(&push_loop(seg, &[origin, tip], &[[0, 1]], &arrow));
        flags |= push_heads(seg, lanes, seg.ribbons.len() - 1, Arrowhead::END);
    }

    Row {
        flags,
        ..Row::thin(bounds)
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::Instance;
    use crate::engine::gpu::vectors::{VectorRow, VectorRows};

    /// A plane is its square and three headed axes from its origin to the square's edges, x, y and z in the grid's colours.
    #[test]
    fn plane_draws_its_square_and_a_gumball() {
        let mut seg = SegRows::default();
        let mut lanes = LaneRows::default();
        let row = walk_plane(&mut seg, &mut lanes, &Plane::xy_plane(), 3);
        let heads = lanes.get::<VectorRows>().map_or(Vec::new(), |v| v.rows.clone());
        let size = plane_size() as f32;

        assert_eq!(seg.ribbons.len(), 7);
        assert_eq!((seg.ribbons[4].p0, seg.ribbons[4].p1), ([0.0, 0.0, 0.0], [size, 0.0, 0.0]));
        assert_eq!((seg.ribbons[5].p0, seg.ribbons[5].p1), ([0.0, 0.0, 0.0], [0.0, size, 0.0]));
        assert_eq!((seg.ribbons[6].p0, seg.ribbons[6].p1), ([0.0, 0.0, 0.0], [0.0, 0.0, size]));
        assert_eq!(seg.ribbons[4].color, pack_rgba(AXIS_COLORS[0]));
        assert_eq!(seg.ribbons[6].color, pack_rgba(AXIS_COLORS[2]));
        assert_eq!(heads.len(), 3);
        assert_eq!((heads[2].end, heads[2].heads), ([0.0, 0.0, size], VectorRow::HEAD_END | VectorRow::HEAD_ONLY));
        assert_eq!(row.flags, Instance::FLAG_HEADS);
        assert_eq!((row.bounds.max_point()[0], row.bounds.max_point()[2]), (size as f64, size as f64));
    }
}
