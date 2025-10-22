#[cfg(test)]
mod vector_tests {
    use crate::encoders::{json_dump, json_load};
    use crate::Vector;

    #[test]
    fn test_vector_constructor() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!((v.x(), v.y(), v.z()), (1.0, 2.0, 3.0));
        assert!(!v.guid.is_empty());
    }

    #[test]
    fn test_vector_equality() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let mut v2 = Vector::new(1.0, 2.0, 3.0);
        v2.guid = v1.guid.clone();
        assert_eq!(v1, v2);
        let v3 = Vector::new(1.1, 2.0, 3.0);
        assert_ne!(v1, v3);
    }

    #[test]
    fn test_vector_to_json_data() {
        let v = Vector::new(15.5, 25.7, 35.9);
        let data = v.jsondump().unwrap();
        assert!(data.contains("Vector") && data.contains("15.5"));
    }

    #[test]
    fn test_vector_from_json_data() {
        let orig = Vector::new(42.1, 84.2, 126.3);
        let rest = Vector::jsonload(&orig.jsondump().unwrap()).unwrap();
        assert_eq!((rest.x(), rest.y(), rest.z()), (42.1, 84.2, 126.3));
    }

    #[test]
    fn test_vector_to_json_from_json() {
        let orig = Vector::new(123.45, 678.90, 999.11);
        let filename = "test_vector.json";
        json_dump(&orig, filename, true).unwrap();
        let load = json_load::<Vector>(filename).unwrap();
        assert_eq!(
            (load.x(), load.y(), load.z()),
            (orig.x(), orig.y(), orig.z())
        );
    }

    #[test]
    fn test_vector_default_constructor() {
        let v = Vector::default();
        assert_eq!((v[0], v[1], v[2]), (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_vector_constructor_values() {
        let v = Vector::new(0.57, -158.63, 180.890);
        assert_eq!((v[0], v[1], v[2]), (0.57, -158.63, 180.890));
    }

    #[test]
    fn test_vector_static_methods() {
        assert_eq!(
            (
                Vector::x_axis()[0],
                Vector::x_axis()[1],
                Vector::x_axis()[2]
            ),
            (1.0, 0.0, 0.0)
        );
        assert_eq!(
            (
                Vector::y_axis()[0],
                Vector::y_axis()[1],
                Vector::y_axis()[2]
            ),
            (0.0, 1.0, 0.0)
        );
        assert_eq!(
            (
                Vector::z_axis()[0],
                Vector::z_axis()[1],
                Vector::z_axis()[2]
            ),
            (0.0, 0.0, 1.0)
        );
    }

    #[test]
    fn test_vector_from_start_and_end() {
        let v =
            Vector::from_start_and_end(&Vector::new(8.7, 5.7, -1.87), &Vector::new(1.0, 1.57, 2.0));
        assert!((v[0] + 7.7).abs() < 1e-5);
        assert!((v[1] + 4.13).abs() < 1e-5);
        assert!((v[2] - 3.87).abs() < 1e-5);
    }

    #[test]
    fn test_vector_operators() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        let v3 = &v1 + &v2;
        assert_eq!((v3[0], v3[1], v3[2]), (5.0, 7.0, 9.0));
        let v3 = &v1 - &v2;
        assert_eq!((v3[0], v3[1], v3[2]), (-3.0, -3.0, -3.0));
        let v3 = &v1 * 2.0;
        assert_eq!((v3[0], v3[1], v3[2]), (2.0, 4.0, 6.0));
        let v3 = &v1 / 2.0;
        assert_eq!((v3[0], v3[1], v3[2]), (0.5, 1.0, 1.5));

        let mut v3 = Vector::new(1.0, 2.0, 3.0);
        v3 += &v2;
        assert_eq!((v3[0], v3[1], v3[2]), (5.0, 7.0, 9.0));
        v3 -= &v2;
        assert_eq!((v3[0], v3[1], v3[2]), (1.0, 2.0, 3.0));
        v3 *= 2.0;
        assert_eq!((v3[0], v3[1], v3[2]), (2.0, 4.0, 6.0));
        v3 /= 2.0;
        assert_eq!((v3[0], v3[1], v3[2]), (1.0, 2.0, 3.0));
    }

    #[test]
    fn test_vector_reverse() {
        let mut v = Vector::new(1.0, 2.0, 3.0);
        v.reverse();
        assert_eq!((v[0], v[1], v[2]), (-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_vector_length() {
        let v = Vector::new(5.5697, -9.84, 1.587);
        let length = v.compute_length();
        assert_eq!(length, 11.4177811806848);
    }

    #[test]
    fn test_vector_unitize() {
        let v = Vector::new(5.5697, -9.84, 1.587);
        assert_eq!(v.normalize().compute_length(), 1.0);
        let mut v = Vector::new(5.5697, -9.84, 1.587);
        v.normalize_self();
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn test_vector_projection() {
        let v = Vector::new(1.0, 1.0, 1.0);
        let x = Vector::x_axis();
        let y = Vector::y_axis();
        let z = Vector::z_axis();
        let (proj_x, _lenx, _perp_x, _perp_lenx) = v.projection(&x);
        let (proj_y, _leny, _perp_y, _perp_leny) = v.projection(&y);
        let (proj_z, _lenz, _perp_z, _perp_lenz) = v.projection(&z);
        assert_eq!((proj_x[0], proj_x[1], proj_x[2]), (1.0, 0.0, 0.0));
        assert_eq!((proj_y[0], proj_y[1], proj_y[2]), (0.0, 1.0, 0.0));
        assert_eq!((proj_z[0], proj_z[1], proj_z[2]), (0.0, 0.0, 1.0));
    }

    #[test]
    fn test_vector_is_parallel_to() {
        let v1 = Vector::new(0.0, 0.0, 1.0);
        let v2 = Vector::new(0.0, 0.0, 2.0);
        let v3 = Vector::new(0.0, 0.0, -1.0);
        let v4 = Vector::new(0.0, 1.0, -1.0);
        assert_eq!(v1.is_parallel_to(&v2), 1);
        assert_eq!(v1.is_parallel_to(&v3), -1);
        assert_eq!(v1.is_parallel_to(&v4), 0);
    }

    #[test]
    fn test_vector_dot() {
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(0.0, 1.0, 0.0);
        let v3 = Vector::new(-1.0, 0.0, 0.0);
        assert_eq!(v1.dot(&v2), 0.0);
        assert_eq!(v1.dot(&v3), -1.0);
        assert_eq!(v1.dot(&v1), 1.0);

        let dot = v1.dot(&v2);
        let mag = v1.compute_length() * v2.compute_length();
        if mag > 0.0 {
            let angle_deg = (dot / mag).acos() * crate::tolerance::TO_DEGREES;
            assert_eq!(angle_deg, 90.0);
        }
    }

    #[test]
    fn test_vector_cross() {
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(0.0, 1.0, 0.0);
        let v3 = v1.cross(&v2);
        assert_eq!((v3[0], v3[1], v3[2]), (0.0, 0.0, 1.0));
    }

    #[test]
    fn test_vector_angle() {
        let v1 = Vector::new(1.0, 1.0, 0.0);
        let v2 = Vector::new(0.0, 1.0, 0.0);
        assert!((v1.angle(&v2, false) - 45.0).abs() < crate::tolerance::Tolerance::ZERO_TOLERANCE);
        assert!(
            (Vector::new(-1.0, 1.0, 0.0).angle(&v2, true) + 45.0).abs()
                < crate::tolerance::Tolerance::ZERO_TOLERANCE
        );
    }

    #[test]
    fn test_vector_get_leveled_vector() {
        let lev = Vector::new(1.0, 1.0, 1.0).get_leveled_vector(1.0);
        assert!((lev.compute_length() - 4.168_432_5).abs() < 1e-4);
    }

    #[test]
    fn test_vector_cosine_law() {
        let c = Vector::cosine_law(100.0, 150.0, 115.0, true);
        assert_eq!((c * 100.0).round() / 100.0, 212.55);
    }

    #[test]
    fn test_vector_sine_law_angle() {
        let angle_b = Vector::sine_law_angle(212.55, 115.0, 150.0, true);
        assert_eq!((angle_b * 100.0).round() / 100.0, 39.76);
    }

    #[test]
    fn test_vector_sine_law_length() {
        let len_b = Vector::sine_law_length(212.55, 115.0, 39.761714, true);
        assert_eq!((len_b * 100.0).round() / 100.0, 150.0);
    }

    #[test]
    fn test_vector_angle_between_vector_xy_components() {
        let v1 = Vector::new(3.0_f64.sqrt(), 1.0, 0.0);
        let v2 = Vector::new(1.0, 3.0_f64.sqrt(), 0.0);
        assert_eq!(
            (Vector::angle_between_vector_xy_components(&v1) * 100.0).round() / 100.0,
            30.0
        );
        assert_eq!(
            (Vector::angle_between_vector_xy_components(&v2) * 100.0).round() / 100.0,
            60.0
        );
    }

    #[test]
    fn test_vector_sum_of_vectors() {
        let vecs = vec![
            Vector::new(1.0, 1.0, 1.0),
            Vector::new(2.0, 2.0, 2.0),
            Vector::new(3.0, 3.0, 3.0),
        ];
        let sum = Vector::sum_of_vectors(&vecs);
        assert_eq!((sum[0], sum[1], sum[2]), (6.0, 6.0, 6.0));
    }

    #[test]
    fn test_vector_coordinate_direction_angles() {
        let abg = Vector::new(35.4, 35.4, 86.6).coordinate_direction_3angles(true);
        assert!((abg[0] - 69.274_2).abs() < 1e-4);
        assert!((abg[1] - 69.274_2).abs() < 1e-4);
        assert!((abg[2] - 30.032058).abs() < 1e-4);

        let pt = Vector::new(1.0, 1.0, 2.0_f64.sqrt()).coordinate_direction_2angles(true);
        assert!((pt[0] - 45.0).abs() < 1e-6);
        assert!((pt[1] - 45.0).abs() < 1e-6);
    }
}
