use crate::camera::{Camera, Projection};
use session_rust::{Point, Vector};

#[test]
fn switching_projection_preserves_scale_on_the_target_plane() {
    let mut camera = Camera::default();
    camera.pan(0.3, -0.2);
    camera.isometric();
    let right = camera.orientation.rotate_vector(Vector::x_axis());
    let up = camera.orientation.rotate_vector(Vector::z_axis());
    let point = Point::new(
        camera.target[0] + right[0] * 0.4 + up[0] * 0.2,
        camera.target[1] + right[1] * 0.4 + up[1] * 0.2,
        camera.target[2] + right[2] * 0.4 + up[2] * 0.2,
    );
    let before = camera.view_projection().transform_point(&point);
    camera.projection = Projection::Orthographic;
    let after = camera.view_projection().transform_point(&point);
    for i in 0..2 {
        assert!((before[i] - after[i]).abs() < 1.0e-12);
    }
}

#[test]
fn orthographic_size_ignores_depth_and_its_pick_rays_are_parallel() {
    let mut camera = Camera { projection: Projection::Orthographic, ..Camera::default() };
    let a = camera.view_projection().transform_point(&Point::new(0.4, 0.0, 0.0));
    let b = camera.view_projection().transform_point(&Point::new(0.4, 0.0, 0.75));
    assert!((a[0] - b[0]).abs() < 1.0e-12);
    let left = camera.ray([-0.5, 0.0]).unwrap();
    let right = camera.ray([0.5, 0.0]).unwrap();
    assert!((&right.origin - &left.origin).magnitude() > 1.0);
    assert!(left.direction.cross(&right.direction).magnitude() < 1.0e-12);
    camera.projection = Projection::Perspective;
    let a = camera.view_projection().transform_point(&Point::new(0.4, 0.0, 0.0));
    let b = camera.view_projection().transform_point(&Point::new(0.4, 0.0, 0.75));
    assert!(b[0] > a[0]);
    assert!(camera.ray([-0.5, 0.0]).unwrap().direction
        .cross(&camera.ray([0.5, 0.0]).unwrap().direction).magnitude() > 0.1);
}

