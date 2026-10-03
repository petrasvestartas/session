    pub fn stats(&self) -> [usize; 4] {
        [self.geometry.uploads, self.settings_allocations, 0, 0]
    }

    pub fn draw(
