use crate::{
    cap_gpu_tests::{at, pixels_at, renderer, wait},
    pick_sources::{Mode, Part, Target},
    renderer::Renderer,
    scene::Scene,
};
use prost::Message;
use session_rust::{Color, Line, Point, Xform, proto};
use std::rc::Rc;

fn project(matrix: [f32; 16], point: [f64; 3]) -> [f32; 2] {
    let point = point.map(|value| value as f32);
    let clip: [f32; 4] = std::array::from_fn(|row| {
        point
            .iter()
            .enumerate()
            .fold(matrix[12 + row], |sum, (i, value)| {
                sum + matrix[i * 4 + row] * value
            })
    });
    [clip[0] / clip[3], clip[1] / clip[3]]
}

fn archive(scene: &Scene) -> (Vec<proto::Session>, serde_json::Value) {
    let bundle =
        crate::archive::Bundle::decode(&crate::scene_archive::save(scene).unwrap()).unwrap();
    (
        bundle
            .documents
            .iter()
            .map(|bytes| proto::Session::decode(bytes.as_slice()).unwrap())
            .collect(),
        serde_json::to_value(bundle.metadata).unwrap(),
    )
}

fn pick(
    gpu: &mut Renderer,
    scene: &Scene,
    matrix: [f32; 16],
    point: [f64; 3],
    mode: Mode,
) -> Option<Target> {
    gpu.set_pick_mode(mode);
    gpu.request_pick(project(matrix, point), matrix).unwrap();
    wait(gpu);
    gpu.take_pick(matrix)
        .expect("Fresh GPU query completed")
        .unwrap()
        .and_then(|raw| gpu.resolve_pick(scene, raw))
}

fn pixel(image: &[u8], matrix: [f32; 16], point: [f64; 3]) -> [u8; 4] {
    let ndc = project(matrix, point);
    let x = ((ndc[0] + 1.) * 160.).floor() as usize;
    let y = ((1. - ndc[1]) * 120.).floor() as usize;
    assert!(x < 320 && y < 240);
    at(image, x, y).try_into().unwrap()
}

#[test]
fn standalone_line_fresh_gpu_picks_follow_placement_and_rebase_without_rewriting_source() {
    let mut line = Line::new(-0.625, -0.375, -0.5, -0.125, -0.375, -0.5);
    line.width = 8.;
    line.linecolor = Color::new(0., 1., 0., 1.);
    let source = Rc::new(line);
    let mut scene = Scene::empty();
    let owner = scene
        .insert_line(crate::stroke::PreparedLine::new(source.clone()).unwrap())
        .unwrap();
    let original = source.to_proto();
    let initial_archive = archive(&scene);
    let mut gpu = renderer(&scene);
    let mut camera = crate::camera_defaults_tests::top_fixture();
    camera.projection = crate::camera::Projection::Orthographic;
    camera.aspect = 4. / 3.;
    let mirror = &Xform::translation(-0.375, -0.25, 0.) * &Xform::scale_xyz(-1.5, 0.5, 1.);
    let far = [1_000_000_000., -1_000_000_000., 1_000_000_000.];
    let poses = [
        (Xform::identity(), [0.; 3]),
        (Xform::translation(0.75, 0.75, 0.), [0.; 3]),
        (mirror.clone(), [0.; 3]),
        (&Xform::translation(far[0], far[1], far[2]) * &mirror, far),
    ];
    let mut previous = None;
    let mut mirror_pixels = None;
    for (index, (model, origin)) in poses.into_iter().enumerate() {
        scene.place_line(owner, model.clone()).unwrap();
        let placed_archive = archive(&scene);
        let saved_lines = placed_archive
            .0
            .iter()
            .flat_map(|doc| &doc.objects.as_ref().unwrap().lines)
            .collect::<Vec<_>>();
        assert_eq!(
            saved_lines,
            [&original],
            "Placement stores a transform, preserving the original Line protobuf"
        );
        gpu.set_scene(&scene, None);
        camera.target = origin;
        let rebase = gpu
            .prepare_origin(&camera, &scene, None, &[], index as f64 * 1000.)
            .unwrap();
        assert_eq!(gpu.anchor(), origin);
        if index == 3 {
            assert!(
                rebase.moved,
                "Distant placement must exercise an actual origin change"
            );
        }
        let matrix = gpu.camera_matrix(&camera);
        let midpoint = model.transform_point(&Point::new(-0.375, -0.375, -0.5));
        let local = std::array::from_fn(|i| midpoint[i] - origin[i]);
        let old = if index == 3 {
            Some([-0.375, -0.375, -0.5])
        } else {
            previous
        };
        for (mode, part) in [(Mode::Object, Part::Object), (Mode::Edge, Part::Curve)] {
            assert_eq!(
                pick(&mut gpu, &scene, matrix, local, mode),
                Some(Target { owner, part }),
                "Fresh pick at placed midpoint, pose {index}"
            );
            if let Some(old) = old {
                assert_eq!(
                    pick(&mut gpu, &scene, matrix, old, mode),
                    None,
                    "Old pose must miss a fresh GPU query, pose {index}"
                );
            }
        }
        let image = pixels_at(&gpu, matrix);
        let green = pixel(&image, matrix, local);
        assert!(
            green[1] > 220 && green[0] < 30 && green[2] < 30,
            "Actual placed line pixels, pose {index}: {green:?}"
        );
        if let Some(old) = old {
            assert_eq!(
                pixel(&image, matrix, old),
                [255; 4],
                "Old pose has no rendered line, pose {index}"
            );
        }
        if index == 2 {
            mirror_pixels = Some(image.clone());
        }
        if index == 3 {
            assert_eq!(
                Some(image),
                mirror_pixels,
                "Far origin retains the complete mirrored line image"
            );
        }
        assert_eq!(scene.lines()[0].id, owner);
        assert_eq!(scene.row_guid(owner), Some(original.guid.as_str()));
        assert!(Rc::ptr_eq(&scene.lines()[0].prepared.source, &source));
        assert_eq!(source.to_proto(), original);
        assert_eq!(
            archive(&scene),
            placed_archive,
            "GPU queries and rendering do not mutate the placed archive"
        );
        previous = Some(local);
    }
    scene.place_line(owner, Xform::identity()).unwrap();
    assert_eq!(
        archive(&scene),
        initial_archive,
        "Restoring placement restores the complete original archive"
    );
}
