//! A walked point cloud into the splat lane: three flat point tables, the LOD octree's nodes
//! and ONE draw record per cloud. The per-file point size rides the object row's spacing
//! column. Writes `CloudRows` only.

use session_rust::{PointCloud, SpatialOctree};
use crate::engine::gpu::cloud::CloudRows;
use crate::engine::gpu::{CloudDraw, LodNode};
use super::{Row, WalkCx};
use super::encode::oct16;

/// The points, the nodes, then the draw record. `first` is ABSOLUTE, counted from the start of
/// the scene: the GPU table is cumulative while `c.pos` is only this upload's delta.
pub fn walk_cloud(c: &mut CloudRows, pc: &PointCloud, cx: &WalkCx) -> Row {
    let first = cx.cloud_base + (c.pos.len() / 3) as u32;
    let node_first = c.nodes.len() as u32;
    push_points(c, pc);
    let node_count = c.nodes.len() as u32 - node_first;
    c.draws.push(CloudDraw { first, count: pc.len() as u32, instance: cx.row, spacing: cloud_spacing(pc), node_first, node_count });
    let px = if cx.cloud_px > 0.0 { cx.cloud_px } else { pc.point_size as f32 };
    Row::point_size_px(px)
}

/// The point tables and the octree nodes, straight from the kernel's flat arrays rather than
/// get_point/get_color (no per-point allocation).
fn push_points(rows: &mut CloudRows, pc: &PointCloud) {
    let coords = pc.coords();
    let colors = pc.colors();
    let normals = pc.normals();
    let n = pc.len();
    rows.pos.reserve(n*3);
    rows.col.reserve(n);
    rows.nrm.reserve(n);
    // The LOD octree, built ONCE and read twice: `order()` is the permutation that makes every
    // node's points contiguous, the node table is this walk's second output. Root accept
    // spacing = the cube over 64; leaves absorb below 8192 points, so a shallow cloud is one node.
    let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    for i in 0..n {
        for k in 0..3 {
            lo[k] = lo[k].min(coords[i * 3 + k]);
            hi[k] = hi[k].max(coords[i * 3 + k]);
        }
    }
    let size = (hi[0] - lo[0]).max(hi[1] - lo[1]).max(hi[2] - lo[2]).max(1.0e-9);
    let tree = SpatialOctree::from_coords(coords, size / 64.0, 8192);
    // `first` and `children` are RELATIVE to this cloud's own first point / node slice: the
    // record builder adds the draw's base, so a re-upload at another offset rewrites nothing.
    for ni in 0..tree.node_count() {
        let (center, sz) = tree.node_cube(ni);
        let (f, count) = tree.node_range(ni);
        // `children` hands back only the octants that exist, so the empty slots stay -1 and
        // the record walk skips them; which octant a child was, the screen-error test never asks.
        let mut children = [-1i32; 8];
        for (slot, &ch) in tree.children(ni).iter().enumerate() {
            children[slot] = ch as i32;
        }
        rows.nodes.push(LodNode {
            center: [center[0] as f32, center[1] as f32, center[2] as f32],
            size: sz as f32,
            spacing: tree.node_spacing(ni) as f32,
            first: f as u32,
            count: count as u32,
            children,
        });
    }
    for &i in tree.order(){
        rows.pos.push(coords[i*3] as f32);
        rows.pos.push(coords[i*3+1] as f32);
        rows.pos.push(coords[i*3+2] as f32);

        // oct16 normal; all-ones = none (a scan without them still pays the 4 B, but the
        // shading branch stays uniform per cloud). Three f64s, not a kernel `Vector`: building
        // one per point was two heap allocations each - 27 million on the 13.8 M-point scan.
        rows.nrm.push(if i*3 + 2 < normals.len() {
            oct16(&[normals[i*3], normals[i*3+1], normals[i*3+2]]).unwrap_or(u32::MAX)
        } else {
            u32::MAX
        });
        let c = i * 4;

        // The colour is 8-bit at the source (proto 0-255): pack it back to the four bytes it is.
        rows.col.push(if c + 3 < colors.len() {
            (colors[c] as u32 & 255) | (colors[c + 1] as u32 & 255) << 8 | (colors[c+2] as u32 & 255) << 16 | (colors[c + 3] as u32 & 255) << 24
        } else {
            0xff00_0000
        });
    }
}

/// Median distance between consecutive points (world units): a scanner emits angular
/// neighbours in order, so successive points are usually adjacent on the surface. Potree gets
/// the same number from its octree; we sample it. Drives the attenuated splat radius.
pub fn cloud_spacing(pc: &PointCloud) -> f32 {
    let c = pc.coords();
    let n = pc.len();
    if n < 2 {
        return 20.0;
    }
    let step = (n / 1024).max(1);
    let mut d: Vec<f64> = Vec::with_capacity(1024);
    let mut i = 0;
    while i + 1 < n {
        let  (a, b) = (i * 3, (i + 1) * 3);
        let dd = (c[a] - c[b]).powi(2) + (c[a + 1] - c[b + 1]).powi(2) + (c[a + 2] - c[b + 2]).powi(2);
        if dd> 0.0 {
            d.push(dd.sqrt());
        }
        i += step;
    }
    if d.is_empty() {
        return 20.0;
    }
    d.sort_by(|x, y| x.partial_cmp(y).unwrap());
    d[d.len() / 2] as f32
}
