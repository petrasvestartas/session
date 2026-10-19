use crate::scene::{ObjectId, Scene};

#[derive(Default)]
pub struct Objects { order: Vec<ObjectId> }

impl Objects {
    // Membership is unique, while pick order is preserved because later curve and element tools interpret their inputs in the order picked.
    pub fn set(&mut self, scene: &Scene, rows: impl IntoIterator<Item = ObjectId>, additive: bool) {
        if !additive { self.order.clear(); }
        for id in rows { if scene.pick_id(id.value()).is_some() && !self.order.contains(&id) { self.order.push(id); } }
        self.retain(scene);
    }

    pub fn retain(&mut self, scene: &Scene) { self.order.retain(|id| scene.pick_id(id.value()).is_some()); }
    pub fn rows(&self) -> &[ObjectId] { &self.order }
    pub fn primary(&self) -> Option<ObjectId> { self.order.iter().copied().min_by_key(|id| id.value()) }
    pub fn clear(&mut self) { self.order.clear(); }
}
