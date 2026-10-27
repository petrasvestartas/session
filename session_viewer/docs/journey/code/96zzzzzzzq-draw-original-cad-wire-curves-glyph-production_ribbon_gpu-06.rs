        }
        if !self.rows.owners.is_empty(){self.prepare_color(device);}
        self.heads.prepare(device,self.samples,false);self.planes.prepare(device,self.samples,false);
        if self.points_visible{self.dots.prepare(device,self.samples,false);}if self.vertices_visible{self.spheres.prepare(device,self.samples,false);}
        self.update_inside(queue,matrix);
        let bytes = matrix
            .into_iter()
            .flat_map(f32::to_ne_bytes)
