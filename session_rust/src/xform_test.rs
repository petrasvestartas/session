#[cfg(test)]
mod xform_tests {
    use crate::{Point, Vector, Xform};

    fn approx_f32(a: f32, b: f32) -> bool {
        (a as f64 - b as f64).abs() < 1e-5
    }

    fn matrices_close(a: &Xform, b: &Xform) -> bool {
        for i in 0..16 {
            if !approx_f32(a.m[i], b.m[i]) {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_xform_identity() {
        let id = Xform::identity();
        assert!(id.is_identity());
    }

    #[test]
    fn test_xform_default() {
        let def = Xform::default();
        assert!(def.is_identity());
    }

    #[test]
    fn test_xform_identity_transformed_point() {
        let p = Point::new(1.0, 2.0, 3.0);
        let t = Xform::identity().transformed_point(&p);
        assert_eq!((t.x(), t.y(), t.z()), (1.0, 2.0, 3.0));
    }

    #[test]
    fn test_xform_translation_point() {
        let t = Xform::translation(1.0, 2.0, 3.0);
        let p = Point::new(4.0, 5.0, 6.0);
        let tp = t.transformed_point(&p);
        assert_eq!((tp.x(), tp.y(), tp.z()), (5.0, 7.0, 9.0));
    }

    #[test]
    fn test_xform_translation_vector() {
        let t = Xform::translation(1.0, 2.0, 3.0);
        let v = Vector::new(1.0, 2.0, 3.0);
        let tv = t.transformed_vector(&v);
        assert_eq!((tv[0], tv[1], tv[2]), (1.0, 2.0, 3.0));
    }

    #[test]
    fn test_xform_scaling_point() {
        let s = Xform::scaling(2.0, 3.0, 4.0);
        let p = Point::new(1.0, -2.0, 0.5);
        let sp = s.transformed_point(&p);
        assert_eq!((sp.x(), sp.y(), sp.z()), (2.0, -6.0, 2.0));
    }

    #[test]
    fn test_xform_scaling_vector() {
        let s = Xform::scaling(2.0, 3.0, 4.0);
        let v = Vector::new(1.0, -2.0, 0.5);
        let sv = s.transformed_vector(&v);
        assert_eq!((sv[0], sv[1], sv[2]), (2.0, -6.0, 2.0));
    }

    #[test]
    fn test_xform_rotation_z() {
        let r = Xform::rotation_z(std::f32::consts::FRAC_PI_2);
        let p = Point::new(1.0, 0.0, 0.0);
        let rp = r.transformed_point(&p);
        assert!(approx_f32(rp.x(), 0.0));
        assert!(approx_f32(rp.y(), 1.0));
        assert!(approx_f32(rp.z(), 0.0));
    }

    #[test]
    fn test_xform_axis_rotation() {
        let axis = Vector::new(0.0, 0.0, 1.0);
        let r1 = Xform::rotation_z(std::f32::consts::FRAC_PI_2);
        let r2 = Xform::axis_rotation(std::f32::consts::FRAC_PI_2, &axis);
        let p = Point::new(1.0, 0.0, 0.0);
        let p1 = r1.transformed_point(&p);
        let p2 = r2.transformed_point(&p);
        assert!(approx_f32(p1.x(), p2.x()));
        assert!(approx_f32(p1.y(), p2.y()));
        assert!(approx_f32(p1.z(), p2.z()));
    }

    #[test]
    fn test_xform_inverse() {
        let t = &(&Xform::translation(1.0, 2.0, 3.0) * &Xform::rotation_z(0.7))
            * &Xform::scaling(2.0, 2.0, 2.0);
        let inv = t.inverse().unwrap();
        let id = &t * &inv;
        assert!(matrices_close(&id, &Xform::identity()));
    }

    #[test]
    fn test_xform_change_basis_alt_identity() {
        let o0 = Point::new(0.0, 0.0, 0.0);
        let o1 = Point::new(0.0, 0.0, 0.0);
        let x = Vector::new(1.0, 0.0, 0.0);
        let y = Vector::new(0.0, 1.0, 0.0);
        let z = Vector::new(0.0, 0.0, 1.0);
        let cb = Xform::change_basis_alt(&o1, &x, &y, &z, &o0, &x, &y, &z);
        assert!(cb.is_identity());
    }

    #[test]
    fn test_xform_change_basis_alt_translation() {
        let o0 = Point::new(4.0, 5.0, 6.0);
        let o1 = Point::new(1.0, 2.0, 3.0);
        let x = Vector::new(1.0, 0.0, 0.0);
        let y = Vector::new(0.0, 1.0, 0.0);
        let z = Vector::new(0.0, 0.0, 1.0);
        let cb = Xform::change_basis_alt(&o1, &x, &y, &z, &o0, &x, &y, &z);
        let p = Point::new(1.0, 1.0, 1.0);
        let tp = cb.transformed_point(&p);
        assert!(approx_f32(tp.x(), p.x() + 3.0));
        assert!(approx_f32(tp.y(), p.y() + 3.0));
        assert!(approx_f32(tp.z(), p.z() + 3.0));
    }

    #[test]
    fn test_xform_plane_to_plane() {
        let o0 = Point::new(1.0, 2.0, 3.0);
        let o1 = Point::new(-2.0, 0.5, 7.0);
        let x0 = Vector::new(1.0, 0.0, 0.0);
        let y0 = Vector::new(0.0, 1.0, 0.0);
        let z0 = Vector::new(0.0, 0.0, 1.0);
        let x1 = Vector::new(1.0, 0.0, 0.0);
        let y1 = Vector::new(0.0, 1.0, 0.0);
        let z1 = Vector::new(0.0, 0.0, 1.0);
        let m = Xform::plane_to_plane(&o0, &x0, &y0, &z0, &o1, &x1, &y1, &z1);
        let mapped = m.transformed_point(&o0);
        assert!(approx_f32(mapped.x(), o1.x()));
        assert!(approx_f32(mapped.y(), o1.y()));
        assert!(approx_f32(mapped.z(), o1.z()));
    }

    #[test]
    fn test_xform_mul() {
        let a = Xform::translation(1.0, 2.0, 3.0);
        let b = Xform::scaling(2.0, 3.0, 4.0);
        let r_ref = &a * &b;
        let r_owned = &a * &b;
        assert!(matrices_close(&r_ref, &r_owned));
    }

    #[test]
    fn test_xform_mul_assign() {
        let a = Xform::translation(1.0, 2.0, 3.0);
        let b = Xform::scaling(2.0, 3.0, 4.0);
        let mut acc = Xform::identity();
        acc *= a;
        acc *= b;
        let r2 = &Xform::identity()
            * &(&Xform::translation(1.0, 2.0, 3.0) * &Xform::scaling(2.0, 3.0, 4.0));
        assert!(matrices_close(&acc, &r2));
    }

    #[test]
    fn test_xform_json_round_trip() {
        let x = Xform::from_matrix([
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 4.0, 5.0, 6.0, 1.0,
        ]);
        let data = x.to_json_data().expect("Failed to serialize Xform to JSON");
        let y = Xform::from_json_data(&data).expect("Failed to deserialize JSON to Xform");
        assert!(matrices_close(&x, &y));
    }
}
