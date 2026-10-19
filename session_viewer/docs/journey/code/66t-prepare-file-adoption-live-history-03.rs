    Ok(candidate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::{Action,Editor};
    #[test]
    fn prepared_file_adoption_undo_redo_and_save_preserve_original_source_and_camera() {
        let mut editor=Editor::default();editor.apply(Action::AddCloud).unwrap();let cloud=editor.scene.clouds()[0].prepared.source.to_proto();let saved=crate::scene_archive::save(&editor.scene).unwrap();let camera=editor.camera.uniform();let old=editor.scene.clouds()[0].id;
        let ready=prepare(&editor.scene,&saved,true,None).unwrap();editor.apply(Action::Adopt {scene:Box::new(ready),replace:true}).unwrap();let new=editor.scene.clouds()[0].id;assert_ne!(new,old);assert_eq!(editor.scene.clouds()[0].prepared.source.to_proto(),cloud);assert_eq!(editor.camera.uniform(),camera);
        editor.apply(Action::Undo).unwrap();assert_eq!(editor.scene.clouds()[0].id,old);editor.apply(Action::Redo).unwrap();assert_eq!(editor.scene.clouds()[0].id,new);
        let crate::edit_replay::Reply::Saved(bytes)=crate::edit_intent::Intent::Save.replay(&mut editor).unwrap() else {panic!("Saved viewer archive")};let mut restored=Scene::demo();crate::scene_document::open(&mut restored,&bytes).unwrap();assert_eq!(restored.clouds()[0].prepared.source.to_proto(),cloud);
    }
    #[test]
    fn malformed_preparation_keeps_redo_and_legacy_source_reload_stays_available() {
        let mut editor=Editor::default();editor.apply(Action::AddBox).unwrap();let id=editor.scene.objects().last().unwrap().id;editor.apply(Action::Undo).unwrap();assert!(prepare(&editor.scene,b"broken",true,None).is_err());editor.apply(Action::Redo).unwrap();assert!(editor.scene.contains(id));
        let location=Rc::new(crate::reload_url::ReloadUrl::new("test:publication".into()));let ready=prepare(&editor.scene,&crate::specimen::bytes(),true,Some(location.clone())).unwrap();assert!(ready.objects().iter().all(|row|row.can_unload()));assert!(Rc::ptr_eq(ready.objects()[0].origin().unwrap().location.as_ref().unwrap(),&location));
    }
    #[test]
    fn live_adoption_releases_prior_history_sources_and_preserves_the_camera() {
        let mut editor=Editor::default();editor.apply(Action::AddBox).unwrap();let old=Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());let display=Rc::downgrade(&editor.scene.objects()[0].mesh);let camera=editor.camera.uniform();let mut next=Scene::empty();next.insert_cloud(crate::cloud::Prepared::new(Rc::new(crate::cloud::example())).unwrap()).unwrap();editor.apply(Action::Live(Box::new(next))).unwrap();assert!(old.upgrade().is_none());assert!(display.upgrade().is_none());assert_eq!(editor.scenes().count(),1);assert_eq!(editor.camera.uniform(),camera);assert_eq!(editor.scene.clouds().len(),1);editor.apply(Action::Undo).unwrap();assert_eq!(editor.scene.clouds().len(),1);
    }

}
