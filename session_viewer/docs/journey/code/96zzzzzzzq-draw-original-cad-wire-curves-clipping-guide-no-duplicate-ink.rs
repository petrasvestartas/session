                sources.mesh_legacy.push(if row.mesh.parts.ink.active||(line_frame&&flag==4096){0}else if line_frame{crate::ink_ranges::LINES}else{crate::ink_ranges::EDGES});
