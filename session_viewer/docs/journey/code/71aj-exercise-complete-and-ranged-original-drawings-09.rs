heets()[1].url.as_deref(),
        Some("https://example.test/second/plan.pb")
    );
    assert_ne!(second.sheets()[0].id, second.sheets()[1].id);
}

#[test]
fn select_all_moves_an_original_sheet_and_point_in_one_transaction_and_undo_restores_sources() {
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    editor.apply(Action::AddSheet).unwrap();
    editor.apply(Action::AddPoint).unwrap();
    let sheet = Rc::clone(&editor.scene.sheets()[0].prepared);
    let point = Rc::clone(&editor.scene.points()[0].prepared.source);
    let ids = [editor.scene.sheets()[0].id, editor.scene.points()[0].id];
    editor.apply(Action::SelectAll).unwrap();
    let mut selected = editor.selected_objects();
    selected.sort_unstable_by_key(|id| id.value());
    assert_eq!(selected, ids);
    editor.apply(Action::Translate([0.25, 0., 0.])).unwrap();
    assert_eq!(editor.scene.sheets()[0].model.m[12], 0.25);
    assert!(Rc::ptr_eq(&sheet, &editor.scene.sheets()[0].prepared));
    assert!(Rc::ptr_eq(&point, &editor.scene.points()[0].prepared.source));
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.sheets()[0].model, Xform::identity());
    assert!(Rc::ptr_eq(&sheet, &editor.scene.sheets()[0].prepared));
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.sheets()[0].model.m[12], 0.25);
}
