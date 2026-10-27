    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        if !self.visible {return;}
        if let Some(frame)=self.frame.borrow().as_ref() {
            if let Some(color)=self.color.borrow().as_ref(){self.draw_with(pass,frame,&color.active);}
            self.heads.draw(pass,&frame.groups,false);
            self.planes.draw(pass,&frame.groups,false);
        }
    }

    pub fn pick(&self,pass:&mut wgpu::RenderPass<'_>,frame:&Frame,mode:crate::pick_sources::Mode) {
        if !self.visible {return;}
        if matches!(mode,crate::pick_sources::Mode::Object|crate::pick_sources::Mode::Edge) {
            if let Some(pipeline)=self.pick.borrow().as_ref(){self.draw_with(pass,frame,pipeline);}
            if mode==crate::pick_sources::Mode::Object {
                self.heads.draw(pass,&frame.groups,true);
                self.planes.draw(pass,&frame.groups,true);
            }
        }
    }
    fn line_uniform(&self,matrix:[f32;16],size:[u32;2],density:f32,opacity:f32)->Vec<u8> {
        let mut bytes=ribbon::line_uniform(matrix,size,density,opacity,self.anchor);
        bytes[76..80].copy_from_slice(&self.plane_size.to_ne_bytes());bytes
    }
    pub fn plane_usage(&self)->[u64;2]{[(self.rows.planes.len()/48)as u64,self.rows.planes.len()as u64]}

    pub fn translate(&self,pixel:[u32;2])->[u32;2]{pixel}
    pub fn path_usage(&self)->[u64;4]{[(self.rows.owners.len()-self.rows.line_count)as u64,self.rows.segments.len()as u64,self.uploads,self.uploaded_bytes]}
