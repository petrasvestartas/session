use serde::{Deserialize, Serialize, Serializer};
use std::fmt;
use crate::{FromJsonData, HasJsonData};
use serde_json::Value;

/// A color in RGBA format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct Color {
    /// Red component (0-255)
    pub r: u8,
    /// Green component (0-255)
    pub g: u8,
    /// Blue component (0-255)
    pub b: u8,
    /// Alpha component (0-255)
    pub a: u8,
}

impl Color {
    /// Create a new Color with RGBA values.
    ///
    /// # Arguments
    ///
    /// * `r` - Red component (0-255)
    /// * `g` - Green component (0-255)
    /// * `b` - Blue component (0-255)
    /// * `a` - Alpha component (0-255)
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Color;
    /// let color = Color::new(255, 0, 0, 255); // Red
    /// assert_eq!(color.r, 255);
    /// assert_eq!(color.g, 0);
    /// assert_eq!(color.b, 0);
    /// assert_eq!(color.a, 255);
    /// ```
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    /// Create a new Color with RGB values and full opacity.
    ///
    /// # Arguments
    ///
    /// * `r` - Red component (0-255)
    /// * `g` - Green component (0-255)
    /// * `b` - Blue component (0-255)
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Color;
    /// let color = Color::rgb(255, 0, 0); // Red
    /// assert_eq!(color.r, 255);
    /// assert_eq!(color.g, 0);
    /// assert_eq!(color.b, 0);
    /// assert_eq!(color.a, 255);
    /// ```
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }

    /// Create a black color.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Color;
    /// let black = Color::black();
    /// assert_eq!(black.r, 0);
    /// assert_eq!(black.g, 0);
    /// assert_eq!(black.b, 0);
    /// assert_eq!(black.a, 255);
    /// ```
    pub fn black() -> Self {
        Color::rgb(0, 0, 0)
    }

    /// Create a white color.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Color;
    /// let white = Color::white();
    /// assert_eq!(white.r, 255);
    /// assert_eq!(white.g, 255);
    /// assert_eq!(white.b, 255);
    /// assert_eq!(white.a, 255);
    /// ```
    pub fn white() -> Self {
        Color::rgb(255, 255, 255)
    }

    /// Create a red color.
    pub fn red() -> Self {
        Color::rgb(255, 0, 0)
    }

    /// Create a green color.
    pub fn green() -> Self {
        Color::rgb(0, 255, 0)
    }

    /// Create a blue color.
    pub fn blue() -> Self {
        Color::rgb(0, 0, 255)
    }

    /// Create a yellow color.
    pub fn yellow() -> Self {
        Color::rgb(255, 255, 0)
    }

    /// Create a cyan color.
    pub fn cyan() -> Self {
        Color::rgb(0, 255, 255)
    }

    /// Create a magenta color.
    pub fn magenta() -> Self {
        Color::rgb(255, 0, 255)
    }

    /// Create a transparent color.
    pub fn transparent() -> Self {
        Color { r: 0, g: 0, b: 0, a: 0 }
    }

    /// Convert color to floating point representation (0.0-1.0 range)
    ///
    /// # Returns
    ///
    /// A tuple (r, g, b, a) with values in range 0.0-1.0
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Color;
    /// let color = Color::rgb(255, 128, 0);
    /// let (r, g, b, a) = color.to_float();
    /// assert_eq!(r, 1.0);
    /// assert_eq!(g, 0.5019607843137255);
    /// assert_eq!(b, 0.0);
    /// assert_eq!(a, 1.0);
    /// ```
    pub fn to_float(&self) -> (f32, f32, f32, f32) {
        (
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        )
    }

    /// Convert color to floating point array representation (0.0-1.0 range)
    ///
    /// # Returns
    ///
    /// An array [r, g, b, a] with values in range 0.0-1.0
    pub fn to_float_array(&self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ]
    }

    /// Create a Color from floating point values (0.0-1.0 range)
    ///
    /// # Arguments
    ///
    /// * `r` - Red component (0.0-1.0)
    /// * `g` - Green component (0.0-1.0)
    /// * `b` - Blue component (0.0-1.0)
    /// * `a` - Alpha component (0.0-1.0)
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Color;
    /// let color = Color::from_float(1.0, 0.5, 0.0, 1.0);
    /// assert_eq!(color.r, 255);
    /// assert_eq!(color.g, 128);
    /// assert_eq!(color.b, 0);
    /// assert_eq!(color.a, 255);
    /// ```
    pub fn from_float(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color {
            r: (r.clamp(0.0, 1.0) * 255.0).round() as u8,
            g: (g.clamp(0.0, 1.0) * 255.0).round() as u8,
            b: (b.clamp(0.0, 1.0) * 255.0).round() as u8,
            a: (a.clamp(0.0, 1.0) * 255.0).round() as u8,
        }
    }

    /// Create a color from a hexadecimal string representation.
    ///
    /// # Arguments
    ///
    /// * `hex` - Hex color string in the format "#RRGGBB" or "#RRGGBBAA"
    ///
    /// # Returns
    ///
    /// * `Some(Color)` - if the string is a valid hex color
    /// * `None` - if the string format is invalid
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Color;
    /// let red = Color::from_hex("#FF0000").unwrap();
    /// assert_eq!(red.r, 255);
    /// assert_eq!(red.g, 0);
    /// assert_eq!(red.b, 0);
    /// assert_eq!(red.a, 255);
    ///
    /// let semi_transparent_blue = Color::from_hex("#0000FF80").unwrap();
    /// assert_eq!(semi_transparent_blue.r, 0);
    /// assert_eq!(semi_transparent_blue.g, 0);
    /// assert_eq!(semi_transparent_blue.b, 255);
    /// assert_eq!(semi_transparent_blue.a, 128);
    /// ```
    pub fn from_hex(hex: &str) -> Option<Self> {
        // Remove leading # if present
        let hex = hex.strip_prefix('#').unwrap_or(hex);
        
        // Parse RGB or RGBA
        match hex.len() {
            6 => {
                // RGB format (#RRGGBB)
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Color { r, g, b, a: 255 })
            },
            8 => {
                // RGBA format (#RRGGBBAA)
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                Some(Color { r, g, b, a })
            },
            _ => None,
        }
    }

    /// Convert color to hexadecimal string representation
    ///
    /// # Arguments
    ///
    /// * `include_alpha` - Whether to include the alpha component
    ///
    /// # Returns
    ///
    /// String in the format "#RRGGBB" or "#RRGGBBAA"
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Color;
    /// let red = Color::rgb(255, 0, 0);
    /// assert_eq!(red.to_hex(false), "#FF0000");
    /// ```
    pub fn to_hex(&self, include_alpha: bool) -> String {
        if include_alpha {
            format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
        } else {
            format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
        }
    }
}

// Implement Display
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.a == 255 {
            write!(f, "RGB({}, {}, {})", self.r, self.g, self.b)
        } else {
            write!(f, "RGBA({}, {}, {}, {})", self.r, self.g, self.b, self.a)
        }
    }
}

// Custom Serialize implementation to use simple format for wink compatibility
impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Use simple format for wink compatibility
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Color", 4)?;
        state.serialize_field("r", &self.r)?;
        state.serialize_field("g", &self.g)?;
        state.serialize_field("b", &self.b)?;
        state.serialize_field("a", &self.a)?;
        state.end()
    }
}

// COMPAS-style JSON serialization support
impl HasJsonData for Color {
    fn to_json_data(&self, _minimal: bool) -> Value {
        serde_json::json!({
            "dtype": "Color",
            "r": self.r,
            "g": self.g,
            "b": self.b,
            "a": self.a
        })
    }
}

impl FromJsonData for Color {
    fn from_json_data(data: &Value) -> Option<Self> {
        if let (Some(r), Some(g), Some(b), Some(a)) = (
            data.get("r").and_then(|v| v.as_u64()).map(|v| v as u8),
            data.get("g").and_then(|v| v.as_u64()).map(|v| v as u8),
            data.get("b").and_then(|v| v.as_u64()).map(|v| v as u8),
            data.get("a").and_then(|v| v.as_u64()).map(|v| v as u8)
        ) {
            Some(Color::new(r, g, b, a))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_new() {
        let color = Color::new(10, 20, 30, 40);
        assert_eq!(color.r, 10);
        assert_eq!(color.g, 20);
        assert_eq!(color.b, 30);
        assert_eq!(color.a, 40);
    }

    #[test]
    fn test_color_rgb() {
        let color = Color::rgb(10, 20, 30);
        assert_eq!(color.r, 10);
        assert_eq!(color.g, 20);
        assert_eq!(color.b, 30);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_color_hex() {
        // Test hex parsing
        let color1 = Color::from_hex("#FF5500").unwrap();
        assert_eq!(color1.r, 255);
        assert_eq!(color1.g, 85);
        assert_eq!(color1.b, 0);
        assert_eq!(color1.a, 255);

        let color2 = Color::from_hex("FF5500AA").unwrap();
        assert_eq!(color2.r, 255);
        assert_eq!(color2.g, 85);
        assert_eq!(color2.b, 0);
        assert_eq!(color2.a, 170);

        // Test hex output
        let color3 = Color::new(255, 85, 0, 170);
        assert_eq!(color3.to_hex(false), "#FF5500");
        assert_eq!(color3.to_hex(true), "#FF5500AA");
    }
}
