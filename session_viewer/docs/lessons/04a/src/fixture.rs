use crate::engine::gpu::{Instance, ObjectRow, Upload};
use session_rust::AABB;
use session_rust::Point;
use session_rust::{RenderVertex, Xform};

/// Small test scenes built in code, no loading.
pub fn scene() -> Upload {
    let mut upload = Upload::default();
    upload.bounds = AABB::from_points(
        &[Point::new(-2.0, -1.3, -0.1), Point::new(2.0, 1.3, 0.1)],
        0.0,
    );

    for _ in 0..1 {
        upload
            .obj
            .rows
            .push(ObjectRow::new(Xform::identity(), Instance::FLAG_OPEN));
    }

    upload.obj.rows[0].faces = true;
    upload.obj.rows[0].bounds = upload.bounds;

    for position in [[-1.7, 0.1, 0.0], [-0.3, 0.1, 0.0], [-1.0, 1.1, 0.0]] {
        upload.arena.verts.push(RenderVertex {
            position,
            normal: [0.0, 0.0, 1.0],
            color: [0.2, 0.7, 1.0, 1.0],
        });
        upload.arena.vids.push(0);
    }

    upload.arena.idx.extend([0, 1, 2]);
    upload
}
