//! Row encodings shared by every producer: pen widths to radii, colours to RGBA8, normals to
//! oct16 and the packed `facing` word the ink shaders test. Pure functions on numbers - no
//! kernel type, no table.

/// An authored width (kernel millimetres) as the world-mm RADIUS the shaders project; the
/// untouched 1.0 default (and 0 / non-finite) is 0.0 = the screen-constant pen. A negative
/// value would mean "multiply the global pen", which is how a 30 mm polyline once drew 120 px wide.
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

/// `signum` that never returns 0: `f64::signum(0.0)` is 0.0, which folds (0,0,-1) onto the
/// code for (0,0,+1) - on an axis-aligned box that is the top and bottom faces, and the
/// collision landed on the "no adjacency" sentinel, so the facing test silently did nothing.
fn sign_not_zero(v: f64) -> f64 {
    if v < 0.0 { -1.0 } else { 1.0 }
}

/// One octahedral coordinate to a signed byte.
fn quant_snorm8(v: f64) -> u32 {
    (((v.clamp(-1.0, 1.0) * 127.0).round() as i32) as u32) & 0xff
}

/// A unit vector in 16 bits, octahedral: project onto the octahedron, fold the lower hemisphere
/// out across the diagonals, and store the two coordinates as signed bytes. ~1.4 degrees of error,
/// which is generous for a value only ever used for the SIGN of a dot product.
pub fn oct16(n: &[f64; 3]) -> Option<u32> {
    let l = n[0].abs() + n[1].abs() + n[2].abs();
    if !(l > 0.0) {
        return None;
    }
    let (mut x, mut y) = (n[0] / l, n[1] / l);
    if n[2] < 0.0 {
        let (ax, ay) = (x.abs(), y.abs());
        (x, y) = ((1.0 - ay) * sign_not_zero(x), (1.0 - ax) * sign_not_zero(y));
    }
    Some(quant_snorm8(x) | quant_snorm8(y) << 8)
}

/// Opaque black, packed. The wireframe's default pen, and what a dense mesh's edges draw as.
pub const BLACK: u32 = 0xff00_0000;

/// `facing` value meaning "this edge has no adjacency, always draw it". It cannot be 0, the
/// honest encoding of +Z; all four corners of the octahedral square collapse onto -Z, so the
/// all-ones word is a value the encoder can produce but never needs - the one safe sentinel.
pub const FACING_UNKNOWN: u32 = u32::MAX;

/// The two faces an edge belongs to, packed into one word for the shader's facing test;
/// `FACING_UNKNOWN` when neither is known.
pub fn pack_facing(n0: Option<&[f64; 3]>, n1: Option<&[f64; 3]>) -> u32 {
    let pair = match (n0, n1) {
        (Some(a), Some(b)) => (oct16(a), oct16(b)),
        // A naked edge is visible whenever its single face is, so duplicating the one normal is
        // the correct answer and needs no special case in the shader.
        (Some(a), None) | (None, Some(a)) => (oct16(a), oct16(a)),
        _ => (None, None),
    };
    match pair {
        (Some(a), Some(b)) => {
            let v = a | b << 16;
            if v == FACING_UNKNOWN { FACING_UNKNOWN } else { v }
        }
        _ => FACING_UNKNOWN,
    }
}
