
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

