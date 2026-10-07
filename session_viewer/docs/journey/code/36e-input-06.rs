use crate::{editor::{Action, Editor}, chain::Source, memory};

#[test]
fn connected_examples_are_original_sources_in_single_history_steps() {
    let mut editor = Editor::default(); editor.apply(Action::Close).unwrap();
    editor.apply(Action::AddPolyline).unwrap(); let first = editor.scene.paths()[0].id;
    assert!(matches!(editor.scene.paths()[0].prepared.source, Source::Polyline(_)));
    editor.apply(Action::AddArrow).unwrap(); let second = editor.scene.paths()[1].id;
    let Source::Line(line) = &editor.scene.paths()[1].prepared.source else { panic!("Original arrow line") };
    assert_eq!(line.arrowhead, session_rust::Arrowhead::BOTH);
    let weak = std::rc::Rc::downgrade(line);
    editor.apply(Action::Zoom(2.0)).unwrap(); editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.paths().len(), 1); assert!(weak.upgrade().is_some());
    editor.apply(Action::Redo).unwrap(); assert_eq!(editor.scene.paths()[1].id, second);
    editor.apply(Action::Undo).unwrap(); editor.apply(Action::Undo).unwrap(); assert!(editor.scene.paths().is_empty());
    editor.apply(Action::Redo).unwrap(); assert_eq!(editor.scene.paths()[0].id, first);
    editor.apply(Action::Close).unwrap(); assert!(weak.upgrade().is_none()); assert_eq!(memory::paths(editor.scenes()), [0; 5]);
}
