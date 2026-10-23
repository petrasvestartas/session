        self.ids.capacity() as u64 * 8
    }
    pub fn clipping_bytes(&self) -> [u64; 2] {
        if self.rows.objects.is_empty(){[0,0]}else{[self.objects.size(),self.rows.objects.capacity() as u64]}
    }
}
