use crate::cloud::{Cloud, LodNode};
use session_rust::Xform;

const LOD_MIN_POINTS: u32 = 2_000_000;

pub struct Range {
    pub first: u32,   // first point in the cloud
    pub count: u32,   // points in the run
    pub spacing: f32, // distance between points
    pub tile: bool,   // true = an octree node, not the whole cloud
}

struct Visit {
    first: u32,    // first point in the cloud
    count: u32,    // points in the node
    spacing: f32,  // finest spacing found in or below it
    parent: usize, // index of the parent visit
}

pub struct Projection<'a> {
    pub eye: [f32; 3],        // camera position
    pub ortho_h: f32,         // ortho half-height, world mm; 0 = perspective
    pub height_px: u32,       // viewport height, px
    pub lod_px: f32,          // split a node while its spacing is wider than this
    pub nodes: &'a [LodNode], // every cloud's octree nodes
}

#[derive(Default)]
pub struct LodWalk {
    pub ranges: Vec<Range>,     // result: the runs to draw
    stack: Vec<(usize, usize)>, // nodes still to visit, with parent
    visits: Vec<Visit>,         // nodes visited
}

fn projected_spacing(p: &Projection, node: &LodNode, model: &[f32; 16], scale: f64) -> f64 {
    let world = node.spacing as f64 * scale * 0.001;
    let c = node.center;
    let wx = (model[0] * c[0] + model[4] * c[1] + model[8] * c[2] + model[12]) as f64;
    let wy = (model[1] * c[0] + model[5] * c[1] + model[9] * c[2] + model[13]) as f64;
    let wz = (model[2] * c[0] + model[6] * c[1] + model[10] * c[2] + model[14]) as f64;
    let e = p.eye;
    let dist =
        ((wx - e[0] as f64).powi(2) + (wy - e[1] as f64).powi(2) + (wz - e[2] as f64).powi(2))
            .sqrt()
            .max(1.0e-6)
            * 0.001;
    let frac = if p.ortho_h > 0.0 {
        world / (2.0 * p.ortho_h as f64 * 0.001)
    } else {
        world * 1.7320508 * 0.5 / dist
    };
    frac * p.height_px as f64
}

pub fn radius_factor(r: &Range, px: f32, scale: f64, ortho_h: f32) -> f32 {
    let mut world_r = (r.spacing as f64).max(1.0e-9) * scale * 0.001 * (px as f64) / 6.0;

    if r.tile {
        world_r = world_r.max(r.spacing as f64 * scale * 0.001 * 0.5);
    }

    let k = if ortho_h > 0.0 {
        world_r / (2.0 * ortho_h as f64 * 0.001)
    } else {
        world_r * 1.7320508 * 0.5
    };
    k as f32
}
