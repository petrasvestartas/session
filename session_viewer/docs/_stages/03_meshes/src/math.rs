//! Small f64/f32 math shared by the app and the engine: the column-major `Mat4`, point
//! transforms, the f32 `Aabb` with empty semantics, and the two camera facts recovered from
//! a view-projection. Nothing here touches wgpu; the only kernel type named is `Xform`.

use session_rust::Xform;

/// One object's world placement as 16 raw column-major doubles (index = col * 4 + row).
/// Not a kernel `Xform`: that struct carries Strings and a guid and allocates per copy.
pub type Mat4 = [f64; 16];

/// `a * b` in the kernel's column-major convention. Allocates nothing.
pub fn mat_mul(a: &Mat4, b: &Mat4) -> Mat4 {
    let mut out = [0.0f64; 16];
    for i in 0..4 {
        for j in 0..4 {
            let mut sum = 0.0;
            for k in 0..4 {
                sum += a[k * 4 + i] * b[j * 4 + k];
            }
            out[j * 4 + i] = sum;
        }
    }
    out
}

/// The GPU edge: f64 world math stays CPU-side, the instance row is f32.
pub fn mat_to_f32(m: &Mat4) -> [f32; 16] {
    std::array::from_fn(|i| m[i] as f32)
}

/// A point through an affine matrix, f32 in and out (the arithmetic runs in f64).
pub fn xform_point(m: &Mat4, p: [f32; 3]) -> [f32; 3] {
    let w = xform_point_f64(m, [p[0] as f64, p[1] as f64, p[2] as f64]);
    [w[0] as f32, w[1] as f32, w[2] as f32]
}

/// A point through an affine matrix in f64.
pub fn xform_point_f64(m: &Mat4, p: [f64; 3]) -> [f64; 3] {
    [
        m[0] * p[0] + m[4] * p[1] + m[8] * p[2] + m[12],
        m[1] * p[0] + m[5] * p[1] + m[9] * p[2] + m[13],
        m[2] * p[0] + m[6] * p[1] + m[10] * p[2] + m[14],
    ]
}

/// The camera's vertical field of view, degrees; the pen math and the shaders' push assume it.
pub const FOVY_DEG: f64 = 60.0;

/// `a * b` for two column-major f32 matrices (the record builder folds mvp x model per cloud).
pub fn mat_mul_f32(a: &[f32; 16], b: &[f32; 16]) -> [f32; 16] {
    let mut m = [0.0f32; 16];
    for col in 0..4 {
        for r in 0..4 {
            let mut s = 0.0;
            for k in 0..4 {
                s += a[k * 4 + r] * b[col * 4 + k];
            }
            m[col * 4 + r] = s;
        }
    }
    m
}

/// The length of the matrix's first column: the uniform scale a placement applies.
pub fn mat_scale(m: &[f32; 16]) -> f64 {
    ((m[0] as f64).powi(2) + (m[1] as f64).powi(2) + (m[2] as f64).powi(2)).sqrt()
}

/// An axis-aligned box in f32 with an EMPTY state (min > max), so a scene can start with no
/// box and grow one file at a time.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl Aabb {
    /// The inverted box nothing has grown yet.
    pub fn empty() -> Self {
        Self { min: [f32::INFINITY; 3], max: [f32::NEG_INFINITY; 3] }
    }

    /// True once at least one point went in.
    pub fn is_finite(&self) -> bool {
        self.min.iter().chain(self.max.iter()).all(|v| v.is_finite()) && self.min[0] <= self.max[0]
    }

    /// Widen by one point.
    pub fn grow(&mut self, p: [f32; 3]) {
        for (k, v) in p.iter().enumerate() {
            self.min[k] = self.min[k].min(*v);
            self.max[k] = self.max[k].max(*v);
        }
    }

    /// Widen by another box; an empty box changes nothing.
    pub fn union(&mut self, other: &Aabb) {
        if !other.is_finite() {
            return;
        }
        self.grow(other.min);
        self.grow(other.max);
    }

    /// The box of this box's eight corners through `m` (conservative for rotations).
    pub fn placed(&self, m: &Mat4) -> Aabb {
        let mut out = Aabb::empty();
        if !self.is_finite() {
            return out;
        }
        for c in 0..8u32 {
            let p = [
                if c & 1 == 0 { self.min[0] } else { self.max[0] },
                if c & 2 == 0 { self.min[1] } else { self.max[1] },
                if c & 4 == 0 { self.min[2] } else { self.max[2] },
            ];
            out.grow(xform_point(m, p));
        }
        out
    }

    /// The smallest axis length - a plate's thickness - 0 when empty.
    pub fn thinnest(&self) -> f32 {
        if !self.is_finite() {
            return 0.0;
        }
        (self.max[0] - self.min[0]).min(self.max[1] - self.min[1]).min(self.max[2] - self.min[2])
    }

    /// The diagonal length, 0 when empty.
    pub fn diagonal(&self) -> f32 {
        if !self.is_finite() {
            return 0.0;
        }
        let d = [self.max[0] - self.min[0], self.max[1] - self.min[1], self.max[2] - self.min[2]];
        (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt()
    }

    /// Whether `p` lies inside (closed box).
    pub fn contains(&self, p: [f64; 3]) -> bool {
        (0..3).all(|k| p[k] >= self.min[k] as f64 && p[k] <= self.max[k] as f64)
    }

}

/// The camera position recovered from the combined view-projection alone: the eye is where
/// clip x, y and w vanish at once, so three rows and one 3x3 solve give it. Orthographic has no
/// eye (rows 0, 1, 3 are dependent); the fallback is the view direction pushed far back.
pub fn eye_from_view_proj(vp: &Xform) -> [f32; 3] {
    let a = [vp[(0, 0)], vp[(0, 1)], vp[(0, 2)], vp[(0, 3)]];
    let b = [vp[(1, 0)], vp[(1, 1)], vp[(1, 2)], vp[(1, 3)]];
    let c = [vp[(3, 0)], vp[(3, 1)], vp[(3, 2)], vp[(3, 3)]];
    let rows = [[a[0], a[1], a[2]], [b[0], b[1], b[2]], [c[0], c[1], c[2]]];
    let rhs = [-a[3], -b[3], -c[3]];
    let d = det3(&rows);

    let norm: f64 = rows.iter().map(|r| (r[0] * r[0] + r[1] * r[1] + r[2] * r[2]).sqrt()).product();
    if d.abs() <= 1e-9 * norm.max(1e-30) {
        let f = [vp[(2, 0)], vp[(2, 1)], vp[(2, 2)]];
        let len = (f[0] * f[0] + f[1] * f[1] + f[2] * f[2]).sqrt().max(1e-30);
        return [0, 1, 2].map(|k| (f[k] / len * 1.0e9) as f32);
    }

    let mut eye = [0.0f32; 3];
    for k in 0..3 {
        let mut m = rows;
        for row in 0..3 {
            m[row][k] = rhs[row];
        }
        eye[k] = (det3(&m) / d) as f32;
    }
    eye
}

/// Determinant of a 3x3 given by rows.
fn det3(m: &[[f64; 3]; 3]) -> f64 {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

/// Ortho half-height in world units, 0.0 in perspective. The w row says which projection this
/// is (all zeros = orthographic); row 1 is the y basis scaled by 1/h, so 1/|row1.xyz| is the
/// world half-height, and rotation and the anchor drop out.
pub fn ortho_half_height(vp: &Xform) -> f32 {
    let w2 = vp[(3, 0)].powi(2) + vp[(3, 1)].powi(2) + vp[(3, 2)].powi(2);
    if w2 > 1e-12 {
        return 0.0;
    }
    let r1 = vp[(1, 0)].powi(2) + vp[(1, 1)].powi(2) + vp[(1, 2)].powi(2);
    if r1 <= 1e-30 {
        return 0.0;
    }
    (1.0 / r1.sqrt()) as f32
}
