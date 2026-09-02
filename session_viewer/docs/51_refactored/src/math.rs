//! Small f64/f32 math shared by the app and the engine: the column-major `Mat4`, point
//! transforms, the f32 `Aabb`, and the two camera facts recovered from a view-projection.
//! Nothing here touches wgpu or a kernel type beyond `Xform`.

use session_rust::Xform;

/// One object's world placement as the 16 raw column-major doubles the GPU row needs.
/// NOT a kernel `Xform`: that one heap-allocates twice per construction (name + guid), which
/// measured as 300 ms of a 90k-line sheet's walk for numbers nothing downstream names.
pub type Mat4 = [f64; 16];

/// `a * b` in the kernel's convention: column-major, index = col * 4 + row.
/// Matches `impl Mul for &Xform` element for element - and allocates nothing.
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

/// A local point through a column-major placement, f64 inside, f32 at the edges.
pub fn xform_point(m: &Mat4, p: [f32; 3]) -> [f32; 3] {
    let x = p[0] as f64;
    let y = p[1] as f64;
    let z = p[2] as f64;
    [
        (m[0] * x + m[4] * y + m[8] * z + m[12]) as f32,
        (m[1] * x + m[5] * y + m[9] * z + m[13]) as f32,
        (m[2] * x + m[6] * y + m[10] * z + m[14]) as f32,
    ]
}

/// The same placement in f64 end to end - the world AABB corners that FLAG_INSIDE tests.
pub fn xform_point_f64(m: &Mat4, p: [f64; 3]) -> [f64; 3] {
    let [x, y, z] = p;
    [
        m[0] * x + m[4] * y + m[8] * z + m[12],
        m[1] * x + m[5] * y + m[9] * z + m[13],
        m[2] * x + m[6] * y + m[10] * z + m[14],
    ]
}

/// Widen a min/max pair to hold `p`.
pub fn grow_bounds(min: &mut [f32; 3], max: &mut [f32; 3], p: [f32; 3]) {
    for k in 0..3 {
        min[k] = min[k].min(p[k]);
        max[k] = max[k].max(p[k]);
    }
}

/// An axis-aligned box in f32 world units. `empty()` is inverted, so the first `grow` sets it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl Aabb {
    /// The inverted box: every `grow` or `union` replaces it.
    pub fn empty() -> Self {
        Self { min: [f32::INFINITY; 3], max: [f32::NEG_INFINITY; 3] }
    }

    /// Widen the box to hold `p`.
    pub fn grow(&mut self, p: [f32; 3]) {
        grow_bounds(&mut self.min, &mut self.max, p);
    }

    /// Widen the box to hold `other`; an empty `other` changes nothing.
    pub fn union(&mut self, other: &Aabb) {
        for k in 0..3 {
            self.min[k] = self.min[k].min(other.min[k]);
            self.max[k] = self.max[k].max(other.max[k]);
        }
    }

    /// False for the empty box: nothing has been grown into it.
    pub fn is_finite(&self) -> bool {
        self.min.iter().chain(&self.max).all(|v| v.is_finite())
    }

    /// The box through a placement: the eight corners transformed, then re-boxed.
    pub fn placed(&self, m: &Mat4) -> Aabb {
        let mut world = Aabb::empty();
        for c in 0..8u32 {
            let corner = [
                if c & 1 == 0 { self.min[0] } else { self.max[0] },
                if c & 2 == 0 { self.min[1] } else { self.max[1] },
                if c & 4 == 0 { self.min[2] } else { self.max[2] },
            ];
            world.grow(xform_point(m, corner));
        }
        world
    }
}

/// The camera position, recovered from the view-projection alone: the eye is where clip x, y
/// and w all vanish, so rows 0, 1, 3 give a 3x3 solve. Orthographic has no eye (those rows are
/// dependent), so the fallback is the view direction pushed 1e9 back - an eye at infinity.
pub fn eye_from_view_proj(vp: &Xform) -> [f32; 3] {
    let r = |i: usize| [vp[(i, 0)], vp[(i, 1)], vp[(i, 2)], vp[(i, 3)]];
    let (a, b, c) = (r(0), r(1), r(3));

    // Cramer on [a b c] . p = -[a3 b3 c3]
    let det3 = |m: [[f64; 3]; 3]| {
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    };
    let rows = [[a[0], a[1], a[2]], [b[0], b[1], b[2]], [c[0], c[1], c[2]]];
    let rhs = [-a[3], -b[3], -c[3]];
    let d = det3(rows);

    // Scale-free singularity test: compare against the product of the row magnitudes, so it
    // fires on genuine dependence rather than on a scene whose units make everything small.
    let norm: f64 = rows.iter().map(|r| (r[0] * r[0] + r[1] * r[1] + r[2] * r[2]).sqrt()).product();
    if d.abs() <= 1e-9 * norm.max(1e-30) {
        // Orthographic: row 3 carries no direction, so take the view axis from row 2 (depth)
        // and stand a long way back along it.
        let f = [vp[(2, 0)], vp[(2, 1)], vp[(2, 2)]];
        let len = (f[0] * f[0] + f[1] * f[1] + f[2] * f[2]).sqrt().max(1e-30);
        return [0, 1, 2].map(|k| (f[k] / len * 1.0e9) as f32);
    }

    [0, 1, 2].map(|k| {
        let mut m = rows;
        for row in 0..3 {
            m[row][k] = rhs[row];
        }
        (det3(m) / d) as f32
    })
}

/// Ortho half-height in world units (mm), 0.0 in perspective. The w row tells the projection
/// apart (ortho: all zeros); row 1 is the y basis scaled by s/h, so 1/|row1.xyz| is the
/// half-height. Left at 0.0 in ortho, every pen pins to a zoom-independent world size.
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
