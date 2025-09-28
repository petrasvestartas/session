#[cfg(test)]
mod tests {
    use crate::globals::*;

    #[test]
    fn test_globals_initial_values() {
        unsafe {
            assert_eq!(ZERO_TOLERANCE, 1e-12);
            assert_eq!(SCALE, 1e6);
            assert_eq!(PI, 3.141592653589793);
            assert_eq!(TO_DEGREES, 180.0 / PI);
            assert_eq!(TO_RADIANS, PI / 180.0);
            assert_eq!(ANGLE, 0.11);
            assert_eq!(TOLERANCE, 1e-3);
            assert_eq!(MILLI, 1e-3);
            assert_eq!(MICRO, 1e-6);
            assert_eq!(NANO, 1e-9);
            assert_eq!(KILO, 1e3);
            assert_eq!(MEGA, 1e6);
            assert_eq!(GIGA, 1e9);
            assert_eq!(FORCE, 1.0);
            assert_eq!(MASS, 1.0);
            assert_eq!(LENGTH, 1.0);
        }
    }

    #[test]
    fn test_globals_modification() {
        unsafe {
            // Test that mutable globals can be changed (like Python/C++)
            let original_tolerance = ZERO_TOLERANCE;
            let original_scale = SCALE;

            ZERO_TOLERANCE = 1e-15;
            SCALE = 2000.0;

            assert_eq!(ZERO_TOLERANCE, 1e-15);
            assert_eq!(SCALE, 2000.0);

            // Reset to original values
            ZERO_TOLERANCE = original_tolerance;
            SCALE = original_scale;
        }
    }

    #[test]
    fn test_mathematical_constants() {
        unsafe {
            // Test that our PI matches std::f64::consts::PI
            assert_eq!(PI, std::f64::consts::PI);

            // Test conversions
            assert!((TO_DEGREES * TO_RADIANS - 1.0).abs() < EPSILON);
            assert!((90.0 * TO_RADIANS - PI / 2.0).abs() < EPSILON);
            assert!((PI * TO_DEGREES - 180.0).abs() < EPSILON);
        }
    }

    #[test]
    fn test_tolerance_hierarchy() {
        unsafe {
            // Ensure tolerance values are in correct order
            assert!(ZERO_TOLERANCE < MICRO);
            assert!(MICRO < MILLI);
            // Note: TOLERANCE (1e-3) equals MILLI (1e-3)
            assert!(MILLI <= TOLERANCE);
            assert!(EPSILON < SQRT_EPSILON);
            assert!(ZERO_TOLERANCE < SQRT_EPSILON);
        }
    }

    #[test]
    fn test_scale_relationships() {
        unsafe {
            assert_eq!(SCALE, MEGA);
            assert_eq!(KILO * KILO, MEGA);
            assert_eq!(MEGA * KILO, GIGA);
            assert_eq!(MILLI * KILO, 1.0);
            assert_eq!(MICRO * MEGA, 1.0);
        }
    }
}
