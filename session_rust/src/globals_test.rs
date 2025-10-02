#[cfg(test)]
mod tests {
    use crate::globals::*;

    #[test]
    fn test_globals_initial_values() {
        unsafe {
            assert_eq!(SCALE, 1e6);
            assert_eq!(PI, 3.141592653589793);
            assert_eq!(ANGLE, 0.11);
            assert_eq!(TOLERANCE, 1e-3);
        }
    }

    #[test]
    fn test_globals_modification() {
        unsafe {
            let original_scale = SCALE;
            SCALE = 2000.0;
            assert_eq!(SCALE, 2000.0);
            SCALE = original_scale;
        }
    }
}
