        None
    }

    pub fn begin(&mut self, handle: Handle, from: &Point, dir: &Vector) -> Option<Drag> {
        let drag = match handle {
            Handle::Translate(axis) => Drag {
                handle,
                grabbed: closest_on_axis(from, dir, &self.origin, &axis.unit())?,
                angle: 0.0,
                reach: 1.0,
                plane: axis.unit(),
            },
            Handle::Rotate(axis) => Drag {
                handle,
                grabbed: self.origin.clone(),
                angle: angle_in_plane(
                    &plane_hit(from, dir, &self.origin, &axis.unit())?,
                    &self.origin,
                    axis,
                ),
                reach: 1.0,
                plane: axis.unit(),
            },
            Handle::Scale(axis) => {
                let p = closest_on_axis(from, dir, &self.origin, &axis.unit())?;
                let reach = (&p - &self.origin).dot(&axis.unit());
                Drag {
                    handle,
                    grabbed: p,
                    angle: 0.0,
                    reach: nonzero(reach),
                    plane: axis.unit(),
                }
            }
            Handle::ScaleUniform => {
                let normal = facing(dir);
                let p = plane_hit(from, dir, &self.origin, &normal)?;
                let reach = (&p - &self.origin).magnitude();
                Drag {
                    handle,
                    grabbed: p,
                    angle: 0.0,
                    reach: nonzero(reach),
                    plane: normal,
                }
            }
        };
        self.drag = Some(drag.clone());
        Some(drag)
    }

}

fn typed_factor(value: f64) -> f64 {
