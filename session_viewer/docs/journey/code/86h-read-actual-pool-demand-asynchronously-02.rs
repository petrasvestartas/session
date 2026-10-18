
    /// Copy the first tile record into the readback buffer.
    pub fn copy(&mut self, encoder: &mut wgpu::CommandEncoder, tiles: &wgpu::Buffer) ->bool {
        if self.inflight {
            return false;
        }

        encoder.copy_buffer_to_buffer(tiles, 0, &self.buffer, 0, 16);
        self.copied = true;
        true
    }

