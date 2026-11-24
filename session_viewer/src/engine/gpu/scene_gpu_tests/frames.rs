use crate::app::scene::{FileDoc, Scene};
use crate::camera::{Camera, View};
use crate::engine::gpu::{FrameInput, Gpu};
use session_rust::{AABB, Arrowhead, Line, Mesh, Plane, Point, Session, Vector, Xform};
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

#[test]
fn authored_line_widths_reach_the_pixels() {
    let Ok(mut gpu) = pollster::block_on(Gpu::new_headless(256, 256)) else {
        eprintln!("no GPU adapter; skipped");
        return;
    };
    gpu.view.show_grid = false;
    for (width, expected) in [(1.0, 1), (3.0, 3), (15.0, 15)] {
        let mut line = Line::new(-50.0, 0.0, 0.0, 50.0, 0.0, 0.0);
        line.width = width;
        line.arrowhead = Arrowhead::NONE;
        let mut session = Session::new("width");
        session.add_line(line, None);
        let pixels = top_view(session, &mut gpu);
        let actual = (0..256)
            .filter(|y| {
                let pixel = &pixels[(y * 256 + 128) * 4..];
                grey(pixel[0].into(), pixel[1].into(), pixel[2].into())
            })
            .count();
        assert!(
            actual.abs_diff(expected) <= 2,
            "width {width}: expected {expected} px, got {actual}"
        );
    }
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
