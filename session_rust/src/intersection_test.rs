#[cfg(test)]
mod tests {
    use crate::intersection::*;
    use crate::{Line, Plane, Point, Tolerance, Vector};

    #[test]
    fn test_line_line_intersection() {
        let l0 = Line::new(500.000, -573.576, -819.152, 500.000, 573.576, 819.152);
        let l1 = Line::new(13.195, 234.832, 534.315, 986.805, 421.775, 403.416);

        let p = line_line(&l0, &l1, Tolerance::APPROXIMATION).expect("Should find intersection");

        assert!((p.x() - 500.0).abs() < 0.1);
        assert!((p.y() - 328.303).abs() < 0.1);
        assert!((p.z() - 468.866).abs() < 0.1);
    }

    #[test]
    fn test_line_line_parameters_with_tolerance() {
        let l0 = Line::new(500.000, -573.576, -819.152, 500.000, 573.576, 819.152);
        let l1 = Line::new(13.195, 234.832, 534.315, 986.805, 421.775, 403.416);

        let result = line_line_parameters(&l0, &l1, Tolerance::APPROXIMATION, true, false)
            .expect("Should find parameters");

        let (t0, t1) = result;
        assert!((0.0..=1.0).contains(&t0));
        assert!((0.0..=1.0).contains(&t1));
    }

    #[test]
    #[allow(clippy::excessive_precision)]
    fn test_plane_plane_intersection() {
        let plane_origin_0 = Point::new(213.787107, 513.797811, -24.743845);
        let plane_xaxis_0 = Vector::new(0.907673, -0.258819, 0.330366);
        let plane_yaxis_0 = Vector::new(0.272094, 0.96225, 0.006285);
        let pl0 = Plane::new(plane_origin_0, plane_xaxis_0, plane_yaxis_0);

        let plane_origin_1 = Point::new(247.17924, 499.115486, 59.619568);
        let plane_xaxis_1 = Vector::new(0.552465, 0.816035, 0.16991);
        let plane_yaxis_1 = Vector::new(0.172987, 0.087156, -0.98106);
        let pl1 = Plane::new(plane_origin_1, plane_xaxis_1, plane_yaxis_1);

        let intersection_line = plane_plane(&pl0, &pl1).expect("Should find intersection");

        let start = intersection_line.start();
        let end = intersection_line.end();

        assert!((start.x() - 252.4632).abs() < 0.01);
        assert!((start.y() - 495.32248).abs() < 0.01);
        assert!((start.z() - (-10.002656)).abs() < 0.01);

        assert!((end.x() - 253.01033).abs() < 0.01);
        assert!((end.y() - 496.1218).abs() < 0.01);
        assert!((end.z() - (-9.888727)).abs() < 0.01);
    }

    #[test]
    #[allow(clippy::excessive_precision)]
    fn test_plane_plane_plane_intersection() {
        let plane_origin_0 = Point::new(213.787107, 513.797811, -24.743845);
        let plane_xaxis_0 = Vector::new(0.907673, -0.258819, 0.330366);
        let plane_yaxis_0 = Vector::new(0.272094, 0.96225, 0.006285);
        let pl0 = Plane::new(plane_origin_0, plane_xaxis_0, plane_yaxis_0);

        let plane_origin_1 = Point::new(247.17924, 499.115486, 59.619568);
        let plane_xaxis_1 = Vector::new(0.552465, 0.816035, 0.16991);
        let plane_yaxis_1 = Vector::new(0.172987, 0.087156, -0.98106);
        let pl1 = Plane::new(plane_origin_1, plane_xaxis_1, plane_yaxis_1);

        let plane_origin_2 = Point::new(221.399816, 605.893667, -54.000116);
        let plane_xaxis_2 = Vector::new(0.903451, -0.360516, -0.231957);
        let plane_yaxis_2 = Vector::new(0.172742, -0.189057, 0.966653);
        let pl2 = Plane::new(plane_origin_2, plane_xaxis_2, plane_yaxis_2);

        let ppp = plane_plane_plane(&pl0, &pl1, &pl2).expect("Should find intersection");

        assert!((ppp.x() - 300.5).abs() < 0.1);
        assert!((ppp.y() - 565.5).abs() < 0.1);
        assert!((ppp.z() - 0.0).abs() < 0.1);
    }

    #[test]
    #[allow(clippy::excessive_precision)]
    fn test_line_plane_intersection() {
        let l0 = Line::new(500.000, -573.576, -819.152, 500.000, 573.576, 819.152);

        let plane_origin_0 = Point::new(213.787107, 513.797811, -24.743845);
        let plane_xaxis_0 = Vector::new(0.907673, -0.258819, 0.330366);
        let plane_yaxis_0 = Vector::new(0.272094, 0.96225, 0.006285);
        let pl0 = Plane::new(plane_origin_0, plane_xaxis_0, plane_yaxis_0);

        let lp = line_plane(&l0, &pl0, true).expect("Should find intersection");

        assert!((lp.x() - 500.0).abs() < 0.1);
        assert!((lp.y() - 77.7531).abs() < 0.01);
        assert!((lp.z() - 111.043).abs() < 0.01);
    }

    #[test]
    #[allow(clippy::excessive_precision)]
    fn test_ray_box_intersection() {
        let l0 = Line::new(500.0, -573.576, -819.152, 500.0, 573.576, 819.152);
        let min_p = Point::new(214.0, 192.0, 484.0);
        let max_p = Point::new(694.0, 567.0, 796.0);
        let box_ = crate::BoundingBox::from_points(&[min_p, max_p], 0.0);

        let points = ray_box(&l0, &box_, 0.0, 1000.0).expect("Should find intersection");

        assert_eq!(points.len(), 2);

        // Entry point
        assert!((points[0].x() - 500.0).abs() < 0.1);
        assert!((points[0].y() - 338.9).abs() < 0.1);
        assert!((points[0].z() - 484.0).abs() < 0.1);

        // Exit point
        assert!((points[1].x() - 500.0).abs() < 0.1);
        assert!((points[1].y() - 557.365).abs() < 0.1);
        assert!((points[1].z() - 796.0).abs() < 0.1);
    }

    #[test]
    fn test_ray_box_no_intersection() {
        let l0 = Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let min_p = Point::new(10.0, 10.0, 10.0);
        let max_p = Point::new(20.0, 20.0, 20.0);
        let box_ = crate::BoundingBox::from_points(&[min_p, max_p], 0.0);

        let points = ray_box(&l0, &box_, 0.0, 1000.0);

        assert!(points.is_none());
    }

    #[test]
    #[allow(clippy::excessive_precision)]
    fn test_ray_sphere_intersection() {
        let l0 = Line::new(500.0, -573.576, -819.152, 500.0, 573.576, 819.152);
        let sphere_center = Point::new(457.0, 192.0, 207.0);
        let radius = 265.0;

        let points = ray_sphere(&l0, &sphere_center, radius).expect("Should find intersection");

        assert_eq!(points.len(), 2);

        // First intersection point
        assert!((points[0].x() - 500.0).abs() < 0.1);
        assert!((points[0].y() - 12.08).abs() < 0.1);
        assert!((points[0].z() - 17.25).abs() < 0.1);

        // Second intersection point
        assert!((points[1].x() - 500.0).abs() < 0.1);
        assert!((points[1].y() - 308.77).abs() < 0.1);
        assert!((points[1].z() - 440.97).abs() < 0.1);
    }

    #[test]
    fn test_ray_sphere_no_intersection() {
        let l0 = Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let sphere_center = Point::new(0.0, 10.0, 0.0);
        let radius = 5.0;

        let points = ray_sphere(&l0, &sphere_center, radius);

        assert!(points.is_none());
    }

    #[test]
    #[allow(clippy::excessive_precision)]
    fn test_ray_triangle_intersection() {
        let l0 = Line::new(500.0, -573.576, -819.152, 500.0, 573.576, 819.152);
        let p1 = Point::new(214.0, 567.0, 484.0);
        let p2 = Point::new(214.0, 192.0, 796.0);
        let p3 = Point::new(694.0, 192.0, 484.0);

        let triangle_hit = ray_triangle(&l0, &p1, &p2, &p3, crate::Tolerance::APPROXIMATION)
            .expect("Should find intersection");

        assert!((triangle_hit.x() - 500.0).abs() < 0.1);
        assert!((triangle_hit.y() - 340.616).abs() < 0.01);
        assert!((triangle_hit.z() - 486.451).abs() < 0.01);
    }

    #[test]
    fn test_ray_triangle_no_intersection() {
        let l0 = Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p1 = Point::new(10.0, 10.0, 10.0);
        let p2 = Point::new(10.0, 20.0, 10.0);
        let p3 = Point::new(10.0, 15.0, 20.0);

        let triangle_hit = ray_triangle(&l0, &p1, &p2, &p3, crate::Tolerance::APPROXIMATION);

        assert!(triangle_hit.is_none());
    }
}
