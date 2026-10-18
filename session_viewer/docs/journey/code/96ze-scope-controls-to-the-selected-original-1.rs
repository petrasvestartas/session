    pub fn owner_controls(&mut self, parent: ObjectId) -> Result<(), &'static str> {
        let cloud = self.clouds.iter().any(|r|r.id==parent) || self.streamed.iter().any(|r|r.id==parent);
        if !cloud {
            let source = crate::source_edit::original(self,parent).ok_or("Source controls are unavailable")?;
            if crate::source_controls::Controls::from_geometry(&source).points.is_empty() {return Err("This object has no source controls");}
        }
        self.show_controls(false);
        self.cloud_controls(cloud.then_some(parent));
        if !cloud {
            Rc::make_mut(&mut self.control_owners).insert(parent);
            for row in &mut self.paths {row.controls_visible=row.id==parent;}
        }
        Ok(())
    }

    pub fn show_controls(&mut self, visible: bool) {