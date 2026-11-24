use super::commands_tests::drag;
use super::tests::{Dice, check, find, live, mark, p, scene};
use super::*;
use crate::app::command::verbs::{curve, line, point, polyline};
use crate::app::modeling::Interval;

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
