            .chain(self.lines.iter().filter(|r| r.visible() && !r.prepared.source.is_locked).map(|r| r.id))
            .chain(self.paths.iter().filter(|r| r.visible() && !r.prepared.source.locked()).map(|r| r.id))
            .chain(self.points.iter().filter(|r| r.visible() && !r.prepared.source.is_locked).map(|r| r.id)).collect();
        ids.retain(|id|!self.forest.readonly(*id));
        ids.sort_unstable_by_key(|id| id.value());
        if ids.is_empty() { return None; }
        let start = ids.iter().position(|id| Some(*id) == current).map_or(0, |i| i+1);
