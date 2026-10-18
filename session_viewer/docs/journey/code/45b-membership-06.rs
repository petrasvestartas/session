use crate::{editor::{Action, Editor}, pick_sources::{Mode, Part, Target}};
use std::rc::Rc;

#[test]
fn additive_hits_preserve_pick_order_and_exclusive_parts_cannot_mix_with_objects() {
    let mut editor = Editor::default(); let ids: Vec<_> = editor.scene.objects().iter().map(|r| r.id).collect();
    let hit = |id| Some(Target { owner: id, part: Part::Object });
    editor.apply(Action::PickedMany { target: hit(ids[1]), additive: false }).unwrap();
    editor.apply(Action::PickedMany { target: hit(ids[0]), additive: true }).unwrap();
    editor.apply(Action::PickedMany { target: hit(ids[1]), additive: true }).unwrap();
    editor.apply(Action::PickedMany { target: None, additive: true }).unwrap();
    assert_eq!(editor.selected_objects(), vec![ids[1], ids[0]]); assert_eq!(editor.selected, Some(ids[0]));
    editor.apply(Action::FitSelected).unwrap(); let bounds = editor.scene.bounds_of(&ids).unwrap();
    assert_eq!(editor.camera.target, bounds.centre());
    editor.apply(Action::PickMode(Mode::Face)).unwrap();
    let face = Target { owner: ids[1], part: Part::Face(0) };
    editor.apply(Action::PickedMany { target: Some(face), additive: true }).unwrap();
    assert_eq!(editor.selection(), Some(face)); assert!(editor.selected_objects().is_empty());
    editor.apply(Action::PickMode(Mode::Object)).unwrap();
    editor.apply(Action::PickedMany { target: None, additive: false }).unwrap(); assert!(editor.selected_objects().is_empty());
}

#[test]
fn one_move_and_delete_transaction_cover_mixed_original_rows_without_copying_sources() {
    let mut editor = Editor::default(); editor.apply(Action::Close).unwrap();
    for action in [Action::AddBox, Action::AddLine, Action::AddPoint, Action::AddPolyline] { editor.apply(action).unwrap(); }
    let mesh = Rc::clone(editor.scene.objects()[0].geometry().unwrap()); let line = Rc::clone(&editor.scene.lines()[0].prepared.source);
    let point = Rc::clone(&editor.scene.points()[0].prepared.source);
    editor.apply(Action::SelectAll).unwrap(); let ids = editor.selected_objects(); assert_eq!(ids.len(), 4);
    let bounds = editor.scene.bounds_of(&ids).unwrap();
    editor.apply(Action::Translate([2., 3., 4.])).unwrap(); let moved = editor.scene.bounds_of(&ids).unwrap();
    for i in 0..3 { assert!((moved.min[i] - bounds.min[i] - [2., 3., 4.][i]).abs() < 1e-12); }
    assert!(Rc::ptr_eq(&mesh, editor.scene.objects()[0].geometry().unwrap()));
    assert!(Rc::ptr_eq(&line, &editor.scene.lines()[0].prepared.source)); assert!(Rc::ptr_eq(&point, &editor.scene.points()[0].prepared.source));
    editor.apply(Action::Undo).unwrap(); assert_eq!(editor.scene.bounds_of(&ids).unwrap().min, bounds.min);
    editor.apply(Action::Redo).unwrap(); assert_eq!(editor.scene.bounds_of(&ids).unwrap().min, moved.min);
    editor.apply(Action::Delete).unwrap(); assert!(editor.scene.bounds().is_none()); assert!(editor.selected_objects().is_empty());
    editor.apply(Action::Undo).unwrap(); assert_eq!(editor.scene.bounds_of(&ids).unwrap().min, moved.min);
    editor.apply(Action::Redo).unwrap(); assert!(editor.scene.bounds().is_none());
}

#[test]
fn an_invalid_batch_rolls_back_earlier_moves_and_preserves_redo() {
    let mut editor = Editor::default(); editor.apply(Action::Close).unwrap(); editor.apply(Action::AddBox).unwrap();
    let hot = editor.scene.objects()[0].id; let old = editor.scene.objects()[0].model.m;
    editor.apply(Action::ImportAt(crate::specimen::bytes(), Rc::new(crate::reload_url::ReloadUrl::new("test:multi".into())))).unwrap();
    editor.apply(Action::Undo).unwrap(); editor.apply(Action::Redo).unwrap();
    let cold = editor.scene.objects()[1].id; editor.apply(Action::UnloadSources).unwrap();
    editor.select_objects([hot, cold], false); assert!(editor.apply(Action::Translate([1., 0., 0.])).is_err());
    assert_eq!(editor.scene.objects()[0].model.m, old); assert_eq!(editor.selected_objects(), vec![hot, cold]);
    assert!(editor.apply(Action::Delete).is_err()); assert_eq!(editor.scene.objects().len(), 4);
    editor.apply(Action::Undo).unwrap(); assert_eq!(editor.scene.objects().len(), 1, "Refused batch adds no history entry");
    editor.apply(Action::Redo).unwrap(); assert_eq!(editor.scene.objects().len(), 4);
}

#[test]
fn select_all_filters_locked_and_hidden_rows_and_close_retains_only_ids() {
    let mut editor = Editor::default(); editor.apply(Action::AddPoint).unwrap();
    let weak = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
    Rc::make_mut(&mut editor.scene.objects_mut()[0].metadata).is_locked = true;
    Rc::make_mut(&mut editor.scene.objects_mut()[1].metadata).is_visible = false;
    editor.apply(Action::SelectAll).unwrap(); assert_eq!(editor.selected_objects(), vec![editor.scene.points()[0].id]);
    editor.apply(Action::Close).unwrap(); assert!(weak.upgrade().is_none()); assert!(editor.selected_objects().is_empty());
}
