    pub fn clear_highlight(&mut self) {
        self.highlighted = None;
    }

    pub fn set_highlight(&mut self, device: &wgpu::Device, scene: &crate::scene::Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        let highlighted = target.filter(|target| target.valid(scene)
            && matches!(target.part, crate::pick_sources::Part::Curve)
            && scene.lines().iter().any(|row| row.id == target.owner)).map(|target| target.owner);
        if highlighted == self.highlighted { return Ok(()); }
        self.highlighted = highlighted;
        let ids = self.ids.clone();
        let selected = self.selected.clone();
        self.set_scene(device, scene, &ids, self.anchor, &selected)
    }

    fn bindings(
