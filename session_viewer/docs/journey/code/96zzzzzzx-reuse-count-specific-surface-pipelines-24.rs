fn pipelines(device:&wgpu::Device,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout,layout:&wgpu::BindGroupLayout,samples:u32)->(wgpu::RenderPipeline,wgpu::RenderPipeline) {
// descriptors
        (create(None),create(Some(wgpu::BlendState::ALPHA_BLENDING)))

}
impl L