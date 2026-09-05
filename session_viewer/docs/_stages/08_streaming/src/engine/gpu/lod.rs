//! The level-of-detail walk over one cloud's octree: which node ranges to draw this frame,
//! given how wide each node's point spacing projects on screen. Pure CPU; the point lane
//! turns the ranges into records.

use crate::math::mat_scale;
use super::cloud::{Cloud, LodNode};

/// Clouds smaller than this draw WHOLE whatever the LOD cutoff says: nothing to save, and a
/// node drawn at its own coarser spacing is fatter than the whole cloud.
const LOD_MIN_POINTS: u32 = 2_000_000;

/// A point range one record covers, cloud-local: rows, spacing, and whether it is an octree
/// NODE (whose spacing is the real pitch between its points, so the radius has a coverage floor).
pub struct Range {
    pub first: u32,
    pub count: u32,
    pub spacing: f32,
    pub tile: bool,
}

/// One visited octree node during the walk: its range and its parent's slot, so the finest
/// spacing found below a node can travel back up to it.
struct Visit {
    first: u32,
    count: u32,
    spacing: f32,
    parent: usize,
}

/// What the walk needs from the frame: the eye, the projection, the viewport height, the
/// cutoff in pixels, and the lane's node table.
pub struct Projection<'a> {
    pub eye: [f32; 3],
    /// Ortho half-height in world mm; 0 in perspective.
    pub ortho_h: f32,
    pub height_px: u32,
    pub lod_px: f32,
    pub nodes: &'a [LodNode],
}

/// The walk's scratch and its output, kept between frames so nothing is reallocated.
#[derive(Default)]
pub struct LodWalk {
    pub ranges: Vec<Range>,
    stack: Vec<(usize, usize)>,
    visits: Vec<Visit>,
}

impl LodWalk {
    /// The ranges one cloud contributes: the whole resident prefix, or the octree nodes whose
    /// spacing still projects wider than `lod_px` pixels (each node OWNS its subsample, so
    /// descending only adds detail), every node sized by the finest spacing selected beneath
    /// it and clipped to the points resident so far.
    pub fn select(&mut self, p: &Projection, c: &Cloud, model: &[f32; 16]) {
        self.ranges.clear();
        if c.node_count == 0 || p.lod_px <= 0.0 || c.resident < LOD_MIN_POINTS {
            self.ranges.push(Range { first: 0, count: c.resident, spacing: c.spacing, tile: false });
            return;
        }

        let base = c.node_first as usize;
        let scale = mat_scale(model);
        self.stack.clear();
        self.visits.clear();
        self.stack.push((0, usize::MAX));
        while let Some((n, parent)) = self.stack.pop() {
            let Some(node) = p.nodes.get(base + n) else { continue };
            if node.first >= c.resident {
                continue;
            }
            let count = node.count.min(c.resident - node.first);
            let slot = self.visits.len();
            self.visits.push(Visit { first: node.first, count, spacing: node.spacing, parent });
            if projected_spacing(p, node, model, scale) > p.lod_px as f64 {
                for &child in &node.children {
                    if child >= 0 {
                        self.stack.push((child as usize, slot));
                    }
                }
            }
        }

        for i in (0..self.visits.len()).rev() {
            let (fine, parent) = (self.visits[i].spacing, self.visits[i].parent);
            if parent != usize::MAX && fine < self.visits[parent].spacing {
                self.visits[parent].spacing = fine;
            }
        }
        for v in &self.visits {
            if v.count > 0 {
                self.ranges.push(Range { first: v.first, count: v.count, spacing: v.spacing, tile: true });
            }
        }
    }
}

/// How wide a node's spacing projects on screen, in pixels. Everything in metres: the
/// spacing through the placement scale, the eye distance, and the ortho half-height.
fn projected_spacing(p: &Projection, node: &LodNode, model: &[f32; 16], scale: f64) -> f64 {
    let world = node.spacing as f64 * scale * 0.001;
    let c = node.center;
    let wx = (model[0] * c[0] + model[4] * c[1] + model[8] * c[2] + model[12]) as f64;
    let wy = (model[1] * c[0] + model[5] * c[1] + model[9] * c[2] + model[13]) as f64;
    let wz = (model[2] * c[0] + model[6] * c[1] + model[10] * c[2] + model[14]) as f64;
    let e = p.eye;
    let dist = ((wx - e[0] as f64).powi(2) + (wy - e[1] as f64).powi(2) + (wz - e[2] as f64).powi(2)).sqrt().max(1.0e-6) * 0.001;
    let frac = if p.ortho_h > 0.0 { world / (2.0 * p.ortho_h as f64 * 0.001) } else { world * 1.7320508 * 0.5 / dist };
    frac * p.height_px as f64
}

/// The radius factor `k` of a range: world radius = spacing x scale x px / 6 (a manifest
/// size of 6 is a full spacing), floored to spacing / 2 for an octree node so discs on a
/// pitch of `spacing` still tile; then folded with the projection so the shader divides once.
/// `ortho_h` is in world mm, the radius in metres.
pub fn radius_factor(r: &Range, px: f32, scale: f64, ortho_h: f32) -> f32 {
    let mut world_r = (r.spacing as f64).max(1.0e-9) * scale * 0.001 * (px as f64) / 6.0;
    if r.tile {
        world_r = world_r.max(r.spacing as f64 * scale * 0.001 * 0.5);
    }
    let k = if ortho_h > 0.0 { world_r / (2.0 * ortho_h as f64 * 0.001) } else { world_r * 1.7320508 * 0.5 };
    k as f32
}
