    pub fn replace(&mut self, loaded: crate::document::Loaded) -> Result<(), &'static str> {
        self.objects.clear();
        self.extra = None;
        self.import(loaded)
    }

    pub fn import(&mut self, loaded: crate::document::Loaded) -> Result<(), &'static str> {
