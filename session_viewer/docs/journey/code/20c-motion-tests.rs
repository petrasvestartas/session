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
