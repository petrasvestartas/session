use super::tests::{Dice, check, file, find, live, p, scene};
use super::*;
use crate::app::command::verbs::{line, point};
use crate::app::modeling::Interval;
use session_rust::{Line, Mesh, Point, Polyline};

/// Notes come from the committed transaction; an empty one and the layer marker give none.
#[test]
fn notes_come_from_the_committed_transaction() {
    let mut session = Session::new("notes");
    session.begin("create");
    let node = session.add_point(p(0.0, 0.0, 0.0), None);
    let guid = node.borrow().name.clone();
    let notes = commit(&mut session);
    assert_eq!(notes.len(), 1);
    assert_eq!(&*notes[0].guid, guid.as_str());
    assert_eq!(notes[0].what, PRESENCE | GEOMETRY);
    assert_eq!(notes[0].parent, Some(("notes".to_string(), 0)));

    session.begin("move");
    session.set_xform(&guid, Xform::translation(1.0, 0.0, 0.0));
    assert_eq!(commit(&mut session)[0].what, PLACE | SUBTREE);

    session.begin("replace");
    session.replace(&guid, Geometry::Point(Rc::new(p(2.0, 0.0, 0.0))));
    assert_eq!(commit(&mut session)[0].what, GEOMETRY);

    session.begin("delete");
    session.remove_object(&guid);
    let notes = commit(&mut session);
    assert_eq!(notes[0].what, PRESENCE | SUBTREE);
    assert!(
        notes[0]
            .node
            .as_ref()
            .is_some_and(|n| n.upgrade().is_some())
    );

    session.begin("empty");
    assert!(commit(&mut session).is_empty());
    session.begin("marker #1");
    session.set_xform("marker #1", Xform::identity());
    session.remove_xform("marker #1");
    assert!(
        commit(&mut session).is_empty(),
        "the layer marker pair notes nothing"
    );
    assert_eq!(session.history.undo_stack.len(), 5);

    assert!(session.undo()); // the marker
    assert!(session.undo()); // the delete
    let back = stepped(&session.history, true);
    assert_eq!(back[0].what, PRESENCE | GEOMETRY | SUBTREE);
    assert!(session.redo());
    assert_eq!(stepped(&session.history, false)[0].what, PRESENCE | SUBTREE);
}

/// A new point takes one row after the others; every other object keeps its id.
#[test]
fn create_adds_one_row_and_keeps_every_other_id() {
    let mut scene = scene();
    let before: Vec<_> = (0..scene.row_count() as u32)
        .map(|row| scene.identity_of(row))
        .collect();
    let revision = scene.row_revision;
    let (doc, guid) = scene.model(&point::SPEC, &[[1.0, 2.0, 3.0]]).unwrap();
    scene.sync();
    assert!(scene.staged.patches.is_empty() && scene.staged.kills.is_empty());
    assert_eq!(scene.tables.obj.rows.len(), 1, "one object row, appended");
    assert_eq!(Counts::of(&scene.tables).dots, 1, "one dot");
    scene.settle();
    scene.verify();
    assert_ne!(scene.row_revision, revision);
    assert_eq!(scene.row_of(doc, &guid), Some(before.len() as u32));

    for (row, id) in before.iter().enumerate() {
        assert_eq!(
            &scene.identity_of(row as u32),
            id,
            "row {row} kept its object"
        );
    }
}

/// A deleted line and mesh stay on the GPU, hidden; undo shows the same ids and rows with nothing written.
#[test]
fn delete_then_undo_restores_id_and_footprint() {
    let mut scene = scene();
    let line = find(&scene, |g| matches!(g, Geometry::Line(_))).unwrap();
    let mesh = find(&scene, |g| matches!(g, Geometry::Mesh(_))).unwrap();
    let feet = (scene.feet[line as usize], scene.feet[mesh as usize]);
    let spans = (scene.spans.span(feet.0), scene.spans.span(feet.1));
    let ids = (
        scene.identity_of(line).unwrap(),
        scene.identity_of(mesh).unwrap(),
    );
    assert!(scene.delete_row(line));
    check(&mut scene);
    assert!(scene.delete_row(mesh));
    check(&mut scene);
    assert_eq!(scene.tombs.len(), 2);
    assert!(
        scene.graves.is_empty() && scene.dead.is_empty(),
        "nothing retired"
    );
    assert!(scene.identity_of(line).is_none() && scene.identity_of(mesh).is_none());
    assert!(scene.ledger[&mesh].flags & Instance::FLAG_HIDDEN != 0);

    assert!(scene.undo());
    scene.sync();
    assert_eq!(scene.staged.unbury.len(), 1, "shown again");
    assert!(scene.staged.patches.is_empty() && scene.staged.kills.is_empty());
    assert!(scene.tables_empty(), "nothing uploaded");
    scene.settle();
    scene.verify();
    assert_eq!(scene.identity_of(mesh), Some(ids.1.clone()), "same id");
    assert_eq!(
        scene.spans.span(scene.feet[mesh as usize]),
        spans.1,
        "same rows"
    );
    assert!(scene.undo());
    check(&mut scene);
    assert_eq!(scene.identity_of(line), Some(ids.0.clone()));
    assert_eq!(scene.spans.span(scene.feet[line as usize]), spans.0);
    assert!(scene.tombs.is_empty() && scene.tombed.is_empty());

    assert!(scene.redo());
    check(&mut scene);
    assert!(scene.identity_of(line).is_none());
    assert_eq!(scene.tombs.len(), 1);
    assert!(scene.graves.is_empty());
}

/// A tomb no record reaches is released: a dropped redo branch, a cleared history, then the ids are reused.
#[test]
fn unreachable_tombs_are_released() {
    let mut scene = scene();
    scene.model(&point::SPEC, &[[1.0, 2.0, 3.0]]).unwrap();
    check(&mut scene);
    assert!(scene.undo());
    check(&mut scene);
    assert_eq!(scene.tombs.len(), 1, "an undone add waits for its redo");

    scene
        .model(&line::SPEC, &[[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]])
        .unwrap();
    check(&mut scene);
    assert!(scene.tombs.is_empty(), "the redo branch is gone");
    assert!(scene.tombed.is_empty() && !scene.dead.is_empty());
    assert_eq!(scene.ids.len(), 1, "its id is free again");

    let line = find(&scene, |g| matches!(g, Geometry::Line(_))).unwrap();
    let doc = scene.identity_of(line).unwrap().0;
    assert!(scene.delete_row(line));
    check(&mut scene);
    assert_eq!(scene.tombs.len(), 1);
    Rc::make_mut(&mut scene.docs[doc].session).purge();
    check(&mut scene);
    assert!(
        scene.tombs.is_empty(),
        "a purge drops the history that held it"
    );
}

/// An edit in another document ends the redo branch there too: the undone add's tomb is released.
#[test]
fn an_edit_elsewhere_releases_a_redo_tomb() {
    let mut scene = scene();
    scene.model(&point::SPEC, &[[1.0, 2.0, 3.0]]).unwrap();
    check(&mut scene);
    assert!(scene.undo());
    check(&mut scene);
    let created = scene.created_doc.unwrap();
    assert!(scene.tombs.keys().all(|(doc, _)| *doc == created));
    assert_eq!(scene.tombs.len(), 1, "an undone add waits for its redo");

    let row = find(&scene, |g| matches!(g, Geometry::Line(_))).unwrap();
    assert_ne!(scene.identity_of(row).unwrap().0, created);
    assert!(scene.delete_row(row));
    check(&mut scene);
    assert!(!scene.docs[created].session.history.can_redo());
    assert!(
        scene.tombs.keys().all(|(doc, _)| *doc != created),
        "the dropped redo branch no longer holds the point"
    );

    while scene.purge_step() {}
    assert!(
        !scene.docs[created].session.purge_due(),
        "the idle purge freed it"
    );
}

/// Past the cap the oldest tombs are released first; an undo of those walks the object again.
#[test]
fn tombs_past_the_cap_release_the_oldest() {
    let mut scene = scene();
    let line = find(&scene, |g| matches!(g, Geometry::Line(_))).unwrap();
    let mesh = find(&scene, |g| matches!(g, Geometry::Mesh(_))).unwrap();
    let size = |scene: &Scene, row: u32| scene.spans.span(scene.feet[row as usize]).count.bytes();
    scene.tomb_cap = size(&scene, mesh);
    assert!(scene.delete_row(line));
    check(&mut scene);
    assert!(scene.delete_row(mesh));
    check(&mut scene);
    assert_eq!(scene.tombs.len(), 1, "the line went first");
    assert!(scene.tombed.bytes() <= scene.tomb_cap);
    assert_eq!(scene.graves.len(), 1, "its rows wait for its return");

    assert!(scene.undo());
    check(&mut scene);
    assert!(scene.tombs.is_empty());
    assert!(scene.undo());
    scene.sync();
    assert_eq!(scene.staged.patches.len(), 1, "written back at its grave");
    scene.settle();
    scene.verify();
    assert!(scene.identity_of(line).is_some());
}

/// Delete, undo, redo, undo ten times hands the same 10k-vertex mesh back: tombs flip in place, the purge keeps the dead few.
#[test]
fn delete_undo_reuses_the_mesh_without_a_copy() {
    let mut session = Session::new("heavy");
    let points: Vec<Point> = (0..10_000)
        .map(|i| p(f64::from(i % 100), f64::from(i / 100), 0.0))
        .collect();
    let faces: Vec<Vec<usize>> = (0..99 * 99)
        .map(|i| {
            let corner = i / 99 * 100 + i % 99;
            vec![corner, corner + 1, corner + 101, corner + 100]
        })
        .collect();
    session.add_mesh(Mesh::from_vertices_and_faces(points, faces), None);
    let mut scene = Scene::new();
    scene.add_file(file("heavy", session, Xform::identity()));
    scene.settle();
    let (doc, guid) = scene.identity_of(0).unwrap();
    let mesh = |scene: &Scene| match scene.docs[doc].session.lookup.get(guid.as_ref()) {
        Some(Geometry::Mesh(mesh)) => Rc::as_ptr(mesh),
        _ => panic!("the mesh is live"),
    };
    let held = mesh(&scene);

    for _ in 0..10 {
        assert!(scene.delete_row(0));
        check(&mut scene);
        assert!(scene.undo());
        check(&mut scene);
        assert_eq!(mesh(&scene), held, "the same mesh, not a copy");
        assert!(scene.redo());
        check(&mut scene);
        assert!(scene.undo());
        check(&mut scene);
        assert_eq!(mesh(&scene), held, "the same mesh, not a copy");

        while scene.purge_step() {}

        let session = &scene.docs[doc].session;
        assert!(!session.purge_due(), "the idle purge ran");
        assert!(session.history.bytes <= session.history.budget);
        assert!(session.number_of_dead() <= 2, "only pinned entries stay");
    }
}

/// A trim that keeps the counts writes in place; one that changes them moves and leaves a grave.
#[test]
fn replace_in_place_or_via_grave() {
    let mut scene = scene();
    let line = find(&scene, |g| matches!(g, Geometry::Line(_))).unwrap();
    let revision = scene.row_revision;
    scene.selected = Some(line);
    scene.edit_interval(Interval::Trim(0.2, 0.8)).unwrap();
    scene.sync();
    assert_eq!(scene.staged.patches.len(), 1);
    assert!(scene.staged.kills.is_empty() && scene.tables_empty());
    scene.settle();
    scene.verify();
    assert_eq!(scene.row_revision, revision, "a redraw keeps the rows");

    let curve = find(&scene, |g| matches!(g, Geometry::NurbsCurve(_))).unwrap();
    let before = scene.spans.span(scene.feet[curve as usize]);
    scene.selected = Some(curve);
    scene.edit_interval(Interval::Extend(-0.5, 1.5)).unwrap();
    scene.sync();
    assert_eq!(scene.staged.kills.len(), 1, "the old rows die");
    assert!(
        Counts::of(&scene.tables).ribbons > before.count.ribbons,
        "the new rows are appended"
    );
    scene.settle();
    scene.verify();
    let after = scene.spans.span(scene.feet[curve as usize]);
    assert_ne!(after.count, before.count);
    assert_eq!(scene.graves.len(), 1);

    assert!(scene.undo());
    check(&mut scene);
    assert_eq!(
        scene.spans.span(scene.feet[curve as usize]),
        before,
        "back at the start"
    );
    assert!(scene.redo());
    check(&mut scene);
    assert_eq!(scene.spans.span(scene.feet[curve as usize]), after);
}

/// Deleting a parent draws its children at their own transform.
#[test]
fn removing_a_parent_draws_children_like_a_rewalk() {
    let mut scene = scene();
    let element = find(&scene, |g| matches!(g, Geometry::Element(_))).unwrap();
    let parent = find(&scene, |g| matches!(g, Geometry::Point(_))).unwrap();
    let count = scene.object_count();
    assert!(scene.delete_row(element));
    check(&mut scene);
    assert_eq!(scene.object_count(), count - 2, "the element goes with its attribute");
    assert!(scene.delete_row(parent));
    check(&mut scene);
    assert!(scene.undo());
    check(&mut scene);
    assert!(scene.undo());
    check(&mut scene);
    assert_eq!(scene.object_count(), count);
}

/// Layer undo and redo flip nodes in place: no tree walk, rows stay right.
#[test]
fn layer_steps_walk_no_tree() {
    let mut scene = scene();
    let line = find(&scene, |g| matches!(g, Geometry::Line(_))).unwrap();
    let (doc, _) = scene.identity_of(line).unwrap();
    scene.change_object_layer(&[line], doc, "roof").unwrap();
    check(&mut scene);
    let searches = scene.searches;

    for back in [true, false, true] {
        assert!(if back { scene.undo() } else { scene.redo() });
        check(&mut scene);
        assert_eq!(scene.searches, searches, "no tree walk");
    }

    scene.delete_layer(doc, "walls").unwrap();
    check(&mut scene);
    assert!(scene.undo());
    check(&mut scene);
    assert!(scene.redo());
    check(&mut scene);
    assert!(scene.undo());
    check(&mut scene);
}

/// A flat drawing's new flat line takes the sheet flag and pens; a 3D point and `Created` do not.
#[test]
fn sheet_rule() {
    let mut flat = Session::new("plan");
    flat.add_line(Line::new(0.0, 0.0, 0.0, 10.0, 0.0, 0.0), None);
    flat.add_line(Line::new(0.0, 5.0, 0.0, 10.0, 5.0, 0.0), None);
    let mut scene = Scene::new();
    scene.add_file(file("plan", flat, Xform::identity()));
    scene.settle();
    assert!(scene.doc_state[0].sheet.is_some());
    scene.current_layer = Some((0, "plan".into()));
    let (_, line) = scene
        .model(&line::SPEC, &[[0.0, 9.0, 0.0], [10.0, 9.0, 0.0]])
        .unwrap();
    let (_, point) = scene.model(&point::SPEC, &[[0.0, 0.0, 50.0]]).unwrap();
    scene.sync();
    let pens: Vec<f32> = scene.tables.seg.ribbons.iter().map(|s| s.radius).collect();
    assert_eq!(pens, vec![0.5], "the flat line takes the sheet pen");
    scene.settle();
    scene.verify();
    let flags = |scene: &Scene, guid: &str| {
        scene.ledger[&scene.row_of(0, guid).unwrap()].flags & Instance::FLAG_SHEET
    };
    assert_ne!(flags(&scene, &line), 0);
    assert_eq!(flags(&scene, &point), 0);
    scene.rewalk_cpu();
    assert_ne!(flags(&scene, &line), 0, "compaction keeps it");

    scene.current_layer = None;
    let (doc, created) = scene
        .model(&line::SPEC, &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]])
        .unwrap();
    check(&mut scene);
    assert_eq!(
        scene.ledger[&scene.row_of(doc, &created).unwrap()].flags & Instance::FLAG_SHEET,
        0,
        "Created is never a sheet"
    );
}

/// Drawing, deleting and undoing cost the same in a scene of a thousand objects and of a hundred thousand.
#[test]
fn edit_cost_does_not_scale_with_the_scene() {
    let time = |objects: usize| {
        let mut session = Session::new("big");

        for i in 0..objects {
            session.add_line(Line::new(i as f64, 0.0, 0.0, i as f64, 1.0, 0.0), None);
        }

        let mut scene = Scene::new();
        scene.add_file(file("big", session, Xform::identity()));
        scene.settle();
        let started = std::time::Instant::now();

        for i in 0..40 {
            let (doc, guid) = scene.model(&point::SPEC, &[[i as f64, 5.0, 0.0]]).unwrap();
            scene.sync();
            scene.settle();
            let row = scene.row_of(doc, &guid).unwrap();
            assert!(scene.delete_row(row));
            scene.sync();
            scene.settle();
            assert!(scene.undo());
            scene.sync();
            scene.settle();
        }

        started.elapsed().as_secs_f64()
    };
    let small = time(1_000);
    let large = time(100_000);
    assert!(
        large < small * 4.0 + 0.05,
        "120 edits: {small:.4} s with 1k objects, {large:.4} s with 100k"
    );
}

/// A drag that grows a curve mostly fits its headroom; cancelling returns to the grave.
#[test]
fn preview_growth_is_bounded_and_cancel_restores() {
    let mut scene = scene();
    let curve = find(&scene, |g| matches!(g, Geometry::NurbsCurve(_))).unwrap();
    let (doc, guid) = scene.identity_of(curve).unwrap();
    let source = scene.docs[doc].session.lookup[guid.as_ref()].clone();
    let start = scene.spans.span(scene.feet[curve as usize]);
    let Geometry::NurbsCurve(original) = &source else {
        panic!()
    };
    let mut moves = 0;
    let mut largest = 0u64;

    for frame in 0..200 {
        let mut grown = (**original).clone();
        let (lo, hi) = grown.domain();
        let reach = 1.0 + (frame % 50 + 1) as f64 * 0.04;
        assert!(grown.extend(lo, lo + (hi - lo) * reach));
        scene.redraw(curve, &Geometry::NurbsCurve(Rc::new(grown)), true);
        moves += usize::from(!scene.tables_empty());
        scene.settle();
        let alloc = scene
            .caps
            .get(&curve)
            .map_or(scene.spans.span(scene.feet[curve as usize]).count, |cap| {
                cap.alloc
            });
        largest = largest.max(alloc.bytes());
    }

    assert!(moves <= 16, "{moves} of 200 frames needed new rows");
    assert!(
        scene.dead.bytes() <= (moves as u64 + 1) * largest,
        "{} dead bytes",
        scene.dead.bytes()
    );
    scene.redraw(curve, &source, false);
    scene.settle();
    assert_eq!(
        scene.spans.span(scene.feet[curve as usize]),
        start,
        "back at its grave"
    );
    scene.verify();
}

/// A few frames of a curve or polyline growing, then the release or the cancel.
pub(super) fn drag(scene: &mut Scene, dice: &mut Dice) {
    let rows: Vec<u32> = live(scene)
        .into_iter()
        .map(|(row, _)| row)
        .filter(|&row| {
            matches!(
                scene.geometry(row),
                Some(Geometry::NurbsCurve(_) | Geometry::Polyline(_))
            )
        })
        .collect();

    if rows.is_empty() {
        return;
    }

    let row = rows[dice.roll(rows.len())];
    let source = scene.geometry(row).unwrap().clone();
    let mut shape = source.clone();

    for _ in 0..1 + dice.roll(6) {
        shape = match &source {
            Geometry::NurbsCurve(curve) => {
                let mut grown = (**curve).clone();
                let (lo, hi) = grown.domain();
                let _ = grown.extend(lo, hi + (hi - lo) * (0.1 + dice.roll(20) as f64 * 0.1));
                Geometry::NurbsCurve(Rc::new(grown))
            }
            Geometry::Polyline(line) => {
                let mut points = line.get_points();

                for i in 0..dice.roll(4) {
                    points.push(p(i as f64, 9.0, 1.0));
                }

                Geometry::Polyline(Rc::new(Polyline::new(points)))
            }
            _ => unreachable!(),
        };
        scene.redraw(row, &shape, true);
        scene.settle();
    }

    if dice.roll(2) == 0 {
        scene.redraw(row, &source, false);
    } else {
        scene.commit_geometry(row, shape, "drag").unwrap();
        scene.sync();
    }

    scene.settle();
}

/// Moving an element moves what hangs under it, its attributes too.
#[test]
fn moving_an_element_moves_its_attributes() {
    let mut scene = scene();
    let element = find(&scene, |g| matches!(g, Geometry::Element(_))).unwrap();
    let (doc, _) = scene.identity_of(element).unwrap();
    let child = live(&scene)
        .into_iter()
        .find(|(_, (owner, guid))| {
            *owner == doc
                && scene
                    .parent_of(doc, guid)
                    .is_some_and(|parent| parent.borrow().name == "attributes")
        })
        .unwrap()
        .0;
    let before = scene.placement_of(child).unwrap();
    scene
        .transform_rows(&[element], &Xform::translation(0.0, 0.0, 300.0), "move")
        .unwrap();
    check(&mut scene);
    let after = scene.placement_of(child).unwrap();
    assert_eq!(after.m[14] - before.m[14], 300.0);
}

/// A move previews what hangs under a row with it: the element's attributes come along.
#[test]
fn a_move_carries_what_hangs_under_the_row() {
    let scene = scene();
    let element = find(&scene, |g| matches!(g, Geometry::Element(_))).unwrap();
    let rows = scene.with_descendants(&[element]);
    assert!(rows.len() > 1 && rows.contains(&element));
    let (doc, _) = scene.identity_of(element).unwrap();

    for row in rows.into_iter().filter(|&row| row != element) {
        let (owner, guid) = scene.identity_of(row).unwrap();
        assert_eq!(owner, doc);
        assert_eq!(scene.parent_of(owner, &guid).unwrap().borrow().name, "attributes");
    }
}

/// Copying an element copies its attributes, hidden while Element Attributes is off; deleting it deletes them.
#[test]
fn an_element_copies_and_deletes_with_its_attributes() {
    let mut scene = scene();
    let element = find(&scene, |g| matches!(g, Geometry::Element(_))).unwrap();
    let (doc, _) = scene.identity_of(element).unwrap();
    let under = |scene: &Scene, row: u32| scene.with_descendants(&[row]).len() - 1;
    assert_eq!(under(&scene, element), 1);

    let copies = scene.copy_rows(&[element], &Xform::translation(0.0, 50.0, 0.0)).unwrap();
    check(&mut scene);
    let copy = scene.row_of(doc, &copies[0].1).unwrap();
    let attribute = *scene.with_descendants(&[copy]).iter().find(|&&row| row != copy).unwrap();
    assert!(scene.hidden.contains(&scene.identity_of(attribute).unwrap()), "hidden while Element Attributes is off");

    scene.attributes = true;
    scene.rewalk_cpu(); // as Element Attributes On does
    let shown = scene.copy_rows(&[element], &Xform::translation(0.0, 90.0, 0.0)).unwrap();
    check(&mut scene);
    let row = scene.row_of(doc, &shown[0].1).unwrap();
    let attribute = *scene.with_descendants(&[row]).iter().find(|&&r| r != row).unwrap();
    assert!(!scene.hidden.contains(&scene.identity_of(attribute).unwrap()), "shown while Element Attributes is on");

    let before = scene.object_count();
    assert_eq!(scene.delete_rows(&[copy]), 2, "the element and its attribute");
    check(&mut scene);
    assert_eq!(scene.object_count(), before - 2);
}
