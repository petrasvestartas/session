use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI as STD_PI;

/// Mathematical constants
pub const PI: f64 = STD_PI;
pub const TO_DEGREES: f64 = 180.0 / STD_PI;
pub const TO_RADIANS: f64 = STD_PI / 180.0;

/// Scale factor
pub const SCALE: f64 = 1e6;

/// Tolerance settings for geometric operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tolerance {
    pub unit: String,
    absolute: Option<f64>,
    relative: Option<f64>,
    angular: Option<f64>,
    approximation: Option<f64>,
    precision: Option<i32>,
    lineardeflection: Option<f64>,
    angulardeflection: Option<f64>,
}

impl Tolerance {
    /// Default tolerance values (f64 only)
    pub const ABSOLUTE: f64 = 1e-9;
    pub const RELATIVE: f64 = 1e-6;
    pub const ANGULAR: f64 = 1e-6;
    pub const APPROXIMATION: f64 = 1e-3;
    pub const PRECISION: i32 = 3;
    pub const LINEARDEFLECTION: f64 = 1e-3;
    pub const ANGULARDEFLECTION: f64 = 1e-1;
    pub const ANGLE_TOLERANCE_DEGREES: f64 = 0.11;
    pub const ZERO_TOLERANCE: f64 = 1e-12;

    pub fn new(unit: &str) -> Self {
        Self {
            unit: unit.to_string(),
            absolute: None,
            relative: None,
            angular: None,
            approximation: None,
            precision: None,
            lineardeflection: None,
            angulardeflection: None,
        }
    }

    pub fn reset(&mut self) {
        self.absolute = None;
        self.relative = None;
        self.angular = None;
        self.approximation = None;
        self.precision = None;
        self.lineardeflection = None;
        self.angulardeflection = None;
    }

    pub fn absolute(&self) -> f64 {
        self.absolute.unwrap_or(Self::ABSOLUTE)
    }

    pub fn set_absolute(&mut self, value: f64) {
        self.absolute = Some(value);
    }

    pub fn relative(&self) -> f64 {
        self.relative.unwrap_or(Self::RELATIVE)
    }

    pub fn set_relative(&mut self, value: f64) {
        self.relative = Some(value);
    }

    pub fn angular(&self) -> f64 {
        self.angular.unwrap_or(Self::ANGULAR)
    }

    pub fn set_angular(&mut self, value: f64) {
        self.angular = Some(value);
    }

    pub fn approximation(&self) -> f64 {
        self.approximation.unwrap_or(Self::APPROXIMATION)
    }

    pub fn set_approximation(&mut self, value: f64) {
        self.approximation = Some(value);
    }

    pub fn precision(&self) -> i32 {
        self.precision.unwrap_or(Self::PRECISION)
    }

    pub fn set_precision(&mut self, value: i32) {
        self.precision = Some(value);
    }

    pub fn lineardeflection(&self) -> f64 {
        self.lineardeflection.unwrap_or(Self::LINEARDEFLECTION)
    }

    pub fn set_lineardeflection(&mut self, value: f64) {
        self.lineardeflection = Some(value);
    }

    pub fn angulardeflection(&self) -> f64 {
        self.angulardeflection.unwrap_or(Self::ANGULARDEFLECTION)
    }

    pub fn set_angulardeflection(&mut self, value: f64) {
        self.angulardeflection = Some(value);
    }

    pub fn tolerance(&self, truevalue: f64, rtol: f64, atol: f64) -> f64 {
        rtol * truevalue.abs() + atol
    }

    pub fn compare(&self, a: f64, b: f64, rtol: f64, atol: f64) -> bool {
        (a - b).abs() <= self.tolerance(b, rtol, atol)
    }

    pub fn is_zero(&self, a: f64, tol: Option<f64>) -> bool {
        let tol = tol.unwrap_or(self.absolute());
        a.abs() <= tol
    }

    pub fn is_positive(&self, a: f64, tol: Option<f64>) -> bool {
        let tol = tol.unwrap_or(self.absolute());
        a > tol
    }

    pub fn is_negative(&self, a: f64, tol: Option<f64>) -> bool {
        let tol = tol.unwrap_or(self.absolute());
        a < -tol
    }

    pub fn is_between(&self, value: f64, minval: f64, maxval: f64, atol: Option<f64>) -> bool {
        let atol = atol.unwrap_or(self.absolute());
        minval - atol <= value && value <= maxval + atol
    }

    pub fn is_close(&self, a: f64, b: f64, rtol: Option<f64>, atol: Option<f64>) -> bool {
        let rtol = rtol.unwrap_or(self.relative());
        let atol = atol.unwrap_or(self.absolute());
        self.compare(a, b, rtol, atol)
    }

    pub fn is_allclose(&self, a: &[f64], b: &[f64], rtol: Option<f64>, atol: Option<f64>) -> bool {
        let rtol = rtol.unwrap_or(self.relative());
        let atol = atol.unwrap_or(self.absolute());
        a.iter()
            .zip(b.iter())
            .all(|(x, y)| self.compare(*x, *y, rtol, atol))
    }

    pub fn is_angle_zero(&self, a: f64, tol: Option<f64>) -> bool {
        let tol = tol.unwrap_or(self.angular());
        a.abs() <= tol
    }

    pub fn is_angles_close(&self, a: f64, b: f64, tol: Option<f64>) -> bool {
        let tol = tol.unwrap_or(self.angular());
        (a - b).abs() <= tol
    }

    pub fn geometric_key(&self, xyz: [f64; 3], precision: Option<i32>) -> String {
        let precision = precision.unwrap_or_else(|| self.precision());
        let [mut x, mut y, mut z] = xyz;

        if precision == -1 {
            return format!("{},{},{}", x as i64, y as i64, z as i64);
        }

        if precision < -1 {
            let p = (-precision - 1) as u32;
            let factor = 10_f64.powi(p as i32);
            return format!(
                "{},{},{}",
                ((x / factor).round() * factor) as i64,
                ((y / factor).round() * factor) as i64,
                ((z / factor).round() * factor) as i64
            );
        }

        let minzero = format!("-{:.prec$}", 0.0, prec = precision as usize);
        if format!("{:.prec$}", x, prec = precision as usize) == minzero {
            x = 0.0;
        }
        if format!("{:.prec$}", y, prec = precision as usize) == minzero {
            y = 0.0;
        }
        if format!("{:.prec$}", z, prec = precision as usize) == minzero {
            z = 0.0;
        }

        format!(
            "{:.prec$},{:.prec$},{:.prec$}",
            x,
            y,
            z,
            prec = precision as usize
        )
    }

    pub fn geometric_key_xy(&self, xy: [f64; 2], precision: Option<i32>) -> String {
        let precision = precision.unwrap_or_else(|| self.precision());
        let [mut x, mut y] = xy;

        if precision == -1 {
            return format!("{},{}", x as i64, y as i64);
        }

        if precision < -1 {
            let p = (-precision - 1) as u32;
            let factor = 10_f64.powi(p as i32);
            return format!(
                "{},{}",
                ((x / factor).round() * factor) as i64,
                ((y / factor).round() * factor) as i64
            );
        }

        let minzero = format!("-{:.prec$}", 0.0, prec = precision as usize);
        if format!("{:.prec$}", x, prec = precision as usize) == minzero {
            x = 0.0;
        }
        if format!("{:.prec$}", y, prec = precision as usize) == minzero {
            y = 0.0;
        }

        format!("{:.prec$},{:.prec$}", x, y, prec = precision as usize)
    }

    pub fn format_number(&self, number: f64, precision: Option<i32>) -> String {
        let precision = precision.unwrap_or_else(|| self.precision());

        if precision == -1 {
            return format!("{}", number.round() as i64);
        }

        if precision < -1 {
            let p = (-precision - 1) as u32;
            let factor = 10_f64.powi(p as i32);
            return format!("{}", ((number / factor).round() * factor) as i64);
        }

        format!("{:.prec$}", number, prec = precision as usize)
    }

    pub fn precision_from_tolerance(&self, tol: Option<f64>) -> i32 {
        let tol = tol.unwrap_or_else(|| self.absolute());
        if tol < 1.0 {
            let s = format!("{tol:e}");
            if let Some(exp_pos) = s.find("e-") {
                if let Ok(exp) = s[exp_pos + 2..].parse::<i32>() {
                    return exp;
                }
            }
        }
        0
    }
}

impl Default for Tolerance {
    fn default() -> Self {
        Self::new("M")
    }
}

pub static TOL: Lazy<Tolerance> = Lazy::new(Tolerance::default);

#[cfg(test)]
#[path = "tolerance_test.rs"]
mod tolerance_test;
