        queue.write_buffer(
            &self.line,
            0,
            &self.line_uniform(matrix,size,density,opacity),
        );
        *self.frame.borrow_mut() = Some(self.bindings(
            device,
