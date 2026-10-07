use crate::{editor::{Action, Editor}, memory};

#[test]
fn line_command_is_one_transaction_and_close_releases_every_source() {
    let mut editor = Editor::default(); editor.apply(Action::Close).unwrap();
    editor.apply(Action::AddLine).unwrap();
    let row = &editor.scene.lines()[0]; let id = row.id;
    let weak = std::rc::Rc::downgrade(&row.prepared.source);
    assert_eq!(row.prepared.source.start()[0], -0.6);
    assert_eq!(row.prepared.display.width, 5.0);
    editor.apply(Action::Zoom(2.0)).unwrap();
    editor.apply(Action::Undo).unwrap(); assert!(editor.scene.lines().is_empty());
    assert!(weak.upgrade().is_some());
    editor.apply(Action::Redo).unwrap(); assert_eq!(editor.scene.lines()[0].id, id);
    editor.apply(Action::Undo).unwrap(); editor.apply(Action::AddLine).unwrap();
    assert_ne!(editor.scene.lines()[0].id, id); assert!(weak.upgrade().is_none());
    editor.apply(Action::Close).unwrap(); assert_eq!(memory::lines(editor.scenes()), [0; 3]);
    editor.apply(Action::Undo).unwrap(); assert!(editor.scene.lines().is_empty());
}
