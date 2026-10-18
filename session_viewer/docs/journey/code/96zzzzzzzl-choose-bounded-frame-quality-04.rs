    pub fn has_solids(&self)->bool {
        self.meshes.iter().any(|mesh|mesh.geometry.index_count>0)||self.text.borrow().as_ref().is_some_and(|lane|lane.document.runs.iter().any(|run|!matches!(run.label.placement,crate::text::TextPlacement::Screen{..})))
    }
    pub fn samples(