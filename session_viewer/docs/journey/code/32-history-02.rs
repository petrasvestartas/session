    #[test]
    fn cleared_history_releases_a_deleted_source_and_display() {
        let mut scene = Scene::demo();
        let mut history = History::default();
        let id = scene.objects()[0].id;
        let display = Rc::downgrade(&scene.objects()[0].mesh);
        let source = Rc::downgrade(&scene.objects()[0].geometry);
        history.edit(&mut scene, |scene| { scene.remove(id); });
        assert!(display.upgrade().is_some() && source.upgrade().is_some());
        assert_eq!(history.scenes().count(), 1);
        history.clear();
        assert!(display.upgrade().is_none() && source.upgrade().is_none());
        assert!(!history.undo(&mut scene) && !history.redo(&mut scene));
        assert_eq!(scene.objects().len(), 1);
    }

    #[test]
    fn clearing_both_branches_releases_a_redo_only_object() {
        let mut scene = Scene::demo();
        let mut history = History::default();
        history.edit(&mut scene, Scene::toggle_extra);
        let display = Rc::downgrade(&scene.objects()[2].mesh);
        history.undo(&mut scene);
        assert!(display.upgrade().is_some());
        history.clear();
        assert!(display.upgrade().is_none());
        assert_eq!(history.scenes().count(), 0);
        assert!(!history.redo(&mut scene));
    }

    #[test]
    fn undo_restores_identity_and_shares_the_original_mesh() {
