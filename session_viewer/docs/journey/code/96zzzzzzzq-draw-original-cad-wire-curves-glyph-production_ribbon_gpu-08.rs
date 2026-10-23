    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        if let Some(frame)=self.frame.borrow().as_ref() {
            if let Some(color)=self.color.borrow().as_ref(){self.draw_with(pass,frame,&color.active);}
            if self.visible{self.heads.draw(pass,&frame.groups,false);self.planes.draw(pass,&frame.groups,false);}

        }
    }

    pub fn pick(&self,pass:&mut wgpu::RenderPass<'_>,frame:&Frame,mode:crate::pick_sources::Mode) {
        if mode==crate::pick_sources::Mode::Object {
            if self.points_visible{self.dots.draw(pass,&frame.groups,true);}if self.vertices_visible{self.spheres.draw(pass,&frame.groups,true);}
        }
        if !self.visible {return;}
        if matches!(mode,crate::pick_sources::Mode::Object|crate::pick_sources::Mode::Edge) {
            if let Some(pipeline)=self.pick.borrow().as_ref(){self.draw_with(pass,frame,pipeline);}
