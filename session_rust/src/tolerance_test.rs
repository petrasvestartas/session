use crate::{tolerance::TOL, Point, Tolerance};

#[test]
fn test_tolerance_default_tolerance() {
    assert_eq!(TOL.precision(), Tolerance::PRECISION);
    assert_eq!(TOL.precision(), 3);
}

#[test]
fn test_tolerance_format_number() {
    assert_eq!(TOL.format_number(0.0, Some(3)), "0.000");
    assert_eq!(TOL.format_number(0.5, Some(3)), "0.500");
    assert_eq!(TOL.format_number(0.0, Some(3)), "0.000");
}

#[test]
fn test_tolerance_format_number_with_default_precision() {
    assert_eq!(TOL.format_number(0.0, None), "0.000");
    assert_eq!(TOL.format_number(0.5, None), "0.500");
    assert_eq!(TOL.format_number(0.0, None), "0.000");
}

#[test]
fn test_tolerance_format_point() {
    let point = Point::new(0.0, 0.0, 0.0);
    assert_eq!(format!("{point}"), "Point(x=0.000, y=0.000, z=0.000)");
}

#[test]
fn test_tolerance_change_values() {
    // Create a mutable tolerance instance
    let mut tol = Tolerance::new("M");

    // Test default values
    assert_eq!(tol.precision(), Tolerance::PRECISION);
    assert_eq!(tol.absolute(), Tolerance::ABSOLUTE);

    // Change precision and test formatting
    tol.set_precision(2);
    assert_eq!(tol.precision(), 2);
    assert_eq!(tol.format_number(1.23456, None), "1.23");

    // Change absolute tolerance and test zero checking
    tol.set_absolute(1e-5);
    assert_eq!(tol.absolute(), 1e-5);
    assert!(tol.is_zero(1e-6, None)); // Should be true with new tolerance
    assert!(!tol.is_zero(1e-4, None)); // Should be false

    // Reset to defaults and verify
    tol.reset();
    assert_eq!(tol.precision(), Tolerance::PRECISION);
    assert_eq!(tol.absolute(), Tolerance::ABSOLUTE);
    assert_eq!(tol.format_number(1.23456, None), "1.235"); // Back to 3 decimal places

    // Verify absolute tolerance is back to default
    assert!(!tol.is_zero(1e-6, None)); // Should be false with default tolerance
}
