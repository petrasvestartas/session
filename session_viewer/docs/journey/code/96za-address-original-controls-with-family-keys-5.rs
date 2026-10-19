            Part::SourceControl(key) => scene.control_markers().iter().any(|m| m.parent == self.owner && m.part == Part::SourceControl(key)),
            Part::Object => true,