use session_rust::element::ElementFeature;
use session_rust::{Element, Mesh, Point, Polyline};

/// A contact lying on its face shows exactly the red of the same contact over a lower face: the depth layer wins at every angle without moving it.
#[test]
#[ignore = "requires a native GPU adapter"]
fn contacts_on_their_face_draw_in_front_of_it() {
    use crate::app::scene::{FileDoc, Scene};
    use crate::camera::Camera;
    use crate::engine::gpu::{FrameInput, Gpu};
    use session_rust::{Session, Xform};
    use std::rc::Rc;

    let red = |gpu: &mut Gpu, top: f64, orbit: (f32, f32), perspective: bool| -> (usize, f64) {
        let mut element = Element::new("plate");
        element.set_geometry(Mesh::create_box(10.0, 10.0, top * 2.0));
        let square = Polyline::new(vec![
            Point::new(-3.0, -3.0, 5.0),
            Point::new(3.0, -3.0, 5.0),
            Point::new(3.0, 3.0, 5.0),
            Point::new(-3.0, 3.0, 5.0),
            Point::new(-3.0, -3.0, 5.0),
        ]);
        element.add_feature(ElementFeature::new("contact", 0, vec![square], "face"));
        let mut source = Session::new("contact on its face");
        source.add_element(element, None);
        gpu.reset();
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "plate".into(),
            session: Rc::new(source),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        scene.upload_to(gpu);
        gpu.set_object_color(0, false, Some([0, 0, 255]));
        gpu.set_object_color(0, true, Some([0, 255, 0]));
        let mut camera = Camera::new();
        camera.perspective = perspective;
        camera.fit(
            &session_rust::AABB::from_points(
                &[Point::new(-5.0, -5.0, -5.0), Point::new(5.0, 5.0, 5.0)],
                0.0,
            ),
            1.0,
        );
        camera.orbit(orbit.0, orbit.1);
        let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 0.0);
        let input = FrameInput {
            view_proj: camera.view_proj_anchored(1.0, &rebase.anchor),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        };
        let pixels = gpu.render_offscreen(&input);
        let red = pixels
            .chunks_exact(4)
            .filter(|p| p[0] > 180 && p[1] < 140 && p[2] < 140)
            .count();
        if red > 100 && camera.position[2] / camera.unit.to_meters() > 5.0 {
            let black = pixels
                .chunks_exact(4)
                .filter(|p| p[..3].iter().all(|c| *c < 180))
                .count();
            assert!(
                black > 20,
                "Contact boundaries stay black under a colored layer: {black}; top={top}, orbit={orbit:?}, perspective={perspective}"
            );
        }
        (red, camera.position[2] / camera.unit.to_meters())
    };

    let mut seen = 0;
    for (size, samples, perspective) in [
        (256, 1, true),
        (640, 4, true),
        (256, 1, false),
        (640, 4, false),
    ] {
        let mut gpu = pollster::block_on(Gpu::new_headless(size, size)).unwrap();
        gpu.view.show_grid = false;
        gpu.view.markers = false;
        gpu.view.msaa_forced = Some(samples);
        gpu.resize(size, size);
        for yaw in (0..360).step_by(30) {
            for pitch in [-80.0, -50.0, -20.0, 0.0, 20.0, 50.0] {
                let orbit = (yaw as f32, pitch);
                let (clear, eye) = red(&mut gpu, 4.0, orbit, perspective);
                let (on_face, _) = red(&mut gpu, 5.0, orbit, perspective);
                if eye <= 5.0 {
                    continue; // from under its plane the face hides the contact, as it should
                }
                seen += 1;
                assert!(
                    on_face * 100 >= clear * 99,
                    "{size} px, perspective {perspective}, orbit {orbit:?}: {on_face} red pixels on the face, {clear} over a lower one"
                );
            }
        }
    }
    assert!(seen > 80, "only {seen} views saw the top face");
}
