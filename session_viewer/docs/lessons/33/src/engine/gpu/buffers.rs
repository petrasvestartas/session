use bytemuck::Pod; // Pod = plain old data: no pointers, so its bytes can go straight to the GPU
use wgpu::util::DeviceExt;

/// Device and queue travel together; one borrow of this passes both.
pub struct GpuCtx {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

/// Row buffers: COPY_DST so rows can be written, COPY_SRC so growing can copy them on the GPU.
pub const ROWS: wgpu::BufferUsages = wgpu::BufferUsages::STORAGE
    .union(wgpu::BufferUsages::COPY_DST)
    .union(wgpu::BufferUsages::COPY_SRC);

/// A vertex buffer holds the corner positions; the GPU walks it once per drawn vertex.
pub const VERTS: wgpu::BufferUsages = wgpu::BufferUsages::VERTEX
    .union(wgpu::BufferUsages::COPY_DST)
    .union(wgpu::BufferUsages::COPY_SRC);

/// An index buffer lists the corners of each triangle, so a cube stores 8 corners for its 12 triangles, not 36.
pub const INDICES: wgpu::BufferUsages = wgpu::BufferUsages::INDEX
    .union(wgpu::BufferUsages::COPY_DST)
    .union(wgpu::BufferUsages::COPY_SRC);

/// A GPU buffer cannot grow in place: growing allocates a bigger one and copies, so each growth adds half to keep that rare.
pub struct GrowBuf {
    pub buf: wgpu::Buffer,
    len: u32, // rows in use
    cap: u64, // rows allocated
    stride: u64, // bytes per row
    usage: wgpu::BufferUsages,
    label: &'static str, // name shown in GPU errors
}

impl GrowBuf {
    /// Room for one row, since a zero-size buffer cannot be bound.
    pub fn new(ctx: &GpuCtx, label: &'static str, stride: u64, usage: wgpu::BufferUsages) -> Self {
        let buf = zeroed_buffer(&ctx.device, label, stride, usage);

        Self {
            buf,
            len: 0,
            cap: 1,
            stride,
            usage,
            label,
        }
    }

    /// true = the buffer was replaced, so every bind group that points at it must be rebuilt.
    pub fn append<T: Pod>(&mut self, ctx: &GpuCtx, data: &[T]) -> bool {
        debug_assert_eq!(std::mem::size_of::<T>() as u64, self.stride); // checked in debug builds only, free in release

        if data.is_empty() {
            return false;
        }

        // rows needed after the append
        let need = self.len as u64 + data.len() as u64;
        let grew = need > self.cap;

        if grew {
            // grow by at least half
            self.grow(ctx, need.max(self.cap * 3 / 2));
        }

        // write the new rows after the existing ones
        ctx.queue.write_buffer(
            &self.buf,
            self.len as u64 * self.stride,
            bytemuck::cast_slice(data),
        );
        self.len += data.len() as u32;
        grew
    }

    /// Move to a bigger buffer, copying the rows in use.
    fn grow(&mut self, ctx: &GpuCtx, new_cap: u64) {
        let nb = zeroed_buffer(&ctx.device, self.label, new_cap * self.stride, self.usage);

        if self.len > 0 {
            // copy old rows on the GPU, no round trip
            let mut enc = ctx.device.create_command_encoder(&Default::default());
            enc.copy_buffer_to_buffer(&self.buf, 0, &nb, 0, self.len as u64 * self.stride);
            ctx.queue.submit([enc.finish()]);
        }

        replace_buffer(&mut self.buf, nb);
        self.cap = new_cap;
    }

    /// Overwrite existing rows starting at `at`.
    pub fn write_at<T: Pod>(&self, ctx: &GpuCtx, at: u32, data: &[T]) {
        if data.is_empty() {
            return;
        }

        debug_assert!(at as u64 + data.len() as u64 <= self.cap);
        ctx.queue.write_buffer(
            &self.buf,
            at as u64 * self.stride,
            bytemuck::cast_slice(data),
        );
    }

    /// Forget the rows; keep the buffer.
    pub fn reset(&mut self) {
        self.len = 0;
    }

    /// Forget the rows and free the buffer.
    pub fn release(&mut self, ctx: &GpuCtx) {
        replace_buffer(
            &mut self.buf,
            zeroed_buffer(&ctx.device, self.label, self.stride, self.usage),
        );
        self.len = 0;
        self.cap = 1;
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// A small mesh drawn many times, once per instance.
pub struct Template {
    pub vbo: wgpu::Buffer, // vbo = vertex buffer object
    pub ibo: wgpu::Buffer, // ibo = index buffer object
    pub index_count: u32,
}

impl Template {
    /// Upload the mesh once.
    pub fn new(ctx: &GpuCtx, label: &str, verts: &[[f32; 3]], idx: &[u32]) -> Self {
        let vbo = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{label}.vbo")),
                contents: bytemuck::cast_slice(verts),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let ibo = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{label}.ibo")),
                contents: bytemuck::cast_slice(idx),
                usage: wgpu::BufferUsages::INDEX,
            });

        Self {
            vbo,
            ibo,
            index_count: idx.len() as u32,
        }
    }

    /// Vertex slot 0 = the first entry in the pipeline's list of vertex buffers.
    pub fn bind(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_vertex_buffer(0, self.vbo.slice(..));
        pass.set_index_buffer(self.ibo.slice(..), wgpu::IndexFormat::Uint32);
    }
}

/// WebGPU zero-fills every new buffer, so the zeros cost no upload.
pub fn zeroed_buffer(
    device: &wgpu::Device,
    label: &str,
    size: u64,
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage,
        mapped_at_creation: false,
    })
}

/// Swap in `fresh` and free the old buffer now.
pub fn replace_buffer(slot: &mut wgpu::Buffer, fresh: wgpu::Buffer) {
    std::mem::replace(slot, fresh).destroy();
}

/// A small buffer holding one `T` for shaders to read.
pub fn uniform_buffer<T: Pod>(device: &wgpu::Device, label: &str, value: &T) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents: bytemuck::bytes_of(value),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}

/// A bind group with `buffers` at bindings 0, 1, 2…
pub fn bind_group(
    ctx: &GpuCtx,
    layout: &wgpu::BindGroupLayout,
    label: &str,
    buffers: &[&wgpu::Buffer],
) -> wgpu::BindGroup {
    let mut entries: Vec<wgpu::BindGroupEntry> = Vec::with_capacity(buffers.len());

    for (i, b) in buffers.iter().enumerate() {
        entries.push(wgpu::BindGroupEntry {
            binding: i as u32,
            resource: b.as_entire_binding(),
        });
    }

    ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some(label),
        layout,
        entries: &entries,
    })
}
