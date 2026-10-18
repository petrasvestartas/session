
    pub fn clear(&mut self) {
        self.groups=Default::default();self.graphs=Default::default();
        self.cad_walks=[0;2];self.instances.clear();
        self.created=None;
