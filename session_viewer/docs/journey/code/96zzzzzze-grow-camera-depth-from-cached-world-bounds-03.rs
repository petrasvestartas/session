    pub fn grow_extent(&mut self,bounds:&crate::bounds::Bounds) {
        if bounds.min.into_iter().chain(bounds.max).any(|v|!v.is_finite())||(0..3).any(|i|bounds.min[i]>bounds.max[i]) {return;}
        let extent=bounds.corners().into_iter().map(|p|(0..3).map(|i|(p[i]-self.target[i]).powi(2)).sum::<f64>().sqrt()).fold(0.,f64::max);
        if extent.is_finite()&&extent>self.radius {self.radius=extent;}
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {