use crate::{colour::Mode, editor::{Action, Editor}, document};
use session_rust::mesh::ColorMode;

#[test]
fn colour_commands_keep_original_modes_arrays_and_one_step_history() {
    for (mode, expected) in [(Mode::Object, ColorMode::OBJECTCOLOR), (Mode::Points, ColorMode::POINTCOLORS), (Mode::Faces, ColorMode::FACECOLORS)] {
        let mut editor = Editor::default(); editor.apply(Action::Close).unwrap(); editor.apply(Action::AddColours(mode)).unwrap();
        let row = &editor.scene.objects()[0]; let id = row.id; let source = row.geometry().unwrap();
        let weak = std::rc::Rc::downgrade(source); assert!(source.color_mode == expected); assert_eq!(source.number_of_vertices(), 6);
        let original = source.to_proto(); let bytes = document::snapshot(&editor.scene).unwrap();
        let reopened = document::load(&bytes).unwrap(); assert_eq!(reopened.meshes[0].geometry.to_proto(), original); drop(reopened);
        editor.apply(Action::Undo).unwrap(); assert!(editor.scene.objects().is_empty()); assert!(weak.upgrade().is_some());
        editor.apply(Action::Redo).unwrap(); assert_eq!(editor.scene.objects()[0].id, id);
        editor.apply(Action::Close).unwrap(); assert!(weak.upgrade().is_none());
    }
}

#[test]
fn packed_colour_validation_refuses_partial_nonfinite_and_unknown_records() {
    use prost::Message;
    let mut editor = Editor::default(); editor.apply(Action::Close).unwrap(); editor.apply(Action::AddColours(Mode::Faces)).unwrap();
    let valid = document::snapshot(&editor.scene).unwrap();
    for case in 0..3 {
        let mut message = session_rust::proto::Session::decode(valid.as_slice()).unwrap(); let mesh = &mut message.objects.as_mut().unwrap().meshes[0];
        match case { 0 => mesh.pointcolors_rgba = vec![1.0, 0.0, 0.0], 1 => mesh.facecolors_rgba[0] = f32::NAN, _ => mesh.color_mode = 4 }
        assert!(document::load(&message.encode_to_vec()).is_err());
    }
}
