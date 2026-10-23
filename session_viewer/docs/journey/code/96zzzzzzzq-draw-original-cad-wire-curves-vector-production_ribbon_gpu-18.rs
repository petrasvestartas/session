        ) {
            if let Some(pipeline) = self.pick.borrow().as_ref() {
                self.draw_with(pass, frame, pipeline);
                if self.visible&&mode==crate::pick_sources::Mode::Object {self.heads.draw(pass,&frame.groups,true);}
            }
        }
    }

    pub fn translate(&self,pixel:[u32;2])->[u32;2]{pixel}
    pub fn path_usage(&self)->[u64;4]{[(self.rows.owners.len()-self.rows.line_count)as u64,self.rows.segments.len()as u64,self.uploads,self.uploaded_bytes]}
    pub fn usage(&self) -> [u64; 2] {
        [self.rows.line_count as u64, self.segments.size()]
    }
    pub fn pick_bytes(&self) -> u64 {
        self.ids.capacity() as u64 * 8
