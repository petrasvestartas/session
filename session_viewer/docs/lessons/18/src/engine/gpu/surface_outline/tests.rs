use crate::engine::gpu::{FrameInput, Gpu, ObjectRow, Upload};
use session_rust::{RenderVertex, Xform};

#[test]
#[ignore = "requires a native GPU adapter"]
/// Selected edges never thin the black outline.
fn selected_cad_edges_do_not_paint_over_the_black_silhouette() {
    use crate::app::scene::{FileDoc, Scene};
    use crate::camera::Camera;
    use session_rust::{BRep, Session};
    use std::rc::Rc;

    let mut gpu = pollster::block_on(Gpu::new_headless(480, 480)).unwrap();
    gpu.view.show_outlines = true;
    gpu.view.show_grid = false;
    gpu.view.markers = false;

    for shape in [
        BRep::create_cone(150.0, 400.0),
        BRep::create_cylinder(150.0, 400.0),
    ] {
        gpu.reset();
        let mut source = Session::new("selected silhouette regression");
        source.add_brep(shape, None);
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "solid".into(),
            session: Rc::new(source),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        scene.upload_to(&mut gpu);
        gpu.set_selected(0, true);

        for samples in [1, 4] {
            for dpr in [1.0, 2.0] {
                gpu.logical_size = [480.0 / dpr; 2];
                gpu.view.msaa_forced = Some(samples);
                gpu.resize(480, 480);

                for orbit in [(0.0, 0.0), (95.0, -65.0), (-80.0, 130.0)] {
                    let mut camera = Camera::new();
                    camera.fit(&gpu.bounds, 1.0);
                    camera.orbit(orbit.0, orbit.1);
                    let rebase =
                        gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 0.0);
                    let input = FrameInput {
                        view_proj: camera.view_proj_anchored(1.0, &rebase.anchor),
                        clear: wgpu::Color::WHITE,
                        now_ms: 0.0,
                    };
                    gpu.view.show_mesh_edges = true;
                    gpu.segments.set_selected(0, false);
                    let silhouette = gpu.render_offscreen(&input);
                    gpu.segments.set_selected(0, true);
                    let edged = gpu.render_offscreen(&input);
                    let mut coverage = 0_u32;

                    for (plain, inked) in silhouette.chunks_exact(4).zip(edged.chunks_exact(4))
                    {
                        let lo = *plain[..3].iter().min().unwrap();
                        let hi = *plain[..3].iter().max().unwrap();

                        if hi - lo <= 2 {
                            coverage += u32::from(255 - hi);
                        }

                        if hi < 8 {
                            assert!(
                                inked[..3].iter().all(|channel| *channel < 12),
                                "yellow CAD strokes must not narrow the black border: {plain:?} -> {inked:?}; samples={samples}, DPR={dpr}, orbit={orbit:?}"
                            );
                        }
                    }

                    assert!(
                        coverage > gpu.config.height * 255 / 2,
                        "visible silhouette includes fractional coverage: {coverage}"
                    );
                }
            }
        }
    }
}

#[test]
#[ignore = "requires a native GPU adapter"]
/// Two touching solids get one outline, none between them.
fn touching_and_overlapping_solids_have_one_continuous_outline() {
    let mut gpu = pollster::block_on(Gpu::new_headless(200, 200)).unwrap();
    gpu.view.show_outlines = true;
    gpu.view.show_grid = false;
    gpu.view.lit = false;
    let input = FrameInput {
        view_proj: Xform::identity(),
        clear: wgpu::Color::WHITE,
        now_ms: 0.0,
    };

    for samples in [1, 4] {
        gpu.view.msaa_forced = Some(samples);
        gpu.resize(200, 200);

        for overlap in [false, true] {
            gpu.reset();
            let mut upload = Upload::default();
            quad(&mut upload, 0.4, 0.5);
            quad(&mut upload, 0.4, 0.6);

            for (i, vertex) in upload.arena.verts.iter_mut().enumerate() {
                vertex.position[0] += if i < 4 {
                    -0.4
                } else if overlap {
                    0.2
                } else {
                    0.4
                };
            }

            gpu.set_scene(&upload);
            let outlined = gpu.render_offscreen(&input);
            gpu.view.show_outlines = false;
            let plain = gpu.render_offscreen(&input);
            let ids = gpu.render_ids_offscreen(&input);
            gpu.view.show_outlines = true;
            assert_eq!(ids, gpu.render_ids_offscreen(&input));

            for y in 65..135 {
                for x in 45..150 {
                    let at = (y * 200 + x) * 4;
                    assert_eq!(
                        &outlined[at..at + 4],
                        &plain[at..at + 4],
                        "no outline inside the combined silhouette, including object joins"
                    );
                }
            }

            assert_ne!(
                outlined, plain,
                "the combined outside silhouette must still be outlined"
            );
        }
    }
}

/// Add one square object to the upload.
fn quad(upload: &mut Upload, extent: f32, depth: f32) {
    let row = upload.obj.rows.len() as u32;
    let first = upload.arena.verts.len() as u32;
    upload.obj.rows.push(ObjectRow::new(Xform::identity(), 0));

    for [x, y] in [
        [-extent, -extent],
        [extent, -extent],
        [extent, extent],
        [-extent, extent],
    ] {
        upload.arena.verts.push(RenderVertex {
            position: [x, y, depth],
            normal: [0.0, 0.0, 1.0],
            color: [0.3, 0.5, 0.7, 1.0],
        });
        upload.arena.vids.push(row);
    }

    upload
        .arena
        .idx
        .extend([0, 1, 2, 0, 2, 3].map(|i| first + i));
}

#[test]
#[ignore = "requires a native GPU adapter"]
/// A hidden selection has no outline; clearing it frees the mask.
fn selected_silhouette_is_black_visible_only_and_releases_coverage() {
    let mut gpu = pollster::block_on(Gpu::new_headless(200, 200)).unwrap();
    gpu.view.show_outlines = true;
    gpu.view.show_grid = false;
    gpu.view.lit = false;
    let mut upload = Upload::default();
    quad(&mut upload, 0.5, 0.5);
    quad(&mut upload, 0.8, 0.7);
    gpu.set_scene(&upload);
    let input = FrameInput {
        view_proj: Xform::identity(),
        clear: wgpu::Color::WHITE,
        now_ms: 0.0,
    };

    for samples in [1, 4, 1] {
        gpu.view.msaa_forced = Some(samples);
        gpu.resize(200, 200);
        let hidden = gpu.render_offscreen(&input);
        gpu.set_selected(0, true);
        assert_eq!(
            hidden,
            gpu.render_offscreen(&input),
            "an entirely occluded selection has no outline or yellow pixels"
        );
        gpu.set_hidden(1, true);
        let selected = gpu.render_offscreen(&input);
        // with only the selected mask the picture is the same
        gpu.pass_mut::<super::Outline>().solid.kind = super::OutlineKind::Selected;
        assert_eq!(
            selected,
            gpu.render_offscreen(&input),
            "selection must not receive a second overlapping outline"
        );
        gpu.pass_mut::<super::Outline>().solid.kind = super::OutlineKind::AllSolids;
        let black = selected
            .chunks_exact(4)
            .filter(|p| p[..3].iter().all(|c| *c < 8))
            .count();
        let yellow = selected
            .chunks_exact(4)
            .filter(|p| p[0] > 240 && p[1] > 240 && p[2] < 8)
            .count();
        assert!(
            black > 350,
            "selected silhouette must have a continuous black border: {black}"
        );
        assert!(
            yellow > 9500,
            "the original selection fill stays yellow: {yellow}"
        );
        let selected_ids = gpu.render_ids_offscreen(&input);
        let capacity = gpu.pass::<super::Outline>().selection.allocated_bytes().1;
        assert_eq!(
            capacity,
            200 * 200 * if samples > 1 { 5 } else { 1 } + 2 * 50 * 50
        );
        gpu.set_selected(0, false);
        assert_eq!(
            gpu.pass::<super::Outline>().selection.allocated_bytes().1,
            0,
            "clearing selection releases coverage immediately"
        );
        let plain = gpu.render_offscreen(&input);
        assert_eq!(
            selected_ids,
            gpu.render_ids_offscreen(&input),
            "a visual border must not add pickable geometry"
        );
        let plain_black = plain
            .chunks_exact(4)
            .filter(|p| p[..3].iter().all(|c| *c < 8))
            .count();
        assert_eq!(
            plain_black, black,
            "ordinary and selected outlines have the same width"
        );
        gpu.view.show_outlines = false;
        let disabled = gpu.render_offscreen(&input);
        assert!(
            disabled
                .chunks_exact(4)
                .all(|p| p[0] > 8 || p[1] > 8 || p[2] > 8)
        );
        assert_eq!(
            selected_ids,
            gpu.render_ids_offscreen(&input),
            "outline toggling never changes source picking"
        );
        gpu.view.show_outlines = true;
        gpu.set_hidden(1, false);
    }

    gpu.release();
    assert_eq!(
        gpu.pass::<super::Outline>().selection.allocated_bytes(),
        (16, 0)
    );
    assert_eq!(
        gpu.pass::<super::Outline>().solid.allocated_bytes(),
        (16, 0),
        "release frees the outline alpha"
    );
}

#[test]
/// The face coverage shader validates at one and at four samples.
fn face_coverage_shader_validates() {
    for samples in [1, 4] {
        let source = crate::engine::pipelines::shared(&super::face_coverage_source(samples));
        let module = naga::front::wgsl::parse_str(&source)
            .unwrap_or_else(|error| panic!("{}", error.emit_to_string(&source)));
        naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::default()
                | naga::valid::Capabilities::SHADER_FLOAT16_IN_FLOAT32,
        )
        .validate(&module)
        .unwrap_or_else(|error| panic!("{}", error.emit_to_string(&source)));
    }
}

#[test]
/// Taps fit the table, run nearest first and the block covers the search.
fn taps_are_sorted_and_blocks_cover_the_extent() {
    for radius in [1.0, 1.6875, 3.375, 5.0625, 12.0] {
        let taps = super::taps(radius);
        assert!(taps.len() <= super::TAPS);
        assert_eq!(taps[0], [0.0, 0.0, 1.0, 0.0]);
        assert!(taps.windows(2).all(|pair| pair[0][2] >= pair[1][2]));
        assert!(super::block(radius) >= (radius + 0.5).ceil() as u32);
    }

    assert_eq!(super::block(3.375), 4);
    assert_eq!(super::block(5.0625), 8);
}

/// A headless canvas with a grid of cones and cylinders, rows 0..5 selectable.
fn solids_scene(
    width: u32,
    height: u32,
    count: usize,
) -> (Gpu, crate::camera::Camera, session_rust::Point) {
    use crate::app::scene::{FileDoc, Scene};
    use crate::camera::Camera;
    use session_rust::{BRep, Session};
    use std::rc::Rc;
    let mut gpu = pollster::block_on(Gpu::new_headless(width, height)).unwrap();
    gpu.view.show_grid = false;
    gpu.view.show_outlines = true;
    gpu.view.markers = false;
    let mut source = Session::new("outline bench");
    let side = (count as f64).sqrt().ceil() as usize;

    for i in 0..count {
        let shape = if i % 2 == 0 {
            BRep::create_cone(40.0, 100.0)
        } else {
            BRep::create_cylinder(40.0, 100.0)
        };
        let solid = source.add_brep(shape, None).unwrap();
        let (x, y) = ((i % side) as f64 * 120.0, (i / side) as f64 * 120.0);
        source.set_xform(&solid.borrow().name, Xform::translation(x, y, 0.0));
    }

    let mut scene = Scene::new();
    scene.add_file(FileDoc {
        name: "solids".into(),
        session: Rc::new(source),
        place: Xform::identity(),
        point_px: 0.0,
        display_only: false,
    });
    scene.upload_to(&mut gpu);
    let mut camera = Camera::new();
    camera.fit(&gpu.bounds, width as f64 / height as f64);
    camera.orbit(60.0, -80.0);
    let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 0.0);
    (gpu, camera, rebase.anchor)
}

/// Milliseconds of one offscreen frame read back to the CPU, and its pixels.
fn timed(
    gpu: &mut Gpu,
    camera: &crate::camera::Camera,
    anchor: &session_rust::Point,
) -> (f64, Vec<u8>) {
    let aspect = gpu.config.width as f64 / gpu.config.height as f64;
    let input = FrameInput {
        view_proj: camera.view_proj_anchored(aspect, anchor),
        clear: wgpu::Color::WHITE,
        now_ms: 0.0,
    };
    let start = std::time::Instant::now();
    let pixels = gpu.render_offscreen(&input);
    (start.elapsed().as_secs_f64() * 1000.0, pixels)
}

/// 0: `o` only, 1: `o` and a selection, 2: the selection only, 3: no outline.
fn outline_mode(gpu: &mut Gpu, mode: usize) {
    gpu.view.show_outlines = mode != 3;
    gpu.pass_mut::<super::Outline>().solid.kind = if mode == 2 {
        super::OutlineKind::Selected
    } else {
        super::OutlineKind::AllSolids
    };

    for row in 0..5 {
        gpu.set_selected(row, mode == 1 || mode == 2);
    }
}

#[test]
#[ignore = "requires a native GPU adapter"]
/// Once each outline mode drew at 1x and 4x, toggles, flips and resizes compile nothing.
fn toggles_flips_and_resizes_compile_nothing_twice() {
    let (mut gpu, camera, anchor) = solids_scene(320, 200, 8);
    let cycle = |gpu: &mut Gpu, width: u32| {
        for samples in [4, 1] {
            gpu.view.msaa_forced = Some(samples);
            gpu.resize(width, 200);

            for mode in [0, 1, 2, 3] {
                outline_mode(gpu, mode);
                timed(gpu, &camera, &anchor);
            }
        }
    };
    cycle(&mut gpu, 320);
    let compiled = crate::engine::pipelines::created();

    for width in 321..331 {
        cycle(&mut gpu, width);
    }

    assert_eq!(crate::engine::pipelines::created(), compiled);
}

#[test]
#[ignore = "requires a native GPU adapter"]
/// Writes outline captures at DPR 1/2/3, MSAA 1/4 for 'o', selection and both.
fn outline_captures() {
    let (mut gpu, camera, anchor) = solids_scene(960, 600, 50);
    std::fs::create_dir_all("target/review/outline").unwrap();

    for dpr in [1.0, 2.0, 3.0] {
        for samples in [1, 4] {
            gpu.logical_size = [960.0 / dpr, 600.0 / dpr];
            gpu.view.msaa_forced = Some(samples);
            gpu.resize(960, 600);

            for (mode, edges) in [(0, false), (1, false), (2, false), (0, true), (1, true)] {
                outline_mode(&mut gpu, mode);
                gpu.view.show_mesh_edges = edges;
                let pixels = timed(&mut gpu, &camera, &anchor).1;
                let name = format!("dpr{dpr}-msaa{samples}-mode{mode}-edges{edges}");
                std::fs::write(format!("target/review/outline/{name}.rgba"), &pixels).unwrap();
            }
        }
    }
}

#[test]
#[ignore = "benchmark, requires a native GPU adapter"]
/// Orbit and still frame times at 2880x1800, DPR 2, MSAA 4, outlines off and on.
fn bench_outline() {
    let (mut gpu, mut camera, anchor) = solids_scene(2880, 1800, 50);
    gpu.logical_size = [1440.0, 900.0];
    gpu.view.msaa_forced = Some(4);
    gpu.resize(2880, 1800);
    let mut report = String::new();

    for (mode, edges) in [
        (3, false),
        (0, false),
        (1, false),
        (2, false),
        (3, true),
        (0, true),
    ] {
        outline_mode(&mut gpu, mode);
        gpu.view.show_mesh_edges = edges;

        for _ in 0..5 {
            timed(&mut gpu, &camera, &anchor);
        }

        let still: f64 = (0..200)
            .map(|_| timed(&mut gpu, &camera, &anchor).0)
            .sum::<f64>()
            / 200.0;
        let mut orbit = 0.0;

        for _ in 0..200 {
            camera.orbit(3.49, 0.0);
            orbit += timed(&mut gpu, &camera, &anchor).0;
        }

        camera.orbit(-3.49 * 200.0, 0.0);
        report += &format!(
            "mode {mode}, edges {edges}: still {still:.2} ms, orbit {:.2} ms\n",
            orbit / 200.0
        );
    }

    std::fs::create_dir_all("target/review").unwrap();
    std::fs::write("target/review/bench-outline.txt", &report).unwrap();
}
