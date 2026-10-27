        if !ids.is_empty() { self.strokes.clear_highlight(); }
        self.strokes.set_scene(&self.device,scene,&self.sources.line_ids,anchor,ids)?;