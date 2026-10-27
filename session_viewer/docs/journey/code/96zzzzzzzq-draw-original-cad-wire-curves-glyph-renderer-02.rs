        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().copied()).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().copied()).expect("Source edge IDs match");
        self.mesh_ink.set_records(&self.device,std::mem::take(&mut self.sources.mesh_metadata)).expect("Original ink metadata");
        drop(std::mem::take(&mut self.sources.mesh_markers));
        let controls=scene.control_markers_at(anchor);let count=controls.len();
        self.markers.set_points(&self.device,controls.into_iter().map(|c|c.display)).expect("Prepared control placement");
        self.paths.set_chains(&self.device,std::iter::empty()).expect("Empty historical path lane");
        self.sources.wire_segments.clear();
        self.strokes.set_scene(&self.device,scene,&self.sources,anchor,&[]).expect("Prepared production line rows");
        self.matrices.set_scene(&self.device,scene,anchor,&[]).expect("Prepared matrix rows");
        self.markers.set_ids(&self.device, self.sources.marker_ids[self.sources.marker_ids.len()-count..].iter().copied()).expect("Marker owners match");
        self.paths.set_ids(&self.device, std::iter::empty()).expect("Path owners match");
        self.mesh_ink.ranges=std::mem::take(&mut self.sources.mesh_runs);
        self.sources.marker_runs=Default::default();self.markers.ranges=Default::default();self.markers.ranges.push(crate::ink_ranges::CONTROLS,0,count as u32);
        self.apply_ink_visibility();
    }

