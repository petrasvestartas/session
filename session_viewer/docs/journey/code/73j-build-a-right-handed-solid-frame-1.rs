    /// Enter takes `value`.
    pub fn or(self, value: f64) -> Self {
        Self {
            default: Some(value),
            ..self
        }
    }
}

/// A right-handed frame: in-plane x and y, normal z.
#[derive(Clone, Debug)]
pub struct Frame {
    pub origin: Point, // the first point answered
    pub x: Vector,     // in the plane
    pub y: Vector,     // in the plane
    pub z: Vector,     // the plane normal
}

impl Frame {
    /// The plane's axes at `origin`; z = x × y.
    pub fn new(plane: &Plane, origin: Point) -> Self {
        let (x, y) = (plane.x_axis(), plane.y_axis());
        let z = x.cross(&y);
        Self { origin, x, y, z }
    }

    /// The point at local coordinates.
    pub fn at(&self, u: f64, v: f64, w: f64) -> Point {
        &self.origin + (&self.x * u + &self.y * v + &self.z * w)
    }

    /// Local coordinates of `p`.
    pub fn local(&self, p: &Point) -> [f64; 3] {
        let d = p - &self.origin;
        [d.dot(&self.x), d.dot(&self.y), d.dot(&self.z)]
    }

    /// Turned about z so x points along in-plane (u, v); unchanged for a zero direction.
    pub fn turned(&self, u: f64, v: f64) -> Self {
        let length = u.hypot(v);

        if length <= 1e-300 {
            return self.clone();
        }

        let x = &self.x * (u / length) + &self.y * (v / length);
        let y = self.z.cross(&x);
        Self {
            origin: self.origin.clone(),
            x,
            y,
            z: self.z.clone(),
        }
    }

    /// Upside down: y and z reversed, still a rotation.
    pub fn flipped(&self) -> Self {
        Self {
            origin: self.origin.clone(),
            x: self.x.clone(),
            y: -&self.y,
            z: -&self.z,
        }
    }

    /// The frame and height with the height made positive.
    pub fn upright(&self, height: f64) -> (Self, f64) {
        if height < 0.0 {
            (self.flipped(), -height)
        } else {
            (self.clone(), height)
        }
    }

    /// The axes turned one step: x, y, z become y, z, x.
    pub fn rolled(&self) -> Self {
        Self {
            origin: self.origin.clone(),
            x: self.y.clone(),
            y: self.z.clone(),
            z: self.x.clone(),
        }
    }

    /// Local to world.
    pub fn to_xform(&self) -> Xform {
        Xform::frame_to_world(&self.origin, &self.x, &self.y, &self.z)
    }

}
