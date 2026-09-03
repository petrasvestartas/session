//! The STREAM cloud lane - clouds whose points never existed on the CPU: three exact-fit
//! buffers, every slice written from the socket at a known offset, one draw record per cloud.
//! The splat groups over these buffers belong to `splat.rs`; the `Gpu` entry points at the
//! bottom keep those groups current when the lane's buffers move.

use super::buffers::{zeroed_buffer, GpuCtx};
use super::cloud::CloudDraw;
use super::splat::PointBufs;
use super::Gpu;

/// The STREAM lane: clouds whose points never existed on the CPU. Their own three buffers
/// and record table - the walked lane above is rebuilt whole by every set_scene, so a
/// streamed cloud cannot live in it. The two lanes meet in the shared per-pixel
/// depth/colour buffers: atomics compose across dispatches.
pub struct StreamLane {
    pos: wgpu::Buffer,
    col: wgpu::Buffer,
    nrm: wgpu::Buffer,
    capacity: u64, // rows
    count: u32,
    pos_at: u32,
    col_at: u32,
    pub draws: Vec<CloudDraw>, // (first, count, instance, spacing)
}

impl StreamLane {
    /// One-row placeholders; `begin` grows them for real, exactly, once per cloud.
    pub fn new(ctx: &GpuCtx) -> Self {
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let pos = zeroed_buffer(&ctx.device, "stream.pos", 12, usage);
        let col = zeroed_buffer(&ctx.device, "stream.col", 4, usage);
        let nrm = zeroed_buffer(&ctx.device, "stream.nrm", 4, usage);

        Self { pos, col, nrm, capacity: 1, count: 0, pos_at: 0, col_at: 0, draws: Vec::new() }
    }

    /// Make room for `need` stream rows total, copying the live prefix GPU-side.
    ///
    /// Returns true when the buffers were replaced: the splat group over them must be rebound.
    ///
    /// EXACT, not doubling: appends here are few and huge, so doubling would waste over a
    /// hundred MB on a multi-scan scene AND worsen the worst transient (old+new live at once).
    /// What doubling avoids is a GPU-side copy - the one thing here that never touches wasm.
    fn reserve(&mut self, ctx: &GpuCtx, need: u64) -> bool {
        if need <= self.capacity {
            return false;
        }
        let cap = need;
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let pos = zeroed_buffer(&ctx.device, "stream.pos", cap * 12, usage);
        let col = zeroed_buffer(&ctx.device, "stream.col", cap * 4, usage);
        let nrm = zeroed_buffer(&ctx.device, "stream.nrm", cap * 4, usage);
        if self.count > 0 {
            let mut enc = ctx.device.create_command_encoder(&Default::default());
            enc.copy_buffer_to_buffer(&self.pos, 0, &pos, 0, self.count as u64 * 12);
            enc.copy_buffer_to_buffer(&self.col, 0, &col, 0, self.count as u64 * 4);
            enc.copy_buffer_to_buffer(&self.nrm, 0, &nrm, 0, self.count as u64 * 4);
            ctx.queue.submit([enc.finish()]);
        }
        // The wire has no normals, and a zeroed buffer is NOT "no normal" - oct code 0 decodes
        // to a real direction. Fill the new region with the sentinel, in 1M-row slabs so the
        // staging cost stays bounded.
        let fill = vec![u32::MAX; 1 << 20];
        let mut at = self.count as u64;
        while at < cap {
            let n = (cap - at).min(1 << 20) as usize;
            ctx.queue.write_buffer(&nrm, at * 4, bytemuck::cast_slice(&fill[..n]));
            ctx.queue.submit([]);
            at += n as u64;
        }
        self.pos = pos;
        self.col = col;
        self.nrm = nrm;
        self.capacity = cap;
        true
    }

    /// A cloud is about to STREAM in. The count is known before a single point has been read -
    /// the protobuf packed-double length prefix gives it - so all three buffers are sized once,
    /// exactly, and every slice afterwards lands at a known offset. No growth mid-cloud.
    pub fn begin(&mut self, ctx: &GpuCtx, count: u32, instance: u32) -> bool {
        let moved = self.reserve(ctx, self.count as u64 + count as u64);
        self.draws.push(CloudDraw { first: self.count, count, instance, spacing: 0.0, node_first: 0, node_count: 0 });
        self.pos_at = self.count;
        self.col_at = self.count;
        self.count += count;
        moved
    }

    /// One slice of positions, straight from the socket into GPU memory. `write_buffer` passes
    /// a subarray VIEW of wasm memory - the slice is the only copy that exists. The FIRST slice
    /// also measures the cloud's point spacing (median consecutive distance - scan order is
    /// surface order), which lesson 41's attenuation needs and a streamed cloud cannot get
    /// from the kernel walk.
    pub fn push_pos(&mut self, ctx: &GpuCtx, pos: &[f32]) {
        if let Some(d) = self.draws.last_mut() {
            if d.spacing == 0.0 && self.pos_at == d.first && pos.len() >= 6 {
                d.spacing = median_gap(pos);
            }
        }
        ctx.queue.write_buffer(&self.pos, self.pos_at as u64 * 12, bytemuck::cast_slice(pos));
        self.pos_at += (pos.len() / 3) as u32;
        // Dawn only recycles its upload staging when a submitted serial completes. Without a
        // flush, 165 MB of write_buffer piles 165 MB of staging on top of the destination.
        ctx.queue.submit([]);
    }

    /// The colour run, packed to RGBA8.
    pub fn push_col(&mut self, ctx: &GpuCtx, col: &[u32]) {
        ctx.queue.write_buffer(&self.col, self.col_at as u64 * 4, bytemuck::cast_slice(col));
        self.col_at += col.len() as u32;
        ctx.queue.submit([]);
    }

    /// Re-issue the instance row draw `i` draws against - a rebuild renumbers the objects while
    /// the streamed points keep their GPU rows.
    pub fn retarget(&mut self, i: usize, row: u32) {
        if let Some(d) = self.draws.get_mut(i) {
            d.instance = row;
        }
    }

    /// The three point buffers as the splat group binds them.
    pub fn buffers(&self) -> PointBufs<'_> {
        PointBufs { pos: &self.pos, col: &self.col, nrm: &self.nrm }
    }
}

/// Median distance between consecutive points over the first 2048 - scan order is surface
/// order, so this is an honest point spacing; 0.0 when no two consecutive points differ.
fn median_gap(pos: &[f32]) -> f32 {
    let n = (pos.len() / 3).min(2048);
    let mut gaps: Vec<f32> = (1..n).map(|i| point_gap(pos, i)).filter(|g| *g > 0.0).collect();
    if gaps.is_empty() {
        return 0.0;
    }
    gaps.sort_by(|x, y| x.partial_cmp(y).unwrap());
    gaps[gaps.len() / 2]
}

/// Distance from point `i - 1` to point `i` of a flat xyz run.
fn point_gap(pos: &[f32], i: usize) -> f32 {
    let (a, b) = ((i - 1) * 3, i * 3);
    ((pos[b] - pos[a]).powi(2) + (pos[b + 1] - pos[a + 1]).powi(2) + (pos[b + 2] - pos[a + 2]).powi(2)).sqrt()
}

impl Gpu {
    /// A cloud is about to stream in: reserve its rows; when the lane's buffers moved, re-point
    /// the splat groups at them.
    pub fn cloud_begin(&mut self, count: u32, instance: u32) {
        if self.stream.begin(&self.ctx, count, instance) {
            self.rebind_splat();
            self.splat.invalidate();
        }
    }

    /// One slice of positions; new points, so the splat buffers are stale.
    pub fn cloud_pos(&mut self, pos: &[f32]) {
        self.stream.push_pos(&self.ctx, pos);
        self.splat.invalidate();
    }

    /// One slice of colours; same staleness.
    pub fn cloud_col(&mut self, col: &[u32]) {
        self.stream.push_col(&self.ctx, col);
        self.splat.invalidate();
    }
}
