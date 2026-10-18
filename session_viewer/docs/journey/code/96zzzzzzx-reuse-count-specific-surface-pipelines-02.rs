pub fn pipelines(device:&wgpu::Device,format:wgpu::TextureFormat,view_layout:&wgpu::BindGroupLayout,model_layout:&wgpu::BindGroupLayout,samples:u32)->(wgpu::RenderPipeline,wgpu::RenderPipeline) {
// shader
// layout
        (mesh_pipeline(crate::depth::surface(),true),mesh_pipeline(crate::depth::ink(),false))

}

