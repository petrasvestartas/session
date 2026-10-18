use session_rust::intersection::{line_line_parameters, line_plane};
use session_rust::{Line, Plane, Point, Vector, Xform};

pub const ARM: f64 = 96.0;

pub const BALL_AT: f64 = ARM * 0.5;

const ARROW_AT: f64 = ARM * 0.9;

const GRAB: f64 = 8.0;

pub const HUB: f64 = 6.0;

const SCALE_SOFTENING: f64 = 0.5;

pub(crate) const MIN_SCALE: f64 = 0.01;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Handle {
    Translate(Axis),
    Rotate(Axis),
    Scale(Axis),
    ScaleUniform,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn unit(self) -> Vector {
        match self {
            Axis::X => Vector::new(1.0, 0.0, 0.0),
            Axis::Y => Vector::new(0.0, 1.0, 0.0),
            Axis::Z => Vector::new(0.0, 0.0, 1.0),
        }
    }

    fn others(self) -> (Vector, Vector) {
        match self {
            Axis::X => (Axis::Y.unit(), Axis::Z.unit()),
            Axis::Y => (Axis::Z.unit(), Axis::X.unit()),
            Axis::Z => (Axis::X.unit(), Axis::Y.unit()),
        }
    }
}

impl Handle {
    pub fn labels(self) -> (&'static str, &'static str, &'static str) {
        let name = |a: Axis| match a {
            Axis::X => "X",
            Axis::Y => "Y",
            Axis::Z => "Z",
        };

        match self {
            Handle::Translate(a) => ("Move", name(a), "mm"),
            Handle::Rotate(a) => ("Rotate", name(a), "deg"),
            Handle::Scale(a) => ("Scale", name(a), "factor"),
            Handle::ScaleUniform => ("Scale", "", "factor"),
        }
    }

    pub fn title(self) -> String {
        let (verb, axis, _) = self.labels();

        if axis.is_empty() {
            verb.to_string()
        } else {
            format!("{verb} {axis}")
        }
    }
}

pub fn typed_value(handle: Handle, text: &str) -> Result<Option<f64>, String> {
    let text = text.trim();

    if text.is_empty() {
        return Ok(None);
    }

    let value = text
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())
        .ok_or_else(|| format!("`{text}` is not a number"))?;

    match handle {
        Handle::Scale(_) | Handle::ScaleUniform if value <= 0.0 => {
            Err("scale wants a factor above zero".into())
        }
        Handle::Scale(_) | Handle::ScaleUniform => Ok((value != 1.0).then_some(value)),
        Handle::Translate(_) | Handle::Rotate(_) => Ok((value != 0.0).then_some(value)),
    }
}

