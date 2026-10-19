    fn apply_ink_visibility(&mut self){
        let mask=self.ink_visibility.mask();self.mesh_ink.ranges.mask=mask;self.markers.ranges.mask=mask;
        for lane in [&mut self.strokes,&mut self.paths]{lane.ranges.mask=if self.ink_visibility.lines{u8::MAX}else{0};}
    }
    pub fn set_ink_visibility(&mut self,value:crate::ink_ranges::Visibility){
        if self.ink_visibility==value{return;}self.ink_visibility=value;self.cancel_pick();self.apply_ink_visibility();
    }

    pub fn set_pick_mode(&mut self, mode: