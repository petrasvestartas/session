// --8<-- [start:step-30]
/// Pen width to a radius: negative = pixels, 0 = default pen.
pub fn encode_width(w: f64) -> f32 {
    if w.is_finite() && w > 0.0 && (w - 1.0).abs() > 1e-9 {
        -(w as f32) * 0.5
        // --8<-- [end:step-30]
    } else {
        0.0
    }
}

/// 0.0..=1.0 to 0..=255; the + 0.5 rounds instead of truncating.
fn quant8(v: f32) -> u32 {
    ((v.clamp(0.0, 1.0) * 255.0 + 0.5) as u32) & 0xff
}

/// Red in the lowest byte: the order WGSL `unpack4x8unorm` reads back.
pub fn pack_rgba(c: [f32; 4]) -> u32 {
    quant8(c[0]) | quant8(c[1]) << 8 | quant8(c[2]) << 16 | quant8(c[3]) << 24
}

/// -1 or +1, never 0, so a coordinate of exactly 0 still folds to one side.
fn sign_not_zero(v: f64) -> f64 {
    if v < 0.0 { -1.0 } else { 1.0 }
}

/// -1.0..=1.0 to -127..=127, kept in the low 8 bits.
fn quant_snorm8(v: f64) -> u32 {
    (((v.clamp(-1.0, 1.0) * 127.0).round() as i32) as u32) & 0xff
}

/// The 16-bit code that oct16_decode in normals.wgsl unpacks; the round trip is good to about a degree.
pub fn oct16(n: &[f64; 3]) -> Option<u32> {
    let l = n[0].abs() + n[1].abs() + n[2].abs();

    if l.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater) {
        return None; // zero or NaN vector
    }

    let (mut x, mut y) = (n[0] / l, n[1] / l); // now |x| + |y| + |z| = 1: a point on an octahedron

    if n[2] < 0.0 {
        // z < 0: fold into the square's corners, so x and y alone still tell the halves apart
        let (ax, ay) = (x.abs(), y.abs());
        (x, y) = ((1.0 - ay) * sign_not_zero(x), (1.0 - ax) * sign_not_zero(y));
    }

    Some(quant_snorm8(x) | quant_snorm8(y) << 8)
}

/// 0xAABBGGRR: alpha 255, colour 0.
pub const BLACK: u32 = 0xff00_0000;

/// No normals known: the shader draws the edge from every side.
pub const FACING_UNKNOWN: u32 = u32::MAX;

/// The normals of the two faces beside an edge; the shader hides the edge when both face away.
pub fn pack_facing(n0: Option<&[f64; 3]>, n1: Option<&[f64; 3]>) -> u32 {
    let pair = match (n0, n1) {
        (Some(a), Some(b)) => (oct16(a), oct16(b)),
        (Some(a), None) | (None, Some(a)) => (oct16(a), oct16(a)), // border edge: its one face twice
        _ => (None, None),
    };

    match pair {
        (Some(a), Some(b)) => {
            let v = a | b << 16;

            if v == FACING_UNKNOWN { v ^ 1 } else { v } // flip one bit so real normals never read as "unknown"
        }
        _ => FACING_UNKNOWN,
    }
}

/// Row, width and colour shared by every segment of one curve or edge set.
pub struct Pen {
    pub row: u32,
    pub radius: f32,
    pub color: u32,
}
