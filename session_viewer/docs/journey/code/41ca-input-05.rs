use crate::{editor::{Action, Editor, Change}, normal_example::Mode, document};

#[test]
fn normal_commands_preserve_original_attributes_history_and_roundtrips() {
    for mode in [Mode::Sharp, Mode::Smooth, Mode::Winding] {
        let mut editor = Editor::default(); editor.apply(Action::Close).unwrap(); editor.apply(Action::AddNormals(mode)).unwrap();
        let id = editor.scene.objects()[0].id; let source = editor.scene.objects()[0].geometry().unwrap();
        let weak = std::rc::Rc::downgrade(source); let original = source.to_proto();
        if matches!(mode, Mode::Smooth) { assert_eq!(source.vertex_attribute(1, "nx"), Some(1.0)); assert_eq!(source.vertex_attribute(0, "nz"), Some(1.0)); }
        let bytes = document::snapshot(&editor.scene).unwrap(); let loaded = document::load(&bytes).unwrap(); assert_eq!(loaded.meshes[0].geometry.to_proto(), original); drop(loaded);
        assert_eq!(editor.apply(Action::NormalView(true)).unwrap(), Change::View); assert!(editor.normal_view);
        editor.apply(Action::Undo).unwrap(); assert!(editor.scene.objects().is_empty()); assert!(weak.upgrade().is_some());
        editor.apply(Action::Redo).unwrap(); assert_eq!(editor.scene.objects()[0].id, id); assert!(editor.normal_view);
        editor.apply(Action::Close).unwrap(); assert!(weak.upgrade().is_none());
    }
}
