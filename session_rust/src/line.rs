use crate::{Color, Point, Vector, Xform};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "Line")]
pub struct Line {
    pub guid: String,
    pub name: String,
    #[serde(rename = "x0")]
    _x0: f64,
    #[serde(rename = "y0")]
    _y0: f64,
    #[serde(rename = "z0")]
    _z0: f64,
    #[serde(rename = "x1")]
    _x1: f64,
    #[serde(rename = "y1")]
    _y1: f64,
    #[serde(rename = "z1")]
    _z1: f64,
    pub width: f64,
    pub linecolor: Color,
    #[serde(default = "Xform::identity")]
    pub xform: Xform,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            _x0: 0.0,
            _y0: 0.0,
            _z0: 0.0,
            _x1: 0.0,
            _y1: 0.0,
            _z1: 1.0,
            guid: Uuid::new_v4().to_string(),
            name: "my_line".to_string(),
            linecolor: Color::white(),
            width: 1.0,
            xform: Xform::identity(),
        }
    }
}

impl Line {
    pub fn new(x0: f64, y0: f64, z0: f64, x1: f64, y1: f64, z1: f64) -> Self {
        Self {
            _x0: x0,
            _y0: y0,
            _z0: z0,
            _x1: x1,
            _y1: y1,
            _z1: z1,
            ..Default::default()
        }
    }

    pub fn from_points(p1: &Point, p2: &Point) -> Self {
        Self::new(p1.x(), p1.y(), p1.z(), p2.x(), p2.y(), p2.z())
    }

    pub fn with_name(name: &str, x0: f64, y0: f64, z0: f64, x1: f64, y1: f64, z1: f64) -> Self {
        Self {
            name: name.to_string(),
            _x0: x0,
            _y0: y0,
            _z0: z0,
            _x1: x1,
            _y1: y1,
            _z1: z1,
            ..Default::default()
        }
    }

    pub fn x0(&self) -> f64 {
        self._x0
    }
    pub fn y0(&self) -> f64 {
        self._y0
    }
    pub fn z0(&self) -> f64 {
        self._z0
    }
    pub fn x1(&self) -> f64 {
        self._x1
    }
    pub fn y1(&self) -> f64 {
        self._y1
    }
    pub fn z1(&self) -> f64 {
        self._z1
    }

    pub fn set_x0(&mut self, v: f64) {
        self._x0 = v;
    }
    pub fn set_y0(&mut self, v: f64) {
        self._y0 = v;
    }
    pub fn set_z0(&mut self, v: f64) {
        self._z0 = v;
    }
    pub fn set_x1(&mut self, v: f64) {
        self._x1 = v;
    }
    pub fn set_y1(&mut self, v: f64) {
        self._y1 = v;
    }
    pub fn set_z1(&mut self, v: f64) {
        self._z1 = v;
    }

    pub fn length(&self) -> f64 {
        let dx = self._x1 - self._x0;
        let dy = self._y1 - self._y0;
        let dz = self._z1 - self._z0;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        let dx = self._x1 - self._x0;
        let dy = self._y1 - self._y0;
        let dz = self._z1 - self._z0;
        dx * dx + dy * dy + dz * dz
    }

    pub fn to_vector(&self) -> Vector {
        Vector::new(
            self._x1 - self._x0,
            self._y1 - self._y0,
            self._z1 - self._z0,
        )
    }

    pub fn point_at(&self, t: f64) -> Point {
        let s = 1.0 - t;
        Point::new(
            s * self._x0 + t * self._x1,
            s * self._y0 + t * self._y1,
            s * self._z0 + t * self._z1,
        )
    }

    pub fn start(&self) -> Point {
        Point::new(self._x0, self._y0, self._z0)
    }

    pub fn end(&self) -> Point {
        Point::new(self._x1, self._y1, self._z1)
    }
}

impl Index<usize> for Line {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self._x0,
            1 => &self._y0,
            2 => &self._z0,
            3 => &self._x1,
            4 => &self._y1,
            5 => &self._z1,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Line {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self._x0,
            1 => &mut self._y0,
            2 => &mut self._z0,
            3 => &mut self._x1,
            4 => &mut self._y1,
            5 => &mut self._z1,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl AddAssign<&Vector> for Line {
    fn add_assign(&mut self, other: &Vector) {
        self._x0 += other.x();
        self._y0 += other.y();
        self._z0 += other.z();
        self._x1 += other.x();
        self._y1 += other.y();
        self._z1 += other.z();
    }
}

impl SubAssign<&Vector> for Line {
    fn sub_assign(&mut self, other: &Vector) {
        self._x0 -= other.x();
        self._y0 -= other.y();
        self._z0 -= other.z();
        self._x1 -= other.x();
        self._y1 -= other.y();
        self._z1 -= other.z();
    }
}

impl MulAssign<f64> for Line {
    fn mul_assign(&mut self, factor: f64) {
        self._x0 *= factor;
        self._y0 *= factor;
        self._z0 *= factor;
        self._x1 *= factor;
        self._y1 *= factor;
        self._z1 *= factor;
    }
}

impl DivAssign<f64> for Line {
    fn div_assign(&mut self, factor: f64) {
        self._x0 /= factor;
        self._y0 /= factor;
        self._z0 /= factor;
        self._x1 /= factor;
        self._y1 /= factor;
        self._z1 /= factor;
    }
}

impl Add<&Vector> for Line {
    type Output = Line;

    fn add(self, other: &Vector) -> Line {
        let mut result = self;
        result += other;
        result
    }
}

impl Sub<&Vector> for Line {
    type Output = Line;

    fn sub(self, other: &Vector) -> Line {
        let mut result = self;
        result -= other;
        result
    }
}

impl Mul<f64> for Line {
    type Output = Line;

    fn mul(self, factor: f64) -> Line {
        let mut result = self;
        result *= factor;
        result
    }
}

impl Div<f64> for Line {
    type Output = Line;

    fn div(self, factor: f64) -> Line {
        let mut result = self;
        result /= factor;
        result
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Line({}, {}, {}, {}, {}, {})",
            self._x0, self._y0, self._z0, self._x1, self._y1, self._z1
        )
    }
}

impl Line {
    pub fn transform(&mut self) {
        let mut start = Point::new(self._x0, self._y0, self._z0);
        let mut end = Point::new(self._x1, self._y1, self._z1);

        let xform = self.xform.clone();
        xform.transform_point(&mut start);
        xform.transform_point(&mut end);

        self._x0 = start.x();
        self._y0 = start.y();
        self._z0 = start.z();
        self._x1 = end.x();
        self._y1 = end.y();
        self._z1 = end.z();
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

#[path = "line_test.rs"]
#[cfg(test)]
mod line_test;
