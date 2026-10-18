    pub fn capture(action:&Action,editor:&Editor)->Option<Self>{let request=Request::capture(action)?;let ids=request.ids(editor);let owners=ids.iter().filter_map(|id|Some((*id,editor.scene.row_guid(*id)?.to_owned(),editor.scene.placement(*id)?.m))).collect();Some(Self{request,ids,part:editor.selected_part,epoch:editor.interaction_epoch,owners})}
    pub fn valid(&self,editor:&Editor)->bool{self.epoch==editor.interaction_epoch&&self.owners.iter().all(|(id,guid,model)|editor.scene.pick_id(id.value())==Some(*id)&&editor.scene.row_guid(*id)==Some(guid)&&editor.scene.placement(*id).is_some_and(|p|p.m==*model))}
    pub fn keys(&self,editor:&Editor)->Vec<crate::rehydrate::ReloadKey>{let mut seen=std::collections::HashSet::new();self.ids.iter().filter_map(|id|editor.scene.objects().iter().find(|r|r.id==*id)).filter_map(crate::rehydrate::ReloadKey::of).filter(|key|seen.insert(key.origin.id)).collect()}
    pub fn replay(self,editor:&mut Editor)->Result<crate::edit_replay::Reply,&'static str>{
        if !self.valid(editor){return Err("Command target changed during source loading; start again")}
        let saved=editor.selected_objects();let saved_part=editor.selected_part;let saved_selected=editor.selected;
        if !matches!(&self.request,Request::Pick{..}){editor.select_objects(self.ids,false);editor.selected_part=self.part;}
        let result=editor.apply(self.request.action()?);if result.is_err(){editor.select_objects(saved,false);editor.selected=saved_selected;editor.selected_part=saved_part;}result.map(crate::edit_replay::Reply::Changed)
    }
}
