use crate::{editor::{Action, Change, Editor}, offset};
use session_rust::Xform;

#[test]
fn offsets_require_three_finite_numbers() {
    assert_eq!(offset::parse("Move 1, -2, 3e-2").unwrap(), [1.0, -2.0, 0.03]);
    for text in ["Move", "Move 1,2", "Move 1,,2", "Move 1 2,3,4", "Move 1,2,3,4", "Move NaN,0,0", "Move inf,0,0", "Other 1,2,3"] {
        assert!(offset::parse(text).is_err(), "{text}");
    }
}

#[test]
fn world_move_keeps_local_mesh_and_history() {
    let mut editor = Editor::default();
    editor.apply(Action::SelectNext).unwrap();
    let id = editor.selected.unwrap();
    let mut initial = Xform::identity();
    initial.m[0] = 2.0;
    let initial_values = initial.m;
    editor.scene.place(id, initial).unwrap();
    let mesh = editor.scene.objects()[0].mesh.clone();
    let vertex = mesh.vertices()[0];
    let before = editor.scene.objects()[0].world_point(vertex);
    let camera = editor.camera.uniform();
    assert_eq!(editor.apply(Action::Translate([3.0, -2.0, 1.0])).unwrap(), Change::Scene);
    let object = &editor.scene.objects()[0];
    let moved = object.model.m;
    let after = object.world_point(vertex);
    assert_eq!(after, [before[0] + 3.0, before[1] - 2.0, before[2] + 1.0]);
    assert_eq!(object.mesh.vertices()[0], vertex);
    assert!(std::rc::Rc::ptr_eq(&mesh, &object.mesh));
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects()[0].model.m, initial_values);
    assert!(editor.apply(Action::Translate([f64::NAN, 0.0, 0.0])).is_err());
    editor.apply(Action::Translate([0.0; 3])).unwrap();
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.objects()[0].model.m, moved);
    assert_eq!(editor.camera.uniform(), camera);
    assert_eq!(editor.selected, Some(id));
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects()[0].model.m, initial_values);
}

#[test]
fn no_selection_cannot_move_an_arbitrary_object() {
    let mut editor = Editor::default();
    let before = editor.scene.objects()[0].model.m;
    assert!(editor.apply(Action::Translate([1.0, 0.0, 0.0])).is_err());
    assert_eq!(editor.scene.objects()[0].model.m, before);
}
