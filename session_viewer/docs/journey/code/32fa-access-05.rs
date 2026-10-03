fn generated_source_and_display_survive_move_and_history() {
    let mut editor = Editor::default();
    editor.apply(Action::AddBox).unwrap();
    for _ in 0..3 { editor.apply(Action::SelectNext).unwrap(); }
    let object = &editor.scene.objects()[2];
    let geometry = Rc::clone(object.geometry().unwrap());
    let display = Rc::clone(&object.mesh);
    let point = geometry.vertex_point(0).unwrap();
    let guid = geometry.guid().to_owned();
    let before = object.model.m;
    editor.apply(Action::Translate([2.0, -1.0, 0.5])).unwrap();
    let moved = editor.scene.objects()[2].model.m;
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects()[2].model.m, before);
    editor.apply(Action::Redo).unwrap();
    let restored = &editor.scene.objects()[2];
    assert_eq!(restored.model.m, moved);
    assert!(Rc::ptr_eq(&geometry, restored.geometry().unwrap()));
    assert!(Rc::ptr_eq(&display, &restored.mesh));
    assert_eq!(restored.geometry().unwrap().vertex_point(0).unwrap(), point);
    assert_eq!(restored.geometry().unwrap().guid(), guid);
    assert!(restored.source().is_none());
}
