            Part::SourceControl(key) => format!("Object {} · original control {key:?}", self.owner.value()),
            Part::Object => format!("Object {}", self.owner.value()),