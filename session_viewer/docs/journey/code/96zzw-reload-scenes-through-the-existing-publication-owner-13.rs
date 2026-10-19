ly.complete(&mut e).unwrap_err(), "Network failed");
    assert_eq!(e.scene.objects()[0].model.m, model); assert!(e.scene.objects()[0].geometry().is_none());
}

#[test]
fn live_empty_manifest_releases_originals_history_and_cached_source_owners(){
 let mut editor=crate::editor::Editor::default();editor.apply(crate::editor::Action::Replace(crate::specimen::bytes())).unwrap();let geometry=std::rc::Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());let origin=std::rc::Rc::downgrade(editor.scene.objects()[0].origin().unwrap());editor.apply(crate::editor::Action::SelectNext).unwrap();editor.apply(crate::editor::Action::Translate([0.1,0.,0.])).unwrap();assert!(editor.scenes().count()>1);
 let mut index=crate::publication_index::Index::new("files/".into());index.manifest("scene.yaml",None,b"items: [{file: original.pb}]").unwrap();index.source("files/original.pb",None,&crate::specimen::bytes()).unwrap();let cached=index.replacement(&crate::scene::Scene::empty()).unwrap().unwrap();let cached_geometry=std::rc::Rc::downgrade(cached.objects()[0].geometry().unwrap());drop(cached);assert!(cached_geometry.upgrade().is_some());
 index.manifest("scene.yaml",None,b"name: empty\nitems: []\n").unwrap();let empty=index.replacement(&editor.scene).unwrap().unwrap();assert!(!index.cached("files/original.pb"));assert!(cached_geometry.upgrade().is_none());editor.apply(crate::editor::Action::Live(Box::new(empty))).unwrap();assert!(editor.scene.objects().is_empty());assert_eq!(editor.scenes().count(),1);editor.apply(crate::editor::Action::Undo).unwrap();editor.apply(crate::editor::Action::Redo).unwrap();assert!(editor.scene.objects().is_empty());assert!(geometry.upgrade().is_none());assert!(origin.upgrade().is_none());
}
