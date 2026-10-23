
    fn apply_ink_visibility(&mut self){
        let mask=self.ink_visibility.mask();self.mesh_ink.ranges.mask=mask;self.markers.ranges.mask=mask;
        self.strokes.visible=self.ink_visibility.lines;self.strokes.points_visible=self.ink_visibility.points;self.strokes.vertices_visible=self.ink_visibility.edges&&self.ink_visibility.vertices;
        self.paths.ranges.mask=if self.ink_visibility.lines{u8::MAX}else{0};
    }
    pub fn set_display_settings(&mut self,settings:crate::display_settings::Settings){
        if self.strokes.settings==settings{return;}self.strokes.settings=settings;self.matrices.settings=settings;self.sheets.settings=settings;
        let mut view=self.cloud_view.get();view.lod_px=settings.lod;view.scale=settings.cloud_scale;self.cloud_view.set(view);self.cancel_pick();
    }
    pub fn set_plane_size(&mut self,size:f64)->Result<(),&'static str>{
        if !size.is_finite()||size<=0.0||!(size as f32).is_finite(){return Err("View Plane Size needs a positive number of mm");}
