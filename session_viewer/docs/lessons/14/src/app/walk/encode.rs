/// Pen width to a radius: negative = pixels, 0 = default pen.
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

/// Four colour bytes in one word, red lowest.
pub fn pack_rgba(c: [f32; 4]) -> u32 {
    quant8(c[0]) | quant8(c[1]) << 8 | quant8(c[2]) << 16 | quant8(c[3]) << 24
}

/// Sign of `v`, +1 for zero.
fn sign_not_zero(v: f64) -> f64 {
    if v < 0.0 { -1.0 } else { 1.0 }
}

/// One octahedral coordinate to a signed byte.
fn quant_snorm8(v: f64) -> u32 {
    (((v.clamp(-1.0, 1.0) * 127.0).round() as i32) as u32) & 0xff
}

/// A direction packed into 16 bits.
pub fn oct16(n: &[f64; 3]) -> Option<u32> {
    let l = n[0].abs() + n[1].abs() + n[2].abs();

    if l.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater) {
        return None; // zero or NaN vector
    }

    let (mut x, mut y) = (n[0] / l, n[1] / l);

    if n[2] < 0.0 {
        // fold the lower half onto the square
        let (ax, ay) = (x.abs(), y.abs());
        (x, y) = ((1.0 - ay) * sign_not_zero(x), (1.0 - ax) * sign_not_zero(y));
    }

    Some(quant_snorm8(x) | quant_snorm8(y) << 8)
}

/// Opaque black, packed.
pub const BLACK: u32 = 0xff00_0000;

/// Facing code for "always draw".
pub const FACING_UNKNOWN: u32 = u32::MAX;

/// Two face normals in one word; one face is used twice.
pub fn pack_facing(n0: Option<&[f64; 3]>, n1: Option<&[f64; 3]>) -> u32 {
    let pair = match (n0, n1) {
        (Some(a), Some(b)) => (oct16(a), oct16(b)),
        (Some(a), None) | (None, Some(a)) => (oct16(a), oct16(a)),
        _ => (None, None),
    };

    match pair {
        (Some(a), Some(b)) => {
            let v = a | b << 16;

            if v == FACING_UNKNOWN { v ^ 1 } else { v } // never the reserved code
        }
        _ => FACING_UNKNOWN,
    }
}

/// One pen for a run of segments.
pub struct Pen {
    pub row: u32,
    pub radius: f32, // half width, negative = pixels
    pub color: u32, // packed RGBA
}
