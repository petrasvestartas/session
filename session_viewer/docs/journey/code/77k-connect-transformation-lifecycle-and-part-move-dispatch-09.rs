    editor.apply(Action::PickMode(Mode::Face)).unwrap(); editor.apply(Action::PickedTarget(Some(target))).unwrap();
    assert_eq!(editor.selection(), Some(target)); assert_eq!(editor.whole_selection(), None);
    assert!(editor.apply(Action::Translate([f64::INFINITY, 0., 0.])).is_err());
    assert!(editor.apply(Action::Delete).is_err()); assert_eq!(editor.scene.objects().len(), 1);
    assert!(std::rc::Rc::ptr_eq(&source, &editor.scene.objects()[0].geometry().unwrap()));
