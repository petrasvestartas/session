use crate::{scene::{Scene, ObjectId}, history::History};
use session_rust::Xform;

// A placement preview shares original geometry, but owns temporary row placements outside the saved document and its history.
pub struct Preview { base: Scene, pub scene: Scene, ids: Vec<ObjectId>, transform: Xform }

impl Preview {
    pub fn new(scene: &Scene, mut ids: Vec<ObjectId>) -> Result<Self, &'static str> {
        ids.sort_unstable_by_key(|id| id.value()); ids.dedup();
        if ids.is_empty() { return Err("Select objects before Move"); }
        for id in &ids {
            if scene.pick_id(id.value())!=Some(*id) { return Err("Move needs visible unlocked objects"); }
            if scene.objects().iter().any(|r| r.id==*id && r.geometry().is_none()) { return Err("Reload editable sources before Move"); }
        }
        Ok(Self { base:scene.clone(),scene:scene.clone(),ids,transform:Xform::identity() })
    }
    pub fn ids(&self) -> &[ObjectId] { &self.ids }
    pub fn matrix(&self) -> &[f64; 16] { &self.transform.m }
    pub fn set(&mut self, transform: Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&transform.m) { return Err("Invalid preview transform"); }
        let mut scene=self.base.clone();
        for &id in &self.ids { let model=&transform * &self.base.placement(id).ok_or("Preview owner missing")?; scene.place_owner(id,model)?; }
        self.scene=scene; self.transform=transform; Ok(())
    }
    pub fn commit(&self, scene: &mut Scene, history: &mut History) -> Result<bool, &'static str> {
        for &id in &self.ids {
            if scene.pick_id(id.value())!=Some(id) || scene.placement(id).map(|m| m.m)!=self.base.placement(id).map(|m| m.m) { return Err("Preview targets changed; start Move again"); }
        }
        if self.transform.m==Xform::identity().m { return Ok(false); }
        history.try_edit(scene,|scene| {
            for &id in &self.ids { scene.place_owner(id,self.scene.placement(id).ok_or("Preview owner missing")?)?; }
            Ok(())
        })?; Ok(true)
    }
}
