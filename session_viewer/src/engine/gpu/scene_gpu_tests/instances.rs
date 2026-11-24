use crate::app::scene::{FileDoc, Scene};
use crate::camera::Camera;
use crate::engine::gpu::{FrameInput, Gpu};
use session_rust::element::ElementFeature;
use session_rust::{AABB, Element, Geometry, InstanceRef, Mesh, Point, Polyline, Session, Xform};
use std::rc::Rc;

fn file(session: Session) -> FileDoc {
    FileDoc {
        name: "inst".into(),
        place: Xform::identity(),
        session: Rc::new(session),
        point_px: 3.0,
        display_only: false,
    }
}

fn placed(n: usize) -> (Session, String) {
    let mut s = Session::new("placed");
    let mut element = Element::new("box");
    element.set_geometry(Mesh::create_box(1.0, 1.0, 1.0));
    let axis = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 2.0)]);
    element.add_feature(ElementFeature::new("axis", -1, vec![axis], "axis"));
    let definition = s.add_definition(Geometry::Element(Rc::new(element)));
    let group = s.add_group("row");
    s.set_xform("row", Xform::translation(0.0, 0.0, 10.0));

    for i in 0..n {
        let mut instance =
            InstanceRef::with_name(&format!("box_{i}"), &definition, Xform::identity());

        if i == 0 {
            let dot = Polyline::new(vec![Point::new(0.5, 0.5, 0.5)]);
            instance
                .features
                .push(ElementFeature::new("joint", 0, vec![dot], "dot"));
        }

        let parent = (i + 1 == n).then_some(&group);
        s.add_instance(
            instance,
            Xform::translation(i as f64 * 3.0, 0.0, 0.0),
            parent,
        );
    }

    (s, definition)
}

fn turned_grid() -> Session {
    let mut s = Session::new("turned");
    let mut element = Element::new("beam");
    element.set_geometry(Mesh::create_box(3.0, 1.0, 1.0));
    let axis = Polyline::new(vec![Point::new(-2.0, 0.0, 0.0), Point::new(2.0, 0.0, 0.0)]);
    element.add_feature(ElementFeature::new("axis", -1, vec![axis], "axis"));
    let beam = s.add_definition(Geometry::Element(Rc::new(element)));
    let floor = s.add_definition(Geometry::Mesh(Rc::new(Mesh::create_box(18.0, 13.0, 0.2))));

    for i in 0..12 {
        let turn = &Xform::rotation_z(i as f64 * 37.0, true)
            * &Xform::rotation_x((i % 3) as f64 * 20.0, true);
        let at = Xform::translation((i % 4) as f64 * 3.5, (i / 4) as f64 * 3.5, 0.0);
        let instance = InstanceRef::with_name(&format!("beam_{i}"), &beam, Xform::identity());
        s.add_instance(instance, &at * &turn, None);
    }

    let under = Xform::translation(5.25, 3.5, -0.9);
    s.add_instance(InstanceRef::new(&floor, Xform::identity()), under, None);
    s
}

fn bake(source: &Session) -> Session {
    let mut baked = Session::new("baked");
    let geometry = source.get_geometry();

    for element in &geometry.elements {
        baked.add_element((**element).clone(), None);
    }

    for mesh in &geometry.meshes {
        baked.add_mesh((**mesh).clone(), None);
    }

    baked
}

fn rows(scene: &Scene) -> Vec<(String, u32)> {
    let mut out: Vec<_> = (0..scene.row_count() as u32)
        .filter(|row| scene.instancing.is_instance(*row))
        .map(|row| (scene.identity_of(row).unwrap().1.to_string(), row))
        .collect();
    out.sort();
    out
}

/// A 400 by 300 frame as `<dir>/<name>.ppm`, for looking at a failure.
fn write_ppm(dir: &std::ffi::OsStr, name: &str, rgba: &[u8]) {
    let mut ppm = b"P6 400 300 255\n".to_vec();
    ppm.extend(rgba.chunks_exact(4).flat_map(|p| [p[0], p[1], p[2]]));
    std::fs::write(std::path::Path::new(dir).join(format!("{name}.ppm")), ppm).unwrap();
}

/// `session` on a headless GPU, with ambient occlusion when `ssao`, and a camera fitted
/// to it, orbited when `rotate`.
fn loaded(session: Session, rotate: bool, ssao: bool) -> (Gpu, Scene, Camera) {
    let mut gpu = pollster::block_on(Gpu::new_headless(400, 300)).expect("a native adapter");
    gpu.view.show_grid = false;
    gpu.view.opacity = 0.7;
    gpu.view.set_arctic(ssao);
    let mut scene = Scene::new();
    scene.add_file(file(session));
    scene.upload_to(&mut gpu);
    let mut camera = Camera::new();
    camera.fit(&gpu.bounds, 4.0 / 3.0);

    if rotate {
        camera.orbit(0.6, 0.4);
    }

    (gpu, scene, camera)
}

/// Colors and object ids of one frame of `session`, fitted to its box, with ambient
/// occlusion when `ssao`.
fn shot_with(
    session: Session,
    rotate: bool,
    ssao: bool,
) -> (Vec<u8>, Vec<Option<String>>, Gpu, Scene) {
    let (mut gpu, scene, camera) = loaded(session, rotate, ssao);
    let (color, ids) = frame(&mut gpu, &scene, &camera);
    (color, ids, gpu, scene)
}

/// Colors and object ids of one frame seen by `camera`.
fn frame(gpu: &mut Gpu, scene: &Scene, camera: &Camera) -> (Vec<u8>, Vec<Option<String>>) {
    let anchor = gpu
        .rebase_anchor(&camera.origin(), camera.distance_world(), 0.0)
        .anchor;
    let input = FrameInput {
        view_proj: camera.view_proj_anchored(4.0 / 3.0, &anchor),
        clear: wgpu::Color::WHITE,
        now_ms: 0.0,
    };
    let color = gpu.render_offscreen(&input);
    let ids = gpu
        .render_ids_offscreen(&input)
        .iter()
        .map(|&[object, _]| {
            (object != 0).then(|| scene.identity_of(object - 1).unwrap().1.to_string())
        })
        .collect();
    (color, ids)
}

/// Instances drawn from one upload look and pick exactly like the same objects baked:
/// every pixel's object is the instance's guid, and faces, hidden edges and axes seen
/// through the glass, with and without ambient occlusion, match to the pixel.
#[test]
#[ignore = "requires a native GPU adapter"]
fn instanced_frames_match_the_baked_scene() {
    let scenes = [(placed(3).0, 1, 3), (turned_grid(), 2, 13)];

    for (source, batches, instances) in scenes {
        let baked = bake(&source);

        for (rotate, ssao) in [(false, false), (true, false), (true, true)] {
            let (color, ids, gpu, scene) = shot_with(source.clone(), rotate, ssao);
            let (want_color, want_ids, want_gpu, _) = shot_with(baked.clone(), rotate, ssao);
            // the fitted box is exact over turned instances, as over their baked twins
            let ends = |b: &AABB| [b.min_point(), b.max_point()];

            for (got, want) in ends(&gpu.bounds).iter().zip(ends(&want_gpu.bounds)) {
                assert!((0..3).all(|k| (got[k] - want[k]).abs() < 1e-3), "fit box");
            }

            assert_eq!(scene.instancing.batch_rows(), batches);
            assert_eq!(gpu.arena.source_faces.slots.instances(), instances);
            let drawn = ids.iter().filter(|id| id.is_some()).count();
            let same = ids.iter().zip(&want_ids).filter(|(a, b)| a == b).count();
            let pairs = color.chunks_exact(4).zip(want_color.chunks_exact(4));
            let apart = |levels: u8| {
                pairs
                    .clone()
                    .filter(|(a, b)| (0..3).any(|k| a[k].abs_diff(b[k]) > levels))
                    .count()
            };
            let (differ, far) = (apart(2), apart(8));

            if let Some(dir) = std::env::var_os("INSTANCING_SHOTS") {
                let name = format!("{}_{rotate}_{ssao}", source.name);
                write_ppm(&dir, &format!("{name}_instanced"), &color);
                write_ppm(&dir, &format!("{name}_baked"), &want_color);
            }
            eprintln!(
                "{} rotate {rotate} ssao {ssao}: {drawn} object pixels, {} id mismatches, {} exact color mismatches, {differ} by more than 2 levels, {far} by more than 8",
                source.name,
                ids.len() - same,
                apart(0)
            );
            assert!(drawn > 1000, "the instances are drawn");
            // turned instances are placed in f32 on the GPU: a few silhouette pixels flip
            assert!(
                (ids.len() - same) * 200 <= drawn,
                "each pixel picks the same instance"
            );
            // a turned normal shades a level or two apart, and f32 placement on the GPU
            // against f64 baking flips a rare edge pixel
            let pixels = color.len() / 4;
            assert!(differ * 2_000 <= drawn, "{differ} pixels differ in color");
            assert!(far * 20_000 <= pixels, "{far} pixels differ visibly");
        }
    }
}

/// Deleting, moving, exploding and undoing turned instances on a live GPU: a delete writes
/// only its own slots, and after every edit the frame and its ids match the same objects
/// loaded fresh.
#[test]
#[ignore = "requires a native GPU adapter"]
fn instance_edits_write_their_own_slots() {
    let (mut gpu, mut scene, camera) = loaded(turned_grid(), true, true);
    let beam = |name: &str| {
        rows(&scene)
            .into_iter()
            .find(|(guid, _)| scene.docs[0].session.instance_lookup[guid].name == name)
            .unwrap()
    };
    let (deleted, moved, exploded) = (beam("beam_2").1, beam("beam_5").1, beam("beam_7").0);
    let mut written = Vec::new();

    for step in 0..6 {
        match step {
            0 => assert!(scene.delete_row(deleted)),
            1 => {
                let turn = Xform::rotation_z(25.0, true);
                assert!(scene.transform_rows(&[moved], &turn, "Move").is_some());
            }
            2 => {
                let session = Rc::make_mut(&mut scene.docs[0].session);
                session.begin("Explode");
                assert!(session.explode(&exploded));
                let notes = crate::app::scene::sync::commit(session);
                scene.noted(0, notes);
                scene.edited(&[0]);
            }
            _ => assert!(scene.undo()),
        }

        gpu.arena.space.written = 0;
        scene.sync();
        scene.upload_to(&mut gpu);
        written.push(gpu.arena.space.written);
        let (color, ids) = frame(&mut gpu, &scene, &camera);
        let fresh = (*scene.docs[0].session).clone();
        let (mut want_gpu, want_scene, _) = loaded(fresh, true, true);
        let (want, want_ids) = frame(&mut want_gpu, &want_scene, &camera);
        let drawn = ids.iter().filter(|id| id.is_some()).count();
        let same = ids.iter().zip(&want_ids).filter(|(a, b)| a == b).count();
        let differ = color
            .chunks_exact(4)
            .zip(want.chunks_exact(4))
            .filter(|(a, b)| (0..3).any(|k| a[k].abs_diff(b[k]) > 2))
            .count();
        eprintln!(
            "step {step}: {} slots written, {drawn} object pixels, {} ids and {differ} colors apart",
            written[step],
            ids.len() - same
        );
        assert!(drawn > 1000);
        // a leaver's slot takes the last member: where two beams meet, a pixel may flip
        assert!((ids.len() - same) * 1000 <= drawn, "step {step}: ids");
        assert!(differ * 1000 <= drawn, "step {step}: colors");
    }

    // head, the slot the last beam moved into, the slot it left
    assert!(written[0] <= 3, "a delete writes {}", written[0]);
    assert_eq!(written[1], 0, "a move writes no slot");
}

/// Walls standing on a floor, one of them turned a quarter: with ambient occlusion the
/// instanced frame matches the baked one, the turned wall included, since each instance
/// triangle names its own row, matrix and contact radius.
#[test]
#[ignore = "requires a native GPU adapter"]
fn ambient_occlusion_of_turned_instances() {
    let mut s = Session::new("walls");
    let wall = s.add_definition(Geometry::Mesh(Rc::new(Mesh::create_box(8.0, 0.4, 3.0))));
    let floor = s.add_definition(Geometry::Mesh(Rc::new(Mesh::create_box(30.0, 30.0, 0.2))));
    let turn = Xform::rotation_z(90.0, true);
    let places = [
        (&wall, Xform::translation(0.0, 0.0, 1.6)),
        (&wall, &Xform::translation(6.0, 4.0, 1.6) * &turn),
        (&floor, Xform::identity()),
    ];

    for (definition, place) in places {
        s.add_instance(InstanceRef::new(definition, Xform::identity()), place, None);
    }

    let mut baked = Session::new("baked");

    for mesh in &s.get_geometry().meshes {
        baked.add_mesh((**mesh).clone(), None);
    }

    let (color, _, _, _) = shot_with(s, true, true);
    let (want, _, _, _) = shot_with(baked, true, true);
    let far = color
        .chunks_exact(4)
        .zip(want.chunks_exact(4))
        .filter(|(a, b)| (0..3).any(|k| a[k].abs_diff(b[k]) > 24))
        .count();

    if let Some(dir) = std::env::var_os("INSTANCING_SHOTS") {
        write_ppm(&dir, "walls_instanced", &color);
        write_ppm(&dir, "walls_baked", &want);
    }

    eprintln!("ambient occlusion: {far} pixels differ by more than 24 levels");
    assert!(far * 20_000 <= 400 * 300, "{far} pixels differ");
}

/// INSTANCING_PAIR=<instanced.pb>,<baked.pb>: pixels of the two files' frames apart by more
/// than 2 and 8 levels, with ambient occlusion, and how far their fitted boxes are apart.
#[test]
#[ignore = "measurement: INSTANCING_PAIR=<instanced.pb>,<baked.pb>"]
fn instanced_file_against_its_baked_twin() {
    let pair = std::env::var("INSTANCING_PAIR").expect("INSTANCING_PAIR=<a.pb>,<b.pb>");
    let load = |path: &str| Session::pb_loads(&std::fs::read(path).unwrap()).unwrap();
    let (on, off) = pair.split_once(',').expect("two paths");

    for ssao in [false, true] {
        let (color, _, gpu, _) = shot_with(load(on), true, ssao);
        let (want, _, want_gpu, _) = shot_with(load(off), true, ssao);
        let pairs = color.chunks_exact(4).zip(want.chunks_exact(4));
        let apart = |levels: u8| {
            pairs
                .clone()
                .filter(|(a, b)| (0..3).any(|k| a[k].abs_diff(b[k]) > levels))
                .count()
        };
        let (a, b) = (&gpu.bounds, &want_gpu.bounds);
        let fit = [a.min_point(), a.max_point()]
            .iter()
            .zip([b.min_point(), b.max_point()])
            .flat_map(|(p, q)| (0..3).map(move |k| (p[k] - q[k]).abs()))
            .fold(0.0, f64::max);
        println!(
            "ssao {ssao}: {} of {} pixels apart by more than 2 levels, {} by more than 8; fit boxes {fit:.6} apart ({} and {})",
            apart(2),
            color.len() / 4,
            apart(8),
            a.str(),
            b.str()
        );
    }
}

/// Median wall time of an offscreen 1920x1080 frame of INSTANCING_BENCH (a .pb), orbiting,
/// with and without ambient occlusion; frames are read back, so the GPU work is inside.
#[test]
#[ignore = "benchmark: INSTANCING_BENCH=<scene.pb>, run with buildslot --exclusive"]
fn bench_frames() {
    let path = std::env::var("INSTANCING_BENCH").expect("INSTANCING_BENCH protobuf path");
    let source = Session::pb_loads(&std::fs::read(&path).unwrap()).unwrap();
    let mut gpu = pollster::block_on(Gpu::new_headless(1920, 1080)).expect("a native adapter");
    gpu.view.msaa_forced = Some(4);
    gpu.resize(1920, 1080);
    gpu.view.show_grid = false;
    gpu.view.opacity = 0.7;
    let mut scene = Scene::new();
    scene.add_file(file(source));
    scene.upload_to(&mut gpu);
    let mut camera = Camera::new();
    camera.fit(&gpu.bounds, 1920.0 / 1080.0);

    for ssao in [false, true] {
        gpu.view.set_arctic(ssao);
        let mut times = Vec::new();

        for i in 0..64 {
            camera.orbit(0.05, 0.0);
            let anchor = gpu
                .rebase_anchor(&camera.origin(), camera.distance_world(), 0.0)
                .anchor;
            let input = FrameInput {
                view_proj: camera.view_proj_anchored(1920.0 / 1080.0, &anchor),
                clear: wgpu::Color::WHITE,
                now_ms: 0.0,
            };
            let start = std::time::Instant::now();
            gpu.render_offscreen(&input);

            // the first frames compile pipelines
            if i >= 8 {
                times.push(start.elapsed().as_secs_f64() * 1000.0);
            }
        }

        times.sort_by(f64::total_cmp);
        println!(
            "ssao {ssao}: median {:.2} ms, p90 {:.2} ms, {} draws",
            times[times.len() / 2],
            times[times.len() * 9 / 10],
            gpu.performance.draws
        );
    }
}
