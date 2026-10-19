use crate::{
    editor::{Action, Editor},
    scene::Scene,
};
use session_rust::{Geometry, Point, Xform};
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
            return Err("Editor command".into());
        };
        a
    };
    e.apply(action).map_err(|message| {
        if message == "Shape input invalid" {
            e.tool_status.clone()
        } else {
            message.into()
        }
    })?;
    Ok(())
}
fn end_screen(e: &Editor, id: crate::scene::ObjectId, end: crate::extend_reach::End) -> [f32; 2] {
    let source = crate::source_edit::original(&e.scene, id).unwrap();
    let (tip, _) = crate::extend_reach::tip(&source, end).unwrap();
    let p = tip.transformed(&e.scene.placement(id).unwrap());
    let q = e.camera.view_projection().transform_point(&p);
    [q[0] as f32, q[1] as f32]
}
#[test]
fn projection_edits_original_basic_lanes_skips_cad_preserves_identity_and_one_undo() {
    let mut e = editor();
    for line in [
        "Point w0,0,2",
        "Line w0,0,1 w2,1,3",
        "Polyline w0,0,1 w1,0,2 w1,1,3",
        "Nurbs Curve Arc Center w3,0,1 0.5 270",
        "Box w5,0,0 1 1 1",
    ] {
        command(&mut e, line).unwrap()
    }
    let mesh = crate::prepared::PreparedMesh::new(session_rust::Mesh::from_vertices_and_faces(
        vec![
            Point::new(0., 0., 1.),
            Point::new(1., 0., 2.),
            Point::new(0., 1., 3.),
        ],
        vec![vec![0, 1, 2]],
    ))
    .unwrap();
    let mesh_id = e.scene.insert(mesh).unwrap();
    let line_id = e.scene.lines()[0].id;
    let source = e.scene.lines()[0].prepared.source.clone();
    let guid = source.guid().to_owned();
    let row_guid = e.scene.lines()[0].guid.clone();
    let cad = e.scene.objects()[0].cad.as_ref().unwrap().source.clone();
    e.scene
        .place_following(line_id, Xform::translation(10., 0., 2.))
        .unwrap();
    command(&mut e, "Select All").unwrap();
    command(&mut e, "Project To Plane XY").unwrap();
    assert!(e.tool_status.contains("1 unsupported"));
    assert_eq!(e.scene.lines()[0].guid, row_guid);
    assert_eq!(e.scene.lines()[0].prepared.source.guid(), guid);
    assert_eq!(e.scene.lines()[0].model.m[12], 10.);
    assert!(
        e.scene.lines()[0]
            .endpoints()
            .iter()
            .all(|p| p[2].abs() < 1e-12)
    );
    assert_eq!(source.start()[2], 1.);
    assert!(Rc::ptr_eq(
        &cad,
        &e.scene.objects()[0].cad.as_ref().unwrap().source
    ));
    let Geometry::Mesh(m) = crate::source_edit::original(&e.scene, mesh_id).unwrap() else {
        panic!()
    };
    assert!(m.vertex.values().all(|v| v.z.abs() < 1e-12));
    let bytes = crate::scene_archive::save(&e.scene).unwrap();
    let mut reopened = Scene::empty();
    crate::scene_document::open(&mut reopened, &bytes).unwrap();
    assert!(
        reopened.lines()[0]
            .endpoints()
            .iter()
            .all(|p| p[2].abs() < 1e-12)
    );
    command(&mut e, "Undo").unwrap();
    assert!(Rc::ptr_eq(&source, &e.scene.lines()[0].prepared.source));
    assert_eq!(e.scene.points()[0].prepared.source[2], 2.);
    command(&mut e, "Redo").unwrap();
    assert!(!Rc::ptr_eq(&source, &e.scene.lines()[0].prepared.source));
}
#[test]
fn three_point_projection_retries_bad_second_and_third_points_without_adopting_input() {
    let mut e = editor();
    command(&mut e, "Line w0,0,1 w1,1,2").unwrap();
    let id = e.scene.lines()[0].id;
    e.select_objects([id], false);
    command(&mut e, "Project To Plane 3Point").unwrap();
    command(&mut e, "w0,0,0").unwrap();
    assert!(command(&mut e, "w0,0,0").is_err());
    assert_eq!(e.draft.as_ref().unwrap().points.len(), 1);
    command(&mut e, "w1,0,0").unwrap();
    e.apply(Action::ToolHover(Some([0., 1., 0.]))).unwrap();
    assert!(command(&mut e, "w2,0,0").is_err());
    assert_eq!(e.draft.as_ref().unwrap().points.len(), 2);
    assert_eq!(e.scene.lines()[0].prepared.source.start()[2], 1.);
    command(&mut e, "w0,1,0").unwrap();
    assert!(e.draft.is_none());
    assert!(
        e.scene.lines()[0]
            .endpoints()
            .iter()
            .all(|p| p[2].abs() < 1e-12)
    );
    command(&mut e, "Undo").unwrap();
    assert_eq!(e.scene.lines()[0].prepared.source.start()[2], 1.);
}
#[test]
fn collapsed_or_unsupported_projection_preserves_selection_redo_and_previous_owner() {
    let mut e = editor();
    command(&mut e, "Line w0,0,0 w0,0,1").unwrap();
    command(&mut e, "Point w3,0,0").unwrap();
    command(&mut e, "Undo").unwrap();
    let id = e.scene.lines()[0].id;
    e.select_objects([id], false);
    command(&mut e, "Curve").unwrap();
    command(&mut e, "w0,0,0").unwrap();
    assert!(command(&mut e, "Project To Plane XY").is_err());
    assert_eq!(e.draft.as_ref().unwrap().tool.name(), "Curve");
    assert_eq!(e.draft.as_ref().unwrap().points.len(), 1);
    assert_eq!(e.selected_objects(), vec![id]);
    command(&mut e, "Escape").unwrap();
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.points().len(), 1);
    assert!(command(&mut e, "Project To Plane 3Point w0,0,0 w1,0,0").is_err());
}
#[test]
fn extend_world_distance_handles_scaled_sources_keeps_identity_and_repeats_transactions() {
    let mut e = editor();
    command(&mut e, "Line w0,0,0 w1,0,0").unwrap();
    let id = e.scene.lines()[0].id;
    let source = e.scene.lines()[0].prepared.source.clone();
    let guid = source.guid().to_owned();
    e.scene
        .place_following(id, Xform::scale_xyz(1000., 1000., 1000.))
        .unwrap();
    command(&mut e, "View Reset").unwrap();
    command(&mut e, "Fit").unwrap();
    command(&mut e, "Extend Distance 25").unwrap();
    for _ in 0..2 {
        let screen = end_screen(&e, id, crate::extend_reach::End::End);
        e.apply(Action::ToolScreen {
            screen,
            size: [640., 480.],
            commit: false,
        })
        .unwrap();
        assert!(
            e.draft
                .as_ref()
                .unwrap()
                .extend
                .as_ref()
                .unwrap()
                .hover
                .is_some()
        );
        e.apply(Action::ToolScreen {
            screen,
            size: [640., 480.],
            commit: true,
        })
        .unwrap();
    }
    assert!((e.scene.lines()[0].endpoints()[1][0] - 1050.).abs() < 1e-7);
    assert_eq!(e.scene.lines()[0].prepared.source.guid(), guid);
    assert_eq!(source.end()[0], 1.);
    command(&mut e, "").unwrap();
    assert!(e.draft.is_none());
    command(&mut e, "Undo").unwrap();
    assert!((e.scene.lines()[0].endpoints()[1][0] - 1025.).abs() < 1e-7);
    command(&mut e, "Undo").unwrap();
    assert!(Rc::ptr_eq(&source, &e.scene.lines()[0].prepared.source));
    command(&mut e, "Redo").unwrap();
    assert!((e.scene.lines()[0].endpoints()[1][0] - 1025.).abs() < 1e-7);
}
#[test]
fn extend_boundary_picks_stay_separate_and_original_end_reaches_nearest_world_boundary() {
    let mut e = editor();
    for line in [
        "Line w0,0,0 w1,0,0",
        "Line w3,-1,0 w3,1,0",
        "Line w5,-1,0 w5,1,0",
        "View Reset",
        "Fit",
    ] {
        command(&mut e, line).unwrap()
    }
    let (id, near, far) = (
        e.scene.lines()[0].id,
        e.scene.lines()[1].id,
        e.scene.lines()[2].id,
    );
    e.select_objects([near, far], false);
    command(&mut e, "Extend").unwrap();
    assert!(e.selected_objects().is_empty());
    assert!(e.editing_boundaries());
    assert_eq!(e.highlighted_objects(), vec![near, far]);
    command(&mut e, "").unwrap();
    let screen = end_screen(&e, id, crate::extend_reach::End::End);
    e.apply(Action::ToolScreen {
        screen,
        size: [640., 480.],
        commit: true,
    })
    .unwrap();
    assert!((e.scene.lines()[0].endpoints()[1][0] - 3.).abs() < 1e-6);
    command(&mut e, "Escape").unwrap();
    assert_eq!(e.selected_objects(), vec![near, far]);
    command(&mut e, "Undo").unwrap();
    assert_eq!(e.scene.lines()[0].endpoints()[1][0], 1.);
}
#[test]
fn normalized_extension_ranges_preserve_original_style_and_reject_invalid_input_atomically() {
    let mut e = editor();
    command(&mut e, "Line w0,0,0 w10,0,0").unwrap();
    let id = e.scene.lines()[0].id;
    let original = e.scene.lines()[0].prepared.source.clone();
    e.select_objects([id], false);
    command(&mut e, "Extend -0.5 1.5").unwrap();
    assert_eq!(e.scene.lines()[0].prepared.source.start()[0], -5.);
    assert_eq!(e.scene.lines()[0].prepared.source.end()[0], 15.);
    assert_eq!(
        e.scene.lines()[0].prepared.source.linecolor,
        original.linecolor
    );
    command(&mut e, "Undo").unwrap();
    command(&mut e, "Extend Distance").unwrap();
    assert!(command(&mut e, "0").is_err());
    assert!(e.draft.as_ref().unwrap().extend.as_ref().unwrap().asking);
    assert!(command(&mut e, "-1").is_err());
    command(&mut e, "2").unwrap();
    assert!(e.draft.as_ref().unwrap().extend.as_ref().unwrap().curves);
    command(&mut e, "Escape").unwrap();
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.lines()[0].prepared.source.end()[0], 15.);
    assert!(command(&mut e, "Extend 0.1 1.5").is_err());
    assert_eq!(e.scene.lines()[0].prepared.source.end()[0], 15.);
}
#[test]
fn stale_extension_sources_and_missed_end_clicks_never_create_history() {
    let mut e = editor();
    command(&mut e, "Line w0,0,0 w1,0,0").unwrap();
    command(&mut e, "Point w2,0,0").unwrap();
    command(&mut e, "Undo").unwrap();
    let id = e.scene.lines()[0].id;
    command(&mut e, "View Reset").unwrap();
    command(&mut e, "Fit").unwrap();
    command(&mut e, "Extend 2").unwrap();
    assert!(
        e.apply(Action::ToolScreen {
            screen: [0.9, 0.9],
            size: [640., 480.],
            commit: true
        })
        .is_err()
    );
    let screen = end_screen(&e, id, crate::extend_reach::End::End);
    e.scene
        .place_following(id, Xform::translation(1., 0., 0.))
        .unwrap();
    assert!(
        e.apply(Action::ToolScreen {
            screen,
            size: [640., 480.],
            commit: true
        })
        .is_err()
    );
    assert_eq!(e.scene.lines()[0].prepared.source.end()[0], 1.);
    command(&mut e, "Escape").unwrap();
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.points().len(), 1);
}
