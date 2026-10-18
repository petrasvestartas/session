    pub fn set(&mut self, transform: Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&transform.m) { return Err("Invalid preview transform"); }
        let mut scene=self.base.clone();
        for &id in &self.ids { let model=&transform * &self.base.placement(id).ok_or("Preview owner missing")?; scene.place_following(id,model)?; }
        self.scene=scene; self.transform=transform; Ok(())
    }
    pub fn commit(&self, scene: &mut Scene, history: &mut History) -> Result<bool, &'static str> {
        for &id in &self.ids {
            if self.roots.contains(&id) && scene.pick_id(id.value())!=Some(id) || scene.placement(id).map(|m| m.m)!=self.base.placement(id).map(|m| m.m) { return Err("Preview targets changed; start Move again"); }
        }
        if self.transform.m==Xform::identity().m { return Ok(false); }
        history.try_edit(scene,|scene| {
            for &id in &self.ids { scene.place_following(id,self.scene.placement(id).ok_or("Preview owner missing")?)?; }
            Ok(())
        })?; Ok(true)
    }
