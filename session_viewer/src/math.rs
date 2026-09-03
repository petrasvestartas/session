//! Small f64/f32 math shared by the app and the engine: the column-major `Mat4`, point
//! transforms, bounds, and the two camera facts recovered from a view-projection.
//!
//! It lives here because both sides need it and neither owns it: `app/scene.rs` places objects
//! with it while `engine/gpu` builds rows and cameras with it. Nothing here touches wgpu, and
//! the only kernel type it names is `Xform`.

use session_rust::Xform;

/// One object's world placement as the 16 raw column-major doubles the GPU row needs.
///
/// NOT a kernel `Xform`: that struct carries `typ`/`name` Strings and a guid `OnceLock`, so
/// `Xform::identity()` heap-allocates TWICE per call and every arena row cost two more on the
/// clone into `objects_base`. On a 90k-line sheet that was ~400k allocations - 300 ms of the
/// walk - to carry 128 bytes of numbers nothing downstream ever reads a name off.
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

/// Grow-and-append one index run. Same shape as the solid arena's own append: the existing
/// prefix is copied GPU-side, never back through wasm memory.
/// Append rows to a growable STORAGE buffer: double the capacity when it runs out, move the
/// prefix GPU-side, and write only the new rows. Returns `true` when the buffer was replaced, so
/// the caller knows to rebuild the bind group pointing at it.
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

pub fn grow_bounds(min: &mut [f32; 3], max: &mut [f32; 3], p: [f32; 3]) {
    for k in 0..3 {
        min[k] = min[k].min(p[k]);
        max[k] = max[k].max(p[k]);
    }
}

/// The camera position, recovered from the combined view-projection alone.
///
/// The eye is the one point that projects to nothing: it is where the clip x, y and w all
/// vanish at once, because every view ray passes through it. Three rows of the matrix, three
/// unknowns, one 3x3 solve - no camera struct needed, so this works for any caller that can
/// produce a view-projection, including the headless harness.
///
/// Orthographic has no eye: rows 0, 1 and 3 are linearly dependent there (w is constant 1),
/// the determinant collapses, and the fallback is the view direction pushed a long way back -
/// which is exactly what an orthographic "eye at infinity" means.
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

/// Ortho half-height in world units (mm), 0.0 in perspective. The w row of the composed
/// matrix says which projection this is: perspective carries the view direction there
/// (magnitude 1), orthographic is all zeros (w is constant 1). Row 1 of the matrix is the
/// y basis scaled by s/h, so 1/|row1.xyz| IS the world half-height - rotation and the
/// anchor (translation lives in column 3) drop out. Left as 0.0, every ink lane falls back
/// to the perspective pen formula with clip.w = 1, which pins pens to a zoom-independent
/// world size: zoom out in ortho and the density taper never fires and far-side ink
/// bleeds through faces.
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
