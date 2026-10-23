        let mut bytes=self.settings.line(matrix,size,density,opacity,self.anchor);
        bytes[76..80].copy_from_slice(&self.plane_size.to_ne_bytes());bytes
    }
    pub fn mesh_usage(&self)->[u64;2]{let count=(self.rows.owners.len()-self.rows.mesh_from)as u64;[count,count*52]}
    pub fn plane_usage(&self)->[u64;2]{[(self.rows.planes.len()/48)as u64,self.rows.planes.len()as u64]}

    pub fn translate(&self,pixel:[u32;2])->[u32;2]{pixel}
    pub fn path_usage(&self)->[u64;4]{[(self.rows.mesh_from-self.rows.line_count)as u64,self.rows.segments.len()as u64,self.uploads,self.uploaded_bytes]}
    pub fn usage(&self) -> [u64; 2] {
        [self.rows.line_count as u64, self.segments.size()]
    }
