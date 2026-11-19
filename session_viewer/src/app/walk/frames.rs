use super::Row;
use super::encode::{FACING_UNKNOWN, Pen, encode_width, pack_rgba};
use crate::engine::gpu::CylinderSegment;
use crate::engine::gpu::lane::LaneRows;
use crate::engine::gpu::planes::{PlaneRow, PlaneRows, plane_size};
use crate::engine::gpu::segments::SegRows;
use session_rust::AABB;
use session_rust::{OBB, Plane};

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

/// The plane as one row of the plane lane, its frame; the lane draws its grid and its arrows, plane_size() from the origin.
pub fn walk_plane(lanes: &mut LaneRows, pl: &Plane, row: u32) -> Row {
    let (o, x, y, z) = (pl.origin(), pl.x_axis(), pl.y_axis(), pl.z_axis());
    let f32s = |v: [f64; 3]| v.map(|c| c as f32);
    lanes.get_mut::<PlaneRows>().rows.push(PlaneRow {
        origin: f32s([o[0], o[1], o[2]]),
        instance_id: row,
        x: f32s([x[0], x[1], x[2]]),
        radius: encode_width(pl.width),
        y: f32s([y[0], y[1], y[2]]),
        pad: 0,
    });
    // the grid's corners and the z arrow's tip, at the size walked with
    let size = plane_size();
    let mut bounds = AABB::empty();

    for (a, b, c) in [
        (1.0, 1.0, 0.0),
        (-1.0, -1.0, 0.0),
        (1.0, -1.0, 0.0),
        (-1.0, 1.0, 0.0),
        (0.0, 0.0, 1.0),
    ] {
        bounds.union_with_point(
            o[0] + (x[0] * a + y[0] * b + z[0] * c) * size,
            o[1] + (x[1] * a + y[1] * b + z[1] * c) * size,
            o[2] + (x[2] * a + y[2] * b + z[2] * c) * size,
        );
    }

    Row::thin(bounds)
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
    use crate::engine::gpu::vectors::VectorRows;

    /// A plane is one plane-lane row of its frame and no strokes; its box holds the grid and the z arrow.
    #[test]
    fn plane_is_one_row_of_its_frame() {
        let mut lanes = LaneRows::default();
        let row = walk_plane(&mut lanes, &Plane::xy_plane(), 3);
        let rows = &lanes.get::<PlaneRows>().unwrap().rows;
        let size = plane_size();

        assert_eq!(rows.len(), 1);
        assert_eq!(
            (rows[0].origin, rows[0].x, rows[0].y, rows[0].instance_id),
            ([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], 3)
        );
        assert!(lanes.get::<VectorRows>().is_none());
        assert_eq!(row.flags, 0);
        assert_eq!(
            (
                row.bounds.min_point()[0],
                row.bounds.max_point()[0],
                row.bounds.max_point()[2]
            ),
            (-size, size, size)
        );
    }
}

#[cfg(test)] // register:application-tests
#[path = "frames/application_tests.rs"] // register:application-tests
mod gpu_tests; // register:application-tests
