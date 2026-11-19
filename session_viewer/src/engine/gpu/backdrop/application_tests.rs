use crate::app::cplane::CPlane;
use crate::app::scene::{FileDoc, Scene};
use crate::camera::{Camera, View};
use crate::engine::gpu::{FrameInput, Gpu};
use session_rust::{AABB, Point, Session, Xform};
use std::rc::Rc;

/// Grey grid pixels seen from above with the grid on `plane`.
fn grey_from_above(plane: CPlane) -> usize {
    let mut gpu = pollster::block_on(Gpu::new_headless(256, 256)).expect("a native adapter");
    gpu.view.show_grid = true;
    gpu.backdrop.set_grid_frame(&gpu.ctx, plane.matrix64());
    let mut scene = Scene::new();
    scene.add_file(FileDoc {
        name: "empty".into(),
        place: Xform::identity(),
        session: Rc::new(Session::new("empty")),
        point_px: 0.0,
        display_only: false,
    });
    scene.upload_to(&mut gpu);
    let mut camera = Camera::new();
    camera.set_view(View::Top);
    let around = AABB::from_points(
        &[
            Point::new(-6000.0, -6000.0, 0.0),
            Point::new(6000.0, 6000.0, 0.0),
        ],
        0.0,
    );
    camera.fit(&around, 1.0);
    let anchor = gpu
        .rebase_anchor(&camera.origin(), camera.distance_world(), 0.0)
        .anchor;
    let rgba = gpu.render_offscreen(&FrameInput {
        view_proj: camera.view_proj_anchored(1.0, &anchor),
        clear: wgpu::Color::WHITE,
        now_ms: 0.0,
    });
    rgba.chunks_exact(4)
        .filter(|p| {
            let (r, g, b) = (i32::from(p[0]), i32::from(p[1]), i32::from(p[2]));
            (r - g).abs() < 6 && (g - b).abs() < 6 && r < 235
        })
        .count()
}

#[test]
#[ignore = "requires Windows WARP and the adjacent DXC libraries"]
fn actual_grid_pixels_keep_far_fractions_and_return_to_the_same_near_frame() {
    let mut gpu = pollster::block_on(Gpu::new_headless(256, 256)).expect("Windows WARP");
    let adapter = gpu.ctx.device.adapter_info();
    assert_eq!(adapter.device_type, wgpu::DeviceType::Cpu, "{adapter:?}");
    assert_eq!(adapter.backend, wgpu::Backend::Dx12, "{adapter:?}");
    assert_eq!(adapter.name, "Microsoft Basic Render Driver", "{adapter:?}");
    gpu.view.show_grid = true;
    gpu.view.ssao = false;
    gpu.view.msaa_forced = Some(1);
    let mut camera = Camera::new();
    camera.set_view(View::Front);
    // An exactly representable eye offset keeps the actual near/far camera matrices equal.
    camera.distance = 1.0 / 256.0;
    camera.scene_extent = 10_000.0;
    let fraction = [0.125, 0.25, 0.5];
    let mut images = Vec::new();
    let mut matrices = Vec::new();
    for (step, base) in [[0.0; 3], [1e12, -1e12, 1e12], [0.0; 3], [0.0; 3]]
        .into_iter()
        .enumerate()
    {
        let offset = if step == 3 { [0.0; 3] } else { fraction };
        let origin = std::array::from_fn(|axis| base[axis] + offset[axis]);
        let plane = CPlane::Frame {
            origin,
            x: [1.0, 0.0, 0.0],
            y: [0.0, 0.0, 1.0],
        };
        camera.target = base.map(|value| value * camera.unit.to_meters());
        camera.grid = Some(origin);
        camera.update_position();
        let now = step as f64 * 1000.0;
        let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), now);
        assert_eq!([rebase.anchor[0], rebase.anchor[1], rebase.anchor[2]], base);
        gpu.backdrop.set_grid_frame(&gpu.ctx, plane.matrix64());
        assert!(!gpu.backdrop.set_grid_frame(&gpu.ctx, plane.matrix64()));
        assert!(!gpu.backdrop.rebase(&gpu.ctx, base));
        assert_eq!(gpu.backdrop.world_frame, plane.matrix64());
        assert_eq!(
            &super::relative_frame(gpu.backdrop.world_frame, base)[12..15],
            &offset.map(|v| v as f32)
        );
        let view_proj = camera.view_proj_anchored(1.0, &rebase.anchor);
        matrices.push(view_proj.to_f32());
        images.push(gpu.render_offscreen(&FrameInput {
            view_proj,
            clear: wgpu::Color::WHITE,
            now_ms: now,
        }));
    }
    if let Ok(directory) = std::env::var("GRID_WITNESS_DIR") {
        let directory = std::path::Path::new(&directory);
        std::fs::create_dir_all(directory).unwrap();
        for (name, image) in ["near", "far", "return", "zero-fraction"]
            .into_iter()
            .zip(&images)
        {
            let mut ppm = b"P6\n256 256\n255\n".to_vec();
            ppm.extend(
                image
                    .chunks_exact(4)
                    .flat_map(|pixel| pixel[..3].iter().copied()),
            );
            std::fs::write(directory.join(format!("grid-{name}.ppm")), ppm).unwrap();
        }
        std::fs::write(
            directory.join("settings.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "adapter":format!("{adapter:?}"), "viewport":[256,256], "samples":1,
                "plane_axes":[[1,0,0],[0,0,1]], "world_fraction":fraction,
                "bases":[[0.0,0.0,0.0],[1e12,-1e12,1e12],[0.0,0.0,0.0]],
                "anchored_camera_matrices":matrices
            }))
            .unwrap(),
        )
        .unwrap();
    }
    assert!(
        matrices.iter().all(|matrix| matrix == &matrices[0]),
        "identical actual anchored camera matrices"
    );
    let changed = |a: &[u8], b: &[u8]| {
        a.chunks_exact(4)
            .zip(b.chunks_exact(4))
            .filter(|(a, b)| a != b)
            .count()
    };
    assert_eq!(
        changed(&images[0], &images[1]),
        0,
        "near to far actual grid pixels"
    );
    assert_eq!(
        changed(&images[0], &images[2]),
        0,
        "return to near actual grid pixels"
    );
    let visible = changed(&images[0], &images[3]);
    assert!(
        visible > 200,
        "fractional placement must visibly change the grid, changed pixels: {visible}"
    );
    eprintln!(
        "actual grid WARP: near/far/return exact; fractional control changed {visible} pixels; adapter={adapter:?}"
    );
}

/// The grid lies on the construction plane: seen from above, the XZ grid is edge on, a line, the XY grid a field of lines.
#[test]
#[ignore = "requires a native GPU adapter"]
fn the_grid_lies_on_the_construction_plane() {
    let ground = grey_from_above(CPlane::Xy);
    let upright = grey_from_above(CPlane::Xz);
    assert!(ground > 2000, "the ground grid draws: {ground}");
    assert!(upright * 5 < ground, "edge on: {upright} of {ground}");
}
