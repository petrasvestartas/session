    fn set_source_ink(&mut self,scene:&Scene) {}

    pub fn request_pick(&mut self, screen: [f32; 2], transform: [f32; 16]) -> Result<(), &'static str> {