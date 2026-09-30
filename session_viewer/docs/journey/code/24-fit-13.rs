use crate::{bounds::Bounds, camera::Camera, editor::{Action, Change, Editor}};
use session_rust::Point;

#[test]
fn fitted_corners_survive_scale_orientation_and_aspect_changes() {
    for scale in [0.001, 1.0, 10_000.0] {
        let mut bounds = Bounds::point([10.0 * scale, -4.0 * scale, 2.0 * scale]);
        bounds.include([14.0 * scale, 2.0 * scale, 3.0 * scale]);
        for aspect in [0.25, 1.0, 4.0] {
            let mut camera = Camera { aspect, ..Camera::default() };
            camera.isometric();
            let forward = camera.orientation.rotate_vector(session_rust::Vector::y_axis());
            camera.fit(&bounds);
            assert_eq!(camera.target, bounds.centre());
            let after = camera.orientation.rotate_vector(session_rust::Vector::y_axis());
            for i in 0..3 {
                assert!((forward[i] - after[i]).abs() < 1.0e-12);
            }
            for x in [bounds.min[0], bounds.max[0]] {
                for y in [bounds.min[1], bounds.max[1]] {
                    for z in [bounds.min[2], bounds.max[2]] {
                        let point = camera.view_projection().transform_point(&Point::new(x, y, z));
                        assert!(point[0].abs() < 1.0 && point[1].abs() < 1.0);
                        assert!((0.0..1.0).contains(&point[2]));
                    }
                }
            }
            let distance = camera.distance;
            camera.zoom(1.01);
            assert!((camera.distance - distance / 1.01_f32 as f64).abs() < distance * 1.0e-10);
        }
    }
}

#[test]
fn fit_uses_imported_vertices_without_changing_selection_or_history() {
    let mut editor = Editor::default();
    editor.apply(Action::Import(crate::specimen::bytes())).unwrap();
    editor.apply(Action::SelectNext).unwrap();
    let selected = editor.selected;
    assert_eq!(editor.apply(Action::Fit).unwrap(), Change::View);
    let bounds = editor.scene.bounds().unwrap();
    assert!(bounds.max[2] >= 1.25);
    let target = editor.camera.target;
    assert_eq!(selected, editor.selected);
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects().len(), 2);
    assert_eq!(editor.camera.target, target);
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.objects().len(), 5);
}

#[test]
fn empty_scene_and_single_point_have_defined_fit_behaviour() {
    let mut editor = Editor::default();
    while let Some(id) = editor.scene.next(None) {
        assert!(editor.scene.remove(id));
    }
    assert!(editor.scene.bounds().is_none());
    let before = editor.camera.uniform();
    editor.apply(Action::Fit).unwrap();
    assert_eq!(before, editor.camera.uniform());
    editor.camera.fit(&Bounds::point([7.0, 8.0, 9.0]));
    let screen = editor.camera.view_projection().transform_point(&Point::new(7.0, 8.0, 9.0));
    assert!(screen[0].abs() < 1.0e-9 && screen[1].abs() < 1.0e-9);
    assert!((0.0..1.0).contains(&screen[2]));
}
