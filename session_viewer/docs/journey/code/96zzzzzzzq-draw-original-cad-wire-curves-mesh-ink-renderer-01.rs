    fn set_source_ink(&mut self,scene:&Scene) {
        let anchor=self.anchor();
        self.sources = crate::pick_sources::Sources::anchored(scene,anchor).expect("Prepared source topology is bounded");
        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().zip(&self.sources.mesh_legacy).filter(|(_,kind)|**kind!=0).map(|(stroke,_)|*stroke)).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().zip(&self.sources.mesh_legacy).filter(|(_,kind)|**kind!=0).map(|(id,_)|*id)).expect("Source edge IDs match");
        self.mesh_ink.set_records(&self.device,self.sources.mesh_metadata.chunks_exact(2).zip(&self.sources.mesh_legacy).filter(|(_,kind)|**kind!=0).flat_map(|(words,_)|words.iter().copied()).collect()).expect("Original ink metadata");
        drop(std::mem::take(&mut self.sources.mesh_markers));
        let controls=scene.control_markers_at(anchor);let count=controls.len();
        self.markers.set_points(&self.device,controls.into_iter().map(|c|c.display)).expect("Prepared control placement");
