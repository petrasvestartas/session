use crate::{cad_display, scene::Scene};
use session_rust::{BRep, NurbsCurve, Point, Xform};
use std::rc::Rc;

fn add_wire(b: &mut BRep) -> usize {
    let points = [
        Point::new(-0.8, 0.7, -0.3),
        Point::new(0., 1., -0.3),
        Point::new(0.8, 0.7, -0.3),
    ];
    let curve = NurbsCurve::create(false, 2, &points);
    let a = b.add_vertex(&points[0], 0.);
    let z = b.add_vertex(&points[2], 0.);
    let c = b.add_curve_3d(&curve);
    let edge = b.add_edge(c as i32, a as i32, z as i32);
    b.add_wire(&[session_rust::brep::BRepRef::new(
        edge as i32,
        session_rust::brep::BRepOrientation::Forward,
    )]);
    edge
}
fn original(mixed: bool) -> BRep {
    let mut b = if mixed {
        BRep::create_block_with_hole(0.8, 0.6, 0.2, 0.12)
    } else {
        BRep::new()
    };
    b.set_guid(
        if mixed {
            "b6802f09-1be8-4cc6-a96c-bcae7869d902"
        } else {
            "b6802f09-1be8-4cc6-a96c-bcae7869d901"
        }
        .into(),
    );
    b.name = if mixed {
        "Original bore and detached wire"
    } else {
        "Original face-free wire"
    }
    .into();
    b.width = 4.;
    b.surfacecolor = session_rust::Color::new(0.35, 0.65, 0.85, 1.);
    add_wire(&mut b);
    b
}
fn documents(scene: &Scene) -> Vec<session_rust::proto::Session> {
    use prost::Message;
    crate::archive::Bundle::decode(&crate::scene_archive::save(scene).unwrap())
        .unwrap()
        .documents
        .into_iter()
        .map(|bytes| session_rust::proto::Session::decode(bytes.as_slice()).unwrap())
        .collect()
}
#[test]
fn original_wire_topology_keeps_source_and_rejects_invalid_references_without_fake_faces() {
    let wire = original(false);
    assert!(
        !wire.is_valid(),
        "The kernel's general BRep validator requires a face; do not change the kernel or insert one"
    );
    let (prepared, display) = cad_display::prepare(Rc::new(wire.clone())).unwrap();
    assert_eq!(prepared.source.to_proto(), wire.to_proto());
    assert_eq!(prepared.face_count, 0);
    assert_eq!(prepared.wires.len(), 1);
    assert!(display.display.indices().is_empty());
    assert!(!prepared.wires[0].points.is_empty());
    for change in 0..8 {
        let mut bad = wire.clone();
        match change {
            0 => bad.m_edges[0].start_vertex = -1,
            1 => bad.m_edges[0].end_vertex = 100,
            2 => bad.m_edges[0].curve_3d_index = 100,
            3 => bad.m_wires[0].edges[0].index = 100,
            4 => bad.m_curves_3d[0].m_cv[0] = f64::NAN,
            5 => bad.m_wires[0].edges.clear(),
            6 => bad.m_edges[0]
                .pcurves
                .push(session_rust::brep::BRepCurveOnSurface {
                    surface_index: 99,
                    curve_2d_index: 99,
                    curve_2d_index_2: -1,
                }),
            _ => bad
                .m_shells
                .push(session_rust::brep::BRepShell { faces: vec![] }),
        }
        assert!(cad_display::prepare(Rc::new(bad)).is_err());
    }
    let mut repeated = wire.clone();
    for _ in 0..128 {
        repeated.m_edges.push(wire.m_edges[0].clone());
    }
    let original_repeated = repeated.to_proto();
    let (deduplicated, display) = cad_display::prepare(Rc::new(repeated)).unwrap();
    assert_eq!(deduplicated.wires.len(), 1);
    assert_eq!(deduplicated.source.to_proto(), original_repeated);
    assert_eq!(
        display.display.vertices().len(),
        deduplicated.wires[0].points.len()
    );
    let points = (0..64)
        .map(|i| Point::new(i as f64 / 63., (i % 2) as f64, 0.))
        .collect::<Vec<_>>();
    let long = NurbsCurve::create(false, 1, &points);
    let mut bounded = BRep::new();
    let start = bounded.add_vertex(&points[0], 0.);
    let end = bounded.add_vertex(&points[63], 0.);
    for _ in 0..2000 {
        let curve = bounded.add_curve_3d(&long);
        let edge = bounded.add_edge(curve as i32, start as i32, end as i32);
        bounded.add_wire(&[session_rust::brep::BRepRef::new(
            edge as i32,
            session_rust::brep::BRepOrientation::Forward,
        )]);
    }
    assert!(matches!(
        cad_display::prepare(Rc::new(bounded)),
        Err("CAD wire sample budget exceeded")
    ));
    let mixed = original(true);
    assert!(mixed.is_valid());
    let (prepared, mesh) = cad_display::prepare(Rc::new(mixed.clone())).unwrap();
    assert!(!prepared.edge_ids.is_empty());
    assert!(prepared.wires.len() == 1);
    assert!(!mesh.display.indices().is_empty());
    assert_eq!(prepared.source.to_proto(), mixed.to_proto());
    let mut cache = crate::cad_shape_cache::Cache::default();
    cache.prepare(Rc::new(mixed.clone())).unwrap();
    let motion = &Xform::translation(25., -12., 9.) * &Xform::rotation_z(37., true);
    let shifted = Rc::new(mixed.transformed(&motion));
    let (replayed, _) = cache.prepare(shifted.clone()).unwrap();
    let (independent, _) = cad_display::prepare(shifted.clone()).unwrap();
    assert_eq!(
        cache.replays, 1,
        "A congruent mixed CAD source reuses the recorded walk"
    );
    assert_eq!(replayed.wires.len(), independent.wires.len());
    for (a, b) in replayed.wires.iter().zip(&independent.wires) {
        assert_eq!(a.edge, b.edge);
        assert_eq!(a.points.len(), b.points.len());
        for (a, b) in a.points.iter().zip(&b.points) {
            for i in 0..3 {
                assert!(
                    (a[i] - b[i]).abs() < 1e-9,
                    "Replayed original wire coordinates"
                );
            }
        }
    }
    assert_eq!(replayed.source.to_proto(), shifted.to_proto());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn actual_original_wire_and_mixed_cad_keep_family_queries_shared_sources_selection_and_archives() {
    use crate::{
        camera::{Camera, Projection, View},
        cap_gpu_tests::{pixels_at, pixels_format, renderer, wait},
        ink_ranges::Visibility,
        pick_sources::{Mode, Part, Target},
    };
    fn source_upload_bytes(r: &crate::renderer::Renderer) -> u64 {
        r.uploaded_bytes() - r.stats()[3] as u64 - r.instance_usage()[4]
    }
    fn project(m: [f32; 16], p: [f64; 3]) -> [f32; 2] {
        let c: [f64; 4] = std::array::from_fn(|i| {
            (0..3).map(|j| m[j * 4 + i] as f64 * p[j]).sum::<f64>() + m[12 + i] as f64
        });
        [(c[0] / c[3]) as f32, (c[1] / c[3]) as f32]
    }
    fn query(
        r: &mut crate::renderer::Renderer,
        scene: &Scene,
        m: [f32; 16],
        p: [f64; 3],
        mode: Mode,
    ) -> Option<Target> {
        r.set_pick_mode(mode);
        r.request_pick(project(m, p), m).unwrap();
        wait(r);
        r.take_pick(m)
            .unwrap()
            .unwrap()
            .and_then(|v| r.resolve_pick(scene, v))
    }
    for mixed in [false, true] {
        let source = Rc::new(original(mixed));
        let before = source.to_proto();
        let mut scene = Scene::empty();
        let place = &Xform::translation(2.2, 0., 0.) * &Xform::scale_xyz(-1., 0.8, 1.3);
        let input = session_rust::proto::Session {
            objects: Some(session_rust::proto::Objects {
                instances: vec![
                    session_rust::proto::InstanceRef {
                        guid: "b6802f09-1be8-4cc6-a96c-bcae7869d911".into(),
                        definition_guid: source.guid().into(),
                        ..Default::default()
                    },
                    session_rust::proto::InstanceRef {
                        guid: "b6802f09-1be8-4cc6-a96c-bcae7869d912".into(),
                        definition_guid: source.guid().into(),
                        xform: Some(place.to_proto()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            definitions: Some(session_rust::proto::Objects {
                breps: vec![before.clone()],
                ..Default::default()
            }),
            ..Default::default()
        };
        crate::scene_document::import(
            &mut scene,
            &prost::Message::encode_to_vec(&input),
            "",
            &Xform::identity(),
            0.,
        )
        .unwrap();
        assert_eq!(scene.objects().len(), 2);
        let owner = scene.objects()[0].id;
        let copy = scene.objects()[1].id;
        assert_eq!(scene.objects()[0].model.m, Xform::identity().m);
        assert_eq!(scene.objects()[1].model.m, place.m);
        assert_eq!(
            scene.objects()[0].cad.as_ref().unwrap().source.to_proto(),
            before
        );
        let saved = documents(&scene);
        let bytes = crate::scene_archive::save(&scene).unwrap();
        let kind = if mixed { "mixed" } else { "wire" };
        crate::sample_test_proof::write(format!("cad-wire/{kind}.json"), &serde_json::to_vec_pretty(&serde_json::json!({"bytes":bytes,"guid":source.guid(),"scope":"Actual original CAD archive from focused Native fixture; all later source-bound Browser checks still required."})).unwrap()).unwrap();
        crate::sample_test_proof::write(format!("cad-wire/{kind}.session"), &bytes).unwrap();
        let bundle = crate::archive::Bundle::decode(&bytes).unwrap();
        assert_eq!(bundle.documents.len(), 1);
        crate::sample_test_proof::write(format!("cad-wire/{kind}.pb"), &bundle.documents[0])
            .unwrap();
        let prepared = scene.objects()[0].cad.as_ref().unwrap().clone();
        assert!(Rc::ptr_eq(
            &prepared,
            scene.objects()[1].cad.as_ref().unwrap()
        ));
        let mut r = renderer(&scene);
        let format = wgpu::TextureFormat::Rgba8UnormSrgb;
        let mut matched =
            crate::renderer::Renderer::new(r.device.clone(), r.queue.clone(), format, &scene);
        matched.resize(crate::viewport::Viewport {
            width: 320,
            height: 240,
        });
        matched.set_opacity(1.);
        matched.set_lighting(false, false);
        r.set_opacity(1.);
        r.set_lighting(false, false);
        assert!(!crate::pick_sources::Sources::new(&scene)
            .unwrap()
            .edges
            .iter()
            .any(|t| matches!(t.part, Part::BRepEdge(e) if e == source.m_edges.len()-1)));
        for projection in [Projection::Orthographic, Projection::Perspective] {
            let mut camera = Camera::default();
            camera.set_view(View::Top);
            camera.target = [1.1, 0.4, 0.];
            camera.distance = 6.;
            camera.aspect = 4. / 3.;
            camera.projection = projection;
            r.cloud_camera(&camera);
            let m = camera.uniform();
            for samples in [1, 4] {
                r.set_samples(samples);
                r.set_ink_visibility(Visibility::default());
                let all = pixels_at(&r, m);
                let usage = (
                    [r.stats()[0], r.stats()[1]],
                    source_upload_bytes(&r),
                    r.part_usage(),
                    r.stroke_usage(),
                    r.path_usage(),
                );
                for (id, p) in [(owner, [0., 0.85, -0.3]), (copy, [2.2, 0.68, -0.39])] {
                    assert_eq!(
                        query(&mut r, &scene, m, p, Mode::Object),
                        Some(Target {
                            owner: id,
                            part: Part::Object
                        })
                    );
                    assert_eq!(
                        query(&mut r, &scene, m, p, Mode::Edge),
                        None,
                        "A fallback wire has no mapped BRepEdge query address"
                    );
                }
                r.set_ink_visibility(Visibility {
                    edges: false,
                    ..Default::default()
                });
                let no_edges = pixels_at(&r, m);
                assert_eq!(
                    query(&mut r, &scene, m, [0., 0.85, -0.3], Mode::Object)
                        .unwrap()
                        .owner,
                    owner
                );
                assert_eq!(
                    no_edges == all,
                    !mixed,
                    "Only mapped face borders follow Edges"
                );
                r.set_ink_visibility(Visibility {
                    lines: false,
                    ..Default::default()
                });
                let no_lines = pixels_at(&r, m);
                assert_ne!(no_lines, all);
                assert_eq!(
                    query(&mut r, &scene, m, [0., 0.85, -0.3], Mode::Object),
                    None
                );
                r.set_ink_visibility(Visibility::default());
                assert_eq!(pixels_at(&r, m), all);
                r.select_objects(&scene, &[owner, copy]).unwrap();
                let selected = pixels_at(&r, m);
                assert_ne!(selected, all);
                r.set_ink_visibility(Visibility {
                    lines: false,
                    ..Default::default()
                });
                let hidden_selected = pixels_at(&r, m);
                r.set_highlight(&scene, None).unwrap();
                r.select_objects(&scene, &[]).unwrap();
                let hidden_unselected = pixels_at(&r, m);
                if !mixed {
                    assert_eq!(
                        hidden_selected, hidden_unselected,
                        "Hide Lines also hides selected wire ink"
                    );
                }
                for p in [[0., 0.85, -0.3], [2.2, 0.68, -0.39]] {
                    let at = project(m, p);
                    let x = ((at[0] + 1.) * 160.).round() as usize;
                    let y = ((1. - at[1]) * 120.).round() as usize;
                    for j in y.saturating_sub(3)..(y + 4).min(240) {
                        for i in x.saturating_sub(3)..(x + 4).min(320) {
                            assert_eq!(
                                crate::cap_gpu_tests::at(&hidden_selected, i, j),
                                crate::cap_gpu_tests::at(&hidden_unselected, i, j),
                                "No selected fallback wire may survive Hide Lines over the independent face selection"
                            );
                        }
                    }
                }
                r.set_ink_visibility(Visibility::default());
                assert_eq!(pixels_at(&r, m), all);
                assert_eq!(
                    (
                        [r.stats()[0], r.stats()[1]],
                        source_upload_bytes(&r),
                        r.part_usage(),
                        r.stroke_usage(),
                        r.path_usage()
                    ),
                    usage
                );
                assert_eq!(documents(&scene), saved);
                assert_eq!(source.to_proto(), before);
                assert_eq!(prepared.source.to_proto(), before);
                assert!(Rc::ptr_eq(
                    &prepared,
                    scene.objects()[0].cad.as_ref().unwrap()
                ));
                if projection == Projection::Orthographic {
                    matched.cloud_camera(&camera);
                    matched.set_samples(samples);
                    let encoded = pixels_format(&matched, m, format);
                    crate::sample_test_proof::write(
                        format!("cad-wire/{kind}-matched-{samples}.rgba"),
                        &encoded,
                    )
                    .unwrap();
                    if samples == 4 {
                        crate::sample_test_proof::write(format!("cad-wire/{kind}-pose.json"), &serde_json::to_vec_pretty(&serde_json::json!({"matchedFormat":"Rgba8UnormSrgb","componentFormat":"Rgba8Unorm","projection":"Orthographic","viewport":[320,240],"target":camera.target,"distance":camera.distance,"matrix":m})).unwrap()).unwrap();
                    }
                    for (name, image) in [
                        ("all", &all),
                        ("no-edges", &no_edges),
                        ("no-lines", &no_lines),
                        ("selected", &selected),
                    ] {
                        crate::sample_test_proof::write(
                            format!(
                                "cad-wire/{}-{name}-{samples}.rgba",
                                if mixed { "mixed" } else { "wire" }
                            ),
                            image,
                        )
                        .unwrap();
                    }
                }
            }
        }
        let bounds = scene.bounds().unwrap();
        assert!(bounds.min[0] <= -0.8 && bounds.max[0] >= 2.99 && bounds.max[1] >= 0.849);
        assert_eq!(documents(&scene), saved);
    }
}
