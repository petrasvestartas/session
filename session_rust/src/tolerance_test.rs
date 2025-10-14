use crate::{tolerance::TOL, Tolerance};

#[test]
fn test_tolerance_default_tolerance() {
    assert_eq!(TOL.precision(), Tolerance::PRECISION);
    assert_eq!(TOL.precision(), 3);
}

#[test]
fn test_tolerance_format_number() {
    assert_eq!(TOL.format_number(0.0, Some(3)), "0.000");
    assert_eq!(TOL.format_number(0.5, Some(3)), "0.500");
}

#[test]
fn test_tolerance_is_zero() {
    assert!(TOL.is_zero(0.0, None));
    assert!(TOL.is_zero(1e-15, None));
    assert!(!TOL.is_zero(0.1, None));
}
