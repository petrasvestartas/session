use crate::{editor::{Action, Change, Editor}, camera::Projection};

#[test]
fn selected_fit_changes_only_the_camera() {
    for mode in [Projection::Perspective, Projection::Orthographic] {
        let mut editor = Editor::default();
        editor.camera.projection = mode;
        editor.apply(Action::AddBox).unwrap();
        editor.apply(Action::SelectNext).unwrap();
        let id = editor.selected.unwrap();
        let mesh = editor.scene.objects()[0].mesh.clone();
        let bounds = editor.scene.selected_bounds(id).unwrap();
        assert_eq!(editor.apply(Action::FitSelected).unwrap(), Change::View);
        assert_eq!(editor.camera.target, bounds.centre());
        assert_eq!(editor.selected, Some(id));
        assert!(std::rc::Rc::ptr_eq(&mesh, &editor.scene.objects()[0].mesh));
        editor.apply(Action::Undo).unwrap();
        assert_eq!(editor.scene.objects().len(), 2);
        assert_eq!(editor.camera.target, bounds.centre());
    }
}

#[test]
fn no_selection_leaves_the_view_alone() {
    let mut editor = Editor::default();
    let before = editor.camera.uniform();
    editor.apply(Action::FitSelected).unwrap();
    assert_eq!(editor.camera.uniform(), before);
}
