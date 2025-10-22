use crate::{Point, Vector};
use serde::{ser::Serialize as SerTrait, Deserialize, Serialize};
use std::fmt;
use std::ops::{Index, IndexMut, Mul, MulAssign};
use uuid::Uuid;

/// A 4x4 column-major transformation matrix in 3D space
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Xform")]
pub struct Xform {
    #[serde(rename = "type")]
    pub typ: String,
    pub guid: String,
    pub name: String,
    /// The matrix elements stored in column-major order as a flattened array
    pub m: [f64; 16],
}

impl Xform {
    ///////////////////////////////////////////////////////////////////////////////////////////
    // Basic Constructors
    ///////////////////////////////////////////////////////////////////////////////////////////

    pub fn new() -> Self {
        Self::identity()
    }

    pub fn from_matrix(matrix: [f64; 16]) -> Self {
        Xform {
            typ: "Xform".to_string(),
            guid: Uuid::new_v4().to_string(),
            name: "my_xform".to_string(),
            m: matrix,
        }
    }

    pub fn identity() -> Self {
        let mut xform = Xform {
            typ: "Xform".to_string(),
            guid: Uuid::new_v4().to_string(),
            name: "my_xform".to_string(),
            m: [0.0; 16],
        };
        xform.m[0] = 1.0;
        xform.m[5] = 1.0;
        xform.m[10] = 1.0;
        xform.m[15] = 1.0;
        xform
    }

    pub fn from_cols(col_x: Vector, col_y: Vector, col_z: Vector) -> Self {
        let mut xform = Self::identity();
        xform.m[0] = col_x.x();
        xform.m[1] = col_x.y();
        xform.m[2] = col_x.z();
        xform.m[4] = col_y.x();
        xform.m[5] = col_y.y();
        xform.m[6] = col_y.z();
        xform.m[8] = col_z.x();
        xform.m[9] = col_z.y();
        xform.m[10] = col_z.z();
        xform
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Transformations
    ///////////////////////////////////////////////////////////////////////////////////////////

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        let mut xform = Self::identity();
        xform.m[12] = x;
        xform.m[13] = y;
        xform.m[14] = z;
        xform
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        let mut xform = Self::identity();
        xform.m[0] = x;
        xform.m[5] = y;
        xform.m[10] = z;
        xform
    }

    pub fn rotation_x(angle_radians: f64) -> Self {
        let mut xform = Self::identity();

        let cos_angle = angle_radians.cos();
        let sin_angle = angle_radians.sin();

        xform.m[5] = cos_angle;
        xform.m[6] = sin_angle;
        xform.m[9] = -sin_angle;
        xform.m[10] = cos_angle;

        xform
    }

    pub fn rotation_y(angle_radians: f64) -> Self {
        let mut xform = Self::identity();

        let cos_angle = angle_radians.cos();
        let sin_angle = angle_radians.sin();

        xform.m[0] = cos_angle;
        xform.m[2] = -sin_angle;
        xform.m[8] = sin_angle;
        xform.m[10] = cos_angle;

        xform
    }

    pub fn rotation_z(angle_radians: f64) -> Self {
        let mut xform = Self::identity();
        let cos_angle = angle_radians.cos();
        let sin_angle = angle_radians.sin();

        xform.m[0] = cos_angle;
        xform.m[1] = sin_angle;
        xform.m[4] = -sin_angle;
        xform.m[5] = cos_angle;

        xform
    }

    pub fn rotation(axis: &Vector, angle_radians: f64) -> Self {
        let axis = axis.normalize();

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

    pub fn look_at_rh(eye: &Point, target: &Point, up: &Vector) -> Self {
        let f = (target.clone() - eye.clone()).normalize();
        let s = f.cross(&up.normalize()).normalize();
        let u = s.cross(&f);

        let mut xform = Self::identity();

        xform.m[0] = s.x();
        xform.m[4] = s.y();
        xform.m[8] = s.z();

        xform.m[1] = u.x();
        xform.m[5] = u.y();
        xform.m[9] = u.z();

        xform.m[2] = -f.x();
        xform.m[6] = -f.y();
        xform.m[10] = -f.z();

        xform.m[12] = -s.dot(&Vector::new(eye.x(), eye.y(), eye.z()));
        xform.m[13] = -u.dot(&Vector::new(eye.x(), eye.y(), eye.z()));
        xform.m[14] = f.dot(&Vector::new(eye.x(), eye.y(), eye.z()));

        xform
    }

    pub fn change_basis(origin: &Point, x_axis: &Vector, y_axis: &Vector, z_axis: &Vector) -> Self {
        let x_axis = x_axis.normalize();
        let y_axis = y_axis.normalize();
        let z_axis = z_axis.normalize();

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

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Apply Transformations
    ///////////////////////////////////////////////////////////////////////////////////////////

    pub fn transformed_point(&self, point: &Point) -> Point {
        let m = &self.m;
        let w = m[3] * point.x() + m[7] * point.y() + m[11] * point.z() + m[15];
        let w_inv = if w.abs() > 1e-10 { 1.0 / w } else { 1.0 };

        Point::new(
            (m[0] * point.x() + m[4] * point.y() + m[8] * point.z() + m[12]) * w_inv,
            (m[1] * point.x() + m[5] * point.y() + m[9] * point.z() + m[13]) * w_inv,
            (m[2] * point.x() + m[6] * point.y() + m[10] * point.z() + m[14]) * w_inv,
        )
    }

    pub fn transformed_vector(&self, vector: &Vector) -> Vector {
        let m = &self.m;

        Vector::new(
            m[0] * vector.x() + m[4] * vector.y() + m[8] * vector.z(),
            m[1] * vector.x() + m[5] * vector.y() + m[9] * vector.z(),
            m[2] * vector.x() + m[6] * vector.y() + m[10] * vector.z(),
        )
    }

    pub fn transform_point(&self, point: &mut Point) {
        let m = &self.m;
        let x = point[0];
        let y = point[1];
        let z = point[2];
        let w = m[3] * x + m[7] * y + m[11] * z + m[15];
        let w_inv = if w.abs() > 1e-10 { 1.0 / w } else { 1.0 };

        point[0] = (m[0] * x + m[4] * y + m[8] * z + m[12]) * w_inv;
        point[1] = (m[1] * x + m[5] * y + m[9] * z + m[13]) * w_inv;
        point[2] = (m[2] * x + m[6] * y + m[10] * z + m[14]) * w_inv;
    }

    pub fn transform_vector(&self, vector: &mut Vector) {
        let m = &self.m;
        let x = vector[0];
        let y = vector[1];
        let z = vector[2];

        vector[0] = m[0] * x + m[4] * y + m[8] * z;
        vector[1] = m[1] * x + m[5] * y + m[9] * z;
        vector[2] = m[2] * x + m[6] * y + m[10] * z;
    }

    pub fn x(&self) -> Vector {
        Vector::new(self.m[0], self.m[1], self.m[2])
    }

    pub fn y(&self) -> Vector {
        Vector::new(self.m[4], self.m[5], self.m[6])
    }

    pub fn z(&self) -> Vector {
        Vector::new(self.m[8], self.m[9], self.m[10])
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
        m_xform.m[0] = r[0][3];
        m_xform.m[4] = r[0][4];
        m_xform.m[8] = r[0][5];
        m_xform.m[1] = r[1][3];
        m_xform.m[5] = r[1][4];
        m_xform.m[9] = r[1][5];
        m_xform.m[2] = r[2][3];
        m_xform.m[6] = r[2][4];
        m_xform.m[10] = r[2][5];

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
        x0.normalize_self();
        y0.normalize_self();
        z0.normalize_self();
        x1.normalize_self();
        y1.normalize_self();
        z1.normalize_self();

        let t0 = Self::translation(-origin_0.x(), -origin_0.y(), -origin_0.z());

        let mut f0 = Self::identity();
        f0.m[0] = x0.x();
        f0.m[1] = x0.y();
        f0.m[2] = x0.z();
        f0.m[4] = y0.x();
        f0.m[5] = y0.y();
        f0.m[6] = y0.z();
        f0.m[8] = z0.x();
        f0.m[9] = z0.y();
        f0.m[10] = z0.z();

        let mut f1 = Self::identity();
        f1.m[0] = x1.x();
        f1.m[4] = x1.y();
        f1.m[8] = x1.z();
        f1.m[1] = y1.x();
        f1.m[5] = y1.y();
        f1.m[9] = y1.z();
        f1.m[2] = z1.x();
        f1.m[6] = z1.y();
        f1.m[10] = z1.z();

        let r = &f1 * &f0;
        let t1 = Self::translation(origin_1.x(), origin_1.y(), origin_1.z());
        &t1 * &(&r * &t0)
    }

    pub fn plane_to_xy(origin: &Point, x_axis: &Vector, y_axis: &Vector, z_axis: &Vector) -> Self {
        let mut x = x_axis.clone();
        let mut y = y_axis.clone();
        let mut z = z_axis.clone();
        x.normalize_self();
        y.normalize_self();
        z.normalize_self();

        let t = Self::translation(-origin.x(), -origin.y(), -origin.z());
        let mut f = Self::identity();
        f.m[0] = x.x();
        f.m[1] = x.y();
        f.m[2] = x.z();
        f.m[4] = y.x();
        f.m[5] = y.y();
        f.m[6] = y.z();
        f.m[8] = z.x();
        f.m[9] = z.y();
        f.m[10] = z.z();
        &f * &t
    }

    pub fn xy_to_plane(origin: &Point, x_axis: &Vector, y_axis: &Vector, z_axis: &Vector) -> Self {
        let mut x = x_axis.clone();
        let mut y = y_axis.clone();
        let mut z = z_axis.clone();
        x.normalize_self();
        y.normalize_self();
        z.normalize_self();

        let mut f = Self::identity();
        f.m[0] = x.x();
        f.m[4] = y.x();
        f.m[8] = z.x();
        f.m[1] = x.y();
        f.m[5] = y.y();
        f.m[9] = z.y();
        f.m[2] = x.z();
        f.m[6] = y.z();
        f.m[10] = z.z();

        let t = Self::translation(origin.x(), origin.y(), origin.z());
        &t * &f
    }

    pub fn scale_xyz(scale_x: f64, scale_y: f64, scale_z: f64) -> Self {
        let mut xform = Self::identity();
        xform.m[0] = scale_x;
        xform.m[5] = scale_y;
        xform.m[10] = scale_z;
        xform
    }

    pub fn scale_uniform(origin: &Point, scale_value: f64) -> Self {
        let t0 = Self::translation(-origin.x(), -origin.y(), -origin.z());
        let t1 = Self::scaling(scale_value, scale_value, scale_value);
        let t2 = Self::translation(origin.x(), origin.y(), origin.z());
        &t2 * &(&t1 * &t0)
    }

    pub fn scale_non_uniform(origin: &Point, scale_x: f64, scale_y: f64, scale_z: f64) -> Self {
        let t0 = Self::translation(-origin.x(), -origin.y(), -origin.z());
        let t1 = Self::scale_xyz(scale_x, scale_y, scale_z);
        let t2 = Self::translation(origin.x(), origin.y(), origin.z());
        &t2 * &(&t1 * &t0)
    }

    pub fn axis_rotation(angle: f64, axis: &Vector) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        let ux = axis.x();
        let uy = axis.y();
        let uz = axis.z();
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

    pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        SerTrait::serialize(self, &mut ser)?;
        Ok(String::from_utf8(buf)?)
    }

    pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json_data)?)
    }

    pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.jsondump()?;
        std::fs::write(filepath, json)?;
        Ok(())
    }

    pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(filepath)?;
        Self::jsonload(&json)
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
    type Output = f64;

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
        let mut result = Xform::identity();

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
