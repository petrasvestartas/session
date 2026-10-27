        queue.write_buffer(
            &self.line,
            0,
            &self.settings.line(matrix, size, density, opacity, self.anchor),
        );
        *self.frame.borrow_mut() = Some(self.bindings(
            device,
