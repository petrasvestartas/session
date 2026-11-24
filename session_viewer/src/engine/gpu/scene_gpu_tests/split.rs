use crate::app::clipping::{Mode, clip_plane, plane_from};
use crate::app::command::verbs::{curve, line, point, polyline};
use crate::app::modeling::Interval;
use crate::app::scene::{FileDoc, Scene, SheetInit, StreamedInit};
use crate::app::stream::{CloudFields, CloudLod, SheetFields};
use crate::app::walk::cloud::StreamRows;
use crate::app::walk::sheet::SheetRows;
use crate::camera::Camera;
use crate::engine::gpu::{FrameInput, Gpu};
use session_rust::{
    BRep, Color, Element, Geometry, Line, Mesh, NurbsCurve, Point, PointCloud, Polyline, Session,
    TreeNode, Vector, Xform,
};
use std::cell::RefCell;
use std::rc::Rc;

/// A document placed at `place`.
fn file(name: &str, session: Session, place: Xform) -> FileDoc {
    FileDoc {
        name: name.into(),
        session: Rc::new(session),
        place,
        point_px: 0.0,
        display_only: false,
    }
}

/// A point.
fn p(x: f64, y: f64, z: f64) -> Point {
    Point::new(x, y, z)
}

/// A curve through eight points of an arc: trimming it changes its sample count.
fn arc() -> NurbsCurve {
    let points: Vec<Point> = (0..8)
        .map(|i| {
            let angle = i as f64 * 0.4;
            p(10.0 * angle.cos(), 10.0 * angle.sin(), 0.0)
        })
        .collect();
    NurbsCurve::create(false, 3, &points)
}

/// A streamed sheet and a streamed cloud, read-only shells.
fn shells(scene: &mut Scene) {
    scene.stream_sheet(SheetInit {
        name: "plan".into(),
        url: "plan.pb".into(),
        meta_url: None,
        place: Xform::identity(),
        rows: SheetRows {
            positions: vec![
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 10.0, 0.0,
            ],
            colors: Vec::new(),
            widths: Vec::new(),
            ids: vec![7, 8],
        },
        fields: SheetFields {
            count: 2,
            ..Default::default()
        },
        resident: 2,
    });
    scene.stream_cloud(StreamedInit {
        name: "scan".into(),
        url: "scan.pb".into(),
        place: Xform::translation(0.0, 0.0, 5.0),
        rows: StreamRows {
            positions: vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
            colors: vec![u32::MAX; 2],
            normals: Vec::new(),
        },
        lod: CloudLod::default(),
        fields: CloudFields {
            end: 0,
            coords_at: 0,
            coords_len: 0,
            colors_at: 0,
            colors_len: 0,
            normals_at: 0,
            normals_len: 0,
            count: 2,
            ids_at: 0,
            ids_len: 0,
            revision: None,
        },
        resident: 2,
        point_px: 3.0,
        col_at: 0,
        ceiling: 2,
    });
}

/// Live rows of editable documents, by identity.
fn live(scene: &Scene) -> Vec<(u32, (usize, Rc<str>))> {
    (0..scene.row_count() as u32)
        .filter_map(|row| {
            let id = scene.identity_of(row)?;
            scene
                .docs
                .get(id.0)
                .is_some_and(|file| !file.display_only)
                .then_some((row, id))
        })
        .collect()
}

/// The first live row whose geometry passes `test`.
fn find(scene: &Scene, test: impl Fn(&Geometry) -> bool) -> Option<u32> {
    live(scene)
        .into_iter()
        .map(|(row, _)| row)
        .find(|&row| scene.geometry(row).is_some_and(&test))
}

/// A tiny deterministic random source.
struct Dice(u64);

impl Dice {
    /// A number below `n`.
    fn roll(&mut self, n: usize) -> usize {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.0 >> 33) as usize) % n.max(1)
    }
}

/// One random edit of the kinds the viewer makes.
fn edit(scene: &mut Scene, dice: &mut Dice) {
    let rows: Vec<u32> = live(scene).into_iter().map(|(row, _)| row).collect();
    let pick = |dice: &mut Dice| rows[dice.roll(rows.len())];
    let layer = |scene: &Scene, doc: usize| {
        let mut names: Vec<String> = scene.docs[doc]
            .session
            .tree
            .nodes()
            .iter()
            .skip(1)
            .map(|node| node.borrow().name.clone())
            .filter(|name| !scene.docs[doc].session.lookup.contains_key(name))
            .collect();
        names.sort();
        names
    };
    let x = dice.roll(100) as f64;

    match dice.roll(20) {
        0 => drop(scene.model(&point::SPEC, &[[x, 1.0, 2.0]])),
        1 => drop(scene.model(&line::SPEC, &[[x, 0.0, 0.0], [x, 5.0, 1.0]])),
        2 => drop(scene.model(
            &polyline::SPEC,
            &[[0.0, 0.0, 0.0], [x, 1.0, 0.0], [x, x, 0.0]],
        )),
        3 => drop(scene.model(
            &curve::SPEC,
            &[[0.0, 0.0, 0.0], [x, 3.0, 0.0], [x, x, 2.0], [0.0, x, 1.0]],
        )),
        4 if !rows.is_empty() => {
            scene.delete_row(pick(dice));
        }
        5 if !rows.is_empty() => {
            let picked = [pick(dice), pick(dice), pick(dice)];
            scene.delete_rows(&picked);
        }
        6 if !rows.is_empty() => {
            scene.selected = Some(pick(dice));
            let _ = scene.edit_interval(Interval::Trim(0.1, 0.6));
        }
        7 if !rows.is_empty() => {
            scene.selected = Some(pick(dice));
            let _ = scene.edit_interval(Interval::Extend(-0.5, 1.2));
        }
        8 if !rows.is_empty() => {
            let _ = scene.explode_rows(&[pick(dice)]);
        }
        9 if !rows.is_empty() => {
            let picked = [pick(dice), pick(dice)];
            scene.transform_rows(&picked, &Xform::translation(x, 1.0, -2.0), "move");
        }
        10 | 11 => {
            scene.undo();
        }
        12 => {
            scene.redo();
        }
        13 => {
            let doc = dice.roll(scene.docs.len());
            if let Some(at) = layer(scene, doc).first().cloned() {
                let _ = scene.new_layer(doc, &at, dice.roll(2) == 0);
            }
        }
        14 => {
            let doc = dice.roll(scene.docs.len());
            let names = layer(scene, doc);
            if !names.is_empty() {
                let name = names[dice.roll(names.len())].clone();
                let to = if dice.roll(4) == 0 {
                    "attributes".to_string()
                } else {
                    format!("{name} {x}")
                };
                let _ = scene.rename_layer(doc, &name, &to);
            }
        }
        15 => {
            let doc = dice.roll(scene.docs.len());
            let names = layer(scene, doc);
            if !names.is_empty() {
                let name = names[dice.roll(names.len())].clone();
                let _ = scene.delete_layer(doc, &name);
            }
        }
        16 => {
            let doc = dice.roll(scene.docs.len());
            let names = layer(scene, doc);
            if !names.is_empty() {
                let name = names[dice.roll(names.len())].clone();
                let _ = scene.duplicate_layer(doc, &name);
            }
        }
        17 | 18 if !rows.is_empty() => {
            let doc = dice.roll(scene.docs.len());
            let names = layer(scene, doc);
            if !names.is_empty() {
                let name = names[dice.roll(names.len())].clone();
                let picked = [pick(dice)];
                let _ = if dice.roll(3) == 0 {
                    scene.copy_object_layer(&picked, doc, &name)
                } else {
                    scene.change_object_layer(&picked, doc, &name)
                };
            }
        }
        19 if rows.len() > 1 => {
            let target = pick(dice);
            let cutter = pick(dice);
            let _ = scene.split_rows(target, None, &[cutter]);
        }
        _ => {}
    }
}

/// Colors and ids of one frame.
struct Shot {
    color: Vec<u8>,     // RGBA pixels
    ids: Vec<[u32; 2]>, // object and sub id per pixel
}

/// Draw one frame and its id frame.
fn shot(gpu: &mut Gpu, camera: &Camera) -> Shot {
    let anchor = gpu
        .rebase_anchor(&camera.origin(), camera.distance_world(), 0.0)
        .anchor;
    let input = FrameInput {
        view_proj: camera.view_proj_anchored(4.0 / 3.0, &anchor),
        clear: wgpu::Color::WHITE,
        now_ms: 0.0,
    };
    Shot {
        color: gpu.render_offscreen(&input),
        ids: gpu.render_ids_offscreen(&input),
    }
}

/// The ids of a frame as identities; a drawn id must be a live row.
fn named(scene: &Scene, ids: &[[u32; 2]]) -> Vec<Option<(usize, Rc<str>)>> {
    ids.iter()
        .map(|&[object, _]| {
            (object != 0).then(|| {
                scene
                    .identity_of(object - 1)
                    .unwrap_or_else(|| panic!("row {} is drawn but dead", object - 1))
            })
        })
        .collect()
}

/// A mesh, a BRep, a polyline, a point, a curve, a cloud and two overlapping elements.
fn solids() -> Session {
    let mut session = Session::new("solids");
    let at = |session: &mut Session, node: Rc<RefCell<TreeNode>>, x: f64, y: f64| {
        let guid = node.borrow().name.clone();
        session.set_xform(&guid, Xform::translation(x, y, 0.0));
    };
    let mesh = session
        .add_mesh(Mesh::create_box(10.0, 10.0, 10.0), None)
        .unwrap();
    at(&mut session, mesh, 0.0, 0.0);
    let brep = session
        .add_brep(BRep::create_box(10.0, 10.0, 10.0), None)
        .unwrap();
    at(&mut session, brep, 20.0, 0.0);
    session.add_polyline(
        Polyline::new(vec![
            p(35.0, -5.0, 0.0),
            p(45.0, 5.0, 0.0),
            p(50.0, -5.0, 0.0),
        ]),
        None,
    );
    let mut point = p(60.0, 0.0, 0.0);
    point.width = 12.0;
    session.add_point(point, None);
    let curve = session.add_nurbscurve(arc(), None).unwrap();
    at(&mut session, curve, 0.0, 20.0);
    let points: Vec<Point> = (0..400)
        .map(|i| p((i % 20) as f64, -20.0 - (i / 20) as f64, 0.0))
        .collect();
    let normals = vec![Vector::new(0.0, 0.0, 1.0); points.len()];
    let colors = vec![Color::red(); points.len()];
    session.add_pointcloud(PointCloud::new(points, normals, colors), None);

    for (x, y) in [(40.0, 30.0), (45.0, 33.0)] {
        let mut element = Element::new("glass");
        element.set_geometry(Mesh::create_box(10.0, 10.0, 10.0));
        let node = session.add_element(element, None);
        at(&mut session, node, x, y);
    }

    session
}

/// A headless GPU with the scene loaded and a camera fitted to it.
fn loaded(session: &Rc<Session>) -> (Gpu, Scene, Camera) {
    let mut gpu = pollster::block_on(Gpu::new_headless(400, 300)).expect("a native adapter");
    gpu.view.show_grid = false;
    gpu.view.opacity = 0.7;
    let mut scene = Scene::new();
    scene.add_file(FileDoc {
        name: "solids".into(),
        session: Rc::clone(session),
        place: Xform::identity(),
        point_px: 0.0,
        display_only: false,
    });
    scene.upload_to(&mut gpu);
    let mut camera = Camera::new();
    camera.fit(&gpu.bounds, 4.0 / 3.0);
    (gpu, scene, camera)
}

/// Sync and flush.
fn commit(scene: &mut Scene, gpu: &mut Gpu) {
    scene.sync();
    scene.upload_to(gpu);
}

/// A deleted object leaves no pixel and no id: the frame equals a fresh load without it.
#[test]
#[ignore = "requires a native GPU adapter"]
fn kill_leaves_no_pixels_and_no_ids() {
    let (mut gpu, mut scene, camera) = loaded(&Rc::new(solids()));
    let kinds: [fn(&Geometry) -> bool; 5] = [
        |g| matches!(g, Geometry::Mesh(_)),
        |g| matches!(g, Geometry::BRep(_)),
        |g| matches!(g, Geometry::Polyline(_)),
        |g| matches!(g, Geometry::Point(_)),
        |g| matches!(g, Geometry::PointCloud(_)),
    ];

    for kind in kinds {
        let row = find(&scene, kind).unwrap();
        assert!(scene.delete_row(row));
        commit(&mut scene, &mut gpu);
        let (mut fresh_gpu, fresh, _) = loaded(&scene.docs[0].session);

        for samples in [1, 4] {
            for gpu in [&mut gpu, &mut fresh_gpu] {
                gpu.view.msaa_forced = Some(samples);
                gpu.resize(400, 300);
            }

            let after = shot(&mut gpu, &camera);
            let want = shot(&mut fresh_gpu, &camera);
            let differ = after
                .color
                .chunks_exact(4)
                .zip(want.color.chunks_exact(4))
                .filter(|(a, b)| a != b)
                .count();
            assert_eq!(differ, 0, "{samples}x: pixels differ from a fresh load");
            assert_eq!(named(&scene, &after.ids), named(&fresh, &want.ids));
        }
    }
}

/// Delete, create, a count-changing trim and a cancelled drag, undone: the same pixels and ids.
#[test]
#[ignore = "requires a native GPU adapter"]
fn edit_then_undo_is_pixel_and_id_identical() {
    let (mut gpu, mut scene, camera) = loaded(&Rc::new(solids()));

    for samples in [1, 4] {
        gpu.view.msaa_forced = Some(samples);
        gpu.resize(400, 300);
        let before = shot(&mut gpu, &camera);
        let same = |gpu: &mut Gpu, label: &str| {
            let now = shot(gpu, &camera);
            assert!(now.color == before.color, "{samples}x {label}: pixels");
            assert!(now.ids == before.ids, "{samples}x {label}: ids");
        };

        let element = find(&scene, |g| matches!(g, Geometry::Element(_))).unwrap();
        assert!(scene.delete_row(element));
        commit(&mut scene, &mut gpu);
        assert!(scene.undo());
        commit(&mut scene, &mut gpu);
        same(&mut gpu, "delete, undo");

        scene.model(&point::SPEC, &[[5.0, 5.0, 5.0]]).unwrap();
        commit(&mut scene, &mut gpu);
        assert!(scene.undo());
        commit(&mut scene, &mut gpu);
        same(&mut gpu, "create, undo");

        let curve = find(&scene, |g| matches!(g, Geometry::NurbsCurve(_))).unwrap();
        scene.selected = Some(curve);
        scene.edit_interval(Interval::Extend(-0.5, 1.5)).unwrap();
        commit(&mut scene, &mut gpu);
        assert!(scene.undo());
        commit(&mut scene, &mut gpu);
        same(&mut gpu, "trim, undo");

        let source = scene.geometry(curve).unwrap().clone();
        let Geometry::NurbsCurve(original) = &source else {
            panic!()
        };
        let mut grown = (**original).clone();
        let (lo, hi) = grown.domain();
        assert!(grown.extend(lo, hi + (hi - lo)));
        scene.redraw(curve, &Geometry::NurbsCurve(Rc::new(grown)), true);
        scene.upload_to(&mut gpu);
        scene.redraw(curve, &source, false);
        scene.upload_to(&mut gpu);
        same(&mut gpu, "drag, cancel");
    }
}

/// Arrowheads follow set, move, copy and undo: each frame equals a fresh load of the document.
#[test]
#[ignore = "requires a native GPU adapter"]
fn arrowheads_follow_edits() {
    use session_rust::Arrowhead;

    let mut session = Session::new("heads");
    // not flat, so the document loads as models, not as a drawing sheet
    session.add_polyline(
        Polyline::new(vec![
            p(0.0, 0.0, 0.0),
            p(20.0, 10.0, 5.0),
            p(40.0, 0.0, 10.0),
        ]),
        None,
    );
    session.add_nurbscurve(arc(), None);
    let (mut gpu, mut scene, camera) = loaded(&Rc::new(session));
    let before = shot(&mut gpu, &camera);
    let fresh = |scene: &Scene, gpu: &mut Gpu, label: &str| {
        let (mut fresh_gpu, _, _) = loaded(&scene.docs[0].session);
        let want = shot(&mut fresh_gpu, &camera);
        assert!(
            shot(gpu, &camera).color == want.color,
            "{label}: pixels differ from a fresh load"
        );
    };
    let rows: Vec<u32> = (0..scene.object_count() as u32).collect();
    let edits = rows
        .iter()
        .map(|&row| {
            let geometry = match scene.geometry(row).unwrap() {
                Geometry::Polyline(pl) => {
                    let mut pl = (**pl).clone();
                    pl.arrowhead = Arrowhead::BOTH;
                    Geometry::Polyline(Rc::new(pl))
                }
                Geometry::NurbsCurve(c) => {
                    let mut c = (**c).clone();
                    c.arrowhead = Arrowhead::END;
                    Geometry::NurbsCurve(Rc::new(c))
                }
                _ => panic!("a curve"),
            };
            (row, geometry)
        })
        .collect();
    scene.replace_rows(edits, "arrowhead").unwrap();
    commit(&mut scene, &mut gpu);
    assert!(
        shot(&mut gpu, &camera).color != before.color,
        "the heads draw"
    );
    fresh(&scene, &mut gpu, "heads");

    scene
        .transform_rows(&rows, &Xform::translation(5.0, 5.0, 0.0), "move")
        .unwrap();
    commit(&mut scene, &mut gpu);
    fresh(&scene, &mut gpu, "move");

    scene
        .copy_rows(&rows, &Xform::translation(0.0, -30.0, 0.0))
        .unwrap();
    commit(&mut scene, &mut gpu);
    fresh(&scene, &mut gpu, "copy");

    // selected, every head turns yellow; hidden, nothing is drawn
    let dark = |rgba: &[u8]| {
        rgba.chunks_exact(4)
            .filter(|c| c[0] < 128 && c[1] < 128)
            .count()
    };
    let all: Vec<u32> = (0..64)
        .filter(|&row| scene.identity_of(row).is_some())
        .collect();
    assert_eq!(all.len(), 4, "two curves and their copies");
    assert!(dark(&shot(&mut gpu, &camera).color) > 100);

    for &row in &all {
        gpu.set_selected(row, true);
    }

    assert_eq!(
        dark(&shot(&mut gpu, &camera).color),
        0,
        "no black head while selected"
    );

    for &row in &all {
        gpu.set_selected(row, false);
        gpu.set_hidden(row, true);
    }

    assert_eq!(
        dark(&shot(&mut gpu, &camera).color),
        0,
        "hidden heads draw nothing"
    );

    for &row in &all {
        gpu.set_hidden(row, false);
    }

    // a clipping plane with every curve on its cut side leaves no head behind
    let plane = plane_from(
        Mode::Normal,
        &[[-1000.0, 0.0, 0.0], [-999.0, 0.0, 0.0]],
        2000.0,
    )
    .unwrap();
    gpu.set_clip_planes(&[clip_plane(&plane, &Xform::identity()).unwrap()]);
    assert_eq!(
        dark(&shot(&mut gpu, &camera).color),
        0,
        "clipped heads draw nothing"
    );
    gpu.set_clip_planes(&[]);

    for _ in 0..3 {
        assert!(scene.undo());
        commit(&mut scene, &mut gpu);
    }

    let back = shot(&mut gpu, &camera);
    assert!(back.color == before.color, "undo takes the heads off");
    assert!(back.ids == before.ids, "undo restores the ids");
}

/// A compaction draws what a fresh load draws, ids mapped by identity, and shrinks the lanes.
#[test]
#[ignore = "requires a native GPU adapter"]
fn compaction_is_exact_and_shrinks() {
    let (mut gpu, mut scene, camera) = loaded(&Rc::new(solids()));
    let mut dice = Dice(9);

    for _ in 0..60 {
        edit(&mut scene, &mut dice);
        commit(&mut scene, &mut gpu);
    }

    assert!(scene.dead_bytes() > 0);
    scene.rewalk_editable(&mut gpu);
    let (mut fresh_gpu, fresh, _) = loaded(&scene.docs[0].session);
    let mut extra = Vec::new();

    for file in &scene.docs[1..] {
        extra.push(FileDoc {
            name: file.name.clone(),
            session: Rc::clone(&file.session),
            place: file.place.clone(),
            point_px: file.point_px,
            display_only: false,
        });
    }

    let mut fresh = fresh;
    fresh.created_doc = scene.created_doc;

    for doc in extra {
        fresh.add_file(doc);
        fresh.upload_to(&mut fresh_gpu);
    }

    for samples in [1, 4] {
        for gpu in [&mut gpu, &mut fresh_gpu] {
            gpu.view.msaa_forced = Some(samples);
            gpu.resize(400, 300);
        }

        let after = shot(&mut gpu, &camera);
        let want = shot(&mut fresh_gpu, &camera);
        assert!(after.color == want.color, "{samples}x: pixels");
        assert_eq!(named(&scene, &after.ids), named(&fresh, &want.ids));
        let subs = |shot: &Shot| shot.ids.iter().map(|id| id[1]).collect::<Vec<_>>();
        assert_eq!(
            subs(&after),
            subs(&want),
            "{samples}x: lane rows as a fresh load"
        );
    }

    let lanes = |gpu: &Gpu| {
        gpu.arena.allocated_bytes() + gpu.segments.allocated_bytes() + gpu.glyphs.allocated_bytes()
    };
    assert_eq!(
        lanes(&gpu),
        lanes(&fresh_gpu),
        "lanes as small as a fresh load"
    );
}

/// Time the phases of one edit in a scene the size of `view_lines`: kernel, sync, flush.
#[test]
#[ignore = "requires a native GPU adapter"]
fn bench_edit_phases() {
    let mut gpu = pollster::block_on(Gpu::new_headless(400, 300)).expect("a native adapter");
    let mut scene = Scene::new();

    for doc in 0..9 {
        let mut session = Session::new(&format!("sheet {doc}"));

        for i in 0..80_000 {
            let x = (i % 400) as f64;
            let y = (i / 400) as f64 + doc as f64 * 300.0;
            session.add_line(Line::new(x, y, 0.0, x + 0.5, y + 0.5, 0.0), None);
        }

        scene.add_file(file(&format!("sheet {doc}"), session, Xform::identity()));
        scene.upload_to(&mut gpu);
    }

    let ms = |start: std::time::Instant| start.elapsed().as_secs_f64() * 1e3;
    let mut phases = Vec::new();

    for round in 0..5 {
        let t = std::time::Instant::now();
        let (doc, guid) = scene
            .model(&point::SPEC, &[[round as f64, 0.0, 0.0]])
            .unwrap();
        let kernel = ms(t);
        let t = std::time::Instant::now();
        scene.sync();
        let sync = ms(t);
        let t = std::time::Instant::now();
        scene.upload_to(&mut gpu);
        let flush = ms(t);
        let row = scene.row_of(doc, &guid).unwrap();
        let t = std::time::Instant::now();
        scene.delete_row(row);
        scene.sync();
        scene.upload_to(&mut gpu);
        let delete = ms(t);
        let t = std::time::Instant::now();
        scene.undo();
        scene.sync();
        scene.upload_to(&mut gpu);
        let undo = ms(t);
        phases.push(format!(
            "create {kernel:.2}+{sync:.2}+{flush:.2} ms, delete {delete:.2} ms, undo {undo:.2} ms"
        ));
    }

    eprintln!("{} objects:\n{}", scene.object_count(), phases.join("\n"));
}

/// Streamed sheet segments and cloud points survive edits and both compactions.
#[test]
#[ignore = "requires a native GPU adapter"]
fn streamed_rows_survive_edits_and_compaction() {
    let (mut gpu, mut scene, camera) = loaded(&Rc::new(solids()));
    shells(&mut scene);
    scene.upload_to(&mut gpu);
    let sheet = scene.sheets[0].row;
    let cloud = scene.streamed[0].row;
    let points = |gpu: &Gpu| {
        let found = gpu
            .cloud
            .clouds
            .iter()
            .find(|c| c.instance == cloud)
            .unwrap();
        (found.resident, found.chunks.len())
    };
    let before = points(&gpu);
    let entity = |gpu: &Gpu| {
        (0..gpu.segments.ribbon_count())
            .filter_map(|row| {
                gpu.segments
                    .row_of(row)
                    .map(|hit| (hit, gpu.segments.source_id(row)))
            })
            .collect::<Vec<_>>()
    };
    let entities = entity(&gpu);
    assert_eq!(entities.len(), 2);

    // a document cloud dies and comes back from its tomb, nothing uploaded; purged, its points are dead
    let row = find(&scene, |g| matches!(g, Geometry::PointCloud(_))).unwrap();
    let resident = gpu.cloud.point_count;

    for _ in 0..3 {
        assert!(scene.delete_row(row));
        commit(&mut scene, &mut gpu);
        assert!(scene.undo());
        commit(&mut scene, &mut gpu);
    }

    assert_eq!(
        gpu.cloud.point_count, resident,
        "an undo draws the buried cloud again"
    );
    let doc = scene.identity_of(row).unwrap().0;
    assert!(scene.delete_row(row));
    Rc::make_mut(&mut scene.docs[doc].session).purge();
    commit(&mut scene, &mut gpu);
    assert!(scene.dead_points() > 0);

    let mut dice = Dice(21);

    for _ in 0..40 {
        edit(&mut scene, &mut dice);
        commit(&mut scene, &mut gpu);
    }

    scene.compact_clouds(&mut gpu);
    scene.rewalk_editable(&mut gpu);
    assert_eq!(points(&gpu), before, "the streamed cloud keeps its points");
    assert_eq!(gpu.cloud.point_count, gpu.cloud.resident());
    assert_eq!(entity(&gpu), entities, "sheet segments keep their entities");
    assert_eq!(
        scene.identity_of(sheet).map(|id| id.1.to_string()),
        Some("sheet:plan.pb".into())
    );
    let now = shot(&mut gpu, &camera);
    assert!(named(&scene, &now.ids).iter().flatten().count() > 0);
}
