//! The gumball: ten handles, a hit test, and the transform each drag produces.
//!
//! Pure CPU and f64: no wgpu, no kernel objects, no `State`. What a drag produces is a world
//! matrix, which the caller applies to one object row or to many.
//!
//! It is hit tested on the CPU rather than through the id targets, and that is deliberate.
//! Scene picking draws integer ids into a window and reads them back asynchronously, dropping
//! stale answers by generation, so an answer arrives a frame after the pointer moved. Hover
//! feedback a frame late is unusable, and the widget's geometry is the viewer's own, so there
//! is nothing to read back.
//!
//! Every length below is in CSS pixels and multiplied by `world_per_px` at the gumball's
//! depth, which is what keeps the widget the same size on screen at any zoom or projection: a
//! control that shrinks with distance stops being grabbable exactly when the object needs it.

use session_rust::{Point, Vector};

/// The one length everything else is a fraction of, in CSS pixels.
pub const ARM: f64 = 72.0;
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
    /// Where the grab landed: a point on the axis, or in the handle's plane.
    grabbed: Point,
    /// The angle of the grab about the axis, for a rotate.
    angle: f64,
    /// The distance of the grab from the origin, for a scale. Never zero.
    reach: f64,
}

/// Where the widget is and what it is doing.
pub struct Gizmo {
    pub origin: Point,
    pub hovered: Option<Handle>,
    pub drag: Option<Drag>,
}

impl Gizmo {
    pub fn new(origin: Point) -> Self {
        Self { origin, hovered: None, drag: None }
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
        let s = world_per_px;
        if within(from, dir, &self.origin, HUB * s) {
            return Some(Handle::ScaleUniform);
        }
        for axis in [Axis::X, Axis::Y, Axis::Z] {
            let at = along(&self.origin, &axis.unit(), BALL_AT * s);
            if within(from, dir, &at, GRAB * s) {
                return Some(Handle::Scale(axis));
            }
        }
        for axis in [Axis::X, Axis::Y, Axis::Z] {
            if let Some(p) = closest_on_axis(from, dir, &self.origin, &axis.unit()) {
                let t = dot(&sub(&p, &self.origin), &axis.unit());
                if (0.0..=ARM * s).contains(&t) && within(from, dir, &p, GRAB * s) {
                    return Some(Handle::Translate(axis));
                }
            }
        }
        for axis in [Axis::X, Axis::Y, Axis::Z] {
            if let Some(p) = plane_hit(from, dir, &self.origin, &axis.unit()) {
                let d = sub(&p, &self.origin);
                let (u, v) = axis.others();
                // A quarter arc, in the quadrant the arms and balls do not occupy. Sharing a
                // radius with the arm tip would make the two ambiguous exactly where a reader
                // aims for one of them.
                if dot(&d, &u) < 0.0 && dot(&d, &v) < 0.0 && (length(&d) - ARM * s).abs() < GRAB * s
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
            },
            Handle::Rotate(axis) => Drag {
                handle,
                grabbed: self.origin.clone(),
                angle: angle_in_plane(&plane_hit(from, dir, &self.origin, &axis.unit())?, &self.origin, axis),
                reach: 1.0,
            },
            Handle::Scale(axis) => {
                let p = closest_on_axis(from, dir, &self.origin, &axis.unit())?;
                let reach = dot(&sub(&p, &self.origin), &axis.unit());
                Drag { handle, grabbed: p, angle: 0.0, reach: nonzero(reach) }
            }
            Handle::ScaleUniform => {
                let normal = facing(dir);
                let p = plane_hit(from, dir, &self.origin, &normal)?;
                let reach = length(&sub(&p, &self.origin));
                Drag { handle, grabbed: p, angle: 0.0, reach: nonzero(reach) }
            }
        };
        self.drag = Some(drag.clone());
        Some(drag)
    }

    /// The world transform this drag now stands for, as a column-major 4x4.
    ///
    /// Always measured from the grab, never accumulated frame to frame: a drag that adds a
    /// delta per move drifts, and a dropped frame changes the result.
    pub fn update(&self, drag: &Drag, from: &Point, dir: &Vector) -> Option<[f64; 16]> {
        match drag.handle {
            Handle::Translate(axis) => {
                let now = closest_on_axis(from, dir, &self.origin, &axis.unit())?;
                let d = sub(&now, &drag.grabbed);
                Some(translation(d[0], d[1], d[2]))
            }
            Handle::Rotate(axis) => {
                let now = plane_hit(from, dir, &self.origin, &axis.unit())?;
                let turned = angle_in_plane(&now, &self.origin, axis) - drag.angle;
                Some(about(&self.origin, rotation(axis, turned)))
            }
            Handle::Scale(axis) => {
                let now = closest_on_axis(from, dir, &self.origin, &axis.unit())?;
                let reach = dot(&sub(&now, &self.origin), &axis.unit());
                let k = softened(reach / drag.reach);
                let (x, y, z) = match axis {
                    Axis::X => (k, 1.0, 1.0),
                    Axis::Y => (1.0, k, 1.0),
                    Axis::Z => (1.0, 1.0, k),
                };
                Some(about(&self.origin, scaling(x, y, z)))
            }
            Handle::ScaleUniform => {
                let normal = facing(dir);
                let now = plane_hit(from, dir, &self.origin, &normal)?;
                let k = softened(length(&sub(&now, &self.origin)) / drag.reach);
                Some(about(&self.origin, scaling(k, k, k)))
            }
        }
    }

    /// The transform for a value typed into the numeric box, with the same meaning a drag has:
    /// a move is millimetres, a rotation degrees, a scale a factor.
    pub fn typed(&self, handle: Handle, value: f64) -> [f64; 16] {
        match handle {
            Handle::Translate(axis) => {
                let u = axis.unit();
                translation(u[0] * value, u[1] * value, u[2] * value)
            }
            Handle::Rotate(axis) => about(&self.origin, rotation(axis, value.to_radians())),
            Handle::Scale(axis) => {
                let k = value.max(MIN_SCALE);
                let (x, y, z) = match axis {
                    Axis::X => (k, 1.0, 1.0),
                    Axis::Y => (1.0, k, 1.0),
                    Axis::Z => (1.0, 1.0, k),
                };
                about(&self.origin, scaling(x, y, z))
            }
            Handle::ScaleUniform => {
                let k = value.max(MIN_SCALE);
                about(&self.origin, scaling(k, k, k))
            }
        }
    }
}

// ---- the arithmetic, all f64 -------------------------------------------------------------

fn sub(a: &Point, b: &Point) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: &[f64; 3], b: &Vector) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn length(v: &[f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn along(p: &Point, v: &Vector, d: f64) -> Point {
    Point::new(p[0] + v[0] * d, p[1] + v[1] * d, p[2] + v[2] * d)
}

/// A factor that never collapses or mirrors, softened so the drag is usable near the centre.
fn softened(ratio: f64) -> f64 {
    if !ratio.is_finite() || ratio <= 0.0 {
        return MIN_SCALE;
    }
    ratio.powf(SCALE_SOFTENING).max(MIN_SCALE)
}

fn nonzero(v: f64) -> f64 {
    if v.abs() < 1e-9 { 1e-9_f64.copysign(if v < 0.0 { -1.0 } else { 1.0 }) } else { v }
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
fn closest_on_axis(from: &Point, dir: &Vector, origin: &Point, axis: &Vector) -> Option<Point> {
    let w = sub(from, origin);
    let b = dot(&[dir[0], dir[1], dir[2]], axis);
    let denom = 1.0 - b * b;
    if denom.abs() < 1e-9 {
        return None;
    }
    let t = (dot(&w, axis) - b * dot(&w, dir)) / denom;
    Some(along(origin, axis, t))
}

/// Where a ray meets the plane through `origin` with this normal, in front of the ray.
fn plane_hit(from: &Point, dir: &Vector, origin: &Point, normal: &Vector) -> Option<Point> {
    let denom = dot(&[dir[0], dir[1], dir[2]], normal);
    if denom.abs() < 1e-9 {
        return None;
    }
    let t = dot(&sub(origin, from), normal) / denom;
    (t > 0.0 && t.is_finite()).then(|| along(from, dir, t))
}

/// Whether a ray passes within `radius` of a point.
fn within(from: &Point, dir: &Vector, at: &Point, radius: f64) -> bool {
    let w = sub(at, from);
    let t = dot(&w, dir);
    if t < 0.0 {
        return false;
    }
    let closest = along(from, dir, t);
    length(&sub(&closest, at)) <= radius
}

/// The angle of a point about an axis, measured from that axis's first companion.
fn angle_in_plane(p: &Point, origin: &Point, axis: Axis) -> f64 {
    let (u, v) = axis.others();
    let d = sub(p, origin);
    dot(&d, &v).atan2(dot(&d, &u))
}

fn translation(x: f64, y: f64, z: f64) -> [f64; 16] {
    let mut m = identity();
    m[12] = x;
    m[13] = y;
    m[14] = z;
    m
}

fn scaling(x: f64, y: f64, z: f64) -> [f64; 16] {
    let mut m = identity();
    m[0] = x;
    m[5] = y;
    m[10] = z;
    m
}

fn rotation(axis: Axis, radians: f64) -> [f64; 16] {
    let (c, s) = (radians.cos(), radians.sin());
    let mut m = identity();
    match axis {
        Axis::X => {
            m[5] = c;
            m[6] = s;
            m[9] = -s;
            m[10] = c;
        }
        Axis::Y => {
            m[0] = c;
            m[2] = -s;
            m[8] = s;
            m[10] = c;
        }
        Axis::Z => {
            m[0] = c;
            m[1] = s;
            m[4] = -s;
            m[5] = c;
        }
    }
    m
}

fn identity() -> [f64; 16] {
    let mut m = [0.0; 16];
    m[0] = 1.0;
    m[5] = 1.0;
    m[10] = 1.0;
    m[15] = 1.0;
    m
}

/// `m` applied about `pivot` rather than the world origin.
fn about(pivot: &Point, m: [f64; 16]) -> [f64; 16] {
    let to = translation(pivot[0], pivot[1], pivot[2]);
    let back = translation(-pivot[0], -pivot[1], -pivot[2]);
    mul(&mul(&to, &m), &back)
}

/// Column-major 4x4 product.
fn mul(a: &[f64; 16], b: &[f64; 16]) -> [f64; 16] {
    let mut out = [0.0; 16];
    for col in 0..4 {
        for row in 0..4 {
            out[col * 4 + row] = (0..4).map(|k| a[k * 4 + row] * b[col * 4 + k]).sum();
        }
    }
    out
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
        assert_eq!(g.hit(&f, &d, SCALE), Some(Handle::Translate(Axis::X)), "the arm tip is not an arc");
        let (f, d) = down(ARM * 3.0, ARM * 3.0);
        assert_eq!(g.hit(&f, &d, SCALE), None);
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
        let drag = g.begin(Handle::Translate(Axis::X), &f0, &d0).expect("grabbed");
        let (f1, d1) = down(42.0, 0.0);
        let m = g.update(&drag, &f1, &d1).expect("a transform");
        assert!((m[12] - 12.0).abs() < 1e-9);
        assert!(m[13].abs() < 1e-9 && m[14].abs() < 1e-9);
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
        assert!((m[0]).abs() < 1e-9, "x.x");
        assert!((m[1] - 1.0).abs() < 1e-9, "x.y");
    }

    /// A scale is measured from the grab, so re-reading the same pointer gives 1, and it can
    /// never produce a factor that collapses or mirrors the object.
    #[test]
    fn a_scale_is_relative_to_the_grab_and_never_collapses() {
        let mut g = at_origin();
        let (f0, d0) = down(BALL_AT, 0.0);
        let drag = g.begin(Handle::Scale(Axis::X), &f0, &d0).expect("grabbed");
        let m = g.update(&drag, &f0, &d0).expect("a transform");
        assert!((m[0] - 1.0).abs() < 1e-9, "no movement is no change");

        let (f1, d1) = down(BALL_AT * 4.0, 0.0);
        let grown = g.update(&drag, &f1, &d1).expect("a transform");
        assert!(grown[0] > 1.0 && grown[5] == 1.0 && grown[10] == 1.0, "one axis only");

        let (f2, d2) = down(-BALL_AT * 4.0, 0.0);
        let flipped = g.update(&drag, &f2, &d2).expect("a transform");
        assert!(flipped[0] >= MIN_SCALE, "never mirrors");
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
            let p = [100.0, 200.0, 300.0, 1.0];
            let moved: Vec<f64> = (0..3)
                .map(|r| (0..4).map(|k| m[k * 4 + r] * p[k]).sum())
                .collect();
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
        assert_eq!([m[12], m[13], m[14]], [0.0, 12.5, 0.0]);
        let z = g.typed(Handle::ScaleUniform, 0.0);
        assert!(z[0] >= MIN_SCALE && z[5] >= MIN_SCALE && z[10] >= MIN_SCALE);
    }
}
