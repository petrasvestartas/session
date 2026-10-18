
    pub fn clear(&mut self) {
        self.cad_walks=[0;2];self.instances.clear();
        self.created=None;
        self.ids=Default::default();
