    pub fn payload_bytes(&self) -> usize {
        self.vertices.capacity() * std::mem::size_of::<[f32; 6]>()
            + self.indices.capacity() * std::mem::size_of::<u16>()
    }

    pub fn vertices(&self) -> &[[f32; 6]] {
