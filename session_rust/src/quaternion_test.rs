#[cfg(test)]
mod quaternion_tests {
    use crate::encoders::{json_dump, json_load};
    use crate::{Quaternion, Vector};
    use std::f64::consts::PI;

    fn approx_f32(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-5
    }

    fn vectors_close(a: &Vector, b: &Vector) -> bool {
        approx_f32(a.x(), b.x()) && approx_f32(a.y(), b.y()) && approx_f32(a.z(), b.z())
    }

    #[test]
    fn test_quaternion_identity() {
        let q = Quaternion::identity();
        assert_eq!(q.s, 1.0);
        assert_eq!(q.v.x(), 0.0);
        assert_eq!(q.v.y(), 0.0);
        assert_eq!(q.v.z(), 0.0);
    }

    #[test]
    fn test_quaternion_from_axis_angle_90deg_z() {
        let axis = Vector::new(0.0, 0.0, 1.0);
        let angle = PI / 2.0;
        let q = Quaternion::from_axis_angle(axis, angle);

        assert!(approx_f32(q.s, (PI / 4.0).cos()));
        assert!(approx_f32(q.v.z(), (PI / 4.0).sin()));
    }

    #[test]
    fn test_quaternion_rotate_vector_90deg_z() {
        let axis = Vector::new(0.0, 0.0, 1.0);
        let angle = PI / 2.0;
        let q = Quaternion::from_axis_angle(axis, angle);

        let v = Vector::new(1.0, 0.0, 0.0);
        let rotated = q.rotate_vector(v);

        let expected = Vector::new(0.0, 1.0, 0.0);
        assert!(vectors_close(&rotated, &expected));
    }

    #[test]
    fn test_quaternion_rotate_vector_180deg_z() {
        let axis = Vector::new(0.0, 0.0, 1.0);
        let angle = PI;
        let q = Quaternion::from_axis_angle(axis, angle);

        let v = Vector::new(1.0, 0.0, 0.0);
        let rotated = q.rotate_vector(v);

        let expected = Vector::new(-1.0, 0.0, 0.0);
        assert!(vectors_close(&rotated, &expected));
    }

    #[test]
    fn test_quaternion_normalize() {
        let q = Quaternion::from_sv(2.0, 0.0, 0.0, 0.0);
        let normalized = q.normalize();

        assert!(approx_f32(normalized.magnitude(), 1.0));
        assert!(approx_f32(normalized.s, 1.0));
    }

    #[test]
    fn test_quaternion_multiplication() {
        let q1 = Quaternion::from_axis_angle(Vector::new(0.0, 0.0, 1.0), PI / 2.0);
        let q2 = Quaternion::from_axis_angle(Vector::new(0.0, 0.0, 1.0), PI / 2.0);
        let q_combined = q1 * q2;

        let v = Vector::new(1.0, 0.0, 0.0);
        let rotated = q_combined.rotate_vector(v);

        let expected = Vector::new(-1.0, 0.0, 0.0);
        assert!(vectors_close(&rotated, &expected));
    }

    #[test]
    fn test_quaternion_identity_rotation() {
        let q = Quaternion::identity();
        let v = Vector::new(1.0, 2.0, 3.0);
        let rotated = q.rotate_vector(v.clone());

        assert!(vectors_close(&rotated, &v));
    }

    #[test]
    fn test_quaternion_conjugate() {
        let q = Quaternion::from_sv(0.5, 0.5, 0.5, 0.5);
        let conj = q.conjugate();

        assert_eq!(conj.s, 0.5);
        assert_eq!(conj.v.x(), -0.5);
        assert_eq!(conj.v.y(), -0.5);
        assert_eq!(conj.v.z(), -0.5);
    }

    #[test]
    fn test_quaternion_magnitude() {
        let q = Quaternion::from_sv(1.0, 0.0, 0.0, 0.0);
        assert!(approx_f32(q.magnitude(), 1.0));

        let q2 = Quaternion::from_sv(2.0, 0.0, 0.0, 0.0);
        assert!(approx_f32(q2.magnitude(), 2.0));
    }

    #[test]
    fn test_quaternion_rotate_around_x() {
        let axis = Vector::new(1.0, 0.0, 0.0);
        let angle = PI / 2.0;
        let q = Quaternion::from_axis_angle(axis, angle);

        let v = Vector::new(0.0, 1.0, 0.0);
        let rotated = q.rotate_vector(v);

        let expected = Vector::new(0.0, 0.0, 1.0);
        assert!(vectors_close(&rotated, &expected));
    }

    #[test]
    fn test_quaternion_rotate_around_y() {
        let axis = Vector::new(0.0, 1.0, 0.0);
        let angle = PI / 2.0;
        let q = Quaternion::from_axis_angle(axis, angle);

        let v = Vector::new(0.0, 0.0, 1.0);
        let rotated = q.rotate_vector(v);

        let expected = Vector::new(1.0, 0.0, 0.0);
        assert!(vectors_close(&rotated, &expected));
    }

    #[test]
    fn test_quaternion_to_json_from_json() {
        let axis = Vector::new(0.0, 0.0, 1.0);
        let angle = PI / 4.0;
        let orig = Quaternion::from_axis_angle(axis, angle);

        let filepath = "test_quaternion.json";
        json_dump(&orig, filepath, true).unwrap();
        let loaded = json_load::<Quaternion>(filepath).unwrap();

        assert!(approx_f32(loaded.s, orig.s));
        assert!(vectors_close(&loaded.v, &orig.v));
    }
}
