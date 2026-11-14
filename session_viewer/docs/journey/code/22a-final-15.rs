use crate::{bounds::Bounds, camera::{Camera, Projection}, editor::{Action, Change, Editor}};
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

#[test]
fn an_orthographic_fit_contains_every_corner_in_tall_and_wide_views() {
    let mut bounds = Bounds::point([-3.0, -1.0, -0.5]);
    bounds.include([2.0, 4.0, 1.0]);
    for aspect in [0.25, 1.0, 4.0] {
        let mut camera = Camera { projection: Projection::Orthographic, aspect, ..Camera::default() };
        camera.isometric();
        camera.fit(&bounds);
        for x in [bounds.min[0], bounds.max[0]] {
            for y in [bounds.min[1], bounds.max[1]] {
                for z in [bounds.min[2], bounds.max[2]] {
                    let point = camera.view_projection().transform_point(&Point::new(x, y, z));
                    assert!(point[0].abs() < 1.0 && point[1].abs() < 1.0);
                    assert!((0.0..1.0).contains(&point[2]));
                }
            }
        }
    }
}

#[test]
fn close_orthographic_picking_sees_geometry_behind_the_eye() {
    let mut editor = Editor::default();
    let before = editor.camera.uniform();
    assert_eq!(editor.apply(Action::Projection(Projection::Orthographic)).unwrap(), Change::View);
    editor.apply(Action::Projection(Projection::Perspective)).unwrap();
    assert_eq!(editor.camera.uniform(), before);
    editor.apply(Action::Projection(Projection::Orthographic)).unwrap();
    editor.apply(Action::Zoom(100.0)).unwrap();
    assert!(editor.camera.distance < 0.75);
    editor.apply(Action::Pick([0.0, 0.0])).unwrap();
    assert_eq!(editor.selected, Some(editor.scene.objects()[1].id));
    editor.apply(Action::AddBox).unwrap();
    editor.apply(Action::Projection(Projection::Perspective)).unwrap();
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects().len(), 2);
    assert_eq!(editor.camera.projection, Projection::Perspective);
    editor.apply(Action::Projection(Projection::Orthographic)).unwrap();
    editor.apply(Action::ResetView).unwrap();
    assert_eq!(editor.camera.projection, Projection::Perspective);
}
