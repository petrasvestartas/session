        let vertex_end=sources.marker_ids.len()as u32;
        for row in scene.points().iter().filter(|row| scene.visible(row.id)) { sources.marker_ids.push(id(row.id, 1)); }
        let point_end=sources.marker_ids.len()as u32;