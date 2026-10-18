        self.entries.retain(|_, geometry| geometry.strong_count() > 0);
        if self.entries.is_empty(){self.arena=None;}