use crate::{Point, Vector};
use serde::{ser::Serialize as SerTrait, Deserialize, Serialize};
use std::fmt;
use std::ops::{Index, IndexMut, Mul, MulAssign};

/// A 4x4 column-major transformation matrix in 3D space
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Xform")]
pub struct Xform {
    /// The matrix elements stored in column-major order as a flattened array
    pub m: [f32; 16],
}

impl Xform {
    ///////////////////////////////////////////////////////////////////////////////////////////
    // Basic Constructors
    ///////////////////////////////////////////////////////////////////////////////////////////

    pub fn new() -> Self {
        Self::identity()
    }

    pub fn from_matrix(matrix: [f32; 16]) -> Self {
        Xform { m: matrix }
    }

    pub fn identity() -> Self {
        let mut xform = Xform { m: [0.0; 16] };
        xform.m[0] = 1.0;
        xform.m[5] = 1.0;
        xform.m[10] = 1.0;
        xform.m[15] = 1.0;
        xform
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Basic Transformations
    ///////////////////////////////////////////////////////////////////////////////////////////

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        let mut xform = Self::identity();
        xform.m[12] = x;
        xform.m[13] = y;
        xform.m[14] = z;
        xform
    }

    pub fn scaling(x: f32, y: f32, z: f32) -> Self {
        let mut xform = Self::identity();
        xform.m[0] = x;
        xform.m[5] = y;
        xform.m[10] = z;
        xform
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Rotations
    ///////////////////////////////////////////////////////////////////////////////////////////

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

    pub fn rotation(axis: &Vector, angle_radians: f32) -> Self {
        let axis = axis.unitized();

        let mut xform = Self::identity();
        let cos_angle = angle_radians.cos();
        let sin_angle = angle_radians.sin();
        let one_minus_cos = 1.0 - cos_angle;

        let xx = axis.x() * axis.x();
        let xy = axis.x() * axis.y();
        let xz = axis.x() * axis.z();
        let yy = axis.y() * axis.y();
        let yz = axis.y() * axis.z();
        let zz = axis.z() * axis.z();

        xform.m[0] = cos_angle + xx * one_minus_cos;
        xform.m[1] = xy * one_minus_cos + axis.z() * sin_angle;
        xform.m[2] = xz * one_minus_cos - axis.y() * sin_angle;

        xform.m[4] = xy * one_minus_cos - axis.z() * sin_angle;
        xform.m[5] = cos_angle + yy * one_minus_cos;
        xform.m[6] = yz * one_minus_cos + axis.x() * sin_angle;

        xform.m[8] = xz * one_minus_cos + axis.y() * sin_angle;
        xform.m[9] = yz * one_minus_cos - axis.x() * sin_angle;
        xform.m[10] = cos_angle + zz * one_minus_cos;

        xform
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Advanced Transformations
    ///////////////////////////////////////////////////////////////////////////////////////////

    pub fn change_basis(origin: &Point, x_axis: &Vector, y_axis: &Vector, z_axis: &Vector) -> Self {
        let x_axis = x_axis.unitized();
        let y_axis = y_axis.unitized();
        let z_axis = z_axis.unitized();

        let mut xform = Self::identity();

        xform.m[0] = x_axis.x();
        xform.m[1] = x_axis.y();
        xform.m[2] = x_axis.z();

        xform.m[4] = y_axis.x();
        xform.m[5] = y_axis.y();
        xform.m[6] = y_axis.z();

        xform.m[8] = z_axis.x();
        xform.m[9] = z_axis.y();
        xform.m[10] = z_axis.z();

        // Set the origin
        xform.m[12] = origin.x();
        xform.m[13] = origin.y();
        xform.m[14] = origin.z();

        xform
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Matrix Operations
    ///////////////////////////////////////////////////////////////////////////////////////////

    pub fn inverse(&self) -> Option<Xform> {
        let a00 = self[(0, 0)];
        let a01 = self[(0, 1)];
        let a02 = self[(0, 2)];
        let a10 = self[(1, 0)];
        let a11 = self[(1, 1)];
        let a12 = self[(1, 2)];
        let a20 = self[(2, 0)];
        let a21 = self[(2, 1)];
        let a22 = self[(2, 2)];

        let det = a00 * (a11 * a22 - a12 * a21) - a01 * (a10 * a22 - a12 * a20)
            + a02 * (a10 * a21 - a11 * a20);
        if det.abs() < 1e-12 {
            return None;
        }
        let inv_det = 1.0 / det;

        let m00 = (a11 * a22 - a12 * a21) * inv_det;
        let m01 = (a02 * a21 - a01 * a22) * inv_det;
        let m02 = (a01 * a12 - a02 * a11) * inv_det;
        let m10 = (a12 * a20 - a10 * a22) * inv_det;
        let m11 = (a00 * a22 - a02 * a20) * inv_det;
        let m12 = (a02 * a10 - a00 * a12) * inv_det;
        let m20 = (a10 * a21 - a11 * a20) * inv_det;
        let m21 = (a01 * a20 - a00 * a21) * inv_det;
        let m22 = (a00 * a11 - a01 * a10) * inv_det;

        let tx = self[(0, 3)];
        let ty = self[(1, 3)];
        let tz = self[(2, 3)];
        let itx = -(m00 * tx + m01 * ty + m02 * tz);
        let ity = -(m10 * tx + m11 * ty + m12 * tz);
        let itz = -(m20 * tx + m21 * ty + m22 * tz);

        let mut res = Xform::identity();
        res[(0, 0)] = m00;
        res[(0, 1)] = m01;
        res[(0, 2)] = m02;
        res[(1, 0)] = m10;
        res[(1, 1)] = m11;
        res[(1, 2)] = m12;
        res[(2, 0)] = m20;
        res[(2, 1)] = m21;
        res[(2, 2)] = m22;
        res[(0, 3)] = itx;
        res[(1, 3)] = ity;
        res[(2, 3)] = itz;
        Some(res)
    }

    pub fn transform_point(&self, point: &Point) -> Point {
        let m = &self.m;
        let w = m[3] * point.x() + m[7] * point.y() + m[11] * point.z() + m[15];
        let w_inv = if w.abs() > 1e-10 { 1.0 / w } else { 1.0 };

        Point::new(
            (m[0] * point.x() + m[4] * point.y() + m[8] * point.z() + m[12]) * w_inv,
            (m[1] * point.x() + m[5] * point.y() + m[9] * point.z() + m[13]) * w_inv,
            (m[2] * point.x() + m[6] * point.y() + m[10] * point.z() + m[14]) * w_inv,
        )
    }

    pub fn transform_vector(&self, vector: &Vector) -> Vector {
        let m = &self.m;

        Vector::new(
            m[0] * vector.x() + m[4] * vector.y() + m[8] * vector.z(),
            m[1] * vector.x() + m[5] * vector.y() + m[9] * vector.z(),
            m[2] * vector.x() + m[6] * vector.y() + m[10] * vector.z(),
        )
    }

    pub fn is_identity(&self) -> bool {
        let identity = Xform::identity();
        for i in 0..16 {
            if (self.m[i] - identity.m[i]).abs() > 1e-10 {
                return false;
            }
        }
        true
    }

    #[allow(clippy::too_many_arguments)]
    pub fn change_basis_alt(
        origin_1: &Point,
        x_axis_1: &Vector,
        y_axis_1: &Vector,
        z_axis_1: &Vector,
        origin_0: &Point,
        x_axis_0: &Vector,
        y_axis_0: &Vector,
        z_axis_0: &Vector,
    ) -> Self {
        let a = x_axis_1.dot(y_axis_1);
        let b = x_axis_1.dot(z_axis_1);
        let c = y_axis_1.dot(z_axis_1);

        let mut r = [
            [
                x_axis_1.dot(x_axis_1),
                a,
                b,
                x_axis_1.dot(x_axis_0),
                x_axis_1.dot(y_axis_0),
                x_axis_1.dot(z_axis_0),
            ],
            [
                a,
                y_axis_1.dot(y_axis_1),
                c,
                y_axis_1.dot(x_axis_0),
                y_axis_1.dot(y_axis_0),
                y_axis_1.dot(z_axis_0),
            ],
            [
                b,
                c,
                z_axis_1.dot(z_axis_1),
                z_axis_1.dot(x_axis_0),
                z_axis_1.dot(y_axis_0),
                z_axis_1.dot(z_axis_0),
            ],
        ];

        let mut i0 = if r[0][0] >= r[1][1] { 0 } else { 1 };
        if r[2][2] > r[i0][i0] {
            i0 = 2;
        }
        let i1 = (i0 + 1) % 3;
        let i2 = (i1 + 1) % 3;

        if r[i0][i0] == 0.0 {
            return Self::identity();
        }

        let d = 1.0 / r[i0][i0];
        for j in 0..6 {
            r[i0][j] *= d;
        }
        r[i0][i0] = 1.0;

        if r[i1][i0] != 0.0 {
            let d = -r[i1][i0];
            for j in 0..6 {
                r[i1][j] += d * r[i0][j];
            }
            r[i1][i0] = 0.0;
        }
        if r[i2][i0] != 0.0 {
            let d = -r[i2][i0];
            for j in 0..6 {
                r[i2][j] += d * r[i0][j];
            }
            r[i2][i0] = 0.0;
        }

        let (i1, i2) = if r[i1][i1].abs() < r[i2][i2].abs() {
            (i2, i1)
        } else {
            (i1, i2)
        };
        if r[i1][i1] == 0.0 {
            return Self::identity();
        }

        let d = 1.0 / r[i1][i1];
        for j in 0..6 {
            r[i1][j] *= d;
        }
        r[i1][i1] = 1.0;

        if r[i0][i1] != 0.0 {
            let d = -r[i0][i1];
            for j in 0..6 {
                r[i0][j] += d * r[i1][j];
            }
            r[i0][i1] = 0.0;
        }
        if r[i2][i1] != 0.0 {
            let d = -r[i2][i1];
            for j in 0..6 {
                r[i2][j] += d * r[i1][j];
            }
            r[i2][i1] = 0.0;
        }

        if r[i2][i2] == 0.0 {
            return Self::identity();
        }

        let d = 1.0 / r[i2][i2];
        for j in 0..6 {
            r[i2][j] *= d;
        }
        r[i2][i2] = 1.0;

        if r[i0][i2] != 0.0 {
            let d = -r[i0][i2];
            for j in 0..6 {
                r[i0][j] += d * r[i2][j];
            }
            r[i0][i2] = 0.0;
        }
        if r[i1][i2] != 0.0 {
            let d = -r[i1][i2];
            for j in 0..6 {
                r[i1][j] += d * r[i2][j];
            }
            r[i1][i2] = 0.0;
        }

        let mut m_xform = Self::identity();
        m_xform.m[0] = r[0][3] as f32;
        m_xform.m[4] = r[0][4] as f32;
        m_xform.m[8] = r[0][5] as f32;
        m_xform.m[1] = r[1][3] as f32;
        m_xform.m[5] = r[1][4] as f32;
        m_xform.m[9] = r[1][5] as f32;
        m_xform.m[2] = r[2][3] as f32;
        m_xform.m[6] = r[2][4] as f32;
        m_xform.m[10] = r[2][5] as f32;

        let t0 = Self::translation(-origin_1.x(), -origin_1.y(), -origin_1.z());
        let t2 = Self::translation(origin_0.x(), origin_0.y(), origin_0.z());
        &t2 * &(&m_xform * &t0)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn plane_to_plane(
        origin_0: &Point,
        x_axis_0: &Vector,
        y_axis_0: &Vector,
        z_axis_0: &Vector,
        origin_1: &Point,
        x_axis_1: &Vector,
        y_axis_1: &Vector,
        z_axis_1: &Vector,
    ) -> Self {
        let mut x0 = x_axis_0.clone();
        let mut y0 = y_axis_0.clone();
        let mut z0 = z_axis_0.clone();
        let mut x1 = x_axis_1.clone();
        let mut y1 = y_axis_1.clone();
        let mut z1 = z_axis_1.clone();
        x0.unitize();
        y0.unitize();
        z0.unitize();
        x1.unitize();
        y1.unitize();
        z1.unitize();

        let t0 = Self::translation(-origin_0.x(), -origin_0.y(), -origin_0.z());

        let mut f0 = Self::identity();
        f0.m[0] = x0.x() as f32;
        f0.m[1] = x0.y() as f32;
        f0.m[2] = x0.z() as f32;
        f0.m[4] = y0.x() as f32;
        f0.m[5] = y0.y() as f32;
        f0.m[6] = y0.z() as f32;
        f0.m[8] = z0.x() as f32;
        f0.m[9] = z0.y() as f32;
        f0.m[10] = z0.z() as f32;

        let mut f1 = Self::identity();
        f1.m[0] = x1.x() as f32;
        f1.m[4] = x1.y() as f32;
        f1.m[8] = x1.z() as f32;
        f1.m[1] = y1.x() as f32;
        f1.m[5] = y1.y() as f32;
        f1.m[9] = y1.z() as f32;
        f1.m[2] = z1.x() as f32;
        f1.m[6] = z1.y() as f32;
        f1.m[10] = z1.z() as f32;

        let r = &f1 * &f0;
        let t1 = Self::translation(origin_1.x(), origin_1.y(), origin_1.z());
        &t1 * &(&r * &t0)
    }

    pub fn plane_to_xy(origin: &Point, x_axis: &Vector, y_axis: &Vector, z_axis: &Vector) -> Self {
        let mut x = x_axis.clone();
        let mut y = y_axis.clone();
        let mut z = z_axis.clone();
        x.unitize();
        y.unitize();
        z.unitize();

        let t = Self::translation(-origin.x(), -origin.y(), -origin.z());
        let mut f = Self::identity();
        f.m[0] = x.x() as f32;
        f.m[1] = x.y() as f32;
        f.m[2] = x.z() as f32;
        f.m[4] = y.x() as f32;
        f.m[5] = y.y() as f32;
        f.m[6] = y.z() as f32;
        f.m[8] = z.x() as f32;
        f.m[9] = z.y() as f32;
        f.m[10] = z.z() as f32;
        &f * &t
    }

    pub fn xy_to_plane(origin: &Point, x_axis: &Vector, y_axis: &Vector, z_axis: &Vector) -> Self {
        let mut x = x_axis.clone();
        let mut y = y_axis.clone();
        let mut z = z_axis.clone();
        x.unitize();
        y.unitize();
        z.unitize();

        let mut f = Self::identity();
        f.m[0] = x.x() as f32;
        f.m[4] = y.x() as f32;
        f.m[8] = z.x() as f32;
        f.m[1] = x.y() as f32;
        f.m[5] = y.y() as f32;
        f.m[9] = z.y() as f32;
        f.m[2] = x.z() as f32;
        f.m[6] = y.z() as f32;
        f.m[10] = z.z() as f32;

        let t = Self::translation(origin.x(), origin.y(), origin.z());
        &t * &f
    }

    pub fn scale_xyz(scale_x: f32, scale_y: f32, scale_z: f32) -> Self {
        let mut xform = Self::identity();
        xform.m[0] = scale_x;
        xform.m[5] = scale_y;
        xform.m[10] = scale_z;
        xform
    }

    pub fn scale_uniform(origin: &Point, scale_value: f32) -> Self {
        let t0 = Self::translation(-origin.x(), -origin.y(), -origin.z());
        let t1 = Self::scaling(scale_value, scale_value, scale_value);
        let t2 = Self::translation(origin.x(), origin.y(), origin.z());
        &t2 * &(&t1 * &t0)
    }

    pub fn scale_non_uniform(origin: &Point, scale_x: f32, scale_y: f32, scale_z: f32) -> Self {
        let t0 = Self::translation(-origin.x(), -origin.y(), -origin.z());
        let t1 = Self::scale_xyz(scale_x, scale_y, scale_z);
        let t2 = Self::translation(origin.x(), origin.y(), origin.z());
        &t2 * &(&t1 * &t0)
    }

    pub fn axis_rotation(angle: f32, axis: &Vector) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        let ux = axis.x() as f32;
        let uy = axis.y() as f32;
        let uz = axis.z() as f32;
        let t = 1.0 - c;

        let mut xform = Self::identity();
        xform.m[0] = t * ux * ux + c;
        xform.m[4] = t * ux * uy - uz * s;
        xform.m[8] = t * ux * uz + uy * s;

        xform.m[1] = t * ux * uy + uz * s;
        xform.m[5] = t * uy * uy + c;
        xform.m[9] = t * uy * uz - ux * s;

        xform.m[2] = t * ux * uz - uy * s;
        xform.m[6] = t * uy * uz + ux * s;
        xform.m[10] = t * uz * uz + c;

        xform
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    pub fn to_json_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        SerTrait::serialize(self, &mut ser)?;
        Ok(String::from_utf8(buf)?)
    }

    pub fn from_json_data(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json_data)?)
    }

    pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.to_json_data()?;
        std::fs::write(filepath, json)?;
        Ok(())
    }

    pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(filepath)?;
        Self::from_json_data(&json)
    }
}

// Implement Display for Xform
impl fmt::Display for Xform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Transform Matrix:")?;
        writeln!(
            f,
            "[{:.4}, {:.4}, {:.4}, {:.4}]",
            self.m[0], self.m[4], self.m[8], self.m[12]
        )?;
        writeln!(
            f,
            "[{:.4}, {:.4}, {:.4}, {:.4}]",
            self.m[1], self.m[5], self.m[9], self.m[13]
        )?;
        writeln!(
            f,
            "[{:.4}, {:.4}, {:.4}, {:.4}]",
            self.m[2], self.m[6], self.m[10], self.m[14]
        )?;
        write!(
            f,
            "[{:.4}, {:.4}, {:.4}, {:.4}]",
            self.m[3], self.m[7], self.m[11], self.m[15]
        )
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
        assert!(row < 4 && col < 4, "Index out of bounds: ({row}, {col})");
        // Column-major order: index = col * 4 + row
        &self.m[col * 4 + row]
    }
}

// Implement IndexMut trait for modifying matrix elements with [(row, col)] syntax
impl IndexMut<(usize, usize)> for Xform {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        let (row, col) = idx;
        assert!(row < 4 && col < 4, "Index out of bounds: ({row}, {col})");
        // Column-major order: index = col * 4 + row
        &mut self.m[col * 4 + row]
    }
}

// Implement Mul for matrix multiplication: Xform * Xform = Xform
impl Mul for &Xform {
    type Output = Xform;

    fn mul(self, rhs: &Xform) -> Self::Output {
        let mut result = Xform { m: [0.0; 16] };

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
#[path = "xform_test.rs"]
mod tests;
