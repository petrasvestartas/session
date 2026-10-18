        }
    }

    pub fn hit(&self, from: &Point, dir: &Vector, world_per_px: f64) -> Option<Handle> {
        self.hit_with_radius(from, dir, world_per_px, GRAB)
    }

    pub fn hit_with_radius(
        &self,
        from: &Point,
        dir: &Vector,
        world_per_px: f64,
        radius: f64,
    ) -> Option<Handle> {
        let s = world_per_px;
        let grab = radius.max(GRAB);

        if within(from, dir, &self.origin, HUB * s) {
            return Some(Handle::ScaleUniform);
        }

        for axis in [Axis::X, Axis::Y, Axis::Z] {
            if end_on(dir, axis) {
                continue;
            }

            let at = &self.origin + &(&axis.unit() * (BALL_AT * s));

            if within(from, dir, &at, grab * s) {
                return Some(Handle::Scale(axis));
            }
        }

        for axis in [Axis::X, Axis::Y, Axis::Z] {
            if end_on(dir, axis) {
                continue;
            }

            if let Some(p) = closest_on_axis(from, dir, &self.origin, &axis.unit()) {
                let t = (&p - &self.origin).dot(&axis.unit());

                if (HUB * s..=ARM * s).contains(&t) && within(from, dir, &p, grab * s) {
                    return Some(Handle::Translate(axis));
                }
            }
        }

        for axis in [Axis::X, Axis::Y, Axis::Z] {
            if let Some(p) = plane_hit(from, dir, &self.origin, &axis.unit()) {
                let d = &p - &self.origin;
                let (u, v) = axis.others();

                if d.dot(&u) < 0.0 && d.dot(&v) < 0.0 && (d.magnitude() - ARM * s).abs() < grab * s
                {
                    return Some(Handle::Rotate(axis));
                }
            }
        }

        None
    }

}

fn typed_factor(value: f64) -> f64 {
