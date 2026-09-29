        let near = (self.distance * 0.001).max(self.radius * 0.0001);
        let far = self.distance + self.radius * 2.0;
        let projection = Xform::perspective(60.0_f64.to_radians(), self.aspect, near, far);
