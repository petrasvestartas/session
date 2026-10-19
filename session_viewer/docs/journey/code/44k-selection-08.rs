use crate::{editor::{Action, Editor}, pick_sources::{Mode, Part, Target}};

#[test]
fn selecting_a_part_does_not_edit_history_or_implicitly_delete_its_parent() {
    let mut editor = Editor::default(); editor.apply(Action::Close).unwrap(); editor.apply(Action::AddBox).unwrap();
    let owner = editor.scene.objects()[0].id; let source = std::rc::Rc::clone(editor.scene.objects()[0].geometry().unwrap());
    let target = Target { owner, part: Part::Face(0) };
    editor.apply(Action::PickMode(Mode::Face)).unwrap(); editor.apply(Action::PickedTarget(Some(target))).unwrap();
    assert_eq!(editor.selection(), Some(target)); assert_eq!(editor.whole_selection(), None);
    assert!(editor.apply(Action::Translate([1., 0., 0.])).is_err());
    assert!(editor.apply(Action::Delete).is_err()); assert_eq!(editor.scene.objects().len(), 1);
    assert!(std::rc::Rc::ptr_eq(&source, &editor.scene.objects()[0].geometry().unwrap()));
    editor.apply(Action::Undo).unwrap(); assert!(editor.scene.objects().is_empty()); assert_eq!(editor.selection(), None);
    editor.apply(Action::Redo).unwrap(); assert_eq!(editor.scene.objects()[0].id, owner);
    editor.apply(Action::PickedTarget(Some(target))).unwrap(); editor.apply(Action::PickMode(Mode::Object)).unwrap();
    assert_eq!(editor.selection(), None); editor.apply(Action::Picked(Some(owner))).unwrap();
    editor.apply(Action::Delete).unwrap(); assert!(editor.scene.objects().is_empty()); editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects()[0].id, owner);
}

#[test]
fn hidden_controls_closed_owners_and_invalid_source_addresses_are_rejected() {
    let mut editor = Editor::default(); editor.apply(Action::Close).unwrap(); editor.apply(Action::AddCurve).unwrap();
    editor.apply(Action::Controls(true)).unwrap(); let owner = editor.scene.paths()[0].id;
    let control = Target { owner, part: Part::Control(1) };
    editor.apply(Action::PickedTarget(Some(control))).unwrap(); assert_eq!(editor.selection(), Some(control));
    editor.apply(Action::Controls(false)).unwrap(); assert_eq!(editor.selection(), None);
    editor.apply(Action::PickedTarget(Some(control))).unwrap(); assert_eq!(editor.selection(), None);
    editor.apply(Action::AddPolyline).unwrap(); let owner = editor.scene.paths()[1].id;
    for part in [Part::PathEdge(usize::MAX), Part::Face(usize::MAX), Part::Control(usize::MAX)] {
        editor.apply(Action::PickedTarget(Some(Target { owner, part }))).unwrap(); assert_eq!(editor.selection(), None);
    }
    editor.apply(Action::Close).unwrap(); assert!(!control.valid(&editor.scene));
}
