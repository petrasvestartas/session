        self.mesh_ink.set_records(&self.device,std::mem::take(&mut self.sources.mesh_metadata)).expect("Original ink metadata");
        self.markers.set_points(&self.device, std::mem::take(&mut self.sources.mesh_markers).into_iter().chain(scene.points().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut marker=row.marker_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {marker.colour=colour;}marker})).chain(scene.control_markers_at(anchor).into_iter().map(|c| c.display)))
            .expect("Prepared point placement fits the display range");
        self.paths.set_chains(&self.device,std::iter::empty()).expect("Empty historical path lane");
        self.sources.wire_segments.clear();
        self.strokes.set_scene(&self.device,scene,&self.sources,anchor,&[]).expect("Prepared production line rows");
        self.matrices.set_scene(&self.device,scene,anchor,&[]).expect("Prepared matrix rows");
        self.markers.set_ids(&self.device, self.sources.marker_ids.iter().copied()).expect("Marker owners match");
        self.paths.set_ids(&self.device, std::iter::empty()).expect("Path owners match");
        self.mesh_ink.ranges=std::mem::take(&mut self.sources.mesh_runs);
        self.markers.ranges=std::mem::take(&mut self.sources.marker_runs);
        self.apply_ink_visibility();
