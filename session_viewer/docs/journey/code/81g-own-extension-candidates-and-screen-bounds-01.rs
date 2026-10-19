use crate::{
    extend_reach::{self, End, Reach},
    scene::{ObjectId, Scene},
    source_cut::{self, Cutter},
    source_edit::Owner,
};
use session_rust::{Geometry, Point, Xform};
use std::rc::Rc;
#[derive(Clone)]
pub struct Hover {
    pub owner: Owner,
    pub end: End,
    pub geometry: Geometry,
    pub tail: Vec<[f64; 3]>,
}
#[derive(Clone)]
pub struct Draft {
    pub before: Vec<ObjectId>,
    pub boundaries: Vec<Owner>,
    pub curves: bool,
    pub distance: Option<f64>,
    pub asking: bool,
    pub candidates: Vec<(Owner, Vec<[f64; 3]>, [[f64; 3]; 2])>,
    pub hover: Option<Hover>,
    pub error: String,
    pub cutters: Vec<(Owner, Cutter)>,
    pub meshes: Vec<(Owner, Rc<session_rust::Mesh>)>,
    pub last: Option<(ObjectId, End)>,
}
impl Draft {
}
fn apart(a: [f64; 3], b: [f64; 3]) -> f64 {
    (0..3).map(|i| (a[i] - b[i]).powi(2)).sum::<f64>().sqrt()
}
fn bounds(points: &[[f64; 3]]) -> [[f64; 3]; 2] {
    let mut bounds = crate::bounds::Bounds::point(points[0]);
    for p in points {
        bounds.include(*p)
    }
    [bounds.min, bounds.max]
}
fn boxed(
    screen: &crate::snap_math::Screen,
    bounds: &[[f64; 3]; 2],
    at: [f64; 2],
    aperture: f64,
) -> bool {
    let (mut min, mut max) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);
    for corner in 0..8 {
        let p = std::array::from_fn(|i| {
            if corner & (1 << i) == 0 {
                bounds[0][i]
            } else {
                bounds[1][i]
            }
        });
        let Some(p) = screen.point(p) else {
            return true;
        };
        for i in 0..2 {
            min[i] = min[i].min(p[i]);
            max[i] = max[i].max(p[i]);
        }
    }
    (0..2).all(|i| at[i] >= min[i] - aperture && at[i] <= max[i] + aperture)
}
