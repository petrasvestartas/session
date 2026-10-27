    groups: Vec<[wgpu::BindGroup; 4]>,
}
pub struct Lane {
    pub settings:crate::display_settings::Settings,
    layouts: [wgpu::BindGroupLayout; 4],
    shader: RefCell<Option<wgpu::ShaderModule>>,
    color: RefCell<Option<crate::sample_pipelines::Variants<wgpu::RenderPipeline>>>,
