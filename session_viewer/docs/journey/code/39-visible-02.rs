    pub fn visible(&self, id: ObjectId) -> bool {
        if let Some(row) = self.objects.iter().find(|row| row.id == id) { return row.metadata.is_visible; }
        if let Some(row) = self.lines.iter().find(|row| row.id == id) { return row.prepared.source.is_visible; }
        if let Some(row) = self.paths.iter().find(|row| row.id == id) { return row.prepared.source.visible(); }
        self.points.iter().find(|row| row.id == id).is_some_and(|row| row.prepared.source.is_visible)
    }

    pub fn bounds(&self) -> Option<crate::bounds::Bounds> {