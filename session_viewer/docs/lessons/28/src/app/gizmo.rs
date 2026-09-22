use session_rust::intersection::{line_line_parameters, line_plane};
use session_rust::{Line, Plane, Point, Vector, Xform};

/// Arm length in CSS pixels.
pub const ARM: f64 = 96.0;

/// Distance of the scale balls along each arm.
pub const BALL_AT: f64 = ARM * 0.5;

/// Grab radius in pixels.
const GRAB: f64 = 8.0;

/// Radius of the centre ball.
pub const HUB: f64 = 6.0;

/// Exponent that slows a scale drag near the centre.
const SCALE_SOFTENING: f64 = 0.5;

/// Smallest scale factor allowed.
const MIN_SCALE: f64 = 0.01;

/// One grabbable part of the gizmo.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Handle {
    Translate(Axis), // an arm
    Rotate(Axis),    // an arc
    Scale(Axis),     // a ball on an arm
    ScaleUniform,    // the centre ball
}

/// A world axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    /// The unit vector along this axis.
    pub fn unit(self) -> Vector {
        match self {
            Axis::X => Vector::new(1.0, 0.0, 0.0),
            Axis::Y => Vector::new(0.0, 1.0, 0.0),
            Axis::Z => Vector::new(0.0, 0.0, 1.0),
        }
    }

    /// The other two axes.
    fn others(self) -> (Vector, Vector) {
        match self {
            Axis::X => (Axis::Y.unit(), Axis::Z.unit()),
            Axis::Y => (Axis::Z.unit(), Axis::X.unit()),
            Axis::Z => (Axis::X.unit(), Axis::Y.unit()),
        }
    }
}

impl Handle {
    /// Verb, axis name and unit for the number box.
    pub fn labels(self) -> (&'static str, &'static str, &'static str) {
        let name = |a: Axis| match a {
            Axis::X => "X",
            Axis::Y => "Y",
            Axis::Z => "Z",
        };

        match self {
            Handle::Translate(a) => ("Move", name(a), "mm"),
            Handle::Rotate(a) => ("Rotate", name(a), "deg"),
            Handle::Scale(a) => ("Scale", name(a), "factor"),
            Handle::ScaleUniform => ("Scale", "", "factor"),
        }
    }
}

/// What a drag remembers from its grab.
#[derive(Clone, Debug)]
pub struct Drag {
    pub handle: Handle, // the handle being dragged
    grabbed: Point,     // where the grab landed
    angle: f64,         // grab angle about the axis, for rotate
    reach: f64,         // grab distance from the centre, for scale
    plane: Vector,      // plane normal a uniform scale is measured in
}

/// The gizmo's position and state.
pub struct Gizmo {
    pub origin: Point,           // centre in world
    pub hovered: Option<Handle>, // handle under the pointer
    pub drag: Option<Drag>,      // drag in progress
}

impl Gizmo {
    /// A gizmo at `origin`, idle.
    pub fn new(origin: Point) -> Self {
        Self {
            origin,
            hovered: None,
            drag: None,
        }
    }

    /// Move the gizmo and end any drag.
    pub fn set_origin(&mut self, origin: Point) {
        self.origin = origin;
        self.hovered = None;
        self.drag = None;
    }

    /// The handle under a ray, tested from the centre outward.
    pub fn hit(&self, from: &Point, dir: &Vector, world_per_px: f64) -> Option<Handle> {
        let s = world_per_px; // pixel sizes to world

        if within(from, dir, &self.origin, HUB * s) {
            return Some(Handle::ScaleUniform);
        }

        // the scale balls
        for axis in [Axis::X, Axis::Y, Axis::Z] {
            if end_on(dir, axis) {
                continue;
            }

            let at = &self.origin + &(&axis.unit() * (BALL_AT * s));

            if within(from, dir, &at, GRAB * s) {
                return Some(Handle::Scale(axis));
            }
        }

        // the arms
        for axis in [Axis::X, Axis::Y, Axis::Z] {
            if end_on(dir, axis) {
                continue;
            }

            if let Some(p) = closest_on_axis(from, dir, &self.origin, &axis.unit()) {
                let t = (&p - &self.origin).dot(&axis.unit());

                // The arm's grabbable run starts where the hub ends: inside it all three arms
                if (HUB * s..=ARM * s).contains(&t) && within(from, dir, &p, GRAB * s) {
                    return Some(Handle::Translate(axis));
                }
            }
        }

        // the rotate arcs, in the quadrant without arms
        for axis in [Axis::X, Axis::Y, Axis::Z] {
            if let Some(p) = plane_hit(from, dir, &self.origin, &axis.unit()) {
                let d = &p - &self.origin;
                let (u, v) = axis.others();

                // A quarter arc, in the quadrant the arms and balls do not occupy. Sharing a
                if d.dot(&u) < 0.0 && d.dot(&v) < 0.0 && (d.magnitude() - ARM * s).abs() < GRAB * s
                {
                    return Some(Handle::Rotate(axis));
                }
            }
        }

        None
    }

    /// Start a drag on `handle`; None when the ray runs along the axis.
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
                let normal = facing(dir); // measure in the plane facing the view
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

    /// The transform from the grab to where the ray is now.
    pub fn update(&self, drag: &Drag, from: &Point, dir: &Vector) -> Option<Xform> {
        match drag.handle {
            Handle::Translate(axis) => {
                let now = closest_on_axis(from, dir, &self.origin, &axis.unit())?;
                let d = &now - &drag.grabbed;
                Some(Xform::translation(d[0], d[1], d[2]))
            }
            Handle::Rotate(axis) => {
                let now = plane_hit(from, dir, &self.origin, &axis.unit())?;
                let turned = angle_in_plane(&now, &self.origin, axis) - drag.angle;
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

    /// The transform for a typed number: mm, degrees or a factor.
    pub fn typed(&self, handle: Handle, value: f64) -> Xform {
        match handle {
            Handle::Translate(axis) => {
                let u = axis.unit();
                Xform::translation(u[0] * value, u[1] * value, u[2] * value)
            }
            Handle::Rotate(axis) => about(&self.origin, rotation(axis, value.to_radians())),
            Handle::Scale(axis) => {
                let k = value.max(MIN_SCALE);
                let (x, y, z) = match axis {
                    Axis::X => (k, 1.0, 1.0),
                    Axis::Y => (1.0, k, 1.0),
                    Axis::Z => (1.0, 1.0, k),
                };
                about(&self.origin, Xform::scale_xyz(x, y, z))
            }
            Handle::ScaleUniform => {
                let k = value.max(MIN_SCALE);
                about(&self.origin, Xform::scale_xyz(k, k, k))
            }
        }
    }
}

/// A scale factor slowed near the centre, never below the minimum.
fn softened(ratio: f64) -> f64 {
    if !ratio.is_finite() || ratio <= 0.0 {
        return MIN_SCALE;
    }

    ratio.powf(SCALE_SOFTENING).max(MIN_SCALE)
}

/// `v`, or a tiny value with its sign when zero.
fn nonzero(v: f64) -> f64 {
    if v.abs() < 1e-9 {
        1e-9_f64.copysign(if v < 0.0 { -1.0 } else { 1.0 })
    } else {
        v
    }
}

/// The world axis a ray runs most along.
fn facing(dir: &Vector) -> Vector {
    let (x, y, z) = (dir[0].abs(), dir[1].abs(), dir[2].abs());

    if x >= y && x >= z {
        Axis::X.unit()
    } else if y >= z {
        Axis::Y.unit()
    } else {
        Axis::Z.unit()
    }
}

/// True when the ray looks almost straight along the axis.
fn end_on(dir: &Vector, axis: Axis) -> bool {
    dir.dot(&axis.unit()).abs() > 0.97
}

/// The point on the axis closest to the ray, `None` when the two run parallel.
fn closest_on_axis(from: &Point, dir: &Vector, origin: &Point, axis: &Vector) -> Option<Point> {
    let ray = Line::from_point_direction_length(from, dir, 1.0);
    let line = Line::from_point_direction_length(origin, axis, 1.0);
    let (_, t) = line_line_parameters(&ray, &line, 1e-9, false, false)?;

    Some(origin + &(axis * t))
}

/// Where a ray hits the plane through `origin`, in front of the eye.
fn plane_hit(from: &Point, dir: &Vector, origin: &Point, normal: &Vector) -> Option<Point> {
    if dir.dot(normal).abs() < 1e-9 {
        return None; // ray parallel to the plane
    }

    let ray = Line::from_point_direction_length(from, dir, 1.0);
    let plane = Plane::from_point_normal(origin.clone(), normal.clone(), Some(false));
    let hit = line_plane(&ray, &plane, false)?;
    let t = (&hit - from).dot(dir);

    (t > 0.0 && t.is_finite()).then_some(hit)
}

/// True when the ray passes within `radius` of `at`.
fn within(from: &Point, dir: &Vector, at: &Point, radius: f64) -> bool {
    let t = (at - from).dot(dir);

    if t < 0.0 {
        return false;
    }

    let closest = from + &(dir * t);
    (&closest - at).magnitude() <= radius
}

/// Angle of `p` around `axis`, seen from `origin`.
fn angle_in_plane(p: &Point, origin: &Point, axis: Axis) -> f64 {
    let (u, v) = axis.others();
    let d = p - origin;
    d.dot(&v).atan2(d.dot(&u))
}

/// A rotation about one world axis, in radians.
fn rotation(axis: Axis, radians: f64) -> Xform {
    match axis {
        Axis::X => Xform::rotation_x(radians, false),
        Axis::Y => Xform::rotation_y(radians, false),
        Axis::Z => Xform::rotation_z(radians, false),
    }
}

/// `m` applied about `pivot` instead of the origin.
fn about(pivot: &Point, m: Xform) -> Xform {
    let to = Xform::translation(pivot[0], pivot[1], pivot[2]);
    let back = Xform::translation(-pivot[0], -pivot[1], -pivot[2]);

    &(&to * &m) * &back
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCALE: f64 = 1.0; // one world unit per pixel

    /// A gizmo at the origin.
    fn at_origin() -> Gizmo {
        Gizmo::new(Point::new(0.0, 0.0, 0.0))
    }

    /// A ray straight down through (x, y).
    fn down(x: f64, y: f64) -> (Point, Vector) {
        (Point::new(x, y, 500.0), Vector::new(0.0, 0.0, -1.0))
    }

    /// Hub, ball, arm and arc each answer at their own place.
    #[test]
    fn the_handles_do_not_shadow_each_other() {
        let g = at_origin();
        let (f, d) = down(0.0, 0.0);
        assert_eq!(g.hit(&f, &d, SCALE), Some(Handle::ScaleUniform));
        let (f, d) = down(BALL_AT, 0.0);
        assert_eq!(g.hit(&f, &d, SCALE), Some(Handle::Scale(Axis::X)));
        let (f, d) = down(ARM * 0.8, 0.0);
        assert_eq!(g.hit(&f, &d, SCALE), Some(Handle::Translate(Axis::X)));
        // the arcs live in the quadrant the arms do not
        let r = ARM / 2.0_f64.sqrt();
        let (f, d) = down(-r, -r);
        assert_eq!(g.hit(&f, &d, SCALE), Some(Handle::Rotate(Axis::Z)));
        let (f, d) = down(ARM, 0.0);
        assert_eq!(
            g.hit(&f, &d, SCALE),
            Some(Handle::Translate(Axis::X)),
            "the arm tip is not an arc"
        );
        let (f, d) = down(ARM * 3.0, ARM * 3.0);
        assert_eq!(g.hit(&f, &d, SCALE), None);
    }

    /// An axis pointing at the eye cannot be grabbed.
    #[test]
    fn an_axis_seen_end_on_is_not_grabbable() {
        let g = at_origin();
        // just past the hub, where only the Z ball could answer
        let (f, d) = down(-7.0, 0.0);
        assert_eq!(g.hit(&f, &d, SCALE), None);
        let (f, d) = down(BALL_AT, 0.0);
        assert_eq!(g.hit(&f, &d, SCALE), Some(Handle::Scale(Axis::X)));
    }

    /// Handles keep their pixel size at any zoom.
    #[test]
    fn the_widget_is_screen_constant() {
        let g = at_origin();
        let (f, d) = down(BALL_AT * 2.0, 0.0);
        assert_eq!(g.hit(&f, &d, 2.0), Some(Handle::Scale(Axis::X)));
        assert_ne!(g.hit(&f, &d, 1.0), Some(Handle::Scale(Axis::X)));
    }

    /// A translate drag moves only along its axis.
    #[test]
    fn a_translate_drag_moves_along_its_axis_only() {
        let mut g = at_origin();
        let (f0, d0) = down(30.0, 0.0);
        let drag = g
            .begin(Handle::Translate(Axis::X), &f0, &d0)
            .expect("grabbed");
        let (f1, d1) = down(42.0, 0.0);
        let m = g.update(&drag, &f1, &d1).expect("a transform");
        assert!((m.m[12] - 12.0).abs() < 1e-9);
        assert!(m.m[13].abs() < 1e-9 && m.m[14].abs() < 1e-9);
    }

    /// A drag along the viewing direction is refused.
    #[test]
    fn a_drag_down_its_own_axis_refuses() {
        let mut g = at_origin();
        let from = Point::new(-500.0, 0.0, 0.0);
        let dir = Vector::new(1.0, 0.0, 0.0);
        assert!(g.begin(Handle::Translate(Axis::X), &from, &dir).is_none());
    }

    /// A quarter turn about Z takes the x axis onto the y axis.
    #[test]
    fn a_rotate_drag_turns_by_the_angle_swept() {
        let mut g = at_origin();
        let (f0, d0) = down(ARM, 0.0);
        let drag = g.begin(Handle::Rotate(Axis::Z), &f0, &d0).expect("grabbed");
        let (f1, d1) = down(0.0, ARM);
        let m = g.update(&drag, &f1, &d1).expect("a transform");
        // column 0 is the image of the x axis
        assert!((m.m[0]).abs() < 1e-9, "x.x");
        assert!((m.m[1] - 1.0).abs() < 1e-9, "x.y");
    }

    /// A scale starts at 1 and never flips.
    #[test]
    fn a_scale_is_relative_to_the_grab_and_never_collapses() {
        let mut g = at_origin();
        let (f0, d0) = down(BALL_AT, 0.0);
        let drag = g.begin(Handle::Scale(Axis::X), &f0, &d0).expect("grabbed");
        let m = g.update(&drag, &f0, &d0).expect("a transform");
        assert!((m.m[0] - 1.0).abs() < 1e-9, "no movement is no change");

        let (f1, d1) = down(BALL_AT * 4.0, 0.0);
        let grown = g.update(&drag, &f1, &d1).expect("a transform");
        assert!(
            grown.m[0] > 1.0 && grown.m[5] == 1.0 && grown.m[10] == 1.0,
            "one axis only"
        );

        let (f2, d2) = down(-BALL_AT * 4.0, 0.0);
        let flipped = g.update(&drag, &f2, &d2).expect("a transform");
        assert!(flipped.m[0] >= MIN_SCALE, "never mirrors");
    }

    /// A point at the gizmo centre does not move.
    #[test]
    fn transforms_act_about_the_gumball() {
        let g = Gizmo::new(Point::new(100.0, 200.0, 300.0));

        for m in [
            g.typed(Handle::Rotate(Axis::Z), 90.0),
            g.typed(Handle::ScaleUniform, 3.0),
        ] {
            let p = Point::new(100.0, 200.0, 300.0);
            let moved = m.transform_point(&p);

            for i in 0..3 {
                assert!((moved[i] - p[i]).abs() < 1e-9, "axis {i}");
            }
        }
    }

    /// Typed values give the same transforms as drags.
    #[test]
    fn typed_values_match_the_drag_and_are_clamped() {
        let g = at_origin();
        let m = g.typed(Handle::Translate(Axis::Y), 12.5);
        assert_eq!([m.m[12], m.m[13], m.m[14]], [0.0, 12.5, 0.0]);
        let z = g.typed(Handle::ScaleUniform, 0.0);
        assert!(z.m[0] >= MIN_SCALE && z.m[5] >= MIN_SCALE && z.m[10] >= MIN_SCALE);
    }
}
