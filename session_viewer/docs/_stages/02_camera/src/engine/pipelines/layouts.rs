//! `Layouts` — every bind-group layout the viewer binds, built once per device. A layout is
//! the SHAPE of a bind group; the buffers live in `gpu/`. Group scheme for every draw:
//! 0 = mvp, 1 = line/pen uniform.

/// One buffer binding, visible to `stages`.
fn buffer_entry(binding: u32, stages: wgpu::ShaderStages, ty: wgpu::BufferBindingType) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: stages,
        ty: wgpu::BindingType::Buffer { ty, has_dynamic_offset: false, min_binding_size: None },
        count: None,
    }
}

/// One uniform buffer at binding 0.
fn uniform_layout(device: &wgpu::Device, label: &str, stages: wgpu::ShaderStages) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some(label),
        entries: &[buffer_entry(0, stages, wgpu::BufferBindingType::Uniform)],
    })
}

/// The bind-group layouts every lane shares.
pub struct Layouts {
    pub mvp: wgpu::BindGroupLayout,
    pub line: wgpu::BindGroupLayout,
}

impl Layouts {
    /// Build every layout once; they outlive any pipeline or bind group made from them.
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            mvp: uniform_layout(device, "mvp.layout", wgpu::ShaderStages::VERTEX),
            line: uniform_layout(device, "line.layout", wgpu::ShaderStages::VERTEX_FRAGMENT),
        }
    }
}
