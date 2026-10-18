                let mut object = row.clone(); object.mesh = Rc::new(display);
                self.mesh = Some(GpuMesh::upload(device, layout, &object, true));
            }
            Part::SurfaceEdge(edge)=>{
                let row=scene.objects().iter().find(|r|r.id==target.owner).unwrap();let surface=row.surface.as_ref().unwrap();let boundary=surface.grid.boundaries.iter().find(|b|b.edge==edge).unwrap();
                for ends in &boundary.segments {let ends=ends.map(|index|{let p=surface.grid.samples[index as usize].point;row.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32()});strokes.push(crate::stroke::Stroke {start:ends[0],end:ends[1],colour:yellow,width:3.});}
            }
            Part::MeshEdge(a, b) => {
