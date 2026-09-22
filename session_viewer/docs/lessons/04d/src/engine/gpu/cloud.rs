// --8<-- [start:step-1a]
use super::buffers::{GpuCtx, GrowBuf, ROWS};
use super::upload::drop_rows;

/// Marker for a cloud that has no normals.
pub const NO_NORMALS: u32 = u32::MAX;

/// One batch of points added to a cloud.
pub struct CloudDraw {
    pub instance: u32, // object row of the cloud
    pub from: u32, // first point index within the cloud
    pub count: u32, // points in this batch
    pub first: u32, // row of the first point in the upload
    pub spacing: f32, // typical distance between points
    pub node_first: u32, // This cloud's nodes; 0 nodes = no octree.
    pub node_count: u32,
    pub nrm_first: u32, // first normal row, or NO_NORMALS
}

/// One box of the cloud's octree.
#[derive(Clone, Copy)]
pub struct LodNode {
    pub center: [f32; 3], // box center
    pub size: f32, // box edge length
    pub spacing: f32, // point spacing inside the box
    pub first: u32, // first point of the box, within the cloud
    pub count: u32, // points in the box
    pub children: [i32; 8], // child node indices, -1 = none
}

/// A run of cloud points stored on the GPU.
#[derive(Clone, Copy)]
pub struct Chunk {
    pub from: u32, // first point index within the cloud
    pub to: u32, // one past the last point index
    pub row: u32, // GPU row of the first point
}

impl Chunk {
    /// GPU row of cloud point `i`.
    pub fn row_of(&self, i: u32) -> u32 {
        self.row + (i - self.from)
    }
}

// --8<-- [end:step-1a]
// --8<-- [start:step-1b]
/// One point cloud on the GPU.
pub struct Cloud {
    pub instance: u32, // object row
    pub spacing: f32, // typical distance between points
    pub node_first: u32,
    pub node_count: u32,
    pub nrm_first: u32, // first normal row, or NO_NORMALS
    pub resident: u32, // points uploaded so far
    pub chunks: Vec<Chunk>, // where those points live
}

impl Cloud {
    /// GPU row of cloud point `i`, None if not uploaded.
    pub fn row_of(&self, i: u32) -> Option<u32> {
        for chunk in &self.chunks {
            if i >= chunk.from && i < chunk.to {
                return Some(chunk.row_of(i));
            }
        }

        None
    }
}

/// Point rows of one upload.
#[derive(Default)]
pub struct CloudRows {
    pub pos: Vec<f32>, // x, y, z per point
    pub col: Vec<u32>, // packed color per point
    pub nrm: Vec<u32>, // packed normal per point
    pub draws: Vec<CloudDraw>, // point batches in this upload
    pub nodes: Vec<LodNode>, // octree nodes in this upload
}

impl CloudRows {
    /// Points in this upload so far.
    pub fn point_count(&self) -> u32 {
        (self.pos.len() / 3) as u32
    }

    /// Empty every table and free its memory.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.pos);
        drop_rows(&mut self.col);
        drop_rows(&mut self.nrm);
        drop_rows(&mut self.draws);
        drop_rows(&mut self.nodes);
    }
}

/// The three point buffers, borrowed for binding.
pub struct PointBufs<'a> {
    pub pos: &'a wgpu::Buffer,
    pub col: &'a wgpu::Buffer,
    pub nrm: &'a wgpu::Buffer,
}

// --8<-- [end:step-1b]
// --8<-- [start:step-1c]
/// All point clouds on the GPU.
pub struct CloudLane {
    pos: GrowBuf, // positions
    col: GrowBuf, // colors
    nrm: GrowBuf, // normals
    pub clouds: Vec<Cloud>, // one entry per cloud
    pub nodes: Vec<LodNode>, // octree nodes of every cloud
    pub point_count: u32, // points on the GPU
}

impl CloudLane {
    /// Bytes reserved on the GPU by this lane.
    pub fn allocated_bytes(&self) -> u64 {
        self.pos.buf.size() + self.col.buf.size() + self.nrm.buf.size()
    }

    /// Create the lane with empty buffers.
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

    // --8<-- [end:step-1c]
    // --8<-- [start:step-1d]
    /// Append one upload; returns true if a buffer was replaced.
    pub fn append(&mut self, ctx: &GpuCtx, up: &CloudRows) -> bool {
        debug_assert_eq!(up.col.len() * 3, up.pos.len());
        // rows before this upload
        let point_base = self.point_count;
        let nrm_base = self.nrm.len();
        let node_base = self.nodes.len() as u32;

        let mut moved = self.pos.append(ctx, &up.pos);
        moved |= self.col.append(ctx, &up.col);
        moved |= self.nrm.append(ctx, &up.nrm);
        self.point_count = self.pos.len() / 3;
        self.nodes.extend_from_slice(&up.nodes);

        for d in &up.draws {
            let chunk = Chunk {
                from: d.from,
                to: d.from + d.count,
                row: point_base + d.first,
            };

            // a later batch extends an existing cloud
            if d.from > 0 {
                self.extend(d.instance, chunk);
                continue;
            }

            let nrm_first = if d.nrm_first == NO_NORMALS {
                NO_NORMALS
            } else {
                nrm_base + d.nrm_first
            };
            // first batch opens a new cloud
            self.clouds.push(Cloud {
                instance: d.instance,
                spacing: d.spacing,
                node_first: d.node_first + node_base,
                node_count: d.node_count,
                nrm_first,
                resident: chunk.to,
                chunks: vec![chunk],
            });
        }

        moved
    }

    /// Add a batch to the cloud on object row `instance`.
    fn extend(&mut self, instance: u32, chunk: Chunk) {
        for cloud in &mut self.clouds {
            if cloud.instance != instance {
                continue;
            }

            // batches must arrive in order
            if chunk.from != cloud.resident {
                log::warn!(
                    "cloud chunk [{}, {}) does not continue the {} resident points; dropped",
                    chunk.from,
                    chunk.to,
                    cloud.resident
                );
                return;
            }

            cloud.resident = chunk.to;
            cloud.chunks.push(chunk);
            return;
        }

        log::warn!("cloud chunk for row {instance} arrived before its cloud; dropped");
    }

    // --8<-- [end:step-1d]
    // --8<-- [start:step-1e]
    /// Cloud of a GPU point row: (object row, point index).
    pub fn row_of(&self, row: u32) -> Option<(u32, u32)> {
        for c in &self.clouds {
            for k in &c.chunks {
                if row >= k.row && row < k.row + (k.to - k.from) {
                    return Some((c.instance, k.from + (row - k.row)));
                }
            }
        }

        None
    }

    /// The three point buffers.
    pub fn buffers(&self) -> PointBufs<'_> {
        PointBufs {
            pos: &self.pos.buf,
            col: &self.col.buf,
            nrm: &self.nrm.buf,
        }
    }

    /// Points uploaded across every cloud.
    pub fn resident(&self) -> u32 {
        let mut resident = 0;

        for cloud in &self.clouds {
            resident += cloud.resident;
        }

        resident
    }

    /// Forget every row; keep the buffers.
    pub fn reset(&mut self) {
        self.pos.reset();
        self.col.reset();
        self.nrm.reset();
        self.point_count = 0;
        self.clouds.clear();
        self.nodes.clear();
    }

    /// Forget every row and free the buffers.
    pub fn release(&mut self, ctx: &GpuCtx) {
        self.reset();
        self.pos.release(ctx);
        self.col.release(ctx);
        self.nrm.release(ctx);
        self.clouds.shrink_to_fit();
        self.nodes.shrink_to_fit();
    }
}
// --8<-- [end:step-1e]
