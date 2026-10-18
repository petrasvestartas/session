use crate::{colour::Mode, editor::{Action, Change, Editor}};

#[test]
fn normal_view_changes_without_consuming_source_history() {
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    editor.apply(Action::AddColours(Mode::Faces)).unwrap();
    let id = editor.scene.objects()[0].id;
    let original = editor.scene.objects()[0].geometry().unwrap().to_proto();
    assert_eq!(editor.apply(Action::NormalView(true)).unwrap(), Change::View);
    assert_eq!(editor.scene.objects()[0].geometry().unwrap().to_proto(), original);
    editor.apply(Action::Undo).unwrap();
    assert!(editor.scene.objects().is_empty());
    assert!(editor.normal_view);
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.objects()[0].id, id);
    assert_eq!(editor.scene.objects()[0].geometry().unwrap().to_proto(), original);
    assert_eq!(editor.apply(Action::NormalView(false)).unwrap(), Change::View);
    assert!(!editor.normal_view);
}
