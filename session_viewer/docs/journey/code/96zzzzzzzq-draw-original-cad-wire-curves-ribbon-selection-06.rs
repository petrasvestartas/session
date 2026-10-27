        self.highlight.set(&self.device, &self.surfaces.active.0.get_bind_group_layout(1), scene, None).expect("Empty selection");
        self.strokes.clear_highlight();