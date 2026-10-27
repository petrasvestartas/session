        depth: &wgpu::TextureView,
    ) -> Frame {
        self.pipeline(device, true);
        let line = self.settings.line(matrix, window.full, density, 1.0, self.anchor);
        let (matrix, line) = rows::pick_frame(matrix, line, window);
        queue.write_buffer(
            &self.pick_matrix,
