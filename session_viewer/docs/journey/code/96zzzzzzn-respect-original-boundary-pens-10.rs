                let (width,colour)=row.mesh.parts.pen(index);
                let width=boundary_width.unwrap_or(width);
                if width==0. {continue;}
                let ends=ends.map(|p|