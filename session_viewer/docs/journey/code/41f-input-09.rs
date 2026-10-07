use crate::{editor::{Action, Editor, Change}, grid::Settings, normal_example::Mode};

#[test]
fn grid_commands_preserve_source_history_camera_settings_and_viewport_lifetime() {
    let mut editor = Editor::default(); editor.apply(Action::Close).unwrap(); editor.apply(Action::AddNormals(Mode::Sharp)).unwrap();
    let id = editor.scene.objects()[0].id; let weak = std::rc::Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
    assert_eq!(editor.apply(Action::Grid(Some(Settings::default()))).unwrap(), Change::View);
    editor.apply(Action::Orbit(0.2, 0.1)).unwrap(); editor.apply(Action::ResetView).unwrap(); assert_eq!(editor.camera.grid, Some(Settings::default()));
    editor.apply(Action::Undo).unwrap(); assert!(editor.scene.objects().is_empty()); editor.apply(Action::Redo).unwrap(); assert_eq!(editor.scene.objects()[0].id, id);
    let before = editor.camera.grid;
    assert!(editor.apply(Action::Grid(Some(Settings { spacing: f64::NAN, ..Settings::default() }))).is_err()); assert_eq!(editor.camera.grid, before);
    editor.apply(Action::Close).unwrap(); assert!(weak.upgrade().is_none()); assert_eq!(editor.camera.grid, before);
    editor.apply(Action::Grid(None)).unwrap(); assert!(editor.camera.grid.is_none());
}
