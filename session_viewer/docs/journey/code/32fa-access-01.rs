impl Object {
    pub fn geometry(&self) -> Option<&Rc<session_rust::Mesh>> { Some(&self.geometry) }

    pub fn source(&self) -> Option<&crate::document::Source> { self.source.as_ref() }

    pub fn world_point(&self, vertex: [f32; 6]) -> [f64; 3] {
