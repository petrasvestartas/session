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

#[cfg(test)]
mod gpu_tests {
    use crate::app::scene::{FileDoc, Scene};
    use crate::camera::{Camera, View};
    use crate::engine::gpu::{FrameInput, Gpu};
    use session_rust::{AABB, Mesh, Plane, Point, Session, Vector, Xform};
    use std::rc::Rc;
    use std::time::Instant;

    /// One frame of `session` from above, framed on a 500 square round the origin; with the GPU.
    fn top_view(session: Session, gpu: &mut Gpu) -> Vec<u8> {
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "planes".into(),
            place: Xform::identity(),
            session: Rc::new(session),
            point_px: 0.0,
            display_only: false,
        });
        scene.upload_to(gpu);
        let mut camera = Camera::new();
        camera.set_view(View::Top);
        let frame = AABB::from_points(
            &[
                Point::new(-250.0, -250.0, 0.0),
                Point::new(250.0, 250.0, 100.0),
            ],
            0.0,
        );
        camera.fit(&frame, 1.0);
        let anchor = gpu
            .rebase_anchor(&camera.origin(), camera.distance_world(), 0.0)
            .anchor;
        gpu.render_offscreen(&FrameInput {
            view_proj: camera.view_proj_anchored(1.0, &anchor),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        })
    }

    /// Pixels inside the plane's square that `kind` accepts.
    fn count(rgba: &[u8], kind: fn(i32, i32, i32) -> bool) -> usize {
        rgba.chunks_exact(4)
            .enumerate()
            .filter(|(i, p)| {
                (90..166).contains(&(i % 256))
                    && (90..166).contains(&(i / 256))
                    && kind(p[0].into(), p[1].into(), p[2].into())
            })
            .count()
    }

    /// A grey ink pixel: neutral, darker than the paper.
    fn grey(r: i32, g: i32, b: i32) -> bool {
        (r - g).abs() < 6 && (g - b).abs() < 6 && r < 235
    }

    /// A pink pixel: the x arrow.
    fn pink(r: i32, g: i32, b: i32) -> bool {
        r - g > 25 && r > b
    }

    /// A yellow-green pixel: the y arrow.
    fn green(r: i32, g: i32, b: i32) -> bool {
        g - b > 25 && g > r
    }

    /// A solid over a plane hides its grey grid but not its pink, yellow-green and blue arrows.
    #[test]
    #[ignore = "requires a native GPU adapter"]
    fn a_solid_hides_the_grid_not_the_arrows() {
        let Ok(mut gpu) = pollster::block_on(Gpu::new_headless(256, 256)) else {
            panic!("a native adapter");
        };
        gpu.view.show_grid = false;
        gpu.view.opacity = 1.0;
        let plane = Plane::new(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        let mut alone = Session::new("plane");
        alone.add_plane(plane.clone(), None);
        let open = top_view(alone, &mut gpu);
        let mut covered = Session::new("plane");
        covered.add_plane(plane, None);
        let lid = covered.add_mesh(Mesh::create_box(400.0, 400.0, 20.0), None);
        covered.set_xform(
            &lid.expect("the lid").borrow().name,
            Xform::translation(0.0, 0.0, 50.0),
        );
        let mut gpu = pollster::block_on(Gpu::new_headless(256, 256)).unwrap();
        gpu.view.show_grid = false;
        gpu.view.opacity = 1.0;
        let under = top_view(covered, &mut gpu);
        assert!(count(&open, grey) > 200, "the grid draws");
        assert!(
            count(&under, grey) * 10 < count(&open, grey),
            "the lid hides the grid"
        );

        for axis in [pink, green] {
            assert!(count(&open, axis) > 20, "the arrow draws");
            assert!(
                count(&under, axis) * 10 >= count(&open, axis) * 8,
                "the arrow stays over the lid"
            );
        }

        // View Plane Size reaches the next frame through the uniform, no walk
        let mut alone = Session::new("plane");
        alone.add_plane(Plane::xy_plane(), None);
        let mut gpu = pollster::block_on(Gpu::new_headless(256, 256)).unwrap();
        gpu.view.show_grid = false;
        crate::engine::gpu::planes::set_plane_size(50.0);
        let small = top_view(alone, &mut gpu);
        crate::engine::gpu::planes::set_plane_size(100.0);
        // the columns the grid spans, half as many
        let columns = |rgba: &[u8]| {
            (0..256)
                .filter(|x| {
                    (0..256).any(|y| {
                        let p = &rgba[(y * 256 + x) * 4..];
                        grey(p[0].into(), p[1].into(), p[2].into())
                    })
                })
                .count()
        };
        let (wide, narrow) = (columns(&open), columns(&small));
        assert!(
            narrow * 10 < wide * 6 && narrow * 10 > wide * 4,
            "a half size grid: {narrow} of {wide} columns"
        );
    }

    /// 20 000 planes: GPU bytes, walk, upload and frame time.
    #[test]
    #[ignore]
    fn plane_stress() {
        let Ok(mut gpu) = pollster::block_on(Gpu::new_headless(1600, 1000)) else {
            eprintln!("no GPU adapter; skipped");
            return;
        };
        gpu.view.show_grid = false;
        let empty = gpu.allocated_bytes();
        let mut session = Session::new("planes");

        for i in 0..20_000 {
            let at = Point::new((i % 200) as f64 * 300.0, (i / 200) as f64 * 300.0, 0.0);
            session.add_plane(
                Plane::new(at, Vector::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
                None,
            );
        }

        let mut scene = Scene::new();
        let start = Instant::now();
        scene.add_file(FileDoc {
            name: "planes".into(),
            place: Xform::identity(),
            session: Rc::new(session),
            point_px: 0.0,
            display_only: false,
        });
        let walk = start.elapsed();
        let start = Instant::now();
        scene.upload_to(&mut gpu);
        let upload = start.elapsed();
        let full = gpu.allocated_bytes();
        let mut camera = Camera::new();
        camera.fit(&gpu.bounds, 1.6);
        let anchor = gpu
            .rebase_anchor(&camera.origin(), camera.distance_world(), 0.0)
            .anchor;
        let input = FrameInput {
            view_proj: camera.view_proj_anchored(1.6, &anchor),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        };
        gpu.render_offscreen(&input);
        let start = Instant::now();

        for _ in 0..20 {
            gpu.render_offscreen(&input);
        }

        let frame = start.elapsed() / 20;
        eprintln!(
            "STRESS buffers {} KB textures {} KB (scene adds {} KB) walk {:?} upload {:?} frame {:?}",
            full.0 / 1024,
            full.1 / 1024,
            (full.0 - empty.0) / 1024,
            walk,
            upload,
            frame
        );
    }
}
