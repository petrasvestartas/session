    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validators_and_unchanged_bytes_keep_original_sources_and_viewer_ids() {
        let mut index=Index::new("files/".into());let manifest=b"items: [{file: mesh.pb}]";let raw=crate::specimen::bytes();let mut scene=Scene::empty();assert!(index.manifest("scene",Some("a".into()),manifest).unwrap());assert!(index.source("files/mesh.pb",Some("b".into()),&raw).unwrap());scene=index.replacement(&scene).unwrap().unwrap();let id=scene.objects()[0].id;let geometry=Rc::clone(scene.objects()[0].geometry().unwrap());
        assert!(!index.manifest("scene",Some("W/c".into()),manifest).unwrap());assert!(!index.source("files/mesh.pb",None,&raw).unwrap());assert_eq!(index.etag("scene"),Some("W/c".into()));assert_eq!(index.etag("files/mesh.pb"),None);assert!(index.replacement(&scene).unwrap().is_none());assert_eq!(scene.objects()[0].id,id);assert!(Rc::ptr_eq(scene.objects()[0].geometry().unwrap(),&geometry));
        assert!(index.source("files/mesh.pb",None,&crate::specimen::precise_bytes()).unwrap());let changed=index.replacement(&scene).unwrap().unwrap();assert_ne!(changed.objects()[0].id,id);assert!(!Rc::ptr_eq(changed.objects()[0].geometry().unwrap(),&geometry));assert!(Rc::ptr_eq(scene.objects()[0].geometry().unwrap(),&geometry));
    }
    #[test]
    fn invalid_or_incomplete_live_updates_keep_the_last_display_and_retry_failed_bytes() {
        let mut index=Index::new(String::new());let manifest=b"items: [{file: mesh.pb}]";let raw=crate::specimen::bytes();index.manifest("scene",None,manifest).unwrap();assert!(index.replacement(&Scene::empty()).is_err());index.source("mesh.pb",Some("good".into()),&raw).unwrap();let scene=index.replacement(&Scene::empty()).unwrap().unwrap();let id=scene.objects()[0].id;
        assert!(index.source("mesh.pb",Some("bad".into()),b"broken").is_err());assert_eq!(index.etag("mesh.pb"),None);assert!(index.replacement(&scene).unwrap().is_none());assert_eq!(scene.objects()[0].id,id);assert!(index.manifest("scene",None,b"invalid: manifest").is_err());assert_eq!(index.etag("scene"),None);assert!(index.manifest("scene",None,manifest).unwrap());assert!(index.replacement(&scene).unwrap().is_some());
        let mesh=Rc::downgrade(scene.objects()[0].geometry().unwrap());drop(scene);index.manifest("scene",None,b"items: []").unwrap();assert!(mesh.upgrade().is_none());assert!(!index.cached("mesh.pb"));
    }
}
