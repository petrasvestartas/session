//! The cloud lane's tables: positions, colours, optional normals, the octree nodes, and one
//! `Cloud` record per cloud. `CloudRows` is one upload's delta; `CloudLane` is the GPU side.

use super::buffers::{GpuCtx, GrowBuf, ROWS};
use super::upload::drop_rows;

/// `nrm_first` value of a cloud without normals.
pub const NO_NORMALS: u32 = u32::MAX;

/// One cloud in an upload: `count` points landing at upload-local row `first`, with its node
/// table and spacing.
pub struct CloudDraw {
    pub instance: u32,
    pub count: u32,
    pub first: u32,
    /// Measured point spacing, cloud-local units (drives the splat radius).
    pub spacing: f32,
    /// This cloud's slice of the upload's node table; `node_count` 0 = no octree.
    pub node_first: u32,
    pub node_count: u32,
    /// Upload-local first row in the normals table, or `NO_NORMALS`.
    pub nrm_first: u32,
}

/// One octree node, read off the file: `first`/`count` are RELATIVE to the cloud's point 0
/// and `children` are indices RELATIVE to the cloud's node slice (-1 = none).
#[derive(Clone, Copy)]
pub struct LodNode {
    pub center: [f32; 3],
    pub size: f32,
    pub spacing: f32,
    pub first: u32,
    pub count: u32,
    pub children: [i32; 8],
}

/// One cloud as the lane knows it: its object row, its node slice, and its rows
/// `[first, first + count)` in the lane.
pub struct Cloud {
    pub instance: u32,
    pub spacing: f32,
    pub node_first: u32,
    pub node_count: u32,
    pub nrm_first: u32,
    pub first: u32,
    pub count: u32,
}

/// One upload's clouds: this file's rows only.
#[derive(Default)]
pub struct CloudRows {
    pub pos: Vec<f32>,
    pub col: Vec<u32>,
    pub nrm: Vec<u32>,
    pub draws: Vec<CloudDraw>,
    pub nodes: Vec<LodNode>,
}

impl CloudRows {
    /// Points in this upload so far - the `first` of the next draw.
    pub fn point_count(&self) -> u32 {
        (self.pos.len() / 3) as u32
    }

    /// Empty every table and hand the allocations back.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.pos);
        drop_rows(&mut self.col);
        drop_rows(&mut self.nrm);
        drop_rows(&mut self.draws);
        drop_rows(&mut self.nodes);
    }
}

/// The three point buffers as the point lane binds them.
pub struct PointBufs<'a> {
    pub pos: &'a wgpu::Buffer,
    pub col: &'a wgpu::Buffer,
    pub nrm: &'a wgpu::Buffer,
}

/// The cloud lane on the GPU: three append-only tables, the node table, the clouds.
pub struct CloudLane {
    pos: GrowBuf,
    col: GrowBuf,
    nrm: GrowBuf,
    pub clouds: Vec<Cloud>,
    pub nodes: Vec<LodNode>,
    pub point_count: u32,
}

impl CloudLane {
    /// Three one-row tables - empty until the first upload fills them.
    pub fn new(ctx: &GpuCtx) -> Self {
        Self {
            pos: GrowBuf::new(ctx, "points.buffer", 4, ROWS),
            col: GrowBuf::new(ctx, "points.col.buffer", 4, ROWS),
            nrm: GrowBuf::new(ctx, "points.nrm.buffer", 4, ROWS),
            clouds: Vec::new(),
            nodes: Vec::new(),
            point_count: 0,
        }
    }

    /// Append one upload: rows to the tables, nodes to the node table, one cloud per draw.
    /// Returns whether a buffer moved (the point lane must rebind).
    pub fn append(&mut self, ctx: &GpuCtx, up: &CloudRows) -> bool {
        debug_assert_eq!(up.col.len() * 3, up.pos.len());
        let point_base = self.point_count;
        let nrm_base = self.nrm.len();
        let node_base = self.nodes.len() as u32;

        let mut moved = self.pos.append(ctx, &up.pos);
        moved |= self.col.append(ctx, &up.col);
        moved |= self.nrm.append(ctx, &up.nrm);
        self.point_count = self.pos.len() / 3;
        self.nodes.extend_from_slice(&up.nodes);

        for d in &up.draws {
            let nrm_first = if d.nrm_first == NO_NORMALS { NO_NORMALS } else { nrm_base + d.nrm_first };
            self.clouds.push(Cloud {
                instance: d.instance,
                spacing: d.spacing,
                node_first: d.node_first + node_base,
                node_count: d.node_count,
                nrm_first,
                first: point_base + d.first,
                count: d.count,
            });
        }
        moved
    }

    /// Which cloud a global point row belongs to: (object row, index within that cloud).
    pub fn row_of(&self, row: u32) -> Option<(u32, u32)> {
        for c in &self.clouds {
            if row >= c.first && row < c.first + c.count {
                return Some((c.instance, row - c.first));
            }
        }
        None
    }

    /// The three point buffers.
    pub fn buffers(&self) -> PointBufs<'_> {
        PointBufs { pos: &self.pos.buf, col: &self.col.buf, nrm: &self.nrm.buf }
    }

    /// Forget every row and record; capacity stays.
    pub fn reset(&mut self) {
        self.pos.reset();
        self.col.reset();
        self.nrm.reset();
        self.point_count = 0;
        self.clouds.clear();
        self.nodes.clear();
    }

    /// Hand every buffer and both lists back; the caller rebinds the point lane.
    pub fn release(&mut self, ctx: &GpuCtx) {
        self.reset();
        self.pos.release(ctx);
        self.col.release(ctx);
        self.nrm.release(ctx);
        self.clouds.shrink_to_fit();
        self.nodes.shrink_to_fit();
    }
}
