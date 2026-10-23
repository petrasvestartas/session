                )
            });
        }
        self.heads.prepare(device,1,true);self.planes.prepare(device,1,true);
        let line = self.line_uniform(matrix,window.full,density,1.0);
        let (matrix, line) = ribbon::pick_frame(matrix, line, window);
        queue.write_buffer(
            &self.pick_matrix,
