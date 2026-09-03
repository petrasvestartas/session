//! The walked cloud lane - points that came through the kernel walk: three flat tables
//! (positions, RGBA8 colours, oct16 normals), one draw record per cloud and the octree nodes
//! the LOD walk reads. `CloudRows` is one upload; `CloudLane` the GPU. Streamed clouds live in
//! `stream.rs`; the splatter that reads both lanes is `splat.rs`.

use super::buffers::{GpuCtx, GrowBuf};
use super::splat::PointBufs;

/// One cloud's contiguous point range, as the record builder sees it. It was a
/// `(first, count, instance, spacing)` tuple until the octree gave every cloud a second
/// range - its slice of the LOD node table - and six positional fields is where a tuple
/// stops being readable.
#[derive(Clone, Copy)]
pub struct CloudDraw {
    pub first: u32,      // absolute first row in the cloud tables
    pub count: u32,
    pub instance: u32,   // the instance row this cloud draws against
    pub spacing: f32,    // measured point spacing, world units (0 = unknown)
    pub node_first: u32, // first LodNode of this cloud in the nodes table (walked lane)
    pub node_count: u32, // 0 = no octree (streamed clouds) - the record covers everything
}

/// One octree node of a WALKED cloud (kernel `SpatialOctree`): its own spacing-limited
/// subsample as a row range, its cube for the screen-error test, and the accept spacing
/// that drives the attenuated splat radius. `first` is RELATIVE to the cloud's own first
/// point and `children` are indices RELATIVE to the cloud's node slice; -1 = none.
#[derive(Clone, Copy)]
pub struct LodNode {
    pub center: [f32; 3], // cube centre, cloud-LOCAL units
    pub size: f32,        // cube edge, cloud-local units
    pub spacing: f32,     // accept spacing, cloud-local units
    pub first: u32,       // row offset from the draw's own `first`
    pub count: u32,
    pub children: [i32; 8],
}

/// One upload's clouds: this file's rows only. A draw's `first` is ABSOLUTE (`Scene` keeps the
/// running base across files); a node's ranges are relative to its own cloud.
#[derive(Default)]
pub struct CloudRows {
    pub pos: Vec<f32>, // 3 floats per point, 12 B
    pub col: Vec<u32>, // RGBA8 per point, 4 B
    pub nrm: Vec<u32>, // oct16 normal per point (u32::MAX = none), 4 B -> 20 B/pt
    pub draws: Vec<CloudDraw>, // first, count, instance, point spacing world units
    pub nodes: Vec<LodNode>, // every walked cloud's octree nodes; a draw owns one slice
}

/// The walked lane on the GPU: three append-only point tables (the splat compute binds them)
/// and the cumulative draw and node lists the record builder walks every frame.
pub struct CloudLane {
    pub pos: GrowBuf, // positions, array<f32> - three rows per point
    pub col: GrowBuf, // colours, array<u32> RGBA8
    pub nrm: GrowBuf, // normals, array<u32> oct16 (u32::MAX = none)
    pub draws: Vec<CloudDraw>,
    pub nodes: Vec<LodNode>,
    pub point_count: u32,
}

impl CloudLane {
    /// Three one-row tables - empty until the first set_scene fills them from an upload.
    pub fn new(ctx: &GpuCtx) -> Self {
        let rows = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let pos = GrowBuf::new(ctx, "points.buffer", 4, rows);
        let col = GrowBuf::new(ctx, "points.col.buffer", 4, rows);
        let nrm = GrowBuf::new(ctx, "points.nrm.buffer", 4, rows);

        Self { pos, col, nrm, draws: Vec::new(), nodes: Vec::new(), point_count: 0 }
    }

    /// Append one file's rows (a DELTA). Returns true when any buffer was replaced: the splat
    /// groups bind these three buffers and must be rebound. `draws` carries each cloud's
    /// absolute first-point offset, which `Scene` keeps running across files - so the draw
    /// records append too.
    pub fn append(&mut self, ctx: &GpuCtx, up: &CloudRows) -> bool {
        let mut moved = self.pos.append(ctx, &up.pos);
        moved |= self.col.append(ctx, &up.col);
        moved |= self.nrm.append(ctx, &up.nrm);
        self.point_count = self.pos.len() / 3;

        // The walk numbers a cloud's nodes from the start of ITS upload; the lane's table is
        // cumulative, so every draw's node slice is rebased on the way in - the same thing
        // `Scene.bases.cloud` already does for the point rows.
        let node_base = self.nodes.len() as u32;
        self.nodes.extend_from_slice(&up.nodes);
        self.draws.extend(up.draws.iter().map(|d| CloudDraw { node_first: d.node_first + node_base, ..*d }));
        moved
    }

    /// Forget every row and record; the buffers keep their capacity.
    pub fn reset(&mut self) {
        self.pos.reset();
        self.col.reset();
        self.nrm.reset();
        self.point_count = 0;
        self.draws.clear();
        self.nodes.clear();
    }

    /// Hand every buffer and both lists back; the caller rebinds the splat groups.
    pub fn release(&mut self, ctx: &GpuCtx) {
        self.reset();
        self.pos.release(ctx);
        self.col.release(ctx);
        self.nrm.release(ctx);
        self.draws.shrink_to_fit();
        self.nodes.shrink_to_fit();
    }

    /// The three point buffers as the splat group binds them.
    pub fn buffers(&self) -> PointBufs<'_> {
        PointBufs { pos: &self.pos.buf, col: &self.col.buf, nrm: &self.nrm.buf }
    }
}
