//! The GPU floor every lane stands on: `GpuCtx` (device + queue), `GrowBuf` (a table that
//! grows by appending, its live prefix copied GPU-side) and the two buffer helpers. No lane,
//! no shader and no per-frame state lives here.

use bytemuck::Pod;
use wgpu::util::DeviceExt;

/// The device/queue pair every resource is made with and every write goes through.
pub struct GpuCtx {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

/// Storage rows that grow by appending and can be copied GPU-side.
pub const ROWS: wgpu::BufferUsages = wgpu::BufferUsages::STORAGE
    .union(wgpu::BufferUsages::COPY_DST)
    .union(wgpu::BufferUsages::COPY_SRC);

/// Vertex rows that grow by appending.
pub const VERTS: wgpu::BufferUsages = wgpu::BufferUsages::VERTEX
    .union(wgpu::BufferUsages::COPY_DST)
    .union(wgpu::BufferUsages::COPY_SRC);

/// Index rows that grow by appending.
pub const INDICES: wgpu::BufferUsages = wgpu::BufferUsages::INDEX
    .union(wgpu::BufferUsages::COPY_DST)
    .union(wgpu::BufferUsages::COPY_SRC);

/// A growable GPU table under ONE growth policy: capacity becomes `max(need, cap * 3 / 2)`,
/// the live prefix is copied GPU-side and only the new rows are written.
pub struct GrowBuf {
    pub buf: wgpu::Buffer,
    len: u32,
    cap: u64,
    stride: u64,
    usage: wgpu::BufferUsages,
    label: &'static str,
}

impl GrowBuf {
    /// One zeroed row: wgpu cannot bind a 0-byte buffer, and `len` starts at 0 so nothing
    /// draws from it.
    pub fn new(ctx: &GpuCtx, label: &'static str, stride: u64, usage: wgpu::BufferUsages) -> Self {
        let buf = zeroed_buffer(&ctx.device, label, stride, usage);

        Self { buf, len: 0, cap: 1, stride, usage, label }
    }

    /// Append rows. Returns `true` when the buffer was replaced, so the caller rebuilds the
    /// bind group pointing at it.
    pub fn append<T: Pod>(&mut self, ctx: &GpuCtx, data: &[T]) -> bool {
        debug_assert_eq!(std::mem::size_of::<T>() as u64, self.stride);
        if data.is_empty() {
            return false;
        }

        let need = self.len as u64 + data.len() as u64;
        let grew = need > self.cap;
        if grew {
            self.grow(ctx, need.max(self.cap * 3 / 2));
        }
        ctx.queue.write_buffer(&self.buf, self.len as u64 * self.stride, bytemuck::cast_slice(data));
        self.len += data.len() as u32;
        grew
    }

    /// Replace the buffer with one of `new_cap` rows, moving the live prefix GPU-side.
    fn grow(&mut self, ctx: &GpuCtx, new_cap: u64) {
        let nb = zeroed_buffer(&ctx.device, self.label, new_cap * self.stride, self.usage);
        if self.len > 0 {
            let mut enc = ctx.device.create_command_encoder(&Default::default());
            enc.copy_buffer_to_buffer(&self.buf, 0, &nb, 0, self.len as u64 * self.stride);
            ctx.queue.submit([enc.finish()]);
        }
        self.buf = nb;
        self.cap = new_cap;
    }

    /// Overwrite rows `[at, at + data.len())`, which must already exist.
    pub fn write_at<T: Pod>(&self, ctx: &GpuCtx, at: u32, data: &[T]) {
        debug_assert!(at as u64 + data.len() as u64 <= self.cap);
        ctx.queue.write_buffer(&self.buf, at as u64 * self.stride, bytemuck::cast_slice(data));
    }

    /// Forget the rows; the buffer and its capacity stay, so a rebuild costs no allocation.
    pub fn reset(&mut self) {
        self.len = 0;
    }

    /// Forget the rows AND the buffer: back to one zeroed row, so a cleared scene holds no
    /// GPU memory. The caller rebuilds any bind group over it.
    pub fn release(&mut self, ctx: &GpuCtx) {
        self.buf = zeroed_buffer(&ctx.device, self.label, self.stride, self.usage);
        self.len = 0;
        self.cap = 1;
    }

    /// Rows on the GPU - the base for the next append and the instance count of a draw.
    pub fn len(&self) -> u32 {
        self.len
    }

    /// No rows: the draw that reads this table is skipped.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// A static index buffer holding `indices`, for a per-instance vertex pattern.
pub fn index_buffer(ctx: &GpuCtx, label: &str, indices: &[u16]) -> wgpu::Buffer {
    ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some(label), contents: bytemuck::cast_slice(indices), usage: wgpu::BufferUsages::INDEX })
}

/// A fresh buffer of `size` bytes, zero-initialized by WebGPU.
pub fn zeroed_buffer(device: &wgpu::Device, label: &str, size: u64, usage: wgpu::BufferUsages) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor { label: Some(label), size, usage, mapped_at_creation: false })
}

/// A uniform buffer holding one `T`, writable every frame.
pub fn uniform_buffer<T: Pod>(device: &wgpu::Device, label: &str, value: &T) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents: bytemuck::bytes_of(value),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}

/// A bind group over `buffers` in binding order, one entry each.
pub fn bind_group(ctx: &GpuCtx, layout: &wgpu::BindGroupLayout, label: &str, buffers: &[&wgpu::Buffer]) -> wgpu::BindGroup {
    let mut entries: Vec<wgpu::BindGroupEntry> = Vec::with_capacity(buffers.len());
    for (i, b) in buffers.iter().enumerate() {
        entries.push(wgpu::BindGroupEntry { binding: i as u32, resource: b.as_entire_binding() });
    }
    ctx.device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some(label), layout, entries: &entries })
}
