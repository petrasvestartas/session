//! Point clouds into the cloud lane: a walked kernel `PointCloud` (points, optional normals,
//! the octree it carries, one draw).

use session_rust::PointCloud;
use crate::engine::gpu::cloud::CloudRows;
use crate::engine::gpu::{CloudDraw, LodNode, NO_NORMALS};
use crate::math::Aabb;
use super::{Row, WalkCx};
use super::encode::oct16;

/// Spacing reported when a cloud is too small to measure.
const DEFAULT_SPACING: f32 = 20.0;

/// The points, the nodes, then the draw record; the per-file point size rides the row.
pub fn walk_cloud(c: &mut CloudRows, pc: &PointCloud, cx: &WalkCx) -> Row {
    let first = c.point_count();
    let node_first = c.nodes.len() as u32;
    let nrm_first = if pc.normals().len() >= pc.len() * 3 { c.nrm.len() as u32 } else { NO_NORMALS };
    let bounds = push_points(c, pc);
    push_nodes(c, pc);
    c.draws.push(CloudDraw {
        instance: cx.row,
        count: pc.len() as u32,
        first,
        spacing: cloud_spacing(pc, &bounds),
        node_first,
        node_count: pc.lod_node_count() as u32,
        nrm_first,
    });
    let px = if cx.cloud_px > 0.0 { cx.cloud_px } else { pc.point_size as f32 };
    Row { bounds, spacing: px, flags: 0, faces: false, thickness: bounds.thinnest() }
}

/// Positions, colours and (when every point has one) normals, from the kernel's flat arrays.
fn push_points(rows: &mut CloudRows, pc: &PointCloud) -> Aabb {
    let coords = pc.coords();
    let colors = pc.colors();
    let normals = pc.normals();
    let n = pc.len();
    let has_normals = normals.len() >= n * 3;
    rows.pos.reserve(n * 3);
    rows.col.reserve(n);
    let mut bounds = Aabb::empty();
    for i in 0..n {
        let p = [coords[i * 3] as f32, coords[i * 3 + 1] as f32, coords[i * 3 + 2] as f32];
        bounds.grow(p);
        rows.pos.extend_from_slice(&p);
        let c = i * 4;
        rows.col.push(if c + 3 < colors.len() { pack_color(&colors[c..c + 4]) } else { 0xff00_0000 });
        if has_normals {
            rows.nrm.push(oct16(&[normals[i * 3], normals[i * 3 + 1], normals[i * 3 + 2]]).unwrap_or(0));
        }
    }
    bounds
}

/// The octree nodes the file carries, relative to this cloud's own rows and node slice.
fn push_nodes(rows: &mut CloudRows, pc: &PointCloud) {
    for k in 0..pc.lod_node_count() {
        let (c, size) = pc.lod_cube(k);
        let (nf, nc) = pc.lod_range(k);
        let mut children = [-1i32; 8];
        for (slot, v) in pc.lod_children(k).into_iter().enumerate().take(8) {
            children[slot] = v;
        }
        rows.nodes.push(LodNode {
            center: [c[0] as f32, c[1] as f32, c[2] as f32],
            size: size as f32,
            spacing: pc.lod_spacing(k) as f32,
            first: nf as u32,
            count: nc as u32,
            children,
        });
    }
}

/// Four 0-255 channels to one RGBA8 word.
fn pack_color(c: &[i32]) -> u32 {
    (c[0] as u32 & 255) | (c[1] as u32 & 255) << 8 | (c[2] as u32 & 255) << 16 | (c[3] as u32 & 255) << 24
}

/// The cloud's point spacing from its density: `sqrt(area / n)` over the two longest box
/// edges - a scan samples a surface. Invariant to point order.
fn cloud_spacing(pc: &PointCloud, bounds: &Aabb) -> f32 {
    let n = pc.len();
    if n < 2 || !bounds.is_finite() {
        return DEFAULT_SPACING;
    }
    let mut e = [bounds.max[0] - bounds.min[0], bounds.max[1] - bounds.min[1], bounds.max[2] - bounds.min[2]];
    e.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let area = e[0] as f64 * e[1] as f64;
    if area <= 0.0 || !area.is_finite() {
        return DEFAULT_SPACING;
    }
    (area / n as f64).sqrt() as f32
}
