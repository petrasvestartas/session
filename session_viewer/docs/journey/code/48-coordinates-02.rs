#[test]
fn absolute_points_require_valid_finite_coordinates() {
    assert_eq!(crate::tool_coords::parse("1, 2").unwrap(), [1., 2., 0.]);
    assert_eq!(crate::tool_coords::parse("1e-12, -2, 3").unwrap(), [1e-12, -2., 3.]);
    for text in ["1", "1,,2", "1,2,3,4", "NaN,2,3", "inf,2,3", "1e13,2,3"] { assert!(crate::tool_coords::parse(text).is_err(), "{text}"); }
}

#[test]
fn plane_points_use_the_current_camera_and_reject_parallel_or_clipped_rays() {
    for projection in [crate::camera::Projection::Perspective, crate::camera::Projection::Orthographic] {
        let mut camera = crate::camera::Camera::default(); camera.projection = projection; camera.pan(0.25, -0.5);
        let p = [0.125, -0.25, 0.]; let screen = camera.view_projection().transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
        let actual = crate::tool_coords::on_xy(&camera, [screen[0] as f32, screen[1] as f32]).unwrap();
        for i in 0..3 { assert!((actual[i]-p[i]).abs()<1e-6); }
        assert!(crate::tool_coords::on_xy(&camera, [f32::NAN, 0.]).is_none());
        assert!(crate::tool_coords::on_xy(&camera, [2., 0.]).is_none());
        camera.orientation = session_rust::Quaternion::identity(); assert!(crate::tool_coords::on_xy(&camera, [0., 0.]).is_none());
    }
}
