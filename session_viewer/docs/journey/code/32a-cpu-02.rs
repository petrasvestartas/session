    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
