        sources.marker_runs.push(crate::ink_ranges::VERTICES,0,vertex_end);
        sources.marker_runs.push(crate::ink_ranges::POINTS,vertex_end,point_end);
        sources.marker_runs.push(crate::ink_ranges::CONTROLS,point_end,sources.marker_ids.len()as u32);
        for row in scene.sheets().iter().filter