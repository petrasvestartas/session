use super::*;
use crate::{
    cap_gpu_tests::{matrix, pixels, renderer, wait},
    ink_ranges::Visibility,
    pick_sources::{Mode, Part, Target},
};
use session_rust::{Mesh, OBB, Point, Vector, Xform};
use std::rc::Rc;
fn query(r: &mut Renderer, scene: &Scene, mode: Mode) -> Option<Target> {
    r.set_pick_mode(mode);
    r.request_pick([0., 0.2], matrix()).unwrap();
    wait(r);
    r.take_pick(matrix())
        .unwrap()
        .unwrap()
        .and_then(|v| r.resolve_pick(scene, v))
}
fn original_documents(scene: &Scene) -> Vec<Vec<u8>> {
    crate::archive::Bundle::decode(&crate::scene_archive::save(scene).unwrap())
        .unwrap()
        .documents
}
#[test]
fn actual_original_frames_keep_object_only_queries_and_reveal_the_real_edge_behind() {
    for kind in ["plane", "obb"] {
        let mut scene = Scene::empty();
        let mut mesh = Mesh::from_vertices_and_faces(
            vec![
                Point::new(-0.6, 0.2, -0.8),
                Point::new(0.6, 0.2, -0.8),
                Point::new(0., -0.6, -0.8),
            ],
            vec![vec![0, 1, 2]],
        );
        mesh.set_linecolors(vec![], vec![6.]);
        let solid = scene
            .insert(crate::prepared::PreparedMesh::new(mesh).unwrap())
            .unwrap();
        let frame = if kind == "plane" {
            let id =
                crate::plane_original::insert(&mut scene, Rc::new(session_rust::Plane::xy_plane()))
                    .unwrap();
            scene
                .place(
                    id,
                    &Xform::translation(0., 0., -0.3) * &Xform::scale_xyz(0.2, 0.2, 0.2),
                )
                .unwrap();
            id
        } else {
            crate::bounds_original::insert(
                &mut scene,
                Rc::new(OBB::new(
                    Point::new(0., 0., -0.35),
                    Vector::x_axis(),
                    Vector::y_axis(),
                    Vector::z_axis(),
                    Vector::new(0.2, 0.2, 0.05),
                )),
            )
            .unwrap()
        };
        let original = original_documents(&scene);
        let prepared = scene
            .objects()
            .iter()
            .map(|r| r.mesh.clone())
            .collect::<Vec<_>>();
        let mut r = renderer(&scene);
        r.set_opacity(1.);
        r.set_lighting(false, false);
        assert!(
            !r.sources.edges.iter().any(|t| t.owner == frame),
            "An original frame must not invent proxy MeshEdge subparts"
        );
        for samples in [1, 4] {
            r.set_samples(samples);
            r.set_ink_visibility(Visibility::default());
            let all = pixels(&r);
            let usage = (
                r.stats()[0],
                r.uploaded_bytes(),
                r.part_usage(),
                r.point_usage(),
                r.stroke_usage(),
            );
            assert_eq!(
                query(&mut r, &scene, Mode::Object),
                Some(Target {
                    owner: frame,
                    part: Part::Object
                })
            );
            assert_eq!(
                query(&mut r, &scene, Mode::Edge),
                Some(Target {
                    owner: solid,
                    part: Part::MeshEdge(0, 1)
                }),
                "The real source edge behind an object-only frame must remain queryable"
            );
            r.set_ink_visibility(Visibility {
                edges: false,
                ..Default::default()
            });
            pixels(&r);
            assert_eq!(query(&mut r, &scene, Mode::Edge), None);
            assert_eq!(
                query(&mut r, &scene, Mode::Object),
                Some(Target {
                    owner: frame,
                    part: Part::Object
                })
            );
            r.set_ink_visibility(Visibility::default());
            r.set_pick_mode(Mode::Object);
            r.request_pick([0., 0.2], matrix()).unwrap();
            r.set_ink_visibility(Visibility {
                lines: false,
                ..Default::default()
            });
            wait(&r);
            assert!(
                r.take_pick(matrix()).is_none(),
                "A visibility change cancels the pending old frame hit"
            );
            let hidden = pixels(&r);
            assert!(hidden != all);
            assert_eq!(
                query(&mut r, &scene, Mode::Object),
                Some(Target {
                    owner: solid,
                    part: Part::Object
                })
            );
            assert_eq!(
                query(&mut r, &scene, Mode::Edge),
                Some(Target {
                    owner: solid,
                    part: Part::MeshEdge(0, 1)
                })
            );
            r.set_ink_visibility(Visibility::default());
            assert!(pixels(&r) == all);
            assert_eq!(
                (
                    r.stats()[0],
                    r.uploaded_bytes(),
                    r.part_usage(),
                    r.point_usage(),
                    r.stroke_usage()
                ),
                usage,
                "Family changes retain original geometry, IDs, buffers and source uploads"
            );
            assert_eq!(original_documents(&scene), original);
            for (before, row) in prepared.iter().zip(scene.objects()) {
                assert!(Rc::ptr_eq(before, &row.mesh));
            }
        }
    }
}

fn probe(r: &mut Renderer, scene: &Scene, m: [f32; 16], p: [f64; 3], mode: Mode) -> Option<Target> {
    r.set_pick_mode(mode);
    r.request_pick(super::contact_feature_tests::project(m, p), m)
        .unwrap();
    wait(r);
    r.take_pick(m)
        .unwrap()
        .unwrap()
        .and_then(|v| r.resolve_pick(scene, v))
}
fn visibility_picture(name: &str, image: &[u8]) {
    crate::sample_test_proof::write(
        std::path::Path::new("visibility-pixels").join(format!("{name}.rgba")),
        image,
    )
    .unwrap();
}
#[test]
fn actual_mixed_families_keep_shared_sources_highlights_queries_and_active_controls() {
    let mut scene = Scene::empty();
    let mut mesh = Mesh::from_vertices_and_faces(
        vec![
            Point::new(-0.9, -0.75, -0.5),
            Point::new(-0.3, -0.75, -0.5),
            Point::new(-0.6, -0.2, -0.5),
        ],
        vec![vec![0, 1, 2]],
    );
    mesh.set_linecolors(vec![], vec![6.]);
    mesh.set_pointcolors(vec![
        session_rust::Color::new(1., 0.2, 0.1, 1.),
        session_rust::Color::new(0.1, 0.8, 0.2, 1.),
        session_rust::Color::new(0.2, 0.3, 1., 1.),
    ]);
    let left = scene
        .insert(crate::prepared::PreparedMesh::new(mesh).unwrap())
        .unwrap();
    let right = scene.copy_row(left, false).unwrap();
    scene.place(right, Xform::translation(1.2, 0., 0.)).unwrap();
    assert!(Rc::ptr_eq(
        &scene.objects()[0].mesh,
        &scene.objects()[1].mesh
    ));
    let mut line = session_rust::Line::new(-0.85, 0.65, -0.5, -0.45, 0.65, -0.5);
    line.width = 6.;
    let line = scene
        .insert_line(crate::stroke::PreparedLine::new(Rc::new(line)).unwrap())
        .unwrap();
    let mut point = Point::new(0.65, 0.65, -0.5);
    point.width = 14.;
    point.pointcolor = session_rust::Color::new(0.1, 0.25, 0.9, 1.);
    let point = scene
        .insert_point(crate::marker::PreparedPoint::new(Rc::new(point)).unwrap())
        .unwrap();
    let mut sheet = crate::sheet_example::source();
    sheet.coords = vec![-0.2, 0.65, -0.5, 0.2, 0.65, -0.5];
    sheet.colors = vec![0xff00_0000];
    sheet.widths = vec![0.025];
    sheet.segment_count = 1;
    sheet.entity_count = 1;
    sheet.source_ids = vec![0];
    sheet.meta = String::new();
    let sheet = scene
        .insert_sheet(Rc::new(
            crate::sheet_source::Prepared::new(Rc::new(sheet)).unwrap(),
        ))
        .unwrap();
    let mut source = session_rust::NurbsCurve::create(
        false,
        2,
        &[
            Point::new(-0.2, 0., -0.5),
            Point::new(0., 0.4, -0.5),
            Point::new(0.2, 0., -0.5),
        ],
    );
    source.width = 4.;
    let curve = scene
        .insert_path(crate::chain::PreparedChain::curve(Rc::new(source)).unwrap())
        .unwrap();
    scene.owner_controls(curve).unwrap();
    let original = original_documents(&scene);
    let mut r = renderer(&scene);
    r.set_opacity(1.);
    r.set_lighting(false, false);
    r.set_highlight(
        &scene,
        Some(Target {
            owner: line,
            part: Part::Curve,
        }),
    )
    .unwrap();
    for projection in [
        crate::camera::Projection::Orthographic,
        crate::camera::Projection::Perspective,
    ] {
        let mut camera = crate::camera::Camera::default();
        camera.set_view(crate::camera::View::Top);
        camera.projection = projection;
        camera.target = [0., 0., -0.5];
        camera.distance = 3.;
        camera.aspect = 4. / 3.;
        r.cloud_camera(&camera);
        let m = camera.uniform();
        for samples in [1, 4] {
            r.set_samples(samples);
            r.set_ink_visibility(Visibility::default());
            let all = crate::cap_gpu_tests::pixels_at(&r, m);
            let usage = (
                r.stats()[0],
                r.uploaded_bytes(),
                r.part_usage(),
                r.point_usage(),
                r.stroke_usage(),
                r.path_usage(),
                r.sheet_usage(),
            );
            let geometry = r.geometry_owners().cloned().collect::<Vec<_>>();
            assert_eq!(
                probe(&mut r, &scene, m, [-0.65, 0.65, -0.5], Mode::Object)
                    .unwrap()
                    .owner,
                line
            );
            assert_eq!(
                probe(&mut r, &scene, m, [0., 0.65, -0.5], Mode::Object)
                    .unwrap()
                    .owner,
                sheet
            );
            assert_eq!(
                probe(&mut r, &scene, m, [0.65, 0.65, -0.5], Mode::Object)
                    .unwrap()
                    .owner,
                point
            );
            for (owner, x) in [(left, -0.6), (right, 0.6)] {
                assert_eq!(
                    probe(&mut r, &scene, m, [x, -0.75, -0.5], Mode::Edge),
                    Some(Target {
                        owner,
                        part: Part::MeshEdge(0, 1)
                    })
                );
            }
            if projection == crate::camera::Projection::Orthographic {
                visibility_picture(if samples == 1 { "all-one" } else { "all-four" }, &all);
            }
            for (name, visible) in [
                (
                    "lines-off",
                    Visibility {
                        lines: false,
                        ..Default::default()
                    },
                ),
                (
                    "edges-off",
                    Visibility {
                        edges: false,
                        ..Default::default()
                    },
                ),
                (
                    "points-off",
                    Visibility {
                        points: false,
                        ..Default::default()
                    },
                ),
                (
                    "vertices-off",
                    Visibility {
                        vertices: false,
                        ..Default::default()
                    },
                ),
                (
                    "ink-off-controls",
                    Visibility {
                        edges: false,
                        lines: false,
                        points: false,
                        vertices: false,
                    },
                ),
            ] {
                r.set_ink_visibility(visible);
                let image = crate::cap_gpu_tests::pixels_at(&r, m);
                assert!(
                    image != all,
                    "The actual {name} frame must change its intended visible family"
                );
                assert_eq!(
                    probe(&mut r, &scene, m, [-0.65, 0.65, -0.5], Mode::Object).map(|t| t.owner),
                    visible.lines.then_some(line)
                );
                assert_eq!(
                    probe(&mut r, &scene, m, [0., 0.65, -0.5], Mode::Object).map(|t| t.owner),
                    visible.lines.then_some(sheet)
                );
                assert_eq!(
                    probe(&mut r, &scene, m, [0.65, 0.65, -0.5], Mode::Object).map(|t| t.owner),
                    visible.points.then_some(point)
                );
                for (owner, x) in [(left, -0.6), (right, 0.6)] {
                    assert_eq!(
                        probe(&mut r, &scene, m, [x, -0.75, -0.5], Mode::Edge),
                        visible.edges.then_some(Target {
                            owner,
                            part: Part::MeshEdge(0, 1)
                        })
                    );
                }
                assert_eq!(
                    probe(&mut r, &scene, m, [-0.2, 0., -0.5], Mode::Control),
                    Some(Target {
                        owner: curve,
                        part: Part::Control(0)
                    }),
                    "Active original controls remain independent of ordinary family flags"
                );
                assert_eq!(
                    (
                        r.stats()[0],
                        r.uploaded_bytes(),
                        r.part_usage(),
                        r.point_usage(),
                        r.stroke_usage(),
                        r.path_usage(),
                        r.sheet_usage()
                    ),
                    usage
                );
                if projection == crate::camera::Projection::Orthographic && samples == 4 {
                    visibility_picture(name, &image);
                }
                r.set_ink_visibility(Visibility::default());
                assert!(
                    crate::cap_gpu_tests::pixels_at(&r, m) == all,
                    "Exact {name} return without source/ID repacking"
                );
            }
            for owner in [left, right] {
                r.set_highlight(&scene, None).unwrap();
                let base = crate::cap_gpu_tests::pixels_at(&r, m);
                r.set_highlight(
                    &scene,
                    Some(Target {
                        owner,
                        part: Part::MeshEdge(0, 1),
                    }),
                )
                .unwrap();
                let marked = crate::cap_gpu_tests::pixels_at(&r, m);
                assert!(
                    marked != base,
                    "The actual selected mesh edge must be visibly highlighted"
                );
                r.set_ink_visibility(Visibility {
                    edges: false,
                    ..Default::default()
                });
                let hidden = crate::cap_gpu_tests::pixels_at(&r, m);
                r.set_highlight(&scene, None).unwrap();
                assert!(
                    crate::cap_gpu_tests::pixels_at(&r, m) == hidden,
                    "Hide Edges also hides the retained selected-edge highlight"
                );
                r.set_ink_visibility(Visibility::default());
            }
            r.set_highlight(
                &scene,
                Some(Target {
                    owner: line,
                    part: Part::Curve,
                }),
            )
            .unwrap();
            assert!(crate::cap_gpu_tests::pixels_at(&r, m) == all);
            assert_eq!(original_documents(&scene), original);
            for (before, current) in geometry.iter().zip(r.geometry_owners()) {
                assert!(Rc::ptr_eq(before, current));
            }
        }
    }
}
