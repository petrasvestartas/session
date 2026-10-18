        self.strokes.set_ids(&self.device, self.sources.line_ids.iter().copied()).expect("Line owners match");
        self.mesh_ink.ranges=std::mem::take(&mut self.sources.mesh_runs);
        self.markers.ranges=std::mem::take(&mut self.sources.marker_runs);