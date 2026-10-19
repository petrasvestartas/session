            if !scene.forest.derived(row.id) {
                for &key in &row.mesh.parts.faces {
                    append(&mut sources.faces,Kind::Face,Target{owner:row.id,part:Part::Face(key)})?;
                }
            }
