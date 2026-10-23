            });
        }
        self.heads.prepare(device,1,true);self.planes.prepare(device,1,true);
        if self.points_visible{self.dots.prepare(device,1,true);}if self.vertices_visible{self.spheres.prepare(device,1,true);}
        self.update_inside(queue,matrix);
        let line = self.line_uniform(matrix,window.full,density,1.0);
        let (matrix, line) = ribbon::pick_frame(matrix, line, window);
        queue.write_buffer(
