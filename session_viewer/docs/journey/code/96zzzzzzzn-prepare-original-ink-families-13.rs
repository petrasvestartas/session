                let ids = if line_frame {[owner,0]}else{append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?};
                let (width,colour)=row.mesh.parts.pen(index);