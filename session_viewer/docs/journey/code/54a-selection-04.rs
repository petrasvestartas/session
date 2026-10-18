    }

    pub fn remove(&mut self,scene:&Scene,rows:impl IntoIterator<Item=ObjectId>) {
        let rows=rows.into_iter().collect::<std::collections::HashSet<_>>();self.order.retain(|id|!rows.contains(id));self.retain(scene);
    }

    pub fn retain(&mut self, scene: &Scene) { self.order.retain(|id| scene.pick_id(id.value()).is_some()); }
    pub fn rows(&self) -> &[ObjectId] { &self.order }
