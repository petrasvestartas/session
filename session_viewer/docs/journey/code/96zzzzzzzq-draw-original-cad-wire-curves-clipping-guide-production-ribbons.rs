        if crate::clipping_geometry::is_clipping(source) {
            let index=out.object_ids.len()as u32;add(&mut out,row.id,&row.model,4096);
            for (edge,ends) in row.mesh.parts.endpoints.iter().enumerate() {
                let (width,colour)=row.mesh.parts.pen(edge);if width==0. {continue;}
                let colour=scene.view.edges.get(&row.id).map_or(colour,|c|[c[0]as f32/255.,c[1]as f32/255.,c[2]as f32/255.,1.]);
                segments.ribbons.push(walk::CylinderSegment{p0:ends[0].map(|v|v as f32),p1:ends[1].map(|v|v as f32),radius:crate::production_ribbon::encode_width(width as f64),instance_id:index,color:crate::production_ribbon::pack_rgba(colour),facing:walk::FACING_UNKNOWN});
                out.ids.push(pick(scene,sources,row.id,Part::Object)?);out.edges.push(u32::MAX);out.owners.push(row.id.value());
            }
            continue;
        }
