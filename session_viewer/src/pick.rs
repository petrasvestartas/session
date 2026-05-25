//! Screen-space ray unprojection and CPU-side picking.

/// World-space ray; `direction` is assumed normalized.
#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: [f32; 3],
    pub direction: [f32; 3],
}

/// Pick by world-space ray. Wraps `session.ray_cast` with the local Ray type.
pub fn pick_by_ray(
    session: &mut session_rust::session::Session,
    ray: Ray,
    pick_radius: f32,
) -> Vec<session_rust::session::RayHit> {
    let origin = session_rust::Point::new(ray.origin[0], ray.origin[1], ray.origin[2]);
    let direction = session_rust::Vector::new(ray.direction[0], ray.direction[1], ray.direction[2]);
    session.ray_cast(&origin, &direction, pick_radius)
}

/// Build a world-space ray from a screen-space cursor.
/// `is_ortho`: for symmetric ortho frustums (near=-N, far=+N), ndc_z=0 maps to
/// z_view=far (behind the camera); use ndc_z=0.5 instead (camera eye plane).
pub fn screen_to_world_ray(
    view: &[[f32; 4]; 4],
    proj: &[[f32; 4]; 4],
    viewport: (f32, f32),
    cursor: (f32, f32),
    is_ortho: bool,
) -> Ray {
    let (w, h) = viewport;
    let (cx, cy) = cursor;
    let ndc_x = (cx / w) * 2.0 - 1.0;
    let ndc_y = 1.0 - (cy / h) * 2.0;

    let inv = mat4_inverse(&mat4_mul(proj, view));
    let ndc_z_near = if is_ortho { 0.5 } else { 0.0 };
    // Use ndc_z=0.5 for the far point instead of 1.0.
    // At ndc_z=1, the perspective matrix is nearly singular (A ≈ -1 in f32 when
    // far/near >> 1), making mat4_inverse lose precision.  ndc_z=0.5 keeps the
    // denominator (ndc_z + A) ≈ -0.5, which is well-conditioned.
    let ndc_z_far = if is_ortho { 1.0 } else { 0.5 };
    let p_near = mat4_unproject(&inv, [ndc_x, ndc_y, ndc_z_near]);
    let p_far = mat4_unproject(&inv, [ndc_x, ndc_y, ndc_z_far]);

    let dx = p_far[0] - p_near[0];
    let dy = p_far[1] - p_near[1];
    let dz = p_far[2] - p_near[2];
    let len = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-30);
    Ray {
        origin: p_near,
        direction: [dx / len, dy / len, dz / len],
    }
}

fn mat4_mul(a: &[[f32; 4]; 4], b: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut out = [[0.0; 4]; 4];
    for c in 0..4 {
        for r in 0..4 {
            let mut s = 0.0;
            for k in 0..4 {
                s += a[k][r] * b[c][k];
            }
            out[c][r] = s;
        }
    }
    out
}

fn mat4_unproject(inv_vp: &[[f32; 4]; 4], ndc: [f32; 3]) -> [f32; 3] {
    let x = ndc[0];
    let y = ndc[1];
    let z = ndc[2];
    let w = 1.0;
    let out_x = inv_vp[0][0] * x + inv_vp[1][0] * y + inv_vp[2][0] * z + inv_vp[3][0] * w;
    let out_y = inv_vp[0][1] * x + inv_vp[1][1] * y + inv_vp[2][1] * z + inv_vp[3][1] * w;
    let out_z = inv_vp[0][2] * x + inv_vp[1][2] * y + inv_vp[2][2] * z + inv_vp[3][2] * w;
    let out_w = inv_vp[0][3] * x + inv_vp[1][3] * y + inv_vp[2][3] * z + inv_vp[3][3] * w;
    let inv_w = if out_w.abs() > 1e-30 { 1.0 / out_w } else { 1.0 };
    [out_x * inv_w, out_y * inv_w, out_z * inv_w]
}

fn mat4_inverse(m: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let f = |c: usize, r: usize| m[c][r];

    let m00 = f(0, 0); let m01 = f(1, 0); let m02 = f(2, 0); let m03 = f(3, 0);
    let m10 = f(0, 1); let m11 = f(1, 1); let m12 = f(2, 1); let m13 = f(3, 1);
    let m20 = f(0, 2); let m21 = f(1, 2); let m22 = f(2, 2); let m23 = f(3, 2);
    let m30 = f(0, 3); let m31 = f(1, 3); let m32 = f(2, 3); let m33 = f(3, 3);

    let a2323 = m22 * m33 - m23 * m32;
    let a1323 = m21 * m33 - m23 * m31;
    let a1223 = m21 * m32 - m22 * m31;
    let a0323 = m20 * m33 - m23 * m30;
    let a0223 = m20 * m32 - m22 * m30;
    let a0123 = m20 * m31 - m21 * m30;
    let a2313 = m12 * m33 - m13 * m32;
    let a1313 = m11 * m33 - m13 * m31;
    let a1213 = m11 * m32 - m12 * m31;
    let a2312 = m12 * m23 - m13 * m22;
    let a1312 = m11 * m23 - m13 * m21;
    let a1212 = m11 * m22 - m12 * m21;
    let a0313 = m10 * m33 - m13 * m30;
    let a0213 = m10 * m32 - m12 * m30;
    let a0312 = m10 * m23 - m13 * m20;
    let a0212 = m10 * m22 - m12 * m20;
    let a0113 = m10 * m31 - m11 * m30;
    let a0112 = m10 * m21 - m11 * m20;

    let det = m00 * (m11 * a2323 - m12 * a1323 + m13 * a1223)
        - m01 * (m10 * a2323 - m12 * a0323 + m13 * a0223)
        + m02 * (m10 * a1323 - m11 * a0323 + m13 * a0123)
        - m03 * (m10 * a1223 - m11 * a0223 + m12 * a0123);
    let inv_det = if det.abs() > 1e-30 { 1.0 / det } else { 0.0 };

    let r = |a: f32, b: f32, c: f32, d: f32| (a - b + c - d) * inv_det;

    let mut out = [[0.0; 4]; 4];
    out[0][0] = r(m11 * a2323, m12 * a1323, m13 * a1223, 0.0);
    out[0][1] = r(0.0, m01 * a2323, m02 * a1323, m03 * a1223);
    out[0][2] = r(m01 * a2313, m02 * a1313, m03 * a1213, 0.0);
    out[0][3] = r(0.0, m01 * a2312, m02 * a1312, m03 * a1212);

    out[1][0] = r(0.0, m10 * a2323, m12 * a0323, m13 * a0223);
    out[1][1] = r(m00 * a2323, m02 * a0323, m03 * a0223, 0.0);
    out[1][2] = r(0.0, m00 * a2313, m02 * a0313, m03 * a0213);
    out[1][3] = r(m00 * a2312, m02 * a0312, m03 * a0212, 0.0);

    out[2][0] = r(m10 * a1323, m11 * a0323, m13 * a0123, 0.0);
    out[2][1] = r(0.0, m00 * a1323, m01 * a0323, m03 * a0123);
    out[2][2] = r(m00 * a1313, m01 * a0313, m03 * a0113, 0.0);
    out[2][3] = r(0.0, m00 * a1312, m01 * a0312, m03 * a0112);

    out[3][0] = r(0.0, m10 * a1223, m11 * a0223, m12 * a0123);
    out[3][1] = r(m00 * a1223, m01 * a0223, m02 * a0123, 0.0);
    out[3][2] = r(0.0, m00 * a1213, m01 * a0213, m02 * a0113);
    out[3][3] = r(m00 * a1212, m01 * a0212, m02 * a0112, 0.0);

    let mut t = [[0.0f32; 4]; 4];
    for c in 0..4 {
        for rr in 0..4 {
            t[c][rr] = out[rr][c];
        }
    }
    t
}

#[cfg(test)]
mod pick_tests {
    use super::*;
    use session_rust::xform::Xform;
    use session_rust::{Point, Vector};
    use session_rust::tolerance::Tolerance;

    const MM_TO_UNIT: f32 = 0.001;

    fn build_view_matrix(eye_m: [f32; 3], target_m: [f32; 3], up: [f32; 3]) -> [[f32; 4]; 4] {
        let eye = Point::new(eye_m[0], eye_m[1], eye_m[2]);
        let tgt = Point::new(target_m[0], target_m[1], target_m[2]);
        let up  = Vector::new(up[0], up[1], up[2]);
        let view  = Xform::look_at_right_handed(&eye, &tgt, &up);
        let scale = Xform::scale_xyz(MM_TO_UNIT, MM_TO_UNIT, MM_TO_UNIT);
        (&view * &scale).to_cols()
    }

    fn build_proj_matrix(fov_y: f32, aspect: f32, near: f32, far: f32) -> [[f32; 4]; 4] {
        Xform::perspective(fov_y, aspect, near, far).to_cols()
    }

    fn mat4_mul_point(m: &[[f32; 4]; 4], p: [f32; 4]) -> [f32; 4] {
        let mut out = [0.0f32; 4];
        for r in 0..4 {
            out[r] = m[0][r]*p[0] + m[1][r]*p[1] + m[2][r]*p[2] + m[3][r]*p[3];
        }
        out
    }

    /// Project world_mm through view_proj, return NDC after perspective divide.
    fn project_to_ndc(view: &[[f32; 4]; 4], proj: &[[f32; 4]; 4], world_mm: [f32; 3]) -> [f32; 3] {
        let p4 = [world_mm[0], world_mm[1], world_mm[2], 1.0];
        let view_space = mat4_mul_point(view, p4);
        let clip = mat4_mul_point(proj, view_space);
        let inv_w = if clip[3].abs() > 1e-10 { 1.0 / clip[3] } else { 1.0 };
        [clip[0]*inv_w, clip[1]*inv_w, clip[2]*inv_w]
    }

    /// Convert NDC → screen pixel (inverse of screen_to_world_ray's NDC computation).
    fn ndc_to_pixel(ndc_x: f32, ndc_y: f32, viewport: (f32, f32)) -> (f32, f32) {
        let (w, h) = viewport;
        let cx = (ndc_x + 1.0) / 2.0 * w;
        let cy = (1.0 - ndc_y) / 2.0 * h;
        (cx, cy)
    }

    /// Round-trip: project world point to screen, unproject back, check direction.
    fn ray_roundtrip_ok(
        view: &[[f32; 4]; 4],
        proj: &[[f32; 4]; 4],
        viewport: (f32, f32),
        world_mm: [f32; 3],
        label: &str,
    ) -> bool {
        let ndc = project_to_ndc(view, proj, world_mm);
        // Must be within the frustum to be visible.
        if ndc[0].abs() > 1.0 || ndc[1].abs() > 1.0 {
            eprintln!("{label}: NDC ({:.3}, {:.3}) outside frustum — object not visible from this camera", ndc[0], ndc[1]);
            return false;
        }
        let pixel = ndc_to_pixel(ndc[0], ndc[1], viewport);
        let ray = screen_to_world_ray(view, proj, viewport, pixel, false);
        // Direction from ray origin toward world point.
        let dx = world_mm[0] - ray.origin[0];
        let dy = world_mm[1] - ray.origin[1];
        let dz = world_mm[2] - ray.origin[2];
        let len = (dx*dx + dy*dy + dz*dz).sqrt().max(1e-6);
        let expected_dir = [dx/len, dy/len, dz/len];
        let dot = ray.direction[0]*expected_dir[0]
                + ray.direction[1]*expected_dir[1]
                + ray.direction[2]*expected_dir[2];
        if dot < 0.999 {
            eprintln!("{label}: dot={:.6} ray.dir=({:.4},{:.4},{:.4}) expected=({:.4},{:.4},{:.4})",
                dot,
                ray.direction[0], ray.direction[1], ray.direction[2],
                expected_dir[0], expected_dir[1], expected_dir[2]);
            return false;
        }
        true
    }

    /// Camera looking straight from +Z toward origin; object at origin.
    #[test]
    fn test_roundtrip_simple_z_axis() {
        let eye_m  = [0.0f32, 0.0, 10.0];
        let tgt_m  = [0.0f32, 0.0, 0.0];
        let up     = [0.0f32, 1.0, 0.0];
        let view   = build_view_matrix(eye_m, tgt_m, up);
        let proj   = build_proj_matrix(Tolerance::PI / 3.0, 16.0/9.0, 0.01, 100.0);
        let vp     = (1920.0f32, 1080.0);
        // Object at origin in mm
        assert!(ray_roundtrip_ok(&view, &proj, vp, [0.0, 0.0, 0.0], "origin"));
        // Object at [1000, 500, 0] mm
        assert!(ray_roundtrip_ok(&view, &proj, vp, [1000.0, 500.0, 0.0], "side_point"));
    }

    /// Default-ish camera; target = box_blue_center so it's in view.
    #[test]
    fn test_roundtrip_box_blue_centered() {
        let box_blue = [2000.0f32, 2000.0, 1000.0]; // mm
        // Camera target = box_blue center in meters, distance = 3m
        let tgt_m = [2.0f32, 2.0, 1.0];
        // Look from +Y side
        let eye_m  = [2.0f32, 2.0 - 3.0, 1.0]; // directly in front along -Y
        let up     = [0.0f32, 0.0, 1.0];
        let near   = 3.0_f32 * 0.001;
        let view   = build_view_matrix(eye_m, tgt_m, up);
        let proj   = build_proj_matrix(Tolerance::PI / 3.0, 16.0/9.0, near, 100_000.0);
        let vp     = (1920.0f32, 1080.0);
        assert!(ray_roundtrip_ok(&view, &proj, vp, box_blue, "box_blue_center"));
        // Also test a point slightly offset (edge of box)
        assert!(ray_roundtrip_ok(&view, &proj, vp, [1600.0, 2000.0, 1000.0], "box_blue_edge"));
    }

    /// Simulated default view (camera at [1.836,-1.836,1.5] m, target=[0,0,0]).
    /// Objects must be within the frustum — box_blue is typically outside default view.
    #[test]
    fn test_roundtrip_default_camera_near_objects() {
        let eye_m  = [1.836f32, -1.836, 1.5];
        let tgt_m  = [0.0f32, 0.0, 0.0];
        let up     = [-0.354f32, 0.354, 0.866];
        let near   = 3.0_f32 * 0.001;
        let view   = build_view_matrix(eye_m, tgt_m, up);
        let proj   = build_proj_matrix(Tolerance::PI / 3.0, 16.0/9.0, near, 100_000.0);
        let vp     = (1920.0f32, 1080.0);
        // Origin is the camera target — should project near center, definitely in frustum
        assert!(ray_roundtrip_ok(&view, &proj, vp, [0.0, 0.0, 0.0], "origin_default_cam"));
        // Small box at [600, 600, 200] mm
        assert!(ray_roundtrip_ok(&view, &proj, vp, [600.0, 600.0, 200.0], "box_small_default_cam"));
    }

    /// Verify ray origin is in mm (near the camera position in mm).
    #[test]
    fn test_ray_origin_units_are_mm() {
        let eye_m  = [1.836f32, -1.836, 1.5];
        let tgt_m  = [0.0f32, 0.0, 0.0];
        let up     = [-0.354f32, 0.354, 0.866];
        let near   = 3.0_f32 * 0.001;
        let view   = build_view_matrix(eye_m, tgt_m, up);
        let proj   = build_proj_matrix(Tolerance::PI / 3.0, 16.0/9.0, near, 100_000.0);
        let vp     = (1920.0f32, 1080.0);
        // Center pixel
        let ray = screen_to_world_ray(&view, &proj, vp, (960.0, 540.0), false);
        // Ray origin should be ~3mm from camera eye (near plane), so ~1836mm from world origin
        let dist_from_eye_mm = {
            let dx = ray.origin[0] - eye_m[0]*1000.0;
            let dy = ray.origin[1] - eye_m[1]*1000.0;
            let dz = ray.origin[2] - eye_m[2]*1000.0;
            (dx*dx + dy*dy + dz*dz).sqrt()
        };
        assert!(dist_from_eye_mm < 10.0,
            "ray origin should be ~near-plane distance (3mm) from camera eye, got {:.2}mm", dist_from_eye_mm);
    }
}
