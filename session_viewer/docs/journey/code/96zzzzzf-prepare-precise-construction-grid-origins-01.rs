    pub fn matrix(self)->[f32;16] {self.matrix64().to_f32()}
    pub fn matrix64(self)->session_rust::Xform {
        let (x, y) = self.axes();
        let z = cross([x[0], x[1], x[2]], [y[0], y[1], y[2]]);
        let o = self.origin();
        session_rust::Xform::from_matrix([
            x[0],
            x[1],
            x[2],
            0.0,
            y[0],
            y[1],
            y[2],
            0.0,
            z[0],
            z[1],
            z[2],
            0.0,
            o[0],
            o[1],
            o[2],
            1.0,
        ])
    }