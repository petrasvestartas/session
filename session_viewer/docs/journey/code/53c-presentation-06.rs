            }
        }
        for row in scene.points().iter().filter(|row| scene.visible(row.id)) { sources.marker_ids.push(id(row.id, 1)); }
        for row in scene.control_markers() {
            let ids = append(&mut sources.controls, Kind::Control, Target { owner: row.parent, part: Part::Control(row.control.index) })?;
