//! Row encodings shared by every producer: pen widths to radii, colours to RGBA8, normals to
//! oct16 and the packed `facing` word the ink shaders test. Pure functions on numbers.

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

/// `signum` that never returns 0, so the -Z pole does not fold onto the +Z code.
fn sign_not_zero(v: f64) -> f64 {
    if v < 0.0 { -1.0 } else { 1.0 }
}

/// One octahedral coordinate to a signed byte.
fn quant_snorm8(v: f64) -> u32 {
    (((v.clamp(-1.0, 1.0) * 127.0).round() as i32) as u32) & 0xff
}

/// A unit vector in 16 bits, octahedral (~1.4 deg of error, used for the SIGN of a dot).
pub fn oct16(n: &[f64; 3]) -> Option<u32> {
    let l = n[0].abs() + n[1].abs() + n[2].abs();
    if l.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater) {
        return None;
    }
    let (mut x, mut y) = (n[0] / l, n[1] / l);
    if n[2] < 0.0 {
        let (ax, ay) = (x.abs(), y.abs());
        (x, y) = ((1.0 - ay) * sign_not_zero(x), (1.0 - ax) * sign_not_zero(y));
    }
    Some(quant_snorm8(x) | quant_snorm8(y) << 8)
}

/// Opaque black, packed: the wireframe's default pen.
pub const BLACK: u32 = 0xff00_0000;

/// `facing` meaning "no adjacency, always draw". All-ones: (0, 0) is the honest code for +Z.
pub const FACING_UNKNOWN: u32 = u32::MAX;

/// The two faces an edge belongs to, packed into one word; a lone face is duplicated.
pub fn pack_facing(n0: Option<&[f64; 3]>, n1: Option<&[f64; 3]>) -> u32 {
    let pair = match (n0, n1) {
        (Some(a), Some(b)) => (oct16(a), oct16(b)),
        (Some(a), None) | (None, Some(a)) => (oct16(a), oct16(a)),
        _ => (None, None),
    };
    match pair {
        (Some(a), Some(b)) => {
            let v = a | b << 16;
            if v == FACING_UNKNOWN { v ^ 1 } else { v }
        }
        _ => FACING_UNKNOWN,
    }
}

/// One pen for a run of segments: the object row, the encoded radius and the packed colour.
pub struct Pen {
    pub row: u32,
    pub radius: f32,
    pub color: u32,
    /// `FACING_UNKNOWN`, or the host face's normal twice for an outline lying on a plate.
    pub facing: u32,
}
