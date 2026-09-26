use super::cloud::{Cloud, LodNode};
use session_rust::Xform;

/// Below 2 million points a cloud is cheap enough to draw in full.
const LOD_MIN_POINTS: u32 = 2_000_000;

/// One run of points to draw: a distant cloud draws fewer, wider-spaced points, since the rest would land on the same pixel.
pub struct Range {
    pub first: u32, // index into the cloud's points
    pub count: u32,
    pub spacing: f32, // gap between neighbouring points, mm
    pub tile: bool, // true = an octree node, not the whole cloud
}

/// An octree splits space into eight boxes again and again, so a whole box can be skipped or coarsened in one test.
struct Visit {
    first: u32,
    count: u32,
    spacing: f32, // finest spacing found in or below it
    parent: usize, // index of the parent visit; usize::MAX for the root
}

/// Camera facts the walk needs.
pub struct Projection<'a> {
    pub eye: [f32; 3],
    pub ortho_h: f32, // world mm; 0 = perspective
    pub height_px: u32,
    pub lod_px: f32, // split a node while its spacing is wider than this
    pub nodes: &'a [LodNode], // every cloud's octree nodes
}

/// Kept between frames, so the lists reuse their memory instead of allocating every frame.
#[derive(Default)]
pub struct LodWalk {
    pub ranges: Vec<Range>, // result: the runs to draw
    stack: Vec<(usize, usize)>, // nodes still to visit, with parent
    visits: Vec<Visit>,
}

impl LodWalk {
    /// Pick which runs of one cloud to draw at this camera.
    pub fn select(&mut self, p: &Projection, c: &Cloud, model: &[f32; 16]) {
        self.ranges.clear();

        // no octree or small cloud: draw everything
        if c.node_count == 0 || p.lod_px <= 0.0 || c.resident < LOD_MIN_POINTS {
            self.ranges.push(Range {
                first: 0,
                count: c.resident,
                spacing: c.spacing,
                tile: false,
            });
            return;
        }

        let base = c.node_first as usize;
        // object scale, applied to spacings
        let scale = Xform::from_matrix(model.map(f64::from)).uniform_scale();
        self.stack.clear();
        self.visits.clear();
        // the root has no parent
        self.stack.push((0, usize::MAX));

        while let Some((n, parent)) = self.stack.pop() { // depth-first, with a stack instead of recursion
            let Some(node) = p.nodes.get(base + n) else {
                continue;
            };

            if node.first >= c.resident { // not streamed in yet
                continue;
            }

            // only points uploaded so far
            let count = node.count.min(c.resident - node.first);
            let slot = self.visits.len();
            self.visits.push(Visit {
                first: node.first,
                count,
                spacing: node.spacing,
                parent,
            });

            // still too coarse on screen: visit the children
            if projected_spacing(p, node, model, scale) > p.lod_px as f64 {
                for &child in &node.children {
                    if child >= 0 { // -1 = no child in that eighth
                        self.stack.push((child as usize, slot));
                    }
                }
            }
        }

        // pass the finest spacing up; in reverse, because children sit after their parent
        for i in (0..self.visits.len()).rev() {
            let (fine, parent) = (self.visits[i].spacing, self.visits[i].parent);

            if parent != usize::MAX && fine < self.visits[parent].spacing {
                self.visits[parent].spacing = fine;
            }
        }

        // every visited node becomes a run
        for v in &self.visits {
            if v.count > 0 {
                self.ranges.push(Range {
                    first: v.first,
                    count: v.count,
                    spacing: v.spacing,
                    tile: true,
                });
            }
        }
    }
}

/// Pixels between two neighbouring points of the node on screen.
fn projected_spacing(p: &Projection, node: &LodNode, model: &[f32; 16], scale: f64) -> f64 {
    // spacing in metres
    let world = node.spacing as f64 * scale * 0.001;
    let c = node.center;
    // node center in world space
    let wx = (model[0] * c[0] + model[4] * c[1] + model[8] * c[2] + model[12]) as f64;
    let wy = (model[1] * c[0] + model[5] * c[1] + model[9] * c[2] + model[13]) as f64;
    let wz = (model[2] * c[0] + model[6] * c[1] + model[10] * c[2] + model[14]) as f64;
    let e = p.eye;
    // distance from the eye, metres
    let dist =
        ((wx - e[0] as f64).powi(2) + (wy - e[1] as f64).powi(2) + (wz - e[2] as f64).powi(2))
            .sqrt()
            .max(1.0e-6)
            * 0.001;
    // spacing as a fraction of the view height
    let frac = if p.ortho_h > 0.0 {
        world / (2.0 * p.ortho_h as f64 * 0.001)
    } else {
        world * 1.7320508 * 0.5 / dist // 1.732 = 1 / tan(30°), half the 60° field of view
    };
    frac * p.height_px as f64
}

/// Disc radius of a run, folded so the shader divides once.
pub fn radius_factor(r: &Range, px: f32, scale: f64, ortho_h: f32) -> f32 {
    // radius in metres; a size of 6 spans one spacing
    let mut world_r = (r.spacing as f64).max(1.0e-9) * scale * 0.001 * (px as f64) / 6.0;

    // an octree node needs discs at least half a spacing wide
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
