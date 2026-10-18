    }

}

#[cfg(test)]
mod streamed_tests {
    use super::*;
    #[test]
    fn remote_original_revisions_share_placements_and_keep_ids_until_a_changed_validator() {
        let mut index=Index::new(String::new());index.manifest("scene",None,b"items: [{file: points.pb}, {file: points.pb, at: [2,0,0], point_size: 7}]").unwrap();let first=crate::range_example::prepared();index.stream("points.pb",first.clone()).unwrap();let scene=index.replacement(&Scene::empty()).unwrap().unwrap();assert_eq!(scene.streams().len(),2);assert!(Rc::ptr_eq(&scene.streams()[0].prepared,&scene.streams()[1].prepared));let ids=scene.streams().iter().map(|r|r.id).collect::<Vec<_>>();assert!(!index.stream("points.pb",first.clone()).unwrap());assert!(index.replacement(&scene).unwrap().is_none());assert_eq!(scene.streams().iter().map(|r|r.id).collect::<Vec<_>>(),ids);
        let mut changed=crate::range_example::prepared();let inner=Rc::get_mut(&mut changed).unwrap();Rc::get_mut(&mut inner.source).unwrap().fields.revision=Some("v2".into());assert!(index.stream("points.pb",changed.clone()).unwrap());let next=index.replacement(&scene).unwrap().unwrap();assert!(Rc::ptr_eq(&next.streams()[0].prepared,&changed));assert_ne!(next.streams()[0].id,ids[0]);assert!(Rc::ptr_eq(&scene.streams()[0].prepared,&first));assert_eq!(next.streams()[1].point_px,7.);
    }
}
