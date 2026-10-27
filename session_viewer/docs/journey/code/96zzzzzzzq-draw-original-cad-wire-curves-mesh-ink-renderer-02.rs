        self.matrices.set_scene(&self.device,scene,anchor,&[]).expect("Prepared matrix rows");
        self.markers.set_ids(&self.device, self.sources.marker_ids[self.sources.marker_ids.len()-count..].iter().copied()).expect("Marker owners match");
        self.paths.set_ids(&self.device, std::iter::empty()).expect("Path owners match");
        self.mesh_ink.ranges=Default::default();let mut row=0;
        for &kind in &self.sources.mesh_legacy{if kind!=0{self.mesh_ink.ranges.push(kind,row,row+1);row+=1;}}
        self.sources.marker_runs=Default::default();self.markers.ranges=Default::default();self.markers.ranges.push(crate::ink_ranges::CONTROLS,0,count as u32);
        self.apply_ink_visibility();
    }
