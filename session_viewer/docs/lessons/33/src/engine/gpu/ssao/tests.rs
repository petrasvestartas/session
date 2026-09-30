#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "requires a native GPU adapter"]
/// Contact darkens pixels, far ground stays bright, memory returns.
fn occlusion_darkens_contact_and_releases_its_small_uniform() {
    use crate::app::scene::{FileDoc, Scene};
    use crate::camera::Camera;
    use crate::engine::gpu::{FrameInput, Gpu};
    use session_rust::{BRep, Session, Xform};
    use std::rc::Rc;
    let mut gpu = pollster::block_on(Gpu::new_headless(256, 256)).unwrap();
    gpu.view.show_grid = false;
    let mut source = Session::new("contact");
    source.add_brep(BRep::create_box(100.0, 100.0, 10.0), None);
    let tower = source
        .add_brep(BRep::create_box(30.0, 30.0, 80.0), None)
        .unwrap();
    source.set_xform(&tower.borrow().name, Xform::translation(0.0, 0.0, 35.0));
    let raised = source
        .add_brep(BRep::create_box(40.0, 40.0, 10.0), None)
        .unwrap();
    source.set_xform(&raised.borrow().name, Xform::translation(110.0, 0.0, 12.0));
    if std::env::var_os("VIEWER_AO_CAPTURE").is_some() {
        std::fs::create_dir_all("target/review").unwrap();
        std::fs::write("target/review/ambient-contact.pb", source.pb_dumps()).unwrap();
    }
    let mut scene = Scene::new();
    scene.add_file(FileDoc {
        name: "contact".into(),
        session: Rc::new(source),
        place: Xform::identity(),
        point_px: 0.0,
        display_only: false,
    });
    scene.upload_to(&mut gpu);
    let mut camera = Camera::new();
    camera.fit(&gpu.bounds, 1.0);
    let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 0.0);
    for (samples, perspective) in [(1, true), (4, true), (1, false), (4, false)] {
        camera.perspective = perspective;
        let input = FrameInput {
            view_proj: camera.view_proj_anchored(1.0, &rebase.anchor),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        };
        gpu.view.msaa_forced = Some(samples);
        gpu.resize(256, 256);
        gpu.view.ssao = false;
        let plain = gpu.render_offscreen(&input);
        let memory = gpu.allocated_bytes();
        gpu.view.ssao = true;
        let partial = gpu.render_offscreen(&input);
        let shaded = gpu.render_offscreen(&input);
        assert_eq!(partial, shaded, "one frame computes the complete image");
        if std::env::var_os("VIEWER_AO_CAPTURE").is_some() {
            std::fs::write(format!("target/review/ambient-{samples}x-off.rgba"), &plain)
                .unwrap();
            std::fs::write(format!("target/review/ambient-{samples}x-on.rgba"), &shaded)
                .unwrap();
        }
        for shaded in [&partial, &shaded] {
            let ground_shadow = plain
                .chunks_exact(4)
                .zip(shaded.chunks_exact(4))
                .filter(|(a, b)| {
                    a[0] == 255
                        && a[1] == 255
                        && a[2] == 255
                        && b[0] < shaded[0].saturating_sub(3)
                })
                .count();
            assert!(
                ground_shadow > 20,
                "virtual ground receives soft shadows: {ground_shadow}"
            );
            let mut distant_ground = 0;
            for (i, (a, b)) in plain
                .chunks_exact(4)
                .zip(shaded.chunks_exact(4))
                .enumerate()
            {
                if a[..3] != [255, 255, 255] {
                    continue;
                }
                let (origin, direction) = camera
                    .ray(
                        ((i % 256) as f64 + 0.5, (i / 256) as f64 + 0.5),
                        (256.0, 256.0),
                    )
                    .unwrap();
                if direction[2] >= 0.0 {
                    continue;
                }
                let t = (-5.0 - origin[2]) / direction[2];
                let x = origin[0] + t * direction[0];
                let y = origin[1] + t * direction[1];
                let distance = (x.abs() - 50.0).max(0.0).hypot((y.abs() - 50.0).max(0.0));
                if t > 0.0 && distance > 25.0 {
                    distant_ground += 1;
                    assert!(
                        b[0] >= shaded[0].saturating_sub(2),
                        "ground halo away from base at ({x}, {y}), {samples}x, perspective={perspective}: {}",
                        b[0]
                    );
                }
            }
            assert!(
                distant_ground > 1000,
                "check exposed ground around raised geometry"
            );
            let darkened = plain
                .chunks_exact(4)
                .zip(shaded.chunks_exact(4))
                .filter(|(a, b)| a[0] > b[0].saturating_add(2))
                .count();
            assert!(
                darkened > 20,
                "contact occlusion changes pixels at {samples}x: {darkened}"
            );
        }
        assert_eq!(
            gpu.allocated_bytes(),
            (
                memory.0 + gpu.pass::<super::Ambient>().ambient().buffer_bytes(),
                memory.1 + gpu.pass::<super::Ambient>().ambient().texture_bytes()
            )
        );
        gpu.view.ssao = false;
        assert_eq!(gpu.render_offscreen(&input), plain);
        assert_eq!(gpu.allocated_bytes(), memory);
    }
}

/// A headless canvas showing the contact scene, and its camera.
#[cfg(not(target_arch = "wasm32"))]
fn contact_scene(
    width: u32,
    height: u32,
) -> (
    crate::engine::gpu::Gpu,
    crate::camera::Camera,
    session_rust::Point,
) {
    use crate::app::scene::{FileDoc, Scene};
    use crate::camera::Camera;
    use crate::engine::gpu::Gpu;
    use session_rust::{BRep, Session, Xform};
    use std::rc::Rc;
    let mut gpu = pollster::block_on(Gpu::new_headless(width, height)).unwrap();
    gpu.view.show_grid = false;
    let mut source = Session::new("contact");
    source.add_brep(BRep::create_box(100.0, 100.0, 10.0), None);
    let tower = source
        .add_brep(BRep::create_box(30.0, 30.0, 80.0), None)
        .unwrap();
    source.set_xform(&tower.borrow().name, Xform::translation(0.0, 0.0, 35.0));
    let raised = source
        .add_brep(BRep::create_box(40.0, 40.0, 10.0), None)
        .unwrap();
    source.set_xform(&raised.borrow().name, Xform::translation(110.0, 0.0, 12.0));
    let mut scene = Scene::new();
    scene.add_file(FileDoc {
        name: "contact".into(),
        session: Rc::new(source),
        place: Xform::identity(),
        point_px: 0.0,
        display_only: false,
    });
    scene.upload_to(&mut gpu);
    let mut camera = Camera::new();
    camera.fit(&gpu.bounds, width as f64 / height as f64);
    let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 0.0);
    (gpu, camera, rebase.anchor)
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "native GPU benchmark; run with buildslot --exclusive"]
fn benchmark_arctic() {
    use crate::app::scene::{FileDoc, Scene};
    use crate::camera::Camera;
    use crate::engine::gpu::{Gpu, timing::PassTimer};
    use session_rust::{Session, Xform};
    use std::rc::Rc;
    let path = std::env::var("AO_SCENE").expect("AO_SCENE protobuf path");
    let output = std::env::var("AO_OUTPUT").expect("AO_OUTPUT directory");
    let samples: u32 = std::env::var("AO_SAMPLES")
        .unwrap_or("1".into())
        .parse()
        .unwrap();
    let source = Session::pb_loads(&std::fs::read(&path).unwrap()).unwrap();
    let mut scene = Scene::new();
    let scale: f64 = std::env::var("AO_SCALE")
        .unwrap_or("1".into())
        .parse()
        .unwrap();
    scene.add_file(FileDoc {
        name: "arctic benchmark".into(),
        session: Rc::new(source),
        place: Xform::from_matrix([
            scale, 0.0, 0.0, 0.0, 0.0, 0.0, scale, 0.0, 0.0, -scale, 0.0, 0.0, 0.0, 0.0, 0.0,
            1.0,
        ]),
        point_px: 0.0,
        display_only: false,
    });
    let mut gpu = pollster::block_on(Gpu::new_headless(1920, 1080)).unwrap();
    gpu.view.msaa_forced = Some(samples);
    gpu.resize(1920, 1080);
    gpu.view.show_grid = false;
    scene.upload_to(&mut gpu);
    if std::env::var_os("AO_NO_EDGES").is_some() {
        gpu.view.show_mesh_edges = false;
        gpu.view.show_lines = false;
    }
    let mut camera = Camera::new();
    camera.fit(&gpu.bounds, 1920.0 / 1080.0);
    let anchor = gpu
        .rebase_anchor(&camera.origin(), camera.distance_world(), 0.0)
        .anchor;
    std::fs::create_dir_all(&output).unwrap();
    let write = |name: &str, pixels: Vec<u8>| {
        std::fs::write(format!("{output}/{name}.rgba"), pixels).unwrap();
    };
    write("off", timed(&mut gpu, &camera, &anchor).1);
    gpu.view.ssao = true;
    for _ in 0..4 {
        timed(&mut gpu, &camera, &anchor);
    }
    write("still", timed(&mut gpu, &camera, &anchor).1);
    gpu.performance.interacting = true;
    write("drag", timed(&mut gpu, &camera, &anchor).1);
    gpu.performance.interacting = false;
    write("release", timed(&mut gpu, &camera, &anchor).1);
    let mut report = String::new();
    report += &format!(
        "samples={samples} buffers={} textures={} ao_textures={}\n",
        gpu.allocated_bytes().0,
        gpu.allocated_bytes().1,
        gpu.pass::<super::Ambient>().ambient().texture_bytes()
    );
    for mode in ["still", "moved", "drag"] {
        gpu.performance.interacting = mode == "drag";
        for _ in 0..4 {
            timed(&mut gpu, &camera, &anchor);
        }
        gpu.timer = Some(PassTimer::new(&gpu.ctx).expect("GPU timestamp support"));
        for _ in 0..24 {
            if mode != "still" {
                camera.orbit(1.745, 0.0);
            }
            timed(&mut gpu, &camera, &anchor);
        }
        let timer = gpu.timer.as_ref().unwrap();
        report += &format!("{mode}: {:?}\n", timer.medians());
        let mut ao_total = [0.0_f64; 24];
        let mut frame_total = [0.0_f64; 24];
        for (label, series) in &timer.spans {
            for (i, value) in series.iter().take(24).enumerate() {
                frame_total[i] += value;
                if *label == "ssao" || label.starts_with("ao.") {
                    ao_total[i] += value;
                }
            }
        }
        ao_total.sort_by(f64::total_cmp);
        frame_total.sort_by(f64::total_cmp);
        report += &format!(
            "{mode} AO total: {:.3} ms; frame GPU: {:.3} ms\n",
            ao_total[12], frame_total[12]
        );
        gpu.timer = None;
        if mode == "moved" {
            // the last moving frame against a fresh frame at the same camera
            write("moved", timed(&mut gpu, &camera, &anchor).1);
            gpu.view.ssao = false;
            timed(&mut gpu, &camera, &anchor);
            gpu.view.ssao = true;
            write("fresh", timed(&mut gpu, &camera, &anchor).1);
        }
        if mode != "still" {
            for _ in 0..24 {
                camera.orbit(-1.745, 0.0);
            }
        }
    }
    std::fs::write(format!("{output}/timings.txt"), &report).unwrap();
    println!("{report}");
}

/// Milliseconds of one offscreen frame, read back to the CPU.
#[cfg(not(target_arch = "wasm32"))]
fn timed(
    gpu: &mut crate::engine::gpu::Gpu,
    camera: &crate::camera::Camera,
    anchor: &session_rust::Point,
) -> (f64, Vec<u8>) {
    let aspect = gpu.config.width as f64 / gpu.config.height as f64;
    let input = crate::engine::gpu::FrameInput {
        view_proj: camera.view_proj_anchored(aspect, anchor),
        clear: wgpu::Color::WHITE,
        now_ms: 0.0,
    };
    let start = std::time::Instant::now();
    let pixels = gpu.render_offscreen(&input);
    (start.elapsed().as_secs_f64() * 1000.0, pixels)
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "benchmark, requires a native GPU adapter"]
/// Toggle, resize, still and orbit timings at 1920x1080; writes captures.
fn bench_ambient() {
    let (mut gpu, mut camera, anchor) = contact_scene(1920, 1080);
    let mut report = String::new();
    gpu.view.msaa_forced = Some(4);
    gpu.resize(1920, 1080);
    timed(&mut gpu, &camera, &anchor);
    let mut toggles = Vec::new();
    for _ in 0..10 {
        gpu.view.ssao = true;
        toggles.push(timed(&mut gpu, &camera, &anchor).0);
        gpu.view.ssao = false;
        timed(&mut gpu, &camera, &anchor);
    }
    report += &format!("toggle on ms: {toggles:.1?}\n");
    gpu.view.ssao = true;
    let resizes: Vec<f64> = (1..=10)
        .map(|k| {
            gpu.resize(1920 + k, 1080);
            timed(&mut gpu, &camera, &anchor).0
        })
        .collect();
    report += &format!("resize ms: {resizes:.1?}\n");
    for samples in [1, 4] {
        gpu.view.msaa_forced = Some(samples);
        gpu.resize(1920, 1080);
        for ssao in [false, true] {
            gpu.view.ssao = ssao;
            let pixels = (0..4).fold(Vec::new(), |_, _| timed(&mut gpu, &camera, &anchor).1);
            if ssao {
                std::fs::create_dir_all("target/review").unwrap();
                std::fs::write(format!("target/review/bench-{samples}x.rgba"), &pixels)
                    .unwrap();
            }
            let still: f64 = (0..120)
                .map(|_| timed(&mut gpu, &camera, &anchor).0)
                .sum::<f64>()
                / 120.0;
            let mut orbit = 0.0;
            for _ in 0..120 {
                camera.orbit(1.745, 0.0);
                orbit += timed(&mut gpu, &camera, &anchor).0;
            }
            for _ in 0..120 {
                camera.orbit(-1.745, 0.0);
            }
            // the same orbit as a drag
            gpu.performance.interacting = true;
            let mut drag = 0.0;
            for _ in 0..120 {
                camera.orbit(1.745, 0.0);
                drag += timed(&mut gpu, &camera, &anchor).0;
            }
            for _ in 0..120 {
                camera.orbit(-1.745, 0.0);
            }
            gpu.performance.interacting = false;
            report += &format!(
                "{samples}x ssao={ssao}: still {still:.2} ms, orbit {:.2} ms, drag {:.2} ms\n",
                orbit / 120.0,
                drag / 120.0
            );
        }
    }
    std::fs::write("target/review/bench-ambient.txt", &report).unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "requires a native GPU adapter"]
/// After the first frame with occlusion, toggles and resizes compile no pipeline or shader.
fn toggles_and_resizes_compile_nothing_after_the_first_frame() {
    let (mut gpu, camera, anchor) = contact_scene(256, 256);
    gpu.view.msaa_forced = Some(4);
    gpu.resize(256, 256);
    gpu.view.ssao = true;
    timed(&mut gpu, &camera, &anchor);
    let compiled = crate::engine::pipelines::created();

    for _ in 0..20 {
        gpu.view.ssao = false;
        timed(&mut gpu, &camera, &anchor);
        gpu.view.ssao = true;
        timed(&mut gpu, &camera, &anchor);
    }

    for step in 1..=20 {
        gpu.resize(256 + step, 256);
        timed(&mut gpu, &camera, &anchor);
    }

    assert_eq!(crate::engine::pipelines::created(), compiled);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "requires a native GPU adapter"]
/// Arctic preserves shading, lines and outlines through slow drags and release.
fn navigation_preserves_the_same_image_and_needs_no_settling() {
    for samples in [1, 4] {
        let (mut gpu, mut camera, anchor) = contact_scene(256, 256);
        gpu.view.msaa_forced = Some(samples);
        gpu.resize(256, 256);
        gpu.view.ssao = false;
        let plain = timed(&mut gpu, &camera, &anchor).1;
        gpu.view.set_arctic(true);
        let still = timed(&mut gpu, &camera, &anchor).1;
        let memory = gpu.allocated_bytes();

        if std::env::var_os("VIEWER_AO_CAPTURE").is_some() {
            std::fs::create_dir_all("target/review").unwrap();
            std::fs::write(
                format!("target/review/ambient-{samples}x-still.rgba"),
                &still,
            )
            .unwrap();
        }

        gpu.performance.interacting = true;
        let dragged = timed(&mut gpu, &camera, &anchor).1;
        assert!(!gpu.ambient_pending());
        assert_eq!(dragged, still, "drag entry preserves the image");

        if std::env::var_os("VIEWER_AO_CAPTURE").is_some() {
            std::fs::create_dir_all("target/review").unwrap();
            std::fs::write(
                format!("target/review/ambient-{samples}x-drag.rgba"),
                &dragged,
            )
            .unwrap();
        }

        assert_eq!(gpu.allocated_bytes(), memory, "no texture for the drag");
        let darkened = plain
            .chunks_exact(4)
            .zip(dragged.chunks_exact(4))
            .filter(|(a, b)| a[0] > b[0].saturating_add(2))
            .count();
        assert!(
            darkened > 20,
            "a drag still shades contact at {samples}x: {darkened}"
        );
        gpu.performance.interacting = false;
        let restored = timed(&mut gpu, &camera, &anchor).1;
        assert!(!gpu.ambient_pending());
        assert!(restored == still, "the still image returns at {samples}x");
        for tier in 0..=2 {
            gpu.performance.interacting = true;
            if tier > 0 {
                for frame in 0..8 {
                    gpu.performance
                        .frame(0, 0, 1000.0 * (frame + tier * 8) as f64, false);
                }
            }
            if tier == 2 {
                assert_eq!(gpu.performance.drag_tier(), 2);
            }
            camera.orbit(7.0, 3.0);
            let moving = timed(&mut gpu, &camera, &anchor).1;
            gpu.performance.interacting = false;
            let stopped = timed(&mut gpu, &camera, &anchor).1;
            assert!(
                moving == stopped,
                "Arctic lines, outlines and shadows stay unchanged after release at tier {tier}"
            );
            assert!(!gpu.ambient_pending());
            assert_eq!(gpu.allocated_bytes(), memory);
        }
        gpu.set_hidden(1, true);
        let hidden = timed(&mut gpu, &camera, &anchor).1;
        gpu.pass_mut::<super::Ambient>().ssao = None;
        assert_eq!(
            hidden,
            timed(&mut gpu, &camera, &anchor).1,
            "geometry changes discard old shadows immediately"
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "requires a native GPU adapter"]
fn rotation_reprojects_ground_shadows_without_erasing_them() {
    let probes: Vec<_> = (-35_i32..=35)
        .flat_map(|x| {
            (-35_i32..=35).filter_map(move |y| {
                (x.abs() > 25 || y.abs() > 25).then_some([
                    f64::from(x * 2),
                    f64::from(y * 2),
                    -5.0,
                ])
            })
        })
        .collect();
    let measure = |history| {
        let (mut gpu, mut camera, anchor) = contact_scene(512, 512);
        gpu.view.msaa_forced = Some(4);
        gpu.resize(512, 512);
        gpu.view.ssao = true;
        timed(&mut gpu, &camera, &anchor);
        gpu.pass_mut::<super::Ambient>()
            .ssao
            .as_mut()
            .unwrap()
            .history_enabled = history;
        let mut frames = Vec::new();
        for _ in 0..32 {
            camera.orbit(1.0, 0.0);
            let matrix = camera.view_proj_anchored(1.0, &anchor).m;
            frames.push((matrix, timed(&mut gpu, &camera, &anchor).1));
        }
        gpu.view.ssao = false;
        let mut previous = vec![None; probes.len()];
        let (mut energy, mut changes, mut strength, mut visible) = (0.0_f64, 0, 0.0_f64, 0);
        for (matrix, shaded) in frames {
            let mut view_proj = session_rust::Xform::identity();
            view_proj.m = matrix;
            let plain = gpu.render_offscreen(&crate::engine::gpu::FrameInput {
                view_proj,
                clear: wgpu::Color::WHITE,
                now_ms: 0.0,
            });
            for (index, point) in probes.iter().enumerate() {
                let p: [f64; 3] = std::array::from_fn(|i| point[i] - anchor[i]);
                let clip: [f64; 4] = std::array::from_fn(|r| {
                    matrix[r] * p[0]
                        + matrix[r + 4] * p[1]
                        + matrix[r + 8] * p[2]
                        + matrix[r + 12]
                });
                let x = (clip[0] / clip[3] * 0.5 + 0.5) * 512.0 - 0.5;
                let y = (0.5 - clip[1] / clip[3] * 0.5) * 512.0 - 0.5;
                let value = if clip[3] > 0.0 && x >= 0.0 && y >= 0.0 && x < 511.0 && y < 511.0 {
                    let at = (y.floor() as usize * 512 + x.floor() as usize) * 4;
                    let taps = [at, at + 4, at + 512 * 4, at + 513 * 4];
                    if taps.iter().all(|at| plain[*at..*at + 3] == [255, 255, 255]) {
                        let a = f64::from(shaded[taps[0]]) * (1.0 - x.fract())
                            + f64::from(shaded[taps[1]]) * x.fract();
                        let b = f64::from(shaded[taps[2]]) * (1.0 - x.fract())
                            + f64::from(shaded[taps[3]]) * x.fract();
                        Some(f64::from(shaded[0]) - a * (1.0 - y.fract()) - b * y.fract())
                    } else {
                        None
                    }
                } else {
                    None
                };
                if let Some(v) = value {
                    strength += v;
                    visible += 1;
                    if let Some(old) = previous[index] {
                        let delta: f64 = v - old;
                        energy += delta * delta;
                        changes += 1;
                    }
                }
                previous[index] = value;
            }
        }
        assert!(changes > 10_000);
        (
            (energy / f64::from(changes)).sqrt(),
            strength / f64::from(visible),
        )
    };
    let spatial = measure(false);
    let temporal = measure(true);
    println!(
        "ground rotation: spatial {spatial:?}, reprojected {temporal:?} (RMS change, mean shadow)"
    );
    assert!(
        temporal.0 < spatial.0 * 0.9,
        "rotation should reduce shimmer"
    );
    assert!(
        temporal.1 > spatial.1 * 0.85,
        "stabilizing must retain contact shadows"
    );
}

#[test]
fn resolution_follows_device_pixels_without_a_navigation_mode() {
    assert_eq!(super::resolution((1920, 1080), 1.0), (960, 540));
    assert_eq!(super::resolution((3840, 2160), 2.0), (1920, 1080));
    assert_eq!(super::resolution((1179, 2556), 3.0), (393, 852));
    assert_eq!(super::resolution((1920, 1080), 5.0), (480, 270));
}

#[test]
fn shader_validates_for_both_depth_sample_counts() {
    let sources = [
        super::shader_source(1),
        super::shader_source(4),
        shader!("ambient_depth.wgsl").to_owned(),
        shader!("ambient_composite.wgsl").to_owned(),
    ];
    for source in sources {
        let module = naga::front::wgsl::parse_str(&source).unwrap();
        naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        )
        .validate(&module)
        .unwrap();
        for (_, ty) in module.types.iter() {
            if ty.name.as_deref() == Some("Object") {
                use super::super::instance::Instance;
                use std::mem::{offset_of, size_of};
                let naga::TypeInner::Struct { ref members, span } = ty.inner else {
                    panic!("Object is a struct");
                };
                assert_eq!(span as usize, size_of::<Instance>());
                assert_eq!(
                    members
                        .iter()
                        .map(|member| member.offset as usize)
                        .collect::<Vec<_>>(),
                    [
                        offset_of!(Instance, model),
                        offset_of!(Instance, color),
                        offset_of!(Instance, flags),
                        offset_of!(Instance, ao_radius),
                        offset_of!(Instance, spacing),
                        offset_of!(Instance, _pad)
                    ]
                );
            }
            if ty.name.as_deref() == Some("Ambient") {
                let naga::TypeInner::Struct { ref members, span } = ty.inner else {
                    panic!("Ambient is a struct");
                };
                assert_eq!(span, 320);
                assert_eq!(
                    members
                        .iter()
                        .map(|member| member.offset)
                        .collect::<Vec<_>>(),
                    [0, 64, 128, 144, 160, 176, 192, 208, 224, 240, 256]
                );
            }
        }
    }
}
