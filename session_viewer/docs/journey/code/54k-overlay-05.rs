        if over { event.prevent_default(); }
        over
    }

    pub fn free_frame(&self,size:[f64;2])->Option<[f64;4]> {
        if size.iter().any(|v|!v.is_finite()||*v<=0.) {return None;}
        let right=self.tree.rect.map_or(size[0],|r|r.left() as f64).clamp(size[0]*0.1,size[0]);
        let bottom=(self.top as f64).min(size[1]).max(size[1]*0.1);
        Some([0.,0.,right/size[0],bottom/size[1]])
    }

