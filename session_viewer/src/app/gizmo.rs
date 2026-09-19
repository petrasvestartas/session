use session_rust::intersection::{line_line_parameters, line_plane};
use session_rust::{Line, Plane, Point, Vector, Xform};

/// The one length everything else is a fraction of, in CSS pixels.
pub const ARM: f64 = 96.0;

/// Axis scale balls, on the same side as the arrows so pulling out always grows.
pub const BALL_AT: f64 = ARM * 0.5;

/// Grab radius. Wider than the 6 px pick radius because a handle is grabbed, not aimed at.
const GRAB: f64 = 8.0;

/// Uniform-scale ball at the centre.
pub const HUB: f64 = 6.0;

/// Drag softening. A scale that follows the raw distance ratio doubles an object within a few
/// pixels of the centre, where that ratio changes fastest. A square root flattens it without
/// moving its fixed point: 1 is still 1, and any factor is still reachable, just further out.
const SCALE_SOFTENING: f64 = 0.5;

/// The smallest factor a scale may produce. Below this a matrix is singular or mirrors, and
/// either one is baked into geometry for good.
const MIN_SCALE: f64 = 0.01;

/// Which handle, and so which law the drag follows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Handle {
    Translate(Axis),
    Rotate(Axis),
    Scale(Axis),
    ScaleUniform,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn unit(self) -> Vector {
        match self {
            Axis::X => Vector::new(1.0, 0.0, 0.0),
            Axis::Y => Vector::new(0.0, 1.0, 0.0),
            Axis::Z => Vector::new(0.0, 0.0, 1.0),
        }
    }

    /// The two axes spanning the plane this one is normal to.
    fn others(self) -> (Vector, Vector) {
        match self {
            Axis::X => (Axis::Y.unit(), Axis::Z.unit()),
            Axis::Y => (Axis::Z.unit(), Axis::X.unit()),
            Axis::Z => (Axis::X.unit(), Axis::Y.unit()),
        }
    }
}

impl Handle {
    /// (verb, axis, unit) for a numeric entry box. The axis is empty for uniform scale.
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

/// What a drag needs to remember from the moment the handle was grabbed.
#[derive(Clone, Debug)]
pub struct Drag {
    pub handle: Handle,
    grabbed: Point, // Where the grab landed: a point on the axis, or in the handle's plane.
    angle: f64,     // The angle of the grab about the axis, for a rotate.
    reach: f64, // Where the grab was, for a scale, and never zero. SIGNED along the axis for `Scale(axis)`, so grabbing the negative side stores a negative reach and the ratio in `update` still comes out positive; a plain distance for `ScaleUniform`.
    plane: Vector, // The plane a uniform scale is measured in, chosen once at the grab.  Choosing it again from the live ray on every move lets it flip to another world axis mid-drag, and the reach is then measured in a different plane than the one the grab was measured in - the object jumps. Unused by the other handles, which have an axis.
}

/// Where the widget is and what it is doing.
pub struct Gizmo {
    pub origin: Point,
    pub hovered: Option<Handle>,
    pub drag: Option<Drag>,
}

impl Gizmo {
    pub fn new(origin: Point) -> Self {
        Self {
            origin,
            hovered: None,
            drag: None,
        }
    }

    /// Move the widget and forget any interaction: a new selection is not a continued drag.
    pub fn set_origin(&mut self, origin: Point) {
        self.origin = origin;
        self.hovered = None;
        self.drag = None;
    }

    /// The handle under a ray, or none.
    ///
    /// Order is the disambiguation, not geometry: near the centre the hub, the balls and the
    /// three arms all overlap, so they are tested outward from the centre and the first hit
    /// wins. `world_per_px` converts the CSS-pixel sizes above into world units at this depth.
    pub fn hit(&self, from: &Point, dir: &Vector, world_per_px: f64) -> Option<Handle> {
        self.hit_with_radius(from, dir, world_per_px, GRAB)
    }

    /// Touch widens the hit tolerance without changing the visible handle positions.
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

                // The arm's grabbable run starts where the hub ends: inside it all three arms
                // overlap, and whichever was tested first would win a click aimed at the centre.
                if (HUB * s..=ARM * s).contains(&t) && within(from, dir, &p, grab * s) {
                    return Some(Handle::Translate(axis));
                }
            }
        }

        for axis in [Axis::X, Axis::Y, Axis::Z] {
            if let Some(p) = plane_hit(from, dir, &self.origin, &axis.unit()) {
                let d = &p - &self.origin;
                let (u, v) = axis.others();

                // A quarter arc, in the quadrant the arms and balls do not occupy. Sharing a
                // radius with the arm tip would make the two ambiguous exactly where a reader
                // aims for one of them.
                if d.dot(&u) < 0.0 && d.dot(&v) < 0.0 && (d.magnitude() - ARM * s).abs() < grab * s
                {
                    return Some(Handle::Rotate(axis));
                }
            }
        }

        None
    }

    /// Grab a handle. `None` when the ray cannot resolve against it, which happens when you
    /// sight straight down a translate axis: there is no answer, so there is no drag.
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

    /// The world transform this drag now stands for, as a column-major 4x4.
    ///
    /// Always measured from the grab, never accumulated frame to frame: a drag that adds a
    /// delta per move drifts, and a dropped frame changes the result.
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
                // The plane the grab was measured in, not one chosen again from this frame's
                // ray: a plane that flips mid-drag makes the object jump.
                let now = plane_hit(from, dir, &self.origin, &drag.plane)?;
                let k = softened((&now - &self.origin).magnitude() / drag.reach);
                Some(about(&self.origin, Xform::scale_xyz(k, k, k)))
            }
        }
    }

    /// The transform for a value typed into the numeric box, with the same meaning a drag has:
    /// a move is millimetres, a rotation degrees, a scale a factor.
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

// ═══════════════════════════════════════════════════════════════════════════
// Ray geometry
// ═══════════════════════════════════════════════════════════════════════════

/// A factor that never collapses or mirrors, softened so the drag is usable near the centre.
fn softened(ratio: f64) -> f64 {
    if !ratio.is_finite() || ratio <= 0.0 {
        return MIN_SCALE;
    }

    ratio.powf(SCALE_SOFTENING).max(MIN_SCALE)
}

fn nonzero(v: f64) -> f64 {
    if v.abs() < 1e-9 {
        1e-9_f64.copysign(if v < 0.0 { -1.0 } else { 1.0 })
    } else {
        v
    }
}

/// The world axis a ray runs most along: the plane to measure a uniform scale in, chosen so the
/// ray is never near-parallel to it.
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

/// The point on the axis closest to the ray. `None` when the ray runs along the axis, which is
/// the classic gumball failure: without this guard a drag down the axis throws the object away.
/// An axis seen end-on: within about fourteen degrees of the view direction.
///
/// Its arm and its ball then sit on top of the hub in screen terms, so a cursor a few pixels
/// from the centre grabs the axis rather than what it is pointing at - and `begin` would refuse
/// the drag anyway, because there is no answer to where along an axis a ray that runs down it
/// is. Skipping the handle is the same refusal, made early enough that the click falls through.
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

/// Where a ray meets the plane through `origin` with this normal, in front of the ray.
fn plane_hit(from: &Point, dir: &Vector, origin: &Point, normal: &Vector) -> Option<Point> {
    if dir.dot(normal).abs() < 1e-9 {
        return None;
    }

    let ray = Line::from_point_direction_length(from, dir, 1.0);
    let plane = Plane::from_point_normal(origin.clone(), normal.clone(), Some(false));
    let hit = line_plane(&ray, &plane, false)?;
    let t = (&hit - from).dot(dir);

    (t > 0.0 && t.is_finite()).then_some(hit)
}

/// Whether a ray passes within `radius` of a point.
fn within(from: &Point, dir: &Vector, at: &Point, radius: f64) -> bool {
    let t = (at - from).dot(dir);

    if t < 0.0 {
        return false;
    }

    let closest = from + &(dir * t);
    (&closest - at).magnitude() <= radius
}

/// The angle of a point about an axis, measured from that axis's first companion.
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

/// `m` applied about `pivot` rather than the world origin.
fn about(pivot: &Point, m: Xform) -> Xform {
    let to = Xform::translation(pivot[0], pivot[1], pivot[2]);
    let back = Xform::translation(-pivot[0], -pivot[1], -pivot[2]);

    &(&to * &m) * &back
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCALE: f64 = 1.0; // one world unit per CSS pixel keeps the numbers readable

    fn at_origin() -> Gizmo {
        Gizmo::new(Point::new(0.0, 0.0, 0.0))
    }

    /// A ray from above, pointing down, through a world x/y.
    fn down(x: f64, y: f64) -> (Point, Vector) {
        (Point::new(x, y, 500.0), Vector::new(0.0, 0.0, -1.0))
    }

    /// The three collinear handles on one axis are told apart by where the ray passes, and the
    /// hub wins at the centre where all of them overlap.
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

    /// An axis pointing at the camera is seen end-on: its ball and its arm project onto the hub,
    /// so a cursor a few pixels from the centre would grab Z in a Top view rather than what it
    /// is pointing at. The handle is skipped, and the click falls through to the picker.
    #[test]
    fn an_axis_seen_end_on_is_not_grabbable() {
        let g = at_origin();
        // Seven pixels from the centre along -x, looking down Z: past the hub, clear of both
        // drawn arms (which run along +x and +y), and inside the Z ball's grab radius - the Z
        // ball sits 36 units up the axis, which in this view is straight at the eye.
        let (f, d) = down(-7.0, 0.0);
        assert_eq!(g.hit(&f, &d, SCALE), None);
        // The other two axes are across the view and still answer.
        let (f, d) = down(BALL_AT, 0.0);
        assert_eq!(g.hit(&f, &d, SCALE), Some(Handle::Scale(Axis::X)));
    }

    /// Everything is measured in CSS pixels times the world size of one: at twice the scale
    /// the same handle sits twice as far out.
    #[test]
    fn the_widget_is_screen_constant() {
        let g = at_origin();
        let (f, d) = down(BALL_AT * 2.0, 0.0);
        assert_eq!(g.hit(&f, &d, 2.0), Some(Handle::Scale(Axis::X)));
        assert_ne!(g.hit(&f, &d, 1.0), Some(Handle::Scale(Axis::X)));
    }

    /// A translate drag moves by exactly the distance the grab point travelled along the axis,
    /// and by nothing on the other two.
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

    /// Sighting straight down the axis has no answer, so there is no drag rather than a wrong
    /// one. This is the failure that throws an object to infinity in a naive gumball.
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

    /// A scale is measured from the grab, so re-reading the same pointer gives 1, and it can
    /// never produce a factor that collapses or mirrors the object.
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

    /// A rotation and a scale act about the gumball, not the world origin: a point AT the
    /// gumball does not move.
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

    /// A typed value means what the drag means, and a typed zero cannot bake a singular matrix.
    #[test]
    fn typed_values_match_the_drag_and_are_clamped() {
        let g = at_origin();
        let m = g.typed(Handle::Translate(Axis::Y), 12.5);
        assert_eq!([m.m[12], m.m[13], m.m[14]], [0.0, 12.5, 0.0]);
        let z = g.typed(Handle::ScaleUniform, 0.0);
        assert!(z.m[0] >= MIN_SCALE && z.m[5] >= MIN_SCALE && z.m[10] >= MIN_SCALE);
    }
}
