    pub fn fit(&mut self, bounds: &crate::bounds::Bounds) {
        let half_y = 30.0_f64.to_radians();
        let half_x = (self.aspect * half_y.tan()).atan();
        let radius = bounds.radius();
        self.radius = if radius > 0.0 { radius } else { 1.0 };
        self.target = bounds.centre();
        self.distance = self.radius * 1.1 / half_x.min(half_y).sin();
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
