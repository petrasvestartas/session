use serde::{Deserialize, Serialize, Serializer};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use std::fmt;
use crate::{HasJsonData, FromJsonData};
use serde_json::Value;


/// A vector in 3D space with x, y, z components
#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub struct Vector {
    /// The x component of the vector.
    pub x: f32,
    /// The y component of the vector.
    pub y: f32,
    /// The z component of the vector.
    pub z: f32,
}

// Custom Serialize implementation for simple format compatible with wink
impl Serialize for Vector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Vector", 3)?;
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.serialize_field("z", &self.z)?;
        state.end()
    }
}


impl Vector {
    /// Creates a new `Vector`.
    ///
    /// # Arguments
    ///
    /// * `x` - The x component.
    /// * `y` - The y component.
    /// * `z` - The z component.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Vector;
    /// let v = Vector::new(1.0, 2.0, 3.0);
    /// assert_eq!(v.x, 1.0);
    /// assert_eq!(v.y, 2.0);
    /// assert_eq!(v.z, 3.0);
    /// ```
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector { x, y, z }
    }

    /// Creates a zero vector (0, 0, 0).
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Vector;
    /// let v = Vector::zero();
    /// assert_eq!(v.x, 0.0);
    /// assert_eq!(v.y, 0.0);
    /// assert_eq!(v.z, 0.0);
    /// ```
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Creates a unit vector in the x direction.
    pub fn unit_x() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    /// Creates a unit vector in the y direction.
    pub fn unit_y() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    /// Creates a unit vector in the z direction.
    pub fn unit_z() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    /// Compute the squared magnitude (length) of the vector.
    /// This is more efficient than magnitude() when you only need to compare lengths.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Vector;
    /// let v = Vector::new(3.0, 4.0, 0.0);
    /// assert_eq!(v.magnitude_squared(), 25.0);
    /// ```
    pub fn magnitude_squared(&self) -> f32 {
        self.length_squared()
    }

    /// Check if all components of the vector are finite (not NaN or infinite).
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Vector;
    /// let v = Vector::new(1.0, 2.0, 3.0);
    /// assert!(v.is_finite());
    /// ```
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    /// Calculate the dot product of this vector with another vector.
    ///
    /// # Arguments
    ///
    /// * `other` - The other vector to calculate the dot product with.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Vector;
    /// let v1 = Vector::new(1.0, 2.0, 3.0);
    /// let v2 = Vector::new(4.0, 5.0, 6.0);
    /// assert_eq!(v1.dot(&v2), 32.0);
    /// ```
    pub fn dot(&self, other: &Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculate the cross product of this vector with another vector.
    ///
    /// # Arguments
    ///
    /// * `other` - The other vector to calculate the cross product with.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Vector;
    /// let v1 = Vector::new(1.0, 0.0, 0.0);
    /// let v2 = Vector::new(0.0, 1.0, 0.0);
    /// let v3 = v1.cross(&v2);
    /// assert_eq!(v3.x, 0.0);
    /// assert_eq!(v3.y, 0.0);
    /// assert_eq!(v3.z, 1.0);
    /// ```
    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Calculate the length (magnitude) of this vector.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Vector;
    /// let v = Vector::new(3.0, 4.0, 0.0);
    /// assert_eq!(v.length(), 5.0);
    /// ```
    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Calculate the squared length of this vector.
    /// This is faster than `length` as it avoids the square root calculation.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Vector;
    /// let v = Vector::new(3.0, 4.0, 0.0);
    /// assert_eq!(v.length_squared(), 25.0);
    /// ```
    pub fn length_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    /// Normalize this vector to have a length of 1.0.
    ///
    /// # Panics
    ///
    /// Panics if the vector has zero length.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Vector;
    /// let v = Vector::new(3.0, 0.0, 0.0);
    /// let normalized = v.normalize();
    /// assert_eq!(normalized.x, 1.0);
    /// assert_eq!(normalized.y, 0.0);
    /// assert_eq!(normalized.z, 0.0);
    /// ```
    pub fn normalize(&self) -> Vector {
        let length = self.length();
        assert!(length > 0.0, "Cannot normalize a zero-length vector");
        Vector {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    /// Normalize this vector in-place to have a length of 1.0.
    ///
    /// # Panics
    ///
    /// Panics if the vector has zero length.
    pub fn normalize_mut(&mut self) {
        let length = self.length();
        assert!(length > 0.0, "Cannot normalize a zero-length vector");
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    /// Check if the vector is very close to zero in all components
    pub fn is_zero(&self, tolerance: f32) -> bool {
        self.x.abs() < tolerance && self.y.abs() < tolerance && self.z.abs() < tolerance
    }
    
    /// Unitize the vector (normalize it to have a length of 1.0)
    /// Returns false if the vector is too small to be unitized
    /// 
    /// This is an alias for normalize_mut() that returns a boolean instead of panicking
    /// on zero vectors, for compatibility with geometry module code
    pub fn unitize(&mut self) -> bool {
        let length_squared = self.length_squared();
        if length_squared < 1e-10 {
            return false;
        }
        
        let length = length_squared.sqrt();
        self.x /= length;
        self.y /= length;
        self.z /= length;
        true
    }
}

// Implement Display
impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector({}, {}, {})", self.x, self.y, self.z)
    }
}

impl HasJsonData for Vector {
    fn to_json_data(&self, _minimal: bool) -> Value {
        serde_json::json!({
            "dtype": "Vector",
            "x": self.x,
            "y": self.y,
            "z": self.z
        })
    }
}

impl FromJsonData for Vector {
    fn from_json_data(data: &Value) -> Option<Self> {
        if let (Some(x), Some(y), Some(z)) = (
            data.get("x").and_then(|v| v.as_f64()).map(|v| v as f32),
            data.get("y").and_then(|v| v.as_f64()).map(|v| v as f32),
            data.get("z").and_then(|v| v.as_f64()).map(|v| v as f32),
        ) {
            Some(Vector::new(x, y, z))
        } else {
            None
        }
    }
}

// Implement Add for Vector + Vector = Vector
impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

// Implement Add for &Vector + &Vector = Vector
impl Add for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

// Implement AddAssign for Vector += Vector
impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// Implement Sub for Vector - Vector = Vector
impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// Implement Sub for &Vector - &Vector = Vector
impl Sub for &Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// Implement SubAssign for Vector -= Vector
impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

// Implement Mul for Vector * f64 = Vector
impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f32) -> Self::Output {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

// Implement Mul for &Vector * f64 = Vector
impl Mul<f32> for &Vector {
    type Output = Vector;

    fn mul(self, scalar: f32) -> Self::Output {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

// Implement MulAssign for Vector *= f64
impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

// Implement Div for Vector / f64 = Vector
impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, scalar: f32) -> Self::Output {
        assert!(!scalar.is_nan() && scalar != 0.0, "Division by zero or NaN");
        Vector {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

// Implement Div for &Vector / f64 = Vector
impl Div<f32> for &Vector {
    type Output = Vector;

    fn div(self, scalar: f32) -> Self::Output {
        assert!(!scalar.is_nan() && scalar != 0.0, "Division by zero or NaN");
        Vector {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

// Implement DivAssign for Vector /= f64
impl DivAssign<f32> for Vector {
    fn div_assign(&mut self, scalar: f32) {
        assert!(!scalar.is_nan() && scalar != 0.0, "Division by zero or NaN");
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

// Implement Neg for -Vector
impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// Implement Neg for -&Vector
impl Neg for &Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// Implement Index for vector[0], vector[1], vector[2]
impl Index<usize> for Vector {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds: {}", index),
        }
    }
}

// Implement IndexMut for vector[0] = 1.0, etc.
impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds: {}", index),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_new() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vector_zero() {
        let v = Vector::zero();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn test_vector_dot() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), 32.0);
    }

    #[test]
    fn test_vector_cross() {
        let v1 = Vector::unit_x();
        let v2 = Vector::unit_y();
        let v3 = v1.cross(&v2);
        assert_eq!(v3.x, 0.0);
        assert_eq!(v3.y, 0.0);
        assert_eq!(v3.z, 1.0);
    }

    #[test]
    fn test_vector_length() {
        let v = Vector::new(3.0, 4.0, 0.0);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn test_vector_normalize() {
        let v = Vector::new(3.0, 4.0, 0.0);
        let n = v.normalize();
        assert_eq!(n.x, 0.6);
        assert_eq!(n.y, 0.8);
        assert_eq!(n.z, 0.0);
    }
}
