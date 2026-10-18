        Some(drag)
    }

    pub fn update(&self, drag: &Drag, from: &Point, dir: &Vector, snap: bool) -> Option<Xform> {
        match drag.handle {
            Handle::Translate(axis) => {
                let now = closest_on_axis(from, dir, &self.origin, &axis.unit())?;
                let d = &now - &drag.grabbed;
                Some(Xform::translation(d[0], d[1], d[2]))
            }
            Handle::Rotate(axis) => {
                let now = plane_hit(from, dir, &self.origin, &axis.unit())?;
                let turned = angle_in_plane(&now, &self.origin, axis) - drag.angle;
                let turned = if snap {
                    (turned / std::f64::consts::FRAC_PI_2).round() * std::f64::consts::FRAC_PI_2
                } else {
                    turned
                };
                Some(about(&self.origin, rotation(axis, turned)))
            }
            Handle::Scale(axis) => {
                let now = closest_on_axis(from, dir, &self.origin, &axis.unit())?;
                let reach = (&now - &self.origin).dot(&axis.unit());
                let k = softened(reach / drag.reach);
                let (x, y, z) = match axis {
                    Axis::X => (k, 1.0, 1.0),
                    Axis::Y => (1.0, k, 1.0),
                    Axis::Z => (1.0, 1.0, k),
                };
                Some(about(&self.origin, Xform::scale_xyz(x, y, z)))
            }
            Handle::ScaleUniform => {
                let now = plane_hit(from, dir, &self.origin, &drag.plane)?;
                let k = softened((&now - &self.origin).magnitude() / drag.reach);
                Some(about(&self.origin, Xform::scale_xyz(k, k, k)))
            }
        }
    }

    pub fn typed(&self, handle: Handle, value: f64) -> Xform {
        match handle {
            Handle::Translate(axis) => {
                let u = axis.unit();
                Xform::translation(u[0] * value, u[1] * value, u[2] * value)
            }
            Handle::Rotate(axis) => about(&self.origin, rotation(axis, value.to_radians())),
            Handle::Scale(axis) => {
                let k = typed_factor(value);
                let (x, y, z) = match axis {
                    Axis::X => (k, 1.0, 1.0),
                    Axis::Y => (1.0, k, 1.0),
                    Axis::Z => (1.0, 1.0, k),
                };
                about(&self.origin, Xform::scale_xyz(x, y, z))
            }
            Handle::ScaleUniform => {
                let k = typed_factor(value);
                about(&self.origin, Xform::scale_xyz(k, k, k))
            }
        }
    }
}

fn typed_factor(value: f64) -> f64 {
