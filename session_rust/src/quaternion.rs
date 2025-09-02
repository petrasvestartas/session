use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use std::fmt;

use crate::Vector;

/// A quaternion in scalar/vector form for 3D rotations.
/// 
/// Quaternions provide a compact and numerically stable way to represent 3D rotations.
/// They avoid gimbal lock and provide smooth interpolation between orientations.
#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Quaternion {
    /// The vector part of the quaternion (i, j, k components).
    pub v: Vector,
    /// The scalar part of the quaternion (w component).
    pub s: f32,
}

impl Quaternion {
    /// Construct a new quaternion from scalar and vector components.
    ///
    /// # Arguments
    ///
    /// * `w` - The scalar (real) component
    /// * `xi` - The i (x) component of the vector part
    /// * `yj` - The j (y) component of the vector part
    /// * `zk` - The k (z) component of the vector part
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Quaternion;
    /// let q = Quaternion::new(1.0, 0.0, 0.0, 0.0); // Identity quaternion
    /// ```
    #[inline]
    pub fn new(w: f32, xi: f32, yj: f32, zk: f32) -> Quaternion {
        Quaternion::from_sv(w, Vector::new(xi, yj, zk))
    }

    /// Construct a new quaternion from a scalar and a vector.
    ///
    /// # Arguments
    ///
    /// * `s` - The scalar component
    /// * `v` - The vector component
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::{Quaternion, Vector};
    /// let v = Vector::new(0.0, 0.0, 0.0);
    /// let q = Quaternion::from_sv(1.0, v); // Identity quaternion
    /// ```
    #[inline]
    pub fn from_sv(s: f32, v: Vector) -> Quaternion {
        Quaternion { v, s }
    }

    /// Create an identity quaternion (no rotation).
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Quaternion;
    /// let q = Quaternion::identity();
    /// assert_eq!(q.s, 1.0);
    /// assert_eq!(q.v.x, 0.0);
    /// ```
    #[inline]
    pub fn identity() -> Quaternion {
        Quaternion::from_sv(1.0, Vector::new(0.0, 0.0, 0.0))
    }

    /// Create a zero quaternion.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Quaternion;
    /// let q = Quaternion::zero();
    /// assert_eq!(q.s, 0.0);
    /// ```
    #[inline]
    pub fn zero() -> Quaternion {
        Quaternion::from_sv(0.0, Vector::new(0.0, 0.0, 0.0))
    }

    /// Create a quaternion from axis-angle representation.
    ///
    /// # Arguments
    ///
    /// * `axis` - The rotation axis (should be normalized)
    /// * `angle` - The rotation angle in radians
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::{Quaternion, Vector};
    /// use std::f32::consts::PI;
    /// let axis = Vector::new(0.0, 0.0, 1.0); // Z-axis
    /// let q = Quaternion::from_axis_angle(axis, PI / 2.0); // 90 degree rotation
    /// ```
    pub fn from_axis_angle(axis: Vector, angle: f32) -> Quaternion {
        let half_angle = angle * 0.5;
        let (sin_half, cos_half) = half_angle.sin_cos();
        Quaternion::from_sv(cos_half, axis * sin_half)
    }

    /// Create a quaternion representing the rotation from one vector to another.
    ///
    /// # Arguments
    ///
    /// * `src` - Source vector
    /// * `dst` - Destination vector
    /// * `fallback` - Optional fallback axis for 180-degree rotations
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::{Quaternion, Vector};
    /// let src = Vector::new(1.0, 0.0, 0.0);
    /// let dst = Vector::new(0.0, 1.0, 0.0);
    /// let q = Quaternion::from_arc(src, dst, None);
    /// ```
    pub fn from_arc(src: Vector, dst: Vector, fallback: Option<Vector>) -> Quaternion {
        let mag_avg = (src.magnitude_squared() * dst.magnitude_squared()).sqrt();
        let dot = src.dot(&dst);
        
        const EPSILON: f32 = 1e-6;
        
        if (dot - mag_avg).abs() < EPSILON {
            // Vectors are the same
            Quaternion::identity()
        } else if (dot + mag_avg).abs() < EPSILON {
            // Vectors are opposite
            let axis = fallback.unwrap_or_else(|| {
                let mut v = Vector::new(1.0, 0.0, 0.0).cross(&src);
                if v.magnitude_squared() < EPSILON {
                    v = Vector::new(0.0, 1.0, 0.0).cross(&src);
                }
                v.normalize()
            });
            Quaternion::from_axis_angle(axis, std::f32::consts::PI)
        } else {
            Quaternion::from_sv(mag_avg + dot, src.cross(&dst)).normalize()
        }
    }

    /// Compute the dot product of two quaternions.
    ///
    /// # Arguments
    ///
    /// * `other` - The other quaternion
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Quaternion;
    /// let q1 = Quaternion::identity();
    /// let q2 = Quaternion::identity();
    /// let dot = q1.dot(&q2);
    /// assert_eq!(dot, 1.0);
    /// ```
    #[inline]
    pub fn dot(&self, other: &Quaternion) -> f32 {
        self.s * other.s + self.v.dot(&other.v)
    }

    /// Compute the squared magnitude of the quaternion.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Quaternion;
    /// let q = Quaternion::identity();
    /// assert_eq!(q.magnitude_squared(), 1.0);
    /// ```
    #[inline]
    pub fn magnitude_squared(&self) -> f32 {
        self.s * self.s + self.v.magnitude_squared()
    }

    /// Compute the magnitude (length) of the quaternion.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Quaternion;
    /// let q = Quaternion::identity();
    /// assert_eq!(q.magnitude(), 1.0);
    /// ```
    #[inline]
    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    /// Normalize the quaternion to unit length.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Quaternion;
    /// let q = Quaternion::new(2.0, 0.0, 0.0, 0.0);
    /// let normalized = q.normalize();
    /// assert!((normalized.magnitude() - 1.0).abs() < 1e-6);
    /// ```
    pub fn normalize(&self) -> Quaternion {
        let mag = self.magnitude();
        if mag > 0.0 {
            *self / mag
        } else {
            Quaternion::identity()
        }
    }

    /// Compute the conjugate of the quaternion.
    ///
    /// The conjugate of a quaternion q = (w, x, y, z) is q* = (w, -x, -y, -z).
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Quaternion;
    /// let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    /// let conj = q.conjugate();
    /// assert_eq!(conj.s, 1.0);
    /// assert_eq!(conj.v.x, -2.0);
    /// ```
    #[inline]
    pub fn conjugate(&self) -> Quaternion {
        Quaternion::from_sv(self.s, -self.v)
    }

    /// Compute the inverse of the quaternion.
    ///
    /// For unit quaternions, the inverse is the same as the conjugate.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Quaternion;
    /// let q = Quaternion::identity();
    /// let inv = q.inverse();
    /// assert_eq!(inv.s, 1.0);
    /// ```
    pub fn inverse(&self) -> Quaternion {
        let mag_sq = self.magnitude_squared();
        if mag_sq > 0.0 {
            self.conjugate() / mag_sq
        } else {
            Quaternion::identity()
        }
    }

    /// Normalized linear interpolation between two quaternions.
    ///
    /// This is faster than slerp but less accurate for large rotations.
    ///
    /// # Arguments
    ///
    /// * `other` - The target quaternion
    /// * `t` - Interpolation parameter (0.0 = self, 1.0 = other)
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Quaternion;
    /// let q1 = Quaternion::identity();
    /// let q2 = Quaternion::new(0.0, 1.0, 0.0, 0.0);
    /// let result = q1.nlerp(q2, 0.5);
    /// ```
    pub fn nlerp(&self, mut other: Quaternion, t: f32) -> Quaternion {
        if self.dot(&other) < 0.0 {
            other = -other;
        }
        (*self * (1.0 - t) + other * t).normalize()
    }

    /// Spherical linear interpolation between two quaternions.
    ///
    /// This provides the smoothest interpolation but is more expensive than nlerp.
    ///
    /// # Arguments
    ///
    /// * `other` - The target quaternion
    /// * `t` - Interpolation parameter (0.0 = self, 1.0 = other)
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Quaternion;
    /// let q1 = Quaternion::identity();
    /// let q2 = Quaternion::new(0.0, 1.0, 0.0, 0.0);
    /// let result = q1.slerp(q2, 0.5);
    /// ```
    pub fn slerp(&self, mut other: Quaternion, t: f32) -> Quaternion {
        let mut dot = self.dot(&other);
        const DOT_THRESHOLD: f32 = 0.9995;

        if dot < 0.0 {
            other = -other;
            dot = -dot;
        }

        // If quaternions are close together, use nlerp
        if dot > DOT_THRESHOLD {
            self.nlerp(other, t)
        } else {
            // Stay within the domain of acos()
            let robust_dot = dot.min(1.0).max(-1.0);
            let theta = robust_dot.acos();

            let scale1 = (theta * (1.0 - t)).sin();
            let scale2 = (theta * t).sin();

            (*self * scale1 + other * scale2).normalize()
        }
    }

    /// Rotate a vector by this quaternion.
    ///
    /// # Arguments
    ///
    /// * `vec` - The vector to rotate
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::{Quaternion, Vector};
    /// let q = Quaternion::identity();
    /// let v = Vector::new(1.0, 0.0, 0.0);
    /// let rotated = q.rotate_vector(&v);
    /// ```
    pub fn rotate_vector(&self, vec: &Vector) -> Vector {
        // Using the formula: v' = q * v * q^(-1)
        // Optimized version: v' = v + 2 * cross(q.v, cross(q.v, v) + q.s * v)
        let two = 2.0;
        let tmp = self.v.cross(vec) + (*vec * self.s);
        *vec + (self.v.cross(&tmp) * two)
    }

    /// Check if the quaternion has finite components.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Quaternion;
    /// let q = Quaternion::identity();
    /// assert!(q.is_finite());
    /// ```
    pub fn is_finite(&self) -> bool {
        self.s.is_finite() && self.v.is_finite()
    }

    /// Convert quaternion to axis-angle representation.
    ///
    /// Returns (axis, angle) where axis is normalized and angle is in radians.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::{Quaternion, Vector};
    /// let axis = Vector::new(0.0, 0.0, 1.0);
    /// let angle = std::f32::consts::PI / 2.0;
    /// let q = Quaternion::from_axis_angle(axis, angle);
    /// let (recovered_axis, recovered_angle) = q.to_axis_angle();
    /// ```
    pub fn to_axis_angle(&self) -> (Vector, f32) {
        let q = self.normalize();
        let s = q.s.abs();
        
        if s >= 1.0 {
            // No rotation
            (Vector::new(0.0, 0.0, 1.0), 0.0)
        } else {
            let angle = 2.0 * q.s.acos();
            let sin_half_angle = (1.0 - s * s).sqrt();
            
            if sin_half_angle < 1e-6 {
                // Avoid division by zero
                (Vector::new(0.0, 0.0, 1.0), 0.0)
            } else {
                let axis = q.v / sin_half_angle;
                (axis, angle)
            }
        }
    }

    /// Create a quaternion that rotates from one direction to another.
    ///
    /// Both vectors should be normalized.
    ///
    /// # Arguments
    ///
    /// * `from` - The starting direction vector
    /// * `to` - The target direction vector
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::{Quaternion, Vector};
    /// let from = Vector::new(1.0, 0.0, 0.0);
    /// let to = Vector::new(0.0, 1.0, 0.0);
    /// let q = Quaternion::between_vectors(from, to);
    /// ```
    pub fn between_vectors(from: Vector, to: Vector) -> Self {
        let dot = from.dot(&to);
        
        // If vectors are nearly identical, return identity
        if dot >= 0.99999 {
            return Self::identity();
        }
        
        // If vectors are nearly opposite, find a perpendicular axis
        if dot <= -0.99999 {
            // Find a perpendicular vector
            let mut axis = Vector::new(1.0, 0.0, 0.0).cross(&from);
            if axis.magnitude_squared() < 0.000001 {
                axis = Vector::new(0.0, 1.0, 0.0).cross(&from);
            }
            axis = axis.normalize();
            return Self::from_axis_angle(axis, std::f32::consts::PI);
        }
        
        // Normal case: compute rotation axis and angle
        let axis = from.cross(&to).normalize();
        let angle = dot.acos();
        Self::from_axis_angle(axis, angle)
    }

    /// Create a quaternion from a look-at direction and up vector.
    ///
    /// This creates a rotation that aligns the negative Z axis with the direction
    /// and the Y axis with the up vector (right-handed coordinate system).
    ///
    /// # Arguments
    ///
    /// * `direction` - The forward direction vector
    /// * `up` - The up direction vector
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::{Quaternion, Vector};
    /// let direction = Vector::new(0.0, 0.0, -1.0);
    /// let up = Vector::new(0.0, 1.0, 0.0);
    /// let q = Quaternion::look_at(direction, up);
    /// ```
    pub fn look_at(direction: Vector, up: Vector) -> Self {
        let forward = direction.normalize();
        let right = forward.cross(&up).normalize();
        let up_corrected = right.cross(&forward).normalize();
        
        // Create rotation matrix columns
        let m00 = right.x;
        let m01 = up_corrected.x;
        let m02 = -forward.x;
        
        let m10 = right.y;
        let m11 = up_corrected.y;
        let m12 = -forward.y;
        
        let m20 = right.z;
        let m21 = up_corrected.z;
        let m22 = -forward.z;
        
        // Convert rotation matrix to quaternion
        Self::from_rotation_matrix(m00, m01, m02, m10, m11, m12, m20, m21, m22)
    }

    /// Create a quaternion from a 3x3 rotation matrix elements.
    ///
    /// Matrix is in column-major order: [right, up, forward]
    ///
    /// # Arguments
    ///
    /// * Matrix elements in row-major order
    fn from_rotation_matrix(
        m00: f32, m01: f32, m02: f32,
        m10: f32, m11: f32, m12: f32,
        m20: f32, m21: f32, m22: f32,
    ) -> Self {
        // Shepperd's method for converting rotation matrix to quaternion
        let trace = m00 + m11 + m22;
        
        if trace > 0.0 {
            let s = (trace + 1.0).sqrt() * 2.0; // s = 4 * qw
            let w = 0.25 * s;
            let x = (m21 - m12) / s;
            let y = (m02 - m20) / s;
            let z = (m10 - m01) / s;
            Self::new(w, x, y, z)
        } else if m00 > m11 && m00 > m22 {
            let s = (1.0 + m00 - m11 - m22).sqrt() * 2.0; // s = 4 * qx
            let w = (m21 - m12) / s;
            let x = 0.25 * s;
            let y = (m01 + m10) / s;
            let z = (m02 + m20) / s;
            Self::new(w, x, y, z)
        } else if m11 > m22 {
            let s = (1.0 + m11 - m00 - m22).sqrt() * 2.0; // s = 4 * qy
            let w = (m02 - m20) / s;
            let x = (m01 + m10) / s;
            let y = 0.25 * s;
            let z = (m12 + m21) / s;
            Self::new(w, x, y, z)
        } else {
            let s = (1.0 + m22 - m00 - m11).sqrt() * 2.0; // s = 4 * qz
            let w = (m10 - m01) / s;
            let x = (m02 + m20) / s;
            let y = (m12 + m21) / s;
            let z = 0.25 * s;
            Self::new(w, x, y, z)
        }
    }
}

impl Default for Quaternion {
    /// Creates an identity quaternion (no rotation).
    fn default() -> Self {
        Quaternion::identity()
    }
}

impl PartialEq for Quaternion {
    /// Check if two quaternions are equal within floating-point precision.
    fn eq(&self, other: &Self) -> bool {
        const EPSILON: f32 = 1e-6;
        (self.s - other.s).abs() < EPSILON && self.v == other.v
    }
}

// Arithmetic operations

impl Neg for Quaternion {
    type Output = Quaternion;

    /// Negate all components of the quaternion.
    fn neg(self) -> Quaternion {
        Quaternion::from_sv(-self.s, -self.v)
    }
}

impl Add<Quaternion> for Quaternion {
    type Output = Quaternion;

    /// Add two quaternions component-wise.
    fn add(self, other: Quaternion) -> Quaternion {
        Quaternion::from_sv(self.s + other.s, self.v + other.v)
    }
}

impl AddAssign<Quaternion> for Quaternion {
    /// Add another quaternion to this one in-place.
    fn add_assign(&mut self, other: Quaternion) {
        self.s += other.s;
        self.v += other.v;
    }
}

impl Sub<Quaternion> for Quaternion {
    type Output = Quaternion;

    /// Subtract two quaternions component-wise.
    fn sub(self, other: Quaternion) -> Quaternion {
        Quaternion::from_sv(self.s - other.s, self.v - other.v)
    }
}

impl SubAssign<Quaternion> for Quaternion {
    /// Subtract another quaternion from this one in-place.
    fn sub_assign(&mut self, other: Quaternion) {
        self.s -= other.s;
        self.v -= other.v;
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    /// Multiply two quaternions (quaternion composition).
    fn mul(self, other: Quaternion) -> Quaternion {
        Quaternion::new(
            self.s * other.s - self.v.x * other.v.x - self.v.y * other.v.y - self.v.z * other.v.z,
            self.s * other.v.x + self.v.x * other.s + self.v.y * other.v.z - self.v.z * other.v.y,
            self.s * other.v.y + self.v.y * other.s + self.v.z * other.v.x - self.v.x * other.v.z,
            self.s * other.v.z + self.v.z * other.s + self.v.x * other.v.y - self.v.y * other.v.x,
        )
    }
}

impl Mul<f32> for Quaternion {
    type Output = Quaternion;

    /// Multiply quaternion by a scalar.
    fn mul(self, scalar: f32) -> Quaternion {
        Quaternion::from_sv(self.s * scalar, self.v * scalar)
    }
}

impl Mul<Quaternion> for f32 {
    type Output = Quaternion;

    /// Multiply scalar by quaternion.
    fn mul(self, quat: Quaternion) -> Quaternion {
        quat * self
    }
}

impl MulAssign<f32> for Quaternion {
    /// Multiply this quaternion by a scalar in-place.
    fn mul_assign(&mut self, scalar: f32) {
        self.s *= scalar;
        self.v *= scalar;
    }
}

impl Div<f32> for Quaternion {
    type Output = Quaternion;

    /// Divide quaternion by a scalar.
    fn div(self, scalar: f32) -> Quaternion {
        Quaternion::from_sv(self.s / scalar, self.v / scalar)
    }
}

impl DivAssign<f32> for Quaternion {
    /// Divide this quaternion by a scalar in-place.
    fn div_assign(&mut self, scalar: f32) {
        self.s /= scalar;
        self.v /= scalar;
    }
}

// Array and tuple conversions

impl From<[f32; 4]> for Quaternion {
    /// Create quaternion from array [x, y, z, w].
    fn from(arr: [f32; 4]) -> Quaternion {
        Quaternion::new(arr[3], arr[0], arr[1], arr[2])
    }
}

impl From<Quaternion> for [f32; 4] {
    /// Convert quaternion to array [x, y, z, w].
    fn from(q: Quaternion) -> [f32; 4] {
        [q.v.x, q.v.y, q.v.z, q.s]
    }
}

impl From<(f32, f32, f32, f32)> for Quaternion {
    /// Create quaternion from tuple (x, y, z, w).
    fn from(tuple: (f32, f32, f32, f32)) -> Quaternion {
        Quaternion::new(tuple.3, tuple.0, tuple.1, tuple.2)
    }
}

impl From<Quaternion> for (f32, f32, f32, f32) {
    /// Convert quaternion to tuple (x, y, z, w).
    fn from(q: Quaternion) -> (f32, f32, f32, f32) {
        (q.v.x, q.v.y, q.v.z, q.s)
    }
}

// Indexing

impl Index<usize> for Quaternion {
    type Output = f32;

    /// Access quaternion components by index: [0] = x, [1] = y, [2] = z, [3] = w.
    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.v.x,
            1 => &self.v.y,
            2 => &self.v.z,
            3 => &self.s,
            _ => panic!("Index out of bounds for Quaternion"),
        }
    }
}

impl IndexMut<usize> for Quaternion {
    /// Mutably access quaternion components by index.
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.v.x,
            1 => &mut self.v.y,
            2 => &mut self.v.z,
            3 => &mut self.s,
            _ => panic!("Index out of bounds for Quaternion"),
        }
    }
}

impl fmt::Display for Quaternion {
    /// Format quaternion for display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Quaternion(w: {}, x: {}, y: {}, z: {})", self.s, self.v.x, self.v.y, self.v.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_identity() {
        let q = Quaternion::identity();
        assert_eq!(q.s, 1.0);
        assert_eq!(q.v.x, 0.0);
        assert_eq!(q.v.y, 0.0);
        assert_eq!(q.v.z, 0.0);
    }

    #[test]
    fn test_from_axis_angle() {
        let axis = Vector::new(0.0, 0.0, 1.0);
        let angle = PI / 2.0;
        let q = Quaternion::from_axis_angle(axis, angle);
        
        // Should be approximately (cos(π/4), 0, 0, sin(π/4))
        let expected_w = (PI / 4.0).cos();
        let expected_z = (PI / 4.0).sin();
        
        assert!((q.s - expected_w).abs() < 1e-6);
        assert!((q.v.z - expected_z).abs() < 1e-6);
    }

    #[test]
    fn test_normalize() {
        let q = Quaternion::new(2.0, 0.0, 0.0, 0.0);
        let normalized = q.normalize();
        assert!((normalized.magnitude() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_conjugate() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let conj = q.conjugate();
        assert_eq!(conj.s, 1.0);
        assert_eq!(conj.v.x, -2.0);
        assert_eq!(conj.v.y, -3.0);
        assert_eq!(conj.v.z, -4.0);
    }

    #[test]
    fn test_multiplication() {
        let q1 = Quaternion::identity();
        let q2 = Quaternion::new(0.0, 1.0, 0.0, 0.0);
        let result = q1 * q2;
        assert_eq!(result, q2);
    }

    #[test]
    fn test_vector_rotation() {
        // 90-degree rotation around Z-axis
        let axis = Vector::new(0.0, 0.0, 1.0);
        let q = Quaternion::from_axis_angle(axis, PI / 2.0);
        
        let v = Vector::new(1.0, 0.0, 0.0);
        let rotated = q.rotate_vector(&v);
        
        // Should rotate (1,0,0) to approximately (0,1,0)
        assert!((rotated.x - 0.0).abs() < 1e-6);
        assert!((rotated.y - 1.0).abs() < 1e-6);
        assert!((rotated.z - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_slerp() {
        let q1 = Quaternion::identity();
        let q2 = Quaternion::from_axis_angle(Vector::new(0.0, 0.0, 1.0), PI / 2.0);
        
        let result = q1.slerp(q2, 0.5);
        
        // Should be halfway between identity and 90-degree rotation
        let expected = Quaternion::from_axis_angle(Vector::new(0.0, 0.0, 1.0), PI / 4.0);
        
        assert!((result.s - expected.s).abs() < 1e-6);
        assert!((result.v.z - expected.v.z).abs() < 1e-6);
    }

    #[test]
    fn test_conversions() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        
        // Test array conversion
        let arr: [f32; 4] = q.into();
        assert_eq!(arr, [2.0, 3.0, 4.0, 1.0]);
        
        let q2 = Quaternion::from(arr);
        assert_eq!(q, q2);
        
        // Test tuple conversion
        let tuple: (f32, f32, f32, f32) = q.into();
        assert_eq!(tuple, (2.0, 3.0, 4.0, 1.0));
        
        let q3 = Quaternion::from(tuple);
        assert_eq!(q, q3);
    }

    #[test]
    fn test_array_conversion() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let arr: [f32; 4] = q.into();
        assert_eq!(arr, [2.0, 3.0, 4.0, 1.0]); // [x, y, z, w]
        
        let q2 = Quaternion::from(arr);
        assert_eq!(q, q2);
    }

    #[test]
    fn test_indexing() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(q[0], 2.0); // x
        assert_eq!(q[1], 3.0); // y
        assert_eq!(q[2], 4.0); // z
        assert_eq!(q[3], 1.0); // w
    }

    #[test]
    fn test_between_vectors() {
        // Test identity case
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(1.0, 0.0, 0.0);
        let q = Quaternion::between_vectors(v1, v2);
        assert!((q.s - 1.0).abs() < 1e-6);

        // Test 90 degree rotation
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(0.0, 1.0, 0.0);
        let q = Quaternion::between_vectors(v1, v2);
        let rotated = q.rotate_vector(&v1);
        assert!((rotated.x - 0.0).abs() < 1e-6);
        assert!((rotated.y - 1.0).abs() < 1e-6);
        assert!((rotated.z - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_look_at() {
        // Test looking down negative Z (default forward)
        let direction = Vector::new(0.0, 0.0, -1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let q = Quaternion::look_at(direction, up);
        
        // Should be close to identity for looking down -Z
        let forward_vec = Vector::new(0.0, 0.0, -1.0);
        let rotated = q.rotate_vector(&forward_vec);
        assert!((rotated.x - direction.x).abs() < 1e-6);
        assert!((rotated.y - direction.y).abs() < 1e-6);
        assert!((rotated.z - direction.z).abs() < 1e-6);
    }
}