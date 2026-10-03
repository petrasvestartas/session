    pub(crate) fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        self.undo.iter_mut().chain(&mut self.redo)
    }

    pub fn clear(&mut self) {
