use crate::{editor::{Action, Editor}, memory};

#[test]
fn point_command_preserves_source_identity_in_one_history_step() {
    let mut editor = Editor::default(); editor.apply(Action::Close).unwrap();
    editor.apply(Action::AddPoint).unwrap(); let id = editor.scene.points()[0].id;
    let row = &editor.scene.points()[0]; let weak = std::rc::Rc::downgrade(&row.prepared.source);
    assert_eq!(row.prepared.source.width, 12.0); assert_eq!(row.prepared.source.pointcolor.to_f32(), [0.0, 0.0, 0.0, 0.5]);
    assert_eq!(row.guid, row.prepared.source.guid());
    editor.apply(Action::Zoom(2.0)).unwrap(); editor.apply(Action::Undo).unwrap();
    assert!(editor.scene.points().is_empty()); assert!(weak.upgrade().is_some());
    editor.apply(Action::Redo).unwrap(); assert_eq!(editor.scene.points()[0].id, id);
    assert!(crate::document::snapshot(&editor.scene).unwrap_err().contains("cannot save point objects yet"));
    editor.apply(Action::Close).unwrap(); assert!(weak.upgrade().is_none()); assert_eq!(memory::points(editor.scenes()), [0; 3]);
}
