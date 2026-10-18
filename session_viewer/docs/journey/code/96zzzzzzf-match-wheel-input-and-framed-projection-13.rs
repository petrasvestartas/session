use crate::{editor::{Action, Change, Editor}, navigation::wheel};

fn distance(delta: f64, mode: u32) -> f64 {
    let mut editor = crate::camera_defaults_tests::editor_fixture();
    assert_eq!(editor.apply(wheel(delta, mode, 1.0).unwrap()).unwrap(), Change::View);
    editor.camera.distance
}

#[test]
fn wheel_units_agree_and_small_movements_accumulate() {
    assert!((distance(300.0, 0) - distance(3.0, 1)).abs() < 1.0e-6);
    assert!(wheel(0.1, 2, 1.0).is_none());
    let mut editor = crate::camera_defaults_tests::editor_fixture();
    for _ in 0..10 {
        editor.apply(wheel(-12.0, 0, 1.0).unwrap()).unwrap();
    }
    assert!((editor.camera.distance - distance(-120.0, 0)).abs() < 1.0e-5);
    editor.apply(wheel(120.0, 0, 1.0).unwrap()).unwrap();
    assert!((editor.camera.distance - 3.0).abs() < 1.0e-5);
}

#[test]
fn wheel_rejects_invalid_input_and_bounds_large_jumps() {
    for delta in [0.0, f64::NAN, f64::INFINITY] {
        assert!(wheel(delta, 0, 1.0).is_none());
    }
    for height in [0.0, -1.0, f64::NAN] {
        assert!(wheel(10.0, 0, height).is_none());
    }
    assert!(wheel(10.0, 99, 1.0).is_none());
    assert!(wheel(f64::MAX, 0, 1.0).is_none());
    assert_eq!(distance(1e9, 0), distance(1_000.0, 0));
    assert!(wheel(-f64::MAX, 0, 1.0).is_none());
    assert_eq!(distance(-1e9, 0), distance(-1_000.0, 0));
}

#[test]
fn navigation_preserves_document_history() {
    let mut editor = crate::camera_defaults_tests::editor_fixture();
    editor.apply(Action::AddBox).unwrap();
    editor.apply(wheel(-120.0, 0, 1.0).unwrap()).unwrap();
    let zoomed = editor.camera.distance;
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects().len(), 2);
    assert_eq!(editor.camera.distance, zoomed);
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.objects().len(), 3);
    editor.apply(Action::SelectNext).unwrap();
    let selected = editor.selected.unwrap();
    editor.apply(Action::Delete).unwrap();
    assert!(!editor.scene.contains(selected));
    editor.apply(Action::Undo).unwrap();
    assert!(editor.scene.contains(selected));
}
