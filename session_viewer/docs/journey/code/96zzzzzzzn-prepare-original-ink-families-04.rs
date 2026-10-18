            let kind=if line_frame {crate::ink_ranges::LINES}else{crate::ink_ranges::EDGES};
            sources.mesh_runs.push(kind,ink_start,sources.mesh_ink.len()as u32);
        }
        for row in scene.lines()