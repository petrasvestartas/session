
    pub fn point_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.markers.usage();
        let glyph=self.strokes.glyph_usage();[count+glyph[0],bytes+glyph[1],self.markers.uploads,self.markers.uploaded_bytes]
    }

    #[cfg(test)]
