//! The GPU floor every lane stands on: `GpuCtx` (device + queue) and the two buffer helpers.
//! No lane, no shader and no per-frame state lives here.

use bytemuck::Pod;
use wgpu::util::DeviceExt;

/// The device/queue pair every resource is made with and every write goes through.
pub struct GpuCtx {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
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
