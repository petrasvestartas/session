        let vertices=self.grid.samples.iter().map(|s|[s.point[0] as f32,s.point[1] as f32,s.point[2] as f32,colour[0],colour[1],colour[2]]).collect();
        let normals=(0..self.grid.samples.len()).map(|i|self.grid.normal(i)).collect::<Result<Vec<_>,_>>()?;
        let mut display=crate::mesh::Mesh::new(vertices,self.grid.indices.clone())?.with_normals(normals)?;
        display.parts.ink=crate::production_mesh_ink::surface(&self.source,&self.grid);
        display.parts.faces=vec![0];display.parts.triangles=vec![0;self.grid.indices.len()/3];
        for boundary in &self.grid.boundaries {for &[a,b] in &boundary.segments {display.parts.edges.push((a as usize,b as usize));display.parts.endpoints.push([self.grid.samples[a as usize].point,self.grid.samples[b as usize].point]);}}
        Ok(crate::prepared::PreparedMesh {geometry:std::rc::Rc::new(source),display,source:None,model:session_rust::Xform::identity()})
