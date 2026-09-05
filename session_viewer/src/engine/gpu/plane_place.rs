//! Bake the fixed linear placement of physical planes once. The instance's separate
//! translation table still moves their points into the current camera-relative frame.

use crate::math::Mat4;
use super::arena::FacePlane;

/// Cross product in the precision used for the one-time placement calculation.
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// Dot product in the precision used for the one-time placement calculation.
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Match the instance's rounded linear matrix; preserve translation and anchoring separately.
pub fn bake(plane: &mut FacePlane, placement: &Mat4) {
    let matrix = placement.map(|value| f64::from(value as f32));
    let point = plane.point.map(f64::from);
    plane.point = std::array::from_fn(|axis| (matrix[axis] * point[0] + matrix[4 + axis] * point[1] + matrix[8 + axis] * point[2]) as f32);
    let x = [matrix[0], matrix[1], matrix[2]];
    let y = [matrix[4], matrix[5], matrix[6]];
    let z = [matrix[8], matrix[9], matrix[10]];
    let cofactors = [cross(y, z), cross(z, x), cross(x, y)];
    let normal = plane.normal.map(f64::from);
    let transformed = std::array::from_fn(|axis| cofactors[0][axis] * normal[0] + cofactors[1][axis] * normal[1] + cofactors[2][axis] * normal[2]);
    let length = dot(transformed, transformed).sqrt();
    // After normalization only the determinant's sign is needed for inverse-transpose
    // orientation. A singular flattening retains its valid plane without dividing by zero.
    let sign = if dot(x, cofactors[0]) < 0.0 { -1.0 } else { 1.0 };
    plane.normal = if length > 0.0 { transformed.map(|value| (value * sign / length) as f32) } else { [0.0; 3] };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::xform_point_f64;

    /// Scale, shear and reflections retain both the plane and inverse-transpose orientation.
    #[test]
    fn nonuniform_reflected_plane_matches_transformed_tangents() {
        for scale in [-2.0, 2.0] {
            let placement = [scale, 0.25, 0.0, 0.0, 0.5, 3.0, 0.2, 0.0, 0.1, 0.0, 4.0, 0.0, 0.0, 0.0, 0.0, 1.0];
            let mut plane = FacePlane { point: [1.0, 2.0, 3.0], instance_id: 17, normal: [0.0, -1.0, 1.0], _pad: 0 };
            bake(&mut plane, &placement);
            let placed = [[1.0, 2.0, 3.0], [2.0, 2.0, 3.0], [1.0, 3.0, 4.0]].map(|point| xform_point_f64(&placement, point));
            let normal = plane.normal.map(f64::from);
            for point in placed {
                let delta = std::array::from_fn(|axis| point[axis] - f64::from(plane.point[axis]));
                assert!(dot(normal, delta).abs() < 1e-6);
            }
            let a = std::array::from_fn(|axis| placed[1][axis] - placed[0][axis]);
            let b = std::array::from_fn(|axis| placed[2][axis] - placed[0][axis]);
            assert_eq!(dot(normal, cross(a, b)).is_sign_positive(), scale > 0.0);
            assert!((dot(normal, normal) - 1.0).abs() < 1e-7);
            assert_eq!(plane.instance_id, 17);
        }
    }

    /// True translation stays exclusively in the re-anchored instance table, including large sites.
    #[test]
    fn large_translation_is_never_baked() {
        let mut placement = session_rust::Xform::identity().m;
        placement[0] = 2.0;
        placement[5] = 3.0;
        placement[10] = 4.0;
        let mut local = FacePlane { point: [1.0, 2.0, 3.0], instance_id: 5, normal: [0.0, 0.0, 1.0], _pad: 0 };
        let mut shifted = local;
        bake(&mut local, &placement);
        placement[12..15].copy_from_slice(&[1e12, -1e12, 1e12]);
        bake(&mut shifted, &placement);
        assert_eq!(local.point, [2.0, 6.0, 12.0]);
        assert_eq!(shifted.point, local.point);
        assert_eq!(shifted.normal, local.normal);
    }

    /// A singular flattening keeps a surviving face and marks a collapsed side as degenerate.
    #[test]
    fn singular_flattening_preserves_its_surviving_plane() {
        let mut placement = session_rust::Xform::identity().m;
        placement[10] = 0.0;
        let mut plane = FacePlane { point: [1.0, 2.0, 3.0], instance_id: 0, normal: [0.0, 0.0, 1.0], _pad: 0 };
        bake(&mut plane, &placement);
        assert_eq!(plane.point, [1.0, 2.0, 0.0]);
        assert_eq!(plane.normal, [0.0, 0.0, 1.0]);
        plane.normal = [1.0, 0.0, 0.0];
        bake(&mut plane, &placement);
        assert_eq!(plane.normal, [0.0; 3]);
    }
}
