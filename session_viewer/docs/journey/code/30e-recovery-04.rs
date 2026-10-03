    assert_eq!((editor.camera.distance, editor.camera.aspect), (7.0, 2.0));
}

#[test]
fn refused_replacement_preserves_selection_source_owners_and_redo() {
    let mut editor = Editor::default();
    editor.apply(Action::AddBox).unwrap(); editor.apply(Action::Undo).unwrap();
    editor.apply(Action::SelectNext).unwrap();
    let selected = editor.selected;
    let old = editor.scene.objects()[0].clone();
    assert!(editor.apply(Action::Replace(vec![255])).is_err());
    assert_eq!(editor.selected, selected);
    assert_eq!(editor.scene.objects().len(), 2);
    assert!(Rc::ptr_eq(&editor.scene.objects()[0].geometry, &old.geometry));
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.objects().len(), 3);
}

#[test]
fn a_finite_double_matrix_can_exceed_the_current_float_upload() {
    let mut editor = Editor::default();
    let id = editor.scene.objects()[0].id;
    let before = editor.scene.objects()[0].model.m;
    let mut model = session_rust::Xform::identity(); model.m[0] = f64::MAX;
    assert!(model.m.iter().all(|value| value.is_finite()));
    assert!(!(model.m[0] as f32).is_finite());
    assert!(editor.scene.place(id, model.clone()).is_err());
    assert_eq!(editor.scene.objects()[0].model.m, before);
    use prost::Message;
    let bytes = crate::document::snapshot(&editor.scene).unwrap();
    let mut message = session_rust::proto::Session::decode(bytes.as_slice()).unwrap();
    message.xforms.push(session_rust::proto::XformEntry {
        guid: editor.scene.objects()[0].guid.clone(), xform: Some(model.to_proto()),
    });
    assert!(crate::document::load(&message.encode_to_vec()).is_err());
}
