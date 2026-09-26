// --8<-- [start:cloud-walk]
use super::encode::oct16;
use super::{Row, WalkCx};
use crate::engine::gpu::cloud::CloudRows;
use crate::engine::gpu::{CloudDraw, LodNode, NO_NORMALS};
use session_rust::AABB;
use session_rust::PointCloud;

/// Spacing for a cloud too small to measure.
const DEFAULT_SPACING: f32 = 20.0;

/// A point cloud: points, octree nodes, one draw.
pub fn walk_cloud(c: &mut CloudRows, pc: &PointCloud, cx: &WalkCx) -> Row {
    let first = c.point_count(); // index of this cloud's first point
    let node_first = c.nodes.len() as u32; // index of this cloud's first node
    // normals only when every point has one
    let nrm_first = if pc.normals().len() >= pc.len() * 3 {
        c.nrm.len() as u32
    } else {
        NO_NORMALS
    };
    let bounds = push_points(c, pc);
    push_nodes(c, pc);
    c.draws.push(CloudDraw {
        instance: cx.row,
        from: 0,
        count: pc.len() as u32,
        first,
        spacing: cloud_spacing(pc, &bounds),
        node_first,
        node_count: pc.lod_node_count() as u32,
        nrm_first,
    });
    // file override wins over the cloud's own size
    let px = if cx.cloud_px > 0.0 {
        cx.cloud_px
    } else {
        pc.point_size as f32
    };
    Row {
        bounds,
        spacing: px,
        flags: 0,
        faces: false,
    }
}
// --8<-- [end:cloud-walk]

// --8<-- [start:cloud-copy]
/// Copy positions, colours and normals into the rows.
fn push_points(rows: &mut CloudRows, pc: &PointCloud) -> AABB {
    let coords = pc.coords();
    let colors = pc.colors();
    let normals = pc.normals();
    let n = pc.len();
    let has_normals = normals.len() >= n * 3;
    rows.pos.reserve(n * 3);
    rows.col.reserve(n);
    let mut bounds = AABB::empty();

    for i in 0..n {
        let p = [
            coords[i * 3] as f32,
            coords[i * 3 + 1] as f32,
            coords[i * 3 + 2] as f32,
        ];
        bounds.union_with_point(p[0] as f64, p[1] as f64, p[2] as f64);
        rows.pos.extend_from_slice(&p);
        let c = i * 4;
        rows.col.push(if c + 3 < colors.len() {
            pack_color(&colors[c..c + 4])
        } else {
            0xff00_0000 // black when missing
        });

        if has_normals {
            rows.nrm.push(
                oct16(&[normals[i * 3], normals[i * 3 + 1], normals[i * 3 + 2]]).unwrap_or(0),
            );
        }
    }

    bounds
}

// Octree node = a cube with the points inside it, split into up to 8 child cubes (lesson 04d).
/// Copy the cloud's octree nodes.
fn push_nodes(rows: &mut CloudRows, pc: &PointCloud) {
    for k in 0..pc.lod_node_count() {
        let (c, size) = pc.lod_cube(k); // node cube
        let (nf, nc) = pc.lod_range(k); // its points
        let mut children = [-1i32; 8]; // -1 = no child

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

/// Four 0-255 channels to one word.
fn pack_color(c: &[i32]) -> u32 {
    (c[0] as u32 & 255)
        | (c[1] as u32 & 255) << 8
        | (c[2] as u32 & 255) << 16
        | (c[3] as u32 & 255) << 24
}

/// Point spacing from the cloud's density.
fn cloud_spacing(pc: &PointCloud, bounds: &AABB) -> f32 {
    let n = pc.len();

    if n < 2 || !bounds.is_valid() {
        return DEFAULT_SPACING;
    }

    let mut e = [
        (2.0 * bounds.hx) as f32,
        (2.0 * bounds.hy) as f32,
        (2.0 * bounds.hz) as f32,
    ];
    e.sort_unstable_by(descending_extent);
    // points spread over the two longest sides: 1 000 000 points on 10 m x 10 m are 10 mm apart
    let area = e[0] as f64 * e[1] as f64; // two longest sides

    if area <= 0.0 || !area.is_finite() {
        return DEFAULT_SPACING;
    }

    (area / n as f64).sqrt() as f32
}

/// Largest first.
fn descending_extent(a: &f32, b: &f32) -> std::cmp::Ordering {
    b.partial_cmp(a).unwrap()
}
// --8<-- [end:cloud-copy]
