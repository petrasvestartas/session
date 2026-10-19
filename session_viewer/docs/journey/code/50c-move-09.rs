    let mut editor = Editor::default(); editor.apply(Action::SelectNext).unwrap();
    editor.apply(Action::Translate([1., 0., 0.])).unwrap(); editor.apply(Action::Undo).unwrap();
    let source = editor.scene.objects()[0].mesh.clone(); let placement = editor.scene.objects()[0].model.m;
    for text in ["", "Unknown", "Delete all", "Open Replace extra", "Fit Selected extra", "Move 1 2", "Move 1,,2", "Move NaN,0,0", "Move 1,2,inf", "Move 1,2,3,4", "Move 1 2,3,4"] {
        assert!(commands::parse(text).is_err(), "{text}");
        assert_eq!(editor.scene.objects()[0].model.m, placement);
        assert!(std::rc::Rc::ptr_eq(&source, &editor.scene.objects()[0].mesh));
