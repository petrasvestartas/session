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
