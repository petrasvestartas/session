    input.cancel();
    assert_eq!(input.move_to(2, [20.0, 0.0]), None);
    assert!(input.press(1, 1, [0.0, 0.0]));
    assert_eq!(input.move_to(1,[10.,0.]),Some(Motion::Pan([10.,0.])));input.cancel();
    assert!(!input.press(1, 3, [0.0, 0.0]));
    assert!(!input.press(1, 0, [f64::NAN, 0.0]));
}
