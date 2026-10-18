            if row.cad.is_none()&&row.surface.is_none()&&!derived {
                for(i,&address)in row.mesh.parts.dots.iter().enumerate(){
                    let p=row.mesh.parts.endpoints[address as usize/2][address as usize%2];
                    sources.mesh_markers.push(crate::marker::Marker{center:model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32(),diameter:1.,colour:edge_colour.unwrap_or_else(||row.mesh.parts.dot_colour(i))});
                    sources.marker_ids.push([owner,0]);
                }
            }
            let flag=if planes.contains(&row.id){4096}else{0};