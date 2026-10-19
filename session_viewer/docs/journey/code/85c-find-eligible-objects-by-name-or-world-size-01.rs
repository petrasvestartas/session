use crate::scene::{ObjectId,Scene};
pub enum Query{Small(f64),Name(String)}
impl Query{
    pub fn name(words:&[&str])->Result<Self,String>{let joined=words.join(" ");let text=['"','\''].iter().find_map(|quote|joined.strip_prefix(*quote)?.strip_suffix(*quote)).unwrap_or(&joined).trim();if text.is_empty(){return Err("Type a name after Select By Name".into())}Ok(Self::Name(text.into()))}
    pub fn found(&self,scene:&Scene)->Result<Vec<ObjectId>,String>{
        let mut ids=scene.all_ids().into_iter().filter(|id|scene.pick_id(id.value())==Some(*id)).filter(|id|match self{
            Self::Small(length)=>scene.selected_bounds(*id).is_some_and(|bounds|{let d=std::array::from_fn::<_,3,_>(|i|bounds.max[i]-bounds.min[i]);d[0].hypot(d[1]).hypot(d[2])<*length}),
            Self::Name(text)=>scene.forest.documents.iter().flat_map(|d|&d.nodes).find(|n|n.object==Some(*id)).is_some_and(|n|n.label.to_lowercase().contains(&text.to_lowercase())),
        }).collect::<Vec<_>>();ids.sort_unstable_by_key(|id|id.value());if ids.is_empty(){return Err("No visible unlocked object matches; selection unchanged".into())}Ok(ids)
    }
}
