use crate::tolerance::{Tolerance, SCALE, TO_DEGREES, TO_RADIANS};
use serde::{ser::Serialize as SerTrait, Deserialize, Serialize};
use std::fmt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};
use uuid::Uuid;

/// A 3D vector with visual properties and JSON serialization support.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename = "Vector")]
pub struct Vector {
    pub guid: String,
    pub name: String,
    #[serde(rename = "x")]
    _x: f64,
    #[serde(rename = "y")]
    _y: f64,
    #[serde(rename = "z")]
    _z: f64,
    #[serde(skip)]
    _length: f64,
    #[serde(skip)]
    _has_length: bool,
}

impl Vector {
    /// Creates a new Vector with specified coordinates.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            _x: x,
            _y: y,
            _z: z,
            guid: Uuid::new_v4().to_string(),
            name: "my_vector".to_string(),
            _length: 0.0,
            _has_length: false,
        }
    }

    /// Creates a zero vector.
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Creates a unit vector along X-axis.
    ///
    /// Returns
    /// -------
    /// Vector
    ///     Unit vector (1, 0, 0).
    pub fn x_axis() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    /// Creates a unit vector along Y-axis.
    ///
    /// Returns
    /// -------
    /// Vector
    ///     Unit vector (0, 1, 0).
    pub fn y_axis() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    /// Getters for coordinates
    pub fn x(&self) -> f64 {
        self._x
    }
    pub fn y(&self) -> f64 {
        self._y
    }
    pub fn z(&self) -> f64 {
        self._z
    }

    /// Setters for coordinates (invalidate cached length)
    pub fn set_x(&mut self, v: f64) {
        self._x = v;
        self.invalidate_length_cache();
    }
    pub fn set_y(&mut self, v: f64) {
        self._y = v;
        self.invalidate_length_cache();
    }
    pub fn set_z(&mut self, v: f64) {
        self._z = v;
        self.invalidate_length_cache();
    }

    /// Creates a unit vector along Z-axis.
    ///
    /// Returns
    /// -------
    /// Vector
    ///     Unit vector (0, 0, 1).
    pub fn z_axis() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    /// Creates a vector from start point to end point.
    ///
    /// Parameters
    /// ----------
    /// start : &Vector
    ///     The starting point.
    /// end : &Vector
    ///     The ending point.
    ///
    /// Returns
    /// -------
    /// Vector
    ///     The vector from start to end (end - start).
    pub fn from_start_and_end(start: &Vector, end: &Vector) -> Self {
        Self::new(end._x - start._x, end._y - start._y, end._z - start._z)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Vector Operations
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Invalidates the cached length when coordinates change.
    fn invalidate_length_cache(&mut self) {
        self._has_length = false;
    }

    /// Computes the length (magnitude) of the vector without caching.
    ///
    /// Returns
    /// -------
    /// f64
    ///     The length of the vector.
    pub fn compute_length(&self) -> f64 {
        (self._x * self._x + self._y * self._y + self._z * self._z).sqrt()
    }

    /// Gets the (cached) magnitude. Avoids recalculating if unchanged.
    ///
    /// Returns
    /// -------
    /// f64
    ///     The magnitude (length) of the vector.
    pub fn magnitude(&mut self) -> f64 {
        if !self._has_length {
            self._length = self.compute_length();
            self._has_length = true;
        }
        self._length
    }

    /// Computes the squared length of the vector (avoids sqrt for performance).
    pub fn length_squared(&self) -> f64 {
        self._x * self._x + self._y * self._y + self._z * self._z
    }

    /// Normalizes the vector in place.
    pub fn normalize_self(&mut self) {
        let len = self.magnitude();
        if len > Tolerance::ZERO_TOLERANCE {
            self._x /= len;
            self._y /= len;
            self._z /= len;
            self.invalidate_length_cache();
        }
    }

    /// Returns a normalized copy of the vector.
    pub fn normalize(&self) -> Self {
        let mut result = self.clone();
        result.normalize_self();
        result
    }

    /// Reverses the vector direction in place.
    pub fn reverse(&mut self) {
        self._x = -self._x;
        self._y = -self._y;
        self._z = -self._z;
        // Length magnitude stays the same, no need to invalidate cache
    }

    /// Scales the vector by a factor.
    pub fn scale(&mut self, factor: f64) {
        self._x *= factor;
        self._y *= factor;
        self._z *= factor;
        self.invalidate_length_cache();
    }

    /// Scales the vector up by the global scale factor.
    pub fn scale_up(&mut self) {
        self.scale(SCALE);
    }

    /// Scales the vector down by the global scale factor.
    pub fn scale_down(&mut self) {
        self.scale(1.0 / SCALE);
    }

    /// Computes the dot product with another vector.
    ///
    /// Parameters
    /// ----------
    /// other : &Vector
    ///     Other vector.
    ///
    /// Returns
    /// -------
    /// f64
    ///     Dot product value.
    pub fn dot(&self, other: &Vector) -> f64 {
        self._x * other._x + self._y * other._y + self._z * other._z
    }

    /// Computes the cross product with another vector.
    ///
    /// Parameters
    /// ----------
    /// other : &Vector
    ///     Other vector.
    ///
    /// Returns
    /// -------
    /// Vector
    ///     Cross product vector (orthogonal to inputs).
    pub fn cross(&self, other: &Vector) -> Vector {
        Vector::new(
            self._y * other._z - self._z * other._y,
            self._z * other._x - self._x * other._z,
            self._x * other._y - self._y * other._x,
        )
    }

    /// Computes the angle between this vector and another in degrees.
    pub fn angle(&self, other: &Vector, sign_by_cross_product: bool) -> f64 {
        let dotp = self.dot(other);
        let len_product = self.compute_length() * other.compute_length();

        if len_product < Tolerance::ZERO_TOLERANCE {
            return 0.0;
        }

        let cos_angle = (dotp / len_product).clamp(-1.0, 1.0);
        let mut angle = cos_angle.acos() * TO_DEGREES;

        if sign_by_cross_product {
            let cp = self.cross(other);
            if cp.z() < 0.0 {
                angle = -angle;
            }
        }

        angle
    }

    /// Projects this vector onto another vector and returns detailed results.
    ///
    /// Returns a tuple of:
    /// - projection vector of `self` onto `onto`
    /// - projected length (scalar projection)
    /// - perpendicular projected vector (self - projection)
    /// - perpendicular projected vector length
    pub fn projection(&self, onto: &Vector) -> (Vector, f64, Vector, f64) {
        self.projection_with(onto, Tolerance::ZERO_TOLERANCE)
    }

    /// Same as `projection` but allows specifying a tolerance.
    pub fn projection_with(&self, onto: &Vector, tolerance: f64) -> (Vector, f64, Vector, f64) {
        let onto_len_sq = onto.length_squared();

        if onto_len_sq < tolerance {
            return (Vector::zero(), 0.0, Vector::zero(), 0.0);
        }

        // Unit vector along 'onto'
        let onto_len = onto_len_sq.sqrt();
        let onto_unit = Vector::new(onto._x / onto_len, onto._y / onto_len, onto._z / onto_len);

        // Scalar projection and projected vector
        let projected_len = self.dot(&onto_unit);
        let projection_vec = Vector::new(
            onto_unit._x * projected_len,
            onto_unit._y * projected_len,
            onto_unit._z * projected_len,
        );

        // Perpendicular component and its length
        let perp_vec = Vector::new(
            self._x - projection_vec._x,
            self._y - projection_vec._y,
            self._z - projection_vec._z,
        );
        let perp_len = perp_vec.compute_length();

        (projection_vec, projected_len, perp_vec, perp_len)
    }

    /// Checks if this vector is parallel to another vector.
    /// Returns: 1 for parallel, -1 for antiparallel, 0 for not parallel.
    pub fn is_parallel_to(&self, other: &Vector) -> i32 {
        let len_product = self.compute_length() * other.compute_length();

        if len_product <= 0.0 {
            return 0;
        }

        let cos_angle = self.dot(other) / len_product;
        let angle_in_radians = Tolerance::ANGLE_TOLERANCE_DEGREES * TO_RADIANS;
        let cos_tolerance = angle_in_radians.cos();

        if cos_angle >= cos_tolerance {
            1 // Parallel
        } else if cos_angle <= -cos_tolerance {
            -1 // Antiparallel
        } else {
            0 // Not parallel
        }
    }

    /// Gets a leveled vector (replicates statics bug with degrees passed to cos).
    pub fn get_leveled_vector(&self, vertical_height: f64) -> Vector {
        let mut copy = self.clone();
        copy.normalize_self();

        if vertical_height != 0.0 {
            let reference = Vector::z_axis();
            let angle = copy.angle(&reference, true); // returns degrees
                                                      // CRITICAL: statics bug - passes degrees directly to cos (expects radians)
            let inclined_offset_by_vertical_distance = vertical_height / angle.cos();
            copy.scale(inclined_offset_by_vertical_distance);
        }

        copy
    }

    /// Set this vector to be perpendicular to `v` (matches Python semantics).
    /// Returns true on success, false otherwise.
    pub fn perpendicular_to(&mut self, v: &Vector) -> bool {
        // Ported from Python implementation to ensure identical behavior
        let i: usize;
        let j: usize;
        let k: usize;
        let a: f64;
        let b: f64;

        if v.y().abs() > v.x().abs() {
            if v.z().abs() > v.y().abs() {
                // |v.z| > |v.y| > |v.x|
                i = 2;
                j = 1;
                k = 0;
                a = v.z();
                b = -v.y();
            } else if v.z().abs() >= v.x().abs() {
                // |v.y| >= |v.z| >= |v.x|
                i = 1;
                j = 2;
                k = 0;
                a = v.y();
                b = -v.z();
            } else {
                // |v.y| > |v.x| > |v.z|
                i = 1;
                j = 0;
                k = 2;
                a = v.y();
                b = -v.x();
            }
        } else if v.z().abs() > v.x().abs() {
            // |v.z| > |v.x| >= |v.y|
            i = 2;
            j = 0;
            k = 1;
            a = v.z();
            b = -v.x();
        } else if v.z().abs() > v.y().abs() {
            // |v.x| >= |v.z| > |v.y|
            i = 0;
            j = 2;
            k = 1;
            a = v.x();
            b = -v.z();
        } else {
            // |v.x| >= |v.y| >= |v.z|
            i = 0;
            j = 1;
            k = 2;
            a = v.x();
            b = -v.y();
        }

        let mut coords = [0.0, 0.0, 0.0];
        coords[i] = b;
        coords[j] = a;
        coords[k] = 0.0;

        self._x = coords[0];
        self._y = coords[1];
        self._z = coords[2];
        self.invalidate_length_cache();

        a != 0.0
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Static Methods
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Computes the cosine law for triangle edge length.
    pub fn cosine_law(
        triangle_edge_length_a: f64,
        triangle_edge_length_b: f64,
        angle_in_degrees_between_edges: f64,
        degrees: bool,
    ) -> f64 {
        let angle = if degrees {
            angle_in_degrees_between_edges * TO_RADIANS
        } else {
            angle_in_degrees_between_edges
        };

        (triangle_edge_length_a.powi(2) + triangle_edge_length_b.powi(2)
            - 2.0 * triangle_edge_length_a * triangle_edge_length_b * angle.cos())
        .sqrt()
    }

    /// Computes the sine law for triangle angle.
    pub fn sine_law_angle(
        triangle_edge_length_a: f64,
        angle_in_degrees_in_front_of_a: f64,
        triangle_edge_length_b: f64,
        degrees: bool,
    ) -> f64 {
        let angle_a = if degrees {
            angle_in_degrees_in_front_of_a * TO_RADIANS
        } else {
            angle_in_degrees_in_front_of_a
        };

        let sin_b = (triangle_edge_length_b * angle_a.sin()) / triangle_edge_length_a;
        let angle_b = sin_b.asin();

        if degrees {
            angle_b * TO_DEGREES
        } else {
            angle_b
        }
    }

    /// Computes the sine law for triangle edge length.
    pub fn sine_law_length(
        triangle_edge_length_a: f64,
        angle_in_degrees_in_front_of_a: f64,
        angle_in_degrees_in_front_of_b: f64,
        degrees: bool,
    ) -> f64 {
        let angle_a = if degrees {
            angle_in_degrees_in_front_of_a * TO_RADIANS
        } else {
            angle_in_degrees_in_front_of_a
        };

        let angle_b = if degrees {
            angle_in_degrees_in_front_of_b * TO_RADIANS
        } else {
            angle_in_degrees_in_front_of_b
        };

        (triangle_edge_length_a * angle_b.sin()) / angle_a.sin()
    }

    /// Computes the angle between vector XY components in degrees.
    pub fn angle_between_vector_xy_components(vector: &Vector) -> f64 {
        vector._y.atan2(vector._x) * TO_DEGREES
    }

    /// Deprecated: use `angle_between_vector_xy_components`.
    #[allow(dead_code)]
    pub fn angle_between_vector_xy_components_degrees(vector: &Vector) -> f64 {
        Self::angle_between_vector_xy_components(vector)
    }

    /// Sums a collection of vectors.
    pub fn sum_of_vectors(vectors: &[Vector]) -> Vector {
        let mut result = Vector::zero();
        for vector in vectors {
            result._x += vector._x;
            result._y += vector._y;
            result._z += vector._z;
        }
        result
    }

    /// Computes coordinate direction angles (alpha, beta, gamma) in degrees.
    pub fn coordinate_direction_3angles(&self, degrees: bool) -> [f64; 3] {
        let length = self.compute_length();
        if length < Tolerance::ZERO_TOLERANCE {
            return [0.0, 0.0, 0.0];
        }

        let cos_alpha = self._x / length;
        let cos_beta = self._y / length;
        let cos_gamma = self._z / length;

        let alpha = cos_alpha.acos();
        let beta = cos_beta.acos();
        let gamma = cos_gamma.acos();

        if degrees {
            [alpha * TO_DEGREES, beta * TO_DEGREES, gamma * TO_DEGREES]
        } else {
            [alpha, beta, gamma]
        }
    }

    /// Computes coordinate direction angles (phi, theta) in degrees.
    pub fn coordinate_direction_2angles(&self, degrees: bool) -> [f64; 2] {
        let length_xy = (self._x * self._x + self._y * self._y).sqrt();
        let length = self.compute_length();

        if length < Tolerance::ZERO_TOLERANCE {
            return [0.0, 0.0];
        }

        let phi = self._y.atan2(self._x);
        let theta = length_xy.atan2(self._z);

        if degrees {
            [phi * TO_DEGREES, theta * TO_DEGREES]
        } else {
            [phi, theta]
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Serializes the Vector to a JSON string.
    pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        SerTrait::serialize(self, &mut ser)?;
        Ok(String::from_utf8(buf)?)
    }

    /// Deserializes a Vector from a JSON string.
    pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json_data)?)
    }

    /// Serializes the Vector to a JSON file.
    pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.jsondump()?;
        std::fs::write(filepath, json)?;
        Ok(())
    }

    /// Deserializes a Vector from a JSON file.
    pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(filepath)?;
        Self::jsonload(&json)
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

// Index trait for array-like access
impl Index<usize> for Vector {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self._x,
            1 => &self._y,
            2 => &self._z,
            _ => panic!("Index out of bounds for Vector"),
        }
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.invalidate_length_cache();
        match index {
            0 => &mut self._x,
            1 => &mut self._y,
            2 => &mut self._z,
            _ => panic!("Index out of bounds for Vector"),
        }
    }
}

// Arithmetic operators
impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl Add for &Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Vector {
        Vector::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl Sub for &Vector {
    type Output = Vector;

    fn sub(self, other: &Vector) -> Vector {
        Vector::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f64) -> Vector {
        Vector::new(self.x() * scalar, self.y() * scalar, self.z() * scalar)
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, scalar: f64) -> Vector {
        Vector::new(self.x() * scalar, self.y() * scalar, self.z() * scalar)
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, scalar: f64) -> Vector {
        Vector::new(self.x() / scalar, self.y() / scalar, self.z() / scalar)
    }
}

impl Div<f64> for &Vector {
    type Output = Vector;

    fn div(self, scalar: f64) -> Vector {
        Vector::new(self.x() / scalar, self.y() / scalar, self.z() / scalar)
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector::new(-self.x(), -self.y(), -self.z())
    }
}

impl Neg for &Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector::new(-self.x(), -self.y(), -self.z())
    }
}

// Compound assignment operators
impl AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        self.set_x(self.x() + other.x());
        self.set_y(self.y() + other.y());
        self.set_z(self.z() + other.z());
    }
}

impl AddAssign<&Vector> for Vector {
    fn add_assign(&mut self, other: &Vector) {
        self.set_x(self.x() + other.x());
        self.set_y(self.y() + other.y());
        self.set_z(self.z() + other.z());
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Vector) {
        self.set_x(self.x() - other.x());
        self.set_y(self.y() - other.y());
        self.set_z(self.z() - other.z());
    }
}

impl SubAssign<&Vector> for Vector {
    fn sub_assign(&mut self, other: &Vector) {
        self.set_x(self.x() - other.x());
        self.set_y(self.y() - other.y());
        self.set_z(self.z() - other.z());
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, scalar: f64) {
        self.set_x(self.x() * scalar);
        self.set_y(self.y() * scalar);
        self.set_z(self.z() * scalar);
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, scalar: f64) {
        self.set_x(self.x() / scalar);
        self.set_y(self.y() / scalar);
        self.set_z(self.z() / scalar);
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Vector({}, {}, {}, {}, {})",
            self.x(),
            self.y(),
            self.z(),
            self.guid,
            self.name
        )
    }
}

#[cfg(test)]
#[path = "vector_test.rs"]
mod tests;
