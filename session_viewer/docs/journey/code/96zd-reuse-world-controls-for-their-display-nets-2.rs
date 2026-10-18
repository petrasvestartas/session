        for (owner, stroke) in scene.control_nets() {
            sources.mesh_ink.push(stroke);
            sources.mesh_ids.push([owner.value(), 0]);
        }
        for row in scene.control_markers() {