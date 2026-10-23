
    fn apply_ink_visibility(&mut self){
        let mask=self.ink_visibility.mask();self.mesh_ink.ranges.mask=mask;self.markers.ranges.mask=mask;
        self.strokes.visible=self.ink_visibility.lines;self.strokes.edges_visible=self.ink_visibility.edges;self.strokes.points_visible=self.ink_visibility.points;self.strokes.vertices_visible=self.ink_visibility.edges&&self.ink_visibility.vertices;
        self.paths.ranges.mask=if self.ink_visibility.lines{u8::MAX}else{0};
    }
    pub fn set_display_settings(&mut self,settings:crate::display_settings::Settings){
