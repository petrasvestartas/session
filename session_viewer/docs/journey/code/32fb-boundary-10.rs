    fn metadata_does_not_keep_a_closed_kernel_source_alive() {
        let mut editor = Editor::default();
        let mut mesh = session_rust::Mesh::create_box(1.0, 1.0, 1.0);
        mesh.name = "Released beam".into(); mesh.is_visible = false; mesh.is_locked = true;
        let id = editor.scene.insert(crate::prepared::PreparedMesh::new(mesh).unwrap()).unwrap();
        let row = editor.scene.objects().iter().find(|row| row.id == id).unwrap();
        let weak = Rc::downgrade(row.geometry().unwrap());
        let metadata = Rc::clone(&row.metadata);
        editor.apply(Action::Close).unwrap();
        assert!(weak.upgrade().is_none());
        assert_eq!(metadata.name, "Released beam");
        assert!(!metadata.is_visible && metadata.is_locked);
        assert!(!metadata.source_guid.is_empty());
    }
