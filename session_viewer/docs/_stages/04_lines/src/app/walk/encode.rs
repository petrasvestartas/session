//! Row encodings shared by every producer: pen widths to radii, colours to RGBA8. Pure
//! functions on numbers.

/// An authored width (kernel millimetres) as the world-mm RADIUS the shaders project; the
/// untouched 1.0 default (and 0 / non-finite) is 0.0 = the screen-constant pen.
pub fn encode_width(w: f64) -> f32 {
    if w.is_finite() && w > 0.0 && (w - 1.0).abs() > 1e-9 {
        (w as f32) * 0.5
    } else {
        0.0
    }
}

/// One colour channel to a byte, rounded.
fn quant8(v: f32) -> u32 {
    ((v.clamp(0.0, 1.0) * 255.0 + 0.5) as u32) & 0xff
}

/// RGBA8 in one word, low byte red - the layout `unpack4x8unorm` expects in WGSL.
pub fn pack_rgba(c: [f32; 4]) -> u32 {
    quant8(c[0]) | quant8(c[1]) << 8 | quant8(c[2]) << 16 | quant8(c[3]) << 24
}

/// `facing` meaning "no adjacency, always draw". All-ones: (0, 0) is the honest code for +Z.
pub const FACING_UNKNOWN: u32 = u32::MAX;

/// One pen for a run of segments: the object row, the encoded radius and the packed colour.
pub struct Pen {
    pub row: u32,
    pub radius: f32,
    pub color: u32,
}
