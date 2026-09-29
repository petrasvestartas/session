use super::commands_tests::drag;
use super::tests::{Dice, arc, check, file, find, live, mark, p, scene, shells};
use super::*;
use crate::app::command::verbs::{curve, line, point, polyline};
use crate::app::modeling::Interval;
use crate::app::scene::FileDoc;
use session_rust::{Element, Line, Mesh, Point, Polyline};

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

/// Hundreds of random edits, undos and layer moves: after each, the rows match a fresh walk.
#[test]
fn incremental_matches_fresh_oracle() {
    for seed in [1, 7, 42] {
        let mut scene = scene();
        let mut dice = Dice(seed);

        for step in 0..150 {
            edit(&mut scene, &mut dice);
            scene.sync();
            scene.settle();
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| scene.verify()))
                .unwrap_or_else(|_| panic!("seed {seed} step {step}"));
        }
    }
}

/// Random edits with a tomb cap of a few rows: releases, graves and re-walks on undo match a fresh walk.
#[test]
fn tombs_under_a_small_cap_match_fresh_oracle() {
    for seed in [3, 11, 14] {
        let mut scene = scene();
        scene.tomb_cap = 4096;
        let mut dice = Dice(seed);

        for step in 0..150 {
            edit(&mut scene, &mut dice);
            scene.sync();
            scene.settle();
            assert!(
                scene.tombed.bytes() <= scene.tomb_cap,
                "seed {seed} step {step} cap"
            );
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| scene.verify()))
                .unwrap_or_else(|_| panic!("seed {seed} step {step}"));
        }
    }
}

/// Hints and the node cache only save walks: without them every row ends up the same.
#[test]
fn hints_and_node_cache_are_only_a_cache() {
    let run = |cached: bool| {
        let mut scene = scene();
        let mut dice = Dice(5);

        for _ in 0..120 {
            edit(&mut scene, &mut dice);

            if !cached {
                scene.hints.clear();

                for state in &mut scene.doc_state {
                    state.nodes_from = 0;
                }
            }

            check(&mut scene);
        }

        // guids are new in every run: compare documents, rows and lanes
        let rows: Vec<_> = (0..scene.row_count() as u32)
            .map(|row| {
                (
                    scene.identity_of(row).map(|id| id.0),
                    scene.spans.span(scene.feet[row as usize]),
                )
            })
            .collect();
        (rows, scene.searches)
    };
    let (cached, searched) = run(true);
    let (plain, all) = run(false);
    assert_eq!(cached, plain);
    assert!(searched < all, "the cache saves walks: {searched} of {all}");

    // undo of a move finds every node in the cache
    let mut scene = scene();
    let row = find(&scene, |g| matches!(g, Geometry::Point(_))).unwrap();
    scene.transform_rows(&[row], &Xform::translation(1.0, 0.0, 0.0), "move");
    check(&mut scene);
    let searches = scene.searches;
    assert!(scene.undo());
    check(&mut scene);
    assert_eq!(scene.searches, searches, "no tree walk");

    // undo of a create drops a node the tree no longer has: no walk looks for it
    scene.model(&point::SPEC, &[[1.0, 2.0, 3.0]]).unwrap();
    check(&mut scene);
    let searches = scene.searches;
    assert!(scene.undo());
    check(&mut scene);
    assert_eq!(
        scene.searches, searches,
        "no tree walk for a removed object"
    );

    // a weak handle on a session, as the inspection keeps, moves it at every edit: the cache stays
    let row = find(&scene, |g| matches!(g, Geometry::Point(_))).unwrap();
    let (doc, _) = scene.identity_of(row).unwrap();
    let held = Rc::downgrade(&scene.docs[doc].session);
    scene.transform_rows(&[row], &Xform::translation(0.0, 1.0, 0.0), "move");
    check(&mut scene);
    assert!(held.upgrade().is_none(), "the edit moved the session");
    assert_eq!(
        scene.searches, searches,
        "no tree walk after the session moved"
    );

    // a tree swapped under the cache is walked once by the next sync, not per row by later lookups
    scene.forget_nodes(doc);
    check(&mut scene);
    assert!(scene.cache_hit(row).is_some_and(|(_, in_tree)| in_tree));

    // a session another holder shares is copied by the edit; the copy is cached before the sync
    let shared = Rc::clone(&scene.docs[doc].session);
    scene.transform_rows(&[row], &Xform::translation(0.0, 0.0, 1.0), "move");
    assert!(scene.cache_hit(row).is_some_and(|(_, in_tree)| in_tree));
    drop(shared);
    check(&mut scene);
}

/// Noting everything, the fallback for an edit path that notes nothing, changes no row.
#[test]
fn touch_all_keeps_a_synced_scene() {
    let mut scene = scene();
    let mut dice = Dice(13);

    for _ in 0..40 {
        edit(&mut scene, &mut dice);
        check(&mut scene);
    }

    let before: Vec<_> = (0..scene.row_count() as u32)
        .map(|row| scene.identity_of(row))
        .collect();
    let revision = scene.row_revision;
    scene.touch_all();
    check(&mut scene);
    let after: Vec<_> = (0..scene.row_count() as u32)
        .map(|row| scene.identity_of(row))
        .collect();
    assert_eq!(before, after);
    assert_eq!(scene.row_revision, revision, "no row came or went");
}

/// Hidden and colored objects keep both through every kind of edit, undo and compaction.
#[test]
fn hidden_and_colored_objects_keep_their_flags() {
    for seed in [2, 9] {
        let mut scene = scene();
        let mut dice = Dice(seed);

        for step in 0..150 {
            if dice.roll(3) == 0 {
                mark(&mut scene, &mut dice);
            }

            edit(&mut scene, &mut dice);
            scene.sync();
            scene.settle();

            if step % 50 == 49 {
                scene.rewalk_cpu();
            }

            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| scene.verify()))
                .unwrap_or_else(|_| panic!("seed {seed} step {step}"));
        }
    }
}

/// With a streamed sheet and cloud loaded every edit works; the shells stay read-only.
#[test]
fn streamed_scene_is_editable() {
    let mut scene = scene();
    let sheet = scene.sheets[0].row;
    let cloud = scene.streamed[0].row;
    let shell = |scene: &Scene| {
        (
            scene.identity_of(sheet),
            scene.identity_of(cloud),
            scene.feet[sheet as usize],
            scene.feet[cloud as usize],
            scene.sheet_slot(sheet),
        )
    };
    let before = shell(&scene);
    let (point_doc, point) = scene.model(&point::SPEC, &[[1.0, 1.0, 1.0]]).unwrap();
    check(&mut scene);
    let point = scene.row_of(point_doc, &point).unwrap();
    scene
        .model(&line::SPEC, &[[0.0, 0.0, 0.0], [4.0, 4.0, 0.0]])
        .unwrap();
    check(&mut scene);
    assert!(scene.delete_row(point));
    check(&mut scene);
    assert!(scene.undo());
    check(&mut scene);
    assert!(scene.redo());
    check(&mut scene);
    let line = find(&scene, |g| matches!(g, Geometry::Line(_))).unwrap();
    scene.selected = Some(line);
    scene.edit_interval(Interval::Trim(0.1, 0.9)).unwrap();
    check(&mut scene);
    let polyline = find(&scene, |g| matches!(g, Geometry::Polyline(_))).unwrap();
    scene.explode_rows(&[polyline]).unwrap();
    check(&mut scene);
    let (doc, _) = scene.identity_of(line).unwrap();
    let made = scene.new_layer(doc, "roof", false).unwrap();
    check(&mut scene);
    scene.rename_layer(doc, &made, "made").unwrap();
    check(&mut scene);
    scene.change_object_layer(&[line], doc, "made").unwrap();
    check(&mut scene);

    for _ in 0..3 {
        assert!(scene.undo());
        check(&mut scene);
    }

    assert_eq!(shell(&scene), before, "shell rows untouched");
    assert!(!scene.delete_row(sheet), "a shell cannot be deleted");
    assert!(
        scene
            .transform_rows(&[cloud], &Xform::translation(1.0, 0.0, 0.0), "move")
            .is_none()
    );
    assert!(
        scene
            .commit_geometry(sheet, Geometry::Point(Rc::new(p(0.0, 0.0, 0.0))), "edit")
            .is_err()
    );

    // a picked sheet entity or cloud point says why it stays put
    let entity = crate::app::deform::Target::Edge(0);
    let shift = Xform::translation(1.0, 0.0, 0.0);
    let point = crate::app::selection::ControlId::Point(0);
    let read_only = Err(crate::app::scene::READ_ONLY.to_string());
    assert_eq!(
        scene.edit_subobject(sheet, entity, &shift, "move"),
        read_only
    );
    assert_eq!(
        scene.set_source_control(cloud, point, &p(1.0, 0.0, 0.0)),
        read_only
    );
    scene.hidden.insert(scene.identity_of(sheet).unwrap());
    scene.locked.insert(scene.identity_of(cloud).unwrap());
    assert_eq!(scene.hidden_rows(), vec![sheet]);
    assert!(!scene.selectable(cloud));
    check(&mut scene);
}

/// After edits a picked row still names its object, and its lane rows name the row.
#[test]
fn row_identity_survives_edits() {
    let mut scene = scene();
    let mut dice = Dice(11);

    for _ in 0..60 {
        edit(&mut scene, &mut dice);
        check(&mut scene);
    }

    for (row, (doc, guid)) in live(&scene) {
        let geometry = scene.geometry(row).expect("a live row has geometry");
        assert_eq!(geometry.guid(), guid.as_ref());
        assert!(scene.docs[doc].session.lookup.contains_key(guid.as_ref()));

        if let Some(range) = scene.ribbon_range(row) {
            assert!(range.end <= scene.uploaded.ribbons);
        }
    }
}

/// Dead rows past the threshold call a compaction, which leaves the lanes as a fresh walk.
#[test]
fn compaction_reclaims_dead_rows_and_keeps_ids() {
    let mut scene = scene();
    let mut dice = Dice(3);

    for _ in 0..80 {
        edit(&mut scene, &mut dice);
        check(&mut scene);
    }

    let ids: Vec<_> = (0..scene.row_count() as u32)
        .map(|row| scene.identity_of(row))
        .collect();
    scene.rewalk_cpu();
    scene.verify();
    assert_eq!(scene.dead, Counts::default());
    assert!(scene.graves.is_empty() && scene.caps.is_empty());
    let after: Vec<_> = (0..scene.row_count() as u32)
        .map(|row| scene.identity_of(row))
        .collect();
    assert_eq!(ids, after, "ids stay");

    // a fresh walk's lanes: every live row packed in document order
    let packed: u64 = live(&scene)
        .iter()
        .map(|(row, _)| scene.spans.span(scene.feet[*row as usize]).count.bytes())
        .sum();
    assert_eq!(packed, scene.uploaded.bytes());
    assert!(!scene.compaction_due());
}

/// Drags released or cancelled between edits, undos and compactions leave what a fresh scene draws.
#[test]
fn drags_between_edits_match_a_fresh_scene() {
    for seed in [4, 17, 23] {
        let mut scene = scene();
        let mut dice = Dice(seed);

        for step in 0..120 {
            if dice.roll(2) == 0 {
                drag(&mut scene, &mut dice);
            } else {
                edit(&mut scene, &mut dice);
                scene.sync();
                scene.settle();
            }

            if step % 40 == 39 {
                scene.rewalk_cpu();
            }

            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| scene.verify()))
                .unwrap_or_else(|_| panic!("seed {seed} step {step}"));
        }
    }
}

/// Frames drawn by a headless GPU; every test here needs a native adapter.
#[cfg(not(target_arch = "wasm32"))]
mod gpu {
    use super::*;
    use crate::camera::Camera;
    use crate::engine::gpu::FrameInput;
    use session_rust::{BRep, Color, PointCloud, Vector};

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
        use crate::app::clipping::{Mode, clip_plane, plane_from};
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

        assert!(scene.dead.bytes() > 0);
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
            gpu.arena.allocated_bytes()
                + gpu.segments.allocated_bytes()
                + gpu.glyphs.allocated_bytes()
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
        assert!(scene.dead_points > 0);

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
}
