use crate::text::{TextLabel,TextPlacement};
use session_rust::{Point,Vector};
const FONT_SIZE:f32=18.;const LINE_HEIGHT:f32=26.;
pub fn label(text: &str, point: &Point, right: [f64; 3], up: [f64; 3], height: f64) -> TextLabel {
    let lift = height * f64::from(LINE_HEIGHT) / f64::from(FONT_SIZE); // the line box above the point
    TextLabel {
        id: 0,
        object: None,
        text: text.into(),
        font_size: FONT_SIZE,
        line_height: LINE_HEIGHT,
        color: [255; 4],
        placement: TextPlacement::WorldPlane {
            world: [0, 1, 2].map(|axis| point[axis] + up[axis] * lift),
            right,
            up,
            world_height: height,
        },
        clip: None,
    }
}

/// Of the plane axes `a` and `b`, the one along the screen's right and the other toward its top.
pub fn reading_axes(a: &Vector, b: &Vector, right: &Vector, up: &Vector) -> ([f64; 3], [f64; 3]) {
    let (along, across) = if a.dot(right).abs() >= b.dot(right).abs() {
        (a, b)
    } else {
        (b, a)
    };
    let sign = |axis: &Vector, toward: &Vector| {
        let flip = if axis.dot(toward) < 0.0 { -1.0 } else { 1.0 };
        [axis[0] * flip, axis[1] * flip, axis[2] * flip]
    };
    (sign(along, right), sign(across, up))
}

/// `value` rounded down to 1, 2 or 5 times a power of ten; 1 when it is not a positive number.
pub fn nice(value: f64) -> f64 {
    if !(value > 0.0 && value.is_finite()) {
        return 1.0;
    }

    let power = 10f64.powf(value.log10().floor());
    let step = match value / power {
        m if m >= 5.0 => 5.0,
        m if m >= 2.0 => 2.0,
        _ => 1.0,
    };
    step * power
}


