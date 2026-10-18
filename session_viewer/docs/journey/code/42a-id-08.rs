}

impl Scene {
    pub fn pick_id(&self, value: u32) -> Option<ObjectId> {
        self.objects.iter().filter(|r| r.visible() && !r.metadata.is_locked).map(|r| r.id)
            .chain(self.lines.iter().filter(|r| r.visible() && !r.prepared.source.is_locked).map(|r| r.id))
            .chain(self.paths.iter().filter(|r| r.visible() && !r.prepared.source.locked()).map(|r| r.id))
            .chain(self.points.iter().filter(|r| r.visible() && !r.prepared.source.is_locked).map(|r| r.id))
            .find(|id| id.value() == value)
    }

    pub fn demo() -> Self {
        let pink = PreparedMesh::triangle([
            [-0.7, -0.6, 0.25], [0.5, -0.6, 0.25], [-0.1, 0.6, 0.25],
