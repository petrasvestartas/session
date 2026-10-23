                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: edge_colour.unwrap_or(colour), width });
                sources.mesh_ids.push([owner,if derived{0}else{ids[1]}]);
                sources.mesh_metadata.extend([flag,facing]);
                sources.mesh_legacy.push(if row.mesh.parts.ink.active{0}else if line_frame{crate::ink_ranges::LINES}else{crate::ink_ranges::EDGES});
            }
            let kind=if line_frame {crate::ink_ranges::LINES}else{crate::ink_ranges::EDGES};
            sources.mesh_runs.push(kind,ink_start,sources.mesh_ink.len()as u32);
