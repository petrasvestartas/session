use crate::curve::validate;
use session_rust::{NurbsCurve, Point};

fn curve(periodic: bool) -> NurbsCurve {
    NurbsCurve::create(periodic, 2, &[Point::new(-0.6, -0.4, 0.0), Point::new(0.0, 0.6, 0.0), Point::new(0.6, -0.4, 0.0)])
}

#[test]
fn valid_curve_domains_keep_original_knots_and_controls() {
    for periodic in [false, true] {
        let source = curve(periodic); let cvs = source.m_cv.clone(); let knots = source.m_nurbsknot.clone();
        let domain = validate(&source).unwrap(); assert!(domain.0 < domain.1);
        assert_eq!(source.m_cv, cvs); assert_eq!(source.m_nurbsknot, knots);
    }
    let mut rational = NurbsCurve::new(3, true, 2, 2);
    rational.m_cv = vec![0.0, 0.0, 0.0, 2.0, 2.0, 0.0, 0.0, 1.0];
    rational.m_nurbsknot = vec![0.0, 1.0]; assert_eq!(validate(&rational).unwrap(), (0.0, 1.0));
}

#[test]
fn malicious_curve_layouts_return_errors_before_kernel_evaluation() {
    for case in 0..9 {
        let mut source = curve(false);
        match case {
            0 => source.m_cv_count = usize::MAX, 1 => source.m_cv_stride = usize::MAX,
            2 => source.m_order = usize::MAX, 3 => source.m_cv.clear(),
            4 => source.m_nurbsknot.clear(), 5 => source.m_nurbsknot[0] = f64::NAN,
            6 => source.m_cv[0] = f64::INFINITY, 7 => source.m_nurbsknot.fill(0.0),
            _ => source.m_nurbsknot[0] = 1e6,
        }
        assert!(validate(&source).is_err());
    }
    let mut rational = NurbsCurve::new(3, true, 2, 2); rational.m_nurbsknot = vec![0.0, 1.0];
    rational.m_cv = vec![0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 1.0]; assert!(validate(&rational).is_err());
}
