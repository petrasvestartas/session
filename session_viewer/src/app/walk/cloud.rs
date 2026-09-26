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

use crate::app::stream::CloudLod;

/// Raw point columns of one streamed slice.
pub struct StreamRows {
    pub positions: Vec<f32>, // three floats per point
    pub colors: Vec<u32>,    // packed RGBA per point
    pub normals: Vec<u32>,   // packed normal per point, empty = none
}

/// One slice of a streamed cloud and where it goes.
pub struct StreamSlice<'a> {
    pub rows: StreamRows,  // the points
    pub lod: &'a CloudLod, // the whole node table
    pub from: u32,         // first point index in the cloud
    pub to: u32,           // one past the last
    pub row: u32,          // object row
    pub point_px: f32,     // file's point size
}

/// Append one streamed slice; return its box.
pub fn walk_stream_slice(c: &mut CloudRows, s: &StreamSlice) -> AABB {
    let first = c.point_count();
    let node_first = c.nodes.len() as u32;
    let mut node_count = 0u32;

    // the first slice brings the node table
    if s.from == 0 {
        for k in 0..s.lod.len() {
            c.nodes.push(lod_node(s.lod, k));
        }

        node_count = s.lod.len() as u32;
    }

    let mut bounds = AABB::empty();

    for p in s.rows.positions.chunks_exact(3) {
        bounds.union_with_point(p[0] as f64, p[1] as f64, p[2] as f64);
    }

    let count = (s.rows.positions.len() / 3) as u32;
    let colors = &s.rows.colors[..s.rows.colors.len().min(count as usize)];
    c.col.extend_from_slice(colors);
    c.col.resize(first as usize + count as usize, 0xff00_0000); // pad with black
    c.pos.extend_from_slice(&s.rows.positions);
    // normals only when the slice has one per point
    let nrm_first = if s.rows.normals.len() == count as usize {
        c.nrm.extend_from_slice(&s.rows.normals);
        (c.nrm.len() - count as usize) as u32
    } else {
        NO_NORMALS
    };
    c.draws.push(CloudDraw {
        instance: s.row,
        from: s.from,
        count,
        first,
        spacing: resident_spacing(s.lod, s.to).unwrap_or(s.point_px.max(DEFAULT_SPACING)), // no whole node yet: guess
        node_first,
        node_count,
        nrm_first,
    });
    bounds
}

/// Finest spacing of the nodes fully loaded so far.
fn resident_spacing(lod: &CloudLod, to: u32) -> Option<f32> {
    let mut spacing = f64::INFINITY;

    for k in 0..lod.len() {
        let (f, n) = (lod.first[k], lod.count[k]);

        if f >= 0 && n >= 0 && (f + n) as u32 <= to {
            spacing = spacing.min(lod.spacing[k]);
        }
    }

    spacing.is_finite().then_some(spacing as f32)
}

/// One node of a streamed cloud's node table.
fn lod_node(lod: &CloudLod, k: usize) -> LodNode {
    let mut children = [-1i32; 8];

    for (slot, v) in lod.children[k * 8..k * 8 + 8].iter().enumerate() {
        children[slot] = *v;
    }

    let half = lod.size[k] as f32 * 0.5;
    LodNode {
        center: [
            lod.min[k * 3] as f32 + half,
            lod.min[k * 3 + 1] as f32 + half,
            lod.min[k * 3 + 2] as f32 + half,
        ],
        size: lod.size[k] as f32,
        spacing: lod.spacing[k] as f32,
        first: lod.first[k] as u32,
        count: lod.count[k] as u32,
        children,
    }
}
