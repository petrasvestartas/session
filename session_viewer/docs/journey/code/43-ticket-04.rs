use crate::pick_view::View;

#[test]
fn a_camera_viewport_or_density_change_invalidates_the_readback_view() {
    let transform = session_rust::Xform::identity().m.map(|n| n as f32); let base = View::new(transform, [640, 480], 1.0).unwrap();
    assert_eq!(base, View::new(transform, [640, 480], 1.0).unwrap());
    let mut orbit = transform; orbit[0] = 0.9; assert_ne!(base, View::new(orbit, [640, 480], 1.0).unwrap());
    let mut pan = transform; pan[12] = 0.01; assert_ne!(base, View::new(pan, [640, 480], 1.0).unwrap());
    assert_ne!(base, View::new(transform, [641, 480], 1.0).unwrap());
    assert_ne!(base, View::new(transform, [640, 480], 2.0).unwrap());
    let mut invalid = transform; invalid[3] = f32::NAN; assert!(View::new(invalid, [640, 480], 1.0).is_err());
    assert!(View::new(transform, [0, 480], 1.0).is_err()); assert!(View::new(transform, [640, 480], f32::INFINITY).is_err());
}
