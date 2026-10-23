    }

    pub fn clear_highlight(&mut self) {
        self.highlighted = None;self.edge_highlight=None;
    }

    pub fn set_highlight(&mut self, device: &wgpu::Device, scene: &crate::scene::Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        let target=target.filter(|target|target.valid(scene));
        self.edge_highlight=target.filter(|target|matches!(target.part,crate::pick_sources::Part::PathEdge(_)));
        self.highlighted=target.filter(|target|matches!(target.part,crate::pick_sources::Part::Curve)&&scene.lines().iter().any(|row|row.id==target.owner)||matches!(target.part,crate::pick_sources::Part::Curve)&&scene.paths().iter().any(|row|row.id==target.owner)).map(|target|target.owner);
        let selected=self.selected.clone();let sources=crate::pick_sources::Sources::anchored(scene,self.anchor)?;
        self.set_scene(device,scene,&sources,self.anchor,&selected)
    }

    fn bindings(
