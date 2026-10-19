    let mut editor = Editor::default(); editor.apply(Action::AddPoint).unwrap();
    let weak = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
    let locked=editor.scene.objects()[0].id;let hidden=editor.scene.objects()[1].id;
    Rc::make_mut(&mut editor.scene.view).lock(&[locked],true);Rc::make_mut(&mut editor.scene.view).hide(&[hidden],true);
    editor.apply(Action::SelectAll).unwrap(); assert_eq!(editor.selected_objects(), vec![editor.scene.points()[0].id]);
    editor.apply(Action::Close).unwrap(); assert!(weak.upgrade().is_none()); assert!(editor.selected_objects().is_empty());
