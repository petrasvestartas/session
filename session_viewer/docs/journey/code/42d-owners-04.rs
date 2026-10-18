            .expect("Prepared path placement fits the display range");
        self.strokes.set(&self.device, scene.lines().iter().filter(|row| row.visible()).map(crate::scene::LineObject::stroke))
            .expect("Prepared line placement fits the display range");
        let controls = scene.control_markers();
        self.markers.set_ids(&self.device, scene.points().iter().filter(|r| r.visible()).map(|r|
            [if r.prepared.source.is_locked { 0 } else { r.id.value() }, 1]).chain(controls.iter().map(|r|
                [scene.pick_id(r.parent.value()).map_or(0, ObjectId::value), 1]))) .expect("Marker IDs match markers");
        self.paths.set_ids(&self.device, scene.paths().iter().filter(|r| r.visible()).flat_map(|r|
            vec![[if r.prepared.source.locked() { 0 } else { r.id.value() }, 1]; r.segments().len()])) .expect("Path IDs match segments");
        self.strokes.set_ids(&self.device, scene.lines().iter().filter(|r| r.visible()).map(|r|
            [if r.prepared.source.is_locked { 0 } else { r.id.value() }, 1])) .expect("Line IDs match lines");
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene.objects().iter().filter(|row| row.visible()) {
