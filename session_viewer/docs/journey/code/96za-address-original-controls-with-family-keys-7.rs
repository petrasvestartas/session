Part::Control(_) | Part::SourceControl(_) => {
                let marker = scene.control_markers().into_iter().find(|m| m.parent == target.owner && m.part == target.part).unwrap();