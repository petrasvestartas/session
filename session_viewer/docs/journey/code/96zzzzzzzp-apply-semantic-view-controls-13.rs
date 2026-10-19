use crate::{
    commands::Command,
    editor::{Action, Change, Editor},
};
fn apply(editor: &mut Editor, text: &str) -> Change {
    let Command::Editor(action) = crate::commands::parse(text).unwrap() else {
        panic!("Expected semantic editor command")
    };
    editor.apply(action).unwrap()
}
fn documents(editor: &Editor) -> Vec<session_rust::proto::Session> {
    use prost::Message;
    crate::archive::Bundle::decode(&crate::scene_archive::save(&editor.scene).unwrap())
        .unwrap()
        .documents.into_iter()
        .map(|bytes| session_rust::proto::Session::decode(bytes.as_slice()).unwrap())
        .collect()
}
#[test]
fn semantic_family_commands_preserve_active_draft_original_sources_camera_and_history() {
    let mut e = Editor::default();
    e.scene = crate::scene::Scene::empty();
    e.scene
        .insert(
            crate::prepared::PreparedMesh::triangle(
                [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
                [0.2, 0.4, 0.6],
            )
            .unwrap(),
        )
        .unwrap();
    apply(&mut e, "Line");
    assert!(e.draft.is_some());
    let history = e.scenes().count();
    let original = documents(&e);
    let camera = e.camera.uniform();
    let source = e.scene.objects()[0].mesh.clone();
    for (text, expected) in [
        ("View Hide Edges", [false, true, true]),
        ("View Hide Lines", [false, false, true]),
        ("View Hide Points", [false, false, false]),
        ("View Show Edges", [true, false, false]),
        ("View Show Lines", [true, true, false]),
        ("View Show Points", [true, true, true]),
    ] {
        assert!(
            e.tool_line(text).is_none(),
            "A view command must reach the semantic registry while drawing"
        );
        assert_eq!(apply(&mut e, text), Change::View);
        assert_eq!(
            [
                e.ink_visibility.edges,
                e.ink_visibility.lines,
                e.ink_visibility.points
            ],
            expected
        );
        assert!(e.draft.is_some());
        assert_eq!(e.scenes().count(), history);
        assert_eq!(e.camera.uniform(), camera);
        assert_eq!(documents(&e), original);
        assert!(std::rc::Rc::ptr_eq(&source, &e.scene.objects()[0].mesh));
    }
    for text in [
        "View Hide Edges Off",
        "View Show Lines extra",
        "View Hide Points 1",
    ] {
        assert!(crate::commands::parse(text).is_err());
    }
    assert_eq!(e.scenes().count(), history);
    assert!(e.draft.is_some());
}
#[test]
fn xray_on_off_and_toggle_preserve_nonzero_opacity_until_an_actual_switch() {
    let mut e = Editor::default();
    e.scene = crate::scene::Scene::empty();
    e.opacity = 0.95;
    let history = e.scenes().count();
    let camera = e.camera.uniform();
    for (text, opacity) in [
        ("View Xray Off", 0.95),
        ("View Xray On", 0.),
        ("View Xray On", 0.),
        ("View Xray Off", 1.),
        ("View Xray Off", 1.),
        ("View Xray", 0.),
        ("View Xray", 1.),
    ] {
        assert_eq!(apply(&mut e, text), Change::View);
        assert_eq!(e.opacity, opacity);
        assert_eq!(e.scenes().count(), history);
        assert_eq!(e.camera.uniform(), camera);
        assert!(e.scene.all_ids().is_empty());
    }
    assert!(crate::commands::parse("View Xray invalid").is_err());
    assert_eq!(e.opacity, 1.);
}
