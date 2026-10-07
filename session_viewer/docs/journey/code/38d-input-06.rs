use crate::{editor::{Action, Editor}, chain::Source, memory};

#[test]
fn curve_command_keeps_original_identity_and_control_visibility_does_not_consume_undo() {
    let mut editor = Editor::default(); editor.apply(Action::Close).unwrap(); editor.apply(Action::AddCurve).unwrap();
    let id = editor.scene.paths()[0].id; let Source::Curve(source, _) = &editor.scene.paths()[0].prepared.source else { panic!("Original curve") };
    let weak = std::rc::Rc::downgrade(source); assert_eq!(source.m_order, 3); assert_eq!(source.width, 5.0);
    assert!(editor.scene.control_markers().is_empty()); editor.apply(Action::Controls(true)).unwrap();
    assert_eq!(editor.scene.control_markers().len(), 3); editor.apply(Action::Controls(false)).unwrap();
    editor.apply(Action::Controls(true)).unwrap(); editor.apply(Action::Undo).unwrap(); assert!(editor.scene.paths().is_empty());
    assert!(weak.upgrade().is_some()); editor.apply(Action::Redo).unwrap(); assert_eq!(editor.scene.paths()[0].id, id);
    assert_eq!(editor.scene.control_markers().len(), 3); editor.apply(Action::Close).unwrap();
    assert!(weak.upgrade().is_none()); assert_eq!(memory::curves(editor.scenes()), [0; 4]);
}
