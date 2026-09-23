// --8<-- [start:step-19]
use crate::engine::gpu::{CylinderSegment, GlyphPoint, ObjectRow, Upload};
use session_rust::AABB;
use session_rust::Point;
use session_rust::{RenderVertex, Xform};

/// Small test scenes built in code, no loading.
pub fn scene() -> Upload {
    if crate::app::route::query("fixture").as_deref() == Some("floor") {
        floor()
    } else {
        grey_box()
    }
}

/// A box: six faces, twelve red edges, eight markers.
fn grey_box() -> Upload {
    let mut upload = Upload::default();
    let points = [
        [-0.5, -0.5, -0.5],
        [0.5, -0.5, -0.5],
        [0.5, 0.5, -0.5],
        [-0.5, 0.5, -0.5],
        [-0.5, -0.5, 0.5],
        [0.5, -0.5, 0.5],
        [0.5, 0.5, 0.5],
        [-0.5, 0.5, 0.5],
    ];
    upload.bounds = AABB::from_points(
        &[Point::new(-0.5, -0.5, -0.5), Point::new(0.5, 0.5, 0.5)],
        0.0,
    );
    row(&mut upload, true);
    let faces = [
        ([0, 3, 2, 1], [0., 0., -1.]),
        ([4, 5, 6, 7], [0., 0., 1.]),
        ([0, 1, 5, 4], [0., -1., 0.]),
        ([3, 7, 6, 2], [0., 1., 0.]),
        ([0, 4, 7, 3], [-1., 0., 0.]),
        ([1, 2, 6, 5], [1., 0., 0.]),
    ];

    for (corners, normal) in faces {
        let mut positions = [[0.0; 3]; 4];

        for (slot, corner) in corners.into_iter().enumerate() {
            positions[slot] = points[corner];
        }

        quad(&mut upload, 0, positions, normal);
    }

    for [first, last] in [
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
    ] {
        stroke(&mut upload, 0, points[first], points[last], 0xff0000ff);
    }

    for point in points {
        marker(&mut upload, 0, point, [0., 0., 0., 1.]);
    }

    upload
}

/// Magenta ink 4 mm under a sloping floor.
fn floor() -> Upload {
    let mut upload = Upload::default();
    upload.bounds = AABB::from_points(
        &[Point::new(-2.0, -1.5, -0.504), Point::new(2.0, 1.5, 0.5)],
        0.0,
    );
    row(&mut upload, true);
    row(&mut upload, false);
    quad(
        &mut upload,
        0,
        [
            [-2., -1.5, -0.5],
            [2., -1.5, 0.5],
            [2., 1.5, 0.5],
            [-2., 1.5, -0.5],
        ],
        [-0.24253562, 0., 0.9701425],
    );

    for y in [-0.6, 0.0, 0.6] {
        stroke(&mut upload, 1, [-1., y, -0.254], [1., y, 0.246], 0xffff00ff);
        marker(&mut upload, 1, [0., y, -0.004], [1., 0., 1., 1.]);
    }

    stroke(
        &mut upload,
        0,
        [-1.3, -1., -0.325],
        [1.3, -1., 0.325],
        0xffff0000,
    );
    upload
}

/// One entry per object; index = row.
fn row(upload: &mut Upload, faces: bool) {
    let mut row = ObjectRow::new(Xform::identity(), 0);
    row.bounds = upload.bounds;
    row.faces = faces;
    upload.obj.rows.push(row);
}

/// Two triangles on the four stroke corners.
fn quad(upload: &mut Upload, row: u32, points: [[f32; 3]; 4], normal: [f32; 3]) {
    let first = upload.arena.verts.len() as u32;

    for position in points {
        upload.arena.verts.push(RenderVertex {
            position,
            normal,
            color: [0.6, 0.6, 0.6, 1.],
        });
        upload.arena.vids.push(row);
    }

    for index in [0, 1, 2, 0, 2, 3] {
        upload.arena.idx.push(first + index);
    }
}

/// Screen-space ribbon extrusion consumes unchanged source endpoints.
fn stroke(upload: &mut Upload, row: u32, p0: [f32; 3], p1: [f32; 3], color: u32) {
    upload.seg.ribbons.push(CylinderSegment {
        p0,
        p1,
        radius: 0.,
        instance_id: row,
        color,
        facing: u32::MAX,
    });
}

/// Markers test depth the same way as strokes.
fn marker(upload: &mut Upload, row: u32, center: [f32; 3], color: [f32; 4]) {
    upload.glyph.dots.push(GlyphPoint {
        center,
        radius: 0.018,
        color,
        instance_id: row,
        facing: u32::MAX,
        facing_ext: [u32::MAX; 2],
        // --8<-- [end:step-19]
    });
}
