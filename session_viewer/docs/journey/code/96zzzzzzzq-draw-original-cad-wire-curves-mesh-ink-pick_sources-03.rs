            sources.mesh_ink.push(stroke);
            sources.mesh_ids.push([owner.value(), 0]);
            sources.mesh_metadata.extend([0,crate::packed_vertex::NO_NORMAL]);
            sources.mesh_legacy.push(crate::ink_ranges::CONTROLS);
        }
        sources.mesh_runs.push(crate::ink_ranges::CONTROLS,control_start,sources.mesh_ink.len()as u32);
        for row in scene.control_markers_at(anchor) {
