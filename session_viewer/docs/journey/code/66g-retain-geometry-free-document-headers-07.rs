    }
    #[test]
    fn mixed_tree_snapshot_keeps_line_copy_identity_and_nonmesh_placements() {
        use std::rc::Rc;
        let mut scene=crate::scene::Scene::demo();scene.clear();let source=Rc::new(session_rust::Line::new(0.,0.,0.,1.,0.,0.));let _=source.guid();let before=source.to_proto();
        let first=scene.insert_line(crate::stroke::PreparedLine::new(source.clone()).unwrap()).unwrap();
        let key=format!("{}/{}",scene.forest.documents[0].key,scene.forest.documents[0].nodes[0].source.guid);
        crate::layer_edit::apply(&mut scene,&key,crate::layer_edit::Edit::Copy(vec![first])).unwrap();let copy=scene.lines()[1].id;
        assert_ne!(scene.row_guid(first),scene.row_guid(copy));assert!(Rc::ptr_eq(&scene.lines()[0].prepared.source,&scene.lines()[1].prepared.source));
        scene.place_line(copy,session_rust::Xform::translation(1_000_000_000.125,0.,0.)).unwrap();
        let point=scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(0.,2.,0.))).unwrap()).unwrap();
        let cloud=scene.insert_cloud(crate::cloud::Prepared::new(Rc::new(session_rust::PointCloud::from_coords(vec![0.,0.,0.],vec![],vec![]))).unwrap()).unwrap();
        let mut xforms=Vec::new();let (roots,placed)=scene.forest.snapshot(&scene,&mut xforms).unwrap();
        for id in [first,copy,point,cloud] {assert!(placed.contains(&id));}
        assert_eq!(xforms.iter().find(|entry|entry.guid==scene.row_guid(copy).unwrap()).unwrap().xform.as_ref().unwrap().matrix[12],1_000_000_000.125);
        assert!(!roots.is_empty());assert_eq!(source.to_proto(),before);
    }
    #[test]
    fn old_metadata_keeps_face_colour_fallback_and_default_identity_fields() {
        let source=br#"{"documents":[{"name":"old","place":[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1],"point_px":0}],"hidden":[],"colors":[[0,"guid",[12,34,56]]],"texts":[]}"#;
