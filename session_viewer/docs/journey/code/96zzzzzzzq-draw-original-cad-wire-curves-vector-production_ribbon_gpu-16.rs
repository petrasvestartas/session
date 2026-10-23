                )
            });
        }
        self.heads.prepare(device,1,true);
        let line = ribbon::line_uniform(matrix, window.full, density, 1.0, self.anchor);
        let (matrix, line) = ribbon::pick_frame(matrix, line, window);
        queue.write_buffer(
