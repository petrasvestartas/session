//! The GPU floor every lane stands on: `GpuCtx` (device + queue).
//! No lane, no shader and no per-frame state lives here.

/// The device/queue pair every resource is made with and every write goes through.
pub struct GpuCtx {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}
