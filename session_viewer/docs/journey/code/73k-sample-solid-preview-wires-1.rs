    /// Local to world.
    pub fn to_xform(&self) -> Xform {
        Xform::frame_to_world(&self.origin, &self.x, &self.y, &self.z)
    }

    /// A closed ellipse of radii `rx`, `ry` at height `w`.
    pub fn oval(&self, rx: f64, ry: f64, w: f64) -> Vec<Point> {
        (0..=RING)
            .map(|i| {
                let angle = std::f64::consts::TAU * i as f64 / RING as f64;
                self.at(rx * angle.cos(), ry * angle.sin(), w)
            })
            .collect()
    }

    /// A closed circle at height `w`.
    pub fn ring(&self, radius: f64, w: f64) -> Vec<Point> {
        self.oval(radius, radius, w)
    }

    /// A closed rectangle centered on the axis at height `w`.
    pub fn rectangle(&self, sx: f64, sy: f64, w: f64) -> Vec<Point> {
        let corner = self.at(-sx * 0.5, -sy * 0.5, w);
        Polyline::rectangle(&corner, &self.x, &self.y, sx, sy, true).get_points()
    }

    /// A box standing on the plane, `h` along the normal: two rectangles and four edges.
    pub fn cuboid(&self, sx: f64, sy: f64, h: f64) -> Vec<Vec<Point>> {
        let mut wires = vec![self.rectangle(sx, sy, 0.0), self.rectangle(sx, sy, h)];

        for (u, v) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
            wires.push(vec![
                self.at(u * sx * 0.5, v * sy * 0.5, 0.0),
                self.at(u * sx * 0.5, v * sy * 0.5, h),
            ]);
        }

        wires
    }

}
