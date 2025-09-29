use std::f64::consts::PI as STD_PI;

/// Mutable global variables (direct access like Python/C++)
/// WARNING: Requires unsafe blocks for access - not thread-safe!
/// Mathematical constants
pub static mut PI: f64 = STD_PI;
pub static mut TO_DEGREES: f64 = 180.0 / STD_PI;
pub static mut TO_RADIANS: f64 = STD_PI / 180.0;

/// Tolerance values
pub static mut ZERO_TOLERANCE: f64 = 1e-12;
pub static mut ANGLE: f64 = 0.11;
pub static mut TOLERANCE: f64 = 1e-3;

/// Double precision limits
pub static mut DOUBLE_MIN: f64 = f64::MIN;
pub static mut DOUBLE_MAX: f64 = f64::MAX;
pub static mut EPSILON: f64 = f64::EPSILON;
pub static mut SQRT_EPSILON: f64 = 1.4901161193847656e-8; // sqrt(f64::EPSILON)

/// Metric prefixes
pub static mut GIGA: f64 = 1e9;
pub static mut MEGA: f64 = 1e6;
pub static mut KILO: f64 = 1e3;
pub static mut MILLI: f64 = 1e-3;
pub static mut MICRO: f64 = 1e-6;
pub static mut NANO: f64 = 1e-9;

/// Base units
pub static mut FORCE: f64 = 1.0;
pub static mut MASS: f64 = 1.0;
pub static mut LENGTH: f64 = 1.0;

/// Scale factor
pub static mut SCALE: f64 = 1e6;
