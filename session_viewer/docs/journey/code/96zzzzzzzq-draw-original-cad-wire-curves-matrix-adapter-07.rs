        self.paths.set_chains(&self.device, scene.paths().iter().filter(|row| scene.visible(row.id)).flat_map(|row| {let mut segments=row.segments_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {for segment in &mut segments {segment.stroke.colour=colour;}}segments}).chain(std::mem::take(&mut self.sources.wire_segments)))
            .expect("Prepared path placement fits the display range");
        self.strokes.set_scene(&self.device,scene,&self.sources.line_ids,anchor,&[]).expect("Prepared production line rows");
        self.matrices.set_scene(&self.device,scene,anchor,&[]).expect("Prepared matrix rows");
        self.markers.set_ids(&self.device, self.sources.marker_ids.iter().copied()).expect("Marker owners match");
        self.paths.set_ids(&self.device, self.sources.path_ids.iter().copied()).expect("Path owners match");
        self.mesh_ink.ranges=std::mem::take(&mut self.sources.mesh_runs);
