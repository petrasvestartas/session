        let mesh=Rc::downgrade(scene.objects()[0].geometry().unwrap());drop(scene);index.manifest("scene",None,b"items: []").unwrap();assert!(mesh.upgrade().is_none());assert!(!index.cached("mesh.pb"));
    }
    #[test]
    fn refused_encoding_changes_keep_the_existing_manifest_and_original_sources() {
        let mut index=Index::new("files/".into());let raw=crate::specimen::bytes();index.manifest("scene",None,b"items: [{file: mesh.pb}]").unwrap();index.source("files/mesh.pb",None,&raw).unwrap();let scene=index.replacement(&Scene::empty()).unwrap().unwrap();let original=Rc::clone(scene.objects()[0].geometry().unwrap());
        for manifest in [b"items: [{file: mesh.pb, encoding: gzip, size: 2}, {file: mesh.pb, encoding: gzip, size: 3}]".as_slice(),b"items: [{file: mesh.pb, encoding: brotli, size: 24}]".as_slice()] {
            assert!(index.manifest("scene",Some("bad".into()),manifest).is_err());assert_eq!(index.etag("scene"),None);assert_eq!(index.urls(),["files/mesh.pb"]);assert!(index.replacement(&scene).unwrap().is_none());assert!(Rc::ptr_eq(scene.objects()[0].geometry().unwrap(),&original));
        }
        assert!(index.manifest("scene",None,b"items: [{file: mesh.pb}]").unwrap());assert!(index.replacement(&scene).unwrap().is_some());
    }

}
