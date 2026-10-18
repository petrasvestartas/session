        self.markers.ranges=std::mem::take(&mut self.sources.marker_runs);
        self.apply_ink_visibility();