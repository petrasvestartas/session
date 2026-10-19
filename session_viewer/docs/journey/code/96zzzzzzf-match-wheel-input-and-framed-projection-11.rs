    pub fn set_projection_framed(&mut self,mode:Projection,bounds:Option<&crate::bounds::Bounds>) {
        if self.projection==mode {return;}
        self.projection=mode;
        let Some(bounds)=bounds.filter(|b|mode==Projection::Perspective&&b.valid()) else{return;};
        let right=self.orientation.rotate_vector(Vector::x_axis());let up=self.orientation.rotate_vector(Vector::z_axis());let forward=self.orientation.rotate_vector(Vector::y_axis());
        let h=self.distance*HALF_FOV.tan();let w=h*self.aspect;let shift=self.shift();let mut clipped=None;
        for corner in bounds.corners() {
            let p: [f64;3]=std::array::from_fn(|i|corner[i]-self.target[i]);let dot=|axis:&Vector|(0..3).map(|i|p[i]*axis[i]).sum::<f64>();
            let x=dot(&right).clamp((-1.-shift[0])*w,(1.-shift[0])*w);let y=dot(&up).clamp((-1.-shift[1])*h,(1.-shift[1])*h);let z=dot(&forward);
            let point=std::array::from_fn(|i|self.target[i]+x*right[i]+y*up[i]+z*forward[i]);
            if let Some(bounds)=&mut clipped {crate::bounds::Bounds::include(bounds,point);}else{clipped=Some(crate::bounds::Bounds::point(point));}
        }
        self.fit(&clipped.unwrap());
    }

    pub fn grow_extent(&mut self,bounds:&crate::bounds::Bounds) {