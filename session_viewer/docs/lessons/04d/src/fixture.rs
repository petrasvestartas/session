// --8<-- [start:step-9a]
use crate::engine::gpu::{
    CloudDraw, CylinderSegment, GlyphPoint, Instance, NO_NORMALS, ObjectRow, Upload,
};
// --8<-- [end:step-9a]
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

    // --8<-- [start:step-9b]
    for _ in 0..4 {
    // --8<-- [end:step-9b]
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
    let chain = [
        [0.3, 0.2, 0.0],
        [0.7, 1.0, 0.0],
        [1.2, 0.4, 0.0],
        [1.7, 1.0, 0.0],
    ];

    for pair in chain.windows(2) {
        upload.seg.ribbons.push(CylinderSegment {
            p0: pair[0],
            p1: pair[1],
            radius: 0.0,
            instance_id: 1,
            color: 0xff55ccff,
            facing: 0xffffffff,
        });
    }

    upload.glyph.dots.push(GlyphPoint {
        center: [-1.0, -0.7, 0.0],
        radius: 0.09,
        color: [1.0, 0.4, 0.2, 1.0],
        instance_id: 2,
        facing: 0xffffffff,
        facing_ext: [0xffffffff; 2],
    });
// --8<-- [start:step-9c]

    for y in 0..9 {
        for x in 0..13 {
            upload
                .cloud
                .pos
                .extend([0.4 + x as f32 * 0.1, -1.1 + y as f32 * 0.07, 0.0]);
            upload.cloud.col.push(0xffffaa55);
        }
    }

    upload.cloud.draws.push(CloudDraw {
        instance: 3,
        from: 0,
        count: 117,
        first: 0,
        spacing: 0.07,
        node_first: 0,
        node_count: 0,
        nrm_first: NO_NORMALS,
    });
    upload.obj.rows[3].spacing = 2.0;
    // --8<-- [end:step-9c]
    upload
}
