        if mode==crate::pick_sources::Mode::Object {
            if self.points_visible{self.dots.draw(pass,&frame.groups,true);}if self.vertices_visible{self.spheres.draw(pass,&frame.groups,true);}
        }
        if matches!(mode,crate::pick_sources::Mode::Object|crate::pick_sources::Mode::Edge) {
            if let Some(pipeline)=self.pick.borrow().as_ref(){self.draw_with(pass,frame,pipeline);}
            if self.visible&&mode==crate::pick_sources::Mode::Object {
                self.heads.draw(pass,&frame.groups,true);
                self.planes.draw(pass,&frame.groups,true);
            }
