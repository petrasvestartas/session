use crate::{cap_gpu_tests, cloud::Prepared, pick_sources::{Mode, Part}, scene::Scene};
use prost::Message;
use session_rust::{PointCloud, Xform};
use std::rc::Rc;

fn original() -> Rc<PointCloud> {
    let mut proto = PointCloud::from_coords(
        vec![-0.25, -0.25, 0.125, 0.25, -0.25, 0.125, -0.25, 0.25, 0.125, 0.25, 0.25, 0.125],
        vec![255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 0, 255, 255],
        vec![],
    ).to_proto();
    proto.guid = "c10d0000-0000-4000-8000-000000000001".into();
    proto.point_ids = vec![u32::MAX, 16_777_217, 77, 101];
    proto.point_size = 1.0;
    Rc::new(PointCloud::from_proto(proto))
}

fn archive(scene: &Scene, original: &session_rust::proto::PointCloud, model: &Xform) {
    let bytes = crate::scene_archive::save(scene).unwrap();
    let bundle = crate::archive::Bundle::decode(&bytes).unwrap();
    assert_eq!(bundle.documents.len(), 1);
    let document = session_rust::proto::Session::decode(bundle.documents[0].as_slice()).unwrap();
    let saved = &document.objects.as_ref().unwrap().pointclouds;
    assert_eq!(saved.len(), 1);
    assert_eq!(saved[0].guid, original.guid);
    assert_eq!(saved[0].coords, original.coords);
    assert_eq!(saved[0].colors, original.colors);
    assert_eq!(saved[0].normals, original.normals);
    assert_eq!(saved[0].point_ids, original.point_ids);
    assert_eq!(saved[0].point_size, original.point_size);
    let entry = document.xforms.iter().find(|entry| entry.guid == original.guid).unwrap();
    assert_eq!(entry.xform.as_ref().unwrap().matrix, model.m);
    let mut reopened = Scene::empty();
    crate::scene_document::open(&mut reopened, &bytes).unwrap();
    assert_eq!(reopened.clouds().len(), 1);
    assert_eq!(reopened.clouds()[0].model.m, model.m);
    assert_eq!(reopened.clouds()[0].prepared.source.to_proto(), saved[0]);
}

#[test]
fn actual_placed_clouds_keep_original_colours_cached_ids_archives_and_far_rebased_pixels() {
    let source = original();
    let before = source.to_proto();
    let mut scene = Scene::empty();
    let owner = scene.insert_cloud(Prepared::new(source.clone()).unwrap()).unwrap();
    let mut renderer = cap_gpu_tests::renderer(&scene);
    let matrix = cap_gpu_tests::reversed(Xform::identity().to_f32());
    let mut camera = crate::camera::Camera::default();
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 1.0 / std::f64::consts::FRAC_PI_6.tan();
    let mut clock = 0.0;
    let mut evidence = Vec::new();
    for (name, scale) in [("mirror", [-1.5, 0.5, 2.0]), ("nonuniform", [0.5, 1.5, 0.5])] {
        let mut local = Xform::scale_xyz(scale[0], scale[1], scale[2]);
        local.m[12..15].copy_from_slice(&[0.125, -0.125, 0.125]);
        for density in [1.0, 2.0] {
            renderer.set_density(density);
            for samples in [1, 4] {
                renderer.set_samples(samples);
                let mut baseline = None;
                let uploads = renderer.cloud_usage()[2];
                for anchor in [[0.0; 3], [1e12, -1e12, 1e12], [0.0; 3]] {
                    let mut model = local.clone();
                    for axis in 0..3 { model.m[12 + axis] += anchor[axis]; }
                    scene.place_cloud(owner, model.clone()).unwrap();
                    scene.cloud_controls(None);
                    renderer.set_scene(&scene, None);
                    camera.target = anchor;
                    clock += 300.0;
                    renderer.prepare_origin(&camera, &scene, None, &[], clock).unwrap();
                    assert_eq!(renderer.anchor(), anchor);
                    let pixels = cap_gpu_tests::pixels_at(&renderer, matrix);
                    if let Some(expected) = &baseline { assert_eq!(&pixels, expected, "Exact local/far/return pixels: {name}"); }
                    else { baseline = Some(pixels.clone()); }
                    for mode in [Mode::Object, Mode::Control] {
                        scene.cloud_controls(if mode == Mode::Control { Some(owner) } else { None });
                        renderer.set_scene(&scene, None);
                        renderer.set_pick_mode(mode);
                        for index in [0, 1] {
                            let p = source.get_point(index);
                            let at = local.transform_point(&p);
                            let x = ((at[0] + 1.0) * 160.0) as usize;
                            let y = ((1.0 - at[1]) * 120.0) as usize;
                            assert_eq!(cap_gpu_tests::at(&pixels, x, y), &before.colors[index * 4..index * 4 + 4].iter().map(|v| *v as u8).collect::<Vec<_>>());
                            renderer.request_pick([at[0] as f32, at[1] as f32], matrix).unwrap();
                            cap_gpu_tests::wait(&renderer);
                            let hit = renderer.take_pick(matrix).unwrap().unwrap().unwrap();
                            let target = renderer.resolve_pick(&scene, hit).unwrap();
                            assert_eq!(target.owner, owner);
                            if mode == Mode::Control {
                                assert_eq!(target.part, Part::CloudPoint(source.point_id(index)));
                                let precise = crate::cloud_pick::position(&scene, owner, source.point_id(index)).unwrap();
                                let expected = model.transform_point(&p);
                                assert_eq!([precise[0], precise[1], precise[2]], [expected[0], expected[1], expected[2]]);
                            } else { assert_eq!(target.part, Part::Object); }
                        }
                    }
                    assert_eq!(renderer.cloud_usage()[2], uploads, "Placement, rebase and fresh queries retain source columns");
                    assert!(Rc::ptr_eq(&scene.clouds()[0].prepared.source, &source));
                    assert_eq!(scene.row_guid(owner), Some(before.guid.as_str()));
                    archive(&scene, &before, &model);
                    evidence.push(serde_json::json!({"placement":name,"density":density,"samples":samples,"anchor":anchor,"model":model.m,"guid":before.guid,"source_uploads":uploads}));
                }
            }
        }
    }
    assert_eq!(source.to_proto(), before);
    crate::sample_test_proof::write("cloud-placement-source-and-cache.json", serde_json::to_vec_pretty(&evidence).unwrap()).unwrap();
}

#[test]
fn real_large_cloud_resident_prefixes_obey_the_exact_two_million_boundary_and_bounds() {
    let source = Rc::new(crate::cloud::large_example());
    let prepared = Prepared::new(source.clone()).unwrap();
    assert_eq!(source.len(), 2_000_064);
    assert_eq!(prepared.bounds.min, [-1.0, -1.0, 0.2]);
    assert_eq!(prepared.bounds.max, [1.0, 1.0, 0.2]);
    let mut cloud = prepared.descriptor();
    let mut walk = crate::cloud_lod::LodWalk::default();
    let mut view = crate::cloud_lod::Projection {eye:[0.0,0.0,3.0], ortho_h:100.0, height_px:240, lod_px:2.0, nodes:&prepared.nodes};
    for resident in [1_999_999, 2_000_000, 2_000_001] {
        cloud.resident = resident;
        for ortho in [100.0, 0.7, 0.0] {
            view.ortho_h = ortho;
            walk.select(&view, &cloud, &Xform::identity().to_f32());
            let expected = if resident < 2_000_000 || ortho < 1.0 { resident } else { 64 };
            assert_eq!(walk.ranges.iter().map(|range| range.count).sum::<u32>(), expected);
            assert!(walk.ranges.iter().all(|range| range.first + range.count <= resident));
            assert!(walk.ranges.iter().all(|range| range.tile == (resident >= 2_000_000)));
            if expected == resident {
                let mut intervals = walk.ranges.iter().map(|r| (r.first, r.first+r.count)).collect::<Vec<_>>();
                intervals.sort_unstable();
                assert_eq!(intervals.first().unwrap().0, 0);
                assert_eq!(intervals.last().unwrap().1, resident);
                assert!(intervals.windows(2).all(|pair| pair[0].1 == pair[1].0), "Every resident row occurs exactly once");
            }
        }
    }
    cloud.resident = 2_000_001;
    view.ortho_h = 100.0;
    view.lod_px = 0.0;
    walk.select(&view, &cloud, &Xform::identity().to_f32());
    assert_eq!((walk.ranges[0].first, walk.ranges[0].count, walk.ranges[0].tile), (0, 2_000_001, false));
    let mut scene = Scene::empty();
    let owner = scene.insert_cloud(prepared).unwrap();
    let mut model = Xform::scale_xyz(-2.0, 0.5, 4.0);
    model.m[12..15].copy_from_slice(&[1e12, -1e12, 1e12]);
    scene.place_cloud(owner, model).unwrap();
    let bounds = scene.clouds()[0].bounds();
    assert_eq!(bounds.min, [1e12-2.0, -1e12-0.5, 1e12+0.8]);
    assert_eq!(bounds.max, [1e12+2.0, -1e12+0.5, 1e12+0.8]);
    assert_eq!(source.point_id(2_000_000), 2_000_000);
    assert_eq!(source.len(), 2_000_064);
}
