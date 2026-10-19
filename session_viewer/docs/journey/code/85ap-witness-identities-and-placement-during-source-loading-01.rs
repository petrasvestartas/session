    pub fn ids(&self,editor:&Editor)->Vec<ObjectId>{match self{Self::Pick{id,..}=>id.iter().copied().collect(),_=>editor.selected_part.and(editor.selected).into_iter().chain(editor.selected_objects()).collect()}}
}
#[derive(Clone,Debug,PartialEq)]
pub struct Pending{pub request:Request,pub ids:Vec<ObjectId>,pub part:Option<crate::pick_sources::Part>,pub epoch:u64,pub owners:Vec<(ObjectId,String,[f64;16])>}
impl Pending{
    pub fn capture(action:&Action,editor:&Editor)->Option<Self>{let request=Request::capture(action)?;let ids=request.ids(editor);let owners=ids.iter().filter_map(|id|Some((*id,editor.scene.row_guid(*id)?.to_owned(),editor.scene.placement(*id)?.m))).collect();Some(Self{request,ids,part:editor.selected_part,epoch:editor.interaction_epoch,owners})}
    pub fn valid(&self,editor:&Editor)->bool{self.epoch==editor.interaction_epoch&&self.owners.iter().all(|(id,guid,model)|editor.scene.pick_id(id.value())==Some(*id)&&editor.scene.row_guid(*id)==Some(guid)&&editor.scene.placement(*id).is_some_and(|p|p.m==*model))}


}
