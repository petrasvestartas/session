    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        self.undo.iter().chain(&self.redo)
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn edit(&mut self, scene: &mut Scene, action: impl FnOnce(&mut Scene)) {
