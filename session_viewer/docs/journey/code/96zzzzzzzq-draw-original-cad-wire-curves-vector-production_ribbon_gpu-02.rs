    )
}

pub(crate) fn uniform(binding: u32, stages: ShaderStages) -> wgpu::BindGroupLayoutEntry {
    crate::tile_pipelines::entry(binding, stages, BufferBindingType::Uniform)
}

pub(crate) fn layouts(device: &wgpu::Device) -> [wgpu::BindGroupLayout; 4] {
    let vf = ShaderStages::VERTEX_FRAGMENT;
    let line = ShaderStages::VERTEX_FRAGMENT | ShaderStages::COMPUTE;
    let texture = |binding, stages, sample_type, multisampled| {
