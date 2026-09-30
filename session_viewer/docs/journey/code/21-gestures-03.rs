use crate::{editor::{Action, Change, Editor}, gesture::{Gesture, Motion}};

#[test]
fn a_click_selects_but_a_left_drag_does_not() {
    let mut input = Gesture::default();
    assert!(input.press(1, 0, [10.0, 20.0]));
    assert_eq!(input.release(1, [12.0, 20.0]), Some(Motion::Pick([12.0, 20.0])));
    input.press(1, 0, [10.0, 20.0]);
    assert_eq!(input.move_to(1, [20.0, 20.0]), None);
    assert_eq!(input.release(1, [10.0, 20.0]), None);
    input.press(1, 0, [10.0, 20.0]);
    assert_eq!(input.release(1, [30.0, 20.0]), None);
}

#[test]
fn only_the_captured_pointer_can_move_or_finish_the_gesture() {
    let mut input = Gesture::default();
    assert!(input.press(1, 2, [0.0, 0.0]));
    assert!(!input.press(2, 0, [0.0, 0.0]));
    assert_eq!(input.move_to(2, [100.0, 50.0]), None);
    assert_eq!(input.release(2, [0.0, 0.0]), None);
    input.cancel_pointer(2);
    assert_eq!(input.move_to(1, [10.0, 5.0]), Some(Motion::Orbit([10.0, 5.0])));
    assert_eq!(input.move_to(1, [30.0, 8.0]), Some(Motion::Orbit([20.0, 3.0])));
    assert_eq!(input.release(1, [30.0, 8.0]), None);
    assert_eq!(input.move_to(1, [40.0, 8.0]), None);
    input.press(3, 2, [0.0, 0.0]);
    assert_eq!(input.release(3, [5.0, 2.0]), Some(Motion::Orbit([5.0, 2.0])));
}

#[test]
fn cancellation_never_selects_and_allows_a_fresh_press() {
    let mut input = Gesture::default();
    input.press(1, 0, [0.0, 0.0]);
    input.cancel_pointer(1);
    assert_eq!(input.release(1, [0.0, 0.0]), None);
    assert!(input.press(2, 2, [0.0, 0.0]));
    input.cancel();
    assert_eq!(input.move_to(2, [20.0, 0.0]), None);
    assert!(!input.press(1, 1, [0.0, 0.0]));
    assert!(!input.press(1, 0, [f64::NAN, 0.0]));
}

#[test]
fn css_coordinates_reach_picking_without_a_device_pixel_ratio() {
    let Some(Action::Pick(screen)) = Motion::Pick([420.0, 290.0]).action([100.0, 50.0, 640.0, 480.0]) else {
        panic!("Expected a pick");
    };
    assert_eq!(screen, [0.0, 0.0]);
    assert!(Motion::Orbit([1.0, 1.0]).action([0.0; 4]).is_none());
}

#[test]
fn a_drag_changes_the_view_without_editing_the_document() {
    let mut editor = Editor::default();
    editor.apply(Action::AddBox).unwrap();
    let before = editor.camera.uniform();
    let mut input = Gesture::default();
    input.press(1, 2, [0.0, 0.0]);
    let action = input.move_to(1, [100.0, 50.0]).unwrap().action([0.0, 0.0, 640.0, 480.0]).unwrap();
    assert_eq!(editor.apply(action).unwrap(), Change::View);
    assert_ne!(editor.camera.uniform(), before);
    let moved = editor.camera.uniform();
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects().len(), 2);
    assert_eq!(editor.camera.uniform(), moved);
}
