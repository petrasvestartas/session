        self.strokes.visible=self.ink_visibility.lines;
        self.paths.ranges.mask=if self.ink_visibility.lines{u8::MAX}else{0};
    }
    pub fn set_plane_size(&mut self,size:f64)->Result<(),&'static str>{
        if !size.is_finite()||size<=0.0||!(size as f32).is_finite(){return Err("View Plane Size needs a positive number of mm");}
        if self.strokes.plane_size!=size as f32{self.strokes.plane_size=size as f32;self.cancel_pick();}Ok(())
    }
    pub fn plane_usage(&self)->[u64;2]{self.strokes.plane_usage()}
    pub fn set_ink_visibility(&mut self,value:crate::ink_ranges::Visibility){
        if self.ink_visibility==value{return;}self.ink_visibility=value;self.cancel_pick();self.apply_ink_visibility();
    }
