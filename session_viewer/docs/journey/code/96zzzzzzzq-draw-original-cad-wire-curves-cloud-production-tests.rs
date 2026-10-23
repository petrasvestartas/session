use crate::{cap_gpu_tests, cloud::Prepared, pick_sources::{Mode, Part}, scene::Scene};
use session_rust::{PointCloud, Xform};
use std::rc::Rc;

#[test]
fn extracted_production_cloud_modules_validate_with_the_cached_identity_adapter() {
    use wgpu::naga;
    for source in [
        crate::production_cloud::point_source(),
        crate::production_cloud::RESOLVE_SOURCE.to_owned(),
    ] {
        let module = naga::front::wgsl::parse_str(&source)
            .unwrap_or_else(|error| panic!("{}", error.emit_to_string(&source)));
        naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::all())
            .validate(&module).unwrap();
        assert!(module.entry_points.iter().any(|entry| entry.stage == naga::ShaderStage::Vertex));
        assert!(module.entry_points.iter().any(|entry| entry.stage == naga::ShaderStage::Fragment));
    }
}

#[test]
fn streamed_cloud_page_addresses_never_highlight_an_unselected_original_point() {
    let prepared = crate::range_example::prepared();
    let ids: Vec<_> = prepared.pages().iter().map(|page| page.value.ids.clone()).collect();
    let mut scene = Scene::empty();
    let owner = scene.insert_stream(prepared.clone(), Xform::identity(), 12.0).unwrap();
    scene.cloud_controls(Some(owner));
    let mut renderer = cap_gpu_tests::renderer(&scene);
    renderer.set_pick_mode(Mode::Control);
    let matrix = cap_gpu_tests::reversed(Xform::identity().to_f32());
    for density in [1.0, 2.0] {
        renderer.set_density(density);
        for samples in [1, 4] {
            renderer.set_samples(samples);
            let pixels = cap_gpu_tests::pixels_at(&renderer, matrix);
            for x in [80, 240] {
                let blue = cap_gpu_tests::at(&pixels, x, 60);
                assert!(blue[2] > 240 && blue[0] < 10 && blue[1] < 10,
                    "Both original points on the second page stay blue: {blue:?}");
            }
            assert!(!pixels.chunks_exact(4).any(|p| p[0] > 240 && p[1] > 240 && p[2] < 20),
                "A page address must never become point selection");
            renderer.request_pick([0.5, 0.5], matrix).unwrap();
            cap_gpu_tests::wait(&renderer);
            let hit = renderer.take_pick(matrix).unwrap().unwrap().unwrap();
            let target = renderer.resolve_pick(&scene, hit).unwrap();
            assert_eq!(target.owner, owner);
            assert_eq!(target.part, Part::CloudPoint(16_777_217));
        }
    }
    assert_eq!(prepared.pages().iter().map(|page| page.value.ids.clone()).collect::<Vec<_>>(), ids);
}

#[test]
fn equal_depth_cloud_points_keep_the_first_original_colour_and_pick_identity() {
    let mut proto = PointCloud::from_coords(
        vec![0.0, 0.0, 0.3, 0.0, 0.0, 0.3],
        vec![255, 0, 0, 255, 0, 0, 255, 255],
        vec![],
    ).to_proto();
    proto.point_size = 12.0;
    proto.point_ids = vec![u32::MAX, 16_777_217];
    let source = Rc::new(PointCloud::from_proto(proto));
    let _ = source.guid();
    let original = source.to_proto();
    let mut scene = Scene::empty();
    let owner = scene.insert_cloud(Prepared::new(source.clone()).unwrap()).unwrap();
    scene.cloud_controls(Some(owner));
    let mut renderer = cap_gpu_tests::renderer(&scene);
    renderer.set_pick_mode(Mode::Control);
    let matrix = cap_gpu_tests::reversed(Xform::identity().to_f32());
    for samples in [1, 4] {
        renderer.set_samples(samples);
        let pixels = cap_gpu_tests::pixels_at(&renderer, matrix);
        let center = cap_gpu_tests::at(&pixels, 160, 120);
        assert!(center[0] > 240 && center[1] < 10 && center[2] < 10,
            "Production strict depth preserves the first original point: {center:?}");
        renderer.request_pick([0.0, 0.0], matrix).unwrap();
        cap_gpu_tests::wait(&renderer);
        let hit = renderer.take_pick(matrix).unwrap().unwrap().unwrap();
        let target = renderer.resolve_pick(&scene, hit).unwrap();
        assert_eq!(target.owner, owner);
        assert_eq!(target.part, Part::CloudPoint(u32::MAX));
    }
    assert_eq!(source.to_proto(), original);
}
