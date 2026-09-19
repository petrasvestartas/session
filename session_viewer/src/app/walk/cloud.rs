use super::encode::oct16;
use super::{Row, WalkCx};
use crate::app::stream::CloudLod;
use crate::engine::gpu::cloud::CloudRows;
use crate::engine::gpu::{CloudDraw, LodNode, NO_NORMALS};
use session_rust::AABB;
use session_rust::PointCloud;

/// Spacing reported when a cloud is too small to measure.
const DEFAULT_SPACING: f32 = 20.0;

/// The points, the nodes, then the draw record; the per-file point size rides the row.
pub fn walk_cloud(c: &mut CloudRows, pc: &PointCloud, cx: &WalkCx) -> Row {
    let first = c.point_count();
    let node_first = c.nodes.len() as u32;
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

/// Positions, colours and (when every point has one) normals, from the kernel's flat arrays.
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
            0xff00_0000
        });

        if has_normals {
            rows.nrm.push(
                oct16(&[normals[i * 3], normals[i * 3 + 1], normals[i * 3 + 2]]).unwrap_or(0),
            );
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
    (c[0] as u32 & 255)
        | (c[1] as u32 & 255) << 8
        | (c[2] as u32 & 255) << 16
        | (c[3] as u32 & 255) << 24
}

/// The cloud's point spacing from its density: `sqrt(area / n)` over the two longest box
/// edges - a scan samples a surface. Invariant to point order.
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
    e.sort_by(descending_extent);
    let area = e[0] as f64 * e[1] as f64;

    if area <= 0.0 || !area.is_finite() {
        return DEFAULT_SPACING;
    }

    (area / n as f64).sqrt() as f32
}

/// A streamed slice: raw rows off the wire, already converted.
pub struct StreamRows {
    pub positions: Vec<f32>,
    pub colors: Vec<u32>,
}

/// One streamed slice into the lane: rows `[from, to)` of the cloud. The first slice
/// (`from == 0`) carries the cloud's WHOLE node table, so the walk can descend to nodes
/// whose points arrive later and clip them to what is resident.
pub struct StreamSlice<'a> {
    pub rows: StreamRows,
    pub lod: &'a CloudLod,
    pub from: u32,
    pub to: u32,
    pub row: u32,
    pub point_px: f32,
}

/// Append one streamed slice; returns the slice's local box (the first slice's box is the
/// prefix's, which spreads over the whole cloud since the octree stores coarse levels first).
/// A short colour run is padded with opaque black so the colour table stays one word a point.
pub fn walk_stream_slice(c: &mut CloudRows, s: &StreamSlice) -> AABB {
    let first = c.point_count();
    let node_first = c.nodes.len() as u32;
    let mut node_count = 0u32;

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
    c.col.resize(first as usize + count as usize, 0xff00_0000);
    c.pos.extend_from_slice(&s.rows.positions);
    c.draws.push(CloudDraw {
        instance: s.row,
        from: s.from,
        count,
        first,
        // No node is complete yet, so there is no measured spacing to report: stand in the
        // file's own point size, never below `DEFAULT_SPACING`, and read both as cloud-local
        // units. A placeholder until the first whole node lands, not a measurement.
        spacing: resident_spacing(s.lod, s.to).unwrap_or(s.point_px.max(DEFAULT_SPACING)),
        node_first,
        node_count,
        nrm_first: NO_NORMALS,
    });
    bounds
}

/// The finest node spacing among the nodes complete within the first `to` points.
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

/// One node of a streamed cloud's LOD table.
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

/// Rank finite box extents from largest to smallest for the spacing area estimate.
fn descending_extent(a: &f32, b: &f32) -> std::cmp::Ordering {
    b.partial_cmp(a).unwrap()
}
