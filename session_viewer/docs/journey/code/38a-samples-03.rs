use crate::curve::sample;
use session_rust::{NurbsCurve, Point};

#[test]
fn sampling_draws_the_curve_and_keeps_exact_original_controls() {
    let controls = [Point::new(-0.6, -0.4, 0.0), Point::new(0.0, 0.6, 0.0), Point::new(0.6, -0.4, 0.0)];
    let source = NurbsCurve::create(false, 2, &controls); let original = source.m_cv.clone();
    let points = sample(&source).unwrap(); assert!(points.len() > 20 && points.len() <= 513);
    assert_eq!(source.m_cv, original); assert_eq!(points.first().unwrap(), &[-0.6, -0.4, 0.0]);
    assert_eq!(points.last().unwrap(), &[0.6, -0.4, 0.0]);
    let apex = points.iter().map(|p| p[1]).fold(f64::NEG_INFINITY, f64::max);
    assert!((apex - 0.1).abs() < 0.001); assert!(apex < controls[1][1] - 0.4);
    let (a, b) = source.domain();
    for i in 0..=1000 {
        let p = source.point_at(a + (b - a) * i as f64 / 1000.0);
        let error = points.windows(2).map(|pair| {
            let d = [pair[1][0] - pair[0][0], pair[1][1] - pair[0][1]];
            let length = d[0] * d[0] + d[1] * d[1];
            let t = (((p[0] - pair[0][0]) * d[0] + (p[1] - pair[0][1]) * d[1]) / length).clamp(0.0, 1.0);
            (p[0] - pair[0][0] - d[0] * t).hypot(p[1] - pair[0][1] - d[1] * t)
        }).fold(f64::INFINITY, f64::min);
        assert!(error < 0.001, "Measured chord error: {error}");
    }
}

#[test]
fn straight_and_periodic_curves_keep_required_endpoints_and_knots() {
    let source = NurbsCurve::create(false, 2, &[Point::new(0.0, 0.0, 0.0), Point::new(0.5, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)]);
    assert_eq!(sample(&source).unwrap(), vec![[0.0; 3], [1.0, 0.0, 0.0]]);
    let points = [Point::new(-0.6, -0.4, 0.0), Point::new(0.0, 0.6, 0.0), Point::new(0.6, -0.4, 0.0)];
    let periodic = NurbsCurve::create(true, 2, &points); let sampled = sample(&periodic).unwrap();
    assert!(sampled.len() <= 10_513); let first = sampled.first().unwrap(); let last = sampled.last().unwrap();
    assert!((0..3).all(|axis| (first[axis] - last[axis]).abs() < 1e-10));
    let corners = NurbsCurve::create(false, 1, &points); let sampled = sample(&corners).unwrap();
    assert!(sampled.iter().any(|p| (p[0] - points[1][0]).abs() < 1e-12 && (p[1] - points[1][1]).abs() < 1e-12));
    let mut invalid = corners.clone(); invalid.m_cv_stride = usize::MAX; assert!(sample(&invalid).is_err());
}
