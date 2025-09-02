use crate::{Vector, Point};
use serde::{Deserialize, Serialize, Serializer};
use std::ops::{Index, IndexMut, Mul, MulAssign};
use std::fmt;

/// A 4x4 transformation matrix in 3D space
/// Stored in column-major order (standard in graphics)
#[derive(Debug, Clone, Deserialize)]
pub struct Xform {
    /// The matrix elements stored in column-major order as a flattened array
    pub m: [f32; 16],
}

impl Xform {
    /// Creates a new transformation matrix with all elements set to the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to initialize all elements with
    pub fn new(value: f32) -> Self {
        Xform { m: [value; 16] }
    }

    /// Creates a new transformation matrix from a 4x4 matrix array.
    ///
    /// # Arguments
    ///
    /// * `matrix` - The 16-element array representing the 4x4 matrix in column-major order
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Xform;
    /// let matrix = [
    ///     1.0, 0.0, 0.0, 0.0,
    ///     0.0, 1.0, 0.0, 0.0,
    ///     0.0, 0.0, 1.0, 0.0,
    ///     0.0, 0.0, 0.0, 1.0
    /// ];
    /// let xform = Xform::from_matrix(matrix);
    /// ```
    pub fn from_matrix(matrix: [f32; 16]) -> Self {
        Xform { m: matrix }
    }

    /// Creates a new identity transformation matrix.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Xform;
    /// let xform = Xform::identity();
    /// assert_eq!(xform[(0, 0)], 1.0);
    /// assert_eq!(xform[(1, 1)], 1.0);
    /// assert_eq!(xform[(2, 2)], 1.0);
    /// assert_eq!(xform[(3, 3)], 1.0);
    /// assert_eq!(xform[(0, 1)], 0.0);
    /// ```
    pub fn identity() -> Self {
        let mut xform = Xform::new(0.0);
        xform.m[0] = 1.0;
        xform.m[5] = 1.0;
        xform.m[10] = 1.0;
        xform.m[15] = 1.0;
        xform
    }

    /// Creates a new translation transformation matrix.
    ///
    /// # Arguments
    ///
    /// * `tx`, `ty`, `tz` - Translation amounts in x, y, and z directions
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Xform;
    /// use session_rust::primitives::Point;
    /// let translate = Xform::translation(2.0, 3.0, 4.0);
    /// let point = Point::new(1.0, 1.0, 1.0);
    /// let transformed = translate.transform_point(&point);
    /// assert_eq!(transformed.x, 3.0);
    /// assert_eq!(transformed.y, 4.0);
    /// assert_eq!(transformed.z, 5.0);
    /// ```
    pub fn translation(tx: f32, ty: f32, tz: f32) -> Self {
        let mut xform = Self::identity();
        xform.m[12] = tx;
        xform.m[13] = ty;
        xform.m[14] = tz;
        xform
    }

    /// Creates a new scaling transformation matrix.
    ///
    /// # Arguments
    ///
    /// * `sx`, `sy`, `sz` - Scale factors in x, y, and z directions
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Xform;
    /// use session_rust::primitives::Point;
    /// let scale = Xform::scaling(2.0, 3.0, 4.0);
    /// let point = Point::new(1.0, 1.0, 1.0);
    /// let transformed = scale.transform_point(&point);
    /// assert_eq!(transformed.x, 2.0);
    /// assert_eq!(transformed.y, 3.0);
    /// assert_eq!(transformed.z, 4.0);
    /// ```
    pub fn scaling(sx: f32, sy: f32, sz: f32) -> Self {
        let mut xform = Self::identity();
        xform.m[0] = sx;
        xform.m[5] = sy;
        xform.m[10] = sz;
        xform
    }

    /// Creates a new rotation transformation matrix around the X axis.
    ///
    /// # Arguments
    ///
    /// * `angle_radians` - Rotation angle in radians
    pub fn rotation_x(angle_radians: f32) -> Self {
        let mut xform = Self::identity();
        let cos_angle = angle_radians.cos();
        let sin_angle = angle_radians.sin();
        
        xform.m[5] = cos_angle;
        xform.m[6] = sin_angle;
        xform.m[9] = -sin_angle;
        xform.m[10] = cos_angle;
        
        xform
    }

    /// Creates a new rotation transformation matrix around the Y axis.
    ///
    /// # Arguments
    ///
    /// * `angle_radians` - Rotation angle in radians
    pub fn rotation_y(angle_radians: f32) -> Self {
        let mut xform = Self::identity();
        let cos_angle = angle_radians.cos();
        let sin_angle = angle_radians.sin();
        
        xform.m[0] = cos_angle;
        xform.m[2] = -sin_angle;
        xform.m[8] = sin_angle;
        xform.m[10] = cos_angle;
        
        xform
    }

    /// Creates a new rotation transformation matrix around the Z axis.
    ///
    /// # Arguments
    ///
    /// * `angle_radians` - Rotation angle in radians
    pub fn rotation_z(angle_radians: f32) -> Self {
        let mut xform = Self::identity();
        let cos_angle = angle_radians.cos();
        let sin_angle = angle_radians.sin();
        
        xform.m[0] = cos_angle;
        xform.m[1] = sin_angle;
        xform.m[4] = -sin_angle;
        xform.m[5] = cos_angle;
        
        xform
    }

    /// Creates a new rotation transformation matrix around an arbitrary axis.
    ///
    /// # Arguments
    ///
    /// * `axis` - The axis of rotation (should be normalized)
    /// * `angle_radians` - Rotation angle in radians
    pub fn rotation(axis: &Vector, angle_radians: f32) -> Self {
        assert!((axis.length() - 1.0).abs() < 1e-6, "Axis must be normalized");
        
        let mut xform = Self::identity();
        let cos_angle = angle_radians.cos();
        let sin_angle = angle_radians.sin();
        let one_minus_cos = 1.0 - cos_angle;
        
        let xx = axis.x * axis.x;
        let xy = axis.x * axis.y;
        let xz = axis.x * axis.z;
        let yy = axis.y * axis.y;
        let yz = axis.y * axis.z;
        let zz = axis.z * axis.z;
        
        xform.m[0] = cos_angle + xx * one_minus_cos;
        xform.m[1] = xy * one_minus_cos + axis.z * sin_angle;
        xform.m[2] = xz * one_minus_cos - axis.y * sin_angle;
        
        xform.m[4] = xy * one_minus_cos - axis.z * sin_angle;
        xform.m[5] = cos_angle + yy * one_minus_cos;
        xform.m[6] = yz * one_minus_cos + axis.x * sin_angle;
        
        xform.m[8] = xz * one_minus_cos + axis.y * sin_angle;
        xform.m[9] = yz * one_minus_cos - axis.x * sin_angle;
        xform.m[10] = cos_angle + zz * one_minus_cos;
        
        xform
    }

    /// Creates a new transformation matrix that changes the basis from one coordinate system to another.
    ///
    /// # Arguments
    ///
    /// * `origin` - The origin of the new coordinate system in the old coordinate system
    /// * `x_axis`, `y_axis`, `z_axis` - The basis vectors of the new coordinate system (should be normalized)
    pub fn change_basis(origin: &Point, x_axis: &Vector, y_axis: &Vector, z_axis: &Vector) -> Self {
        assert!((x_axis.length() - 1.0).abs() < 1e-6, "X axis must be normalized");
        assert!((y_axis.length() - 1.0).abs() < 1e-6, "Y axis must be normalized");
        assert!((z_axis.length() - 1.0).abs() < 1e-6, "Z axis must be normalized");
        
        let mut xform = Self::identity();
        
        // Set the basis vectors
        xform.m[0] = x_axis.x;
        xform.m[1] = x_axis.y;
        xform.m[2] = x_axis.z;
        
        xform.m[4] = y_axis.x;
        xform.m[5] = y_axis.y;
        xform.m[6] = y_axis.z;
        
        xform.m[8] = z_axis.x;
        xform.m[9] = z_axis.y;
        xform.m[10] = z_axis.z;
        
        // Set the origin
        xform.m[12] = origin.x;
        xform.m[13] = origin.y;
        xform.m[14] = origin.z;
        
        xform
    }

    /// Returns the inverse of this transformation matrix.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Xform;
    /// use session_rust::primitives::Point;
    /// let transform = Xform::translation(2.0, 3.0, 4.0);
    /// let inverse = transform.inverse().unwrap();
    /// let point = Point::new(5.0, 6.0, 7.0);
    /// let transformed = transform.transform_point(&point);
    /// let back = inverse.transform_point(&transformed);
    /// assert!((back.x - point.x).abs() < 1e-10);
    /// assert!((back.y - point.y).abs() < 1e-10);
    /// assert!((back.z - point.z).abs() < 1e-10);
    /// ```
    pub fn inverse(&self) -> Option<Xform> {
        let mut result = Xform::new(0.0);
        
        // Compute the inverse using the adjugate and determinant
        let m = &self.m;
        
        // Calculate cofactors
        let c00 = m[5] * (m[10] * m[15] - m[11] * m[14]) - m[6] * (m[9] * m[15] - m[11] * m[13]) + m[7] * (m[9] * m[14] - m[10] * m[13]);
        let c01 = -(m[4] * (m[10] * m[15] - m[11] * m[14]) - m[6] * (m[8] * m[15] - m[11] * m[12]) + m[7] * (m[8] * m[14] - m[10] * m[12]));
        let c02 = m[4] * (m[9] * m[15] - m[11] * m[13]) - m[5] * (m[8] * m[15] - m[11] * m[12]) + m[7] * (m[8] * m[13] - m[9] * m[12]);
        let c03 = -(m[4] * (m[9] * m[14] - m[10] * m[13]) - m[5] * (m[8] * m[14] - m[10] * m[12]) + m[6] * (m[8] * m[13] - m[9] * m[12]));
        
        let c10 = -(m[1] * (m[10] * m[15] - m[11] * m[14]) - m[2] * (m[9] * m[15] - m[11] * m[13]) + m[3] * (m[9] * m[14] - m[10] * m[13]));
        let c11 = m[0] * (m[10] * m[15] - m[11] * m[14]) - m[2] * (m[8] * m[15] - m[11] * m[12]) + m[3] * (m[8] * m[14] - m[10] * m[12]);
        let c12 = -(m[0] * (m[9] * m[15] - m[11] * m[13]) - m[1] * (m[8] * m[15] - m[11] * m[12]) + m[3] * (m[8] * m[13] - m[9] * m[12]));
        let c13 = m[0] * (m[9] * m[14] - m[10] * m[13]) - m[1] * (m[8] * m[14] - m[10] * m[12]) + m[2] * (m[8] * m[13] - m[9] * m[12]);
        
        let c20 = m[1] * (m[6] * m[15] - m[7] * m[14]) - m[2] * (m[5] * m[15] - m[7] * m[13]) + m[3] * (m[5] * m[14] - m[6] * m[13]);
        let c21 = -(m[0] * (m[6] * m[15] - m[7] * m[14]) - m[2] * (m[4] * m[15] - m[7] * m[12]) + m[3] * (m[4] * m[14] - m[6] * m[12]));
        let c22 = m[0] * (m[5] * m[15] - m[7] * m[13]) - m[1] * (m[4] * m[15] - m[7] * m[12]) + m[3] * (m[4] * m[13] - m[5] * m[12]);
        let c23 = -(m[0] * (m[5] * m[14] - m[6] * m[13]) - m[1] * (m[4] * m[14] - m[6] * m[12]) + m[2] * (m[4] * m[13] - m[5] * m[12]));
        
        let c30 = -(m[1] * (m[6] * m[11] - m[7] * m[10]) - m[2] * (m[5] * m[11] - m[7] * m[9]) + m[3] * (m[5] * m[10] - m[6] * m[9]));
        let c31 = m[0] * (m[6] * m[11] - m[7] * m[10]) - m[2] * (m[4] * m[11] - m[7] * m[8]) + m[3] * (m[4] * m[10] - m[6] * m[8]);
        let c32 = -(m[0] * (m[5] * m[11] - m[7] * m[9]) - m[1] * (m[4] * m[11] - m[7] * m[8]) + m[3] * (m[4] * m[9] - m[5] * m[8]));
        let c33 = m[0] * (m[5] * m[10] - m[6] * m[9]) - m[1] * (m[4] * m[10] - m[6] * m[8]) + m[2] * (m[4] * m[9] - m[5] * m[8]);
        
        // Calculate determinant
        let det = m[0] * c00 + m[1] * c01 + m[2] * c02 + m[3] * c03;
        
        // Check if determinant is too close to zero
        if det.abs() < 1e-10 {
            return None;
        }
        
        // Calculate inverse
        let inv_det = 1.0 / det;
        
        // Transpose of cofactor matrix divided by determinant
        result.m[0] = c00 * inv_det;
        result.m[1] = c10 * inv_det;
        result.m[2] = c20 * inv_det;
        result.m[3] = c30 * inv_det;
        
        result.m[4] = c01 * inv_det;
        result.m[5] = c11 * inv_det;
        result.m[6] = c21 * inv_det;
        result.m[7] = c31 * inv_det;
        
        result.m[8] = c02 * inv_det;
        result.m[9] = c12 * inv_det;
        result.m[10] = c22 * inv_det;
        result.m[11] = c32 * inv_det;
        
        result.m[12] = c03 * inv_det;
        result.m[13] = c13 * inv_det;
        result.m[14] = c23 * inv_det;
        result.m[15] = c33 * inv_det;
        
        Some(result)
    }

    /// Transforms a point by this transformation matrix.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to transform
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Xform;
    /// use session_rust::primitives::Point;
    /// let transform = Xform::translation(4.0, 6.0, 8.0);
    /// let point = Point::new(1.0, 1.0, 1.0);
    /// let transformed = transform.transform_point(&point);
    /// assert_eq!(transformed.x, 5.0);
    /// assert_eq!(transformed.y, 7.0);
    /// assert_eq!(transformed.z, 9.0);
    /// ```
    pub fn transform_point(&self, point: &Point) -> Point {
        let m = &self.m;
        let w = m[3] * point.x + m[7] * point.y + m[11] * point.z + m[15];
        let w_inv = if w.abs() > 1e-10 { 1.0 / w } else { 1.0 };
        
        Point::new(
            (m[0] * point.x + m[4] * point.y + m[8] * point.z + m[12]) * w_inv,
            (m[1] * point.x + m[5] * point.y + m[9] * point.z + m[13]) * w_inv,
            (m[2] * point.x + m[6] * point.y + m[10] * point.z + m[14]) * w_inv
        )
    }

    /// Transforms a vector by this transformation matrix.
    /// Unlike points, vectors are not affected by the translation part of the transformation.
    ///
    /// # Arguments
    ///
    /// * `vector` - The vector to transform
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::{Xform, Vector};
    /// let transform = Xform::scaling(2.0, 3.0, 4.0);
    /// let vector = Vector::new(1.0, 1.0, 1.0);
    /// let transformed = transform.transform_vector(&vector);
    /// assert_eq!(transformed.x, 2.0);
    /// assert_eq!(transformed.y, 3.0);
    /// assert_eq!(transformed.z, 4.0);
    /// ```
    pub fn transform_vector(&self, vector: &Vector) -> Vector {
        let m = &self.m;
        
        Vector {
            x: m[0] * vector.x + m[4] * vector.y + m[8] * vector.z,
            y: m[1] * vector.x + m[5] * vector.y + m[9] * vector.z,
            z: m[2] * vector.x + m[6] * vector.y + m[10] * vector.z,
        }
    }

    /// Checks if this transform is the identity matrix.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Xform;
    /// let identity = Xform::identity();
    /// assert!(identity.is_identity());
    /// ```
    pub fn is_identity(&self) -> bool {
        let identity = Xform::identity();
        for i in 0..16 {
            if (self.m[i] - identity.m[i]).abs() > 1e-10 {
                return false;
            }
        }
        true
    }
}

// Implement Display for Xform
impl fmt::Display for Xform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Transform Matrix:")?;
        writeln!(f, "[{:.4}, {:.4}, {:.4}, {:.4}]", self.m[0], self.m[4], self.m[8], self.m[12])?;
        writeln!(f, "[{:.4}, {:.4}, {:.4}, {:.4}]", self.m[1], self.m[5], self.m[9], self.m[13])?;
        writeln!(f, "[{:.4}, {:.4}, {:.4}, {:.4}]", self.m[2], self.m[6], self.m[10], self.m[14])?;
        write!(f, "[{:.4}, {:.4}, {:.4}, {:.4}]", self.m[3], self.m[7], self.m[11], self.m[15])
    }
}

/// Implement Default for Xform to return identity matrix
impl Default for Xform {
    fn default() -> Self {
        Self::identity()
    }
}

// Implement Index trait for accessing matrix elements with [(row, col)] syntax
impl Index<(usize, usize)> for Xform {
    type Output = f32;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        let (row, col) = idx;
        assert!(row < 4 && col < 4, "Index out of bounds: ({}, {})", row, col);
        // Column-major order: index = col * 4 + row
        &self.m[col * 4 + row]
    }
}

// Implement IndexMut trait for modifying matrix elements with [(row, col)] syntax
impl IndexMut<(usize, usize)> for Xform {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        let (row, col) = idx;
        assert!(row < 4 && col < 4, "Index out of bounds: ({}, {})", row, col);
        // Column-major order: index = col * 4 + row
        &mut self.m[col * 4 + row]
    }
}

// Implement Mul for matrix multiplication: Xform * Xform = Xform
impl Mul for &Xform {
    type Output = Xform;

    fn mul(self, rhs: &Xform) -> Self::Output {
        let mut result = Xform::new(0.0);
        
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    // self[i,k] * rhs[k,j]
                    sum += self[(i, k)] * rhs[(k, j)];
                }
                result[(i, j)] = sum;
            }
        }
        
        result
    }
}

// Implement Mul for owned matrices
impl Mul for Xform {
    type Output = Xform;

    fn mul(self, rhs: Xform) -> Self::Output {
        &self * &rhs
    }
}

// Implement MulAssign for in-place matrix multiplication: xform *= other_xform
impl MulAssign for Xform {
    fn mul_assign(&mut self, rhs: Self) {
        *self = &*self * &rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_identity() {
        let identity = Xform::identity();
        assert_eq!(identity[(0, 0)], 1.0);
        assert_eq!(identity[(1, 1)], 1.0);
        assert_eq!(identity[(2, 2)], 1.0);
        assert_eq!(identity[(3, 3)], 1.0);
        
        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    assert_eq!(identity[(i, j)], 1.0);
                } else {
                    assert_eq!(identity[(i, j)], 0.0);
                }
            }
        }
    }

    #[test]
    fn test_translation() {
        let translate = Xform::translation(2.0, 3.0, 4.0);
        let point = Point::new(1.0, 1.0, 1.0);
        let transformed = translate.transform_point(&point);
        
        assert_eq!(transformed.x, 3.0);
        assert_eq!(transformed.y, 4.0);
        assert_eq!(transformed.z, 5.0);
    }

    #[test]
    fn test_scaling() {
        let scale = Xform::scaling(2.0, 3.0, 4.0);
        let point = Point::new(1.0, 1.0, 1.0);
        let transformed = scale.transform_point(&point);
        
        assert_eq!(transformed.x, 2.0);
        assert_eq!(transformed.y, 3.0);
        assert_eq!(transformed.z, 4.0);
    }

    #[test]
    fn test_rotation() {
        // Test 90-degree rotation around z-axis
        let rotation = Xform::rotation_z(PI / 2.0);
        let point = Point::new(1.0, 0.0, 0.0);
        let transformed = rotation.transform_point(&point);
        
        // Use reasonable floating point tolerance for trigonometric operations
        assert!((transformed.x - 0.0).abs() < 1e-6);
        assert!((transformed.y - 1.0).abs() < 1e-6);
        assert!((transformed.z - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_multiplication() {
        let translate = Xform::translation(1.0, 2.0, 3.0);
        let scale = Xform::scaling(2.0, 2.0, 2.0);
        
        // Combine transformations
        let combined = &translate * &scale;
        
        let point = Point::new(1.0, 1.0, 1.0);
        let transformed = combined.transform_point(&point);
        
        // Expected: scale first, then translate
        assert_eq!(transformed.x, 3.0); // 1.0 * 2.0 + 1.0
        assert_eq!(transformed.y, 4.0); // 1.0 * 2.0 + 2.0
        assert_eq!(transformed.z, 5.0); // 1.0 * 2.0 + 3.0
    }

    #[test]
    fn test_mul_assign() {
        let mut transform = Xform::translation(1.0, 2.0, 3.0);
        let scale = Xform::scaling(2.0, 2.0, 2.0);
        
        // Apply scaling to translation in-place
        transform *= scale;
        
        let point = Point::new(1.0, 1.0, 1.0);
        let transformed = transform.transform_point(&point);
        
        // Expected: scale first, then translate
        assert_eq!(transformed.x, 3.0);
        assert_eq!(transformed.y, 4.0);
        assert_eq!(transformed.z, 5.0);
    }

    #[test]
    fn test_inverse() {
        let transform = Xform::translation(2.0, 3.0, 4.0);
        let inverse = transform.inverse().unwrap();
        
        let point = Point::new(5.0, 6.0, 7.0);
        let transformed = transform.transform_point(&point);
        let back = inverse.transform_point(&transformed);
        
        assert!((back.x - point.x).abs() < 1e-10);
        assert!((back.y - point.y).abs() < 1e-10);
        assert!((back.z - point.z).abs() < 1e-10);
    }

    #[test]
    fn test_change_basis() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let x_axis = Vector::unit_x();
        let y_axis = Vector::unit_y();
        let z_axis = Vector::unit_z();
        
        let basis = Xform::change_basis(&origin, &x_axis, &y_axis, &z_axis);
        
        // The basis transform places the origin at (1,2,3) with standard orientation
        let point = Point::new(0.0, 0.0, 0.0);
        let transformed = basis.transform_point(&point);
        
        assert_eq!(transformed.x, 1.0);
        assert_eq!(transformed.y, 2.0);
        assert_eq!(transformed.z, 3.0);
    }
}

// Custom Serialize implementation for simple format compatible with wink
impl Serialize for Xform {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Xform", 1)?;
        state.serialize_field("m", &self.m)?;
        state.end()
    }
}

