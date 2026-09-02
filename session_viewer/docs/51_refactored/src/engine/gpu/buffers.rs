//! The GPU floor every lane stands on: `GpuCtx` (device + queue), `GrowBuf` (a table that
//! grows by appending, its live prefix copied GPU-side), `Template` (a unit mesh drawn N
//! times) and the two buffer helpers. No lane, no shader and no per-frame state lives here.

use bytemuck::Pod;
use wgpu::util::DeviceExt;

/// The device/queue pair every resource is made with and every write goes through.
pub struct GpuCtx {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

/// A growable GPU table: ONE growth policy for every lane - capacity grows to
/// `max(need, cap * 3/2)`, the live prefix is copied GPU-side and only the new rows are written.
/// Appending is what lets the CPU copy go after upload. The arena used to grow exact-fit, which
/// copied the whole table per file (drawings: 65-525 ms per upload); 3/2 bounds the slack to 50%.
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
    /// draws from it. COPY_SRC is what lets a grown buffer take the old prefix GPU-side.
    pub fn new(ctx: &GpuCtx, label: &'static str, stride: u64, usage: wgpu::BufferUsages) -> Self {
        let buf = zeroed_buffer(&ctx.device, label, stride, usage);

        Self { buf, len: 0, cap: 1, stride, usage, label }
    }

    /// Append rows. Returns `true` when the buffer was replaced, so the caller knows to rebuild
    /// the bind group pointing at it.
    pub fn append<T: Pod>(&mut self, ctx: &GpuCtx, data: &[T]) -> bool {
        debug_assert_eq!(std::mem::size_of::<T>() as u64, self.stride);

        if data.is_empty() {
            return false;
        }
        let stride = self.stride;
        let need = self.len as u64 + data.len() as u64;
        let mut grew = false;
        if need > self.cap {
            let new_cap = need.max(self.cap * 3 / 2);
            let nb = zeroed_buffer(&ctx.device, self.label, new_cap * stride, self.usage);
            if self.len > 0 {
                // the prefix moves GPU-side; it never travels back through wasm memory
                let mut enc = ctx.device.create_command_encoder(&Default::default());
                enc.copy_buffer_to_buffer(&self.buf, 0, &nb, 0, self.len as u64 * stride);
                ctx.queue.submit([enc.finish()]);
            }
            self.buf = nb;
            self.cap = new_cap;
            grew = true;
        }
        ctx.queue.write_buffer(&self.buf, self.len as u64 * stride, bytemuck::cast_slice(data));
        self.len += data.len() as u32;
        grew
    }

    /// Forget the rows; the buffer and its capacity stay, so a rebuild costs no allocation.
    pub fn reset(&mut self) {
        self.len = 0;
    }

    /// Forget the rows AND the buffer: back to the one zeroed row `new` made, so a cleared
    /// scene holds no GPU memory. The caller rebuilds any bind group over it.
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

/// A unit mesh drawn N times by an instanced lane (the cylinder, the marker quad).
pub struct Template {
    pub vbo: wgpu::Buffer,
    pub ibo: wgpu::Buffer,
    pub index_count: u32,
}

impl Template {
    /// Upload positions and indices once; `label` names the lane (`<label>.vbo`, `<label>.ibo`).
    pub fn new(ctx: &GpuCtx, label: &str, verts: &[[f32; 3]], idx: &[u32]) -> Self {
        let vbo = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{label}.vbo")),
            contents: bytemuck::cast_slice(verts),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let ibo = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{label}.ibo")),
            contents: bytemuck::cast_slice(idx),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self { vbo, ibo, index_count: idx.len() as u32 }
    }
}

/// A fresh buffer of `size` bytes, zero-initialized by WebGPU - the write_buffer splice and the
/// empty-category placeholders both rely on that guarantee.
pub fn zeroed_buffer(
    device: &wgpu::Device,
    label: &str,
    size: u64,
    usage: wgpu::BufferUsages
) -> wgpu::Buffer {
    device.create_buffer(
        &wgpu::BufferDescriptor {
            label: Some(label),
            size,
            usage,
            mapped_at_creation: false,
        })
}

/// One read-only storage buffer at binding 0 - the shape every ink lane's bind group has.
pub fn rows_group(ctx: &GpuCtx, layout: &wgpu::BindGroupLayout, label: &str, buf: &wgpu::Buffer) -> wgpu::BindGroup {
    ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some(label),
        layout,
        entries: &[wgpu::BindGroupEntry { binding: 0, resource: buf.as_entire_binding() }],
    })
}
