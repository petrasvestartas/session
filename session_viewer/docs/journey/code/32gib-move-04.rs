use crate::{edit_intent::Intent, editor::{Action, Change, Editor}};
use session_rust::Xform;

#[test]
fn captured_move_uses_current_placement_and_preserves_later_selection() {
    let mut editor = Editor::default(); let id = editor.scene.objects()[0].id;
    let intent = Intent::capture(&Action::Translate([0.125, -0.25, 0.5]), Some(id)).unwrap().unwrap();
    let before = Xform::translation(2.0, 3.0, 4.0);
    editor.scene.place(id, before.clone()).unwrap();
    let later = editor.scene.objects()[1].id; editor.selected = Some(later);
    editor.apply(Action::Orbit(0.2, 0.1)).unwrap(); let camera = editor.camera.uniform();
    let Intent::Move { id, offset } = intent else { panic!("Move intent") };
    assert_eq!(editor.move_object(id, offset).unwrap(), Change::Scene);
    let expected = Xform::translation(2.125, 2.75, 4.5);
    assert_eq!(editor.scene.objects()[0].model.m, expected.m);
    assert_eq!(editor.selected, Some(later)); assert_eq!(editor.camera.uniform(), camera);
    editor.apply(Action::Undo).unwrap(); assert_eq!(editor.scene.objects()[0].model.m, before.m);
    editor.apply(Action::Undo).unwrap(); assert_eq!(editor.scene.objects()[0].model.m, before.m);
    editor.apply(Action::Redo).unwrap(); assert_eq!(editor.scene.objects()[0].model.m, expected.m);
    assert_eq!(editor.selected, Some(later));
}

#[test]
fn no_op_and_refused_moves_keep_redo() {
    let mut editor = Editor::default(); let id = editor.scene.objects()[0].id;
    editor.apply(Action::AddBox).unwrap(); editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.move_object(id, [0.0; 3]).unwrap(), Change::View);
    assert!(editor.move_object(id, [f64::NAN, 0.0, 0.0]).is_err());
    editor.apply(Action::Redo).unwrap(); assert_eq!(editor.scene.objects().len(), 3);
    editor.scene.remove(id);
    assert_eq!(editor.move_object(id, [1.0, 0.0, 0.0]), Err("Object not found"));
}
