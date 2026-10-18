use crate::{
    editor::{Action, Editor},
    scene::Scene,
};
use prost::Message;
use session_rust::{Line, Point, Session, Xform};
use std::rc::Rc;
fn editor() -> Editor {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e
}
fn command(e: &mut Editor, line: &str) -> Result<(), String> {
    let action = if let Some(a) = e.tool_line(line) {
        a.map_err(str::to_owned)?
    } else {
        let crate::commands::Command::Editor(a) = crate::commands::parse(line)? else {
            return Err("Editor action".into());
        };
        a
    };
    e.apply(action).map_err(|m| {
        if m == "Shape input invalid" {
            e.tool_status.clone()
        } else {
            m.into()
        }
    })?;
    Ok(())
}
#[test]
fn split_preserves_tree_placement_shared_original_colours_and_exact_archive_with_one_undo() {
    let mut source = Session::new("shared");
    let parent = source.add_group("parts");
    source.set_xform(&parent.borrow().name, Xform::translation(10., 0., 0.));
    source.add_line(
        Line::from_points(&Point::new(-2., 0., 0.), &Point::new(2., 0., 0.)),
        Some(&parent),
    );
    let bytes = source.to_proto().encode_to_vec();
    let mut e = editor();
    crate::scene_document::import(
        &mut e.scene,
        &bytes,
        "first",
        &Xform::translation(100., 0., 0.),
        3.,
    )
    .unwrap();
    crate::scene_document::import(
        &mut e.scene,
        &bytes,
        "second",
        &Xform::translation(200., 0., 0.),
        3.,
    )
    .unwrap();
    command(&mut e, "Line w110,-2,0 w110,2,0").unwrap();
    let ids = e.scene.lines().iter().map(|r| r.id).collect::<Vec<_>>();
    let (target, other, cutter) = (ids[0], ids[1], ids[2]);
    let old = e.scene.lines()[0].prepared.source.clone();
    let other_source = e.scene.lines()[1].prepared.source.clone();
    let guid = e.scene.row_guid(target).unwrap().to_owned();
    let model = e.scene.placement(target).unwrap();
    let v = Rc::make_mut(&mut e.scene.view);
    v.faces.insert(target, [60, 170, 100]);
    v.edges.insert(target, [0, 0, 0]);
    e.select_objects([target], false);
    command(&mut e, "Split").unwrap();
    assert!(e.gizmo_origin().is_none());
    e.apply(Action::SplitPick(Some(cutter))).unwrap();
    assert_eq!(e.selected_objects(), [target]);
    assert!(e.highlighted_objects().contains(&cutter));
    command(&mut e, "").unwrap();
    assert!(e.split.is_none());
    assert_eq!(e.scene.lines().len(), 4);
    assert_eq!(e.scene.row_guid(target), Some(guid.as_str()));
    assert_eq!(e.scene.placement(target).unwrap().m, model.m);
    assert_eq!(old.end()[0], 2.);
    assert!(Rc::ptr_eq(
        &other_source,
        &e.scene
            .lines()
            .iter()
            .find(|r| r.id == other)
            .unwrap()
            .prepared
            .source
    ));
    let pieces = e
        .scene
        .lines()
        .iter()
        .filter(|r| r.model.m == model.m)
        .collect::<Vec<_>>();
    assert_eq!(pieces.len(), 2);
    assert_eq!(pieces[0].prepared.source.end()[0], 0.);
    assert_eq!(pieces[1].prepared.source.start()[0], 0.);
    assert!(
        pieces
            .iter()
            .all(|r| e.scene.view.edges.get(&r.id) == Some(&[0, 0, 0]))
    );
    let doc = &e.scene.forest.documents[0];
    let parent = doc.nodes.iter().position(|n| n.label == "parts").unwrap();
    assert_eq!(
        doc.nodes
            .iter()
            .filter(|n| n.parent == Some(parent) && n.object.is_some())
            .count(),
        2
    );
    assert_eq!(
        e.scene.forest.documents[1]
            .nodes
            .iter()
            .filter(|n| n.object.is_some())
            .count(),
        1
    );
    let mut reopened = Scene::empty();
    crate::scene_document::open(
        &mut reopened,
        &crate::scene_archive::save(&e.scene).unwrap(),
    )
    .unwrap();
    assert_eq!(reopened.lines().len(), 4);
    assert_eq!(
        reopened
            .lines()
            .iter()
            .filter(|r| r.model.m == model.m)
            .count(),
        2
    );
    command(&mut e, "Undo").unwrap();
    assert_eq!(e.scene.lines().len(), 3);
    assert!(Rc::ptr_eq(&old, &e.scene.lines()[0].prepared.source));
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.lines().len(), 4);
}
#[test]
fn polyline_and_rational_curve_pieces_keep_original_family_corners_and_old_allocations() {
    for text in [
        "Polyline w-2,0,0 w-1,0,0 w1,0,0 w2,0,0",
        "Nurbs Curve Arc Center w0,0,0 2 180",
    ] {
        let mut e = editor();
        command(&mut e, "CPlane XY").unwrap();
        command(&mut e, text).unwrap();
        let target = e.scene.paths()[0].id;
        let old = crate::source_edit::original(&e.scene, target).unwrap();
        command(&mut e, "Line w0,-3,0 w0,3,0").unwrap();
        let cutter = e.scene.lines()[0].id;
        e.select_objects([target], false);
        command(&mut e, "Split").unwrap();
        e.apply(Action::SplitPick(Some(cutter))).unwrap();
        command(&mut e, "Split").unwrap();
        assert_eq!(e.scene.paths().len(), 2);
        assert!(e.scene.paths().iter().all(|r| std::mem::discriminant(
            &crate::source_edit::original(&e.scene, r.id).unwrap()
        ) == std::mem::discriminant(&old)));
        if let session_rust::Geometry::Polyline(source) = &old {
            assert_eq!(source.get_points().len(), 4);
            assert_eq!(
                e.scene
                    .paths()
                    .iter()
                    .map(|r| r.prepared.source.coordinates().len())
                    .sum::<usize>(),
                6
            );
        } else if let session_rust::Geometry::NurbsCurve(source) = &old {
            assert!(source.m_is_rat);
            assert!(e.scene.paths().iter().all(
                |r| matches!(&r.prepared.source,crate::chain::Source::Curve(c,_)if c.m_is_rat)
            ));
        }
        command(&mut e, "Undo").unwrap();
        assert_eq!(e.scene.paths().len(), 1);
        assert!(crate::source_edit::same(
            &old,
            &crate::source_edit::original(&e.scene, target).unwrap()
        ));
    }
}
#[test]
fn split_cutter_toggles_whole_tree_rows_and_empty_confirmation_keep_the_pending_owner() {
    let mut e = editor();
    for text in ["Line w-2,0,0 w2,0,0", "Line w0,-2,0 w0,2,0", "Point w0,0,0"] {
        command(&mut e, text).unwrap()
    }
    let (target, cutter) = (e.scene.lines()[0].id, e.scene.lines()[1].id);
    e.select_objects([target], false);
    command(&mut e, "Split").unwrap();
    assert!(command(&mut e, "Split").is_err());
    assert!(e.split.is_some());
    assert!(e.apply(Action::SplitPick(Some(target))).is_err());
    assert!(
        e.apply(Action::SplitPick(Some(e.scene.points()[0].id)))
            .is_err()
    );
    e.apply(Action::TreePick {
        ids: vec![cutter],
        additive: false,
    })
    .unwrap();
    assert_eq!(e.split.as_ref().unwrap().cutters.len(), 1);
    assert_eq!(e.selected_objects(), [target]);
    e.apply(Action::SelectRows {
        ids: vec![cutter],
        add: false,
        remove: false,
    })
    .unwrap();
    assert!(e.split.as_ref().unwrap().cutters.is_empty());
    command(&mut e, "Escape").unwrap();
    assert!(e.split.is_none());
    assert_eq!(e.selected_objects(), [target]);
    assert!(e.gizmo_origin().is_some());
}
#[test]
fn split_no_division_stale_and_locked_owners_preserve_redo_without_partial_edits() {
    let mut e = editor();
    for text in [
        "Line w-2,0,0 w2,0,0",
        "Line w5,-2,0 w5,2,0",
        "Point w0,0,0",
        "Undo",
    ] {
        command(&mut e, text).unwrap()
    }
    let (target, cutter) = (e.scene.lines()[0].id, e.scene.lines()[1].id);
    let old = e.scene.lines()[0].prepared.source.clone();
    e.select_objects([target], false);
    command(&mut e, "Split").unwrap();
    e.apply(Action::SplitPick(Some(cutter))).unwrap();
    command(&mut e, "").unwrap();
    assert!(e.split.is_none());
    assert_eq!(e.scene.lines().len(), 2);
    assert!(Rc::ptr_eq(&old, &e.scene.lines()[0].prepared.source));
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.points().len(), 1);
    e.select_objects([target], false);
    command(&mut e, "Split").unwrap();
    e.apply(Action::SplitPick(Some(cutter))).unwrap();
    e.scene
        .place_owner(cutter, Xform::translation(-5., 0., 0.))
        .unwrap();
    assert!(command(&mut e, "").is_err());
    assert!(e.split.is_none());
    assert!(Rc::ptr_eq(&old, &e.scene.lines()[0].prepared.source));
    Rc::make_mut(&mut e.scene.view).lock(&[target], true);
    assert!(command(&mut e, "Split").is_err());
    assert!(e.split.is_none());
}
#[test]
fn split_arity_undo_and_new_commands_end_pending_state_while_fit_preserves_it() {
    let mut e = editor();
    command(&mut e, "Line w-2,0,0 w2,0,0").unwrap();
    let target = e.scene.lines()[0].id;
    e.select_objects([target], false);
    command(&mut e, "Split").unwrap();
    assert!(crate::commands::parse("Split extra").is_err());
    command(&mut e, "Fit").unwrap();
    assert!(e.split.is_some());
    command(&mut e, "Undo").unwrap();
    assert!(e.split.is_none());
    command(&mut e, "Redo").unwrap();
    e.select_objects([target], false);
    command(&mut e, "Split").unwrap();
    command(&mut e, "Point w0,0,0").unwrap();
    assert!(e.split.is_none());
    assert_eq!(e.scene.points().len(), 1);
}
