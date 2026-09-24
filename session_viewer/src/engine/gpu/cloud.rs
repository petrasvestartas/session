use super::buffers::{GpuCtx, GrowBuf, ROWS};
use super::upload::drop_rows;
use crate::engine::pipelines::Layouts;

/// Marker for a cloud that has no normals.
pub const NO_NORMALS: u32 = u32::MAX;

/// One batch of points added to a cloud.
pub struct CloudDraw {
    pub instance: u32, // object row of the cloud
    pub from: u32, // first point index within the cloud
    pub count: u32, // points in this batch
    pub first: u32, // row of the first point in the upload
    pub spacing: f32, // typical distance between points
    pub node_first: u32, // first octree node in the upload
    pub node_count: u32, // octree nodes; 0 = no octree
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
    pub nrm: u32, // GPU row of its first normal, or NO_NORMALS
}

impl Chunk {
    /// GPU row of cloud point `i`.
    pub fn row_of(&self, i: u32) -> u32 {
        self.row + (i - self.from)
    }
}

/// One point cloud on the GPU.
pub struct Cloud {
    pub instance: u32, // object row
    pub spacing: f32, // typical distance between points
    pub node_first: u32, // first octree node
    pub node_count: u32, // octree nodes; 0 = no octree
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
    pub expect: u32, // points known to follow this upload
    pub expect_normals: u32, // normals known to follow this upload
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

    /// True when `more` points fit in the largest buffers this device allows.
    pub fn fits(&self, ctx: &GpuCtx, more: u32) -> bool {
        let more = u64::from(more);
        self.pos.fits(ctx, more * 3) && self.col.fits(ctx, more) && self.nrm.fits(ctx, more)
    }

    /// Append one upload; returns true if a buffer was replaced.
    pub fn append(&mut self, ctx: &GpuCtx, up: &CloudRows) -> bool {
        debug_assert_eq!(up.col.len() * 3, up.pos.len());

        // past the device's largest buffer the upload is dropped, not the device
        if !self.fits(ctx, up.point_count()) {
            log::warn!(
                "{} cloud points do not fit beside the {} on the GPU; not drawn",
                up.point_count(),
                self.point_count
            );
            return false;
        }

        // rows before this upload
        let point_base = self.point_count;
        let nrm_base = self.nrm.len();
        let node_base = self.nodes.len() as u32;

        // a full buffer grows to exactly the points known to come, not by half
        let (expect, normals) = (u64::from(up.expect), u64::from(up.expect_normals));
        let mut moved = self.pos.reserve(ctx, up.pos.len() as u64, expect * 3);
        moved |= self.col.reserve(ctx, up.col.len() as u64, expect);
        moved |= self.nrm.reserve(ctx, up.nrm.len() as u64, normals);
        moved |= self.pos.append(ctx, &up.pos);
        moved |= self.col.append(ctx, &up.col);
        moved |= self.nrm.append(ctx, &up.nrm);
        self.point_count = self.pos.len() / 3;
        self.nodes.extend_from_slice(&up.nodes);

        for d in &up.draws {
            let chunk = Chunk {
                from: d.from,
                to: d.from + d.count,
                row: point_base + d.first,
                nrm: if d.nrm_first == NO_NORMALS {
                    NO_NORMALS
                } else {
                    nrm_base + d.nrm_first
                },
            };

            // a later batch extends an existing cloud
            if d.from > 0 {
                self.extend(d.instance, chunk);
                continue;
            }

            // first batch opens a new cloud
            self.clouds.push(Cloud {
                instance: d.instance,
                spacing: d.spacing,
                node_first: d.node_first + node_base,
                node_count: d.node_count,
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

    /// Stop drawing the cloud on object row `instance`; returns its resident points, now dead.
    pub fn kill_instance(&mut self, instance: u32) -> u32 {
        let Some(at) = self.clouds.iter().position(|c| c.instance == instance) else {
            return 0;
        };

        self.clouds.remove(at).resident
    }

    /// Copy the live clouds' points, normals and nodes into buffers of exact size; the old ones are freed.
    pub fn compact(&mut self, ctx: &GpuCtx) {
        let mut pos = Vec::new(); // (first row, rows) runs to keep
        let mut col = Vec::new();
        let mut nrm = Vec::new();
        let mut nodes = Vec::new();
        let mut points = 0u32;
        let mut normals = 0u32;

        for cloud in &mut self.clouds {
            for chunk in &mut cloud.chunks {
                let count = chunk.to - chunk.from;
                pos.push((chunk.row * 3, count * 3));
                col.push((chunk.row, count));
                chunk.row = points;
                points += count;

                if chunk.nrm != NO_NORMALS {
                    nrm.push((chunk.nrm, count));
                    chunk.nrm = normals;
                    normals += count;
                }
            }

            let first = cloud.node_first as usize;
            let end = (first + cloud.node_count as usize).min(self.nodes.len());
            cloud.node_first = nodes.len() as u32;
            nodes.extend_from_slice(&self.nodes[first.min(end)..end]);
        }

        let mut encoder = ctx.device.create_command_encoder(&Default::default());
        let fresh = [
            self.pos.packed(ctx, &mut encoder, &pos),
            self.col.packed(ctx, &mut encoder, &col),
            self.nrm.packed(ctx, &mut encoder, &nrm),
        ];
        ctx.queue.submit([encoder.finish()]);
        let [p, c, n] = fresh;
        self.pos.swap_in(p);
        self.col.swap_in(c);
        self.nrm.swap_in(n);
        self.nodes = nodes;
        self.point_count = points;
    }

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

impl super::lane::Lane for CloudLane {
    fn on_reset(&mut self, _ctx: &GpuCtx) {
        self.reset();
    }

    fn on_release(&mut self, ctx: &GpuCtx, _layouts: &Layouts) {
        self.release(ctx);
    }

    fn bytes(&self) -> (u64, u64) {
        (self.allocated_bytes(), 0)
    }
}
