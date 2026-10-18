use crate::{primitive_options as options,drawing_coords as coords};
use session_rust::{Geometry,NurbsCurve,Point,Vector,Xform};
fn typed_number(word:&str)->Option<f64>{word.parse::<f64>().ok().filter(|v|v.is_finite())}
/// One question a gathering command asks; Curves steps come first.
#[derive(Clone, Copy, Debug)]
pub enum Step {
    Curves {
        prompt: &'static str,
        min: usize,
        max: usize,
    }, // curves clicked in order; reaching max or Enter moves on
    Point(&'static str), // a click, snap or typed x,y,z
    Number {
        prompt: &'static str,
        default: f64,
        low: f64,
        high: f64,
    }, // low < value <= high; Enter takes the default
    Distance(&'static str), // along the recipe's axis: a click, a signed number or a typed vector
}

/// An answer to a Point, Number or Distance step.
#[derive(Clone, Debug)]
pub enum Answer {
    Point(Point),   // a location
    Number(f64),    // a value, or a distance along the axis
    Vector(Vector), // a Distance given as a vector
}

/// What a command gathered, in the world.
#[derive(Clone)]
pub struct Input {
    pub curves: Vec<Vec<Picked>>, // per Curves step, in pick order
    pub answers: Vec<Answer>,     // per other step
    pub choice: &'static str,     // the chosen option, empty when none
    pub normal: Vector,           // the construction plane the camera faces
}

/// A picked curve in world coordinates.
#[derive(Clone, Debug)]
pub struct Picked {
    pub curve: NurbsCurve,           // clamped, domain 0..1
    pub corners: Option<Vec<Point>>, // a line's or polyline's points
}

pub const SAMPLES: usize = 48; // points per curve for normals and previews
