    pub fn display_scene_mut(&mut self)->&mut Scene {
        if let Some(scene)=self.trim.as_mut().and_then(|t|t.preview.as_mut()) {return scene;}
        if let Some(p)=self.canvas_drag.as_mut().and_then(|d|d.preview.as_mut()) {return &mut p.scene;}
        if let Some(g)=self.gizmo_edit.as_mut() {return g.preview.scene_mut();}
        if let Some(p)=self.transform_preview.as_mut() {return p.scene_mut();}
        if let Some(p)=self.edit_preview.as_mut() {return &mut p.scene;}
        &mut self.scene
    }

    pub fn gizmo_origin(&self)->Option<[f64;3]> {