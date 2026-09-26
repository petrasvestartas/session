// --8<-- [start:002-ctx]
use bytemuck::Pod; // Pod = plain old data: a type that may be copied as raw bytes
use wgpu::util::DeviceExt; // a trait: `use` brings its methods, such as create_buffer_init, into scope

/// The GPU connection: device makes resources, queue runs commands.
pub struct GpuCtx {
    pub device: wgpu::Device, // creates buffers, textures, pipelines
    pub queue: wgpu::Queue,   // uploads data and submits commands
}

impl GpuCtx {
    /// A connection with nothing compiled yet.
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        Self {
            device,
            queue,
        }
    }
}
// --8<-- [end:002-ctx]
