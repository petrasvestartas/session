
#[test]
fn a_click_selects_but_a_left_drag_starts_a_probe() {
    let mut input = Gesture::default();
    assert!(input.press(1, 0, [10.0, 20.0]));
    assert_eq!(input.release(1, [12.0, 20.0]), Some(Motion::Pick([12.0, 20.0])));
    input.press(1, 0, [10.0, 20.0]);
    assert_eq!(input.move_to(1, [20.0, 20.0]), Some(Motion::Probe {from:[10.,20.],to:[20.,20.]}));
    assert_eq!(input.release(1, [10.0, 20.0]), Some(Motion::Finish([10.,20.])));
    input.press(1, 0, [10.0, 20.0]);
    assert_eq!(input.release(1, [30.0, 20.0]), Some(Motion::Probe {from:[10.,20.],to:[30.,20.]}));
}

