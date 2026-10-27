    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        self.strokes.set_highlight(&self.device,scene,target)?;self.sheets.set_highlight(&self.queue,target.filter(|t|t.valid(scene)));
        let anchor=self.anchor();self.highlight.set_at(&self.device,&self.surfaces.active.0.get_bind_group_layout(1),scene,target,anchor)
    }

