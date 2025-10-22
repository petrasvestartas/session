use crate::{Point, Vector, Xform};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "Plane")]
pub struct Plane {
    pub guid: String,
    pub name: String,
    #[serde(rename = "origin")]
    _origin: Point,
    #[serde(rename = "x_axis")]
    _x_axis: Vector,
    #[serde(rename = "y_axis")]
    _y_axis: Vector,
    #[serde(rename = "z_axis")]
    _z_axis: Vector,
    #[serde(rename = "a")]
    _a: f64,
    #[serde(rename = "b")]
    _b: f64,
    #[serde(rename = "c")]
    _c: f64,
    #[serde(rename = "d")]
    _d: f64,
    #[serde(default = "Xform::identity")]
    pub xform: Xform,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            guid: Uuid::new_v4().to_string(),
            name: "my_plane".to_string(),
            _origin: Point::default(),
            _x_axis: Vector::x_axis(),
            _y_axis: Vector::y_axis(),
            _z_axis: Vector::z_axis(),
            _a: 0.0,
            _b: 0.0,
            _c: 1.0,
            _d: 0.0,
            xform: Xform::identity(),
        }
    }
}

impl Plane {
    pub fn new(point: Point, mut x_axis: Vector, mut y_axis: Vector) -> Self {
        x_axis.normalize_self();
        let dot_product = y_axis.dot(&x_axis);
        y_axis -= x_axis.clone() * dot_product;
        y_axis.normalize_self();
        let mut z_axis = x_axis.cross(&y_axis);
        z_axis.normalize_self();

        let a = z_axis.x();
        let b = z_axis.y();
        let c = z_axis.z();
        let d = -(a * point.x() + b * point.y() + c * point.z());

        Self {
            guid: Uuid::new_v4().to_string(),
            name: "my_plane".to_string(),
            _origin: point,
            _x_axis: x_axis,
            _y_axis: y_axis,
            _z_axis: z_axis,
            _a: a,
            _b: b,
            _c: c,
            _d: d,
            xform: Xform::identity(),
        }
    }

    pub fn with_name(point: Point, mut x_axis: Vector, mut y_axis: Vector, name: String) -> Self {
        x_axis.normalize_self();
        let dot_product = y_axis.dot(&x_axis);
        y_axis -= x_axis.clone() * dot_product;
        y_axis.normalize_self();
        let mut z_axis = x_axis.cross(&y_axis);
        z_axis.normalize_self();

        let a = z_axis.x();
        let b = z_axis.y();
        let c = z_axis.z();
        let d = -(a * point.x() + b * point.y() + c * point.z());

        Self {
            guid: Uuid::new_v4().to_string(),
            name,
            _origin: point,
            _x_axis: x_axis,
            _y_axis: y_axis,
            _z_axis: z_axis,
            _a: a,
            _b: b,
            _c: c,
            _d: d,
            xform: Xform::identity(),
        }
    }

    pub fn from_point_normal(point: Point, normal: Vector) -> Self {
        let origin = point.clone();
        let mut z_axis = normal;
        z_axis.normalize_self();
        let mut x_axis = Vector::default();
        x_axis.perpendicular_to(&z_axis);
        x_axis.normalize_self();
        let mut y_axis = z_axis.cross(&x_axis);
        y_axis.normalize_self();

        let a = z_axis.x();
        let b = z_axis.y();
        let c = z_axis.z();
        let d = -(a * origin.x() + b * origin.y() + c * origin.z());

        Self {
            guid: Uuid::new_v4().to_string(),
            name: "my_plane".to_string(),
            _origin: origin,
            _x_axis: x_axis,
            _y_axis: y_axis,
            _z_axis: z_axis,
            _a: a,
            _b: b,
            _c: c,
            _d: d,
            xform: Xform::identity(),
        }
    }

    pub fn from_points(points: Vec<Point>) -> Self {
        if points.len() < 3 {
            return Self::default();
        }

        let point1 = &points[0];
        let point2 = &points[1];
        let point3 = &points[2];
        let v1 = point2.clone() - point1.clone();
        let v2 = point3.clone() - point1.clone();
        let mut z_axis = v1.cross(&v2);
        z_axis.normalize_self();
        let mut x_axis = Vector::default();
        x_axis.perpendicular_to(&z_axis);
        x_axis.normalize_self();
        let mut y_axis = z_axis.cross(&x_axis);
        y_axis.normalize_self();
        let origin = point1.clone();

        let a = z_axis.x();
        let b = z_axis.y();
        let c = z_axis.z();
        let d = -(a * origin.x() + b * origin.y() + c * origin.z());

        Self {
            guid: Uuid::new_v4().to_string(),
            name: "my_plane".to_string(),
            _origin: origin,
            _x_axis: x_axis,
            _y_axis: y_axis,
            _z_axis: z_axis,
            _a: a,
            _b: b,
            _c: c,
            _d: d,
            xform: Xform::identity(),
        }
    }

    pub fn from_two_points(point1: Point, point2: Point) -> Self {
        let origin = point1.clone();

        let mut direction = point2.clone() - point1.clone();
        direction.normalize_self();
        let mut z_axis = Vector::default();
        z_axis.perpendicular_to(&direction);
        z_axis.normalize_self();

        let x_axis = direction;
        let mut y_axis = z_axis.cross(&x_axis);
        y_axis.normalize_self();

        let a = z_axis.x();
        let b = z_axis.y();
        let c = z_axis.z();
        let d = -(a * origin.x() + b * origin.y() + c * origin.z());

        Self {
            guid: Uuid::new_v4().to_string(),
            name: "my_plane".to_string(),
            _origin: origin,
            _x_axis: x_axis,
            _y_axis: y_axis,
            _z_axis: z_axis,
            _a: a,
            _b: b,
            _c: c,
            _d: d,
            xform: Xform::identity(),
        }
    }

    pub fn xy_plane() -> Self {
        Self {
            guid: Uuid::new_v4().to_string(),
            name: "xy_plane".to_string(),
            _origin: Point::new(0.0, 0.0, 0.0),
            _x_axis: Vector::x_axis(),
            _y_axis: Vector::y_axis(),
            _z_axis: Vector::z_axis(),
            _a: 0.0,
            _b: 0.0,
            _c: 1.0,
            _d: 0.0,
            xform: Xform::identity(),
        }
    }

    pub fn yz_plane() -> Self {
        Self {
            guid: Uuid::new_v4().to_string(),
            name: "yz_plane".to_string(),
            _origin: Point::new(0.0, 0.0, 0.0),
            _x_axis: Vector::y_axis(),
            _y_axis: Vector::z_axis(),
            _z_axis: Vector::x_axis(),
            _a: 1.0,
            _b: 0.0,
            _c: 0.0,
            _d: 0.0,
            xform: Xform::identity(),
        }
    }

    pub fn xz_plane() -> Self {
        Self {
            guid: Uuid::new_v4().to_string(),
            name: "xz_plane".to_string(),
            _origin: Point::new(0.0, 0.0, 0.0),
            _x_axis: Vector::x_axis(),
            _y_axis: Vector::new(0.0, 0.0, -1.0),
            _z_axis: Vector::new(0.0, 1.0, 0.0),
            _a: 0.0,
            _b: 1.0,
            _c: 0.0,
            _d: 0.0,
            xform: Xform::identity(),
        }
    }

    pub fn origin(&self) -> Point {
        self._origin.clone()
    }

    pub fn x_axis(&self) -> Vector {
        self._x_axis.clone()
    }

    pub fn y_axis(&self) -> Vector {
        self._y_axis.clone()
    }

    pub fn z_axis(&self) -> Vector {
        self._z_axis.clone()
    }

    pub fn a(&self) -> f64 {
        self._a
    }

    pub fn b(&self) -> f64 {
        self._b
    }

    pub fn c(&self) -> f64 {
        self._c
    }

    pub fn d(&self) -> f64 {
        self._d
    }

    pub fn reverse(&mut self) {
        std::mem::swap(&mut self._x_axis, &mut self._y_axis);
        self._z_axis.reverse();

        self._a = self._z_axis.x();
        self._b = self._z_axis.y();
        self._c = self._z_axis.z();
        self._d =
            -(self._a * self._origin.x() + self._b * self._origin.y() + self._c * self._origin.z());
    }

    pub fn rotate(&mut self, angles_in_radians: f64) {
        let cos_angle = angles_in_radians.cos();
        let sin_angle = angles_in_radians.sin();

        let new_x = self._x_axis.clone() * cos_angle + self._y_axis.clone() * sin_angle;
        let new_y = self._y_axis.clone() * cos_angle - self._x_axis.clone() * sin_angle;

        self._x_axis = new_x;
        self._y_axis = new_y;

        self._a = self._z_axis.x();
        self._b = self._z_axis.y();
        self._c = self._z_axis.z();
        self._d =
            -(self._a * self._origin.x() + self._b * self._origin.y() + self._c * self._origin.z());
    }

    pub fn is_right_hand(&self) -> bool {
        let x_copy = self._x_axis.clone();
        let y_copy = self._y_axis.clone();
        let z_copy = self._z_axis.clone();
        let cross = x_copy.cross(&y_copy);
        let dot_product = cross.dot(&z_copy);
        dot_product > 0.999
    }

    pub fn is_same_direction(plane0: &Plane, plane1: &Plane, can_be_flipped: bool) -> bool {
        let n0 = plane0._z_axis.clone();
        let n1 = plane1._z_axis.clone();

        let parallel = n0.is_parallel_to(&n1);

        if can_be_flipped {
            parallel != 0
        } else {
            parallel == 1
        }
    }

    pub fn is_same_position(plane0: &Plane, plane1: &Plane) -> bool {
        let dist0 = (plane0._a * plane1._origin.x()
            + plane0._b * plane1._origin.y()
            + plane0._c * plane1._origin.z()
            + plane0._d)
            .abs();

        let dist1 = (plane1._a * plane0._origin.x()
            + plane1._b * plane0._origin.y()
            + plane1._c * plane0._origin.z()
            + plane1._d)
            .abs();

        let tolerance = crate::tolerance::Tolerance::ZERO_TOLERANCE;
        dist0 < tolerance && dist1 < tolerance
    }

    pub fn is_coplanar(plane0: &Plane, plane1: &Plane, can_be_flipped: bool) -> bool {
        Self::is_same_direction(plane0, plane1, can_be_flipped)
            && Self::is_same_position(plane0, plane1)
    }
}

impl std::ops::Index<usize> for Plane {
    type Output = Vector;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self._x_axis,
            1 => &self._y_axis,
            _ => &self._z_axis,
        }
    }
}

impl std::ops::IndexMut<usize> for Plane {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self._x_axis,
            1 => &mut self._y_axis,
            _ => &mut self._z_axis,
        }
    }
}

impl std::ops::AddAssign<Vector> for Plane {
    fn add_assign(&mut self, other: Vector) {
        self._origin += other;
        self._d =
            -(self._a * self._origin.x() + self._b * self._origin.y() + self._c * self._origin.z());
    }
}

impl std::ops::SubAssign<Vector> for Plane {
    fn sub_assign(&mut self, other: Vector) {
        self._origin -= other;
        self._d =
            -(self._a * self._origin.x() + self._b * self._origin.y() + self._c * self._origin.z());
    }
}

impl std::ops::Add<Vector> for Plane {
    type Output = Plane;

    fn add(self, other: Vector) -> Plane {
        let mut result = self.clone();
        result += other;
        result
    }
}

impl std::ops::Sub<Vector> for Plane {
    type Output = Plane;

    fn sub(self, other: Vector) -> Plane {
        let mut result = self.clone();
        result -= other;
        result
    }
}

impl PartialEq<Point> for Plane {
    fn eq(&self, other: &Point) -> bool {
        self._origin == *other
    }
}

impl Plane {
    /// Translate (move) a plane along its normal direction by a specified distance
    pub fn translate_by_normal(&self, distance: f64) -> Plane {
        let mut normal = self._z_axis.clone();
        normal.normalize_self();

        let new_origin = self._origin.clone() + (normal * distance);

        Plane::new(new_origin, self._x_axis.clone(), self._y_axis.clone())
    }
}

impl std::fmt::Display for Plane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Plane(origin={}, x_axis={}, y_axis={}, z_axis={}, guid={}, name={})",
            self._origin, self._x_axis, self._y_axis, self._z_axis, self.guid, self.name
        )
    }
}

impl Plane {
    pub fn transform(&mut self) {
        let xform = self.xform.clone();
        xform.transform_point(&mut self._origin);
        xform.transform_vector(&mut self._x_axis);
        xform.transform_vector(&mut self._y_axis);
        xform.transform_vector(&mut self._z_axis);
        self.xform = Xform::identity();
    }

    pub fn transformed(&self) -> Self {
        let mut result = self.clone();
        result.transform();
        result
    }

    pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json_data)?)
    }
}

#[cfg(test)]
#[path = "plane_test.rs"]
mod plane_test;
